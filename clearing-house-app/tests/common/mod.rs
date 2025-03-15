pub fn build_multipart_body<T: serde::Serialize>(
    client: &reqwest::Client,
    method: http::Method,
    url: impl reqwest::IntoUrl,
    msg: clearing_house_app::model::ids::message::IdsMessage<T>,
) -> http::Request<reqwest::Body> {
    let header = serde_json::to_vec_pretty(&msg.header).unwrap();
    let header_part = reqwest::multipart::Part::bytes(header)
        .mime_str("application/json")
        .unwrap();

    let mut form = reqwest::multipart::Form::new().part("header", header_part);

    // Handle optional payload
    if let Some(payload) = msg.payload {
        let serialized_payload = serde_json::to_vec_pretty(&payload).unwrap();
        tracing::trace!("Payload: {:?}", &serialized_payload);

        let payload_part = reqwest::multipart::Part::bytes(serialized_payload)
            .mime_str("application/json")
            .unwrap();

        form = form.part("payload", payload_part);
    }

    // Build request
    client
        .request(method, url)
        .multipart(form)
        .build()
        .unwrap()
        .try_into()
        .unwrap()
}

pub async fn parse_multipart_payload<T: serde::de::DeserializeOwned + std::fmt::Debug>(
    response: http::Response<axum::body::Body>,
) -> clearing_house_app::model::ids::message::IdsMessage<T> {
    use std::io::Read;

    // Check that the response is a multipart response
    {
        let content_type_value = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .unwrap()
            .to_str();
        tracing::trace!("Content-Type: {:?}", content_type_value);

        assert!(
            content_type_value
                .unwrap()
                .starts_with("multipart/form-data")
        );
    }

    let boundary = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| ct.split("boundary=").last())
        .expect("Failed to parse boundary")
        .to_string();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    tracing::trace!("Body: {:?}", &body);

    let mut multipart: multipart::server::Multipart<&[u8]> =
        multipart::server::Multipart::with_body(body.as_ref(), boundary);
    let mut header: Option<clearing_house_app::model::ids::message::IdsHeader> = None;
    let mut payload: Option<T> = None;

    while let Some(mut field) = multipart.read_entry().unwrap() {
        let mut buf = Vec::new();
        field.data.read_to_end(&mut buf).unwrap();

        tracing::trace!("Field headers: {:?}", &field.headers);

        match &*field.headers.name {
            "header" => {
                header = Some(serde_json::from_slice(&buf).unwrap());
            }
            "payload" => {
                payload = Some(serde_json::from_slice(&buf).unwrap());
            }
            name => {
                panic!("Unexpected field: {}", name);
            }
        }
    }

    clearing_house_app::model::ids::message::IdsMessage {
        header: header.expect("Header is required"),
        payload,
        payload_type: None,
    }
}

pub async fn create_security_token(
    daps_client: &ids_daps_client::ReqwestDapsClient,
) -> Result<clearing_house_app::model::ids::SecurityToken, ids_daps_client::DapsError> {
    let token_response = daps_client.request_dat().await?;

    Ok(clearing_house_app::model::ids::SecurityToken {
        type_message: clearing_house_app::model::ids::MessageType::DAPSToken,
        id: Some(format!(
            "https://w3id.org/idsa/autogen/dynamicAttributeToken/{}",
            clearing_house_app::util::new_uuid()
        )),
        token_format: Some(
            clearing_house_app::model::ids::InfoModelComplexId::new(
                "https://w3id.org/idsa/code/JWT".to_string(),
            )
            .into(),
        ),
        token_value: token_response,
    })
}

pub async fn start_daps() -> (
    testcontainers::ContainerAsync<testcontainers::GenericImage>,
    String,
    String,
) {
    use testcontainers::runners::AsyncRunner;

    // Starting the test DAPS
    let image = testcontainers::GenericImage::new("ghcr.io/ids-basecamp/daps", "test");
    let container = image
        .with_exposed_port(4567.into()) // will default to TCP protocol
        .with_wait_for(testcontainers::core::WaitFor::message_on_stdout(
            "Listening on 0.0.0.0:4567, CTRL+C to stop",
        ))
        .start()
        .await
        .expect("Failed to start DAPS container. Is Docker running?");

    let host = container.get_host().await.expect("Failed to get host");
    let host_port = container
        .get_host_port_ipv4(4567)
        .await
        .expect("Failed to get port");

    let certs_url = format!("http://{host}:{host_port}/jwks.json");
    let token_url = format!("http://{host}:{host_port}/token");

    // Return container, so it is not dropped and stopped automatically
    (container, certs_url, token_url)
}

pub async fn start_postgres() -> (
    testcontainers::ContainerAsync<testcontainers_modules::postgres::Postgres>,
    String,
) {
    use testcontainers::runners::AsyncRunner;

    let postgres_instance = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .expect("Failed to start Postgres container");

    let connection_string = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        postgres_instance
            .get_host()
            .await
            .expect("Failed to get host"),
        postgres_instance
            .get_host_port_ipv4(5432)
            .await
            .expect("Failed to get port")
    );

    (postgres_instance, connection_string)
}
