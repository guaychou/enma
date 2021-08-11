use {
    crate::{
        config::Config,
        handler::{
            cpu_requested_core::cpu_requested_core, cpu_used_core::cpu_used_core,
            memory_heap_used::memory_heap_used, response_time_average::response_time_average,
            thread_count::thread_count, throughput::throughput, total_pods::total_pods,
        },
        newrelic::newrelic::Newrelic,
    },
    actix_web::{
        dev::Server,
        error, middleware,
        web::{scope, Data, JsonConfig},
        App, HttpResponse, HttpServer,
    },
    std::net::TcpListener,
};

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn build(config: Config) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", config.server.host, config.server.port);
        let listener = TcpListener::bind(&address)?;
        let newrelic = Newrelic::new(&config.newrelic);

        let server = run(listener, newrelic)?;
        Ok(Self { server: server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(listener: TcpListener, newrelic: Newrelic) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                scope("/newrelic/v1")
                    .app_data(Data::new(newrelic.clone()))
                    .service(cpu_requested_core)
                    .service(cpu_used_core)
                    .service(memory_heap_used)
                    .service(response_time_average)
                    .service(thread_count)
                    .service(throughput)
                    .service(total_pods),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
