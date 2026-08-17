use std::ffi::c_int;

pub const X509_FILETYPE_PEM: c_int = 1;
pub const X509_FILETYPE_ASN1: c_int = 2;
pub const X509_FILETYPE_DEFAULT: c_int = 3;

pub const ASN1_R_HEADER_TOO_LONG: c_int = 123;
