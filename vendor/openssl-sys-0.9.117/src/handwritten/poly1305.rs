use super::super::*;
use std::ffi::c_uchar;

cfg_if! {
    if #[cfg(libressl)] {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct poly1305_context {
            pub aligner: usize,
            pub opaque: [c_uchar; 136usize],
        }
        pub type poly1305_state = poly1305_context;
        extern "C" {
            pub fn CRYPTO_poly1305_init(ctx: *mut poly1305_context, key: *const c_uchar);
            pub fn CRYPTO_poly1305_update(
                ctx: *mut poly1305_context,
                in_: *const c_uchar,
                len: usize,
            );
            pub fn CRYPTO_poly1305_finish(ctx: *mut poly1305_context, mac: *mut c_uchar);
        }
    }
}
