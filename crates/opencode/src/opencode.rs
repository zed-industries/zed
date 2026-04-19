use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    id: String,
}

pub async fn list_model_ids(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
) -> Result<Vec<String>> {
    let uri = format!("{api_url}/v1/models");
    let request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()));

    let request = request_builder
        .body(AsyncBody::default())
        .map_err(|e| anyhow!("failed to build request: {e}"))?;
    let mut response = client
        .send(request)
        .await
        .map_err(|e| anyhow!("failed to send request: {e}"))?;

    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .map_err(|e| anyhow!("failed to read response: {e}"))?;

    if response.status().is_success() {
        let models_response: ModelsResponse =
            serde_json::from_str(&body).map_err(|e| anyhow!("failed to parse response: {e}"))?;
        Ok(models_response
            .data
            .into_iter()
            .map(|entry| entry.id)
            .collect())
    } else {
        Err(anyhow!(
            "failed to list models, status: {:?}, body: {}",
            response.status(),
            body
        ))
    }
}

pub const OPENCODE_API_URL: &str = "https://opencode.ai/zen";
pub const OPENCODE_GO_API_URL: &str = "https://opencode.ai/zen/go";

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
    #[serde(rename = "claude-opus-4-7")]
    ClaudeOpus4_7,
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

    // -- Google protocol models --
    #[serde(rename = "gemini-3.1-pro")]
    Gemini3_1Pro,
    #[serde(rename = "gemini-3-flash")]
    Gemini3Flash,

    // -- Anthropic protocol models (non-Claude) --
    #[serde(rename = "minimax-m2.7")]
    MiniMaxM2_7,

    // -- OpenAI Chat Completions protocol models --
    #[serde(rename = "minimax-m2.5")]
    MiniMaxM2_5,
    #[serde(rename = "minimax-m2.5-free")]
    MiniMaxM2_5Free,
    #[serde(rename = "glm-5.1")]
    Glm5_1,
    #[serde(rename = "glm-5")]
    Glm5,
    #[serde(rename = "kimi-k2.5")]
    KimiK2_5,
    #[serde(rename = "mimo-v2-pro")]
    MimoV2Pro,
    #[serde(rename = "mimo-v2-omni")]
    MimoV2Omni,
    #[serde(rename = "qwen3.6-plus")]
    Qwen3_6Plus,
    #[serde(rename = "qwen3.5-plus")]
    Qwen3_5Plus,
    #[serde(rename = "big-pickle")]
    BigPickle,
    #[serde(rename = "nemotron-3-super-free")]
    Nemotron3SuperFree,

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
            Self::ClaudeOpus4_7 => "claude-opus-4-7",
            Self::ClaudeOpus4_6 => "claude-opus-4-6",
            Self::ClaudeOpus4_5 => "claude-opus-4-5",
            Self::ClaudeOpus4_1 => "claude-opus-4-1",
            Self::ClaudeSonnet4_6 => "claude-sonnet-4-6",
            Self::ClaudeSonnet4_5 => "claude-sonnet-4-5",
            Self::ClaudeSonnet4 => "claude-sonnet-4",
            Self::ClaudeHaiku4_5 => "claude-haiku-4-5",
            Self::Claude3_5Haiku => "claude-3-5-haiku",

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

            Self::Gemini3_1Pro => "gemini-3.1-pro",
            Self::Gemini3Flash => "gemini-3-flash",

            Self::MiniMaxM2_7 => "minimax-m2.7",

            Self::MiniMaxM2_5 => "minimax-m2.5",
            Self::MiniMaxM2_5Free => "minimax-m2.5-free",
            Self::Glm5_1 => "glm-5.1",
            Self::Glm5 => "glm-5",
            Self::KimiK2_5 => "kimi-k2.5",
            Self::MimoV2Pro => "mimo-v2-pro",
            Self::MimoV2Omni => "mimo-v2-omni",
            Self::Qwen3_6Plus => "qwen3.6-plus",
            Self::Qwen3_5Plus => "qwen3.5-plus",
            Self::BigPickle => "big-pickle",
            Self::Nemotron3SuperFree => "nemotron-3-super-free",

            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::ClaudeOpus4_7 => "Claude Opus 4.7",
            Self::ClaudeOpus4_6 => "Claude Opus 4.6",
            Self::ClaudeOpus4_5 => "Claude Opus 4.5",
            Self::ClaudeOpus4_1 => "Claude Opus 4.1",
            Self::ClaudeSonnet4_6 => "Claude Sonnet 4.6",
            Self::ClaudeSonnet4_5 => "Claude Sonnet 4.5",
            Self::ClaudeSonnet4 => "Claude Sonnet 4",
            Self::ClaudeHaiku4_5 => "Claude Haiku 4.5",
            Self::Claude3_5Haiku => "Claude Haiku 3.5",

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

            Self::Gemini3_1Pro => "Gemini 3.1 Pro",
            Self::Gemini3Flash => "Gemini 3 Flash",

            Self::MiniMaxM2_7 => "MiniMax M2.7",

            Self::MiniMaxM2_5 => "MiniMax M2.5",
            Self::MiniMaxM2_5Free => "MiniMax M2.5 Free",
            Self::Glm5_1 => "GLM 5.1",
            Self::Glm5 => "GLM 5",
            Self::KimiK2_5 => "Kimi K2.5",
            Self::MimoV2Pro => "MiMo V2 Pro",
            Self::MimoV2Omni => "MiMo V2 Omni",
            Self::Qwen3_6Plus => "Qwen3.6 Plus",
            Self::Qwen3_5Plus => "Qwen3.5 Plus",
            Self::BigPickle => "Big Pickle",
            Self::Nemotron3SuperFree => "Nemotron 3 Super Free",

            Self::Custom {
                name, display_name, ..
            } => display_name.as_deref().unwrap_or(name),
        }
    }

    pub fn protocol(&self) -> ApiProtocol {
        match self {
            Self::ClaudeOpus4_7
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeHaiku4_5
            | Self::Claude3_5Haiku
            | Self::MiniMaxM2_7 => ApiProtocol::Anthropic,

            Self::Gpt5_4
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

            Self::Gemini3_1Pro | Self::Gemini3Flash => ApiProtocol::Google,

            Self::MiniMaxM2_5
            | Self::MiniMaxM2_5Free
            | Self::Glm5_1
            | Self::Glm5
            | Self::KimiK2_5
            | Self::MimoV2Pro
            | Self::MimoV2Omni
            | Self::Qwen3_6Plus
            | Self::Qwen3_5Plus
            | Self::BigPickle
            | Self::Nemotron3SuperFree => ApiProtocol::OpenAiChat,

            Self::Custom { protocol, .. } => *protocol,
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            // Anthropic models
            Self::ClaudeOpus4_7 | Self::ClaudeOpus4_6 | Self::ClaudeSonnet4_6 => 1_000_000,
            Self::ClaudeOpus4_5 | Self::ClaudeSonnet4_5 | Self::ClaudeSonnet4 => 200_000,
            Self::ClaudeOpus4_1 => 200_000,
            Self::ClaudeHaiku4_5 => 200_000,
            Self::Claude3_5Haiku => 200_000,

            // OpenAI models
            Self::Gpt5_4 | Self::Gpt5_4Pro => 1_050_000,
            Self::Gpt5_4Mini | Self::Gpt5_4Nano => 400_000,
            Self::Gpt5_3Codex => 400_000,
            Self::Gpt5_3Spark => 128_000,
            Self::Gpt5_2 | Self::Gpt5_2Codex => 400_000,
            Self::Gpt5_1 | Self::Gpt5_1Codex | Self::Gpt5_1CodexMax | Self::Gpt5_1CodexMini => {
                400_000
            }
            Self::Gpt5 | Self::Gpt5Codex | Self::Gpt5Nano => 400_000,

            // Google models
            Self::Gemini3_1Pro => 1_048_576,
            Self::Gemini3Flash => 1_048_576,

            // Anthropic non-Claude models
            Self::MiniMaxM2_7 => 196_608,

            // OpenAI-compatible models
            Self::MiniMaxM2_5 | Self::MiniMaxM2_5Free => 196_608,
            Self::Glm5 | Self::Glm5_1 => 200_000,
            Self::KimiK2_5 => 262_144,
            Self::MimoV2Pro => 1_048_576,
            Self::MimoV2Omni => 262_144,
            Self::Qwen3_6Plus | Self::Qwen3_5Plus => 131_072,
            Self::BigPickle => 200_000,
            Self::Nemotron3SuperFree => 262_144,

            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        match self {
            // Anthropic models
            Self::ClaudeOpus4_7 | Self::ClaudeOpus4_6 => Some(128_000),
            Self::ClaudeSonnet4_6 => Some(64_000),
            Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeHaiku4_5 => Some(64_000),
            Self::Claude3_5Haiku => Some(8_192),

            // OpenAI models
            Self::Gpt5_4
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
            Self::Gemini3_1Pro | Self::Gemini3Flash => Some(65_536),

            // Anthropic non-Claude models
            Self::MiniMaxM2_7 => Some(65_536),

            // OpenAI-compatible models
            Self::MiniMaxM2_5 | Self::MiniMaxM2_5Free => Some(65_536),
            Self::Glm5 | Self::Glm5_1 | Self::BigPickle => Some(128_000),
            Self::KimiK2_5 => Some(65_536),
            Self::MimoV2Pro => Some(131_072),
            Self::MimoV2Omni => Some(65_536),
            Self::Qwen3_6Plus | Self::Qwen3_5Plus => Some(65_536),
            Self::Nemotron3SuperFree => Some(16_384),

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
            // Anthropic models support images
            Self::ClaudeOpus4_7
            | Self::ClaudeOpus4_6
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_1
            | Self::ClaudeSonnet4_6
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeHaiku4_5
            | Self::Claude3_5Haiku => true,

            // OpenAI models support images
            Self::Gpt5_4
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
            | Self::Gpt5Nano => true,

            // Google models support images
            Self::Gemini3_1Pro | Self::Gemini3Flash => true,

            // Anthropic non-Claude models support images
            Self::MiniMaxM2_7 => true,

            // OpenAI-compatible models — conservative default
            Self::MiniMaxM2_5
            | Self::MiniMaxM2_5Free
            | Self::Glm5_1
            | Self::Glm5
            | Self::KimiK2_5
            | Self::MimoV2Pro
            | Self::MimoV2Omni
            | Self::Qwen3_6Plus
            | Self::Qwen3_5Plus
            | Self::BigPickle
            | Self::Nemotron3SuperFree => false,

            Self::Custom { protocol, .. } => matches!(
                protocol,
                ApiProtocol::Anthropic
                    | ApiProtocol::OpenAiResponses
                    | ApiProtocol::OpenAiChat
                    | ApiProtocol::Google
            ),
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
    request: google_ai::GenerateContentRequest,
) -> Result<BoxStream<'static, Result<google_ai::GenerateContentResponse>>> {
    let api_key = api_key.trim();

    let model_id = &request.model.model_id;

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
            "error during streamGenerateContent via OpenCode Zen, status code: {:?}, body: {}",
            response.status(),
            text
        ))
    }
}
