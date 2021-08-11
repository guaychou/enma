use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestData {
    pub application_name: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub data: RequestData,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct ResponseData {
    pub result: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub api_version: String,
    pub data: ResponseData,
}