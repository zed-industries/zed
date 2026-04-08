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

pub mod batches;
pub mod completion;

pub const ANTHROPIC_API_URL: &str = "https://api.anthropic.com";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AnthropicModelCacheConfiguration {
    pub min_total_token: u64,
    pub should_speculate: bool,
    pub max_cache_anchors: usize,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum AnthropicModelMode {
    #[default]
    Default,
    Thinking {
        budget_tokens: Option<u32>,
    },
    AdaptiveThinking,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[serde(
        rename = "claude-opus-4",
        alias = "claude-opus-4-latest",
        alias = "claude-opus-4-thinking",
        alias = "claude-opus-4-thinking-latest"
    )]
    ClaudeOpus4,
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
        alias = "claude-opus-4-6-1m-context",
        alias = "claude-opus-4-6-1m-context-latest",
        alias = "claude-opus-4-6-thinking",
        alias = "claude-opus-4-6-thinking-latest",
        alias = "claude-opus-4-6-1m-context-thinking",
        alias = "claude-opus-4-6-1m-context-thinking-latest"
    )]
    ClaudeOpus4_6,
    #[serde(
        rename = "claude-sonnet-4",
        alias = "claude-sonnet-4-latest",
        alias = "claude-sonnet-4-thinking",
        alias = "claude-sonnet-4-thinking-latest"
    )]
    ClaudeSonnet4,
    #[serde(
        rename = "claude-sonnet-4-5",
        alias = "claude-sonnet-4-5-latest",
        alias = "claude-sonnet-4-5-thinking",
        alias = "claude-sonnet-4-5-thinking-latest"
    )]
    ClaudeSonnet4_5,
    #[default]
    #[serde(
        rename = "claude-sonnet-4-6",
        alias = "claude-sonnet-4-6-latest",
        alias = "claude-sonnet-4-6-1m-context",
        alias = "claude-sonnet-4-6-1m-context-latest",
        alias = "claude-sonnet-4-6-thinking",
        alias = "claude-sonnet-4-6-thinking-latest",
        alias = "claude-sonnet-4-6-1m-context-thinking",
        alias = "claude-sonnet-4-6-1m-context-thinking-latest"
    )]
    ClaudeSonnet4_6,
    #[serde(
        rename = "claude-haiku-4-5",
        alias = "claude-haiku-4-5-latest",
        alias = "claude-haiku-4-5-thinking",
        alias = "claude-haiku-4-5-thinking-latest"
    )]
    ClaudeHaiku4_5,
    #[serde(rename = "claude-3-haiku", alias = "claude-3-haiku-latest")]
    Claude3Haiku,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        max_tokens: u64,
        /// The name displayed in the UI, such as in the assistant panel model dropdown menu.
        display_name: Option<String>,
        /// Override this model with a different Anthropic model for tool calls.
        tool_override: Option<String>,
        /// Indicates whether this custom model supports caching.
        cache_configuration: Option<AnthropicModelCacheConfiguration>,
        max_output_tokens: Option<u64>,
        default_temperature: Option<f32>,
        #[serde(default)]
        extra_beta_headers: Vec<String>,
        #[serde(default)]
        mode: AnthropicModelMode,
    },
}

impl Model {
    pub fn default_fast() -> Self {
        Self::ClaudeHaiku4_5
    }

