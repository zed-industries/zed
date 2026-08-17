use std::{ptr::null, slice::from_raw_parts};

use core_foundation::{
    base::{CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::{c_void, size_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{
    base::CGFloat,
    color_space::{CGColorRenderingIntent, CGColorSpace, CGColorSpaceRef},
    pattern::{CGPattern, CGPatternRef},
};

#[repr(C)]
pub struct __CGColor(c_void);

pub type CGColorRef = *mut __CGColor;

extern "C" {
    pub fn CGColorCreate(color_space: CGColorSpaceRef, components: *const CGFloat) -> CGColorRef;
    pub fn CGColorCreateGenericGray(gray: CGFloat, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorCreateGenericRGB(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorCreateGenericCMYK(cyan: CGFloat, magenta: CGFloat, yellow: CGFloat, black: CGFloat, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorCreateGenericGrayGamma2_2(gray: CGFloat, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorCreateSRGB(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorGetConstantColor(colorName: CFStringRef) -> CGColorRef;
    pub fn CGColorCreateWithPattern(space: CGColorSpaceRef, pattern: CGPatternRef, components: *const CGFloat) -> CGColorRef;
    pub fn CGColorCreateCopy(color: CGColorRef) -> CGColorRef;
    pub fn CGColorCreateCopyWithAlpha(color: CGColorRef, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorCreateCopyByMatchingToColorSpace(
        space: CGColorSpaceRef,
        intent: CGColorRenderingIntent,
        color: CGColorRef,
        options: CFDictionaryRef,
    ) -> CGColorRef;
    pub fn CGColorRetain(color: CGColorRef) -> CGColorRef;
    pub fn CGColorRelease(color: CGColorRef);
    pub fn CGColorEqualToColor(color1: CGColorRef, color2: CGColorRef) -> bool;
    pub fn CGColorGetNumberOfComponents(color: CGColorRef) -> size_t;
    pub fn CGColorGetComponents(color: CGColorRef) -> *const CGFloat;
    pub fn CGColorGetAlpha(color: CGColorRef) -> CGFloat;
    pub fn CGColorGetColorSpace(color: CGColorRef) -> CGColorSpaceRef;
    pub fn CGColorGetPattern(color: CGColorRef) -> CGPatternRef;
    pub fn CGColorGetTypeID() -> CFTypeID;

    pub static kCGColorWhite: CFStringRef;
    pub static kCGColorBlack: CFStringRef;
    pub static kCGColorClear: CFStringRef;
}

pub struct CGColor(CGColorRef);

impl Drop for CGColor {
    fn drop(&mut self) {
        unsafe { CGColorRelease(self.0) }
    }
}

impl_TCFType!(CGColor, CGColorRef, CGColorGetTypeID);
impl_CFTypeDescription!(CGColor);

impl CGColor {
    pub fn new(color_space: &CGColorSpace, components: &[CGFloat]) -> Option<Self> {
        unsafe {
            let color = CGColorCreate(color_space.as_concrete_TypeRef(), components.as_ptr());
            if color.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(color))
            }
        }
    }

    pub fn new_generic_gray(gray: CGFloat, alpha: CGFloat) -> Self {
        unsafe {
            let color = CGColorCreateGenericGray(gray, alpha);
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn new_generic_rgb(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Self {
        unsafe {
            let color = CGColorCreateGenericRGB(red, green, blue, alpha);
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn new_generic_cmyk(cyan: CGFloat, magenta: CGFloat, yellow: CGFloat, black: CGFloat, alpha: CGFloat) -> Self {
        unsafe {
            let color = CGColorCreateGenericCMYK(cyan, magenta, yellow, black, alpha);
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn new_generic_gray_gamma_2_2(gray: CGFloat, alpha: CGFloat) -> Self {
        unsafe {
            let color = CGColorCreateGenericGrayGamma2_2(gray, alpha);
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn new_srgb(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> Self {
        unsafe {
            let color = CGColorCreateSRGB(red, green, blue, alpha);
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn new_copy(color: &CGColor) -> Self {
        unsafe {
            let color = CGColorCreateCopy(color.as_concrete_TypeRef());
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn new_copy_with_alpha(color: &CGColor, alpha: CGFloat) -> Self {
        unsafe {
            let color = CGColorCreateCopyWithAlpha(color.as_concrete_TypeRef(), alpha);
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn new_copy_by_matching_to_color_space(
        space: &CGColorSpace,
        intent: CGColorRenderingIntent,
        color: &CGColor,
        options: Option<&CFDictionary<CFString, CFType>>,
    ) -> Self {
        unsafe {
            let color = CGColorCreateCopyByMatchingToColorSpace(
                space.as_concrete_TypeRef(),
                intent,
                color.as_concrete_TypeRef(),
                options.map_or(null(), |o| o.as_concrete_TypeRef()),
            );
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn from_pattern(space: &CGColorSpace, pattern: Option<&CGPattern>, components: Option<&[CGFloat]>) -> Self {
        unsafe {
            let color = CGColorCreateWithPattern(
                space.as_concrete_TypeRef(),
                pattern.map_or(null(), |p| p.as_concrete_TypeRef()),
                components.map_or(null(), |c| c.as_ptr()),
            );
            TCFType::wrap_under_create_rule(color)
        }
    }

    pub fn constant_color(color_name: &CFString) -> Option<Self> {
        unsafe {
            let color = CGColorGetConstantColor(color_name.as_concrete_TypeRef());
            if color.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(color))
            }
        }
    }

    pub fn equal(&self, color: &CGColor) -> bool {
        unsafe { CGColorEqualToColor(self.as_concrete_TypeRef(), color.as_concrete_TypeRef()) }
    }

    pub fn number_of_components(&self) -> size_t {
        unsafe { CGColorGetNumberOfComponents(self.as_concrete_TypeRef()) }
    }

    pub fn components(&self) -> &[CGFloat] {
        unsafe {
            let ptr = CGColorGetComponents(self.as_concrete_TypeRef());
            let count = self.number_of_components();
            if ptr.is_null() || count == 0 {
                &[]
            } else {
                from_raw_parts(ptr, count)
            }
        }
    }

    pub fn alpha(&self) -> CGFloat {
        unsafe { CGColorGetAlpha(self.as_concrete_TypeRef()) }
    }

    pub fn color_space(&self) -> Option<CGColorSpace> {
        unsafe {
            let space = CGColorGetColorSpace(self.as_concrete_TypeRef());
            if space.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(space))
            }
        }
    }

    pub fn pattern(&self) -> Option<CGPattern> {
        unsafe {
            let pattern = CGColorGetPattern(self.as_concrete_TypeRef());
            if pattern.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(pattern))
            }
        }
    }
}

pub enum CGColorName {
    White,
    Black,
    Clear,
}

impl From<CGColorName> for CFStringRef {
    fn from(name: CGColorName) -> Self {
        unsafe {
            match name {
                CGColorName::White => kCGColorWhite,
                CGColorName::Black => kCGColorBlack,
                CGColorName::Clear => kCGColorClear,
            }
        }
    }
}

impl From<CGColorName> for CFString {
    fn from(name: CGColorName) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(name)) }
    }
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for __CGColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGColor", &[]));
}
