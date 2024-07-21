use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AppContext, Global, Model, ModelContext, Task};
use language_model::{
    LanguageModel, LanguageModelProvider, LanguageModelProviderName, LanguageModelRegistry,
    LanguageModelRequest,
};
use smol::lock::{Semaphore, SemaphoreGuardArc};
use std::{pin::Pin, sync::Arc, task::Poll};
use ui::Context;

pub fn init(cx: &mut AppContext) {
    let completion_provider = cx.new_model(|cx| LanguageModelCompletionProvider::new(cx));
    cx.set_global(GlobalLanguageModelCompletionProvider(completion_provider));
}

struct GlobalLanguageModelCompletionProvider(Model<LanguageModelCompletionProvider>);

impl Global for GlobalLanguageModelCompletionProvider {}

pub struct LanguageModelCompletionProvider {
    active_provider: Option<Arc<dyn LanguageModelProvider>>,
    active_model: Option<Arc<dyn LanguageModel>>,
    request_limiter: Arc<Semaphore>,
}

const MAX_CONCURRENT_COMPLETION_REQUESTS: usize = 4;

pub struct LanguageModelCompletionResponse {
    pub inner: BoxStream<'static, Result<String>>,
    _lock: SemaphoreGuardArc,
}

impl futures::Stream for LanguageModelCompletionResponse {
    type Item = Result<String>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
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
            active_provider: None,
            active_model: None,
            request_limiter: Arc::new(Semaphore::new(MAX_CONCURRENT_COMPLETION_REQUESTS)),
        }
    }

    pub fn active_provider(&self) -> Option<Arc<dyn LanguageModelProvider>> {
        self.active_provider.clone()
    }

    pub fn set_active_provider(
        &mut self,
        provider_name: LanguageModelProviderName,
        cx: &mut ModelContext<Self>,
    ) {
        self.active_provider = LanguageModelRegistry::global(cx)
            .read(cx)
            .provider(&provider_name);
        cx.notify();
    }

    pub fn active_model(&self) -> Option<Arc<dyn LanguageModel>> {
        self.active_model.clone()
    }

    pub fn set_active_model(&mut self, model: Arc<dyn LanguageModel>, cx: &mut ModelContext<Self>) {
        self.active_provider = LanguageModelRegistry::global(cx)
            .read(cx)
            .provider(&model.provider_name());
        self.active_model = Some(model);
        cx.notify();
    }

    pub fn is_authenticated(&self, cx: &AppContext) -> bool {
        self.active_provider
            .as_ref()
            .map_or(false, |provider| provider.is_authenticated(cx))
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        self.active_provider
            .as_ref()
            .map_or(Task::ready(Ok(())), |provider| provider.authenticate(cx))
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        self.active_provider
            .as_ref()
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

    pub fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> Task<Result<LanguageModelCompletionResponse>> {
        if let Some(language_model) = self.active_model() {
            let rate_limiter = self.request_limiter.clone();
            cx.spawn(|cx| async move {
                let lock = cx
                    .background_executor()
                    .spawn(async move { rate_limiter.acquire_arc().await })
                    .await;

                let Ok(response) = cx.update(|cx| language_model.stream_completion(request, &cx))
                else {
                    return Err(anyhow!("App state dropped"));
                };

                let response = response.await?;
                Ok(LanguageModelCompletionResponse {
                    inner: response,
                    _lock: lock,
                })
            })
        } else {
            Task::ready(Err(anyhow!("No active model set")))
        }
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

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;

//     use gpui::AppContext;
//     use parking_lot::RwLock;
//     use settings::SettingsStore;
//     use smol::stream::StreamExt;

//     use crate::{
//         FakeCompletionProvider, LanguageModelCompletionProvider, LanguageModelRequest,
//         MAX_CONCURRENT_COMPLETION_REQUESTS,
//     };

//     #[gpui::test]
//     fn test_rate_limiting(cx: &mut AppContext) {
//         SettingsStore::test(cx);
//         let fake_provider = FakeCompletionProvider::setup_test(cx);

//         let provider = LanguageModelCompletionProvider::new(
//             Arc::new(RwLock::new(fake_provider.clone())),
//             None,
//         );

//         // Enqueue some requests
//         for i in 0..MAX_CONCURRENT_COMPLETION_REQUESTS * 2 {
//             let response = provider.stream_completion(
//                 LanguageModelRequest {
//                     temperature: i as f32 / 10.0,
//                     ..Default::default()
//                 },
//                 cx,
//             );
//             cx.background_executor()
//                 .spawn(async move {
//                     let mut stream = response.await.unwrap();
//                     while let Some(message) = stream.next().await {
//                         message.unwrap();
//                     }
//                 })
//                 .detach();
//         }
//         cx.background_executor().run_until_parked();

//         assert_eq!(
//             fake_provider.completion_count(),
//             MAX_CONCURRENT_COMPLETION_REQUESTS
//         );

//         // Get the first completion request that is in flight and mark it as completed.
//         let completion = fake_provider
//             .pending_completions()
//             .into_iter()
//             .next()
//             .unwrap();
//         fake_provider.finish_completion(&completion);

//         // Ensure that the number of in-flight completion requests is reduced.
//         assert_eq!(
//             fake_provider.completion_count(),
//             MAX_CONCURRENT_COMPLETION_REQUESTS - 1
//         );

//         cx.background_executor().run_until_parked();

//         // Ensure that another completion request was allowed to acquire the lock.
//         assert_eq!(
//             fake_provider.completion_count(),
//             MAX_CONCURRENT_COMPLETION_REQUESTS
//         );

//         // Mark all completion requests as finished that are in flight.
//         for request in fake_provider.pending_completions() {
//             fake_provider.finish_completion(&request);
//         }

//         assert_eq!(fake_provider.completion_count(), 0);

//         // Wait until the background tasks acquire the lock again.
//         cx.background_executor().run_until_parked();

//         assert_eq!(
//             fake_provider.completion_count(),
//             MAX_CONCURRENT_COMPLETION_REQUESTS - 1
//         );

//         // Finish all remaining completion requests.
//         for request in fake_provider.pending_completions() {
//             fake_provider.finish_completion(&request);
//         }

//         cx.background_executor().run_until_parked();

//         assert_eq!(fake_provider.completion_count(), 0);
//     }
// }
