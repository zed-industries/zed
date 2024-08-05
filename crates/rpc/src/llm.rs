use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PerformCompletionParams {
    pub provider_request: Box<serde_json::value::RawValue>,
}
