use super::Atomic;

use std::sync::atomic::Ordering;

/// Mock implementation of `std::sync::atomic::AtomicPtr`.
///
/// NOTE: Unlike `std::sync::atomic::AtomicPtr`, this type has a different
/// in-memory representation than `*mut T`.
pub struct AtomicPtr<T>(Atomic<*mut T>);

impl<T> std::fmt::Debug for AtomicPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> AtomicPtr<T> {
    /// Creates a new instance of `AtomicPtr`.
    #[track_caller]
    pub fn new(v: *mut T) -> AtomicPtr<T> {
        AtomicPtr(Atomic::new(v, location!()))
    }

    /// Load the value without any synchronization.
    ///
    /// # Safety
    ///
    /// An unsynchronized atomic load technically always has undefined behavior.
    /// However, if the atomic value is not currently visible by other threads,
    /// this *should* always be equivalent to a non-atomic load of an un-shared
    /// `*mut T` value.
    pub unsafe fn unsync_load(&self) -> *mut T {
        self.0.unsync_load()
    }

    /// Get access to a mutable reference to the inner value.
    #[track_caller]
    pub fn with_mut<R>(&mut self, f: impl FnOnce(&mut *mut T) -> R) -> R {
        self.0.with_mut(f)
    }

    /// Consumes the atomic and returns the contained value.
    #[track_caller]
    pub fn into_inner(self) -> *mut T {
        // SAFETY: ownership guarantees that no other threads are concurrently
        // accessing the atomic value.
        unsafe { self.unsync_load() }
    }

    /// Loads a value from the pointer.
    #[track_caller]
    pub fn load(&self, order: Ordering) -> *mut T {
        self.0.load(order)
    }

    /// Stores a value into the pointer.
    #[track_caller]
    pub fn store(&self, val: *mut T, order: Ordering) {
        self.0.store(val, order)
    }

    /// Stores a value into the pointer, returning the previous value.
    #[track_caller]
    pub fn swap(&self, val: *mut T, order: Ordering) -> *mut T {
        self.0.swap(val, order)
    }

    /// Stores a value into the pointer if the current value is the same as the `current` value.
    #[track_caller]
    pub fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T {
        self.0.compare_and_swap(current, new, order)
    }

    /// Stores a value into the pointer if the current value is the same as the `current` value.
    #[track_caller]
    pub fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.0.compare_exchange(current, new, success, failure)
    }

    /// Stores a value into the atomic if the current value is the same as the current value.
    #[track_caller]
    pub fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.compare_exchange(current, new, success, failure)
    }

    /// Fetches the value, and applies a function to it that returns an optional new value. Returns
    /// a [`Result`] of [`Ok`]`(previous_value)` if the function returned [`Some`]`(_)`, else
    /// [`Err`]`(previous_value)`.
    #[track_caller]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<*mut T, *mut T>
    where
        F: FnMut(*mut T) -> Option<*mut T>,
    {
        self.0.fetch_update(set_order, fetch_order, f)
    }
}

impl<T> Default for AtomicPtr<T> {
    fn default() -> AtomicPtr<T> {
        use std::ptr;
        AtomicPtr::new(ptr::null_mut())
    }
}

impl<T> From<*mut T> for AtomicPtr<T> {
    fn from(p: *mut T) -> Self {
        Self::new(p)
    }
}
