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

struct ResponseData {
    result: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    api_version: String,
    data: ResponseData,
}

impl Response {
    pub fn set_response(res: f32) -> Self {
        let data = ResponseData { result: res };
        return Self {
            api_version: String::from("v1"),
            data: data,
        };
    }

    pub fn default() -> Self {
        let data = ResponseData { result: 0.0 };
        return Self {
            api_version: String::from("v1"),
            data: data,
        };
    }
}
