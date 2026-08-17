use super::cancellation_queue::Sender;
use crate::loom::sync::{Arc, Mutex};
use crate::util::linked_list;

use std::marker::PhantomPinned;
use std::ptr::NonNull;
use std::task::Waker;

pub(super) type EntryList = linked_list::LinkedList<Entry, Entry>;

#[derive(Debug)]
struct State {
    cancelled: bool,
    woken_up: bool,
    waker: Option<Waker>,
    cancel_tx: Option<Sender>,
}

#[derive(Debug)]
pub(crate) struct Entry {
    /// The intrusive pointer used by [`super::cancellation_queue`].
    cancel_pointers: linked_list::Pointers<Entry>,

    /// The intrusive pointer used by any of the following queues:
    ///
    /// - [`Wheel`]
    /// - [`RegistrationQueue`]
    /// - [`WakeQueue`]
    ///
    /// We can guarantee that this pointer is only used by one of the above
    /// at any given time. See below for the journey of this pointer.
    ///
    /// Initially, this pointer is used by the [`RegistrationQueue`].
    ///
    /// And then, before parking the resource driver,
    /// the scheduler removes the entry from the [`RegistrationQueue`]
    /// [`RegistrationQueue`] and insert it into the [`Wheel`].
    ///
    /// Finally, after parking the resource driver, the scheduler removes
    /// the entry from the [`Wheel`] and insert it into the [`WakeQueue`].
    ///
    /// [`RegistrationQueue`]: super::RegistrationQueue
    /// [`Wheel`]: super::Wheel
    /// [`WakeQueue`]: super::WakeQueue
    extra_pointers: linked_list::Pointers<Entry>,

    /// The tick when this entry is scheduled to expire.
    deadline: u64,

    state: Mutex<State>,

    /// Make the type `!Unpin` to prevent LLVM from emitting
    /// the `noalias` attribute for mutable references.
    ///
    /// See <https://github.com/rust-lang/rust/pull/82834>.
    _pin: PhantomPinned,
}

// Safety: `Entry` is always in an `Arc`.
unsafe impl linked_list::Link for Entry {
    type Handle = Handle;
    type Target = Entry;

    fn as_raw(hdl: &Self::Handle) -> NonNull<Self::Target> {
        unsafe { NonNull::new_unchecked(Arc::as_ptr(&hdl.entry).cast_mut()) }
    }

    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle {
        Handle {
            entry: unsafe { Arc::from_raw(ptr.as_ptr()) },
        }
    }

    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<Self::Target>> {
        let this = target.as_ptr();
        let field = unsafe { std::ptr::addr_of_mut!((*this).extra_pointers) };
        unsafe { NonNull::new_unchecked(field) }
    }
}

/// An ZST to allow [`super::registration_queue`] to utilize the [`Entry::extra_pointers`]
/// by impl [`linked_list::Link`] as we cannot impl it on [`Entry`]
/// directly due to the conflicting implementations.
///
/// This type should never be constructed.
pub(super) struct RegistrationQueueEntry;

// Safety: `Entry` is always in an `Arc`.
unsafe impl linked_list::Link for RegistrationQueueEntry {
    type Handle = Handle;
    type Target = Entry;

    fn as_raw(hdl: &Self::Handle) -> NonNull<Self::Target> {
        unsafe { NonNull::new_unchecked(Arc::as_ptr(&hdl.entry).cast_mut()) }
    }

    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle {
        Handle {
            entry: unsafe { Arc::from_raw(ptr.as_ptr()) },
        }
    }

    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<Self::Target>> {
        let this = target.as_ptr();
        let field = unsafe { std::ptr::addr_of_mut!((*this).extra_pointers) };
        unsafe { NonNull::new_unchecked(field) }
    }
}

/// An ZST to allow [`super::cancellation_queue`] to utilize the [`Entry::cancel_pointers`]
/// by impl [`linked_list::Link`] as we cannot impl it on [`Entry`]
/// directly due to the conflicting implementations.
///
/// This type should never be constructed.
pub(super) struct CancellationQueueEntry;

// Safety: `Entry` is always in an `Arc`.
unsafe impl linked_list::Link for CancellationQueueEntry {
    type Handle = Handle;
    type Target = Entry;

