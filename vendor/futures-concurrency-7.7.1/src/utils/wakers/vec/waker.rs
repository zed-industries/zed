use alloc::sync::Arc;
use alloc::task::Wake;
use std::sync::Mutex;

use super::ReadinessVec;

/// An efficient waker which delegates wake events.
#[derive(Debug, Clone)]
pub(crate) struct InlineWakerVec {
    pub(crate) id: usize,
    pub(crate) readiness: Arc<Mutex<ReadinessVec>>,
}

impl InlineWakerVec {
    /// Create a new instance of `InlineWaker`.
    pub(crate) fn new(id: usize, readiness: Arc<Mutex<ReadinessVec>>) -> Self {
        Self { id, readiness }
    }
}

impl Wake for InlineWakerVec {
    fn wake(self: Arc<Self>) {
        let mut readiness = self.readiness.lock().unwrap();
        if !readiness.set_ready(self.id) {
            readiness
                .parent_waker()
                .expect("`parent_waker` not available from `Readiness`. Did you forget to call `Readiness::set_waker`?")
                .wake_by_ref()
        }
    }
}
