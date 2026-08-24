use anyhow::{Result, anyhow};
use futures::{AsyncReadExt, StreamExt, stream::BoxStream};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt, http,
};
use open_ai::ChatCompletionStreamEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use settings::DataCollection;
pub use settings::ModelMode;
pub use settings::OpenRouterAvailableModel as AvailableModel;
pub use settings::OpenRouterProvider as Provider;
use std::{convert::TryFrom, io, time::Duration};
use thiserror::Error;

pub const OPEN_ROUTER_API_URL: &str = "https://openrouter.ai/api/v1";
const OPEN_ROUTER_APP_TITLE: &str = "Zed";

fn extract_retry_after(headers: &http::HeaderMap) -> Option<std::time::Duration> {
    if let Some(reset) = headers.get("X-RateLimit-Reset") {
        if let Ok(s) = reset.to_str() {
            if let Ok(epoch_ms) = s.parse::<u64>() {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64;
                if epoch_ms > now {
                    return Some(std::time::Duration::from_millis(epoch_ms - now));
                }
            }
        }
    }
    None
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

impl TryFrom<String> for Role {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "system" => Ok(Self::System),
            "tool" => Ok(Self::Tool),
            _ => Err(anyhow!("invalid role '{value}'")),
        }
    }
}

impl From<Role> for String {
    fn from(val: Role) -> Self {
        match val {
            Role::User => "user".to_owned(),
            Role::Assistant => "assistant".to_owned(),
            Role::System => "system".to_owned(),
            Role::Tool => "tool".to_owned(),
        }
    }
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
    pub session_id: Option<String>,
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
        reasoning_details: Option<std::sync::Arc<serde_json::Value>>,
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
                    cache_control: None,
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
            && let MessagePart::Text {
                text,
                cache_control,
            } = &parts[0]
            && cache_control.is_none()
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
                if let MessagePart::Text { text, .. } = &parts[0] {
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
                    if let MessagePart::Text { text, .. } = part {
                        Some(text.as_str())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .concat(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CacheControlType {
    Ephemeral,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum CacheTtl {
    /// Anthropic's default ephemeral TTL (currently 5 minutes). Refreshes for
    /// free on every cache hit.
    #[serde(rename = "5m")]
    FiveMinutes,
    /// Anthropic's extended ephemeral TTL (currently 1 hour). Costs 2x base
    /// input tokens to write, but persists across longer idle gaps.
    #[serde(rename = "1h")]
    OneHour,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub struct CacheControl {
    #[serde(rename = "type")]
    pub cache_type: CacheControlType,
    /// Omitting this field uses the API's default 5-minute TTL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<CacheTtl>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessagePart {
    Text {
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cache_control: Option<CacheControl>,
    },
    #[serde(rename = "image_url")]
    Image { image_url: String },
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PromptTokensDetails {
    #[serde(default)]
    pub cached_tokens: u64,
    #[serde(default)]
    pub cache_write_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
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

#[derive(Deserialize)]
#[serde(untagged)]
enum ResponseStreamResult {
    Response(ResponseStreamEvent),
    Error(OpenRouterErrorResponse),
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
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<ResponseStreamEvent, OpenRouterError>>, OpenRouterError> {
    let headers = completion_headers(extra_headers);
    let events =
        open_ai::stream_chat_completion(client, "OpenRouter", api_url, api_key, &headers, &request)
            .await
            .map_err(OpenRouterError::from_chat_completion_request_error)?;
    Ok(events
        .filter_map(|event| async move {
            let value = match event {
                Ok(ChatCompletionStreamEvent::Data(value)) => value,
                Ok(ChatCompletionStreamEvent::Done) => return None,
                Err(error) => {
                    return Some(Err(OpenRouterError::ChatCompletion(error)));
                }
            };
            match serde_json::from_value(value) {
                Ok(ResponseStreamResult::Response(response)) => Some(Ok(response)),
                Ok(ResponseStreamResult::Error(OpenRouterErrorResponse { error })) => {
                    Some(Err(OpenRouterError::ApiError(ApiError {
                        status: None,
                        code: error.code,
                        message: error.message,
                        retry_after: None,
                    })))
                }
                Err(error) => Some(Err(OpenRouterError::DeserializeResponse(error))),
            }
        })
        .boxed())
}

fn completion_headers(extra_headers: &CustomHeaders) -> CustomHeaders {
    let mut headers = Vec::with_capacity(extra_headers.iter().len() + 2);
    headers.push((
        http::HeaderName::from_static("http-referer"),
        http::HeaderValue::from_static("https://zed.dev"),
    ));
    headers.push((
        http::HeaderName::from_static("x-title"),
        http::HeaderValue::from_static(OPEN_ROUTER_APP_TITLE),
    ));
    headers.extend(
        extra_headers
            .iter()
            .map(|(name, value)| (name.clone(), value.clone())),
    );
    CustomHeaders::new(headers)
}

pub async fn list_models(
    client: &dyn HttpClient,
    api_url: &str,
    api_key: &str,
    extra_headers: &CustomHeaders,
) -> Result<Vec<Model>, OpenRouterError> {
    let uri = format!("{api_url}/models/user");
    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "https://zed.dev")
        .header("X-Title", OPEN_ROUTER_APP_TITLE)
        .extra_headers(extra_headers)
        .body(AsyncBody::default())
        .map_err(OpenRouterError::BuildRequestBody)?;
    let host = request.uri().host().unwrap_or(api_url).to_owned();
    let mut response = client
        .send(request)
        .await
        .map_err(|error| OpenRouterError::HttpSend { host, error })?;

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
        let status = response.status();
        let error_response = match serde_json::from_str::<OpenRouterErrorResponse>(&body) {
            Ok(OpenRouterErrorResponse { error }) => error,
            Err(_) => OpenRouterErrorBody {
                code: status.as_u16(),
                message: body,
                metadata: None,
            },
        };

        Err(OpenRouterError::ApiError(ApiError {
            status: Some(status.as_u16()),
            code: error_response.code,
            message: error_response.message,
            retry_after: retry_after_with_rate_limit_default(status, response.headers()),
        }))
    }
}

#[derive(Debug)]
pub enum OpenRouterError {
    /// Failed to construct the HTTP request body
    BuildRequestBody(http::Error),

    /// Failed to send the HTTP request
    HttpSend { host: String, error: anyhow::Error },

    /// Failed to deserialize the response from JSON
    DeserializeResponse(serde_json::Error),

    /// Failed to read from response stream
    ReadResponse(io::Error),

    /// API returned an error response
    ApiError(ApiError),

    /// The shared Chat Completions transport failed.
    ChatCompletion(open_ai::RequestError),
}

impl OpenRouterError {
    fn from_chat_completion_request_error(error: open_ai::RequestError) -> Self {
        let open_ai::RequestError::HttpResponseError {
            status_code,
            body,
            headers,
            ..
        } = error
        else {
            return Self::ChatCompletion(error);
        };
        let error_response = match serde_json::from_str::<OpenRouterErrorResponse>(&body) {
            Ok(OpenRouterErrorResponse { error }) => error,
            Err(_) => OpenRouterErrorBody {
                code: status_code.as_u16(),
                message: body,
                metadata: None,
            },
        };
        Self::ApiError(ApiError {
            status: Some(status_code.as_u16()),
            code: error_response.code,
            message: error_response.message,
            retry_after: retry_after_with_rate_limit_default(status_code, &headers),
        })
    }
}

/// OpenRouter reports a rate limit's reset time via `X-RateLimit-Reset` when
/// present, but omits it on some rate-limited responses; a minute is a
/// reasonable default backoff for those.
fn retry_after_with_rate_limit_default(
    status: http_client::StatusCode,
    headers: &http::HeaderMap,
) -> Option<Duration> {
    extract_retry_after(headers).or_else(|| {
        (status == http_client::StatusCode::TOO_MANY_REQUESTS).then(|| Duration::from_secs(60))
    })
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
    /// The HTTP status remains distinct from the provider's numeric error code
    /// because OpenRouter can report different values for them, and streaming
    /// errors do not have their own HTTP response.
    pub status: Option<u16>,
    pub code: u16,
    pub message: String,
    pub retry_after: Option<Duration>,
}

// -- Conversions to `language_model_core` types --

impl From<OpenRouterError> for language_model_core::LanguageModelCompletionError {
    fn from(error: OpenRouterError) -> Self {
        let provider = language_model_core::LanguageModelProviderName::new("OpenRouter");
        match error {
            OpenRouterError::BuildRequestBody(error) => Self::BuildRequestBody { provider, error },
            OpenRouterError::HttpSend { host, error } => Self::HttpSend {
                provider,
                host,
                error,
            },
            OpenRouterError::DeserializeResponse(error) => {
                Self::DeserializeResponse { provider, error }
            }
            OpenRouterError::ReadResponse(error) => Self::ApiReadResponseError { provider, error },
            OpenRouterError::ApiError(api_error) => api_error.into(),
            OpenRouterError::ChatCompletion(error) => error.into(),
        }
    }
}

impl From<ApiError> for language_model_core::LanguageModelCompletionError {
    fn from(error: ApiError) -> Self {
        use language_model_core::ProviderErrorCategory;

        let provider = language_model_core::LanguageModelProviderName::new("OpenRouter");
        let status = error
            .status
            .and_then(|status| http_client::StatusCode::from_u16(status).ok());
        let category = http_client::StatusCode::from_u16(error.code)
            .ok()
            .map(|status| ProviderErrorCategory::from_http_status(status, &error.message))
            .filter(|category| *category != ProviderErrorCategory::Other)
            .or_else(|| {
                status.map(|status| ProviderErrorCategory::from_http_status(status, &error.message))
            })
            .unwrap_or(ProviderErrorCategory::Other);
        Self::from_provider_response(
            provider,
            status,
            Some(error.code.to_string()),
            error.message,
            error.retry_after,
            category,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use http_client::{
        FakeHttpClient, Response,
        http::{HeaderName, HeaderValue},
    };
    use std::sync::{Arc, Mutex};

    #[test]
    fn completion_uses_shared_transport_with_open_router_headers() {
        let captured_headers = Arc::new(Mutex::new(None));
        let captured_headers_for_handler = captured_headers.clone();
        let client = FakeHttpClient::create(move |request| {
            let captured_headers = captured_headers_for_handler.clone();
            async move {
                captured_headers
                    .lock()
                    .expect("captured headers lock")
                    .replace(request.headers().clone());
                Ok(Response::builder()
                    .status(200)
                    .body(AsyncBody::from(concat!(
                        "data: {\"id\":\"response-1\",\"created\":1,\"model\":\"vendor/model\",\"choices\":[],\"usage\":null}\n\n",
                        "data: [DONE]\n\n"
                    )))?)
            }
        });
        let extra_headers = CustomHeaders::new(vec![(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom-value"),
        )]);
        let request = Request {
            model: "vendor/model".to_string(),
            messages: vec![RequestMessage::User {
                content: MessageContent::Plain("Hello".to_string()),
            }],
            stream: true,
            session_id: None,
            max_tokens: None,
            stop: Vec::new(),
            temperature: 0.4,
            tool_choice: None,
            parallel_tool_calls: None,
            tools: Vec::new(),
            reasoning: None,
            usage: RequestUsage { include: true },
            provider: None,
        };

        let responses = block_on(async {
            stream_completion(
                client.as_ref(),
                OPEN_ROUTER_API_URL,
                "secret",
                request,
                &extra_headers,
            )
            .await
            .expect("streaming request")
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()
            .expect("stream responses")
        });

        assert_eq!(responses.len(), 1);
        assert_eq!(responses[0].model, "vendor/model");
        let headers = captured_headers.lock().expect("captured headers lock");
        let headers = headers.as_ref().expect("captured headers");
        assert_eq!(headers["http-referer"], "https://zed.dev");
        assert_eq!(headers["x-title"], OPEN_ROUTER_APP_TITLE);
        assert_eq!(headers["x-custom-header"], "custom-value");
    }

    #[test]
    fn shared_transport_errors_retain_language_model_classification() {
        let error = OpenRouterError::ChatCompletion(open_ai::RequestError::HttpSend {
            provider: "OpenRouter".to_string(),
            host: "openrouter.ai".to_string(),
            error: anyhow!("network unavailable"),
        });
        let error = language_model_core::LanguageModelCompletionError::from(error);

        assert!(matches!(
            error,
            language_model_core::LanguageModelCompletionError::HttpSend {
                provider,
                host,
                ..
            } if provider.0 == "OpenRouter" && host == "openrouter.ai"
        ));
    }

    #[test]
    fn provider_code_remains_distinct_from_http_status() {
        let error = OpenRouterError::from_chat_completion_request_error(
            open_ai::RequestError::HttpResponseError {
                provider: "OpenRouter".to_string(),
                status_code: http_client::StatusCode::INTERNAL_SERVER_ERROR,
                body: r#"{"error":{"code":499,"message":"upstream provider failed"}}"#.to_string(),
                headers: Box::default(),
            },
        );
        let OpenRouterError::ApiError(api_error) = &error else {
            panic!("expected ApiError, got {error:?}");
        };
        assert_eq!(api_error.status, Some(500));
        assert_eq!(api_error.code, 499);

        let completion_error = language_model_core::LanguageModelCompletionError::from(error);
        assert!(matches!(
            completion_error,
            language_model_core::LanguageModelCompletionError::ProviderRejection {
                status: Some(status),
                code: Some(code),
                category: language_model_core::ProviderErrorCategory::InternalServer,
                ..
            } if status == http_client::StatusCode::INTERNAL_SERVER_ERROR && code == "499"
        ));
    }

    #[test]
    fn streaming_provider_code_does_not_become_http_status() {
        let completion_error = language_model_core::LanguageModelCompletionError::from(ApiError {
            status: None,
            code: 402,
            message: "Insufficient credits".to_string(),
            retry_after: None,
        });

        assert!(matches!(
            completion_error,
            language_model_core::LanguageModelCompletionError::ProviderRejection {
                status: None,
                code: Some(code),
                category: language_model_core::ProviderErrorCategory::PaymentRequired,
                ..
            } if code == "402"
        ));
    }

    #[test]
    fn streaming_server_error_remains_retryable_without_http_status() {
        let completion_error = language_model_core::LanguageModelCompletionError::from(ApiError {
            status: None,
            code: 502,
            message: "Upstream provider failed".to_string(),
            retry_after: None,
        });

        assert!(matches!(
            &completion_error,
            language_model_core::LanguageModelCompletionError::ProviderRejection {
                status: None,
                code: Some(code),
                category: language_model_core::ProviderErrorCategory::InternalServer,
                ..
            } if code == "502"
        ));
        assert_eq!(
            completion_error.retry_delay(1),
            Some(Duration::from_secs(5))
        );
    }
}
