#![allow(non_upper_case_globals)]

pub use screencapturekit_sys::os_types::base::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFDictionary {
    _unused: [u8; 0],
}

pub type CFDictionaryRef = *const __CFDictionary;
pub type CFTypeRef = *const ::std::os::raw::c_void;
pub type CFIndex = ::std::os::raw::c_long;
pub type CFNumberType = CFIndex;
pub type Boolean = ::std::os::raw::c_uchar;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNumber {
    _unused: [u8; 0],
}

#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSString(pub id);

pub type CFNumberRef = *const __CFNumber;
pub type SCStreamFrameInfo = NSString;
extern "C" {
    pub fn CFDictionaryGetValue(
        theDict: CFDictionaryRef,
        key: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_void;
    pub fn CFNumberGetValue(
        number: CFNumberRef,
        theType: CFNumberType,
        valuePtr: *mut ::std::os::raw::c_void,
    ) -> Boolean;
    pub fn CMTimeGetSeconds(time: CMTime) -> Float64;
    pub static SCStreamFrameInfoStatus: SCStreamFrameInfo;
}
pub const CFNumberType_kCFNumberSInt64Type: CFNumberType = 4;
pub type NSInteger = ::std::os::raw::c_long;
pub type SCFrameStatus = NSInteger;
pub const SCFrameStatus_SCFrameStatusComplete: SCFrameStatus = 0;
