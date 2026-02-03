use crate::PredictEditsRequestTrigger;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize)]
pub struct RawCompletionRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    pub stop: Vec<Cow<'static, str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictEditsV3Request {
    #[serde(flatten)]
    pub input: zeta_prompt::ZetaPromptInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub prompt_version: zeta_prompt::ZetaVersion,
    #[serde(default)]
    pub trigger: PredictEditsRequestTrigger,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PredictEditsV3Response {
    pub request_id: String,
    pub output: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<RawCompletionChoice>,
    pub usage: RawCompletionUsage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawCompletionChoice {
    pub text: String,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawCompletionUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
