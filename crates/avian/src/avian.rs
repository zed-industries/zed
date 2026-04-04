use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const AVIAN_API_URL: &str = "https://api.avian.io/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "deepseek-v3-0324")]
    DeepSeekV3,
    #[serde(rename = "kimi-k2.5")]
    KimiK2_5,
    #[serde(rename = "glm-5")]
    Glm5,
    #[serde(rename = "minimax-m2.5")]
    MiniMaxM2_5,
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
        Self::DeepSeekV3
    }

    pub fn id(&self) -> &str {
        match self {
            Self::DeepSeekV3 => "deepseek-v3-0324",
            Self::KimiK2_5 => "kimi-k2.5",
            Self::Glm5 => "glm-5",
            Self::MiniMaxM2_5 => "minimax-m2.5",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::DeepSeekV3 => "DeepSeek V3.2",
            Self::KimiK2_5 => "Kimi K2.5",
            Self::Glm5 => "GLM-5",
            Self::MiniMaxM2_5 => "MiniMax M2.5",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::DeepSeekV3 => 164_000,
            Self::KimiK2_5 => 128_000,
            Self::Glm5 => 128_000,
            Self::MiniMaxM2_5 => 1_000_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::DeepSeekV3 => Some(8_192),
            Self::KimiK2_5 => Some(8_192),
            Self::Glm5 => Some(8_192),
            Self::MiniMaxM2_5 => Some(8_192),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::DeepSeekV3 | Self::KimiK2_5 | Self::Glm5 | Self::MiniMaxM2_5 => false,
            Self::Custom {
                parallel_tool_calls: Some(support),
                ..
            } => *support,
            Self::Custom { .. } => false,
        }
    }

    pub fn requires_json_schema_subset(&self) -> bool {
        false
    }

    pub fn supports_prompt_cache_key(&self) -> bool {
        false
    }

    pub fn supports_tool(&self) -> bool {
        match self {
            Self::DeepSeekV3 | Self::KimiK2_5 | Self::Glm5 | Self::MiniMaxM2_5 => true,
            Self::Custom {
                supports_tools: Some(support),
                ..
            } => *support,
            Self::Custom { .. } => false,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::Custom {
                supports_images: Some(support),
                ..
            } => *support,
            _ => false,
        }
    }
}
