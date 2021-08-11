use {
    crate::handler::model,
    crate::newrelic::{metric::Metric, model::NewrelicQueryResult, newrelic::Newrelic},
    actix_web::{post, web, HttpResponse},
    log::{error, warn},
};

#[post("/throughput")]
async fn throughput(req: web::Json<model::Request>, newrelic: web::Data<Newrelic>) -> HttpResponse {
    let metric = Metric::Throughput;
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
            NewrelicQueryResult::Ok(res) => match res.get_result() {
                Some(res) => {
                    if res.eq(&0.0) {
                        warn!(
                            "Returning zero from newrelic with service: {}, and metric: {:?}",
                            req.data.application_name.as_str(),
                            metric
                        );
                        return HttpResponse::NotFound().json(model::Response {
                            api_version: String::from("v1"),
                            data: model::ResponseData { result: res },
                        });
                    }
                    HttpResponse::Ok().json(model::Response {
                        api_version: String::from("v1"),
                        data: model::ResponseData { result: res },
                    })
                }
                None => {
                    warn!(
                        "Returning null from newrelic with service: {}, and metric: {:?}",
                        req.data.application_name.as_str(),
                        metric
                    );
                    HttpResponse::NotFound().json(model::Response {
                        api_version: String::from("v1"),
                        data: model::ResponseData { result: 0.0 },
                    })
                }
            },
            NewrelicQueryResult::Err(e) => {
                error!("{:?}", e.get_error_msg());
                HttpResponse::BadRequest().json(model::Response {
                    api_version: String::from("v1"),
                    data: model::ResponseData { result: 0.0 },
                })
            }
        },
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::BadGateway().json(model::Response {
                api_version: String::from("v1"),
                data: model::ResponseData { result: 0.0 },
            })
        }
    }
}
