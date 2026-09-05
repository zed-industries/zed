use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictEditsV4Request {
    #[serde(flatten)]
    pub input: zeta_prompt::Zeta3PromptInput,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PredictEditsV4Response {
    pub request_id: String,
    pub patch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
}
