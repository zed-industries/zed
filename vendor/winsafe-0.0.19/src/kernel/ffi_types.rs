//! Low-level types used in FFI declarations.

#![allow(dead_code)]

pub(crate) type BOOL = i32;
pub(crate) type COMPTR = *mut std::ffi::c_void;
pub(crate) type HANDLE = *mut std::ffi::c_void;
pub(crate) type HRES = u32; // originally declared as i32
pub(crate) type PCSTR = *const u16;
pub(crate) type PCVOID = *const std::ffi::c_void;
pub(crate) type PFUNC = *const std::ffi::c_void;
pub(crate) type PSTR = *mut u16;
pub(crate) type PVOID = *mut std::ffi::c_void;
