#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]

use super::four_char_code::FourCharCode;
pub type VoidPtr = *mut ::std::ffi::c_void;
pub type UInt8 = ::std::ffi::c_uchar;
pub type UInt16 = ::std::ffi::c_ushort;
pub type SInt8 = ::std::ffi::c_schar;
pub type SInt16 = ::std::ffi::c_short;
pub type UInt32 = ::std::ffi::c_uint;
pub type SInt32 = ::std::ffi::c_int;
pub type SInt64 = ::std::ffi::c_longlong;
pub type UInt64 = ::std::ffi::c_ulonglong;
pub type Float32 = ::std::ffi::c_float;
pub type Float64 = ::std::ffi::c_double;
pub type CGFloat = ::std::ffi::c_double;
pub type Size = ::std::ffi::c_long;
pub type BOOL = ::std::ffi::c_schar;
pub type OSType = FourCharCode;
pub type Boolean = ::std::ffi::c_uchar;
pub type SizeT = ::std::ffi::c_ulong;
pub type PidT = ::std::ffi::c_int;
pub type CMTimeValue = SInt64;
pub type CMTimeScale = SInt32;
pub type CMTimeEpoch = SInt64;
pub type CVPixelBufferLockFlags = UInt64;
pub type CVReturn = SInt32;
pub const CMTIME_FLAGS_VALID: CMTimeFlags = 1;
pub const CMTIME_FLAGS_HAS_BEEN_ROUNDED: CMTimeFlags = 2;
pub const CMTIME_FLAGS_POSITIVE_INFINITY: CMTimeFlags = 4;
pub const CMTIME_FLAGS_NEGATIVE_INFINITY: CMTimeFlags = 8;
pub const CMTIME_FLAGS_INDEFINITE: CMTimeFlags = 16;
pub const CMTIME_FLAGS_IMPLIED_VALUE_FLAGS_MASK: CMTimeFlags = 28;
pub type CMTimeFlags = UInt32;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}
