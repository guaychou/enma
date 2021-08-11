use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Enma, uncontrolled kalimdor replacement")]
pub struct Options {
    /// Enma config path
    #[structopt(long = "config.enma", default_value = "configuration/enma.yaml")]
    config: String,
    /// log4rs config path
    #[structopt(long = "config.log", default_value = "configuration/log4rs.yaml")]
    log_config: String,
}

impl Options {
    pub fn new() -> Self {
        Options::from_args()
    }

    pub fn get_config_path(&self) -> &str {
        self.config.as_str()
    }

    pub fn get_log_config_path(&self) -> &str {
        self.log_config.as_str()
    }
}
