#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_DLNACAP: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 16 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_DLNADOC: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 15 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_MaxVolume: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 19 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_PacketWakeSupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 0 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SendPacketWakeSupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 1 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SinkProtocolInfo: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 14 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsAudio: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 8 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsImages: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 10 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsMute: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 18 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsSearch: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 17 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsSetNextAVT: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 20 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsVideo: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 9 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_UDN: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_sys::core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 6 };
pub const GUID_DEVINTERFACE_DMP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25b4e268_2a05_496e_803b_266837fbda4b);
pub const GUID_DEVINTERFACE_DMR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd0875fb4_2196_4c7a_a63d_e416addd60a1);
pub const GUID_DEVINTERFACE_DMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc96037ae_a558_4470_b432_115a31b85553);
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = 0i32;
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = 1i32;
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = 0i32;
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = 2i32;
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = 1i32;
pub type MF_MEDIASOURCE_STATUS_INFO = i32;
pub type MF_TRANSFER_VIDEO_FRAME_FLAGS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FaceRectInfo {
    pub Region: super::super::Foundation::RECT,
    pub confidenceLevel: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HistogramHeader {
    pub Size: u32,
    pub Bins: u32,
    pub FourCC: u32,
    pub ChannelMasks: u32,
    pub Grid: HistogramGrid,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
