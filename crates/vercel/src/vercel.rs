use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const VERCEL_API_URL: &str = "https://api.v0.dev/v1";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "v-0")]
    #[default]
    VZero,

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
        Self::VZero
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "v-0" => Ok(Self::VZero),
            invalid_id => anyhow::bail!("invalid model id '{invalid_id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::VZero => "v-0",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::VZero => "Vercel v0",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::VZero => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
            Self::VZero => Some(32_768),
        }
    }

    /// Returns whether the given model supports the `parallel_tool_calls` parameter.
    ///
    /// If the model does not support the parameter, do not pass it up, or the API will return an error.
    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::VZero => true,
            Model::Custom { .. } => false,
        }
    }
}
