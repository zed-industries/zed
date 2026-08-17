#[cfg(any(doc, not(feature = "unstable-objfw")))]
use core::ffi::c_void;
use std::os::raw::c_char;

#[cfg(any(doc, not(feature = "unstable-objfw")))]
use crate::objc_ivar;
#[cfg(any(doc, target_vendor = "apple"))]
use crate::BOOL;
use crate::{objc_class, OpaqueData};

/// An opaque type that represents an object / an instance of a class.
#[repr(C)]
pub struct objc_object {
    // `isa` field is deprecated, so we don't expose it here.
    //
    // Also, we need this to be a zero-sized, so that the compiler doesn't
    // assume anything about the layout.
    //
    // Use `object_getClass` instead.
    _priv: [u8; 0],
    _p: OpaqueData,
}

extern_c! {
    pub fn object_getClass(obj: *const objc_object) -> *const objc_class;
    pub fn object_getClassName(obj: *const objc_object) -> *const c_char;
    pub fn object_setClass(obj: *mut objc_object, cls: *const objc_class) -> *const objc_class;
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn object_isClass(obj: *const objc_object) -> BOOL;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_getIndexedIvars(obj: *const objc_object) -> *const c_void;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_getIvar(obj: *const objc_object, ivar: *const objc_ivar) -> *const objc_object;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_setIvar(obj: *mut objc_object, ivar: *const objc_ivar, value: *mut objc_object);

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn object_copy(obj: *const objc_object, size: usize) -> *mut objc_object;

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_dispose(obj: *mut objc_object) -> *mut objc_object;

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_setInstanceVariable(
        obj: *mut objc_object,
        name: *const c_char,
        value: *mut c_void,
    ) -> *const objc_ivar;

    // Available in macOS 10.12
    // #[deprecated = "Not needed since ARC"]
    // #[cfg(any(doc, target_vendor = "apple"))]
    // pub fn object_setInstanceVariableWithStrongDefault(
    //     obj: *mut objc_object,
    //     name: *const c_char,
    //     value: *mut c_void,
    // ) -> *const objc_ivar;

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_getInstanceVariable(
        obj: *const objc_object,
        name: *const c_char,
        out_value: *mut *const c_void,
    ) -> *const objc_ivar;

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn objc_getFutureClass(name: *const c_char) -> *const objc_class;
    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn objc_constructInstance(cls: *const objc_class, bytes: *mut c_void) -> *mut objc_object;
    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn objc_destructInstance(obj: *mut objc_object) -> *mut c_void;

    // TODO: Unsure if we should expose these; are they useful, and stable?
    // Defined in objc-abi.h
    // pub fn objc_getProperty(
    //     obj: *const objc_object,
    //     sel: *const objc_selector,
    //     offset: isize,
    //     atomic: BOOL,
    // ) -> *mut c_void;
    // pub fn objc_setProperty(
    //     obj: *const objc_object,
    //     sel: *const objc_selector,
    //     offset: isize,
    //     newValue: *const c_void,
    //     atomic: BOOL,
    //     shouldCopy: i8,
    // );
    // + the atomic versions

    // This is generated in setters to struct properties.
    // pub fn objc_copyStruct(
    //     dest: *mut c_void,
    //     src: *const c_void,
    //     size: isize,
    //     atomic: BOOL,
    //     hasStrong: BOOL,
    // );

    // #[deprecated = "use object_copy instead"]
    // #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos")))]
    // object_copyFromZone
    // #[deprecated = "use class_createInstance instead"]
    // #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos")))]
    // class_createInstanceFromZone
}
