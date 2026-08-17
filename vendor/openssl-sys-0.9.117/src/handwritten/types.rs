use libc::size_t;
use std::ffi::{c_char, c_int, c_uint, c_void};

#[allow(unused_imports)]
use super::super::*;

pub enum ASN1_OBJECT {}
pub enum ASN1_VALUE {}

pub type ASN1_BOOLEAN = c_int;
pub enum ASN1_INTEGER {}
pub enum ASN1_ENUMERATED {}
pub enum ASN1_GENERALIZEDTIME {}
pub enum ASN1_STRING {}
pub enum ASN1_BIT_STRING {}
pub enum ASN1_TIME {}
pub enum ASN1_OCTET_STRING {}
pub enum ASN1_NULL {}
pub enum ASN1_PRINTABLESTRING {}
pub enum ASN1_T61STRING {}
pub enum ASN1_IA5STRING {}
pub enum ASN1_GENERALSTRING {}
pub enum ASN1_BMPSTRING {}
pub enum ASN1_UNIVERSALSTRING {}
pub enum ASN1_UTCTIME {}
pub enum ASN1_VISIBLESTRING {}
pub enum ASN1_UTF8STRING {}

pub enum bio_st {} // FIXME remove
pub enum BIO {}
cfg_if! {
    if #[cfg(ossl320)] {
        pub enum BIO_ADDR {}
        pub enum BIO_POLL_DESCRIPTOR {}
        #[repr(C)]
        pub struct BIO_MSG {
            pub data: *mut c_void,
            pub data_len: usize,
            pub peer: *mut BIO_ADDR,
            pub local: *mut BIO_ADDR,
            pub flags: u64,
        }
    }
}
pub enum BIGNUM {}
pub enum BN_BLINDING {}
pub enum BN_MONT_CTX {}

pub enum BN_CTX {}
pub enum BN_GENCB {}

pub enum EVP_CIPHER {}
pub enum EVP_CIPHER_CTX {}
pub enum EVP_MD {}
pub enum EVP_MD_CTX {}

pub enum PKCS8_PRIV_KEY_INFO {}

pub enum EVP_PKEY_ASN1_METHOD {}

pub enum EVP_PKEY_CTX {}

pub enum CMAC_CTX {}

pub enum HMAC_CTX {}

pub enum DH {}
pub enum DH_METHOD {}

pub enum DSA {}
pub enum DSA_METHOD {}

pub enum RSA {}
pub enum RSA_METHOD {}

pub enum EC_KEY {}

pub enum X509 {}
cfg_if! {
    if #[cfg(any(ossl110, libressl382))] {
        pub enum X509_ALGOR {}
    } else {
        #[repr(C)]
        pub struct X509_ALGOR {
            pub algorithm: *mut ASN1_OBJECT,
            parameter: *mut c_void,
        }
    }
}

stack!(stack_st_X509_ALGOR);

pub enum X509_LOOKUP_METHOD {}

pub enum X509_NAME {}

pub enum X509_STORE {}

pub enum X509_STORE_CTX {}
pub enum X509_VERIFY_PARAM {}
pub enum X509_OBJECT {}
pub enum X509_LOOKUP {}

#[repr(C)]
pub struct X509V3_CTX {
    flags: c_int,
    issuer_cert: *mut c_void,
    subject_cert: *mut c_void,
    subject_req: *mut c_void,
    crl: *mut c_void,
    #[cfg(not(libressl400))]
    db_meth: *mut c_void,
    db: *mut c_void,
    #[cfg(ossl300)]
    issuer_pkey: *mut c_void,
    // I like the last comment line, it is copied from OpenSSL sources:
    // Maybe more here
}
pub enum CONF {}
#[cfg(ossl110)]
pub enum OPENSSL_INIT_SETTINGS {}

pub enum ENGINE {}
pub enum SSL {}
pub enum SSL_CTX {}

cfg_if! {
    if #[cfg(ossl320)] {
        #[repr(C)]
        pub struct SSL_CONN_CLOSE_INFO {
            pub error_code: u64,
            pub frame_type: u64,
            pub reason: *const c_char,
            pub reason_len: usize,
            pub flags: u32,
        }
        #[repr(C)]
        pub struct SSL_SHUTDOWN_EX_ARGS {
            pub quic_error_code: u64,
            pub quic_reason: *const c_char,
        }
        #[repr(C)]
        pub struct SSL_STREAM_RESET_ARGS {
            pub quic_error_code: u64,
        }
    }
}

pub enum COMP_CTX {}

#[cfg(not(osslconf = "OPENSSL_NO_COMP"))]
pub enum COMP_METHOD {}

pub enum CRYPTO_EX_DATA {}

pub enum OCSP_RESPONSE {}

#[cfg(ossl300)]
pub enum OSSL_PROVIDER {}

#[cfg(ossl300)]
pub enum OSSL_LIB_CTX {}

#[cfg(ossl300)]
#[repr(C)]
pub struct OSSL_PARAM {
    pub key: *const c_char,
    pub data_type: c_uint,
    pub data: *mut c_void,
    pub data_size: size_t,
    /// Number of bytes the most recent get-params call wrote into this
    /// parameter's data buffer. Only meaningful once
    /// [`crate::OSSL_PARAM_modified`] has confirmed the parameter was
    /// touched -- before that, this field still holds the sentinel set
    /// by the `OSSL_PARAM_construct_*` typed constructors.
    pub return_size: size_t,
}

#[cfg(ossl300)]
pub enum OSSL_PARAM_BLD {}

#[cfg(ossl300)]
pub enum EVP_KDF {}
#[cfg(ossl300)]
pub enum EVP_KDF_CTX {}

#[cfg(ossl300)]
pub enum OSSL_ENCODER_CTX {}
#[cfg(ossl300)]
pub enum OSSL_DECODER_CTX {}

#[cfg(ossl300)]
pub type OSSL_PASSPHRASE_CALLBACK = Option<
    unsafe extern "C" fn(
        pass: *mut c_char,
        pass_size: size_t,
        pass_len: *mut size_t,
        params: *const OSSL_PARAM,
        arg: *mut c_void,
    ) -> c_int,
>;

#[cfg(ossl300)]
pub enum EVP_MAC {}
#[cfg(ossl300)]
pub enum EVP_MAC_CTX {}
