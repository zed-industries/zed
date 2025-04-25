use crate::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
};
use futures::{FutureExt, StreamExt, channel::mpsc, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Entity, Task, Window};
use http_client::Result;
use parking_lot::Mutex;
use std::sync::Arc;

pub fn language_model_id() -> LanguageModelId {
    LanguageModelId::from("fake".to_string())
}

pub fn language_model_name() -> LanguageModelName {
    LanguageModelName::from("Fake".to_string())
}

pub fn provider_id() -> LanguageModelProviderId {
    LanguageModelProviderId::from("fake".to_string())
}

pub fn provider_name() -> LanguageModelProviderName {
    LanguageModelProviderName::from("Fake".to_string())
}

#[derive(Clone, Default)]
pub struct FakeLanguageModelProvider;

impl LanguageModelProviderState for FakeLanguageModelProvider {
    type ObservableEntity = ();

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        None
    }
}

impl LanguageModelProvider for FakeLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        provider_id()
    }

    fn name(&self) -> LanguageModelProviderName {
        provider_name()
    }

    fn default_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(Arc::new(FakeLanguageModel::default()))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        Some(Arc::new(FakeLanguageModel::default()))
    }

    fn provided_models(&self, _: &App) -> Vec<Arc<dyn LanguageModel>> {
        vec![Arc::new(FakeLanguageModel::default())]
    }

    fn is_authenticated(&self, _: &App) -> bool {
        true
    }

    fn authenticate(&self, _: &mut App) -> Task<Result<(), AuthenticateError>> {
        Task::ready(Ok(()))
    }

    fn configuration_view(&self, _window: &mut Window, _: &mut App) -> AnyView {
        unimplemented!()
    }

    fn reset_credentials(&self, _: &mut App) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }
}

impl FakeLanguageModelProvider {
    pub fn test_model(&self) -> FakeLanguageModel {
        FakeLanguageModel::default()
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolUseRequest {
    pub request: LanguageModelRequest,
    pub name: String,
    pub description: String,
    pub schema: serde_json::Value,
}

#[derive(Default)]
pub struct FakeLanguageModel {
    current_completion_txs: Mutex<Vec<(LanguageModelRequest, mpsc::UnboundedSender<String>)>>,
}

impl FakeLanguageModel {
    pub fn pending_completions(&self) -> Vec<LanguageModelRequest> {
        self.current_completion_txs
            .lock()
            .iter()
            .map(|(request, _)| request.clone())
            .collect()
    }

    pub fn completion_count(&self) -> usize {
        self.current_completion_txs.lock().len()
    }

    pub fn stream_completion_response(&self, request: &LanguageModelRequest, chunk: String) {
        let current_completion_txs = self.current_completion_txs.lock();
        let tx = current_completion_txs
            .iter()
            .find(|(req, _)| req == request)
            .map(|(_, tx)| tx)
            .unwrap();
        tx.unbounded_send(chunk).unwrap();
    }

    pub fn end_completion_stream(&self, request: &LanguageModelRequest) {
        self.current_completion_txs
            .lock()
            .retain(|(req, _)| req != request);
    }

    pub fn stream_last_completion_response(&self, chunk: String) {
        self.stream_completion_response(self.pending_completions().last().unwrap(), chunk);
    }

    pub fn end_last_completion_stream(&self) {
        self.end_completion_stream(self.pending_completions().last().unwrap());
    }
}

impl LanguageModel for FakeLanguageModel {
    fn id(&self) -> LanguageModelId {
        language_model_id()
    }

    fn name(&self) -> LanguageModelName {
        language_model_name()
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        provider_id()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        provider_name()
    }

    fn supports_tools(&self) -> bool {
        false
    }

    fn telemetry_id(&self) -> String {
        "fake".to_string()
    }

    fn max_token_count(&self) -> usize {
        1000000
    }

    fn count_tokens(&self, _: LanguageModelRequest, _: &App) -> BoxFuture<'static, Result<usize>> {
        futures::future::ready(Ok(0)).boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        _: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
        >,
    > {
        let (tx, rx) = mpsc::unbounded();
        self.current_completion_txs.lock().push((request, tx));
        async move {
            Ok(rx
                .map(|text| Ok(LanguageModelCompletionEvent::Text(text)))
                .boxed())
        }
        .boxed()
    }

    fn as_fake(&self) -> &Self {
        self
    }
}
