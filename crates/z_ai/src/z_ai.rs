use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const ZAI_API_URL: &str = "https://api.z.ai/api/paas/v4";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "glm-4.5")]
    Glm4_5,
    #[serde(rename = "glm-4.5-air")]
    Glm4_5Air,
}

impl Model {
    pub fn default_fast() -> Self {
        Self::Glm4_5Air
    }

    pub fn from_id(id: &str) -> Result<Self> {
        match id {
            "glm-4.5" => Ok(Self::Glm4_5),
            "glm-4.5-air" => Ok(Self::Glm4_5Air),
            _ => anyhow::bail!("invalid model id '{id}'"),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Glm4_5 => "glm-4.5",
            Self::Glm4_5Air => "glm-4.5-air",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Glm4_5 => "GLM 4.5",
            Self::Glm4_5Air => "GLM 4.5 Air",
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Glm4_5 => 128_000,
            Self::Glm4_5Air => 128_000,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Glm4_5 => Some(8_192),
            Self::Glm4_5Air => Some(8_192),
        }
    }

    pub fn supports_tool(&self) -> bool {
        match self {
            Self::Glm4_5 | Self::Glm4_5Air => true,
        }
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        match self {
            Self::Glm4_5 | Self::Glm4_5Air => false,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::Glm4_5 | Self::Glm4_5Air => true,
        }
    }
}
