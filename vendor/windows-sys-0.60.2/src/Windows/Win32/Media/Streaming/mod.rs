#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
pub const DEVPKEY_Device_DLNACAP: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 16 };
pub const DEVPKEY_Device_DLNADOC: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 15 };
pub const DEVPKEY_Device_MaxVolume: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 19 };
pub const DEVPKEY_Device_PacketWakeSupported: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 0 };
pub const DEVPKEY_Device_SendPacketWakeSupported: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 1 };
pub const DEVPKEY_Device_SinkProtocolInfo: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 14 };
pub const DEVPKEY_Device_SupportsAudio: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 8 };
pub const DEVPKEY_Device_SupportsImages: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 10 };
pub const DEVPKEY_Device_SupportsMute: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 18 };
pub const DEVPKEY_Device_SupportsSearch: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 17 };
pub const DEVPKEY_Device_SupportsSetNextAVT: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 20 };
pub const DEVPKEY_Device_SupportsVideo: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 9 };
pub const DEVPKEY_Device_UDN: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 6 };
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FaceRectInfo {
    pub Region: super::super::Foundation::RECT,
    pub confidenceLevel: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
pub const GUID_DEVINTERFACE_DMP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25b4e268_2a05_496e_803b_266837fbda4b);
pub const GUID_DEVINTERFACE_DMR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd0875fb4_2196_4c7a_a63d_e416addd60a1);
pub const GUID_DEVINTERFACE_DMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc96037ae_a558_4470_b432_115a31b85553);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HistogramHeader {
    pub Size: u32,
    pub Bins: u32,
    pub FourCC: u32,
    pub ChannelMasks: u32,
    pub Grid: HistogramGrid,
}
pub type MF_MEDIASOURCE_STATUS_INFO = i32;
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = 0i32;
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = 1i32;
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = 0i32;
pub type MF_TRANSFER_VIDEO_FRAME_FLAGS = i32;
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = 2i32;
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
