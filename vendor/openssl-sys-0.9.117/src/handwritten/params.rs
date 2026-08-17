use super::super::*;
use libc::size_t;
use std::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    pub fn OSSL_PARAM_free(p: *mut OSSL_PARAM);
    pub fn OSSL_PARAM_dup(params: *const OSSL_PARAM) -> *mut OSSL_PARAM;
    pub fn OSSL_PARAM_merge(
        params: *const OSSL_PARAM,
        params1: *const OSSL_PARAM,
    ) -> *mut OSSL_PARAM;
    pub fn OSSL_PARAM_construct_uint(key: *const c_char, buf: *mut c_uint) -> OSSL_PARAM;
    pub fn OSSL_PARAM_construct_end() -> OSSL_PARAM;
    pub fn OSSL_PARAM_construct_octet_string(
        key: *const c_char,
        buf: *mut c_void,
        bsize: size_t,
    ) -> OSSL_PARAM;

    pub fn OSSL_PARAM_modified(p: *const OSSL_PARAM) -> c_int;

    pub fn OSSL_PARAM_locate(p: *mut OSSL_PARAM, key: *const c_char) -> *mut OSSL_PARAM;
    pub fn OSSL_PARAM_locate_const(
        params: *const OSSL_PARAM,
        key: *const c_char,
    ) -> *const OSSL_PARAM;
    pub fn OSSL_PARAM_get_BN(p: *const OSSL_PARAM, val: *mut *mut BIGNUM) -> c_int;
    pub fn OSSL_PARAM_get_utf8_string(
        p: *const OSSL_PARAM,
        val: *mut *mut c_char,
        max_len: usize,
    ) -> c_int;
    pub fn OSSL_PARAM_get_utf8_string_ptr(p: *const OSSL_PARAM, val: *mut *const c_char) -> c_int;
    pub fn OSSL_PARAM_get_octet_string(
        p: *const OSSL_PARAM,
        val: *mut *mut c_void,
        max_len: usize,
        used_len: *mut usize,
    ) -> c_int;
    pub fn OSSL_PARAM_get_octet_string_ptr(
        p: *const OSSL_PARAM,
        val: *mut *const c_void,
        used_len: *mut usize,
    ) -> c_int;

    pub fn OSSL_PARAM_BLD_new() -> *mut OSSL_PARAM_BLD;
    pub fn OSSL_PARAM_BLD_free(bld: *mut OSSL_PARAM_BLD);
    pub fn OSSL_PARAM_BLD_to_param(bld: *mut OSSL_PARAM_BLD) -> *mut OSSL_PARAM;
    pub fn OSSL_PARAM_BLD_push_int(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        val: c_int,
    ) -> c_int;
    pub fn OSSL_PARAM_BLD_push_uint(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        val: c_uint,
    ) -> c_int;
    pub fn OSSL_PARAM_BLD_push_size_t(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        val: size_t,
    ) -> c_int;
    pub fn OSSL_PARAM_BLD_push_BN(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        bn: *const BIGNUM,
    ) -> c_int;
    pub fn OSSL_PARAM_BLD_push_BN_pad(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        bn: *const BIGNUM,
        sz: size_t,
    ) -> c_int;
    pub fn OSSL_PARAM_BLD_push_utf8_string(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        buf: *const c_char,
        bsize: size_t,
    ) -> c_int;
    pub fn OSSL_PARAM_BLD_push_utf8_ptr(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        buf: *mut c_char,
        bsize: size_t,
    ) -> c_int;
    pub fn OSSL_PARAM_BLD_push_octet_string(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        buf: *const c_void,
        bsize: size_t,
    ) -> c_int;
    pub fn OSSL_PARAM_BLD_push_octet_ptr(
        bld: *mut OSSL_PARAM_BLD,
        key: *const c_char,
        buf: *mut c_void,
        bsize: size_t,
    ) -> c_int;
}
