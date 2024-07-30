use crate::{
    LanguageModel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest,
};
use anyhow::anyhow;
use collections::HashMap;
use futures::{channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, AsyncAppContext, Task};
use http_client::Result;
use std::{
    future,
    sync::{Arc, Mutex},
};
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
pub struct FakeLanguageModelProvider {
    current_completion_txs: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<String>>>>,
}

impl LanguageModelProviderState for FakeLanguageModelProvider {
    fn subscribe<T: 'static>(&self, _: &mut gpui::ModelContext<T>) -> Option<gpui::Subscription> {
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
        vec![Arc::new(FakeLanguageModel {
            current_completion_txs: self.current_completion_txs.clone(),
        })]
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
}

impl FakeLanguageModelProvider {
    pub fn test_model(&self) -> FakeLanguageModel {
        FakeLanguageModel {
            current_completion_txs: self.current_completion_txs.clone(),
        }
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
        self.current_completion_txs
            .lock()
            .unwrap()
            .insert(serde_json::to_string(&request).unwrap(), tx);
        async move { Ok(rx.map(Ok).boxed()) }.boxed()
    }

    fn use_tool(
        &self,
        _request: LanguageModelRequest,
        _name: String,
        _description: String,
        _schema: serde_json::Value,
        _cx: &AsyncAppContext,
    ) -> BoxFuture<'static, Result<serde_json::Value>> {
        future::ready(Err(anyhow!("not implemented"))).boxed()
    }
}
