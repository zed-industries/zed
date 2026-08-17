use std::os::raw::c_char;
#[cfg(any(doc, not(feature = "unstable-objfw")))]
use std::os::raw::c_uint;

#[cfg(any(doc, not(feature = "unstable-objfw")))]
use crate::{objc_method_description, objc_property, objc_property_attribute_t, objc_selector};
use crate::{OpaqueData, BOOL};

/// Opaque type for Objective-C protocols.
///
/// Note that, although protocols are objects, sending messages to them is
/// deprecated and may not work in the future.
///
/// The naming of this follows GNUStep; this struct does not exist in Apple's
/// runtime, there `Protocol` is a type alias of `objc_object`.
#[repr(C)]
pub struct objc_protocol {
    _priv: [u8; 0],
    _p: OpaqueData,
}

extern_c! {
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_getProtocol(name: *const c_char) -> *const objc_protocol;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::free].
    pub fn objc_copyProtocolList(out_len: *mut c_uint) -> *mut *const objc_protocol;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_allocateProtocol(name: *const c_char) -> *mut objc_protocol;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_registerProtocol(proto: *mut objc_protocol);

    pub fn protocol_conformsToProtocol(
        proto: *const objc_protocol,
        other: *const objc_protocol,
    ) -> BOOL;
    pub fn protocol_isEqual(proto: *const objc_protocol, other: *const objc_protocol) -> BOOL;
    pub fn protocol_getName(proto: *const objc_protocol) -> *const c_char;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_addMethodDescription(
        proto: *mut objc_protocol,
        name: *const objc_selector,
        types: *const c_char,
        is_required_method: BOOL,
        is_instance_method: BOOL,
    );
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_addProperty(
        proto: *mut objc_protocol,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributes_len: c_uint,
        is_required_property: BOOL,
        is_instance_property: BOOL,
    );
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_addProtocol(proto: *mut objc_protocol, addition: *const objc_protocol);
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::free].
    pub fn protocol_copyMethodDescriptionList(
        proto: *const objc_protocol,
        is_required_method: BOOL,
        is_instance_method: BOOL,
        out_len: *mut c_uint,
    ) -> *mut objc_method_description;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::free].
    pub fn protocol_copyPropertyList(
        proto: *const objc_protocol,
        out_len: *mut c_uint,
    ) -> *mut *const objc_property;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::free].
    pub fn protocol_copyProtocolList(
        proto: *const objc_protocol,
        out_len: *mut c_uint,
    ) -> *mut *const objc_protocol;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_getMethodDescription(
        proto: *const objc_protocol,
        sel: *const objc_selector,
        is_required_method: BOOL,
        is_instance_method: BOOL,
    ) -> objc_method_description;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_getProperty(
        proto: *const objc_protocol,
        name: *const c_char,
        is_required_property: BOOL,
        is_instance_property: BOOL,
    ) -> *const objc_property;

    // #[cfg(any(doc, macos >= 10.12))]
    // protocol_copyPropertyList2

    // #[cfg(any(doc, feature = "gnustep-1-7"))]
    // _protocol_getMethodTypeEncoding
}
