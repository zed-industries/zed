use anyhow::Result;
use collections::HashMap;
use futures::{channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, Task};
use std::sync::Arc;
use ui::WindowContext;

use crate::{LanguageModel, LanguageModelCompletionProvider, LanguageModelRequest};

#[derive(Clone, Default)]
pub struct FakeCompletionProvider {
    current_completion_txs: Arc<parking_lot::Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
}

impl FakeCompletionProvider {
    pub fn setup_test(cx: &mut AppContext) -> Self {
        use crate::CompletionProvider;
        use parking_lot::RwLock;

        let this = Self::default();
        let provider = CompletionProvider::new(Arc::new(RwLock::new(this.clone())), None);
        cx.set_global(provider);
        this
    }

    pub fn pending_completions(&self) -> Vec<LanguageModelRequest> {
        self.current_completion_txs
            .lock()
            .keys()
            .map(|k| serde_json::from_str(k).unwrap())
            .collect()
    }

    pub fn completion_count(&self) -> usize {
        self.current_completion_txs.lock().len()
    }

    pub fn send_completion_chunk(&self, request: &LanguageModelRequest, chunk: String) {
        let json = serde_json::to_string(request).unwrap();
        self.current_completion_txs
            .lock()
            .get(&json)
            .unwrap()
            .unbounded_send(chunk)
            .unwrap();
    }

    pub fn send_last_completion_chunk(&self, chunk: String) {
        self.send_completion_chunk(self.pending_completions().last().unwrap(), chunk);
    }

    pub fn finish_completion(&self, request: &LanguageModelRequest) {
        self.current_completion_txs
            .lock()
            .remove(&serde_json::to_string(request).unwrap())
            .unwrap();
    }

    pub fn finish_last_completion(&self) {
        self.finish_completion(self.pending_completions().last().unwrap());
    }
}

impl LanguageModelCompletionProvider for FakeCompletionProvider {
    fn available_models(&self) -> Vec<LanguageModel> {
        vec![LanguageModel::default()]
    }

    fn settings_version(&self) -> usize {
        0
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn authenticate(&self, _cx: &AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn authentication_prompt(&self, _cx: &mut WindowContext) -> AnyView {
        unimplemented!()
    }

    fn reset_credentials(&self, _cx: &AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn model(&self) -> LanguageModel {
        LanguageModel::default()
    }

    fn count_tokens(
        &self,
        _request: LanguageModelRequest,
        _cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        futures::future::ready(Ok(0)).boxed()
    }

    fn stream_completion(
        &self,
        _request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let (tx, rx) = mpsc::unbounded();
        self.current_completion_txs
            .lock()
            .insert(serde_json::to_string(&_request).unwrap(), tx);
        async move { Ok(rx.map(Ok).boxed()) }.boxed()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
