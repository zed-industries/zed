// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    mem::ManuallyDrop,
    sync::atomic::{AtomicUsize, Ordering},
};
fn next_thread_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

// rustdoc-stripper-ignore-next
/// Returns a unique ID for the current thread.
///
/// Actual thread IDs can be reused by the OS once the old thread finished.
/// This works around ambiguity created by ID reuse by using a separate TLS counter for threads.
pub fn thread_id() -> usize {
    thread_local!(static THREAD_ID: usize = next_thread_id());
    THREAD_ID.with(|&x| x)
}

// rustdoc-stripper-ignore-next
/// Thread guard that only gives access to the contained value on the thread it was created on.
pub struct ThreadGuard<T> {
    thread_id: usize,
    // This is a `ManuallyDrop` so that the automatic drop glue does not drop `T`
    // if this `ThreadGuard` is dropped on the wrong thread.
    value: ManuallyDrop<T>,
}

impl<T> ThreadGuard<T> {
    // rustdoc-stripper-ignore-next
    /// Create a new thread guard around `value`.
    ///
    /// The thread guard ensures that access to the value is only allowed from the thread it was
    /// created on, and otherwise panics.
    ///
    /// The thread guard implements the `Send` trait even if the contained value does not.
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            thread_id: thread_id(),
            value: ManuallyDrop::new(value),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return a reference to the contained value from the thread guard.
    ///
    /// # Panics
    ///
    /// This function panics if called from a different thread than where the thread guard was
    /// created.
    #[inline]
    pub fn get_ref(&self) -> &T {
        assert!(
            self.thread_id == thread_id(),
            "Value accessed from different thread than where it was created"
        );

        &self.value
    }

    // rustdoc-stripper-ignore-next
    /// Return a mutable reference to the contained value from the thread guard.
    ///
    /// # Panics
    ///
    /// This function panics if called from a different thread than where the thread guard was
    /// created.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        assert!(
            self.thread_id == thread_id(),
            "Value accessed from different thread than where it was created"
        );

        &mut self.value
    }

    // rustdoc-stripper-ignore-next
    /// Return the contained value from the thread guard.
    ///
    /// # Panics
    ///
    /// This function panics if called from a different thread than where the thread guard was
    /// created.
    #[inline]
    pub fn into_inner(self) -> T {
        // We wrap `self` in `ManuallyDrop` to defuse `ThreadGuard`'s `Drop` impl.
        let mut this = ManuallyDrop::new(self);

        assert!(
            this.thread_id == thread_id(),
            "Value accessed from different thread than where it was created"
        );

        // SAFETY: We are on the right thread, and this.value will not be touched after this
        unsafe { ManuallyDrop::take(&mut this.value) }
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if the current thread owns the value, i.e. it can be accessed safely.
    #[inline]
    pub fn is_owner(&self) -> bool {
        self.thread_id == thread_id()
    }
}

impl<T> Drop for ThreadGuard<T> {
    #[inline]
    fn drop(&mut self) {
        assert!(
            self.thread_id == thread_id(),
            "Value dropped on a different thread than where it was created"
        );

        // SAFETY: We are on the right thread, and self.value will not be touched after this
        unsafe { ManuallyDrop::drop(&mut self.value) }
    }
}

unsafe impl<T> Send for ThreadGuard<T> {}
unsafe impl<T> Sync for ThreadGuard<T> {}

impl<T> std::ops::Deref for ThreadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}

impl<T> std::ops::DerefMut for ThreadGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
