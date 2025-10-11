use std::time::Duration;

use anthropic::{AnthropicError, ApiError};
use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, Utc};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::http::{HeaderMap, HeaderValue};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::{Deserialize, Serialize};
use settings::ModelMode;
use strum::EnumIter;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AnthropicVertexAISettings {
    pub project_id: Option<String>,
    pub location: Option<String>,
}

pub const ANTHROPIC_API_URL: &str = "https://aiplatform.googleapis.com";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AnthropicVertexModelCacheConfiguration {
    pub min_total_token: u64,
    pub should_speculate: bool,
    pub max_cache_anchors: usize,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "claude-opus-4@20250514")]
    ClaudeOpus4,
    #[serde(rename = "claude-opus-4-thinking")]
    ClaudeOpus4Thinking,
    #[serde(rename = "claude-opus-4-1@20250805")]
    ClaudeOpus4_1,
    #[serde(rename = "claude-opus-4-1-thinking")]
    ClaudeOpus4_1Thinking,
    #[default]
    #[serde(rename = "claude-sonnet-4@20250514")]
    ClaudeSonnet4,
    #[serde(rename = "claude-sonnet-4-thinking")]
    ClaudeSonnet4Thinking,
    #[serde(rename = "claude-sonnet-4-5@20250929")]
    ClaudeSonnet4_5,
    #[serde(rename = "claude-sonnet-4-5-thinking")]
    ClaudeSonnet4_5Thinking,
    #[serde(rename = "claude-3-7-sonnet@20250219")]
    Claude3_7Sonnet,
    #[serde(rename = "claude-3-7-sonnet-thinking")]
    Claude3_7SonnetThinking,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        max_tokens: u64,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        /// Override this model with a different Anthropic model for tool calls.
        tool_override: Option<String>,
        /// Indicates whether this custom model supports caching.
        cache_configuration: Option<AnthropicVertexModelCacheConfiguration>,
        max_output_tokens: Option<u64>,
        default_temperature: Option<f32>,
        #[serde(default)]
        mode: ModelMode,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::ClaudeSonnet4
    }

    pub fn from_id(id: &str) -> Result<Self> {
        if id.starts_with("claude-opus-4-thinking") {
            return Ok(Self::ClaudeOpus4Thinking);
        }

        if id.starts_with("claude-opus-4") {
            return Ok(Self::ClaudeOpus4);
        }

        if id.starts_with("claude-sonnet-4-5-thinking") {
            return Ok(Self::ClaudeSonnet4Thinking);
        }

        if id.starts_with("claude-sonnet-4-5") {
            return Ok(Self::ClaudeSonnet4);
        }

        if id.starts_with("claude-sonnet-4-thinking") {
            return Ok(Self::ClaudeSonnet4Thinking);
        }

        if id.starts_with("claude-sonnet-4") {
            return Ok(Self::ClaudeSonnet4);
        }

        if id.starts_with("claude-3-7-sonnet-thinking") {
            return Ok(Self::Claude3_7SonnetThinking);
        }

        if id.starts_with("claude-3-7-sonnet") {
            return Ok(Self::Claude3_7Sonnet);
        }

        Err(anyhow!("invalid model ID: {id}"))
    }

    pub fn id(&self) -> &str {
        match self {
            Self::ClaudeOpus4_1 => "claude-opus-4-1@20250805",
            Self::ClaudeOpus4_1Thinking => "claude-opus-4-1-thinking@20250805",
            Self::ClaudeOpus4 => "claude-opus-4@20250514",
            Self::ClaudeOpus4Thinking => "claude-opus-4-thinking@20250514",
            Self::ClaudeSonnet4_5 => "claude-sonnet-4-5@20250929",
            Self::ClaudeSonnet4_5Thinking => "claude-sonnet-4-5-thinking@20250929",
            Self::ClaudeSonnet4 => "claude-sonnet-4@20250514",
            Self::ClaudeSonnet4Thinking => "claude-sonnet-4-thinking@20250514",
            Self::Claude3_7Sonnet => "	claude-3-7-sonnet@20250219",
            Self::Claude3_7SonnetThinking => "claude-3-7-sonnet-thinking@20250219",
            Self::Custom { name, .. } => name,
        }
    }

    /// The id of the model that should be used for making API requests
    pub fn request_id(&self) -> &str {
        match self {
            Self::ClaudeOpus4_1 | Self::ClaudeOpus4_1Thinking => "claude-opus-4-1@20250805",
            Self::ClaudeOpus4 | Self::ClaudeOpus4Thinking => "claude-opus-4@20250514",
            Self::ClaudeSonnet4_5 | Self::ClaudeSonnet4_5Thinking => "claude-sonnet-4-5@20250929",
            Self::ClaudeSonnet4 | Self::ClaudeSonnet4Thinking => "claude-sonnet-4@20250514",
            Self::Claude3_7Sonnet | Self::Claude3_7SonnetThinking => "claude-3-7-sonnet@20250219",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::ClaudeOpus4_1 => "Claude Opus 4.1 (VAI)",
            Self::ClaudeOpus4 => "Claude Opus 4 (VAI)",
            Self::ClaudeOpus4_1Thinking => "Claude Opus 4.1 Thinking (VAI)",
            Self::ClaudeOpus4Thinking => "Claude Opus 4 Thinking (VAI)",
            Self::ClaudeSonnet4_5 => "Claude Sonnet 4.5 (VAI)",
            Self::ClaudeSonnet4 => "Claude Sonnet 4 (VAI)",
            Self::ClaudeSonnet4_5Thinking => "Claude Sonnet 4.5 Thinking (VAI)",
            Self::ClaudeSonnet4Thinking => "Claude Sonnet 4 Thinking (VAI)",
            Self::Claude3_7Sonnet => "Claude 3.7 Sonnet (VAI)",
            Self::Claude3_7SonnetThinking => "Claude 3.7 Sonnet Thinking (VAI)",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn cache_configuration(&self) -> Option<AnthropicVertexModelCacheConfiguration> {
        match self {
            Self::ClaudeOpus4_1
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4_1Thinking
            | Self::ClaudeOpus4Thinking
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5Thinking
            | Self::ClaudeSonnet4Thinking
            | Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking => Some(AnthropicVertexModelCacheConfiguration {
                min_total_token: 2_048,
                should_speculate: true,
                max_cache_anchors: 4,
            }),
            Self::Custom {
                cache_configuration,
                ..
            } => cache_configuration.clone(),
        }
    }

    pub fn max_token_count(&self) -> u64 {
        match self {
            Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_5Thinking
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4Thinking => 1_000_000,
            Self::ClaudeOpus4_1
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4_1Thinking
            | Self::ClaudeOpus4Thinking
            | Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking => 264_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> u64 {
        match self {
            Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_5Thinking
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4Thinking => 64_000,
            Self::ClaudeOpus4_1
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4_1Thinking
            | Self::ClaudeOpus4Thinking
            | Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking => 8_192,
            Self::Custom {
                max_output_tokens, ..
            } => max_output_tokens.unwrap_or(8_192),
        }
    }

    pub fn default_temperature(&self) -> f32 {
        match self {
            Self::ClaudeOpus4_1
            | Self::ClaudeOpus4
            | Self::ClaudeOpus4_1Thinking
            | Self::ClaudeOpus4Thinking
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_5Thinking
            | Self::ClaudeSonnet4Thinking
            | Self::Claude3_7Sonnet
            | Self::Claude3_7SonnetThinking => 1.0,
            Self::Custom {
                default_temperature,
                ..
            } => default_temperature.unwrap_or(1.0),
        }
    }

    pub fn mode(&self) -> ModelMode {
        match self {
            Self::ClaudeOpus4_1
            | Self::ClaudeOpus4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4
            | Self::Claude3_7Sonnet => ModelMode::Default,
            Self::ClaudeSonnet4Thinking | Self::ClaudeSonnet4_5Thinking => ModelMode::Thinking {
                budget_tokens: Some(32_000),
            },
            Self::ClaudeOpus4_1Thinking
            | Self::ClaudeOpus4Thinking
            | Self::Claude3_7SonnetThinking => ModelMode::Thinking {
                budget_tokens: Some(4_096),
            },
            Self::Custom { mode, .. } => mode.clone(),
        }
    }

    pub fn tool_model_id(&self) -> &str {
        if let Self::Custom {
            tool_override: Some(tool_override),
            ..
        } = self
        {
            tool_override
        } else {
            self.request_id()
        }
    }
}

pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    request: Request,
) -> Result<Response, AnthropicError> {
    let uri = format!("{api_url}/v1/messages");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json");

    let serialized_request =
        serde_json::to_string(&request).map_err(AnthropicError::SerializeRequest)?;
    let request = request_builder
        .body(AsyncBody::from(serialized_request))
        .map_err(AnthropicError::BuildRequestBody)?;

    let mut response = client
        .send(request)
        .await
        .map_err(AnthropicError::HttpSend)?;
    let status_code = response.status();
    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .map_err(AnthropicError::ReadResponse)?;

    if status_code.is_success() {
        Ok(serde_json::from_str(&body).map_err(AnthropicError::DeserializeResponse)?)
    } else {
        Err(AnthropicError::HttpResponseError {
            status_code,
            message: body,
        })
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    project_id: &str,
    location_id: &str,
    access_token: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<Event, AnthropicError>>, AnthropicError> {
    stream_completion_with_rate_limit_info(
        client,
        api_url,
        project_id,
        location_id,
        access_token,
        request,
    )
    .await
    .map(|output| output.0)
}

/// An individual rate limit.
#[derive(Debug)]
pub struct RateLimit {
    pub limit: usize,
    pub remaining: usize,
    pub reset: DateTime<Utc>,
}

impl RateLimit {
    fn from_headers(resource: &str, headers: &HeaderMap<HeaderValue>) -> Result<Self> {
        let limit =
            get_header(&format!("anthropic-ratelimit-{resource}-limit"), headers)?.parse()?;
        let remaining = get_header(
            &format!("anthropic-ratelimit-{resource}-remaining"),
            headers,
        )?
        .parse()?;
        let reset = DateTime::parse_from_rfc3339(get_header(
            &format!("anthropic-ratelimit-{resource}-reset"),
            headers,
        )?)?
        .to_utc();

        Ok(Self {
            limit,
            remaining,
            reset,
        })
    }
}

/// <https://docs.anthropic.com/en/api/rate-limits#response-headers>
#[derive(Debug)]
pub struct RateLimitInfo {
    pub retry_after: Option<Duration>,
    pub requests: Option<RateLimit>,
    pub tokens: Option<RateLimit>,
    pub input_tokens: Option<RateLimit>,
    pub output_tokens: Option<RateLimit>,
}

impl RateLimitInfo {
    fn from_headers(headers: &HeaderMap<HeaderValue>) -> Self {
        // Check if any rate limit headers exist
        let has_rate_limit_headers = headers
            .keys()
            .any(|k| k == "retry-after" || k.as_str().starts_with("anthropic-ratelimit-"));

        if !has_rate_limit_headers {
            return Self {
                retry_after: None,
                requests: None,
                tokens: None,
                input_tokens: None,
                output_tokens: None,
            };
        }

        Self {
            retry_after: parse_retry_after(headers),
            requests: RateLimit::from_headers("requests", headers).ok(),
            tokens: RateLimit::from_headers("tokens", headers).ok(),
            input_tokens: RateLimit::from_headers("input-tokens", headers).ok(),
            output_tokens: RateLimit::from_headers("output-tokens", headers).ok(),
        }
    }
}

/// Parses the Retry-After header value as an integer number of seconds (anthropic always uses
/// seconds). Note that other services might specify an HTTP date or some other format for this
/// header. Returns `None` if the header is not present or cannot be parsed.
pub fn parse_retry_after(headers: &HeaderMap<HeaderValue>) -> Option<Duration> {
    headers
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
}

fn get_header<'a>(key: &str, headers: &'a HeaderMap) -> anyhow::Result<&'a str> {
    Ok(headers
        .get(key)
        .with_context(|| format!("missing header `{key}`"))?
        .to_str()?)
}

pub async fn stream_completion_with_rate_limit_info(
    client: &dyn HttpClient,
    api_url: &str,
    project_id: &str,
    location_id: &str,
    access_token: &str,
    request: Request,
) -> Result<
    (
        BoxStream<'static, Result<Event, AnthropicError>>,
        Option<RateLimitInfo>,
    ),
    AnthropicError,
> {
    let model_id = request.model.clone();
    let request = StreamingRequest {
        base: request,
        stream: true,
    };

    let endpoint = if location_id == "global" {
        "https://{api_url}".to_string()
    } else {
        format!("https://{location_id}-{api_url}")
    };

    let uri = format!(
        "{endpoint}/v1/projects/{project_id}/locations/{location_id}/publishers/anthropic/models/{model_id}:streamRawPredict"
    );

    // MODIFICATION 4: Add Authorization header for bearer token authentication.
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .header("anthropic-beta", ["context-1m-2025-08-07"].join(","));

    let serialized_request =
        serde_json::to_string(&request).map_err(AnthropicError::SerializeRequest)?;
    let request = request_builder
        .body(AsyncBody::from(serialized_request))
        .map_err(AnthropicError::BuildRequestBody)?;

    let mut response = client
        .send(request)
        .await
        .map_err(AnthropicError::HttpSend)?;
    let rate_limits = RateLimitInfo::from_headers(response.headers());
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        let stream = reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        match serde_json::from_str(line) {
                            Ok(response) => Some(Ok(response)),
                            Err(error) => Some(Err(AnthropicError::DeserializeResponse(error))),
                        }
                    }
                    Err(error) => Some(Err(AnthropicError::ReadResponse(error))),
                }
            })
            .boxed();
        Ok((stream, Some(rate_limits)))
    } else if response.status().as_u16() == 529 {
        Err(AnthropicError::ServerOverloaded {
            retry_after: rate_limits.retry_after,
        })
    } else if let Some(retry_after) = rate_limits.retry_after {
        Err(AnthropicError::RateLimit { retry_after })
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(AnthropicError::ReadResponse)?;

        match serde_json::from_str::<Event>(&body) {
            Ok(Event::Error { error }) => Err(AnthropicError::ApiError(error)),
            Ok(_) | Err(_) => Err(AnthropicError::HttpResponseError {
                status_code: response.status(),
                message: body,
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CacheControlType {
    Ephemeral,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: CacheControlType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<RequestContent>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RequestContent {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "thinking")]
    Thinking {
        thinking: String,
        signature: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
    #[serde(rename = "image")]
    Image {
        source: ImageSource,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        is_error: bool,
        content: ToolResultContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolResultContent {
    Plain(String),
    Multipart(Vec<ToolResultPart>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolResultPart {
    Text { text: String },
    Image { source: ImageSource },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String },
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Any,
    Tool { name: String },
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Thinking {
    Enabled { budget_tokens: Option<u32> },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrContents {
    String(String),
    Content(Vec<RequestContent>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(skip)]
    pub model: String,
    pub anthropic_version: String,
    pub max_tokens: u64,
    pub messages: Vec<Message>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<StringOrContents>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop_sequences: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StreamingRequest {
    #[serde(flatten)]
    pub base: Request,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_creation_input_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_read_input_tokens: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: Role,
    pub content: Vec<ResponseContent>,
    pub model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "message_start")]
    MessageStart { message: Response },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: ResponseContent,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { index: usize, delta: ContentDelta },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: usize },
    #[serde(rename = "message_delta")]
    MessageDelta { delta: MessageDelta, usage: Usage },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "error")]
    Error { error: ApiError },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentDelta {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "thinking_delta")]
    ThinkingDelta { thinking: String },
    #[serde(rename = "signature_delta")]
    SignatureDelta { signature: String },
    #[serde(rename = "input_json_delta")]
    InputJsonDelta { partial_json: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
}

pub fn parse_prompt_too_long(message: &str) -> Option<u64> {
    message
        .strip_prefix("prompt is too long: ")?
        .split_once(" tokens")?
        .0
        .parse()
        .ok()
}

#[test]
fn test_match_window_exceeded() {
    let error = ApiError {
        error_type: "invalid_request_error".to_string(),
        message: "prompt is too long: 220000 tokens > 200000".to_string(),
    };
    assert_eq!(error.match_window_exceeded(), Some(220_000));

    let error = ApiError {
        error_type: "invalid_request_error".to_string(),
        message: "prompt is too long: 1234953 tokens".to_string(),
    };
    assert_eq!(error.match_window_exceeded(), Some(1234953));

    let error = ApiError {
        error_type: "invalid_request_error".to_string(),
        message: "not a prompt length error".to_string(),
    };
    assert_eq!(error.match_window_exceeded(), None);

    let error = ApiError {
        error_type: "rate_limit_error".to_string(),
        message: "prompt is too long: 12345 tokens".to_string(),
    };
    assert_eq!(error.match_window_exceeded(), None);

    let error = ApiError {
        error_type: "invalid_request_error".to_string(),
        message: "prompt is too long: invalid tokens".to_string(),
    };
    assert_eq!(error.match_window_exceeded(), None);
}
