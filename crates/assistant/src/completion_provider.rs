mod anthropic;
mod cloud;
#[cfg(test)]
mod fake;
mod ollama;
mod open_ai;

pub use anthropic::*;
pub use cloud::*;
use collections::HashMap;
#[cfg(test)]
pub use fake::*;
pub use ollama::*;
pub use open_ai::*;
use parking_lot::RwLock;
use smol::lock::{Semaphore, SemaphoreGuardArc};

use crate::{
    assistant_settings::{AssistantProvider, AssistantSettings},
    LanguageModel, LanguageModelRequest,
};
use anyhow::Result;
use client::Client;
use futures::{future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, AppContext, BorrowAppContext, Task, WindowContext};
use settings::{Settings, SettingsStore};
use std::any::TypeId;
use std::{any::Any, sync::Arc};

//TODO(completion_provider) use this again
/// Choose which model to use for openai provider.
/// If the model is not available, try to use the first available model, or fallback to the original model.
fn choose_openai_model(
    model: &::open_ai::Model,
    available_models: &[::open_ai::Model],
) -> ::open_ai::Model {
    available_models
        .iter()
        .find(|&m| m == model)
        .or_else(|| available_models.first())
        .unwrap_or_else(|| model)
        .clone()
}

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let mut completion_provider = CompletionProvider::new(Some(client.clone()));
    for (type_id, provider) in create_providers_from_settings(client.clone(), cx) {
        completion_provider.register_provider(type_id, provider);
    }
    cx.set_global(completion_provider);

    let mut settings_version = 0;
    cx.observe_global::<SettingsStore>(move |cx| {
        settings_version += 1;
        cx.update_global::<CompletionProvider, _>(|provider, cx| {
            provider.update_settings(settings_version, cx);
        })
    })
    .detach();
}

pub struct CompletionResponse {
    pub inner: BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>,
    _lock: SemaphoreGuardArc,
}

pub trait LanguageModelSettings: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn boxed(&self) -> Box<dyn LanguageModelSettings>;
}

pub trait LanguageModelCompletionProvider: Send + Sync {
    type Settings: LanguageModelSettings + Default;

    fn update(&mut self, settings: &Self::Settings, cx: &AppContext);
    fn set_model(&mut self, model: LanguageModel, cx: &mut AppContext);

    fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel>;
    fn is_authenticated(&self) -> bool;
    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>>;
    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView;
    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>>;
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

pub trait AnyLanguageModelCompletionProvider: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn type_id(&self) -> TypeId;

    fn update(&mut self, settings: &dyn LanguageModelSettings, cx: &AppContext);
    fn set_model(&mut self, model: LanguageModel, cx: &mut AppContext);

    fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel>;
    fn is_authenticated(&self) -> bool;
    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>>;
    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView;
    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>>;
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

impl<T> AnyLanguageModelCompletionProvider for T
where
    T: LanguageModelCompletionProvider + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn update(&mut self, settings: &dyn LanguageModelSettings, cx: &AppContext) {
        self.update(settings.as_any().downcast_ref().unwrap(), cx)
    }

    fn set_model(&mut self, model: LanguageModel, cx: &mut AppContext) {
        self.set_model(model, cx)
    }

    fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel> {
        self.available_models(cx)
    }

    fn is_authenticated(&self) -> bool {
        self.is_authenticated()
    }

    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        self.authenticate(cx)
    }

    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        self.authentication_prompt(cx)
    }

    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        self.reset_credentials(cx)
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        self.count_tokens(request, cx)
    }

    fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        self.complete(request)
    }
}

const MAX_CONCURRENT_COMPLETION_REQUESTS: usize = 4;

pub struct CompletionProvider {
    providers: HashMap<TypeId, Arc<RwLock<dyn AnyLanguageModelCompletionProvider>>>,
    active_model: LanguageModel,
    client: Option<Arc<Client>>,
    request_limiter: Arc<Semaphore>,
}

impl CompletionProvider {
    pub fn new(client: Option<Arc<Client>>) -> Self {
        Self {
            providers: HashMap::default(),
            active_model: LanguageModel::default(),
            client,
            request_limiter: Arc::new(Semaphore::new(MAX_CONCURRENT_COMPLETION_REQUESTS)),
        }
    }

