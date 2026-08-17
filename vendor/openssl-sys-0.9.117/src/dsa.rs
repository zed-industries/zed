use std::ffi::c_int;
use std::ptr;

use super::super::*;

cfg_if! {
    if #[cfg(not(ossl300))] {
        pub unsafe fn EVP_PKEY_CTX_set_dsa_paramgen_bits(ctx: *mut EVP_PKEY_CTX, nbits: c_int) -> c_int {
            EVP_PKEY_CTX_ctrl(
                ctx,
                EVP_PKEY_DSA,
                EVP_PKEY_OP_PARAMGEN,
                EVP_PKEY_CTRL_DSA_PARAMGEN_BITS,
                nbits,
                ptr::null_mut(),
            )
        }

        pub const EVP_PKEY_CTRL_DSA_PARAMGEN_BITS: c_int = EVP_PKEY_ALG_CTRL + 1;
    }
}
