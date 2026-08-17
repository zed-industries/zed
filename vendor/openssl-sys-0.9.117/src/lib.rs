#![allow(
    clippy::missing_safety_doc,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports
)]
#![doc(html_root_url = "https://docs.rs/openssl-sys/0.9")]
#![recursion_limit = "128"] // configure fixed limit across all rust versions

extern crate libc;
pub use std::ffi::c_int;

#[cfg(feature = "unstable_boringssl")]
extern crate bssl_sys;

#[cfg(boringssl)]
#[path = "."]
mod boringssl {
    #[cfg(feature = "unstable_boringssl")]
    pub use bssl_sys::*;
    #[cfg(not(feature = "unstable_boringssl"))]
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));

    // BoringSSL does not require initialization.
    pub fn init() {}
}
#[cfg(boringssl)]
pub use boringssl::*;

#[cfg(feature = "aws-lc")]
extern crate aws_lc_sys;

#[cfg(awslc)]
#[path = "."]
#[allow(unpredictable_function_pointer_comparisons)]
mod aws_lc {
    #[cfg(all(feature = "aws-lc", not(feature = "aws-lc-fips")))]
    pub use aws_lc_sys::*;

    #[cfg(feature = "aws-lc-fips")]
    pub use aws_lc_fips_sys::*;

    #[cfg(not(any(feature = "aws-lc", feature = "aws-lc-fips")))]
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));

    use std::ffi::{c_char, c_int, c_long, c_uint, c_void};

    pub fn init() {
        unsafe { CRYPTO_library_init() }
    }

    // BIO_get_mem_data is a C preprocessor macro by definition
    #[allow(non_snake_case)]
    pub unsafe fn BIO_get_mem_data(b: *mut BIO, pp: *mut *mut c_char) -> c_long {
        unsafe { BIO_ctrl(b, BIO_CTRL_INFO, 0, pp.cast::<c_void>()) }
    }

    // ERR_GET_{LIB,REASON,FUNC} are macros/static inlines in AWS-LC and
    // therefore not emitted by pregenerated bindings. We provide pure-Rust
    // implementations matching the logic in aws-lc-sys.
    //
    // When aws-lc-sys is used (feature = "aws-lc" or "aws-lc-fips"), these
    // come from the glob import instead. When normal bindgen runs
    // (wrap_static_fns), they're in the generated output.
    #[cfg(awslc_pregenerated)]
    #[allow(non_snake_case, clippy::cast_possible_wrap)]
    pub fn ERR_GET_LIB(packed_error: c_uint) -> c_int {
        ((packed_error >> 24) & 0xFF) as c_int
    }

    #[cfg(awslc_pregenerated)]
    #[allow(non_snake_case, clippy::cast_possible_wrap)]
    pub fn ERR_GET_REASON(packed_error: c_uint) -> c_int {
        (packed_error & 0xFFF) as c_int
    }

    #[cfg(awslc_pregenerated)]
    #[allow(non_snake_case)]
    pub fn ERR_GET_FUNC(_packed_error: c_uint) -> c_int {
        0
    }
}
#[cfg(awslc)]
pub use aws_lc::*;

#[cfg(openssl)]
#[path = "."]
mod openssl {
    use std::ffi::{c_char, c_int, c_void};

    #[cfg(feature = "bindgen")]
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));

    pub use self::aes::*;
    pub use self::asn1::*;
    pub use self::bio::*;
    pub use self::bn::*;
    pub use self::cms::*;
    #[cfg(ossl300)]
    pub use self::core_dispatch::*;
    pub use self::crypto::*;
    pub use self::dh::*;
    pub use self::dsa::*;
    pub use self::dtls1::*;
    pub use self::ec::*;
    pub use self::err::*;
    pub use self::evp::*;
    #[cfg(not(feature = "bindgen"))]
    pub use self::handwritten::*;
    pub use self::obj_mac::*;
    pub use self::ocsp::*;
    pub use self::pem::*;
    pub use self::pkcs7::*;
    pub use self::rsa::*;
    pub use self::sha::*;
    pub use self::srtp::*;
    pub use self::ssl::*;
    pub use self::ssl3::*;
    pub use self::tls1::*;
    pub use self::types::*;
    pub use self::x509::*;
    pub use self::x509_vfy::*;
    pub use self::x509v3::*;

    #[macro_use]
    mod macros;

    mod aes;
    mod asn1;
    mod bio;
    mod bn;
    mod cms;
    #[cfg(ossl300)]
    mod core_dispatch;
    mod crypto;
    mod dh;
    mod dsa;
    mod dtls1;
    mod ec;
    mod err;
    mod evp;
    #[cfg(not(feature = "bindgen"))]
    mod handwritten;
    mod obj_mac;
    mod ocsp;
    mod pem;
    mod pkcs7;
    mod rsa;
    mod sha;
    mod srtp;
    mod ssl;
    mod ssl3;
    mod tls1;
    mod types;
    mod x509;
    mod x509_vfy;
    mod x509v3;

    use std::sync::Once;
    // explicitly initialize to work around https://github.com/openssl/openssl/issues/3505
    static INIT: Once = Once::new();

    // FIXME remove
    pub type PasswordCallback = unsafe extern "C" fn(
        buf: *mut c_char,
        size: c_int,
        rwflag: c_int,
        user_data: *mut c_void,
    ) -> c_int;

    #[cfg(ossl110)]
    pub fn init() {
        use std::ptr;

        #[cfg(not(ossl111b))]
        let init_options = OPENSSL_INIT_LOAD_SSL_STRINGS;
        #[cfg(ossl111b)]
        let init_options = OPENSSL_INIT_LOAD_SSL_STRINGS | OPENSSL_INIT_NO_ATEXIT;

        INIT.call_once(|| unsafe {
            OPENSSL_init_ssl(init_options, ptr::null_mut());
        })
    }

    #[cfg(libressl)]
    pub fn init() {}

    /// Disable explicit initialization of the openssl libs.
    ///
    /// This is only appropriate to use if the openssl crate is being consumed by an application
    /// that will be performing the initialization explicitly.
    ///
    /// # Safety
    ///
    /// In some versions of openssl, skipping initialization will fall back to the default procedure
    /// while other will cause difficult to debug errors so care must be taken when calling this.
    pub unsafe fn assume_init() {
        INIT.call_once(|| {});
    }
}
#[cfg(openssl)]
pub use openssl::*;
