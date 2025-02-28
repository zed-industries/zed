mod model;
mod rate_limiter;
mod registry;
mod request;
mod role;
mod telemetry;

#[cfg(any(test, feature = "test-support"))]
pub mod fake_provider;

use anyhow::Result;
use client::Client;
use futures::FutureExt;
use futures::{future::BoxFuture, stream::BoxStream, StreamExt, TryStreamExt as _};
use gpui::{AnyElement, AnyView, App, AsyncApp, SharedString, Task, Window};
use proto::Plan;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;
use std::{future::Future, sync::Arc};
use thiserror::Error;
use ui::IconName;

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
    ToolUse(LanguageModelToolUse),
    StartMessage { message_id: String },
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    ToolUse,
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
    pub input: serde_json::Value,
}

pub struct LanguageModelTextStream {
    pub message_id: Option<String>,
    pub stream: BoxStream<'static, Result<String>>,
}

impl Default for LanguageModelTextStream {
    fn default() -> Self {
        Self {
            message_id: None,
            stream: Box::pin(futures::stream::empty()),
        }
    }
}

pub trait LanguageModel: Send + Sync {
    fn id(&self) -> LanguageModelId;
    fn name(&self) -> LanguageModelName;
    /// If None, falls back to [LanguageModelProvider::icon]
    fn icon(&self) -> Option<IconName> {
        None
    }
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
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelCompletionEvent>>>>;

    fn stream_completion_text(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelTextStream>> {
        let events = self.stream_completion(request, cx);

        async move {
            let mut events = events.await?.fuse();
            let mut message_id = None;
            let mut first_item_text = None;

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
                .chain(events.filter_map(|result| async move {
                    match result {
                        Ok(LanguageModelCompletionEvent::StartMessage { .. }) => None,
                        Ok(LanguageModelCompletionEvent::Text(text)) => Some(Ok(text)),
                        Ok(LanguageModelCompletionEvent::Stop(_)) => None,
                        Ok(LanguageModelCompletionEvent::ToolUse(_)) => None,
                        Err(err) => Some(Err(err)),
                    }
                }))
                .boxed();

            Ok(LanguageModelTextStream { message_id, stream })
        }
        .boxed()
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        name: String,
        description: String,
        schema: serde_json::Value,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        None
    }

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> &fake_provider::FakeLanguageModel {
        unimplemented!()
    }
}

impl dyn LanguageModel {
    pub fn use_tool<T: LanguageModelTool>(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> impl 'static + Future<Output = Result<T>> {
        let schema = schemars::schema_for!(T);
        let schema_json = serde_json::to_value(&schema).unwrap();
        let stream = self.use_any_tool(request, T::name(), T::description(), schema_json, cx);
        async move {
            let stream = stream.await?;
            let response = stream.try_collect::<String>().await?;
            Ok(serde_json::from_str(&response)?)
        }
    }

    pub fn use_tool_stream<T: LanguageModelTool>(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let schema = schemars::schema_for!(T);
        let schema_json = serde_json::to_value(&schema).unwrap();
        self.use_any_tool(request, T::name(), T::description(), schema_json, cx)
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
    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>>;
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
    ThreadEmptyState,
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

#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
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
