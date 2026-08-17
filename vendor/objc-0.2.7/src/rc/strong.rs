use std::fmt;
use std::mem;
use std::ops::Deref;

use runtime::{Object, self};
use super::WeakPtr;

/// A pointer that strongly references an object, ensuring it won't be deallocated.
pub struct StrongPtr(*mut Object);

impl StrongPtr {
    /// Constructs a `StrongPtr` to a newly created object that already has a
    /// +1 retain count. This will not retain the object.
    /// When dropped, the object will be released.
    /// Unsafe because the caller must ensure the given object pointer is valid.
    pub unsafe fn new(ptr: *mut Object) -> Self {
        StrongPtr(ptr)
    }

    /// Retains the given object and constructs a `StrongPtr` to it.
    /// When dropped, the object will be released.
    /// Unsafe because the caller must ensure the given object pointer is valid.
    pub unsafe fn retain(ptr: *mut Object) -> Self {
        StrongPtr(runtime::objc_retain(ptr))
    }

    /// Autoreleases self, meaning that the object is not immediately released,
    /// but will be when the autorelease pool is drained. A pointer to the
    /// object is returned, but its validity is no longer ensured.
    pub fn autorelease(self) -> *mut Object {
        let ptr = self.0;
        mem::forget(self);
        unsafe {
            runtime::objc_autorelease(ptr);
        }
        ptr
    }

    /// Returns a `WeakPtr` to self.
    pub fn weak(&self) -> WeakPtr {
        unsafe { WeakPtr::new(self.0) }
    }
}

impl Drop for StrongPtr {
    fn drop(&mut self) {
        unsafe {
            runtime::objc_release(self.0);
        }
    }
}

impl Clone for StrongPtr {
    fn clone(&self) -> StrongPtr {
        unsafe {
            StrongPtr::retain(self.0)
        }
    }
}

impl Deref for StrongPtr {
    type Target = *mut Object;

    fn deref(&self) -> &*mut Object {
        &self.0
    }
}

impl fmt::Pointer for StrongPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}
