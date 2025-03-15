use crate::db::DocumentStore;
use crate::model::claims::ChClaims;
use crate::model::constants::{DEFAULT_NUM_RESPONSE_ENTRIES, MAX_NUM_RESPONSE_ENTRIES};
use crate::model::document::Document;
use crate::model::{SortingOrder, parse_date, validate_and_sanitize_dates};
use crate::services::{DocumentReceipt, QueryResult};
use std::convert::TryFrom;

/// Error type for `DocumentService`
#[derive(thiserror::Error, Debug)]
pub enum DocumentServiceError {
    #[error("Document already exists!")]
    DocumentAlreadyExists,
    #[error("Document contains no payload!")]
    MissingPayload,
    #[error("Error during database operation: {description}: {source}")]
    DatabaseError {
        source: Box<dyn std::error::Error + Sync + Send>,
        description: String,
    },
    #[error("Invalid dates in query!")]
    InvalidDates,
    #[error("Document not found!")]
    NotFound,
}

impl axum::response::IntoResponse for DocumentServiceError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        match self {
            Self::DocumentAlreadyExists | Self::MissingPayload | Self::InvalidDates => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
            Self::DatabaseError {
                source,
                description,
            } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{description}: {source}"),
            )
                .into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DocumentService<T> {
    db: T,
}

impl<T: DocumentStore> DocumentService<T> {
    pub fn new(db: T) -> Self {
        Self { db }
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn create_enc_document(
        &self,
        ch_claims: ChClaims,
        doc: Document<String>,
    ) -> Result<DocumentReceipt, DocumentServiceError> {
        trace!("...user '{:?}'", &ch_claims.client_id);
        // data validation
        if doc.content.payload.is_none() {
            return Err(DocumentServiceError::MissingPayload);
        }

        // check if doc id already exists
        if let Ok(true) = self.db.exists_document(&doc.id).await {
            warn!("Document exists already!");
            Err(DocumentServiceError::DocumentAlreadyExists)
        } else {
            // prepare the success result message
            let receipt = DocumentReceipt::new(doc.ts, &doc.pid, &doc.id.to_string());

            trace!("storing document ....");
            // store document
            match self.db.add_document(doc).await {
                Ok(_b) => Ok(receipt),
                Err(e) => {
                    error!("Error while adding: {:?}", e);
                    Err(DocumentServiceError::DatabaseError {
                        source: e.into(),
                        description: "Error while adding document".to_string(),
                    })
                }
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn get_enc_documents_for_pid(
        &self,
        ch_claims: ChClaims,
        page: Option<u64>,
        size: Option<u64>,
        sort: Option<SortingOrder>,
        (date_from, date_to): (Option<String>, Option<String>),
        pid: String,
    ) -> Result<QueryResult, DocumentServiceError> {
        debug!("Trying to retrieve documents for pid '{pid}'...");
        trace!("...user '{:?}'", &ch_claims.client_id);
        debug!("...page: {page:?}, size:{size:?} and sort:{sort:?}");

        let sanitized_page = Self::sanitize_page(page);
        let sanitized_size = Self::sanitize_size(size);

        // Sorting order is already validated and defaults to descending
        let sanitized_sort = sort.unwrap_or(SortingOrder::Descending);

        // Parsing the dates for duration queries
        let parsed_date_from = parse_date(date_from, false);
        let parsed_date_to = parse_date(date_to, true);

        // Validation of dates with various checks. If none given dates default to date_now (date_to) and (date_now - 2 weeks) (date_from)
        let Ok((sanitized_date_from, sanitized_date_to)) =
            validate_and_sanitize_dates(parsed_date_from, parsed_date_to, None)
        else {
            debug!("date validation failed!");
            return Err(DocumentServiceError::InvalidDates);
        };

        //new behavior: if pages are "invalid" return {}. Do not adjust page
        //either call db with type filter or without to get cts
        debug!(
            "... using pagination with page: {}, size:{} and sort:{:#?}",
            sanitized_page, sanitized_size, &sanitized_sort
        );

        let docs = match self
            .db
            .get_documents_for_pid(
                &pid,
                sanitized_page,
                sanitized_size,
                &sanitized_sort,
                (&sanitized_date_from, &sanitized_date_to),
            )
            .await
        {
            Ok(docs) => docs,
            Err(e) => {
                error!("Error while retrieving document: {:?}", e);
                return Err(DocumentServiceError::DatabaseError {
                    source: e.into(),
                    description: "Error while retrieving document".to_string(),
                });
            }
        };

        let result_size = i32::try_from(sanitized_size).ok();
        let result_page = i32::try_from(sanitized_page).ok();
        let result_sort = match sanitized_sort {
            SortingOrder::Ascending => String::from("asc"),
            SortingOrder::Descending => String::from("desc"),
        };

        let mut result = QueryResult::new(
            sanitized_date_from.and_utc().timestamp(),
            sanitized_date_to.and_utc().timestamp(),
            result_page,
            result_size,
            result_sort,
            vec![],
        );

        // The db might contain no documents in which case we get an empty vector
        if docs.is_empty() {
            debug!("Queried empty pid: {}", &pid);
            Ok(result)
        } else {
            result.documents = docs;
            Ok(result)
        }
    }

    #[tracing::instrument(skip_all)]
    pub(crate) async fn get_enc_document(
        &self,
        ch_claims: ChClaims,
        pid: String,
        id: String,
        hash: Option<String>,
    ) -> Result<Document<String>, DocumentServiceError> {
        trace!("...user '{:?}'", &ch_claims.client_id);
        trace!("trying to retrieve document with id '{id}' for pid '{pid}'");
        if let Some(hash) = hash {
            debug!("integrity check with hash: {}", hash);
        }

        match self.db.get_document(&id, &pid).await {
            Ok(Some(ct)) => Ok(ct),
            Ok(None) => {
                debug!("Nothing found in db!");
                Err(DocumentServiceError::NotFound) // NotFound
            }
            Err(e) => {
                error!("Error while retrieving document: {:?}", e);
                Err(DocumentServiceError::DatabaseError {
                    source: e.into(),
                    description: "Error while retrieving document".to_string(),
                })
            }
        }
    }

    #[inline]
    fn sanitize_page(page: Option<u64>) -> u64 {
        // Parameter validation for pagination:
        // Valid pages start from 1
        match page {
            Some(p) => {
                if p > 0 {
                    p
                } else {
                    warn!("...invalid page requested. Falling back to 1.");
                    1
                }
            }
            None => 1,
        }
    }

    #[inline]
    fn sanitize_size(size: Option<u64>) -> u64 {
        // Valid sizes are between 1 and MAX_NUM_RESPONSE_ENTRIES (1000)
        match size {
            Some(s) => {
                if s > 0 && s <= MAX_NUM_RESPONSE_ENTRIES {
                    s
                } else {
                    warn!("...invalid size requested. Falling back to default.");
                    DEFAULT_NUM_RESPONSE_ENTRIES
                }
            }
            None => DEFAULT_NUM_RESPONSE_ENTRIES,
        }
    }
}
