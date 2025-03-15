use crate::AppState;
use crate::model::constants::ENV_SHARED_SECRET;
use crate::model::ids;
use axum::extract::FromRequestParts;
use axum::response::IntoResponse;
use ids_daps_client::DapsError;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChClaims {
    pub client_id: String,
}

impl ChClaims {
    #[must_use]
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
        }
    }
}

impl std::fmt::Display for ChClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.client_id)
    }
}

pub struct ExtractIdsMessage<T> {
    pub ch_claims: ChClaims,
    pub ids_message: ids::message::IdsMessage<T>,
}

impl<S: Send + Sync, T: serde::de::DeserializeOwned + Send> axum::extract::FromRequest<S>
    for ExtractIdsMessage<T>
where
    AppState: axum::extract::FromRef<S>,
{
    type Rejection = axum::response::Response;

    #[allow(clippy::too_many_lines)]
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the state to get the DAPS Client
        let (mut parts, body) = req.into_parts();
        let axum::extract::State(app_state) =
            axum::extract::State::<AppState>::from_request_parts(&mut parts, state)
                .await
                .map_err(axum::response::IntoResponse::into_response)?;

        // Assemble request again and do the Multipart extraction
        let req = axum::extract::Request::from_parts(parts, body);
        let multipart = axum::extract::Multipart::from_request(req, state)
            .await
            .map_err(|_| {
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Expecting multipart request",
                )
                    .into_response()
            })?;

        // Extracting the relevant multipart fields
        let multipart_fields = match extract_multipart_fields(multipart).await {
            Ok(fields) => fields,
            Err(e) => {
                tracing::error!("Error extracting multipart fields: {e}");
                return Err(e.into_response());
            }
        };

        // Parsing the header
        let header: ids::message::IdsHeader = multipart_fields
            .get("header")
            .cloned()
            .map(|b| serde_json::from_slice(&b))
            .transpose()
            .map_err(|e| {
                let raw_body = String::from_utf8_lossy(
                    multipart_fields
                        .get("header")
                        .expect("The 'header' field should exist"),
                );

                tracing::error!(
                    "...retrieve and parse header: {} | raw body: {:?}",
                    e,
                    raw_body
                );
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Invalid 'header' multipart",
                )
                    .into_response()
            })?
            .ok_or_else(|| {
                (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Missing 'header' multipart",
                )
                    .into_response()
            })?;
        tracing::trace!("Header: {:#?}", header);

        // Parsing the payload if exists
        let payload = multipart_fields.get("payload").cloned();

        let payload: Option<T> = if let Some(payload) = payload {
            tracing::trace!("Payload: {:?}", payload);
            let parsed_payload: Option<T> = serde_json::from_slice(&payload).map_err(|e| {
                let raw_body = String::from_utf8_lossy(payload.as_ref());

                tracing::error!("...retrieve and parse payload: {} as json '{raw_body}'", e);
                (axum::http::StatusCode::BAD_REQUEST, "Invalid payload").into_response()
            })?;
            parsed_payload
        } else {
            None
        };

        // Validate the DAPS Token
        tracing::debug!("Validating the DAPS Token ...");
        let token = header.clone().security_token.ok_or_else(|| {
            tracing::error!("Missing security_token");
            (
                axum::http::StatusCode::BAD_REQUEST,
                "Missing security_token",
            )
                .into_response()
        })?;

        let token_claims = app_state
            .daps_client
            .validate_dat(&token.token_value)
            .await
            .map(|t| t.claims)
            .map_err(|e| match e {
                DapsError::InvalidToken => {
                    tracing::error!("Invalid DAPS Token");
                    (axum::http::StatusCode::BAD_REQUEST, "Invalid token").into_response()
                }
                DapsError::DapsHttpClient(ee) => {
                    tracing::error!("Issues with DAPS: {ee}");
                    (
                        axum::http::StatusCode::PRECONDITION_FAILED,
                        "Issues with DAPS, please consult the log",
                    )
                        .into_response()
                }
                DapsError::CacheError(ee) => {
                    tracing::error!("Issues with Certificates Cache: {ee}");
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Issues with the Certificates Cache",
                    )
                        .into_response()
                }
            })?;

        let ch_claims = ChClaims {
            client_id: token_claims.subject,
        };
        let ids_message = ids::message::IdsMessage {
            header,
            payload,
            payload_type: None,
        };

        Ok(ExtractIdsMessage {
            ch_claims,
            ids_message,
        })
    }
}

