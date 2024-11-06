use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "claude-3-5-sonnet", alias = "claude-3-5-sonnet-latest")]
    Claude3_5Sonnet,
    #[serde(rename = "claude-3-opus", alias = "claude-3-opus-latest")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet", alias = "claude-3-sonnet-latest")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-5-haiku", alias = "claude-3-5-haiku-latest")]
    Claude3_5Haiku,
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
    // Anthropic models (already included)
    // Cohere models
    CohereCommandTextV14_4k,
    CohereCommandRV1,
    CohereCommandRPlusV1,
    CohereCommandLightTextV14_4k,
    // Meta models
    MetaLlama38BInstructV1,
    MetaLlama370BInstructV1,
    MetaLlama318BInstructV1_128k,
    MetaLlama318BInstructV1,
    MetaLlama3170BInstructV1_128k,
    MetaLlama3170BInstructV1,
    MetaLlama3211BInstructV1,
    MetaLlama3290BInstructV1,
    MetaLlama321BInstructV1,
    MetaLlama323BInstructV1,
    // Mistral models
    MistralMistral7BInstructV0,
    MistralMixtral8x7BInstructV0,
    MistralMistralLarge2402V1,
    MistralMistralSmall2402V1,
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
    pub fn from_id(id: &str) -> anyhow::Result<Self> {
        if id.starts_with("claude-3-5-sonnet") {
            Ok(Self::Claude3_5Sonnet)
        } else if id.starts_with("claude-3-opus") {
            Ok(Self::Claude3Opus)
        } else if id.starts_with("claude-3-sonnet") {
            Ok(Self::Claude3Sonnet)
        } else if id.starts_with("claude-3-5-haiku") {
            Ok(Self::Claude3_5Haiku)
        } else {
            Err(anyhow!("invalid model id"))
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Model::Claude3_5Sonnet => "anthropic.claude-3-5-sonnet-20241022-v2:0",
            Model::Claude3Opus => "anthropic.claude-3-opus-20240229-v1:0",
            Model::Claude3Sonnet => "anthropic.claude-3-sonnet-20240229-v1:0",
            Model::Claude3_5Haiku => "anthropic.claude-3-5-haiku-20241022-v1:0",
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
            Model::MetaLlama38BInstructV1 => "meta.llama3-8b-instruct-v1:0",
            Model::MetaLlama370BInstructV1 => "meta.llama3-70b-instruct-v1:0",
            Model::MetaLlama318BInstructV1_128k => "meta.llama3-1-8b-instruct-v1:0:128k",
            Model::MetaLlama318BInstructV1 => "meta.llama3-1-8b-instruct-v1:0",
            Model::MetaLlama3170BInstructV1_128k => "meta.llama3-1-70b-instruct-v1:0:128k",
            Model::MetaLlama3170BInstructV1 => "meta.llama3-1-70b-instruct-v1:0",
            Model::MetaLlama3211BInstructV1 => "meta.llama3-2-11b-instruct-v1:0",
            Model::MetaLlama3290BInstructV1 => "meta.llama3-2-90b-instruct-v1:0",
            Model::MetaLlama321BInstructV1 => "meta.llama3-2-1b-instruct-v1:0",
            Model::MetaLlama323BInstructV1 => "meta.llama3-2-3b-instruct-v1:0",
            Model::MistralMistral7BInstructV0 => "mistral.mistral-7b-instruct-v0:2",
            Model::MistralMixtral8x7BInstructV0 => "mistral.mixtral-8x7b-instruct-v0:1",
            Model::MistralMistralLarge2402V1 => "mistral.mistral-large-2402-v1:0",
            Model::MistralMistralSmall2402V1 => "mistral.mistral-small-2402-v1:0",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3_5Haiku => "Claude 3.5 Haiku",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3_5Haiku => 200_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> u32 {
        match self {
            Self::Claude3Opus | Self::Claude3Sonnet | Self::Claude3_5Haiku => 4_096,
            Self::Claude3_5Sonnet => 8_192,
            Self::Custom {
                max_output_tokens, ..
            } => max_output_tokens.unwrap_or(4_096),
        }
    }

    pub fn default_temperature(&self) -> f32 {
        match self {
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3_5Haiku => 1.0,
            Self::Custom {
                default_temperature,
                ..
            } => default_temperature.unwrap_or(1.0),
        }
    }
}

/**
"ai21.j2-grande-instruct"
"ai21.j2-jumbo-instruct"
"ai21.j2-mid"
"ai21.j2-mid-v1"
"ai21.j2-ultra"
"ai21.j2-ultra-v1:0:8k"
"ai21.j2-ultra-v1"
"ai21.jamba-instruct-v1:0"
"ai21.jamba-1-5-large-v1:0"
"ai21.jamba-1-5-mini-v1:0"
"anthropic.claude-3-sonnet-20240229-v1:0"
"anthropic.claude-3-haiku-20240307-v1:0"
"anthropic.claude-3-opus-20240229-v1:0"
"anthropic.claude-3-5-sonnet-20241022-v2:0"
"anthropic.claude-3-5-haiku-20241022-v1:0"
"cohere.command-text-v14:7:4k"
"cohere.command-r-v1:0"
"cohere.command-r-plus-v1:0"
"cohere.command-light-text-v14:7:4k"
"meta.llama3-8b-instruct-v1:0"
"meta.llama3-70b-instruct-v1:0"
"meta.llama3-1-8b-instruct-v1:0:128k"
"meta.llama3-1-8b-instruct-v1:0"
"meta.llama3-1-70b-instruct-v1:0:128k"
"meta.llama3-1-70b-instruct-v1:0"
"meta.llama3-2-11b-instruct-v1:0"
"meta.llama3-2-90b-instruct-v1:0"
"meta.llama3-2-1b-instruct-v1:0"
"meta.llama3-2-3b-instruct-v1:0"
"mistral.mistral-7b-instruct-v0:2"
"mistral.mixtral-8x7b-instruct-v0:1"
"mistral.mistral-large-2402-v1:0"
"mistral.mistral-small-2402-v1:0"
**/