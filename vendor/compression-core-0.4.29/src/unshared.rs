#![allow(dead_code)] // unused without any features

use core::fmt::{self, Debug};

/// Wraps a type and only allows unique borrowing.
///
/// The main use case is to wrap a `!Sync` type and implement `Sync` for it as this type blocks
/// having multiple shared references to the inner value.
///
/// # Safety
///
/// We must be careful when accessing `inner`, there must be no way to create a shared reference to
/// it from a shared reference to an `Unshared`, as that would allow creating shared references on
/// multiple threads.
///
/// As an example deriving or implementing `Clone` is impossible, two threads could attempt to
/// clone a shared `Unshared<T>` reference which would result in accessing the same inner value
/// concurrently.
pub struct Unshared<T> {
    inner: T,
}

impl<T> From<T> for Unshared<T> {
    fn from(inner: T) -> Self {
        Unshared { inner }
    }
}

impl<T> Unshared<T> {
    /// Constructs a new `Unshared` from `inner`.
    pub fn new(inner: T) -> Self {
        Self::from(inner)
    }

    /// Returns a unique reference to the inner `T`.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

/// Safety: See comments on main docs for `Unshared`
unsafe impl<T> Sync for Unshared<T> {}

impl<T> Debug for Unshared<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(core::any::type_name::<T>()).finish()
    }
}
