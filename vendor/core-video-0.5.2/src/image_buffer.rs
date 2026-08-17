use std::mem;

use core_foundation::{
    base::{Boolean, CFGetTypeID, CFType, CFTypeID, CFTypeRef, TCFType, TCFTypeRef},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription,
    string::{CFString, CFStringRef},
};
use core_graphics::{
    color_space::{CGColorSpace, CGColorSpaceRef},
    geometry::{CGRect, CGSize},
};
use libc::c_void;

use crate::buffer::{CVBuffer, CVBufferRef, CVBufferRetain, TCVBuffer};

pub type CVImageBufferRef = CVBufferRef;

extern "C" {
    pub static kCVImageBufferCGColorSpaceKey: CFStringRef;
    pub static kCVImageBufferCleanApertureKey: CFStringRef;
    pub static kCVImageBufferCleanApertureWidthKey: CFStringRef;
    pub static kCVImageBufferCleanApertureHeightKey: CFStringRef;
    pub static kCVImageBufferCleanApertureHorizontalOffsetKey: CFStringRef;
    pub static kCVImageBufferCleanApertureVerticalOffsetKey: CFStringRef;
    pub static kCVImageBufferPreferredCleanApertureKey: CFStringRef;
    pub static kCVImageBufferFieldCountKey: CFStringRef;
    pub static kCVImageBufferFieldDetailKey: CFStringRef;
    pub static kCVImageBufferFieldDetailTemporalTopFirst: CFStringRef;
    pub static kCVImageBufferFieldDetailTemporalBottomFirst: CFStringRef;
    pub static kCVImageBufferFieldDetailSpatialFirstLineEarly: CFStringRef;
    pub static kCVImageBufferFieldDetailSpatialFirstLineLate: CFStringRef;
    pub static kCVImageBufferPixelAspectRatioKey: CFStringRef;
    pub static kCVImageBufferPixelAspectRatioHorizontalSpacingKey: CFStringRef;
    pub static kCVImageBufferPixelAspectRatioVerticalSpacingKey: CFStringRef;
    pub static kCVImageBufferDisplayDimensionsKey: CFStringRef;
    pub static kCVImageBufferDisplayWidthKey: CFStringRef;
    pub static kCVImageBufferDisplayHeightKey: CFStringRef;
    pub static kCVImageBufferGammaLevelKey: CFStringRef;
    pub static kCVImageBufferICCProfileKey: CFStringRef;
    pub static kCVImageBufferYCbCrMatrixKey: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_601_4: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_DCI_P3: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_P3_D65: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferColorPrimariesKey: CFStringRef;
    pub static kCVImageBufferColorPrimaries_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferColorPrimaries_EBU_3213: CFStringRef;
    pub static kCVImageBufferColorPrimaries_SMPTE_C: CFStringRef;
    pub static kCVImageBufferColorPrimaries_P22: CFStringRef;
    pub static kCVImageBufferColorPrimaries_DCI_P3: CFStringRef;
    pub static kCVImageBufferColorPrimaries_P3_D65: CFStringRef;
    pub static kCVImageBufferColorPrimaries_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferTransferFunctionKey: CFStringRef;
    pub static kCVImageBufferTransferFunction_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_240M_1995: CFStringRef;
    pub static kCVImageBufferTransferFunction_UseGamma: CFStringRef;
    pub static kCVImageBufferTransferFunction_sRGB: CFStringRef;
    pub static kCVImageBufferTransferFunction_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_ST_428_1: CFStringRef;
    pub static kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ: CFStringRef;
    pub static kCVImageBufferTransferFunction_ITU_R_2100_HLG: CFStringRef;
    pub static kCVImageBufferTransferFunction_Linear: CFStringRef;
    pub static kCVImageBufferChromaLocationTopFieldKey: CFStringRef;
    pub static kCVImageBufferChromaLocationBottomFieldKey: CFStringRef;
    pub static kCVImageBufferChromaLocation_Left: CFStringRef;
    pub static kCVImageBufferChromaLocation_Center: CFStringRef;
    pub static kCVImageBufferChromaLocation_TopLeft: CFStringRef;
    pub static kCVImageBufferChromaLocation_Top: CFStringRef;
    pub static kCVImageBufferChromaLocation_BottomLeft: CFStringRef;
    pub static kCVImageBufferChromaLocation_Bottom: CFStringRef;
    pub static kCVImageBufferChromaLocation_DV420: CFStringRef;
    pub static kCVImageBufferChromaSubsamplingKey: CFStringRef;
    pub static kCVImageBufferChromaSubsampling_420: CFStringRef;
    pub static kCVImageBufferChromaSubsampling_422: CFStringRef;
    pub static kCVImageBufferChromaSubsampling_411: CFStringRef;
    pub static kCVImageBufferAlphaChannelIsOpaque: CFStringRef;
    pub static kCVImageBufferAlphaChannelModeKey: CFStringRef;
    pub static kCVImageBufferAlphaChannelMode_StraightAlpha: CFStringRef;
    pub static kCVImageBufferAlphaChannelMode_PremultipliedAlpha: CFStringRef;

    pub fn CVYCbCrMatrixGetIntegerCodePointForString(yCbCrMatrixString: CFStringRef) -> i32;
    pub fn CVColorPrimariesGetIntegerCodePointForString(colorPrimariesString: CFStringRef) -> i32;
    pub fn CVTransferFunctionGetIntegerCodePointForString(transferFunctionString: CFStringRef) -> i32;
    pub fn CVYCbCrMatrixGetStringForIntegerCodePoint(yCbCrMatrixCodePoint: i32) -> CFStringRef;
    pub fn CVColorPrimariesGetStringForIntegerCodePoint(colorPrimariesCodePoint: i32) -> CFStringRef;
    pub fn CVTransferFunctionGetStringForIntegerCodePoint(transferFunctionCodePoint: i32) -> CFStringRef;

    pub fn CVImageBufferGetEncodedSize(imageBuffer: CVImageBufferRef) -> CGSize;
    pub fn CVImageBufferGetDisplaySize(imageBuffer: CVImageBufferRef) -> CGSize;
    pub fn CVImageBufferGetCleanRect(imageBuffer: CVImageBufferRef) -> CGRect;
    pub fn CVImageBufferIsFlipped(imageBuffer: CVImageBufferRef) -> Boolean;
    pub fn CVImageBufferGetColorSpace(imageBuffer: CVImageBufferRef) -> CGColorSpaceRef;
    pub fn CVImageBufferCreateColorSpaceFromAttachments(attachments: CFDictionaryRef) -> CGColorSpaceRef;

    pub static kCVImageBufferMasteringDisplayColorVolumeKey: CFStringRef;
    pub static kCVImageBufferContentLightLevelInfoKey: CFStringRef;
    pub static kCVImageBufferAmbientViewingEnvironmentKey: CFStringRef;
    pub static kCVImageBufferRegionOfInterestKey: CFStringRef;
}

