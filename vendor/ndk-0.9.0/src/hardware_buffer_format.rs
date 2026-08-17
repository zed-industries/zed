//! Bindings for [`AHardwareBuffer_Format`]
//!
//! [`AHardwareBuffer_Format`]: https://developer.android.com/ndk/reference/group/a-hardware-buffer#ahardwarebuffer_format

use num_enum::{FromPrimitive, IntoPrimitive};

/// Buffer pixel formats.
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum HardwareBufferFormat {
    /// Matches deprecated [`ffi::ANativeWindow_LegacyFormat::WINDOW_FORMAT_RGBA_8888`].0.
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM")]
    R8G8B8A8_UNORM = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM.0 as i32,
    /// Matches deprecated [`ffi::ANativeWindow_LegacyFormat::WINDOW_FORMAT_RGBX_8888`].0.
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM")]
    R8G8B8X8_UNORM = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_R8G8B8_UNORM")]
    R8G8B8_UNORM = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R8G8B8_UNORM.0 as i32,
    /// Matches deprecated [`ffi::ANativeWindow_LegacyFormat::WINDOW_FORMAT_RGB_565`].0.
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_R5G6B5_UNORM")]
    R5G6B5_UNORM = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R5G6B5_UNORM.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT")]
    R16G16B16A16_FLOAT =
        ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_R10G10B10A2_UNORM")]
    R10G10B10A2_UNORM =
        ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R10G10B10A2_UNORM.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_BLOB")]
    BLOB = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_BLOB.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_D16_UNORM")]
    D16_UNORM = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_D16_UNORM.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_D24_UNORM")]
    D24_UNORM = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_D24_UNORM.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_D24_UNORM_S8_UINT")]
    D24_UNORM_S8_UINT =
        ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_D24_UNORM_S8_UINT.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_D32_FLOAT")]
    D32_FLOAT = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_D32_FLOAT.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_D32_FLOAT_S8_UINT")]
    D32_FLOAT_S8_UINT =
        ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_D32_FLOAT_S8_UINT.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_S8_UINT")]
    S8_UINT = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_S8_UINT.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_Y8Cb8Cr8_420")]
    Y8Cb8Cr8_420 = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_Y8Cb8Cr8_420.0 as i32,
    #[cfg(feature = "api-level-26")]
    #[doc(alias = "AHARDWAREBUFFER_FORMAT_YCbCr_P010")]
    YCbCr_P010 = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_YCbCr_P010.0 as i32,
    #[cfg(feature = "api-level-26")]
    R8_UNORM = ffi::AHardwareBuffer_Format::AHARDWAREBUFFER_FORMAT_R8_UNORM.0 as i32,

    #[doc(hidden)]
    #[num_enum(catch_all)]
    __Unknown(i32),
}

impl HardwareBufferFormat {
    /// Returns [`None`] when there is no immediate byte size available for this format, for
    /// example on planar buffer formats.
    pub fn bytes_per_pixel(self) -> Option<usize> {
        Some(match self {
            Self::R8G8B8A8_UNORM | Self::R8G8B8X8_UNORM => 4,
            #[cfg(feature = "api-level-26")]
            Self::R8G8B8_UNORM => 3,
            Self::R5G6B5_UNORM => 2,
            #[cfg(feature = "api-level-26")]
            Self::R16G16B16A16_FLOAT => 8,
            #[cfg(feature = "api-level-26")]
            Self::R10G10B10A2_UNORM => 4,
            #[cfg(feature = "api-level-26")]
            Self::BLOB => 1,
            #[cfg(feature = "api-level-26")]
            Self::D16_UNORM => 2,
            #[cfg(feature = "api-level-26")]
            Self::D24_UNORM => 3,
            #[cfg(feature = "api-level-26")]
            Self::D24_UNORM_S8_UINT => 4,
            #[cfg(feature = "api-level-26")]
            Self::D32_FLOAT => 4,
            #[cfg(feature = "api-level-26")]
            Self::D32_FLOAT_S8_UINT => 5,
            #[cfg(feature = "api-level-26")]
            Self::S8_UINT => 1,
            #[cfg(feature = "api-level-26")]
            Self::Y8Cb8Cr8_420 => 3,
            #[cfg(feature = "api-level-26")]
            Self::YCbCr_P010 => return None,
            #[cfg(feature = "api-level-26")]
            Self::R8_UNORM => 1,
            Self::__Unknown(_) => return None,
        })
    }
}
