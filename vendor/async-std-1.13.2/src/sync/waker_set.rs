//! A common utility for building synchronization primitives.
//!
//! When an async operation is blocked, it needs to register itself somewhere so that it can be
//! notified later on. The `WakerSet` type helps with keeping track of such async operations and
//! notifying them when they may make progress.

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Waker};

use crossbeam_utils::Backoff;
use slab::Slab;

/// Set when the entry list is locked.
#[allow(clippy::identity_op)]
const LOCKED: usize = 1 << 0;

/// Set when there is at least one entry that has already been notified.
const NOTIFIED: usize = 1 << 1;

/// Set when there is at least one notifiable entry.
const NOTIFIABLE: usize = 1 << 2;

/// Inner representation of `WakerSet`.
struct Inner {
    /// A list of entries in the set.
    ///
    /// Each entry has an optional waker associated with the task that is executing the operation.
    /// If the waker is set to `None`, that means the task has been woken up but hasn't removed
    /// itself from the `WakerSet` yet.
    ///
    /// The key of each entry is its index in the `Slab`.
    entries: Slab<Option<Waker>>,

    /// The number of notifiable entries.
    notifiable: usize,
}

/// A set holding wakers.
pub struct WakerSet {
    /// Holds three bits: `LOCKED`, `NOTIFY_ONE`, and `NOTIFY_ALL`.
    flag: AtomicUsize,

    /// A set holding wakers.
    inner: UnsafeCell<Inner>,
}

impl WakerSet {
    /// Creates a new `WakerSet`.
    #[inline]
    pub fn new() -> WakerSet {
        WakerSet {
            flag: AtomicUsize::new(0),
            inner: UnsafeCell::new(Inner {
                entries: Slab::new(),
                notifiable: 0,
            }),
        }
    }

    /// Inserts a waker for a blocked operation and returns a key associated with it.
    #[cold]
    pub fn insert(&self, cx: &Context<'_>) -> usize {
        let w = cx.waker().clone();
        let mut inner = self.lock();

        let key = inner.entries.insert(Some(w));
        inner.notifiable += 1;
        key
    }

    /// If the waker for this key is still waiting for a notification, then update
    /// the waker for the entry, and return false. If the waker has been notified,
    /// treat the entry as completed and return true.
    #[cfg(feature = "unstable")]
    pub fn remove_if_notified(&self, key: usize, cx: &Context<'_>) -> bool {
        let mut inner = self.lock();

        match &mut inner.entries[key] {
            None => {
                inner.entries.remove(key);
                true
            }
            Some(w) => {
                // We were never woken, so update instead
                if !w.will_wake(cx.waker()) {
                    *w = cx.waker().clone();
                }
                false
            }
        }
    }

    /// Removes the waker of a cancelled operation.
    ///
    /// Returns `true` if another blocked operation from the set was notified.
    #[cold]
    pub fn cancel(&self, key: usize) -> bool {
        let mut inner = self.lock();

        match inner.entries.remove(key) {
            Some(_) => inner.notifiable -= 1,
            None => {
                // The operation was cancelled and notified so notify another operation instead.
                for (_, opt_waker) in inner.entries.iter_mut() {
                    // If there is no waker in this entry, that means it was already woken.
                    if let Some(w) = opt_waker.take() {
                        w.wake();
                        inner.notifiable -= 1;
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Notifies one additional blocked operation.
    ///
    /// Returns `true` if an operation was notified.
    #[inline]
    #[cfg(feature = "unstable")]
    pub fn notify_one(&self) -> bool {
        // Use `SeqCst` ordering to synchronize with `Lock::drop()`.
        if self.flag.load(Ordering::SeqCst) & NOTIFIABLE != 0 {
            self.notify(Notify::One)
        } else {
            false
        }
    }

    /// Notifies all blocked operations.
    ///
    /// Returns `true` if at least one operation was notified.
    #[inline]
    pub fn notify_all(&self) -> bool {
        // Use `SeqCst` ordering to synchronize with `Lock::drop()`.
        if self.flag.load(Ordering::SeqCst) & NOTIFIABLE != 0 {
            self.notify(Notify::All)
        } else {
            false
        }
    }

    /// Notifies blocked operations, either one or all of them.
    ///
    /// Returns `true` if at least one operation was notified.
    #[cold]
    fn notify(&self, n: Notify) -> bool {
        let inner = &mut *self.lock();
        let mut notified = false;

        for (_, opt_waker) in inner.entries.iter_mut() {
            // If there is no waker in this entry, that means it was already woken.
            if let Some(w) = opt_waker.take() {
                w.wake();
                inner.notifiable -= 1;
                notified = true;

                if n == Notify::One {
                    break;
                }
            }

            if n == Notify::Any {
                break;
            }
        }

        notified
    }

    /// Locks the list of entries.
    fn lock(&self) -> Lock<'_> {
        let backoff = Backoff::new();
        while self.flag.fetch_or(LOCKED, Ordering::Acquire) & LOCKED != 0 {
            backoff.snooze();
        }
        Lock { waker_set: self }
    }
}

/// A guard holding a `WakerSet` locked.
struct Lock<'a> {
    waker_set: &'a WakerSet,
}

impl Drop for Lock<'_> {
    #[inline]
    fn drop(&mut self) {
        let mut flag = 0;

        // Set the `NOTIFIED` flag if there is at least one notified entry.
        if self.entries.len() - self.notifiable > 0 {
            flag |= NOTIFIED;
        }

        // Set the `NOTIFIABLE` flag if there is at least one notifiable entry.
        if self.notifiable > 0 {
            flag |= NOTIFIABLE;
        }

        // Use `SeqCst` ordering to synchronize with `WakerSet::lock_to_notify()`.
        self.waker_set.flag.store(flag, Ordering::SeqCst);
    }
}

impl Deref for Lock<'_> {
    type Target = Inner;

    #[inline]
    fn deref(&self) -> &Inner {
        unsafe { &*self.waker_set.inner.get() }
    }
}

impl DerefMut for Lock<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Inner {
        unsafe { &mut *self.waker_set.inner.get() }
    }
}

/// Notification strategy.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Notify {
    /// Make sure at least one entry is notified.
    Any,
    /// Notify one additional entry.
    One,
    /// Notify all entries.
    All,
}
