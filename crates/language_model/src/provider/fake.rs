use crate::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest,
};
use futures::{channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, AsyncAppContext, Task};
use http_client::Result;
use parking_lot::Mutex;
use serde::Serialize;
use std::sync::Arc;
use ui::WindowContext;

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

    fn observable_entity(&self) -> Option<gpui::Model<Self::ObservableEntity>> {
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

    fn provided_models(&self, _: &AppContext) -> Vec<Arc<dyn LanguageModel>> {
        vec![Arc::new(FakeLanguageModel::default())]
    }

    fn is_authenticated(&self, _: &AppContext) -> bool {
        true
    }

    fn authenticate(&self, _: &mut AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn configuration_view(&self, _: &mut WindowContext) -> AnyView {
        unimplemented!()
    }

    fn reset_credentials(&self, _: &mut AppContext) -> Task<Result<()>> {
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
    current_tool_use_txs: Mutex<Vec<(ToolUseRequest, mpsc::UnboundedSender<String>)>>,
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

    pub fn respond_to_last_tool_use<T: Serialize>(&self, response: T) {
        let response = serde_json::to_string(&response).unwrap();
        let mut current_tool_call_txs = self.current_tool_use_txs.lock();
        let (_, tx) = current_tool_call_txs.pop().unwrap();
        tx.unbounded_send(response).unwrap();
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

    fn telemetry_id(&self) -> String {
        "fake".to_string()
    }

    fn max_token_count(&self) -> usize {
        1000000
    }

    fn count_tokens(
        &self,
        _: LanguageModelRequest,
        _: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        futures::future::ready(Ok(0)).boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        _: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let (tx, rx) = mpsc::unbounded();
        self.current_completion_txs.lock().push((request, tx));
        async move { Ok(rx.map(Ok).boxed()) }.boxed()
    }

    fn use_any_tool(
        &self,
        request: LanguageModelRequest,
        name: String,
        description: String,
        schema: serde_json::Value,
        _cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let (tx, rx) = mpsc::unbounded();
        let tool_call = ToolUseRequest {
            request,
            name,
            description,
            schema,
        };
        self.current_tool_use_txs.lock().push((tool_call, tx));
        async move { Ok(rx.map(Ok).boxed()) }.boxed()
    }

    fn as_fake(&self) -> &Self {
        self
    }
}
