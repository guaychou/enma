use enma::application;
use enma::configuration;
use enma::domain::newrelic::Newrelic;
use enma::log;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    log::log_init();
    let config = configuration::read_config().await;
    let newrelic = Newrelic::new(config.newrelic);
    let addr: SocketAddr = SocketAddr::from((
        "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap(),
        *config.server.get_port(),
    ));
    let server = axum::Server::bind(&addr)
        .serve(
            application::build(config.server, newrelic)
                .into_make_service_with_connect_info::<SocketAddr, _>(),
        )
        .with_graceful_shutdown(application::shutdown_signal());
    tracing::info!("Listening on {:?}", addr);
    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}
