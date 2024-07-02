use anyhow::Result;
use futures::{channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, Task};
use std::sync::Arc;
use ui::WindowContext;

use crate::{LanguageModel, LanguageModelCompletionProvider, LanguageModelRequest};

#[derive(Clone, Default)]
pub struct FakeCompletionProvider {
    current_completion_tx: Arc<parking_lot::Mutex<Option<mpsc::UnboundedSender<String>>>>,
}

impl FakeCompletionProvider {
    #[cfg(test)]
    pub fn setup_test(cx: &mut AppContext) -> Self {
        let this = Self::default();
        let provider = crate::CompletionProvider {
            provider: Box::new(this.clone()),
            client: None,
        };
        cx.set_global(provider);
        this
    }

    pub fn send_completion(&self, chunk: String) {
        self.current_completion_tx
            .lock()
            .as_ref()
            .unwrap()
            .unbounded_send(chunk)
            .unwrap();
    }

    pub fn finish_completion(&self) {
        self.current_completion_tx.lock().take();
    }
}

impl LanguageModelCompletionProvider for FakeCompletionProvider {
    fn available_models(&self, _cx: &AppContext) -> Vec<LanguageModel> {
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

    fn complete(
        &self,
        _request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let (tx, rx) = mpsc::unbounded();
        *self.current_completion_tx.lock() = Some(tx);
        async move { Ok(rx.map(Ok).boxed()) }.boxed()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}
