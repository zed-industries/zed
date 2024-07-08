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
use std::{any::Any, sync::Arc};
use std::{any::TypeId, time::Duration};

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
    let provider = create_provider_from_settings(client.clone(), 0, cx);
    cx.set_global(CompletionProvider::new(vec![provider], Some(client)));

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

trait LanguageModelSettings: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn boxed(&self) -> Box<dyn LanguageModelSettings>;
}

pub trait LanguageModelCompletionProvider: Send + Sync {
    type Settings: LanguageModelSettings + Default;

    fn available_models(&self, settings: &Self::Settings, cx: &AppContext) -> Vec<LanguageModel>;
    fn is_authenticated(&self) -> bool;
    fn authenticate(&self, settings: &Self::Settings, cx: &AppContext) -> Task<Result<()>>;
    fn authentication_prompt(&self, settings: &Self::Settings, cx: &mut WindowContext) -> AnyView;
    fn reset_credentials(&self, settings: &Self::Settings, cx: &AppContext) -> Task<Result<()>>;
    fn model(&self) -> LanguageModel;
    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        settings: &Self::Settings,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>>;
    fn complete(
        &self,
        request: LanguageModelRequest,
        settings: &Self::Settings,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}

trait AnyLanguageModelCompletionProvider: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn type_id(&self) -> TypeId;
    fn available_models(
        &self,
        settings: &dyn LanguageModelSettings,
        cx: &AppContext,
    ) -> Vec<LanguageModel>;
    fn is_authenticated(&self) -> bool;
    fn authenticate(
        &self,
        settings: &dyn LanguageModelSettings,
        cx: &AppContext,
    ) -> Task<Result<()>>;
    fn authentication_prompt(
        &self,
        settings: &dyn LanguageModelSettings,
        cx: &mut WindowContext,
    ) -> AnyView;
    fn reset_credentials(
        &self,
        settings: &dyn LanguageModelSettings,
        cx: &AppContext,
    ) -> Task<Result<()>>;
    fn model(&self) -> LanguageModel;
    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        settings: &dyn LanguageModelSettings,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>>;
    fn complete(
        &self,
        request: LanguageModelRequest,
        settings: &dyn LanguageModelSettings,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}

impl<T: LanguageModelCompletionProvider + 'static> AnyLanguageModelCompletionProvider for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn available_models(&self, settings: &dyn Any, cx: &AppContext) -> Vec<LanguageModel> {
        self.available_models(settings.downcast_ref().unwrap(), cx)
    }

    fn is_authenticated(&self) -> bool {
        self.is_authenticated()
    }

    fn authenticate(
        &self,
        settings: &dyn LanguageModelSettings,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        self.authenticate(settings.as_any().downcast_ref().unwrap(), cx)
    }

    fn authentication_prompt(
        &self,
        settings: &dyn LanguageModelSettings,
        cx: &mut WindowContext,
    ) -> AnyView {
        self.authentication_prompt(settings.as_any().downcast_ref().unwrap(), cx)
    }

    fn reset_credentials(
        &self,
        settings: &dyn LanguageModelSettings,
        cx: &AppContext,
    ) -> Task<Result<()>> {
        self.reset_credentials(settings.as_any().downcast_ref().unwrap(), cx)
    }

    fn model(&self) -> LanguageModel {
        self.model()
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        settings: &dyn LanguageModelSettings,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        self.count_tokens(request, settings.as_any().downcast_ref().unwrap(), cx)
    }

    fn complete(
        &self,
        request: LanguageModelRequest,
        settings: &dyn LanguageModelSettings,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        self.complete(request, settings.as_any().downcast_ref().unwrap())
    }
}

const MAX_CONCURRENT_COMPLETION_REQUESTS: usize = 4;

struct LanguageModelCompletionProviderData {
    provider: Arc<dyn AnyLanguageModelCompletionProvider>,
    settings: Box<dyn LanguageModelSettings>,
}

