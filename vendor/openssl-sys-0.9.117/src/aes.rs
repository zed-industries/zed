use std::ffi::c_int;

#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
pub const AES_ENCRYPT: c_int = 1;
#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
pub const AES_DECRYPT: c_int = 0;

#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
pub const AES_MAXNR: c_int = 14;
pub const AES_BLOCK_SIZE: c_int = 16;
