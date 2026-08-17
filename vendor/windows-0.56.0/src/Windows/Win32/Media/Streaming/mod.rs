#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_DLNACAP: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 16 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_DLNADOC: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 15 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_MaxVolume: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 19 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_PacketWakeSupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 0 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SendPacketWakeSupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 1 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SinkProtocolInfo: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 14 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsAudio: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 8 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsImages: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 10 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsMute: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 18 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsSearch: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 17 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsSetNextAVT: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 20 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_SupportsVideo: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 9 };
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_Device_UDN: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 6 };
pub const GUID_DEVINTERFACE_DMP: windows_core::GUID = windows_core::GUID::from_u128(0x25b4e268_2a05_496e_803b_266837fbda4b);
pub const GUID_DEVINTERFACE_DMR: windows_core::GUID = windows_core::GUID::from_u128(0xd0875fb4_2196_4c7a_a63d_e416addd60a1);
pub const GUID_DEVINTERFACE_DMS: windows_core::GUID = windows_core::GUID::from_u128(0xc96037ae_a558_4470_b432_115a31b85553);
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(0i32);
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(1i32);
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(0i32);
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(2i32);
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MF_MEDIASOURCE_STATUS_INFO(pub i32);
impl windows_core::TypeKind for MF_MEDIASOURCE_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MF_MEDIASOURCE_STATUS_INFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MF_MEDIASOURCE_STATUS_INFO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MF_TRANSFER_VIDEO_FRAME_FLAGS(pub i32);
impl windows_core::TypeKind for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MF_TRANSFER_VIDEO_FRAME_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
impl Copy for CapturedMetadataExposureCompensation {}
impl Clone for CapturedMetadataExposureCompensation {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CapturedMetadataExposureCompensation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CapturedMetadataExposureCompensation").field("Flags", &self.Flags).field("Value", &self.Value).finish()
    }
}
impl windows_core::TypeKind for CapturedMetadataExposureCompensation {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for CapturedMetadataExposureCompensation {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Value == other.Value
    }
}
impl Eq for CapturedMetadataExposureCompensation {}
impl Default for CapturedMetadataExposureCompensation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
impl Copy for CapturedMetadataISOGains {}
impl Clone for CapturedMetadataISOGains {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CapturedMetadataISOGains {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CapturedMetadataISOGains").field("AnalogGain", &self.AnalogGain).field("DigitalGain", &self.DigitalGain).finish()
    }
}
impl windows_core::TypeKind for CapturedMetadataISOGains {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for CapturedMetadataISOGains {
    fn eq(&self, other: &Self) -> bool {
        self.AnalogGain == other.AnalogGain && self.DigitalGain == other.DigitalGain
    }
}
impl Eq for CapturedMetadataISOGains {}
impl Default for CapturedMetadataISOGains {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
impl Copy for CapturedMetadataWhiteBalanceGains {}
impl Clone for CapturedMetadataWhiteBalanceGains {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for CapturedMetadataWhiteBalanceGains {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CapturedMetadataWhiteBalanceGains").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl windows_core::TypeKind for CapturedMetadataWhiteBalanceGains {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for CapturedMetadataWhiteBalanceGains {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl Eq for CapturedMetadataWhiteBalanceGains {}
impl Default for CapturedMetadataWhiteBalanceGains {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
impl Copy for FaceCharacterization {}
impl Clone for FaceCharacterization {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for FaceCharacterization {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FaceCharacterization").field("BlinkScoreLeft", &self.BlinkScoreLeft).field("BlinkScoreRight", &self.BlinkScoreRight).field("FacialExpression", &self.FacialExpression).field("FacialExpressionScore", &self.FacialExpressionScore).finish()
    }
}
impl windows_core::TypeKind for FaceCharacterization {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for FaceCharacterization {
    fn eq(&self, other: &Self) -> bool {
        self.BlinkScoreLeft == other.BlinkScoreLeft && self.BlinkScoreRight == other.BlinkScoreRight && self.FacialExpression == other.FacialExpression && self.FacialExpressionScore == other.FacialExpressionScore
    }
}
impl Eq for FaceCharacterization {}
impl Default for FaceCharacterization {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl Copy for FaceCharacterizationBlobHeader {}
impl Clone for FaceCharacterizationBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for FaceCharacterizationBlobHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FaceCharacterizationBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl windows_core::TypeKind for FaceCharacterizationBlobHeader {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for FaceCharacterizationBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl Eq for FaceCharacterizationBlobHeader {}
impl Default for FaceCharacterizationBlobHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FaceRectInfo {
    pub Region: super::super::Foundation::RECT,
    pub confidenceLevel: i32,
}
impl Copy for FaceRectInfo {}
impl Clone for FaceRectInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for FaceRectInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FaceRectInfo").field("Region", &self.Region).field("confidenceLevel", &self.confidenceLevel).finish()
    }
}
impl windows_core::TypeKind for FaceRectInfo {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for FaceRectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Region == other.Region && self.confidenceLevel == other.confidenceLevel
    }
}
impl Eq for FaceRectInfo {}
impl Default for FaceRectInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl Copy for FaceRectInfoBlobHeader {}
impl Clone for FaceRectInfoBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for FaceRectInfoBlobHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FaceRectInfoBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl windows_core::TypeKind for FaceRectInfoBlobHeader {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for FaceRectInfoBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl Eq for FaceRectInfoBlobHeader {}
impl Default for FaceRectInfoBlobHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
impl Copy for HistogramBlobHeader {}
impl Clone for HistogramBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for HistogramBlobHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HistogramBlobHeader").field("Size", &self.Size).field("Histograms", &self.Histograms).finish()
    }
}
impl windows_core::TypeKind for HistogramBlobHeader {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for HistogramBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Histograms == other.Histograms
    }
}
impl Eq for HistogramBlobHeader {}
impl Default for HistogramBlobHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
impl Copy for HistogramDataHeader {}
impl Clone for HistogramDataHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for HistogramDataHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HistogramDataHeader").field("Size", &self.Size).field("ChannelMask", &self.ChannelMask).field("Linear", &self.Linear).finish()
    }
}
impl windows_core::TypeKind for HistogramDataHeader {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for HistogramDataHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ChannelMask == other.ChannelMask && self.Linear == other.Linear
    }
}
impl Eq for HistogramDataHeader {}
impl Default for HistogramDataHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::super::Foundation::RECT,
}
impl Copy for HistogramGrid {}
impl Clone for HistogramGrid {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for HistogramGrid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HistogramGrid").field("Width", &self.Width).field("Height", &self.Height).field("Region", &self.Region).finish()
    }
}
impl windows_core::TypeKind for HistogramGrid {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for HistogramGrid {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Region == other.Region
    }
}
impl Eq for HistogramGrid {}
impl Default for HistogramGrid {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HistogramHeader {
    pub Size: u32,
    pub Bins: u32,
    pub FourCC: u32,
    pub ChannelMasks: u32,
    pub Grid: HistogramGrid,
}
impl Copy for HistogramHeader {}
impl Clone for HistogramHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for HistogramHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HistogramHeader").field("Size", &self.Size).field("Bins", &self.Bins).field("FourCC", &self.FourCC).field("ChannelMasks", &self.ChannelMasks).field("Grid", &self.Grid).finish()
    }
}
impl windows_core::TypeKind for HistogramHeader {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for HistogramHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Bins == other.Bins && self.FourCC == other.FourCC && self.ChannelMasks == other.ChannelMasks && self.Grid == other.Grid
    }
}
impl Eq for HistogramHeader {}
impl Default for HistogramHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
impl Copy for MetadataTimeStamps {}
impl Clone for MetadataTimeStamps {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for MetadataTimeStamps {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MetadataTimeStamps").field("Flags", &self.Flags).field("Device", &self.Device).field("Presentation", &self.Presentation).finish()
    }
}
impl windows_core::TypeKind for MetadataTimeStamps {
    type TypeKind = windows_core::CopyType;
}
impl PartialEq for MetadataTimeStamps {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Device == other.Device && self.Presentation == other.Presentation
    }
}
impl Eq for MetadataTimeStamps {}
impl Default for MetadataTimeStamps {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
