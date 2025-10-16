use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const XAI_API_URL: &str = "https://api.x.ai/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "grok-2-vision-latest")]
    Grok2Vision,
    #[default]
    #[serde(rename = "grok-3-latest")]
    Grok3,
    #[serde(rename = "grok-3-mini-latest")]
    Grok3Mini,
    #[serde(rename = "grok-3-fast-latest")]
    Grok3Fast,
    #[serde(rename = "grok-3-mini-fast-latest")]
    Grok3MiniFast,
    #[serde(rename = "grok-4", alias = "grok-4-latest")]
    Grok4,
    #[serde(
        rename = "grok-4-fast-reasoning",
        alias = "grok-4-fast-reasoning-latest"
    )]
    Grok4FastReasoning,
    #[serde(
        rename = "grok-4-fast-non-reasoning",
        alias = "grok-4-fast-non-reasoning-latest"
    )]
    Grok4FastNonReasoning,
    #[serde(rename = "grok-code-fast-1", alias = "grok-code-fast-1-0825")]
    GrokCodeFast1,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
        supports_images: Option<bool>,
        supports_tools: Option<bool>,
        parallel_tool_calls: Option<bool>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::Grok3Fast
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "grok-4" => Ok(Self::Grok4),
            "grok-4-fast-reasoning" => Ok(Self::Grok4FastReasoning),
            "grok-4-fast-non-reasoning" => Ok(Self::Grok4FastNonReasoning),
            "grok-2-vision" => Ok(Self::Grok2Vision),
            "grok-3" => Ok(Self::Grok3),
            "grok-3-mini" => Ok(Self::Grok3Mini),
            "grok-3-fast" => Ok(Self::Grok3Fast),
            "grok-3-mini-fast" => Ok(Self::Grok3MiniFast),
            "grok-code-fast-1" => Ok(Self::GrokCodeFast1),
            _ => anyhow::bail!("invalid model id '{id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Grok2Vision => "grok-2-vision",
            Self::Grok3 => "grok-3",
            Self::Grok3Mini => "grok-3-mini",
            Self::Grok3Fast => "grok-3-fast",
            Self::Grok3MiniFast => "grok-3-mini-fast",
            Self::Grok4 => "grok-4",
            Self::Grok4FastReasoning => "grok-4-fast-reasoning",
            Self::Grok4FastNonReasoning => "grok-4-fast-non-reasoning",
            Self::GrokCodeFast1 => "grok-code-fast-1",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Grok2Vision => "Grok 2 Vision",
            Self::Grok3 => "Grok 3",
            Self::Grok3Mini => "Grok 3 Mini",
            Self::Grok3Fast => "Grok 3 Fast",
            Self::Grok3MiniFast => "Grok 3 Mini Fast",
            Self::Grok4 => "Grok 4",
            Self::Grok4FastReasoning => "Grok 4 Fast",
            Self::Grok4FastNonReasoning => "Grok 4 Fast (Non-Reasoning)",
            Self::GrokCodeFast1 => "Grok Code Fast 1",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Grok3 | Self::Grok3Mini | Self::Grok3Fast | Self::Grok3MiniFast => 131_072,
            Self::Grok4 | Self::GrokCodeFast1 => 256_000,
            Self::Grok4FastReasoning | Self::Grok4FastNonReasoning => 128_000,
            Self::Grok2Vision => 8_192,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Grok3 | Self::Grok3Mini | Self::Grok3Fast | Self::Grok3MiniFast => Some(8_192),
            Self::Grok4
            | Self::Grok4FastReasoning
            | Self::Grok4FastNonReasoning
            | Self::GrokCodeFast1 => Some(64_000),
            Self::Grok2Vision => Some(4_096),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::Grok2Vision
            | Self::Grok3
            | Self::Grok3Mini
            | Self::Grok3Fast
            | Self::Grok3MiniFast
            | Self::Grok4
            | Self::Grok4FastReasoning
            | Self::Grok4FastNonReasoning => true,
            Self::Custom {
                parallel_tool_calls: Some(support),
                ..
            } => *support,
            Self::GrokCodeFast1 | Model::Custom { .. } => false,
        }
    }

    pub fn supports_prompt_cache_key(&self) -> bool {
        false
    }

    pub fn supports_tool(&self) -> bool {
        match self {
            Self::Grok2Vision
            | Self::Grok3
            | Self::Grok3Mini
            | Self::Grok3Fast
            | Self::Grok3MiniFast
            | Self::Grok4
            | Self::Grok4FastReasoning
            | Self::Grok4FastNonReasoning
            | Self::GrokCodeFast1 => true,
            Self::Custom {
                supports_tools: Some(support),
                ..
            } => *support,
            Model::Custom { .. } => false,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::Grok2Vision => true,
            Self::Custom {
                supports_images: Some(support),
                ..
            } => *support,
            _ => false,
        }
    }
}
