//! Azure AI Foundry client for Claude models
//!
//! This module provides a client for interacting with Claude models
//! deployed on Azure AI Foundry using the Anthropic-compatible API.

use std::io;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow};
use chrono::{DateTime, Utc};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::http::{self, HeaderMap, HeaderValue};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest, StatusCode};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
use thiserror::Error;

/// Default model for Azure AI Foundry deployments
pub const DEFAULT_MODEL: &str = "claude-opus-4-5";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AzureFoundryModelCacheConfiguration {
    pub min_total_token: u64,
    pub should_speculate: bool,
    pub max_cache_anchors: usize,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum AzureFoundryModelMode {
    #[default]
    Default,
    Thinking {
        budget_tokens: Option<u32>,
    },
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(rename = "claude-opus-4-5", alias = "claude-opus-4-5-latest")]
    #[default]
    ClaudeOpus4_5,
    #[serde(
        rename = "claude-opus-4-5-thinking",
        alias = "claude-opus-4-5-thinking-latest"
    )]
    ClaudeOpus4_5Thinking,
    #[serde(rename = "claude-sonnet-4-5", alias = "claude-sonnet-4-5-latest")]
    ClaudeSonnet4_5,
    #[serde(
        rename = "claude-sonnet-4-5-thinking",
        alias = "claude-sonnet-4-5-thinking-latest"
    )]
    ClaudeSonnet4_5Thinking,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        max_tokens: u64,
        display_name: Option<String>,
        tool_override: Option<String>,
        cache_configuration: Option<AzureFoundryModelCacheConfiguration>,
        max_output_tokens: Option<u64>,
        default_temperature: Option<f32>,
        #[serde(default)]
        mode: AzureFoundryModelMode,
    },
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        if id.starts_with("claude-opus-4-5-thinking") {
            return Ok(Self::ClaudeOpus4_5Thinking);
        }

        if id.starts_with("claude-opus-4-5") {
            return Ok(Self::ClaudeOpus4_5);
        }

        if id.starts_with("claude-sonnet-4-5-thinking") {
            return Ok(Self::ClaudeSonnet4_5Thinking);
        }

        if id.starts_with("claude-sonnet-4-5") {
            return Ok(Self::ClaudeSonnet4_5);
        }

        Err(anyhow!("invalid model ID: {id}"))
    }

    pub fn id(&self) -> &str {
        match self {
            Self::ClaudeOpus4_5 => "claude-opus-4-5-latest",
            Self::ClaudeOpus4_5Thinking => "claude-opus-4-5-thinking-latest",
            Self::ClaudeSonnet4_5 => "claude-sonnet-4-5-latest",
            Self::ClaudeSonnet4_5Thinking => "claude-sonnet-4-5-thinking-latest",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn request_id(&self) -> &str {
        match self {
            Self::ClaudeOpus4_5 | Self::ClaudeOpus4_5Thinking => "claude-opus-4-5",
            Self::ClaudeSonnet4_5 | Self::ClaudeSonnet4_5Thinking => "claude-sonnet-4-5",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::ClaudeOpus4_5 => "Claude Opus 4.5",
            Self::ClaudeOpus4_5Thinking => "Claude Opus 4.5 Thinking",
            Self::ClaudeSonnet4_5 => "Claude Sonnet 4.5",
            Self::ClaudeSonnet4_5Thinking => "Claude Sonnet 4.5 Thinking",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn cache_configuration(&self) -> Option<AzureFoundryModelCacheConfiguration> {
        match self {
            Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_5Thinking
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_5Thinking => Some(AzureFoundryModelCacheConfiguration {
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
            Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_5Thinking
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_5Thinking => 200_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> u64 {
        match self {
            Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_5Thinking
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_5Thinking => 8_192,
            Self::Custom {
                max_output_tokens, ..
            } => max_output_tokens.unwrap_or(4_096),
        }
    }

    pub fn default_temperature(&self) -> f32 {
        match self {
            Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_5Thinking
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_5Thinking => 1.0,
            Self::Custom {
                default_temperature,
                ..
            } => default_temperature.unwrap_or(1.0),
        }
    }

    pub fn mode(&self) -> AzureFoundryModelMode {
        match self {
            Self::ClaudeOpus4_5 | Self::ClaudeSonnet4_5 => AzureFoundryModelMode::Default,
            Self::ClaudeOpus4_5Thinking | Self::ClaudeSonnet4_5Thinking => {
                AzureFoundryModelMode::Thinking {
                    budget_tokens: Some(4_096),
                }
            }
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

/// Configuration for Azure AI Foundry endpoint
#[derive(Clone, Debug)]
pub struct AzureFoundryConfig {
    /// The full endpoint URL (e.g., "https://your-resource.services.ai.azure.com/anthropic/v1/messages")
    pub endpoint_url: String,
    /// The API key for authentication
    pub api_key: String,
    /// The model deployment name
    pub model: String,
}

impl AzureFoundryConfig {
    pub fn new(endpoint_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            endpoint_url: endpoint_url.into(),
            api_key: api_key.into(),
            model: DEFAULT_MODEL.to_string(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    config: &AzureFoundryConfig,
    request: Request,
) -> Result<BoxStream<'static, Result<Event, AzureFoundryError>>, AzureFoundryError> {
    stream_completion_with_rate_limit_info(client, config, request)
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
    config: &AzureFoundryConfig,
    request: Request,
) -> Result<
    (
        BoxStream<'static, Result<Event, AzureFoundryError>>,
        Option<RateLimitInfo>,
    ),
    AzureFoundryError,
> {
    let request = StreamingRequest {
        base: request,
        stream: true,
    };

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(&config.endpoint_url)
        .header("anthropic-version", "2023-06-01")
        .header("x-api-key", config.api_key.trim())
        .header("Content-Type", "application/json");

    let serialized_request =
        serde_json::to_string(&request).map_err(AzureFoundryError::SerializeRequest)?;
    let request = request_builder
        .body(AsyncBody::from(serialized_request))
        .map_err(AzureFoundryError::BuildRequestBody)?;

    let mut response = client
        .send(request)
        .await
        .map_err(AzureFoundryError::HttpSend)?;
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
                            Err(error) => Some(Err(AzureFoundryError::DeserializeResponse(error))),
                        }
                    }
                    Err(error) => Some(Err(AzureFoundryError::ReadResponse(error))),
                }
            })
            .boxed();
        Ok((stream, Some(rate_limits)))
    } else if response.status().as_u16() == 529 {
        Err(AzureFoundryError::ServerOverloaded {
            retry_after: rate_limits.retry_after,
        })
    } else if let Some(retry_after) = rate_limits.retry_after {
        Err(AzureFoundryError::RateLimit { retry_after })
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(AzureFoundryError::ReadResponse)?;

        match serde_json::from_str::<Event>(&body) {
            Ok(Event::Error { error }) => Err(AzureFoundryError::ApiError(error)),
            Ok(_) | Err(_) => Err(AzureFoundryError::HttpResponseError {
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
    pub model: String,
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

#[derive(Debug)]
pub enum AzureFoundryError {
    SerializeRequest(serde_json::Error),
    BuildRequestBody(http::Error),
    HttpSend(anyhow::Error),
    DeserializeResponse(serde_json::Error),
    ReadResponse(io::Error),
    HttpResponseError {
        status_code: StatusCode,
        message: String,
    },
    RateLimit {
        retry_after: Duration,
    },
    ServerOverloaded {
        retry_after: Option<Duration>,
    },
    ApiError(ApiError),
}

#[derive(Debug, Serialize, Deserialize, Error)]
#[error("Azure AI Foundry API Error: {error_type}: {message}")]
pub struct ApiError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ApiErrorCode {
    InvalidRequestError,
    AuthenticationError,
    PermissionError,
    NotFoundError,
    RequestTooLarge,
    RateLimitError,
    ApiError,
    OverloadedError,
}

impl ApiError {
    pub fn code(&self) -> Option<ApiErrorCode> {
        ApiErrorCode::from_str(&self.error_type).ok()
    }

    pub fn is_rate_limit_error(&self) -> bool {
        matches!(self.error_type.as_str(), "rate_limit_error")
    }

    pub fn match_window_exceeded(&self) -> Option<u64> {
        let Some(ApiErrorCode::InvalidRequestError) = self.code() else {
            return None;
        };

        parse_prompt_too_long(&self.message)
    }
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
