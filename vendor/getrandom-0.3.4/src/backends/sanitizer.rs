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
            extern "C" {
                fn __msan_unpoison(a: *mut core::ffi::c_void, size: usize);
            }
            let a = buf.as_mut_ptr().cast();
            let size = buf.len();
            #[allow(unused_unsafe)] // TODO(MSRV 1.65): Remove this.
            unsafe {
                __msan_unpoison(a, size);
            }
        } else {
            let _ = buf;
        }
    }
}

/// Interprets the result of the `getrandom` syscall of Linux, unpoisoning any
/// written part of `buf`.
///
/// `buf` must be the output buffer that was originally passed to the `getrandom`
/// syscall.
///
/// `ret` must be the result returned by `getrandom`. If `ret` is negative or
/// larger than the length of `buf` then nothing is done.
///
/// Memory Sanitizer only intercepts `getrandom` on this condition (from its
/// source code):
/// ```c
/// #define SANITIZER_INTERCEPT_GETRANDOM \
///   ((SI_LINUX && __GLIBC_PREREQ(2, 25)) || SI_FREEBSD || SI_SOLARIS)
/// ```
/// So, effectively, we have to assume that it is never intercepted on Linux.
#[cfg(any(target_os = "android", target_os = "linux"))]
pub unsafe fn unpoison_linux_getrandom_result(buf: &mut [MaybeUninit<u8>], ret: isize) {
    if let Ok(bytes_written) = usize::try_from(ret) {
        if let Some(written) = buf.get_mut(..bytes_written) {
            #[allow(unused_unsafe)] // TODO(MSRV 1.65): Remove this.
            unsafe {
                unpoison(written)
            }
        }
    }
}
