use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "code" : 200,
            "message" : "Oeeeei, I'm healthy",
            "version" : env!("CARGO_PKG_VERSION")
        })),
    )
}
