use crate::db::doc_store::bucket::{restore_from_bucket, DocumentBucketSize, DocumentBucketUpdate};
use crate::db::{init_database_client, DataStoreApi};
use crate::model::constants::{
    DOCUMENT_DB, DOCUMENT_DB_CLIENT, MAX_NUM_RESPONSE_ENTRIES, MONGO_COLL_DOCUMENT_BUCKET,
    MONGO_COUNTER, MONGO_DOC_ARRAY, MONGO_DT_ID, MONGO_FROM_TS, MONGO_ID, MONGO_PID, MONGO_TC,
    MONGO_TO_TS, MONGO_TS,
};
use crate::model::document::{Document, EncryptedDocument};
use crate::model::SortingOrder;
use anyhow::anyhow;
use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::options::{AggregateOptions, CreateCollectionOptions, UpdateOptions, WriteConcern};
use mongodb::{bson, Client, IndexModel};

#[derive(Clone, Debug)]
pub struct DataStore {
    pub(crate) client: mongodb::Client,
    database: mongodb::Database,
}

impl DataStoreApi for DataStore {
    fn new(client: Client) -> DataStore {
        DataStore {
            client: client.clone(),
            database: client.database(DOCUMENT_DB),
        }
    }
}

impl DataStore {
    pub async fn init_datastore(db_url: &str, clear_db: bool) -> anyhow::Result<Self> {
        debug!("Using mongodb url: '{:#?}'", &db_url);
        match init_database_client::<DataStore>(db_url, Some(DOCUMENT_DB_CLIENT.to_string())).await
        {
            Ok(datastore) => {
                debug!("Check if database is empty...");
                match datastore
                    .client
                    .database(DOCUMENT_DB)
                    .list_collection_names(None)
                    .await
                {
                    Ok(colls) => {
                        debug!("... found collections: {:#?}", &colls);
                        let number_of_colls =
                            match colls.contains(&MONGO_COLL_DOCUMENT_BUCKET.to_string()) {
                                true => colls.len(),
                                false => 0,
                            };

                        if number_of_colls > 0 && clear_db {
                            debug!("Database not empty and clear_db == true. Dropping database...");
                            match datastore.client.database(DOCUMENT_DB).drop(None).await {
                                Ok(_) => {
                                    debug!("... done.");
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to drop database"));
                                }
                            };
                        }
                        if number_of_colls == 0 || clear_db {
                            debug!("Database empty. Need to initialize...");
                            let mut write_concern = WriteConcern::default();
                            write_concern.journal = Some(true);
                            let mut options = CreateCollectionOptions::default();
                            options.write_concern = Some(write_concern);
                            debug!("Create collection {} ...", MONGO_COLL_DOCUMENT_BUCKET);
                            match datastore
                                .client
                                .database(DOCUMENT_DB)
                                .create_collection(MONGO_COLL_DOCUMENT_BUCKET, options)
                                .await
                            {
                                Ok(_) => {
                                    debug!("... done.");
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to create collection"));
                                }
                            };

                            // This purpose of this index is to ensure that the transaction counter is unique
                            /*let mut index_options = IndexOptions::default();
                            index_options.unique = Some(true);
                            let mut index_model = IndexModel::default();
                            index_model.keys = doc! {format!("{}.{}",MONGO_DOC_ARRAY, MONGO_TC): 1};
                            index_model.options = Some(index_options);

                            debug!("Create unique index for {} ...", MONGO_COLL_DOCUMENT_BUCKET);
                            match datastore
                                .client
                                .database(DOCUMENT_DB)
                                .collection::<Document>(MONGO_COLL_DOCUMENT_BUCKET)
                                .create_index(index_model, None)
                                .await
                            {
                                Ok(result) => {
                                    debug!("... index {} created", result.index_name);
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to create index"));
                                }
                            }*/

                            // This creates a compound index over pid and the timestamp to enable paging using buckets
                            let mut compound_index_model = IndexModel::default();
                            compound_index_model.keys = doc! {MONGO_PID: 1, MONGO_TS: 1};

                            debug!("Create unique index for {} ...", MONGO_COLL_DOCUMENT_BUCKET);
                            match datastore
                                .client
                                .database(DOCUMENT_DB)
                                .collection::<Document>(MONGO_COLL_DOCUMENT_BUCKET)
                                .create_index(compound_index_model, None)
                                .await
                            {
                                Ok(result) => {
                                    debug!("... index {} created", result.index_name);
                                }
                                Err(_) => {
                                    debug!("... failed.");
                                    return Err(anyhow!("Failed to create compound index"));
                                }
                            }
                        }
                        debug!("... database initialized.");
                        Ok(datastore)
                    }
                    Err(_) => Err(anyhow!("Failed to list collections")),
                }
            }
            Err(_) => Err(anyhow!("Failed to initialize database client")),
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn add_document(&self, doc: EncryptedDocument) -> anyhow::Result<bool> {
        debug!("add_document to bucket");
        let coll = self
            .database
            .collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);
        let bucket_update = DocumentBucketUpdate::from(&doc);
        let mut update_options = UpdateOptions::default();
        update_options.upsert = Some(true);
        let id = format!("^{}_", doc.pid.clone());
        let re = mongodb::bson::Regex {
            pattern: id,
            options: String::new(),
        };

        let query = doc! {"_id": re, MONGO_PID: doc.pid.clone(), MONGO_COUNTER: mongodb::bson::bson!({"$lt": MAX_NUM_RESPONSE_ENTRIES as i64})};

        match coll.update_one(query,
                              doc! {
                            "$push": {
                                MONGO_DOC_ARRAY: mongodb::bson::to_bson(&bucket_update)?,
                            },
                            "$inc": {"counter": 1},
                            "$setOnInsert": { "_id": format!("{}_{}_{}", doc.pid.clone(), doc.ts, crate::util::new_uuid()), MONGO_DT_ID: doc.dt_id.clone(), MONGO_FROM_TS: doc.ts},
                            "$set": {MONGO_TO_TS: doc.ts},
                        }, update_options).await {
            Ok(_r) => {
                debug!("added new document: {:#?}", &_r.upserted_id);
                Ok(true)
            }
            Err(e) => {
                error!("failed to store document: {:#?}", &e);
                Err(e.into())
            }
        }
    }

    /// checks if the document exists
    /// document ids are globally unique
    #[tracing::instrument(skip_all)]
    pub async fn exists_document(&self, id: &String) -> anyhow::Result<bool> {
        debug!("Check if document with id '{}' exists...", id);
        let query = doc! {format!("{}.{}", MONGO_DOC_ARRAY, MONGO_ID): id.clone()};

        let coll = self
            .database
            .collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);
        match coll.count_documents(Some(query), None).await? {
            0 => {
                debug!("Document with id '{}' does not exist!", &id);
                Ok(false)
            }
            _ => {
                debug!("... found.");
                Ok(true)
            }
        }
    }

