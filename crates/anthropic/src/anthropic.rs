mod supported_countries;

use anyhow::{anyhow, Context, Result};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, Stream, StreamExt};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use isahc::config::Configurable;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{pin::Pin, str::FromStr};
use strum::{EnumIter, EnumString};
use thiserror::Error;

pub use supported_countries::*;

pub const ANTHROPIC_API_URL: &'static str = "https://api.anthropic.com";

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Model {
    #[default]
    #[serde(rename = "claude-3-5-sonnet", alias = "claude-3-5-sonnet-20240620")]
    Claude3_5Sonnet,
    #[serde(rename = "claude-3-opus", alias = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet", alias = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku", alias = "claude-3-haiku-20240307")]
    Claude3Haiku,
    #[serde(rename = "custom")]
    Custom {
        name: String,
        max_tokens: usize,
        /// Override this model with a different Anthropic model for tool calls.
        tool_override: Option<String>,
    },
}

impl Model {
    pub fn from_id(id: &str) -> Result<Self> {
        if id.starts_with("claude-3-5-sonnet") {
            Ok(Self::Claude3_5Sonnet)
        } else if id.starts_with("claude-3-opus") {
            Ok(Self::Claude3Opus)
        } else if id.starts_with("claude-3-sonnet") {
            Ok(Self::Claude3Sonnet)
        } else if id.starts_with("claude-3-haiku") {
            Ok(Self::Claude3Haiku)
        } else {
            Err(anyhow!("invalid model id"))
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Model::Claude3_5Sonnet => "claude-3-5-sonnet-20240620",
            Model::Claude3Opus => "claude-3-opus-20240229",
            Model::Claude3Sonnet => "claude-3-sonnet-20240229",
            Model::Claude3Haiku => "claude-3-haiku-20240307",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Custom { name, .. } => name,
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3Haiku => 200_000,
            Self::Custom { max_tokens, .. } => *max_tokens,
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
            self.id()
        }
    }
}

pub async fn complete(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<Response, AnthropicError> {
    let uri = format!("{api_url}/v1/messages");
    let request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Anthropic-Version", "2023-06-01")
        .header("Anthropic-Beta", "tools-2024-04-04")
        .header("X-Api-Key", api_key)
        .header("Content-Type", "application/json");

    let serialized_request =
        serde_json::to_string(&request).context("failed to serialize request")?;
    let request = request_builder
        .body(AsyncBody::from(serialized_request))
        .context("failed to construct request body")?;

    let mut response = client
        .send(request)
        .await
        .context("failed to send request to Anthropic")?;
    if response.status().is_success() {
        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("failed to read response body")?;
        let response_message: Response =
            serde_json::from_slice(&body).context("failed to deserialize response body")?;
        Ok(response_message)
    } else {
        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("failed to read response body")?;
        let body_str =
            std::str::from_utf8(&body).context("failed to parse response body as UTF-8")?;
        Err(AnthropicError::Other(anyhow!(
            "Failed to connect to API: {} {}",
            response.status(),
            body_str
        )))
    }
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
    low_speed_timeout: Option<Duration>,
) -> Result<BoxStream<'static, Result<Event, AnthropicError>>, AnthropicError> {
    let request = StreamingRequest {
        base: request,
        stream: true,
    };
    let uri = format!("{api_url}/v1/messages");
    let mut request_builder = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Anthropic-Version", "2023-06-01")
        .header("Anthropic-Beta", "tools-2024-04-04")
        .header("X-Api-Key", api_key)
        .header("Content-Type", "application/json");
    if let Some(low_speed_timeout) = low_speed_timeout {
        request_builder = request_builder.low_speed_timeout(100, low_speed_timeout);
    }
    let serialized_request =
        serde_json::to_string(&request).context("failed to serialize request")?;
    let request = request_builder
        .body(AsyncBody::from(serialized_request))
        .context("failed to construct request body")?;

    let mut response = client
        .send(request)
        .await
        .context("failed to send request to Anthropic")?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        match serde_json::from_str(line) {
                            Ok(response) => Some(Ok(response)),
                            Err(error) => Some(Err(AnthropicError::Other(anyhow!(error)))),
                        }
                    }
                    Err(error) => Some(Err(AnthropicError::Other(anyhow!(error)))),
                }
            })
            .boxed())
    } else {
        let mut body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut body)
            .await
            .context("failed to read response body")?;

        let body_str =
            std::str::from_utf8(&body).context("failed to parse response body as UTF-8")?;

        match serde_json::from_str::<Event>(body_str) {
            Ok(Event::Error { error }) => Err(AnthropicError::ApiError(error)),
            Ok(_) => Err(AnthropicError::Other(anyhow!(
                "Unexpected success response while expecting an error: '{body_str}'",
            ))),
            Err(_) => Err(AnthropicError::Other(anyhow!(
                "Failed to connect to API: {} {}",
                response.status(),
                body_str,
            ))),
        }
    }
}

pub fn extract_text_from_events(
    response: impl Stream<Item = Result<Event, AnthropicError>>,
) -> impl Stream<Item = Result<String, AnthropicError>> {
    response.filter_map(|response| async move {
        match response {
            Ok(response) => match response {
                Event::ContentBlockStart { content_block, .. } => match content_block {
                    Content::Text { text } => Some(Ok(text)),
                    _ => None,
                },
                Event::ContentBlockDelta { delta, .. } => match delta {
                    ContentDelta::TextDelta { text } => Some(Ok(text)),
                    _ => None,
                },
                Event::Error { error } => Some(Err(AnthropicError::ApiError(error))),
                _ => None,
            },
            Err(error) => Some(Err(error)),
        }
    })
}

pub async fn extract_tool_args_from_events(
    tool_name: String,
    mut events: Pin<Box<dyn Send + Stream<Item = Result<Event>>>>,
) -> Result<impl Send + Stream<Item = Result<String>>> {
    let mut tool_use_index = None;
    while let Some(event) = events.next().await {
        if let Event::ContentBlockStart {
            index,
            content_block,
        } = event?
        {
            if let Content::ToolUse { name, .. } = content_block {
                if name == tool_name {
                    tool_use_index = Some(index);
                    break;
                }
            }
        }
    }

    let Some(tool_use_index) = tool_use_index else {
        return Err(anyhow!("tool not used"));
    };

    Ok(events.filter_map(move |event| {
        let result = match event {
            Err(error) => Some(Err(error)),
            Ok(Event::ContentBlockDelta { index, delta }) => match delta {
                ContentDelta::TextDelta { .. } => None,
                ContentDelta::InputJsonDelta { partial_json } => {
                    if index == tool_use_index {
                        Some(Ok(partial_json))
                    } else {
                        None
                    }
                }
            },
            _ => None,
        };

        async move { result }
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: ImageSource },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: String,
        content: String,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<Message>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: Role,
    pub content: Vec<Content>,
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
        content_block: Content,
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
    #[serde(rename = "input_json_delta")]
    InputJsonDelta { partial_json: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDelta {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
}

#[derive(Error, Debug)]
pub enum AnthropicError {
    #[error("an error occurred while interacting with the Anthropic API: {error_type}: {message}", error_type = .0.error_type, message = .0.message)]
    ApiError(ApiError),
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

/// An Anthropic API error code.
/// https://docs.anthropic.com/en/api/errors#http-errors
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
        match self.error_type.as_str() {
            "rate_limit_error" => true,
            _ => false,
        }
    }
}
