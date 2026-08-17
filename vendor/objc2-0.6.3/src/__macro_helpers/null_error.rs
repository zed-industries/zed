use core::ffi::CStr;
use std::sync::OnceLock;

use crate::ffi::NSInteger;
use crate::rc::{autoreleasepool, Retained};
use crate::runtime::{AnyClass, NSObject};
use crate::{msg_send, ClassType};

// Marked `#[cold]` to tell the optimizer that errors are comparatively rare.
//
// And intentionally not `#[inline]`, we'll let the optimizer figure out if it
// wants to do that or not.
#[cold]
pub(crate) unsafe fn encountered_error<E: ClassType>(err: *mut E) -> Retained<E> {
    // SAFETY: Caller ensures that the pointer is valid.
    unsafe { Retained::retain(err) }.unwrap_or_else(|| {
        let err = null_error();
        assert!(E::IS_NSERROR_COMPATIBLE);
        // SAFETY: Just checked (via `const` assertion) that the `E` type is
        // either `NSError` or `NSObject`, and hence it is valid to cast the
        // `NSObject` that we have here to that.
        unsafe { Retained::cast_unchecked(err) }
    })
}

/// Poor mans string equality in `const`. Implements `a == b`.
const fn is_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();

    if a.len() != b.len() {
        return false;
    }

    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }

    true
}

// TODO: Use inline `const` once in MSRV (or add proper trait bounds).
trait IsNSError {
    const IS_NSERROR_COMPATIBLE: bool;
}

impl<T: ClassType> IsNSError for T {
    const IS_NSERROR_COMPATIBLE: bool = {
        if is_eq(T::NAME, "NSError") || is_eq(T::NAME, "NSObject") {
            true
        } else {
            // The post monomorphization error here is not nice, but it's
            // better than UB because the user used a type that cannot be
            // converted to NSError.
            //
            // TODO: Add a trait bound or similar instead.
            panic!("error parameter must be either `NSError` or `NSObject`")
        }
    };
}

#[cold] // Mark the NULL error branch as cold
fn null_error() -> Retained<NSObject> {
    static CACHED_NULL_ERROR: OnceLock<NSErrorWrapper> = OnceLock::new();

    // We use a OnceLock here, since performance doesn't really matter, and
    // using an AtomicPtr would leak under (very) high initialization
    // contention.
    CACHED_NULL_ERROR.get_or_init(create_null_error).0.clone()
}

struct NSErrorWrapper(Retained<NSObject>);

// SAFETY: NSError is immutable and thread safe.
unsafe impl Send for NSErrorWrapper {}
unsafe impl Sync for NSErrorWrapper {}

#[cold] // Mark the error creation branch as cold
fn create_null_error() -> NSErrorWrapper {
    // Wrap creation in an autoreleasepool, since we don't know anything about
    // the outside world, and we don't want to appear to leak.
    autoreleasepool(|_| {
        // TODO: Replace with c string literals once in MSRV.

        // SAFETY: The string is NUL terminated.
        let cls = unsafe { CStr::from_bytes_with_nul_unchecked(b"NSString\0") };
        // Intentional dynamic lookup, we don't know if Foundation is linked.
        let cls = AnyClass::get(cls).unwrap_or_else(foundation_not_linked);

        // SAFETY: The string is NUL terminated.
        let domain = unsafe { CStr::from_bytes_with_nul_unchecked(b"__objc2.missingError\0") };
        // SAFETY: The signate is correct, and the string is UTF-8 encoded and
        // NUL terminated.
        let domain: Retained<NSObject> =
            unsafe { msg_send![cls, stringWithUTF8String: domain.as_ptr()] };

        // SAFETY: The string is valid.
        let cls = unsafe { CStr::from_bytes_with_nul_unchecked(b"NSError\0") };
        // Intentional dynamic lookup, we don't know if Foundation is linked.
        let cls = AnyClass::get(cls).unwrap_or_else(foundation_not_linked);

        let domain: &NSObject = &domain;
        let code: NSInteger = 0;
        let user_info: Option<&NSObject> = None;
        // SAFETY: The signate is correct.
        let err: Retained<NSObject> =
            unsafe { msg_send![cls, errorWithDomain: domain, code: code, userInfo: user_info] };
        NSErrorWrapper(err)
    })
}

fn foundation_not_linked() -> &'static AnyClass {
    panic!("Foundation must be linked to get a proper error message on NULL errors")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_eq() {
        assert!(is_eq("NSError", "NSError"));
        assert!(!is_eq("nserror", "NSError"));
        assert!(!is_eq("CFError", "NSError"));
        assert!(!is_eq("NSErr", "NSError"));
        assert!(!is_eq("NSErrorrrr", "NSError"));
    }

    #[test]
    fn test_create() {
        let _ = create_null_error().0;
    }
}
