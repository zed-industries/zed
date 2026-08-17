use crate::prelude::*;

// The below configuration for machines with 32-bit word size aligns with the
// declaration in the `mingw-w64` headers.
cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        s_no_extra_traits! {
            #[repr(align(16))]
            pub struct max_align_t {
                priv_: [f64; 4],
            }
        }
    } else if #[cfg(target_pointer_width = "32")] {
        s_no_extra_traits! {
            #[repr(align(8))]
            pub struct max_align_t {
                priv_: [i64; 3],
            }
        }
    }
}

// stdio file descriptor numbers
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

extern "C" {
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;

    // NOTE: For MSVC target, `wmemchr` is only a inline function in `<wchar.h>`
    //      header file. We cannot find a way to link to that symbol from Rust.
    pub fn wmemchr(cx: *const crate::wchar_t, c: crate::wchar_t, n: size_t) -> *mut crate::wchar_t;
}
