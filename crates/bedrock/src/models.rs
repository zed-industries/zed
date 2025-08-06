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
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BedrockModelCacheConfiguration {
    pub max_cache_anchors: usize,
    pub min_total_token: u64,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    // Anthropic models (already included)
    #[default]
    #[serde(rename = "claude-sonnet-4", alias = "claude-sonnet-4-latest")]
    ClaudeSonnet4,
    #[serde(
        rename = "claude-sonnet-4-thinking",
        alias = "claude-sonnet-4-thinking-latest"
    )]
    ClaudeSonnet4Thinking,
    #[serde(rename = "claude-opus-4", alias = "claude-opus-4-latest")]
    ClaudeOpus4,
    #[serde(rename = "claude-opus-4-1", alias = "claude-opus-4-1-latest")]
    ClaudeOpus4_1,
    #[serde(
        rename = "claude-opus-4-thinking",
        alias = "claude-opus-4-thinking-latest"
    )]
    ClaudeOpus4Thinking,
    #[serde(
        rename = "claude-opus-4-1-thinking",
        alias = "claude-opus-4-1-thinking-latest"
    )]
    ClaudeOpus4_1Thinking,
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
    MetaLlama38BInstructV1,
    MetaLlama370BInstructV1,
    MetaLlama318BInstructV1_128k,
    MetaLlama318BInstructV1,
    MetaLlama3170BInstructV1_128k,
    MetaLlama3170BInstructV1,
    MetaLlama31405BInstructV1,
    MetaLlama321BInstructV1,
    MetaLlama323BInstructV1,
    MetaLlama3211BInstructV1,
    MetaLlama3290BInstructV1,
    MetaLlama3370BInstructV1,
    #[allow(non_camel_case_types)]
    MetaLlama4Scout17BInstructV1,
    #[allow(non_camel_case_types)]
    MetaLlama4Maverick17BInstructV1,
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
        max_tokens: u64,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        max_output_tokens: Option<u64>,
        default_temperature: Option<f32>,
        cache_configuration: Option<BedrockModelCacheConfiguration>,
    },
}

