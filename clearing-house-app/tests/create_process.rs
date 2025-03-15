mod common;

use axum::http::{Request, StatusCode};
use clearing_house_app::model::ids::message::{IdsHeader, IdsMessage};
use clearing_house_app::model::ids::{IdsQueryResult, InfoModelId, MessageType, SecurityToken};
use clearing_house_app::model::process::{DataTransaction, OwnerList, Receipt};
use clearing_house_app::util::new_uuid;
use tower::ServiceExt;

#[tokio::test]
async fn log_message() {
    let cert_util = ids_daps_cert::CertUtil::load_certificate(
        std::path::Path::new("keys/connector-certificate.p12"),
        "Password1",
    )
    .expect("The cert_util should be already ready");

    // Starting the test DAPS and creating the DAPS client for executing requests against the Clearing House Server
    let (_daps_container, certs_url, token_url) = common::start_daps().await;
    let daps_client = ids_daps_client::ReqwestDapsClient::from_cert_util(
        &cert_util,
        "idsc:IDS_CONNECTORS_ALL",
        &certs_url,
        &token_url,
        300,
    );

    let client_id = cert_util.ski_aki().unwrap().to_string();

    // Start Postgres
    let (_postgres_container, connection_string) = common::start_postgres().await;

    #[allow(unsafe_code)] // Deprecated safe from rust edition 2024
    unsafe {
        std::env::set_var("CH_APP_LOG_LEVEL", "TRACE");
        std::env::set_var("CH_APP_DAPS_CERTS_URL", certs_url);
        std::env::set_var("CH_APP_DAPS_TOKEN_URL", token_url);
        std::env::set_var("CH_APP_CLEAR_DB", "false");
        std::env::set_var("CH_APP_STATIC_PROCESS_OWNER", "MDS_EDC_CONNECTOR");
        std::env::set_var("CH_APP_DATABASE_URL", connection_string);
    }

    let app = clearing_house_app::app().await.unwrap();

    // Prerequisite JWKS for checking the signature
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/.well-known/jwks.json")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(!body.is_empty());
    let jwks =
        serde_json::from_slice::<jsonwebtoken::jwk::JwkSet>(&body).expect("Decoded the JWKSet");

    // ---------------------------------------------------------------------------------------------

    // Create a process
    let pid = new_uuid();
    let id = new_uuid();

    let process_owners = OwnerList {
        owners: vec![client_id.to_string()],
    };

    let msg = IdsMessage {
        header: IdsHeader {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::RequestMessage,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            security_token: Some(
                common::create_security_token(&daps_client)
                    .await
                    .expect("DAPS Token inserted"),
            ),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: InfoModelId::new("https://w3id.org/idsa/core/ClearingHouse".to_string()),
            ..Default::default()
        },
        payload: Some(process_owners),
        payload_type: None,
    };

    let client = reqwest::Client::new();
    let req = common::build_multipart_body(
        &client,
        http::Method::POST,
        format!("http://0.0.0.0:8080/process/{}", pid),
        msg,
    );

    // Send create process message
    let response = app.clone().oneshot(req).await.unwrap();

    // Check status code
    assert_eq!(response.status(), StatusCode::CREATED);

    // ---------------------------------------------------------------------------------------------

    // Send authorized log message
    let log_msg_payload = serde_json::json!({
        "foo": "Hello World",
        "msg": "MDS",
    });
    let log_msg = IdsMessage {
        header: IdsHeader {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::LogMessage,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            security_token: Some(
                common::create_security_token(&daps_client)
                    .await
                    .expect("DAPS Token inserted"),
            ),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: InfoModelId::new("https://w3id.org/idsa/core/ClearingHouse".to_string()),
            ..Default::default()
        },
        payload: Some(log_msg_payload.clone()),
        payload_type: None,
    };
    let log_req = common::build_multipart_body(
        &client,
        http::Method::POST,
        format!("http://0.0.0.0:8080/messages/log/{}", pid),
        log_msg.clone(),
    );

    // Send log message
    let log_response = app.clone().oneshot(log_req).await.unwrap();

    // Check status code
    assert_eq!(log_response.status(), StatusCode::CREATED);

    // Parse body
    let log_resp: IdsMessage<Receipt> = common::parse_multipart_payload(log_response).await;
    let receipt = log_resp.payload.expect("Receipt is there");

    // Validate receipt
    let decoding_key =
        jsonwebtoken::DecodingKey::from_jwk(&jwks.keys[0]).expect("Decoding Key should be created");

    let mut validation_config = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::PS512);
    validation_config.set_required_spec_claims::<&str>(&[]);

    let decoded_receipt: jsonwebtoken::TokenData<DataTransaction> =
        jsonwebtoken::decode(receipt.data.as_str(), &decoding_key, &validation_config)
            .expect("Decoding JWS successful");
    tracing::debug!("Decoded Receipt: {:?}", decoded_receipt);

    let decoded_receipt_header = decoded_receipt.header;

    assert_eq!(decoded_receipt_header.kid, cert_util.fingerprint().ok(),);

    let decoded_receipt_payload = decoded_receipt.claims;
    tracing::debug!("Decoded Receipt Payload: {:?}", decoded_receipt_payload);
    assert_eq!(decoded_receipt_payload.process_id, pid);
    assert_eq!(
        decoded_receipt_payload.payload,
        serde_json::to_string(&log_msg_payload).unwrap()
    );

    // ---------------------------------------------------------------------------------------------

    // Query ID
    let query_msg = IdsMessage::<()> {
        header: IdsHeader {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::QueryMessage,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            security_token: Some(
                common::create_security_token(&daps_client)
                    .await
                    .expect("DAPS Token inserted"),
            ),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: InfoModelId::new("https://w3id.org/idsa/core/ClearingHouse".to_string()),
            ..Default::default()
        },
        payload: None,
        payload_type: None,
    };
    let query_req = common::build_multipart_body(
        &client,
        http::Method::POST,
        format!("http://0.0.0.0:8080/messages/query/{}", pid),
        query_msg.clone(),
    );

    let query_response = app.clone().oneshot(query_req).await.unwrap();
    assert_eq!(query_response.status(), StatusCode::OK);

    let query_resp: IdsMessage<IdsQueryResult<String>> =
        common::parse_multipart_payload(query_response).await;

    let ids_message = query_resp.payload.expect("IDS Query Result is there");
    tracing::info!("IDS Query Result: {:?}", ids_message);
    let query_docs = ids_message.documents;

    // Check the only document in the result
    assert_eq!(query_docs.len(), 1);
    let doc = query_docs
        .first()
        .expect("Document is there, just checked")
        .to_owned();
    assert_eq!(
        doc.payload.expect("Payload is there"),
        serde_json::to_string(&log_msg_payload).unwrap()
    );
    assert_eq!(doc.header.model_version, "test".to_string());

    // ---------------------------------------------------------------------------------------------

    // Send authorized log message
    let log_msg_payload = "test";
    let log_msg_unauth = IdsMessage {
        header: IdsHeader {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::LogMessage,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            security_token: Some(SecurityToken {
                type_message: MessageType::DAPSToken,
                token_value: "test".to_string(),
                token_format: Some(
                    clearing_house_app::model::ids::InfoModelComplexId::new(
                        "https://w3id.org/idsa/code/JWT".to_string(),
                    )
                    .into(),
                ),
                id: Some(format!(
                    "https://w3id.org/idsa/autogen/dynamicAttributeToken/{}",
                    clearing_house_app::util::new_uuid()
                )),
            }),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: InfoModelId::new("https://w3id.org/idsa/core/ClearingHouse".to_string()),
            ..Default::default()
        },
        payload: Some(log_msg_payload),
        payload_type: None,
    };
    let log_req_unauth = common::build_multipart_body(
        &client,
        http::Method::POST,
        format!("http://0.0.0.0:8080/messages/log/{}", pid),
        log_msg_unauth.clone(),
    );

    // Send log message
    let log_response_unauth = app.clone().oneshot(log_req_unauth).await.unwrap();

    assert_eq!(log_response_unauth.status(), StatusCode::BAD_REQUEST);

    // ---------------------------------------------------------------------------------------------

    // With the real daps client we cannot fake
    /*
    // Send log message from static_process_owner
    let static_process_owner_claims = ChClaims::new("MDS_EDC_CONNECTOR");

    // Send log message
    let log_response_unauth = app
        .clone()
        .oneshot(log_req)
        .await
        .unwrap();

    // Check status code
    assert_eq!(log_response.status(), StatusCode::CREATED);

    // Parse body
    let log_resp: IdsMessage<Receipt> = common::parse_multipart_payload(log_response).await;
    let receipt = log_resp.payload.expect("Receipt is there");

    // Validate receipt
    let decoding_key =
        jsonwebtoken::DecodingKey::from_jwk(&jwks.keys[0]).expect("Decoding Key should be created");

    let mut validation_config = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::PS512);
    validation_config.set_required_spec_claims::<&str>(&[]);

    let decoded_receipt: jsonwebtoken::TokenData<DataTransaction> =
        jsonwebtoken::decode(receipt.data.as_str(), &decoding_key, &validation_config)
            .expect("Decoding JWS successful");
    tracing::debug!("Decoded Receipt: {:?}", decoded_receipt);

    let decoded_receipt_header = decoded_receipt.header;

    assert_eq!(decoded_receipt_header.kid, cert_util.fingerprint().ok(),);

    let decoded_receipt_payload = decoded_receipt.claims;
    tracing::debug!("Decoded Receipt Payload: {:?}", decoded_receipt_payload);
    assert_eq!(decoded_receipt_payload.process_id, pid);
    assert_eq!(decoded_receipt_payload.payload, serde_json::to_string(&log_msg_payload).unwrap());

    // ---------------------------------------------------------------------------------------------

    // Query ID
    let query_msg = IdsMessage::<()> {
        header: IdsHeader {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::QueryMessage,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            security_token: Some(common::create_security_token(&daps_client).await.expect("DAPS Token inserted")),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: InfoModelId::new("https://w3id.org/idsa/core/ClearingHouse".to_string()),
            ..Default::default()
        },
        payload: None,
        payload_type: None,
    };
    let query_req = common::build_multipart_body(&client, http::Method::POST, format!("http://0.0.0.0:8080/messages/query/{}", pid), query_msg.clone());

    let query_response = app
        .clone()
        .oneshot(query_req)
        .await
        .unwrap();
    assert_eq!(query_response.status(), StatusCode::OK);

    let query_resp: IdsMessage<IdsQueryResult<String>> = common::parse_multipart_payload(query_response).await;

    let ids_message = query_resp.payload.expect("IDS Query Result is there");
    tracing::info!("IDS Query Result: {:?}", ids_message);
    let query_docs = ids_message.documents;

    // Check the only document in the result
    assert_eq!(query_docs.len(), 1);
    let doc = query_docs
        .first()
        .expect("Document is there, just checked")
        .to_owned();
    assert_eq!(doc.payload.expect("Payload is there"), serde_json::to_string(&log_msg_payload).unwrap());
    assert_eq!(doc.header.model_version, "test".to_string());

    // ---------------------------------------------------------------------------------------------

    // Send authorized log message
    let log_msg_payload = "test";
    let log_msg_unauth = IdsMessage {
        header: IdsHeader {
            context: Some(std::collections::HashMap::from([
                ("ids".to_string(), "https://w3id.org/idsa/core/".to_string()),
                (
                    "idsc".to_string(),
                    "https://w3id.org/idsa/code/".to_string(),
                ),
            ])),
            type_message: MessageType::LogMessage,
            id: Some(id.clone()),
            model_version: "test".to_string(),
            security_token: Some(SecurityToken {
                type_message: MessageType::DAPSToken,
                token_value: "test".to_string(),
                token_format: Some(clearing_house_app::model::ids::InfoModelComplexId::new("https://w3id.org/idsa/code/JWT".to_string()).into()),
                id: Some(format!("https://w3id.org/idsa/autogen/dynamicAttributeToken/{}", clearing_house_app::util::new_uuid())),
            }),
            issuer_connector: InfoModelId::new("test-connector".to_string()),
            sender_agent: InfoModelId::new("https://w3id.org/idsa/core/ClearingHouse".to_string()),
            ..Default::default()
        },
        payload: Some(log_msg_payload),
        payload_type: None,
    };
    let log_req_unauth = common::build_multipart_body(&client, http::Method::POST, format!("http://0.0.0.0:8080/messages/log/{}", pid), log_msg_unauth.clone());

    // Send log message
    let log_response_unauth = app
        .clone()
        .oneshot(log_req_unauth)
        .await
        .unwrap();

    assert_eq!(log_response_unauth.status(), StatusCode::BAD_REQUEST);

    // ---------------------------------------------------------------------------------------------

    // With the real daps client we cannot fake
    /*
    // Send log message from static_process_owner
    let static_process_owner_claims = ChClaims::new("MDS_EDC_CONNECTOR");

    // Send log message
    let log_response_unauth = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/messages/log/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(
                    SERVICE_HEADER,
                    create_token("test", "test", &static_process_owner_claims),
                )
                .body(serde_json::to_string(&log_msg).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(log_response_unauth.status(), StatusCode::CREATED);

    // ---------------------------------------------------------------------------------------------

    // Query as static_process_owner
    let query_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/messages/query/{}", pid))
                .method("POST")
                .header("Content-Type", "application/json")
                .header(
                    SERVICE_HEADER,
                    create_token("test", "test", &static_process_owner_claims),
                )
                .body(serde_json::to_string(&log_msg).unwrap())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(query_resp.status(), StatusCode::OK);

    let body = axum::body::to_bytes(query_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(!body.is_empty());

    let ids_message = serde_json::from_slice::<IdsQueryResult<String>>(&body).unwrap();
    tracing::info!("IDS Query Result: {:?}", ids_message);
    let query_docs = ids_message.documents;

    // Check the only document in the result
    assert_eq!(query_docs.len(), 2);
    let doc = query_docs
        .first()
        .expect("Document is there, just checked")
        .to_owned();
    assert_eq!(doc.payload.expect("Payload is there"), "test".to_string());
    assert_eq!(doc.header.model_version, "test".to_string());*/
}
