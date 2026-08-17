//! [`Semaphore`] is a synchronization primitive that allows a fixed number of threads to access a
//! resource concurrently.

#![deny(unsafe_code)]

use std::fmt;
use std::pin::Pin;
use std::ptr::{null_mut, without_provenance_mut};
#[cfg(not(feature = "loom"))]
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{self, Acquire, Relaxed, Release};

#[cfg(feature = "loom")]
use loom::sync::atomic::AtomicPtr;

use crate::Pager;
use crate::opcode::Opcode;
use crate::pager::{self, SyncResult};
use crate::sync_primitive::SyncPrimitive;
use crate::wait_queue::{Entry, WaitQueue};

/// [`Semaphore`] is a synchronization primitive that allows a fixed number of threads to access a
/// resource concurrently.
#[derive(Default)]
pub struct Semaphore {
    /// [`Semaphore`] state.
    state: AtomicPtr<()>,
}

impl Semaphore {
    /// Maximum number of concurrent owners.
    pub const MAX_PERMITS: usize = WaitQueue::DATA_MASK;

    /// Creates a new [`Semaphore`].
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    ///
    /// let semaphore = Semaphore::new();
    /// ```
    #[cfg(not(feature = "loom"))]
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: AtomicPtr::new(null_mut()),
        }
    }

    /// Creates a new [`Semaphore`].
    #[cfg(feature = "loom")]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: AtomicPtr::new(null_mut()),
        }
    }

    /// Creates a new [`Semaphore`] with the given number of initially available permits.
    ///
    /// The maximum number of available permits is [`MAX_PERMITS`](Self::MAX_PERMITS), and if a
    /// value greater than or equal to [`MAX_PERMITS`](Self::MAX_PERMITS) is provided, it will be
    /// set to [`MAX_PERMITS`](Self::MAX_PERMITS).
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::with_permits(11);
    ///
    /// assert_eq!(semaphore.available_permits(Relaxed), 11);
    ///
    /// assert!(semaphore.try_acquire_many(11));
    /// assert!(!semaphore.is_open(Relaxed));
    /// ```
    #[inline]
    #[must_use]
    pub fn with_permits(permits: usize) -> Self {
        let adjusted_permits = permits.min(Self::MAX_PERMITS);
        Self {
            state: AtomicPtr::new(without_provenance_mut(Self::MAX_PERMITS - adjusted_permits)),
        }
    }

    /// Returns `true` if the semaphore is currently open.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    /// assert!(semaphore.is_open(Relaxed));
    ///
    /// assert!(semaphore.try_acquire_many(Semaphore::MAX_PERMITS));
    /// assert!(!semaphore.is_open(Relaxed));
    /// ```
    #[inline]
    pub fn is_open(&self, mo: Ordering) -> bool {
        let state = self.state.load(mo);
        (state.addr() & WaitQueue::DATA_MASK) != Self::MAX_PERMITS
    }

    /// Returns `true` if the semaphore is currently closed.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    /// assert!(!semaphore.is_closed(Relaxed));
    /// assert!(semaphore.is_open(Relaxed));
    ///
    /// assert!(semaphore.try_acquire());
    /// assert!(!semaphore.is_closed(Relaxed));
    /// assert!(semaphore.is_open(Relaxed));
    ///
    /// semaphore.try_acquire_many(Semaphore::MAX_PERMITS - 1);
    /// assert!(semaphore.is_closed(Relaxed));
    /// ```
    #[inline]
    pub fn is_closed(&self, mo: Ordering) -> bool {
        (self.state.load(mo).addr() & WaitQueue::DATA_MASK) == WaitQueue::DATA_MASK
    }

    /// Returns the number of available permits.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS);
    ///
    /// assert!(semaphore.try_acquire());
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 1);
    /// ```
    #[inline]
    pub fn available_permits(&self, mo: Ordering) -> usize {
        Self::MAX_PERMITS - (self.state.load(mo).addr() & WaitQueue::DATA_MASK)
    }

    /// Gets a permit from the semaphore asynchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// async {
    ///     semaphore.acquire_async().await;
    ///     assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 1);
    /// };
    /// ```
    #[inline]
    pub async fn acquire_async(&self) {
        self.acquire_many_async_with(1, || {}).await;
    }

    /// Gets a permit from the semaphore synchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// semaphore.acquire_sync();
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 1);
    /// ```
    #[inline]
    pub fn acquire_sync(&self) {
        self.acquire_many_sync_with(1, || ());
    }

    /// Gets a permit from the semaphore asynchronously with a wait callback.
    ///
    /// The callback is invoked when the task starts waiting for a permit.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// async {
    ///     let mut wait = false;
    ///     semaphore.acquire_async_with(|| { wait = true; }).await;
    ///     assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 1);
    ///     assert!(!wait);
    /// };
    /// ```
    #[inline]
    pub async fn acquire_async_with<F: FnOnce()>(&self, begin_wait: F) {
        self.acquire_many_async_with(1, begin_wait).await;
    }

    /// Gets multiple permits from the semaphore synchronously with a wait callback.
    ///
    /// The callback is invoked when the task starts waiting for permits.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// let mut wait = false;
    /// semaphore.acquire_sync_with(|| { wait = true; });
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 1);
    /// assert!(!wait);
    /// ```
    #[inline]
    pub fn acquire_sync_with<F: FnOnce()>(&self, begin_wait: F) {
        self.acquire_many_sync_with(1, begin_wait);
    }

    /// Tries to get a permit from the semaphore.
    ///
    /// Returns `false` if no permits are available.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// assert!(semaphore.try_acquire());
    /// assert!(!semaphore.try_acquire_many(Semaphore::MAX_PERMITS));
    /// ```
    #[inline]
    pub fn try_acquire(&self) -> bool {
        self.try_acquire_internal(1).0
    }

    /// Gets multiple permits from the semaphore asynchronously.
    ///
    /// Returns `false` if the count exceeds [`Self::MAX_PERMITS`].
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// async {
    ///     assert!(semaphore.acquire_many_async(11).await);
    ///     assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 11);
    /// };
    /// ```
    #[inline]
    pub async fn acquire_many_async(&self, count: usize) -> bool {
        self.acquire_many_async_with(count, || {}).await
    }

    /// Gets multiple permits from the semaphore synchronously.
    ///
    /// Returns `false` if the count exceeds [`Self::MAX_PERMITS`].
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// assert!(semaphore.acquire_many_sync(11));
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 11);
    /// ```
    #[inline]
    pub fn acquire_many_sync(&self, count: usize) -> bool {
        self.acquire_many_sync_with(count, || ())
    }

    /// Gets multiple permits from the semaphore asynchronously with a wait callback.
    ///
    /// Returns `false` if the count exceeds [`Self::MAX_PERMITS`]. The callback is invoked when the
    /// task starts waiting for permits.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// async {
    ///     let mut wait = false;
    ///     assert!(semaphore.acquire_many_async_with(2, || { wait = true; }).await);
    ///     assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 2);
    ///     assert!(!wait);
    /// };
    /// ```
    #[inline]
    pub async fn acquire_many_async_with<F: FnOnce()>(&self, count: usize, begin_wait: F) -> bool {
        if count > Self::MAX_PERMITS {
            return false;
        }
        let Ok(count) = u8::try_from(count) else {
            return false;
        };
        loop {
            let (result, state) = self.try_acquire_internal(count);
            if result {
                return true;
            }

            let async_wait = WaitQueue::default();
            let async_wait_pinned = async_wait.pin();
            async_wait_pinned.construct(self, Opcode::Semaphore(count), false);
            if self.try_push_wait_queue_entry(async_wait_pinned, state) {
                begin_wait();
                async_wait_pinned.await;
                return true;
            }
        }
    }

    /// Gets multiple permits from the semaphore synchronously with a wait callback.
    ///
    /// Returns `false` if the count exceeds [`Self::MAX_PERMITS`]. The callback is invoked when the
    /// task starts waiting for permits.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// let mut wait = false;
    /// assert!(semaphore.acquire_many_sync_with(2, || { wait = true; }));
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 2);
    /// assert!(!wait);
    /// ```
    #[inline]
    pub fn acquire_many_sync_with<F: FnOnce()>(&self, count: usize, mut begin_wait: F) -> bool {
        if count > Self::MAX_PERMITS {
            return false;
        }
        let Ok(count) = u8::try_from(count) else {
            return false;
        };
        loop {
            let (result, state) = self.try_acquire_internal(count);
            if result {
                return true;
            }
            // The value is checked in `try_acquire_internal`.
            if let Err(returned) =
                self.wait_resources_sync(state, Opcode::Semaphore(count), begin_wait)
            {
                begin_wait = returned;
            } else {
                return true;
            }
        }
    }

    /// Tries to get multiple permits from the semaphore.
    ///
    /// Returns `false` if no permits are available.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// assert!(semaphore.try_acquire_many(Semaphore::MAX_PERMITS));
    /// assert!(!semaphore.try_acquire());
    /// ```
    #[inline]
    pub fn try_acquire_many(&self, count: usize) -> bool {
        if count > Self::MAX_PERMITS {
            return false;
        }
        let Ok(count) = u8::try_from(count) else {
            return false;
        };
        self.try_acquire_internal(count).0
    }

    /// Registers a [`Pager`] to allow it to get a permit remotely.
    ///
    /// `is_sync` indicates whether the [`Pager`] will be polled asynchronously (`false`) or
    /// synchronously (`true`).
    ///
    /// Returns `false` if the [`Pager`] was already registered, or if the count is greater than the
    /// maximum number of permits.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::pin::pin;
    ///
    /// use saa::{Pager, Semaphore};
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// let mut pinned_pager = pin!(Pager::default());
    ///
    /// assert!(semaphore.register_pager(&mut pinned_pager, 1, true));
    /// assert!(!semaphore.register_pager(&mut pinned_pager, 1, true));
    ///
    /// assert!(pinned_pager.poll_sync().is_ok());
    /// ```
    #[inline]
    pub fn register_pager<'s>(
        &'s self,
        pager: &mut Pin<&mut Pager<'s, Self>>,
        count: usize,
        is_sync: bool,
    ) -> bool {
        if count > Self::MAX_PERMITS || pager.is_registered() {
            return false;
        }
        let Ok(count) = u8::try_from(count) else {
            return false;
        };

        pager
            .wait_queue()
            .construct(self, Opcode::Semaphore(count), is_sync);

        loop {
            let (result, state) = self.try_acquire_internal(count);
            if result {
                Entry::set_result(pager.wait_queue().entry_ptr(), 0);
                break;
            }
            if self.try_push_wait_queue_entry(pager.wait_queue(), state) {
                break;
            }
        }
        true
    }

    /// Releases a permit.
    ///
    /// Returns `true` if a permit was successfully released.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// assert!(semaphore.try_acquire_many(11));
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 11);
    ///
    /// assert!(semaphore.release());
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 10);
    /// ```
    #[inline]
    pub fn release(&self) -> bool {
        match self
            .state
            .compare_exchange(without_provenance_mut(1), null_mut(), Release, Relaxed)
        {
            Ok(_) => true,
            Err(state) => self.release_loop(state, Opcode::Semaphore(1)),
        }
    }

    /// Releases permits.
    ///
    /// Returns `true` if the specified number of permits were successfully released.
    ///
    /// # Examples
    ///
    /// ```
    /// use saa::Semaphore;
    /// use std::sync::atomic::Ordering::Relaxed;
    ///
    /// let semaphore = Semaphore::default();
    ///
    /// assert!(semaphore.try_acquire_many(11));
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 11);
    ///
    /// assert!(semaphore.release_many(10));
    /// assert_eq!(semaphore.available_permits(Relaxed), Semaphore::MAX_PERMITS - 1);
    /// ```
    #[inline]
    pub fn release_many(&self, count: usize) -> bool {
        let Ok(count) = u8::try_from(count) else {
            return false;
        };
        match self.state.compare_exchange(
            without_provenance_mut(count as usize),
            null_mut(),
            Release,
            Relaxed,
        ) {
            Ok(_) => true,
            Err(state) => self.release_loop(state, Opcode::Semaphore(count)),
        }
    }

    /// Tries to acquire a permit.
    #[inline]
    fn try_acquire_internal(&self, count: u8) -> (bool, *mut ()) {
        let mut state = self.state.load(Acquire);
        loop {
            if state.addr() & WaitQueue::ADDR_MASK != 0
                || (state.addr() & WaitQueue::DATA_MASK) + usize::from(count) > Self::MAX_PERMITS
            {
                // There is a waiting thread, or the semaphore can no longer be shared.
                return (false, state);
            }

            match self.state.compare_exchange(
                state,
                state.map_addr(|addr| addr + usize::from(count)),
                Acquire,
                Acquire,
            ) {
                Ok(_) => return (true, null_mut()),
                Err(new_state) => state = new_state,
            }
        }
    }
}

