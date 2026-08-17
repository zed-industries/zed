#![allow(missing_docs)]

use core::mem::{self, MaybeUninit};
use core::ptr::NonNull;
use core::str;

// ABI compatible with C++ rust::Str (not necessarily &str).
#[repr(C)]
pub struct RustStr {
    repr: [MaybeUninit<usize>; mem::size_of::<NonNull<str>>() / mem::size_of::<usize>()],
}

impl RustStr {
    pub fn from(repr: &str) -> Self {
        let repr = NonNull::from(repr);
        unsafe { mem::transmute::<NonNull<str>, RustStr>(repr) }
    }

    pub unsafe fn as_str<'a>(self) -> &'a str {
        unsafe {
            let repr = mem::transmute::<RustStr, NonNull<str>>(self);
            &*repr.as_ptr()
        }
    }
}

const_assert_eq!(mem::size_of::<NonNull<str>>(), mem::size_of::<RustStr>());
const_assert_eq!(mem::align_of::<NonNull<str>>(), mem::align_of::<RustStr>());