pub enum CVImageBufferKeys {
    CGColorSpace,
    CleanAperture,
    PreferredCleanAperture,
    FieldCount,
    FieldDetail,
    PixelAspectRatio,
    DisplayDimensions,
    GammaLevel,
    ICCProfile,
    YCbCrMatrix,
    ColorPrimaries,
    TransferFunction,
    ChromaLocationTopField,
    ChromaLocationBottomField,
    ChromaSubsampling,
    AlphaChannelIsOpaque,
    AlphaChannelMode,
    MasteringDisplayColorVolume,
    ContentLightLevelInfo,
    AmbientViewingEnvironment,
    RegionOfInterest,
}

impl From<CVImageBufferKeys> for CFStringRef {
    fn from(key: CVImageBufferKeys) -> CFStringRef {
        unsafe {
            match key {
                CVImageBufferKeys::CGColorSpace => kCVImageBufferCGColorSpaceKey,
                CVImageBufferKeys::CleanAperture => kCVImageBufferCleanApertureKey,
                CVImageBufferKeys::PreferredCleanAperture => kCVImageBufferPreferredCleanApertureKey,
                CVImageBufferKeys::FieldCount => kCVImageBufferFieldCountKey,
                CVImageBufferKeys::FieldDetail => kCVImageBufferFieldDetailKey,
                CVImageBufferKeys::PixelAspectRatio => kCVImageBufferPixelAspectRatioKey,
                CVImageBufferKeys::DisplayDimensions => kCVImageBufferDisplayDimensionsKey,
                CVImageBufferKeys::GammaLevel => kCVImageBufferGammaLevelKey,
                CVImageBufferKeys::ICCProfile => kCVImageBufferICCProfileKey,
                CVImageBufferKeys::YCbCrMatrix => kCVImageBufferYCbCrMatrixKey,
                CVImageBufferKeys::ColorPrimaries => kCVImageBufferColorPrimariesKey,
                CVImageBufferKeys::TransferFunction => kCVImageBufferTransferFunctionKey,
                CVImageBufferKeys::ChromaLocationTopField => kCVImageBufferChromaLocationTopFieldKey,
                CVImageBufferKeys::ChromaLocationBottomField => kCVImageBufferChromaLocationBottomFieldKey,
                CVImageBufferKeys::ChromaSubsampling => kCVImageBufferChromaSubsamplingKey,
                CVImageBufferKeys::AlphaChannelIsOpaque => kCVImageBufferAlphaChannelIsOpaque,
                CVImageBufferKeys::AlphaChannelMode => kCVImageBufferAlphaChannelModeKey,
                CVImageBufferKeys::MasteringDisplayColorVolume => kCVImageBufferMasteringDisplayColorVolumeKey,
                CVImageBufferKeys::ContentLightLevelInfo => kCVImageBufferContentLightLevelInfoKey,
                CVImageBufferKeys::AmbientViewingEnvironment => kCVImageBufferAmbientViewingEnvironmentKey,
                CVImageBufferKeys::RegionOfInterest => kCVImageBufferRegionOfInterestKey,
            }
        }
    }
}

