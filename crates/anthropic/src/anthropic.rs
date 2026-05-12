use std::io;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::http::{self, HeaderMap, HeaderValue};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest, StatusCode};
use serde::{Deserialize, Serialize};
use strum::EnumString;
use thiserror::Error;

pub mod batches;
pub mod completion;

pub const ANTHROPIC_API_URL: &str = "https://api.anthropic.com";

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

/// Capabilities reported by the Anthropic models endpoint for a given model.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ModelCapabilities {
    #[serde(default)]
    pub thinking: Option<ThinkingCapability>,
    #[serde(default)]
    pub image_input: Option<SupportedCapability>,
    #[serde(default)]
    pub effort: Option<EffortCapability>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SupportedCapability {
    #[serde(default)]
    pub supported: bool,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ThinkingCapability {
    #[serde(default)]
    pub supported: bool,
    #[serde(default)]
    pub types: Option<ThinkingTypes>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ThinkingTypes {
    #[serde(default)]
    pub adaptive: SupportedCapability,
    #[serde(default)]
    pub enabled: SupportedCapability,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct EffortCapability {
    #[serde(default)]
    pub supported: bool,
    #[serde(default)]
    pub low: Option<SupportedCapability>,
    #[serde(default)]
    pub medium: Option<SupportedCapability>,
    #[serde(default)]
    pub high: Option<SupportedCapability>,
    #[serde(default)]
    pub max: Option<SupportedCapability>,
    #[serde(default)]
    pub xhigh: Option<SupportedCapability>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub id: String,
    pub display_name: String,
    pub max_input_tokens: u64,
    pub max_output_tokens: u64,
    pub default_temperature: f32,
    pub mode: AnthropicModelMode,
    pub supports_thinking: bool,
    pub supports_adaptive_thinking: bool,
    pub supports_images: bool,
    pub supports_speed: bool,
    pub supported_effort_levels: Vec<Effort>,
    /// A model id to substitute when invoking tools, used for models that
    /// don't support tool calling natively.
    pub tool_override: Option<String>,
    /// Extra `Anthropic-Beta` header values to send with each request.
    pub extra_beta_headers: Vec<String>,
}

impl Model {
    /// Construct a `Model` from an entry returned by the `/v1/models` listing endpoint.
    pub fn from_listed(entry: ListModelEntry) -> Self {
        let supports_thinking = entry
            .capabilities
            .as_ref()
            .and_then(|t| t.thinking.as_ref())
            .map(|t| t.supported)
            .unwrap_or(false);
        let supports_adaptive_thinking = entry
            .capabilities
            .as_ref()
            .and_then(|t| t.thinking.as_ref())
            .and_then(|t| t.types.as_ref())
            .map(|types| types.adaptive.supported)
            .unwrap_or(false);
        let supports_images = entry
            .capabilities
            .as_ref()
            .and_then(|c| c.image_input.as_ref())
            .map(|c| c.supported)
            .unwrap_or(false);

        let mut supported_effort_levels = Vec::new();
        if let Some(effort) = entry.capabilities.as_ref().and_then(|e| e.effort.as_ref()) {
            // The `xhigh` effort level reported by the API has no
            // corresponding `Effort` variant in the request enum, so it is
            // intentionally dropped here.
            for (level, supported) in [
                (Effort::Low, effort.low.as_ref()),
                (Effort::Medium, effort.medium.as_ref()),
                (Effort::High, effort.high.as_ref()),
                (Effort::XHigh, effort.xhigh.as_ref()),
                (Effort::Max, effort.max.as_ref()),
            ] {
                if supported.map(|c| c.supported).unwrap_or(false) {
                    supported_effort_levels.push(level);
                }
            }
        }

        let mode = if supports_adaptive_thinking {
            AnthropicModelMode::AdaptiveThinking
        } else if supports_thinking {
            AnthropicModelMode::Thinking {
                budget_tokens: Some(4_096),
            }
        } else {
            AnthropicModelMode::Default
        };

        let supports_speed = entry.id == "claude-opus-4-6";

        Self {
            display_name: entry.display_name,
            id: entry.id,
            max_input_tokens: entry.max_input_tokens,
            max_output_tokens: entry.max_tokens,
            default_temperature: 1.0,
            mode,
            supports_thinking,
            supports_adaptive_thinking,
            supports_images,
            supports_speed,
            supported_effort_levels,
            tool_override: None,
            extra_beta_headers: Vec::new(),
        }
    }

    pub fn beta_headers(&self) -> Option<String> {
        let headers: Vec<&str> = self
            .extra_beta_headers
            .iter()
            .map(|h| h.trim())
            .filter(|h| !h.is_empty())
            .collect();
        if headers.is_empty() {
            None
        } else {
            Some(headers.join(","))
        }
    }

    pub fn request_id(&self, has_tools: bool) -> &str {
        if has_tools {
            self.tool_override.as_deref().unwrap_or(&self.id)
        } else {
            &self.id
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

/// A raw model entry returned by the Anthropic models listing endpoint.
#[derive(Clone, Debug, Deserialize)]
pub struct ListModelEntry {
    pub id: String,
    pub display_name: String,
    pub max_input_tokens: u64,
    pub max_tokens: u64,
    #[serde(default)]
    pub capabilities: Option<ModelCapabilities>,
}

#[derive(Debug, Deserialize)]
struct ListModelsResponse {
    data: Vec<ListModelEntry>,
}

/// Fetch the list of models available to the current API key. The returned
/// models are constructed by feeding each raw entry through
/// [`Model::from_listed`].
///
/// See https://docs.claude.com/en/api/models-list.
pub async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
) -> Result<Vec<Model>> {
    let uri = format!("{api_url}/v1/models?limit=1000");

    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Anthropic-Version", "2023-06-01")
        .header("X-Api-Key", api_key.trim())
        .header("Accept", "application/json")
        .body(AsyncBody::default())
        .context("failed to build Anthropic models list request")?;

    let mut response = client
        .send(request)
        .await
        .context("failed to send Anthropic models list request")?;

    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .context("failed to read Anthropic models list response")?;

    anyhow::ensure!(
        response.status().is_success(),
        "failed to list Anthropic models: {} {}",
        response.status(),
        body,
    );

    let parsed: ListModelsResponse =
        serde_json::from_str(&body).context("failed to parse Anthropic models list response")?;

    let models = parsed
        .data
        .into_iter()
        .map(Model::from_listed)
        .collect::<Vec<_>>();
    Ok(models)
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
    Enabled {
        budget_tokens: Option<u32>,
    },
    Adaptive {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        display: Option<AdaptiveThinkingDisplay>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AdaptiveThinkingDisplay {
    Omitted,
    Summarized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Effort {
    Low,
    Medium,
    High,
    XHigh,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn listed_entry(id: &str, capabilities: ModelCapabilities) -> ListModelEntry {
        ListModelEntry {
            id: id.to_string(),
            display_name: id.to_string(),
            max_input_tokens: 200_000,
            max_tokens: 64_000,
            capabilities: Some(capabilities),
        }
    }

    #[test]
    fn from_listed_picks_adaptive_thinking_mode() {
        let entry = listed_entry(
            "claude-test-adaptive",
            ModelCapabilities {
                thinking: Some(ThinkingCapability {
                    supported: true,
                    types: Some(ThinkingTypes {
                        adaptive: SupportedCapability { supported: true },
                        enabled: SupportedCapability { supported: true },
                    }),
                }),
                ..Default::default()
            },
        );
        let model = Model::from_listed(entry);
        assert!(model.supports_thinking);
        assert!(model.supports_adaptive_thinking);
        assert_eq!(model.mode, AnthropicModelMode::AdaptiveThinking);
    }

    #[test]
    fn from_listed_picks_thinking_mode_when_only_enabled_supported() {
        let entry = listed_entry(
            "claude-test-thinking",
            ModelCapabilities {
                thinking: Some(ThinkingCapability {
                    supported: true,
                    types: Some(ThinkingTypes {
                        adaptive: SupportedCapability { supported: false },
                        enabled: SupportedCapability { supported: true },
                    }),
                }),
                ..Default::default()
            },
        );
        let model = Model::from_listed(entry);
        assert!(model.supports_thinking);
        assert!(!model.supports_adaptive_thinking);
        assert!(matches!(model.mode, AnthropicModelMode::Thinking { .. }));
    }

    #[test]
    fn from_listed_default_mode_when_no_thinking() {
        let entry = listed_entry("claude-test-default", ModelCapabilities::default());
        let model = Model::from_listed(entry);
        assert!(!model.supports_thinking);
        assert!(!model.supports_adaptive_thinking);
        assert_eq!(model.mode, AnthropicModelMode::Default);
    }

    #[test]
    fn from_listed_collects_supported_effort_levels() {
        let entry = listed_entry(
            "claude-test-effort",
            ModelCapabilities {
                effort: Some(EffortCapability {
                    supported: true,
                    low: Some(SupportedCapability { supported: true }),
                    medium: Some(SupportedCapability { supported: false }),
                    high: Some(SupportedCapability { supported: true }),
                    max: Some(SupportedCapability { supported: true }),
                    xhigh: Some(SupportedCapability { supported: true }),
                }),
                ..Default::default()
            },
        );
        let model = Model::from_listed(entry);
        assert_eq!(
            &model.supported_effort_levels,
            &[Effort::Low, Effort::High, Effort::XHigh, Effort::Max]
        );
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
