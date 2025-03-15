# Clearinghouse Quick Start Guide

This guide provides instructions to quickly set up and run the Clearinghouse software implementing the IDSA RAM 4.

## Prerequisites
Before starting, ensure you have the following prerequisites in place:

- A running **DAPS** (Dynamic Attribute Provisioning Service)
- A valid **certificate** in `.p12` format for authentication
- A **PostgreSQL database**

## Running the Application

### 1. Using Cargo
If running with **Cargo**, the environment variables must be set in a `config.toml` file using the following syntax:

```toml
log_level = "INFO"
database_url = "$CLEARING_APP_DATABASE_URL"
p12_path = "/run/secrets/connector.p12"
p12_password = "$CLEARING_EDC_KEYSTORE_PASSWORD"
daps_token_url = "https://${DAPS_DOMAIN}/realms/DAPS/protocol/openid-connect/token"
daps_certs_url = "https://${DAPS_DOMAIN}/realms/DAPS/protocol/openid-connect/certs"
token_scope = "https://daps.dev.mobility-dataspace.eu/realms/DAPS"
static_process_owner = "MDS"
issuer = "https://clearing.dev.mobility-dataspace.eu"
```

To start the application with **Cargo**, navigate to the project directory and run:

```bash
cargo run
```

### 2. Using Docker
Alternatively, you can use **Docker Compose** to run the application in a containerized environment. Ensure that you have **Docker** and **Docker Compose** installed.

#### Running with Docker Compose
1. Create an `docker-compose.yml` file (example [file](../../docker/docker-compose.yml)).
2. Start the application with:

```bash
docker-compose up -d
```

This will spin up the required services in detached mode.

## Environment Variables Breakdown

Below is an explanation of each environment variable required for the application:

- **CH_APP_LOG_LEVEL**: Defines the logging level for the application (e.g., INFO, DEBUG, ERROR).
- **CH_APP_DATABASE_URL**: The connection string for the PostgreSQL database.
- **CH_APP_P12_PATH**: Path to the `.p12` certificate file used for authentication.
- **CH_APP_P12_PASSWORD**: Password for the `.p12` certificate file.
- **CH_APP_DAPS_TOKEN_URL**: URL for obtaining tokens from the DAPS service.
- **CH_APP_DAPS_CERTS_URL**: URL for retrieving DAPS certificates.
- **CH_APP_TOKEN_SCOPE**: Scope of the token used in DAPS authentication.
- **CH_APP_STATIC_PROCESS_OWNER**: Static identifier for the process owner, typically set to "MDS".
- **CH_APP_ISSUER**: The issuer URL for the Clearinghouse instance.

## Additional Notes
- Ensure that your `.p12` certificate is properly mounted in the container when using Docker.
- Verify connectivity to the **DAPS** service to avoid authentication issues.

For further configuration options, refer to the project documentation.

---

*You're now ready to use the Clearinghouse in your Mobility Data Space setup!*