impl From<CVImageBufferKeys> for CFString {
    fn from(key: CVImageBufferKeys) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

pub enum CVImageBufferFieldDetail {
    TemporalTopFirst,
    TemporalBottomFirst,
    SpatialFirstLineEarly,
    SpatialFirstLineLate,
}

impl From<CVImageBufferFieldDetail> for CFStringRef {
    fn from(field_detail: CVImageBufferFieldDetail) -> CFStringRef {
        unsafe {
            match field_detail {
                CVImageBufferFieldDetail::TemporalTopFirst => kCVImageBufferFieldDetailTemporalTopFirst,
                CVImageBufferFieldDetail::TemporalBottomFirst => kCVImageBufferFieldDetailTemporalBottomFirst,
                CVImageBufferFieldDetail::SpatialFirstLineEarly => kCVImageBufferFieldDetailSpatialFirstLineEarly,
                CVImageBufferFieldDetail::SpatialFirstLineLate => kCVImageBufferFieldDetailSpatialFirstLineLate,
            }
        }
    }
}

impl From<CVImageBufferFieldDetail> for CFString {
    fn from(field_detail: CVImageBufferFieldDetail) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(field_detail)) }
    }
}

pub enum CVImageBufferPixelAspectRatio {
    HorizontalSpacing,
    VerticalSpacing,
}

impl From<CVImageBufferPixelAspectRatio> for CFStringRef {
    fn from(pixel_aspect_ratio: CVImageBufferPixelAspectRatio) -> CFStringRef {
        unsafe {
            match pixel_aspect_ratio {
                CVImageBufferPixelAspectRatio::HorizontalSpacing => kCVImageBufferPixelAspectRatioHorizontalSpacingKey,
                CVImageBufferPixelAspectRatio::VerticalSpacing => kCVImageBufferPixelAspectRatioVerticalSpacingKey,
            }
        }
    }
}

impl From<CVImageBufferPixelAspectRatio> for CFString {
    fn from(pixel_aspect_ratio: CVImageBufferPixelAspectRatio) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(pixel_aspect_ratio)) }
    }
}

pub enum CVImageBufferYCbCrMatrix {
    ITU_R_709_2,
    ITU_R_601_4,
    SMPTE_240M_1995,
    DCI_P3,
    P3_D65,
    ITU_R_2020,
}

impl From<CVImageBufferYCbCrMatrix> for CFStringRef {
    fn from(ycbcr_matrix: CVImageBufferYCbCrMatrix) -> CFStringRef {
        unsafe {
            match ycbcr_matrix {
                CVImageBufferYCbCrMatrix::ITU_R_709_2 => kCVImageBufferYCbCrMatrix_ITU_R_709_2,
                CVImageBufferYCbCrMatrix::ITU_R_601_4 => kCVImageBufferYCbCrMatrix_ITU_R_601_4,
                CVImageBufferYCbCrMatrix::SMPTE_240M_1995 => kCVImageBufferYCbCrMatrix_SMPTE_240M_1995,
                CVImageBufferYCbCrMatrix::DCI_P3 => kCVImageBufferYCbCrMatrix_DCI_P3,
                CVImageBufferYCbCrMatrix::P3_D65 => kCVImageBufferYCbCrMatrix_P3_D65,
                CVImageBufferYCbCrMatrix::ITU_R_2020 => kCVImageBufferYCbCrMatrix_ITU_R_2020,
            }
        }
    }
}

