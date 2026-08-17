use core::ptr;
use core::str;
use core::sync::atomic::{AtomicPtr, Ordering};
use std::ffi::CStr;
use std::os::raw::c_char;

use crate::ffi;
use crate::runtime::{AnyClass, Sel};

/// Allows storing a [`Sel`] in a static and lazily loading it.
#[derive(Debug)]
pub struct CachedSel {
    ptr: AtomicPtr<ffi::objc_selector>,
}

impl CachedSel {
    /// Constructs a new [`CachedSel`].
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    // Mark as cold since this should only ever be called once (or maybe twice
    // if running on multiple threads).
    #[cold]
    unsafe fn fetch(&self, name: *const c_char) -> Sel {
        // The panic inside `Sel::register_unchecked` is unfortunate, but
        // strict correctness is more important than speed

        // SAFETY: Input is a non-null, NUL-terminated C-string pointer.
        //
        // We know this, because we construct it in `sel!` ourselves
        let sel = unsafe { Sel::register_unchecked(name) };
        self.ptr
            .store(sel.as_ptr() as *mut ffi::objc_selector, Ordering::Relaxed);
        sel
    }

    /// Returns the cached selector. If no selector is yet cached, registers
    /// one with the given name and stores it.
    #[inline]
    pub unsafe fn get(&self, name: &str) -> Sel {
        // `Relaxed` should be fine since `sel_registerName` is thread-safe.
        let ptr = self.ptr.load(Ordering::Relaxed);
        if let Some(sel) = unsafe { Sel::from_ptr(ptr) } {
            sel
        } else {
            // SAFETY: Checked by caller
            unsafe { self.fetch(name.as_ptr().cast()) }
        }
    }
}

/// Allows storing a [`AnyClass`] reference in a static and lazily loading it.
#[derive(Debug)]
pub struct CachedClass {
    ptr: AtomicPtr<AnyClass>,
}

impl CachedClass {
    /// Constructs a new [`CachedClass`].
    #[allow(clippy::new_without_default)]
    pub const fn new() -> CachedClass {
        CachedClass {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    // Mark as cold since this should only ever be called once (or maybe twice
    // if running on multiple threads).
    #[cold]
    #[track_caller]
    unsafe fn fetch(&self, name: *const c_char) -> &'static AnyClass {
        let ptr: *const AnyClass = unsafe { ffi::objc_getClass(name) }.cast();
        self.ptr.store(ptr as *mut AnyClass, Ordering::Relaxed);
        if let Some(cls) = unsafe { ptr.as_ref() } {
            cls
        } else {
            // Recover the name from the pointer. We do it like this so that
            // we don't have to pass the length of the class to this method,
            // improving binary size.
            let name = unsafe { CStr::from_ptr(name) };
            let name = str::from_utf8(name.to_bytes()).unwrap();
            panic!("class {name} could not be found")
        }
    }

    /// Returns the cached class. If no class is yet cached, gets one with
    /// the given name and stores it.
    #[inline]
    #[track_caller]
    pub unsafe fn get(&self, name: &str) -> &'static AnyClass {
        // `Relaxed` should be fine since `objc_getClass` is thread-safe.
        let ptr = self.ptr.load(Ordering::Relaxed);
        if let Some(cls) = unsafe { ptr.as_ref() } {
            cls
        } else {
            // SAFETY: Checked by caller
            unsafe { self.fetch(name.as_ptr().cast()) }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic = "class NonExistantClass could not be found"]
    #[cfg(not(feature = "unstable-static-class"))]
    fn test_not_found() {
        let _ = crate::class!(NonExistantClass);
    }
}
