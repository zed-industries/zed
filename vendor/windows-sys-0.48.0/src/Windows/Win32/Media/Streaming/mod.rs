#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub type MF_MEDIASOURCE_STATUS_INFO = i32;
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = 0i32;
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = 1i32;
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub type MF_TRANSFER_VIDEO_FRAME_FLAGS = i32;
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
impl ::core::marker::Copy for CapturedMetadataExposureCompensation {}
impl ::core::clone::Clone for CapturedMetadataExposureCompensation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
impl ::core::marker::Copy for CapturedMetadataISOGains {}
impl ::core::clone::Clone for CapturedMetadataISOGains {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
impl ::core::marker::Copy for CapturedMetadataWhiteBalanceGains {}
impl ::core::clone::Clone for CapturedMetadataWhiteBalanceGains {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
impl ::core::marker::Copy for FaceCharacterization {}
impl ::core::clone::Clone for FaceCharacterization {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for FaceCharacterizationBlobHeader {}
impl ::core::clone::Clone for FaceCharacterizationBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FaceRectInfo {
    pub Region: super::super::Foundation::RECT,
    pub confidenceLevel: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FaceRectInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FaceRectInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for FaceRectInfoBlobHeader {}
impl ::core::clone::Clone for FaceRectInfoBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
impl ::core::marker::Copy for HistogramBlobHeader {}
impl ::core::clone::Clone for HistogramBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
impl ::core::marker::Copy for HistogramDataHeader {}
impl ::core::clone::Clone for HistogramDataHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HistogramGrid {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HistogramGrid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HistogramHeader {
    pub Size: u32,
    pub Bins: u32,
    pub FourCC: u32,
    pub ChannelMasks: u32,
    pub Grid: HistogramGrid,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HistogramHeader {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HistogramHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
impl ::core::marker::Copy for MetadataTimeStamps {}
impl ::core::clone::Clone for MetadataTimeStamps {
    fn clone(&self) -> Self {
        *self
    }
}
