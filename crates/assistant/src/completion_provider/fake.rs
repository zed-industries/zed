use anyhow::Result;
use futures::{channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct FakeCompletionProvider {
    current_completion_tx: Arc<parking_lot::Mutex<Option<mpsc::UnboundedSender<String>>>>,
}

impl FakeCompletionProvider {
    pub fn complete(&self) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let (tx, rx) = mpsc::unbounded();
        *self.current_completion_tx.lock() = Some(tx);
        async move { Ok(rx.map(Ok).boxed()) }.boxed()
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
