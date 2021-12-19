use crate::{
    domain::newrelic::{Metric, Newrelic},
    error::AppError,
    extractor::JsonExtractor,
    handler::v1::model::Request,
    handler::v1::model::Response,
};
use axum::{extract::Extension, Json};

// Memory heap used Handler
pub async fn memory_heap_used_handler(
    JsonExtractor(req): JsonExtractor<Request>,
    Extension(newrelic): Extension<Newrelic>,
) -> Result<Json<Response>, AppError> {
    let metric = Metric::MemoryHeapUsed;
    let data = newrelic
        .go_query(
            req.data.application_name.as_str(),
            req.data.start_time.as_str(),
            req.data.end_time.as_str(),
            metric,
        )
        .await?
        .get_average()
        .ok_or_else(|| AppError::NewrelicNull(req.data.application_name, metric))?;
    Ok(Response::set_response(data).into())
}
