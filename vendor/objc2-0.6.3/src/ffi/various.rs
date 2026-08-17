#[cfg(any(doc, not(feature = "unstable-objfw")))]
use core::ffi::c_char;
use core::ffi::c_int;
#[cfg(any(doc, target_vendor = "apple"))]
use core::ffi::c_uint;
use core::ffi::c_void;

use crate::runtime::AnyObject;
use crate::runtime::Imp;
#[cfg(any(doc, not(feature = "unstable-objfw")))]
use crate::{
    ffi::objc_AssociationPolicy,
    runtime::{Bool, Ivar},
};

// /// Remember that this is non-null!
// #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
// type objc_hook_getClass =
//     unsafe extern "C" fn(name: *const c_char, out_cls: *mut *const crate::runtime::AnyClass) -> Bool;
//
// /// Remember that this is non-null!
// #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
// type objc_hook_lazyClassNamer =
//     unsafe extern "C" fn(cls: *const crate::runtime::AnyClass) -> *const c_char;

extern_c_unwind! {
    // Instead of being able to change this, it's a weak symbol on GNUStep.
    #[cfg(any(doc, target_vendor = "apple", feature = "unstable-objfw"))]
    pub fn objc_enumerationMutation(obj: *mut AnyObject);
}

extern_c! {
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn imp_getBlock(imp: Imp) -> *mut AnyObject;
    // See also <https://landonf.org/code/objc/imp_implementationWithBlock.20110413.html>
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn imp_implementationWithBlock(block: *mut AnyObject) -> Imp;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn imp_removeBlock(imp: Imp) -> Bool;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn ivar_getName(ivar: *const Ivar) -> *const c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn ivar_getOffset(ivar: *const Ivar) -> isize;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn ivar_getTypeEncoding(ivar: *const Ivar) -> *const c_char;

    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn objc_copyClassNamesForImage(
        image: *const c_char,
        out_len: *mut c_uint,
    ) -> *mut *const c_char;
    #[cfg(any(doc, target_vendor = "apple"))]
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn objc_copyImageNames(out_len: *mut c_uint) -> *mut *const c_char;

    #[cfg(any(doc, target_vendor = "apple", feature = "unstable-objfw"))]
    pub fn objc_setEnumerationMutationHandler(
        handler: Option<unsafe extern "C-unwind" fn(obj: *mut AnyObject)>,
    );

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_getAssociatedObject(
        object: *const AnyObject,
        key: *const c_void,
    ) -> *const AnyObject;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_setAssociatedObject(
        object: *mut AnyObject,
        key: *const c_void,
        value: *mut AnyObject,
        policy: objc_AssociationPolicy,
    );
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_removeAssociatedObjects(object: *mut AnyObject);

    #[cfg(any(doc, target_vendor = "apple", feature = "unstable-objfw"))]
    pub fn objc_setForwardHandler(fwd: *mut c_void, fwd_stret: *mut c_void);
    // These two are defined in:
    // - Apple: objc-sync.h
    // - GNUStep: dtable.h / associate.m
    // - ObjFW: ObjFW-RT.h
    pub fn objc_sync_enter(obj: *mut AnyObject) -> c_int;
    pub fn objc_sync_exit(obj: *mut AnyObject) -> c_int;

    // Available in macOS 10.14.4
    // /// Remember that this is non-null!
    // #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    // pub fn objc_setHook_getClass(
    //     new_value: objc_hook_getClass,
    //     out_old_value: *mut objc_hook_getClass,
    // );
    // Available in macOS 11
    // /// Remember that this is non-null!
    // #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    // pub fn objc_setHook_lazyClassNamer(
    //     new_value: objc_hook_lazyClassNamer,
    //     out_old_value: *mut objc_hook_lazyClassNamer,
    // );

    // #[deprecated = "not recommended"]
    // #[cfg(any(doc, target_vendor = "apple"))]
    // pub fn _objc_flush_caches

    // #[cfg(any(doc, feature = "gnustep-1-7"))]
    // objc_test_capability
}
