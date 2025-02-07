use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

pub const EXPIRED_LLM_TOKEN_HEADER_NAME: &str = "x-zed-expired-token";

pub const MAX_LLM_MONTHLY_SPEND_REACHED_HEADER_NAME: &str = "x-zed-llm-max-monthly-spend-reached";

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, EnumString, EnumIter, Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LanguageModelProvider {
    Anthropic,
    OpenAi,
    Google,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageModel {
    pub provider: LanguageModelProvider,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub models: Vec<LanguageModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformCompletionParams {
    pub provider: LanguageModelProvider,
    pub model: String,
    pub provider_request: Box<serde_json::value::RawValue>,
}
