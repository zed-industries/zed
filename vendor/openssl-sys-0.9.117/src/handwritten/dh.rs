use super::super::*;
use std::ffi::{c_int, c_long, c_uchar, c_void};

#[cfg(ossl300)]
extern "C" {
    pub fn EVP_PKEY_CTX_set_dh_paramgen_prime_len(ctx: *mut EVP_PKEY_CTX, len: c_int) -> c_int;
    pub fn EVP_PKEY_CTX_set_dh_paramgen_generator(ctx: *mut EVP_PKEY_CTX, gen: c_int) -> c_int;
}

#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
extern "C" {
    pub fn DH_new() -> *mut DH;
    pub fn DH_free(dh: *mut DH);
    pub fn DH_check(dh: *const DH, codes: *mut c_int) -> c_int;

    #[cfg(not(libressl382))]
    pub fn DH_generate_parameters(
        prime_len: c_int,
        generator: c_int,
        callback: Option<extern "C" fn(c_int, c_int, *mut c_void)>,
        cb_arg: *mut c_void,
    ) -> *mut DH;

    pub fn DH_generate_parameters_ex(
        dh: *mut DH,
        prime_len: c_int,
        generator: c_int,
        cb: *mut BN_GENCB,
    ) -> c_int;

    pub fn DH_generate_key(dh: *mut DH) -> c_int;
    pub fn DH_compute_key(key: *mut c_uchar, pub_key: *const BIGNUM, dh: *mut DH) -> c_int;
    pub fn DH_size(dh: *const DH) -> c_int;

    pub fn d2i_DHparams(k: *mut *mut DH, pp: *mut *const c_uchar, length: c_long) -> *mut DH;
    pub fn i2d_DHparams(dh: *const DH, pp: *mut *mut c_uchar) -> c_int;

    #[cfg(ossl110)]
    pub fn DH_get_1024_160() -> *mut DH;
    #[cfg(ossl110)]
    pub fn DH_get_2048_224() -> *mut DH;
    #[cfg(ossl110)]
    pub fn DH_get_2048_256() -> *mut DH;

    pub fn DH_set0_pqg(dh: *mut DH, p: *mut BIGNUM, q: *mut BIGNUM, g: *mut BIGNUM) -> c_int;
    pub fn DH_get0_pqg(
        dh: *const DH,
        p: *mut *const BIGNUM,
        q: *mut *const BIGNUM,
        g: *mut *const BIGNUM,
    );

    pub fn DH_set0_key(dh: *mut DH, pub_key: *mut BIGNUM, priv_key: *mut BIGNUM) -> c_int;
    pub fn DH_get0_key(dh: *const DH, pub_key: *mut *const BIGNUM, priv_key: *mut *const BIGNUM);
}
