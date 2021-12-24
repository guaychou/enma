use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::{
    filter::EnvFilter,
    fmt::{format, time::LocalTime},
    layer::SubscriberExt,
    Registry,
};
use {env, figlet_rs::FIGfont, log::info};
pub fn log_init() {
    let time_format = LocalTime::rfc_3339();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    if std::env::var("LOG_COLOR").is_err() {
        std::env::set_var("LOG_COLOR", "false")
    }
    let tracing_format = format()
        .with_timer(time_format)
        .with_ansi(std::env::var("LOG_COLOR").unwrap().parse::<bool>().unwrap());
    let fmt_layer = tracing_subscriber::fmt::Layer::default().event_format(tracing_format);
    let collector = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer);
    LogTracer::init().expect("Failed to set logger");
    set_global_default(collector).expect("Failed to set subscriber");
    print_banner();
}

fn print_banner() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert(env!("CARGO_PKG_NAME"));
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    info!(
        "Starting {} version: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
