#![deny(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    rust_2018_idioms,
    rust_2024_compatibility
)]
#![allow(clippy::module_name_repetitions)]

#[macro_use]
extern crate tracing;

use crate::model::constants::ENV_LOGGING_SERVICE_ID;
use crate::util::ServiceConfig;
use std::sync::Arc;

mod config;
mod db;
pub mod model;
mod ports;
mod services;
pub mod util;

type PostgresLoggingService = services::logging_service::LoggingService<
    db::postgres_process_store::PostgresProcessStore,
    db::postgres_document_store::PostgresDocumentStore,
>;

/// Contains the application state
#[derive(Clone)]
pub(crate) struct AppState {
    pub logging_service: Arc<PostgresLoggingService>,
    pub service_config: Arc<ServiceConfig>,
    pub signing_key_path: String,
}

impl AppState {
    /// Connect to the database and execute database migrations
    async fn setup_postgres(conf: &config::CHConfig) -> anyhow::Result<sqlx::PgPool> {
        info!("Connecting to database");
        let pool = sqlx::PgPool::connect(&conf.database_url).await?;

        info!("Migrating database");
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to migrate database!");

        Ok(pool)
    }

    /// Initialize the application state from config
    async fn init(conf: &config::CHConfig) -> anyhow::Result<Self> {
        #[cfg(feature = "postgres")]
        let pool = Self::setup_postgres(conf).await?;

        trace!("Initializing Process store");
        let process_store =
            db::postgres_process_store::PostgresProcessStore::new(pool.clone(), conf.clear_db)
                .await;

        trace!("Initializing Document store");
        let doc_store =
            db::postgres_document_store::PostgresDocumentStore::new(pool, conf.clear_db).await;

        trace!("Initializing services");
        let doc_service = Arc::new(services::document_service::DocumentService::new(doc_store));
        let logging_service = Arc::new(services::logging_service::LoggingService::new(
            process_store,
            doc_service.clone(),
            conf.static_process_owner.clone(),
        ));

        let service_config = Arc::new(util::init_service_config(ENV_LOGGING_SERVICE_ID)?);
        let signing_key = util::init_signing_key(conf.signing_key.as_deref())?;

        Ok(Self {
            signing_key_path: signing_key,
            service_config,
            logging_service,
        })
    }
}

/// Initialize the application
///
/// # Errors
///
/// Throws an error if the `AppState` cannot be initialized
pub async fn app() -> anyhow::Result<axum::Router> {
    // Read configuration
    let conf = config::read_config(None);
    config::configure_logging(&conf);

    tracing::info!("Config read successfully! Initializing application ...");

    // Initialize application state
    let app_state = AppState::init(&conf).await?;

    // Setup router
    Ok(ports::router().with_state(app_state))
}
