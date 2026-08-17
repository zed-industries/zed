//! Define base operations for synchronization primitives.

#[cfg(not(feature = "loom"))]
use std::hint::spin_loop;
use std::pin::{Pin, pin};
use std::ptr::{null, without_provenance_mut};
#[cfg(not(feature = "loom"))]
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};

#[cfg(feature = "loom")]
use loom::hint::spin_loop;
#[cfg(feature = "loom")]
use loom::sync::atomic::AtomicPtr;

use crate::opcode::Opcode;
use crate::wait_queue::{Entry, WaitQueue};

/// Defines base operations for synchronization primitives.
pub(crate) trait SyncPrimitive: Sized {
    /// Returns a reference to the state.
    fn state(&self) -> &AtomicPtr<()>;

    /// Returns the maximum number of shared owners.
    fn max_shared_owners() -> usize;

    /// Called when an enqueued wait queue entry is being dropped without acknowledging the result.
    fn drop_wait_queue_entry(entry_ptr: *const Entry);

    /// Tries to push a wait queue entry into the wait queue.
    #[must_use]
    fn try_push_wait_queue_entry(&self, wait_queue: Pin<&WaitQueue>, state: *mut ()) -> bool {
        let anchor_ptr = wait_queue.anchor_ptr().0;
        debug_assert_eq!(anchor_ptr.addr() & (!WaitQueue::ADDR_MASK), 0);

        let tail_anchor_ptr = WaitQueue::to_anchor_ptr(state);
        Entry::entry_ref(wait_queue.entry_ptr()).update_next_entry_anchor_ptr(tail_anchor_ptr);

        // The anchor pointer, instead of an entry pointer, is stored in the state.
        let next_state = anchor_ptr
            .map_addr(|addr| addr | (state.addr() & (!WaitQueue::ADDR_MASK)))
            .cast::<()>()
            .cast_mut();
        if self
            .state()
            .compare_exchange(state, next_state, AcqRel, Acquire)
            .is_ok()
        {
            // The entry cannot be dropped until the result is acknowledged.
            Entry::entry_ref(wait_queue.entry_ptr()).set_pollable();
            true
        } else {
            false
        }
    }

    /// Waits for the desired resource synchronously.
    fn wait_resources_sync<F: FnOnce()>(
        &self,
        state: *mut (),
        opcode: Opcode,
        begin_wait: F,
    ) -> Result<u8, F> {
        debug_assert!(
            state.addr() & WaitQueue::ADDR_MASK != 0 || state.addr() & WaitQueue::DATA_MASK != 0
        );

        let pinned_wait_queue = pin!(WaitQueue::default());
        pinned_wait_queue.as_ref().construct(self, opcode, true);
        if self.try_push_wait_queue_entry(pinned_wait_queue.as_ref(), state) {
            begin_wait();
            Ok(Entry::entry_ref(pinned_wait_queue.entry_ptr()).poll_result_sync())
        } else {
            Err(begin_wait)
        }
    }

    /// Releases the resource represented by the supplied operation mode.
    ///
    /// Returns `false` if the resource cannot be released.
    fn release_loop(&self, mut state: *mut (), opcode: Opcode) -> bool {
        while opcode.can_release(state.addr()) {
            if state.addr() & WaitQueue::ADDR_MASK == 0
                || state.addr() & WaitQueue::LOCKED_FLAG == WaitQueue::LOCKED_FLAG
            {
                // Release the resource in-place.
                match self.state().compare_exchange(
                    state,
                    state.map_addr(|addr| addr - opcode.acquired_count()),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => return true,
                    Err(new_state) => state = new_state,
                }
            } else {
                // The wait queue is not empty and is not being processed.
                let next_state = state
                    .map_addr(|addr| (addr | WaitQueue::LOCKED_FLAG) - opcode.acquired_count());
                if let Err(new_state) = self
                    .state()
                    .compare_exchange(state, next_state, AcqRel, Relaxed)
                {
                    state = new_state;
                    continue;
                }
                self.process_wait_queue(next_state);
                return true;
            }
        }
        false
    }

