use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const VERCEL_API_URL: &str = "https://api.v0.dev/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "v0-1.5-md")]
    VZeroOnePointFiveMedium,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::VZeroOnePointFiveMedium
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "v0-1.5-md" => Ok(Self::VZeroOnePointFiveMedium),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::VZeroOnePointFiveMedium => "v0-1.5-md",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::VZeroOnePointFiveMedium => "v0-1.5-md",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::VZeroOnePointFiveMedium => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::VZeroOnePointFiveMedium => Some(32_000),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::VZeroOnePointFiveMedium => true,
            Model::Custom { .. } => false,
        }
    }

    pub fn supports_prompt_cache_key(&self) -> bool {
        false
    }
}
