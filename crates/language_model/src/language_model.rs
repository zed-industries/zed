mod model;
mod rate_limiter;
mod registry;
mod request;
mod role;
mod telemetry;

#[cfg(any(test, feature = "test-support"))]
pub mod fake_provider;

use anyhow::{Result, anyhow};
use client::Client;
use futures::FutureExt;
use futures::{StreamExt, future::BoxFuture, stream::BoxStream};
use gpui::{AnyElement, AnyView, App, AsyncApp, SharedString, Task, Window};
use http_client::http::{HeaderMap, HeaderValue};
use icons::IconName;
use parking_lot::Mutex;
use proto::Plan;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr as _;
use std::sync::Arc;
use thiserror::Error;
use util::serde::is_default;
use zed_llm_client::{
    MODEL_REQUESTS_USAGE_AMOUNT_HEADER_NAME, MODEL_REQUESTS_USAGE_LIMIT_HEADER_NAME, UsageLimit,
};

pub use crate::model::*;
pub use crate::rate_limiter::*;
pub use crate::registry::*;
pub use crate::request::*;
pub use crate::role::*;
pub use crate::telemetry::*;

pub const ZED_CLOUD_PROVIDER_ID: &str = "zed.dev";

pub fn init(client: Arc<Client>, cx: &mut App) {
    registry::init(cx);
    RefreshLlmTokenListener::register(client.clone(), cx);
}

/// The availability of a [`LanguageModel`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LanguageModelAvailability {
    /// The language model is available to the general public.
    Public,
    /// The language model is available to users on the indicated plan.
    RequiresPlan(Plan),
}

/// Configuration for caching language model messages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LanguageModelCacheConfiguration {
    pub max_cache_anchors: usize,
    pub should_speculate: bool,
    pub min_total_token: usize,
}

/// A completion event from a language model.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LanguageModelCompletionEvent {
    Stop(StopReason),
    Text(String),
    Thinking {
        text: String,
        signature: Option<String>,
    },
    ToolUse(LanguageModelToolUse),
    StartMessage {
        message_id: String,
        role: Role,
    },
    UsageUpdate(TokenUsage),
}

#[derive(Error, Debug)]
pub enum LanguageModelCompletionError {
    #[error("received bad input JSON")]
    BadInputJson {
        id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        raw_input: Arc<str>,
        json_parse_error: String,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
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
}

#[derive(Debug, Clone, Copy)]
pub struct RequestUsage {
    pub limit: UsageLimit,
    pub amount: i32,
}

impl RequestUsage {
    pub fn from_headers(headers: &HeaderMap<HeaderValue>) -> Result<Self> {
        let limit = headers
            .get(MODEL_REQUESTS_USAGE_LIMIT_HEADER_NAME)
            .ok_or_else(|| anyhow!("missing {MODEL_REQUESTS_USAGE_LIMIT_HEADER_NAME:?} header"))?;
        let limit = UsageLimit::from_str(limit.to_str()?)?;

        let amount = headers
            .get(MODEL_REQUESTS_USAGE_AMOUNT_HEADER_NAME)
            .ok_or_else(|| anyhow!("missing {MODEL_REQUESTS_USAGE_AMOUNT_HEADER_NAME:?} header"))?;
        let amount = amount.to_str()?.parse::<i32>()?;

        Ok(Self { limit, amount })
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    #[serde(default, skip_serializing_if = "is_default")]
    pub input_tokens: u32,
    #[serde(default, skip_serializing_if = "is_default")]
    pub output_tokens: u32,
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_creation_input_tokens: u32,
    #[serde(default, skip_serializing_if = "is_default")]
    pub cache_read_input_tokens: u32,
}

impl TokenUsage {
    pub fn total_tokens(&self) -> u32 {
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

    fn api_key(&self, _cx: &App) -> Option<String> {
        None
    }

    /// Returns the availability of this language model.
    fn availability(&self) -> LanguageModelAvailability {
        LanguageModelAvailability::Public
    }

    /// Whether this model supports tools.
    fn supports_tools(&self) -> bool;

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchema
    }

    fn max_token_count(&self) -> usize;
    fn max_output_tokens(&self) -> Option<u32> {
        None
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<usize>>;

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
        >,
    >;

    fn stream_completion_with_usage(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<(
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
            Option<RequestUsage>,
        )>,
    > {
        self.stream_completion(request, cx)
            .map(|result| result.map(|stream| (stream, None)))
            .boxed()
    }

    fn stream_completion_text(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelTextStream>> {
        self.stream_completion_text_with_usage(request, cx)
            .map(|result| result.map(|(stream, _usage)| stream))
            .boxed()
    }

    fn stream_completion_text_with_usage(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<(LanguageModelTextStream, Option<RequestUsage>)>> {
        let future = self.stream_completion_with_usage(request, cx);

        async move {
            let (events, usage) = future.await?;
            let mut events = events.fuse();
            let mut message_id = None;
            let mut first_item_text = None;
            let last_token_usage = Arc::new(Mutex::new(TokenUsage::default()));

            if let Some(first_event) = events.next().await {
                match first_event {
                    Ok(LanguageModelCompletionEvent::StartMessage { message_id: id, .. }) => {
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
                                Ok(LanguageModelCompletionEvent::StartMessage { .. }) => None,
                                Ok(LanguageModelCompletionEvent::Text(text)) => Some(Ok(text)),
                                Ok(LanguageModelCompletionEvent::Thinking { .. }) => None,
                                Ok(LanguageModelCompletionEvent::Stop(_)) => None,
                                Ok(LanguageModelCompletionEvent::ToolUse(_)) => None,
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

            Ok((
                LanguageModelTextStream {
                    message_id,
                    stream,
                    last_token_usage,
                },
                usage,
            ))
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

#[derive(Debug, Error)]
pub enum LanguageModelKnownError {
    #[error("Context window limit exceeded ({tokens})")]
    ContextWindowLimitExceeded { tokens: usize },
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
    fn load_model(&self, _model: Arc<dyn LanguageModel>, _cx: &App) {}
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

impl fmt::Display for LanguageModelProviderId {
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
