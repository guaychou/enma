use crate::configuration::ServerConfig;
use crate::domain::newrelic::Newrelic;
use crate::error::handle_error;
use crate::handler::v1;
use axum::error_handling::HandleErrorLayer;
use axum::AddExtensionLayer;
use axum::{
    extract::ConnectInfo,
    http::{Request, Response},
    routing::post,
    Router,
};
use hyper::Body;
use std::net::SocketAddr;
use std::time::Duration;
use tower::{
    buffer::BufferLayer,
    limit::{ConcurrencyLimitLayer, RateLimitLayer},
    timeout::TimeoutLayer,
    ServiceBuilder,
};

use tower_http::{trace::TraceLayer, ServiceBuilderExt};
use tracing::Span;
pub fn build(config: ServerConfig, newrelic: Newrelic) -> Router {
    let http_trace = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            tracing::info_span!(
                "Request",
                status_code = tracing::field::Empty,
                ms = tracing::field::Empty,
                path = tracing::field::display(request.uri().path()),
                ip = tracing::field::debug(
                    request
                        .extensions()
                        .get::<ConnectInfo<SocketAddr>>()
                        .unwrap()
                ),
                x_real_ip = tracing::field::debug(
                    request.headers().get("X-Real-IP")
                )
                
            )
        })
        .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
            span.record(
                "status_code",
                &tracing::field::display(response.status().as_u16()),
            );
            span.record("ms", &tracing::field::display(latency.as_millis()));
            tracing::info!("response processed")
        });

    let middleware_stack = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .load_shed()
        .layer(BufferLayer::new(*config.get_buffer()))
        .layer(ConcurrencyLimitLayer::new(*config.get_concurrency_limit()))
        .layer(TimeoutLayer::new(*config.get_timeout()))
        .layer(RateLimitLayer::new(
            *config.get_rate_limit(),
            *config.get_limiter_timeout(),
        ))
        .layer(http_trace)
        .layer(AddExtensionLayer::new(newrelic))
        .compression();

    Router::new()
        .nest(
            "/v1/newrelic",
            Router::new().route("/cpu-requested-cores", post(v1::cpu_requested_core_handler)),
        )
        .layer(middleware_stack)
}

pub async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;
    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    tracing::info!("signal received, starting graceful shutdown");
}
