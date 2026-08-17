#![allow(dead_code)]
use core::{fmt, ptr::NonNull};

use objc2::runtime::__nsstring::nsstring_to_str;
use objc2::{
    rc::{autoreleasepool_leaking, Retained},
    runtime::NSObject,
};

pub(crate) fn retained_ptr_cast<T: ?Sized>(objects: *mut Retained<T>) -> *mut NonNull<T> {
    // SAFETY: `Retained<T>` has the same memory layout as `NonNull<T>`, and
    // stronger guarantees.
    objects.cast()
}

pub(crate) fn ref_ptr_cast_const<T: ?Sized>(objects: *const &T) -> *mut NonNull<T> {
    // SAFETY: `&T` has the same memory layout as `NonNull<T>`, and stronger
    // guarantees.
    (objects as *mut &T).cast()
}

pub(crate) fn retained_ptr_cast_const<T: ?Sized>(objects: *const Retained<T>) -> *mut NonNull<T> {
    retained_ptr_cast(objects as *mut Retained<T>)
}

/// Display the string.
///
/// Put here to allow using it without the `"NSString"` feature being active.
///
/// # Safety
///
/// The string must be an instance of `NSString`.
pub(crate) unsafe fn display_string(string: &NSObject, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // SAFETY:
    // - The caller upholds that the object is a `NSString`.
    // - We control the scope in which the string is alive, so we know
    //   it is not moved outside the current autorelease pool.
    //
    // TODO: Use more performant APIs, maybe by copying bytes into a
    // temporary stack buffer so that we avoid allocating?
    //
    // Beware though that the string may be mutable internally, and that
    // mutation may happen on every call to the formatter `f` (so
    // `CFStringGetCharactersPtr` is probably out of the question, unless we
    // somehow check that the string is immutable?).
    autoreleasepool_leaking(|pool| fmt::Display::fmt(unsafe { nsstring_to_str(string, pool) }, f))
}
