use serde::{Deserialize, Serialize};
use strum::{VariantArray, VariantNames};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, VariantArray, VariantNames,
)]
#[serde(rename_all = "lowercase")]
pub enum Region {
    #[default]
    International,
    China,
}

impl Region {
    pub fn display_name(&self) -> &str {
        match self {
            Self::International => "Z.AI (International)",
            Self::China => "Zhipu (China)",
        }
    }
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, VariantArray, VariantNames,
)]
#[serde(rename_all = "lowercase")]
pub enum BillingType {
    /// Pay-as-you-go standard billing.
    #[serde(rename = "standard")]
    Standard,
    /// Coding Plan subscription billing.
    #[default]
    #[serde(rename = "coding_plan")]
    CodingPlan,
}

impl BillingType {
    pub fn display_name(&self) -> &str {
        match self {
            Self::Standard => "Pay-as-you-go",
            Self::CodingPlan => "Coding Plan",
        }
    }
}

pub fn openai_api_url(region: Region, billing: BillingType) -> &'static str {
    match (region, billing) {
        (Region::International, BillingType::Standard) => "https://api.z.ai/api/paas/v4",
        (Region::International, BillingType::CodingPlan) => "https://api.z.ai/api/coding/paas/v4",
        (Region::China, BillingType::Standard) => "https://open.bigmodel.cn/api/paas/v4",
        (Region::China, BillingType::CodingPlan) => "https://open.bigmodel.cn/api/coding/paas/v4",
    }
}

pub fn anthropic_api_url(region: Region) -> &'static str {
    match region {
        Region::International => "https://api.z.ai/api/anthropic",
        Region::China => "https://open.bigmodel.cn/api/anthropic",
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum ZhipuModel {
    #[serde(rename = "glm-5.2")]
    #[default]
    Glm5_2,
    #[serde(rename = "glm-5.1")]
    Glm5_1,
    #[serde(rename = "glm-5-turbo")]
    Glm5Turbo,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
    },
}

impl ZhipuModel {
    pub fn default_fast() -> Self {
        Self::Glm5Turbo
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Glm5_2 => "glm-5.2",
            Self::Glm5_1 => "glm-5.1",
            Self::Glm5Turbo => "glm-5-turbo",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Glm5_2 => "GLM 5.2",
            Self::Glm5_1 => "GLM 5.1",
            Self::Glm5Turbo => "GLM 5 Turbo",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name).as_str(),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Glm5_2 => 1_000_000,
            Self::Glm5_1 | Self::Glm5Turbo => 200_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            Self::Glm5_2 | Self::Glm5_1 | Self::Glm5Turbo => Some(128_000),
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn supports_thinking(&self) -> bool {
        !matches!(self, Self::Custom { .. })
    }
}
