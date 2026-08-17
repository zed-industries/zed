use std::ptr::{null, null_mut};

use core_foundation::{
    base::{CFType, CFTypeID, TCFType},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::c_void;

use crate::color_space::{CGColorSpace, CGColorSpaceRef};

#[repr(C)]
pub struct __CGColorConversionInfo(c_void);

pub type CGColorConversionInfoRef = *const __CGColorConversionInfo;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGColorConversionInfoTransformType {
    #[doc(alias = "kCGColorConversionTransformFromSpace")]
    FromSpace  = 0,
    #[doc(alias = "kCGColorConversionTransformToSpace")]
    ToSpace    = 1,
    #[doc(alias = "kCGColorConversionTransformApplySpace")]
    ApplySpace = 2,
}

extern "C" {
    pub fn CGColorConversionInfoGetTypeID() -> CFTypeID;
    pub fn CGColorConversionInfoCreate(src: CGColorSpaceRef, dst: CGColorSpaceRef) -> CGColorConversionInfoRef;
    pub fn CGColorConversionInfoCreateWithOptions(src: CGColorSpaceRef, dst: CGColorSpaceRef, options: CFDictionaryRef) -> CGColorConversionInfoRef;

    pub static kCGColorConversionBlackPointCompensation: CFStringRef;
    pub static kCGColorConversionTRCSize: CFStringRef;
}

declare_TCFType!(CGColorConversionInfo, CGColorConversionInfoRef);
impl_TCFType!(CGColorConversionInfo, CGColorConversionInfoRef, CGColorConversionInfoGetTypeID);
impl_CFTypeDescription!(CGColorConversionInfo);

impl CGColorConversionInfo {
    pub fn new(src: Option<&CGColorSpace>, dst: Option<&CGColorSpace>) -> Option<CGColorConversionInfo> {
        unsafe {
            let info =
                CGColorConversionInfoCreate(src.map_or(null_mut(), |s| s.as_concrete_TypeRef()), dst.map_or(null_mut(), |d| d.as_concrete_TypeRef()));
            if info.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(info))
            }
        }
    }

    pub fn new_with_options(
        src: Option<&CGColorSpace>,
        dst: Option<&CGColorSpace>,
        options: Option<&CFDictionary<CFString, CFType>>,
    ) -> Option<CGColorConversionInfo> {
        unsafe {
            let info = CGColorConversionInfoCreateWithOptions(
                src.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                dst.map_or(null_mut(), |d| d.as_concrete_TypeRef()),
                options.map_or(null(), |o| o.as_concrete_TypeRef()),
            );
            if info.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(info))
            }
        }
    }
}

pub enum CGColorConversionKeys {
    BlackPointCompensation,
    TRCSize,
}

impl From<CGColorConversionKeys> for CFStringRef {
    fn from(key: CGColorConversionKeys) -> Self {
        unsafe {
            match key {
                CGColorConversionKeys::BlackPointCompensation => kCGColorConversionBlackPointCompensation,
                CGColorConversionKeys::TRCSize => kCGColorConversionTRCSize,
            }
        }
    }
}

impl From<CGColorConversionKeys> for CFString {
    fn from(key: CGColorConversionKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}
