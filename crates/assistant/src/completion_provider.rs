use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, stream::BoxStream};
use gpui::{AppContext, Global, Model, ModelContext, Task};
use language_model::{
    registry::LanguageModelRegistry, LanguageModel, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderName, LanguageModelRequest,
};
use settings::{Settings, SettingsStore};
use smol::{
    future::FutureExt,
    lock::{Semaphore, SemaphoreGuardArc},
};
use ui::Context;

use crate::assistant_settings::AssistantSettings;

pub fn init(cx: &mut AppContext) {
    let completion_provider = cx.new_model(|cx| LanguageModelCompletionProvider::new(cx));
    cx.set_global(GlobalLanguageModelCompletionProvider(completion_provider));

    update_active_model_from_settings(cx);

    cx.observe_global::<SettingsStore>(move |cx| update_active_model_from_settings(cx))
        .detach();
}

fn update_active_model_from_settings(cx: &mut AppContext) {
    let settings = AssistantSettings::get_global(cx);
    let provider_name = LanguageModelProviderName::from(settings.default_model.provider.clone());
    let model_name = LanguageModelName::from(settings.default_model.model.clone());

    let Some(provider) = LanguageModelRegistry::global(cx)
        .read(cx)
        .provider(&provider_name)
    else {
        return;
    };

    let Some(model_id) = provider
        .provided_models(cx)
        .iter()
        .find(|model| model.name == model_name)
        .map(|model| model.id.clone())
    else {
        return;
    };

    let Ok(model) = provider.model(model_id, cx) else {
        return;
    };

    LanguageModelCompletionProvider::global(cx).update(cx, |completion_provider, cx| {
        completion_provider.set_active_model(model, cx);
    });
}

struct GlobalLanguageModelCompletionProvider(Model<LanguageModelCompletionProvider>);

impl Global for GlobalLanguageModelCompletionProvider {}

pub struct LanguageModelCompletionProvider {
    active_model: Option<Arc<dyn LanguageModel>>,
    request_limiter: Arc<Semaphore>,
}

const MAX_CONCURRENT_COMPLETION_REQUESTS: usize = 4;

pub struct CompletionResponse {
    pub inner: BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>,
    _lock: SemaphoreGuardArc,
}

impl LanguageModelCompletionProvider {
    pub fn global(cx: &AppContext) -> Model<Self> {
        cx.global::<GlobalLanguageModelCompletionProvider>()
            .0
            .clone()
    }

    pub fn read_global(cx: &AppContext) -> &Self {
        cx.global::<GlobalLanguageModelCompletionProvider>()
            .0
            .read(cx)
    }

    pub fn new(cx: &mut ModelContext<Self>) -> Self {
        cx.observe(&LanguageModelRegistry::global(cx), |_, _, cx| {
            cx.notify();
        })
        .detach();

        Self {
            active_model: None,
            request_limiter: Arc::new(Semaphore::new(MAX_CONCURRENT_COMPLETION_REQUESTS)),
        }
    }

    pub fn active_model(&self) -> Option<Arc<dyn LanguageModel>> {
        self.active_model.clone()
    }

    pub fn set_active_model(&mut self, model: Arc<dyn LanguageModel>, cx: &mut ModelContext<Self>) {
        self.active_model = Some(model);
        cx.notify();
    }

    pub fn current_provider(&self, cx: &AppContext) -> Option<Arc<dyn LanguageModelProvider>> {
        let provider_name = self.active_model.as_ref()?.provider_name();
        LanguageModelRegistry::global(cx)
            .read(cx)
            .provider(&provider_name)
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
            cx.spawn(|cx| async move {
                let lock = cx
                    .background_executor()
                    .spawn(async move { rate_limiter.acquire_arc().await })
                    .await;

                let Ok(response) = cx.update(|cx| language_model.complete(request, &cx)) else {
                    return Err(anyhow!("App state dropped"));
                };

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
