#![allow(missing_docs)]

use core::ffi::c_void;

#[repr(C)]
pub struct FatFunction {
    pub trampoline: *const c_void,
    pub ptr: *const c_void,
}
