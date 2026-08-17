use super::super::*;
use libc::{size_t, FILE};
use std::ffi::{c_char, c_int, c_uchar, c_void};

extern "C" {
    pub fn OSSL_DECODER_CTX_new() -> *mut OSSL_DECODER_CTX;
    pub fn OSSL_DECODER_CTX_free(ctx: *mut OSSL_DECODER_CTX);

    pub fn OSSL_DECODER_CTX_new_for_pkey(
        pkey: *mut *mut EVP_PKEY,
        input_type: *const c_char,
        input_struct: *const c_char,
        keytype: *const c_char,
        selection: c_int,
        libctx: *mut OSSL_LIB_CTX,
        propquery: *const c_char,
    ) -> *mut OSSL_DECODER_CTX;

    pub fn OSSL_DECODER_CTX_set_selection(ctx: *mut OSSL_DECODER_CTX, selection: c_int) -> c_int;
    pub fn OSSL_DECODER_CTX_set_input_type(
        ctx: *mut OSSL_DECODER_CTX,
        input_type: *const c_char,
    ) -> c_int;
    pub fn OSSL_DECODER_CTX_set_input_structure(
        ctx: *mut OSSL_DECODER_CTX,
        input_structure: *const c_char,
    ) -> c_int;

    pub fn OSSL_DECODER_CTX_set_passphrase(
        ctx: *mut OSSL_DECODER_CTX,
        kstr: *const c_uchar,
        klen: size_t,
    ) -> c_int;
    pub fn OSSL_DECODER_CTX_set_pem_password_cb(
        ctx: *mut OSSL_DECODER_CTX,
        cb: pem_password_cb,
        cbarg: *mut c_void,
    ) -> c_int;
    pub fn OSSL_DECODER_CTX_set_passphrase_cb(
        ctx: *mut OSSL_DECODER_CTX,
        cb: OSSL_PASSPHRASE_CALLBACK,
        cbarg: *mut c_void,
    ) -> c_int;

    pub fn OSSL_DECODER_from_bio(ctx: *mut OSSL_DECODER_CTX, b_in: *mut BIO) -> c_int;
    #[cfg(not(osslconf = "OPENSSL_NO_STDIO"))]
    pub fn OSSL_DECODER_from_fp(ctx: *mut OSSL_DECODER_CTX, fp: *mut FILE) -> c_int;
    pub fn OSSL_DECODER_from_data(
        ctx: *mut OSSL_DECODER_CTX,
        pdata: *mut *const c_uchar,
        pdata_len: *mut size_t,
    ) -> c_int;
}
