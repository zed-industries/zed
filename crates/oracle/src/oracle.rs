use anyhow::Result;
use open_ai::ReasoningEffort;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub use open_ai::stream_completion;

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "gpt-4.1")]
    FourPointOne,
    #[serde(rename = "openai-o3")]
    O3,
    #[serde(rename = "grok3")]
    Grok3,
    #[serde(rename = "grok4")]
    Grok4,
    #[serde(rename = "llama4")]
    Llama4,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        max_completion_tokens: Option<u64>,
        reasoning_effort: Option<ReasoningEffort>,
    },
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum ModelVendor {
    OpenAI,
    XAi,
    Meta,
}

impl Model {
    pub fn default_fast() -> Self {
        Self::FourPointOne
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "oca/gpt-4.1" => Ok(Self::FourPointOne),
            "oca/openai-o3" => Ok(Self::O3),
            "oca/grok3" => Ok(Self::Grok3),
            "oca/grok4" => Ok(Self::Grok4),
            "oca/llama4" => Ok(Self::Llama4),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::FourPointOne => "oca/gpt-4.1",
            Self::O3 => "oca/openai-o3",
            Self::Grok3 => "oca/grok3",
            Self::Grok4 => "oca/grok4",
            Self::Llama4 => "oca/llama4",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::FourPointOne => "OpenAI GPT-4.1",
            Self::O3 => "OpenAI O3",
            Self::Grok3 => "Grok 3",
            Self::Grok4 => "Grok 4",
            Self::Llama4 => "Llama 4",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn model_vendor(&self) -> ModelVendor {
        match self {
            Self::FourPointOne | Self::O3 => ModelVendor::OpenAI,
            Self::Grok3 | Self::Grok4 => ModelVendor::XAi,
            Self::Llama4 => ModelVendor::Meta,
            // Assume custom models are OpenAI compatible
            Self::Custom { .. } => ModelVendor::OpenAI,
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::FourPointOne => 1_047_576,
            Self::O3 => 200_000,
            Self::Grok3 => 131_072,
            Self::Grok4 => 256_000,
            Self::Llama4 => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            Self::FourPointOne => Some(32_768),
            Self::O3 => Some(100_000),
            Self::Grok3 => Some(8_192),
            Self::Grok4 => Some(64_000),
            Self::Llama4 => None,
        }
    }

    pub fn reasoning_effort(&self) -> Option<ReasoningEffort> {
        match self {
            Self::Custom {
                reasoning_effort, ..
            } => reasoning_effort.to_owned(),
            _ => None,
        }
    }

    /// Returns whether the given model supports the `parallel_tool_calls` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up, or the API will return an error.
    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::FourPointOne | Self::Grok3 | Self::Grok4 | Self::Llama4 => true,
            Self::O3 | Model::Custom { .. } => false,
        }
    }

    /// Returns whether the given model supports the `prompt_cache_key` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up.
    pub fn supports_prompt_cache_key(&self) -> bool {
        match self {
            Self::FourPointOne | Self::O3 => true,
            Self::Grok3 | Self::Grok4 | Self::Llama4 | Model::Custom { .. } => false,
        }
    }
}
