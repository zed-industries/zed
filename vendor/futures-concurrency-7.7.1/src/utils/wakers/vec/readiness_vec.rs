use core::task::Waker;
use fixedbitset::FixedBitSet;

/// Tracks which wakers are "ready" and should be polled.
#[derive(Debug)]
pub(crate) struct ReadinessVec {
    ready_count: usize,
    max_count: usize,
    readiness_list: FixedBitSet,
    parent_waker: Option<Waker>,
}

impl ReadinessVec {
    /// Create a new instance of readiness.
    pub(crate) fn new(len: usize) -> Self {
        Self {
            ready_count: len,
            max_count: len,
            // See https://github.com/petgraph/fixedbitset/issues/101
            readiness_list: FixedBitSet::with_capacity_and_blocks(len, std::iter::repeat(!0)),
            parent_waker: None,
        }
    }

    /// Set the ready state to `true` for the given index
    ///
    /// Returns the old ready state for this id
    pub(crate) fn set_ready(&mut self, index: usize) -> bool {
        if !self.readiness_list[index] {
            self.ready_count += 1;
            self.readiness_list.set(index, true);
            false
        } else {
            true
        }
    }

    /// Set all markers to ready.
    pub(crate) fn set_all_ready(&mut self) {
        self.readiness_list.set_range(.., true);
        self.ready_count = self.max_count;
    }

    /// Set the ready state to `false` for the given index
    ///
    /// Returns whether the task id was previously ready
    pub(crate) fn clear_ready(&mut self, index: usize) -> bool {
        if self.readiness_list[index] {
            self.ready_count -= 1;
            self.readiness_list.set(index, false);
            true
        } else {
            false
        }
    }

    /// Returns whether the task id was previously ready
    #[allow(unused)]
    pub(crate) fn clear_all_ready(&mut self) {
        self.readiness_list.set_range(.., false);
        self.ready_count = 0;
    }

    /// Returns `true` if any of the wakers are ready.
    pub(crate) fn any_ready(&self) -> bool {
        self.ready_count > 0
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
    pub(crate) fn resize(&mut self, len: usize) {
        self.max_count = len;

        let old_len = self.readiness_list.len();
        match len.cmp(&old_len) {
            std::cmp::Ordering::Less => {
                // shrink
                self.ready_count -= self.readiness_list.count_ones(len..);
                self.readiness_list = FixedBitSet::with_capacity_and_blocks(
                    len,
                    self.readiness_list.as_slice().iter().cloned(),
                );
            }
            std::cmp::Ordering::Equal => {
                // no-op
            }
            std::cmp::Ordering::Greater => {
                // grow
                self.readiness_list.grow(len);
                self.readiness_list.set_range(old_len..len, true);
                self.ready_count += len - old_len;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn resize() {
        let mut readiness = ReadinessVec::new(10);
        assert!(readiness.any_ready());
        readiness.clear_all_ready();
        assert!(!readiness.any_ready());
        readiness.set_ready(9);
        assert!(readiness.any_ready());
        readiness.resize(9);
        assert!(!readiness.any_ready());
        readiness.resize(10);
        assert!(readiness.any_ready());
    }
}
