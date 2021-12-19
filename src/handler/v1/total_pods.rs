use crate::{
    domain::newrelic::{Metric, Newrelic},
    error::AppError,
    extractor::JsonExtractor,
    handler::v1::model::Request,
    handler::v1::model::Response,
};
use axum::{extract::Extension, Json};

// Total Pods Handler
pub async fn total_pods_handler(
    JsonExtractor(req): JsonExtractor<Request>,
    Extension(newrelic): Extension<Newrelic>,
) -> Result<Json<Response>, AppError> {
    let metric = Metric::TotalPods;
    let data = newrelic
        .go_query(
            req.data.application_name.as_str(),
            req.data.start_time.as_str(),
            req.data.end_time.as_str(),
            Metric::TotalPods,
        )
        .await?
        .get_unique_count()
        .ok_or_else(|| AppError::NewrelicNull(req.data.application_name, metric))?;
    Ok(Response::set_response(data).into())
}
