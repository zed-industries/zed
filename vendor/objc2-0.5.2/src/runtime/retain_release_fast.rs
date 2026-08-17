//! Optimized versions of `objc_retain` and `objc_release`.
//!
//! On macOS 13.0 / iOS 16.0 / tvOS 16.0 / watchOS 9.0, on ARM64, optimized
//! versions of these two functions that use a different calling convention
//! than the usual C calling convention, are available.
//!
//! Specifically, the expected input register is changed. The output register
//! is unchanged.
//!
//! As an example, if the object is stored in the `x19` register and we need
//! to release it, we usually end up emitting an extra `mov` to get the object
//! into the `x0` register first, as expected by the C calling convention:
//!
//! ```asm
//! mov x0, x19
//! bl _objc_release
//! ```
//!
//! With this optimization though, since the expected register is encoded in
//! the name of the function instead, we can avoid the move altogether.
//!
//! ```asm
//! bl _objc_release_x19
//! ```
//!
//!
//!
//! Safety of our two uses of the `asm!` macro:
//!
//! 1. We use the register class `reg`, with the modifier `x`, which on
//!    Aarch64 is defined as `x[0-30]`, see [this][asm-reg-cls].
//!
//!    The functions are only available in the variants `x[0-15]` and
//!    `x[19-28]` though, see [this][objc4-source], so if the register
//!    allocator ends up using `x16`, `x17`, `x18`, `x29` or `x30`, we will
//!    emit a call to e.g. `objc_retain_x29`, which will fail at link time.
//!
//!    TODO: Before this option can be stable, we need a way to prevent that!
//!
//! 2. We use the `clobber_abi("C")` since we're effectively calling a C
//!    C function.
//!
//! [asm-reg-cls]: https://doc.rust-lang.org/nightly/reference/inline-assembly.html#register-operands
//! [objc4-source]: https://github.com/apple-oss-distributions/objc4/blob/objc4-866.9/runtime/objc-abi.h#L442-L498
use crate::ffi;

/// A potentially faster version of `ffi::objc_retain`.
///
///
/// # Safety
///
/// Same as `ffi::objc_retain`.
#[inline]
pub(crate) unsafe fn objc_retain_fast(obj: *mut ffi::objc_object) -> *mut ffi::objc_object {
    #[cfg(all(feature = "unstable-apple-new", target_arch = "aarch64"))]
    // SAFETY: See the file header.
    //
    // As per the ARM64 calling convention, the return value is put in `x0`.
    //
    // That the function itself is safe to call is upheld by the caller.
    unsafe {
        let result;
        core::arch::asm!(
            "bl _objc_retain_{obj:x}",
            obj = in(reg) obj,
            lateout("x0") result,
            clobber_abi("C"),
        );
        result
    }

    #[cfg(not(all(feature = "unstable-apple-new", target_arch = "aarch64")))]
    // SAFETY: Upheld by caller.
    unsafe {
        ffi::objc_retain(obj)
    }
}

/// A potentially faster version of `ffi::objc_release`.
///
///
/// # Safety
///
/// Same as `ffi::objc_release`.
#[inline]
pub(crate) unsafe fn objc_release_fast(obj: *mut ffi::objc_object) {
    #[cfg(all(feature = "unstable-apple-new", target_arch = "aarch64"))]
    // SAFETY: See the file header.
    //
    // That the function itself is safe to call is upheld by the caller.
    unsafe {
        core::arch::asm!(
            "bl _objc_release_{obj:x}",
            obj = in(reg) obj,
            clobber_abi("C"),
        )
    }

    #[cfg(not(all(feature = "unstable-apple-new", target_arch = "aarch64")))]
    // SAFETY: Upheld by caller.
    unsafe {
        ffi::objc_release(obj)
    }
}
