use std::sync::{Arc, Mutex};

use collections::HashMap;
use futures::{channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};

use crate::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderName, LanguageModelProviderState, LanguageModelRequest,
    ProvidedLanguageModel,
};
use gpui::{AnyView, AppContext, Task};
use http::Result;
use ui::WindowContext;

#[derive(Clone, Default)]
pub struct FakeLanguageModelProvider {}

impl LanguageModelProviderState for FakeLanguageModelProvider {
    fn subscribe<T: 'static>(&self, _: &mut gpui::ModelContext<T>) -> Option<gpui::Subscription> {
        None
    }
}

impl LanguageModelProvider for FakeLanguageModelProvider {
    fn name(&self, _: &AppContext) -> LanguageModelProviderName {
        LanguageModelProviderName::from("Fake Language Model Provider".to_string())
    }

    fn provided_models(&self, _: &AppContext) -> Vec<ProvidedLanguageModel> {
        vec![ProvidedLanguageModel {
            id: LanguageModelId::from("fake".to_string()),
            name: LanguageModelName::from("Fake".to_string()),
        }]
    }

    fn is_authenticated(&self, _: &AppContext) -> bool {
        true
    }

    fn authenticate(&self, _: &AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn authentication_prompt(&self, _: &mut WindowContext) -> AnyView {
        unimplemented!()
    }

    fn reset_credentials(&self, _: &AppContext) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn model(&self, _: LanguageModelId, _: &AppContext) -> Result<Arc<dyn LanguageModel>> {
        Ok(Arc::new(FakeLanguageModel {
            current_completion_txs: Arc::new(Mutex::new(HashMap::default())),
        }))
    }
}

impl FakeLanguageModelProvider {
    pub fn test_model() -> Arc<FakeLanguageModel> {
        Arc::new(FakeLanguageModel {
            current_completion_txs: Arc::new(Mutex::new(HashMap::default())),
        })
    }
}

pub struct FakeLanguageModel {
    current_completion_txs: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
}

impl FakeLanguageModel {
    pub fn pending_completions(&self) -> Vec<LanguageModelRequest> {
        self.current_completion_txs
            .lock()
            .unwrap()
            .keys()
            .map(|k| serde_json::from_str(k).unwrap())
            .collect()
    }

    pub fn completion_count(&self) -> usize {
        self.current_completion_txs.lock().unwrap().len()
    }

    pub fn send_completion_chunk(&self, request: &LanguageModelRequest, chunk: String) {
        let json = serde_json::to_string(request).unwrap();
        self.current_completion_txs
            .lock()
            .unwrap()
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
            .unwrap()
            .remove(&serde_json::to_string(request).unwrap())
            .unwrap();
    }

    pub fn finish_last_completion(&self) {
        self.finish_completion(self.pending_completions().last().unwrap());
    }
}

impl LanguageModel for FakeLanguageModel {
    fn id(&self) -> LanguageModelId {
        LanguageModelId::from("fake".to_string())
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from("Fake".to_string())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName::from("Fake Language Model Provider".to_string())
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
        _: &AppContext,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let (tx, rx) = mpsc::unbounded();
        self.current_completion_txs
            .lock()
            .unwrap()
            .insert(serde_json::to_string(&request).unwrap(), tx);
        async move { Ok(rx.map(Ok).boxed()) }.boxed()
    }
}
