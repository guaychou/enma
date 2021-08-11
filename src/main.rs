use enma::{cli, config::Config, log, startup::Application};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = cli::Options::new();
    log::log_init(cli.get_log_config_path());
    let config = Config::new(cli.get_config_path());
    let app = Application::build(config).await?;
    app.run_until_stopped().await?;
    Ok(())
}
