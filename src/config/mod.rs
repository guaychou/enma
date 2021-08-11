use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub newrelic: NewrelicConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: i32,
    pub host: String,
}

#[derive(Deserialize)]
pub struct NewrelicConfig {
    api_key: String,
    account_id: i32,
}

impl NewrelicConfig {
    pub fn get_api_key(&self) -> &str {
        return self.api_key.as_str();
    }
    pub fn get_account_id(&self) -> i32 {
        return self.account_id;
    }
}

impl Config {
    pub fn new(path: &str) -> Self {
        let f = std::fs::File::open(path).expect("Config file not found");
        serde_yaml::from_reader(f).expect("Could not parse the config")
    }
}
