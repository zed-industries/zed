use core::cell::UnsafeCell;

/// A wrapper around `UnsafeCell` that implements `Send` and `Sync` for types
/// that are themselves `Send` and `Sync`.
pub struct SendSyncUnsafeCell<T>(UnsafeCell<T>);

// Safety: `T` is `Send` and users guarantee that any pointers derived from the
// inner `UnsafeCell` are used in a way that it is safe to implement `Send` and
// `Sync` for `SendSyncUnsafeCell<T>`.
unsafe impl<T: Send> Send for SendSyncUnsafeCell<T> {}
unsafe impl<T: Sync> Sync for SendSyncUnsafeCell<T> {}

impl<T> SendSyncUnsafeCell<T>
where
    T: Send + Sync,
{
    /// Create a new `SendUnsafeCell` with the given value.
    pub fn new(inner: T) -> Self {
        Self(UnsafeCell::new(inner))
    }

    /// Get an unsafe pointer to the inner value.
    ///
    /// # Safety
    ///
    /// In addition to the safety invariants of `UnsafeCell::get` that must be
    /// upheld, this pointer may only be accessed in a way that it is
    /// dynamically safe to send the underlying value between threads and share
    /// `&T` references between threads.
    pub unsafe fn get(&self) -> *mut T {
        self.0.get()
    }

    /// Get a mutable reference to the inner value.
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}
