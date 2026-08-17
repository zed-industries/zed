use std::os::raw::c_char;
#[cfg(any(doc, not(feature = "unstable-objfw")))]
use std::os::raw::c_uint;

use crate::OpaqueData;

/// An opaque type that describes a property in a class.
#[repr(C)]
pub struct objc_property {
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// Describes an Objective-C property attribute.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_property_attribute_t {
    /// The name of the attribute.
    pub name: *const c_char,
    /// The value of the attribute
    ///
    /// Usually NULL.
    pub value: *const c_char,
}

extern_c! {
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::free].
    pub fn property_copyAttributeList(
        property: *const objc_property,
        out_len: *mut c_uint,
    ) -> *mut objc_property_attribute_t;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn property_copyAttributeValue(
        property: *const objc_property,
        attribute_name: *const c_char,
    ) -> *mut c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn property_getAttributes(property: *const objc_property) -> *const c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn property_getName(property: *const objc_property) -> *const c_char;
}
