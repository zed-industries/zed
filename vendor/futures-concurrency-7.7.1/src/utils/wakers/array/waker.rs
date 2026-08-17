use alloc::sync::Arc;
use alloc::task::Wake;
use std::sync::Mutex;

use super::ReadinessArray;

/// An efficient waker which delegates wake events.
#[derive(Debug, Clone)]
pub(crate) struct InlineWakerArray<const N: usize> {
    pub(crate) id: usize,
    pub(crate) readiness: Arc<Mutex<ReadinessArray<N>>>,
}

impl<const N: usize> InlineWakerArray<N> {
    /// Create a new instance of `InlineWaker`.
    pub(crate) fn new(id: usize, readiness: Arc<Mutex<ReadinessArray<N>>>) -> Self {
        Self { id, readiness }
    }
}

impl<const N: usize> Wake for InlineWakerArray<N> {
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
