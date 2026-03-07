use std::mem;

use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub const OPENCODE_API_URL: &str = "https://opencode.ai/zen";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ApiProtocol {
    #[default]
    Anthropic,
    OpenAiResponses,
    OpenAiChat,
    Google,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    // -- Anthropic protocol models --
    #[serde(rename = "claude-opus-4-6")]
    ClaudeOpus4_6,
    #[serde(rename = "claude-opus-4-5")]
    ClaudeOpus4_5,
    #[serde(rename = "claude-opus-4-1")]
    ClaudeOpus4_1,
    #[default]
    #[serde(rename = "claude-sonnet-4-6")]
    ClaudeSonnet4_6,
    #[serde(rename = "claude-sonnet-4-5")]
    ClaudeSonnet4_5,
    #[serde(rename = "claude-sonnet-4")]
    ClaudeSonnet4,
    #[serde(rename = "claude-haiku-4-5")]
    ClaudeHaiku4_5,
    #[serde(rename = "claude-3-5-haiku")]
    Claude3_5Haiku,

    // -- OpenAI Responses API models --
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

    // -- Google protocol models --
    #[serde(rename = "gemini-3-pro")]
    Gemini3Pro,
    #[serde(rename = "gemini-3-flash")]
    Gemini3Flash,

    // -- OpenAI Chat Completions protocol models --
    #[serde(rename = "minimax-m2.5")]
    MiniMaxM2_5,
    #[serde(rename = "minimax-m2.5-free")]
    MiniMaxM2_5Free,
    #[serde(rename = "minimax-m2.1")]
    MiniMaxM2_1,
    #[serde(rename = "glm-5")]
    Glm5,
    #[serde(rename = "glm-5-free")]
    Glm5Free,
    #[serde(rename = "glm-4.7")]
    Glm4_7,
    #[serde(rename = "glm-4.6")]
    Glm4_6,
    #[serde(rename = "kimi-k2.5")]
    KimiK2_5,
    #[serde(rename = "kimi-k2.5-free")]
    KimiK2_5Free,
    #[serde(rename = "kimi-k2-thinking")]
    KimiK2Thinking,
    #[serde(rename = "kimi-k2")]
    KimiK2,
    #[serde(rename = "qwen3-coder")]
    Qwen3Coder,
    #[serde(rename = "big-pickle")]
    BigPickle,

    // -- Custom model --
    #[serde(rename = "custom")]
    Custom {
        name: String,
        display_name: Option<String>,
        max_tokens: u64,
        max_output_tokens: Option<u64>,
        protocol: ApiProtocol,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::ClaudeHaiku4_5
    }

    pub fn id(&self) -> &str {
        match self {
            Self::ClaudeOpus4_6 => "claude-opus-4-6",
            Self::ClaudeOpus4_5 => "claude-opus-4-5",
            Self::ClaudeOpus4_1 => "claude-opus-4-1",
            Self::ClaudeSonnet4_6 => "claude-sonnet-4-6",
            Self::ClaudeSonnet4_5 => "claude-sonnet-4-5",
            Self::ClaudeSonnet4 => "claude-sonnet-4",
            Self::ClaudeHaiku4_5 => "claude-haiku-4-5",
            Self::Claude3_5Haiku => "claude-3-5-haiku",

            Self::Gpt5_2 => "gpt-5.2",
            Self::Gpt5_2Codex => "gpt-5.2-codex",
            Self::Gpt5_1 => "gpt-5.1",
            Self::Gpt5_1Codex => "gpt-5.1-codex",
            Self::Gpt5_1CodexMax => "gpt-5.1-codex-max",
            Self::Gpt5_1CodexMini => "gpt-5.1-codex-mini",
            Self::Gpt5 => "gpt-5",
            Self::Gpt5Codex => "gpt-5-codex",
            Self::Gpt5Nano => "gpt-5-nano",

            Self::Gemini3Pro => "gemini-3-pro",
            Self::Gemini3Flash => "gemini-3-flash",

            Self::MiniMaxM2_5 => "minimax-m2.5",
            Self::MiniMaxM2_5Free => "minimax-m2.5-free",
            Self::MiniMaxM2_1 => "minimax-m2.1",
            Self::Glm5 => "glm-5",
            Self::Glm5Free => "glm-5-free",
            Self::Glm4_7 => "glm-4.7",
            Self::Glm4_6 => "glm-4.6",
            Self::KimiK2_5 => "kimi-k2.5",
            Self::KimiK2_5Free => "kimi-k2.5-free",
            Self::KimiK2Thinking => "kimi-k2-thinking",
            Self::KimiK2 => "kimi-k2",
            Self::Qwen3Coder => "qwen3-coder",
            Self::BigPickle => "big-pickle",

            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::ClaudeOpus4_6 => "Claude Opus 4.6",
            Self::ClaudeOpus4_5 => "Claude Opus 4.5",
            Self::ClaudeOpus4_1 => "Claude Opus 4.1",
            Self::ClaudeSonnet4_6 => "Claude Sonnet 4.6",
            Self::ClaudeSonnet4_5 => "Claude Sonnet 4.5",
            Self::ClaudeSonnet4 => "Claude Sonnet 4",
            Self::ClaudeHaiku4_5 => "Claude Haiku 4.5",
            Self::Claude3_5Haiku => "Claude Haiku 3.5",

            Self::Gpt5_2 => "GPT 5.2",
            Self::Gpt5_2Codex => "GPT 5.2 Codex",
            Self::Gpt5_1 => "GPT 5.1",
            Self::Gpt5_1Codex => "GPT 5.1 Codex",
            Self::Gpt5_1CodexMax => "GPT 5.1 Codex Max",
            Self::Gpt5_1CodexMini => "GPT 5.1 Codex Mini",
            Self::Gpt5 => "GPT 5",
            Self::Gpt5Codex => "GPT 5 Codex",
            Self::Gpt5Nano => "GPT 5 Nano",

            Self::Gemini3Pro => "Gemini 3 Pro",
            Self::Gemini3Flash => "Gemini 3 Flash",

            Self::MiniMaxM2_5 => "MiniMax M2.5",
            Self::MiniMaxM2_5Free => "MiniMax M2.5 Free",
            Self::MiniMaxM2_1 => "MiniMax M2.1",
            Self::Glm5 => "GLM 5",
            Self::Glm5Free => "GLM 5 Free",
            Self::Glm4_7 => "GLM 4.7",
            Self::Glm4_6 => "GLM 4.6",
            Self::KimiK2_5 => "Kimi K2.5",
            Self::KimiK2_5Free => "Kimi K2.5 Free",
            Self::KimiK2Thinking => "Kimi K2 Thinking",
            Self::KimiK2 => "Kimi K2",
            Self::Qwen3Coder => "Qwen3 Coder 480B",
            Self::BigPickle => "Big Pickle",

            Self::Custom {
                name,
                display_name,
                ..
            } => display_name.as_deref().unwrap_or(name),
        }
    }

    pub fn protocol(&self) -> ApiProtocol {
        match self {
            Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeHaiku4_5
            | Self::Claude3_5Haiku => ApiProtocol::Anthropic,

            Self::Gpt5_2
            | Self::Gpt5_2Codex
            | Self::Gpt5_1
            | Self::Gpt5_1Codex
            | Self::Gpt5_1CodexMax
            | Self::Gpt5_1CodexMini
            | Self::Gpt5
            | Self::Gpt5Codex
            | Self::Gpt5Nano => ApiProtocol::OpenAiResponses,

            Self::Gemini3Pro | Self::Gemini3Flash => ApiProtocol::Google,

            Self::MiniMaxM2_5
            | Self::MiniMaxM2_5Free
            | Self::MiniMaxM2_1
            | Self::Glm5
            | Self::Glm5Free
            | Self::Glm4_7
            | Self::Glm4_6
            | Self::KimiK2_5
            | Self::KimiK2_5Free
            | Self::KimiK2Thinking
            | Self::KimiK2
            | Self::Qwen3Coder
            | Self::BigPickle => ApiProtocol::OpenAiChat,

            Self::Custom { protocol, .. } => *protocol,
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            // Anthropic models
            Self::ClaudeOpus4_6 | Self::ClaudeSonnet4_6 => 200_000,
            Self::ClaudeOpus4_5 | Self::ClaudeSonnet4_5 | Self::ClaudeSonnet4 => 200_000,
            Self::ClaudeOpus4_1 => 200_000,
            Self::ClaudeHaiku4_5 => 200_000,
            Self::Claude3_5Haiku => 200_000,

            // OpenAI models
            Self::Gpt5_2 | Self::Gpt5_2Codex => 256_000,
            Self::Gpt5_1
            | Self::Gpt5_1Codex
            | Self::Gpt5_1CodexMax
            | Self::Gpt5_1CodexMini => 256_000,
            Self::Gpt5 | Self::Gpt5Codex => 256_000,
            Self::Gpt5Nano => 128_000,

            // Google models
            Self::Gemini3Pro => 1_048_576,
            Self::Gemini3Flash => 1_048_576,

            // OpenAI-compatible models
            Self::MiniMaxM2_5 | Self::MiniMaxM2_5Free | Self::MiniMaxM2_1 => 128_000,
            Self::Glm5 | Self::Glm5Free | Self::Glm4_7 | Self::Glm4_6 => 128_000,
            Self::KimiK2_5 | Self::KimiK2_5Free | Self::KimiK2Thinking | Self::KimiK2 => 128_000,
            Self::Qwen3Coder => 128_000,
            Self::BigPickle => 128_000,

            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            // Anthropic models
            Self::ClaudeOpus4_6 | Self::ClaudeSonnet4_6 => Some(16_384),
            Self::ClaudeOpus4_5 | Self::ClaudeSonnet4_5 | Self::ClaudeSonnet4 => Some(16_384),
            Self::ClaudeOpus4_1 => Some(16_384),
            Self::ClaudeHaiku4_5 => Some(8_192),
            Self::Claude3_5Haiku => Some(8_192),

            // OpenAI models
            Self::Gpt5_2 | Self::Gpt5_2Codex => Some(32_768),
            Self::Gpt5_1
            | Self::Gpt5_1Codex
            | Self::Gpt5_1CodexMax
            | Self::Gpt5_1CodexMini => Some(32_768),
            Self::Gpt5 | Self::Gpt5Codex => Some(32_768),
            Self::Gpt5Nano => Some(16_384),

            // Google models
            Self::Gemini3Pro | Self::Gemini3Flash => Some(65_536),

            // OpenAI-compatible models — use reasonable defaults
            Self::MiniMaxM2_5 | Self::MiniMaxM2_5Free | Self::MiniMaxM2_1 => Some(16_384),
            Self::Glm5 | Self::Glm5Free | Self::Glm4_7 | Self::Glm4_6 => Some(16_384),
            Self::KimiK2_5 | Self::KimiK2_5Free | Self::KimiK2Thinking | Self::KimiK2 => {
                Some(16_384)
            }
            Self::Qwen3Coder => Some(16_384),
            Self::BigPickle => Some(16_384),

            Self::Custom {
                max_output_tokens, ..
            } => *max_output_tokens,
        }
    }

    pub fn supports_tools(&self) -> bool {
        match self {
            Self::Custom { .. } => true,
            _ => true,
        }
    }

    pub fn supports_images(&self) -> bool {
        match self {
            // Anthropic models support images
            Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeHaiku4_5
            | Self::Claude3_5Haiku => true,

            // OpenAI models support images
            Self::Gpt5_2
            | Self::Gpt5_2Codex
            | Self::Gpt5_1
            | Self::Gpt5_1Codex
            | Self::Gpt5_1CodexMax
            | Self::Gpt5_1CodexMini
            | Self::Gpt5
            | Self::Gpt5Codex
            | Self::Gpt5Nano => true,

            // Google models support images
            Self::Gemini3Pro | Self::Gemini3Flash => true,

            // OpenAI-compatible models — conservative default
            _ => false,
        }
    }
}

/// Stream generate content for Google models via OpenCode Zen.
///
/// Unlike `google_ai::stream_generate_content()`, this uses:
/// - `/v1/models/{model}` path (not `/v1beta/models/{model}`)
/// - `Authorization: Bearer` header (not `key=` query param)
pub async fn stream_generate_content_zen(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    mut request: google_ai::GenerateContentRequest,
) -> Result<BoxStream<'static, Result<google_ai::GenerateContentResponse>>> {
    let api_key = api_key.trim();

    let model_id = mem::take(&mut request.model.model_id);

    let uri = format!("{api_url}/v1/models/{model_id}:streamGenerateContent?alt=sse");

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"));

    let request = request_builder.body(AsyncBody::from(serde_json::to_string(&request)?))?;
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
                                Err(error) => Some(Err(anyhow!(
                                    "Error parsing JSON: {error:?}\n{line:?}"
                                ))),
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
            "error during streamGenerateContent via OpenCode Zen, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}