impl From<CVImageBufferYCbCrMatrix> for CFString {
    fn from(ycbcr_matrix: CVImageBufferYCbCrMatrix) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(ycbcr_matrix)) }
    }
}

pub enum CVImageBufferColorPrimaries {
    ITU_R_709_2,
    EBU_3213,
    SMPTE_C,
    P22,
    DCI_P3,
    P3_D65,
    ITU_R_2020,
}

impl From<CVImageBufferColorPrimaries> for CFStringRef {
    fn from(color_primaries: CVImageBufferColorPrimaries) -> CFStringRef {
        unsafe {
            match color_primaries {
                CVImageBufferColorPrimaries::ITU_R_709_2 => kCVImageBufferColorPrimaries_ITU_R_709_2,
                CVImageBufferColorPrimaries::EBU_3213 => kCVImageBufferColorPrimaries_EBU_3213,
                CVImageBufferColorPrimaries::SMPTE_C => kCVImageBufferColorPrimaries_SMPTE_C,
                CVImageBufferColorPrimaries::P22 => kCVImageBufferColorPrimaries_P22,
                CVImageBufferColorPrimaries::DCI_P3 => kCVImageBufferColorPrimaries_DCI_P3,
                CVImageBufferColorPrimaries::P3_D65 => kCVImageBufferColorPrimaries_P3_D65,
                CVImageBufferColorPrimaries::ITU_R_2020 => kCVImageBufferColorPrimaries_ITU_R_2020,
            }
        }
    }
}

impl From<CVImageBufferColorPrimaries> for CFString {
    fn from(color_primaries: CVImageBufferColorPrimaries) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(color_primaries)) }
    }
}

pub enum CVImageBufferTransferFunction {
    ITU_R_709_2,
    SMPTE_240M_1995,
    UseGamma,
    sRGB,
    ITU_R_2020,
    SMPTE_ST_428_1,
    SMPTE_ST_2084_PQ,
    ITU_R_2100_HLG,
    Linear,
}

impl From<CVImageBufferTransferFunction> for CFStringRef {
    fn from(transfer_function: CVImageBufferTransferFunction) -> CFStringRef {
        unsafe {
            match transfer_function {
                CVImageBufferTransferFunction::ITU_R_709_2 => kCVImageBufferTransferFunction_ITU_R_709_2,
                CVImageBufferTransferFunction::SMPTE_240M_1995 => kCVImageBufferTransferFunction_SMPTE_240M_1995,
                CVImageBufferTransferFunction::UseGamma => kCVImageBufferTransferFunction_UseGamma,
                CVImageBufferTransferFunction::sRGB => kCVImageBufferTransferFunction_sRGB,
                CVImageBufferTransferFunction::ITU_R_2020 => kCVImageBufferTransferFunction_ITU_R_2020,
                CVImageBufferTransferFunction::SMPTE_ST_428_1 => kCVImageBufferTransferFunction_SMPTE_ST_428_1,
                CVImageBufferTransferFunction::SMPTE_ST_2084_PQ => kCVImageBufferTransferFunction_SMPTE_ST_2084_PQ,
                CVImageBufferTransferFunction::ITU_R_2100_HLG => kCVImageBufferTransferFunction_ITU_R_2100_HLG,
                CVImageBufferTransferFunction::Linear => kCVImageBufferTransferFunction_Linear,
            }
        }
    }
}

impl From<CVImageBufferTransferFunction> for CFString {
    fn from(transfer_function: CVImageBufferTransferFunction) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(transfer_function)) }
    }
}

pub enum CVImageBufferChromaLocation {
    Left,
    Center,
    TopLeft,
    Top,
    BottomLeft,
    Bottom,
    DV420,
}

