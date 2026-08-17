// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFTypeID};
use core_foundation::string::CFStringRef;
use foreign_types::{foreign_type, ForeignType};

foreign_type! {
    #[doc(hidden)]
    pub unsafe type CGColorSpace {
        type CType = crate::sys::CGColorSpace;
        fn drop = |p| CFRelease(p as *mut _);
        fn clone = |p| CFRetain(p as *const _) as *mut _;
    }
}

impl CGColorSpace {
    pub fn type_id() -> CFTypeID {
        unsafe { CGColorSpaceGetTypeID() }
    }

    pub fn create_with_name(name: CFStringRef) -> Option<CGColorSpace> {
        unsafe {
            let p = CGColorSpaceCreateWithName(name);
            if !p.is_null() {
                Some(CGColorSpace::from_ptr(p))
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn create_device_rgb() -> CGColorSpace {
        unsafe {
            let result = CGColorSpaceCreateDeviceRGB();
            CGColorSpace::from_ptr(result)
        }
    }

    #[inline]
    pub fn create_device_gray() -> CGColorSpace {
        unsafe {
            let result = CGColorSpaceCreateDeviceGray();
            CGColorSpace::from_ptr(result)
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    /// The Display P3 color space, created by Apple.
    pub static kCGColorSpaceDisplayP3: CFStringRef;
    /// The Display P3 color space, using the HLG transfer function.
    pub static kCGColorSpaceDisplayP3_HLG: CFStringRef;
    /// The Display P3 color space with a linear transfer function
    /// and extended-range values.
    pub static kCGColorSpaceExtendedLinearDisplayP3: CFStringRef;
    /// The standard Red Green Blue (sRGB) color space.
    pub static kCGColorSpaceSRGB: CFStringRef;
    /// The sRGB color space with a linear transfer function.
    pub static kCGColorSpaceLinearSRGB: CFStringRef;
    /// The extended sRGB color space.
    pub static kCGColorSpaceExtendedSRGB: CFStringRef;
    /// The sRGB color space with a linear transfer function and
    /// extended-range values.
    pub static kCGColorSpaceExtendedLinearSRGB: CFStringRef;
    /// The generic gray color space that has an exponential transfer
    /// function with a power of 2.2.
    pub static kCGColorSpaceGenericGrayGamma2_2: CFStringRef;
    /// The gray color space using a linear transfer function.
    pub static kCGColorSpaceLinearGray: CFStringRef;
    /// The extended gray color space.
    pub static kCGColorSpaceExtendedGray: CFStringRef;
    /// The extended gray color space with a linear transfer function.
    pub static kCGColorSpaceExtendedLinearGray: CFStringRef;
    /// The generic RGB color space with a linear transfer function.
    pub static kCGColorSpaceGenericRGBLinear: CFStringRef;
    /// The generic CMYK color space.
    pub static kCGColorSpaceGenericCMYK: CFStringRef;
    /// The XYZ color space, as defined by the CIE 1931 standard.
    pub static kCGColorSpaceGenericXYZ: CFStringRef;
    /// The generic LAB color space.
    pub static kCGColorSpaceGenericLab: CFStringRef;
    /// The ACEScg color space.
    pub static kCGColorSpaceACESCGLinear: CFStringRef;
    /// The Adobe RGB (1998) color space.
    pub static kCGColorSpaceAdobeRGB1998: CFStringRef;
    /// The DCI P3 color space, which is the digital cinema standard.
    pub static kCGColorSpaceDCIP3: CFStringRef;
    /// The recommendation of the International Telecommunication Union
    /// (ITU) Radiocommunication sector for the BT.709 color space.
    pub static kCGColorSpaceITUR_709: CFStringRef;
    /// The Reference Output Medium Metric (ROMM) RGB color space.
    pub static kCGColorSpaceROMMRGB: CFStringRef;
    /// The recommendation of the International Telecommunication Union
    /// (ITU) Radiocommunication sector for the BT.2020 color space.
    pub static kCGColorSpaceITUR_2020: CFStringRef;
    /// The recommendation of the International Telecommunication Union
    /// (ITU) Radiocommunication sector for the BT.2020 color space, with
    /// a linear transfer function and extended range values.
    pub static kCGColorSpaceExtendedLinearITUR_2020: CFStringRef;
    /// The name of the generic RGB color space.
    pub static kCGColorSpaceGenericRGB: CFStringRef;
    /// The name of the generic gray color space.
    pub static kCGColorSpaceGenericGray: CFStringRef;

    fn CGColorSpaceCreateDeviceRGB() -> crate::sys::CGColorSpaceRef;
    fn CGColorSpaceCreateDeviceGray() -> crate::sys::CGColorSpaceRef;
    fn CGColorSpaceCreateWithName(name: CFStringRef) -> crate::sys::CGColorSpaceRef;
    fn CGColorSpaceGetTypeID() -> CFTypeID;
}
