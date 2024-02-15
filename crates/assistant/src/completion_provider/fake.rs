use std::sync::Arc;

#[derive(Clone, Default)]
pub struct FakeCompletionProvider {
    current_completion_tx:
        Arc<parking_lot::Mutex<Option<futures::channel::mpsc::UnboundedSender<String>>>>,
}

impl FakeCompletionProvider {
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