    pub fn register_provider(
        &mut self,
        type_id: TypeId,
        provider: Arc<RwLock<dyn AnyLanguageModelCompletionProvider>>,
    ) {
        self.providers.insert(type_id, provider);
    }

    pub fn model(&self) -> LanguageModel {
        self.active_model.clone()
    }

    pub fn set_model(&mut self, model: LanguageModel, cx: &mut AppContext) {
        let type_id = match self.active_model {
            LanguageModel::Cloud(_) => TypeId::of::<CloudCompletionProvider>(),
            LanguageModel::OpenAi(_) => TypeId::of::<OpenAiCompletionProvider>(),
            LanguageModel::Anthropic(_) => TypeId::of::<AnthropicCompletionProvider>(),
            LanguageModel::Ollama(_) => TypeId::of::<OllamaCompletionProvider>(),
        };
        let provider = self.providers.get(&type_id).unwrap();
        provider.write().set_model(model.clone(), cx);
        self.active_model = model;
    }

    fn active_provider(&self) -> Arc<RwLock<dyn AnyLanguageModelCompletionProvider>> {
        let type_id = match self.active_model {
            LanguageModel::Cloud(_) => TypeId::of::<CloudCompletionProvider>(),
            LanguageModel::OpenAi(_) => TypeId::of::<OpenAiCompletionProvider>(),
            LanguageModel::Anthropic(_) => TypeId::of::<AnthropicCompletionProvider>(),
            LanguageModel::Ollama(_) => TypeId::of::<OllamaCompletionProvider>(),
        };
        self.providers.get(&type_id).unwrap().clone()
    }

    pub fn update_provider_of_type<C, T: LanguageModelCompletionProvider + 'static>(
        &mut self,
        func: impl FnOnce(&mut T) -> C,
    ) -> Option<C> {
        if let Some(provider) = self.providers.get_mut(&TypeId::of::<T>()) {
            let mut provider = provider.write();
            Some(func(provider.as_any_mut().downcast_mut::<T>().unwrap()))
        } else {
            None
        }
    }

    pub fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel> {
        self.providers
            .values()
            .map(|provider| provider.read().available_models(cx))
            .flatten()
            .collect()
    }

    pub fn settings_version(&self) -> usize {
        0
    }

    pub fn is_authenticated(&self) -> bool {
        true
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        todo!()
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        todo!()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        self.active_provider().read().count_tokens(request, cx)
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> Task<CompletionResponse> {
        let rate_limiter = self.request_limiter.clone();
        let provider = self.active_provider();
        cx.background_executor().spawn(async move {
            let lock = rate_limiter.acquire_arc().await;
            let response = provider.read().complete(request);
            CompletionResponse {
                inner: response,
                _lock: lock,
            }
        })
    }
}

impl gpui::Global for CompletionProvider {}