    /// Processes the wait queue.
    ///
    /// The tail entry of the wait queue is either reset or stays the same.
    fn process_wait_queue(&self, mut state: *mut ()) {
        let mut head_entry_ptr: *const Entry = null();
        let mut unlocked = false;
        while !unlocked {
            debug_assert_eq!(
                state.addr() & WaitQueue::LOCKED_FLAG,
                WaitQueue::LOCKED_FLAG
            );

            let anchor_ptr = WaitQueue::to_anchor_ptr(state);
            let tail_entry_ptr = WaitQueue::to_entry_ptr(anchor_ptr);
            if head_entry_ptr.is_null() {
                Entry::iter_forward(tail_entry_ptr, true, |entry_ptr, next_entry| {
                    head_entry_ptr = entry_ptr;
                    next_entry.is_null()
                });
            } else {
                Entry::set_prev_ptr(tail_entry_ptr);
            }

            let data = state.addr() & WaitQueue::DATA_MASK;
            let mut transferred = 0;
            let mut resolved_entry_ptr: *const Entry = null();
            let mut reset_failed = false;

            Entry::iter_backward(head_entry_ptr, |entry_ptr, prev_entry| {
                let opcode = Entry::entry_ref(entry_ptr).opcode();
                let desired = opcode.desired_count();
                if data + transferred == 0
                    || data + transferred + desired <= Self::max_shared_owners()
                {
                    // The entry can inherit ownership.
                    let acquired = opcode.acquired_count();
                    debug_assert!(acquired <= desired);
                    if prev_entry.is_null() {
                        // This is the tail of the wait queue: try to reset.
                        debug_assert_eq!(tail_entry_ptr, entry_ptr);
                        if self
                            .state()
                            .compare_exchange(
                                state,
                                without_provenance_mut(data + transferred + acquired),
                                AcqRel,
                                Acquire,
                            )
                            .is_err()
                        {
                            // This entry will be processed on the next retry.
                            Entry::entry_ref(entry_ptr).update_next_entry_anchor_ptr(null());
                            head_entry_ptr = entry_ptr;
                            reset_failed = true;
                            return true;
                        }

                        // The wait queue was reset.
                        unlocked = true;
                        resolved_entry_ptr = entry_ptr;
                        true
                    } else {
                        transferred += acquired;
                        resolved_entry_ptr = entry_ptr;
                        false
                    }
                } else {
                    // Unlink those that have succeeded in acquiring shared ownership.
                    Entry::entry_ref(entry_ptr).update_next_entry_anchor_ptr(null());
                    head_entry_ptr = entry_ptr;
                    true
                }
            });
            debug_assert!(!reset_failed || !unlocked);

            if !reset_failed && !unlocked {
                unlocked = self
                    .state()
                    .fetch_update(AcqRel, Acquire, |new_state| {
                        let new_data = new_state.addr() & WaitQueue::DATA_MASK;
                        debug_assert!(new_data <= data);
                        debug_assert!(new_data + transferred <= WaitQueue::DATA_MASK);

                        if new_data == data {
                            Some(new_state.map_addr(|addr| {
                                (addr & WaitQueue::ADDR_MASK) | (new_data + transferred)
                            }))
                        } else {
                            None
                        }
                    })
                    .is_ok();
            }

            if !unlocked {
                state = self
                    .state()
                    .fetch_update(AcqRel, Acquire, |new_state| {
                        Some(new_state.map_addr(|addr| addr + transferred))
                    })
                    .unwrap()
                    .map_addr(|addr| addr + transferred);
            }

            Entry::iter_forward(resolved_entry_ptr, false, |entry_ptr, _next_entry| {
                Entry::set_result(entry_ptr, 0);
                false
            });
        }
    }

