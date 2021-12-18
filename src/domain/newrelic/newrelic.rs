use crate::{
    configuration::NewrelicConfig,
    domain::{
        newrelic::metric::Metric,
        newrelic::model::{NewrelicQueryResult, NewrelicResponseModel},
    },
    error::AppError,
};

use {reqwest::Client, tokio::time::Duration};

#[derive(Clone)]
pub struct Newrelic {
    http_client: Client,
    api_key: String,
    account_id: i32,
}

impl Newrelic {
    pub fn new(newrelic_config: NewrelicConfig) -> Self {
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let timeout_duration = Duration::new(5, 0);
        let client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .connect_timeout(timeout_duration)
            .http2_adaptive_window(true)
            .build()
            .unwrap_or_default();
        return Self {
            http_client: client,
            api_key: newrelic_config.get_api_key().to_string(),
            account_id: *newrelic_config.get_account_id(),
        };
    }
    pub async fn go_query(
        &self,
        application_name: &str,
        start_time: &str,
        end_time: &str,
        metric: Metric,
    ) -> Result<NewrelicResponseModel, AppError> {
        let query = metric.get_query(application_name, start_time, end_time);
        let full_url = format!(
            "https://insights-api.newrelic.com/v1/accounts/{}/query?nrql={}",
            self.account_id, query
        );
        match self
            .http_client
            .get(full_url)
            .header("X-Query-Key", self.api_key.as_str())
            .send()
            .await?
            .json::<NewrelicQueryResult>()
            .await?
        {
            NewrelicQueryResult::Ok(data) => return Ok(data),
            NewrelicQueryResult::Err(err) => return Err(err.into()),
        }
    }
}
