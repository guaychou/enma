use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewrelicResultModel {
    pub result: Option<f32>,
    pub average: Option<f32>,
    #[serde(rename(deserialize = "uniqueCount"))]
    pub unique_count: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct NewrelicResponseModel {
    results: Vec<NewrelicResultModel>,
}

impl NewrelicResponseModel {
    pub fn get_average(&self) -> Option<f32> {
        self.results[0].average
    }

    pub fn get_unique_count(&self) -> Option<f32> {
        self.results[0].unique_count
    }

    pub fn get_result(&self) -> Option<f32> {
        self.results[0].result
    }
}

#[derive(Deserialize, Debug)]
pub struct NewRelicErrorResponseModel {
    #[serde(rename(deserialize = "error"))]
    error_msg: String,
}

impl NewRelicErrorResponseModel {
    pub fn get_error_msg(&self) -> &str {
        return self.error_msg.as_str();
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum NewrelicQueryResult {
    Ok(NewrelicResponseModel),
    Err(NewRelicErrorResponseModel),
}
