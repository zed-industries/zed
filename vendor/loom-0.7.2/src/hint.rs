//! Mocked versions of [`std::hint`] functions.

/// Signals the processor that it is entering a busy-wait spin-loop.
pub fn spin_loop() {
    crate::sync::atomic::spin_loop_hint();
}

/// Informs the compiler that this point in the code is not reachable, enabling
/// further optimizations.
///
/// This is a mocked version of the standard library's
/// [`std::hint::unreachable_unchecked`]. Loom's wrapper of this function
/// unconditionally panics.
///
/// # Safety
///
/// Technically, this function is safe to call (unlike the standard library's
/// version), as it always panics rather than invoking UB. However, this
/// function is marked as `unsafe` because it's intended to be used as a
/// simulated version of [`std::hint::unreachable_unchecked`], which is unsafe.
///
/// See [the documentation for
/// `std::hint::unreachable_unchecked`](`std::hint::unreachable_unchecked#Safety)
/// for safety details.
#[track_caller]
pub unsafe fn unreachable_unchecked() -> ! {
    unreachable!("unreachable_unchecked was reached!");
}
