mod api_key;
mod model;
mod rate_limiter;
mod registry;
mod request;
mod role;
mod telemetry;
pub mod tool_schema;

#[cfg(any(test, feature = "test-support"))]
pub mod fake_provider;

use anthropic::{AnthropicError, parse_prompt_too_long};
use anyhow::{Result, anyhow};
use client::Client;
use cloud_llm_client::CompletionRequestStatus;
use futures::FutureExt;
use futures::{StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, SharedString, Task, Window};
use http_client::{StatusCode, http};
use icons::IconName;
use open_router::OpenRouterError;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
pub use settings::LanguageModelCacheConfiguration;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::{fmt, io};
use thiserror::Error;
use util::serde::is_default;

pub use crate::api_key::{ApiKey, ApiKeyState};
pub use crate::model::*;
pub use crate::rate_limiter::*;
pub use crate::registry::*;
pub use crate::request::*;
pub use crate::role::*;
pub use crate::telemetry::*;
pub use crate::tool_schema::LanguageModelToolSchemaFormat;
pub use zed_env_vars::{EnvVar, env_var};

pub const ANTHROPIC_PROVIDER_ID: LanguageModelProviderId =
    LanguageModelProviderId::new("anthropic");
pub const ANTHROPIC_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Anthropic");

pub const GOOGLE_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("google");
pub const GOOGLE_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Google AI");

pub const OPEN_AI_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openai");
pub const OPEN_AI_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("OpenAI");

pub const X_AI_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("x_ai");
pub const X_AI_PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("xAI");

pub const ZED_CLOUD_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("zed.dev");
pub const ZED_CLOUD_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Zed");

pub fn init(client: Arc<Client>, cx: &mut App) {
    init_settings(cx);
    RefreshLlmTokenListener::register(client, cx);
}

pub fn init_settings(cx: &mut App) {
    registry::init(cx);
}

/// A completion event from a language model.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LanguageModelCompletionEvent {
    Queued {
        position: usize,
    },
    Started,
    Stop(StopReason),
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    RedactedThinking {
        data: String,
    },
    ToolUse(LanguageModelToolUse),
    ToolUseJsonParseError {
        id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        raw_input: Arc<str>,
        json_parse_error: String,
    },
    StartMessage {
        message_id: String,
    },
    ReasoningDetails(serde_json::Value),
    UsageUpdate(TokenUsage),
}

impl LanguageModelCompletionEvent {
    pub fn from_completion_request_status(
        status: CompletionRequestStatus,
        upstream_provider: LanguageModelProviderName,
    ) -> Result<Self, LanguageModelCompletionError> {
        match status {
            CompletionRequestStatus::Queued { position } => {
                Ok(LanguageModelCompletionEvent::Queued { position })
            }
            CompletionRequestStatus::Started => Ok(LanguageModelCompletionEvent::Started),
            CompletionRequestStatus::UsageUpdated { .. }
            | CompletionRequestStatus::ToolUseLimitReached => Err(
                LanguageModelCompletionError::Other(anyhow!("Unexpected status: {status:?}")),
            ),
            CompletionRequestStatus::Failed {
                code,
                message,
                request_id: _,
                retry_after,
            } => Err(LanguageModelCompletionError::from_cloud_failure(
                upstream_provider,
                code,
                message,
                retry_after.map(Duration::from_secs_f64),
            )),
        }
    }
}

#[derive(Error, Debug)]
pub enum LanguageModelCompletionError {
    #[error("prompt too large for context window")]
    PromptTooLarge { tokens: Option<u64> },
    #[error("missing {provider} API key")]
    NoApiKey { provider: LanguageModelProviderName },
    #[error("{provider}'s API rate limit exceeded")]
    RateLimitExceeded {
        provider: LanguageModelProviderName,
        retry_after: Option<Duration>,
    },
    #[error("{provider}'s API servers are overloaded right now")]
    ServerOverloaded {
        provider: LanguageModelProviderName,
        retry_after: Option<Duration>,
    },
    #[error("{provider}'s API server reported an internal server error: {message}")]
    ApiInternalServerError {
        provider: LanguageModelProviderName,
        message: String,
    },
    #[error("{message}")]
    UpstreamProviderError {
        message: String,
        status: StatusCode,
        retry_after: Option<Duration>,
    },
    #[error("HTTP response error from {provider}'s API: status {status_code} - {message:?}")]
    HttpResponseError {
        provider: LanguageModelProviderName,
        status_code: StatusCode,
        message: String,
    },