    /// Removes a wait queue entry from the wait queue.
    fn remove_wait_queue_entry(
        &self,
        mut state: *mut (),
        entry_ptr_to_remove: *const Entry,
    ) -> (*mut (), bool) {
        let mut result = Ok((state, false));

        loop {
            debug_assert_eq!(
                state.addr() & WaitQueue::LOCKED_FLAG,
                WaitQueue::LOCKED_FLAG
            );
            debug_assert_ne!(state.addr() & WaitQueue::ADDR_MASK, 0);

            let anchor_ptr = WaitQueue::to_anchor_ptr(state);
            let tail_entry_ptr = WaitQueue::to_entry_ptr(anchor_ptr);
            Entry::iter_forward(tail_entry_ptr, true, |entry_ptr, next_entry_ptr| {
                if entry_ptr == entry_ptr_to_remove {
                    // Found the entry to remove.
                    let prev_entry_ptr = Entry::entry_ref(entry_ptr).prev_entry_ptr();
                    if !next_entry_ptr.is_null() {
                        Entry::entry_ref(next_entry_ptr).update_prev_entry_ptr(prev_entry_ptr);
                    }
                    result = if !prev_entry_ptr.is_null() {
                        // Successfully unlinked the target entry without updating the state.
                        Entry::entry_ref(prev_entry_ptr).update_next_entry_anchor_ptr(
                            Entry::entry_ref(entry_ptr).next_entry_anchor_ptr(),
                        );
                        Ok((state, true))
                    } else if !next_entry_ptr.is_null() {
                        // The next entry becomes the new tail of the wait queue.
                        let new_tail_ptr = Entry::to_wait_queue_ptr(next_entry_ptr);
                        let new_anchor_ptr = unsafe { (*new_tail_ptr).anchor_ptr().0 };
                        debug_assert_eq!(new_anchor_ptr.addr() & (!WaitQueue::ADDR_MASK), 0);

                        let next_state = new_anchor_ptr
                            .map_addr(|addr| addr | (state.addr() & (!WaitQueue::ADDR_MASK)))
                            .cast::<()>()
                            .cast_mut();
                        debug_assert_eq!(
                            next_state.addr() & WaitQueue::LOCKED_FLAG,
                            WaitQueue::LOCKED_FLAG
                        );

                        self.state()
                            .compare_exchange(state, next_state, AcqRel, Acquire)
                            .map(|_| (next_state, true))
                    } else {
                        // Reset the wait queue and unlock.
                        let next_state = state.map_addr(|addr| addr & WaitQueue::DATA_MASK);
                        self.state()
                            .compare_exchange(state, next_state, AcqRel, Acquire)
                            .map(|_| (next_state, true))
                    };
                    true
                } else {
                    false
                }
            });

            match result {
                Ok((state, removed)) => return (state, removed),
                Err(new_state) => state = new_state,
            }
        }
    }

    /// Removes a [`WaitQueue`] entry that was pushed into the wait queue but has not been
    /// processed.
    fn force_remove_wait_queue_entry(entry_ptr: *const Entry) {
        let this_ref: &Self = Entry::entry_ref(entry_ptr).sync_primitive_ref();

        // Remove the wait queue entry from the wait queue list.
        let mut state = this_ref.state().load(Acquire);
        let mut need_completion = false;
        loop {
            if state.addr() & WaitQueue::LOCKED_FLAG == WaitQueue::LOCKED_FLAG {
                // Another thread is processing the wait queue.
                spin_loop();
                state = this_ref.state().load(Acquire);
            } else if state.addr() & WaitQueue::ADDR_MASK == 0 {
                // The wait queue is empty.
                need_completion = true;
                break;
            } else if let Err(new_state) = this_ref.state().compare_exchange(
                state,
                state.map_addr(|addr| addr | WaitQueue::LOCKED_FLAG),
                AcqRel,
                Acquire,
            ) {
                state = new_state;
            } else {
                let (new_state, removed) = this_ref.remove_wait_queue_entry(
                    state.map_addr(|addr| addr | WaitQueue::LOCKED_FLAG),
                    entry_ptr,
                );
                if new_state.addr() & WaitQueue::LOCKED_FLAG == WaitQueue::LOCKED_FLAG {
                    // We need to process the wait queue if it is still locked.
                    this_ref.process_wait_queue(new_state);
                }
                if !removed {
                    need_completion = true;
                }
                break;
            }
        }

        if need_completion {
            // The entry was removed by another thread, so it will be completed.
            let opcode = Entry::entry_ref(entry_ptr).opcode();
            while !Entry::entry_ref(entry_ptr).result_finalized() {
                spin_loop();
            }
            this_ref.release_loop(state, opcode);
        }
    }
}
