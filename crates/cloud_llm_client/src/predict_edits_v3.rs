use crate::PredictEditsRequestTrigger;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Range;

#[derive(Debug, Deserialize, Serialize)]
pub struct RawCompletionRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    pub stop: Vec<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictEditsV3Request {
    #[serde(flatten)]
    pub input: zeta_prompt::ZetaPromptInput,
    #[serde(default)]
    pub trigger: PredictEditsRequestTrigger,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PredictEditsV3Response {
    pub request_id: String,
    pub output: String,
    /// The editable region byte range within `cursor_excerpt` that the
    /// server used for this request. When present, the client should use
    /// this range to extract the old text from its local excerpt for
    /// diffing, rather than relying on its own format-derived range.
    pub editable_range: Range<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
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
