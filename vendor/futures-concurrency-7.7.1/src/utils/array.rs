use core::mem::{self, MaybeUninit};

/// Extracts the values from an array of `MaybeUninit` containers.
///
/// # Safety
///
/// It is up to the caller to guarantee that all elements of the array are
/// in an initialized state.
///
/// Inlined version of: <https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.array_assume_init>
pub(crate) unsafe fn array_assume_init<T, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    // SAFETY:
    // * The caller guarantees that all elements of the array are initialized
    // * `MaybeUninit<T>` and T are guaranteed to have the same layout
    // * `MaybeUninit` does not drop, so there are no double-frees
    // And thus the conversion is safe
    let ret = unsafe { (&array as *const _ as *const [T; N]).read() };

    // FIXME: required to avoid `~const Destruct` bound
    #[allow(clippy::forget_non_drop)]
    mem::forget(array);
    ret
}
