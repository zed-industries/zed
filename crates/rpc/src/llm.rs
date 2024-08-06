use serde::{Deserialize, Serialize};

pub const EXPIRED_LLM_TOKEN_HEADER_NAME: &str = "x-zed-expired-token";

#[derive(Serialize, Deserialize)]
pub struct PerformCompletionParams {
    pub provider_request: Box<serde_json::value::RawValue>,
}
