use crate::configuration::ServerConfig;
use crate::domain::newrelic::Newrelic;
use crate::error::handle_error;
use crate::handler::{health, v1};
use axum::error_handling::HandleErrorLayer;
use axum::AddExtensionLayer;
use axum::{
    extract::{ConnectInfo, MatchedPath},
    http::{Request, Response},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_server::Handle;
use hyper::{http::HeaderValue, Body};
use std::{
    future::ready,
    net::SocketAddr,
    time::{Duration, Instant},
};
use tokio::time::sleep;
use tower::{
    buffer::BufferLayer,
    limit::{ConcurrencyLimitLayer, RateLimitLayer},
    timeout::TimeoutLayer,
    ServiceBuilder,
};

use axum_extra::middleware::{self, Next};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

use tower_http::{trace::TraceLayer, ServiceBuilderExt};
use tracing::Span;

pub struct Application {
    pub router: Router,
    pub handle: Handle,
}

pub fn build(config: ServerConfig, newrelic: Newrelic) -> Application {
    tracing::info!("Building enma application");
    tracing::info!("Initialize HTTP tracing");
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
                    request
                        .headers()
                        .get("X-Real-IP")
                        .unwrap_or(&HeaderValue::from_static(""))
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
    tracing::info!("Initialize middleware stack | {}", config);
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
    tracing::info!("Setting up router...");
    let router = Router::new()
        .route("/health", get(health::health))
        .nest(
            "/v1/newrelic",
            Router::new()
                .route(
                    "/cpu-requested-cores",
                    post(v1::cpu_requested_cores_handler),
                )
                .route("/cpu-used-cores", post(v1::cpu_used_cores_handler))
                .route("/pods-total", post(v1::total_pods_handler))
                .route("/thread-count", post(v1::thread_count_handler))
                .route("/memory-heap-used", post(v1::memory_heap_used_handler))
                .route("/throughput", post(v1::throughput_handler))
                .route(
                    "/response-time-average",
                    post(v1::response_time_average_handler),
                ),
        )
        .layer(middleware_stack);
    let router = if *config.get_metrics() {
        tracing::info!("Prometheus Metrics is Enabled...");
        let recorder_handle = setup_metrics_recorder();
        router
            .route_layer(middleware::from_fn(track_metrics))
            .route("/metrics", get(move || ready(recorder_handle.render())))
    } else {
        tracing::info!("Prometheus Metrics is Disabled...");
        router
    };
    let handle = Handle::new();
    Application {
        router: router,
        handle: handle,
    }
}

pub async fn graceful_shutdown(handle: Handle) {
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
    // Signal the server to shutdown using Handle.
    handle.graceful_shutdown(Some(Duration::from_secs(30)));

    // Print alive connection count every second.
    loop {
        sleep(Duration::from_secs(1)).await;
        tracing::info!("alive connections: {}", handle.connection_count());
    }
}

// Setup metrics recorder
fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    let recorder = PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("enma_http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .expect("Setup prometheus builder error")
        .build_recorder();
    let recorder_handle = recorder.handle();
    metrics::set_boxed_recorder(Box::new(recorder)).unwrap();
    recorder_handle
}

// Registering and add request count to prometheus handler
async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("enma_http_requests_total", &labels);
    metrics::histogram!("enma_http_requests_duration_seconds", latency, &labels);

    response
}
