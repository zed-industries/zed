use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

pub const EXPIRED_LLM_TOKEN_HEADER_NAME: &str = "x-zed-expired-token";

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, EnumString, EnumIter, Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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
