use crate::model::claims::ExtractIdsMessage;
use crate::model::ids::message::IdsMessage;
use crate::model::ids::{MessageProcessedNotificationMessage, RejectionMessage, ResultMessage};
use crate::model::process::OwnerList;
use crate::{AppState, model::SortingOrder, model::claims::get_jwks};
use axum::http::StatusCode;
use axum::response::IntoResponse;

async fn log(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    ExtractIdsMessage {
        ch_claims,
        ids_message,
    }: ExtractIdsMessage<serde_json::Value>,
) -> super::ApiResult {
    let correlation_id = ids_message.header.id.clone();
    let daps_token = state.daps_client.request_dat().await.map_err(|e| {
        RejectionMessage::new(
            state.logging_service.issuer(),
            format!("DAPS error: {e:?}"),
            correlation_id.clone(),
        )
    })?;

    let cloned_ids_message: IdsMessage<String> = IdsMessage {
        header: ids_message.header.clone(),
        payload: ids_message.payload.map(|t| t.to_string()),
        payload_type: None,
    };

    match state
        .logging_service
        .log(ch_claims, cloned_ids_message, pid)
        .await
    {
        Ok(receipt) => Ok((
            StatusCode::CREATED,
            MessageProcessedNotificationMessage::new(
                state.logging_service.issuer(),
                &daps_token,
                receipt,
                correlation_id,
            ),
        )
            .into_response()),
        Err(e) => {
            error!("Error while logging: {:?}", e);
            Err(RejectionMessage::new(
                state.logging_service.issuer(),
                format!("Error while logging: {e:?}"),
                correlation_id,
            ))
        }
    }
}

#[derive(serde::Serialize)]
struct CreateProcessResponse {
    pub pid: String,
}

async fn create_process(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    ExtractIdsMessage {
        ch_claims,
        ids_message,
    }: ExtractIdsMessage<OwnerList>,
) -> super::ApiResult {
    let correlation_id = ids_message.header.id.clone();
    let daps_token = state.daps_client.request_dat().await.map_err(|e| {
        RejectionMessage::new(
            state.logging_service.issuer(),
            format!("DAPS error: {e:?}"),
            correlation_id.clone(),
        )
    })?;

    match state
        .logging_service
        .create_process(ch_claims, ids_message, pid)
        .await
    {
        Ok(id) => Ok((
            StatusCode::CREATED,
            MessageProcessedNotificationMessage::new(
                state.logging_service.issuer(),
                &daps_token,
                CreateProcessResponse { pid: id },
                correlation_id,
            ),
        )
            .into_response()),
        Err(e) => {
            error!("Error while creating process: {e:?}");
            Err(RejectionMessage::new(
                state.logging_service.issuer(),
                format!("Error while creating process: {e:?}"),
                correlation_id,
            ))
        }
    }
}

#[derive(serde::Deserialize)]
struct QueryParams {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub sort: Option<SortingOrder>,
    pub date_to: Option<String>,
    pub date_from: Option<String>,
}

async fn query_pid(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(params): axum::extract::Query<QueryParams>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    ExtractIdsMessage {
        ch_claims,
        ids_message,
    }: ExtractIdsMessage<()>,
) -> super::ApiResult {
    let correlation_id = ids_message.header.id.clone();
    let daps_token = state.daps_client.request_dat().await.map_err(|e| {
        RejectionMessage::new(
            state.logging_service.issuer(),
            format!("DAPS error: {e:?}"),
            correlation_id.clone(),
        )
    })?;

    match state
        .logging_service
        .query_pid(
            ch_claims,
            params.page,
            params.size,
            params.sort,
            (params.date_to, params.date_from),
            pid,
        )
        .await
    {
        Ok(result) => Ok((
            StatusCode::OK,
            ResultMessage::new(
                state.logging_service.issuer(),
                &daps_token,
                result,
                correlation_id,
            ),
        )
            .into_response()),
        Err(e) => {
            error!("Error while querying: {e:?}");
            Err(RejectionMessage::new(
                state.logging_service.issuer(),
                format!("Error while querying: {e:?}"),
                correlation_id,
            ))
        }
    }
}

async fn query_id(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pid): axum::extract::Path<String>,
    axum::extract::Path(id): axum::extract::Path<String>,
    ExtractIdsMessage {
        ch_claims,
        ids_message,
    }: ExtractIdsMessage<()>,
) -> super::ApiResult {
    let correlation_id = ids_message.header.id.clone();
    let daps_token = state.daps_client.request_dat().await.map_err(|e| {
        RejectionMessage::new(
            state.logging_service.issuer(),
            format!("DAPS error: {e:?}"),
            correlation_id.clone(),
        )
    })?;

    match state
        .logging_service
        .query_id(ch_claims, pid, id, ids_message)
        .await
    {
        Ok(result) => Ok((
            StatusCode::OK,
            ResultMessage::new(
                state.logging_service.issuer(),
                &daps_token,
                result,
                correlation_id,
            ),
        )
            .into_response()),
        Err(e) => {
            error!("Error while querying: {:?}", e);
            Err(RejectionMessage::new(
                state.logging_service.issuer(),
                format!("Error while querying: {e:?}"),
                correlation_id,
            ))
        }
    }
}

async fn get_public_sign_key(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> super::ApiResult {
    match get_jwks(&state.cert_util) {
        Some(jwks) => Ok((StatusCode::OK, axum::Json(jwks)).into_response()),
        None => Err(RejectionMessage::new(
            state.logging_service.issuer(),
            "Error reading signing key".to_string(),
            None,
        )),
    }
}

pub(crate) fn router() -> axum::routing::Router<AppState> {
    axum::Router::new()
        .route("/messages/log/{pid}", axum::routing::post(log))
        .route("/process/{pid}", axum::routing::post(create_process))
        .route("/messages/query/{pid}", axum::routing::post(query_pid))
        .route("/messages/query/{pid}/{id}", axum::routing::post(query_id))
        .route(
            "/.well-known/jwks.json",
            axum::routing::get(get_public_sign_key),
        )
}