impl From<CVImageBufferChromaLocation> for CFStringRef {
    fn from(chroma_location: CVImageBufferChromaLocation) -> CFStringRef {
        unsafe {
            match chroma_location {
                CVImageBufferChromaLocation::Left => kCVImageBufferChromaLocation_Left,
                CVImageBufferChromaLocation::Center => kCVImageBufferChromaLocation_Center,
                CVImageBufferChromaLocation::TopLeft => kCVImageBufferChromaLocation_TopLeft,
                CVImageBufferChromaLocation::Top => kCVImageBufferChromaLocation_Top,
                CVImageBufferChromaLocation::BottomLeft => kCVImageBufferChromaLocation_BottomLeft,
                CVImageBufferChromaLocation::Bottom => kCVImageBufferChromaLocation_Bottom,
                CVImageBufferChromaLocation::DV420 => kCVImageBufferChromaLocation_DV420,
            }
        }
    }
}

impl From<CVImageBufferChromaLocation> for CFString {
    fn from(chroma_location: CVImageBufferChromaLocation) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(chroma_location)) }
    }
}

pub enum CVImageBufferChromaSubsampling {
    CS420,
    CS422,
    CS411,
}

impl From<CVImageBufferChromaSubsampling> for CFStringRef {
    fn from(chroma_subsampling: CVImageBufferChromaSubsampling) -> CFStringRef {
        unsafe {
            match chroma_subsampling {
                CVImageBufferChromaSubsampling::CS420 => kCVImageBufferChromaSubsampling_420,
                CVImageBufferChromaSubsampling::CS422 => kCVImageBufferChromaSubsampling_422,
                CVImageBufferChromaSubsampling::CS411 => kCVImageBufferChromaSubsampling_411,
            }
        }
    }
}

impl From<CVImageBufferChromaSubsampling> for CFString {
    fn from(chroma_subsampling: CVImageBufferChromaSubsampling) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(chroma_subsampling)) }
    }
}

pub enum CVImageBufferAlphaChannelMode {
    StraightAlpha,
    PremultipliedAlpha,
}

impl From<CVImageBufferAlphaChannelMode> for CFStringRef {
    fn from(alpha_channel_mode: CVImageBufferAlphaChannelMode) -> CFStringRef {
        unsafe {
            match alpha_channel_mode {
                CVImageBufferAlphaChannelMode::StraightAlpha => kCVImageBufferAlphaChannelMode_StraightAlpha,
                CVImageBufferAlphaChannelMode::PremultipliedAlpha => kCVImageBufferAlphaChannelMode_PremultipliedAlpha,
            }
        }
    }
}

impl From<CVImageBufferAlphaChannelMode> for CFString {
    fn from(alpha_channel_mode: CVImageBufferAlphaChannelMode) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(alpha_channel_mode)) }
    }
}

pub fn ycbcr_matrix_get_integer_code_point_for_string(ycbcr_matrix_string: &CFString) -> i32 {
    unsafe { CVYCbCrMatrixGetIntegerCodePointForString(ycbcr_matrix_string.as_concrete_TypeRef()) }
}

pub fn color_primaries_get_integer_code_point_for_string(color_primaries_string: &CFString) -> i32 {
    unsafe { CVColorPrimariesGetIntegerCodePointForString(color_primaries_string.as_concrete_TypeRef()) }
}

pub fn transfer_function_get_integer_code_point_for_string(transfer_function_string: &CFString) -> i32 {
    unsafe { CVTransferFunctionGetIntegerCodePointForString(transfer_function_string.as_concrete_TypeRef()) }
}

pub fn ycbcr_matrix_get_string_for_integer_code_point(ycbcr_matrix_code_point: i32) -> CFString {
    unsafe { CFString::wrap_under_get_rule(CVYCbCrMatrixGetStringForIntegerCodePoint(ycbcr_matrix_code_point)) }
}

pub fn color_primaries_get_string_for_integer_code_point(color_primaries_code_point: i32) -> CFString {
    unsafe { CFString::wrap_under_get_rule(CVColorPrimariesGetStringForIntegerCodePoint(color_primaries_code_point)) }
}

pub fn transfer_function_get_string_for_integer_code_point(transfer_function_code_point: i32) -> CFString {
    unsafe { CFString::wrap_under_get_rule(CVTransferFunctionGetStringForIntegerCodePoint(transfer_function_code_point)) }
}

declare_TCFType!(CVImageBuffer, CVImageBufferRef);
impl_CFTypeDescription!(CVImageBuffer);

