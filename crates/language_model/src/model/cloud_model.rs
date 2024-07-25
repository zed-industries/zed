pub use anthropic::Model as AnthropicModel;
use anyhow::{anyhow, Result};
pub use ollama::Model as OllamaModel;
pub use open_ai::Model as OpenAiModel;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, EnumIter)]
pub enum CloudModel {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3Point5Turbo,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4o")]
    #[default]
    Gpt4Omni,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4OmniMini,
    #[serde(rename = "claude-3-5-sonnet")]
    Claude3_5Sonnet,
    #[serde(rename = "claude-3-opus")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku")]
    Claude3Haiku,
    #[serde(rename = "gemini-1.5-pro")]
    Gemini15Pro,
    #[serde(rename = "gemini-1.5-flash")]
    Gemini15Flash,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        max_tokens: Option<usize>,
    },
}

impl CloudModel {
    pub fn from_id(value: &str) -> Result<Self> {
        match value {
            "gpt-3.5-turbo" => Ok(Self::Gpt3Point5Turbo),
            "gpt-4" => Ok(Self::Gpt4),
            "gpt-4-turbo-preview" => Ok(Self::Gpt4Turbo),
            "gpt-4o" => Ok(Self::Gpt4Omni),
            "gpt-4o-mini" => Ok(Self::Gpt4OmniMini),
            "claude-3-5-sonnet" => Ok(Self::Claude3_5Sonnet),
            "claude-3-opus" => Ok(Self::Claude3Opus),
            "claude-3-sonnet" => Ok(Self::Claude3Sonnet),
            "claude-3-haiku" => Ok(Self::Claude3Haiku),
            "gemini-1.5-pro" => Ok(Self::Gemini15Pro),
            "gemini-1.5-flash" => Ok(Self::Gemini15Flash),
            _ => Err(anyhow!("invalid model id")),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "gpt-3.5-turbo",
            Self::Gpt4 => "gpt-4",
            Self::Gpt4Turbo => "gpt-4-turbo-preview",
            Self::Gpt4Omni => "gpt-4o",
            Self::Gpt4OmniMini => "gpt-4o-mini",
            Self::Claude3_5Sonnet => "claude-3-5-sonnet",
            Self::Claude3Opus => "claude-3-opus",
            Self::Claude3Sonnet => "claude-3-sonnet",
            Self::Claude3Haiku => "claude-3-haiku",
            Self::Gemini15Pro => "gemini-1.5-pro",
            Self::Gemini15Flash => "gemini-1.5-flash",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "GPT 3.5 Turbo",
            Self::Gpt4 => "GPT 4",
            Self::Gpt4Turbo => "GPT 4 Turbo",
            Self::Gpt4Omni => "GPT 4 Omni",
            Self::Gpt4OmniMini => "GPT 4 Omni Mini",
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Gemini15Pro => "Gemini 1.5 Pro",
            Self::Gemini15Flash => "Gemini 1.5 Flash",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Gpt3Point5Turbo => 2048,
            Self::Gpt4 => 4096,
            Self::Gpt4Turbo | Self::Gpt4Omni => 128000,
            Self::Gpt4OmniMini => 128000,
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3Haiku => 200000,
            Self::Gemini15Pro => 128000,
            Self::Gemini15Flash => 32000,
            Self::Custom { max_tokens, .. } => max_tokens.unwrap_or(200_000),
        }
    }
}