    /// gets the model from the db
    #[tracing::instrument(skip_all)]
    pub async fn get_document(
        &self,
        id: &str,
        pid: &str,
    ) -> anyhow::Result<Option<EncryptedDocument>> {
        debug!("Trying to get doc with id {}...", id);
        let coll = self
            .database
            .collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);

        let pipeline = vec![
            doc! {"$match":{
                MONGO_PID: pid.to_owned(),
                format!("{}.{}", MONGO_DOC_ARRAY, MONGO_ID): id.to_owned(),
            }},
            doc! {"$unwind": format!("${}", MONGO_DOC_ARRAY)},
            doc! {"$addFields": {format!("{}.{}", MONGO_DOC_ARRAY, MONGO_PID): format!("${}", MONGO_PID), format!("{}.{}", MONGO_DOC_ARRAY, MONGO_DT_ID): format!("${}", MONGO_DT_ID)}},
            doc! {"$replaceRoot": { "newRoot": format!("${}", MONGO_DOC_ARRAY)}},
            doc! {"$match":{ MONGO_ID: id.to_owned()}},
        ];

        let mut results = coll.aggregate(pipeline, None).await?;

        if let Some(result) = results.next().await {
            let doc: EncryptedDocument = bson::from_document(result?)?;
            return Ok(Some(doc));
        }

