pub(crate) mod postgres_document_store;
pub(crate) mod postgres_process_store;

use crate::model::SortingOrder;
use crate::model::document::Document;
use crate::model::process::Process;

pub(crate) trait ProcessStore {
    #[allow(dead_code)]
    async fn get_processes(&self) -> anyhow::Result<Vec<Process>>;
    #[allow(dead_code)]
    async fn delete_process(&self, pid: &str) -> anyhow::Result<bool>;
    #[allow(dead_code)]
    async fn exists_process(&self, pid: &str) -> anyhow::Result<bool>;
    async fn get_process(&self, pid: &str) -> anyhow::Result<Option<Process>>;
    async fn store_process(&self, process: Process) -> anyhow::Result<()>;
}

pub(crate) trait DocumentStore {
    async fn add_document(&self, doc: Document<String>) -> anyhow::Result<bool>;
    async fn exists_document(&self, id: &uuid::Uuid) -> anyhow::Result<bool>;
    async fn get_document(&self, id: &str, pid: &str) -> anyhow::Result<Option<Document<String>>>;
    async fn get_documents_for_pid(
        &self,
        pid: &str,
        page: u64,
        size: u64,
        sort: &SortingOrder,
        date: (&chrono::NaiveDateTime, &chrono::NaiveDateTime),
    ) -> anyhow::Result<Vec<Document<String>>>;
}
