use serde::{Deserialize, Serialize};

pub enum CheckStatus {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStatusParams {
    pub local_checks_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStatusResult {
    pub status: String,
    pub user: Option<String>,
}

impl lsp::request::Request for CheckStatus {
    type Params = CheckStatusParams;
    type Result = CheckStatusResult;
    const METHOD: &'static str = "checkStatus";
}
