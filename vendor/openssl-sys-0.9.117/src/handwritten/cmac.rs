use libc::size_t;
use std::ffi::{c_int, c_uchar, c_void};

use super::super::*;

#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
extern "C" {
    pub fn CMAC_CTX_new() -> *mut CMAC_CTX;
    pub fn CMAC_CTX_free(ctx: *mut CMAC_CTX);
    pub fn CMAC_Init(
        ctx: *mut CMAC_CTX,
        key: *const c_void,
        len: size_t,
        cipher: *const EVP_CIPHER,
        impl_: *mut ENGINE,
    ) -> c_int;
    pub fn CMAC_Update(ctx: *mut CMAC_CTX, data: *const c_void, len: size_t) -> c_int;
    pub fn CMAC_Final(ctx: *mut CMAC_CTX, out: *mut c_uchar, len: *mut size_t) -> c_int;
    pub fn CMAC_CTX_copy(dst: *mut CMAC_CTX, src: *const CMAC_CTX) -> c_int;
}
