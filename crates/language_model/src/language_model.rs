mod model;
mod rate_limiter;
mod registry;
mod request;
mod role;
mod telemetry;

#[cfg(any(test, feature = "test-support"))]
pub mod fake_provider;

use anthropic::{AnthropicError, parse_prompt_too_long};
use anyhow::Result;
use client::Client;
use futures::FutureExt;
use futures::{StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyElement, AnyView, App, AsyncApp, SharedString, Task, Window};
use http_client::http;
use icons::IconName;
use parking_lot::Mutex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::ops::{Add, Sub};
use std::sync::Arc;
use std::time::Duration;
use std::{fmt, io};
use thiserror::Error;
use util::serde::is_default;
use zed_llm_client::CompletionRequestStatus;

pub use crate::model::*;
pub use crate::rate_limiter::*;
pub use crate::registry::*;
pub use crate::request::*;
pub use crate::role::*;
pub use crate::telemetry::*;

pub const ZED_CLOUD_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("zed.dev");

pub const ANTHROPIC_PROVIDER_ID: LanguageModelProviderId =
    LanguageModelProviderId::new("anthropic");
pub const ANTHROPIC_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Anthropic");

pub fn init(client: Arc<Client>, cx: &mut App) {
    init_settings(cx);
    RefreshLlmTokenListener::register(client.clone(), cx);
}

pub fn init_settings(cx: &mut App) {
    registry::init(cx);
}

/// Configuration for caching language model messages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LanguageModelCacheConfiguration {
    pub max_cache_anchors: usize,
    pub should_speculate: bool,
    pub min_total_token: u64,
}

/// A completion event from a language model.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LanguageModelCompletionEvent {
    StatusUpdate(CompletionRequestStatus),
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
    UsageUpdate(TokenUsage),
}

#[derive(Error, Debug)]
pub enum LanguageModelCompletionError {
    // Generic completion handling errors
    #[error("JSON parse error in tool use input")]
    ToolUseJsonParseError,

    // User errors
    #[error("prompt too large for context window")]
    PromptTooLarge { tokens: Option<u64> },
    #[error("missing {provider} API key")]
    NoApiKey { provider: LanguageModelProviderName },

    // Provider errors
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
    #[error("{provider}'s API server reported an internal server error")]
    ApiInternalServerError { provider: LanguageModelProviderName },
    // todo!
    #[error("HTTP response error from {provider}'s API: status {status} - {body:?}")]
    HttpResponseError {
        provider: LanguageModelProviderName,
        status: u16,
        body: String,
    },

    // Client errors
    //
    // todo! which of these should be retriable?
    #[error("invalid request format to {provider}'s API")]
    BadRequestFormat { provider: LanguageModelProviderName },
    #[error("authentication error with {provider}'s API")]
    AuthenticationError { provider: LanguageModelProviderName },
    #[error("permission error with {provider}'s API")]
    PermissionError { provider: LanguageModelProviderName },
    #[error("language model provider API endpoint not found")]
    ApiEndpointNotFound { provider: LanguageModelProviderName },
    #[error("I/O error reading response from {provider}'s API: {error:?}")]
    ApiReadResponseError {
        provider: LanguageModelProviderName,
        error: io::Error,
    },
    #[error("error serializing request to {provider} API: {error}")]
    SerializeRequest {
        provider: LanguageModelProviderName,
        error: serde_json::Error,
    },
    #[error("error building request body to {provider} API: {error}")]
    BuildRequestBody {
        provider: LanguageModelProviderName,
        error: http::Error,
    },
    #[error("error sending HTTP request to {provider} API: {error}")]
    HttpSend {
        provider: LanguageModelProviderName,
        error: anyhow::Error,
    },
    #[error("error deserializing {provider} API response: {error}")]
    DeserializeResponse {
        provider: LanguageModelProviderName,
        error: serde_json::Error,
    },
    #[error("unexpected {provider} API response format: {error}")]
    UnknownResponseFormat {
        provider: LanguageModelProviderName,
        error: String,
    },

    /// Error from cloud provider - message is used directly rather than converting it to one of the
    /// above types.
    #[error("{error}")]
    ZedCloudError { error: String },
    #[error("{error}")]
    RetriableZedCloudError {
        error: String,
        retry_after: Option<Duration>,
    },

