use core::ops::{Deref, DerefMut};
use core::task::Waker;

#[derive(Debug)]
pub(crate) struct ReadinessArray<const N: usize> {
    parent_waker: Option<Waker>,
}

impl<const N: usize> ReadinessArray<N> {
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
}

pub(crate) struct ReadinessArrayRef<'a, const N: usize> {
    inner: &'a mut ReadinessArray<N>,
}

impl<'a, const N: usize> Deref for ReadinessArrayRef<'a, N> {
    type Target = ReadinessArray<N>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, const N: usize> DerefMut for ReadinessArrayRef<'a, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

/// A collection of wakers which delegate to an in-line waker.
pub(crate) struct WakerArray<const N: usize> {
    readiness: ReadinessArray<N>,
}

impl<const N: usize> WakerArray<N> {
    /// Create a new instance of `WakerArray`.
    pub(crate) fn new() -> Self {
        let readiness = ReadinessArray::new();
        Self { readiness }
    }

    pub(crate) fn get(&self, _index: usize) -> Option<&Waker> {
        self.readiness.parent_waker()
    }

    /// Access the `Readiness`.
    pub(crate) fn readiness(&mut self) -> ReadinessArrayRef<'_, N> {
        ReadinessArrayRef {
            inner: &mut self.readiness,
        }
    }
}
