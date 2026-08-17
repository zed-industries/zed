use super::Atomic;

use std::sync::atomic::Ordering;

/// Mock implementation of `std::sync::atomic::AtomicBool`.
///
/// NOTE: Unlike `std::sync::atomic::AtomicBool`, this type has a different
/// in-memory representation than `bool`.
#[derive(Debug)]
pub struct AtomicBool(Atomic<bool>);

impl AtomicBool {
    /// Creates a new instance of `AtomicBool`.
    #[track_caller]
    pub fn new(v: bool) -> AtomicBool {
        AtomicBool(Atomic::new(v, location!()))
    }

    /// Load the value without any synchronization.
    ///
    /// # Safety
    ///
    /// An unsynchronized atomic load technically always has undefined behavior.
    /// However, if the atomic value is not currently visible by other threads,
    /// this *should* always be equivalent to a non-atomic load of an un-shared
    /// `bool` value.
    #[track_caller]
    pub unsafe fn unsync_load(&self) -> bool {
        self.0.unsync_load()
    }

    /// Consumes the atomic and returns the contained value.
    #[track_caller]
    pub fn into_inner(self) -> bool {
        // SAFETY: ownership guarantees that no other threads are concurrently
        // accessing the atomic value.
        unsafe { self.unsync_load() }
    }

    /// Loads a value from the atomic bool.
    #[track_caller]
    pub fn load(&self, order: Ordering) -> bool {
        self.0.load(order)
    }

    /// Stores a value into the atomic bool.
    #[track_caller]
    pub fn store(&self, val: bool, order: Ordering) {
        self.0.store(val, order)
    }

    /// Stores a value into the atomic bool, returning the previous value.
    #[track_caller]
    pub fn swap(&self, val: bool, order: Ordering) -> bool {
        self.0.swap(val, order)
    }

    /// Stores a value into the atomic bool if the current value is the same as the `current` value.
    #[track_caller]
    pub fn compare_and_swap(&self, current: bool, new: bool, order: Ordering) -> bool {
        self.0.compare_and_swap(current, new, order)
    }

    /// Stores a value into the atomic if the current value is the same as the `current` value.
    #[track_caller]
    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        self.0.compare_exchange(current, new, success, failure)
    }

    /// Stores a value into the atomic if the current value is the same as the current value.
    #[track_caller]
    pub fn compare_exchange_weak(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        self.compare_exchange(current, new, success, failure)
    }

    /// Logical "and" with the current value.
    #[track_caller]
    pub fn fetch_and(&self, val: bool, order: Ordering) -> bool {
        self.0.rmw(|v| v & val, order)
    }

    /// Logical "nand" with the current value.
    #[track_caller]
    pub fn fetch_nand(&self, val: bool, order: Ordering) -> bool {
        self.0.rmw(|v| !(v & val), order)
    }

    /// Logical "or" with the current value.
    #[track_caller]
    pub fn fetch_or(&self, val: bool, order: Ordering) -> bool {
        self.0.rmw(|v| v | val, order)
    }

    /// Logical "xor" with the current value.
    #[track_caller]
    pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool {
        self.0.rmw(|v| v ^ val, order)
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
    ) -> Result<bool, bool>
    where
        F: FnMut(bool) -> Option<bool>,
    {
        self.0.fetch_update(set_order, fetch_order, f)
    }
}

impl Default for AtomicBool {
    fn default() -> AtomicBool {
        AtomicBool::new(Default::default())
    }
}

impl From<bool> for AtomicBool {
    fn from(b: bool) -> Self {
        Self::new(b)
    }
}
