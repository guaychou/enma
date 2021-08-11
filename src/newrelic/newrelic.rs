use crate::{
    config::NewrelicConfig, newrelic::metric::Metric, newrelic::model::NewrelicQueryResult,
};

#[derive(Clone)]
pub struct Newrelic {
    api_key: String,
    account_id: i32,
    http_client: reqwest::Client,
}

impl Newrelic {
    pub fn new(newrelic_config: &NewrelicConfig) -> Self {
        let client = reqwest::Client::new();
        return Self {
            http_client: client,
            api_key: newrelic_config.get_api_key().to_string(),
            account_id: newrelic_config.get_account_id(),
        };
    }
    pub async fn go_query(
        &self,
        application_name: &str,
        start_time: &str,
        end_time: &str,
        metric: Metric,
    ) -> Result<NewrelicQueryResult, reqwest::Error> {
        let query = metric.get_query(application_name, start_time, end_time);
        let full_url = format!(
            "https://insights-api.newrelic.com/v1/accounts/{}/query?nrql={}",
            self.account_id, query
        );
        let resp = self
            .http_client
            .get(full_url)
            .header("X-Query-Key", self.api_key.as_str())
            .send()
            .await?;

        resp.json::<NewrelicQueryResult>().await
    }
}
