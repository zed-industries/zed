use std::ffi::{c_char, c_int};
use std::ptr;

use super::*;

pub const OPENSSL_EC_NAMED_CURVE: c_int = 1;

cfg_if! {
    if #[cfg(not(ossl300))] {
        pub unsafe fn EVP_PKEY_CTX_set_ec_paramgen_curve_nid(ctx: *mut EVP_PKEY_CTX, nid: c_int) -> c_int {
            EVP_PKEY_CTX_ctrl(
                ctx,
                EVP_PKEY_EC,
                EVP_PKEY_OP_PARAMGEN|EVP_PKEY_OP_KEYGEN,
                EVP_PKEY_CTRL_EC_PARAMGEN_CURVE_NID,
                nid,
                ptr::null_mut(),
            )
        }
    }
}
#[cfg(ossl300)]
pub unsafe fn EVP_EC_gen(curve: *const c_char) -> *mut EVP_PKEY {
    EVP_PKEY_Q_keygen(ptr::null_mut(), ptr::null_mut(), c"EC".as_ptr(), curve)
}

pub const EVP_PKEY_CTRL_EC_PARAMGEN_CURVE_NID: c_int = EVP_PKEY_ALG_CTRL + 1;
