use super::*;
use libc::size_t;
use std::ffi::{c_int, c_uchar, c_uint, c_void};
use std::ptr;

#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
pub const SHA_LBLOCK: c_int = 16;

#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
pub type SHA_LONG = c_uint;

cfg_if! {
    if #[cfg(ossl300)] {
        #[cfg(ossl300)]
        // Ideally we'd macro define these, but that crashes ctest :(
        pub unsafe fn SHA1(d: *const c_uchar, n: size_t, md: *mut c_uchar) -> *mut c_uchar {
            if EVP_Q_digest(
                ptr::null_mut(),
                c"SHA1".as_ptr(),
                ptr::null(),
                d as *const c_void,
                n,
                md,
                ptr::null_mut(),
            ) != 0
            {
                md
            } else {
                ptr::null_mut()
            }
        }

        pub unsafe fn SHA224(d: *const c_uchar, n: size_t, md: *mut c_uchar) -> *mut c_uchar {
            if EVP_Q_digest(
                ptr::null_mut(),
                c"SHA224".as_ptr(),
                ptr::null(),
                d as *const c_void,
                n,
                md,
                ptr::null_mut(),
            ) != 0 {
                md
            } else {
                ptr::null_mut()
            }
        }

        pub unsafe fn SHA256(d: *const c_uchar, n: size_t, md: *mut c_uchar) -> *mut c_uchar {
            if EVP_Q_digest(
                ptr::null_mut(),
                c"SHA256".as_ptr(),
                ptr::null(),
                d as *const c_void,
                n,
                md,
                ptr::null_mut(),
            ) != 0 {
                md
            } else {
                ptr::null_mut()
            }
        }
    }
}

#[cfg(not(osslconf = "OPENSSL_NO_DEPRECATED_3_0"))]
pub type SHA_LONG64 = u64;

cfg_if! {
    if #[cfg(ossl300)] {
        pub unsafe fn SHA384(d: *const c_uchar, n: size_t, md: *mut c_uchar) -> *mut c_uchar {
            if EVP_Q_digest(
                ptr::null_mut(),
                c"SHA384".as_ptr(),
                ptr::null(),
                d as *const c_void,
                n,
                md,
                ptr::null_mut(),
            ) != 0 {
                md
            } else {
                ptr::null_mut()
            }
        }

        pub unsafe fn SHA512(d: *const c_uchar, n: size_t, md: *mut c_uchar) -> *mut c_uchar {
            if EVP_Q_digest(
                ptr::null_mut(),
                c"SHA512".as_ptr(),
                ptr::null(),
                d as *const c_void,
                n,
                md,
                ptr::null_mut(),
            ) != 0 {
                md
            } else {
                ptr::null_mut()
            }
        }
    }
}
