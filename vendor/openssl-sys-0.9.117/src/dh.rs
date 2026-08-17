use std::ffi::c_int;
use std::ptr;

use super::super::*;

cfg_if! {
    if #[cfg(not(ossl300))] {
        pub unsafe fn EVP_PKEY_CTX_set_dh_paramgen_prime_len(ctx: *mut EVP_PKEY_CTX, len: c_int) -> c_int {
            EVP_PKEY_CTX_ctrl(
                ctx,
                EVP_PKEY_DH,
                EVP_PKEY_OP_PARAMGEN,
                EVP_PKEY_CTRL_DH_PARAMGEN_PRIME_LEN,
                len,
                ptr::null_mut(),
            )
        }
        pub unsafe fn EVP_PKEY_CTX_set_dh_paramgen_generator(ctx: *mut EVP_PKEY_CTX, gen: c_int) -> c_int {
            EVP_PKEY_CTX_ctrl(
                ctx,
                EVP_PKEY_DH,
                EVP_PKEY_OP_PARAMGEN,
                EVP_PKEY_CTRL_DH_PARAMGEN_GENERATOR,
                gen,
                ptr::null_mut(),
            )
        }
    }
}

pub const EVP_PKEY_CTRL_DH_PARAMGEN_PRIME_LEN: c_int = EVP_PKEY_ALG_CTRL + 1;
pub const EVP_PKEY_CTRL_DH_PARAMGEN_GENERATOR: c_int = EVP_PKEY_ALG_CTRL + 2;