    pub fn from_id(id: &str) -> Result<Self> {
        if id.starts_with("claude-opus-4-6") {
            return Ok(Self::ClaudeOpus4_6);
        }

        if id.starts_with("claude-opus-4-5") {
            return Ok(Self::ClaudeOpus4_5);
        }

        if id.starts_with("claude-opus-4-1") {
            return Ok(Self::ClaudeOpus4_1);
        }

        if id.starts_with("claude-opus-4") {
            return Ok(Self::ClaudeOpus4);
        }

        if id.starts_with("claude-sonnet-4-6") {
            return Ok(Self::ClaudeSonnet4_6);
        }

        if id.starts_with("claude-sonnet-4-5") {
            return Ok(Self::ClaudeSonnet4_5);
        }

        if id.starts_with("claude-sonnet-4") {
            return Ok(Self::ClaudeSonnet4);
        }

        if id.starts_with("claude-haiku-4-5") {
            return Ok(Self::ClaudeHaiku4_5);
        }

        if id.starts_with("claude-3-haiku") {
            return Ok(Self::Claude3Haiku);
        }

        Err(anyhow!("invalid model ID: {id}"))
    }

    pub fn id(&self) -> &str {
        match self {
            Self::ClaudeOpus4 => "claude-opus-4-latest",
            Self::ClaudeOpus4_1 => "claude-opus-4-1-latest",
            Self::ClaudeOpus4_5 => "claude-opus-4-5-latest",
            Self::ClaudeOpus4_6 => "claude-opus-4-6-latest",
            Self::ClaudeSonnet4 => "claude-sonnet-4-latest",
            Self::ClaudeSonnet4_5 => "claude-sonnet-4-5-latest",
            Self::ClaudeSonnet4_6 => "claude-sonnet-4-6-latest",
            Self::ClaudeHaiku4_5 => "claude-haiku-4-5-latest",
            Self::Claude3Haiku => "claude-3-haiku-20240307",
            Self::Custom { name, .. } => name,
        }
    }

    /// The id of the model that should be used for making API requests
    pub fn request_id(&self) -> &str {
        match self {
            Self::ClaudeOpus4 => "claude-opus-4-20250514",
            Self::ClaudeOpus4_1 => "claude-opus-4-1-20250805",
            Self::ClaudeOpus4_5 => "claude-opus-4-5-20251101",
            Self::ClaudeOpus4_6 => "claude-opus-4-6",
            Self::ClaudeSonnet4 => "claude-sonnet-4-20250514",
            Self::ClaudeSonnet4_5 => "claude-sonnet-4-5-20250929",
            Self::ClaudeSonnet4_6 => "claude-sonnet-4-6",
            Self::ClaudeHaiku4_5 => "claude-haiku-4-5-20251001",
            Self::Claude3Haiku => "claude-3-haiku-20240307",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::ClaudeOpus4 => "Claude Opus 4",
            Self::ClaudeOpus4_1 => "Claude Opus 4.1",
            Self::ClaudeOpus4_5 => "Claude Opus 4.5",
            Self::ClaudeOpus4_6 => "Claude Opus 4.6",
            Self::ClaudeSonnet4 => "Claude Sonnet 4",
            Self::ClaudeSonnet4_5 => "Claude Sonnet 4.5",
            Self::ClaudeSonnet4_6 => "Claude Sonnet 4.6",
            Self::ClaudeHaiku4_5 => "Claude Haiku 4.5",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Custom {
                name, display_name, ..
            } => display_name.as_ref().unwrap_or(name),
        }
    }