async fn extract_multipart_fields(
    mut multipart: axum::extract::Multipart,
) -> Result<HashMap<String, bytes::Bytes>, axum::extract::multipart::MultipartError> {
    let mut fields: HashMap<String, bytes::Bytes> = HashMap::new();

    while let Some(field) = multipart.next_field().await? {
        let name = if let Some(name) = field.name() {
            name.to_string()
        } else {
            // We require a name for each field, otherwise we skip it...
            continue;
        };
        let data = field.bytes().await?;

        tracing::debug!("Length of `{}` is {} bytes", name, data.len());

        fields.insert(name, data);
    }

    Ok(fields)
}

/// Returns the `JWKSet` for the RSA keypair at `key_path`
///
/// # Panics
///
/// Panics if the key at `key_path` is not a valid RSA keypair or does not exist.
#[must_use]
pub fn get_jwks(cert_util: &Arc<ids_daps_cert::CertUtil>) -> Option<jsonwebtoken::jwk::JwkSet> {
    use base64::Engine;

    let params = cert_util
        .rsa_exponent_and_modulus()
        .expect("Cannot extract RSA parameters from certificate");

    let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let exponent = engine.encode(params.exponent);
    let modulus = engine.encode(params.modulus);

    let jwk = jsonwebtoken::jwk::Jwk {
        common: jsonwebtoken::jwk::CommonParameters {
            public_key_use: None,
            key_operations: None,
            key_algorithm: None,
            key_id: cert_util.fingerprint().ok(),
            x509_url: None,
            x509_chain: None,
            x509_sha1_fingerprint: None,
            x509_sha256_fingerprint: None,
        },
        algorithm: jsonwebtoken::jwk::AlgorithmParameters::RSA(
            jsonwebtoken::jwk::RSAKeyParameters {
                key_type: jsonwebtoken::jwk::RSAKeyType::RSA,
                n: modulus,
                e: exponent,
            },
        ),
    };

    Some(jsonwebtoken::jwk::JwkSet { keys: vec![jwk] })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Claims<T> {
    #[serde(rename = "iss")]
    issuer: String,
    #[serde(rename = "iat")]
    issued_at: Timestamp,
    #[serde(rename = "aud")]
    audience: String,
    #[serde(rename = "exp")]
    expiration_time: Timestamp,
    #[allow(clippy::struct_field_names)]
    #[serde(flatten)]
    private_claims: T,
}

struct Timestamp(chrono::DateTime<chrono::Utc>);

impl serde::Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let unix_secs = i64::deserialize(deserializer)?;
        let timestamp = chrono::DateTime::from_timestamp(unix_secs, 0);

        if let Some(ts) = timestamp {
            Ok(Timestamp(ts))
        } else {
            Err(D::Error::custom("Invalid timestamp"))
        }
    }
}

/// Decodes the given `token` and validates it against the given `audience`
///
/// # Errors
///
/// Returns an error if the token is invalid or the audience is not as expected.
pub fn decode_token<T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>>(
    token: &str,
    audience: &str,
) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let key = match env::var(ENV_SHARED_SECRET) {
        Ok(secret) => jsonwebtoken::DecodingKey::from_secret(&secret.to_string().into_bytes()),
        Err(e) => {
            error!(
                "Shared Secret not configured. Please configure environment variable {}",
                ENV_SHARED_SECRET
            );
            return Err(e.into());
        }
    };

    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_required_spec_claims(&["iss", "aud", "iat", "exp"]);
    validation.set_audience(&[audience]);
    Ok(jsonwebtoken::decode(token, &key, &validation)
        .map(|t| t.claims)
        .map_err(|e| anyhow::anyhow!("{e}"))?)
}
