//! [`Barrier`] is a synchronization primitive that enables multiple tasks to start execution at the
//! same time.

#![deny(unsafe_code)]

use std::fmt;
use std::pin::{Pin, pin};
use std::ptr::{null_mut, without_provenance_mut};
#[cfg(not(feature = "loom"))]
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{self, AcqRel, Acquire, Relaxed};

#[cfg(feature = "loom")]
use loom::sync::atomic::AtomicPtr;

use crate::Pager;
use crate::opcode::Opcode;
use crate::pager::{self, SyncResult};
use crate::sync_primitive::SyncPrimitive;
use crate::wait_queue::{Entry, WaitQueue};

/// [`Barrier`] is a synchronization primitive that enables multiple tasks to start execution at the
/// same time.
pub struct Barrier {
    /// [`Barrier`] state.
    state: AtomicPtr<()>,
}

impl Barrier {
    /// Maximum number of tasks to block.
    pub const MAX_TASKS: usize = WaitQueue::DATA_MASK;

    /// Creates a new [`Barrier`] that can block the given number of tasks.
    ///
    /// The maximum number of tasks to block is defined by [`MAX_TASKS`](Self::MAX_TASKS), and if a
    /// value greater than or equal to [`MAX_TASKS`](Self::MAX_TASKS) is provided, it will be set to
    /// [`MAX_TASKS`](Self::MAX_TASKS).
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Barrier;
    ///
    /// let barrier = Barrier::with_count(1);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_count(count: usize) -> Self {
        let adjusted_count = Self::MAX_TASKS.min(count);
        Self {
            state: AtomicPtr::new(without_provenance_mut(adjusted_count)),
        }
    }

    /// Returns the current count of tasks to block.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// use saa::Barrier;
    ///
    /// let barrier = Barrier::with_count(1);
    ///
    /// assert_eq!(barrier.count(Relaxed), 1);
    /// ```
    #[inline]
    pub fn count(&self, mo: Ordering) -> usize {
        self.state.load(mo).addr() & WaitQueue::DATA_MASK
    }

    /// Waits until a sufficient number of tasks have reached the barrier.
    ///
    /// Returns `true` if the task was the last one to reach the barrier.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Barrier;
    ///
    /// let barrier = Barrier::with_count(1);
    ///
    /// async {
    ///     assert!(barrier.wait_async().await);
    /// };
    /// ```
    #[inline]
    pub async fn wait_async(&self) -> bool {
        self.wait_async_with(|| {}).await
    }

    /// Waits until a sufficient number of tasks have reached the barrier.
    ///
    /// Returns `true` if the task was the last one to reach the barrier. The callback is invoked
    /// when the task starts waiting.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Barrier;
    ///
    /// let barrier = Barrier::with_count(1);
    ///
    /// async {
    ///     let mut wait = false;
    ///     assert!(barrier.wait_async_with(|| { wait = true; }).await);
    ///     assert!(!wait);
    /// };
    /// ```
    #[inline]
    pub async fn wait_async_with<F: FnOnce()>(&self, mut begin_wait: F) -> bool {
        let mut pinned_pager = pin!(Pager::default());
        loop {
            pinned_pager
                .wait_queue()
                .construct(self, Opcode::Barrier(false), false);
            if let Some(returned) = self.count_down(&mut pinned_pager, false, begin_wait) {
                begin_wait = returned;
                let result = pinned_pager.poll_async().await.unwrap_or(false);
                debug_assert!(!result);
            } else {
                return pinned_pager.poll_async().await.unwrap_or(false);
            }
        }
    }

    /// Waits until a sufficient number of tasks have reached the barrier.
    ///
    /// Returns `true` if the task was the last one to reach the barrier.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Barrier;
    ///
    /// let barrier = Barrier::with_count(1);
    ///
    /// assert!(barrier.wait_sync());
    /// ```
    #[inline]
    pub fn wait_sync(&self) -> bool {
        self.wait_sync_with(|| ())
    }

    /// Waits until a sufficient number of tasks have reached the barrier.
    ///
    /// Returns `true` if the task was the last one to reach the barrier. The callback is invoked
    /// when the task starts waiting.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Barrier;
    ///
    /// let barrier = Barrier::with_count(1);
    ///
    /// let mut wait = false;
    /// assert!(barrier.wait_sync_with(|| { wait = true; }));
    /// assert!(!wait);
    /// ```
    #[inline]
    pub fn wait_sync_with<F: FnOnce()>(&self, mut begin_wait: F) -> bool {
        let mut pinned_pager = pin!(Pager::default());
        loop {
            pinned_pager
                .wait_queue()
                .construct(self, Opcode::Barrier(false), true);
            if let Some(returned) = self.count_down(&mut pinned_pager, true, begin_wait) {
                begin_wait = returned;
                let result = pinned_pager.poll_sync().unwrap_or(false);
                debug_assert!(!result);
            } else {
                return pinned_pager.poll_sync().unwrap_or(false);
            }
        }
    }

    /// Counts down the barrier counter.
    ///
    /// Returns the wait callback if it needs to be retried.
    #[inline]
    fn count_down<F: FnOnce()>(
        &self,
        pager: &mut Pin<&mut Pager<Self>>,
        is_sync: bool,
        begin_wait: F,
    ) -> Option<F> {
        let mut state = self.state.load(Acquire);
        let wait_queue = pager.wait_queue();
        loop {
            let mut count = state.addr() & WaitQueue::DATA_MASK;
            if count == 0 {
                // The counter cannot be decremented, therefore wait for the counter to be reset.
                wait_queue.construct(self, Opcode::Barrier(true), is_sync);
                if self.try_push_wait_queue_entry(pager.wait_queue(), state) {
                    return Some(begin_wait);
                }
                state = self.state.load(Acquire);
            } else if count == 1 {
                // This is the last task to reach the barrier, therefore we can reset the counter.
                match self
                    .state
                    .compare_exchange(state, null_mut(), Acquire, Acquire)
                {
                    Ok(value) => {
                        let mut anchor_ptr = WaitQueue::to_anchor_ptr(value);
                        if !anchor_ptr.is_null() {
                            let tail_entry_ptr = WaitQueue::to_entry_ptr(anchor_ptr);
                            Entry::iter_forward(tail_entry_ptr, false, |entry_ptr, _| {
                                count += 1;
                                // `0` means that all the tasks have reached the barrier, but it is
                                // not the last one.
                                Entry::set_result(entry_ptr, 0);
                                false
                            });
                        }
                        debug_assert!(count <= Self::MAX_TASKS);

                        // Wake-up waiting tasks.
                        anchor_ptr = WaitQueue::to_anchor_ptr(
                            self.state.swap(without_provenance_mut(count), AcqRel),
                        );
                        if !anchor_ptr.is_null() {
                            let tail_entry_ptr = WaitQueue::to_entry_ptr(anchor_ptr);
                            Entry::iter_forward(tail_entry_ptr, false, |entry_ptr, _| {
                                // `2` means that the waiting task needs to retry.
                                Entry::set_result(entry_ptr, 2);
                                false
                            });
                        }

                        // `1` means that the task is the last one to count down the barrier.
                        Entry::set_result(wait_queue.entry_ptr(), 1);
                        return None;
                    }
                    Err(new_state) => state = new_state,
                }
            } else {
                let anchor_ptr = wait_queue.anchor_ptr().0;
                debug_assert_eq!(anchor_ptr.addr() & (!WaitQueue::ADDR_MASK), 0);

                Entry::entry_ref(wait_queue.entry_ptr())
                    .update_next_entry_anchor_ptr(WaitQueue::to_anchor_ptr(state));

                // Count down here.
                let next_state = anchor_ptr
                    .map_addr(|addr| addr | ((state.addr() - 1) & (!WaitQueue::ADDR_MASK)))
                    .cast::<()>()
                    .cast_mut();
                match self
                    .state
                    .compare_exchange(state, next_state, AcqRel, Acquire)
                {
                    Ok(_) => {
                        // The entry cannot be dropped until the result is acknowledged.
                        Entry::entry_ref(wait_queue.entry_ptr()).set_pollable();
                        begin_wait();
                        return None;
                    }
                    Err(new_state) => state = new_state,
                }
            }
        }
    }
}

