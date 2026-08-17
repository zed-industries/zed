use core::ffi::c_char;
#[cfg(any(doc, not(feature = "unstable-objfw")))]
use core::ffi::c_void;

#[cfg(any(doc, not(feature = "unstable-objfw")))]
use crate::runtime::Ivar;
use crate::runtime::{AnyClass, AnyObject};

extern_c! {
    pub fn object_getClass(obj: *const AnyObject) -> *const AnyClass;
    pub fn object_getClassName(obj: *const AnyObject) -> *const c_char;
    pub fn object_setClass(obj: *mut AnyObject, cls: *const AnyClass) -> *const AnyClass;
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn object_isClass(obj: *const AnyObject) -> crate::runtime::Bool;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_getIndexedIvars(obj: *const AnyObject) -> *const c_void;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_getIvar(obj: *const AnyObject, ivar: *const Ivar) -> *const AnyObject;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_setIvar(obj: *mut AnyObject, ivar: *const Ivar, value: *mut AnyObject);

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn object_copy(obj: *const AnyObject, size: usize) -> *mut AnyObject;

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_dispose(obj: *mut AnyObject) -> *mut AnyObject;

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_setInstanceVariable(
        obj: *mut AnyObject,
        name: *const c_char,
        value: *mut c_void,
    ) -> *const Ivar;

    // Available in macOS 10.12
    // #[deprecated = "Not needed since ARC"]
    // #[cfg(any(doc, target_vendor = "apple"))]
    // pub fn object_setInstanceVariableWithStrongDefault(
    //     obj: *mut AnyObject,
    //     name: *const c_char,
    //     value: *mut c_void,
    // ) -> *const Ivar;

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn object_getInstanceVariable(
        obj: *const AnyObject,
        name: *const c_char,
        out_value: *mut *const c_void,
    ) -> *const Ivar;

    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn objc_getFutureClass(name: *const c_char) -> *const AnyClass;
    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn objc_constructInstance(cls: *const AnyClass, bytes: *mut c_void) -> *mut AnyObject;
    #[deprecated = "Not needed since ARC"]
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn objc_destructInstance(obj: *mut AnyObject) -> *mut c_void;

    // TODO: Unsure if we should expose these; are they useful, and stable?
    // Defined in objc-abi.h
    // pub fn objc_getProperty(
    //     obj: *const AnyObject,
    //     sel: Sel,
    //     offset: isize,
    //     atomic: Bool,
    // ) -> *mut c_void;
    // pub fn objc_setProperty(
    //     obj: *const AnyObject,
    //     sel: Sel,
    //     offset: isize,
    //     newValue: *const c_void,
    //     atomic: Bool,
    //     shouldCopy: i8,
    // );
    // + the atomic versions

    // This is generated in setters to struct properties.
    // pub fn objc_copyStruct(
    //     dest: *mut c_void,
    //     src: *const c_void,
    //     size: isize,
    //     atomic: Bool,
    //     hasStrong: Bool,
    // );

    // #[deprecated = "use object_copy instead"]
    // #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos")))]
    // object_copyFromZone
    // #[deprecated = "use class_createInstance instead"]
    // #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos")))]
    // class_createInstanceFromZone
}
