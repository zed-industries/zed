use proto::Plan;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use ui::IconName;

use crate::LanguageModelAvailability;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum CloudModel {
    Anthropic(anthropic::Model),
    OpenAi(open_ai::Model),
    Google(google_ai::Model),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, EnumIter)]
pub enum ZedModel {
    #[serde(rename = "Qwen/Qwen2-7B-Instruct")]
    Qwen2_7bInstruct,
}

impl Default for CloudModel {
    fn default() -> Self {
        Self::Anthropic(anthropic::Model::default())
    }
}

impl CloudModel {
    pub fn id(&self) -> &str {
        match self {
            Self::Anthropic(model) => model.id(),
            Self::OpenAi(model) => model.id(),
            Self::Google(model) => model.id(),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Anthropic(model) => model.display_name(),
            Self::OpenAi(model) => model.display_name(),
            Self::Google(model) => model.display_name(),
        }
    }

    pub fn icon(&self) -> Option<IconName> {
        match self {
            Self::Anthropic(_) => Some(IconName::AiAnthropicHosted),
            _ => None,
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Anthropic(model) => model.max_token_count(),
            Self::OpenAi(model) => model.max_token_count(),
            Self::Google(model) => model.max_token_count(),
        }
    }

    /// Returns the availability of this model.
    pub fn availability(&self) -> LanguageModelAvailability {
        match self {
            Self::Anthropic(model) => match model {
                anthropic::Model::Claude3_5Sonnet => {
                    LanguageModelAvailability::RequiresPlan(Plan::Free)
                }
                anthropic::Model::Claude3Opus
                | anthropic::Model::Claude3Sonnet
                | anthropic::Model::Claude3Haiku
                | anthropic::Model::Custom { .. } => {
                    LanguageModelAvailability::RequiresPlan(Plan::ZedPro)
                }
            },
            Self::OpenAi(model) => match model {
                open_ai::Model::ThreePointFiveTurbo
                | open_ai::Model::Four
                | open_ai::Model::FourTurbo
                | open_ai::Model::FourOmni
                | open_ai::Model::FourOmniMini
                | open_ai::Model::O1Mini
                | open_ai::Model::O1Preview
                | open_ai::Model::Custom { .. } => {
                    LanguageModelAvailability::RequiresPlan(Plan::ZedPro)
                }
            },
            Self::Google(model) => match model {
                google_ai::Model::Gemini15Pro
                | google_ai::Model::Gemini15Flash
                | google_ai::Model::Custom { .. } => {
                    LanguageModelAvailability::RequiresPlan(Plan::ZedPro)
                }
            },
        }
    }
}