pub struct CompletionProvider {
    providers: HashMap<TypeId, LanguageModelCompletionProviderData>,
    active_provider: TypeId,
    client: Option<Arc<Client>>,
    request_limiter: Arc<Semaphore>,
}

impl CompletionProvider {
    pub fn new(client: Option<Arc<Client>>) -> Self {
        Self {
            active_provider: Some(()).type_id(),
            providers: HashMap::default(),
            client,
            request_limiter: Arc::new(Semaphore::new(MAX_CONCURRENT_COMPLETION_REQUESTS)),
        }
    }

    pub fn register_provider<T: LanguageModelCompletionProvider + 'static>(&mut self, provider: T) {
        self.providers.insert(
            TypeId::of::<T>(),
            LanguageModelCompletionProviderData {
                provider: Arc::new(provider),
                settings: Box::new(T::Settings::default()),
            },
        );
        self.active_provider = TypeId::of::<T>();
    }

    fn active_provider(
        &self,
    ) -> (
        Arc<dyn AnyLanguageModelCompletionProvider>,
        &dyn LanguageModelSettings,
    ) {
        let provider = self.providers.get(&self.active_provider).unwrap();
        (provider.provider.clone(), provider.settings.as_ref())
    }

    pub fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel> {
        let (provider, settings) = self.active_provider();
        provider.available_models(settings, cx)
    }

    pub fn settings_version(&self) -> usize {
        0
    }

    pub fn is_authenticated(&self) -> bool {
        let (provider, _) = self.active_provider();
        provider.is_authenticated()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        let (provider, settings) = self.active_provider();
        provider.authenticate(settings, cx)
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        let (provider, settings) = self.active_provider();
        provider.authentication_prompt(settings, cx)
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        let (provider, settings) = self.active_provider();
        provider.reset_credentials(settings, cx)
    }

    pub fn model(&self) -> LanguageModel {
        let (provider, _) = self.active_provider();
        provider.model()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        let (provider, settings) = self.active_provider();
        provider.count_tokens(request, settings, cx)
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> Task<CompletionResponse> {
        let rate_limiter = self.request_limiter.clone();
        let (provider, settings) = self.active_provider();
        let settings = settings.boxed();
        cx.background_executor().spawn(async move {
            let lock = rate_limiter.acquire_arc().await;
            let response = provider.complete(request, settings.as_ref());
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
        let type_id = match &AssistantSettings::get_global(cx).provider {
            AssistantProvider::ZedDotDev { .. } => TypeId::of::<CloudCompletionProvider>(),
            AssistantProvider::OpenAi { .. } => TypeId::of::<OpenAiCompletionProvider>(),
            AssistantProvider::Anthropic { .. } => TypeId::of::<AnthropicCompletionProvider>(),
            AssistantProvider::Ollama { .. } => TypeId::of::<OllamaCompletionProvider>(),
        };

        if let Some(provider) = self.providers.get_mut(&type_id) {
            provider.settings = Box::new(todo!());
        } else {
            let provider = create_provider_from_settings(client, version, cx);
            todo!()
        }
    }
}

fn create_provider_from_settings(
    client: Arc<Client>,
    settings_version: usize,
    cx: &mut AppContext,
) -> Arc<dyn LanguageModelCompletionProvider> {
    match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { model } => Arc::new(CloudCompletionProvider::new(
            model.clone(),
            client.clone(),
            settings_version,
            cx,
        )),
        AssistantProvider::OpenAi {
            model,
            api_url,
            low_speed_timeout_in_seconds,
            available_models,
        } => Arc::new(OpenAiCompletionProvider::new(
            choose_openai_model(&model, &available_models),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
        )),
        AssistantProvider::Anthropic {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => Arc::new(AnthropicCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
        )),
        AssistantProvider::Ollama {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => Arc::new(OllamaCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
            cx,
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

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

        let provider = CompletionProvider::new(vec![Arc::new(fake_provider.clone())], None);

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
