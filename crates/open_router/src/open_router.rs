use anyhow::{Result, anyhow};
use futures::{AsyncBufReadExt, AsyncReadExt, StreamExt, io::BufReader, stream::BoxStream};
use http_client::StatusCode;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest, http};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use settings::DataCollection;
pub use settings::ModelMode;
pub use settings::OpenRouterAvailableModel as AvailableModel;
pub use settings::OpenRouterProvider as Provider;
use std::{io, time::Duration};
use strum::EnumString;
use thiserror::Error;

pub const OPEN_ROUTER_API_URL: &str = "https://openrouter.ai/api/v1";

fn extract_retry_after(headers: &http::HeaderMap) -> Option<std::time::Duration> {
    let epoch_ms = headers
        .get("X-RateLimit-Reset")
        .and_then(|header| header.to_str().ok())
        .and_then(|reset| reset.parse::<u64>().ok())?;

    let retry_at = Duration::from_millis(epoch_ms);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("SystemTime before Unix Epoch");

    Some(retry_at.saturating_sub(now))
}

fn is_none_or_empty<T: AsRef<[U]>, U>(opt: &Option<T>) -> bool {
    opt.as_ref().is_none_or(|v| v.as_ref().is_empty())
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: u64,
    pub supports_tools: Option<bool>,
    pub supports_images: Option<bool>,
    #[serde(default)]
    pub mode: ModelMode,
    pub provider: Option<Provider>,
}

impl Model {
    pub fn default() -> Self {
        Self::new(
            "openrouter/auto",
            Some("Auto Router"),
            Some(2000000),
            Some(true),
            Some(false),
            Some(ModelMode::Default),
            None,
        )
    }