impl CVImageBuffer {
    #[inline]
    pub fn as_concrete_TypeRef(&self) -> CVImageBufferRef {
        self.0
    }

    #[inline]
    pub fn as_CFType(&self) -> CFType {
        unsafe { CFType::wrap_under_get_rule(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn as_CFTypeRef(&self) -> CFTypeRef {
        self.as_concrete_TypeRef() as CFTypeRef
    }

    #[inline]
    pub fn into_CFType(self) -> CFType {
        let reference = self.as_CFTypeRef();
        mem::forget(self);
        unsafe { CFType::wrap_under_create_rule(reference) }
    }

    #[inline]
    pub unsafe fn wrap_under_create_rule(reference: CVImageBufferRef) -> CVImageBuffer {
        CVImageBuffer(reference)
    }

    #[inline]
    pub unsafe fn wrap_under_get_rule(reference: CVImageBufferRef) -> CVImageBuffer {
        CVImageBuffer(CVBufferRetain(reference))
    }

    #[inline]
    pub fn type_of(&self) -> CFTypeID {
        unsafe { CFGetTypeID(self.as_CFTypeRef()) }
    }

    #[inline]
    pub fn instance_of<T: TCFType>(&self) -> bool {
        self.type_of() == T::type_id()
    }
}

impl Clone for CVImageBuffer {
    fn clone(&self) -> CVImageBuffer {
        unsafe { CVImageBuffer::wrap_under_get_rule(self.0) }
    }
}

impl PartialEq for CVImageBuffer {
    fn eq(&self, other: &CVImageBuffer) -> bool {
        self.as_CFType().eq(&other.as_CFType())
    }
}

impl Eq for CVImageBuffer {}

pub trait TCVImageBuffer: TCVBuffer {
    #[inline]
    fn as_image_buffer(&self) -> CVImageBuffer {
        unsafe { CVImageBuffer::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr() as CVImageBufferRef) }
    }

    #[inline]
    fn into_image_buffer(self) -> CVImageBuffer
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr() as CVImageBufferRef;
        mem::forget(self);
        unsafe { CVImageBuffer::wrap_under_create_rule(reference) }
    }
}

impl CVImageBuffer {
    #[inline]
    pub fn downcast<T: TCVImageBuffer>(&self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe { Some(T::wrap_under_get_rule(T::Ref::from_void_ptr(self.as_concrete_TypeRef() as *const c_void))) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_into<T: TCVImageBuffer>(self) -> Option<T> {
        if self.instance_of::<T>() {
            unsafe {
                let reference = T::Ref::from_void_ptr(self.as_concrete_TypeRef() as *const c_void);
                mem::forget(self);
                Some(T::wrap_under_create_rule(reference))
            }
        } else {
            None
        }
    }
}

impl CVImageBuffer {
    #[inline]
    pub fn as_buffer(&self) -> CVBuffer {
        unsafe { CVBuffer::wrap_under_get_rule(self.as_concrete_TypeRef() as CVBufferRef) }
    }

    #[inline]
    pub fn into_buffer(self) -> CVBuffer
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef() as CVBufferRef;
        mem::forget(self);
        unsafe { CVBuffer::wrap_under_create_rule(reference) }
    }

    #[inline]
    pub fn get_encoded_size(&self) -> CGSize {
        unsafe { CVImageBufferGetEncodedSize(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_display_size(&self) -> CGSize {
        unsafe { CVImageBufferGetDisplaySize(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn get_clean_rect(&self) -> CGRect {
        unsafe { CVImageBufferGetCleanRect(self.as_concrete_TypeRef()) }
    }

    #[inline]
    pub fn is_flipped(&self) -> bool {
        unsafe { CVImageBufferIsFlipped(self.as_concrete_TypeRef()) != 0 }
    }

    #[inline]
    pub fn get_color_space(&self) -> Option<CGColorSpace> {
        unsafe {
            let color_space = CVImageBufferGetColorSpace(self.as_concrete_TypeRef());
            if color_space.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(color_space))
            }
        }
    }
}

pub fn create_color_space_from_attachments(attachments: &CFDictionary<CFString, CFType>) -> Option<CGColorSpace> {
    unsafe {
        let color_space = CVImageBufferCreateColorSpaceFromAttachments(attachments.as_concrete_TypeRef());
        if color_space.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(color_space))
        }
    }
}