    pub fn cache_configuration(&self) -> Option<AnthropicModelCacheConfiguration> {
        match self {
            Self::ClaudeOpus4
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_6
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_6
            | Self::ClaudeHaiku4_5
            | Self::Claude3Haiku => Some(AnthropicModelCacheConfiguration {
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
            Self::ClaudeOpus4
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeHaiku4_5
            | Self::Claude3Haiku => 200_000,
            Self::ClaudeOpus4_6 | Self::ClaudeSonnet4_6 => 1_000_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
        }
    }

    pub fn max_output_tokens(&self) -> u64 {
        match self {
            Self::ClaudeOpus4 | Self::ClaudeOpus4_1 => 32_000,
            Self::ClaudeOpus4_5
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_6
            | Self::ClaudeHaiku4_5 => 64_000,
            Self::ClaudeOpus4_6 => 128_000,
            Self::Claude3Haiku => 4_096,
            Self::Custom {
                max_output_tokens, ..
            } => max_output_tokens.unwrap_or(4_096),
        }
    }

    pub fn default_temperature(&self) -> f32 {
        match self {
            Self::ClaudeOpus4
            | Self::ClaudeOpus4_1
            | Self::ClaudeOpus4_5
            | Self::ClaudeOpus4_6
            | Self::ClaudeSonnet4
            | Self::ClaudeSonnet4_5
            | Self::ClaudeSonnet4_6
            | Self::ClaudeHaiku4_5
            | Self::Claude3Haiku => 1.0,
            Self::Custom {
                default_temperature,
                ..
            } => default_temperature.unwrap_or(1.0),
        }
    }

    pub fn mode(&self) -> AnthropicModelMode {
        if self.supports_adaptive_thinking() {
            AnthropicModelMode::AdaptiveThinking
        } else if self.supports_thinking() {
            AnthropicModelMode::Thinking {
                budget_tokens: Some(4_096),
            }
        } else {
            AnthropicModelMode::Default
        }
    }

    pub fn supports_thinking(&self) -> bool {
        matches!(
            self,
            Self::ClaudeOpus4
                | Self::ClaudeOpus4_1
                | Self::ClaudeOpus4_5
                | Self::ClaudeOpus4_6
                | Self::ClaudeSonnet4
                | Self::ClaudeSonnet4_5
                | Self::ClaudeSonnet4_6
                | Self::ClaudeHaiku4_5
        )
    }

    pub fn supports_adaptive_thinking(&self) -> bool {
        matches!(self, Self::ClaudeOpus4_6 | Self::ClaudeSonnet4_6)
    }

    pub fn beta_headers(&self) -> Option<String> {
        let mut headers = vec![];

        match self {
            Self::Custom {
                extra_beta_headers, ..
            } => {
                headers.extend(
                    extra_beta_headers
                        .iter()
                        .filter(|header| !header.trim().is_empty())
                        .cloned(),
                );
            }
            _ => {}
        }

        if headers.is_empty() {
            None
        } else {
            Some(headers.join(","))
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

/// Generate completion with streaming.
pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
    beta_headers: Option<String>,
) -> Result<BoxStream<'static, Result<Event, AnthropicError>>, AnthropicError> {
    stream_completion_with_rate_limit_info(client, api_url, api_key, request, beta_headers)
        .await
        .map(|output| output.0)
}

/// Generate completion without streaming.
pub async fn non_streaming_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
    beta_headers: Option<String>,
) -> Result<Response, AnthropicError> {
    let (mut response, rate_limits) =
        send_request(client, api_url, api_key, &request, beta_headers).await?;

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(AnthropicError::ReadResponse)?;

        serde_json::from_str(&body).map_err(AnthropicError::DeserializeResponse)
    } else {
        Err(handle_error_response(response, rate_limits).await)
    }
}

async fn send_request(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: impl Serialize,
    beta_headers: Option<String>,
) -> Result<(http::Response<AsyncBody>, RateLimitInfo), AnthropicError> {
    let uri = format!("{api_url}/v1/messages");

    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Anthropic-Version", "2023-06-01")
        .header("X-Api-Key", api_key.trim())
        .header("Content-Type", "application/json");

    if let Some(beta_headers) = beta_headers {
        request_builder = request_builder.header("Anthropic-Beta", beta_headers);
    }

    let serialized_request =
        serde_json::to_string(&request).map_err(AnthropicError::SerializeRequest)?;
    let request = request_builder
        .body(AsyncBody::from(serialized_request))
        .map_err(AnthropicError::BuildRequestBody)?;

    let response = client
        .send(request)
        .await
        .map_err(AnthropicError::HttpSend)?;

    let rate_limits = RateLimitInfo::from_headers(response.headers());

    Ok((response, rate_limits))
}

async fn handle_error_response(
    mut response: http::Response<AsyncBody>,
    rate_limits: RateLimitInfo,
) -> AnthropicError {
    if response.status().as_u16() == 529 {
        return AnthropicError::ServerOverloaded {
            retry_after: rate_limits.retry_after,
        };
    }

    if let Some(retry_after) = rate_limits.retry_after {
        return AnthropicError::RateLimit { retry_after };
    }

    let mut body = String::new();
    let read_result = response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .map_err(AnthropicError::ReadResponse);

    if let Err(err) = read_result {
        return err;
    }

    match serde_json::from_str::<Event>(&body) {
        Ok(Event::Error { error }) => AnthropicError::ApiError(error),
        Ok(_) | Err(_) => AnthropicError::HttpResponseError {
            status_code: response.status(),
            message: body,
        },
    }
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
    api_key: &str,
    request: Request,
    beta_headers: Option<String>,
) -> Result<
    (
        BoxStream<'static, Result<Event, AnthropicError>>,
        Option<RateLimitInfo>,
    ),
    AnthropicError,
> {
    let request = StreamingRequest {
        base: request,
        stream: true,
    };

    let (response, rate_limits) =
        send_request(client, api_url, api_key, &request, beta_headers).await?;

    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        let stream = reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line
                            .strip_prefix("data: ")
                            .or_else(|| line.strip_prefix("data:"))?;

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
    } else {
        Err(handle_error_response(response, rate_limits).await)
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

fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    #[serde(default, skip_serializing_if = "is_false")]
    pub eager_input_streaming: bool,
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
    Adaptive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Effort {
    Low,
    Medium,
    High,
    Max,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub effort: Option<Effort>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_config: Option<OutputConfig>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop_sequences: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<Speed>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Speed {
    #[default]
    Standard,
    Fast,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingRequest {
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
pub enum AnthropicError {
    /// Failed to serialize the HTTP request body to JSON
    SerializeRequest(serde_json::Error),

    /// Failed to construct the HTTP request body
    BuildRequestBody(http::Error),

    /// Failed to send the HTTP request
    HttpSend(anyhow::Error),

    /// Failed to deserialize the response from JSON
    DeserializeResponse(serde_json::Error),

    /// Failed to read from response stream
    ReadResponse(io::Error),

    /// HTTP error response from the API
    HttpResponseError {
        status_code: StatusCode,
        message: String,
    },

    /// Rate limit exceeded
    RateLimit { retry_after: Duration },

    /// Server overloaded
    ServerOverloaded { retry_after: Option<Duration> },

    /// API returned an error response
    ApiError(ApiError),
}

#[derive(Debug, Serialize, Deserialize, Error)]
#[error("Anthropic API Error: {error_type}: {message}")]
pub struct ApiError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

/// An Anthropic API error code.
/// <https://docs.anthropic.com/en/api/errors#http-errors>
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ApiErrorCode {
    /// 400 - `invalid_request_error`: There was an issue with the format or content of your request.
    InvalidRequestError,
    /// 401 - `authentication_error`: There's an issue with your API key.
    AuthenticationError,
    /// 403 - `permission_error`: Your API key does not have permission to use the specified resource.
    PermissionError,
    /// 404 - `not_found_error`: The requested resource was not found.
    NotFoundError,
    /// 413 - `request_too_large`: Request exceeds the maximum allowed number of bytes.
    RequestTooLarge,
    /// 429 - `rate_limit_error`: Your account has hit a rate limit.
    RateLimitError,
    /// 500 - `api_error`: An unexpected error has occurred internal to Anthropic's systems.
    ApiError,
    /// 529 - `overloaded_error`: Anthropic's API is temporarily overloaded.
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

/// Request body for the token counting API.
/// Similar to `Request` but without `max_tokens` since it's not needed for counting.
#[derive(Debug, Serialize)]
pub struct CountTokensRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<StringOrContents>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<Thinking>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

/// Response from the token counting API.
#[derive(Debug, Deserialize)]
pub struct CountTokensResponse {
    pub input_tokens: u64,
}

/// Count the number of tokens in a message without creating it.
pub async fn count_tokens(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: CountTokensRequest,
) -> Result<CountTokensResponse, AnthropicError> {
    let uri = format!("{api_url}/v1/messages/count_tokens");

    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Anthropic-Version", "2023-06-01")
        .header("X-Api-Key", api_key.trim())
        .header("Content-Type", "application/json");

    let serialized_request =
        serde_json::to_string(&request).map_err(AnthropicError::SerializeRequest)?;
    let http_request = request_builder
        .body(AsyncBody::from(serialized_request))
        .map_err(AnthropicError::BuildRequestBody)?;

    let mut response = client
        .send(http_request)
        .await
        .map_err(AnthropicError::HttpSend)?;

    let rate_limits = RateLimitInfo::from_headers(response.headers());

    if response.status().is_success() {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(AnthropicError::ReadResponse)?;

        serde_json::from_str(&body).map_err(AnthropicError::DeserializeResponse)
    } else {
        Err(handle_error_response(response, rate_limits).await)
    }
}

// -- Conversions from/to `language_model_core` types --

impl From<language_model_core::Speed> for Speed {
    fn from(speed: language_model_core::Speed) -> Self {
        match speed {
            language_model_core::Speed::Standard => Speed::Standard,
            language_model_core::Speed::Fast => Speed::Fast,
        }
    }
}

impl From<AnthropicError> for language_model_core::LanguageModelCompletionError {
    fn from(error: AnthropicError) -> Self {
        let provider = language_model_core::ANTHROPIC_PROVIDER_NAME;
        match error {
            AnthropicError::SerializeRequest(error) => Self::SerializeRequest { provider, error },
            AnthropicError::BuildRequestBody(error) => Self::BuildRequestBody { provider, error },
            AnthropicError::HttpSend(error) => Self::HttpSend { provider, error },
            AnthropicError::DeserializeResponse(error) => {
                Self::DeserializeResponse { provider, error }
            }
            AnthropicError::ReadResponse(error) => Self::ApiReadResponseError { provider, error },
            AnthropicError::HttpResponseError {
                status_code,
                message,
            } => Self::HttpResponseError {
                provider,
                status_code,
                message,
            },
            AnthropicError::RateLimit { retry_after } => Self::RateLimitExceeded {
                provider,
                retry_after: Some(retry_after),
            },
            AnthropicError::ServerOverloaded { retry_after } => Self::ServerOverloaded {
                provider,
                retry_after,
            },
            AnthropicError::ApiError(api_error) => api_error.into(),
        }
    }
}

impl From<ApiError> for language_model_core::LanguageModelCompletionError {
    fn from(error: ApiError) -> Self {
        use ApiErrorCode::*;
        let provider = language_model_core::ANTHROPIC_PROVIDER_NAME;
        match error.code() {
            Some(code) => match code {
                InvalidRequestError => Self::BadRequestFormat {
                    provider,
                    message: error.message,
                },
                AuthenticationError => Self::AuthenticationError {
                    provider,
                    message: error.message,
                },
                PermissionError => Self::PermissionError {
                    provider,
                    message: error.message,
                },
                NotFoundError => Self::ApiEndpointNotFound { provider },
                RequestTooLarge => Self::PromptTooLarge {
                    tokens: language_model_core::parse_prompt_too_long(&error.message),
                },
                RateLimitError => Self::RateLimitExceeded {
                    provider,
                    retry_after: None,
                },
                ApiError => Self::ApiInternalServerError {
                    provider,
                    message: error.message,
                },
                OverloadedError => Self::ServerOverloaded {
                    provider,
                    retry_after: None,
                },
            },
            None => Self::Other(error.into()),
        }
    }
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
