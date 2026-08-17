use core::ffi::c_char;
#[cfg(any(doc, not(feature = "unstable-objfw")))]
use core::ffi::c_uint;

use crate::runtime::{Imp, Method, Sel};

/// Describes an Objective-C method.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_method_description {
    /// The name of the method.
    pub name: Option<Sel>,
    /// The types of the method arguments.
    pub types: *const c_char,
}

extern_c! {
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The return value is deallocated with [`free`][crate::ffi::free].
    pub fn method_copyArgumentType(method: *const Method, index: c_uint) -> *mut c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The return value is deallocated with [`free`][crate::ffi::free].
    pub fn method_copyReturnType(method: *const Method) -> *mut c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_exchangeImplementations(method1: *mut Method, method2: *mut Method);
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getArgumentType(
        method: *const Method,
        index: c_uint,
        dst: *mut c_char,
        dst_len: usize,
    );
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn method_getDescription(m: *const Method) -> *const objc_method_description;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getImplementation(method: *const Method) -> Option<Imp>;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getName(method: *const Method) -> Option<Sel>;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getNumberOfArguments(method: *const Method) -> c_uint;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getReturnType(method: *const Method, dst: *mut c_char, dst_len: usize);
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getTypeEncoding(method: *const Method) -> *const c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_setImplementation(method: *const Method, imp: Imp) -> Option<Imp>;
}