impl Model {
    pub fn default_fast(region: &str) -> Self {
        if region.starts_with("us-") {
            Self::Claude3_5Haiku
        } else {
            Self::Claude3Haiku
        }
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
            anyhow::bail!("invalid model id {id}");
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Model::ClaudeSonnet4 => "claude-4-sonnet",
            Model::ClaudeSonnet4Thinking => "claude-4-sonnet-thinking",
            Model::ClaudeOpus4 => "claude-4-opus",
            Model::ClaudeOpus4_1 => "claude-4-opus-1",
            Model::ClaudeOpus4Thinking => "claude-4-opus-thinking",
            Model::ClaudeOpus4_1Thinking => "claude-4-opus-1-thinking",
            Model::Claude3_5SonnetV2 => "claude-3-5-sonnet-v2",
            Model::Claude3_5Sonnet => "claude-3-5-sonnet",
            Model::Claude3Opus => "claude-3-opus",
            Model::Claude3Sonnet => "claude-3-sonnet",
            Model::Claude3Haiku => "claude-3-haiku",
            Model::Claude3_5Haiku => "claude-3-5-haiku",
            Model::Claude3_7Sonnet => "claude-3-7-sonnet",
            Model::Claude3_7SonnetThinking => "claude-3-7-sonnet-thinking",
            Model::AmazonNovaLite => "amazon-nova-lite",
            Model::AmazonNovaMicro => "amazon-nova-micro",
            Model::AmazonNovaPro => "amazon-nova-pro",
            Model::AmazonNovaPremier => "amazon-nova-premier",
            Model::DeepSeekR1 => "deepseek-r1",
            Model::AI21J2GrandeInstruct => "ai21-j2-grande-instruct",
            Model::AI21J2JumboInstruct => "ai21-j2-jumbo-instruct",
            Model::AI21J2Mid => "ai21-j2-mid",
            Model::AI21J2MidV1 => "ai21-j2-mid-v1",
            Model::AI21J2Ultra => "ai21-j2-ultra",
            Model::AI21J2UltraV1_8k => "ai21-j2-ultra-v1-8k",
            Model::AI21J2UltraV1 => "ai21-j2-ultra-v1",
            Model::AI21JambaInstructV1 => "ai21-jamba-instruct-v1",
            Model::AI21Jamba15LargeV1 => "ai21-jamba-1-5-large-v1",
            Model::AI21Jamba15MiniV1 => "ai21-jamba-1-5-mini-v1",
            Model::CohereCommandTextV14_4k => "cohere-command-text-v14-4k",
            Model::CohereCommandRV1 => "cohere-command-r-v1",
            Model::CohereCommandRPlusV1 => "cohere-command-r-plus-v1",
            Model::CohereCommandLightTextV14_4k => "cohere-command-light-text-v14-4k",
            Model::MetaLlama38BInstructV1 => "meta-llama3-8b-instruct-v1",
            Model::MetaLlama370BInstructV1 => "meta-llama3-70b-instruct-v1",
            Model::MetaLlama318BInstructV1_128k => "meta-llama3-1-8b-instruct-v1-128k",
            Model::MetaLlama318BInstructV1 => "meta-llama3-1-8b-instruct-v1",
            Model::MetaLlama3170BInstructV1_128k => "meta-llama3-1-70b-instruct-v1-128k",
            Model::MetaLlama3170BInstructV1 => "meta-llama3-1-70b-instruct-v1",
            Model::MetaLlama31405BInstructV1 => "meta-llama3-1-405b-instruct-v1",
            Model::MetaLlama321BInstructV1 => "meta-llama3-2-1b-instruct-v1",
            Model::MetaLlama323BInstructV1 => "meta-llama3-2-3b-instruct-v1",
            Model::MetaLlama3211BInstructV1 => "meta-llama3-2-11b-instruct-v1",
            Model::MetaLlama3290BInstructV1 => "meta-llama3-2-90b-instruct-v1",
            Model::MetaLlama3370BInstructV1 => "meta-llama3-3-70b-instruct-v1",
            Model::MetaLlama4Scout17BInstructV1 => "meta-llama4-scout-17b-instruct-v1",
            Model::MetaLlama4Maverick17BInstructV1 => "meta-llama4-maverick-17b-instruct-v1",
            Model::MistralMistral7BInstructV0 => "mistral-7b-instruct-v0",
            Model::MistralMixtral8x7BInstructV0 => "mistral-mixtral-8x7b-instruct-v0",
            Model::MistralMistralLarge2402V1 => "mistral-large-2402-v1",
            Model::MistralMistralSmall2402V1 => "mistral-small-2402-v1",
            Model::MistralPixtralLarge2502V1 => "mistral-pixtral-large-2502-v1",
            Model::PalmyraWriterX4 => "palmyra-writer-x4",
            Model::PalmyraWriterX5 => "palmyra-writer-x5",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn request_id(&self) -> &str {
        match self {
            Model::ClaudeSonnet4 | Model::ClaudeSonnet4Thinking => {
                "anthropic.claude-sonnet-4-20250514-v1:0"
            }
            Model::ClaudeOpus4 | Model::ClaudeOpus4Thinking => {
                "anthropic.claude-opus-4-20250514-v1:0"
            }
            Model::ClaudeOpus4_1 | Model::ClaudeOpus4_1Thinking => {
                "anthropic.claude-opus-4-1-20250805-v1:0"
            }
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
            Model::MetaLlama38BInstructV1 => "meta.llama3-8b-instruct-v1:0",
            Model::MetaLlama370BInstructV1 => "meta.llama3-70b-instruct-v1:0",
            Model::MetaLlama318BInstructV1_128k => "meta.llama3-1-8b-instruct-v1:0",
            Model::MetaLlama318BInstructV1 => "meta.llama3-1-8b-instruct-v1:0",
            Model::MetaLlama3170BInstructV1_128k => "meta.llama3-1-70b-instruct-v1:0",
            Model::MetaLlama3170BInstructV1 => "meta.llama3-1-70b-instruct-v1:0",
            Model::MetaLlama31405BInstructV1 => "meta.llama3-1-405b-instruct-v1:0",
            Model::MetaLlama3211BInstructV1 => "meta.llama3-2-11b-instruct-v1:0",
            Model::MetaLlama3290BInstructV1 => "meta.llama3-2-90b-instruct-v1:0",
            Model::MetaLlama321BInstructV1 => "meta.llama3-2-1b-instruct-v1:0",
            Model::MetaLlama323BInstructV1 => "meta.llama3-2-3b-instruct-v1:0",
            Model::MetaLlama3370BInstructV1 => "meta.llama3-3-70b-instruct-v1:0",
            Model::MetaLlama4Scout17BInstructV1 => "meta.llama4-scout-17b-instruct-v1:0",
            Model::MetaLlama4Maverick17BInstructV1 => "meta.llama4-maverick-17b-instruct-v1:0",
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
            Self::ClaudeSonnet4 => "Claude Sonnet 4",
            Self::ClaudeSonnet4Thinking => "Claude Sonnet 4 Thinking",
            Self::ClaudeOpus4 => "Claude Opus 4",
            Self::ClaudeOpus4_1 => "Claude Opus 4.1",
            Self::ClaudeOpus4Thinking => "Claude Opus 4 Thinking",
            Self::ClaudeOpus4_1Thinking => "Claude Opus 4.1 Thinking",
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
            Self::MetaLlama38BInstructV1 => "Meta Llama 3 8B Instruct",
            Self::MetaLlama370BInstructV1 => "Meta Llama 3 70B Instruct",
            Self::MetaLlama318BInstructV1_128k => "Meta Llama 3.1 8B Instruct 128K",
            Self::MetaLlama318BInstructV1 => "Meta Llama 3.1 8B Instruct",
            Self::MetaLlama3170BInstructV1_128k => "Meta Llama 3.1 70B Instruct 128K",
            Self::MetaLlama3170BInstructV1 => "Meta Llama 3.1 70B Instruct",
            Self::MetaLlama31405BInstructV1 => "Meta Llama 3.1 405B Instruct",
            Self::MetaLlama3211BInstructV1 => "Meta Llama 3.2 11B Instruct",
            Self::MetaLlama3290BInstructV1 => "Meta Llama 3.2 90B Instruct",
            Self::MetaLlama321BInstructV1 => "Meta Llama 3.2 1B Instruct",
            Self::MetaLlama323BInstructV1 => "Meta Llama 3.2 3B Instruct",
            Self::MetaLlama3370BInstructV1 => "Meta Llama 3.3 70B Instruct",
            Self::MetaLlama4Scout17BInstructV1 => "Meta Llama 4 Scout 17B Instruct",
            Self::MetaLlama4Maverick17BInstructV1 => "Meta Llama 4 Maverick 17B Instruct",
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

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::Claude3_5SonnetV2
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3_5Haiku
            | Self::Claude3_7Sonnet
            | Self::ClaudeSonnet4
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4_1
            | Self::ClaudeSonnet4Thinking
            | Self::ClaudeOpus4Thinking
            | Self::ClaudeOpus4_1Thinking => 200_000,
            Self::AmazonNovaPremier => 1_000_000,
            Self::PalmyraWriterX5 => 1_000_000,
            Self::PalmyraWriterX4 => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
            _ => 128_000,
        }
    }

    pub fn max_output_tokens(&self) -> u64 {
        match self {
            Self::Claude3Opus | Self::Claude3Sonnet | Self::Claude3_5Haiku => 4_096,
            Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4Thinking
            | Self::ClaudeOpus4
            | Model::ClaudeOpus4Thinking
            | Self::ClaudeOpus4_1
            | Model::ClaudeOpus4_1Thinking => 128_000,
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
            | Self::Claude3_7Sonnet
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4Thinking
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_1Thinking
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4Thinking => 1.0,
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
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4Thinking
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_1Thinking
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4Thinking
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

    pub fn supports_caching(&self) -> bool {
        match self {
            // Only Claude models on Bedrock support caching
            // Nova models support only text caching
            // https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models
            Self::Claude3_5Haiku
            | Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4Thinking
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4Thinking
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_1Thinking => true,

            // Custom models - check if they have cache configuration
            Self::Custom {
                cache_configuration,
                ..
            } => cache_configuration.is_some(),

            // All other models don't support caching
            _ => false,
        }
    }

    pub fn cache_configuration(&self) -> Option<BedrockModelCacheConfiguration> {
        match self {
            Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4Thinking
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4Thinking
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_1Thinking => Some(BedrockModelCacheConfiguration {
                max_cache_anchors: 4,
                min_total_token: 1024,
            }),

            Self::Claude3_5Haiku => Some(BedrockModelCacheConfiguration {
                max_cache_anchors: 4,
                min_total_token: 2048,
            }),

            Self::Custom {
                cache_configuration,
                ..
            } => cache_configuration.clone(),

            _ => None,
        }
    }

    pub fn mode(&self) -> BedrockModelMode {
        match self {
            Model::Claude3_7SonnetThinking => BedrockModelMode::Thinking {
                budget_tokens: Some(4096),
            },
            Model::ClaudeSonnet4Thinking => BedrockModelMode::Thinking {
                budget_tokens: Some(4096),
            },
            Model::ClaudeOpus4Thinking | Model::ClaudeOpus4_1Thinking => {
                BedrockModelMode::Thinking {
                    budget_tokens: Some(4096),
                }
            }
            _ => BedrockModelMode::Default,
        }
    }

    pub fn cross_region_inference_id(&self, region: &str) -> anyhow::Result<String> {
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
            anyhow::bail!("Unsupported Region {region}");
        };

        let model_id = self.request_id();

        match (self, region_group) {
            // Custom models can't have CRI IDs
            (Model::Custom { .. }, _) => Ok(self.request_id().into()),

            // Models with US Gov only
            (Model::Claude3_5Sonnet, "us-gov") | (Model::Claude3Haiku, "us-gov") => {
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
                | Model::ClaudeSonnet4
                | Model::ClaudeSonnet4Thinking
                | Model::ClaudeOpus4
                | Model::ClaudeOpus4Thinking
                | Model::ClaudeOpus4_1
                | Model::ClaudeOpus4_1Thinking
                | Model::Claude3Haiku
                | Model::Claude3Opus
                | Model::Claude3Sonnet
                | Model::DeepSeekR1
                | Model::MetaLlama31405BInstructV1
                | Model::MetaLlama3170BInstructV1_128k
                | Model::MetaLlama3170BInstructV1
                | Model::MetaLlama318BInstructV1_128k
                | Model::MetaLlama318BInstructV1
                | Model::MetaLlama3211BInstructV1
                | Model::MetaLlama321BInstructV1
                | Model::MetaLlama323BInstructV1
                | Model::MetaLlama3290BInstructV1
                | Model::MetaLlama3370BInstructV1
                | Model::MetaLlama4Maverick17BInstructV1
                | Model::MetaLlama4Scout17BInstructV1
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
                | Model::ClaudeSonnet4
                | Model::ClaudeSonnet4Thinking
                | Model::Claude3Haiku
                | Model::Claude3Sonnet
                | Model::MetaLlama321BInstructV1
                | Model::MetaLlama323BInstructV1
                | Model::MistralPixtralLarge2502V1,
                "eu",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // Models available in APAC
            (
                Model::Claude3_5Sonnet
                | Model::Claude3_5SonnetV2
                | Model::Claude3Haiku
                | Model::Claude3Sonnet
                | Model::Claude3_7Sonnet
                | Model::Claude3_7SonnetThinking
                | Model::ClaudeSonnet4
                | Model::ClaudeSonnet4Thinking,
                "apac",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // Any other combination is not supported
            _ => Ok(self.request_id().into()),
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
            Model::ClaudeSonnet4.cross_region_inference_id("eu-west-1")?,
            "eu.anthropic.claude-sonnet-4-20250514-v1:0"
        );
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
            Model::MetaLlama370BInstructV1.cross_region_inference_id("us-east-1")?,
            "meta.llama3-70b-instruct-v1:0"
        );
        assert_eq!(
            Model::MetaLlama3170BInstructV1.cross_region_inference_id("us-east-1")?,
            "us.meta.llama3-1-70b-instruct-v1:0"
        );
        assert_eq!(
            Model::MetaLlama321BInstructV1.cross_region_inference_id("eu-west-1")?,
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
            cache_configuration: None,
        };

        // Custom model should return its name unchanged
        assert_eq!(
            custom_model.cross_region_inference_id("us-east-1")?,
            "custom.my-model-v1:0"
        );

        Ok(())
    }

    #[test]
    fn test_friendly_id_vs_request_id() {
        // Test that id() returns friendly identifiers
        assert_eq!(Model::Claude3_5SonnetV2.id(), "claude-3-5-sonnet-v2");
        assert_eq!(Model::AmazonNovaLite.id(), "amazon-nova-lite");
        assert_eq!(Model::DeepSeekR1.id(), "deepseek-r1");
        assert_eq!(
            Model::MetaLlama38BInstructV1.id(),
            "meta-llama3-8b-instruct-v1"
        );

        // Test that request_id() returns actual backend model IDs
        assert_eq!(
            Model::Claude3_5SonnetV2.request_id(),
            "anthropic.claude-3-5-sonnet-20241022-v2:0"
        );
        assert_eq!(Model::AmazonNovaLite.request_id(), "amazon.nova-lite-v1:0");
        assert_eq!(Model::DeepSeekR1.request_id(), "deepseek.r1-v1:0");
        assert_eq!(
            Model::MetaLlama38BInstructV1.request_id(),
            "meta.llama3-8b-instruct-v1:0"
        );

        // Test thinking models have different friendly IDs but same request IDs
        assert_eq!(Model::ClaudeSonnet4.id(), "claude-4-sonnet");
        assert_eq!(
            Model::ClaudeSonnet4Thinking.id(),
            "claude-4-sonnet-thinking"
        );
        assert_eq!(
            Model::ClaudeSonnet4.request_id(),
            Model::ClaudeSonnet4Thinking.request_id()
        );
    }
}
