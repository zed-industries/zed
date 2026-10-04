use crate::{
    AuthenticateError, LanguageModel, LanguageModelClient, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelCompletionStream, LanguageModelId,
    LanguageModelName, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest,
};
use anyhow::anyhow;
use futures::{FutureExt, channel::mpsc, future::BoxFuture, stream::StreamExt};
use gpui::{App, AsyncApp, Entity, Task};
use http_client::Result;
use parking_lot::Mutex;
use std::collections::VecDeque;

/// A provider for tests that serves any model id and records every request.
///
/// Tests get models with [`Self::model`] and drive responses to the requests
/// the provider has received, like `send_last_text`.
pub struct FakeLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
    state: Mutex<FakeProviderState>,
}

#[derive(Default)]
struct FakeProviderState {
    models: Vec<LanguageModel>,
    pending_completions: Vec<PendingCompletion>,
    forbid_requests: bool,
    input_token_counts: VecDeque<u64>,
    input_token_count_requests: Vec<LanguageModelRequest>,
}

struct PendingCompletion {
    model_id: LanguageModelId,
    request: LanguageModelRequest,
    events:
        mpsc::UnboundedSender<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
}

impl Default for FakeLanguageModelProvider {
    fn default() -> Self {
        Self::new(
            LanguageModelProviderId::from("fake".to_string()),
            LanguageModelProviderName::from("Fake".to_string()),
        )
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

    fn default_model(&self, _cx: &App) -> Option<LanguageModel> {
        self.state.lock().models.first().cloned()
    }

    fn default_fast_model(&self, _cx: &App) -> Option<LanguageModel> {
        self.state.lock().models.first().cloned()
    }

    fn provided_models(&self, _: &App) -> Vec<LanguageModel> {
        self.state.lock().models.clone()
    }

    fn is_authenticated(&self, _: &App) -> bool {
        true
    }

    fn authenticate(&self, _: &mut App) -> Task<Result<(), AuthenticateError>> {
        Task::ready(Ok(()))
    }

    fn settings_view(&self, _: &mut App) -> Option<crate::ProviderSettingsView> {
        None
    }
}

impl LanguageModelClient for FakeLanguageModelProvider {
    fn stream_completion(
        &self,
        model: &LanguageModel,
        request: LanguageModelRequest,
        _: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let mut state = self.state.lock();
        if state.forbid_requests {
            return async { Err(anyhow!("requests are forbidden").into()) }.boxed();
        }
        let (events, rx) = mpsc::unbounded();
        state.pending_completions.push(PendingCompletion {
            model_id: model.id.clone(),
            request,
            events,
        });
        async move { Ok(rx.boxed()) }.boxed()
    }

    fn count_input_tokens(
        &self,
        _model: &LanguageModel,
        request: LanguageModelRequest,
        _: &AsyncApp,
    ) -> BoxFuture<'static, Result<Option<u64>, LanguageModelCompletionError>> {
        let mut state = self.state.lock();
        if state.forbid_requests {
            return async { Err(anyhow!("requests are forbidden").into()) }.boxed();
        }
        state.input_token_count_requests.push(request);
        let count = state.input_token_counts.pop_front();
        async move { Ok(count) }.boxed()
    }
}

impl FakeLanguageModelProvider {
    /// A provider offering one model, with id `fake`.
    pub fn new(id: LanguageModelProviderId, name: LanguageModelProviderName) -> Self {
        let provider = Self {
            id,
            name,
            state: Mutex::default(),
        };
        provider.update_model("fake", |model| {
            model.name = LanguageModelName::from("Fake".to_string())
        });
        provider
    }

    /// The model this provider offers as `id`, adding it with default
    /// capabilities if it isn't offered yet.
    pub fn model(&self, id: &str) -> LanguageModel {
        self.update_model(id, |_| {})
    }

    /// Changes the capabilities of the model offered as `id`, adding it first
    /// if needed, and returns the updated model.
    pub fn update_model(&self, id: &str, update: impl FnOnce(&mut LanguageModel)) -> LanguageModel {
        let mut state = self.state.lock();
        let index = match state
            .models
            .iter()
            .position(|model| model.id.0.as_ref() == id)
        {
            Some(index) => index,
            None => {
                state.models.push(LanguageModel {
                    supports_disabling_thinking: true,
                    ..LanguageModel::new(
                        LanguageModelId::from(id.to_string()),
                        LanguageModelName::from(id.to_string()),
                        self.id.clone(),
                        self.name.clone(),
                        "fake",
                        1_000_000,
                    )
                });
                state.models.len() - 1
            }
        };
        update(&mut state.models[index]);
        state.models[index].clone()
    }

    pub fn allow_requests(&self) {
        self.state.lock().forbid_requests = false;
    }

    pub fn forbid_requests(&self) {
        self.state.lock().forbid_requests = true;
    }