    // todo! remove - having From<anyhow::Error> discourages using proper error values
    #[error(transparent)]
    Other(#[from] anyhow::Error),
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
            AnthropicError::HttpResponseError { status, body } => Self::HttpResponseError {
                provider,
                status,
                body,
            },
            AnthropicError::RateLimit { retry_after } => Self::RateLimitExceeded {
                provider,
                retry_after: Some(retry_after),
            },
            AnthropicError::ServerOverloaded { retry_after } => Self::ServerOverloaded {
                provider,
                retry_after: retry_after,
            },
            AnthropicError::ApiError(api_error) => api_error.into(),
            AnthropicError::UnexpectedResponseFormat(error) => {
                Self::UnknownResponseFormat { provider, error }
            }
        }
    }
}

impl From<anthropic::ApiError> for LanguageModelCompletionError {
    fn from(error: anthropic::ApiError) -> Self {
        use anthropic::ApiErrorCode::*;
        let provider = ANTHROPIC_PROVIDER_NAME;
        match error.code() {
            Some(code) => match code {
                InvalidRequestError => Self::BadRequestFormat { provider },
                AuthenticationError => Self::AuthenticationError { provider },
                PermissionError => Self::PermissionError { provider },
                NotFoundError => Self::ApiEndpointNotFound { provider },
                RequestTooLarge => Self::PromptTooLarge {
                    tokens: parse_prompt_too_long(&error.message),
                },
                RateLimitError => Self::RateLimitExceeded {
                    provider,
                    retry_after: None,
                },
                ApiError => Self::ApiInternalServerError { provider },
                OverloadedError => Self::ServerOverloaded {
                    provider,
                    retry_after: None,
                },
            },
            None => Self::Other(error.into()),
        }
    }
}

/// Indicates the format used to define the input schema for a language model tool.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LanguageModelToolSchemaFormat {
    /// A JSON schema, see https://json-schema.org
    JsonSchema,
    /// A subset of an OpenAPI 3.0 schema object supported by Google AI, see https://ai.google.dev/api/caching#Schema
    JsonSchemaSubset,
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
    fn telemetry_id(&self) -> String;

    fn is_zed(&self) -> bool {
        self.provider_id().is_zed()
    }

    fn api_key(&self, _cx: &App) -> Option<String> {
        None
    }

    /// Whether this model supports images
    fn supports_images(&self) -> bool;

    /// Whether this model supports tools.
    fn supports_tools(&self) -> bool;

    /// Whether this model supports choosing which tool to use.
    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool;

    /// Returns whether this model supports "burn mode";
    fn supports_burn_mode(&self) -> bool {
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
                        message_id = Some(id.clone());
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
                                Ok(LanguageModelCompletionEvent::StatusUpdate { .. }) => None,
                                Ok(LanguageModelCompletionEvent::StartMessage { .. }) => None,
                                Ok(LanguageModelCompletionEvent::Text(text)) => Some(Ok(text)),
                                Ok(LanguageModelCompletionEvent::Thinking { .. }) => None,
                                Ok(LanguageModelCompletionEvent::RedactedThinking { .. }) => None,
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

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        None
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &fake_provider::FakeLanguageModel {
        unimplemented!()
    }
}

pub trait LanguageModelTool: 'static + DeserializeOwned + JsonSchema {
    fn name() -> String;
    fn description() -> String;
}

/// An error that occurred when trying to authenticate the language model provider.
#[derive(Debug, Error)]
pub enum AuthenticateError {
    #[error("credentials not found")]
    CredentialsNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub trait LanguageModelProvider: 'static {
    fn id(&self) -> LanguageModelProviderId;
    fn name(&self) -> LanguageModelProviderName;
    fn icon(&self) -> IconName {
        IconName::ZedAssistant
    }
    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>>;
    fn default_fast_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>>;
    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>>;
    fn recommended_models(&self, _cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        Vec::new()
    }
    fn is_authenticated(&self, cx: &App) -> bool;
    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>>;
    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView;
    fn must_accept_terms(&self, _cx: &App) -> bool {
        false
    }
    fn render_accept_terms(
        &self,
        _view: LanguageModelProviderTosView,
        _cx: &mut App,
    ) -> Option<AnyElement> {
        None
    }
    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>>;
}

#[derive(PartialEq, Eq)]
pub enum LanguageModelProviderTosView {
    /// When there are some past interactions in the Agent Panel.
    ThreadtEmptyState,
    /// When there are no past interactions in the Agent Panel.
    ThreadFreshStart,
    PromptEditorPopup,
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

    pub fn is_zed(&self) -> bool {
        self == &ZED_CLOUD_PROVIDER_ID
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