    pub fn new(
        name: &str,
        display_name: Option<&str>,
        max_tokens: Option<u64>,
        supports_tools: Option<bool>,
        supports_images: Option<bool>,
        mode: Option<ModelMode>,
        provider: Option<Provider>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            display_name: display_name.map(|s| s.to_owned()),
            max_tokens: max_tokens.unwrap_or(2000000),
            supports_tools,
            supports_images,
            mode: mode.unwrap_or(ModelMode::Default),
            provider,
        }
    }

    pub fn id(&self) -> &str {
        &self.name
    }

    pub fn display_name(&self) -> &str {
        self.display_name.as_ref().unwrap_or(&self.name)
    }

    pub fn max_token_count(&self) -> u64 {
        self.max_tokens
    }

    pub fn max_output_tokens(&self) -> Option<u64> {
        None
    }

    pub fn supports_tool_calls(&self) -> bool {
        self.supports_tools.unwrap_or(false)
    }

    pub fn supports_parallel_tool_calls(&self) -> bool {
        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<RequestMessage>,
    pub stream: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,
    pub temperature: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
    pub usage: RequestUsage,
    pub provider: Option<Provider>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RequestUsage {
    pub include: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    #[serde(untagged)]
    Other(ToolDefinition),
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    #[allow(dead_code)]
    Function { function: FunctionDefinition },
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reasoning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum RequestMessage {
    Assistant {
        content: Option<MessageContent>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tool_calls: Vec<ToolCall>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reasoning_details: Option<serde_json::Value>,
    },
    User {
        content: MessageContent,
    },
    System {
        content: MessageContent,
    },
    Tool {
        content: MessageContent,
        tool_call_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    Plain(String),
    Multipart(Vec<MessagePart>),
}

impl MessageContent {
    pub fn empty() -> Self {
        Self::Plain(String::new())
    }

    pub fn push_part(&mut self, part: MessagePart) {
        match self {
            Self::Plain(text) if text.is_empty() => {
                *self = Self::Multipart(vec![part]);
            }
            Self::Plain(text) => {
                let text_part = MessagePart::Text {
                    text: std::mem::take(text),
                };
                *self = Self::Multipart(vec![text_part, part]);
            }
            Self::Multipart(parts) => parts.push(part),
        }
    }
}

impl From<Vec<MessagePart>> for MessageContent {
    fn from(parts: Vec<MessagePart>) -> Self {
        if parts.len() == 1
            && let MessagePart::Text { text } = &parts[0]
        {
            return Self::Plain(text.clone());
        }
        Self::Multipart(parts)
    }
}

impl From<String> for MessageContent {
    fn from(text: String) -> Self {
        Self::Plain(text)
    }
}

impl From<&str> for MessageContent {
    fn from(text: &str) -> Self {
        Self::Plain(text.to_string())
    }
}

impl MessageContent {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Plain(text) => Some(text),
            Self::Multipart(parts) if parts.len() == 1 => {
                if let MessagePart::Text { text } = &parts[0] {
                    Some(text)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn to_text(&self) -> String {
        match self {
            Self::Plain(text) => text.clone(),
            Self::Multipart(parts) => parts
                .iter()
                .filter_map(|part| {
                    if let MessagePart::Text { text } = part {
                        Some(text.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessagePart {
    Text {
        text: String,
    },
    #[serde(rename = "image_url")]
    Image {
        image_url: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCall {
    pub id: String,
    #[serde(flatten)]
    pub content: ToolCallContent,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolCallContent {
    Function { function: FunctionContent },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionContent {
    pub name: String,
    pub arguments: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseMessageDelta {
    pub role: Option<Role>,
    pub content: Option<String>,
    pub reasoning: Option<String>,
    #[serde(default, skip_serializing_if = "is_none_or_empty")]
    pub tool_calls: Option<Vec<ToolCallChunk>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_details: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ToolCallChunk {
    pub index: usize,
    pub id: Option<String>,
    pub function: Option<FunctionChunk>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FunctionChunk {
    pub name: Option<String>,
    pub arguments: Option<String>,
    #[serde(default)]
    pub thought_signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessageDelta,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStreamEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: u32,
    pub message: RequestMessage,
    pub finish_reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListModelsResponse {
    pub data: Vec<ModelEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    pub name: String,
    pub created: usize,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_length: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_parameters: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<ModelArchitecture>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ModelArchitecture {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_modalities: Vec<String>,
}

pub async fn stream_completion(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    request: Request,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent, OpenRouterError>>, OpenRouterError> {
    let uri = format!("{api_url}/chat/completions");
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "https://zed.dev")
        .header("X-Title", "Zed Editor")
        .body(AsyncBody::from(
            serde_json::to_string(&request).map_err(OpenRouterError::SerializeRequest)?,
        ))
        .map_err(OpenRouterError::BuildRequestBody)?;
    let mut response = client
        .send(request)
        .await
        .map_err(OpenRouterError::HttpSend)?;

    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(|line| async move {
                match line {
                    Ok(line) => {
                        if line.starts_with(':') {
                            return None;
                        }

                        let line = line.strip_prefix("data: ")?;
                        if line == "[DONE]" {
                            None
                        } else {
                            match serde_json::from_str::<ResponseStreamEvent>(line) {
                                Ok(response) => Some(Ok(response)),
                                Err(error) => {
                                    if line.trim().is_empty() {
                                        None
                                    } else {
                                        Some(Err(OpenRouterError::DeserializeResponse(error)))
                                    }
                                }
                            }
                        }
                    }
                    Err(error) => Some(Err(OpenRouterError::ReadResponse(error))),
                }
            })
            .boxed())
    } else {
        let code = ApiErrorCode::from(response.status());

        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(OpenRouterError::ReadResponse)?;

        let error_response = match serde_json::from_str::<OpenRouterErrorResponse>(&body) {
            Ok(OpenRouterErrorResponse { error }) => error,
            Err(_) => OpenRouterErrorBody {
                code: response.status().as_u16(),
                message: body,
                metadata: None,
            },
        };

        match code {
            ApiErrorCode::RateLimitError => {
                let retry_after = extract_retry_after(response.headers());
                Err(OpenRouterError::RateLimit {
                    retry_after: retry_after.unwrap_or_else(|| std::time::Duration::from_secs(60)),
                })
            }
            ApiErrorCode::OverloadedError => {
                let retry_after = extract_retry_after(response.headers());
                Err(OpenRouterError::ServerOverloaded { retry_after })
            }
            _ => Err(OpenRouterError::ApiError(ApiError {
                code: code,
                message: error_response.message,
            })),
        }
    }
}

pub async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
) -> Result<Vec<Model>, OpenRouterError> {
    let uri = format!("{api_url}/models/user");
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "https://zed.dev")
        .header("X-Title", "Zed Editor")
        .body(AsyncBody::default())
        .map_err(OpenRouterError::BuildRequestBody)?;
    let mut response = client
        .send(request)
        .await
        .map_err(OpenRouterError::HttpSend)?;

    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .map_err(OpenRouterError::ReadResponse)?;

    if response.status().is_success() {
        let response: ListModelsResponse =
            serde_json::from_str(&body).map_err(OpenRouterError::DeserializeResponse)?;

        let models = response
            .data
            .into_iter()
            .map(|entry| Model {
                name: entry.id,
                // OpenRouter returns display names in the format "provider_name: model_name".
                // When displayed in the UI, these names can get truncated from the right.
                // Since users typically already know the provider, we extract just the model name
                // portion (after the colon) to create a more concise and user-friendly label
                // for the model dropdown in the agent panel.
                display_name: Some(
                    entry
                        .name
                        .split(':')
                        .next_back()
                        .unwrap_or(&entry.name)
                        .trim()
                        .to_string(),
                ),
                max_tokens: entry.context_length.unwrap_or(2000000),
                supports_tools: Some(entry.supported_parameters.contains(&"tools".to_string())),
                supports_images: Some(
                    entry
                        .architecture
                        .as_ref()
                        .map(|arch| arch.input_modalities.contains(&"image".to_string()))
                        .unwrap_or(false),
                ),
                mode: if entry
                    .supported_parameters
                    .contains(&"reasoning".to_string())
                {
                    ModelMode::Thinking {
                        budget_tokens: Some(4_096),
                    }
                } else {
                    ModelMode::Default
                },
                provider: None,
            })
            .collect();

        Ok(models)
    } else {
        let code = ApiErrorCode::from(response.status());

        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(OpenRouterError::ReadResponse)?;

        let error_response = match serde_json::from_str::<OpenRouterErrorResponse>(&body) {
            Ok(OpenRouterErrorResponse { error }) => error,
            Err(_) => OpenRouterErrorBody {
                code: response.status().as_u16(),
                message: body,
                metadata: None,
            },
        };

        match code {
            ApiErrorCode::RateLimitError => {
                let retry_after = extract_retry_after(response.headers());
                Err(OpenRouterError::RateLimit {
                    retry_after: retry_after.unwrap_or_else(|| std::time::Duration::from_secs(60)),
                })
            }
            ApiErrorCode::OverloadedError => {
                let retry_after = extract_retry_after(response.headers());
                Err(OpenRouterError::ServerOverloaded { retry_after })
            }
            _ => Err(OpenRouterError::ApiError(ApiError {
                code: code,
                message: error_response.message,
            })),
        }
    }
}

#[derive(Debug)]
pub enum OpenRouterError {
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

    /// Rate limit exceeded
    RateLimit { retry_after: Duration },

    /// Server overloaded
    ServerOverloaded { retry_after: Option<Duration> },

    /// API returned an error response
    ApiError(ApiError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenRouterErrorBody {
    pub code: u16,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenRouterErrorResponse {
    pub error: OpenRouterErrorBody,
}

#[derive(Debug, Serialize, Deserialize, Error)]
#[error("OpenRouter API Error: {code}: {message}")]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub message: String,
}

/// An OpenRouter API error code.
/// <https://openrouter.ai/docs/api/reference/errors-and-debugging#error-codes>
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum ApiErrorCode {
    /// 400: Bad Request (invalid or missing params, CORS)
    InvalidRequestError,
    /// 401: Invalid credentials (OAuth session expired, disabled/invalid API key)
    AuthenticationError,
    /// 402: Your account or API key has insufficient credits. Add more credits and retry the request.
    PaymentRequiredError,
    /// 403: Your chosen model requires moderation and your input was flagged
    PermissionError,
    /// 408: Your request timed out
    RequestTimedOut,
    /// 429: You are being rate limited
    RateLimitError,
    /// 502: Your chosen model is down or we received an invalid response from it
    ApiError,
    /// 503: There is no available model provider that meets your routing requirements
    OverloadedError,
}

impl std::fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ApiErrorCode::InvalidRequestError => "invalid_request_error",
            ApiErrorCode::AuthenticationError => "authentication_error",
            ApiErrorCode::PaymentRequiredError => "payment_required_error",
            ApiErrorCode::PermissionError => "permission_error",
            ApiErrorCode::RequestTimedOut => "request_timed_out",
            ApiErrorCode::RateLimitError => "rate_limit_error",
            ApiErrorCode::ApiError => "api_error",
            ApiErrorCode::OverloadedError => "overloaded_error",
        };
        write!(f, "{s}")
    }
}

impl From<StatusCode> for ApiErrorCode {
    fn from(value: StatusCode) -> Self {
        match value {
            StatusCode::BAD_REQUEST => Self::InvalidRequestError,
            StatusCode::UNAUTHORIZED => Self::AuthenticationError,
            StatusCode::PAYMENT_REQUIRED => Self::PaymentRequiredError,
            StatusCode::FORBIDDEN => Self::PermissionError,
            StatusCode::REQUEST_TIMEOUT => Self::RequestTimedOut,
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimitError,
            StatusCode::BAD_GATEWAY => Self::ApiError,
            StatusCode::SERVICE_UNAVAILABLE => Self::OverloadedError,
            _ => Self::ApiError,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use http_client::http::{HeaderMap, HeaderValue};

    use crate::extract_retry_after;

    #[test]
    fn extract_retry_reset_passed() {
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let reset_header = HeaderValue::from_str(&format!("{}", since_epoch.as_millis())).unwrap();

        let mut map = HeaderMap::default();
        map.append("X-RateLimit-Reset", reset_header);

        assert_eq!(extract_retry_after(&map), Some(Duration::ZERO));
    }
}
