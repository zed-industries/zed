use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct SpeechSettings {
    pub enabled: Option<bool>,
    pub model: Option<String>,
    pub ai_provider: Option<String>,
}

impl Default for SpeechSettings {
    fn default() -> Self {
        Self {
            enabled: None,
            model: None,
            ai_provider: None,
        }
    }
}
