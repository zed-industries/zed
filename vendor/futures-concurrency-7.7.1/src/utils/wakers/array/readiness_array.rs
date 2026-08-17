use core::task::Waker;

/// Tracks which wakers are "ready" and should be polled.
#[derive(Debug)]
pub(crate) struct ReadinessArray<const N: usize> {
    count: usize,
    readiness_list: [bool; N],
    parent_waker: Option<Waker>,
}

impl<const N: usize> ReadinessArray<N> {
    /// Create a new instance of readiness.
    pub(crate) fn new() -> Self {
        Self {
            count: N,
            readiness_list: [true; N], // TODO: use a bitarray instead
            parent_waker: None,
        }
    }

    /// Returns the old ready state for this id
    pub(crate) fn set_ready(&mut self, id: usize) -> bool {
        if !self.readiness_list[id] {
            self.count += 1;
            self.readiness_list[id] = true;

            false
        } else {
            true
        }
    }

    /// Set all markers to ready.
    pub(crate) fn set_all_ready(&mut self) {
        self.readiness_list.fill(true);
        self.count = N;
    }

    /// Returns whether the task id was previously ready
    pub(crate) fn clear_ready(&mut self, id: usize) -> bool {
        if self.readiness_list[id] {
            self.count -= 1;
            self.readiness_list[id] = false;

            true
        } else {
            false
        }
    }

    /// Returns `true` if any of the wakers are ready.
    pub(crate) fn any_ready(&self) -> bool {
        self.count > 0
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
