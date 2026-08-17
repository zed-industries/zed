use core::mem::MaybeUninit;

/// Unpoisons `buf` if MSAN support is enabled.
///
/// Most backends do not need to unpoison their output. Rust language- and
/// library- provided functionality unpoisons automatically. Similarly, libc
/// either natively supports MSAN and/or MSAN hooks libc-provided functions
/// to unpoison outputs on success. Only when all of these things are
/// bypassed do we need to do it ourselves.
///
/// The call to unpoison should be done as close to the write as possible.
/// For example, if the backend partially fills the output buffer in chunks,
/// each chunk should be unpoisoned individually. This way, the correctness of
/// the chunking logic can be validated (in part) using MSAN.
pub unsafe fn unpoison(buf: &mut [MaybeUninit<u8>]) {
    cfg_if! {
        if #[cfg(getrandom_msan)] {
            unsafe extern "C" {
                fn __msan_unpoison(a: *mut core::ffi::c_void, size: usize);
            }
            let a = buf.as_mut_ptr().cast();
            let size = buf.len();
            unsafe { __msan_unpoison(a, size) };
        } else {
            let _ = buf;
        }
    }
}