impl fmt::Debug for Semaphore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.state.load(Relaxed);
        let available_permits = Self::MAX_PERMITS - (state.addr() & WaitQueue::DATA_MASK);
        let wait_queue_being_processed =
            state.addr() & WaitQueue::LOCKED_FLAG == WaitQueue::LOCKED_FLAG;
        let wait_queue_tail_addr = state.addr() & WaitQueue::ADDR_MASK;
        f.debug_struct("WaitQueue")
            .field("state", &state)
            .field("available_permits", &available_permits)
            .field("wait_queue_being_processed", &wait_queue_being_processed)
            .field("wait_queue_tail_addr", &wait_queue_tail_addr)
            .finish()
    }
}

impl SyncPrimitive for Semaphore {
    #[inline]
    fn state(&self) -> &AtomicPtr<()> {
        &self.state
    }

    #[inline]
    fn max_shared_owners() -> usize {
        Self::MAX_PERMITS
    }

    #[inline]
    fn drop_wait_queue_entry(entry_ptr: *const Entry) {
        Self::force_remove_wait_queue_entry(entry_ptr);
    }
}

impl SyncResult for Semaphore {
    type Result = Result<(), pager::Error>;

    #[inline]
    fn to_result(_: u8, pager_error: Option<pager::Error>) -> Self::Result {
        pager_error.map_or_else(|| Ok(()), Err)
    }
}
