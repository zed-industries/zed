use core::mem::ManuallyDrop;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use objc2::rc::Retained;
use objc2::Message;

/// Allows storing an `Retained` in a static and lazily loading it.
#[derive(Debug)]
pub struct CachedId<T> {
    ptr: AtomicPtr<T>,
}

impl<T> CachedId<T> {
    /// Constructs a new [`CachedId`].
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

impl<T: Message> CachedId<T> {
    /// Returns the cached object. If no object is yet cached, creates one
    /// from the given closure and stores it.
    #[inline]
    pub fn get(&self, f: impl FnOnce() -> Retained<T>) -> &'static T {
        // TODO: Investigate if we can use weaker orderings.
        let ptr = self.ptr.load(Ordering::SeqCst);
        // SAFETY: The pointer is either NULL, or has been created below.
        unsafe { ptr.as_ref() }.unwrap_or_else(|| {
            // "Forget" about releasing the object, promoting it to a static.
            let s = ManuallyDrop::new(f());
            let ptr = Retained::as_ptr(&s);
            self.ptr.store(ptr as *mut T, Ordering::SeqCst);
            // SAFETY: The pointer is valid, and will always be valid, since
            // we haven't released it.
            unsafe { ptr.as_ref().unwrap_unchecked() }
        })
    }
}
