use std::fmt::Display;

/// Represents the configuration for the application
#[derive(Debug, serde::Deserialize)]
pub(crate) struct CHConfig {
    pub(crate) database_url: String,
    pub(crate) clear_db: bool,
    pub(crate) issuer: String,
    #[serde(default)]
    pub(crate) log_level: Option<LogLevel>,
    pub(crate) p12_path: String,
    #[serde(default)]
    pub(crate) p12_password: Option<String>,
    pub(crate) daps_token_url: String,
    pub(crate) daps_certs_url: String,
    pub(crate) token_scope: String,
    #[serde(default)]
    pub(crate) static_process_owner: Option<String>,
    performance_tracing: Option<bool>,
}

/// Contains the log level for the application
#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            LogLevel::Trace => String::from("TRACE"),
            LogLevel::Debug => String::from("DEBUG"),
            LogLevel::Info => String::from("INFO"),
            LogLevel::Warn => String::from("WARN"),
            LogLevel::Error => String::from("ERROR"),
        };
        write!(f, "{str}")
    }
}

/// Read configuration from `config.toml` and environment variables. `config_file_override` can be
/// used to override the default config file, mainly for testing purposes.
pub(crate) fn read_config(config_file_override: Option<&std::path::Path>) -> CHConfig {
    // Create config builder
    let mut conf_builder = config::Config::builder();

    // Override config file override path
    conf_builder = if let Some(config_file) = config_file_override {
        conf_builder.add_source(config::File::from(config_file))
    } else {
        conf_builder.add_source(config::File::with_name("config.toml"))
    };

    // Add environment variables and finish
    conf_builder =
        conf_builder.add_source(config::Environment::with_prefix("CH_APP").prefix_separator("_"));

    // Finalize and deserialize
    conf_builder
        .build()
        .expect("Failure to read configuration! Exiting...")
        .try_deserialize::<CHConfig>()
        .expect("Failure to parse configuration! Exiting...")
}

/// Configure logging based on environment variable `RUST_LOG`
pub(crate) fn configure_logging(config: &CHConfig) {
    if std::env::var("RUST_LOG").is_err() {
        if let Some(level) = &config.log_level {
            // Set log level (do not set bollard, hyper_util, multipart to TRACE or DEBUG)
            if let LogLevel::Trace | LogLevel::Debug = level {
                #[allow(unsafe_code)] // Deprecated safe from rust edition 2024
                unsafe {
                    std::env::set_var(
                        "RUST_LOG",
                        format!("{level},multipart=INFO,hyper_util=INFO,bollard=INFO"),
                    );
                }
            } else {
                #[allow(unsafe_code)] // Deprecated safe from rust edition 2024
                unsafe {
                    std::env::set_var("RUST_LOG", level.to_string());
                }
            }
        }
    }

    // setup logging
    let mut subscriber_builder = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env());

    // Add performance tracing
    if let Some(true) = config.performance_tracing {
        subscriber_builder =
            subscriber_builder.with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE);
    }

    subscriber_builder.init();
}

#[cfg(test)]
mod test {
    use serial_test::serial;

    /// Test reading config from environment variables
    #[test]
    #[serial]
    fn test_read_config_from_env() {
        #[allow(unsafe_code)] // Deprecated safe from rust edition 2024
        unsafe {
            std::env::set_var(
                "CH_APP_DATABASE_URL",
                "postgres://my_user:my_password@localhost:5432/ch",
            );
            std::env::set_var("CH_APP_CLEAR_DB", "true");
            std::env::set_var("CH_APP_LOG_LEVEL", "INFO");
            std::env::set_var("CH_APP_STATIC_PROCESS_OWNER", "ABC");
        }

        let conf = super::read_config(None);
        assert_eq!(
            conf.database_url,
            "postgres://my_user:my_password@localhost:5432/ch"
        );
        assert!(conf.clear_db);
        assert_eq!(conf.log_level, Some(super::LogLevel::Info));
        assert_eq!(conf.static_process_owner, Some("ABC".to_string()));

        // Cleanup
        #[allow(unsafe_code)] // Deprecated safe from rust edition 2024
        unsafe {
            std::env::remove_var("CH_APP_DATABASE_URL");
            std::env::remove_var("CH_APP_CLEAR_DB");
            std::env::remove_var("CH_APP_LOG_LEVEL");
            std::env::remove_var("CH_APP_STATIC_PROCESS_OWNER");
        }
    }

    /// Test reading config from toml file
    #[test]
    #[serial]
    fn test_read_config_from_toml() {
        // Create tempfile
        let file = tempfile::Builder::new()
            .suffix(".toml")
            .tempfile()
            .expect("Failure to create tempfile");

        // Write config to file
        let toml = r#"database_url = "postgres://my_user:my_password@localhost:5432/ch"
clear_db = true
log_level = "ERROR"
static_process_owner = "ABC"
issuer = "https://example.com"
p12_path = "keys/connector-certificate.p12"
p12_password = "Password1"  # Optional
daps_token_url = "http://localhost:4567/jwks.json"
daps_certs_url = "http://localhost:4567/token"
token_scope = "idsc:IDS_CONNECTORS_ALL"
"#;

        // Write to file
        std::fs::write(file.path(), toml).expect("Failure to write config file!");

        // Read config
        let conf = super::read_config(Some(file.path()));

        // Test
        assert_eq!(
            conf.database_url,
            "postgres://my_user:my_password@localhost:5432/ch"
        );
        assert!(conf.clear_db);
        assert_eq!(conf.log_level, Some(super::LogLevel::Error));
        assert_eq!(conf.static_process_owner, Some("ABC".to_string()));
        assert_eq!(conf.issuer, "https://example.com");
    }
}
