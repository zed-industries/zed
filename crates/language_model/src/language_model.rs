pub mod providers;
pub mod registry;

use anyhow::{anyhow, Result};
use client::Client;
use futures::{future::BoxFuture, stream::BoxStream, FutureExt};
use gpui::{AnyView, AppContext, Global, Model, ReadGlobal, Task};
use providers::{
    anthropic::AnthropicLanguageModelProvider, cloud::CloudLanguageModelProvider,
    ollama::OllamaLanguageModelProvider, open_ai::OpenAiLanguageModelProvider,
};
use registry::LanguageModelRegistry;
use serde::{Deserialize, Serialize};
use smol::lock::{Semaphore, SemaphoreGuardArc};
use std::{
    fmt::{self, Display},
    sync::Arc,
};
use ui::{Context, SharedString, WindowContext};

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let mut registry = LanguageModelRegistry::default();
    register_language_model_providers(&mut registry, client, cx);

    let mut completion_provider = LanguageModelCompletionProvider::new();
    let default_model = registry
        .model(registry.available_models(cx).first().unwrap(), cx)
        .ok();
    if let Some(default_model) = default_model {
        completion_provider.set_active_model(default_model);
    }

    cx.set_global(registry);
    cx.set_global(completion_provider);
}

fn register_language_model_providers(
    registry: &mut LanguageModelRegistry,
    client: Arc<Client>,
    cx: &mut AppContext,
) {
    registry.register_provider(
        cx.new_model(|cx| CloudLanguageModelProvider::new(client.clone(), cx)),
        cx,
    );
    registry.register_provider(
        cx.new_model(|cx| AnthropicLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        cx.new_model(|cx| OpenAiLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
    registry.register_provider(
        cx.new_model(|cx| OllamaLanguageModelProvider::new(client.http_client(), cx)),
        cx,
    );
}

#[derive(Debug, Clone)]
pub struct LanguageModelRequest {
    pub messages: Vec<LanguageModelRequestMessage>,
    pub stop: Vec<String>,
    pub temperature: f32,
}

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

    fn complete(
        &self,
        request: LanguageModelRequest,
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

impl<T: LanguageModelProvider> LanguageModelProvider for Model<T> {
    fn name(&self, cx: &AppContext) -> LanguageModelProviderName {
        self.read(cx).name(cx)
    }

    fn provided_models(&self, cx: &AppContext) -> Vec<ProvidedLanguageModel> {
        self.read(cx).provided_models(cx)
    }

    fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.read(cx).is_authenticated(cx)
    }

    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        self.read(cx).authenticate(cx)
    }

    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        let handle = cx.window_handle();

        self.update(cx, |provider, cx| {
            handle.update(cx, |_, cx| provider.authentication_prompt(cx))
        })
        .unwrap() // TODO: Handle this better
    }

    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        self.read(cx).reset_credentials(cx)
    }

    fn model(&self, id: LanguageModelId, cx: &AppContext) -> Result<Arc<dyn LanguageModel>> {
        self.read(cx).model(id, cx)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: String,
}

impl LanguageModelRequestMessage {
    pub fn to_proto(&self) -> proto::LanguageModelRequestMessage {
        proto::LanguageModelRequestMessage {
            role: match self.role {
                Role::User => proto::LanguageModelRole::LanguageModelUser,
                Role::Assistant => proto::LanguageModelRole::LanguageModelAssistant,
                Role::System => proto::LanguageModelRole::LanguageModelSystem,
            } as i32,
            content: self.content.clone(),
            tool_calls: Vec::new(),
            tool_call_id: None,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl Role {
    pub fn cycle(&mut self) {
        *self = match self {
            Role::User => Role::Assistant,
            Role::Assistant => Role::System,
            Role::System => Role::User,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelId(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelName(pub SharedString);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageModelProviderName(SharedString);

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

pub struct LanguageModelCompletionProvider {
    active_model: Option<Arc<dyn LanguageModel>>,
    request_limiter: Arc<Semaphore>,
}

impl Global for LanguageModelCompletionProvider {}

const MAX_CONCURRENT_COMPLETION_REQUESTS: usize = 4;

pub struct CompletionResponse {
    pub inner: BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>,
    _lock: SemaphoreGuardArc,
}

impl LanguageModelCompletionProvider {
    pub fn new() -> Self {
        Self {
            active_model: None,
            request_limiter: Arc::new(Semaphore::new(MAX_CONCURRENT_COMPLETION_REQUESTS)),
        }
    }

    pub fn active_model(&self) -> Option<Arc<dyn LanguageModel>> {
        self.active_model.clone()
    }

    pub fn set_active_model(&mut self, model: Arc<dyn LanguageModel>) {
        self.active_model = Some(model);
    }

    pub fn current_provider(&self, cx: &AppContext) -> Option<Arc<dyn LanguageModelProvider>> {
        let provider_name = self.active_model.as_ref()?.provider_name();
        LanguageModelRegistry::global(cx).provider(&provider_name)
    }

    pub fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.current_provider(cx)
            .map_or(false, |provider| provider.is_authenticated(cx))
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        self.current_provider(cx)
            .map_or(Task::ready(Ok(())), |provider| provider.authenticate(cx))
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        self.current_provider(cx)
            .map_or(Task::ready(Ok(())), |provider| {
                provider.reset_credentials(cx)
            })
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        if let Some(model) = self.active_model() {
            model.count_tokens(request, cx)
        } else {
            std::future::ready(Err(anyhow!("No active model set"))).boxed()
        }
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> Task<Result<CompletionResponse>> {
        if let Some(language_model) = self.active_model() {
            let rate_limiter = self.request_limiter.clone();
            cx.background_executor().spawn(async move {
                let lock = rate_limiter.acquire_arc().await;
                let response = language_model.complete(request);
                Ok(CompletionResponse {
                    inner: response,
                    _lock: lock,
                })
            })
        } else {
            Task::ready(Err(anyhow!("No active model set")))
        }
    }
}
