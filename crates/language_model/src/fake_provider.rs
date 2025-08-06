use crate::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelId, LanguageModelName, LanguageModelProvider, LanguageModelProviderId,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    LanguageModelToolChoice,
};
use futures::{FutureExt, StreamExt, channel::mpsc, future::BoxFuture, stream::BoxStream};
use gpui::{AnyView, App, AsyncApp, Entity, Task, Window};
use http_client::Result;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct FakeLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
}

impl Default for FakeLanguageModelProvider {
    fn default() -> Self {
        Self {
            id: LanguageModelProviderId::from("fake".to_string()),
            name: LanguageModelProviderName::from("Fake".to_string()),
        }
    }
}

impl LanguageModelProviderState for FakeLanguageModelProvider {
    type ObservableEntity = ();

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        None
    }
}

impl LanguageModelProvider for FakeLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelProviderName {
        self.name.clone()
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
    pub fn new(id: LanguageModelProviderId, name: LanguageModelProviderName) -> Self {
        Self { id, name }
    }

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

pub struct FakeLanguageModel {
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    current_completion_txs: Mutex<
        Vec<(
            LanguageModelRequest,
            mpsc::UnboundedSender<LanguageModelCompletionEvent>,
        )>,
    >,
}

impl Default for FakeLanguageModel {
    fn default() -> Self {
        Self {
            provider_id: LanguageModelProviderId::from("fake".to_string()),
            provider_name: LanguageModelProviderName::from("Fake".to_string()),
            current_completion_txs: Mutex::new(Vec::new()),
        }
    }
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

    pub fn send_completion_stream_text_chunk(
        &self,
        request: &LanguageModelRequest,
        chunk: impl Into<String>,
    ) {
        self.send_completion_stream_event(
            request,
            LanguageModelCompletionEvent::Text(chunk.into()),
        );
    }

    pub fn send_completion_stream_event(
        &self,
        request: &LanguageModelRequest,
        event: impl Into<LanguageModelCompletionEvent>,
    ) {
        let current_completion_txs = self.current_completion_txs.lock();
        let tx = current_completion_txs
            .iter()
            .find(|(req, _)| req == request)
            .map(|(_, tx)| tx)
            .unwrap();
        tx.unbounded_send(event.into()).unwrap();
    }

    pub fn end_completion_stream(&self, request: &LanguageModelRequest) {
        self.current_completion_txs
            .lock()
            .retain(|(req, _)| req != request);
    }

    pub fn send_last_completion_stream_text_chunk(&self, chunk: impl Into<String>) {
        self.send_completion_stream_text_chunk(self.pending_completions().last().unwrap(), chunk);
    }

    pub fn send_last_completion_stream_event(
        &self,
        event: impl Into<LanguageModelCompletionEvent>,
    ) {
        self.send_completion_stream_event(self.pending_completions().last().unwrap(), event);
    }

    pub fn end_last_completion_stream(&self) {
        self.end_completion_stream(self.pending_completions().last().unwrap());
    }
}

impl LanguageModel for FakeLanguageModel {
    fn id(&self) -> LanguageModelId {
        LanguageModelId::from("fake".to_string())
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from("Fake".to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        self.provider_id.clone()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        self.provider_name.clone()
    }

    fn supports_tools(&self) -> bool {
        false
    }

    fn supports_tool_choice(&self, _choice: LanguageModelToolChoice) -> bool {
        false
    }

    fn supports_images(&self) -> bool {
        false
    }

    fn telemetry_id(&self) -> String {
        "fake".to_string()
    }

    fn max_token_count(&self) -> u64 {
        1000000
    }

    fn count_tokens(&self, _: LanguageModelRequest, _: &App) -> BoxFuture<'static, Result<u64>> {
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
            LanguageModelCompletionError,
        >,
    > {
        let (tx, rx) = mpsc::unbounded();
        self.current_completion_txs.lock().push((request, tx));
        async move { Ok(rx.map(Ok).boxed()) }.boxed()
    }

    fn as_fake(&self) -> &Self {
        self
    }
}
