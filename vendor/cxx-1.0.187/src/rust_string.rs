#![cfg(feature = "alloc")]
#![allow(missing_docs)]

use alloc::string::String;
use core::mem::{self, MaybeUninit};
use core::ptr;

// ABI compatible with C++ rust::String (not necessarily alloc::string::String).
#[repr(C)]
pub struct RustString {
    repr: [MaybeUninit<usize>; mem::size_of::<String>() / mem::size_of::<usize>()],
}

impl RustString {
    pub fn from(s: String) -> Self {
        unsafe { mem::transmute::<String, RustString>(s) }
    }

    pub fn from_ref(s: &String) -> &Self {
        unsafe { &*(s as *const String as *const RustString) }
    }

    pub fn from_mut(s: &mut String) -> &mut Self {
        unsafe { &mut *(s as *mut String as *mut RustString) }
    }

    pub fn into_string(self) -> String {
        unsafe { mem::transmute::<RustString, String>(self) }
    }

    pub fn as_string(&self) -> &String {
        unsafe { &*(self as *const RustString as *const String) }
    }

    pub fn as_mut_string(&mut self) -> &mut String {
        unsafe { &mut *(self as *mut RustString as *mut String) }
    }
}

impl Drop for RustString {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self.as_mut_string()) }
    }
}

const_assert_eq!(mem::size_of::<[usize; 3]>(), mem::size_of::<RustString>());
const_assert_eq!(mem::size_of::<String>(), mem::size_of::<RustString>());
const_assert_eq!(mem::align_of::<String>(), mem::align_of::<RustString>());
