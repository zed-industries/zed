#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use alloc::sync::Arc;
use core::task::Waker;
use std::sync::{Mutex, MutexGuard};

use super::{InlineWakerVec, ReadinessVec};

/// A collection of wakers which delegate to an in-line waker.
pub(crate) struct WakerVec {
    wakers: Vec<Waker>,
    readiness: Arc<Mutex<ReadinessVec>>,
}

impl Default for WakerVec {
    fn default() -> Self {
        Self::new(0)
    }
}

impl WakerVec {
    /// Create a new instance of `WakerVec`.
    pub(crate) fn new(len: usize) -> Self {
        let readiness = Arc::new(Mutex::new(ReadinessVec::new(len)));
        let wakers = (0..len)
            .map(|i| Arc::new(InlineWakerVec::new(i, readiness.clone())).into())
            .collect();
        Self { wakers, readiness }
    }

    pub(crate) fn get(&self, index: usize) -> Option<&Waker> {
        self.wakers.get(index)
    }

    /// Access the `Readiness`.
    pub(crate) fn readiness(&self) -> MutexGuard<'_, ReadinessVec> {
        self.readiness.lock().unwrap()
    }

    /// Resize the `WakerVec` to the new size.
    pub(crate) fn resize(&mut self, len: usize) {
        // If we grow the vec we'll need to extend beyond the current index.
        // Which means the first position is the current length, and every position
        // beyond that is incremented by 1.
        let mut index = self.wakers.len();
        self.wakers.resize_with(len, || {
            let ret = Arc::new(InlineWakerVec::new(index, self.readiness.clone())).into();
            index += 1;
            ret
        });

        let mut readiness = self.readiness.lock().unwrap();
        readiness.resize(len);
    }
}
