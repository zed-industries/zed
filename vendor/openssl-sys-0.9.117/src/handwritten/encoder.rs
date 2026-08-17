use super::super::*;
use libc::{size_t, FILE};
use std::ffi::{c_char, c_int, c_uchar, c_void};

#[cfg(ossl300)]
extern "C" {
    pub fn OSSL_ENCODER_CTX_new() -> *mut OSSL_ENCODER_CTX;
    pub fn OSSL_ENCODER_CTX_free(ctx: *mut OSSL_ENCODER_CTX);

    pub fn OSSL_ENCODER_CTX_new_for_pkey(
        pkey: *const EVP_PKEY,
        selection: c_int,
        output_type: *const c_char,
        output_structure: *const c_char,
        propquery: *const c_char,
    ) -> *mut OSSL_ENCODER_CTX;

    pub fn OSSL_ENCODER_CTX_set_selection(ctx: *mut OSSL_ENCODER_CTX, selection: c_int) -> c_int;
    pub fn OSSL_ENCODER_CTX_set_output_type(
        ctx: *mut OSSL_ENCODER_CTX,
        output_type: *const c_char,
    ) -> c_int;
    pub fn OSSL_ENCODER_CTX_set_output_structure(
        ctx: *mut OSSL_ENCODER_CTX,
        output_structure: *const c_char,
    ) -> c_int;

    pub fn OSSL_ENCODER_CTX_set_cipher(
        ctx: *mut OSSL_ENCODER_CTX,
        cipher_name: *const c_char,
        propquery: *const c_char,
    ) -> c_int;
    pub fn OSSL_ENCODER_CTX_set_passphrase(
        ctx: *mut OSSL_ENCODER_CTX,
        kstr: *const c_uchar,
        klen: size_t,
    ) -> c_int;
    pub fn OSSL_ENCODER_CTX_set_pem_password_cb(
        ctx: *mut OSSL_ENCODER_CTX,
        cb: pem_password_cb,
        cbarg: *mut c_void,
    ) -> c_int;
    pub fn OSSL_ENCODER_CTX_set_passphrase_cb(
        ctx: *mut OSSL_ENCODER_CTX,
        cb: OSSL_PASSPHRASE_CALLBACK,
        cbarg: *mut c_void,
    ) -> c_int;

    pub fn OSSL_ENCODER_to_data(
        ctx: *mut OSSL_ENCODER_CTX,
        pdata: *mut *mut c_uchar,
        pdata_len: *mut size_t,
    ) -> c_int;
    pub fn OSSL_ENCODER_to_bio(ctx: *mut OSSL_ENCODER_CTX, out: *mut BIO) -> c_int;
    #[cfg(not(osslconf = "OPENSSL_NO_STDIO"))]
    pub fn OSSL_ENCODER_to_fp(ctx: *mut OSSL_ENCODER_CTX, fp: *mut FILE) -> c_int;
}