impl CompletionProvider {
    pub fn global(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn update_settings(&mut self, version: usize, cx: &mut AppContext) {
        for provider_config in AssistantSettings::get_global(cx).providers.clone() {
            let type_id = match provider_config {
                AssistantProvider::ZedDotDev { .. } => TypeId::of::<CloudCompletionProvider>(),
                AssistantProvider::OpenAi { .. } => TypeId::of::<OpenAiCompletionProvider>(),
                AssistantProvider::Anthropic { .. } => TypeId::of::<AnthropicCompletionProvider>(),
                AssistantProvider::Ollama { .. } => TypeId::of::<OllamaCompletionProvider>(),
            };

            if let Some(provider) = self.providers.get(&type_id).cloned() {
                provider.write().update(provider_config.settings(), cx);
            } else if let Some(client) = self.client.clone() {
                let (_, provider) = create_provider_from_settings(client, provider_config, cx);
                self.providers.insert(type_id, provider);
            } else {
                log::warn!("No client available to create provider");
            }
        }
    }
}

fn create_providers_from_settings(
    client: Arc<Client>,
    cx: &mut AppContext,
) -> Vec<(TypeId, Arc<RwLock<dyn AnyLanguageModelCompletionProvider>>)> {
    let mut providers = Vec::new();
    for provider_config in AssistantSettings::get_global(cx).providers.clone() {
        providers.push(create_provider_from_settings(
            client.clone(),
            provider_config,
            cx,
        ));
    }
    providers
}

fn create_provider_from_settings(
    client: Arc<Client>,
    provider: AssistantProvider,
    cx: &mut AppContext,
) -> (TypeId, Arc<RwLock<dyn AnyLanguageModelCompletionProvider>>) {
    match provider {
        AssistantProvider::ZedDotDev => (
            TypeId::of::<CloudCompletionProvider>(),
            Arc::new(RwLock::new(CloudCompletionProvider::new(client, cx))),
        ),
        AssistantProvider::OpenAi(settings) => (
            TypeId::of::<OpenAiCompletionProvider>(),
            Arc::new(RwLock::new(OpenAiCompletionProvider::new(
                client.http_client(),
                settings,
            ))),
        ),
        AssistantProvider::Anthropic(settings) => (
            TypeId::of::<AnthropicCompletionProvider>(),
            Arc::new(RwLock::new(AnthropicCompletionProvider::new(
                client.http_client(),
                settings,
            ))),
        ),
        AssistantProvider::Ollama(settings) => (
            TypeId::of::<OllamaCompletionProvider>(),
            Arc::new(RwLock::new(OllamaCompletionProvider::new(
                client.http_client(),
                settings,
            ))),
        ),
    }
}

#[cfg(test)]
mod tests {
    use gpui::AppContext;
    use settings::SettingsStore;
    use smol::stream::StreamExt;

    use crate::{
        completion_provider::MAX_CONCURRENT_COMPLETION_REQUESTS, CompletionProvider,
        FakeCompletionProvider, LanguageModelRequest,
    };

    #[gpui::test]
    fn test_rate_limiting(cx: &mut AppContext) {
        SettingsStore::test(cx);
        let fake_provider = FakeCompletionProvider::setup_test(cx);
        let provider = cx.global::<CompletionProvider>();

        // Enqueue some requests
        for i in 0..MAX_CONCURRENT_COMPLETION_REQUESTS * 2 {
            let response = provider.complete(
                LanguageModelRequest {
                    temperature: i as f32 / 10.0,
                    ..Default::default()
                },
                cx,
            );
            cx.background_executor()
                .spawn(async move {
                    let response = response.await;
                    let mut stream = response.inner.await.unwrap();
                    while let Some(message) = stream.next().await {
                        message.unwrap();
                    }
                })
                .detach();
        }
        cx.background_executor().run_until_parked();

        assert_eq!(
            fake_provider.completion_count(),
            MAX_CONCURRENT_COMPLETION_REQUESTS
        );

        // Get the first completion request that is in flight and mark it as completed.
        let completion = fake_provider
            .running_completions()
            .into_iter()
            .next()
            .unwrap();
        fake_provider.finish_completion(&completion);

        // Ensure that the number of in-flight completion requests is reduced.
        assert_eq!(
            fake_provider.completion_count(),
            MAX_CONCURRENT_COMPLETION_REQUESTS - 1
        );

        cx.background_executor().run_until_parked();

        // Ensure that another completion request was allowed to acquire the lock.
        assert_eq!(
            fake_provider.completion_count(),
            MAX_CONCURRENT_COMPLETION_REQUESTS
        );

        // Mark all completion requests as finished that are in flight.
        for request in fake_provider.running_completions() {
            fake_provider.finish_completion(&request);
        }

        assert_eq!(fake_provider.completion_count(), 0);

        // Wait until the background tasks acquire the lock again.
        cx.background_executor().run_until_parked();

        assert_eq!(
            fake_provider.completion_count(),
            MAX_CONCURRENT_COMPLETION_REQUESTS - 1
        );

        // Finish all remaining completion requests.
        for request in fake_provider.running_completions() {
            fake_provider.finish_completion(&request);
        }

        cx.background_executor().run_until_parked();

        assert_eq!(fake_provider.completion_count(), 0);
    }
}
