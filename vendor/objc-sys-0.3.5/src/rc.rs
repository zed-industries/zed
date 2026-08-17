//! ARC functions.
//!
//! These are documented in Clang's [documentation][ARC], and available since
//! macOS `10.7` unless otherwise noted, so they are safe to rely on.
//!
//! Defined in:
//! - Apple: `objc-internal.h`
//! - GNUStep: `objc-arc.h`
//! - ObjFW: `runtime/arc.m`
//!
//! [ARC]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#runtime-support
use core::ffi::c_void;

#[cfg(any(
    doc,
    all(
        target_vendor = "apple",
        not(all(target_os = "macos", target_arch = "x86"))
    )
))]
use crate::objc_class;
use crate::objc_object;

// All of these very rarely unwind, but may if the user defined methods
// `retain`, `release`, `autorelease` or `dealloc` do.
extern_c_unwind! {
    // Autoreleasepool
    // ObjFW: Defined in `autorelease.h`, not available with libobjfw-rt!

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_autoreleasePoolPop(pool: *mut c_void);
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_autoreleasePoolPush() -> *mut c_void;

    // Autorelease

    pub fn objc_autorelease(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_autoreleaseReturnValue(value: *mut objc_object) -> *mut objc_object;

    // Weak pointers

    pub fn objc_copyWeak(to: *mut *mut objc_object, from: *mut *mut objc_object);
    pub fn objc_destroyWeak(addr: *mut *mut objc_object);
    pub fn objc_initWeak(addr: *mut *mut objc_object, value: *mut objc_object) -> *mut objc_object;
    // Defined in runtime.h
    pub fn objc_loadWeak(addr: *mut *mut objc_object) -> *mut objc_object;
    pub fn objc_loadWeakRetained(addr: *mut *mut objc_object) -> *mut objc_object;
    pub fn objc_moveWeak(to: *mut *mut objc_object, from: *mut *mut objc_object);

    // Retain / release

    pub fn objc_release(value: *mut objc_object);
    pub fn objc_retain(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_retainAutorelease(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_retainAutoreleaseReturnValue(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_retainAutoreleasedReturnValue(value: *mut objc_object) -> *mut objc_object;
    // Defined in objc-abi.h
    pub fn objc_retainBlock(value: *mut objc_object) -> *mut objc_object;

    // Storing values

    pub fn objc_storeStrong(addr: *mut *mut objc_object, value: *mut objc_object);
    // Defined in runtime.h
    pub fn objc_storeWeak(addr: *mut *mut objc_object, value: *mut objc_object)
        -> *mut objc_object;

    // TODO: Decide about nonstandard extensions like these:
    // #[cfg(any(doc, feature = "gnustep-1-7"))]
    // pub fn objc_delete_weak_refs(obj: *mut objc_object) -> BOOL;

    // Fast paths for certain selectors.
    //
    // These are not defined in the ARC documentation, but are emitted by
    // `clang` and included (and intended to be included) in the final
    // binary, so very likely safe to use.
    //
    // TODO: Unsure why these are not available in the old fragile runtime,
    // the headers seem to indicate that they are.
    //
    // <https://github.com/llvm/llvm-project/blob/llvmorg-17.0.5/clang/include/clang/Basic/ObjCRuntime.h#L229>

    // Available since macOS 10.9.
    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_alloc(value: *const objc_class) -> *mut objc_object;

    // Available since macOS 10.9.
    #[cfg(any(doc, all(target_vendor = "apple", not(all(target_os = "macos", target_arch = "x86")))))]
    pub fn objc_allocWithZone(value: *const objc_class) -> *mut objc_object;

    // TODO: objc_alloc_init once supported
}