        Ok(None)
    }

    /// gets documents for a single process from the db
    #[tracing::instrument(skip_all)]
    pub async fn get_document_with_previous_tc(
        &self,
        tc: i64,
    ) -> anyhow::Result<Option<EncryptedDocument>> {
        let previous_tc = tc - 1;
        debug!("Trying to get document for tc {} ...", previous_tc);
        if previous_tc < 0 {
            info!("... not entry exists.");
            Ok(None)
        } else {
            let coll = self
                .database
                .collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);

            let pipeline = vec![
                doc! {"$match":{
                    format!("{}.{}", MONGO_DOC_ARRAY, MONGO_TC): previous_tc
                }},
                doc! {"$unwind": format!("${}", MONGO_DOC_ARRAY)},
                doc! {"$addFields": {format!("{}.{}", MONGO_DOC_ARRAY, MONGO_PID): format!("${}", MONGO_PID), format!("{}.{}", MONGO_DOC_ARRAY, MONGO_DT_ID): format!("${}", MONGO_DT_ID)}},
                doc! {"$replaceRoot": { "newRoot": format!("${}", MONGO_DOC_ARRAY)}},
                doc! {"$match":{ MONGO_TC: previous_tc}},
            ];

            let mut results = coll.aggregate(pipeline, None).await?;

            if let Some(result) = results.next().await {
                debug!("Found {:#?}", &result);
                let doc: EncryptedDocument = bson::from_document(result?)?;
                Ok(Some(doc))
            } else {
                warn!("Document with tc {} not found!", previous_tc);
                Ok(None)
            }
        }
    }

    /// gets a page of documents of a specific document type for a single process from the db defined by parameters page, size and sort
    #[tracing::instrument(skip_all)]
    pub async fn get_documents_for_pid(
        &self,
        dt_id: &String,
        pid: &String,
        page: u64,
        size: u64,
        sort: &SortingOrder,
        (date_from, date_to): (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
    ) -> anyhow::Result<Vec<EncryptedDocument>> {
        debug!(
            "...trying to get page {} of size {} of documents for pid {} of dt {}...",
            pid, dt_id, page, size
        );

        match self
            .get_start_bucket_size(dt_id, pid, page, size, sort, (date_from, date_to))
            .await
        {
            Ok(bucket_size) => {
                let offset = DataStore::get_offset(&bucket_size);
                let start_bucket = DataStore::get_start_bucket(page, size, &bucket_size, offset);
                trace!(
                    "...working with start_bucket {} and offset {} ...",
                    start_bucket,
                    offset
                );
                let start_entry =
                    DataStore::get_start_entry(page, size, start_bucket, &bucket_size, offset);

                trace!(
                    "...working with start_entry {} in start_bucket {} and offset {} ...",
                    start_entry,
                    start_bucket,
                    offset
                );

                let skip_buckets = (start_bucket - 1) as i32;
                let sort_order = match sort {
                    SortingOrder::Ascending => 1,
                    SortingOrder::Descending => -1,
                };

                let pipeline = vec![
                    doc! {"$match":{
                    MONGO_PID: pid.clone(),
                    MONGO_DT_ID: dt_id.clone(),
                    MONGO_FROM_TS: {"$lte": date_to.timestamp()},
                    MONGO_TO_TS: {"$gte": date_from.timestamp()}
                    }},
                    doc! {"$sort": {MONGO_FROM_TS: sort_order}},
                    doc! {"$skip": skip_buckets},
                    // worst case: overlap between two buckets.
                    doc! {"$limit": 2},
                    doc! {"$unwind": format! ("${}", MONGO_DOC_ARRAY)},
                    doc! {"$replaceRoot": { "newRoot": "$documents"}},
                    doc! {"$match":{
                    MONGO_TS: {"$gte": date_from.timestamp(), "$lte": date_to.timestamp()}
                    }},
                    doc! {"$sort": {MONGO_TS: sort_order}},
                    doc! {"$skip": start_entry as i32},
                    doc! {"$limit": size as i32},
                ];

                let coll = self
                    .database
                    .collection::<EncryptedDocument>(MONGO_COLL_DOCUMENT_BUCKET);

                let mut options = AggregateOptions::default();
                options.allow_disk_use = Some(true);
                let mut results = coll.aggregate(pipeline, options).await?;

                let mut docs = vec![];
                while let Some(result) = results.next().await {
                    let doc: DocumentBucketUpdate = bson::from_document(result?)?;
                    docs.push(restore_from_bucket(pid.clone(), dt_id.clone(), doc));
                }

                Ok(docs)
            }
            Err(e) => {
                error!("Error while getting bucket offset!");
                Err(e)
            }
        }
    }

    /// offset is necessary for duration queries. There, start_entries of bucket depend on timestamps which usually creates an offset in the bucket
    #[tracing::instrument(skip_all)]
    async fn get_start_bucket_size(
        &self,
        dt_id: &String,
        pid: &String,
        page: u64,
        size: u64,
        sort: &SortingOrder,
        (date_from, date_to): (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
    ) -> anyhow::Result<DocumentBucketSize> {
        debug!("...trying to get the offset for page {} of size {} of documents for pid {} of dt {}...", pid, dt_id, page, size);
        let sort_order = match sort {
            SortingOrder::Ascending => 1,
            SortingOrder::Descending => -1,
        };
        let coll = self
            .database
            .collection::<DocumentBucketSize>(MONGO_COLL_DOCUMENT_BUCKET);

        debug!(
            "... match with pid: {}, dt_it: {}, to_ts <= {}, from_ts >= {} ...",
            pid,
            dt_id,
            date_from.timestamp(),
            date_to.timestamp()
        );
        let pipeline = vec![
            doc! {"$match":{
                MONGO_PID: pid.clone(),
                MONGO_DT_ID: dt_id.clone(),
                MONGO_FROM_TS: {"$lte": date_to.timestamp()},
                MONGO_TO_TS: {"$gte": date_from.timestamp()}
            }},
            // sorting according to sorting order, so we get either the start or end
            doc! {"$sort" : {MONGO_FROM_TS: sort_order}},
            doc! {"$limit" : 1},
            // count all relevant documents in the target bucket
            doc! {"$unwind": format!("${}", MONGO_DOC_ARRAY)},
            doc! {"$match":{
                format!("{}.{}", MONGO_DOC_ARRAY, MONGO_TS): {"$lte": date_to.timestamp(), "$gte": date_from.timestamp()}
            }},
            // modify result to return total number of docs in bucket and number of relevant docs in bucket
            doc! { "$group": { "_id": {"total": "$counter"}, "size": { "$sum": 1 } } },
            doc! { "$project": {"_id":0, "capacity": "$_id.total", "size":true}},
        ];

        let mut options = AggregateOptions::default();
        options.allow_disk_use = Some(true);
        let mut results = coll.aggregate(pipeline, options).await?;
        let mut bucket_size = DocumentBucketSize {
            capacity: MAX_NUM_RESPONSE_ENTRIES as i32,
            size: 0,
        };
        while let Some(result) = results.next().await {
            debug!("... retrieved: {:#?}", &result);
            let result_bucket: DocumentBucketSize = bson::from_document(result?)?;
            bucket_size = result_bucket;
        }
        debug!("... sending offset: {:?}", bucket_size);
        Ok(bucket_size)
    }

    #[tracing::instrument(skip_all)]
    fn get_offset(bucket_size: &DocumentBucketSize) -> u64 {
        (bucket_size.capacity - bucket_size.size) as u64 % MAX_NUM_RESPONSE_ENTRIES
    }

    #[tracing::instrument(skip_all)]
    fn get_start_bucket(
        page: u64,
        size: u64,
        bucket_size: &DocumentBucketSize,
        offset: u64,
    ) -> u64 {
        let docs_to_skip =
            (page - 1) * size + offset + MAX_NUM_RESPONSE_ENTRIES - bucket_size.capacity as u64;
        (docs_to_skip / MAX_NUM_RESPONSE_ENTRIES) + 1
    }

    #[tracing::instrument(skip_all)]
    fn get_start_entry(
        page: u64,
        size: u64,
        start_bucket: u64,
        bucket_size: &DocumentBucketSize,
        offset: u64,
    ) -> u64 {
        // docs to skip calculated by page * size
        let docs_to_skip = (page - 1) * size + offset;
        let mut start_entry = 0;
        if start_bucket > 1 {
            start_entry = docs_to_skip - bucket_size.capacity as u64;
            if start_entry > 2 {
                start_entry -= (start_bucket - 2) * MAX_NUM_RESPONSE_ENTRIES
            }
        }
        start_entry
    }
}

mod bucket {
    use super::EncryptedDocument;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct DocumentBucket {
        pub counter: u64,
        pub pid: String,
        pub dt_id: String,
        pub from_ts: i64,
        pub to_ts: i64,
        pub documents: Vec<EncryptedDocument>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct DocumentBucketSize {
        pub capacity: i32,
        pub size: i32,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct DocumentBucketUpdate {
        pub id: String,
        pub ts: i64,
        pub keys_ct: String,
        pub cts: Vec<String>,
    }

    impl From<&EncryptedDocument> for DocumentBucketUpdate {
        fn from(doc: &EncryptedDocument) -> Self {
            DocumentBucketUpdate {
                id: doc.id.clone(),
                ts: doc.ts,
                keys_ct: doc.keys_ct.clone(),
                cts: doc.cts.to_vec(),
            }
        }
    }

    pub fn restore_from_bucket(
        pid: String,
        dt_id: String,
        bucket_update: DocumentBucketUpdate,
    ) -> EncryptedDocument {
        EncryptedDocument {
            id: bucket_update.id.clone(),
            dt_id,
            pid,
            ts: bucket_update.ts,
            keys_ct: bucket_update.keys_ct.clone(),
            cts: bucket_update.cts.to_vec(),
        }
    }
}
