//! This module implements a simplified but safe version of
//! [`scopeguard`](https://crates.io/crates/scopeguard).

use std::mem::{ManuallyDrop, forget};
use std::ops::{Deref, DerefMut};

/// [`ExitGuard`] captures the environment and invokes a defined closure at the end of the scope.
pub(crate) struct ExitGuard<T, F: FnOnce(T)> {
    drop_callback: ManuallyDrop<(T, F)>,
}

impl<T, F: FnOnce(T)> ExitGuard<T, F> {
    /// Creates a new [`ExitGuard`] with the specified variables captured.
    #[inline]
    pub(crate) const fn new(captured: T, drop_callback: F) -> Self {
        Self {
            drop_callback: ManuallyDrop::new((captured, drop_callback)),
        }
    }

    /// Forgets the [`ExitGuard`] without invoking the drop callback.
    #[inline]
    pub(crate) fn forget(mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.drop_callback);
        }
        forget(self);
    }
}

impl<T, F: FnOnce(T)> Drop for ExitGuard<T, F> {
    #[inline]
    fn drop(&mut self) {
        let (c, f) = unsafe { ManuallyDrop::take(&mut self.drop_callback) };
        f(c);
    }
}

impl<T, F: FnOnce(T)> Deref for ExitGuard<T, F> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.drop_callback.0
    }
}

impl<T, F: FnOnce(T)> DerefMut for ExitGuard<T, F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.drop_callback.0
    }
}
