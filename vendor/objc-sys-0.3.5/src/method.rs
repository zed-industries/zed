use std::os::raw::c_char;
#[cfg(any(doc, not(feature = "unstable-objfw")))]
use std::os::raw::c_uint;

#[cfg(any(doc, not(feature = "unstable-objfw")))]
use crate::IMP;
use crate::{objc_selector, OpaqueData};

/// A type that represents a method in a class definition.
#[repr(C)]
pub struct objc_method {
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// Describes an Objective-C method.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_method_description {
    /// The name of the method.
    pub name: *const objc_selector,
    /// The types of the method arguments.
    pub types: *const c_char,
}

extern_c! {
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The return value is deallocated with [`free`][crate::free].
    pub fn method_copyArgumentType(method: *const objc_method, index: c_uint) -> *mut c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The return value is deallocated with [`free`][crate::free].
    pub fn method_copyReturnType(method: *const objc_method) -> *mut c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_exchangeImplementations(method1: *mut objc_method, method2: *mut objc_method);
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getArgumentType(
        method: *const objc_method,
        index: c_uint,
        dst: *mut c_char,
        dst_len: usize,
    );
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn method_getDescription(m: *const objc_method) -> *const objc_method_description;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getImplementation(method: *const objc_method) -> IMP;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getName(method: *const objc_method) -> *const objc_selector;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getNumberOfArguments(method: *const objc_method) -> c_uint;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getReturnType(method: *const objc_method, dst: *mut c_char, dst_len: usize);
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_getTypeEncoding(method: *const objc_method) -> *const c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn method_setImplementation(method: *const objc_method, imp: IMP) -> IMP;
}
