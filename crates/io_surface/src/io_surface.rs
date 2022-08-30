#![allow(non_snake_case)]

use core_foundation::{
    base::{CFTypeID, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};
use std::ffi::c_void;

#[repr(C)]
pub struct __IOSurface(c_void);
// The ref type must be a pointer to the underlying struct.
pub type IOSurfaceRef = *const __IOSurface;

declare_TCFType!(IOSurface, IOSurfaceRef);
impl_TCFType!(IOSurface, IOSurfaceRef, IOSurfaceGetTypeID);
impl_CFTypeDescription!(IOSurface);

#[link(name = "IOSurface", kind = "framework")]
extern "C" {
    fn IOSurfaceGetTypeID() -> CFTypeID;
}
