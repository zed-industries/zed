use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum BedrockModelMode {
    #[default]
    Default,
    Thinking {
        budget_tokens: Option<u64>,
    },
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    // Anthropic models (already included)
    #[default]
    #[serde(rename = "claude-3-5-sonnet-v2", alias = "claude-3-5-sonnet-latest")]
    Claude3_5SonnetV2,
    #[serde(rename = "claude-3-7-sonnet", alias = "claude-3-7-sonnet-latest")]
    Claude3_7Sonnet,
    #[serde(
        rename = "claude-3-7-sonnet-thinking",
        alias = "claude-3-7-sonnet-thinking-latest"
    )]
    Claude3_7SonnetThinking,
    #[serde(rename = "claude-3-opus", alias = "claude-3-opus-latest")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet", alias = "claude-3-sonnet-latest")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-5-haiku", alias = "claude-3-5-haiku-latest")]
    Claude3_5Haiku,
    Claude3_5Sonnet,
    Claude3Haiku,
    // Amazon Nova Models
    AmazonNovaLite,
    AmazonNovaMicro,
    AmazonNovaPro,
    AmazonNovaPremier,
    // AI21 models
    AI21J2GrandeInstruct,
    AI21J2JumboInstruct,
    AI21J2Mid,
    AI21J2MidV1,
    AI21J2Ultra,
    AI21J2UltraV1_8k,
    AI21J2UltraV1,
    AI21JambaInstructV1,
    AI21Jamba15LargeV1,
    AI21Jamba15MiniV1,
    // Cohere models
    CohereCommandTextV14_4k,
    CohereCommandRV1,
    CohereCommandRPlusV1,
    CohereCommandLightTextV14_4k,
    // DeepSeek
    DeepSeekR1,
    // Meta models
    MetaLlama3_8BInstruct,
    MetaLlama3_70BInstruct,
    MetaLlama31_8BInstruct,
    MetaLlama31_70BInstruct,
    MetaLlama31_405BInstruct,
    MetaLlama32_1BInstruct,
    MetaLlama32_3BInstruct,
    MetaLlama32_11BMultiModal,
    MetaLlama32_90BMultiModal,
    MetaLlama33_70BInstruct,
    #[allow(non_camel_case_types)]
    MetaLlama4Scout_17BInstruct,
    #[allow(non_camel_case_types)]
    MetaLlama4Maverick_17BInstruct,
    // Mistral models
    MistralMistral7BInstructV0,
    MistralMixtral8x7BInstructV0,
    MistralMistralLarge2402V1,
    MistralMistralSmall2402V1,
    MistralPixtralLarge2502V1,
    // Writer models
    PalmyraWriterX5,
    PalmyraWriterX4,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        max_tokens: usize,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_output_tokens: Option<u32>,
        default_temperature: Option<f32>,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::Claude3_5Haiku
    }

    pub fn from_id(id: &str) -> anyhow::Result<Self> {
        if id.starts_with("claude-3-5-sonnet-v2") {
            Ok(Self::Claude3_5SonnetV2)
        } else if id.starts_with("claude-3-opus") {
            Ok(Self::Claude3Opus)
        } else if id.starts_with("claude-3-sonnet") {
            Ok(Self::Claude3Sonnet)
        } else if id.starts_with("claude-3-5-haiku") {
            Ok(Self::Claude3_5Haiku)
        } else if id.starts_with("claude-3-7-sonnet") {
            Ok(Self::Claude3_7Sonnet)
        } else if id.starts_with("claude-3-7-sonnet-thinking") {
            Ok(Self::Claude3_7SonnetThinking)
        } else {
            Err(anyhow!("invalid model id"))
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Model::Claude3_5SonnetV2 => "anthropic.claude-3-5-sonnet-20241022-v2:0",
            Model::Claude3_5Sonnet => "anthropic.claude-3-5-sonnet-20240620-v1:0",
            Model::Claude3Opus => "anthropic.claude-3-opus-20240229-v1:0",
            Model::Claude3Sonnet => "anthropic.claude-3-sonnet-20240229-v1:0",
            Model::Claude3Haiku => "anthropic.claude-3-haiku-20240307-v1:0",
            Model::Claude3_5Haiku => "anthropic.claude-3-5-haiku-20241022-v1:0",
            Model::Claude3_7Sonnet | Model::Claude3_7SonnetThinking => {
                "anthropic.claude-3-7-sonnet-20250219-v1:0"
            }
            Model::AmazonNovaLite => "amazon.nova-lite-v1:0",
            Model::AmazonNovaMicro => "amazon.nova-micro-v1:0",
            Model::AmazonNovaPro => "amazon.nova-pro-v1:0",
            Model::AmazonNovaPremier => "amazon.nova-premier-v1:0",
            Model::DeepSeekR1 => "deepseek.r1-v1:0",
            Model::AI21J2GrandeInstruct => "ai21.j2-grande-instruct",
            Model::AI21J2JumboInstruct => "ai21.j2-jumbo-instruct",
            Model::AI21J2Mid => "ai21.j2-mid",
            Model::AI21J2MidV1 => "ai21.j2-mid-v1",
            Model::AI21J2Ultra => "ai21.j2-ultra",
            Model::AI21J2UltraV1_8k => "ai21.j2-ultra-v1:0:8k",
            Model::AI21J2UltraV1 => "ai21.j2-ultra-v1",
            Model::AI21JambaInstructV1 => "ai21.jamba-instruct-v1:0",
            Model::AI21Jamba15LargeV1 => "ai21.jamba-1-5-large-v1:0",
            Model::AI21Jamba15MiniV1 => "ai21.jamba-1-5-mini-v1:0",
            Model::CohereCommandTextV14_4k => "cohere.command-text-v14:7:4k",
            Model::CohereCommandRV1 => "cohere.command-r-v1:0",
            Model::CohereCommandRPlusV1 => "cohere.command-r-plus-v1:0",
            Model::CohereCommandLightTextV14_4k => "cohere.command-light-text-v14:7:4k",
            Model::MetaLlama3_8BInstruct => "meta.llama3-8b-instruct-v1:0",
            Model::MetaLlama3_70BInstruct => "meta.llama3-70b-instruct-v1:0",
            Model::MetaLlama31_8BInstruct => "meta.llama3-1-8b-instruct-v1:0",
            Model::MetaLlama31_70BInstruct => "meta.llama3-1-70b-instruct-v1:0",
            Model::MetaLlama31_405BInstruct => "meta.llama3-1-405b-instruct-v1:0",
            Model::MetaLlama32_11BMultiModal => "meta.llama3-2-11b-instruct-v1:0",
            Model::MetaLlama32_90BMultiModal => "meta.llama3-2-90b-instruct-v1:0",
            Model::MetaLlama32_1BInstruct => "meta.llama3-2-1b-instruct-v1:0",
            Model::MetaLlama32_3BInstruct => "meta.llama3-2-3b-instruct-v1:0",
            Model::MetaLlama33_70BInstruct => "meta.llama3-3-70b-instruct-v1:0",
            Model::MetaLlama4Scout_17BInstruct => "meta.llama4-scout-17b-instruct-v1:0",
            Model::MetaLlama4Maverick_17BInstruct => "meta.llama4-maverick-17b-instruct-v1:0",
            Model::MistralMistral7BInstructV0 => "mistral.mistral-7b-instruct-v0:2",
            Model::MistralMixtral8x7BInstructV0 => "mistral.mixtral-8x7b-instruct-v0:1",
            Model::MistralMistralLarge2402V1 => "mistral.mistral-large-2402-v1:0",
            Model::MistralMistralSmall2402V1 => "mistral.mistral-small-2402-v1:0",
            Model::MistralPixtralLarge2502V1 => "mistral.pixtral-large-2502-v1:0",
            Model::PalmyraWriterX4 => "writer.palmyra-x4-v1:0",
            Model::PalmyraWriterX5 => "writer.palmyra-x5-v1:0",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Claude3_5SonnetV2 => "Claude 3.5 Sonnet v2",
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Claude3_5Haiku => "Claude 3.5 Haiku",
            Self::Claude3_7Sonnet => "Claude 3.7 Sonnet",
            Self::Claude3_7SonnetThinking => "Claude 3.7 Sonnet Thinking",
            Self::AmazonNovaLite => "Amazon Nova Lite",
            Self::AmazonNovaMicro => "Amazon Nova Micro",
            Self::AmazonNovaPro => "Amazon Nova Pro",
            Self::AmazonNovaPremier => "Amazon Nova Premier",
            Self::DeepSeekR1 => "DeepSeek R1",
            Self::AI21J2GrandeInstruct => "AI21 Jurassic2 Grande Instruct",
            Self::AI21J2JumboInstruct => "AI21 Jurassic2 Jumbo Instruct",
            Self::AI21J2Mid => "AI21 Jurassic2 Mid",
            Self::AI21J2MidV1 => "AI21 Jurassic2 Mid V1",
            Self::AI21J2Ultra => "AI21 Jurassic2 Ultra",
            Self::AI21J2UltraV1_8k => "AI21 Jurassic2 Ultra V1 8K",
            Self::AI21J2UltraV1 => "AI21 Jurassic2 Ultra V1",
            Self::AI21JambaInstructV1 => "AI21 Jamba Instruct",
            Self::AI21Jamba15LargeV1 => "AI21 Jamba 1.5 Large",
            Self::AI21Jamba15MiniV1 => "AI21 Jamba 1.5 Mini",
            Self::CohereCommandTextV14_4k => "Cohere Command Text V14 4K",
            Self::CohereCommandRV1 => "Cohere Command R V1",
            Self::CohereCommandRPlusV1 => "Cohere Command R Plus V1",
            Self::CohereCommandLightTextV14_4k => "Cohere Command Light Text V14 4K",
            Self::MetaLlama3_8BInstruct => "Meta Llama 3 8B Instruct",
            Self::MetaLlama3_70BInstruct => "Meta Llama 3 70B Instruct",
            Self::MetaLlama31_8BInstruct => "Meta Llama 3.1 8B Instruct",
            Self::MetaLlama31_70BInstruct => "Meta Llama 3.1 70B Instruct",
            Self::MetaLlama31_405BInstruct => "Meta Llama 3.1 405B Instruct",
            Self::MetaLlama32_11BMultiModal => "Meta Llama 3.2 11B Vision Instruct",
            Self::MetaLlama32_90BMultiModal => "Meta Llama 3.2 90B Vision Instruct",
            Self::MetaLlama32_1BInstruct => "Meta Llama 3.2 1B Instruct",
            Self::MetaLlama32_3BInstruct => "Meta Llama 3.2 3B Instruct",
            Self::MetaLlama33_70BInstruct => "Meta Llama 3.3 70B Instruct",
            Self::MetaLlama4Scout_17BInstruct => "Meta Llama 4 Scout 17B Instruct",
            Self::MetaLlama4Maverick_17BInstruct => "Meta Llama 4 Maverick 17B Instruct",
            Self::MistralMistral7BInstructV0 => "Mistral 7B Instruct V0",
            Self::MistralMixtral8x7BInstructV0 => "Mistral Mixtral 8x7B Instruct V0",
            Self::MistralMistralLarge2402V1 => "Mistral Large 2402 V1",
            Self::MistralMistralSmall2402V1 => "Mistral Small 2402 V1",
            Self::MistralPixtralLarge2502V1 => "Pixtral Large 25.02 V1",
            Self::PalmyraWriterX5 => "Writer Palmyra X5",
            Self::PalmyraWriterX4 => "Writer Palmyra X4",
            Self::Custom {
                display_name, name, ..
            } => display_name.as_deref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Claude3_5SonnetV2
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3_5Haiku
            | Self::Claude3_7Sonnet => 200_000,
            Self::AmazonNovaPremier => 1_000_000,
            Self::PalmyraWriterX5 => 1_000_000,
            Self::PalmyraWriterX4 => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
            _ => 128_000,
        }
    }

    pub fn max_output_tokens(&self) -> u32 {
        match self {
            Self::Claude3Opus | Self::Claude3Sonnet | Self::Claude3_5Haiku => 4_096,
            Self::Claude3_7Sonnet | Self::Claude3_7SonnetThinking => 128_000,
            Self::Claude3_5SonnetV2 | Self::PalmyraWriterX4 | Self::PalmyraWriterX5 => 8_192,
            Self::Custom {
                max_output_tokens, ..
            } => max_output_tokens.unwrap_or(4_096),
            _ => 4_096,
        }
    }

    pub fn default_temperature(&self) -> f32 {
        match self {
            Self::Claude3_5SonnetV2
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3_5Haiku
            | Self::Claude3_7Sonnet => 1.0,
            Self::Custom {
                default_temperature,
                ..
            } => default_temperature.unwrap_or(1.0),
            _ => 1.0,
        }
    }

    pub fn supports_tool_use(&self) -> bool {
        match self {
            // Anthropic Claude 3 models (all support tool use)
            Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3_5Sonnet
            | Self::Claude3_5SonnetV2
            | Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking
            | Self::Claude3_5Haiku => true,

            // Amazon Nova models (all support tool use)
            Self::AmazonNovaPremier
            | Self::AmazonNovaPro
            | Self::AmazonNovaLite
            | Self::AmazonNovaMicro => true,

            // AI21 Jamba 1.5 models support tool use
            Self::AI21Jamba15LargeV1 | Self::AI21Jamba15MiniV1 => true,

            // Cohere Command R models support tool use
            Self::CohereCommandRV1 | Self::CohereCommandRPlusV1 => true,

            // All other models don't support tool use
            // Including Meta Llama 3.2, AI21 Jurassic, and others
            _ => false,
        }
    }

    pub fn mode(&self) -> BedrockModelMode {
        match self {
            Model::Claude3_7SonnetThinking => BedrockModelMode::Thinking {
                budget_tokens: Some(4096),
            },
            _ => BedrockModelMode::Default,
        }
    }

    pub fn cross_region_inference_id(&self, region: &str) -> Result<String, anyhow::Error> {
        let region_group = if region.starts_with("us-gov-") {
            "us-gov"
        } else if region.starts_with("us-") {
            "us"
        } else if region.starts_with("eu-") {
            "eu"
        } else if region.starts_with("ap-") || region == "me-central-1" || region == "me-south-1" {
            "apac"
        } else if region.starts_with("ca-") || region.starts_with("sa-") {
            // Canada and South America regions - default to US profiles
            "us"
        } else {
            // Unknown region
            return Err(anyhow!("Unsupported Region"));
        };

        let model_id = self.id();

        match (self, region_group) {
            // Custom models can't have CRI IDs
            (Model::Custom { .. }, _) => Ok(self.id().into()),

            // Models with US Gov only
            (Model::Claude3_5Sonnet | Model::Claude3Haiku, "us-gov") => {
                Ok(format!("{}.{}", region_group, model_id))
            }

            // Available everywhere
            (Model::AmazonNovaLite | Model::AmazonNovaMicro | Model::AmazonNovaPro, _) => {
                Ok(format!("{}.{}", region_group, model_id))
            }

            // Models in US
            (
                Model::AmazonNovaPremier
                | Model::Claude3_5Haiku
                | Model::Claude3_5Sonnet
                | Model::Claude3_5SonnetV2
                | Model::Claude3_7Sonnet
                | Model::Claude3_7SonnetThinking
                | Model::Claude3Haiku
                | Model::Claude3Opus
                | Model::Claude3Sonnet
                | Model::DeepSeekR1
                | Model::MetaLlama31_405BInstruct
                | Model::MetaLlama31_70BInstruct
                | Model::MetaLlama31_8BInstruct
                | Model::MetaLlama32_11BMultiModal
                | Model::MetaLlama32_1BInstruct
                | Model::MetaLlama32_3BInstruct
                | Model::MetaLlama32_90BMultiModal
                | Model::MetaLlama33_70BInstruct
                | Model::MetaLlama4Maverick_17BInstruct
                | Model::MetaLlama4Scout_17BInstruct
                | Model::MistralPixtralLarge2502V1
                | Model::PalmyraWriterX4
                | Model::PalmyraWriterX5,
                "us",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // Models available in EU
            (
                Model::Claude3_5Sonnet
                | Model::Claude3_7Sonnet
                | Model::Claude3_7SonnetThinking
                | Model::Claude3Haiku
                | Model::Claude3Sonnet
                | Model::MetaLlama32_1BInstruct
                | Model::MetaLlama32_3BInstruct
                | Model::MistralPixtralLarge2502V1,
                "eu",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // Models available in APAC
            (
                Model::Claude3_5Sonnet
                | Model::Claude3_5SonnetV2
                | Model::Claude3Haiku
                | Model::Claude3Sonnet,
                "apac",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // Any other combination is not supported
            _ => Ok(self.id().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_us_region_inference_ids() -> anyhow::Result<()> {
        // Test US regions
        assert_eq!(
            Model::Claude3_5SonnetV2.cross_region_inference_id("us-east-1")?,
            "us.anthropic.claude-3-5-sonnet-20241022-v2:0"
        );
        assert_eq!(
            Model::Claude3_5SonnetV2.cross_region_inference_id("us-west-2")?,
            "us.anthropic.claude-3-5-sonnet-20241022-v2:0"
        );
        assert_eq!(
            Model::AmazonNovaPro.cross_region_inference_id("us-east-2")?,
            "us.amazon.nova-pro-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_eu_region_inference_ids() -> anyhow::Result<()> {
        // Test European regions
        assert_eq!(
            Model::Claude3Sonnet.cross_region_inference_id("eu-west-1")?,
            "eu.anthropic.claude-3-sonnet-20240229-v1:0"
        );
        assert_eq!(
            Model::AmazonNovaMicro.cross_region_inference_id("eu-north-1")?,
            "eu.amazon.nova-micro-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_apac_region_inference_ids() -> anyhow::Result<()> {
        // Test Asia-Pacific regions
        assert_eq!(
            Model::Claude3_5SonnetV2.cross_region_inference_id("ap-northeast-1")?,
            "apac.anthropic.claude-3-5-sonnet-20241022-v2:0"
        );
        assert_eq!(
            Model::Claude3_5SonnetV2.cross_region_inference_id("ap-southeast-2")?,
            "apac.anthropic.claude-3-5-sonnet-20241022-v2:0"
        );
        assert_eq!(
            Model::AmazonNovaLite.cross_region_inference_id("ap-south-1")?,
            "apac.amazon.nova-lite-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_gov_region_inference_ids() -> anyhow::Result<()> {
        // Test Government regions
        assert_eq!(
            Model::Claude3_5Sonnet.cross_region_inference_id("us-gov-east-1")?,
            "us-gov.anthropic.claude-3-5-sonnet-20240620-v1:0"
        );
        assert_eq!(
            Model::Claude3Haiku.cross_region_inference_id("us-gov-west-1")?,
            "us-gov.anthropic.claude-3-haiku-20240307-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_meta_models_inference_ids() -> anyhow::Result<()> {
        // Test Meta models
        assert_eq!(
            Model::MetaLlama3_70BInstruct.cross_region_inference_id("us-east-1")?,
            "us.meta.llama3-70b-instruct-v1:0"
        );
        assert_eq!(
            Model::MetaLlama32_1BInstruct.cross_region_inference_id("eu-west-1")?,
            "eu.meta.llama3-2-1b-instruct-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_mistral_models_inference_ids() -> anyhow::Result<()> {
        // Mistral models don't follow the regional prefix pattern,
        // so they should return their original IDs
        assert_eq!(
            Model::MistralMistralLarge2402V1.cross_region_inference_id("us-east-1")?,
            "mistral.mistral-large-2402-v1:0"
        );
        assert_eq!(
            Model::MistralMixtral8x7BInstructV0.cross_region_inference_id("eu-west-1")?,
            "mistral.mixtral-8x7b-instruct-v0:1"
        );
        Ok(())
    }

    #[test]
    fn test_ai21_models_inference_ids() -> anyhow::Result<()> {
        // AI21 models don't follow the regional prefix pattern,
        // so they should return their original IDs
        assert_eq!(
            Model::AI21J2UltraV1.cross_region_inference_id("us-east-1")?,
            "ai21.j2-ultra-v1"
        );
        assert_eq!(
            Model::AI21JambaInstructV1.cross_region_inference_id("eu-west-1")?,
            "ai21.jamba-instruct-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_cohere_models_inference_ids() -> anyhow::Result<()> {
        // Cohere models don't follow the regional prefix pattern,
        // so they should return their original IDs
        assert_eq!(
            Model::CohereCommandRV1.cross_region_inference_id("us-east-1")?,
            "cohere.command-r-v1:0"
        );
        assert_eq!(
            Model::CohereCommandTextV14_4k.cross_region_inference_id("ap-southeast-1")?,
            "cohere.command-text-v14:7:4k"
        );
        Ok(())
    }

    #[test]
    fn test_custom_model_inference_ids() -> anyhow::Result<()> {
        // Test custom models
        let custom_model = Model::Custom {
            name: "custom.my-model-v1:0".to_string(),
            max_tokens: 100000,
            display_name: Some("My Custom Model".to_string()),
            max_output_tokens: Some(8192),
            default_temperature: Some(0.7),
        };

        // Custom model should return its name unchanged
        assert_eq!(
            custom_model.cross_region_inference_id("us-east-1")?,
            "custom.my-model-v1:0"
        );

        Ok(())
    }
}