    // Client errors
    #[error("invalid request format to {provider}'s API: {message}")]
    BadRequestFormat {
        provider: LanguageModelProviderName,
        message: String,
    },
    #[error("authentication error with {provider}'s API: {message}")]
    AuthenticationError {
        provider: LanguageModelProviderName,
        message: String,
    },
    #[error("Permission error with {provider}'s API: {message}")]
    PermissionError {
        provider: LanguageModelProviderName,
        message: String,
    },
    #[error("language model provider API endpoint not found")]
    ApiEndpointNotFound { provider: LanguageModelProviderName },
    #[error("I/O error reading response from {provider}'s API")]
    ApiReadResponseError {
        provider: LanguageModelProviderName,
        #[source]
        error: io::Error,
    },
    #[error("error serializing request to {provider} API")]
    SerializeRequest {
        provider: LanguageModelProviderName,
        #[source]
        error: serde_json::Error,
    },
    #[error("error building request body to {provider} API")]
    BuildRequestBody {
        provider: LanguageModelProviderName,
        #[source]
        error: http::Error,
    },
    #[error("error sending HTTP request to {provider} API")]
    HttpSend {
        provider: LanguageModelProviderName,
        #[source]
        error: anyhow::Error,
    },
    #[error("error deserializing {provider} API response")]
    DeserializeResponse {
        provider: LanguageModelProviderName,
        #[source]
        error: serde_json::Error,
    },

    // TODO: Ideally this would be removed in favor of having a comprehensive list of errors.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl LanguageModelCompletionError {
    fn parse_upstream_error_json(message: &str) -> Option<(StatusCode, String)> {
        let error_json = serde_json::from_str::<serde_json::Value>(message).ok()?;
        let upstream_status = error_json
            .get("upstream_status")
            .and_then(|v| v.as_u64())
            .and_then(|status| u16::try_from(status).ok())
            .and_then(|status| StatusCode::from_u16(status).ok())?;
        let inner_message = error_json
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or(message)
            .to_string();
        Some((upstream_status, inner_message))
    }

    pub fn from_cloud_failure(
        upstream_provider: LanguageModelProviderName,
        code: String,
        message: String,
        retry_after: Option<Duration>,
    ) -> Self {
        if let Some(tokens) = parse_prompt_too_long(&message) {
            // TODO: currently Anthropic PAYLOAD_TOO_LARGE response may cause INTERNAL_SERVER_ERROR
            // to be reported. This is a temporary workaround to handle this in the case where the
            // token limit has been exceeded.
            Self::PromptTooLarge {
                tokens: Some(tokens),
            }
        } else if code == "upstream_http_error" {
            if let Some((upstream_status, inner_message)) =
                Self::parse_upstream_error_json(&message)
            {
                return Self::from_http_status(
                    upstream_provider,
                    upstream_status,
                    inner_message,
                    retry_after,
                );
            }
            anyhow!("completion request failed, code: {code}, message: {message}").into()
        } else if let Some(status_code) = code
            .strip_prefix("upstream_http_")
            .and_then(|code| StatusCode::from_str(code).ok())
        {
            Self::from_http_status(upstream_provider, status_code, message, retry_after)
        } else if let Some(status_code) = code
            .strip_prefix("http_")
            .and_then(|code| StatusCode::from_str(code).ok())
        {
            Self::from_http_status(ZED_CLOUD_PROVIDER_NAME, status_code, message, retry_after)
        } else {
            anyhow!("completion request failed, code: {code}, message: {message}").into()
        }
    }

    pub fn from_http_status(
        provider: LanguageModelProviderName,
        status_code: StatusCode,
        message: String,
        retry_after: Option<Duration>,
    ) -> Self {
        match status_code {
            StatusCode::BAD_REQUEST => Self::BadRequestFormat { provider, message },
            StatusCode::UNAUTHORIZED => Self::AuthenticationError { provider, message },
            StatusCode::FORBIDDEN => Self::PermissionError { provider, message },
            StatusCode::NOT_FOUND => Self::ApiEndpointNotFound { provider },
            StatusCode::PAYLOAD_TOO_LARGE => Self::PromptTooLarge {
                tokens: parse_prompt_too_long(&message),
            },
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimitExceeded {
                provider,
                retry_after,
            },
            StatusCode::INTERNAL_SERVER_ERROR => Self::ApiInternalServerError { provider, message },
            StatusCode::SERVICE_UNAVAILABLE => Self::ServerOverloaded {
                provider,
                retry_after,
            },
            _ if status_code.as_u16() == 529 => Self::ServerOverloaded {
                provider,
                retry_after,
            },
            _ => Self::HttpResponseError {
                provider,
                status_code,
                message,
            },
        }
    }
}

