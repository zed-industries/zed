use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const GROQ_API_URL: &str = "https://api.groq.com/openai/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "llama-3.1-8b-instant")]
    Llama31_8B,
    #[serde(rename = "llama-3.3-70b-versatile")]
    Llama33_70B,
    #[serde(rename = "meta-llama/llama-guard-4-12b")]
    LlamaGuard4_12B,
    #[serde(rename = "openai/gpt-oss-120b")]
    GptOss120B,
    #[serde(rename = "openai/gpt-oss-20b")]
    GptOss20B,
    #[serde(rename = "custom")]
    Custom {
        name: String,
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
        Self::Llama31_8B
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "llama-3.1-8b-instant" => Ok(Self::Llama31_8B),
            "llama-3.3-70b-versatile" => Ok(Self::Llama33_70B),
            "meta-llama/llama-guard-4-12b" => Ok(Self::LlamaGuard4_12B),
            "openai/gpt-oss-120b" => Ok(Self::GptOss120B),
            "openai/gpt-oss-20b" => Ok(Self::GptOss20B),
            _ => anyhow::bail!("invalid model id '{id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Llama31_8B => "llama-3.1-8b-instant",
            Self::Llama33_70B => "llama-3.3-70b-versatile",
            Self::LlamaGuard4_12B => "meta-llama/llama-guard-4-12b",
            Self::GptOss120B => "openai/gpt-oss-120b",
            Self::GptOss20B => "openai/gpt-oss-20b",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Llama31_8B => "Llama 3.1 8B Instant",
            Self::Llama33_70B => "Llama 3.3 70B Versatile",
            Self::LlamaGuard4_12B => "Llama Guard 4 12B",
            Self::GptOss120B => "GPT OSS 120B",
            Self::GptOss20B => "GPT OSS 20B",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Llama31_8B | Self::Llama33_70B | Self::LlamaGuard4_12B | Self::GptOss120B | Self::GptOss20B => 131_072,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Llama31_8B => Some(131_072),
            Self::Llama33_70B => Some(32_768),
            Self::LlamaGuard4_12B => Some(1_024),
            Self::GptOss120B => Some(65_536),
            Self::GptOss20B => Some(65_536),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn max_completion_tokens(&self) -> Option<u64> {
        self.max_output_tokens()
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::Llama31_8B | Self::Llama33_70B | Self::GptOss120B | Self::GptOss20B => true,
            Self::LlamaGuard4_12B => false,
            Self::Custom {
                parallel_tool_calls: Some(support),
                ..
            } => *support,
            Self::Custom { .. } => false,
        }
    }

    pub fn supports_prompt_cache_key(&self) -> bool {
        false
    }

    pub fn supports_tool(&self) -> bool {
        match self {
            Self::Llama31_8B | Self::Llama33_70B | Self::GptOss120B | Self::GptOss20B => true,
            Self::LlamaGuard4_12B => false,
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

