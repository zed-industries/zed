use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumIter;

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum BedrockAdaptiveThinkingEffort {
    Low,
    Medium,
    #[default]
    High,
    Max,
}

impl BedrockAdaptiveThinkingEffort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Max => "max",
        }
    }
}

impl FromStr for BedrockAdaptiveThinkingEffort {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "max" => Ok(Self::Max),
            other => anyhow::bail!("unknown adaptive thinking effort: {other}"),
        }
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum BedrockModelMode {
    #[default]
    Default,
    Thinking {
        budget_tokens: Option<u64>,
    },
    AdaptiveThinking {
        effort: BedrockAdaptiveThinkingEffort,
    },
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BedrockModelCacheConfiguration {
    pub max_cache_anchors: usize,
    pub min_total_token: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModelCapabilities {
    pub max_tokens: u64,
    pub max_output_tokens: u64,
    pub default_temperature: f32,
    pub supports_tool_use: bool,
    pub supports_images: bool,
    pub supports_thinking: bool,
    pub supports_adaptive_thinking: bool,
    pub cache_configuration: Option<BedrockModelCacheConfiguration>,
    pub extended_context_token_count: Option<u64>,
    pub thinking_mode: BedrockModelMode,
}

macro_rules! define_bedrock_models {
    (
        $(
            $(#[$variant_meta:meta])*
            $variant:ident {
                id: $id:literal,
                $(aliases: [$($alias:literal),* $(,)?],)?
                request_id: $request_id:literal,
                display_name: $display_name:literal,
                max_tokens: $max_tokens:expr,
                max_output_tokens: $max_output_tokens:expr,
                default_temperature: $default_temp:expr,
                supports_tool_use: $tool_use:expr,
                supports_images: $images:expr,
                supports_thinking: $thinking:expr,
                supports_adaptive_thinking: $adaptive:expr,
                cache_configuration: $cache:expr,
                extended_context_token_count: $extended:expr,
                thinking_mode: $thinking_mode:expr $(,)?
            }
        ),* $(,)?
    ) => {
        #[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
        #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
        pub enum Model {
            $(
                $(#[$variant_meta])*
                #[serde(rename = $id $($(, alias = $alias)*)?)]
                $variant,
            )*

            #[serde(rename = "custom")]
            Custom {
                name: String,
                max_tokens: u64,
                display_name: Option<String>,
                max_output_tokens: Option<u64>,
                default_temperature: Option<f32>,
                cache_configuration: Option<BedrockModelCacheConfiguration>,
            },
        }

        impl Model {
            pub fn from_id(id: &str) -> anyhow::Result<Self> {
                $(
                    if id == $id $($(|| id == $alias)*)? {
                        return Ok(Self::$variant);
                    }
                )*
                anyhow::bail!("invalid model id {id}")
            }

            pub fn id(&self) -> &str {
                match self {
                    $(Self::$variant => $id,)*
                    Self::Custom { name, .. } => name,
                }
            }

            pub fn request_id(&self) -> &str {
                match self {
                    $(Self::$variant => $request_id,)*
                    Self::Custom { name, .. } => name,
                }
            }

            pub fn display_name(&self) -> &str {
                match self {
                    $(Self::$variant => $display_name,)*
                    Self::Custom { display_name, name, .. } => {
                        display_name.as_deref().unwrap_or(name.as_str())
                    }
                }
            }

            pub fn capabilities(&self) -> ModelCapabilities {
                match self {
                    $(Self::$variant => ModelCapabilities {
                        max_tokens: $max_tokens,
                        max_output_tokens: $max_output_tokens,
                        default_temperature: $default_temp,
                        supports_tool_use: $tool_use,
                        supports_images: $images,
                        supports_thinking: $thinking,
                        supports_adaptive_thinking: $adaptive,
                        cache_configuration: $cache,
                        extended_context_token_count: $extended,
                        thinking_mode: $thinking_mode,
                    },)*
                    Self::Custom {
                        max_tokens,
                        max_output_tokens,
                        default_temperature,
                        cache_configuration,
                        ..
                    } => ModelCapabilities {
                        max_tokens: *max_tokens,
                        max_output_tokens: max_output_tokens.unwrap_or(4_096),
                        default_temperature: default_temperature.unwrap_or(1.0),
                        supports_tool_use: false,
                        supports_images: false,
                        supports_thinking: false,
                        supports_adaptive_thinking: false,
                        cache_configuration: *cache_configuration,
                        extended_context_token_count: None,
                        thinking_mode: BedrockModelMode::Default,
                    },
                }
            }
        }
    };
}

define_bedrock_models! {
    ClaudeHaiku4_5 {
        id: "claude-haiku-4-5",
        aliases: ["claude-haiku-4-5-latest"],
        request_id: "anthropic.claude-haiku-4-5-20251001-v1:0",
        display_name: "Claude Haiku 4.5",
        max_tokens: 200_000,
        max_output_tokens: 64_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: true,
        supports_adaptive_thinking: false,
        cache_configuration: Some(BedrockModelCacheConfiguration { max_cache_anchors: 4, min_total_token: 2048 }),
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Thinking { budget_tokens: Some(4096) },
    },

    ClaudeSonnet4 {
        id: "claude-sonnet-4",
        aliases: [
            "claude-sonnet-4-latest",
            "claude-sonnet-4-thinking",
            "claude-sonnet-4-thinking-latest",
        ],
        request_id: "anthropic.claude-sonnet-4-20250514-v1:0",
        display_name: "Claude Sonnet 4",
        max_tokens: 200_000,
        max_output_tokens: 64_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: true,
        supports_adaptive_thinking: false,
        cache_configuration: Some(BedrockModelCacheConfiguration { max_cache_anchors: 4, min_total_token: 1024 }),
        extended_context_token_count: Some(1_000_000),
        thinking_mode: BedrockModelMode::Thinking { budget_tokens: Some(4096) },
    },

    #[default]
    ClaudeSonnet4_5 {
        id: "claude-sonnet-4-5",
        aliases: [
            "claude-sonnet-4-5-latest",
            "claude-sonnet-4-5-thinking",
            "claude-sonnet-4-5-thinking-latest",
        ],
        request_id: "anthropic.claude-sonnet-4-5-20250929-v1:0",
        display_name: "Claude Sonnet 4.5",
        max_tokens: 200_000,
        max_output_tokens: 64_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: true,
        supports_adaptive_thinking: false,
        cache_configuration: Some(BedrockModelCacheConfiguration { max_cache_anchors: 4, min_total_token: 1024 }),
        extended_context_token_count: Some(1_000_000),
        thinking_mode: BedrockModelMode::Thinking { budget_tokens: Some(4096) },
    },

    ClaudeOpus4_1 {
        id: "claude-opus-4-1",
        aliases: [
            "claude-opus-4-1-latest",
            "claude-opus-4-1-thinking",
            "claude-opus-4-1-thinking-latest",
        ],
        request_id: "anthropic.claude-opus-4-1-20250805-v1:0",
        display_name: "Claude Opus 4.1",
        max_tokens: 200_000,
        max_output_tokens: 32_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: true,
        supports_adaptive_thinking: false,
        cache_configuration: Some(BedrockModelCacheConfiguration { max_cache_anchors: 4, min_total_token: 1024 }),
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Thinking { budget_tokens: Some(4096) },
    },

    ClaudeOpus4_5 {
        id: "claude-opus-4-5",
        aliases: [
            "claude-opus-4-5-latest",
            "claude-opus-4-5-thinking",
            "claude-opus-4-5-thinking-latest",
        ],
        request_id: "anthropic.claude-opus-4-5-20251101-v1:0",
        display_name: "Claude Opus 4.5",
        max_tokens: 200_000,
        max_output_tokens: 64_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: true,
        supports_adaptive_thinking: false,
        cache_configuration: Some(BedrockModelCacheConfiguration { max_cache_anchors: 4, min_total_token: 1024 }),
        extended_context_token_count: Some(1_000_000),
        thinking_mode: BedrockModelMode::Thinking { budget_tokens: Some(4096) },
    },

    ClaudeOpus4_6 {
        id: "claude-opus-4-6",
        aliases: [
            "claude-opus-4-6-latest",
            "claude-opus-4-6-thinking",
            "claude-opus-4-6-thinking-latest",
        ],
        request_id: "anthropic.claude-opus-4-6-v1",
        display_name: "Claude Opus 4.6",
        max_tokens: 200_000,
        max_output_tokens: 128_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: true,
        supports_adaptive_thinking: true,
        cache_configuration: Some(BedrockModelCacheConfiguration { max_cache_anchors: 4, min_total_token: 1024 }),
        extended_context_token_count: Some(1_000_000),
        thinking_mode: BedrockModelMode::AdaptiveThinking { effort: BedrockAdaptiveThinkingEffort::High },
    },

    ClaudeSonnet4_6 {
        id: "claude-sonnet-4-6",
        aliases: [
            "claude-sonnet-4-6-latest",
            "claude-sonnet-4-6-thinking",
            "claude-sonnet-4-6-thinking-latest",
        ],
        request_id: "anthropic.claude-sonnet-4-6",
        display_name: "Claude Sonnet 4.6",
        max_tokens: 200_000,
        max_output_tokens: 64_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: true,
        supports_adaptive_thinking: true,
        cache_configuration: Some(BedrockModelCacheConfiguration { max_cache_anchors: 4, min_total_token: 1024 }),
        extended_context_token_count: Some(1_000_000),
        thinking_mode: BedrockModelMode::AdaptiveThinking { effort: BedrockAdaptiveThinkingEffort::High },
    },

    Llama4Scout17B {
        id: "llama-4-scout-17b",
        request_id: "meta.llama4-scout-17b-instruct-v1:0",
        display_name: "Llama 4 Scout 17B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: false,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Llama4Maverick17B {
        id: "llama-4-maverick-17b",
        request_id: "meta.llama4-maverick-17b-instruct-v1:0",
        display_name: "Llama 4 Maverick 17B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: false,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Gemma3_4B {
        id: "gemma-3-4b",
        request_id: "google.gemma-3-4b-it",
        display_name: "Gemma 3 4B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: false,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Gemma3_12B {
        id: "gemma-3-12b",
        request_id: "google.gemma-3-12b-it",
        display_name: "Gemma 3 12B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: false,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Gemma3_27B {
        id: "gemma-3-27b",
        request_id: "google.gemma-3-27b-it",
        display_name: "Gemma 3 27B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: false,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    MagistralSmall {
        id: "magistral-small",
        request_id: "mistral.magistral-small-2509",
        display_name: "Magistral Small",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    MistralLarge3 {
        id: "mistral-large-3",
        request_id: "mistral.mistral-large-3-675b-instruct",
        display_name: "Mistral Large 3",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    PixtralLarge {
        id: "pixtral-large",
        request_id: "mistral.pixtral-large-2502-v1:0",
        display_name: "Pixtral Large",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Qwen3_32B {
        id: "qwen3-32b",
        request_id: "qwen.qwen3-32b-v1:0",
        display_name: "Qwen3 32B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Qwen3VL235B {
        id: "qwen3-vl-235b",
        request_id: "qwen.qwen3-vl-235b-a22b",
        display_name: "Qwen3 VL 235B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Qwen3_235B {
        id: "qwen3-235b",
        request_id: "qwen.qwen3-235b-a22b-2507-v1:0",
        display_name: "Qwen3 235B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Qwen3Next80B {
        id: "qwen3-next-80b",
        request_id: "qwen.qwen3-next-80b-a3b",
        display_name: "Qwen3 Next 80B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Qwen3Coder30B {
        id: "qwen3-coder-30b",
        request_id: "qwen.qwen3-coder-30b-a3b-v1:0",
        display_name: "Qwen3 Coder 30B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Qwen3CoderNext {
        id: "qwen3-coder-next",
        request_id: "qwen.qwen3-coder-next",
        display_name: "Qwen3 Coder Next",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Qwen3Coder480B {
        id: "qwen3-coder-480b",
        request_id: "qwen.qwen3-coder-480b-a35b-v1:0",
        display_name: "Qwen3 Coder 480B",
        max_tokens: 128_000,
        max_output_tokens: 8_192,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    NovaLite {
        id: "nova-lite",
        request_id: "amazon.nova-lite-v1:0",
        display_name: "Amazon Nova Lite",
        max_tokens: 300_000,
        max_output_tokens: 5_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    NovaPro {
        id: "nova-pro",
        request_id: "amazon.nova-pro-v1:0",
        display_name: "Amazon Nova Pro",
        max_tokens: 300_000,
        max_output_tokens: 5_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    NovaPremier {
        id: "nova-premier",
        request_id: "amazon.nova-premier-v1:0",
        display_name: "Amazon Nova Premier",
        max_tokens: 1_000_000,
        max_output_tokens: 5_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    Nova2Lite {
        id: "nova-2-lite",
        request_id: "amazon.nova-2-lite-v1:0",
        display_name: "Amazon Nova 2 Lite",
        max_tokens: 300_000,
        max_output_tokens: 5_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    GptOss20B {
        id: "gpt-oss-20b",
        request_id: "openai.gpt-oss-20b-1:0",
        display_name: "GPT OSS 20B",
        max_tokens: 128_000,
        max_output_tokens: 16_000,
        default_temperature: 1.0,
        supports_tool_use: false,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    GptOss120B {
        id: "gpt-oss-120b",
        request_id: "openai.gpt-oss-120b-1:0",
        display_name: "GPT OSS 120B",
        max_tokens: 128_000,
        max_output_tokens: 16_000,
        default_temperature: 1.0,
        supports_tool_use: false,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    MiniMaxM2 {
        id: "minimax-m2",
        request_id: "minimax.minimax-m2",
        display_name: "MiniMax M2",
        max_tokens: 128_000,
        max_output_tokens: 16_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    KimiK2Thinking {
        id: "kimi-k2-thinking",
        request_id: "moonshot.kimi-k2-thinking",
        display_name: "Kimi K2 Thinking",
        max_tokens: 128_000,
        max_output_tokens: 16_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    KimiK2_5 {
        id: "kimi-k2-5",
        request_id: "moonshotai.kimi-k2.5",
        display_name: "Kimi K2.5",
        max_tokens: 128_000,
        max_output_tokens: 16_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: true,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    DeepSeekR1 {
        id: "deepseek-r1",
        request_id: "deepseek.r1-v1:0",
        display_name: "DeepSeek R1",
        max_tokens: 128_000,
        max_output_tokens: 16_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    DeepSeekV3_1 {
        id: "deepseek-v3",
        request_id: "deepseek.v3-v1:0",
        display_name: "DeepSeek V3.1",
        max_tokens: 128_000,
        max_output_tokens: 16_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },

    DeepSeekV3_2 {
        id: "deepseek-v3-2",
        request_id: "deepseek.v3.2",
        display_name: "DeepSeek V3.2",
        max_tokens: 128_000,
        max_output_tokens: 16_000,
        default_temperature: 1.0,
        supports_tool_use: true,
        supports_images: false,
        supports_thinking: false,
        supports_adaptive_thinking: false,
        cache_configuration: None,
        extended_context_token_count: None,
        thinking_mode: BedrockModelMode::Default,
    },
}

impl Model {
    pub fn default_fast(_region: &str) -> Self {
        Self::ClaudeHaiku4_5
    }

    pub fn max_token_count(&self) -> u64 {
        self.capabilities().max_tokens
    }

    pub fn max_tokens(&self) -> u64 {
        self.capabilities().max_tokens
    }

    pub fn max_output_tokens(&self) -> u64 {
        self.capabilities().max_output_tokens
    }

    pub fn default_temperature(&self) -> f32 {
        self.capabilities().default_temperature
    }

    pub fn supports_tool_use(&self) -> bool {
        self.capabilities().supports_tool_use
    }

    pub fn supports_images(&self) -> bool {
        self.capabilities().supports_images
    }

    pub fn supports_extended_context(&self) -> bool {
        self.capabilities().extended_context_token_count.is_some()
    }

    pub fn extended_context_token_count(&self) -> Option<u64> {
        self.capabilities().extended_context_token_count
    }

    pub fn supports_caching(&self) -> bool {
        self.capabilities().cache_configuration.is_some()
    }

    pub fn cache_configuration(&self) -> Option<BedrockModelCacheConfiguration> {
        self.capabilities().cache_configuration
    }

    pub fn supports_thinking(&self) -> bool {
        self.capabilities().supports_thinking
    }

    pub fn supports_adaptive_thinking(&self) -> bool {
        self.capabilities().supports_adaptive_thinking
    }

    pub fn thinking_mode(&self) -> BedrockModelMode {
        self.capabilities().thinking_mode
    }

    pub fn cross_region_inference_id(
        &self,
        region: &str,
        allow_global: bool,
    ) -> anyhow::Result<String> {
        let model_id = self.request_id();

        let supports_global = matches!(
            self,
            Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeSonnet4_6
                | Self::Nova2Lite
        );

        let region_group = if region.starts_with("us-gov-") {
            "us-gov"
        } else if region.starts_with("us-") || region.starts_with("sa-") {
            if allow_global && supports_global {
                "global"
            } else {
                "us"
            }
        } else if region.starts_with("ca-") {
            if allow_global && supports_global {
                "global"
            } else {
                "ca"
            }
        } else if region.starts_with("eu-") {
            if allow_global && supports_global {
                "global"
            } else {
                "eu"
            }
        } else if region == "ap-southeast-2" || region == "ap-southeast-4" {
            if allow_global && supports_global {
                "global"
            } else {
                "au"
            }
        } else if region == "ap-northeast-1" || region == "ap-northeast-3" {
            if allow_global && supports_global {
                "global"
            } else {
                "jp"
            }
        } else if region.starts_with("ap-") || region.starts_with("me-") {
            if allow_global && supports_global {
                "global"
            } else {
                "apac"
            }
        } else {
            anyhow::bail!("Unsupported Region {region}");
        };

        match (self, region_group) {
            (Self::Custom { .. }, _) => Ok(model_id.into()),

            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeSonnet4_6
                | Self::Nova2Lite,
                "global",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            (Self::ClaudeSonnet4_5, "us-gov") => Ok(format!("{}.{}", region_group, model_id)),

            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_1
                | Self::ClaudeOpus4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeSonnet4_6
                | Self::Llama4Scout17B
                | Self::Llama4Maverick17B
                | Self::NovaLite
                | Self::NovaPro
                | Self::NovaPremier
                | Self::Nova2Lite
                | Self::PixtralLarge
                | Self::DeepSeekR1,
                "us",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            (Self::NovaLite, "ca") => Ok(format!("{}.{}", region_group, model_id)),

            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeSonnet4_6
                | Self::NovaLite
                | Self::NovaPro
                | Self::Nova2Lite,
                "eu",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeSonnet4_6,
                "au",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4_5
                | Self::ClaudeSonnet4_6
                | Self::Nova2Lite,
                "jp",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::NovaLite
                | Self::NovaPro
                | Self::Nova2Lite,
                "apac",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            _ => Ok(model_id.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_us_region_inference_ids() -> anyhow::Result<()> {
        assert_eq!(
            Model::ClaudeSonnet4_5.cross_region_inference_id("us-east-1", false)?,
            "us.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(
            Model::ClaudeSonnet4.cross_region_inference_id("us-west-2", false)?,
            "us.anthropic.claude-sonnet-4-20250514-v1:0"
        );
        assert_eq!(
            Model::NovaPro.cross_region_inference_id("us-east-2", false)?,
            "us.amazon.nova-pro-v1:0"
        );
        assert_eq!(
            Model::DeepSeekR1.cross_region_inference_id("us-east-1", false)?,
            "us.deepseek.r1-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_eu_region_inference_ids() -> anyhow::Result<()> {
        assert_eq!(
            Model::ClaudeSonnet4.cross_region_inference_id("eu-west-1", false)?,
            "eu.anthropic.claude-sonnet-4-20250514-v1:0"
        );
        assert_eq!(
            Model::ClaudeSonnet4_5.cross_region_inference_id("eu-west-1", false)?,
            "eu.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(
            Model::NovaLite.cross_region_inference_id("eu-north-1", false)?,
            "eu.amazon.nova-lite-v1:0"
        );
        assert_eq!(
            Model::ClaudeOpus4_6.cross_region_inference_id("eu-west-1", false)?,
            "eu.anthropic.claude-opus-4-6-v1"
        );
        Ok(())
    }

    #[test]
    fn test_apac_region_inference_ids() -> anyhow::Result<()> {
        assert_eq!(
            Model::ClaudeSonnet4_5.cross_region_inference_id("ap-south-1", false)?,
            "apac.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(
            Model::NovaLite.cross_region_inference_id("ap-south-1", false)?,
            "apac.amazon.nova-lite-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_au_region_inference_ids() -> anyhow::Result<()> {
        assert_eq!(
            Model::ClaudeHaiku4_5.cross_region_inference_id("ap-southeast-2", false)?,
            "au.anthropic.claude-haiku-4-5-20251001-v1:0"
        );
        assert_eq!(
            Model::ClaudeSonnet4_5.cross_region_inference_id("ap-southeast-4", false)?,
            "au.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(
            Model::ClaudeOpus4_6.cross_region_inference_id("ap-southeast-2", false)?,
            "au.anthropic.claude-opus-4-6-v1"
        );
        Ok(())
    }

    #[test]
    fn test_jp_region_inference_ids() -> anyhow::Result<()> {
        assert_eq!(
            Model::ClaudeHaiku4_5.cross_region_inference_id("ap-northeast-1", false)?,
            "jp.anthropic.claude-haiku-4-5-20251001-v1:0"
        );
        assert_eq!(
            Model::ClaudeSonnet4_5.cross_region_inference_id("ap-northeast-3", false)?,
            "jp.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(
            Model::Nova2Lite.cross_region_inference_id("ap-northeast-1", false)?,
            "jp.amazon.nova-2-lite-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_ca_region_inference_ids() -> anyhow::Result<()> {
        assert_eq!(
            Model::NovaLite.cross_region_inference_id("ca-central-1", false)?,
            "ca.amazon.nova-lite-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_gov_region_inference_ids() -> anyhow::Result<()> {
        assert_eq!(
            Model::ClaudeSonnet4_5.cross_region_inference_id("us-gov-east-1", false)?,
            "us-gov.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(
            Model::ClaudeSonnet4_5.cross_region_inference_id("us-gov-west-1", false)?,
            "us-gov.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_global_inference_ids() -> anyhow::Result<()> {
        assert_eq!(
            Model::ClaudeSonnet4.cross_region_inference_id("us-east-1", true)?,
            "global.anthropic.claude-sonnet-4-20250514-v1:0"
        );
        assert_eq!(
            Model::ClaudeSonnet4_5.cross_region_inference_id("eu-west-1", true)?,
            "global.anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(
            Model::ClaudeHaiku4_5.cross_region_inference_id("ap-south-1", true)?,
            "global.anthropic.claude-haiku-4-5-20251001-v1:0"
        );
        assert_eq!(
            Model::ClaudeOpus4_6.cross_region_inference_id("us-east-1", true)?,
            "global.anthropic.claude-opus-4-6-v1"
        );
        assert_eq!(
            Model::Nova2Lite.cross_region_inference_id("us-east-1", true)?,
            "global.amazon.nova-2-lite-v1:0"
        );
        assert_eq!(
            Model::NovaPro.cross_region_inference_id("us-east-1", true)?,
            "us.amazon.nova-pro-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_models_without_cross_region() -> anyhow::Result<()> {
        assert_eq!(
            Model::Gemma3_4B.cross_region_inference_id("us-east-1", false)?,
            "google.gemma-3-4b-it"
        );
        assert_eq!(
            Model::MistralLarge3.cross_region_inference_id("eu-west-1", false)?,
            "mistral.mistral-large-3-675b-instruct"
        );
        assert_eq!(
            Model::Qwen3VL235B.cross_region_inference_id("ap-south-1", false)?,
            "qwen.qwen3-vl-235b-a22b"
        );
        assert_eq!(
            Model::GptOss120B.cross_region_inference_id("us-east-1", false)?,
            "openai.gpt-oss-120b-1:0"
        );
        assert_eq!(
            Model::MiniMaxM2.cross_region_inference_id("us-east-1", false)?,
            "minimax.minimax-m2"
        );
        assert_eq!(
            Model::KimiK2Thinking.cross_region_inference_id("us-east-1", false)?,
            "moonshot.kimi-k2-thinking"
        );
        Ok(())
    }

    #[test]
    fn test_custom_model_inference_ids() -> anyhow::Result<()> {
        let custom_model = Model::Custom {
            name: "custom.my-model-v1:0".to_string(),
            max_tokens: 100000,
            display_name: Some("My Custom Model".to_string()),
            max_output_tokens: Some(8192),
            default_temperature: Some(0.7),
            cache_configuration: None,
        };

        assert_eq!(
            custom_model.cross_region_inference_id("us-east-1", false)?,
            "custom.my-model-v1:0"
        );
        assert_eq!(
            custom_model.cross_region_inference_id("eu-west-1", true)?,
            "custom.my-model-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_friendly_id_vs_request_id() {
        assert_eq!(Model::ClaudeSonnet4_5.id(), "claude-sonnet-4-5");
        assert_eq!(Model::NovaLite.id(), "nova-lite");
        assert_eq!(Model::DeepSeekR1.id(), "deepseek-r1");
        assert_eq!(Model::Llama4Scout17B.id(), "llama-4-scout-17b");

        assert_eq!(
            Model::ClaudeSonnet4_5.request_id(),
            "anthropic.claude-sonnet-4-5-20250929-v1:0"
        );
        assert_eq!(Model::NovaLite.request_id(), "amazon.nova-lite-v1:0");
        assert_eq!(Model::DeepSeekR1.request_id(), "deepseek.r1-v1:0");
        assert_eq!(
            Model::Llama4Scout17B.request_id(),
            "meta.llama4-scout-17b-instruct-v1:0"
        );

        assert_eq!(Model::ClaudeSonnet4.id(), "claude-sonnet-4");
        assert_eq!(
            Model::from_id("claude-sonnet-4-thinking").unwrap().id(),
            "claude-sonnet-4"
        );
    }

    #[test]
    fn test_thinking_modes() {
        assert!(Model::ClaudeHaiku4_5.supports_thinking());
        assert!(Model::ClaudeSonnet4.supports_thinking());
        assert!(Model::ClaudeSonnet4_5.supports_thinking());
        assert!(Model::ClaudeOpus4_6.supports_thinking());

        assert!(!Model::ClaudeSonnet4.supports_adaptive_thinking());
        assert!(Model::ClaudeOpus4_6.supports_adaptive_thinking());
        assert!(Model::ClaudeSonnet4_6.supports_adaptive_thinking());

        assert_eq!(
            Model::ClaudeSonnet4.thinking_mode(),
            BedrockModelMode::Thinking {
                budget_tokens: Some(4096)
            }
        );
        assert_eq!(
            Model::ClaudeOpus4_6.thinking_mode(),
            BedrockModelMode::AdaptiveThinking {
                effort: BedrockAdaptiveThinkingEffort::High
            }
        );
        assert_eq!(
            Model::ClaudeHaiku4_5.thinking_mode(),
            BedrockModelMode::Thinking {
                budget_tokens: Some(4096)
            }
        );
    }

    #[test]
    fn test_max_tokens() {
        assert_eq!(Model::ClaudeSonnet4_5.max_tokens(), 200_000);
        assert_eq!(Model::ClaudeOpus4_6.max_tokens(), 200_000);
        assert_eq!(Model::Llama4Scout17B.max_tokens(), 128_000);
        assert_eq!(Model::NovaPremier.max_tokens(), 1_000_000);
    }

    #[test]
    fn test_max_output_tokens() {
        assert_eq!(Model::ClaudeSonnet4_5.max_output_tokens(), 64_000);
        assert_eq!(Model::ClaudeOpus4_6.max_output_tokens(), 128_000);
        assert_eq!(Model::ClaudeOpus4_1.max_output_tokens(), 32_000);
        assert_eq!(Model::Gemma3_4B.max_output_tokens(), 8_192);
    }

    #[test]
    fn test_supports_tool_use() {
        assert!(Model::ClaudeSonnet4_5.supports_tool_use());
        assert!(Model::NovaPro.supports_tool_use());
        assert!(Model::MistralLarge3.supports_tool_use());
        assert!(!Model::Gemma3_4B.supports_tool_use());
        assert!(Model::Qwen3_32B.supports_tool_use());
        assert!(Model::MiniMaxM2.supports_tool_use());
        assert!(Model::KimiK2_5.supports_tool_use());
        assert!(Model::DeepSeekR1.supports_tool_use());
        assert!(!Model::Llama4Scout17B.supports_tool_use());
    }

    #[test]
    fn test_supports_caching() {
        assert!(Model::ClaudeSonnet4_5.supports_caching());
        assert!(Model::ClaudeOpus4_6.supports_caching());
        assert!(!Model::Llama4Scout17B.supports_caching());
        assert!(!Model::NovaPro.supports_caching());
    }

    #[test]
    fn test_from_id_all_models() {
        assert_eq!(Model::from_id("nova-lite").unwrap().id(), "nova-lite");
        assert_eq!(Model::from_id("deepseek-r1").unwrap().id(), "deepseek-r1");
        assert_eq!(
            Model::from_id("llama-4-scout-17b").unwrap().id(),
            "llama-4-scout-17b"
        );
        assert_eq!(Model::from_id("qwen3-32b").unwrap().id(), "qwen3-32b");
        assert_eq!(Model::from_id("minimax-m2").unwrap().id(), "minimax-m2");
        assert_eq!(
            Model::from_id("claude-sonnet-4-5-thinking").unwrap().id(),
            "claude-sonnet-4-5"
        );
        assert_eq!(
            Model::from_id("claude-opus-4-6-latest").unwrap().id(),
            "claude-opus-4-6"
        );
        assert!(Model::from_id("nonexistent-model").is_err());
    }

    #[test]
    fn test_capabilities_round_trip() {
        let caps = Model::ClaudeOpus4_6.capabilities();
        assert_eq!(caps.max_tokens, 200_000);
        assert_eq!(caps.max_output_tokens, 128_000);
        assert!(caps.supports_adaptive_thinking);
        assert!(caps.supports_thinking);
        assert!(caps.cache_configuration.is_some());
        assert_eq!(caps.extended_context_token_count, Some(1_000_000));
    }

    #[test]
    fn test_custom_model_capabilities() {
        let custom = Model::Custom {
            name: "my-model".to_string(),
            max_tokens: 50_000,
            display_name: Some("My Model".to_string()),
            max_output_tokens: Some(8192),
            default_temperature: Some(0.5),
            cache_configuration: Some(BedrockModelCacheConfiguration {
                max_cache_anchors: 2,
                min_total_token: 512,
            }),
        };
        let caps = custom.capabilities();
        assert_eq!(caps.max_tokens, 50_000);
        assert_eq!(caps.max_output_tokens, 8192);
        assert_eq!(caps.default_temperature, 0.5);
        assert!(!caps.supports_tool_use);
        assert!(caps.cache_configuration.is_some());
    }

    #[test]
    fn test_adaptive_thinking_effort_from_str() {
        assert_eq!(
            "low".parse::<BedrockAdaptiveThinkingEffort>().unwrap(),
            BedrockAdaptiveThinkingEffort::Low
        );
        assert_eq!(
            "medium".parse::<BedrockAdaptiveThinkingEffort>().unwrap(),
            BedrockAdaptiveThinkingEffort::Medium
        );
        assert_eq!(
            "high".parse::<BedrockAdaptiveThinkingEffort>().unwrap(),
            BedrockAdaptiveThinkingEffort::High
        );
        assert_eq!(
            "max".parse::<BedrockAdaptiveThinkingEffort>().unwrap(),
            BedrockAdaptiveThinkingEffort::Max
        );
        assert!("invalid".parse::<BedrockAdaptiveThinkingEffort>().is_err());
    }
}