impl From<AnthropicError> for LanguageModelCompletionError {
    fn from(error: AnthropicError) -> Self {
        let provider = ANTHROPIC_PROVIDER_NAME;
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

impl From<anthropic::ApiError> for LanguageModelCompletionError {
    fn from(error: anthropic::ApiError) -> Self {
        use anthropic::ApiErrorCode::*;
        let provider = ANTHROPIC_PROVIDER_NAME;
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
                    tokens: parse_prompt_too_long(&error.message),
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

impl From<open_ai::RequestError> for LanguageModelCompletionError {
    fn from(error: open_ai::RequestError) -> Self {
        match error {
            open_ai::RequestError::HttpResponseError {
                provider,
                status_code,
                body,
                headers,
            } => {
                let retry_after = headers
                    .get(http::header::RETRY_AFTER)
                    .and_then(|val| val.to_str().ok()?.parse::<u64>().ok())
                    .map(Duration::from_secs);

                Self::from_http_status(provider.into(), status_code, body, retry_after)
            }
            open_ai::RequestError::Other(e) => Self::Other(e),
        }
    }
}

impl From<OpenRouterError> for LanguageModelCompletionError {
    fn from(error: OpenRouterError) -> Self {
        let provider = LanguageModelProviderName::new("OpenRouter");
        match error {
            OpenRouterError::SerializeRequest(error) => Self::SerializeRequest { provider, error },
            OpenRouterError::BuildRequestBody(error) => Self::BuildRequestBody { provider, error },
            OpenRouterError::HttpSend(error) => Self::HttpSend { provider, error },
            OpenRouterError::DeserializeResponse(error) => {
                Self::DeserializeResponse { provider, error }
            }
            OpenRouterError::ReadResponse(error) => Self::ApiReadResponseError { provider, error },
            OpenRouterError::RateLimit { retry_after } => Self::RateLimitExceeded {
                provider,
                retry_after: Some(retry_after),
            },
            OpenRouterError::ServerOverloaded { retry_after } => Self::ServerOverloaded {
                provider,
                retry_after,
            },
            OpenRouterError::ApiError(api_error) => api_error.into(),
        }
    }
}

impl From<open_router::ApiError> for LanguageModelCompletionError {
    fn from(error: open_router::ApiError) -> Self {
        use open_router::ApiErrorCode::*;
        let provider = LanguageModelProviderName::new("OpenRouter");
        match error.code {
            InvalidRequestError => Self::BadRequestFormat {
                provider,
                message: error.message,
            },
            AuthenticationError => Self::AuthenticationError {
                provider,
                message: error.message,
            },
            PaymentRequiredError => Self::AuthenticationError {
                provider,
                message: format!("Payment required: {}", error.message),
            },
            PermissionError => Self::PermissionError {
                provider,
                message: error.message,
            },
            RequestTimedOut => Self::HttpResponseError {
                provider,
                status_code: StatusCode::REQUEST_TIMEOUT,
                message: error.message,
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
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    ToolUse,
    Refusal,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    #[serde(default, skip_serializing_if = "is_default")]
    pub input_tokens: u64,
    #[serde(default, skip_serializing_if = "is_default")]
    pub output_tokens: u64,
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_creation_input_tokens: u64,
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_read_input_tokens: u64,
}

impl TokenUsage {
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens
            + self.output_tokens
            + self.cache_read_input_tokens
            + self.cache_creation_input_tokens
    }
}

impl Add<TokenUsage> for TokenUsage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            input_tokens: self.input_tokens + other.input_tokens,
            output_tokens: self.output_tokens + other.output_tokens,
            cache_creation_input_tokens: self.cache_creation_input_tokens
                + other.cache_creation_input_tokens,
            cache_read_input_tokens: self.cache_read_input_tokens + other.cache_read_input_tokens,
        }
    }
}

impl Sub<TokenUsage> for TokenUsage {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            input_tokens: self.input_tokens - other.input_tokens,
            output_tokens: self.output_tokens - other.output_tokens,
            cache_creation_input_tokens: self.cache_creation_input_tokens
                - other.cache_creation_input_tokens,
            cache_read_input_tokens: self.cache_read_input_tokens - other.cache_read_input_tokens,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelToolUseId(Arc<str>);

impl fmt::Display for LanguageModelToolUseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for LanguageModelToolUseId
where
    T: Into<Arc<str>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelToolUse {
    pub id: LanguageModelToolUseId,
    pub name: Arc<str>,
    pub raw_input: String,
    pub input: serde_json::Value,
    pub is_input_complete: bool,
    /// Thought signature the model sent us. Some models require that this
    /// signature be preserved and sent back in conversation history for validation.
    pub thought_signature: Option<String>,
}

pub struct LanguageModelTextStream {
    pub message_id: Option<String>,
    pub stream: BoxStream<'static, Result<String, LanguageModelCompletionError>>,
    // Has complete token usage after the stream has finished
    pub last_token_usage: Arc<Mutex<TokenUsage>>,
}

impl Default for LanguageModelTextStream {
    fn default() -> Self {
        Self {
            message_id: None,
            stream: Box::pin(futures::stream::empty()),
            last_token_usage: Arc::new(Mutex::new(TokenUsage::default())),
        }
    }
}

pub trait LanguageModel: Send + Sync {
    fn id(&self) -> LanguageModelId;
    fn name(&self) -> LanguageModelName;
    fn provider_id(&self) -> LanguageModelProviderId;
    fn provider_name(&self) -> LanguageModelProviderName;
    fn upstream_provider_id(&self) -> LanguageModelProviderId {
        self.provider_id()
    }
    fn upstream_provider_name(&self) -> LanguageModelProviderName {
        self.provider_name()
    }

    fn telemetry_id(&self) -> String;

    fn api_key(&self, _cx: &App) -> Option<String> {
        None
    }

    /// Whether this model supports extended thinking.
    fn supports_thinking(&self) -> bool {
        false
    }

    /// Whether this model supports images
    fn supports_images(&self) -> bool;

    /// Whether this model supports tools.
    fn supports_tools(&self) -> bool;

    /// Whether this model supports choosing which tool to use.
    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool;

    /// Returns whether this model or provider supports streaming tool calls;
    fn supports_streaming_tools(&self) -> bool {
        false
    }

    /// Returns whether this model/provider reports accurate split input/output token counts.
    /// When true, the UI may show separate input/output token indicators.
    fn supports_split_token_display(&self) -> bool {
        false
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchema
    }

    fn max_token_count(&self) -> u64;
    fn max_output_tokens(&self) -> Option<u64> {
        None
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>>;

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            LanguageModelCompletionError,
        >,
    >;

    fn stream_completion_text(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelTextStream, LanguageModelCompletionError>> {
        let future = self.stream_completion(request, cx);

        async move {
            let events = future.await?;
            let mut events = events.fuse();
            let mut message_id = None;
            let mut first_item_text = None;
            let last_token_usage = Arc::new(Mutex::new(TokenUsage::default()));

            if let Some(first_event) = events.next().await {
                match first_event {
                    Ok(LanguageModelCompletionEvent::StartMessage { message_id: id }) => {
                        message_id = Some(id);
                    }
                    Ok(LanguageModelCompletionEvent::Text(text)) => {
                        first_item_text = Some(text);
                    }
                    _ => (),
                }
            }

            let stream = futures::stream::iter(first_item_text.map(Ok))
                .chain(events.filter_map({
                    let last_token_usage = last_token_usage.clone();
                    move |result| {
                        let last_token_usage = last_token_usage.clone();
                        async move {
                            match result {
                                Ok(LanguageModelCompletionEvent::Queued { .. }) => None,
                                Ok(LanguageModelCompletionEvent::Started) => None,
                                Ok(LanguageModelCompletionEvent::StartMessage { .. }) => None,
                                Ok(LanguageModelCompletionEvent::Text(text)) => Some(Ok(text)),
                                Ok(LanguageModelCompletionEvent::Thinking { .. }) => None,
                                Ok(LanguageModelCompletionEvent::RedactedThinking { .. }) => None,
                                Ok(LanguageModelCompletionEvent::ReasoningDetails(_)) => None,
                                Ok(LanguageModelCompletionEvent::Stop(_)) => None,
                                Ok(LanguageModelCompletionEvent::ToolUse(_)) => None,
                                Ok(LanguageModelCompletionEvent::ToolUseJsonParseError {
                                    ..
                                }) => None,
                                Ok(LanguageModelCompletionEvent::UsageUpdate(token_usage)) => {
                                    *last_token_usage.lock() = token_usage;
                                    None
                                }
                                Err(err) => Some(Err(err)),
                            }
                        }
                    }
                }))
                .boxed();

            Ok(LanguageModelTextStream {
                message_id,
                stream,
                last_token_usage,
            })
        }
        .boxed()
    }

    fn stream_completion_tool(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelToolUse, LanguageModelCompletionError>> {
        let future = self.stream_completion(request, cx);

        async move {
            let events = future.await?;
            let mut events = events.fuse();

            // Iterate through events until we find a complete ToolUse
            while let Some(event) = events.next().await {
                match event {
                    Ok(LanguageModelCompletionEvent::ToolUse(tool_use))
                        if tool_use.is_input_complete =>
                    {
                        return Ok(tool_use);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                    _ => {}
                }
            }

            // Stream ended without a complete tool use
            Err(LanguageModelCompletionError::Other(anyhow::anyhow!(
                "Stream ended without receiving a complete tool use"
            )))
        }
        .boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        None
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &fake_provider::FakeLanguageModel {
        unimplemented!()
    }
}

impl std::fmt::Debug for dyn LanguageModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("<dyn LanguageModel>")
            .field("id", &self.id())
            .field("name", &self.name())
            .field("provider_id", &self.provider_id())
            .field("provider_name", &self.provider_name())
            .field("upstream_provider_name", &self.upstream_provider_name())
            .field("upstream_provider_id", &self.upstream_provider_id())
            .field("upstream_provider_id", &self.upstream_provider_id())
            .field("supports_streaming_tools", &self.supports_streaming_tools())
            .finish()
    }
}

/// An error that occurred when trying to authenticate the language model provider.
#[derive(Debug, Error)]
pub enum AuthenticateError {
    #[error("connection refused")]
    ConnectionRefused,
    #[error("credentials not found")]
    CredentialsNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Either a built-in icon name or a path to an external SVG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IconOrSvg {
    /// A built-in icon from Zed's icon set.
    Icon(IconName),
    /// Path to a custom SVG icon file.
    Svg(SharedString),
}

impl Default for IconOrSvg {
    fn default() -> Self {
        Self::Icon(IconName::ZedAssistant)
    }
}

pub trait LanguageModelProvider: 'static {
    fn id(&self) -> LanguageModelProviderId;
    fn name(&self) -> LanguageModelProviderName;
    fn icon(&self) -> IconOrSvg {
        IconOrSvg::default()
    }
    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>>;
    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>>;
    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>>;
    fn recommended_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        Vec::new()
    }
    fn is_authenticated(&self, cx: &App) -> bool;
    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>>;
    fn configuration_view(
        &self,
        target_agent: ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView;
    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>>;
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum ConfigurationViewTargetAgent {
    #[default]
    ZedAgent,
    Other(SharedString),
}

#[derive(PartialEq, Eq)]
pub enum LanguageModelProviderTosView {
    /// When there are some past interactions in the Agent Panel.
    ThreadEmptyState,
    /// When there are no past interactions in the Agent Panel.
    ThreadFreshStart,
    TextThreadPopup,
    Configuration,
}

pub trait LanguageModelProviderState: 'static {
    type ObservableEntity;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>>;

    fn subscribe<T: 'static>(
        &self,
        cx: &mut gpui::Context<T>,
        callback: impl Fn(&mut T, &mut gpui::Context<T>) + 'static,
    ) -> Option<gpui::Subscription> {
        let entity = self.observable_entity()?;
        Some(cx.observe(&entity, move |this, _, cx| {
            callback(this, cx);
        }))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Serialize, Deserialize)]
pub struct LanguageModelId(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct LanguageModelName(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct LanguageModelProviderId(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct LanguageModelProviderName(pub SharedString);

impl LanguageModelProviderId {
    pub const fn new(id: &'static str) -> Self {
        Self(SharedString::new_static(id))
    }
}

impl LanguageModelProviderName {
    pub const fn new(id: &'static str) -> Self {
        Self(SharedString::new_static(id))
    }
}

impl fmt::Display for LanguageModelProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for LanguageModelProviderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for LanguageModelId {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<String> for LanguageModelName {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<String> for LanguageModelProviderId {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<String> for LanguageModelProviderName {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<Arc<str>> for LanguageModelProviderId {
    fn from(value: Arc<str>) -> Self {
        Self(SharedString::from(value))
    }
}

impl From<Arc<str>> for LanguageModelProviderName {
    fn from(value: Arc<str>) -> Self {
        Self(SharedString::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cloud_failure_with_upstream_http_error() {
        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_error".to_string(),
            r#"{"code":"upstream_http_error","message":"Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout","upstream_status":503}"#.to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ServerOverloaded { provider, .. } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!(
                "Expected ServerOverloaded error for 503 status, got: {:?}",
                error
            ),
        }

        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_error".to_string(),
            r#"{"code":"upstream_http_error","message":"Internal server error","upstream_status":500}"#.to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ApiInternalServerError { provider, message } => {
                assert_eq!(provider.0, "anthropic");
                assert_eq!(message, "Internal server error");
            }
            _ => panic!(
                "Expected ApiInternalServerError for 500 status, got: {:?}",
                error
            ),
        }
    }

    #[test]
    fn test_from_cloud_failure_with_standard_format() {
        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_503".to_string(),
            "Service unavailable".to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ServerOverloaded { provider, .. } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!("Expected ServerOverloaded error for upstream_http_503"),
        }
    }

    #[test]
    fn test_upstream_http_error_connection_timeout() {
        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_error".to_string(),
            r#"{"code":"upstream_http_error","message":"Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout","upstream_status":503}"#.to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ServerOverloaded { provider, .. } => {
                assert_eq!(provider.0, "anthropic");
            }
            _ => panic!(
                "Expected ServerOverloaded error for connection timeout with 503 status, got: {:?}",
                error
            ),
        }

        let error = LanguageModelCompletionError::from_cloud_failure(
            String::from("anthropic").into(),
            "upstream_http_error".to_string(),
            r#"{"code":"upstream_http_error","message":"Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout","upstream_status":500}"#.to_string(),
            None,
        );

        match error {
            LanguageModelCompletionError::ApiInternalServerError { provider, message } => {
                assert_eq!(provider.0, "anthropic");
                assert_eq!(
                    message,
                    "Received an error from the Anthropic API: upstream connect error or disconnect/reset before headers. reset reason: connection timeout"
                );
            }
            _ => panic!(
                "Expected ApiInternalServerError for connection timeout with 500 status, got: {:?}",
                error
            ),
        }
    }

    #[test]
    fn test_language_model_tool_use_serializes_with_signature() {
        use serde_json::json;

        let tool_use = LanguageModelToolUse {
            id: LanguageModelToolUseId::from("test_id"),
            name: "test_tool".into(),
            raw_input: json!({"arg": "value"}).to_string(),
            input: json!({"arg": "value"}),
            is_input_complete: true,
            thought_signature: Some("test_signature".to_string()),
        };

        let serialized = serde_json::to_value(&tool_use).unwrap();

        assert_eq!(serialized["id"], "test_id");
        assert_eq!(serialized["name"], "test_tool");
        assert_eq!(serialized["thought_signature"], "test_signature");
    }

    #[test]
    fn test_language_model_tool_use_deserializes_with_missing_signature() {
        use serde_json::json;

        let json = json!({
            "id": "test_id",
            "name": "test_tool",
            "raw_input": "{\"arg\":\"value\"}",
            "input": {"arg": "value"},
            "is_input_complete": true
        });

        let tool_use: LanguageModelToolUse = serde_json::from_value(json).unwrap();

        assert_eq!(tool_use.id, LanguageModelToolUseId::from("test_id"));
        assert_eq!(tool_use.name.as_ref(), "test_tool");
        assert_eq!(tool_use.thought_signature, None);
    }

    #[test]
    fn test_language_model_tool_use_round_trip_with_signature() {
        use serde_json::json;

        let original = LanguageModelToolUse {
            id: LanguageModelToolUseId::from("round_trip_id"),
            name: "round_trip_tool".into(),
            raw_input: json!({"key": "value"}).to_string(),
            input: json!({"key": "value"}),
            is_input_complete: true,
            thought_signature: Some("round_trip_sig".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: LanguageModelToolUse = serde_json::from_value(serialized).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.name, original.name);
        assert_eq!(deserialized.thought_signature, original.thought_signature);
    }

    #[test]
    fn test_language_model_tool_use_round_trip_without_signature() {
        use serde_json::json;

        let original = LanguageModelToolUse {
            id: LanguageModelToolUseId::from("no_sig_id"),
            name: "no_sig_tool".into(),
            raw_input: json!({"arg": "value"}).to_string(),
            input: json!({"arg": "value"}),
            is_input_complete: true,
            thought_signature: None,
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: LanguageModelToolUse = serde_json::from_value(serialized).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.name, original.name);
        assert_eq!(deserialized.thought_signature, None);
    }
}
