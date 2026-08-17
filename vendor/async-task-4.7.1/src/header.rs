use core::cell::UnsafeCell;
use core::fmt;
use core::task::{RawWaker, Waker};

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
#[cfg(feature = "portable-atomic")]
use portable_atomic::AtomicUsize;

use crate::raw::TaskVTable;
use crate::state::*;
use crate::utils::abort;
use crate::utils::abort_on_panic;

/// Actions to take upon calling [`Header::drop_waker`].
pub(crate) enum DropWakerAction {
    /// Re-schedule the task.
    Schedule,
    /// Destroy the task.
    Destroy,
    /// Do nothing.
    None,
}

pub(crate) struct Header {
    /// Current state of the task.
    ///
    /// Contains flags representing the current state and the reference count.
    pub(crate) state: AtomicUsize,

    /// The task that is blocked on the `Task` handle.
    ///
    /// This waker needs to be woken up once the task completes or is closed.
    pub(crate) awaiter: UnsafeCell<Option<Waker>>,

    /// The virtual table.
    ///
    /// In addition to the actual waker virtual table, it also contains pointers to several other
    /// methods necessary for bookkeeping the heap-allocated task.
    pub(crate) vtable: &'static TaskVTable,

    /// Whether or not a panic that occurs in the task should be propagated.
    #[cfg(feature = "std")]
    pub(crate) propagate_panic: bool,
}

impl Header {
    /// Notifies the awaiter blocked on this task.
    ///
    /// If the awaiter is the same as the current waker, it will not be notified.
    #[inline]
    pub(crate) fn notify(&self, current: Option<&Waker>) {
        if let Some(w) = self.take(current) {
            abort_on_panic(|| w.wake());
        }
    }

    /// Takes the awaiter blocked on this task.
    ///
    /// If there is no awaiter or if it is the same as the current waker, returns `None`.
    #[inline]
    pub(crate) fn take(&self, current: Option<&Waker>) -> Option<Waker> {
        // Set the bit indicating that the task is notifying its awaiter.
        let state = self.state.fetch_or(NOTIFYING, Ordering::AcqRel);

        // If the task was not notifying or registering an awaiter...
        if state & (NOTIFYING | REGISTERING) == 0 {
            // Take the waker out.
            let waker = unsafe { (*self.awaiter.get()).take() };

            // Unset the bit indicating that the task is notifying its awaiter.
            self.state
                .fetch_and(!NOTIFYING & !AWAITER, Ordering::Release);

            // Finally, notify the waker if it's different from the current waker.
            if let Some(w) = waker {
                match current {
                    None => return Some(w),
                    Some(c) if !w.will_wake(c) => return Some(w),
                    Some(_) => abort_on_panic(|| drop(w)),
                }
            }
        }

        None
    }

    /// Registers a new awaiter blocked on this task.
    ///
    /// This method is called when `Task` is polled and it has not yet completed.
    #[inline]
    pub(crate) fn register(&self, waker: &Waker) {
        // Load the state and synchronize with it.
        let mut state = self.state.fetch_or(0, Ordering::Acquire);

        loop {
            // There can't be two concurrent registrations because `Task` can only be polled
            // by a unique pinned reference.
            debug_assert!(state & REGISTERING == 0);

            // If we're in the notifying state at this moment, just wake and return without
            // registering.
            if state & NOTIFYING != 0 {
                abort_on_panic(|| waker.wake_by_ref());
                return;
            }

            // Mark the state to let other threads know we're registering a new awaiter.
            match self.state.compare_exchange_weak(
                state,
                state | REGISTERING,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    state |= REGISTERING;
                    break;
                }
                Err(s) => state = s,
            }
        }

        // Put the waker into the awaiter field.
        unsafe {
            abort_on_panic(|| (*self.awaiter.get()) = Some(waker.clone()));
        }

        // This variable will contain the newly registered waker if a notification comes in before
        // we complete registration.
        let mut waker = None;

        loop {
            // If there was a notification, take the waker out of the awaiter field.
            if state & NOTIFYING != 0 {
                if let Some(w) = unsafe { (*self.awaiter.get()).take() } {
                    abort_on_panic(|| waker = Some(w));
                }
            }

            // The new state is not being notified nor registered, but there might or might not be
            // an awaiter depending on whether there was a concurrent notification.
            let new = if waker.is_none() {
                (state & !NOTIFYING & !REGISTERING) | AWAITER
            } else {
                state & !NOTIFYING & !REGISTERING & !AWAITER
            };

            match self
                .state
                .compare_exchange_weak(state, new, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => break,
                Err(s) => state = s,
            }
        }

        // If there was a notification during registration, wake the awaiter now.
        if let Some(w) = waker {
            abort_on_panic(|| w.wake());
        }
    }

    /// Clones a waker.
    pub(crate) unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
        let header = ptr as *const Header;

        // Increment the reference count. With any kind of reference-counted data structure,
        // relaxed ordering is appropriate when incrementing the counter.
        let state = (*header).state.fetch_add(REFERENCE, Ordering::Relaxed);

        // If the reference count overflowed, abort.
        if state > isize::MAX as usize {
            abort();
        }

        RawWaker::new(ptr, (*header).vtable.raw_waker_vtable)
    }

    #[inline(never)]
    pub(crate) unsafe fn drop_waker(ptr: *const ()) -> DropWakerAction {
        let header = ptr as *const Header;

        // Decrement the reference count.
        let new = (*header).state.fetch_sub(REFERENCE, Ordering::AcqRel) - REFERENCE;

        // If this was the last reference to the task and the `Task` has been dropped too,
        // then we need to decide how to destroy the task.
        if new & !(REFERENCE - 1) == 0 && new & TASK == 0 {
            if new & (COMPLETED | CLOSED) == 0 {
                // If the task was not completed nor closed, close it and schedule one more time so
                // that its future gets dropped by the executor.
                (*header)
                    .state
                    .store(SCHEDULED | CLOSED | REFERENCE, Ordering::Release);
                DropWakerAction::Schedule
            } else {
                // Otherwise, destroy the task right away.
                DropWakerAction::Destroy
            }
        } else {
            DropWakerAction::None
        }
    }
}

// SAFETY: repr(C) is explicitly used here so that casts between `Header` and `HeaderWithMetadata`
// can be done safely without additional offsets.
//
/// The header of a task.
///
/// This header is stored in memory at the beginning of the heap-allocated task.
#[repr(C)]
pub(crate) struct HeaderWithMetadata<M> {
    pub(crate) header: Header,

    /// Metadata associated with the task.
    ///
    /// This metadata may be provided to the user.
    pub(crate) metadata: M,
}

impl<M: fmt::Debug> fmt::Debug for HeaderWithMetadata<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.header.state.load(Ordering::SeqCst);

        f.debug_struct("Header")
            .field("scheduled", &(state & SCHEDULED != 0))
            .field("running", &(state & RUNNING != 0))
            .field("completed", &(state & COMPLETED != 0))
            .field("closed", &(state & CLOSED != 0))
            .field("awaiter", &(state & AWAITER != 0))
            .field("task", &(state & TASK != 0))
            .field("ref_count", &(state / REFERENCE))
            .field("metadata", &self.metadata)
            .finish()
    }
}
