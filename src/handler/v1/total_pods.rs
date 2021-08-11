use {
    crate::handler::v1::model,
    crate::newrelic::{metric::Metric, model::NewrelicQueryResult, newrelic::Newrelic},
    actix_web::{post, web, HttpResponse},
    log::{error, warn},
};

#[post("/pods-total")]
async fn total_pods(req: web::Json<model::Request>, newrelic: web::Data<Newrelic>) -> HttpResponse {
    let metric = Metric::TotalPods;
    match newrelic
        .go_query(
            req.data.application_name.as_str(),
            req.data.start_time.as_str(),
            req.data.end_time.as_str(),
            metric,
        )
        .await
    {
        Ok(result) => match result {
            NewrelicQueryResult::Ok(res) => match res.get_unique_count() {
                Some(uniq) => {
                    if uniq.eq(&0.0) {
                        warn!(
                            "Returning zero from newrelic with service: {}, and metric: {:?}",
                            req.data.application_name.as_str(),
                            metric
                        );
                        return HttpResponse::NotFound().json(model::Response::default());
                    }
                    HttpResponse::Ok().json(model::Response::set_response(uniq))
                }
                None => {
                    warn!(
                        "Returning null from newrelic with service: {}, and metric: {:?}",
                        req.data.application_name.as_str(),
                        metric
                    );
                    HttpResponse::NotFound().json(model::Response::default())
                }
            },
            NewrelicQueryResult::Err(e) => {
                error!("{:?}", e.get_error_msg());
                HttpResponse::BadRequest().json(model::Response::default())
            }
        },
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::BadGateway().json(model::Response::default())
        }
    }
}