    pub fn queue_input_token_count(&self, count: u64) {
        self.state.lock().input_token_counts.push_back(count);
    }

    pub fn input_token_count_requests(&self) -> Vec<LanguageModelRequest> {
        self.state.lock().input_token_count_requests.clone()
    }

    /// Requests still streaming, to any model, oldest first.
    pub fn pending_completions(&self) -> Vec<LanguageModelRequest> {
        self.state
            .lock()
            .pending_completions
            .iter()
            .map(|pending| pending.request.clone())
            .collect()
    }

    /// Requests still streaming to `model`, oldest first.
    pub fn pending_completions_for(&self, model: &LanguageModel) -> Vec<LanguageModelRequest> {
        self.state
            .lock()
            .pending_completions
            .iter()
            .filter(|pending| pending.model_id == model.id)
            .map(|pending| pending.request.clone())
            .collect()
    }

    pub fn completion_count(&self) -> usize {
        self.state.lock().pending_completions.len()
    }

    /// Streams `chunk` as text to the oldest pending `request` to `model`.
    pub fn send_text(
        &self,
        model: &LanguageModel,
        request: &LanguageModelRequest,
        chunk: impl Into<String>,
    ) {
        self.send_event(
            model,
            request,
            LanguageModelCompletionEvent::Text(chunk.into()),
        );
    }

    /// Streams `event` to the oldest pending `request` to `model`.
    pub fn send_event(
        &self,
        model: &LanguageModel,
        request: &LanguageModelRequest,
        event: impl Into<LanguageModelCompletionEvent>,
    ) {
        let state = self.state.lock();
        state.send(state.pending_index(model, request), Ok(event.into()));
    }

    /// Streams `error` to the oldest pending `request` to `model`.
    pub fn send_error(
        &self,
        model: &LanguageModel,
        request: &LanguageModelRequest,
        error: impl Into<LanguageModelCompletionError>,
    ) {
        let state = self.state.lock();
        state.send(state.pending_index(model, request), Err(error.into()));
    }

    /// Ends the stream of the oldest pending `request` to `model`.
    pub fn end_stream(&self, model: &LanguageModel, request: &LanguageModelRequest) {
        let mut state = self.state.lock();
        let index = state.pending_index(model, request);
        state.pending_completions.remove(index);
    }

    /// Whether the oldest pending `request` to `model` has ended or had its
    /// receiver dropped.
    pub fn is_stream_closed(&self, model: &LanguageModel, request: &LanguageModelRequest) -> bool {
        self.state
            .lock()
            .pending_completions
            .iter()
            .find(|pending| pending.model_id == model.id && &pending.request == request)
            .is_none_or(|pending| pending.events.is_closed())
    }

    /// Streams `chunk` as text to the most recent pending request to `model`.
    pub fn send_last_text(&self, model: &LanguageModel, chunk: impl Into<String>) {
        self.send_last_event(model, LanguageModelCompletionEvent::Text(chunk.into()));
    }

    /// Streams `event` to the most recent pending request to `model`.
    pub fn send_last_event(
        &self,
        model: &LanguageModel,
        event: impl Into<LanguageModelCompletionEvent>,
    ) {
        let state = self.state.lock();
        state.send(state.last_pending_index(model), Ok(event.into()));
    }

    /// Streams `error` to the most recent pending request to `model`.
    pub fn send_last_error(
        &self,
        model: &LanguageModel,
        error: impl Into<LanguageModelCompletionError>,
    ) {
        let state = self.state.lock();
        state.send(state.last_pending_index(model), Err(error.into()));
    }

    /// Ends the stream of the most recent pending request to `model`.
    pub fn end_last(&self, model: &LanguageModel) {
        let mut state = self.state.lock();
        let index = state.last_pending_index(model);
        state.pending_completions.remove(index);
    }
}

impl FakeProviderState {
    fn pending_index(&self, model: &LanguageModel, request: &LanguageModelRequest) -> usize {
        self.pending_completions
            .iter()
            .position(|pending| pending.model_id == model.id && &pending.request == request)
            .unwrap_or_else(|| {
                panic!(
                    "no pending completion to model `{}` matches the request",
                    model.id.0
                )
            })
    }

    fn last_pending_index(&self, model: &LanguageModel) -> usize {
        self.pending_completions
            .iter()
            .rposition(|pending| pending.model_id == model.id)
            .unwrap_or_else(|| panic!("no pending completion to model `{}`", model.id.0))
    }

    fn send(
        &self,
        index: usize,
        event: Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
    ) {
        self.pending_completions[index]
            .events
            .unbounded_send(event)
            .unwrap();
    }
}

#[derive(Debug, PartialEq)]
pub struct ToolUseRequest {
    pub request: LanguageModelRequest,
    pub name: String,
    pub description: String,
    pub schema: serde_json::Value,
}
