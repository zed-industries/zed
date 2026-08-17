//! Defined in:
//! Apple: `objc-exception.h`
//! GNUStep: `eh_personality.c`, which is a bit brittle to rely on, but I
//!   think it's fine...
#![allow(non_camel_case_types)]

// A few things here are defined differently depending on the __OBJC2__
// variable, which is set for all platforms except 32-bit macOS.

use crate::runtime::AnyObject;

/// Remember that this is non-null!
#[cfg(any(
    doc,
    all(
        target_vendor = "apple",
        not(all(target_os = "macos", target_arch = "x86"))
    )
))]
type objc_exception_matcher = unsafe extern "C" fn(
    catch_type: *mut crate::runtime::AnyClass,
    exception: *mut AnyObject,
) -> core::ffi::c_int;

/// Remember that this is non-null!
#[cfg(any(
    doc,
    all(
        target_vendor = "apple",
        not(all(target_os = "macos", target_arch = "x86"))
    )
))]
type objc_exception_preprocessor =
    unsafe extern "C" fn(exception: *mut AnyObject) -> *mut AnyObject;

/// Remember that this is non-null!
#[cfg(any(
    doc,
    all(
        target_vendor = "apple",
        not(all(target_os = "macos", target_arch = "x86"))
    )
))]
type objc_uncaught_exception_handler = unsafe extern "C" fn(exception: *mut AnyObject);

#[cfg(feature = "unstable-objfw")]
type objc_uncaught_exception_handler = Option<unsafe extern "C" fn(exception: *mut AnyObject)>;

/// Remember that this is non-null!
#[cfg(any(
    doc,
    all(target_vendor = "apple", target_os = "macos", not(target_arch = "x86"))
))]
type objc_exception_handler =
    unsafe extern "C" fn(unused: *mut AnyObject, context: *mut core::ffi::c_void);

extern_c_unwind! {
    /// See [`objc-exception.h`].
    ///
    /// [`objc-exception.h`]: https://github.com/apple-oss-distributions/objc4/blob/objc4-818.2/runtime/objc-exception.h
    #[cold]
    pub fn objc_exception_throw(exception: *mut AnyObject) -> !;

    #[cfg(all(target_vendor = "apple", not(feature = "gnustep-1-7"), not(all(target_os = "macos", target_arch = "x86"))))]
    #[cold]
    pub fn objc_exception_rethrow() -> !;

    #[cfg(feature = "gnustep-1-7")]
    #[cold]
    pub fn objc_exception_rethrow(exc_buf: *mut core::ffi::c_void) -> !;
}

extern_c! {
    #[cfg(any(doc, feature = "gnustep-1-7", all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_begin_catch(exc_buf: *mut core::ffi::c_void) -> *mut AnyObject;
    #[cfg(any(doc, feature = "gnustep-1-7", all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_end_catch();

    #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos", target_arch = "x86")))]
    pub fn objc_exception_try_enter(exception_data: *const core::ffi::c_void);

    #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos", target_arch = "x86")))]
    pub fn objc_exception_try_exit(exception_data: *const core::ffi::c_void);

    // objc_exception_extract
    // objc_exception_match
    // objc_exception_get_functions
    // objc_exception_set_functions

    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_setExceptionMatcher(f: objc_exception_matcher) -> objc_exception_matcher;
    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_setExceptionPreprocessor(
        f: objc_exception_preprocessor,
    ) -> objc_exception_preprocessor;
    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86"))), feature = "unstable-objfw"))]
    pub fn objc_setUncaughtExceptionHandler(
        f: objc_uncaught_exception_handler,
    ) -> objc_uncaught_exception_handler;

    #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos", not(target_arch = "x86"))))]
    pub fn objc_addExceptionHandler(f: objc_exception_handler, context: *mut core::ffi::c_void) -> usize;
    #[cfg(any(doc, all(target_vendor = "apple", target_os = "macos", not(target_arch = "x86"))))]
    pub fn objc_removeExceptionHandler(token: usize);

    // Only available when ENABLE_OBJCXX is set, and a useable C++ runtime is
    // present when building libobjc2.
    //
    // #[cfg(any(doc, feature = "gnustep-1-7"))]
    // pub fn objc_set_apple_compatible_objcxx_exceptions(newValue: core::ffi::c_int) -> core::ffi::c_int;

    #[cold]
    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_terminate() -> !;
}
