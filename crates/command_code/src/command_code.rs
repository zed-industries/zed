use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const COMMAND_CODE_API_URL: &str = "https://api.commandcode.ai/provider/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "command-code")]
    CommandCode,
    #[serde(rename = "command-code-pro")]
    CommandCodePro,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the agent panel model dropdown menu.
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
        Self::CommandCode
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "command-code" => Ok(Self::CommandCode),
            "command-code-pro" => Ok(Self::CommandCodePro),
            _ => anyhow::bail!("invalid model id '{id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::CommandCode => "command-code",
            Self::CommandCodePro => "command-code-pro",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::CommandCode => "Command Code",
            Self::CommandCodePro => "Command Code Pro",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::CommandCode | Self::CommandCodePro => 200_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::CommandCode | Self::CommandCodePro => Some(64_000),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::CommandCode | Self::CommandCodePro => true,
            Self::Custom {
                parallel_tool_calls: Some(support),
                ..
            } => *support,
            Model::Custom { .. } => false,
        }
    }

    pub fn requires_json_schema_subset(&self) -> bool {
        true
    }

    pub fn supports_prompt_cache_key(&self) -> bool {
        false
    }

    pub fn supports_tool(&self) -> bool {
        match self {
            Self::CommandCode | Self::CommandCodePro => true,
            Self::Custom {
                supports_tools: Some(support),
                ..
            } => *support,
            Model::Custom { .. } => false,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::CommandCode | Self::CommandCodePro => true,
            Self::Custom {
                supports_images: Some(support),
                ..
            } => *support,
            Self::Custom { .. } => false,
        }
    }

    pub fn supports_reasoning_effort(&self) -> bool {
        false
    }
}
