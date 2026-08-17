use std::ptr::{null, null_mut};

use core_foundation::{
    base::{CFType, CFTypeID, CFTypeRef, TCFType},
    data::{CFData, CFDataRef},
    impl_CFTypeDescription, impl_TCFType,
    propertylist::{CFPropertyList, CFPropertyListRef},
    string::{CFString, CFStringRef},
};
use libc::{c_void, size_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{
    base::CGFloat,
    data_provider::{CGDataProvider, CGDataProviderRef},
};

#[repr(C)]
pub struct __CGColorSpace(c_void);

pub type CGColorSpaceRef = *mut __CGColorSpace;

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGColorRenderingIntent {
    #[doc(alias = "kCGRenderingIntentDefault")]
    Default              = 0,
    #[doc(alias = "kCGRenderingIntentAbsoluteColorimetric")]
    AbsoluteColorimetric = 1,
    #[doc(alias = "kCGRenderingIntentRelativeColorimetric")]
    RelativeColorimetric = 2,
    #[doc(alias = "kCGRenderingIntentPerceptual")]
    Perceptual           = 3,
    #[doc(alias = "kCGRenderingIntentSaturation")]
    Saturation           = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGColorSpaceModel {
    #[doc(alias = "kCGColorSpaceModelUnknown")]
    Unknown    = -1,
    #[doc(alias = "kCGColorSpaceModelMonochrome")]
    Monochrome = 0,
    #[doc(alias = "kCGColorSpaceModelRGB")]
    RGB        = 1,
    #[doc(alias = "kCGColorSpaceModelCMYK")]
    CMYK       = 2,
    #[doc(alias = "kCGColorSpaceModelLab")]
    Lab        = 3,
    #[doc(alias = "kCGColorSpaceModelDeviceN")]
    DeviceN    = 4,
    #[doc(alias = "kCGColorSpaceModelIndexed")]
    Indexed    = 5,
    #[doc(alias = "kCGColorSpaceModelPattern")]
    Pattern    = 6,
    #[doc(alias = "kCGColorSpaceModelXYZ")]
    XYZ        = 7,
}

extern "C" {
    pub static kCGColorSpaceGenericGray: CFStringRef;
    pub static kCGColorSpaceGenericRGB: CFStringRef;
    pub static kCGColorSpaceGenericCMYK: CFStringRef;
    pub static kCGColorSpaceDisplayP3: CFStringRef;
    pub static kCGColorSpaceGenericRGBLinear: CFStringRef;
    pub static kCGColorSpaceAdobeRGB1998: CFStringRef;
    pub static kCGColorSpaceSRGB: CFStringRef;
    pub static kCGColorSpaceGenericGrayGamma2_2: CFStringRef;
    pub static kCGColorSpaceGenericXYZ: CFStringRef;
    pub static kCGColorSpaceGenericLab: CFStringRef;
    pub static kCGColorSpaceACESCGLinear: CFStringRef;
    pub static kCGColorSpaceITUR_709: CFStringRef;
    pub static kCGColorSpaceITUR_709_PQ: CFStringRef;
    pub static kCGColorSpaceITUR_709_HLG: CFStringRef;
    pub static kCGColorSpaceITUR_2020: CFStringRef;
    pub static kCGColorSpaceITUR_2020_sRGBGamma: CFStringRef;
    pub static kCGColorSpaceROMMRGB: CFStringRef;
    pub static kCGColorSpaceDCIP3: CFStringRef;
    pub static kCGColorSpaceLinearITUR_2020: CFStringRef;
    pub static kCGColorSpaceExtendedITUR_2020: CFStringRef;
    pub static kCGColorSpaceExtendedLinearITUR_2020: CFStringRef;
    pub static kCGColorSpaceLinearDisplayP3: CFStringRef;
    pub static kCGColorSpaceExtendedDisplayP3: CFStringRef;
    pub static kCGColorSpaceExtendedLinearDisplayP3: CFStringRef;
    pub static kCGColorSpaceITUR_2100_PQ: CFStringRef;
    pub static kCGColorSpaceITUR_2100_HLG: CFStringRef;
    pub static kCGColorSpaceDisplayP3_PQ: CFStringRef;
    pub static kCGColorSpaceDisplayP3_HLG: CFStringRef;
    pub static kCGColorSpaceExtendedSRGB: CFStringRef;
    pub static kCGColorSpaceLinearSRGB: CFStringRef;
    pub static kCGColorSpaceExtendedLinearSRGB: CFStringRef;
    pub static kCGColorSpaceExtendedGray: CFStringRef;
    pub static kCGColorSpaceLinearGray: CFStringRef;
    pub static kCGColorSpaceExtendedLinearGray: CFStringRef;

    pub fn CGColorSpaceCreateDeviceGray() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateDeviceCMYK() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateCalibratedGray(whitePoint: *const CGFloat, blackPoint: *const CGFloat, gamma: CGFloat) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateCalibratedRGB(
        whitePoint: *const CGFloat,
        blackPoint: *const CGFloat,
        gamma: *const CGFloat,
        matrix: *const CGFloat,
    ) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateLab(whitePoint: *const CGFloat, blackPoint: *const CGFloat, range: *const CGFloat) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateWithICCData(data: CFTypeRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateICCBased(
        nComponents: usize,
        range: *const CGFloat,
        profile: CGDataProviderRef,
        alternate: CGColorSpaceRef,
    ) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateIndexed(baseSpace: CGColorSpaceRef, lastIndex: size_t, colorTable: *const u8) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreatePattern(baseSpace: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateWithName(name: CFStringRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceRetain(space: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceRelease(space: CGColorSpaceRef);
    pub fn CGColorSpaceGetName(space: CGColorSpaceRef) -> CFStringRef;
    pub fn CGColorSpaceCopyName(space: CGColorSpaceRef) -> CFStringRef;
    pub fn CGColorSpaceGetTypeID() -> CFTypeID;
    pub fn CGColorSpaceGetNumberOfComponents(space: CGColorSpaceRef) -> size_t;
    pub fn CGColorSpaceGetModel(space: CGColorSpaceRef) -> CGColorSpaceModel;
    pub fn CGColorSpaceGetBaseColorSpace(space: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceGetColorTableCount(space: CGColorSpaceRef) -> size_t;
    pub fn CGColorSpaceGetColorTable(space: CGColorSpaceRef, table: *mut u8);
    pub fn CGColorSpaceCopyICCData(space: CGColorSpaceRef) -> CFDataRef;
    pub fn CGColorSpaceIsWideGamutRGB(space: CGColorSpaceRef) -> bool;
    pub fn CGColorSpaceUsesITUR_2100TF(space: CGColorSpaceRef) -> bool;
    pub fn CGColorSpaceIsPQBased(space: CGColorSpaceRef) -> bool;
    pub fn CGColorSpaceIsHLGBased(space: CGColorSpaceRef) -> bool;
    pub fn CGColorSpaceSupportsOutput(space: CGColorSpaceRef) -> bool;
    pub fn CGColorSpaceCopyPropertyList(space: CGColorSpaceRef) -> CFPropertyListRef;
    pub fn CGColorSpaceCreateWithPropertyList(plist: CFPropertyListRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceUsesExtendedRange(space: CGColorSpaceRef) -> bool;
    pub fn CGColorSpaceCreateLinearized(space: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateExtended(space: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateExtendedLinearized(space: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateCopyWithStandardRange(space: CGColorSpaceRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateWithICCProfile(data: CFDataRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCopyICCProfile(space: CGColorSpaceRef) -> CFDataRef;
    pub fn CGColorSpaceCreateWithPlatformColorSpace(ref_: *mut c_void) -> CGColorSpaceRef;
}

pub struct CGColorSpace(CGColorSpaceRef);

impl Drop for CGColorSpace {
    fn drop(&mut self) {
        unsafe { CGColorSpaceRelease(self.0) }
    }
}

impl_TCFType!(CGColorSpace, CGColorSpaceRef, CGColorSpaceGetTypeID);
impl_CFTypeDescription!(CGColorSpace);

impl CGColorSpace {
    pub fn new_device_gray() -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateDeviceGray();
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_device_rgb() -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateDeviceRGB();
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_device_cmyk() -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateDeviceCMYK();
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_calibrated_gray(white_point: &[CGFloat; 3], black_point: Option<&[CGFloat; 3]>, gamma: CGFloat) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateCalibratedGray(white_point.as_ptr(), black_point.map_or(null(), |p| p.as_ptr()), gamma);
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_calibrated_rgb(
        white_point: &[CGFloat; 3],
        black_point: Option<&[CGFloat; 3]>,
        gamma: Option<&[CGFloat; 3]>,
        matrix: Option<&[CGFloat; 9]>,
    ) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateCalibratedRGB(
                white_point.as_ptr(),
                black_point.map_or(null(), |p| p.as_ptr()),
                gamma.map_or(null(), |g| g.as_ptr()),
                matrix.map_or(null(), |m| m.as_ptr()),
            );
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_lab(white_point: &[CGFloat; 3], black_point: Option<&[CGFloat; 3]>, range: Option<&[CGFloat; 4]>) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateLab(white_point.as_ptr(), black_point.map_or(null(), |p| p.as_ptr()), range.map_or(null(), |r| r.as_ptr()));
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_icc_based(n_components: usize, range: &[CGFloat], profile: Option<&CGDataProvider>, alternate: Option<&CGColorSpace>) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateICCBased(
                n_components,
                range.as_ptr(),
                profile.map_or(null_mut(), |p| p.as_concrete_TypeRef()),
                alternate.map_or(null_mut(), |a| a.as_concrete_TypeRef()),
            );
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_indexed(base_space: Option<&CGColorSpace>, last_index: size_t, color_table: Option<&[u8]>) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateIndexed(
                base_space.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                last_index,
                color_table.map_or(null(), |t| t.as_ptr()),
            );
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_pattern(base_space: Option<&CGColorSpace>) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreatePattern(base_space.map_or(null_mut(), |s| s.as_concrete_TypeRef()));
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_linearized(&self) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateLinearized(self.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_extended(&self) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateExtended(self.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_extended_linearized(&self) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateExtendedLinearized(self.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn new_copy_with_standard_range(&self) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateCopyWithStandardRange(self.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn from_name(name: &CFString) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateWithName(name.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn from_property_list(plist: &CFPropertyList) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateWithPropertyList(plist.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn from_icc_profile(data: &CFData) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateWithICCProfile(data.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn from_icc_data(data: &CFType) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceCreateWithICCData(data.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(cs))
            }
        }
    }

    pub fn name(&self) -> Option<CFString> {
        unsafe {
            let name = CGColorSpaceGetName(self.as_concrete_TypeRef());
            if name.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(name))
            }
        }
    }

    pub fn copy_name(&self) -> Option<CFString> {
        unsafe {
            let name = CGColorSpaceCopyName(self.as_concrete_TypeRef());
            if name.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(name))
            }
        }
    }

    pub fn number_of_components(&self) -> size_t {
        unsafe { CGColorSpaceGetNumberOfComponents(self.as_concrete_TypeRef()) }
    }

    pub fn model(&self) -> CGColorSpaceModel {
        unsafe { CGColorSpaceGetModel(self.as_concrete_TypeRef()) }
    }

    pub fn base_color_space(&self) -> Option<Self> {
        unsafe {
            let cs = CGColorSpaceGetBaseColorSpace(self.as_concrete_TypeRef());
            if cs.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(cs))
            }
        }
    }

    pub fn color_table(&self) -> Option<Vec<u8>> {
        let count = unsafe { CGColorSpaceGetColorTableCount(self.as_concrete_TypeRef()) };
        if count == 0 {
            return None;
        }
        let mut table = vec![0; count];
        unsafe {
            CGColorSpaceGetColorTable(self.as_concrete_TypeRef(), table.as_mut_ptr());
        }
        Some(table)
    }

    pub fn copy_icc_data(&self) -> Option<CFData> {
        unsafe {
            let data = CGColorSpaceCopyICCData(self.as_concrete_TypeRef());
            if data.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(data))
            }
        }
    }

    pub fn is_wide_gamut_rgb(&self) -> bool {
        unsafe { CGColorSpaceIsWideGamutRGB(self.as_concrete_TypeRef()) }
    }

    pub fn uses_itur_2100_tf(&self) -> bool {
        unsafe { CGColorSpaceUsesITUR_2100TF(self.as_concrete_TypeRef()) }
    }

    pub fn is_pq_based(&self) -> bool {
        unsafe { CGColorSpaceIsPQBased(self.as_concrete_TypeRef()) }
    }

    pub fn is_hlg_based(&self) -> bool {
        unsafe { CGColorSpaceIsHLGBased(self.as_concrete_TypeRef()) }
    }

    pub fn supports_output(&self) -> bool {
        unsafe { CGColorSpaceSupportsOutput(self.as_concrete_TypeRef()) }
    }

    pub fn copy_property_list(&self) -> Option<CFPropertyList> {
        unsafe {
            let plist = CGColorSpaceCopyPropertyList(self.as_concrete_TypeRef());
            if plist.is_null() {
                None
            } else {
                Some(CFPropertyList::wrap_under_create_rule(plist))
            }
        }
    }

    pub fn uses_extended_range(&self) -> bool {
        unsafe { CGColorSpaceUsesExtendedRange(self.as_concrete_TypeRef()) }
    }

    pub fn copy_icc_profile(&self) -> Option<CFData> {
        unsafe {
            let data = CGColorSpaceCopyICCProfile(self.as_concrete_TypeRef());
            if data.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(data))
            }
        }
    }
}

pub enum CGColorSpaceNames {
    GenericGray,
    GenericRGB,
    GenericCMYK,
    DisplayP3,
    GenericRGBLinear,
    AdobeRGB1998,
    SRGB,
    GenericGrayGamma2_2,
    GenericXYZ,
    GenericLab,
    ACESCGLinear,
    ITUR_709,
    ITUR_709_PQ,
    ITUR_709_HLG,
    ITUR_2020,
    ITUR_2020_sRGBGamma,
    ROMMRGB,
    DCIP3,
    LinearITUR_2020,
    ExtendedITUR_2020,
    ExtendedLinearITUR_2020,
    LinearDisplayP3,
    ExtendedDisplayP3,
    ExtendedLinearDisplayP3,
    ITUR_2100_PQ,
    ITUR_2100_HLG,
    DisplayP3_PQ,
    DisplayP3_HLG,
    ExtendedSRGB,
    LinearSRGB,
    ExtendedLinearSRGB,
    ExtendedGray,
    LinearGray,
    ExtendedLinearGray,
}

impl From<CGColorSpaceNames> for CFStringRef {
    fn from(key: CGColorSpaceNames) -> Self {
        unsafe {
            match key {
                CGColorSpaceNames::GenericGray => kCGColorSpaceGenericGray,
                CGColorSpaceNames::GenericRGB => kCGColorSpaceGenericRGB,
                CGColorSpaceNames::GenericCMYK => kCGColorSpaceGenericCMYK,
                CGColorSpaceNames::DisplayP3 => kCGColorSpaceDisplayP3,
                CGColorSpaceNames::GenericRGBLinear => kCGColorSpaceGenericRGBLinear,
                CGColorSpaceNames::AdobeRGB1998 => kCGColorSpaceAdobeRGB1998,
                CGColorSpaceNames::SRGB => kCGColorSpaceSRGB,
                CGColorSpaceNames::GenericGrayGamma2_2 => kCGColorSpaceGenericGrayGamma2_2,
                CGColorSpaceNames::GenericXYZ => kCGColorSpaceGenericXYZ,
                CGColorSpaceNames::GenericLab => kCGColorSpaceGenericLab,
                CGColorSpaceNames::ACESCGLinear => kCGColorSpaceACESCGLinear,
                CGColorSpaceNames::ITUR_709 => kCGColorSpaceITUR_709,
                CGColorSpaceNames::ITUR_709_PQ => kCGColorSpaceITUR_709_PQ,
                CGColorSpaceNames::ITUR_709_HLG => kCGColorSpaceITUR_709_HLG,
                CGColorSpaceNames::ITUR_2020 => kCGColorSpaceITUR_2020,
                CGColorSpaceNames::ITUR_2020_sRGBGamma => kCGColorSpaceITUR_2020_sRGBGamma,
                CGColorSpaceNames::ROMMRGB => kCGColorSpaceROMMRGB,
                CGColorSpaceNames::DCIP3 => kCGColorSpaceDCIP3,
                CGColorSpaceNames::LinearITUR_2020 => kCGColorSpaceLinearITUR_2020,
                CGColorSpaceNames::ExtendedITUR_2020 => kCGColorSpaceExtendedITUR_2020,
                CGColorSpaceNames::ExtendedLinearITUR_2020 => kCGColorSpaceExtendedLinearITUR_2020,
                CGColorSpaceNames::LinearDisplayP3 => kCGColorSpaceLinearDisplayP3,
                CGColorSpaceNames::ExtendedDisplayP3 => kCGColorSpaceExtendedDisplayP3,
                CGColorSpaceNames::ExtendedLinearDisplayP3 => kCGColorSpaceExtendedLinearDisplayP3,
                CGColorSpaceNames::ITUR_2100_PQ => kCGColorSpaceITUR_2100_PQ,
                CGColorSpaceNames::ITUR_2100_HLG => kCGColorSpaceITUR_2100_HLG,
                CGColorSpaceNames::DisplayP3_PQ => kCGColorSpaceDisplayP3_PQ,
                CGColorSpaceNames::DisplayP3_HLG => kCGColorSpaceDisplayP3_HLG,
                CGColorSpaceNames::ExtendedSRGB => kCGColorSpaceExtendedSRGB,
                CGColorSpaceNames::LinearSRGB => kCGColorSpaceLinearSRGB,
                CGColorSpaceNames::ExtendedLinearSRGB => kCGColorSpaceExtendedLinearSRGB,
                CGColorSpaceNames::ExtendedGray => kCGColorSpaceExtendedGray,
                CGColorSpaceNames::LinearGray => kCGColorSpaceLinearGray,
                CGColorSpaceNames::ExtendedLinearGray => kCGColorSpaceExtendedLinearGray,
            }
        }
    }
}

impl From<CGColorSpaceNames> for CFString {
    fn from(key: CGColorSpaceNames) -> Self {
        unsafe { CFString::wrap_under_get_rule(key.into()) }
    }
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for __CGColorSpace {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGColorSpace", &[]));
}