impl fmt::Debug for Barrier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.state.load(Relaxed);
        let counter = state.addr() & WaitQueue::DATA_MASK;
        let wait_queue_being_processed =
            state.addr() & WaitQueue::LOCKED_FLAG == WaitQueue::LOCKED_FLAG;
        let wait_queue_tail_addr = state.addr() & WaitQueue::ADDR_MASK;
        f.debug_struct("WaitQueue")
            .field("state", &state)
            .field("counter", &counter)
            .field("wait_queue_being_processed", &wait_queue_being_processed)
            .field("wait_queue_tail_addr", &wait_queue_tail_addr)
            .finish()
    }
}

impl Default for Barrier {
    /// The default number of tasks to block is [`MAX_TASKS`](Self::MAX_TASKS).
    #[inline]
    fn default() -> Self {
        Self {
            state: AtomicPtr::new(without_provenance_mut(Self::MAX_TASKS)),
        }
    }
}

impl SyncPrimitive for Barrier {
    #[inline]
    fn state(&self) -> &AtomicPtr<()> {
        &self.state
    }

    #[inline]
    fn max_shared_owners() -> usize {
        Self::MAX_TASKS
    }

    #[inline]
    fn drop_wait_queue_entry(entry_ptr: *const Entry) {
        Self::force_remove_wait_queue_entry(entry_ptr);
    }
}

impl SyncResult for Barrier {
    type Result = Result<bool, pager::Error>;

    #[inline]
    fn to_result(result: u8, pager_error: Option<pager::Error>) -> Self::Result {
        pager_error.map_or_else(|| Ok(result == 1), Err)
    }
}
