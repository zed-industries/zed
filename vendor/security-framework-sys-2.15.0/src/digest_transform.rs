use core_foundation_sys::base::{CFIndex, CFTypeRef};
use core_foundation_sys::error::CFErrorRef;
use core_foundation_sys::string::CFStringRef;

use crate::transform::SecTransformRef;

extern "C" {
    pub static kSecDigestHMACKeyAttribute: CFStringRef;
    pub static kSecDigestHMACMD5: CFStringRef;
    pub static kSecDigestHMACSHA1: CFStringRef;
    pub static kSecDigestHMACSHA2: CFStringRef;
    pub static kSecDigestLengthAttribute: CFStringRef;
    pub static kSecDigestMD2: CFStringRef;
    pub static kSecDigestMD4: CFStringRef;
    pub static kSecDigestMD5: CFStringRef;
    pub static kSecDigestSHA1: CFStringRef;
    pub static kSecDigestSHA2: CFStringRef;
    pub static kSecDigestTypeAttribute: CFStringRef;

    pub fn SecDigestTransformCreate(
        digestType: CFTypeRef,
        digestLength: CFIndex,
        error: *mut CFErrorRef,
    ) -> SecTransformRef;

// this symbol is apparently missing in 10.13.3?
// pub fn SecDigestTransformGetTypeID() -> CFTypeID;
}
