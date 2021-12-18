use coult::{Config as VaultConfig, Vault};
use std::env;
use std::time::Duration;
use {getset::Getters,log::{info,debug}, serde::Deserialize};

#[derive(Debug, Deserialize, Getters, Clone)]
pub struct NewrelicConfig {
    #[getset(get = "pub with_prefix")]
    api_key: String,
    #[getset(get = "pub with_prefix")]
    account_id: i32,
}

#[derive(Debug,Deserialize, Getters)]
#[serde(default = "default_server_config")]
pub struct ServerConfig {
    #[getset(get = "pub with_prefix")]
    port: u16,
    #[getset(get = "pub with_prefix")]
    buffer: usize,
    #[getset(get = "pub with_prefix")]
    concurrency_limit: usize,
    #[getset(get = "pub with_prefix")]
    rate_limit: u64,
    #[getset(get = "pub with_prefix")]
    #[serde(with = "humantime_serde")]
    limiter_timeout: Duration,
    #[getset(get = "pub with_prefix")]
    #[serde(with = "humantime_serde")]
    timeout: Duration,
}

#[derive(Debug, Deserialize, Getters)]
pub struct Config {
    pub newrelic: NewrelicConfig,
    #[serde(default = "default_server_config")]
    pub server: ServerConfig,
}

fn default_server_config() -> ServerConfig {
    ServerConfig {
        port: 8080,
        buffer: 10,
        concurrency_limit: 5,
        rate_limit: 5,
        limiter_timeout: Duration::from_secs(10),
        timeout: Duration::from_secs(10),
    }
}

pub async fn read_config() -> Config {
    if std::env::var("VAULT_ADDR").is_err() {
        std::env::set_var("VAULT_ADDR", "127.0.0.1")
    };
    if std::env::var("VAULT_PORT").is_err() {
        std::env::set_var("VAULT_PORT", "8200")
    };
    if std::env::var("VAULT_CONFIG_PATH").is_err() {
        std::env::set_var("VAULT_CONFIG_PATH", "secret/config/enma")
    };
    let config = VaultConfig::new(
        env::var("VAULT_ADDR").unwrap(),
        env::var("VAULT_PORT").unwrap().parse::<u16>().unwrap(),
        env::var("VAULT_CONFIG_PATH").unwrap(),
        env::var("VAULT_TOKEN").unwrap(),
    );
    let vault = Vault::new(config).await.unwrap();
    let data = vault.get_secret::<Config>().await.unwrap();
    debug!{"debug server config {:?}", data.server};
    info!("Config has been read from Vault");
    return data;
}