    fn as_raw(hdl: &Self::Handle) -> NonNull<Self::Target> {
        unsafe { NonNull::new_unchecked(Arc::as_ptr(&hdl.entry).cast_mut()) }
    }

    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle {
        Handle {
            entry: unsafe { Arc::from_raw(ptr.as_ptr()) },
        }
    }

    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<Self::Target>> {
        let this = target.as_ptr();
        let field = unsafe { std::ptr::addr_of_mut!((*this).cancel_pointers) };
        unsafe { NonNull::new_unchecked(field) }
    }
}

/// An ZST to allow [`super::WakeQueue`] to utilize the [`Entry::extra_pointers`]
/// by impl [`linked_list::Link`] as we cannot impl it on [`Entry`]
/// directly due to the conflicting implementations.
///
/// This type should never be constructed.
pub(super) struct WakeQueueEntry;

// Safety: `Entry` is always in an `Arc`.
unsafe impl linked_list::Link for WakeQueueEntry {
    type Handle = Handle;
    type Target = Entry;

    fn as_raw(hdl: &Self::Handle) -> NonNull<Self::Target> {
        unsafe { NonNull::new_unchecked(Arc::as_ptr(&hdl.entry).cast_mut()) }
    }

    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle {
        Handle {
            entry: unsafe { Arc::from_raw(ptr.as_ptr()) },
        }
    }

    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<Self::Target>> {
        let this = target.as_ptr();
        let field = unsafe { std::ptr::addr_of_mut!((*this).extra_pointers) };
        unsafe { NonNull::new_unchecked(field) }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Handle {
    pub(crate) entry: Arc<Entry>,
}

impl From<&Handle> for NonNull<Entry> {
    fn from(hdl: &Handle) -> Self {
        // Safety: entry is in an `Arc`, so the pointer is valid.
        unsafe { NonNull::new_unchecked(Arc::as_ptr(&hdl.entry) as *mut Entry) }
    }
}

impl Handle {
    pub(crate) fn new(deadline: u64, waker: Waker) -> Self {
        let state = State {
            cancelled: false,
            woken_up: false,
            waker: Some(waker),
            cancel_tx: None,
        };

        let entry = Arc::new(Entry {
            cancel_pointers: linked_list::Pointers::new(),
            extra_pointers: linked_list::Pointers::new(),
            deadline,
            state: Mutex::new(state),
            _pin: PhantomPinned,
        });

        Handle { entry }
    }

    /// Wake the entry if it is already in the pending queue of the timer wheel.
    pub(crate) fn wake(&self) {
        let mut lock = self.entry.state.lock();

        if !lock.cancelled {
            lock.woken_up = true;
            if let Some(waker) = lock.waker.take() {
                // unlock before calling waker
                drop(lock);
                waker.wake();
            }
        }
    }

    pub(crate) fn register_cancel_tx(&self, cancel_tx: Sender) {
        let mut lock = self.entry.state.lock();
        if !lock.cancelled && !lock.woken_up {
            let old_tx = lock.cancel_tx.replace(cancel_tx);
            // don't unlock — poisoning the `Mutex` stops others from using the bad state.
            assert!(old_tx.is_none(), "cancel_tx is already registered");
        }
    }

    pub(crate) fn register_waker(&self, waker: Waker) {
        let mut lock = self.entry.state.lock();
        if !lock.cancelled && !lock.woken_up {
            let maybe_old_waker = lock.waker.replace(waker);
            // unlock before calling waker
            drop(lock);
            drop(maybe_old_waker);
        }
    }

    pub(crate) fn cancel(&self) {
        let mut lock = self.entry.state.lock();
        if !lock.cancelled {
            lock.cancelled = true;
            if let Some(cancel_tx) = lock.cancel_tx.take() {
                drop(lock);

                // Safety: we can guarantee that `self` is not in any cancellation queue
                // because the `self.cancelled` was just set to `true`.
                unsafe {
                    cancel_tx.send(self.clone());
                }
            }
        }
    }

    pub(crate) fn deadline(&self) -> u64 {
        self.entry.deadline
    }

    pub(crate) fn is_woken_up(&self) -> bool {
        let lock = self.entry.state.lock();
        lock.woken_up
    }

    pub(crate) fn is_cancelled(&self) -> bool {
        let lock = self.entry.state.lock();
        lock.cancelled
    }

    #[cfg(test)]
    /// Only used for unit tests.
    pub(crate) fn inner_strong_count(&self) -> usize {
        Arc::strong_count(&self.entry)
    }
}
