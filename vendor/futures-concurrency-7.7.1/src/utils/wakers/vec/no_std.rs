use core::ops::{Deref, DerefMut};
use core::task::Waker;

#[derive(Debug)]
pub(crate) struct ReadinessVec {
    parent_waker: Option<Waker>,
}

impl ReadinessVec {
    pub(crate) fn new() -> Self {
        Self { parent_waker: None }
    }

    /// Returns the old ready state for this id
    pub(crate) fn set_ready(&mut self, _id: usize) -> bool {
        false
    }

    /// Set all markers to ready.
    pub(crate) fn set_all_ready(&mut self) {}

    /// Returns whether the task id was previously ready
    pub(crate) fn clear_ready(&mut self, _id: usize) -> bool {
        true
    }

    /// Returns `true` if any of the wakers are ready.
    pub(crate) fn any_ready(&self) -> bool {
        true
    }

    /// Access the parent waker.
    #[inline]
    pub(crate) fn parent_waker(&self) -> Option<&Waker> {
        self.parent_waker.as_ref()
    }

    /// Set the parent `Waker`. This needs to be called at the start of every
    /// `poll` function.
    pub(crate) fn set_waker(&mut self, parent_waker: &Waker) {
        match &mut self.parent_waker {
            Some(prev) => prev.clone_from(parent_waker),
            None => self.parent_waker = Some(parent_waker.clone()),
        }
    }

    /// Resize `readiness` to the new length.
    ///
    /// If new entries are created, they will be marked as 'ready'.
    pub(crate) fn resize(&mut self, _len: usize) {}
}

pub(crate) struct ReadinessVecRef<'a> {
    inner: &'a mut ReadinessVec,
}

impl<'a> Deref for ReadinessVecRef<'a> {
    type Target = ReadinessVec;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> DerefMut for ReadinessVecRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

/// A collection of wakers which delegate to an in-line waker.
pub(crate) struct WakerVec {
    readiness: ReadinessVec,
}

impl Default for WakerVec {
    fn default() -> Self {
        Self::new(0)
    }
}

impl WakerVec {
    /// Create a new instance of `WakerArray`.
    pub(crate) fn new(_len: usize) -> Self {
        let readiness = ReadinessVec::new();
        Self { readiness }
    }

    pub(crate) fn get(&self, _index: usize) -> Option<&Waker> {
        self.readiness.parent_waker()
    }

    /// Access the `Readiness`.
    pub(crate) fn readiness(&mut self) -> ReadinessVecRef<'_> {
        ReadinessVecRef {
            inner: &mut self.readiness,
        }
    }

    /// Resize the `WakerVec` to the new size.
    pub(crate) fn resize(&mut self, len: usize) {
        self.readiness.resize(len);
    }
}
