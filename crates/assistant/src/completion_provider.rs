mod anthropic;
mod cloud;
#[cfg(any(test, feature = "test-support"))]
mod fake;
mod ollama;
mod open_ai;

pub use anthropic::*;
pub use cloud::*;
#[cfg(any(test, feature = "test-support"))]
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
use futures::{future::BoxFuture, stream::BoxStream, StreamExt};
use gpui::{AnyView, AppContext, BorrowAppContext, Task, WindowContext};
use settings::{Settings, SettingsStore};
use std::{any::Any, pin::Pin, sync::Arc, task::Poll, time::Duration};

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
    cx.set_global(CompletionProvider::new(provider, Some(client)));

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
    inner: BoxStream<'static, Result<String>>,
    _lock: SemaphoreGuardArc,
}

impl futures::Stream for CompletionResponse {
    type Item = Result<String>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

pub trait LanguageModelCompletionProvider: Send + Sync {
    fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel>;
    fn settings_version(&self) -> usize;
    fn is_authenticated(&self) -> bool;
    fn authenticate(&self, cx: &AppContext) -> Task<Result<()>>;
    fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView;
    fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>>;
    fn model(&self) -> LanguageModel;
    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>>;
    fn stream_completion(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

const MAX_CONCURRENT_COMPLETION_REQUESTS: usize = 4;

pub struct CompletionProvider {
    provider: Arc<RwLock<dyn LanguageModelCompletionProvider>>,
    client: Option<Arc<Client>>,
    request_limiter: Arc<Semaphore>,
}

impl CompletionProvider {
    pub fn new(
        provider: Arc<RwLock<dyn LanguageModelCompletionProvider>>,
        client: Option<Arc<Client>>,
    ) -> Self {
        Self {
            provider,
            client,
            request_limiter: Arc::new(Semaphore::new(MAX_CONCURRENT_COMPLETION_REQUESTS)),
        }
    }

    pub fn available_models(&self, cx: &AppContext) -> Vec<LanguageModel> {
        self.provider.read().available_models(cx)
    }

    pub fn settings_version(&self) -> usize {
        self.provider.read().settings_version()
    }

    pub fn is_authenticated(&self) -> bool {
        self.provider.read().is_authenticated()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        self.provider.read().authenticate(cx)
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        self.provider.read().authentication_prompt(cx)
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        self.provider.read().reset_credentials(cx)
    }

    pub fn model(&self) -> LanguageModel {
        self.provider.read().model()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        self.provider.read().count_tokens(request, cx)
    }

    pub fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> Task<Result<CompletionResponse>> {
        let rate_limiter = self.request_limiter.clone();
        let provider = self.provider.clone();
        cx.foreground_executor().spawn(async move {
            let lock = rate_limiter.acquire_arc().await;
            let response = provider.read().stream_completion(request);
            let response = response.await?;
            Ok(CompletionResponse {
                inner: response,
                _lock: lock,
            })
        })
    }

    pub fn complete(&self, request: LanguageModelRequest, cx: &AppContext) -> Task<Result<String>> {
        let response = self.stream_completion(request, cx);
        cx.foreground_executor().spawn(async move {
            let mut chunks = response.await?;
            let mut completion = String::new();
            while let Some(chunk) = chunks.next().await {
                let chunk = chunk?;
                completion.push_str(&chunk);
            }
            Ok(completion)
        })
    }
}

impl gpui::Global for CompletionProvider {}

impl CompletionProvider {
    pub fn global(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn update_current_as<R, T: LanguageModelCompletionProvider + 'static>(
        &mut self,
        update: impl FnOnce(&mut T) -> R,
    ) -> Option<R> {
        let mut provider = self.provider.write();
        if let Some(provider) = provider.as_any_mut().downcast_mut::<T>() {
            Some(update(provider))
        } else {
            None
        }
    }

    pub fn update_settings(&mut self, version: usize, cx: &mut AppContext) {
        let updated = match &AssistantSettings::get_global(cx).provider {
            AssistantProvider::ZedDotDev { model } => self
                .update_current_as::<_, CloudCompletionProvider>(|provider| {
                    provider.update(model.clone(), version);
                }),
            AssistantProvider::OpenAi {
                model,
                api_url,
                low_speed_timeout_in_seconds,
                available_models,
            } => self.update_current_as::<_, OpenAiCompletionProvider>(|provider| {
                provider.update(
                    choose_openai_model(&model, &available_models),
                    api_url.clone(),
                    low_speed_timeout_in_seconds.map(Duration::from_secs),
                    version,
                );
            }),
            AssistantProvider::Anthropic {
                model,
                api_url,
                low_speed_timeout_in_seconds,
            } => self.update_current_as::<_, AnthropicCompletionProvider>(|provider| {
                provider.update(
                    model.clone(),
                    api_url.clone(),
                    low_speed_timeout_in_seconds.map(Duration::from_secs),
                    version,
                );
            }),
            AssistantProvider::Ollama {
                model,
                api_url,
                low_speed_timeout_in_seconds,
            } => self.update_current_as::<_, OllamaCompletionProvider>(|provider| {
                provider.update(
                    model.clone(),
                    api_url.clone(),
                    low_speed_timeout_in_seconds.map(Duration::from_secs),
                    version,
                    cx,
                );
            }),
        };

        // Previously configured provider was changed to another one
        if updated.is_none() {
            if let Some(client) = self.client.clone() {
                self.provider = create_provider_from_settings(client, version, cx);
            } else {
                log::warn!("completion provider cannot be created because client is not set");
            }
        }
    }
}

fn create_provider_from_settings(
    client: Arc<Client>,
    settings_version: usize,
    cx: &mut AppContext,
) -> Arc<RwLock<dyn LanguageModelCompletionProvider>> {
    match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { model } => Arc::new(RwLock::new(
            CloudCompletionProvider::new(model.clone(), client.clone(), settings_version, cx),
        )),
        AssistantProvider::OpenAi {
            model,
            api_url,
            low_speed_timeout_in_seconds,
            available_models,
        } => Arc::new(RwLock::new(OpenAiCompletionProvider::new(
            choose_openai_model(&model, &available_models),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
        ))),
        AssistantProvider::Anthropic {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => Arc::new(RwLock::new(AnthropicCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
        ))),
        AssistantProvider::Ollama {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => Arc::new(RwLock::new(OllamaCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
            cx,
        ))),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use gpui::AppContext;
    use parking_lot::RwLock;
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

        let provider = CompletionProvider::new(Arc::new(RwLock::new(fake_provider.clone())), None);

        // Enqueue some requests
        for i in 0..MAX_CONCURRENT_COMPLETION_REQUESTS * 2 {
            let response = provider.stream_completion(
                LanguageModelRequest {
                    temperature: i as f32 / 10.0,
                    ..Default::default()
                },
                cx,
            );
            cx.background_executor()
                .spawn(async move {
                    let mut stream = response.await.unwrap();
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
            .pending_completions()
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
        for request in fake_provider.pending_completions() {
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
        for request in fake_provider.pending_completions() {
            fake_provider.finish_completion(&request);
        }

        cx.background_executor().run_until_parked();

        assert_eq!(fake_provider.completion_count(), 0);
    }
}
