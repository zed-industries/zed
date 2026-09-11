use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
};
use language_model_core::ReasoningEffort;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const OPENCODE_API_URL: &str = "https://opencode.ai/zen";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ApiProtocol {
    #[default]
    Anthropic,
    Google,
    OpenAiResponses,
    OpenAiChat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum OpenCodeSubscription {
    Zen,
    Go,
}

impl OpenCodeSubscription {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Zen => "Zen",
            Self::Go => "Go",
        }
    }

    pub fn id_prefix(&self) -> &'static str {
        match self {
            Self::Zen => "zen",
            Self::Go => "go",
        }
    }

    pub fn api_path_suffix(&self) -> &'static str {
        match self {
            Self::Zen => "",
            Self::Go => "/go",
        }
    }
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    // -- Anthropic models --
    #[serde(rename = "claude-opus-5")]
    ClaudeOpus5,
    #[serde(rename = "claude-opus-4-8")]
    ClaudeOpus4_8,
    #[serde(rename = "claude-opus-4-7")]
    ClaudeOpus4_7,
    #[serde(rename = "claude-opus-4-6")]
    ClaudeOpus4_6,
    #[serde(rename = "claude-opus-4-5")]
    ClaudeOpus4_5,
    #[serde(rename = "claude-sonnet-5")]
    ClaudeSonnet5,
    #[default]
    #[serde(rename = "claude-sonnet-4-6")]
    ClaudeSonnet4_6,
    #[serde(rename = "claude-sonnet-4-5")]
    ClaudeSonnet4_5,
    #[serde(rename = "claude-sonnet-4")]
    ClaudeSonnet4,
    #[serde(rename = "claude-haiku-4-5")]
    ClaudeHaiku4_5,
    #[serde(rename = "claude-fable-5-1")]
    ClaudeFable5_1,
    #[serde(rename = "claude-fable-5")]
    ClaudeFable5,

    // -- OpenAI models --
    #[serde(rename = "gpt-6-astra")]
    Gpt6Astra,
    #[serde(rename = "gpt-5.6-sol")]
    Gpt5_6Sol,
    #[serde(rename = "gpt-5.6-terra")]
    Gpt5_6Terra,
    #[serde(rename = "gpt-5.6-luna")]
    Gpt5_6Luna,
    #[serde(rename = "gpt-5.5")]
    Gpt5_5,
    #[serde(rename = "gpt-5.5-pro")]
    Gpt5_5Pro,
    #[serde(rename = "gpt-5.4")]
    Gpt5_4,
    #[serde(rename = "gpt-5.4-pro")]
    Gpt5_4Pro,
    #[serde(rename = "gpt-5.4-mini")]
    Gpt5_4Mini,
    #[serde(rename = "gpt-5.4-nano")]
    Gpt5_4Nano,
    #[serde(rename = "gpt-5.3-codex")]
    Gpt5_3Codex,
    #[serde(rename = "gpt-5.3-codex-spark")]
    Gpt5_3Spark,
    #[serde(rename = "gpt-5.2")]
    Gpt5_2,
    #[serde(rename = "gpt-5.2-codex")]
    Gpt5_2Codex,
    #[serde(rename = "gpt-5.1")]
    Gpt5_1,
    #[serde(rename = "gpt-5.1-codex")]
    Gpt5_1Codex,
    #[serde(rename = "gpt-5.1-codex-max")]
    Gpt5_1CodexMax,
    #[serde(rename = "gpt-5.1-codex-mini")]
    Gpt5_1CodexMini,
    #[serde(rename = "gpt-5")]
    Gpt5,
    #[serde(rename = "gpt-5-codex")]
    Gpt5Codex,
    #[serde(rename = "gpt-5-nano")]
    Gpt5Nano,

    // -- Google models --
    #[serde(rename = "gemini-3.1-pro")]
    Gemini3_1Pro,
    #[serde(rename = "gemini-3.8-flash")]
    Gemini3_8Flash,
    #[serde(rename = "gemini-3.7-flash")]
    Gemini3_7Flash,
    #[serde(rename = "gemini-3.6-flash")]
    Gemini3_6Flash,
    #[serde(rename = "gemini-3.5-flash")]
    Gemini3_5Flash,
    #[serde(rename = "gemini-3-flash")]
    Gemini3Flash,
    #[serde(rename = "gemini-3.5-flash-lite")]
    Gemini3_5FlashLite,

    // -- Meta models --
    #[serde(rename = "muse-spark-1.3")]
    MuseSpark1_3,
    #[serde(rename = "muse-spark-1.2")]
    MuseSpark1_2,
    #[serde(rename = "muse-spark-1.3-contributor")]
    MuseSpark1_3Contributor,
    #[serde(rename = "muse-spark-1.2-contributor")]
    MuseSpark1_2Contributor,

    // -- Alibaba models --
    #[serde(rename = "qwen3.8-max")]
    Qwen3_8Max,
    #[serde(rename = "qwen3.8-flash")]
    Qwen3_8Flash,
    #[serde(rename = "qwen3.7-max")]
    Qwen3_7Max,
    #[serde(rename = "qwen3.7-plus")]
    Qwen3_7Plus,
    #[serde(rename = "qwen3.6-plus")]
    Qwen3_6Plus,
    #[serde(rename = "qwen3.5-plus")]
    Qwen3_5Plus,

    // -- xAI models --
    #[serde(rename = "grok-4.6")]
    Grok4_6,
    #[serde(rename = "grok-4.5")]
    Grok4_5,
    #[serde(rename = "grok-build-0.1")]
    GrokBuild0_1,

    // -- Z.ai models --
    #[serde(rename = "glm-5.3")]
    Glm5_3,
    #[serde(rename = "glm-5.3-flash")]
    Glm5_3Flash,
    #[serde(rename = "glm-5.2")]
    Glm5_2,
    #[serde(rename = "glm-5.1")]
    Glm5_1,
    #[serde(rename = "glm-5")]
    Glm5,

    // -- Moonshot AI models --
    #[serde(rename = "kimi-k3")]
    KimiK3,
    #[serde(rename = "kimi-k2.7-code")]
    KimiK2_7Code,
    #[serde(rename = "kimi-k2.6")]
    KimiK2_6,
    #[serde(rename = "kimi-k2.5")]
    KimiK2_5,

    // -- DeepSeek models --
    #[serde(rename = "deepseek-v4.1-flash")]
    DeepSeekV4_1Flash,
    #[serde(rename = "deepseek-v4-pro")]
    DeepSeekV4Pro,
    #[serde(rename = "deepseek-v4-flash-vision-exp")]
    DeepSeekV4FlashVisionExp,
    #[serde(rename = "deepseek-v4-flash")]
    DeepSeekV4Flash,

    // -- Minimax Group models --
    #[serde(rename = "minimax-m3")]
    MiniMaxM3,
    #[serde(rename = "minimax-m2.7")]
    MiniMaxM2_7,
    #[serde(rename = "minimax-m2.5")]
    MiniMaxM2_5,

    // -- Others models --
    #[serde(rename = "mimo-v2.5-pro")]
    MimoV2_5Pro,
    #[serde(rename = "mimo-v2.5")]
    MimoV2_5,
    #[serde(rename = "hy4-preview")]
    Hy4Preview,
    #[serde(rename = "hy3")]
    Hy3,
    #[serde(rename = "longcat-2.0")]
    LongCat2_0,

    // -- Custom model --
    #[serde(rename = "custom")]
    Custom {
        name: String,
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        protocol: ApiProtocol,
        reasoning_effort_levels: Option<Vec<ReasoningEffort>>,
        custom_model_api_url: Option<String>,
        interleaved_reasoning: bool,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::ClaudeHaiku4_5
    }

    pub fn default_go() -> Self {
        Self::KimiK2_6
    }

    pub fn default_go_fast() -> Self {
        Self::MiniMaxM2_7
    }

    pub fn available_subscriptions(&self) -> &'static [OpenCodeSubscription] {
        match self {
            // Models available in both Zen and Go
            Self::Gpt5_6Luna
            | Self::Qwen3_6Plus
            | Self::Grok4_6
            | Self::Glm5_3
            | Self::Glm5_3Flash
            | Self::Glm5_2
            | Self::Glm5_1
            | Self::KimiK3
            | Self::KimiK2_7Code
            | Self::KimiK2_6
            | Self::DeepSeekV4Pro
            | Self::DeepSeekV4FlashVisionExp
            | Self::DeepSeekV4Flash
            | Self::MiniMaxM3
            | Self::MiniMaxM2_7 => &[OpenCodeSubscription::Zen, OpenCodeSubscription::Go],

            // Go-only models
            Self::MuseSpark1_3Contributor
            | Self::MuseSpark1_2Contributor
            | Self::Qwen3_8Max
            | Self::Qwen3_8Flash
            | Self::Qwen3_7Max
            | Self::Qwen3_7Plus
            | Self::DeepSeekV4_1Flash
            | Self::MimoV2_5Pro
            | Self::MimoV2_5
            | Self::Hy4Preview
            | Self::Hy3
            | Self::LongCat2_0 => &[OpenCodeSubscription::Go],

            // Deprecated on Go (per models.dev); still offered on Zen
            Self::Qwen3_5Plus | Self::Grok4_5 | Self::Glm5 | Self::KimiK2_5 | Self::MiniMaxM2_5 => {
                &[OpenCodeSubscription::Zen]
            }

            // Custom models get their subscription from settings, not from here
            Self::Custom { .. } => &[],

            // All other built-in models are Zen-only
            _ => &[OpenCodeSubscription::Zen],
        }
    }

    pub fn id(&self) -> &str {
        match self {
            // -- Anthropic models --
            Self::ClaudeOpus5 => "claude-opus-5",
            Self::ClaudeOpus4_8 => "claude-opus-4-8",
            Self::ClaudeOpus4_7 => "claude-opus-4-7",
            Self::ClaudeOpus4_6 => "claude-opus-4-6",
            Self::ClaudeOpus4_5 => "claude-opus-4-5",
            Self::ClaudeSonnet5 => "claude-sonnet-5",
            Self::ClaudeSonnet4_6 => "claude-sonnet-4-6",
            Self::ClaudeSonnet4_5 => "claude-sonnet-4-5",
            Self::ClaudeSonnet4 => "claude-sonnet-4",
            Self::ClaudeHaiku4_5 => "claude-haiku-4-5",
            Self::ClaudeFable5_1 => "claude-fable-5-1",
            Self::ClaudeFable5 => "claude-fable-5",

            // -- OpenAI models --
            Self::Gpt6Astra => "gpt-6-astra",
            Self::Gpt5_6Sol => "gpt-5.6-sol",
            Self::Gpt5_6Terra => "gpt-5.6-terra",
            Self::Gpt5_6Luna => "gpt-5.6-luna",
            Self::Gpt5_5 => "gpt-5.5",
            Self::Gpt5_5Pro => "gpt-5.5-pro",
            Self::Gpt5_4 => "gpt-5.4",
            Self::Gpt5_4Pro => "gpt-5.4-pro",
            Self::Gpt5_4Mini => "gpt-5.4-mini",
            Self::Gpt5_4Nano => "gpt-5.4-nano",
            Self::Gpt5_3Codex => "gpt-5.3-codex",
            Self::Gpt5_3Spark => "gpt-5.3-codex-spark",
            Self::Gpt5_2 => "gpt-5.2",
            Self::Gpt5_2Codex => "gpt-5.2-codex",
            Self::Gpt5_1 => "gpt-5.1",
            Self::Gpt5_1Codex => "gpt-5.1-codex",
            Self::Gpt5_1CodexMax => "gpt-5.1-codex-max",
            Self::Gpt5_1CodexMini => "gpt-5.1-codex-mini",
            Self::Gpt5 => "gpt-5",
            Self::Gpt5Codex => "gpt-5-codex",
            Self::Gpt5Nano => "gpt-5-nano",

            // -- Google models --
            Self::Gemini3_1Pro => "gemini-3.1-pro",
            Self::Gemini3_8Flash => "gemini-3.8-flash",
            Self::Gemini3_7Flash => "gemini-3.7-flash",
            Self::Gemini3_6Flash => "gemini-3.6-flash",
            Self::Gemini3_5Flash => "gemini-3.5-flash",
            Self::Gemini3Flash => "gemini-3-flash",
            Self::Gemini3_5FlashLite => "gemini-3.5-flash-lite",

            // -- Meta models --
            Self::MuseSpark1_3 => "muse-spark-1.3",
            Self::MuseSpark1_2 => "muse-spark-1.2",
            Self::MuseSpark1_3Contributor => "muse-spark-1.3-contributor",
            Self::MuseSpark1_2Contributor => "muse-spark-1.2-contributor",

            // -- Alibaba models --
            Self::Qwen3_8Max => "qwen3.8-max",
            Self::Qwen3_8Flash => "qwen3.8-flash",
            Self::Qwen3_7Max => "qwen3.7-max",
            Self::Qwen3_7Plus => "qwen3.7-plus",
            Self::Qwen3_6Plus => "qwen3.6-plus",
            Self::Qwen3_5Plus => "qwen3.5-plus",

            // -- xAI models --
            Self::Grok4_6 => "grok-4.6",
            Self::Grok4_5 => "grok-4.5",
            Self::GrokBuild0_1 => "grok-build-0.1",

            // -- Z.ai models --
            Self::Glm5_3 => "glm-5.3",
            Self::Glm5_3Flash => "glm-5.3-flash",
            Self::Glm5_2 => "glm-5.2",
            Self::Glm5_1 => "glm-5.1",
            Self::Glm5 => "glm-5",

            // -- Moonshot AI models --
            Self::KimiK3 => "kimi-k3",
            Self::KimiK2_7Code => "kimi-k2.7-code",
            Self::KimiK2_6 => "kimi-k2.6",
            Self::KimiK2_5 => "kimi-k2.5",

            // -- DeepSeek models --
            Self::DeepSeekV4_1Flash => "deepseek-v4.1-flash",
            Self::DeepSeekV4Pro => "deepseek-v4-pro",
            Self::DeepSeekV4FlashVisionExp => "deepseek-v4-flash-vision-exp",
            Self::DeepSeekV4Flash => "deepseek-v4-flash",

            // -- Minimax Group models --
            Self::MiniMaxM3 => "minimax-m3",
            Self::MiniMaxM2_7 => "minimax-m2.7",
            Self::MiniMaxM2_5 => "minimax-m2.5",

            // -- Others models --
            Self::MimoV2_5Pro => "mimo-v2.5-pro",
            Self::MimoV2_5 => "mimo-v2.5",
            Self::Hy4Preview => "hy4-preview",
            Self::Hy3 => "hy3",
            Self::LongCat2_0 => "longcat-2.0",

            // -- Custom model --
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            // -- Anthropic models --
            Self::ClaudeOpus5 => "Claude Opus 5",
            Self::ClaudeOpus4_8 => "Claude Opus 4.8",
            Self::ClaudeOpus4_7 => "Claude Opus 4.7",
            Self::ClaudeOpus4_6 => "Claude Opus 4.6",
            Self::ClaudeOpus4_5 => "Claude Opus 4.5",
            Self::ClaudeSonnet5 => "Claude Sonnet 5",
            Self::ClaudeSonnet4_6 => "Claude Sonnet 4.6",
            Self::ClaudeSonnet4_5 => "Claude Sonnet 4.5",
            Self::ClaudeSonnet4 => "Claude Sonnet 4",
            Self::ClaudeHaiku4_5 => "Claude Haiku 4.5",
            Self::ClaudeFable5_1 => "Claude Fable 5.1",
            Self::ClaudeFable5 => "Claude Fable 5",

            // -- OpenAI models --
            Self::Gpt6Astra => "GPT 6 Astra",
            Self::Gpt5_6Sol => "GPT 5.6 Sol",
            Self::Gpt5_6Terra => "GPT 5.6 Terra",
            Self::Gpt5_6Luna => "GPT 5.6 Luna",
            Self::Gpt5_5 => "GPT 5.5",
            Self::Gpt5_5Pro => "GPT 5.5 Pro",
            Self::Gpt5_4 => "GPT 5.4",
            Self::Gpt5_4Pro => "GPT 5.4 Pro",
            Self::Gpt5_4Mini => "GPT 5.4 Mini",
            Self::Gpt5_4Nano => "GPT 5.4 Nano",
            Self::Gpt5_3Codex => "GPT 5.3 Codex",
            Self::Gpt5_3Spark => "GPT 5.3 Codex Spark",
            Self::Gpt5_2 => "GPT 5.2",
            Self::Gpt5_2Codex => "GPT 5.2 Codex",
            Self::Gpt5_1 => "GPT 5.1",
            Self::Gpt5_1Codex => "GPT 5.1 Codex",
            Self::Gpt5_1CodexMax => "GPT 5.1 Codex Max",
            Self::Gpt5_1CodexMini => "GPT 5.1 Codex Mini",
            Self::Gpt5 => "GPT 5",
            Self::Gpt5Codex => "GPT 5 Codex",
            Self::Gpt5Nano => "GPT 5 Nano",

            // -- Google models --
            Self::Gemini3_1Pro => "Gemini 3.1 Pro",
            Self::Gemini3_8Flash => "Gemini 3.8 Flash",
            Self::Gemini3_7Flash => "Gemini 3.7 Flash",
            Self::Gemini3_6Flash => "Gemini 3.6 Flash",
            Self::Gemini3_5Flash => "Gemini 3.5 Flash",
            Self::Gemini3Flash => "Gemini 3 Flash",
            Self::Gemini3_5FlashLite => "Gemini 3.5 Flash Lite",

            // -- Meta models --
            Self::MuseSpark1_3 => "Muse Spark 1.3",
            Self::MuseSpark1_2 => "Muse Spark 1.2",
            Self::MuseSpark1_3Contributor => "Muse Spark 1.3 (Contributor)",
            Self::MuseSpark1_2Contributor => "Muse Spark 1.2 (Contributor)",

            // -- Alibaba models --
            Self::Qwen3_8Max => "Qwen3.8 Max",
            Self::Qwen3_8Flash => "Qwen3.8 Flash",
            Self::Qwen3_7Max => "Qwen3.7 Max",
            Self::Qwen3_7Plus => "Qwen3.7 Plus",
            Self::Qwen3_6Plus => "Qwen3.6 Plus",
            Self::Qwen3_5Plus => "Qwen3.5 Plus",

            // -- xAI models --
            Self::Grok4_6 => "Grok 4.6",
            Self::Grok4_5 => "Grok 4.5",
            Self::GrokBuild0_1 => "Grok Build 0.1",

            // -- Z.ai models --
            Self::Glm5_3 => "GLM 5.3",
            Self::Glm5_3Flash => "GLM 5.3 Flash",
            Self::Glm5_2 => "GLM 5.2",
            Self::Glm5_1 => "GLM 5.1",
            Self::Glm5 => "GLM 5",

            // -- Moonshot AI models --
            Self::KimiK3 => "Kimi K3",
            Self::KimiK2_7Code => "Kimi K2.7 Code",
            Self::KimiK2_6 => "Kimi K2.6",
            Self::KimiK2_5 => "Kimi K2.5",

            // -- DeepSeek models --
            Self::DeepSeekV4_1Flash => "DeepSeek V4.1 Flash",
            Self::DeepSeekV4Pro => "DeepSeek V4 Pro",
            Self::DeepSeekV4FlashVisionExp => "DeepSeek V4 Flash Vision (Exp)",
            Self::DeepSeekV4Flash => "DeepSeek V4 Flash",

            // -- Minimax Group models --
            Self::MiniMaxM3 => "MiniMax M3",
            Self::MiniMaxM2_7 => "MiniMax M2.7",
            Self::MiniMaxM2_5 => "MiniMax M2.5",

            // -- Others models --
            Self::MimoV2_5Pro => "MiMo V2.5 Pro",
            Self::MimoV2_5 => "MiMo V2.5",
            Self::Hy4Preview => "Hy4 (Preview)",
            Self::Hy3 => "Hy3",
            Self::LongCat2_0 => "LongCat 2.0",

            // -- Custom model --
            Self::Custom {
                name, display_name, ..
            } => display_name.as_deref().unwrap_or(name),
        }
    }

    pub fn protocol(&self, subscription: OpenCodeSubscription) -> ApiProtocol {
        match self {
            // Anthropic protocol
            Self::ClaudeOpus5
            | Self::ClaudeOpus4_8
            | Self::ClaudeOpus4_7
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_5
            | Self::ClaudeSonnet5
            | Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeHaiku4_5
            | Self::ClaudeFable5_1
            | Self::ClaudeFable5 => ApiProtocol::Anthropic,

            Self::Qwen3_8Flash | Self::Qwen3_5Plus => ApiProtocol::Anthropic,

            // Google protocol
            Self::Gemini3_1Pro
            | Self::Gemini3_8Flash
            | Self::Gemini3_7Flash
            | Self::Gemini3_6Flash
            | Self::Gemini3_5Flash
            | Self::Gemini3Flash
            | Self::Gemini3_5FlashLite => ApiProtocol::Google,

            // OpenAI responses protocol
            Self::Gpt6Astra
            | Self::Gpt5_6Sol
            | Self::Gpt5_6Terra
            | Self::Gpt5_6Luna
            | Self::Gpt5_5
            | Self::Gpt5_5Pro
            | Self::Gpt5_4
            | Self::Gpt5_4Pro
            | Self::Gpt5_4Mini
            | Self::Gpt5_4Nano
            | Self::Gpt5_3Codex
            | Self::Gpt5_3Spark
            | Self::Gpt5_2
            | Self::Gpt5_2Codex
            | Self::Gpt5_1
            | Self::Gpt5_1Codex
            | Self::Gpt5_1CodexMax
            | Self::Gpt5_1CodexMini
            | Self::Gpt5
            | Self::Gpt5Codex
            | Self::Gpt5Nano => ApiProtocol::OpenAiResponses,

            Self::MuseSpark1_3
            | Self::MuseSpark1_2
            | Self::MuseSpark1_3Contributor
            | Self::MuseSpark1_2Contributor
            | Self::Grok4_6
            | Self::Grok4_5
            | Self::GrokBuild0_1 => ApiProtocol::OpenAiResponses,

            // OpenAI chat protocol
            Self::Glm5_3 | Self::Glm5_3Flash | Self::Glm5_2 | Self::Glm5_1 | Self::Glm5 => {
                ApiProtocol::OpenAiChat
            }

            Self::Qwen3_8Max
            | Self::Qwen3_7Max
            | Self::Qwen3_7Plus
            | Self::KimiK3
            | Self::KimiK2_7Code
            | Self::KimiK2_6
            | Self::KimiK2_5
            | Self::DeepSeekV4_1Flash
            | Self::DeepSeekV4Pro
            | Self::DeepSeekV4FlashVisionExp
            | Self::DeepSeekV4Flash
            | Self::MimoV2_5Pro
            | Self::MimoV2_5
            | Self::Hy4Preview
            | Self::Hy3
            | Self::LongCat2_0 => ApiProtocol::OpenAiChat,

            // Protocol used by subscription type
            Self::Qwen3_6Plus => {
                if subscription == OpenCodeSubscription::Zen {
                    ApiProtocol::Anthropic
                } else {
                    ApiProtocol::OpenAiChat
                }
            }
            Self::MiniMaxM3 | Self::MiniMaxM2_7 | Self::MiniMaxM2_5 => {
                if subscription == OpenCodeSubscription::Zen {
                    ApiProtocol::OpenAiChat
                } else {
                    ApiProtocol::Anthropic
                }
            }

            // Custom
            Self::Custom { protocol, .. } => *protocol,
        }
    }

    pub fn interleaved_reasoning(&self, subscription: OpenCodeSubscription) -> bool {
        match self {
            Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5
            | Self::Glm5_3
            | Self::Glm5_3Flash
            | Self::Glm5_2
            | Self::Glm5_1
            | Self::Glm5
            | Self::KimiK3
            | Self::KimiK2_7Code
            | Self::KimiK2_6
            | Self::KimiK2_5
            | Self::DeepSeekV4Pro
            | Self::DeepSeekV4FlashVisionExp
            | Self::DeepSeekV4Flash
            | Self::DeepSeekV4_1Flash
            | Self::MiniMaxM2_5
            | Self::MimoV2_5Pro
            | Self::MimoV2_5
            | Self::LongCat2_0 => true,

            Self::MiniMaxM3 | Self::MiniMaxM2_7 => subscription == OpenCodeSubscription::Zen,

            // Custom
            Self::Custom {
                interleaved_reasoning,
                ..
            } => *interleaved_reasoning,

            _ => false,
        }
    }

    pub fn max_token_count(&self, subscription: OpenCodeSubscription) -> u64 {
        match self {
            // Anthropic models
            Self::ClaudeOpus5
            | Self::ClaudeOpus4_8
            | Self::ClaudeOpus4_7
            | Self::ClaudeOpus4_6
            | Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5 => 1_000_000,
            Self::ClaudeOpus4_5 | Self::ClaudeHaiku4_5 => 200_000,
            Self::ClaudeSonnet4
            | Self::ClaudeSonnet5
            | Self::ClaudeFable5_1
            | Self::ClaudeFable5 => 1_000_000,

            // OpenAI models
            Self::Gpt6Astra
            | Self::Gpt5_6Sol
            | Self::Gpt5_6Terra
            | Self::Gpt5_6Luna
            | Self::Gpt5_5
            | Self::Gpt5_5Pro
            | Self::Gpt5_4
            | Self::Gpt5_4Pro => 1_050_000,
            Self::Gpt5_4Mini | Self::Gpt5_4Nano => 400_000,
            Self::Gpt5_3Codex => 400_000,
            Self::Gpt5_3Spark => 128_000,
            Self::Gpt5_2
            | Self::Gpt5_2Codex
            | Self::Gpt5_1
            | Self::Gpt5_1Codex
            | Self::Gpt5_1CodexMax
            | Self::Gpt5_1CodexMini
            | Self::Gpt5
            | Self::Gpt5Codex
            | Self::Gpt5Nano => 400_000,

            // Google models
            Self::Gemini3_1Pro
            | Self::Gemini3_8Flash
            | Self::Gemini3_7Flash
            | Self::Gemini3_6Flash
            | Self::Gemini3_5Flash
            | Self::Gemini3Flash
            | Self::Gemini3_5FlashLite => 1_048_576,

            // Meta models
            Self::MuseSpark1_3
            | Self::MuseSpark1_2
            | Self::MuseSpark1_3Contributor
            | Self::MuseSpark1_2Contributor => 1_048_576,

            // Alibaba models
            Self::Qwen3_8Max | Self::Qwen3_8Flash | Self::Qwen3_7Max | Self::Qwen3_7Plus => {
                1_000_000
            }
            Self::Qwen3_6Plus => {
                if subscription == OpenCodeSubscription::Go {
                    1_000_000
                } else {
                    262_144
                }
            }
            Self::Qwen3_5Plus => 262_144,

            // xAI models
            Self::Grok4_6 | Self::Grok4_5 => 500_000,
            Self::GrokBuild0_1 => 256_000,

            // Z.ai models
            Self::Glm5_3 | Self::Glm5_3Flash | Self::Glm5_2 => 1_000_000,
            Self::Glm5_1 | Self::Glm5 => {
                if subscription == OpenCodeSubscription::Go {
                    202_752
                } else {
                    204_800
                }
            }

            // Moonshot AI models
            Self::KimiK3 => 1_048_576,
            Self::KimiK2_7Code | Self::KimiK2_6 | Self::KimiK2_5 => 262_144,

            // DeepSeek models
            Self::DeepSeekV4_1Flash
            | Self::DeepSeekV4Pro
            | Self::DeepSeekV4FlashVisionExp
            | Self::DeepSeekV4Flash => 1_000_000,

            // Minimax Group models
            Self::MiniMaxM3 => {
                if subscription == OpenCodeSubscription::Go {
                    1_000_000
                } else {
                    512_000
                }
            }
            Self::MiniMaxM2_7 | Self::MiniMaxM2_5 => 204_800,

            // Others models
            Self::MimoV2_5Pro => 1_048_576,
            Self::MimoV2_5 => 1_000_000,
            Self::Hy4Preview => 1_024_000,
            Self::Hy3 => 256_000,
            Self::LongCat2_0 => 1_000_000,

            // Custom model
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self, subscription: OpenCodeSubscription) -> Option<u64> {
        match self {
            // Anthropic models
            Self::ClaudeOpus5 | Self::ClaudeOpus4_8 | Self::ClaudeOpus4_7 | Self::ClaudeOpus4_6 => {
                Some(128_000)
            }
            Self::ClaudeSonnet5 => Some(128_000),
            Self::ClaudeFable5_1 | Self::ClaudeFable5 => Some(128_000),
            Self::ClaudeOpus4_5
            | Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeHaiku4_5 => Some(64_000),

            // OpenAI models
            Self::Gpt6Astra
            | Self::Gpt5_6Sol
            | Self::Gpt5_6Terra
            | Self::Gpt5_6Luna
            | Self::Gpt5_5
            | Self::Gpt5_5Pro
            | Self::Gpt5_4
            | Self::Gpt5_4Pro
            | Self::Gpt5_4Mini
            | Self::Gpt5_4Nano
            | Self::Gpt5_3Codex
            | Self::Gpt5_3Spark
            | Self::Gpt5_2
            | Self::Gpt5_2Codex
            | Self::Gpt5_1
            | Self::Gpt5_1Codex
            | Self::Gpt5_1CodexMax
            | Self::Gpt5_1CodexMini
            | Self::Gpt5
            | Self::Gpt5Codex
            | Self::Gpt5Nano => Some(128_000),

            // Google models
            Self::Gemini3_1Pro
            | Self::Gemini3_8Flash
            | Self::Gemini3_7Flash
            | Self::Gemini3_6Flash
            | Self::Gemini3_5Flash
            | Self::Gemini3Flash
            | Self::Gemini3_5FlashLite => Some(65_536),

            // Meta models
            Self::MuseSpark1_3
            | Self::MuseSpark1_2
            | Self::MuseSpark1_3Contributor
            | Self::MuseSpark1_2Contributor => Some(131_072),

            // Alibaba models
            Self::Qwen3_8Max | Self::Qwen3_8Flash => Some(131_072),
            Self::Qwen3_7Max | Self::Qwen3_7Plus | Self::Qwen3_6Plus | Self::Qwen3_5Plus => {
                Some(65_536)
            }

            // xAI models
            Self::Grok4_6 | Self::Grok4_5 => Some(500_000),
            Self::GrokBuild0_1 => Some(256_000),

            // Z.ai models
            Self::Glm5_3 | Self::Glm5_3Flash | Self::Glm5_2 => Some(131_072),
            Self::Glm5_1 | Self::Glm5 => {
                if subscription == OpenCodeSubscription::Go {
                    Some(32_768)
                } else {
                    Some(131_072)
                }
            }

            // Moonshot AI models
            Self::KimiK3 => Some(131_072),
            Self::KimiK2_7Code => Some(262_144),
            Self::KimiK2_6 | Self::KimiK2_5 => Some(65_536),

            // DeepSeek models
            Self::DeepSeekV4_1Flash
            | Self::DeepSeekV4Pro
            | Self::DeepSeekV4FlashVisionExp
            | Self::DeepSeekV4Flash => Some(384_000),

            // Minimax Group models
            Self::MiniMaxM3 => {
                if subscription == OpenCodeSubscription::Go {
                    Some(131_072)
                } else {
                    Some(128_000)
                }
            }
            Self::MiniMaxM2_7 => Some(131_072),
            Self::MiniMaxM2_5 => {
                if subscription == OpenCodeSubscription::Go {
                    Some(65_536)
                } else {
                    Some(131_072)
                }
            }

            // Others models
            Self::MimoV2_5Pro | Self::MimoV2_5 => Some(128_000),
            Self::Hy3 => Some(128_000),
            Self::Hy4Preview => Some(64_000),
            Self::LongCat2_0 => Some(131_072),

            // Custom model
            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn supports_tools(&self) -> bool {
        true
    }

    pub fn supports_images(&self) -> bool {
        match self {
            // Models with image support
            Self::ClaudeOpus5
            | Self::ClaudeOpus4_8
            | Self::ClaudeOpus4_7
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_5
            | Self::ClaudeSonnet5
            | Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeHaiku4_5
            | Self::ClaudeFable5_1
            | Self::ClaudeFable5
            | Self::Gpt6Astra
            | Self::Gpt5_6Sol
            | Self::Gpt5_6Terra
            | Self::Gpt5_6Luna
            | Self::Gpt5_5
            | Self::Gpt5_5Pro
            | Self::Gpt5_4
            | Self::Gpt5_4Pro
            | Self::Gpt5_4Mini
            | Self::Gpt5_4Nano
            | Self::Gpt5_3Codex
            | Self::Gpt5_2
            | Self::Gpt5_2Codex
            | Self::Gpt5_1
            | Self::Gpt5_1Codex
            | Self::Gpt5_1CodexMax
            | Self::Gpt5_1CodexMini
            | Self::Gpt5
            | Self::Gpt5Codex
            | Self::Gpt5Nano
            | Self::Gemini3_1Pro
            | Self::Gemini3_8Flash
            | Self::Gemini3_7Flash
            | Self::Gemini3_6Flash
            | Self::Gemini3_5Flash
            | Self::Gemini3Flash
            | Self::Gemini3_5FlashLite
            | Self::MuseSpark1_3
            | Self::MuseSpark1_2
            | Self::MuseSpark1_3Contributor
            | Self::MuseSpark1_2Contributor
            | Self::Qwen3_8Max
            | Self::Qwen3_8Flash
            | Self::Qwen3_7Plus
            | Self::Qwen3_6Plus
            | Self::Qwen3_5Plus
            | Self::Grok4_6
            | Self::Grok4_5
            | Self::GrokBuild0_1
            | Self::Glm5_3Flash
            | Self::KimiK3
            | Self::KimiK2_7Code
            | Self::KimiK2_6
            | Self::KimiK2_5
            | Self::DeepSeekV4FlashVisionExp
            | Self::DeepSeekV4_1Flash
            | Self::MiniMaxM3
            | Self::MimoV2_5 => true,

            // Models without image support
            Self::Gpt5_3Spark
            | Self::Qwen3_7Max
            | Self::Glm5_3
            | Self::Glm5_2
            | Self::Glm5_1
            | Self::Glm5
            | Self::DeepSeekV4Pro
            | Self::DeepSeekV4Flash
            | Self::MiniMaxM2_7
            | Self::MiniMaxM2_5
            | Self::MimoV2_5Pro
            | Self::Hy4Preview
            | Self::Hy3
            | Self::LongCat2_0 => false,

            // Custom model
            Self::Custom { protocol, .. } => matches!(
                protocol,
                ApiProtocol::Anthropic
                    | ApiProtocol::Google
                    | ApiProtocol::OpenAiResponses
                    | ApiProtocol::OpenAiChat
            ),
        }
    }

    pub fn supports_thinking(&self, subscription: OpenCodeSubscription) -> bool {
        match self {
            // These models reason, but offer no selectable reasoning efforts
            Self::Glm5
            | Self::Glm5_1
            | Self::GrokBuild0_1
            | Self::KimiK2_5
            | Self::KimiK2_6
            | Self::KimiK2_7Code
            | Self::MiniMaxM2_5
            | Self::MiniMaxM2_7
            | Self::MiniMaxM3
            | Self::MimoV2_5
            | Self::MimoV2_5Pro
            | Self::Qwen3_6Plus
            | Self::Qwen3_7Max
            | Self::Qwen3_7Plus => true,

            _ => self
                .supported_reasoning_effort_levels(subscription)
                .is_some_and(|levels| levels.iter().any(|effort| *effort != ReasoningEffort::None)),
        }
    }

    pub fn supported_reasoning_effort_levels(
        &self,
        subscription: OpenCodeSubscription,
    ) -> Option<Vec<ReasoningEffort>> {
        match self {
            // Anthropic models
            Self::ClaudeOpus5
            | Self::ClaudeOpus4_8
            | Self::ClaudeOpus4_7
            | Self::ClaudeSonnet5
            | Self::ClaudeFable5_1
            | Self::ClaudeFable5 => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::Max,
                ReasoningEffort::XHigh,
            ]),

            Self::ClaudeOpus4_6 | Self::ClaudeSonnet4_6 => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::Max,
                ReasoningEffort::High,
            ]),

            Self::ClaudeOpus4_5 => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ]),

            Self::ClaudeSonnet4_5 | Self::ClaudeSonnet4 | Self::ClaudeHaiku4_5 => {
                Some(vec![ReasoningEffort::Max, ReasoningEffort::High])
            }

            // OpenAI models
            Self::Gpt6Astra => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::Max,
                ReasoningEffort::XHigh,
            ]),

            Self::Gpt5_6Sol | Self::Gpt5_6Terra | Self::Gpt5_6Luna => Some(vec![
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::Max,
                ReasoningEffort::XHigh,
            ]),

            Self::Gpt5_5
            | Self::Gpt5_4
            | Self::Gpt5_4Mini
            | Self::Gpt5_4Nano
            | Self::Gpt5_3Codex
            | Self::Gpt5_2 => Some(vec![
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ]),

            Self::Gpt5_5Pro | Self::Gpt5_4Pro => Some(vec![
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ]),

            Self::Gpt5_3Spark | Self::Gpt5_2Codex | Self::Gpt5_1CodexMax => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ]),

            Self::Gpt5_1 => Some(vec![
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ]),

            Self::Gpt5_1Codex | Self::Gpt5_1CodexMini | Self::Gpt5Codex => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ]),

            Self::Gpt5 | Self::Gpt5Nano => Some(vec![
                ReasoningEffort::Minimal,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ]),

            // Google models
            Self::Gemini3_6Flash
            | Self::Gemini3_5Flash
            | Self::Gemini3Flash
            | Self::Gemini3_5FlashLite => Some(vec![
                ReasoningEffort::Minimal,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ]),

            Self::Gemini3_1Pro | Self::Gemini3_8Flash | Self::Gemini3_7Flash => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ]),

            // Meta models
            Self::MuseSpark1_2 | Self::MuseSpark1_3Contributor | Self::MuseSpark1_2Contributor => {
                Some(vec![
                    ReasoningEffort::Minimal,
                    ReasoningEffort::Low,
                    ReasoningEffort::Medium,
                    ReasoningEffort::High,
                    ReasoningEffort::XHigh,
                ])
            }

            Self::MuseSpark1_3 => Some(vec![
                ReasoningEffort::Minimal,
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::Max,
                ReasoningEffort::XHigh,
            ]),

            // Alibaba models
            Self::Qwen3_8Max | Self::Qwen3_8Flash => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::XHigh,
            ]),

            Self::Qwen3_5Plus => Some(vec![ReasoningEffort::Max, ReasoningEffort::High]),

            Self::Qwen3_6Plus => {
                if subscription == OpenCodeSubscription::Zen {
                    Some(vec![ReasoningEffort::Max, ReasoningEffort::High])
                } else {
                    None
                }
            }

            // xAI models
            Self::Grok4_6 => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
                ReasoningEffort::XHigh,
            ]),

            Self::Grok4_5 => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ]),

            // Z.ai models
            Self::Glm5_2 => Some(vec![ReasoningEffort::Max, ReasoningEffort::High]),

            Self::Glm5_3 => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Max,
                ReasoningEffort::High,
            ]),

            Self::Glm5_3Flash => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Max,
                ReasoningEffort::High,
            ]),

            // Moonshot AI models
            Self::KimiK3 => Some(vec![ReasoningEffort::Max]),

            // DeepSeek models
            Self::DeepSeekV4_1Flash => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::High,
                ReasoningEffort::Max,
            ]),
            Self::DeepSeekV4Pro => Some(vec![ReasoningEffort::Max, ReasoningEffort::High]),
            Self::DeepSeekV4FlashVisionExp | Self::DeepSeekV4Flash => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Max,
                ReasoningEffort::High,
            ]),

            // Minimax Group models
            Self::MiniMaxM3 => {
                if subscription == OpenCodeSubscription::Go {
                    Some(vec![ReasoningEffort::None])
                } else {
                    None
                }
            }

            // Others models
            Self::LongCat2_0 => Some(vec![
                ReasoningEffort::Low,
                ReasoningEffort::Medium,
                ReasoningEffort::High,
            ]),

            Self::Hy3 => Some(vec![
                ReasoningEffort::None,
                ReasoningEffort::Low,
                ReasoningEffort::High,
            ]),
            Self::Hy4Preview => Some(vec![ReasoningEffort::None, ReasoningEffort::High]),

            // Custom model
            Self::Custom {
                reasoning_effort_levels,
                ..
            } => reasoning_effort_levels.clone(),

            _ => None,
        }
    }
}

/// Stream generate content for Google models via OpenCode.
///
/// Unlike `google_ai::stream_generate_content()`, this uses:
/// - `/v1/models/{model}` path (not `/v1beta/models/{model}`)
/// - `x-goog-api-key` header (not `key=` query param)
pub async fn stream_generate_content(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: google_ai::GenerateContentRequest,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<google_ai::GenerateContentResponse>>> {
    let api_key = api_key.trim();

    let model_id = &request.model.model_id;

    let uri = format!("{api_url}/v1/models/{model_id}:streamGenerateContent?alt=sse");

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("x-goog-api-key", api_key)
        .extra_headers(extra_headers)
        .body(AsyncBody::from(serde_json::to_string(&request)?))?;
    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        if let Some(line) = line.strip_prefix("data: ") {
                            match serde_json::from_str(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => {
                                    Some(Err(anyhow!("Error parsing JSON: {error:?}\n{line:?}")))
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut text = String::new();
        response.body_mut().read_to_string(&mut text).await?;
        Err(anyhow!(
            "error during streamGenerateContent via OpenCode, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}
