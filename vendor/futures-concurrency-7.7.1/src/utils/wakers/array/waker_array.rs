use alloc::sync::Arc;
use core::array;
use core::task::Waker;
use std::sync::{Mutex, MutexGuard};

use super::{InlineWakerArray, ReadinessArray};

/// A collection of wakers which delegate to an in-line waker.
pub(crate) struct WakerArray<const N: usize> {
    wakers: [Waker; N],
    readiness: Arc<Mutex<ReadinessArray<N>>>,
}

impl<const N: usize> WakerArray<N> {
    /// Create a new instance of `WakerArray`.
    pub(crate) fn new() -> Self {
        let readiness = Arc::new(Mutex::new(ReadinessArray::new()));
        Self {
            wakers: array::from_fn(|i| {
                Arc::new(InlineWakerArray::new(i, readiness.clone())).into()
            }),
            readiness,
        }
    }

    pub(crate) fn get(&self, index: usize) -> Option<&Waker> {
        self.wakers.get(index)
    }

    /// Access the `Readiness`.
    pub(crate) fn readiness(&mut self) -> MutexGuard<'_, ReadinessArray<N>> {
        self.readiness.as_ref().lock().unwrap()
    }
}
