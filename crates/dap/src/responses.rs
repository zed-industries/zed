use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    _type: String,
    success: bool,
    arguments: Option<ResponseArguments>,
    request_seq: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum ResponseArguments {
    LaunchResponse,
}
