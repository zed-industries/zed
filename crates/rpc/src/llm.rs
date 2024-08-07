use serde::{Deserialize, Serialize};

pub const EXPIRED_LLM_TOKEN_HEADER_NAME: &str = "x-zed-expired-token";

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageModelProvider {
    Anthropic,
    OpenAi,
    Google,
    Zed,
}

#[derive(Serialize, Deserialize)]
pub struct PerformCompletionParams {
    pub provider: LanguageModelProvider,
    pub model: String,
    pub provider_request: Box<serde_json::value::RawValue>,
}
