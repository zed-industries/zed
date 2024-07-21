mod model;
pub mod provider;
mod registry;
mod request;
mod role;
pub mod settings;

use std::sync::Arc;

use anyhow::Result;
use futures::{future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, AppContext, SharedString, Task, WindowContext};

pub use model::*;
pub use registry::*;
pub use request::*;
pub use role::*;

#[derive(Clone)]
pub struct ProvidedLanguageModel {
    pub id: LanguageModelId,
    pub name: LanguageModelName,
}

#[derive(Clone)]
pub struct AvailableLanguageModel {
    pub provider: LanguageModelProviderName,
    pub model: ProvidedLanguageModel,
}

pub trait LanguageModel: Send + Sync {
    fn id(&self) -> LanguageModelId;
    fn name(&self) -> LanguageModelName;
    fn provider_name(&self) -> LanguageModelProviderName;
    fn telemetry_id(&self) -> String;

    fn max_token_count(&self) -> usize;

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>>;

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}

pub trait LanguageModelProvider: 'static {
    fn name(&self, cx: &AppContext) -> LanguageModelProviderName;
    fn provided_models(&self, cx: &AppContext) -> Vec<ProvidedLanguageModel>;
    fn is_authenticated(&self, cx: &AppContext) -> bool;
    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>>;
    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView;
    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>>;
    fn model(&self, id: LanguageModelId, cx: &AppContext) -> Result<Arc<dyn LanguageModel>>;
}

pub trait LanguageModelProviderState: 'static {
    fn subscribe<T: 'static>(&self, cx: &mut gpui::ModelContext<T>) -> gpui::Subscription;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelId(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelName(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelProviderName(pub SharedString);

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

impl From<String> for LanguageModelProviderName {
    fn from(value: String) -> Self {
        Self(SharedString::from(value))
    }
}
