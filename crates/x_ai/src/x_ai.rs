use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const XAI_API_URL: &str = "https://api.x.ai/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "grok-4.3", alias = "grok-4.3-latest")]
    Grok43,
    #[serde(rename = "grok-4.20-0309-reasoning")]
    Grok420Reasoning,
    #[serde(rename = "grok-4.20-0309-non-reasoning")]
    Grok420NonReasoning,
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
        Self::Grok43
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "grok-4.3" => Ok(Self::Grok43),
            "grok-4.20-0309-reasoning" => Ok(Self::Grok420Reasoning),
            "grok-4.20-0309-non-reasoning" => Ok(Self::Grok420NonReasoning),
            _ => anyhow::bail!("invalid model id '{id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Grok43 => "grok-4.3",
            Self::Grok420Reasoning => "grok-4.20-0309-reasoning",
            Self::Grok420NonReasoning => "grok-4.20-0309-non-reasoning",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Grok43 => "Grok 4.3",
            Self::Grok420Reasoning => "Grok 4.20 Reasoning",
            Self::Grok420NonReasoning => "Grok 4.20 (Non-Reasoning)",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Grok43 => 1_000_000,
            Self::Grok420Reasoning | Self::Grok420NonReasoning => 2_000_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Grok43 | Self::Grok420Reasoning | Self::Grok420NonReasoning => Some(64_000),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::Grok43 | Self::Grok420Reasoning | Self::Grok420NonReasoning => true,
            Self::Custom {
                parallel_tool_calls: Some(support),
                ..
            } => *support,
            Model::Custom { .. } => false,
        }
    }

    pub fn requires_json_schema_subset(&self) -> bool {
        match self {
            Self::Grok43 | Self::Grok420Reasoning | Self::Grok420NonReasoning => true,
            Self::Custom { .. } => false,
        }
    }

    pub fn supports_prompt_cache_key(&self) -> bool {
        false
    }

    pub fn supports_tool(&self) -> bool {
        match self {
            Self::Grok43 | Self::Grok420Reasoning | Self::Grok420NonReasoning => true,
            Self::Custom {
                supports_tools: Some(support),
                ..
            } => *support,
            Model::Custom { .. } => false,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::Grok43 | Self::Grok420Reasoning | Self::Grok420NonReasoning => true,
            Self::Custom {
                supports_images: Some(support),
                ..
            } => *support,
            Self::Custom { .. } => false,
        }
    }
}
