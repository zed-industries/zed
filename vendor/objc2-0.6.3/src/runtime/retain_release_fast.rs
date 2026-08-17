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
//!    Of thesee five registers, `x18` is a reserved register on Apple, so the
//!    register allocator won't use it. `x29` is the frame pointer, so that
//!    will also never be used (at least not for storing a pointer to an
//!    object).
//!
//!    Left are `x16`, `x17` and `x30`, which we prevent from being selected
//!    by specifying `out("x16") _`, `out("x17") _` and `out("x30") _`, which
//!    tricks the register allocator into believing that it cannot use those
//!    registers. This may be slightly worse than optimal, since it also
//!    clobbers the registers, but then again, so does `clobber_abi("C")`, so
//!    it probably won't matter.
//!
//!    (NOTE: I'm not _entirely_ sure that the register allocator won't select
//!    one of the clobbered registers, but luckily, it is not a safety
//!    requirement either way, it will "just" lead to a link error).
//!
//! 2. We use the `clobber_abi("C")` since we're effectively calling a C
//!    C function.
//!
//! [asm-reg-cls]: https://doc.rust-lang.org/nightly/reference/inline-assembly.html#register-operands
//! [objc4-source]: https://github.com/apple-oss-distributions/objc4/blob/objc4-866.9/runtime/objc-abi.h#L442-L498
use crate::runtime::AnyObject;

/// A potentially faster version of `ffi::objc_retain`.
///
///
/// # Safety
///
/// Same as `ffi::objc_retain`.
#[inline]
pub(crate) unsafe fn objc_retain_fast(obj: *mut AnyObject) -> *mut AnyObject {
    #[cfg(all(feature = "unstable-apple-new", target_arch = "aarch64"))]
    // SAFETY: See the file header.
    //
    // As per the ARM64 calling convention, the return value is put in `x0`.
    //
    // That the function itself is safe to call is upheld by the caller.
    //
    // TODO: Unwinding.
    unsafe {
        let result;
        core::arch::asm!(
            "bl _objc_retain_{obj:x}",
            obj = in(reg) obj,
            lateout("x0") result,
            out("x16") _,
            out("x17") _,
            out("x30") _,
            clobber_abi("C"),
        );
        result
    }

    #[cfg(not(all(feature = "unstable-apple-new", target_arch = "aarch64")))]
    // SAFETY: Upheld by caller.
    unsafe {
        crate::ffi::objc_retain(obj)
    }
}

/// A potentially faster version of `ffi::objc_release`.
///
///
/// # Safety
///
/// Same as `ffi::objc_release`.
#[inline]
pub(crate) unsafe fn objc_release_fast(obj: *mut AnyObject) {
    #[cfg(all(feature = "unstable-apple-new", target_arch = "aarch64"))]
    // SAFETY: See the file header.
    //
    // That the function itself is safe to call is upheld by the caller.
    //
    // TODO: Unwinding.
    unsafe {
        core::arch::asm!(
            "bl _objc_release_{obj:x}",
            obj = in(reg) obj,
            out("x16") _,
            out("x17") _,
            out("x30") _,
            clobber_abi("C"),
        )
    }

    #[cfg(not(all(feature = "unstable-apple-new", target_arch = "aarch64")))]
    // SAFETY: Upheld by caller.
    unsafe {
        crate::ffi::objc_release(obj)
    }
}

#[cfg(test)]
mod tests {
    use crate::rc::Retained;
    use crate::runtime::NSObject;

    #[test]
    fn retain_release_30_times() {
        #[inline(never)]
        fn new_obj() -> Retained<NSObject> {
            NSObject::new()
        }

        let obj00 = new_obj();
        let obj01 = new_obj();
        let obj02 = new_obj();
        let obj03 = new_obj();
        let obj04 = new_obj();
        let obj05 = new_obj();
        let obj06 = new_obj();
        let obj07 = new_obj();
        let obj08 = new_obj();
        let obj09 = new_obj();
        let obj10 = new_obj();
        let obj11 = new_obj();
        let obj12 = new_obj();
        let obj13 = new_obj();
        let obj14 = new_obj();
        let obj15 = new_obj();
        let obj16 = new_obj();
        let obj17 = new_obj();
        let obj18 = new_obj();
        let obj19 = new_obj();
        let obj20 = new_obj();
        let obj21 = new_obj();
        let obj22 = new_obj();
        let obj23 = new_obj();
        let obj24 = new_obj();
        let obj25 = new_obj();
        let obj26 = new_obj();
        let obj27 = new_obj();
        let obj28 = new_obj();
        let obj29 = new_obj();
        let obj30 = new_obj();

        let _obj00 = obj00.clone();
        let _obj01 = obj01.clone();
        let _obj02 = obj02.clone();
        let _obj03 = obj03.clone();
        let _obj04 = obj04.clone();
        let _obj05 = obj05.clone();
        let _obj06 = obj06.clone();
        let _obj07 = obj07.clone();
        let _obj08 = obj08.clone();
        let _obj09 = obj09.clone();
        let _obj10 = obj10.clone();
        let _obj11 = obj11.clone();
        let _obj12 = obj12.clone();
        let _obj13 = obj13.clone();
        let _obj14 = obj14.clone();
        let _obj15 = obj15.clone();
        let _obj16 = obj16.clone();
        let _obj17 = obj17.clone();
        let _obj18 = obj18.clone();
        let _obj19 = obj19.clone();
        let _obj20 = obj20.clone();
        let _obj21 = obj21.clone();
        let _obj22 = obj22.clone();
        let _obj23 = obj23.clone();
        let _obj24 = obj24.clone();
        let _obj25 = obj25.clone();
        let _obj26 = obj26.clone();
        let _obj27 = obj27.clone();
        let _obj28 = obj28.clone();
        let _obj29 = obj29.clone();
        let _obj30 = obj30.clone();
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn move_to_register_x16() {
        let obj = NSObject::new();
        let mut ptr = Retained::into_raw(obj);
        unsafe { core::arch::asm!("nop", inout("x16") ptr) };
        let _ = unsafe { Retained::from_raw(ptr) };

        let obj = NSObject::new();
        let mut ptr = Retained::autorelease_ptr(obj);
        unsafe { core::arch::asm!("nop", inout("x16") ptr) };
        let _ = unsafe { Retained::retain(ptr) };
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn move_to_register_x17() {
        let obj = NSObject::new();
        let mut ptr = Retained::into_raw(obj);
        unsafe { core::arch::asm!("nop", inout("x17") ptr) };
        let _ = unsafe { Retained::from_raw(ptr) };

        let obj = NSObject::new();
        let mut ptr = Retained::autorelease_ptr(obj);
        unsafe { core::arch::asm!("nop", inout("x17") ptr) };
        let _ = unsafe { Retained::retain(ptr) };
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn move_to_register_x30() {
        let obj = NSObject::new();
        let mut ptr = Retained::into_raw(obj);
        unsafe { core::arch::asm!("nop", inout("x30") ptr) };
        let _ = unsafe { Retained::from_raw(ptr) };

        let obj = NSObject::new();
        let mut ptr = Retained::autorelease_ptr(obj);
        unsafe { core::arch::asm!("nop", inout("x30") ptr) };
        let _ = unsafe { Retained::retain(ptr) };
    }
}
