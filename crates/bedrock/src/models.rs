use serde::{Deserialize, Serialize};
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

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BedrockModelCacheConfiguration {
    pub max_cache_anchors: usize,
    pub min_total_token: u64,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    // Anthropic Claude 4+ models
    #[serde(rename = "claude-haiku-4-5", alias = "claude-haiku-4-5-latest")]
    ClaudeHaiku4_5,
    #[serde(
        rename = "claude-sonnet-4",
        alias = "claude-sonnet-4-latest",
        alias = "claude-sonnet-4-thinking",
        alias = "claude-sonnet-4-thinking-latest"
    )]
    ClaudeSonnet4,
    #[default]
    #[serde(
        rename = "claude-sonnet-4-5",
        alias = "claude-sonnet-4-5-latest",
        alias = "claude-sonnet-4-5-thinking",
        alias = "claude-sonnet-4-5-thinking-latest"
    )]
    ClaudeSonnet4_5,
    #[serde(
        rename = "claude-opus-4-1",
        alias = "claude-opus-4-1-latest",
        alias = "claude-opus-4-1-thinking",
        alias = "claude-opus-4-1-thinking-latest"
    )]
    ClaudeOpus4_1,
    #[serde(
        rename = "claude-opus-4-5",
        alias = "claude-opus-4-5-latest",
        alias = "claude-opus-4-5-thinking",
        alias = "claude-opus-4-5-thinking-latest"
    )]
    ClaudeOpus4_5,
    #[serde(
        rename = "claude-opus-4-6",
        alias = "claude-opus-4-6-latest",
        alias = "claude-opus-4-6-thinking",
        alias = "claude-opus-4-6-thinking-latest"
    )]
    ClaudeOpus4_6,
    #[serde(
        rename = "claude-opus-4-7",
        alias = "claude-opus-4-7-latest",
        alias = "claude-opus-4-7-thinking",
        alias = "claude-opus-4-7-thinking-latest"
    )]
    ClaudeOpus4_7,
    #[serde(
        rename = "claude-sonnet-4-6",
        alias = "claude-sonnet-4-6-latest",
        alias = "claude-sonnet-4-6-thinking",
        alias = "claude-sonnet-4-6-thinking-latest"
    )]
    ClaudeSonnet4_6,

    // Meta Llama 4 models
    #[serde(rename = "llama-4-scout-17b")]
    Llama4Scout17B,
    #[serde(rename = "llama-4-maverick-17b")]
    Llama4Maverick17B,

    // Google Gemma 3 models
    #[serde(rename = "gemma-3-4b")]
    Gemma3_4B,
    #[serde(rename = "gemma-3-12b")]
    Gemma3_12B,
    #[serde(rename = "gemma-3-27b")]
    Gemma3_27B,

    // Mistral models
    #[serde(rename = "magistral-small")]
    MagistralSmall,
    #[serde(rename = "mistral-large-3")]
    MistralLarge3,
    #[serde(rename = "pixtral-large")]
    PixtralLarge,
    #[serde(rename = "devstral-2-123b")]
    Devstral2_123B,
    #[serde(rename = "ministral-14b")]
    Ministral14B,

    // Qwen models
    #[serde(rename = "qwen3-32b")]
    Qwen3_32B,
    #[serde(rename = "qwen3-vl-235b")]
    Qwen3VL235B,
    #[serde(rename = "qwen3-235b")]
    Qwen3_235B,
    #[serde(rename = "qwen3-next-80b")]
    Qwen3Next80B,
    #[serde(rename = "qwen3-coder-30b")]
    Qwen3Coder30B,
    #[serde(rename = "qwen3-coder-next")]
    Qwen3CoderNext,
    #[serde(rename = "qwen3-coder-480b")]
    Qwen3Coder480B,

    // Amazon Nova models
    #[serde(rename = "nova-lite")]
    NovaLite,
    #[serde(rename = "nova-pro")]
    NovaPro,
    #[serde(rename = "nova-premier")]
    NovaPremier,
    #[serde(rename = "nova-2-lite")]
    Nova2Lite,

    // OpenAI GPT OSS models
    #[serde(rename = "gpt-oss-20b")]
    GptOss20B,
    #[serde(rename = "gpt-oss-120b")]
    GptOss120B,

    // NVIDIA Nemotron models
    #[serde(rename = "nemotron-super-3-120b")]
    NemotronSuper3_120B,
    #[serde(rename = "nemotron-nano-3-30b")]
    NemotronNano3_30B,

    // MiniMax models
    #[serde(rename = "minimax-m2")]
    MiniMaxM2,
    #[serde(rename = "minimax-m2-1")]
    MiniMaxM2_1,
    #[serde(rename = "minimax-m2-5")]
    MiniMaxM2_5,

    // Z.AI GLM models
    #[serde(rename = "glm-5")]
    GLM5,
    #[serde(rename = "glm-4-7")]
    GLM4_7,
    #[serde(rename = "glm-4-7-flash")]
    GLM4_7Flash,

    // Moonshot models
    #[serde(rename = "kimi-k2-thinking")]
    KimiK2Thinking,
    #[serde(rename = "kimi-k2-5")]
    KimiK2_5,

    // DeepSeek models
    #[serde(rename = "deepseek-r1")]
    DeepSeekR1,
    #[serde(rename = "deepseek-v3")]
    DeepSeekV3_1,
    #[serde(rename = "deepseek-v3-2")]
    DeepSeekV3_2,

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
    pub fn default_fast(_region: &str) -> Self {
        Self::ClaudeHaiku4_5
    }

    pub fn from_id(id: &str) -> anyhow::Result<Self> {
        if id.starts_with("claude-opus-4-7") {
            Ok(Self::ClaudeOpus4_7)
        } else if id.starts_with("claude-opus-4-6") {
            Ok(Self::ClaudeOpus4_6)
        } else if id.starts_with("claude-opus-4-5") {
            Ok(Self::ClaudeOpus4_5)
        } else if id.starts_with("claude-opus-4-1") {
            Ok(Self::ClaudeOpus4_1)
        } else if id.starts_with("claude-sonnet-4-6") {
            Ok(Self::ClaudeSonnet4_6)
        } else if id.starts_with("claude-sonnet-4-5") {
            Ok(Self::ClaudeSonnet4_5)
        } else if id.starts_with("claude-sonnet-4") {
            Ok(Self::ClaudeSonnet4)
        } else if id.starts_with("claude-haiku-4-5") {
            Ok(Self::ClaudeHaiku4_5)
        } else {
            anyhow::bail!("invalid model id {id}");
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::ClaudeHaiku4_5 => "claude-haiku-4-5",
            Self::ClaudeSonnet4 => "claude-sonnet-4",
            Self::ClaudeSonnet4_5 => "claude-sonnet-4-5",
            Self::ClaudeOpus4_1 => "claude-opus-4-1",
            Self::ClaudeOpus4_5 => "claude-opus-4-5",
            Self::ClaudeOpus4_6 => "claude-opus-4-6",
            Self::ClaudeOpus4_7 => "claude-opus-4-7",
            Self::ClaudeSonnet4_6 => "claude-sonnet-4-6",
            Self::Llama4Scout17B => "llama-4-scout-17b",
            Self::Llama4Maverick17B => "llama-4-maverick-17b",
            Self::Gemma3_4B => "gemma-3-4b",
            Self::Gemma3_12B => "gemma-3-12b",
            Self::Gemma3_27B => "gemma-3-27b",
            Self::MagistralSmall => "magistral-small",
            Self::MistralLarge3 => "mistral-large-3",
            Self::PixtralLarge => "pixtral-large",
            Self::Devstral2_123B => "devstral-2-123b",
            Self::Ministral14B => "ministral-14b",
            Self::Qwen3_32B => "qwen3-32b",
            Self::Qwen3VL235B => "qwen3-vl-235b",
            Self::Qwen3_235B => "qwen3-235b",
            Self::Qwen3Next80B => "qwen3-next-80b",
            Self::Qwen3Coder30B => "qwen3-coder-30b",
            Self::Qwen3CoderNext => "qwen3-coder-next",
            Self::Qwen3Coder480B => "qwen3-coder-480b",
            Self::NovaLite => "nova-lite",
            Self::NovaPro => "nova-pro",
            Self::NovaPremier => "nova-premier",
            Self::Nova2Lite => "nova-2-lite",
            Self::GptOss20B => "gpt-oss-20b",
            Self::GptOss120B => "gpt-oss-120b",
            Self::NemotronSuper3_120B => "nemotron-super-3-120b",
            Self::NemotronNano3_30B => "nemotron-nano-3-30b",
            Self::MiniMaxM2 => "minimax-m2",
            Self::MiniMaxM2_1 => "minimax-m2-1",
            Self::MiniMaxM2_5 => "minimax-m2-5",
            Self::GLM5 => "glm-5",
            Self::GLM4_7 => "glm-4-7",
            Self::GLM4_7Flash => "glm-4-7-flash",
            Self::KimiK2Thinking => "kimi-k2-thinking",
            Self::KimiK2_5 => "kimi-k2-5",
            Self::DeepSeekR1 => "deepseek-r1",
            Self::DeepSeekV3_1 => "deepseek-v3",
            Self::DeepSeekV3_2 => "deepseek-v3-2",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn request_id(&self) -> &str {
        match self {
            Self::ClaudeHaiku4_5 => "anthropic.claude-haiku-4-5-20251001-v1:0",
            Self::ClaudeSonnet4 => "anthropic.claude-sonnet-4-20250514-v1:0",
            Self::ClaudeSonnet4_5 => "anthropic.claude-sonnet-4-5-20250929-v1:0",
            Self::ClaudeOpus4_1 => "anthropic.claude-opus-4-1-20250805-v1:0",
            Self::ClaudeOpus4_5 => "anthropic.claude-opus-4-5-20251101-v1:0",
            Self::ClaudeOpus4_6 => "anthropic.claude-opus-4-6-v1",
            Self::ClaudeOpus4_7 => "anthropic.claude-opus-4-7-v1",
            Self::ClaudeSonnet4_6 => "anthropic.claude-sonnet-4-6",
            Self::Llama4Scout17B => "meta.llama4-scout-17b-instruct-v1:0",
            Self::Llama4Maverick17B => "meta.llama4-maverick-17b-instruct-v1:0",
            Self::Gemma3_4B => "google.gemma-3-4b-it",
            Self::Gemma3_12B => "google.gemma-3-12b-it",
            Self::Gemma3_27B => "google.gemma-3-27b-it",
            Self::MagistralSmall => "mistral.magistral-small-2509",
            Self::MistralLarge3 => "mistral.mistral-large-3-675b-instruct",
            Self::PixtralLarge => "mistral.pixtral-large-2502-v1:0",
            Self::Devstral2_123B => "mistral.devstral-2-123b",
            Self::Ministral14B => "mistral.ministral-3-14b-instruct",
            Self::Qwen3VL235B => "qwen.qwen3-vl-235b-a22b",
            Self::Qwen3_32B => "qwen.qwen3-32b-v1:0",
            Self::Qwen3_235B => "qwen.qwen3-235b-a22b-2507-v1:0",
            Self::Qwen3Next80B => "qwen.qwen3-next-80b-a3b",
            Self::Qwen3Coder30B => "qwen.qwen3-coder-30b-a3b-v1:0",
            Self::Qwen3CoderNext => "qwen.qwen3-coder-next",
            Self::Qwen3Coder480B => "qwen.qwen3-coder-480b-a35b-v1:0",
            Self::NovaLite => "amazon.nova-lite-v1:0",
            Self::NovaPro => "amazon.nova-pro-v1:0",
            Self::NovaPremier => "amazon.nova-premier-v1:0",
            Self::Nova2Lite => "amazon.nova-2-lite-v1:0",
            Self::GptOss20B => "openai.gpt-oss-20b-1:0",
            Self::GptOss120B => "openai.gpt-oss-120b-1:0",
            Self::NemotronSuper3_120B => "nvidia.nemotron-super-3-120b",
            Self::NemotronNano3_30B => "nvidia.nemotron-nano-3-30b",
            Self::MiniMaxM2 => "minimax.minimax-m2",
            Self::MiniMaxM2_1 => "minimax.minimax-m2.1",
            Self::MiniMaxM2_5 => "minimax.minimax-m2.5",
            Self::GLM5 => "zai.glm-5",
            Self::GLM4_7 => "zai.glm-4.7",
            Self::GLM4_7Flash => "zai.glm-4.7-flash",
            Self::KimiK2Thinking => "moonshot.kimi-k2-thinking",
            Self::KimiK2_5 => "moonshotai.kimi-k2.5",
            Self::DeepSeekR1 => "deepseek.r1-v1:0",
            Self::DeepSeekV3_1 => "deepseek.v3-v1:0",
            Self::DeepSeekV3_2 => "deepseek.v3.2",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::ClaudeHaiku4_5 => "Claude Haiku 4.5",
            Self::ClaudeSonnet4 => "Claude Sonnet 4",
            Self::ClaudeSonnet4_5 => "Claude Sonnet 4.5",
            Self::ClaudeOpus4_1 => "Claude Opus 4.1",
            Self::ClaudeOpus4_5 => "Claude Opus 4.5",
            Self::ClaudeOpus4_6 => "Claude Opus 4.6",
            Self::ClaudeOpus4_7 => "Claude Opus 4.7",
            Self::ClaudeSonnet4_6 => "Claude Sonnet 4.6",
            Self::Llama4Scout17B => "Llama 4 Scout 17B",
            Self::Llama4Maverick17B => "Llama 4 Maverick 17B",
            Self::Gemma3_4B => "Gemma 3 4B",
            Self::Gemma3_12B => "Gemma 3 12B",
            Self::Gemma3_27B => "Gemma 3 27B",
            Self::MagistralSmall => "Magistral Small",
            Self::MistralLarge3 => "Mistral Large 3",
            Self::PixtralLarge => "Pixtral Large",
            Self::Devstral2_123B => "Devstral 2 123B",
            Self::Ministral14B => "Ministral 14B",
            Self::Qwen3VL235B => "Qwen3 VL 235B",
            Self::Qwen3_32B => "Qwen3 32B",
            Self::Qwen3_235B => "Qwen3 235B",
            Self::Qwen3Next80B => "Qwen3 Next 80B",
            Self::Qwen3Coder30B => "Qwen3 Coder 30B",
            Self::Qwen3CoderNext => "Qwen3 Coder Next",
            Self::Qwen3Coder480B => "Qwen3 Coder 480B",
            Self::NovaLite => "Amazon Nova Lite",
            Self::NovaPro => "Amazon Nova Pro",
            Self::NovaPremier => "Amazon Nova Premier",
            Self::Nova2Lite => "Amazon Nova 2 Lite",
            Self::GptOss20B => "GPT OSS 20B",
            Self::GptOss120B => "GPT OSS 120B",
            Self::NemotronSuper3_120B => "Nemotron Super 3 120B",
            Self::NemotronNano3_30B => "Nemotron Nano 3 30B",
            Self::MiniMaxM2 => "MiniMax M2",
            Self::MiniMaxM2_1 => "MiniMax M2.1",
            Self::MiniMaxM2_5 => "MiniMax M2.5",
            Self::GLM5 => "GLM 5",
            Self::GLM4_7 => "GLM 4.7",
            Self::GLM4_7Flash => "GLM 4.7 Flash",
            Self::KimiK2Thinking => "Kimi K2 Thinking",
            Self::KimiK2_5 => "Kimi K2.5",
            Self::DeepSeekR1 => "DeepSeek R1",
            Self::DeepSeekV3_1 => "DeepSeek V3.1",
            Self::DeepSeekV3_2 => "DeepSeek V3.2",
            Self::Custom {
                display_name, name, ..
            } => display_name.as_deref().unwrap_or(name.as_str()),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        self.max_tokens()
    }

    pub fn max_tokens(&self) -> u64 {
        match self {
            Self::ClaudeHaiku4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_7
            | Self::ClaudeSonnet4_6 => 200_000,
            Self::Llama4Scout17B | Self::Llama4Maverick17B => 128_000,
            Self::Gemma3_4B | Self::Gemma3_12B | Self::Gemma3_27B => 128_000,
            Self::MagistralSmall | Self::MistralLarge3 | Self::PixtralLarge => 128_000,
            Self::Devstral2_123B | Self::Ministral14B => 256_000,
            Self::Qwen3_32B
            | Self::Qwen3VL235B
            | Self::Qwen3_235B
            | Self::Qwen3Next80B
            | Self::Qwen3Coder30B
            | Self::Qwen3CoderNext
            | Self::Qwen3Coder480B => 128_000,
            Self::NovaLite | Self::NovaPro => 300_000,
            Self::NovaPremier => 1_000_000,
            Self::Nova2Lite => 300_000,
            Self::GptOss20B | Self::GptOss120B => 128_000,
            Self::NemotronSuper3_120B | Self::NemotronNano3_30B => 262_000,
            Self::MiniMaxM2 | Self::MiniMaxM2_1 | Self::MiniMaxM2_5 => 196_000,
            Self::GLM5 | Self::GLM4_7 | Self::GLM4_7Flash => 203_000,
            Self::KimiK2Thinking | Self::KimiK2_5 => 128_000,
            Self::DeepSeekR1 | Self::DeepSeekV3_1 | Self::DeepSeekV3_2 => 128_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> u64 {
        match self {
            Self::ClaudeHaiku4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeOpus4_5
            | Self::ClaudeSonnet4_6 => 64_000,
            Self::ClaudeOpus4_1 => 32_000,
            Self::ClaudeOpus4_6 | Self::ClaudeOpus4_7 => 128_000,
            Self::Llama4Scout17B
            | Self::Llama4Maverick17B
            | Self::Gemma3_4B
            | Self::Gemma3_12B
            | Self::Gemma3_27B
            | Self::MagistralSmall
            | Self::MistralLarge3
            | Self::PixtralLarge => 8_192,
            Self::Devstral2_123B | Self::Ministral14B => 131_000,
            Self::Qwen3_32B
            | Self::Qwen3VL235B
            | Self::Qwen3_235B
            | Self::Qwen3Next80B
            | Self::Qwen3Coder30B
            | Self::Qwen3CoderNext
            | Self::Qwen3Coder480B => 8_192,
            Self::NovaLite | Self::NovaPro | Self::NovaPremier | Self::Nova2Lite => 5_000,
            Self::GptOss20B | Self::GptOss120B => 16_000,
            Self::NemotronSuper3_120B | Self::NemotronNano3_30B => 131_000,
            Self::MiniMaxM2 | Self::MiniMaxM2_1 | Self::MiniMaxM2_5 => 98_000,
            Self::GLM5 | Self::GLM4_7 | Self::GLM4_7Flash => 101_000,
            Self::KimiK2Thinking | Self::KimiK2_5 => 16_000,
            Self::DeepSeekR1 | Self::DeepSeekV3_1 | Self::DeepSeekV3_2 => 16_000,
            Self::Custom {
                max_output_tokens, ..
            } => max_output_tokens.unwrap_or(4_096),
        }
    }

    pub fn default_temperature(&self) -> f32 {
        match self {
            Self::ClaudeHaiku4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_7
            | Self::ClaudeSonnet4_6 => 1.0,
            Self::Custom {
                default_temperature,
                ..
            } => default_temperature.unwrap_or(1.0),
            _ => 1.0,
        }
    }

    pub fn supports_tool_use(&self) -> bool {
        match self {
            Self::ClaudeHaiku4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_7
            | Self::ClaudeSonnet4_6 => true,
            Self::NovaLite | Self::NovaPro | Self::NovaPremier | Self::Nova2Lite => true,
            Self::MistralLarge3 | Self::PixtralLarge | Self::MagistralSmall => true,
            Self::Devstral2_123B | Self::Ministral14B => true,
            // Gemma accepts toolConfig without error but produces unreliable tool
            // calls -- malformed JSON args, hallucinated tool names, dropped calls.
            Self::Qwen3_32B
            | Self::Qwen3VL235B
            | Self::Qwen3_235B
            | Self::Qwen3Next80B
            | Self::Qwen3Coder30B
            | Self::Qwen3CoderNext
            | Self::Qwen3Coder480B => true,
            Self::MiniMaxM2 | Self::MiniMaxM2_1 | Self::MiniMaxM2_5 => true,
            Self::NemotronSuper3_120B | Self::NemotronNano3_30B => true,
            Self::GLM5 | Self::GLM4_7 | Self::GLM4_7Flash => true,
            Self::KimiK2Thinking | Self::KimiK2_5 => true,
            Self::DeepSeekR1 | Self::DeepSeekV3_1 | Self::DeepSeekV3_2 => true,
            _ => false,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            Self::ClaudeHaiku4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_7
            | Self::ClaudeSonnet4_6 => true,
            Self::NovaLite | Self::NovaPro => true,
            Self::PixtralLarge => true,
            Self::Qwen3VL235B => true,
            Self::KimiK2_5 => true,
            _ => false,
        }
    }

    pub fn supports_extended_context(&self) -> bool {
        matches!(
            self,
            Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeOpus4_7
                | Self::ClaudeSonnet4_6
        )
    }

    pub fn supports_caching(&self) -> bool {
        match self {
            Self::ClaudeHaiku4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_7
            | Self::ClaudeSonnet4_6 => true,
            Self::Custom {
                cache_configuration,
                ..
            } => cache_configuration.is_some(),
            _ => false,
        }
    }

    pub fn cache_configuration(&self) -> Option<BedrockModelCacheConfiguration> {
        match self {
            Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_7
            | Self::ClaudeSonnet4_6 => Some(BedrockModelCacheConfiguration {
                max_cache_anchors: 4,
                min_total_token: 1024,
            }),
            Self::ClaudeHaiku4_5 => Some(BedrockModelCacheConfiguration {
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

    pub fn supports_thinking(&self) -> bool {
        matches!(
            self,
            Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_1
                | Self::ClaudeOpus4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeOpus4_7
                | Self::ClaudeSonnet4_6
        )
    }

    pub fn supports_adaptive_thinking(&self) -> bool {
        matches!(
            self,
            Self::ClaudeOpus4_6 | Self::ClaudeOpus4_7 | Self::ClaudeSonnet4_6
        )
    }

    pub fn thinking_mode(&self) -> BedrockModelMode {
        if self.supports_adaptive_thinking() {
            BedrockModelMode::AdaptiveThinking {
                effort: BedrockAdaptiveThinkingEffort::default(),
            }
        } else if self.supports_thinking() {
            BedrockModelMode::Thinking {
                budget_tokens: Some(4096),
            }
        } else {
            BedrockModelMode::Default
        }
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
                | Self::ClaudeOpus4_7
                | Self::ClaudeSonnet4_6
                | Self::Nova2Lite
        );

        // Determine region group based on AWS region
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
            // Australia
            if allow_global && supports_global {
                "global"
            } else {
                "au"
            }
        } else if region == "ap-northeast-1" || region == "ap-northeast-3" {
            // Japan
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

            // Global inference profiles
            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeOpus4_7
                | Self::ClaudeSonnet4_6
                | Self::Nova2Lite,
                "global",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // US Government region inference profiles
            (Self::ClaudeSonnet4_5, "us-gov") => Ok(format!("{}.{}", region_group, model_id)),

            // US region inference profiles
            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_1
                | Self::ClaudeOpus4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeOpus4_7
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

            // Canada region inference profiles
            (Self::NovaLite, "ca") => Ok(format!("{}.{}", region_group, model_id)),

            // EU region inference profiles
            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeOpus4_7
                | Self::ClaudeSonnet4_6
                | Self::NovaLite
                | Self::NovaPro
                | Self::Nova2Lite,
                "eu",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // Australia region inference profiles
            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeOpus4_7
                | Self::ClaudeSonnet4_6,
                "au",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // Japan region inference profiles
            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4_5
                | Self::ClaudeSonnet4_6
                | Self::Nova2Lite,
                "jp",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // APAC region inference profiles (other than AU/JP)
            (
                Self::ClaudeHaiku4_5
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::NovaLite
                | Self::NovaPro
                | Self::Nova2Lite,
                "apac",
            ) => Ok(format!("{}.{}", region_group, model_id)),

            // Default: use model ID directly
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
        assert_eq!(
            Model::ClaudeOpus4_7.cross_region_inference_id("eu-west-1", false)?,
            "eu.anthropic.claude-opus-4-7-v1"
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
        assert_eq!(
            Model::ClaudeOpus4_7.cross_region_inference_id("ap-southeast-2", false)?,
            "au.anthropic.claude-opus-4-7-v1"
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
            Model::ClaudeOpus4_7.cross_region_inference_id("us-east-1", true)?,
            "global.anthropic.claude-opus-4-7-v1"
        );
        assert_eq!(
            Model::Nova2Lite.cross_region_inference_id("us-east-1", true)?,
            "global.amazon.nova-2-lite-v1:0"
        );

        // Models without global support fall back to regional
        assert_eq!(
            Model::NovaPro.cross_region_inference_id("us-east-1", true)?,
            "us.amazon.nova-pro-v1:0"
        );
        Ok(())
    }

    #[test]
    fn test_models_without_cross_region() -> anyhow::Result<()> {
        // Models without cross-region support return their request_id directly
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

        // Thinking aliases deserialize to the same model
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
}
