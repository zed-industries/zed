use std::cell::UnsafeCell;
use std::pin::{Pin, pin};
use std::ptr;
use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering::Acquire;

use saa::lock::Mode;
use saa::{Lock, Pager};
use sdd::{AtomicShared, Guard};

/// [`AsyncGuard`] is used when an asynchronous task needs to be suspended without invalidating any
/// references.
///
/// The validity of those references must be checked and verified by the user.
#[derive(Debug, Default)]
pub(crate) struct AsyncGuard {
    /// [`Guard`] that can be dropped without invalidating any references.
    guard: UnsafeCell<Option<Guard>>,
}

#[derive(Debug)]
pub(crate) struct AsyncPager {
    /// Allows the user to await the lock anywhere in the code.
    pager: Pager<'static, Lock>,
}

/// [`LockPager`] enables asynchronous code to remotely wait for a [`Lock`].
pub(crate) trait LockPager {
    /// Registers the [`Pager`] in the [`Lock`], or synchronously waits for the [`Lock`] to be
    /// available.
    ///
    /// Returns `true` if the thread can retry the operation in-place.
    #[must_use]
    fn try_wait(&mut self, lock: &Lock) -> bool;

    /// Tries to acquire the [`Lock`] synchronously, or registers the [`Pager`] in the [`Lock`] and
    /// returns an error.
    fn try_acquire(&mut self, lock: &Lock) -> Result<bool, ()>;
}

impl AsyncGuard {
    /// Returns `true` if the [`AsyncGuard`] contains a valid [`Guard`].
    #[inline]
    pub(crate) const fn has_guard(&self) -> bool {
        unsafe { (*self.guard.get()).is_some() }
    }

    /// Returns or creates a new [`Guard`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that any references derived from the returned [`Guard`] do not
    /// outlive the underlying instance.
    #[inline]
    pub(crate) fn guard(&self) -> &Guard {
        unsafe { (*self.guard.get()).get_or_insert_with(Guard::new) }
    }

    /// Resets the [`AsyncGuard`] to its initial state.
    #[inline]
    pub(crate) fn reset(&self) {
        unsafe {
            *self.guard.get() = None;
        }
    }

    /// Loads the content of the [`AtomicShared`] without exposing the [`Guard`] or checking tag
    /// bits.
    #[inline]
    pub(crate) fn load_unchecked<T>(
        &self,
        atomic_ptr: &AtomicShared<T>,
        mo: Ordering,
    ) -> Option<&T> {
        unsafe { atomic_ptr.load(mo, self.guard()).as_ref_unchecked() }
    }

    /// Checks if the reference is valid.
    #[inline]
    pub(crate) fn check_ref<T>(&self, atomic_ptr: &AtomicShared<T>, r: &T, mo: Ordering) -> bool {
        self.load_unchecked(atomic_ptr, mo)
            .is_some_and(|s| ptr::eq(s, r))
    }
}

// SAFETY: this is the sole purpose of `AsyncGuard`; Send-safety should be ensured by the user,
// e.g., the `AsyncGuard` should always be reset before the task is suspended.
unsafe impl Send for AsyncGuard {}
unsafe impl Sync for AsyncGuard {}

impl AsyncPager {
    /// Awaits the [`Lock`] to be available.
    #[inline]
    pub async fn wait(self: &mut Pin<&mut Self>) {
        let this = unsafe { ptr::read(self) };
        let mut pinned_pager = unsafe { Pin::new_unchecked(&mut this.get_unchecked_mut().pager) };
        let _result = pinned_pager.poll_async().await;
    }
}

impl Default for AsyncPager {
    #[inline]
    fn default() -> Self {
        Self {
            pager: unsafe {
                std::mem::transmute::<Pager<'_, Lock>, Pager<'static, Lock>>(Pager::default())
            },
        }
    }
}

impl LockPager for Pin<&mut AsyncPager> {
    #[inline]
    fn try_wait(&mut self, lock: &Lock) -> bool {
        let this = unsafe { ptr::read(self) };
        let mut pinned_pager = unsafe {
            let pager_ref = std::mem::transmute::<&mut Pager<'static, Lock>, &mut Pager<Lock>>(
                &mut this.get_unchecked_mut().pager,
            );
            Pin::new_unchecked(pager_ref)
        };
        lock.register_pager(&mut pinned_pager, Mode::WaitExclusive, false);
        false
    }

    #[inline]
    fn try_acquire(&mut self, lock: &Lock) -> Result<bool, ()> {
        if lock.try_lock() {
            return Ok(true);
        } else if lock.is_poisoned(Acquire) {
            return Ok(false);
        }
        let _: bool = self.try_wait(lock);
        Err(())
    }
}

impl LockPager for () {
    #[inline]
    fn try_wait(&mut self, lock: &Lock) -> bool {
        let mut pinned_pager = pin!(Pager::default());
        lock.register_pager(&mut pinned_pager, Mode::WaitExclusive, true);
        pinned_pager.poll_sync().is_ok_and(|r| r)
    }

    #[inline]
    fn try_acquire(&mut self, lock: &Lock) -> Result<bool, ()> {
        Ok(lock.lock_sync())
    }
}
