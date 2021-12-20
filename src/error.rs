use crate::domain::newrelic::model::NewRelicErrorResponseModel;
use crate::domain::newrelic::Metric;
use crate::handler::v1::model::Response as ApplicationResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::Error as RequestError;
use serde_json::json;
use std::convert::Infallible;
use tower::BoxError;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    NewrelicError(NewRelicErrorResponseModel),
    NewrelicNull(String, Metric),
    RequestError(RequestError),
}

impl From<NewRelicErrorResponseModel> for AppError {
    fn from(inner: NewRelicErrorResponseModel) -> Self {
        AppError::NewrelicError(inner)
    }
}

impl From<RequestError> for AppError {
    fn from(inner: RequestError) -> Self {
        AppError::RequestError(inner)
    }
}

// Handle application error
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::NewrelicError(e) => {
                error!("Error from New Relic: {}", e.get_error_msg());
                StatusCode::BAD_GATEWAY
            }
            AppError::RequestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NewrelicNull(service_name, metric) => {
                error!(
                    "Returning zero / null from newrelic with service: {}, and metric: {:?}",
                    service_name, metric
                );
                StatusCode::NOT_FOUND
            }
        };
        let body = Json(ApplicationResponse::default());
        (status, body).into_response()
    }
}

// Handle tower error
pub async fn handle_error(error: BoxError) -> Result<impl IntoResponse, Infallible> {
    if error.is::<tower::timeout::error::Elapsed>() {
        return Ok((
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "code" : 408,
                "error" : "Uhh ohh, request time out",
            })),
        ));
    };
    if error.is::<tower::load_shed::error::Overloaded>() {
        return Ok((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "code" : 503,
                "error" : "Uhh ohh, service unavailable",
            })),
        ));
    }

    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "code" : 500,
            "error" : "Uhh ohh, unhandled internal error",
        })),
    ))
}
