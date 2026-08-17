#[inline]
pub unsafe fn D3DPERF_BeginEvent<P1>(col: u32, wszname: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_BeginEvent(col : u32, wszname : windows_core::PCWSTR) -> i32);
    unsafe { D3DPERF_BeginEvent(col, wszname.param().abi()) }
}
#[inline]
pub unsafe fn D3DPERF_EndEvent() -> i32 {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_EndEvent() -> i32);
    unsafe { D3DPERF_EndEvent() }
}
#[inline]
pub unsafe fn D3DPERF_GetStatus() -> u32 {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_GetStatus() -> u32);
    unsafe { D3DPERF_GetStatus() }
}
#[inline]
pub unsafe fn D3DPERF_QueryRepeatFrame() -> windows_core::BOOL {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_QueryRepeatFrame() -> windows_core::BOOL);
    unsafe { D3DPERF_QueryRepeatFrame() }
}
#[inline]
pub unsafe fn D3DPERF_SetMarker<P1>(col: u32, wszname: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetMarker(col : u32, wszname : windows_core::PCWSTR));
    unsafe { D3DPERF_SetMarker(col, wszname.param().abi()) }
}
#[inline]
pub unsafe fn D3DPERF_SetOptions(dwoptions: u32) {
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetOptions(dwoptions : u32));
    unsafe { D3DPERF_SetOptions(dwoptions) }
}
#[inline]
pub unsafe fn D3DPERF_SetRegion<P1>(col: u32, wszname: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("d3d9.dll" "system" fn D3DPERF_SetRegion(col : u32, wszname : windows_core::PCWSTR));
    unsafe { D3DPERF_SetRegion(col, wszname.param().abi()) }
}
#[inline]
pub unsafe fn Direct3DCreate9(sdkversion: u32) -> Option<IDirect3D9> {
    windows_core::link!("d3d9.dll" "system" fn Direct3DCreate9(sdkversion : u32) -> Option < IDirect3D9 >);
    unsafe { Direct3DCreate9(sdkversion) }
}
#[inline]
pub unsafe fn Direct3DCreate9Ex(sdkversion: u32) -> windows_core::Result<IDirect3D9Ex> {
    windows_core::link!("d3d9.dll" "system" fn Direct3DCreate9Ex(sdkversion : u32, param1 : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        Direct3DCreate9Ex(sdkversion, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub const D3D9_RESOURCE_PRIORITY_HIGH: u32 = 2684354560u32;
pub const D3D9_RESOURCE_PRIORITY_LOW: u32 = 1342177280u32;
pub const D3D9_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200u32;
pub const D3D9_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640u32;
pub const D3D9_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920u32;
pub const D3D9b_SDK_VERSION: u32 = 31u32;
pub const D3DADAPTER_DEFAULT: u32 = 0u32;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [i8; 512],
    pub Description: [i8; 512],
    pub DeviceName: [i8; 32],
    pub DriverVersion: i64,
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DeviceIdentifier: windows_core::GUID,
    pub WHQLLevel: u32,
}
#[cfg(target_arch = "x86")]
impl Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DADAPTER_IDENTIFIER9 {
    pub Driver: [i8; 512],
    pub Description: [i8; 512],
    pub DeviceName: [i8; 32],
    pub DriverVersion: i64,
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DeviceIdentifier: windows_core::GUID,
    pub WHQLLevel: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct D3DAES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DANTIALIASMODE(pub i32);
pub const D3DANTIALIAS_NONE: D3DANTIALIASMODE = D3DANTIALIASMODE(0i32);
pub const D3DANTIALIAS_SORTDEPENDENT: D3DANTIALIASMODE = D3DANTIALIASMODE(1i32);
pub const D3DANTIALIAS_SORTINDEPENDENT: D3DANTIALIASMODE = D3DANTIALIASMODE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNELTYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub DXVA2DecodeHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub StartSequenceQuery: u32,
    pub StartSequenceConfigure: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub Protections: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
impl Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub ProcessIdentiferType: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::super::Foundation::HANDLE,
    pub AllowAccess: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    pub Parameters: D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    pub omac: D3D_OMAC,
    pub ConfigureType: windows_core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_core::HRESULT,
}
pub const D3DAUTHENTICATEDCHANNEL_D3D9: D3DAUTHENTICATEDCHANNELTYPE = D3DAUTHENTICATEDCHANNELTYPE(1i32);
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_HARDWARE: D3DAUTHENTICATEDCHANNELTYPE = D3DAUTHENTICATEDCHANNELTYPE(3i32);
pub const D3DAUTHENTICATEDCHANNEL_DRIVER_SOFTWARE: D3DAUTHENTICATEDCHANNELTYPE = D3DAUTHENTICATEDCHANNELTYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0,
}
impl Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    pub Anonymous: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DXVA2DecodeHandle: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DXVA2DecodeHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumEncryptionGuids: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub EncryptionGuidIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub BusType: D3DBUSTYPE,
    pub bAccessibleInContiguousBlocks: windows_core::BOOL,
    pub bAccessibleInNonContiguousBlocks: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub NumOutputIDs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProtectionFlags: D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS,
}
impl Default for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumRestrictedSharedResourceProcesses: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    pub Input: D3DAUTHENTICATEDCHANNEL_QUERY_INPUT,
    pub ProcessIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub ProcessIndex: u32,
    pub ProcessIdentifer: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE,
    pub ProcessHandle: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub EncryptionGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    pub Output: D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT,
    pub NumUnrestrictedProtectedSharedResources: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    pub QueryType: windows_core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    pub omac: D3D_OMAC,
    pub QueryType: windows_core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: windows_core::HRESULT,
}
pub const D3DAUTHENTICATEDCONFIGURE_CRYPTOSESSION: windows_core::GUID = windows_core::GUID::from_u128(0x6346cc54_2cfc_4ad4_8224_d15837de7700);
pub const D3DAUTHENTICATEDCONFIGURE_ENCRYPTIONWHENACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0x41fff286_6ae0_4d43_9d55_a46e9efd158a);
pub const D3DAUTHENTICATEDCONFIGURE_INITIALIZE: windows_core::GUID = windows_core::GUID::from_u128(0x06114bdb_3523_470a_8dca_fbc2845154f0);
pub const D3DAUTHENTICATEDCONFIGURE_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0x50455658_3f47_4362_bf99_bfdfcde9ed29);
pub const D3DAUTHENTICATEDCONFIGURE_SHAREDRESOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x0772d047_1b40_48e8_9ca6_b5f510de9f01);
pub const D3DAUTHENTICATEDQUERY_ACCESSIBILITYATTRIBUTES: windows_core::GUID = windows_core::GUID::from_u128(0x6214d9d2_432c_4abb_9fce_216eea269e3b);
pub const D3DAUTHENTICATEDQUERY_CHANNELTYPE: windows_core::GUID = windows_core::GUID::from_u128(0xbc1b18a5_b1fb_42ab_bd94_b5828b4bf7be);
pub const D3DAUTHENTICATEDQUERY_CRYPTOSESSION: windows_core::GUID = windows_core::GUID::from_u128(0x2634499e_d018_4d74_ac17_7f724059528d);
pub const D3DAUTHENTICATEDQUERY_CURRENTENCRYPTIONWHENACCESSIBLE: windows_core::GUID = windows_core::GUID::from_u128(0xec1791c7_dad3_4f15_9ec3_faa93d60d4f0);
pub const D3DAUTHENTICATEDQUERY_DEVICEHANDLE: windows_core::GUID = windows_core::GUID::from_u128(0xec1c539d_8cff_4e2a_bcc4_f5692f99f480);
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUID: windows_core::GUID = windows_core::GUID::from_u128(0xf83a5958_e986_4bda_beb0_411f6a7a01b7);
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUIDCOUNT: windows_core::GUID = windows_core::GUID::from_u128(0xb30f7066_203c_4b07_93fc_ceaafd61241e);
pub const D3DAUTHENTICATEDQUERY_OUTPUTID: windows_core::GUID = windows_core::GUID::from_u128(0x839ddca3_9b4e_41e4_b053_892bd2a11ee7);
pub const D3DAUTHENTICATEDQUERY_OUTPUTIDCOUNT: windows_core::GUID = windows_core::GUID::from_u128(0x2c042b5e_8c07_46d5_aabe_8f75cbad4c31);
pub const D3DAUTHENTICATEDQUERY_PROTECTION: windows_core::GUID = windows_core::GUID::from_u128(0xa84eb584_c495_48aa_b94d_8bd2d6fbce05);
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESS: windows_core::GUID = windows_core::GUID::from_u128(0x649bbadb_f0f4_4639_a15b_24393fc3abac);
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESSCOUNT: windows_core::GUID = windows_core::GUID::from_u128(0x0db207b3_9450_46a6_82de_1b96d44f9cf2);
pub const D3DAUTHENTICATEDQUERY_UNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT: windows_core::GUID = windows_core::GUID::from_u128(0x012f0bd6_e662_4474_befd_aa53e5143c6d);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DBACKBUFFER_TYPE(pub i32);
pub const D3DBACKBUFFER_TYPE_LEFT: D3DBACKBUFFER_TYPE = D3DBACKBUFFER_TYPE(1i32);
pub const D3DBACKBUFFER_TYPE_MONO: D3DBACKBUFFER_TYPE = D3DBACKBUFFER_TYPE(0i32);
pub const D3DBACKBUFFER_TYPE_RIGHT: D3DBACKBUFFER_TYPE = D3DBACKBUFFER_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DBASISTYPE(pub i32);
pub const D3DBASIS_BEZIER: D3DBASISTYPE = D3DBASISTYPE(0i32);
pub const D3DBASIS_BSPLINE: D3DBASISTYPE = D3DBASISTYPE(1i32);
pub const D3DBASIS_CATMULL_ROM: D3DBASISTYPE = D3DBASISTYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DBLEND(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DBLENDOP(pub i32);
pub const D3DBLENDOP_ADD: D3DBLENDOP = D3DBLENDOP(1i32);
pub const D3DBLENDOP_MAX: D3DBLENDOP = D3DBLENDOP(5i32);
pub const D3DBLENDOP_MIN: D3DBLENDOP = D3DBLENDOP(4i32);
pub const D3DBLENDOP_REVSUBTRACT: D3DBLENDOP = D3DBLENDOP(3i32);
pub const D3DBLENDOP_SUBTRACT: D3DBLENDOP = D3DBLENDOP(2i32);
pub const D3DBLEND_BLENDFACTOR: D3DBLEND = D3DBLEND(14i32);
pub const D3DBLEND_BOTHINVSRCALPHA: D3DBLEND = D3DBLEND(13i32);
pub const D3DBLEND_BOTHSRCALPHA: D3DBLEND = D3DBLEND(12i32);
pub const D3DBLEND_DESTALPHA: D3DBLEND = D3DBLEND(7i32);
pub const D3DBLEND_DESTCOLOR: D3DBLEND = D3DBLEND(9i32);
pub const D3DBLEND_INVBLENDFACTOR: D3DBLEND = D3DBLEND(15i32);
pub const D3DBLEND_INVDESTALPHA: D3DBLEND = D3DBLEND(8i32);
pub const D3DBLEND_INVDESTCOLOR: D3DBLEND = D3DBLEND(10i32);
pub const D3DBLEND_INVSRCALPHA: D3DBLEND = D3DBLEND(6i32);
pub const D3DBLEND_INVSRCCOLOR: D3DBLEND = D3DBLEND(4i32);
pub const D3DBLEND_INVSRCCOLOR2: D3DBLEND = D3DBLEND(17i32);
pub const D3DBLEND_ONE: D3DBLEND = D3DBLEND(2i32);
pub const D3DBLEND_SRCALPHA: D3DBLEND = D3DBLEND(5i32);
pub const D3DBLEND_SRCALPHASAT: D3DBLEND = D3DBLEND(11i32);
pub const D3DBLEND_SRCCOLOR: D3DBLEND = D3DBLEND(3i32);
pub const D3DBLEND_SRCCOLOR2: D3DBLEND = D3DBLEND(16i32);
pub const D3DBLEND_ZERO: D3DBLEND = D3DBLEND(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DBOX {
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub Front: u32,
    pub Back: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DBRANCH {
    pub dwMask: u32,
    pub dwValue: u32,
    pub bNegate: windows_core::BOOL,
    pub dwOffset: u32,
}
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR: D3DBUSTYPE = D3DBUSTYPE(262144i32);
pub const D3DBUSIMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: D3DBUSTYPE = D3DBUSTYPE(327680i32);
pub const D3DBUSIMPL_MODIFIER_INSIDE_OF_CHIPSET: D3DBUSTYPE = D3DBUSTYPE(65536i32);
pub const D3DBUSIMPL_MODIFIER_NON_STANDARD: D3DBUSTYPE = D3DBUSTYPE(-2147483648i32);
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: D3DBUSTYPE = D3DBUSTYPE(131072i32);
pub const D3DBUSIMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: D3DBUSTYPE = D3DBUSTYPE(196608i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DBUSTYPE(pub i32);
pub const D3DBUSTYPE_AGP: D3DBUSTYPE = D3DBUSTYPE(4i32);
pub const D3DBUSTYPE_OTHER: D3DBUSTYPE = D3DBUSTYPE(0i32);
pub const D3DBUSTYPE_PCI: D3DBUSTYPE = D3DBUSTYPE(1i32);
pub const D3DBUSTYPE_PCIEXPRESS: D3DBUSTYPE = D3DBUSTYPE(3i32);
pub const D3DBUSTYPE_PCIX: D3DBUSTYPE = D3DBUSTYPE(2i32);
pub const D3DCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824i32;
pub const D3DCAPS2_CANCALIBRATEGAMMA: i32 = 1048576i32;
pub const D3DCAPS2_CANMANAGERESOURCE: i32 = 268435456i32;
pub const D3DCAPS2_CANSHARERESOURCE: i32 = -2147483648i32;
pub const D3DCAPS2_DYNAMICTEXTURES: i32 = 536870912i32;
pub const D3DCAPS2_FULLSCREENGAMMA: i32 = 131072i32;
pub const D3DCAPS2_RESERVED: i32 = 33554432i32;
pub const D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD: i32 = 32i32;
pub const D3DCAPS3_COPY_TO_SYSTEMMEM: i32 = 512i32;
pub const D3DCAPS3_COPY_TO_VIDMEM: i32 = 256i32;
pub const D3DCAPS3_DXVAHD: i32 = 1024i32;
pub const D3DCAPS3_DXVAHD_LIMITED: i32 = 2048i32;
pub const D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION: i32 = 128i32;
pub const D3DCAPS3_RESERVED: i32 = -2147483617i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCAPS9 {
    pub DeviceType: D3DDEVTYPE,
    pub AdapterOrdinal: u32,
    pub Caps: u32,
    pub Caps2: u32,
    pub Caps3: u32,
    pub PresentationIntervals: u32,
    pub CursorCaps: u32,
    pub DevCaps: u32,
    pub PrimitiveMiscCaps: u32,
    pub RasterCaps: u32,
    pub ZCmpCaps: u32,
    pub SrcBlendCaps: u32,
    pub DestBlendCaps: u32,
    pub AlphaCmpCaps: u32,
    pub ShadeCaps: u32,
    pub TextureCaps: u32,
    pub TextureFilterCaps: u32,
    pub CubeTextureFilterCaps: u32,
    pub VolumeTextureFilterCaps: u32,
    pub TextureAddressCaps: u32,
    pub VolumeTextureAddressCaps: u32,
    pub LineCaps: u32,
    pub MaxTextureWidth: u32,
    pub MaxTextureHeight: u32,
    pub MaxVolumeExtent: u32,
    pub MaxTextureRepeat: u32,
    pub MaxTextureAspectRatio: u32,
    pub MaxAnisotropy: u32,
    pub MaxVertexW: f32,
    pub GuardBandLeft: f32,
    pub GuardBandTop: f32,
    pub GuardBandRight: f32,
    pub GuardBandBottom: f32,
    pub ExtentsAdjust: f32,
    pub StencilCaps: u32,
    pub FVFCaps: u32,
    pub TextureOpCaps: u32,
    pub MaxTextureBlendStages: u32,
    pub MaxSimultaneousTextures: u32,
    pub VertexProcessingCaps: u32,
    pub MaxActiveLights: u32,
    pub MaxUserClipPlanes: u32,
    pub MaxVertexBlendMatrices: u32,
    pub MaxVertexBlendMatrixIndex: u32,
    pub MaxPointSize: f32,
    pub MaxPrimitiveCount: u32,
    pub MaxVertexIndex: u32,
    pub MaxStreams: u32,
    pub MaxStreamStride: u32,
    pub VertexShaderVersion: u32,
    pub MaxVertexShaderConst: u32,
    pub PixelShaderVersion: u32,
    pub PixelShader1xMaxValue: f32,
    pub DevCaps2: u32,
    pub MaxNpatchTessellationLevel: f32,
    pub Reserved5: u32,
    pub MasterAdapterOrdinal: u32,
    pub AdapterOrdinalInGroup: u32,
    pub NumberOfAdaptersInGroup: u32,
    pub DeclTypes: u32,
    pub NumSimultaneousRTs: u32,
    pub StretchRectFilterCaps: u32,
    pub VS20Caps: D3DVSHADERCAPS2_0,
    pub PS20Caps: D3DPSHADERCAPS2_0,
    pub VertexTextureFilterCaps: u32,
    pub MaxVShaderInstructionsExecuted: u32,
    pub MaxPShaderInstructionsExecuted: u32,
    pub MaxVertexShader30InstructionSlots: u32,
    pub MaxPixelShader30InstructionSlots: u32,
}
pub const D3DCAPS_OVERLAY: i32 = 2048i32;
pub const D3DCAPS_READ_SCANLINE: i32 = 131072i32;
pub const D3DCLEAR_STENCIL: i32 = 4i32;
pub const D3DCLEAR_TARGET: i32 = 1i32;
pub const D3DCLEAR_ZBUFFER: i32 = 2i32;
pub const D3DCLIPPLANE0: u32 = 1u32;
pub const D3DCLIPPLANE1: u32 = 2u32;
pub const D3DCLIPPLANE2: u32 = 4u32;
pub const D3DCLIPPLANE3: u32 = 8u32;
pub const D3DCLIPPLANE4: u32 = 16u32;
pub const D3DCLIPPLANE5: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCLIPSTATUS {
    pub dwFlags: u32,
    pub dwStatus: u32,
    pub minx: f32,
    pub maxx: f32,
    pub miny: f32,
    pub maxy: f32,
    pub minz: f32,
    pub maxz: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCLIPSTATUS9 {
    pub ClipUnion: u32,
    pub ClipIntersection: u32,
}
pub const D3DCLIPSTATUS_EXTENTS2: i32 = 2i32;
pub const D3DCLIPSTATUS_EXTENTS3: i32 = 4i32;
pub const D3DCLIPSTATUS_STATUS: i32 = 1i32;
pub const D3DCLIP_BACK: i32 = 32i32;
pub const D3DCLIP_BOTTOM: i32 = 8i32;
pub const D3DCLIP_FRONT: i32 = 16i32;
pub const D3DCLIP_GEN0: i32 = 64i32;
pub const D3DCLIP_GEN1: i32 = 128i32;
pub const D3DCLIP_GEN2: i32 = 256i32;
pub const D3DCLIP_GEN3: i32 = 512i32;
pub const D3DCLIP_GEN4: i32 = 1024i32;
pub const D3DCLIP_GEN5: i32 = 2048i32;
pub const D3DCLIP_LEFT: i32 = 1i32;
pub const D3DCLIP_RIGHT: i32 = 2i32;
pub const D3DCLIP_TOP: i32 = 4i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DCMPFUNC(pub i32);
pub const D3DCMP_ALWAYS: D3DCMPFUNC = D3DCMPFUNC(8i32);
pub const D3DCMP_EQUAL: D3DCMPFUNC = D3DCMPFUNC(3i32);
pub const D3DCMP_GREATER: D3DCMPFUNC = D3DCMPFUNC(5i32);
pub const D3DCMP_GREATEREQUAL: D3DCMPFUNC = D3DCMPFUNC(7i32);
pub const D3DCMP_LESS: D3DCMPFUNC = D3DCMPFUNC(2i32);
pub const D3DCMP_LESSEQUAL: D3DCMPFUNC = D3DCMPFUNC(4i32);
pub const D3DCMP_NEVER: D3DCMPFUNC = D3DCMPFUNC(1i32);
pub const D3DCMP_NOTEQUAL: D3DCMPFUNC = D3DCMPFUNC(6i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
pub const D3DCOLOR_MONO: u32 = 1u32;
pub const D3DCOLOR_RGB: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCOMPOSERECTDESC {
    pub X: u16,
    pub Y: u16,
    pub Width: u16,
    pub Height: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCOMPOSERECTDESTINATION {
    pub SrcRectIndex: u16,
    pub Reserved: u16,
    pub X: i16,
    pub Y: i16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DCOMPOSERECTSOP(pub i32);
pub const D3DCOMPOSERECTS_AND: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(3i32);
pub const D3DCOMPOSERECTS_COPY: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(1i32);
pub const D3DCOMPOSERECTS_MAXNUMRECTS: u32 = 65535u32;
pub const D3DCOMPOSERECTS_NEG: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(4i32);
pub const D3DCOMPOSERECTS_OR: D3DCOMPOSERECTSOP = D3DCOMPOSERECTSOP(2i32);
pub const D3DCONVOLUTIONMONO_MAXHEIGHT: u32 = 7u32;
pub const D3DCONVOLUTIONMONO_MAXWIDTH: u32 = 7u32;
pub const D3DCPCAPS_CONTENTKEY: u32 = 16u32;
pub const D3DCPCAPS_ENCRYPTEDREADBACK: u32 = 64u32;
pub const D3DCPCAPS_ENCRYPTEDREADBACKKEY: u32 = 128u32;
pub const D3DCPCAPS_ENCRYPTSLICEDATAONLY: u32 = 512u32;
pub const D3DCPCAPS_FRESHENSESSIONKEY: u32 = 32u32;
pub const D3DCPCAPS_HARDWARE: u32 = 2u32;
pub const D3DCPCAPS_PARTIALDECRYPTION: u32 = 8u32;
pub const D3DCPCAPS_PROTECTIONALWAYSON: u32 = 4u32;
pub const D3DCPCAPS_SEQUENTIAL_CTR_IV: u32 = 256u32;
pub const D3DCPCAPS_SOFTWARE: u32 = 1u32;
pub const D3DCREATE_ADAPTERGROUP_DEVICE: i32 = 512i32;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: i32 = 256i32;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: i32 = 1024i32;
pub const D3DCREATE_DISABLE_PRINTSCREEN: i32 = 32768i32;
pub const D3DCREATE_DISABLE_PSGP_THREADING: i32 = 8192i32;
pub const D3DCREATE_ENABLE_PRESENTSTATS: i32 = 16384i32;
pub const D3DCREATE_FPU_PRESERVE: i32 = 2i32;
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: i32 = 64i32;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: i32 = 128i32;
pub const D3DCREATE_MULTITHREADED: i32 = 4i32;
pub const D3DCREATE_NOWINDOWCHANGES: i32 = 2048i32;
pub const D3DCREATE_PUREDEVICE: i32 = 16i32;
pub const D3DCREATE_SCREENSAVER: i32 = 268435456i32;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: i32 = 32i32;
pub const D3DCRYPTOTYPE_AES128_CTR: windows_core::GUID = windows_core::GUID::from_u128(0x9b6bd711_4f74_41c9_9e7b_0be2d7d93b4f);
pub const D3DCRYPTOTYPE_PROPRIETARY: windows_core::GUID = windows_core::GUID::from_u128(0xab4e9afd_1d1c_46e6_a72f_0869917b0de8);
pub const D3DCS_BACK: i32 = 32i32;
pub const D3DCS_BOTTOM: i32 = 8i32;
pub const D3DCS_FRONT: i32 = 16i32;
pub const D3DCS_LEFT: i32 = 1i32;
pub const D3DCS_PLANE0: i32 = 64i32;
pub const D3DCS_PLANE1: i32 = 128i32;
pub const D3DCS_PLANE2: i32 = 256i32;
pub const D3DCS_PLANE3: i32 = 512i32;
pub const D3DCS_PLANE4: i32 = 1024i32;
pub const D3DCS_PLANE5: i32 = 2048i32;
pub const D3DCS_RIGHT: i32 = 2i32;
pub const D3DCS_TOP: i32 = 4i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DCUBEMAP_FACES(pub i32);
pub const D3DCUBEMAP_FACE_NEGATIVE_X: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(1i32);
pub const D3DCUBEMAP_FACE_NEGATIVE_Y: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(3i32);
pub const D3DCUBEMAP_FACE_NEGATIVE_Z: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(5i32);
pub const D3DCUBEMAP_FACE_POSITIVE_X: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(0i32);
pub const D3DCUBEMAP_FACE_POSITIVE_Y: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(2i32);
pub const D3DCUBEMAP_FACE_POSITIVE_Z: D3DCUBEMAP_FACES = D3DCUBEMAP_FACES(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DCULL(pub i32);
pub const D3DCULL_CCW: D3DCULL = D3DCULL(3i32);
pub const D3DCULL_CW: D3DCULL = D3DCULL(2i32);
pub const D3DCULL_NONE: D3DCULL = D3DCULL(1i32);
pub const D3DCURSORCAPS_COLOR: i32 = 1i32;
pub const D3DCURSORCAPS_LOWRES: i32 = 2i32;
pub const D3DCURSOR_IMMEDIATE_UPDATE: i32 = 1i32;
pub const D3DDD_BCLIPPING: i32 = 16i32;
pub const D3DDD_COLORMODEL: i32 = 1i32;
pub const D3DDD_DEVCAPS: i32 = 2i32;
pub const D3DDD_DEVICERENDERBITDEPTH: i32 = 128i32;
pub const D3DDD_DEVICEZBUFFERBITDEPTH: i32 = 256i32;
pub const D3DDD_LIGHTINGCAPS: i32 = 8i32;
pub const D3DDD_LINECAPS: i32 = 32i32;
pub const D3DDD_MAXBUFFERSIZE: i32 = 512i32;
pub const D3DDD_MAXVERTEXCOUNT: i32 = 1024i32;
pub const D3DDD_TRANSFORMCAPS: i32 = 4i32;
pub const D3DDD_TRICAPS: i32 = 64i32;
pub const D3DDEBCAPS_SYSTEMMEMORY: i32 = 1i32;
pub const D3DDEBCAPS_VIDEOMEMORY: i32 = 2i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDEBUGMONITORTOKENS(pub i32);
pub const D3DDEB_BUFSIZE: i32 = 1i32;
pub const D3DDEB_CAPS: i32 = 2i32;
pub const D3DDEB_LPDATA: i32 = 4i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDECLMETHOD(pub i32);
pub const D3DDECLMETHOD_CROSSUV: D3DDECLMETHOD = D3DDECLMETHOD(3i32);
pub const D3DDECLMETHOD_DEFAULT: D3DDECLMETHOD = D3DDECLMETHOD(0i32);
pub const D3DDECLMETHOD_LOOKUP: D3DDECLMETHOD = D3DDECLMETHOD(5i32);
pub const D3DDECLMETHOD_LOOKUPPRESAMPLED: D3DDECLMETHOD = D3DDECLMETHOD(6i32);
pub const D3DDECLMETHOD_PARTIALU: D3DDECLMETHOD = D3DDECLMETHOD(1i32);
pub const D3DDECLMETHOD_PARTIALV: D3DDECLMETHOD = D3DDECLMETHOD(2i32);
pub const D3DDECLMETHOD_UV: D3DDECLMETHOD = D3DDECLMETHOD(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDECLTYPE(pub i32);
pub const D3DDECLTYPE_D3DCOLOR: D3DDECLTYPE = D3DDECLTYPE(4i32);
pub const D3DDECLTYPE_DEC3N: D3DDECLTYPE = D3DDECLTYPE(14i32);
pub const D3DDECLTYPE_FLOAT1: D3DDECLTYPE = D3DDECLTYPE(0i32);
pub const D3DDECLTYPE_FLOAT16_2: D3DDECLTYPE = D3DDECLTYPE(15i32);
pub const D3DDECLTYPE_FLOAT16_4: D3DDECLTYPE = D3DDECLTYPE(16i32);
pub const D3DDECLTYPE_FLOAT2: D3DDECLTYPE = D3DDECLTYPE(1i32);
pub const D3DDECLTYPE_FLOAT3: D3DDECLTYPE = D3DDECLTYPE(2i32);
pub const D3DDECLTYPE_FLOAT4: D3DDECLTYPE = D3DDECLTYPE(3i32);
pub const D3DDECLTYPE_SHORT2: D3DDECLTYPE = D3DDECLTYPE(6i32);
pub const D3DDECLTYPE_SHORT2N: D3DDECLTYPE = D3DDECLTYPE(9i32);
pub const D3DDECLTYPE_SHORT4: D3DDECLTYPE = D3DDECLTYPE(7i32);
pub const D3DDECLTYPE_SHORT4N: D3DDECLTYPE = D3DDECLTYPE(10i32);
pub const D3DDECLTYPE_UBYTE4: D3DDECLTYPE = D3DDECLTYPE(5i32);
pub const D3DDECLTYPE_UBYTE4N: D3DDECLTYPE = D3DDECLTYPE(8i32);
pub const D3DDECLTYPE_UDEC3: D3DDECLTYPE = D3DDECLTYPE(13i32);
pub const D3DDECLTYPE_UNUSED: D3DDECLTYPE = D3DDECLTYPE(17i32);
pub const D3DDECLTYPE_USHORT2N: D3DDECLTYPE = D3DDECLTYPE(11i32);
pub const D3DDECLTYPE_USHORT4N: D3DDECLTYPE = D3DDECLTYPE(12i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDECLUSAGE(pub i32);
pub const D3DDECLUSAGE_BINORMAL: D3DDECLUSAGE = D3DDECLUSAGE(7i32);
pub const D3DDECLUSAGE_BLENDINDICES: D3DDECLUSAGE = D3DDECLUSAGE(2i32);
pub const D3DDECLUSAGE_BLENDWEIGHT: D3DDECLUSAGE = D3DDECLUSAGE(1i32);
pub const D3DDECLUSAGE_COLOR: D3DDECLUSAGE = D3DDECLUSAGE(10i32);
pub const D3DDECLUSAGE_DEPTH: D3DDECLUSAGE = D3DDECLUSAGE(12i32);
pub const D3DDECLUSAGE_FOG: D3DDECLUSAGE = D3DDECLUSAGE(11i32);
pub const D3DDECLUSAGE_NORMAL: D3DDECLUSAGE = D3DDECLUSAGE(3i32);
pub const D3DDECLUSAGE_POSITION: D3DDECLUSAGE = D3DDECLUSAGE(0i32);
pub const D3DDECLUSAGE_POSITIONT: D3DDECLUSAGE = D3DDECLUSAGE(9i32);
pub const D3DDECLUSAGE_PSIZE: D3DDECLUSAGE = D3DDECLUSAGE(4i32);
pub const D3DDECLUSAGE_SAMPLE: D3DDECLUSAGE = D3DDECLUSAGE(13i32);
pub const D3DDECLUSAGE_TANGENT: D3DDECLUSAGE = D3DDECLUSAGE(6i32);
pub const D3DDECLUSAGE_TESSFACTOR: D3DDECLUSAGE = D3DDECLUSAGE(8i32);
pub const D3DDECLUSAGE_TEXCOORD: D3DDECLUSAGE = D3DDECLUSAGE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDEGREETYPE(pub i32);
pub const D3DDEGREE_CUBIC: D3DDEGREETYPE = D3DDEGREETYPE(3i32);
pub const D3DDEGREE_LINEAR: D3DDEGREETYPE = D3DDEGREETYPE(1i32);
pub const D3DDEGREE_QUADRATIC: D3DDEGREETYPE = D3DDEGREETYPE(2i32);
pub const D3DDEGREE_QUINTIC: D3DDEGREETYPE = D3DDEGREETYPE(5i32);
pub const D3DDEVCAPS2_ADAPTIVETESSNPATCH: i32 = 8i32;
pub const D3DDEVCAPS2_ADAPTIVETESSRTPATCH: i32 = 4i32;
pub const D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES: i32 = 16i32;
pub const D3DDEVCAPS2_DMAPNPATCH: i32 = 2i32;
pub const D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH: i32 = 32i32;
pub const D3DDEVCAPS2_STREAMOFFSET: i32 = 1i32;
pub const D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET: i32 = 64i32;
pub const D3DDEVCAPS_CANBLTSYSTONONLOCAL: i32 = 131072i32;
pub const D3DDEVCAPS_CANRENDERAFTERFLIP: i32 = 2048i32;
pub const D3DDEVCAPS_DRAWPRIMITIVES2: i32 = 8192i32;
pub const D3DDEVCAPS_DRAWPRIMITIVES2EX: i32 = 32768i32;
pub const D3DDEVCAPS_DRAWPRIMTLVERTEX: i32 = 1024i32;
pub const D3DDEVCAPS_EXECUTESYSTEMMEMORY: i32 = 16i32;
pub const D3DDEVCAPS_EXECUTEVIDEOMEMORY: i32 = 32i32;
pub const D3DDEVCAPS_FLOATTLVERTEX: i32 = 1i32;
pub const D3DDEVCAPS_HWRASTERIZATION: i32 = 524288i32;
pub const D3DDEVCAPS_HWTRANSFORMANDLIGHT: i32 = 65536i32;
pub const D3DDEVCAPS_NPATCHES: i32 = 16777216i32;
pub const D3DDEVCAPS_PUREDEVICE: i32 = 1048576i32;
pub const D3DDEVCAPS_QUINTICRTPATCHES: i32 = 2097152i32;
pub const D3DDEVCAPS_RTPATCHES: i32 = 4194304i32;
pub const D3DDEVCAPS_RTPATCHHANDLEZERO: i32 = 8388608i32;
pub const D3DDEVCAPS_SEPARATETEXTUREMEMORIES: i32 = 16384i32;
pub const D3DDEVCAPS_SORTDECREASINGZ: i32 = 4i32;
pub const D3DDEVCAPS_SORTEXACT: i32 = 8i32;
pub const D3DDEVCAPS_SORTINCREASINGZ: i32 = 2i32;
pub const D3DDEVCAPS_TEXTURENONLOCALVIDMEM: i32 = 4096i32;
pub const D3DDEVCAPS_TEXTURESYSTEMMEMORY: i32 = 256i32;
pub const D3DDEVCAPS_TEXTUREVIDEOMEMORY: i32 = 512i32;
pub const D3DDEVCAPS_TLVERTEXSYSTEMMEMORY: i32 = 64i32;
pub const D3DDEVCAPS_TLVERTEXVIDEOMEMORY: i32 = 128i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVICEDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dcmColorModel: u32,
    pub dwDevCaps: u32,
    pub dtcTransformCaps: D3DTRANSFORMCAPS,
    pub bClipping: windows_core::BOOL,
    pub dlcLightingCaps: D3DLIGHTINGCAPS,
    pub dpcLineCaps: D3DPRIMCAPS,
    pub dpcTriCaps: D3DPRIMCAPS,
    pub dwDeviceRenderBitDepth: u32,
    pub dwDeviceZBufferBitDepth: u32,
    pub dwMaxBufferSize: u32,
    pub dwMaxVertexCount: u32,
    pub dwMinTextureWidth: u32,
    pub dwMinTextureHeight: u32,
    pub dwMaxTextureWidth: u32,
    pub dwMaxTextureHeight: u32,
    pub dwMinStippleWidth: u32,
    pub dwMaxStippleWidth: u32,
    pub dwMinStippleHeight: u32,
    pub dwMaxStippleHeight: u32,
    pub dwMaxTextureRepeat: u32,
    pub dwMaxTextureAspectRatio: u32,
    pub dwMaxAnisotropy: u32,
    pub dvGuardBandLeft: f32,
    pub dvGuardBandTop: f32,
    pub dvGuardBandRight: f32,
    pub dvGuardBandBottom: f32,
    pub dvExtentsAdjust: f32,
    pub dwStencilCaps: u32,
    pub dwFVFCaps: u32,
    pub dwTextureOpCaps: u32,
    pub wMaxTextureBlendStages: u16,
    pub wMaxSimultaneousTextures: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVICEDESC7 {
    pub dwDevCaps: u32,
    pub dpcLineCaps: D3DPRIMCAPS,
    pub dpcTriCaps: D3DPRIMCAPS,
    pub dwDeviceRenderBitDepth: u32,
    pub dwDeviceZBufferBitDepth: u32,
    pub dwMinTextureWidth: u32,
    pub dwMinTextureHeight: u32,
    pub dwMaxTextureWidth: u32,
    pub dwMaxTextureHeight: u32,
    pub dwMaxTextureRepeat: u32,
    pub dwMaxTextureAspectRatio: u32,
    pub dwMaxAnisotropy: u32,
    pub dvGuardBandLeft: f32,
    pub dvGuardBandTop: f32,
    pub dvGuardBandRight: f32,
    pub dvGuardBandBottom: f32,
    pub dvExtentsAdjust: f32,
    pub dwStencilCaps: u32,
    pub dwFVFCaps: u32,
    pub dwTextureOpCaps: u32,
    pub wMaxTextureBlendStages: u16,
    pub wMaxSimultaneousTextures: u16,
    pub dwMaxActiveLights: u32,
    pub dvMaxVertexW: f32,
    pub deviceGUID: windows_core::GUID,
    pub wMaxUserClipPlanes: u16,
    pub wMaxVertexBlendMatrices: u16,
    pub dwVertexProcessingCaps: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwReserved4: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVICE_CREATION_PARAMETERS {
    pub AdapterOrdinal: u32,
    pub DeviceType: D3DDEVTYPE,
    pub hFocusWindow: super::super::Foundation::HWND,
    pub BehaviorFlags: u32,
}
pub const D3DDEVINFOID_D3DTEXTUREMANAGER: u32 = 2u32;
pub const D3DDEVINFOID_TEXTUREMANAGER: u32 = 1u32;
pub const D3DDEVINFOID_TEXTURING: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    pub MaxBandwidthUtilized: f32,
    pub FrontEndUploadMemoryUtilizedPercent: f32,
    pub VertexRateUtilizedPercent: f32,
    pub TriangleSetupRateUtilizedPercent: f32,
    pub FillRateUtilizedPercent: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9CACHEUTILIZATION {
    pub TextureCacheHitRate: f32,
    pub PostTransformVertexCacheHitRate: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9INTERFACETIMINGS {
    pub WaitingForGPUToUseApplicationResourceTimePercent: f32,
    pub WaitingForGPUToAcceptMoreCommandsTimePercent: f32,
    pub WaitingForGPUToStayWithinLatencyTimePercent: f32,
    pub WaitingForGPUExclusiveResourceTimePercent: f32,
    pub WaitingForGPUOtherTimePercent: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9PIPELINETIMINGS {
    pub VertexProcessingTimePercent: f32,
    pub PixelProcessingTimePercent: f32,
    pub OtherGPUProcessingTimePercent: f32,
    pub GPUIdleTimePercent: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3D9STAGETIMINGS {
    pub MemoryProcessingPercent: f32,
    pub ComputationProcessingPercent: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_D3DVERTEXSTATS {
    pub NumRenderedTriangles: u32,
    pub NumExtraClippingTriangles: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DDEVINFO_RESOURCEMANAGER {
    pub stats: [D3DRESOURCESTATS; 8],
}
impl Default for D3DDEVINFO_RESOURCEMANAGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDEVINFO_VCACHE {
    pub Pattern: u32,
    pub OptMethod: u32,
    pub CacheSize: u32,
    pub MagicNumber: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDEVTYPE(pub i32);
pub const D3DDEVTYPE_HAL: D3DDEVTYPE = D3DDEVTYPE(1i32);
pub const D3DDEVTYPE_NULLREF: D3DDEVTYPE = D3DDEVTYPE(4i32);
pub const D3DDEVTYPE_REF: D3DDEVTYPE = D3DDEVTYPE(2i32);
pub const D3DDEVTYPE_SW: D3DDEVTYPE = D3DDEVTYPE(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDISPLAYMODE {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: u32,
    pub Format: D3DFORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDISPLAYMODEEX {
    pub Size: u32,
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: u32,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DDISPLAYMODEFILTER {
    pub Size: u32,
    pub Format: D3DFORMAT,
    pub ScanLineOrdering: D3DSCANLINEORDERING,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DDISPLAYROTATION(pub i32);
pub const D3DDISPLAYROTATION_180: D3DDISPLAYROTATION = D3DDISPLAYROTATION(3i32);
pub const D3DDISPLAYROTATION_270: D3DDISPLAYROTATION = D3DDISPLAYROTATION(4i32);
pub const D3DDISPLAYROTATION_90: D3DDISPLAYROTATION = D3DDISPLAYROTATION(2i32);
pub const D3DDISPLAYROTATION_IDENTITY: D3DDISPLAYROTATION = D3DDISPLAYROTATION(1i32);
pub const D3DDMAPSAMPLER: u32 = 256u32;
pub const D3DDMT_DISABLE: D3DDEBUGMONITORTOKENS = D3DDEBUGMONITORTOKENS(1i32);
pub const D3DDMT_ENABLE: D3DDEBUGMONITORTOKENS = D3DDEBUGMONITORTOKENS(0i32);
pub const D3DDP_MAXTEXCOORD: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DDP_PTRSTRIDE {
    pub lpvData: *mut core::ffi::c_void,
    pub dwStride: u32,
}
impl Default for D3DDP_PTRSTRIDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DDRAWPRIMITIVESTRIDEDDATA {
    pub position: D3DDP_PTRSTRIDE,
    pub normal: D3DDP_PTRSTRIDE,
    pub diffuse: D3DDP_PTRSTRIDE,
    pub specular: D3DDP_PTRSTRIDE,
    pub textureCoords: [D3DDP_PTRSTRIDE; 8],
}
impl Default for D3DDRAWPRIMITIVESTRIDEDDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DDTCAPS_DEC3N: i32 = 128i32;
pub const D3DDTCAPS_FLOAT16_2: i32 = 256i32;
pub const D3DDTCAPS_FLOAT16_4: i32 = 512i32;
pub const D3DDTCAPS_SHORT2N: i32 = 4i32;
pub const D3DDTCAPS_SHORT4N: i32 = 8i32;
pub const D3DDTCAPS_UBYTE4: i32 = 1i32;
pub const D3DDTCAPS_UBYTE4N: i32 = 2i32;
pub const D3DDTCAPS_UDEC3: i32 = 64i32;
pub const D3DDTCAPS_USHORT2N: i32 = 16i32;
pub const D3DDTCAPS_USHORT4N: i32 = 32i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: u32,
    pub NumBytesInSkipPattern: u32,
    pub NumBytesInEncryptPattern: u32,
}
pub const D3DENUM_NO_DRIVERVERSION: i32 = 4i32;
pub const D3DENUM_WHQL_LEVEL: i32 = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DEXECUTEBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwBufferSize: u32,
    pub lpData: *mut core::ffi::c_void,
}
impl Default for D3DEXECUTEBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DEXECUTEDATA {
    pub dwSize: u32,
    pub dwVertexOffset: u32,
    pub dwVertexCount: u32,
    pub dwInstructionOffset: u32,
    pub dwInstructionLength: u32,
    pub dwHVertexOffset: u32,
    pub dsStatus: D3DSTATUS,
}
pub const D3DEXECUTE_CLIPPED: i32 = 1i32;
pub const D3DEXECUTE_UNCLIPPED: i32 = 2i32;
pub const D3DFDS_ALPHACMPCAPS: i32 = 256i32;
pub const D3DFDS_COLORMODEL: i32 = 1i32;
pub const D3DFDS_DSTBLENDCAPS: i32 = 1024i32;
pub const D3DFDS_GUID: i32 = 2i32;
pub const D3DFDS_HARDWARE: i32 = 4i32;
pub const D3DFDS_LINES: i32 = 16i32;
pub const D3DFDS_MISCCAPS: i32 = 32i32;
pub const D3DFDS_RASTERCAPS: i32 = 64i32;
pub const D3DFDS_SHADECAPS: i32 = 2048i32;
pub const D3DFDS_SRCBLENDCAPS: i32 = 512i32;
pub const D3DFDS_TEXTUREADDRESSCAPS: i32 = 32768i32;
pub const D3DFDS_TEXTUREBLENDCAPS: i32 = 16384i32;
pub const D3DFDS_TEXTURECAPS: i32 = 4096i32;
pub const D3DFDS_TEXTUREFILTERCAPS: i32 = 8192i32;
pub const D3DFDS_TRIANGLES: i32 = 8i32;
pub const D3DFDS_ZCMPCAPS: i32 = 128i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DFILLMODE(pub i32);
pub const D3DFILL_POINT: D3DFILLMODE = D3DFILLMODE(1i32);
pub const D3DFILL_SOLID: D3DFILLMODE = D3DFILLMODE(3i32);
pub const D3DFILL_WIREFRAME: D3DFILLMODE = D3DFILLMODE(2i32);
pub const D3DFILTER_LINEAR: D3DTEXTUREFILTER = D3DTEXTUREFILTER(2i32);
pub const D3DFILTER_LINEARMIPLINEAR: D3DTEXTUREFILTER = D3DTEXTUREFILTER(6i32);
pub const D3DFILTER_LINEARMIPNEAREST: D3DTEXTUREFILTER = D3DTEXTUREFILTER(5i32);
pub const D3DFILTER_MIPLINEAR: D3DTEXTUREFILTER = D3DTEXTUREFILTER(4i32);
pub const D3DFILTER_MIPNEAREST: D3DTEXTUREFILTER = D3DTEXTUREFILTER(3i32);
pub const D3DFILTER_NEAREST: D3DTEXTUREFILTER = D3DTEXTUREFILTER(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DFINDDEVICERESULT {
    pub dwSize: u32,
    pub guid: windows_core::GUID,
    pub ddHwDesc: D3DDEVICEDESC,
    pub ddSwDesc: D3DDEVICEDESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DFINDDEVICESEARCH {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub bHardware: windows_core::BOOL,
    pub dcmColorModel: u32,
    pub guid: windows_core::GUID,
    pub dwCaps: u32,
    pub dpcPrimCaps: D3DPRIMCAPS,
}
pub const D3DFMT_A1: D3DFORMAT = D3DFORMAT(118u32);
pub const D3DFMT_A16B16G16R16: D3DFORMAT = D3DFORMAT(36u32);
pub const D3DFMT_A16B16G16R16F: D3DFORMAT = D3DFORMAT(113u32);
pub const D3DFMT_A1R5G5B5: D3DFORMAT = D3DFORMAT(25u32);
pub const D3DFMT_A1_SURFACE_MAXHEIGHT: u32 = 2048u32;
pub const D3DFMT_A1_SURFACE_MAXWIDTH: u32 = 8192u32;
pub const D3DFMT_A2B10G10R10: D3DFORMAT = D3DFORMAT(31u32);
pub const D3DFMT_A2B10G10R10_XR_BIAS: D3DFORMAT = D3DFORMAT(119u32);
pub const D3DFMT_A2R10G10B10: D3DFORMAT = D3DFORMAT(35u32);
pub const D3DFMT_A2W10V10U10: D3DFORMAT = D3DFORMAT(67u32);
pub const D3DFMT_A32B32G32R32F: D3DFORMAT = D3DFORMAT(116u32);
pub const D3DFMT_A4L4: D3DFORMAT = D3DFORMAT(52u32);
pub const D3DFMT_A4R4G4B4: D3DFORMAT = D3DFORMAT(26u32);
pub const D3DFMT_A8: D3DFORMAT = D3DFORMAT(28u32);
pub const D3DFMT_A8B8G8R8: D3DFORMAT = D3DFORMAT(32u32);
pub const D3DFMT_A8L8: D3DFORMAT = D3DFORMAT(51u32);
pub const D3DFMT_A8P8: D3DFORMAT = D3DFORMAT(40u32);
pub const D3DFMT_A8R3G3B2: D3DFORMAT = D3DFORMAT(29u32);
pub const D3DFMT_A8R8G8B8: D3DFORMAT = D3DFORMAT(21u32);
pub const D3DFMT_BINARYBUFFER: D3DFORMAT = D3DFORMAT(199u32);
pub const D3DFMT_CxV8U8: D3DFORMAT = D3DFORMAT(117u32);
pub const D3DFMT_D15S1: D3DFORMAT = D3DFORMAT(73u32);
pub const D3DFMT_D16: D3DFORMAT = D3DFORMAT(80u32);
pub const D3DFMT_D16_LOCKABLE: D3DFORMAT = D3DFORMAT(70u32);
pub const D3DFMT_D24FS8: D3DFORMAT = D3DFORMAT(83u32);
pub const D3DFMT_D24S8: D3DFORMAT = D3DFORMAT(75u32);
pub const D3DFMT_D24X4S4: D3DFORMAT = D3DFORMAT(79u32);
pub const D3DFMT_D24X8: D3DFORMAT = D3DFORMAT(77u32);
pub const D3DFMT_D32: D3DFORMAT = D3DFORMAT(71u32);
pub const D3DFMT_D32F_LOCKABLE: D3DFORMAT = D3DFORMAT(82u32);
pub const D3DFMT_D32_LOCKABLE: D3DFORMAT = D3DFORMAT(84u32);
pub const D3DFMT_DXT1: D3DFORMAT = D3DFORMAT(827611204u32);
pub const D3DFMT_DXT2: D3DFORMAT = D3DFORMAT(844388420u32);
pub const D3DFMT_DXT3: D3DFORMAT = D3DFORMAT(861165636u32);
pub const D3DFMT_DXT4: D3DFORMAT = D3DFORMAT(877942852u32);
pub const D3DFMT_DXT5: D3DFORMAT = D3DFORMAT(894720068u32);
pub const D3DFMT_G16R16: D3DFORMAT = D3DFORMAT(34u32);
pub const D3DFMT_G16R16F: D3DFORMAT = D3DFORMAT(112u32);
pub const D3DFMT_G32R32F: D3DFORMAT = D3DFORMAT(115u32);
pub const D3DFMT_G8R8_G8B8: D3DFORMAT = D3DFORMAT(1111970375u32);
pub const D3DFMT_INDEX16: D3DFORMAT = D3DFORMAT(101u32);
pub const D3DFMT_INDEX32: D3DFORMAT = D3DFORMAT(102u32);
pub const D3DFMT_L16: D3DFORMAT = D3DFORMAT(81u32);
pub const D3DFMT_L6V5U5: D3DFORMAT = D3DFORMAT(61u32);
pub const D3DFMT_L8: D3DFORMAT = D3DFORMAT(50u32);
pub const D3DFMT_MULTI2_ARGB8: D3DFORMAT = D3DFORMAT(827606349u32);
pub const D3DFMT_P8: D3DFORMAT = D3DFORMAT(41u32);
pub const D3DFMT_Q16W16V16U16: D3DFORMAT = D3DFORMAT(110u32);
pub const D3DFMT_Q8W8V8U8: D3DFORMAT = D3DFORMAT(63u32);
pub const D3DFMT_R16F: D3DFORMAT = D3DFORMAT(111u32);
pub const D3DFMT_R32F: D3DFORMAT = D3DFORMAT(114u32);
pub const D3DFMT_R3G3B2: D3DFORMAT = D3DFORMAT(27u32);
pub const D3DFMT_R5G6B5: D3DFORMAT = D3DFORMAT(23u32);
pub const D3DFMT_R8G8B8: D3DFORMAT = D3DFORMAT(20u32);
pub const D3DFMT_R8G8_B8G8: D3DFORMAT = D3DFORMAT(1195525970u32);
pub const D3DFMT_S8_LOCKABLE: D3DFORMAT = D3DFORMAT(85u32);
pub const D3DFMT_UNKNOWN: D3DFORMAT = D3DFORMAT(0u32);
pub const D3DFMT_UYVY: D3DFORMAT = D3DFORMAT(1498831189u32);
pub const D3DFMT_V16U16: D3DFORMAT = D3DFORMAT(64u32);
pub const D3DFMT_V8U8: D3DFORMAT = D3DFORMAT(60u32);
pub const D3DFMT_VERTEXDATA: D3DFORMAT = D3DFORMAT(100u32);
pub const D3DFMT_X1R5G5B5: D3DFORMAT = D3DFORMAT(24u32);
pub const D3DFMT_X4R4G4B4: D3DFORMAT = D3DFORMAT(30u32);
pub const D3DFMT_X8B8G8R8: D3DFORMAT = D3DFORMAT(33u32);
pub const D3DFMT_X8L8V8U8: D3DFORMAT = D3DFORMAT(62u32);
pub const D3DFMT_X8R8G8B8: D3DFORMAT = D3DFORMAT(22u32);
pub const D3DFMT_YUY2: D3DFORMAT = D3DFORMAT(844715353u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DFOGMODE(pub i32);
pub const D3DFOG_EXP: D3DFOGMODE = D3DFOGMODE(1i32);
pub const D3DFOG_EXP2: D3DFOGMODE = D3DFOGMODE(2i32);
pub const D3DFOG_LINEAR: D3DFOGMODE = D3DFOGMODE(3i32);
pub const D3DFOG_NONE: D3DFOGMODE = D3DFOGMODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DFORMAT(pub u32);
pub const D3DFVFCAPS_DONOTSTRIPELEMENTS: i32 = 524288i32;
pub const D3DFVFCAPS_PSIZE: i32 = 1048576i32;
pub const D3DFVFCAPS_TEXCOORDCOUNTMASK: i32 = 65535i32;
pub const D3DFVF_DIFFUSE: u32 = 64u32;
pub const D3DFVF_LASTBETA_D3DCOLOR: u32 = 32768u32;
pub const D3DFVF_LASTBETA_UBYTE4: u32 = 4096u32;
pub const D3DFVF_NORMAL: u32 = 16u32;
pub const D3DFVF_POSITION_MASK: u32 = 16398u32;
pub const D3DFVF_PSIZE: u32 = 32u32;
pub const D3DFVF_RESERVED0: u32 = 1u32;
pub const D3DFVF_RESERVED1: u32 = 32u32;
pub const D3DFVF_RESERVED2: u32 = 24576u32;
pub const D3DFVF_SPECULAR: u32 = 128u32;
pub const D3DFVF_TEX0: u32 = 0u32;
pub const D3DFVF_TEX1: u32 = 256u32;
pub const D3DFVF_TEX2: u32 = 512u32;
pub const D3DFVF_TEX3: u32 = 768u32;
pub const D3DFVF_TEX4: u32 = 1024u32;
pub const D3DFVF_TEX5: u32 = 1280u32;
pub const D3DFVF_TEX6: u32 = 1536u32;
pub const D3DFVF_TEX7: u32 = 1792u32;
pub const D3DFVF_TEX8: u32 = 2048u32;
pub const D3DFVF_TEXCOUNT_MASK: u32 = 3840u32;
pub const D3DFVF_TEXCOUNT_SHIFT: u32 = 8u32;
pub const D3DFVF_TEXTUREFORMAT1: u32 = 3u32;
pub const D3DFVF_TEXTUREFORMAT2: u32 = 0u32;
pub const D3DFVF_TEXTUREFORMAT3: u32 = 1u32;
pub const D3DFVF_TEXTUREFORMAT4: u32 = 2u32;
pub const D3DFVF_XYZ: u32 = 2u32;
pub const D3DFVF_XYZB1: u32 = 6u32;
pub const D3DFVF_XYZB2: u32 = 8u32;
pub const D3DFVF_XYZB3: u32 = 10u32;
pub const D3DFVF_XYZB4: u32 = 12u32;
pub const D3DFVF_XYZB5: u32 = 14u32;
pub const D3DFVF_XYZRHW: u32 = 4u32;
pub const D3DFVF_XYZW: u32 = 16386u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DGAMMARAMP {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}
impl Default for D3DGAMMARAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DGETDATA_FLUSH: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DHVERTEX {
    pub dwFlags: u32,
    pub Anonymous1: D3DHVERTEX_0,
    pub Anonymous2: D3DHVERTEX_1,
    pub Anonymous3: D3DHVERTEX_2,
}
impl Default for D3DHVERTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DHVERTEX_0 {
    pub hx: f32,
    pub dvHX: f32,
}
impl Default for D3DHVERTEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DHVERTEX_1 {
    pub hy: f32,
    pub dvHY: f32,
}
impl Default for D3DHVERTEX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DHVERTEX_2 {
    pub hz: f32,
    pub dvHZ: f32,
}
impl Default for D3DHVERTEX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DINDEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DINSTRUCTION {
    pub bOpcode: u8,
    pub bSize: u8,
    pub wCount: u16,
}
pub const D3DISSUE_BEGIN: u32 = 2u32;
pub const D3DISSUE_END: u32 = 1u32;
pub const D3DKEYEXCHANGE_DXVA: windows_core::GUID = windows_core::GUID::from_u128(0x43d3775c_38e5_4924_8d86_d3fccf153e9b);
pub const D3DKEYEXCHANGE_RSAES_OAEP: windows_core::GUID = windows_core::GUID::from_u128(0xc1949895_d72a_4a1d_8e5d_ed857d171520);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DLIGHT {
    pub dwSize: u32,
    pub dltType: D3DLIGHTTYPE,
    pub dcvColor: D3DCOLORVALUE,
    pub dvPosition: super::Direct3D::D3DVECTOR,
    pub dvDirection: super::Direct3D::D3DVECTOR,
    pub dvRange: f32,
    pub dvFalloff: f32,
    pub dvAttenuation0: f32,
    pub dvAttenuation1: f32,
    pub dvAttenuation2: f32,
    pub dvTheta: f32,
    pub dvPhi: f32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DLIGHT2 {
    pub dwSize: u32,
    pub dltType: D3DLIGHTTYPE,
    pub dcvColor: D3DCOLORVALUE,
    pub dvPosition: super::Direct3D::D3DVECTOR,
    pub dvDirection: super::Direct3D::D3DVECTOR,
    pub dvRange: f32,
    pub dvFalloff: f32,
    pub dvAttenuation0: f32,
    pub dvAttenuation1: f32,
    pub dvAttenuation2: f32,
    pub dvTheta: f32,
    pub dvPhi: f32,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DLIGHT7 {
    pub dltType: D3DLIGHTTYPE,
    pub dcvDiffuse: D3DCOLORVALUE,
    pub dcvSpecular: D3DCOLORVALUE,
    pub dcvAmbient: D3DCOLORVALUE,
    pub dvPosition: super::Direct3D::D3DVECTOR,
    pub dvDirection: super::Direct3D::D3DVECTOR,
    pub dvRange: f32,
    pub dvFalloff: f32,
    pub dvAttenuation0: f32,
    pub dvAttenuation1: f32,
    pub dvAttenuation2: f32,
    pub dvTheta: f32,
    pub dvPhi: f32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DLIGHT9 {
    pub Type: D3DLIGHTTYPE,
    pub Diffuse: D3DCOLORVALUE,
    pub Specular: D3DCOLORVALUE,
    pub Ambient: D3DCOLORVALUE,
    pub Position: super::Direct3D::D3DVECTOR,
    pub Direction: super::Direct3D::D3DVECTOR,
    pub Range: f32,
    pub Falloff: f32,
    pub Attenuation0: f32,
    pub Attenuation1: f32,
    pub Attenuation2: f32,
    pub Theta: f32,
    pub Phi: f32,
}
pub const D3DLIGHTCAPS_DIRECTIONAL: i32 = 4i32;
pub const D3DLIGHTCAPS_GLSPOT: i32 = 16i32;
pub const D3DLIGHTCAPS_PARALLELPOINT: i32 = 8i32;
pub const D3DLIGHTCAPS_POINT: i32 = 1i32;
pub const D3DLIGHTCAPS_SPOT: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DLIGHTDATA {
    pub dwSize: u32,
    pub lpIn: *mut D3DLIGHTINGELEMENT,
    pub dwInSize: u32,
    pub lpOut: *mut D3DTLVERTEX,
    pub dwOutSize: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl Default for D3DLIGHTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DLIGHTINGCAPS {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwLightingModel: u32,
    pub dwNumLights: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DLIGHTINGELEMENT {
    pub dvPosition: super::Direct3D::D3DVECTOR,
    pub dvNormal: super::Direct3D::D3DVECTOR,
}
pub const D3DLIGHTINGMODEL_MONO: i32 = 2i32;
pub const D3DLIGHTINGMODEL_RGB: i32 = 1i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DLIGHTSTATETYPE(pub i32);
pub const D3DLIGHTSTATE_AMBIENT: D3DLIGHTSTATETYPE = D3DLIGHTSTATETYPE(2i32);
pub const D3DLIGHTSTATE_COLORMODEL: D3DLIGHTSTATETYPE = D3DLIGHTSTATETYPE(3i32);
pub const D3DLIGHTSTATE_COLORVERTEX: D3DLIGHTSTATETYPE = D3DLIGHTSTATETYPE(8i32);
pub const D3DLIGHTSTATE_FOGDENSITY: D3DLIGHTSTATETYPE = D3DLIGHTSTATETYPE(7i32);
pub const D3DLIGHTSTATE_FOGEND: D3DLIGHTSTATETYPE = D3DLIGHTSTATETYPE(6i32);
pub const D3DLIGHTSTATE_FOGMODE: D3DLIGHTSTATETYPE = D3DLIGHTSTATETYPE(4i32);
pub const D3DLIGHTSTATE_FOGSTART: D3DLIGHTSTATETYPE = D3DLIGHTSTATETYPE(5i32);
pub const D3DLIGHTSTATE_MATERIAL: D3DLIGHTSTATETYPE = D3DLIGHTSTATETYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DLIGHTTYPE(pub i32);
pub const D3DLIGHT_ACTIVE: u32 = 1u32;
pub const D3DLIGHT_DIRECTIONAL: D3DLIGHTTYPE = D3DLIGHTTYPE(3i32);
pub const D3DLIGHT_NO_SPECULAR: u32 = 2u32;
pub const D3DLIGHT_POINT: D3DLIGHTTYPE = D3DLIGHTTYPE(1i32);
pub const D3DLIGHT_SPOT: D3DLIGHTTYPE = D3DLIGHTTYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DLINE {
    pub Anonymous1: D3DLINE_0,
    pub Anonymous2: D3DLINE_1,
}
impl Default for D3DLINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLINE_0 {
    pub v1: u16,
    pub wV1: u16,
}
impl Default for D3DLINE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLINE_1 {
    pub v2: u16,
    pub wV2: u16,
}
impl Default for D3DLINE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DLINECAPS_ALPHACMP: i32 = 8i32;
pub const D3DLINECAPS_ANTIALIAS: i32 = 32i32;
pub const D3DLINECAPS_BLEND: i32 = 4i32;
pub const D3DLINECAPS_FOG: i32 = 16i32;
pub const D3DLINECAPS_TEXTURE: i32 = 1i32;
pub const D3DLINECAPS_ZTEST: i32 = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DLOCKED_BOX {
    pub RowPitch: i32,
    pub SlicePitch: i32,
    pub pBits: *mut core::ffi::c_void,
}
impl Default for D3DLOCKED_BOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DLOCKED_RECT {
    pub Pitch: i32,
    pub pBits: *mut core::ffi::c_void,
}
impl Default for D3DLOCKED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DLOCK_DISCARD: i32 = 8192i32;
pub const D3DLOCK_DONOTWAIT: i32 = 16384i32;
pub const D3DLOCK_NOOVERWRITE: i32 = 4096i32;
pub const D3DLOCK_NOSYSLOCK: i32 = 2048i32;
pub const D3DLOCK_NO_DIRTY_UPDATE: i32 = 32768i32;
pub const D3DLOCK_READONLY: i32 = 16i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DLVERTEX {
    pub Anonymous1: D3DLVERTEX_0,
    pub Anonymous2: D3DLVERTEX_1,
    pub Anonymous3: D3DLVERTEX_2,
    pub dwReserved: u32,
    pub Anonymous4: D3DLVERTEX_3,
    pub Anonymous5: D3DLVERTEX_4,
    pub Anonymous6: D3DLVERTEX_5,
    pub Anonymous7: D3DLVERTEX_6,
}
impl Default for D3DLVERTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLVERTEX_0 {
    pub x: f32,
    pub dvX: f32,
}
impl Default for D3DLVERTEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLVERTEX_1 {
    pub y: f32,
    pub dvY: f32,
}
impl Default for D3DLVERTEX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLVERTEX_2 {
    pub z: f32,
    pub dvZ: f32,
}
impl Default for D3DLVERTEX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLVERTEX_3 {
    pub color: u32,
    pub dcColor: u32,
}
impl Default for D3DLVERTEX_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLVERTEX_4 {
    pub specular: u32,
    pub dcSpecular: u32,
}
impl Default for D3DLVERTEX_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLVERTEX_5 {
    pub tu: f32,
    pub dvTU: f32,
}
impl Default for D3DLVERTEX_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DLVERTEX_6 {
    pub tv: f32,
    pub dvTV: f32,
}
impl Default for D3DLVERTEX_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DMATERIAL {
    pub dwSize: u32,
    pub Anonymous1: D3DMATERIAL_0,
    pub Anonymous2: D3DMATERIAL_1,
    pub Anonymous3: D3DMATERIAL_2,
    pub Anonymous4: D3DMATERIAL_3,
    pub Anonymous5: D3DMATERIAL_4,
    pub hTexture: u32,
    pub dwRampSize: u32,
}
impl Default for D3DMATERIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL_0 {
    pub diffuse: D3DCOLORVALUE,
    pub dcvDiffuse: D3DCOLORVALUE,
}
impl Default for D3DMATERIAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL_1 {
    pub ambient: D3DCOLORVALUE,
    pub dcvAmbient: D3DCOLORVALUE,
}
impl Default for D3DMATERIAL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL_2 {
    pub specular: D3DCOLORVALUE,
    pub dcvSpecular: D3DCOLORVALUE,
}
impl Default for D3DMATERIAL_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL_3 {
    pub emissive: D3DCOLORVALUE,
    pub dcvEmissive: D3DCOLORVALUE,
}
impl Default for D3DMATERIAL_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL_4 {
    pub power: f32,
    pub dvPower: f32,
}
impl Default for D3DMATERIAL_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DMATERIAL7 {
    pub Anonymous1: D3DMATERIAL7_0,
    pub Anonymous2: D3DMATERIAL7_1,
    pub Anonymous3: D3DMATERIAL7_2,
    pub Anonymous4: D3DMATERIAL7_3,
    pub Anonymous5: D3DMATERIAL7_4,
}
impl Default for D3DMATERIAL7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL7_0 {
    pub diffuse: D3DCOLORVALUE,
    pub dcvDiffuse: D3DCOLORVALUE,
}
impl Default for D3DMATERIAL7_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL7_1 {
    pub ambient: D3DCOLORVALUE,
    pub dcvAmbient: D3DCOLORVALUE,
}
impl Default for D3DMATERIAL7_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL7_2 {
    pub specular: D3DCOLORVALUE,
    pub dcvSpecular: D3DCOLORVALUE,
}
impl Default for D3DMATERIAL7_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL7_3 {
    pub emissive: D3DCOLORVALUE,
    pub dcvEmissive: D3DCOLORVALUE,
}
impl Default for D3DMATERIAL7_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DMATERIAL7_4 {
    pub power: f32,
    pub dvPower: f32,
}
impl Default for D3DMATERIAL7_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DMATERIAL9 {
    pub Diffuse: D3DCOLORVALUE,
    pub Ambient: D3DCOLORVALUE,
    pub Specular: D3DCOLORVALUE,
    pub Emissive: D3DCOLORVALUE,
    pub Power: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DMATERIALCOLORSOURCE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DMATRIXLOAD {
    pub hDestMatrix: u32,
    pub hSrcMatrix: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DMATRIXMULTIPLY {
    pub hDestMatrix: u32,
    pub hSrcMatrix1: u32,
    pub hSrcMatrix2: u32,
}
pub const D3DMAX30SHADERINSTRUCTIONS: u32 = 32768u32;
pub const D3DMAXUSERCLIPPLANES: u32 = 32u32;
pub const D3DMCS_COLOR1: D3DMATERIALCOLORSOURCE = D3DMATERIALCOLORSOURCE(1i32);
pub const D3DMCS_COLOR2: D3DMATERIALCOLORSOURCE = D3DMATERIALCOLORSOURCE(2i32);
pub const D3DMCS_MATERIAL: D3DMATERIALCOLORSOURCE = D3DMATERIALCOLORSOURCE(0i32);
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: u64,
    pub SizeOfInefficientAllocation: u64,
    pub LevelOfEfficiency: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DMEMORYPRESSURE {
    pub BytesEvictedFromProcess: u64,
    pub SizeOfInefficientAllocation: u64,
    pub LevelOfEfficiency: u32,
}
pub const D3DMIN30SHADERINSTRUCTIONS: u32 = 512u32;
pub const D3DMP_16: D3DSHADER_MIN_PRECISION = D3DSHADER_MIN_PRECISION(1i32);
pub const D3DMP_2_8: D3DSHADER_MIN_PRECISION = D3DSHADER_MIN_PRECISION(2i32);
pub const D3DMP_DEFAULT: D3DSHADER_MIN_PRECISION = D3DSHADER_MIN_PRECISION(0i32);
pub const D3DMULTISAMPLE_10_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(10i32);
pub const D3DMULTISAMPLE_11_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(11i32);
pub const D3DMULTISAMPLE_12_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(12i32);
pub const D3DMULTISAMPLE_13_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(13i32);
pub const D3DMULTISAMPLE_14_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(14i32);
pub const D3DMULTISAMPLE_15_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(15i32);
pub const D3DMULTISAMPLE_16_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(16i32);
pub const D3DMULTISAMPLE_2_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(2i32);
pub const D3DMULTISAMPLE_3_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(3i32);
pub const D3DMULTISAMPLE_4_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(4i32);
pub const D3DMULTISAMPLE_5_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(5i32);
pub const D3DMULTISAMPLE_6_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(6i32);
pub const D3DMULTISAMPLE_7_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(7i32);
pub const D3DMULTISAMPLE_8_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(8i32);
pub const D3DMULTISAMPLE_9_SAMPLES: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(9i32);
pub const D3DMULTISAMPLE_NONE: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(0i32);
pub const D3DMULTISAMPLE_NONMASKABLE: D3DMULTISAMPLE_TYPE = D3DMULTISAMPLE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DMULTISAMPLE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DOPCODE(pub i32);
pub const D3DOP_BRANCHFORWARD: D3DOPCODE = D3DOPCODE(12i32);
pub const D3DOP_EXIT: D3DOPCODE = D3DOPCODE(11i32);
pub const D3DOP_LINE: D3DOPCODE = D3DOPCODE(2i32);
pub const D3DOP_MATRIXLOAD: D3DOPCODE = D3DOPCODE(4i32);
pub const D3DOP_MATRIXMULTIPLY: D3DOPCODE = D3DOPCODE(5i32);
pub const D3DOP_POINT: D3DOPCODE = D3DOPCODE(1i32);
pub const D3DOP_PROCESSVERTICES: D3DOPCODE = D3DOPCODE(9i32);
pub const D3DOP_SETSTATUS: D3DOPCODE = D3DOPCODE(14i32);
pub const D3DOP_SPAN: D3DOPCODE = D3DOPCODE(13i32);
pub const D3DOP_STATELIGHT: D3DOPCODE = D3DOPCODE(7i32);
pub const D3DOP_STATERENDER: D3DOPCODE = D3DOPCODE(8i32);
pub const D3DOP_STATETRANSFORM: D3DOPCODE = D3DOPCODE(6i32);
pub const D3DOP_TEXTURELOAD: D3DOPCODE = D3DOPCODE(10i32);
pub const D3DOP_TRIANGLE: D3DOPCODE = D3DOPCODE(3i32);
pub const D3DOVERLAYCAPS_FULLRANGERGB: u32 = 1u32;
pub const D3DOVERLAYCAPS_LIMITEDRANGERGB: u32 = 2u32;
pub const D3DOVERLAYCAPS_STRETCHX: u32 = 64u32;
pub const D3DOVERLAYCAPS_STRETCHY: u32 = 128u32;
pub const D3DOVERLAYCAPS_YCbCr_BT601: u32 = 4u32;
pub const D3DOVERLAYCAPS_YCbCr_BT601_xvYCC: u32 = 16u32;
pub const D3DOVERLAYCAPS_YCbCr_BT709: u32 = 8u32;
pub const D3DOVERLAYCAPS_YCbCr_BT709_xvYCC: u32 = 32u32;
pub const D3DPAL_FREE: u32 = 0u32;
pub const D3DPAL_READONLY: u32 = 64u32;
pub const D3DPAL_RESERVED: u32 = 128u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DPATCHEDGESTYLE(pub i32);
pub const D3DPATCHEDGE_CONTINUOUS: D3DPATCHEDGESTYLE = D3DPATCHEDGESTYLE(1i32);
pub const D3DPATCHEDGE_DISCRETE: D3DPATCHEDGESTYLE = D3DPATCHEDGESTYLE(0i32);
pub const D3DPBLENDCAPS_BLENDFACTOR: i32 = 8192i32;
pub const D3DPBLENDCAPS_BOTHINVSRCALPHA: i32 = 4096i32;
pub const D3DPBLENDCAPS_BOTHSRCALPHA: i32 = 2048i32;
pub const D3DPBLENDCAPS_DESTALPHA: i32 = 64i32;
pub const D3DPBLENDCAPS_DESTCOLOR: i32 = 256i32;
pub const D3DPBLENDCAPS_INVDESTALPHA: i32 = 128i32;
pub const D3DPBLENDCAPS_INVDESTCOLOR: i32 = 512i32;
pub const D3DPBLENDCAPS_INVSRCALPHA: i32 = 32i32;
pub const D3DPBLENDCAPS_INVSRCCOLOR: i32 = 8i32;
pub const D3DPBLENDCAPS_INVSRCCOLOR2: i32 = 32768i32;
pub const D3DPBLENDCAPS_ONE: i32 = 2i32;
pub const D3DPBLENDCAPS_SRCALPHA: i32 = 16i32;
pub const D3DPBLENDCAPS_SRCALPHASAT: i32 = 1024i32;
pub const D3DPBLENDCAPS_SRCCOLOR: i32 = 4i32;
pub const D3DPBLENDCAPS_SRCCOLOR2: i32 = 16384i32;
pub const D3DPBLENDCAPS_ZERO: i32 = 1i32;
pub const D3DPCMPCAPS_ALWAYS: i32 = 128i32;
pub const D3DPCMPCAPS_EQUAL: i32 = 4i32;
pub const D3DPCMPCAPS_GREATER: i32 = 16i32;
pub const D3DPCMPCAPS_GREATEREQUAL: i32 = 64i32;
pub const D3DPCMPCAPS_LESS: i32 = 2i32;
pub const D3DPCMPCAPS_LESSEQUAL: i32 = 8i32;
pub const D3DPCMPCAPS_NEVER: i32 = 1i32;
pub const D3DPCMPCAPS_NOTEQUAL: i32 = 32i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DPICKRECORD {
    pub bOpcode: u8,
    pub bPad: u8,
    pub dwOffset: u32,
    pub dvZ: f32,
}
pub const D3DPMISCCAPS_BLENDOP: i32 = 2048i32;
pub const D3DPMISCCAPS_CLIPPLANESCALEDPOINTS: i32 = 256i32;
pub const D3DPMISCCAPS_CLIPTLVERTS: i32 = 512i32;
pub const D3DPMISCCAPS_COLORWRITEENABLE: i32 = 128i32;
pub const D3DPMISCCAPS_CONFORMANT: i32 = 8i32;
pub const D3DPMISCCAPS_CULLCCW: i32 = 64i32;
pub const D3DPMISCCAPS_CULLCW: i32 = 32i32;
pub const D3DPMISCCAPS_CULLNONE: i32 = 16i32;
pub const D3DPMISCCAPS_FOGANDSPECULARALPHA: i32 = 65536i32;
pub const D3DPMISCCAPS_FOGVERTEXCLAMPED: i32 = 1048576i32;
pub const D3DPMISCCAPS_INDEPENDENTWRITEMASKS: i32 = 16384i32;
pub const D3DPMISCCAPS_LINEPATTERNREP: i32 = 4i32;
pub const D3DPMISCCAPS_MASKPLANES: i32 = 1i32;
pub const D3DPMISCCAPS_MASKZ: i32 = 2i32;
pub const D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS: i32 = 262144i32;
pub const D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING: i32 = 524288i32;
pub const D3DPMISCCAPS_NULLREFERENCE: i32 = 4096i32;
pub const D3DPMISCCAPS_PERSTAGECONSTANT: i32 = 32768i32;
pub const D3DPMISCCAPS_POSTBLENDSRGBCONVERT: i32 = 2097152i32;
pub const D3DPMISCCAPS_SEPARATEALPHABLEND: i32 = 131072i32;
pub const D3DPMISCCAPS_TSSARGTEMP: i32 = 1024i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DPOINT {
    pub wCount: u16,
    pub wFirst: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DPOOL(pub i32);
pub const D3DPOOL_DEFAULT: D3DPOOL = D3DPOOL(0i32);
pub const D3DPOOL_MANAGED: D3DPOOL = D3DPOOL(1i32);
pub const D3DPOOL_SCRATCH: D3DPOOL = D3DPOOL(3i32);
pub const D3DPOOL_SYSTEMMEM: D3DPOOL = D3DPOOL(2i32);
pub const D3DPRASTERCAPS_ANISOTROPY: i32 = 131072i32;
pub const D3DPRASTERCAPS_ANTIALIASEDGES: i32 = 4096i32;
pub const D3DPRASTERCAPS_ANTIALIASSORTDEPENDENT: i32 = 1024i32;
pub const D3DPRASTERCAPS_ANTIALIASSORTINDEPENDENT: i32 = 2048i32;
pub const D3DPRASTERCAPS_COLORPERSPECTIVE: i32 = 4194304i32;
pub const D3DPRASTERCAPS_DEPTHBIAS: i32 = 67108864i32;
pub const D3DPRASTERCAPS_DITHER: i32 = 1i32;
pub const D3DPRASTERCAPS_FOGRANGE: i32 = 65536i32;
pub const D3DPRASTERCAPS_FOGTABLE: i32 = 256i32;
pub const D3DPRASTERCAPS_FOGVERTEX: i32 = 128i32;
pub const D3DPRASTERCAPS_MIPMAPLODBIAS: i32 = 8192i32;
pub const D3DPRASTERCAPS_MULTISAMPLE_TOGGLE: i32 = 134217728i32;
pub const D3DPRASTERCAPS_PAT: i32 = 8i32;
pub const D3DPRASTERCAPS_ROP2: i32 = 2i32;
pub const D3DPRASTERCAPS_SCISSORTEST: i32 = 16777216i32;
pub const D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS: i32 = 33554432i32;
pub const D3DPRASTERCAPS_STIPPLE: i32 = 512i32;
pub const D3DPRASTERCAPS_SUBPIXEL: i32 = 32i32;
pub const D3DPRASTERCAPS_SUBPIXELX: i32 = 64i32;
pub const D3DPRASTERCAPS_TRANSLUCENTSORTINDEPENDENT: i32 = 524288i32;
pub const D3DPRASTERCAPS_WBUFFER: i32 = 262144i32;
pub const D3DPRASTERCAPS_WFOG: i32 = 1048576i32;
pub const D3DPRASTERCAPS_XOR: i32 = 4i32;
pub const D3DPRASTERCAPS_ZBIAS: i32 = 16384i32;
pub const D3DPRASTERCAPS_ZBUFFERLESSHSR: i32 = 32768i32;
pub const D3DPRASTERCAPS_ZFOG: i32 = 2097152i32;
pub const D3DPRASTERCAPS_ZTEST: i32 = 16i32;
pub const D3DPRESENTFLAG_DEVICECLIP: u32 = 4u32;
pub const D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL: u32 = 2u32;
pub const D3DPRESENTFLAG_LOCKABLE_BACKBUFFER: u32 = 1u32;
pub const D3DPRESENTFLAG_NOAUTOROTATE: u32 = 32u32;
pub const D3DPRESENTFLAG_OVERLAY_LIMITEDRGB: u32 = 128u32;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_BT709: u32 = 256u32;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC: u32 = 512u32;
pub const D3DPRESENTFLAG_RESTRICTED_CONTENT: u32 = 1024u32;
pub const D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER: u32 = 2048u32;
pub const D3DPRESENTFLAG_UNPRUNEDMODE: u32 = 64u32;
pub const D3DPRESENTFLAG_VIDEO: u32 = 16u32;
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct D3DPRESENTSTATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DPRESENTSTATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
pub const D3DPRESENT_BACK_BUFFERS_MAX: i32 = 3i32;
pub const D3DPRESENT_BACK_BUFFERS_MAX_EX: i32 = 30i32;
pub const D3DPRESENT_DONOTFLIP: i32 = 4i32;
pub const D3DPRESENT_DONOTWAIT: i32 = 1i32;
pub const D3DPRESENT_FLIPRESTART: i32 = 8i32;
pub const D3DPRESENT_FORCEIMMEDIATE: i32 = 256i32;
pub const D3DPRESENT_HIDEOVERLAY: i32 = 64i32;
pub const D3DPRESENT_INTERVAL_DEFAULT: i32 = 0i32;
pub const D3DPRESENT_INTERVAL_FOUR: i32 = 8i32;
pub const D3DPRESENT_INTERVAL_IMMEDIATE: i32 = -2147483648i32;
pub const D3DPRESENT_INTERVAL_ONE: i32 = 1i32;
pub const D3DPRESENT_INTERVAL_THREE: i32 = 4i32;
pub const D3DPRESENT_INTERVAL_TWO: i32 = 2i32;
pub const D3DPRESENT_LINEAR_CONTENT: i32 = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DPRESENT_PARAMETERS {
    pub BackBufferWidth: u32,
    pub BackBufferHeight: u32,
    pub BackBufferFormat: D3DFORMAT,
    pub BackBufferCount: u32,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: u32,
    pub SwapEffect: D3DSWAPEFFECT,
    pub hDeviceWindow: super::super::Foundation::HWND,
    pub Windowed: windows_core::BOOL,
    pub EnableAutoDepthStencil: windows_core::BOOL,
    pub AutoDepthStencilFormat: D3DFORMAT,
    pub Flags: u32,
    pub FullScreen_RefreshRateInHz: u32,
    pub PresentationInterval: u32,
}
pub const D3DPRESENT_RATE_DEFAULT: u32 = 0u32;
pub const D3DPRESENT_UPDATECOLORKEY: i32 = 128i32;
pub const D3DPRESENT_UPDATEOVERLAYONLY: i32 = 32i32;
pub const D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR: i32 = 16i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DPRIMCAPS {
    pub dwSize: u32,
    pub dwMiscCaps: u32,
    pub dwRasterCaps: u32,
    pub dwZCmpCaps: u32,
    pub dwSrcBlendCaps: u32,
    pub dwDestBlendCaps: u32,
    pub dwAlphaCmpCaps: u32,
    pub dwShadeCaps: u32,
    pub dwTextureCaps: u32,
    pub dwTextureFilterCaps: u32,
    pub dwTextureBlendCaps: u32,
    pub dwTextureAddressCaps: u32,
    pub dwStippleWidth: u32,
    pub dwStippleHeight: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DPRIMITIVETYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DPROCESSVERTICES {
    pub dwFlags: u32,
    pub wStart: u16,
    pub wDest: u16,
    pub dwCount: u32,
    pub dwReserved: u32,
}
pub const D3DPROCESSVERTICES_COPY: i32 = 2i32;
pub const D3DPROCESSVERTICES_NOCOLOR: i32 = 16i32;
pub const D3DPROCESSVERTICES_OPMASK: i32 = 7i32;
pub const D3DPROCESSVERTICES_TRANSFORM: i32 = 1i32;
pub const D3DPROCESSVERTICES_TRANSFORMLIGHT: i32 = 0i32;
pub const D3DPROCESSVERTICES_UPDATEEXTENTS: i32 = 8i32;
pub const D3DPS20CAPS_ARBITRARYSWIZZLE: u32 = 1u32;
pub const D3DPS20CAPS_GRADIENTINSTRUCTIONS: u32 = 2u32;
pub const D3DPS20CAPS_NODEPENDENTREADLIMIT: u32 = 8u32;
pub const D3DPS20CAPS_NOTEXINSTRUCTIONLIMIT: u32 = 16u32;
pub const D3DPS20CAPS_PREDICATION: u32 = 4u32;
pub const D3DPS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24u32;
pub const D3DPS20_MAX_NUMINSTRUCTIONSLOTS: u32 = 512u32;
pub const D3DPS20_MAX_NUMTEMPS: u32 = 32u32;
pub const D3DPS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4u32;
pub const D3DPS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0u32;
pub const D3DPS20_MIN_NUMINSTRUCTIONSLOTS: u32 = 96u32;
pub const D3DPS20_MIN_NUMTEMPS: u32 = 12u32;
pub const D3DPS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 0u32;
pub const D3DPSHADECAPS_ALPHAFLATBLEND: i32 = 4096i32;
pub const D3DPSHADECAPS_ALPHAFLATSTIPPLED: i32 = 8192i32;
pub const D3DPSHADECAPS_ALPHAGOURAUDBLEND: i32 = 16384i32;
pub const D3DPSHADECAPS_ALPHAGOURAUDSTIPPLED: i32 = 32768i32;
pub const D3DPSHADECAPS_ALPHAPHONGBLEND: i32 = 65536i32;
pub const D3DPSHADECAPS_ALPHAPHONGSTIPPLED: i32 = 131072i32;
pub const D3DPSHADECAPS_COLORFLATMONO: i32 = 1i32;
pub const D3DPSHADECAPS_COLORFLATRGB: i32 = 2i32;
pub const D3DPSHADECAPS_COLORGOURAUDMONO: i32 = 4i32;
pub const D3DPSHADECAPS_COLORGOURAUDRGB: i32 = 8i32;
pub const D3DPSHADECAPS_COLORPHONGMONO: i32 = 16i32;
pub const D3DPSHADECAPS_COLORPHONGRGB: i32 = 32i32;
pub const D3DPSHADECAPS_FOGFLAT: i32 = 262144i32;
pub const D3DPSHADECAPS_FOGGOURAUD: i32 = 524288i32;
pub const D3DPSHADECAPS_FOGPHONG: i32 = 1048576i32;
pub const D3DPSHADECAPS_SPECULARFLATMONO: i32 = 64i32;
pub const D3DPSHADECAPS_SPECULARFLATRGB: i32 = 128i32;
pub const D3DPSHADECAPS_SPECULARGOURAUDMONO: i32 = 256i32;
pub const D3DPSHADECAPS_SPECULARGOURAUDRGB: i32 = 512i32;
pub const D3DPSHADECAPS_SPECULARPHONGMONO: i32 = 1024i32;
pub const D3DPSHADECAPS_SPECULARPHONGRGB: i32 = 2048i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DPSHADERCAPS2_0 {
    pub Caps: u32,
    pub DynamicFlowControlDepth: i32,
    pub NumTemps: i32,
    pub StaticFlowControlDepth: i32,
    pub NumInstructionSlots: i32,
}
pub const D3DPTADDRESSCAPS_BORDER: i32 = 8i32;
pub const D3DPTADDRESSCAPS_CLAMP: i32 = 4i32;
pub const D3DPTADDRESSCAPS_INDEPENDENTUV: i32 = 16i32;
pub const D3DPTADDRESSCAPS_MIRROR: i32 = 2i32;
pub const D3DPTADDRESSCAPS_MIRRORONCE: i32 = 32i32;
pub const D3DPTADDRESSCAPS_WRAP: i32 = 1i32;
pub const D3DPTBLENDCAPS_ADD: i32 = 128i32;
pub const D3DPTBLENDCAPS_COPY: i32 = 64i32;
pub const D3DPTBLENDCAPS_DECAL: i32 = 1i32;
pub const D3DPTBLENDCAPS_DECALALPHA: i32 = 4i32;
pub const D3DPTBLENDCAPS_DECALMASK: i32 = 16i32;
pub const D3DPTBLENDCAPS_MODULATE: i32 = 2i32;
pub const D3DPTBLENDCAPS_MODULATEALPHA: i32 = 8i32;
pub const D3DPTBLENDCAPS_MODULATEMASK: i32 = 32i32;
pub const D3DPTEXTURECAPS_ALPHA: i32 = 4i32;
pub const D3DPTEXTURECAPS_ALPHAPALETTE: i32 = 128i32;
pub const D3DPTEXTURECAPS_BORDER: i32 = 16i32;
pub const D3DPTEXTURECAPS_COLORKEYBLEND: i32 = 4096i32;
pub const D3DPTEXTURECAPS_CUBEMAP: i32 = 2048i32;
pub const D3DPTEXTURECAPS_CUBEMAP_POW2: i32 = 131072i32;
pub const D3DPTEXTURECAPS_MIPCUBEMAP: i32 = 65536i32;
pub const D3DPTEXTURECAPS_MIPMAP: i32 = 16384i32;
pub const D3DPTEXTURECAPS_MIPVOLUMEMAP: i32 = 32768i32;
pub const D3DPTEXTURECAPS_NONPOW2CONDITIONAL: i32 = 256i32;
pub const D3DPTEXTURECAPS_NOPROJECTEDBUMPENV: i32 = 2097152i32;
pub const D3DPTEXTURECAPS_PERSPECTIVE: i32 = 1i32;
pub const D3DPTEXTURECAPS_POW2: i32 = 2i32;
pub const D3DPTEXTURECAPS_PROJECTED: i32 = 1024i32;
pub const D3DPTEXTURECAPS_SQUAREONLY: i32 = 32i32;
pub const D3DPTEXTURECAPS_TEXREPEATNOTSCALEDBYSIZE: i32 = 64i32;
pub const D3DPTEXTURECAPS_TRANSPARENCY: i32 = 8i32;
pub const D3DPTEXTURECAPS_VOLUMEMAP: i32 = 8192i32;
pub const D3DPTEXTURECAPS_VOLUMEMAP_POW2: i32 = 262144i32;
pub const D3DPTFILTERCAPS_CONVOLUTIONMONO: i32 = 262144i32;
pub const D3DPTFILTERCAPS_LINEAR: i32 = 2i32;
pub const D3DPTFILTERCAPS_LINEARMIPLINEAR: i32 = 32i32;
pub const D3DPTFILTERCAPS_LINEARMIPNEAREST: i32 = 16i32;
pub const D3DPTFILTERCAPS_MAGFAFLATCUBIC: i32 = 134217728i32;
pub const D3DPTFILTERCAPS_MAGFANISOTROPIC: i32 = 67108864i32;
pub const D3DPTFILTERCAPS_MAGFGAUSSIANCUBIC: i32 = 268435456i32;
pub const D3DPTFILTERCAPS_MAGFGAUSSIANQUAD: i32 = 268435456i32;
pub const D3DPTFILTERCAPS_MAGFLINEAR: i32 = 33554432i32;
pub const D3DPTFILTERCAPS_MAGFPOINT: i32 = 16777216i32;
pub const D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD: i32 = 134217728i32;
pub const D3DPTFILTERCAPS_MINFANISOTROPIC: i32 = 1024i32;
pub const D3DPTFILTERCAPS_MINFGAUSSIANQUAD: i32 = 4096i32;
pub const D3DPTFILTERCAPS_MINFLINEAR: i32 = 512i32;
pub const D3DPTFILTERCAPS_MINFPOINT: i32 = 256i32;
pub const D3DPTFILTERCAPS_MINFPYRAMIDALQUAD: i32 = 2048i32;
pub const D3DPTFILTERCAPS_MIPFLINEAR: i32 = 131072i32;
pub const D3DPTFILTERCAPS_MIPFPOINT: i32 = 65536i32;
pub const D3DPTFILTERCAPS_MIPLINEAR: i32 = 8i32;
pub const D3DPTFILTERCAPS_MIPNEAREST: i32 = 4i32;
pub const D3DPTFILTERCAPS_NEAREST: i32 = 1i32;
pub const D3DPT_LINELIST: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(2i32);
pub const D3DPT_LINESTRIP: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(3i32);
pub const D3DPT_POINTLIST: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(1i32);
pub const D3DPT_TRIANGLEFAN: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(6i32);
pub const D3DPT_TRIANGLELIST: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(4i32);
pub const D3DPT_TRIANGLESTRIP: D3DPRIMITIVETYPE = D3DPRIMITIVETYPE(5i32);
pub const D3DPV_DONOTCOPYDATA: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DQUERYTYPE(pub i32);
pub const D3DQUERYTYPE_BANDWIDTHTIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(17i32);
pub const D3DQUERYTYPE_CACHEUTILIZATION: D3DQUERYTYPE = D3DQUERYTYPE(18i32);
pub const D3DQUERYTYPE_EVENT: D3DQUERYTYPE = D3DQUERYTYPE(8i32);
pub const D3DQUERYTYPE_INTERFACETIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(14i32);
pub const D3DQUERYTYPE_MEMORYPRESSURE: D3DQUERYTYPE = D3DQUERYTYPE(19i32);
pub const D3DQUERYTYPE_OCCLUSION: D3DQUERYTYPE = D3DQUERYTYPE(9i32);
pub const D3DQUERYTYPE_PIPELINETIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(13i32);
pub const D3DQUERYTYPE_PIXELTIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(16i32);
pub const D3DQUERYTYPE_RESOURCEMANAGER: D3DQUERYTYPE = D3DQUERYTYPE(5i32);
pub const D3DQUERYTYPE_TIMESTAMP: D3DQUERYTYPE = D3DQUERYTYPE(10i32);
pub const D3DQUERYTYPE_TIMESTAMPDISJOINT: D3DQUERYTYPE = D3DQUERYTYPE(11i32);
pub const D3DQUERYTYPE_TIMESTAMPFREQ: D3DQUERYTYPE = D3DQUERYTYPE(12i32);
pub const D3DQUERYTYPE_VCACHE: D3DQUERYTYPE = D3DQUERYTYPE(4i32);
pub const D3DQUERYTYPE_VERTEXSTATS: D3DQUERYTYPE = D3DQUERYTYPE(6i32);
pub const D3DQUERYTYPE_VERTEXTIMINGS: D3DQUERYTYPE = D3DQUERYTYPE(15i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DRANGE {
    pub Offset: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DRASTER_STATUS {
    pub InVBlank: windows_core::BOOL,
    pub ScanLine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DRECT {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DRECTPATCH_INFO {
    pub StartVertexOffsetWidth: u32,
    pub StartVertexOffsetHeight: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DRENDERSTATETYPE(pub i32);
pub const D3DRENDERSTATE_WRAPBIAS: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DRESOURCESTATS {
    pub bThrashing: windows_core::BOOL,
    pub ApproxBytesDownloaded: u32,
    pub NumEvicts: u32,
    pub NumVidCreates: u32,
    pub LastPri: u32,
    pub NumUsed: u32,
    pub NumUsedInVidMem: u32,
    pub WorkingSet: u32,
    pub WorkingSetBytes: u32,
    pub TotalManaged: u32,
    pub TotalBytes: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DRESOURCETYPE(pub i32);
pub const D3DRS_ADAPTIVETESS_W: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(183i32);
pub const D3DRS_ADAPTIVETESS_X: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(180i32);
pub const D3DRS_ADAPTIVETESS_Y: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(181i32);
pub const D3DRS_ADAPTIVETESS_Z: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(182i32);
pub const D3DRS_ALPHABLENDENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(27i32);
pub const D3DRS_ALPHAFUNC: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(25i32);
pub const D3DRS_ALPHAREF: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(24i32);
pub const D3DRS_ALPHATESTENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(15i32);
pub const D3DRS_AMBIENT: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(139i32);
pub const D3DRS_AMBIENTMATERIALSOURCE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(147i32);
pub const D3DRS_ANTIALIASEDLINEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(176i32);
pub const D3DRS_BLENDFACTOR: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(193i32);
pub const D3DRS_BLENDOP: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(171i32);
pub const D3DRS_BLENDOPALPHA: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(209i32);
pub const D3DRS_CCW_STENCILFAIL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(186i32);
pub const D3DRS_CCW_STENCILFUNC: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(189i32);
pub const D3DRS_CCW_STENCILPASS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(188i32);
pub const D3DRS_CCW_STENCILZFAIL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(187i32);
pub const D3DRS_CLIPPING: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(136i32);
pub const D3DRS_CLIPPLANEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(152i32);
pub const D3DRS_COLORVERTEX: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(141i32);
pub const D3DRS_COLORWRITEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(168i32);
pub const D3DRS_COLORWRITEENABLE1: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(190i32);
pub const D3DRS_COLORWRITEENABLE2: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(191i32);
pub const D3DRS_COLORWRITEENABLE3: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(192i32);
pub const D3DRS_CULLMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(22i32);
pub const D3DRS_DEBUGMONITORTOKEN: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(165i32);
pub const D3DRS_DEPTHBIAS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(195i32);
pub const D3DRS_DESTBLEND: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(20i32);
pub const D3DRS_DESTBLENDALPHA: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(208i32);
pub const D3DRS_DIFFUSEMATERIALSOURCE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(145i32);
pub const D3DRS_DITHERENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(26i32);
pub const D3DRS_EMISSIVEMATERIALSOURCE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(148i32);
pub const D3DRS_ENABLEADAPTIVETESSELLATION: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(184i32);
pub const D3DRS_FILLMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(8i32);
pub const D3DRS_FOGCOLOR: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(34i32);
pub const D3DRS_FOGDENSITY: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(38i32);
pub const D3DRS_FOGENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(28i32);
pub const D3DRS_FOGEND: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(37i32);
pub const D3DRS_FOGSTART: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(36i32);
pub const D3DRS_FOGTABLEMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(35i32);
pub const D3DRS_FOGVERTEXMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(140i32);
pub const D3DRS_INDEXEDVERTEXBLENDENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(167i32);
pub const D3DRS_LASTPIXEL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(16i32);
pub const D3DRS_LIGHTING: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(137i32);
pub const D3DRS_LOCALVIEWER: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(142i32);
pub const D3DRS_MAXTESSELLATIONLEVEL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(179i32);
pub const D3DRS_MINTESSELLATIONLEVEL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(178i32);
pub const D3DRS_MULTISAMPLEANTIALIAS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(161i32);
pub const D3DRS_MULTISAMPLEMASK: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(162i32);
pub const D3DRS_NORMALDEGREE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(173i32);
pub const D3DRS_NORMALIZENORMALS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(143i32);
pub const D3DRS_PATCHEDGESTYLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(163i32);
pub const D3DRS_POINTSCALEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(157i32);
pub const D3DRS_POINTSCALE_A: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(158i32);
pub const D3DRS_POINTSCALE_B: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(159i32);
pub const D3DRS_POINTSCALE_C: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(160i32);
pub const D3DRS_POINTSIZE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(154i32);
pub const D3DRS_POINTSIZE_MAX: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(166i32);
pub const D3DRS_POINTSIZE_MIN: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(155i32);
pub const D3DRS_POINTSPRITEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(156i32);
pub const D3DRS_POSITIONDEGREE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(172i32);
pub const D3DRS_RANGEFOGENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(48i32);
pub const D3DRS_SCISSORTESTENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(174i32);
pub const D3DRS_SEPARATEALPHABLENDENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(206i32);
pub const D3DRS_SHADEMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(9i32);
pub const D3DRS_SLOPESCALEDEPTHBIAS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(175i32);
pub const D3DRS_SPECULARENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(29i32);
pub const D3DRS_SPECULARMATERIALSOURCE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(146i32);
pub const D3DRS_SRCBLEND: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(19i32);
pub const D3DRS_SRCBLENDALPHA: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(207i32);
pub const D3DRS_SRGBWRITEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(194i32);
pub const D3DRS_STENCILENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(52i32);
pub const D3DRS_STENCILFAIL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(53i32);
pub const D3DRS_STENCILFUNC: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(56i32);
pub const D3DRS_STENCILMASK: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(58i32);
pub const D3DRS_STENCILPASS: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(55i32);
pub const D3DRS_STENCILREF: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(57i32);
pub const D3DRS_STENCILWRITEMASK: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(59i32);
pub const D3DRS_STENCILZFAIL: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(54i32);
pub const D3DRS_TEXTUREFACTOR: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(60i32);
pub const D3DRS_TWEENFACTOR: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(170i32);
pub const D3DRS_TWOSIDEDSTENCILMODE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(185i32);
pub const D3DRS_VERTEXBLEND: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(151i32);
pub const D3DRS_WRAP0: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(128i32);
pub const D3DRS_WRAP1: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(129i32);
pub const D3DRS_WRAP10: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(200i32);
pub const D3DRS_WRAP11: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(201i32);
pub const D3DRS_WRAP12: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(202i32);
pub const D3DRS_WRAP13: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(203i32);
pub const D3DRS_WRAP14: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(204i32);
pub const D3DRS_WRAP15: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(205i32);
pub const D3DRS_WRAP2: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(130i32);
pub const D3DRS_WRAP3: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(131i32);
pub const D3DRS_WRAP4: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(132i32);
pub const D3DRS_WRAP5: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(133i32);
pub const D3DRS_WRAP6: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(134i32);
pub const D3DRS_WRAP7: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(135i32);
pub const D3DRS_WRAP8: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(198i32);
pub const D3DRS_WRAP9: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(199i32);
pub const D3DRS_ZENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(7i32);
pub const D3DRS_ZFUNC: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(23i32);
pub const D3DRS_ZWRITEENABLE: D3DRENDERSTATETYPE = D3DRENDERSTATETYPE(14i32);
pub const D3DRTYPECOUNT: u32 = 8u32;
pub const D3DRTYPE_CUBETEXTURE: D3DRESOURCETYPE = D3DRESOURCETYPE(5i32);
pub const D3DRTYPE_INDEXBUFFER: D3DRESOURCETYPE = D3DRESOURCETYPE(7i32);
pub const D3DRTYPE_SURFACE: D3DRESOURCETYPE = D3DRESOURCETYPE(1i32);
pub const D3DRTYPE_TEXTURE: D3DRESOURCETYPE = D3DRESOURCETYPE(3i32);
pub const D3DRTYPE_VERTEXBUFFER: D3DRESOURCETYPE = D3DRESOURCETYPE(6i32);
pub const D3DRTYPE_VOLUME: D3DRESOURCETYPE = D3DRESOURCETYPE(2i32);
pub const D3DRTYPE_VOLUMETEXTURE: D3DRESOURCETYPE = D3DRESOURCETYPE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSAMPLERSTATETYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSAMPLER_TEXTURE_TYPE(pub i32);
pub const D3DSAMP_ADDRESSU: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(1i32);
pub const D3DSAMP_ADDRESSV: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(2i32);
pub const D3DSAMP_ADDRESSW: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(3i32);
pub const D3DSAMP_BORDERCOLOR: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(4i32);
pub const D3DSAMP_DMAPOFFSET: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(13i32);
pub const D3DSAMP_ELEMENTINDEX: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(12i32);
pub const D3DSAMP_MAGFILTER: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(5i32);
pub const D3DSAMP_MAXANISOTROPY: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(10i32);
pub const D3DSAMP_MAXMIPLEVEL: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(9i32);
pub const D3DSAMP_MINFILTER: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(6i32);
pub const D3DSAMP_MIPFILTER: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(7i32);
pub const D3DSAMP_MIPMAPLODBIAS: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(8i32);
pub const D3DSAMP_SRGBTEXTURE: D3DSAMPLERSTATETYPE = D3DSAMPLERSTATETYPE(11i32);
pub const D3DSBT_ALL: D3DSTATEBLOCKTYPE = D3DSTATEBLOCKTYPE(1i32);
pub const D3DSBT_PIXELSTATE: D3DSTATEBLOCKTYPE = D3DSTATEBLOCKTYPE(2i32);
pub const D3DSBT_VERTEXSTATE: D3DSTATEBLOCKTYPE = D3DSTATEBLOCKTYPE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSCANLINEORDERING(pub i32);
pub const D3DSCANLINEORDERING_INTERLACED: D3DSCANLINEORDERING = D3DSCANLINEORDERING(2i32);
pub const D3DSCANLINEORDERING_PROGRESSIVE: D3DSCANLINEORDERING = D3DSCANLINEORDERING(1i32);
pub const D3DSCANLINEORDERING_UNKNOWN: D3DSCANLINEORDERING = D3DSCANLINEORDERING(0i32);
pub const D3DSETSTATUS_EXTENTS: i32 = 2i32;
pub const D3DSETSTATUS_STATUS: i32 = 1i32;
pub const D3DSGR_CALIBRATE: i32 = 1i32;
pub const D3DSGR_NO_CALIBRATION: i32 = 0i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSHADEMODE(pub i32);
pub const D3DSHADER_ADDRESSMODE_SHIFT: u32 = 13u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSHADER_ADDRESSMODE_TYPE(pub i32);
pub const D3DSHADER_ADDRMODE_ABSOLUTE: D3DSHADER_ADDRESSMODE_TYPE = D3DSHADER_ADDRESSMODE_TYPE(0i32);
pub const D3DSHADER_ADDRMODE_RELATIVE: D3DSHADER_ADDRESSMODE_TYPE = D3DSHADER_ADDRESSMODE_TYPE(8192i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSHADER_COMPARISON(pub i32);
pub const D3DSHADER_COMPARISON_SHIFT: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSHADER_INSTRUCTION_OPCODE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSHADER_MIN_PRECISION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSHADER_MISCTYPE_OFFSETS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSHADER_PARAM_REGISTER_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSHADER_PARAM_SRCMOD_TYPE(pub i32);
pub const D3DSHADE_FLAT: D3DSHADEMODE = D3DSHADEMODE(1i32);
pub const D3DSHADE_GOURAUD: D3DSHADEMODE = D3DSHADEMODE(2i32);
pub const D3DSHADE_PHONG: D3DSHADEMODE = D3DSHADEMODE(3i32);
pub const D3DSIO_ABS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(35i32);
pub const D3DSIO_ADD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(2i32);
pub const D3DSIO_BEM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(89i32);
pub const D3DSIO_BREAK: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(44i32);
pub const D3DSIO_BREAKC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(45i32);
pub const D3DSIO_BREAKP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(96i32);
pub const D3DSIO_CALL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(25i32);
pub const D3DSIO_CALLNZ: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(26i32);
pub const D3DSIO_CMP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(88i32);
pub const D3DSIO_CND: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(80i32);
pub const D3DSIO_COMMENT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(65534i32);
pub const D3DSIO_CRS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(33i32);
pub const D3DSIO_DCL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(31i32);
pub const D3DSIO_DEF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(81i32);
pub const D3DSIO_DEFB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(47i32);
pub const D3DSIO_DEFI: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(48i32);
pub const D3DSIO_DP2ADD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(90i32);
pub const D3DSIO_DP3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(8i32);
pub const D3DSIO_DP4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(9i32);
pub const D3DSIO_DST: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(17i32);
pub const D3DSIO_DSX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(91i32);
pub const D3DSIO_DSY: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(92i32);
pub const D3DSIO_ELSE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(42i32);
pub const D3DSIO_END: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(65535i32);
pub const D3DSIO_ENDIF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(43i32);
pub const D3DSIO_ENDLOOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(29i32);
pub const D3DSIO_ENDREP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(39i32);
pub const D3DSIO_EXP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(14i32);
pub const D3DSIO_EXPP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(78i32);
pub const D3DSIO_FRC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(19i32);
pub const D3DSIO_IF: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(40i32);
pub const D3DSIO_IFC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(41i32);
pub const D3DSIO_LABEL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(30i32);
pub const D3DSIO_LIT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(16i32);
pub const D3DSIO_LOG: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(15i32);
pub const D3DSIO_LOGP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(79i32);
pub const D3DSIO_LOOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(27i32);
pub const D3DSIO_LRP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(18i32);
pub const D3DSIO_M3x2: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(24i32);
pub const D3DSIO_M3x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(23i32);
pub const D3DSIO_M3x4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(22i32);
pub const D3DSIO_M4x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(21i32);
pub const D3DSIO_M4x4: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(20i32);
pub const D3DSIO_MAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(4i32);
pub const D3DSIO_MAX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(11i32);
pub const D3DSIO_MIN: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(10i32);
pub const D3DSIO_MOV: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(1i32);
pub const D3DSIO_MOVA: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(46i32);
pub const D3DSIO_MUL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(5i32);
pub const D3DSIO_NOP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(0i32);
pub const D3DSIO_NRM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(36i32);
pub const D3DSIO_PHASE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(65533i32);
pub const D3DSIO_POW: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(32i32);
pub const D3DSIO_RCP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(6i32);
pub const D3DSIO_REP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(38i32);
pub const D3DSIO_RESERVED0: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(75i32);
pub const D3DSIO_RET: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(28i32);
pub const D3DSIO_RSQ: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(7i32);
pub const D3DSIO_SETP: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(94i32);
pub const D3DSIO_SGE: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(13i32);
pub const D3DSIO_SGN: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(34i32);
pub const D3DSIO_SINCOS: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(37i32);
pub const D3DSIO_SLT: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(12i32);
pub const D3DSIO_SUB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(3i32);
pub const D3DSIO_TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(66i32);
pub const D3DSIO_TEXBEM: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(67i32);
pub const D3DSIO_TEXBEML: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(68i32);
pub const D3DSIO_TEXCOORD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(64i32);
pub const D3DSIO_TEXDEPTH: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(87i32);
pub const D3DSIO_TEXDP3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(85i32);
pub const D3DSIO_TEXDP3TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(83i32);
pub const D3DSIO_TEXKILL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(65i32);
pub const D3DSIO_TEXLDD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(93i32);
pub const D3DSIO_TEXLDL: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(95i32);
pub const D3DSIO_TEXM3x2DEPTH: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(84i32);
pub const D3DSIO_TEXM3x2PAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(71i32);
pub const D3DSIO_TEXM3x2TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(72i32);
pub const D3DSIO_TEXM3x3: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(86i32);
pub const D3DSIO_TEXM3x3PAD: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(73i32);
pub const D3DSIO_TEXM3x3SPEC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(76i32);
pub const D3DSIO_TEXM3x3TEX: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(74i32);
pub const D3DSIO_TEXM3x3VSPEC: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(77i32);
pub const D3DSIO_TEXREG2AR: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(69i32);
pub const D3DSIO_TEXREG2GB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(70i32);
pub const D3DSIO_TEXREG2RGB: D3DSHADER_INSTRUCTION_OPCODE_TYPE = D3DSHADER_INSTRUCTION_OPCODE_TYPE(82i32);
pub const D3DSI_COISSUE: u32 = 1073741824u32;
pub const D3DSI_COMMENTSIZE_MASK: u32 = 2147418112u32;
pub const D3DSI_COMMENTSIZE_SHIFT: u32 = 16u32;
pub const D3DSI_INSTLENGTH_MASK: u32 = 251658240u32;
pub const D3DSI_INSTLENGTH_SHIFT: u32 = 24u32;
pub const D3DSI_OPCODE_MASK: u32 = 65535u32;
pub const D3DSMO_FACE: D3DSHADER_MISCTYPE_OFFSETS = D3DSHADER_MISCTYPE_OFFSETS(1i32);
pub const D3DSMO_POSITION: D3DSHADER_MISCTYPE_OFFSETS = D3DSHADER_MISCTYPE_OFFSETS(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DSPAN {
    pub wCount: u16,
    pub wFirst: u16,
}
pub const D3DSPC_EQ: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(2i32);
pub const D3DSPC_GE: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(3i32);
pub const D3DSPC_GT: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(1i32);
pub const D3DSPC_LE: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(6i32);
pub const D3DSPC_LT: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(4i32);
pub const D3DSPC_NE: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(5i32);
pub const D3DSPC_RESERVED0: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(0i32);
pub const D3DSPC_RESERVED1: D3DSHADER_COMPARISON = D3DSHADER_COMPARISON(7i32);
pub const D3DSPD_IUNKNOWN: i32 = 1i32;
pub const D3DSPR_ADDR: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(3i32);
pub const D3DSPR_ATTROUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(5i32);
pub const D3DSPR_COLOROUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(8i32);
pub const D3DSPR_CONST: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(2i32);
pub const D3DSPR_CONST2: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(11i32);
pub const D3DSPR_CONST3: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(12i32);
pub const D3DSPR_CONST4: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(13i32);
pub const D3DSPR_CONSTBOOL: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(14i32);
pub const D3DSPR_CONSTINT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(7i32);
pub const D3DSPR_DEPTHOUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(9i32);
pub const D3DSPR_INPUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(1i32);
pub const D3DSPR_LABEL: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(18i32);
pub const D3DSPR_LOOP: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(15i32);
pub const D3DSPR_MISCTYPE: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(17i32);
pub const D3DSPR_OUTPUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(6i32);
pub const D3DSPR_PREDICATE: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(19i32);
pub const D3DSPR_RASTOUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(4i32);
pub const D3DSPR_SAMPLER: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(10i32);
pub const D3DSPR_TEMP: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(0i32);
pub const D3DSPR_TEMPFLOAT16: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(16i32);
pub const D3DSPR_TEXCRDOUT: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(6i32);
pub const D3DSPR_TEXTURE: D3DSHADER_PARAM_REGISTER_TYPE = D3DSHADER_PARAM_REGISTER_TYPE(3i32);
pub const D3DSPSM_ABS: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(184549376i32);
pub const D3DSPSM_ABSNEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(201326592i32);
pub const D3DSPSM_BIAS: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(33554432i32);
pub const D3DSPSM_BIASNEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(50331648i32);
pub const D3DSPSM_COMP: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(100663296i32);
pub const D3DSPSM_DW: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(167772160i32);
pub const D3DSPSM_DZ: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(150994944i32);
pub const D3DSPSM_NEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(16777216i32);
pub const D3DSPSM_NONE: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(0i32);
pub const D3DSPSM_NOT: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(218103808i32);
pub const D3DSPSM_SIGN: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(67108864i32);
pub const D3DSPSM_SIGNNEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(83886080i32);
pub const D3DSPSM_X2: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(117440512i32);
pub const D3DSPSM_X2NEG: D3DSHADER_PARAM_SRCMOD_TYPE = D3DSHADER_PARAM_SRCMOD_TYPE(134217728i32);
pub const D3DSP_DCL_USAGEINDEX_MASK: u32 = 983040u32;
pub const D3DSP_DCL_USAGEINDEX_SHIFT: u32 = 16u32;
pub const D3DSP_DCL_USAGE_MASK: u32 = 15u32;
pub const D3DSP_DCL_USAGE_SHIFT: u32 = 0u32;
pub const D3DSP_DSTMOD_MASK: u32 = 15728640u32;
pub const D3DSP_DSTMOD_SHIFT: u32 = 20u32;
pub const D3DSP_DSTSHIFT_MASK: u32 = 251658240u32;
pub const D3DSP_DSTSHIFT_SHIFT: u32 = 24u32;
pub const D3DSP_MIN_PRECISION_MASK: u32 = 49152u32;
pub const D3DSP_MIN_PRECISION_SHIFT: u32 = 14u32;
pub const D3DSP_OPCODESPECIFICCONTROL_MASK: u32 = 16711680u32;
pub const D3DSP_OPCODESPECIFICCONTROL_SHIFT: u32 = 16u32;
pub const D3DSP_REGNUM_MASK: u32 = 2047u32;
pub const D3DSP_REGTYPE_MASK: u32 = 1879048192u32;
pub const D3DSP_REGTYPE_MASK2: u32 = 6144u32;
pub const D3DSP_REGTYPE_SHIFT: u32 = 28u32;
pub const D3DSP_REGTYPE_SHIFT2: u32 = 8u32;
pub const D3DSP_SRCMOD_MASK: u32 = 251658240u32;
pub const D3DSP_SRCMOD_SHIFT: u32 = 24u32;
pub const D3DSP_SWIZZLE_MASK: u32 = 16711680u32;
pub const D3DSP_SWIZZLE_SHIFT: u32 = 16u32;
pub const D3DSP_TEXTURETYPE_MASK: u32 = 2013265920u32;
pub const D3DSP_TEXTURETYPE_SHIFT: u32 = 27u32;
pub const D3DSP_WRITEMASK_0: u32 = 65536u32;
pub const D3DSP_WRITEMASK_1: u32 = 131072u32;
pub const D3DSP_WRITEMASK_2: u32 = 262144u32;
pub const D3DSP_WRITEMASK_3: u32 = 524288u32;
pub const D3DSP_WRITEMASK_ALL: u32 = 983040u32;
pub const D3DSRO_FOG: D3DVS_RASTOUT_OFFSETS = D3DVS_RASTOUT_OFFSETS(1i32);
pub const D3DSRO_POINT_SIZE: D3DVS_RASTOUT_OFFSETS = D3DVS_RASTOUT_OFFSETS(2i32);
pub const D3DSRO_POSITION: D3DVS_RASTOUT_OFFSETS = D3DVS_RASTOUT_OFFSETS(0i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DSTATE {
    pub Anonymous1: D3DSTATE_0,
    pub Anonymous2: D3DSTATE_1,
}
impl Default for D3DSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DSTATE_0 {
    pub dlstLightStateType: D3DLIGHTSTATETYPE,
    pub drstRenderStateType: D3DRENDERSTATETYPE,
}
impl Default for D3DSTATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DSTATE_1 {
    pub dwArg: [u32; 1],
    pub dvArg: [f32; 1],
}
impl Default for D3DSTATE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSTATEBLOCKTYPE(pub i32);
pub const D3DSTATE_OVERRIDE_BIAS: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DSTATS {
    pub dwSize: u32,
    pub dwTrianglesDrawn: u32,
    pub dwLinesDrawn: u32,
    pub dwPointsDrawn: u32,
    pub dwSpansDrawn: u32,
    pub dwVerticesProcessed: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DSTATUS {
    pub dwFlags: u32,
    pub dwStatus: u32,
    pub drExtent: D3DRECT,
}
pub const D3DSTATUS_CLIPINTERSECTIONBACK: i32 = 131072i32;
pub const D3DSTATUS_CLIPINTERSECTIONBOTTOM: i32 = 32768i32;
pub const D3DSTATUS_CLIPINTERSECTIONFRONT: i32 = 65536i32;
pub const D3DSTATUS_CLIPINTERSECTIONGEN0: i32 = 262144i32;
pub const D3DSTATUS_CLIPINTERSECTIONGEN1: i32 = 524288i32;
pub const D3DSTATUS_CLIPINTERSECTIONGEN2: i32 = 1048576i32;
pub const D3DSTATUS_CLIPINTERSECTIONGEN3: i32 = 2097152i32;
pub const D3DSTATUS_CLIPINTERSECTIONGEN4: i32 = 4194304i32;
pub const D3DSTATUS_CLIPINTERSECTIONGEN5: i32 = 8388608i32;
pub const D3DSTATUS_CLIPINTERSECTIONLEFT: i32 = 4096i32;
pub const D3DSTATUS_CLIPINTERSECTIONRIGHT: i32 = 8192i32;
pub const D3DSTATUS_CLIPINTERSECTIONTOP: i32 = 16384i32;
pub const D3DSTATUS_CLIPUNIONBACK: i32 = 32i32;
pub const D3DSTATUS_CLIPUNIONBOTTOM: i32 = 8i32;
pub const D3DSTATUS_CLIPUNIONFRONT: i32 = 16i32;
pub const D3DSTATUS_CLIPUNIONGEN0: i32 = 64i32;
pub const D3DSTATUS_CLIPUNIONGEN1: i32 = 128i32;
pub const D3DSTATUS_CLIPUNIONGEN2: i32 = 256i32;
pub const D3DSTATUS_CLIPUNIONGEN3: i32 = 512i32;
pub const D3DSTATUS_CLIPUNIONGEN4: i32 = 1024i32;
pub const D3DSTATUS_CLIPUNIONGEN5: i32 = 2048i32;
pub const D3DSTATUS_CLIPUNIONLEFT: i32 = 1i32;
pub const D3DSTATUS_CLIPUNIONRIGHT: i32 = 2i32;
pub const D3DSTATUS_CLIPUNIONTOP: i32 = 4i32;
pub const D3DSTATUS_ZNOTVISIBLE: i32 = 16777216i32;
pub const D3DSTENCILCAPS_DECR: i32 = 128i32;
pub const D3DSTENCILCAPS_DECRSAT: i32 = 16i32;
pub const D3DSTENCILCAPS_INCR: i32 = 64i32;
pub const D3DSTENCILCAPS_INCRSAT: i32 = 8i32;
pub const D3DSTENCILCAPS_INVERT: i32 = 32i32;
pub const D3DSTENCILCAPS_KEEP: i32 = 1i32;
pub const D3DSTENCILCAPS_REPLACE: i32 = 4i32;
pub const D3DSTENCILCAPS_TWOSIDED: i32 = 256i32;
pub const D3DSTENCILCAPS_ZERO: i32 = 2i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSTENCILOP(pub i32);
pub const D3DSTENCILOP_DECR: D3DSTENCILOP = D3DSTENCILOP(8i32);
pub const D3DSTENCILOP_DECRSAT: D3DSTENCILOP = D3DSTENCILOP(5i32);
pub const D3DSTENCILOP_INCR: D3DSTENCILOP = D3DSTENCILOP(7i32);
pub const D3DSTENCILOP_INCRSAT: D3DSTENCILOP = D3DSTENCILOP(4i32);
pub const D3DSTENCILOP_INVERT: D3DSTENCILOP = D3DSTENCILOP(6i32);
pub const D3DSTENCILOP_KEEP: D3DSTENCILOP = D3DSTENCILOP(1i32);
pub const D3DSTENCILOP_REPLACE: D3DSTENCILOP = D3DSTENCILOP(3i32);
pub const D3DSTENCILOP_ZERO: D3DSTENCILOP = D3DSTENCILOP(2i32);
pub const D3DSTREAMSOURCE_INDEXEDDATA: u32 = 1073741824u32;
pub const D3DSTREAMSOURCE_INSTANCEDATA: u32 = 2147483648u32;
pub const D3DSTT_2D: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(268435456i32);
pub const D3DSTT_CUBE: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(402653184i32);
pub const D3DSTT_UNKNOWN: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(0i32);
pub const D3DSTT_VOLUME: D3DSAMPLER_TEXTURE_TYPE = D3DSAMPLER_TEXTURE_TYPE(536870912i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DSURFACE_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub MultiSampleType: D3DMULTISAMPLE_TYPE,
    pub MultiSampleQuality: u32,
    pub Width: u32,
    pub Height: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DSWAPEFFECT(pub i32);
pub const D3DSWAPEFFECT_COPY: D3DSWAPEFFECT = D3DSWAPEFFECT(3i32);
pub const D3DSWAPEFFECT_DISCARD: D3DSWAPEFFECT = D3DSWAPEFFECT(1i32);
pub const D3DSWAPEFFECT_FLIP: D3DSWAPEFFECT = D3DSWAPEFFECT(2i32);
pub const D3DSWAPEFFECT_FLIPEX: D3DSWAPEFFECT = D3DSWAPEFFECT(5i32);
pub const D3DSWAPEFFECT_OVERLAY: D3DSWAPEFFECT = D3DSWAPEFFECT(4i32);
pub const D3DTADDRESS_BORDER: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(4i32);
pub const D3DTADDRESS_CLAMP: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(3i32);
pub const D3DTADDRESS_MIRROR: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(2i32);
pub const D3DTADDRESS_MIRRORONCE: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(5i32);
pub const D3DTADDRESS_WRAP: D3DTEXTUREADDRESS = D3DTEXTUREADDRESS(1i32);
pub const D3DTA_ALPHAREPLICATE: u32 = 32u32;
pub const D3DTA_COMPLEMENT: u32 = 16u32;
pub const D3DTA_CONSTANT: u32 = 6u32;
pub const D3DTA_CURRENT: u32 = 1u32;
pub const D3DTA_DIFFUSE: u32 = 0u32;
pub const D3DTA_SELECTMASK: u32 = 15u32;
pub const D3DTA_SPECULAR: u32 = 4u32;
pub const D3DTA_TEMP: u32 = 5u32;
pub const D3DTA_TEXTURE: u32 = 2u32;
pub const D3DTA_TFACTOR: u32 = 3u32;
pub const D3DTBLEND_ADD: D3DTEXTUREBLEND = D3DTEXTUREBLEND(8i32);
pub const D3DTBLEND_COPY: D3DTEXTUREBLEND = D3DTEXTUREBLEND(7i32);
pub const D3DTBLEND_DECAL: D3DTEXTUREBLEND = D3DTEXTUREBLEND(1i32);
pub const D3DTBLEND_DECALALPHA: D3DTEXTUREBLEND = D3DTEXTUREBLEND(3i32);
pub const D3DTBLEND_DECALMASK: D3DTEXTUREBLEND = D3DTEXTUREBLEND(5i32);
pub const D3DTBLEND_MODULATE: D3DTEXTUREBLEND = D3DTEXTUREBLEND(2i32);
pub const D3DTBLEND_MODULATEALPHA: D3DTEXTUREBLEND = D3DTEXTUREBLEND(4i32);
pub const D3DTBLEND_MODULATEMASK: D3DTEXTUREBLEND = D3DTEXTUREBLEND(6i32);
pub const D3DTEXF_ANISOTROPIC: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(3i32);
pub const D3DTEXF_CONVOLUTIONMONO: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(8i32);
pub const D3DTEXF_GAUSSIANQUAD: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(7i32);
pub const D3DTEXF_LINEAR: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(2i32);
pub const D3DTEXF_NONE: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(0i32);
pub const D3DTEXF_POINT: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(1i32);
pub const D3DTEXF_PYRAMIDALQUAD: D3DTEXTUREFILTERTYPE = D3DTEXTUREFILTERTYPE(6i32);
pub const D3DTEXOPCAPS_ADD: i32 = 64i32;
pub const D3DTEXOPCAPS_ADDSIGNED: i32 = 128i32;
pub const D3DTEXOPCAPS_ADDSIGNED2X: i32 = 256i32;
pub const D3DTEXOPCAPS_ADDSMOOTH: i32 = 1024i32;
pub const D3DTEXOPCAPS_BLENDCURRENTALPHA: i32 = 32768i32;
pub const D3DTEXOPCAPS_BLENDDIFFUSEALPHA: i32 = 2048i32;
pub const D3DTEXOPCAPS_BLENDFACTORALPHA: i32 = 8192i32;
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHA: i32 = 4096i32;
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHAPM: i32 = 16384i32;
pub const D3DTEXOPCAPS_BUMPENVMAP: i32 = 2097152i32;
pub const D3DTEXOPCAPS_BUMPENVMAPLUMINANCE: i32 = 4194304i32;
pub const D3DTEXOPCAPS_DISABLE: i32 = 1i32;
pub const D3DTEXOPCAPS_DOTPRODUCT3: i32 = 8388608i32;
pub const D3DTEXOPCAPS_LERP: i32 = 33554432i32;
pub const D3DTEXOPCAPS_MODULATE: i32 = 8i32;
pub const D3DTEXOPCAPS_MODULATE2X: i32 = 16i32;
pub const D3DTEXOPCAPS_MODULATE4X: i32 = 32i32;
pub const D3DTEXOPCAPS_MODULATEALPHA_ADDCOLOR: i32 = 131072i32;
pub const D3DTEXOPCAPS_MODULATECOLOR_ADDALPHA: i32 = 262144i32;
pub const D3DTEXOPCAPS_MODULATEINVALPHA_ADDCOLOR: i32 = 524288i32;
pub const D3DTEXOPCAPS_MODULATEINVCOLOR_ADDALPHA: i32 = 1048576i32;
pub const D3DTEXOPCAPS_MULTIPLYADD: i32 = 16777216i32;
pub const D3DTEXOPCAPS_PREMODULATE: i32 = 65536i32;
pub const D3DTEXOPCAPS_SELECTARG1: i32 = 2i32;
pub const D3DTEXOPCAPS_SELECTARG2: i32 = 4i32;
pub const D3DTEXOPCAPS_SUBTRACT: i32 = 512i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTUREADDRESS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTUREBLEND(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTUREFILTER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTUREFILTERTYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DTEXTURELOAD {
    pub hDestTexture: u32,
    pub hSrcTexture: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTUREMAGFILTER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTUREMINFILTER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTUREMIPFILTER(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTUREOP(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTURESTAGESTATETYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTEXTURETRANSFORMFLAGS(pub i32);
pub const D3DTFG_ANISOTROPIC: D3DTEXTUREMAGFILTER = D3DTEXTUREMAGFILTER(5i32);
pub const D3DTFG_FLATCUBIC: D3DTEXTUREMAGFILTER = D3DTEXTUREMAGFILTER(3i32);
pub const D3DTFG_GAUSSIANCUBIC: D3DTEXTUREMAGFILTER = D3DTEXTUREMAGFILTER(4i32);
pub const D3DTFG_LINEAR: D3DTEXTUREMAGFILTER = D3DTEXTUREMAGFILTER(2i32);
pub const D3DTFG_POINT: D3DTEXTUREMAGFILTER = D3DTEXTUREMAGFILTER(1i32);
pub const D3DTFN_ANISOTROPIC: D3DTEXTUREMINFILTER = D3DTEXTUREMINFILTER(3i32);
pub const D3DTFN_LINEAR: D3DTEXTUREMINFILTER = D3DTEXTUREMINFILTER(2i32);
pub const D3DTFN_POINT: D3DTEXTUREMINFILTER = D3DTEXTUREMINFILTER(1i32);
pub const D3DTFP_LINEAR: D3DTEXTUREMIPFILTER = D3DTEXTUREMIPFILTER(3i32);
pub const D3DTFP_NONE: D3DTEXTUREMIPFILTER = D3DTEXTUREMIPFILTER(1i32);
pub const D3DTFP_POINT: D3DTEXTUREMIPFILTER = D3DTEXTUREMIPFILTER(2i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DTLVERTEX {
    pub Anonymous1: D3DTLVERTEX_0,
    pub Anonymous2: D3DTLVERTEX_1,
    pub Anonymous3: D3DTLVERTEX_2,
    pub Anonymous4: D3DTLVERTEX_3,
    pub Anonymous5: D3DTLVERTEX_4,
    pub Anonymous6: D3DTLVERTEX_5,
    pub Anonymous7: D3DTLVERTEX_6,
    pub Anonymous8: D3DTLVERTEX_7,
}
impl Default for D3DTLVERTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTLVERTEX_0 {
    pub sx: f32,
    pub dvSX: f32,
}
impl Default for D3DTLVERTEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTLVERTEX_1 {
    pub sy: f32,
    pub dvSY: f32,
}
impl Default for D3DTLVERTEX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTLVERTEX_2 {
    pub sz: f32,
    pub dvSZ: f32,
}
impl Default for D3DTLVERTEX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTLVERTEX_3 {
    pub rhw: f32,
    pub dvRHW: f32,
}
impl Default for D3DTLVERTEX_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTLVERTEX_4 {
    pub color: u32,
    pub dcColor: u32,
}
impl Default for D3DTLVERTEX_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTLVERTEX_5 {
    pub specular: u32,
    pub dcSpecular: u32,
}
impl Default for D3DTLVERTEX_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTLVERTEX_6 {
    pub tu: f32,
    pub dvTU: f32,
}
impl Default for D3DTLVERTEX_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTLVERTEX_7 {
    pub tv: f32,
    pub dvTV: f32,
}
impl Default for D3DTLVERTEX_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DTOP_ADD: D3DTEXTUREOP = D3DTEXTUREOP(7i32);
pub const D3DTOP_ADDSIGNED: D3DTEXTUREOP = D3DTEXTUREOP(8i32);
pub const D3DTOP_ADDSIGNED2X: D3DTEXTUREOP = D3DTEXTUREOP(9i32);
pub const D3DTOP_ADDSMOOTH: D3DTEXTUREOP = D3DTEXTUREOP(11i32);
pub const D3DTOP_BLENDCURRENTALPHA: D3DTEXTUREOP = D3DTEXTUREOP(16i32);
pub const D3DTOP_BLENDDIFFUSEALPHA: D3DTEXTUREOP = D3DTEXTUREOP(12i32);
pub const D3DTOP_BLENDFACTORALPHA: D3DTEXTUREOP = D3DTEXTUREOP(14i32);
pub const D3DTOP_BLENDTEXTUREALPHA: D3DTEXTUREOP = D3DTEXTUREOP(13i32);
pub const D3DTOP_BLENDTEXTUREALPHAPM: D3DTEXTUREOP = D3DTEXTUREOP(15i32);
pub const D3DTOP_BUMPENVMAP: D3DTEXTUREOP = D3DTEXTUREOP(22i32);
pub const D3DTOP_BUMPENVMAPLUMINANCE: D3DTEXTUREOP = D3DTEXTUREOP(23i32);
pub const D3DTOP_DISABLE: D3DTEXTUREOP = D3DTEXTUREOP(1i32);
pub const D3DTOP_DOTPRODUCT3: D3DTEXTUREOP = D3DTEXTUREOP(24i32);
pub const D3DTOP_LERP: D3DTEXTUREOP = D3DTEXTUREOP(26i32);
pub const D3DTOP_MODULATE: D3DTEXTUREOP = D3DTEXTUREOP(4i32);
pub const D3DTOP_MODULATE2X: D3DTEXTUREOP = D3DTEXTUREOP(5i32);
pub const D3DTOP_MODULATE4X: D3DTEXTUREOP = D3DTEXTUREOP(6i32);
pub const D3DTOP_MODULATEALPHA_ADDCOLOR: D3DTEXTUREOP = D3DTEXTUREOP(18i32);
pub const D3DTOP_MODULATECOLOR_ADDALPHA: D3DTEXTUREOP = D3DTEXTUREOP(19i32);
pub const D3DTOP_MODULATEINVALPHA_ADDCOLOR: D3DTEXTUREOP = D3DTEXTUREOP(20i32);
pub const D3DTOP_MODULATEINVCOLOR_ADDALPHA: D3DTEXTUREOP = D3DTEXTUREOP(21i32);
pub const D3DTOP_MULTIPLYADD: D3DTEXTUREOP = D3DTEXTUREOP(25i32);
pub const D3DTOP_PREMODULATE: D3DTEXTUREOP = D3DTEXTUREOP(17i32);
pub const D3DTOP_SELECTARG1: D3DTEXTUREOP = D3DTEXTUREOP(2i32);
pub const D3DTOP_SELECTARG2: D3DTEXTUREOP = D3DTEXTUREOP(3i32);
pub const D3DTOP_SUBTRACT: D3DTEXTUREOP = D3DTEXTUREOP(10i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DTRANSFORMCAPS {
    pub dwSize: u32,
    pub dwCaps: u32,
}
pub const D3DTRANSFORMCAPS_CLIP: i32 = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3DTRANSFORMDATA {
    pub dwSize: u32,
    pub lpIn: *mut core::ffi::c_void,
    pub dwInSize: u32,
    pub lpOut: *mut core::ffi::c_void,
    pub dwOutSize: u32,
    pub lpHOut: *mut D3DHVERTEX,
    pub dwClip: u32,
    pub dwClipIntersection: u32,
    pub dwClipUnion: u32,
    pub drExtent: D3DRECT,
}
impl Default for D3DTRANSFORMDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DTRANSFORMSTATETYPE(pub i32);
pub const D3DTRANSFORM_CLIPPED: i32 = 1i32;
pub const D3DTRANSFORM_UNCLIPPED: i32 = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DTRIANGLE {
    pub Anonymous1: D3DTRIANGLE_0,
    pub Anonymous2: D3DTRIANGLE_1,
    pub Anonymous3: D3DTRIANGLE_2,
    pub wFlags: u16,
}
impl Default for D3DTRIANGLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTRIANGLE_0 {
    pub v1: u16,
    pub wV1: u16,
}
impl Default for D3DTRIANGLE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTRIANGLE_1 {
    pub v2: u16,
    pub wV2: u16,
}
impl Default for D3DTRIANGLE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DTRIANGLE_2 {
    pub v3: u16,
    pub wV3: u16,
}
impl Default for D3DTRIANGLE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DTRIFLAG_EDGEENABLE1: i32 = 256i32;
pub const D3DTRIFLAG_EDGEENABLE2: i32 = 512i32;
pub const D3DTRIFLAG_EDGEENABLE3: i32 = 1024i32;
pub const D3DTRIFLAG_EVEN: i32 = 31i32;
pub const D3DTRIFLAG_ODD: i32 = 30i32;
pub const D3DTRIFLAG_START: i32 = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DTRIPATCH_INFO {
    pub StartVertexOffset: u32,
    pub NumVertices: u32,
    pub Basis: D3DBASISTYPE,
    pub Degree: D3DDEGREETYPE,
}
pub const D3DTSS_ALPHAARG0: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(27i32);
pub const D3DTSS_ALPHAARG1: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(5i32);
pub const D3DTSS_ALPHAARG2: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(6i32);
pub const D3DTSS_ALPHAOP: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(4i32);
pub const D3DTSS_BUMPENVLOFFSET: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(23i32);
pub const D3DTSS_BUMPENVLSCALE: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(22i32);
pub const D3DTSS_BUMPENVMAT00: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(7i32);
pub const D3DTSS_BUMPENVMAT01: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(8i32);
pub const D3DTSS_BUMPENVMAT10: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(9i32);
pub const D3DTSS_BUMPENVMAT11: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(10i32);
pub const D3DTSS_COLORARG0: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(26i32);
pub const D3DTSS_COLORARG1: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(2i32);
pub const D3DTSS_COLORARG2: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(3i32);
pub const D3DTSS_COLOROP: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(1i32);
pub const D3DTSS_CONSTANT: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(32i32);
pub const D3DTSS_RESULTARG: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(28i32);
pub const D3DTSS_TCI_CAMERASPACENORMAL: u32 = 65536u32;
pub const D3DTSS_TCI_CAMERASPACEPOSITION: u32 = 131072u32;
pub const D3DTSS_TCI_CAMERASPACEREFLECTIONVECTOR: u32 = 196608u32;
pub const D3DTSS_TCI_PASSTHRU: u32 = 0u32;
pub const D3DTSS_TCI_SPHEREMAP: u32 = 262144u32;
pub const D3DTSS_TEXCOORDINDEX: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(11i32);
pub const D3DTSS_TEXTURETRANSFORMFLAGS: D3DTEXTURESTAGESTATETYPE = D3DTEXTURESTAGESTATETYPE(24i32);
pub const D3DTS_PROJECTION: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(3i32);
pub const D3DTS_TEXTURE0: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(16i32);
pub const D3DTS_TEXTURE1: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(17i32);
pub const D3DTS_TEXTURE2: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(18i32);
pub const D3DTS_TEXTURE3: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(19i32);
pub const D3DTS_TEXTURE4: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(20i32);
pub const D3DTS_TEXTURE5: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(21i32);
pub const D3DTS_TEXTURE6: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(22i32);
pub const D3DTS_TEXTURE7: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(23i32);
pub const D3DTS_VIEW: D3DTRANSFORMSTATETYPE = D3DTRANSFORMSTATETYPE(2i32);
pub const D3DTTFF_COUNT1: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(1i32);
pub const D3DTTFF_COUNT2: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(2i32);
pub const D3DTTFF_COUNT3: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(3i32);
pub const D3DTTFF_COUNT4: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(4i32);
pub const D3DTTFF_DISABLE: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(0i32);
pub const D3DTTFF_PROJECTED: D3DTEXTURETRANSFORMFLAGS = D3DTEXTURETRANSFORMFLAGS(256i32);
pub const D3DUSAGE_AUTOGENMIPMAP: i32 = 1024i32;
pub const D3DUSAGE_DEPTHSTENCIL: i32 = 2i32;
pub const D3DUSAGE_DMAP: i32 = 16384i32;
pub const D3DUSAGE_DONOTCLIP: i32 = 32i32;
pub const D3DUSAGE_DYNAMIC: i32 = 512i32;
pub const D3DUSAGE_NONSECURE: i32 = 8388608i32;
pub const D3DUSAGE_NPATCHES: i32 = 256i32;
pub const D3DUSAGE_POINTS: i32 = 64i32;
pub const D3DUSAGE_QUERY_FILTER: i32 = 131072i32;
pub const D3DUSAGE_QUERY_LEGACYBUMPMAP: i32 = 32768i32;
pub const D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING: i32 = 524288i32;
pub const D3DUSAGE_QUERY_SRGBREAD: i32 = 65536i32;
pub const D3DUSAGE_QUERY_SRGBWRITE: i32 = 262144i32;
pub const D3DUSAGE_QUERY_VERTEXTEXTURE: i32 = 1048576i32;
pub const D3DUSAGE_QUERY_WRAPANDMIP: i32 = 2097152i32;
pub const D3DUSAGE_RENDERTARGET: i32 = 1i32;
pub const D3DUSAGE_RESTRICTED_CONTENT: i32 = 2048i32;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE: i32 = 8192i32;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER: i32 = 4096i32;
pub const D3DUSAGE_RTPATCHES: i32 = 128i32;
pub const D3DUSAGE_SOFTWAREPROCESSING: i32 = 16i32;
pub const D3DUSAGE_TEXTAPI: i32 = 268435456i32;
pub const D3DUSAGE_WRITEONLY: i32 = 8i32;
pub const D3DVBCAPS_DONOTCLIP: i32 = 1i32;
pub const D3DVBCAPS_OPTIMIZED: i32 = -2147483648i32;
pub const D3DVBCAPS_SYSTEMMEMORY: i32 = 2048i32;
pub const D3DVBCAPS_WRITEONLY: i32 = 65536i32;
pub const D3DVBF_0WEIGHTS: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(256i32);
pub const D3DVBF_1WEIGHTS: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(1i32);
pub const D3DVBF_2WEIGHTS: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(2i32);
pub const D3DVBF_3WEIGHTS: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(3i32);
pub const D3DVBF_DISABLE: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(0i32);
pub const D3DVBF_TWEENING: D3DVERTEXBLENDFLAGS = D3DVERTEXBLENDFLAGS(255i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DVERTEX {
    pub Anonymous1: D3DVERTEX_0,
    pub Anonymous2: D3DVERTEX_1,
    pub Anonymous3: D3DVERTEX_2,
    pub Anonymous4: D3DVERTEX_3,
    pub Anonymous5: D3DVERTEX_4,
    pub Anonymous6: D3DVERTEX_5,
    pub Anonymous7: D3DVERTEX_6,
    pub Anonymous8: D3DVERTEX_7,
}
impl Default for D3DVERTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DVERTEX_0 {
    pub x: f32,
    pub dvX: f32,
}
impl Default for D3DVERTEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DVERTEX_1 {
    pub y: f32,
    pub dvY: f32,
}
impl Default for D3DVERTEX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DVERTEX_2 {
    pub z: f32,
    pub dvZ: f32,
}
impl Default for D3DVERTEX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DVERTEX_3 {
    pub nx: f32,
    pub dvNX: f32,
}
impl Default for D3DVERTEX_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DVERTEX_4 {
    pub ny: f32,
    pub dvNY: f32,
}
impl Default for D3DVERTEX_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DVERTEX_5 {
    pub nz: f32,
    pub dvNZ: f32,
}
impl Default for D3DVERTEX_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DVERTEX_6 {
    pub tu: f32,
    pub dvTU: f32,
}
impl Default for D3DVERTEX_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DVERTEX_7 {
    pub tv: f32,
    pub dvTV: f32,
}
impl Default for D3DVERTEX_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DVERTEXBLENDFLAGS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVERTEXBUFFERDESC {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwFVF: u32,
    pub dwNumVertices: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVERTEXBUFFER_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Size: u32,
    pub FVF: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVERTEXELEMENT9 {
    pub Stream: u16,
    pub Offset: u16,
    pub Type: u8,
    pub Method: u8,
    pub Usage: u8,
    pub UsageIndex: u8,
}
pub const D3DVERTEXTEXTURESAMPLER0: u32 = 257u32;
pub const D3DVERTEXTEXTURESAMPLER1: u32 = 258u32;
pub const D3DVERTEXTEXTURESAMPLER2: u32 = 259u32;
pub const D3DVERTEXTEXTURESAMPLER3: u32 = 260u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DVERTEXTYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVIEWPORT {
    pub dwSize: u32,
    pub dwX: u32,
    pub dwY: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dvScaleX: f32,
    pub dvScaleY: f32,
    pub dvMaxX: f32,
    pub dvMaxY: f32,
    pub dvMinZ: f32,
    pub dvMaxZ: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVIEWPORT2 {
    pub dwSize: u32,
    pub dwX: u32,
    pub dwY: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dvClipX: f32,
    pub dvClipY: f32,
    pub dvClipWidth: f32,
    pub dvClipHeight: f32,
    pub dvMinZ: f32,
    pub dvMaxZ: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVIEWPORT7 {
    pub dwX: u32,
    pub dwY: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dvMinZ: f32,
    pub dvMaxZ: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVIEWPORT9 {
    pub X: u32,
    pub Y: u32,
    pub Width: u32,
    pub Height: u32,
    pub MinZ: f32,
    pub MaxZ: f32,
}
pub const D3DVIS_INSIDE_BOTTOM: u32 = 0u32;
pub const D3DVIS_INSIDE_FAR: u32 = 0u32;
pub const D3DVIS_INSIDE_FRUSTUM: u32 = 0u32;
pub const D3DVIS_INSIDE_LEFT: u32 = 0u32;
pub const D3DVIS_INSIDE_NEAR: u32 = 0u32;
pub const D3DVIS_INSIDE_RIGHT: u32 = 0u32;
pub const D3DVIS_INSIDE_TOP: u32 = 0u32;
pub const D3DVIS_INTERSECT_BOTTOM: u32 = 256u32;
pub const D3DVIS_INTERSECT_FAR: u32 = 4096u32;
pub const D3DVIS_INTERSECT_FRUSTUM: u32 = 1u32;
pub const D3DVIS_INTERSECT_LEFT: u32 = 4u32;
pub const D3DVIS_INTERSECT_NEAR: u32 = 1024u32;
pub const D3DVIS_INTERSECT_RIGHT: u32 = 16u32;
pub const D3DVIS_INTERSECT_TOP: u32 = 64u32;
pub const D3DVIS_MASK_BOTTOM: u32 = 768u32;
pub const D3DVIS_MASK_FAR: u32 = 12288u32;
pub const D3DVIS_MASK_FRUSTUM: u32 = 3u32;
pub const D3DVIS_MASK_LEFT: u32 = 12u32;
pub const D3DVIS_MASK_NEAR: u32 = 3072u32;
pub const D3DVIS_MASK_RIGHT: u32 = 48u32;
pub const D3DVIS_MASK_TOP: u32 = 192u32;
pub const D3DVIS_OUTSIDE_BOTTOM: u32 = 512u32;
pub const D3DVIS_OUTSIDE_FAR: u32 = 8192u32;
pub const D3DVIS_OUTSIDE_FRUSTUM: u32 = 2u32;
pub const D3DVIS_OUTSIDE_LEFT: u32 = 8u32;
pub const D3DVIS_OUTSIDE_NEAR: u32 = 2048u32;
pub const D3DVIS_OUTSIDE_RIGHT: u32 = 32u32;
pub const D3DVIS_OUTSIDE_TOP: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVOLUME_DESC {
    pub Format: D3DFORMAT,
    pub Type: D3DRESOURCETYPE,
    pub Usage: u32,
    pub Pool: D3DPOOL,
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
}
pub const D3DVOP_CLIP: u32 = 4u32;
pub const D3DVOP_EXTENTS: u32 = 8u32;
pub const D3DVOP_LIGHT: u32 = 1024u32;
pub const D3DVOP_TRANSFORM: u32 = 1u32;
pub const D3DVS20CAPS_PREDICATION: u32 = 1u32;
pub const D3DVS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24u32;
pub const D3DVS20_MAX_NUMTEMPS: u32 = 32u32;
pub const D3DVS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4u32;
pub const D3DVS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0u32;
pub const D3DVS20_MIN_NUMTEMPS: u32 = 12u32;
pub const D3DVS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVSHADERCAPS2_0 {
    pub Caps: u32,
    pub DynamicFlowControlDepth: i32,
    pub NumTemps: i32,
    pub StaticFlowControlDepth: i32,
}
pub const D3DVS_ADDRESSMODE_SHIFT: u32 = 13u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DVS_ADDRESSMODE_TYPE(pub i32);
pub const D3DVS_ADDRMODE_ABSOLUTE: D3DVS_ADDRESSMODE_TYPE = D3DVS_ADDRESSMODE_TYPE(0i32);
pub const D3DVS_ADDRMODE_RELATIVE: D3DVS_ADDRESSMODE_TYPE = D3DVS_ADDRESSMODE_TYPE(8192i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DVS_RASTOUT_OFFSETS(pub i32);
pub const D3DVS_SWIZZLE_MASK: u32 = 16711680u32;
pub const D3DVS_SWIZZLE_SHIFT: u32 = 16u32;
pub const D3DVTXPCAPS_DIRECTIONALLIGHTS: i32 = 8i32;
pub const D3DVTXPCAPS_LOCALVIEWER: i32 = 32i32;
pub const D3DVTXPCAPS_MATERIALSOURCE7: i32 = 2i32;
pub const D3DVTXPCAPS_NO_TEXGEN_NONLOCALVIEWER: i32 = 512i32;
pub const D3DVTXPCAPS_POSITIONALLIGHTS: i32 = 16i32;
pub const D3DVTXPCAPS_TEXGEN: i32 = 1i32;
pub const D3DVTXPCAPS_TEXGEN_SPHEREMAP: i32 = 256i32;
pub const D3DVTXPCAPS_TWEENING: i32 = 64i32;
pub const D3DVTXPCAPS_VERTEXFOG: i32 = 4i32;
pub const D3DVT_LVERTEX: D3DVERTEXTYPE = D3DVERTEXTYPE(2i32);
pub const D3DVT_TLVERTEX: D3DVERTEXTYPE = D3DVERTEXTYPE(3i32);
pub const D3DVT_VERTEX: D3DVERTEXTYPE = D3DVERTEXTYPE(1i32);
pub const D3DWRAPCOORD_0: i32 = 1i32;
pub const D3DWRAPCOORD_1: i32 = 2i32;
pub const D3DWRAPCOORD_2: i32 = 4i32;
pub const D3DWRAPCOORD_3: i32 = 8i32;
pub const D3DWRAP_U: i32 = 1i32;
pub const D3DWRAP_V: i32 = 2i32;
pub const D3DWRAP_W: i32 = 4i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3DZBUFFERTYPE(pub i32);
pub const D3DZB_FALSE: D3DZBUFFERTYPE = D3DZBUFFERTYPE(0i32);
pub const D3DZB_TRUE: D3DZBUFFERTYPE = D3DZBUFFERTYPE(1i32);
pub const D3DZB_USEW: D3DZBUFFERTYPE = D3DZBUFFERTYPE(2i32);
pub const D3D_MAX_SIMULTANEOUS_RENDERTARGETS: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D_OMAC {
    pub Omac: [u8; 16],
}
impl Default for D3D_OMAC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D_OMAC_SIZE: u32 = 16u32;
pub const D3D_SDK_VERSION: u32 = 32u32;
pub const DIRECT3D_VERSION: u32 = 2304u32;
windows_core::imp::define_interface!(IDirect3D9, IDirect3D9_Vtbl, 0x81bdcbca_64d4_426d_ae8d_ad0147f4275c);
windows_core::imp::interface_hierarchy!(IDirect3D9, windows_core::IUnknown);
impl IDirect3D9 {
    pub unsafe fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterSoftwareDevice)(windows_core::Interface::as_raw(self), pinitializefunction as _).ok() }
    }
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterIdentifier)(windows_core::Interface::as_raw(self), adapter, flags, pidentifier as _).ok() }
    }
    pub unsafe fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterModeCount)(windows_core::Interface::as_raw(self), adapter, format) }
    }
    pub unsafe fn EnumAdapterModes(&self, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterModes)(windows_core::Interface::as_raw(self), adapter, format, mode, pmode as _).ok() }
    }
    pub unsafe fn GetAdapterDisplayMode(&self, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterDisplayMode)(windows_core::Interface::as_raw(self), adapter, pmode as _).ok() }
    }
    pub unsafe fn CheckDeviceType(&self, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceType)(windows_core::Interface::as_raw(self), adapter, devtype, adapterformat, backbufferformat, bwindowed.into()).ok() }
    }
    pub unsafe fn CheckDeviceFormat(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceFormat)(windows_core::Interface::as_raw(self), adapter, devicetype, adapterformat, usage, rtype, checkformat).ok() }
    }
    pub unsafe fn CheckDeviceMultiSampleType(&self, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: bool, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceMultiSampleType)(windows_core::Interface::as_raw(self), adapter, devicetype, surfaceformat, windowed.into(), multisampletype, pqualitylevels as _).ok() }
    }
    pub unsafe fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckDepthStencilMatch)(windows_core::Interface::as_raw(self), adapter, devicetype, adapterformat, rendertargetformat, depthstencilformat).ok() }
    }
    pub unsafe fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceFormatConversion)(windows_core::Interface::as_raw(self), adapter, devicetype, sourceformat, targetformat).ok() }
    }
    pub unsafe fn GetDeviceCaps(&self, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceCaps)(windows_core::Interface::as_raw(self), adapter, devicetype, pcaps as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetAdapterMonitor(&self, adapter: u32) -> super::Gdi::HMONITOR {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterMonitor)(windows_core::Interface::as_raw(self), adapter) }
    }
    pub unsafe fn CreateDevice(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), adapter, devicetype, hfocuswindow, behaviorflags, ppresentationparameters as _, core::mem::transmute(ppreturneddeviceinterface)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterSoftwareDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdapterCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetAdapterIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT,
    pub GetAdapterModeCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DFORMAT) -> u32,
    pub EnumAdapterModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DFORMAT, u32, *mut D3DDISPLAYMODE) -> windows_core::HRESULT,
    pub GetAdapterDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DDISPLAYMODE) -> windows_core::HRESULT,
    pub CheckDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DDEVTYPE, D3DFORMAT, D3DFORMAT, windows_core::BOOL) -> windows_core::HRESULT,
    pub CheckDeviceFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DDEVTYPE, D3DFORMAT, u32, D3DRESOURCETYPE, D3DFORMAT) -> windows_core::HRESULT,
    pub CheckDeviceMultiSampleType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DDEVTYPE, D3DFORMAT, windows_core::BOOL, D3DMULTISAMPLE_TYPE, *mut u32) -> windows_core::HRESULT,
    pub CheckDepthStencilMatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DDEVTYPE, D3DFORMAT, D3DFORMAT, D3DFORMAT) -> windows_core::HRESULT,
    pub CheckDeviceFormatConversion: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DDEVTYPE, D3DFORMAT, D3DFORMAT) -> windows_core::HRESULT,
    pub GetDeviceCaps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DDEVTYPE, *mut D3DCAPS9) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetAdapterMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> super::Gdi::HMONITOR,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetAdapterMonitor: usize,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DDEVTYPE, super::super::Foundation::HWND, u32, *mut D3DPRESENT_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3D9_Impl: windows_core::IUnknownImpl {
    fn RegisterSoftwareDevice(&self, pinitializefunction: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterCount(&self) -> u32;
    fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> windows_core::Result<()>;
    fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32;
    fn EnumAdapterModes(&self, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetAdapterDisplayMode(&self, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn CheckDeviceType(&self, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::Result<()>;
    fn CheckDeviceFormat(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceMultiSampleType(&self, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: windows_core::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> windows_core::Result<()>;
    fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> windows_core::Result<()>;
    fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> windows_core::Result<()>;
    fn GetDeviceCaps(&self, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> windows_core::Result<()>;
    fn GetAdapterMonitor(&self, adapter: u32) -> super::Gdi::HMONITOR;
    fn CreateDevice(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: windows_core::OutRef<IDirect3DDevice9>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3D9_Vtbl {
    pub const fn new<Identity: IDirect3D9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterSoftwareDevice<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitializefunction: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::RegisterSoftwareDevice(this, core::mem::transmute_copy(&pinitializefunction)).into()
            }
        }
        unsafe extern "system" fn GetAdapterCount<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterCount(this)
            }
        }
        unsafe extern "system" fn GetAdapterIdentifier<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterIdentifier(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pidentifier)).into()
            }
        }
        unsafe extern "system" fn GetAdapterModeCount<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, format: D3DFORMAT) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterModeCount(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format))
            }
        }
        unsafe extern "system" fn EnumAdapterModes<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::EnumAdapterModes(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&format), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetAdapterDisplayMode<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterDisplayMode(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceType<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devtype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&backbufferformat), core::mem::transmute_copy(&bwindowed)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceFormat<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceFormat(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&rtype), core::mem::transmute_copy(&checkformat)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceMultiSampleType<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: windows_core::BOOL, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceMultiSampleType(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&surfaceformat), core::mem::transmute_copy(&windowed), core::mem::transmute_copy(&multisampletype), core::mem::transmute_copy(&pqualitylevels)).into()
            }
        }
        unsafe extern "system" fn CheckDepthStencilMatch<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDepthStencilMatch(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&adapterformat), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&depthstencilformat)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceFormatConversion<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CheckDeviceFormatConversion(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&sourceformat), core::mem::transmute_copy(&targetformat)).into()
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetAdapterMonitor<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32) -> super::Gdi::HMONITOR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::GetAdapterMonitor(this, core::mem::transmute_copy(&adapter))
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IDirect3D9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9_Impl::CreateDevice(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterSoftwareDevice: RegisterSoftwareDevice::<Identity, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, OFFSET>,
            GetAdapterIdentifier: GetAdapterIdentifier::<Identity, OFFSET>,
            GetAdapterModeCount: GetAdapterModeCount::<Identity, OFFSET>,
            EnumAdapterModes: EnumAdapterModes::<Identity, OFFSET>,
            GetAdapterDisplayMode: GetAdapterDisplayMode::<Identity, OFFSET>,
            CheckDeviceType: CheckDeviceType::<Identity, OFFSET>,
            CheckDeviceFormat: CheckDeviceFormat::<Identity, OFFSET>,
            CheckDeviceMultiSampleType: CheckDeviceMultiSampleType::<Identity, OFFSET>,
            CheckDepthStencilMatch: CheckDepthStencilMatch::<Identity, OFFSET>,
            CheckDeviceFormatConversion: CheckDeviceFormatConversion::<Identity, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, OFFSET>,
            GetAdapterMonitor: GetAdapterMonitor::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3D9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3D9 {}
windows_core::imp::define_interface!(IDirect3D9Ex, IDirect3D9Ex_Vtbl, 0x02177241_69fc_400c_8ff1_93a44df6861d);
impl core::ops::Deref for IDirect3D9Ex {
    type Target = IDirect3D9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3D9Ex, windows_core::IUnknown, IDirect3D9);
impl IDirect3D9Ex {
    pub unsafe fn GetAdapterModeCountEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterModeCountEx)(windows_core::Interface::as_raw(self), adapter, pfilter) }
    }
    pub unsafe fn EnumAdapterModesEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterModesEx)(windows_core::Interface::as_raw(self), adapter, pfilter, mode, pmode as _).ok() }
    }
    pub unsafe fn GetAdapterDisplayModeEx(&self, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterDisplayModeEx)(windows_core::Interface::as_raw(self), adapter, pmode as _, protation as _).ok() }
    }
    pub unsafe fn CreateDeviceEx(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut Option<IDirect3DDevice9Ex>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateDeviceEx)(windows_core::Interface::as_raw(self), adapter, devicetype, hfocuswindow, behaviorflags, ppresentationparameters as _, pfullscreendisplaymode as _, core::mem::transmute(ppreturneddeviceinterface)).ok() }
    }
    pub unsafe fn GetAdapterLUID(&self, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterLUID)(windows_core::Interface::as_raw(self), adapter, pluid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3D9Ex_Vtbl {
    pub base__: IDirect3D9_Vtbl,
    pub GetAdapterModeCountEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3DDISPLAYMODEFILTER) -> u32,
    pub EnumAdapterModesEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3DDISPLAYMODEFILTER, u32, *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT,
    pub GetAdapterDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DDISPLAYMODEEX, *mut D3DDISPLAYROTATION) -> windows_core::HRESULT,
    pub CreateDeviceEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DDEVTYPE, super::super::Foundation::HWND, u32, *mut D3DPRESENT_PARAMETERS, *mut D3DDISPLAYMODEEX, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAdapterLUID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::LUID) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3D9Ex_Impl: IDirect3D9_Impl {
    fn GetAdapterModeCountEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32;
    fn EnumAdapterModesEx(&self, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> windows_core::Result<()>;
    fn GetAdapterDisplayModeEx(&self, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
    fn CreateDeviceEx(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: windows_core::OutRef<IDirect3DDevice9Ex>) -> windows_core::Result<()>;
    fn GetAdapterLUID(&self, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3D9Ex_Vtbl {
    pub const fn new<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdapterModeCountEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterModeCountEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pfilter))
            }
        }
        unsafe extern "system" fn EnumAdapterModesEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pfilter: *const D3DDISPLAYMODEFILTER, mode: u32, pmode: *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::EnumAdapterModesEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&mode), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetAdapterDisplayModeEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterDisplayModeEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        unsafe extern "system" fn CreateDeviceEx<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: super::super::Foundation::HWND, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX, ppreturneddeviceinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::CreateDeviceEx(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&devicetype), core::mem::transmute_copy(&hfocuswindow), core::mem::transmute_copy(&behaviorflags), core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pfullscreendisplaymode), core::mem::transmute_copy(&ppreturneddeviceinterface)).into()
            }
        }
        unsafe extern "system" fn GetAdapterLUID<Identity: IDirect3D9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, pluid: *mut super::super::Foundation::LUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3D9Ex_Impl::GetAdapterLUID(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&pluid)).into()
            }
        }
        Self {
            base__: IDirect3D9_Vtbl::new::<Identity, OFFSET>(),
            GetAdapterModeCountEx: GetAdapterModeCountEx::<Identity, OFFSET>,
            EnumAdapterModesEx: EnumAdapterModesEx::<Identity, OFFSET>,
            GetAdapterDisplayModeEx: GetAdapterDisplayModeEx::<Identity, OFFSET>,
            CreateDeviceEx: CreateDeviceEx::<Identity, OFFSET>,
            GetAdapterLUID: GetAdapterLUID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3D9Ex as windows_core::Interface>::IID || iid == &<IDirect3D9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3D9Ex {}
windows_core::imp::define_interface!(IDirect3DBaseTexture9, IDirect3DBaseTexture9_Vtbl, 0x580ca87e_1d3c_4d54_991d_b7d3e3c298ce);
impl core::ops::Deref for IDirect3DBaseTexture9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DBaseTexture9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DBaseTexture9 {
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetLOD)(windows_core::Interface::as_raw(self), lodnew) }
    }
    pub unsafe fn GetLOD(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLOD)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLevelCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLevelCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutoGenFilterType)(windows_core::Interface::as_raw(self), filtertype).ok() }
    }
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetAutoGenFilterType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        unsafe { (windows_core::Interface::vtable(self).GenerateMipSubLevels)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DBaseTexture9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    pub SetLOD: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetLOD: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLevelCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub SetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    pub GetAutoGenFilterType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3DTEXTUREFILTERTYPE,
    pub GenerateMipSubLevels: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IDirect3DBaseTexture9_Impl: IDirect3DResource9_Impl {
    fn SetLOD(&self, lodnew: u32) -> u32;
    fn GetLOD(&self) -> u32;
    fn GetLevelCount(&self) -> u32;
    fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE;
    fn GenerateMipSubLevels(&self);
}
impl IDirect3DBaseTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLOD<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lodnew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::SetLOD(this, core::mem::transmute_copy(&lodnew))
            }
        }
        unsafe extern "system" fn GetLOD<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetLOD(this)
            }
        }
        unsafe extern "system" fn GetLevelCount<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetLevelCount(this)
            }
        }
        unsafe extern "system" fn SetAutoGenFilterType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filtertype: D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::SetAutoGenFilterType(this, core::mem::transmute_copy(&filtertype)).into()
            }
        }
        unsafe extern "system" fn GetAutoGenFilterType<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DTEXTUREFILTERTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GetAutoGenFilterType(this)
            }
        }
        unsafe extern "system" fn GenerateMipSubLevels<Identity: IDirect3DBaseTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DBaseTexture9_Impl::GenerateMipSubLevels(this)
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            SetLOD: SetLOD::<Identity, OFFSET>,
            GetLOD: GetLOD::<Identity, OFFSET>,
            GetLevelCount: GetLevelCount::<Identity, OFFSET>,
            SetAutoGenFilterType: SetAutoGenFilterType::<Identity, OFFSET>,
            GetAutoGenFilterType: GetAutoGenFilterType::<Identity, OFFSET>,
            GenerateMipSubLevels: GenerateMipSubLevels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DBaseTexture9 {}
windows_core::imp::define_interface!(IDirect3DCubeTexture9, IDirect3DCubeTexture9_Vtbl, 0xfff32f81_d953_473a_9223_93d652aba93f);
impl core::ops::Deref for IDirect3DCubeTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DCubeTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DCubeTexture9 {
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _).ok() }
    }
    pub unsafe fn GetCubeMapSurface(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCubeMapSurface)(windows_core::Interface::as_raw(self), facetype, level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), facetype, level, plockedrect as _, prect, flags).ok() }
    }
    pub unsafe fn UnlockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self), facetype, level).ok() }
    }
    pub unsafe fn AddDirtyRect(&self, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyRect)(windows_core::Interface::as_raw(self), facetype, pdirtyrect).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DCubeTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DSURFACE_DESC) -> windows_core::HRESULT,
    pub GetCubeMapSurface: unsafe extern "system" fn(*mut core::ffi::c_void, D3DCUBEMAP_FACES, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, D3DCUBEMAP_FACES, u32, *mut D3DLOCKED_RECT, *const super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void, D3DCUBEMAP_FACES, u32) -> windows_core::HRESULT,
    pub AddDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, D3DCUBEMAP_FACES, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
}
pub trait IDirect3DCubeTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn GetCubeMapSurface(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn LockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self, facetype: D3DCUBEMAP_FACES, level: u32) -> windows_core::Result<()>;
    fn AddDirtyRect(&self, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl IDirect3DCubeTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetCubeMapSurface<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, ppcubemapsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DCubeTexture9_Impl::GetCubeMapSurface(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level)) {
                    Ok(ok__) => {
                        ppcubemapsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::LockRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, level: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::UnlockRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyRect<Identity: IDirect3DCubeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, facetype: D3DCUBEMAP_FACES, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DCubeTexture9_Impl::AddDirtyRect(this, core::mem::transmute_copy(&facetype), core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetCubeMapSurface: GetCubeMapSurface::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DCubeTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DCubeTexture9 {}
windows_core::imp::define_interface!(IDirect3DDevice9, IDirect3DDevice9_Vtbl, 0xd0223b96_bf7a_43fd_92bd_a43b0d82b9eb);
windows_core::imp::interface_hierarchy!(IDirect3DDevice9, windows_core::IUnknown);
impl IDirect3DDevice9 {
    pub unsafe fn TestCooperativeLevel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TestCooperativeLevel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetAvailableTextureMem(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetAvailableTextureMem)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EvictManagedResources(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EvictManagedResources)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDirect3D(&self) -> windows_core::Result<IDirect3D9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDirect3D)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceCaps)(windows_core::Interface::as_raw(self), pcaps as _).ok() }
    }
    pub unsafe fn GetDisplayMode(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), iswapchain, pmode as _).ok() }
    }
    pub unsafe fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCreationParameters)(windows_core::Interface::as_raw(self), pparameters as _).ok() }
    }
    pub unsafe fn SetCursorProperties<P2>(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCursorProperties)(windows_core::Interface::as_raw(self), xhotspot, yhotspot, pcursorbitmap.param().abi()).ok() }
    }
    pub unsafe fn SetCursorPosition(&self, x: i32, y: i32, flags: u32) {
        unsafe { (windows_core::Interface::vtable(self).SetCursorPosition)(windows_core::Interface::as_raw(self), x, y, flags) }
    }
    pub unsafe fn ShowCursor(&self, bshow: bool) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).ShowCursor)(windows_core::Interface::as_raw(self), bshow.into()) }
    }
    pub unsafe fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut Option<IDirect3DSwapChain9>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateAdditionalSwapChain)(windows_core::Interface::as_raw(self), ppresentationparameters as _, core::mem::transmute(pswapchain)).ok() }
    }
    pub unsafe fn GetSwapChain(&self, iswapchain: u32) -> windows_core::Result<IDirect3DSwapChain9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSwapChain)(windows_core::Interface::as_raw(self), iswapchain, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNumberOfSwapChains(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNumberOfSwapChains)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), ppresentationparameters as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Present(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), psourcerect, pdestrect, hdestwindowoverride, pdirtyregion).ok() }
    }
    pub unsafe fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackBuffer)(windows_core::Interface::as_raw(self), iswapchain, ibackbuffer, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRasterStatus(&self, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRasterStatus)(windows_core::Interface::as_raw(self), iswapchain, prasterstatus as _).ok() }
    }
    pub unsafe fn SetDialogBoxMode(&self, benabledialogs: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDialogBoxMode)(windows_core::Interface::as_raw(self), benabledialogs.into()).ok() }
    }
    pub unsafe fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
        unsafe { (windows_core::Interface::vtable(self).SetGammaRamp)(windows_core::Interface::as_raw(self), iswapchain, flags, pramp) }
    }
    pub unsafe fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
        unsafe { (windows_core::Interface::vtable(self).GetGammaRamp)(windows_core::Interface::as_raw(self), iswapchain, pramp as _) }
    }
    pub unsafe fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut Option<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateTexture)(windows_core::Interface::as_raw(self), width, height, levels, usage, format, pool, core::mem::transmute(pptexture), psharedhandle as _).ok() }
    }
    pub unsafe fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateVolumeTexture)(windows_core::Interface::as_raw(self), width, height, depth, levels, usage, format, pool, core::mem::transmute(ppvolumetexture), psharedhandle as _).ok() }
    }
    pub unsafe fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateCubeTexture)(windows_core::Interface::as_raw(self), edgelength, levels, usage, format, pool, core::mem::transmute(ppcubetexture), psharedhandle as _).ok() }
    }
    pub unsafe fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateVertexBuffer)(windows_core::Interface::as_raw(self), length, usage, fvf, pool, core::mem::transmute(ppvertexbuffer), psharedhandle as _).ok() }
    }
    pub unsafe fn CreateIndexBuffer(&self, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateIndexBuffer)(windows_core::Interface::as_raw(self), length, usage, format, pool, core::mem::transmute(ppindexbuffer), psharedhandle as _).ok() }
    }
    pub unsafe fn CreateRenderTarget(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTarget)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, lockable.into(), core::mem::transmute(ppsurface), psharedhandle as _).ok() }
    }
    pub unsafe fn CreateDepthStencilSurface(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilSurface)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, discard.into(), core::mem::transmute(ppsurface), psharedhandle as _).ok() }
    }
    pub unsafe fn UpdateSurface<P0, P2>(&self, psourcesurface: P0, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: P2, pdestpoint: *const super::super::Foundation::POINT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateSurface)(windows_core::Interface::as_raw(self), psourcesurface.param().abi(), psourcerect, pdestinationsurface.param().abi(), pdestpoint).ok() }
    }
    pub unsafe fn UpdateTexture<P0, P1>(&self, psourcetexture: P0, pdestinationtexture: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DBaseTexture9>,
        P1: windows_core::Param<IDirect3DBaseTexture9>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateTexture)(windows_core::Interface::as_raw(self), psourcetexture.param().abi(), pdestinationtexture.param().abi()).ok() }
    }
    pub unsafe fn GetRenderTargetData<P0, P1>(&self, prendertarget: P0, pdestsurface: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetRenderTargetData)(windows_core::Interface::as_raw(self), prendertarget.param().abi(), pdestsurface.param().abi()).ok() }
    }
    pub unsafe fn GetFrontBufferData<P1>(&self, iswapchain: u32, pdestsurface: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFrontBufferData)(windows_core::Interface::as_raw(self), iswapchain, pdestsurface.param().abi()).ok() }
    }
    pub unsafe fn StretchRect<P0, P2>(&self, psourcesurface: P0, psourcerect: *const super::super::Foundation::RECT, pdestsurface: P2, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P2: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).StretchRect)(windows_core::Interface::as_raw(self), psourcesurface.param().abi(), psourcerect, pdestsurface.param().abi(), pdestrect, filter).ok() }
    }
    pub unsafe fn ColorFill<P0>(&self, psurface: P0, prect: *const super::super::Foundation::RECT, color: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ColorFill)(windows_core::Interface::as_raw(self), psurface.param().abi(), prect, color).ok() }
    }
    pub unsafe fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateOffscreenPlainSurface)(windows_core::Interface::as_raw(self), width, height, format, pool, core::mem::transmute(ppsurface), psharedhandle as _).ok() }
    }
    pub unsafe fn SetRenderTarget<P1>(&self, rendertargetindex: u32, prendertarget: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTarget)(windows_core::Interface::as_raw(self), rendertargetindex, prendertarget.param().abi()).ok() }
    }
    pub unsafe fn GetRenderTarget(&self, rendertargetindex: u32) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderTarget)(windows_core::Interface::as_raw(self), rendertargetindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDepthStencilSurface<P0>(&self, pnewzstencil: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDepthStencilSurface)(windows_core::Interface::as_raw(self), pnewzstencil.param().abi()).ok() }
    }
    pub unsafe fn GetDepthStencilSurface(&self) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDepthStencilSurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginScene(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginScene)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EndScene(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndScene)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clear(&self, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), count, prects, flags, color, z, stencil).ok() }
    }
    pub unsafe fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), state, pmatrix).ok() }
    }
    pub unsafe fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), state, pmatrix as _).ok() }
    }
    pub unsafe fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MultiplyTransform)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetViewport)(windows_core::Interface::as_raw(self), pviewport).ok() }
    }
    pub unsafe fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetViewport)(windows_core::Interface::as_raw(self), pviewport as _).ok() }
    }
    pub unsafe fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaterial)(windows_core::Interface::as_raw(self), pmaterial).ok() }
    }
    pub unsafe fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMaterial)(windows_core::Interface::as_raw(self), pmaterial as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLight)(windows_core::Interface::as_raw(self), index, param1).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLight)(windows_core::Interface::as_raw(self), index, param1 as _).ok() }
    }
    pub unsafe fn LightEnable(&self, index: u32, enable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LightEnable)(windows_core::Interface::as_raw(self), index, enable.into()).ok() }
    }
    pub unsafe fn GetLightEnable(&self, index: u32, penable: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLightEnable)(windows_core::Interface::as_raw(self), index, penable as _).ok() }
    }
    pub unsafe fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClipPlane)(windows_core::Interface::as_raw(self), index, pplane).ok() }
    }
    pub unsafe fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClipPlane)(windows_core::Interface::as_raw(self), index, pplane as _).ok() }
    }
    pub unsafe fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRenderState)(windows_core::Interface::as_raw(self), state, value).ok() }
    }
    pub unsafe fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRenderState)(windows_core::Interface::as_raw(self), state, pvalue as _).ok() }
    }
    pub unsafe fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE) -> windows_core::Result<IDirect3DStateBlock9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStateBlock)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BeginStateBlock(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginStateBlock)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EndStateBlock(&self) -> windows_core::Result<IDirect3DStateBlock9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndStateBlock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClipStatus)(windows_core::Interface::as_raw(self), pclipstatus).ok() }
    }
    pub unsafe fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClipStatus)(windows_core::Interface::as_raw(self), pclipstatus as _).ok() }
    }
    pub unsafe fn GetTexture(&self, stage: u32) -> windows_core::Result<IDirect3DBaseTexture9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTexture)(windows_core::Interface::as_raw(self), stage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTexture<P1>(&self, stage: u32, ptexture: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDirect3DBaseTexture9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTexture)(windows_core::Interface::as_raw(self), stage, ptexture.param().abi()).ok() }
    }
    pub unsafe fn GetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTextureStageState)(windows_core::Interface::as_raw(self), stage, r#type, pvalue as _).ok() }
    }
    pub unsafe fn SetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTextureStageState)(windows_core::Interface::as_raw(self), stage, r#type, value).ok() }
    }
    pub unsafe fn GetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSamplerState)(windows_core::Interface::as_raw(self), sampler, r#type, pvalue as _).ok() }
    }
    pub unsafe fn SetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSamplerState)(windows_core::Interface::as_raw(self), sampler, r#type, value).ok() }
    }
    pub unsafe fn ValidateDevice(&self, pnumpasses: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ValidateDevice)(windows_core::Interface::as_raw(self), pnumpasses as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPaletteEntries)(windows_core::Interface::as_raw(self), palettenumber, pentries).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetPaletteEntries(&self, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPaletteEntries)(windows_core::Interface::as_raw(self), palettenumber, pentries as _).ok() }
    }
    pub unsafe fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTexturePalette)(windows_core::Interface::as_raw(self), palettenumber).ok() }
    }
    pub unsafe fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentTexturePalette)(windows_core::Interface::as_raw(self), palettenumber as _).ok() }
    }
    pub unsafe fn SetScissorRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScissorRect)(windows_core::Interface::as_raw(self), prect).ok() }
    }
    pub unsafe fn GetScissorRect(&self, prect: *mut super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetScissorRect)(windows_core::Interface::as_raw(self), prect as _).ok() }
    }
    pub unsafe fn SetSoftwareVertexProcessing(&self, bsoftware: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSoftwareVertexProcessing)(windows_core::Interface::as_raw(self), bsoftware.into()).ok() }
    }
    pub unsafe fn GetSoftwareVertexProcessing(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetSoftwareVertexProcessing)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetNPatchMode(&self, nsegments: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNPatchMode)(windows_core::Interface::as_raw(self), nsegments).ok() }
    }
    pub unsafe fn GetNPatchMode(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetNPatchMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DrawPrimitive)(windows_core::Interface::as_raw(self), primitivetype, startvertex, primitivecount).ok() }
    }
    pub unsafe fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedPrimitive)(windows_core::Interface::as_raw(self), param0, basevertexindex, minvertexindex, numvertices, startindex, primcount).ok() }
    }
    pub unsafe fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DrawPrimitiveUP)(windows_core::Interface::as_raw(self), primitivetype, primitivecount, pvertexstreamzerodata, vertexstreamzerostride).ok() }
    }
    pub unsafe fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DrawIndexedPrimitiveUP)(windows_core::Interface::as_raw(self), primitivetype, minvertexindex, numvertices, primitivecount, pindexdata, indexdataformat, pvertexstreamzerodata, vertexstreamzerostride).ok() }
    }
    pub unsafe fn ProcessVertices<P3, P4>(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: P3, pvertexdecl: P4, flags: u32) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IDirect3DVertexBuffer9>,
        P4: windows_core::Param<IDirect3DVertexDeclaration9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessVertices)(windows_core::Interface::as_raw(self), srcstartindex, destindex, vertexcount, pdestbuffer.param().abi(), pvertexdecl.param().abi(), flags).ok() }
    }
    pub unsafe fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9) -> windows_core::Result<IDirect3DVertexDeclaration9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVertexDeclaration)(windows_core::Interface::as_raw(self), pvertexelements, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexDeclaration<P0>(&self, pdecl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DVertexDeclaration9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVertexDeclaration)(windows_core::Interface::as_raw(self), pdecl.param().abi()).ok() }
    }
    pub unsafe fn GetVertexDeclaration(&self) -> windows_core::Result<IDirect3DVertexDeclaration9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexDeclaration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFVF(&self, fvf: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFVF)(windows_core::Interface::as_raw(self), fvf).ok() }
    }
    pub unsafe fn GetFVF(&self, pfvf: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFVF)(windows_core::Interface::as_raw(self), pfvf as _).ok() }
    }
    pub unsafe fn CreateVertexShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DVertexShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVertexShader)(windows_core::Interface::as_raw(self), pfunction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexShader<P0>(&self, pshader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DVertexShader9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShader)(windows_core::Interface::as_raw(self), pshader.param().abi()).ok() }
    }
    pub unsafe fn GetVertexShader(&self) -> windows_core::Result<IDirect3DVertexShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVertexShader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4fcount).ok() }
    }
    pub unsafe fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount).ok() }
    }
    pub unsafe fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4icount).ok() }
    }
    pub unsafe fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount).ok() }
    }
    pub unsafe fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVertexShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata, boolcount).ok() }
    }
    pub unsafe fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVertexShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount).ok() }
    }
    pub unsafe fn SetStreamSource<P1>(&self, streamnumber: u32, pstreamdata: P1, offsetinbytes: u32, stride: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDirect3DVertexBuffer9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSource)(windows_core::Interface::as_raw(self), streamnumber, pstreamdata.param().abi(), offsetinbytes, stride).ok() }
    }
    pub unsafe fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: *mut Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStreamSource)(windows_core::Interface::as_raw(self), streamnumber, core::mem::transmute(ppstreamdata), poffsetinbytes as _, pstride as _).ok() }
    }
    pub unsafe fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSourceFreq)(windows_core::Interface::as_raw(self), streamnumber, setting).ok() }
    }
    pub unsafe fn GetStreamSourceFreq(&self, streamnumber: u32, psetting: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStreamSourceFreq)(windows_core::Interface::as_raw(self), streamnumber, psetting as _).ok() }
    }
    pub unsafe fn SetIndices<P0>(&self, pindexdata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DIndexBuffer9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIndices)(windows_core::Interface::as_raw(self), pindexdata.param().abi()).ok() }
    }
    pub unsafe fn GetIndices(&self) -> windows_core::Result<IDirect3DIndexBuffer9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePixelShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DPixelShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePixelShader)(windows_core::Interface::as_raw(self), pfunction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPixelShader<P0>(&self, pshader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DPixelShader9>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShader)(windows_core::Interface::as_raw(self), pshader.param().abi()).ok() }
    }
    pub unsafe fn GetPixelShader(&self) -> windows_core::Result<IDirect3DPixelShader9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelShader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4fcount).ok() }
    }
    pub unsafe fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantF)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4fcount).ok() }
    }
    pub unsafe fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata, vector4icount).ok() }
    }
    pub unsafe fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantI)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, vector4icount).ok() }
    }
    pub unsafe fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPixelShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata, boolcount).ok() }
    }
    pub unsafe fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPixelShaderConstantB)(windows_core::Interface::as_raw(self), startregister, pconstantdata as _, boolcount).ok() }
    }
    pub unsafe fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DrawRectPatch)(windows_core::Interface::as_raw(self), handle, pnumsegs, prectpatchinfo).ok() }
    }
    pub unsafe fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DrawTriPatch)(windows_core::Interface::as_raw(self), handle, pnumsegs, ptripatchinfo).ok() }
    }
    pub unsafe fn DeletePatch(&self, handle: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePatch)(windows_core::Interface::as_raw(self), handle).ok() }
    }
    pub unsafe fn CreateQuery(&self, r#type: D3DQUERYTYPE) -> windows_core::Result<IDirect3DQuery9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateQuery)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TestCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAvailableTextureMem: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub EvictManagedResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDirect3D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DCAPS9) -> windows_core::HRESULT,
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DDISPLAYMODE) -> windows_core::HRESULT,
    pub GetCreationParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT,
    pub SetCursorProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCursorPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32),
    pub ShowCursor: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
    pub CreateAdditionalSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENT_PARAMETERS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumberOfSwapChains: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, super::super::Foundation::HWND, *const super::Gdi::RGNDATA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Present: usize,
    pub GetBackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, D3DBACKBUFFER_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRasterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DRASTER_STATUS) -> windows_core::HRESULT,
    pub SetDialogBoxMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const D3DGAMMARAMP),
    pub GetGammaRamp: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DGAMMARAMP),
    pub CreateTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub CreateVolumeTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, u32, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub CreateCubeTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub CreateVertexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub CreateIndexBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub CreateRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, D3DFORMAT, D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub CreateDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, D3DFORMAT, D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub UpdateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Foundation::RECT, *mut core::ffi::c_void, *const super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub UpdateTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTargetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrontBufferData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StretchRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Foundation::RECT, *mut core::ffi::c_void, *const super::super::Foundation::RECT, D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT,
    pub ColorFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
    pub CreateOffscreenPlainSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRenderTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDepthStencilSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginScene: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndScene: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3DRECT, u32, u32, f32, u32) -> windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, D3DTRANSFORMSTATETYPE, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, D3DTRANSFORMSTATETYPE, *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub MultiplyTransform: unsafe extern "system" fn(*mut core::ffi::c_void, D3DTRANSFORMSTATETYPE, *const windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub SetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DVIEWPORT9) -> windows_core::HRESULT,
    pub GetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVIEWPORT9) -> windows_core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DMATERIAL9) -> windows_core::HRESULT,
    pub GetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DMATERIAL9) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetLight: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3DLIGHT9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetLight: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetLight: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DLIGHT9) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetLight: usize,
    pub LightEnable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLightEnable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetClipPlane: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32) -> windows_core::HRESULT,
    pub GetClipPlane: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetRenderState: unsafe extern "system" fn(*mut core::ffi::c_void, D3DRENDERSTATETYPE, u32) -> windows_core::HRESULT,
    pub GetRenderState: unsafe extern "system" fn(*mut core::ffi::c_void, D3DRENDERSTATETYPE, *mut u32) -> windows_core::HRESULT,
    pub CreateStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, D3DSTATEBLOCKTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndStateBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DCLIPSTATUS9) -> windows_core::HRESULT,
    pub GetClipStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DCLIPSTATUS9) -> windows_core::HRESULT,
    pub GetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextureStageState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DTEXTURESTAGESTATETYPE, *mut u32) -> windows_core::HRESULT,
    pub SetTextureStageState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DTEXTURESTAGESTATETYPE, u32) -> windows_core::HRESULT,
    pub GetSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DSAMPLERSTATETYPE, *mut u32) -> windows_core::HRESULT,
    pub SetSamplerState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DSAMPLERSTATETYPE, u32) -> windows_core::HRESULT,
    pub ValidateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Gdi::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetPaletteEntries: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetPaletteEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::Gdi::PALETTEENTRY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetPaletteEntries: usize,
    pub SetCurrentTexturePalette: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetCurrentTexturePalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetScissorRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetScissorRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetSoftwareVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSoftwareVertexProcessing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetNPatchMode: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetNPatchMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub DrawPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, D3DPRIMITIVETYPE, u32, u32) -> windows_core::HRESULT,
    pub DrawIndexedPrimitive: unsafe extern "system" fn(*mut core::ffi::c_void, D3DPRIMITIVETYPE, i32, u32, u32, u32, u32) -> windows_core::HRESULT,
    pub DrawPrimitiveUP: unsafe extern "system" fn(*mut core::ffi::c_void, D3DPRIMITIVETYPE, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DrawIndexedPrimitiveUP: unsafe extern "system" fn(*mut core::ffi::c_void, D3DPRIMITIVETYPE, u32, u32, u32, *const core::ffi::c_void, D3DFORMAT, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ProcessVertices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DVERTEXELEMENT9, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVertexDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFVF: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFVF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CreateVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVertexShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVertexShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, u32) -> windows_core::HRESULT,
    pub SetVertexShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32, u32) -> windows_core::HRESULT,
    pub SetVertexShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GetVertexShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub SetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetStreamSourceFreq: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetStreamSourceFreq: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPixelShader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPixelShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantF: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, u32) -> windows_core::HRESULT,
    pub SetPixelShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantI: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32, u32) -> windows_core::HRESULT,
    pub SetPixelShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub GetPixelShaderConstantB: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, u32) -> windows_core::HRESULT,
    pub DrawRectPatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const D3DRECTPATCH_INFO) -> windows_core::HRESULT,
    pub DrawTriPatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const D3DTRIPATCH_INFO) -> windows_core::HRESULT,
    pub DeletePatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(*mut core::ffi::c_void, D3DQUERYTYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9_Impl: windows_core::IUnknownImpl {
    fn TestCooperativeLevel(&self) -> windows_core::Result<()>;
    fn GetAvailableTextureMem(&self) -> u32;
    fn EvictManagedResources(&self) -> windows_core::Result<()>;
    fn GetDirect3D(&self) -> windows_core::Result<IDirect3D9>;
    fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::Result<()>;
    fn SetCursorProperties(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn SetCursorPosition(&self, x: i32, y: i32, flags: u32);
    fn ShowCursor(&self, bshow: windows_core::BOOL) -> windows_core::BOOL;
    fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: windows_core::OutRef<IDirect3DSwapChain9>) -> windows_core::Result<()>;
    fn GetSwapChain(&self, iswapchain: u32) -> windows_core::Result<IDirect3DSwapChain9>;
    fn GetNumberOfSwapChains(&self) -> u32;
    fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
    fn Present(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::Result<()>;
    fn SetDialogBoxMode(&self, benabledialogs: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP);
    fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP);
    fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: windows_core::OutRef<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: windows_core::OutRef<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: windows_core::OutRef<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: windows_core::OutRef<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateIndexBuffer(&self, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: windows_core::OutRef<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateRenderTarget(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn CreateDepthStencilSurface(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn UpdateSurface(&self, psourcesurface: windows_core::Ref<IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: windows_core::Ref<IDirect3DSurface9>, pdestpoint: *const super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn UpdateTexture(&self, psourcetexture: windows_core::Ref<IDirect3DBaseTexture9>, pdestinationtexture: windows_core::Ref<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetRenderTargetData(&self, prendertarget: windows_core::Ref<IDirect3DSurface9>, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, iswapchain: u32, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn StretchRect(&self, psourcesurface: windows_core::Ref<IDirect3DSurface9>, psourcerect: *const super::super::Foundation::RECT, pdestsurface: windows_core::Ref<IDirect3DSurface9>, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> windows_core::Result<()>;
    fn ColorFill(&self, psurface: windows_core::Ref<IDirect3DSurface9>, prect: *const super::super::Foundation::RECT, color: u32) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetRenderTarget(&self, rendertargetindex: u32, prendertarget: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetRenderTarget(&self, rendertargetindex: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn SetDepthStencilSurface(&self, pnewzstencil: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetDepthStencilSurface(&self) -> windows_core::Result<IDirect3DSurface9>;
    fn BeginScene(&self) -> windows_core::Result<()>;
    fn EndScene(&self) -> windows_core::Result<()>;
    fn Clear(&self, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> windows_core::Result<()>;
    fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::Result<()>;
    fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> windows_core::Result<()>;
    fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> windows_core::Result<()>;
    fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> windows_core::Result<()>;
    fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> windows_core::Result<()>;
    fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> windows_core::Result<()>;
    fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> windows_core::Result<()>;
    fn LightEnable(&self, index: u32, enable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLightEnable(&self, index: u32, penable: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetClipPlane(&self, index: u32, pplane: *const f32) -> windows_core::Result<()>;
    fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> windows_core::Result<()>;
    fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> windows_core::Result<()>;
    fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> windows_core::Result<()>;
    fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE) -> windows_core::Result<IDirect3DStateBlock9>;
    fn BeginStateBlock(&self) -> windows_core::Result<()>;
    fn EndStateBlock(&self) -> windows_core::Result<IDirect3DStateBlock9>;
    fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> windows_core::Result<()>;
    fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> windows_core::Result<()>;
    fn GetTexture(&self, stage: u32) -> windows_core::Result<IDirect3DBaseTexture9>;
    fn SetTexture(&self, stage: u32, ptexture: windows_core::Ref<IDirect3DBaseTexture9>) -> windows_core::Result<()>;
    fn GetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> windows_core::Result<()>;
    fn SetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::Result<()>;
    fn GetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> windows_core::Result<()>;
    fn SetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> windows_core::Result<()>;
    fn ValidateDevice(&self, pnumpasses: *mut u32) -> windows_core::Result<()>;
    fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> windows_core::Result<()>;
    fn GetPaletteEntries(&self, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> windows_core::Result<()>;
    fn SetCurrentTexturePalette(&self, palettenumber: u32) -> windows_core::Result<()>;
    fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> windows_core::Result<()>;
    fn SetScissorRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetScissorRect(&self, prect: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetSoftwareVertexProcessing(&self, bsoftware: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSoftwareVertexProcessing(&self) -> windows_core::BOOL;
    fn SetNPatchMode(&self, nsegments: f32) -> windows_core::Result<()>;
    fn GetNPatchMode(&self) -> f32;
    fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::Result<()>;
    fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::Result<()>;
    fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()>;
    fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::Result<()>;
    fn ProcessVertices(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: windows_core::Ref<IDirect3DVertexBuffer9>, pvertexdecl: windows_core::Ref<IDirect3DVertexDeclaration9>, flags: u32) -> windows_core::Result<()>;
    fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9) -> windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetVertexDeclaration(&self, pdecl: windows_core::Ref<IDirect3DVertexDeclaration9>) -> windows_core::Result<()>;
    fn GetVertexDeclaration(&self) -> windows_core::Result<IDirect3DVertexDeclaration9>;
    fn SetFVF(&self, fvf: u32) -> windows_core::Result<()>;
    fn GetFVF(&self, pfvf: *mut u32) -> windows_core::Result<()>;
    fn CreateVertexShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShader(&self, pshader: windows_core::Ref<IDirect3DVertexShader9>) -> windows_core::Result<()>;
    fn GetVertexShader(&self) -> windows_core::Result<IDirect3DVertexShader9>;
    fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()>;
    fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn SetStreamSource(&self, streamnumber: u32, pstreamdata: windows_core::Ref<IDirect3DVertexBuffer9>, offsetinbytes: u32, stride: u32) -> windows_core::Result<()>;
    fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: windows_core::OutRef<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::Result<()>;
    fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> windows_core::Result<()>;
    fn GetStreamSourceFreq(&self, streamnumber: u32, psetting: *mut u32) -> windows_core::Result<()>;
    fn SetIndices(&self, pindexdata: windows_core::Ref<IDirect3DIndexBuffer9>) -> windows_core::Result<()>;
    fn GetIndices(&self) -> windows_core::Result<IDirect3DIndexBuffer9>;
    fn CreatePixelShader(&self, pfunction: *const u32) -> windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShader(&self, pshader: windows_core::Ref<IDirect3DPixelShader9>) -> windows_core::Result<()>;
    fn GetPixelShader(&self) -> windows_core::Result<IDirect3DPixelShader9>;
    fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::Result<()>;
    fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::Result<()>;
    fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::Result<()>;
    fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> windows_core::Result<()>;
    fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> windows_core::Result<()>;
    fn DeletePatch(&self, handle: u32) -> windows_core::Result<()>;
    fn CreateQuery(&self, r#type: D3DQUERYTYPE) -> windows_core::Result<IDirect3DQuery9>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TestCooperativeLevel<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::TestCooperativeLevel(this).into()
            }
        }
        unsafe extern "system" fn GetAvailableTextureMem<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetAvailableTextureMem(this)
            }
        }
        unsafe extern "system" fn EvictManagedResources<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::EvictManagedResources(this).into()
            }
        }
        unsafe extern "system" fn GetDirect3D<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppd3d9: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetDirect3D(this) {
                    Ok(ok__) => {
                        ppd3d9.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceCaps<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut D3DCAPS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetDeviceCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetDisplayMode(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetCreationParameters<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetCreationParameters(this, core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn SetCursorProperties<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xhotspot: u32, yhotspot: u32, pcursorbitmap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCursorProperties(this, core::mem::transmute_copy(&xhotspot), core::mem::transmute_copy(&yhotspot), core::mem::transmute_copy(&pcursorbitmap)).into()
            }
        }
        unsafe extern "system" fn SetCursorPosition<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, flags: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCursorPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&flags))
            }
        }
        unsafe extern "system" fn ShowCursor<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshow: windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ShowCursor(this, core::mem::transmute_copy(&bshow))
            }
        }
        unsafe extern "system" fn CreateAdditionalSwapChain<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateAdditionalSwapChain(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pswapchain)).into()
            }
        }
        unsafe extern "system" fn GetSwapChain<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetSwapChain(this, core::mem::transmute_copy(&iswapchain)) {
                    Ok(ok__) => {
                        pswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOfSwapChains<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetNumberOfSwapChains(this)
            }
        }
        unsafe extern "system" fn Reset<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Reset(this, core::mem::transmute_copy(&ppresentationparameters)).into()
            }
        }
        unsafe extern "system" fn Present<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion)).into()
            }
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetBackBuffer(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppbackbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetRasterStatus(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&prasterstatus)).into()
            }
        }
        unsafe extern "system" fn SetDialogBoxMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabledialogs: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetDialogBoxMode(this, core::mem::transmute_copy(&benabledialogs)).into()
            }
        }
        unsafe extern "system" fn SetGammaRamp<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pramp))
            }
        }
        unsafe extern "system" fn GetGammaRamp<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetGammaRamp(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pramp))
            }
        }
        unsafe extern "system" fn CreateTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&pptexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateVolumeTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateVolumeTexture(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvolumetexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateCubeTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateCubeTexture(this, core::mem::transmute_copy(&edgelength), core::mem::transmute_copy(&levels), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppcubetexture), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateVertexBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateVertexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&fvf), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppvertexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateIndexBuffer<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateIndexBuffer(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppindexbuffer), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateRenderTarget(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateDepthStencilSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn UpdateSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: *mut core::ffi::c_void, pdestpoint: *const super::super::Foundation::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::UpdateSurface(this, core::mem::transmute_copy(&psourcesurface), core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestinationsurface), core::mem::transmute_copy(&pdestpoint)).into()
            }
        }
        unsafe extern "system" fn UpdateTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetexture: *mut core::ffi::c_void, pdestinationtexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::UpdateTexture(this, core::mem::transmute_copy(&psourcetexture), core::mem::transmute_copy(&pdestinationtexture)).into()
            }
        }
        unsafe extern "system" fn GetRenderTargetData<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetRenderTargetData(this, core::mem::transmute_copy(&prendertarget), core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetFrontBufferData(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn StretchRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcesurface: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestsurface: *mut core::ffi::c_void, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::StretchRect(this, core::mem::transmute_copy(&psourcesurface), core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestsurface), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&filter)).into()
            }
        }
        unsafe extern "system" fn ColorFill<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psurface: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT, color: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ColorFill(this, core::mem::transmute_copy(&psurface), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::CreateOffscreenPlainSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        unsafe extern "system" fn SetRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: u32, prendertarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex), core::mem::transmute_copy(&prendertarget)).into()
            }
        }
        unsafe extern "system" fn GetRenderTarget<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetindex: u32, pprendertarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetRenderTarget(this, core::mem::transmute_copy(&rendertargetindex)) {
                    Ok(ok__) => {
                        pprendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewzstencil: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetDepthStencilSurface(this, core::mem::transmute_copy(&pnewzstencil)).into()
            }
        }
        unsafe extern "system" fn GetDepthStencilSurface<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppzstencilsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetDepthStencilSurface(this) {
                    Ok(ok__) => {
                        ppzstencilsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginScene<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::BeginScene(this).into()
            }
        }
        unsafe extern "system" fn EndScene<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::EndScene(this).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::Clear(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&prects), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&color), core::mem::transmute_copy(&z), core::mem::transmute_copy(&stencil)).into()
            }
        }
        unsafe extern "system" fn SetTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetTransform(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn MultiplyTransform<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: D3DTRANSFORMSTATETYPE, param1: *const windows_numerics::Matrix4x4) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::MultiplyTransform(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetViewport<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *const D3DVIEWPORT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetViewport(this, core::mem::transmute_copy(&pviewport)).into()
            }
        }
        unsafe extern "system" fn GetViewport<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewport: *mut D3DVIEWPORT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetViewport(this, core::mem::transmute_copy(&pviewport)).into()
            }
        }
        unsafe extern "system" fn SetMaterial<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *const D3DMATERIAL9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
            }
        }
        unsafe extern "system" fn GetMaterial<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaterial: *mut D3DMATERIAL9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetMaterial(this, core::mem::transmute_copy(&pmaterial)).into()
            }
        }
        unsafe extern "system" fn SetLight<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, param1: *const D3DLIGHT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetLight<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, param1: *mut D3DLIGHT9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetLight(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn LightEnable<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, enable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::LightEnable(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&enable)).into()
            }
        }
        unsafe extern "system" fn GetLightEnable<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, penable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetLightEnable(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&penable)).into()
            }
        }
        unsafe extern "system" fn SetClipPlane<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pplane: *const f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetClipPlane(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pplane)).into()
            }
        }
        unsafe extern "system" fn GetClipPlane<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pplane: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetClipPlane(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pplane)).into()
            }
        }
        unsafe extern "system" fn SetRenderState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DRENDERSTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetRenderState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetRenderState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetRenderState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn CreateStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3DSTATEBLOCKTYPE, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreateStateBlock(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppsb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::BeginStateBlock(this).into()
            }
        }
        unsafe extern "system" fn EndStateBlock<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::EndStateBlock(this) {
                    Ok(ok__) => {
                        ppsb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *const D3DCLIPSTATUS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetClipStatus(this, core::mem::transmute_copy(&pclipstatus)).into()
            }
        }
        unsafe extern "system" fn GetClipStatus<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipstatus: *mut D3DCLIPSTATUS9) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetClipStatus(this, core::mem::transmute_copy(&pclipstatus)).into()
            }
        }
        unsafe extern "system" fn GetTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, pptexture: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetTexture(this, core::mem::transmute_copy(&stage)) {
                    Ok(ok__) => {
                        pptexture.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTexture<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, ptexture: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTexture(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&ptexture)).into()
            }
        }
        unsafe extern "system" fn GetTextureStageState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetTextureStageState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetTextureStageState(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetSamplerState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pvalue)).into()
            }
        }
        unsafe extern "system" fn SetSamplerState<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetSamplerState(this, core::mem::transmute_copy(&sampler), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ValidateDevice<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpasses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ValidateDevice(this, core::mem::transmute_copy(&pnumpasses)).into()
            }
        }
        unsafe extern "system" fn SetPaletteEntries<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPaletteEntries(this, core::mem::transmute_copy(&palettenumber), core::mem::transmute_copy(&pentries)).into()
            }
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPaletteEntries(this, core::mem::transmute_copy(&palettenumber), core::mem::transmute_copy(&pentries)).into()
            }
        }
        unsafe extern "system" fn SetCurrentTexturePalette<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetCurrentTexturePalette(this, core::mem::transmute_copy(&palettenumber)).into()
            }
        }
        unsafe extern "system" fn GetCurrentTexturePalette<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palettenumber: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetCurrentTexturePalette(this, core::mem::transmute_copy(&palettenumber)).into()
            }
        }
        unsafe extern "system" fn SetScissorRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetScissorRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn GetScissorRect<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetScissorRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn SetSoftwareVertexProcessing<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsoftware: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetSoftwareVertexProcessing(this, core::mem::transmute_copy(&bsoftware)).into()
            }
        }
        unsafe extern "system" fn GetSoftwareVertexProcessing<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetSoftwareVertexProcessing(this)
            }
        }
        unsafe extern "system" fn SetNPatchMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nsegments: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetNPatchMode(this, core::mem::transmute_copy(&nsegments)).into()
            }
        }
        unsafe extern "system" fn GetNPatchMode<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetNPatchMode(this)
            }
        }
        unsafe extern "system" fn DrawPrimitive<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawPrimitive(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&startvertex), core::mem::transmute_copy(&primitivecount)).into()
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitive<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawIndexedPrimitive(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&basevertexindex), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&primcount)).into()
            }
        }
        unsafe extern "system" fn DrawPrimitiveUP<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
            }
        }
        unsafe extern "system" fn DrawIndexedPrimitiveUP<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const core::ffi::c_void, vertexstreamzerostride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawIndexedPrimitiveUP(this, core::mem::transmute_copy(&primitivetype), core::mem::transmute_copy(&minvertexindex), core::mem::transmute_copy(&numvertices), core::mem::transmute_copy(&primitivecount), core::mem::transmute_copy(&pindexdata), core::mem::transmute_copy(&indexdataformat), core::mem::transmute_copy(&pvertexstreamzerodata), core::mem::transmute_copy(&vertexstreamzerostride)).into()
            }
        }
        unsafe extern "system" fn ProcessVertices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: *mut core::ffi::c_void, pvertexdecl: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::ProcessVertices(this, core::mem::transmute_copy(&srcstartindex), core::mem::transmute_copy(&destindex), core::mem::transmute_copy(&vertexcount), core::mem::transmute_copy(&pdestbuffer), core::mem::transmute_copy(&pvertexdecl), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn CreateVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexelements: *const D3DVERTEXELEMENT9, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreateVertexDeclaration(this, core::mem::transmute_copy(&pvertexelements)) {
                    Ok(ok__) => {
                        ppdecl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexDeclaration(this, core::mem::transmute_copy(&pdecl)).into()
            }
        }
        unsafe extern "system" fn GetVertexDeclaration<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdecl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetVertexDeclaration(this) {
                    Ok(ok__) => {
                        ppdecl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFVF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fvf: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetFVF(this, core::mem::transmute_copy(&fvf)).into()
            }
        }
        unsafe extern "system" fn GetFVF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfvf: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetFVF(this, core::mem::transmute_copy(&pfvf)).into()
            }
        }
        unsafe extern "system" fn CreateVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pfunction)) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShader(this, core::mem::transmute_copy(&pshader)).into()
            }
        }
        unsafe extern "system" fn GetVertexShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetVertexShader(this) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn SetVertexShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn GetVertexShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetVertexShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn SetStreamSource<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, pstreamdata: *mut core::ffi::c_void, offsetinbytes: u32, stride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&pstreamdata), core::mem::transmute_copy(&offsetinbytes), core::mem::transmute_copy(&stride)).into()
            }
        }
        unsafe extern "system" fn GetStreamSource<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, ppstreamdata: *mut *mut core::ffi::c_void, poffsetinbytes: *mut u32, pstride: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetStreamSource(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&ppstreamdata), core::mem::transmute_copy(&poffsetinbytes), core::mem::transmute_copy(&pstride)).into()
            }
        }
        unsafe extern "system" fn SetStreamSourceFreq<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, setting: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&setting)).into()
            }
        }
        unsafe extern "system" fn GetStreamSourceFreq<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: u32, psetting: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetStreamSourceFreq(this, core::mem::transmute_copy(&streamnumber), core::mem::transmute_copy(&psetting)).into()
            }
        }
        unsafe extern "system" fn SetIndices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetIndices(this, core::mem::transmute_copy(&pindexdata)).into()
            }
        }
        unsafe extern "system" fn GetIndices<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppindexdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetIndices(this) {
                    Ok(ok__) => {
                        ppindexdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunction: *const u32, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pfunction)) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShader(this, core::mem::transmute_copy(&pshader)).into()
            }
        }
        unsafe extern "system" fn GetPixelShader<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::GetPixelShader(this) {
                    Ok(ok__) => {
                        ppshader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantF<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantF(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4fcount)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantI<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantI(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&vector4icount)).into()
            }
        }
        unsafe extern "system" fn SetPixelShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *const windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::SetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn GetPixelShaderConstantB<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startregister: u32, pconstantdata: *mut windows_core::BOOL, boolcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::GetPixelShaderConstantB(this, core::mem::transmute_copy(&startregister), core::mem::transmute_copy(&pconstantdata), core::mem::transmute_copy(&boolcount)).into()
            }
        }
        unsafe extern "system" fn DrawRectPatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawRectPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&prectpatchinfo)).into()
            }
        }
        unsafe extern "system" fn DrawTriPatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DrawTriPatch(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&pnumsegs), core::mem::transmute_copy(&ptripatchinfo)).into()
            }
        }
        unsafe extern "system" fn DeletePatch<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9_Impl::DeletePatch(this, core::mem::transmute_copy(&handle)).into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: IDirect3DDevice9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3DQUERYTYPE, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDevice9_Impl::CreateQuery(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TestCooperativeLevel: TestCooperativeLevel::<Identity, OFFSET>,
            GetAvailableTextureMem: GetAvailableTextureMem::<Identity, OFFSET>,
            EvictManagedResources: EvictManagedResources::<Identity, OFFSET>,
            GetDirect3D: GetDirect3D::<Identity, OFFSET>,
            GetDeviceCaps: GetDeviceCaps::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetCreationParameters: GetCreationParameters::<Identity, OFFSET>,
            SetCursorProperties: SetCursorProperties::<Identity, OFFSET>,
            SetCursorPosition: SetCursorPosition::<Identity, OFFSET>,
            ShowCursor: ShowCursor::<Identity, OFFSET>,
            CreateAdditionalSwapChain: CreateAdditionalSwapChain::<Identity, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, OFFSET>,
            GetNumberOfSwapChains: GetNumberOfSwapChains::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Present: Present::<Identity, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, OFFSET>,
            SetDialogBoxMode: SetDialogBoxMode::<Identity, OFFSET>,
            SetGammaRamp: SetGammaRamp::<Identity, OFFSET>,
            GetGammaRamp: GetGammaRamp::<Identity, OFFSET>,
            CreateTexture: CreateTexture::<Identity, OFFSET>,
            CreateVolumeTexture: CreateVolumeTexture::<Identity, OFFSET>,
            CreateCubeTexture: CreateCubeTexture::<Identity, OFFSET>,
            CreateVertexBuffer: CreateVertexBuffer::<Identity, OFFSET>,
            CreateIndexBuffer: CreateIndexBuffer::<Identity, OFFSET>,
            CreateRenderTarget: CreateRenderTarget::<Identity, OFFSET>,
            CreateDepthStencilSurface: CreateDepthStencilSurface::<Identity, OFFSET>,
            UpdateSurface: UpdateSurface::<Identity, OFFSET>,
            UpdateTexture: UpdateTexture::<Identity, OFFSET>,
            GetRenderTargetData: GetRenderTargetData::<Identity, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, OFFSET>,
            StretchRect: StretchRect::<Identity, OFFSET>,
            ColorFill: ColorFill::<Identity, OFFSET>,
            CreateOffscreenPlainSurface: CreateOffscreenPlainSurface::<Identity, OFFSET>,
            SetRenderTarget: SetRenderTarget::<Identity, OFFSET>,
            GetRenderTarget: GetRenderTarget::<Identity, OFFSET>,
            SetDepthStencilSurface: SetDepthStencilSurface::<Identity, OFFSET>,
            GetDepthStencilSurface: GetDepthStencilSurface::<Identity, OFFSET>,
            BeginScene: BeginScene::<Identity, OFFSET>,
            EndScene: EndScene::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            MultiplyTransform: MultiplyTransform::<Identity, OFFSET>,
            SetViewport: SetViewport::<Identity, OFFSET>,
            GetViewport: GetViewport::<Identity, OFFSET>,
            SetMaterial: SetMaterial::<Identity, OFFSET>,
            GetMaterial: GetMaterial::<Identity, OFFSET>,
            SetLight: SetLight::<Identity, OFFSET>,
            GetLight: GetLight::<Identity, OFFSET>,
            LightEnable: LightEnable::<Identity, OFFSET>,
            GetLightEnable: GetLightEnable::<Identity, OFFSET>,
            SetClipPlane: SetClipPlane::<Identity, OFFSET>,
            GetClipPlane: GetClipPlane::<Identity, OFFSET>,
            SetRenderState: SetRenderState::<Identity, OFFSET>,
            GetRenderState: GetRenderState::<Identity, OFFSET>,
            CreateStateBlock: CreateStateBlock::<Identity, OFFSET>,
            BeginStateBlock: BeginStateBlock::<Identity, OFFSET>,
            EndStateBlock: EndStateBlock::<Identity, OFFSET>,
            SetClipStatus: SetClipStatus::<Identity, OFFSET>,
            GetClipStatus: GetClipStatus::<Identity, OFFSET>,
            GetTexture: GetTexture::<Identity, OFFSET>,
            SetTexture: SetTexture::<Identity, OFFSET>,
            GetTextureStageState: GetTextureStageState::<Identity, OFFSET>,
            SetTextureStageState: SetTextureStageState::<Identity, OFFSET>,
            GetSamplerState: GetSamplerState::<Identity, OFFSET>,
            SetSamplerState: SetSamplerState::<Identity, OFFSET>,
            ValidateDevice: ValidateDevice::<Identity, OFFSET>,
            SetPaletteEntries: SetPaletteEntries::<Identity, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, OFFSET>,
            SetCurrentTexturePalette: SetCurrentTexturePalette::<Identity, OFFSET>,
            GetCurrentTexturePalette: GetCurrentTexturePalette::<Identity, OFFSET>,
            SetScissorRect: SetScissorRect::<Identity, OFFSET>,
            GetScissorRect: GetScissorRect::<Identity, OFFSET>,
            SetSoftwareVertexProcessing: SetSoftwareVertexProcessing::<Identity, OFFSET>,
            GetSoftwareVertexProcessing: GetSoftwareVertexProcessing::<Identity, OFFSET>,
            SetNPatchMode: SetNPatchMode::<Identity, OFFSET>,
            GetNPatchMode: GetNPatchMode::<Identity, OFFSET>,
            DrawPrimitive: DrawPrimitive::<Identity, OFFSET>,
            DrawIndexedPrimitive: DrawIndexedPrimitive::<Identity, OFFSET>,
            DrawPrimitiveUP: DrawPrimitiveUP::<Identity, OFFSET>,
            DrawIndexedPrimitiveUP: DrawIndexedPrimitiveUP::<Identity, OFFSET>,
            ProcessVertices: ProcessVertices::<Identity, OFFSET>,
            CreateVertexDeclaration: CreateVertexDeclaration::<Identity, OFFSET>,
            SetVertexDeclaration: SetVertexDeclaration::<Identity, OFFSET>,
            GetVertexDeclaration: GetVertexDeclaration::<Identity, OFFSET>,
            SetFVF: SetFVF::<Identity, OFFSET>,
            GetFVF: GetFVF::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            SetVertexShader: SetVertexShader::<Identity, OFFSET>,
            GetVertexShader: GetVertexShader::<Identity, OFFSET>,
            SetVertexShaderConstantF: SetVertexShaderConstantF::<Identity, OFFSET>,
            GetVertexShaderConstantF: GetVertexShaderConstantF::<Identity, OFFSET>,
            SetVertexShaderConstantI: SetVertexShaderConstantI::<Identity, OFFSET>,
            GetVertexShaderConstantI: GetVertexShaderConstantI::<Identity, OFFSET>,
            SetVertexShaderConstantB: SetVertexShaderConstantB::<Identity, OFFSET>,
            GetVertexShaderConstantB: GetVertexShaderConstantB::<Identity, OFFSET>,
            SetStreamSource: SetStreamSource::<Identity, OFFSET>,
            GetStreamSource: GetStreamSource::<Identity, OFFSET>,
            SetStreamSourceFreq: SetStreamSourceFreq::<Identity, OFFSET>,
            GetStreamSourceFreq: GetStreamSourceFreq::<Identity, OFFSET>,
            SetIndices: SetIndices::<Identity, OFFSET>,
            GetIndices: GetIndices::<Identity, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            SetPixelShader: SetPixelShader::<Identity, OFFSET>,
            GetPixelShader: GetPixelShader::<Identity, OFFSET>,
            SetPixelShaderConstantF: SetPixelShaderConstantF::<Identity, OFFSET>,
            GetPixelShaderConstantF: GetPixelShaderConstantF::<Identity, OFFSET>,
            SetPixelShaderConstantI: SetPixelShaderConstantI::<Identity, OFFSET>,
            GetPixelShaderConstantI: GetPixelShaderConstantI::<Identity, OFFSET>,
            SetPixelShaderConstantB: SetPixelShaderConstantB::<Identity, OFFSET>,
            GetPixelShaderConstantB: GetPixelShaderConstantB::<Identity, OFFSET>,
            DrawRectPatch: DrawRectPatch::<Identity, OFFSET>,
            DrawTriPatch: DrawTriPatch::<Identity, OFFSET>,
            DeletePatch: DeletePatch::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDirect3DDevice9 {}
windows_core::imp::define_interface!(IDirect3DDevice9Ex, IDirect3DDevice9Ex_Vtbl, 0xb18b10ce_2649_405a_870f_95f777d4313a);
impl core::ops::Deref for IDirect3DDevice9Ex {
    type Target = IDirect3DDevice9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DDevice9Ex, windows_core::IUnknown, IDirect3DDevice9);
impl IDirect3DDevice9Ex {
    pub unsafe fn SetConvolutionMonoKernel(&self, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConvolutionMonoKernel)(windows_core::Interface::as_raw(self), width, height, rows as _, columns as _).ok() }
    }
    pub unsafe fn ComposeRects<P0, P1, P2, P4>(&self, psrc: P0, pdst: P1, psrcrectdescs: P2, numrects: u32, pdstrectdescs: P4, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DSurface9>,
        P1: windows_core::Param<IDirect3DSurface9>,
        P2: windows_core::Param<IDirect3DVertexBuffer9>,
        P4: windows_core::Param<IDirect3DVertexBuffer9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ComposeRects)(windows_core::Interface::as_raw(self), psrc.param().abi(), pdst.param().abi(), psrcrectdescs.param().abi(), numrects, pdstrectdescs.param().abi(), operation, xoffset, yoffset).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn PresentEx(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PresentEx)(windows_core::Interface::as_raw(self), psourcerect, pdestrect, hdestwindowoverride, pdirtyregion, dwflags).ok() }
    }
    pub unsafe fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGPUThreadPriority)(windows_core::Interface::as_raw(self), ppriority as _).ok() }
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGPUThreadPriority)(windows_core::Interface::as_raw(self), priority).ok() }
    }
    pub unsafe fn WaitForVBlank(&self, iswapchain: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WaitForVBlank)(windows_core::Interface::as_raw(self), iswapchain).ok() }
    }
    pub unsafe fn CheckResourceResidency(&self, presourcearray: *mut Option<IDirect3DResource9>, numresources: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckResourceResidency)(windows_core::Interface::as_raw(self), core::mem::transmute(presourcearray), numresources).ok() }
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency).ok() }
    }
    pub unsafe fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), pmaxlatency as _).ok() }
    }
    pub unsafe fn CheckDeviceState(&self, hdestinationwindow: super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckDeviceState)(windows_core::Interface::as_raw(self), hdestinationwindow).ok() }
    }
    pub unsafe fn CreateRenderTargetEx(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateRenderTargetEx)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, lockable.into(), core::mem::transmute(ppsurface), psharedhandle as _, usage).ok() }
    }
    pub unsafe fn CreateOffscreenPlainSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateOffscreenPlainSurfaceEx)(windows_core::Interface::as_raw(self), width, height, format, pool, core::mem::transmute(ppsurface), psharedhandle as _, usage).ok() }
    }
    pub unsafe fn CreateDepthStencilSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: bool, ppsurface: *mut Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateDepthStencilSurfaceEx)(windows_core::Interface::as_raw(self), width, height, format, multisample, multisamplequality, discard.into(), core::mem::transmute(ppsurface), psharedhandle as _, usage).ok() }
    }
    pub unsafe fn ResetEx(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetEx)(windows_core::Interface::as_raw(self), ppresentationparameters as _, pfullscreendisplaymode as _).ok() }
    }
    pub unsafe fn GetDisplayModeEx(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeEx)(windows_core::Interface::as_raw(self), iswapchain, pmode as _, protation as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice9Ex_Vtbl {
    pub base__: IDirect3DDevice9_Vtbl,
    pub SetConvolutionMonoKernel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub ComposeRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, D3DCOMPOSERECTSOP, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub PresentEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, super::super::Foundation::HWND, *const super::Gdi::RGNDATA, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    PresentEx: usize,
    pub GetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WaitForVBlank: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CheckResourceResidency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CheckDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub CreateRenderTargetEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, D3DFORMAT, D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE, u32) -> windows_core::HRESULT,
    pub CreateOffscreenPlainSurfaceEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, D3DFORMAT, D3DPOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE, u32) -> windows_core::HRESULT,
    pub CreateDepthStencilSurfaceEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, D3DFORMAT, D3DMULTISAMPLE_TYPE, u32, windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::super::Foundation::HANDLE, u32) -> windows_core::HRESULT,
    pub ResetEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENT_PARAMETERS, *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT,
    pub GetDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DDISPLAYMODEEX, *mut D3DDISPLAYROTATION) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
pub trait IDirect3DDevice9Ex_Impl: IDirect3DDevice9_Impl {
    fn SetConvolutionMonoKernel(&self, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> windows_core::Result<()>;
    fn ComposeRects(&self, psrc: windows_core::Ref<IDirect3DSurface9>, pdst: windows_core::Ref<IDirect3DSurface9>, psrcrectdescs: windows_core::Ref<IDirect3DVertexBuffer9>, numrects: u32, pdstrectdescs: windows_core::Ref<IDirect3DVertexBuffer9>, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::Result<()>;
    fn PresentEx(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self, ppriority: *mut i32) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn WaitForVBlank(&self, iswapchain: u32) -> windows_core::Result<()>;
    fn CheckResourceResidency(&self, presourcearray: windows_core::OutRef<IDirect3DResource9>, numresources: u32) -> windows_core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self, pmaxlatency: *mut u32) -> windows_core::Result<()>;
    fn CheckDeviceState(&self, hdestinationwindow: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn CreateRenderTargetEx(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn CreateOffscreenPlainSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn CreateDepthStencilSurfaceEx(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: windows_core::OutRef<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::Result<()>;
    fn ResetEx(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> windows_core::Result<()>;
    fn GetDisplayModeEx(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl IDirect3DDevice9Ex_Vtbl {
    pub const fn new<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetConvolutionMonoKernel<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetConvolutionMonoKernel(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&rows), core::mem::transmute_copy(&columns)).into()
            }
        }
        unsafe extern "system" fn ComposeRects<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *mut core::ffi::c_void, pdst: *mut core::ffi::c_void, psrcrectdescs: *mut core::ffi::c_void, numrects: u32, pdstrectdescs: *mut core::ffi::c_void, operation: D3DCOMPOSERECTSOP, xoffset: i32, yoffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ComposeRects(this, core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&psrcrectdescs), core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&pdstrectdescs), core::mem::transmute_copy(&operation), core::mem::transmute_copy(&xoffset), core::mem::transmute_copy(&yoffset)).into()
            }
        }
        unsafe extern "system" fn PresentEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::PresentEx(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetGPUThreadPriority(this, core::mem::transmute_copy(&ppriority)).into()
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn WaitForVBlank<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::WaitForVBlank(this, core::mem::transmute_copy(&iswapchain)).into()
            }
        }
        unsafe extern "system" fn CheckResourceResidency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcearray: *mut *mut core::ffi::c_void, numresources: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CheckResourceResidency(this, core::mem::transmute_copy(&presourcearray), core::mem::transmute_copy(&numresources)).into()
            }
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetMaximumFrameLatency(this, core::mem::transmute_copy(&pmaxlatency)).into()
            }
        }
        unsafe extern "system" fn CheckDeviceState<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdestinationwindow: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CheckDeviceState(this, core::mem::transmute_copy(&hdestinationwindow)).into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateRenderTargetEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&lockable), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn CreateOffscreenPlainSurfaceEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateOffscreenPlainSurfaceEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilSurfaceEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: windows_core::BOOL, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE, usage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::CreateDepthStencilSurfaceEx(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&multisample), core::mem::transmute_copy(&multisamplequality), core::mem::transmute_copy(&discard), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle), core::mem::transmute_copy(&usage)).into()
            }
        }
        unsafe extern "system" fn ResetEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pfullscreendisplaymode: *mut D3DDISPLAYMODEEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::ResetEx(this, core::mem::transmute_copy(&ppresentationparameters), core::mem::transmute_copy(&pfullscreendisplaymode)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: IDirect3DDevice9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iswapchain: u32, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDevice9Ex_Impl::GetDisplayModeEx(this, core::mem::transmute_copy(&iswapchain), core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        Self {
            base__: IDirect3DDevice9_Vtbl::new::<Identity, OFFSET>(),
            SetConvolutionMonoKernel: SetConvolutionMonoKernel::<Identity, OFFSET>,
            ComposeRects: ComposeRects::<Identity, OFFSET>,
            PresentEx: PresentEx::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, OFFSET>,
            CheckResourceResidency: CheckResourceResidency::<Identity, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
            CheckDeviceState: CheckDeviceState::<Identity, OFFSET>,
            CreateRenderTargetEx: CreateRenderTargetEx::<Identity, OFFSET>,
            CreateOffscreenPlainSurfaceEx: CreateOffscreenPlainSurfaceEx::<Identity, OFFSET>,
            CreateDepthStencilSurfaceEx: CreateDepthStencilSurfaceEx::<Identity, OFFSET>,
            ResetEx: ResetEx::<Identity, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice9Ex as windows_core::Interface>::IID || iid == &<IDirect3DDevice9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDirect3DDevice9Ex {}
windows_core::imp::define_interface!(IDirect3DIndexBuffer9, IDirect3DIndexBuffer9_Vtbl, 0x7c9dd65e_d3f7_4529_acee_785830acde35);
impl core::ops::Deref for IDirect3DIndexBuffer9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DIndexBuffer9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DIndexBuffer9 {
    pub unsafe fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), offsettolock, sizetolock, ppbdata as _, flags).ok() }
    }
    pub unsafe fn Unlock(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DINDEXBUFFER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DIndexBuffer9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DINDEXBUFFER_DESC) -> windows_core::HRESULT,
}
pub trait IDirect3DIndexBuffer9_Impl: IDirect3DResource9_Impl {
    fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DINDEXBUFFER_DESC) -> windows_core::Result<()>;
}
impl IDirect3DIndexBuffer9_Vtbl {
    pub const fn new<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::Lock(this, core::mem::transmute_copy(&offsettolock), core::mem::transmute_copy(&sizetolock), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::Unlock(this).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DIndexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DINDEXBUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DIndexBuffer9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DIndexBuffer9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DIndexBuffer9 {}
windows_core::imp::define_interface!(IDirect3DPixelShader9, IDirect3DPixelShader9_Vtbl, 0x6d3bdbdc_5b02_4415_b852_ce5e8bccb289);
windows_core::imp::interface_hierarchy!(IDirect3DPixelShader9, windows_core::IUnknown);
impl IDirect3DPixelShader9 {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFunction)(windows_core::Interface::as_raw(self), param0 as _, psizeofdata as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DPixelShader9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDirect3DPixelShader9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
}
impl IDirect3DPixelShader9_Vtbl {
    pub const fn new<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DPixelShader9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunction<Identity: IDirect3DPixelShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DPixelShader9_Impl::GetFunction(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DPixelShader9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DPixelShader9 {}
windows_core::imp::define_interface!(IDirect3DQuery9, IDirect3DQuery9_Vtbl, 0xd9771460_a695_4f26_bbd3_27b840b541cc);
windows_core::imp::interface_hierarchy!(IDirect3DQuery9, windows_core::IUnknown);
impl IDirect3DQuery9 {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetType(&self) -> D3DQUERYTYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetDataSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Issue(&self, dwissueflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Issue)(windows_core::Interface::as_raw(self), dwissueflags).ok() }
    }
    pub unsafe fn GetData(&self, pdata: *mut core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pdata as _, dwsize, dwgetdataflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DQuery9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3DQUERYTYPE,
    pub GetDataSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Issue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IDirect3DQuery9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetType(&self) -> D3DQUERYTYPE;
    fn GetDataSize(&self) -> u32;
    fn Issue(&self, dwissueflags: u32) -> windows_core::Result<()>;
    fn GetData(&self, pdata: *mut core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> windows_core::Result<()>;
}
impl IDirect3DQuery9_Vtbl {
    pub const fn new<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DQuery9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DQUERYTYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetDataSize<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetDataSize(this)
            }
        }
        unsafe extern "system" fn Issue<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwissueflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::Issue(this, core::mem::transmute_copy(&dwissueflags)).into()
            }
        }
        unsafe extern "system" fn GetData<Identity: IDirect3DQuery9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, dwsize: u32, dwgetdataflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DQuery9_Impl::GetData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwgetdataflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetDataSize: GetDataSize::<Identity, OFFSET>,
            Issue: Issue::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DQuery9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DQuery9 {}
windows_core::imp::define_interface!(IDirect3DResource9, IDirect3DResource9_Vtbl, 0x05eec05d_8f7d_4362_b999_d1baf357c704);
windows_core::imp::interface_hierarchy!(IDirect3DResource9, windows_core::IUnknown);
impl IDirect3DResource9 {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags).ok() }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _).ok() }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid).ok() }
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), prioritynew) }
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn PreLoad(&self) {
        unsafe { (windows_core::Interface::vtable(self).PreLoad)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DResource9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub PreLoad: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3DRESOURCETYPE,
}
pub trait IDirect3DResource9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetPriority(&self, prioritynew: u32) -> u32;
    fn GetPriority(&self) -> u32;
    fn PreLoad(&self);
    fn GetType(&self) -> D3DRESOURCETYPE;
}
impl IDirect3DResource9_Vtbl {
    pub const fn new<Identity: IDirect3DResource9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DResource9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn SetPriority<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prioritynew: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::SetPriority(this, core::mem::transmute_copy(&prioritynew))
            }
        }
        unsafe extern "system" fn GetPriority<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetPriority(this)
            }
        }
        unsafe extern "system" fn PreLoad<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::PreLoad(this)
            }
        }
        unsafe extern "system" fn GetType<Identity: IDirect3DResource9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3DRESOURCETYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DResource9_Impl::GetType(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            PreLoad: PreLoad::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DResource9 {}
windows_core::imp::define_interface!(IDirect3DStateBlock9, IDirect3DStateBlock9_Vtbl, 0xb07c4fe5_310d_4ba8_a23c_4f0f206f218b);
windows_core::imp::interface_hierarchy!(IDirect3DStateBlock9, windows_core::IUnknown);
impl IDirect3DStateBlock9 {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Capture(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Capture)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Apply(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DStateBlock9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Capture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirect3DStateBlock9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn Capture(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
}
impl IDirect3DStateBlock9_Vtbl {
    pub const fn new<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DStateBlock9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Capture<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::Capture(this).into()
            }
        }
        unsafe extern "system" fn Apply<Identity: IDirect3DStateBlock9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DStateBlock9_Impl::Apply(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            Capture: Capture::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DStateBlock9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DStateBlock9 {}
windows_core::imp::define_interface!(IDirect3DSurface9, IDirect3DSurface9_Vtbl, 0x0cfbaf3a_9ff6_429a_99b3_a2796af8b89b);
impl core::ops::Deref for IDirect3DSurface9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DSurface9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DSurface9 {
    pub unsafe fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetContainer)(windows_core::Interface::as_raw(self), riid, ppcontainer as _).ok() }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn LockRect(&self, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), plockedrect as _, prect, flags).ok() }
    }
    pub unsafe fn UnlockRect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, phdc: *mut super::Gdi::HDC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), phdc as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC(&self, hdc: super::Gdi::HDC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), hdc).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSurface9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DSURFACE_DESC) -> windows_core::HRESULT,
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DLOCKED_RECT, *const super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3DSurface9_Impl: IDirect3DResource9_Impl {
    fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn LockRect(&self, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self) -> windows_core::Result<()>;
    fn GetDC(&self, phdc: *mut super::Gdi::HDC) -> windows_core::Result<()>;
    fn ReleaseDC(&self, hdc: super::Gdi::HDC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3DSurface9_Vtbl {
    pub const fn new<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContainer<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetContainer(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcontainer)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::LockRect(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::UnlockRect(this).into()
            }
        }
        unsafe extern "system" fn GetDC<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdc: *mut super::Gdi::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::GetDC(this, core::mem::transmute_copy(&phdc)).into()
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDirect3DSurface9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: super::Gdi::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSurface9_Impl::ReleaseDC(this, core::mem::transmute_copy(&hdc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            GetContainer: GetContainer::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            GetDC: GetDC::<Identity, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSurface9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3DSurface9 {}
windows_core::imp::define_interface!(IDirect3DSwapChain9, IDirect3DSwapChain9_Vtbl, 0x794950f2_adfc_458a_905e_10a10b0b503b);
windows_core::imp::interface_hierarchy!(IDirect3DSwapChain9, windows_core::IUnknown);
impl IDirect3DSwapChain9 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Present(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), psourcerect, pdestrect, hdestwindowoverride, pdirtyregion, dwflags).ok() }
    }
    pub unsafe fn GetFrontBufferData<P0>(&self, pdestsurface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFrontBufferData)(windows_core::Interface::as_raw(self), pdestsurface.param().abi()).ok() }
    }
    pub unsafe fn GetBackBuffer(&self, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackBuffer)(windows_core::Interface::as_raw(self), ibackbuffer, r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRasterStatus)(windows_core::Interface::as_raw(self), prasterstatus as _).ok() }
    }
    pub unsafe fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayMode)(windows_core::Interface::as_raw(self), pmode as _).ok() }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPresentParameters)(windows_core::Interface::as_raw(self), ppresentationparameters as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSwapChain9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, super::super::Foundation::HWND, *const super::Gdi::RGNDATA, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Present: usize,
    pub GetFrontBufferData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, D3DBACKBUFFER_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRasterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DRASTER_STATUS) -> windows_core::HRESULT,
    pub GetDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DDISPLAYMODE) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPresentParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3DSwapChain9_Impl: windows_core::IUnknownImpl {
    fn Present(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::Result<()>;
    fn GetFrontBufferData(&self, pdestsurface: windows_core::Ref<IDirect3DSurface9>) -> windows_core::Result<()>;
    fn GetBackBuffer(&self, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> windows_core::Result<IDirect3DSurface9>;
    fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::Result<()>;
    fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> windows_core::Result<()>;
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3DSwapChain9_Vtbl {
    pub const fn new<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Present<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: super::super::Foundation::HWND, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::Present(this, core::mem::transmute_copy(&psourcerect), core::mem::transmute_copy(&pdestrect), core::mem::transmute_copy(&hdestwindowoverride), core::mem::transmute_copy(&pdirtyregion), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetFrontBufferData<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetFrontBufferData(this, core::mem::transmute_copy(&pdestsurface)).into()
            }
        }
        unsafe extern "system" fn GetBackBuffer<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE, ppbackbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9_Impl::GetBackBuffer(this, core::mem::transmute_copy(&ibackbuffer), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppbackbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRasterStatus<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterstatus: *mut D3DRASTER_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetRasterStatus(this, core::mem::transmute_copy(&prasterstatus)).into()
            }
        }
        unsafe extern "system" fn GetDisplayMode<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut D3DDISPLAYMODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetDisplayMode(this, core::mem::transmute_copy(&pmode)).into()
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DSwapChain9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresentParameters<Identity: IDirect3DSwapChain9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9_Impl::GetPresentParameters(this, core::mem::transmute_copy(&ppresentationparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, OFFSET>,
            GetFrontBufferData: GetFrontBufferData::<Identity, OFFSET>,
            GetBackBuffer: GetBackBuffer::<Identity, OFFSET>,
            GetRasterStatus: GetRasterStatus::<Identity, OFFSET>,
            GetDisplayMode: GetDisplayMode::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPresentParameters: GetPresentParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3DSwapChain9 {}
windows_core::imp::define_interface!(IDirect3DSwapChain9Ex, IDirect3DSwapChain9Ex_Vtbl, 0x91886caf_1c3d_4d2e_a0ab_3e4c7d8d3303);
impl core::ops::Deref for IDirect3DSwapChain9Ex {
    type Target = IDirect3DSwapChain9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DSwapChain9Ex, windows_core::IUnknown, IDirect3DSwapChain9);
impl IDirect3DSwapChain9Ex {
    pub unsafe fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLastPresentCount)(windows_core::Interface::as_raw(self), plastpresentcount as _).ok() }
    }
    pub unsafe fn GetPresentStats(&self, ppresentationstatistics: *mut D3DPRESENTSTATS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPresentStats)(windows_core::Interface::as_raw(self), ppresentationstatistics as _).ok() }
    }
    pub unsafe fn GetDisplayModeEx(&self, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeEx)(windows_core::Interface::as_raw(self), pmode as _, protation as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSwapChain9Ex_Vtbl {
    pub base__: IDirect3DSwapChain9_Vtbl,
    pub GetLastPresentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPresentStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DPRESENTSTATS) -> windows_core::HRESULT,
    pub GetDisplayModeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DDISPLAYMODEEX, *mut D3DDISPLAYROTATION) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDirect3DSwapChain9Ex_Impl: IDirect3DSwapChain9_Impl {
    fn GetLastPresentCount(&self, plastpresentcount: *mut u32) -> windows_core::Result<()>;
    fn GetPresentStats(&self, ppresentationstatistics: *mut D3DPRESENTSTATS) -> windows_core::Result<()>;
    fn GetDisplayModeEx(&self, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDirect3DSwapChain9Ex_Vtbl {
    pub const fn new<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLastPresentCount<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetLastPresentCount(this, core::mem::transmute_copy(&plastpresentcount)).into()
            }
        }
        unsafe extern "system" fn GetPresentStats<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationstatistics: *mut D3DPRESENTSTATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetPresentStats(this, core::mem::transmute_copy(&ppresentationstatistics)).into()
            }
        }
        unsafe extern "system" fn GetDisplayModeEx<Identity: IDirect3DSwapChain9Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut D3DDISPLAYMODEEX, protation: *mut D3DDISPLAYROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DSwapChain9Ex_Impl::GetDisplayModeEx(this, core::mem::transmute_copy(&pmode), core::mem::transmute_copy(&protation)).into()
            }
        }
        Self {
            base__: IDirect3DSwapChain9_Vtbl::new::<Identity, OFFSET>(),
            GetLastPresentCount: GetLastPresentCount::<Identity, OFFSET>,
            GetPresentStats: GetPresentStats::<Identity, OFFSET>,
            GetDisplayModeEx: GetDisplayModeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSwapChain9Ex as windows_core::Interface>::IID || iid == &<IDirect3DSwapChain9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDirect3DSwapChain9Ex {}
windows_core::imp::define_interface!(IDirect3DTexture9, IDirect3DTexture9_Vtbl, 0x85c31227_3de5_4f00_9b3a_f11ac38c18b5);
impl core::ops::Deref for IDirect3DTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DTexture9 {
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _).ok() }
    }
    pub unsafe fn GetSurfaceLevel(&self, level: u32) -> windows_core::Result<IDirect3DSurface9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurfaceLevel)(windows_core::Interface::as_raw(self), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LockRect(&self, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockRect)(windows_core::Interface::as_raw(self), level, plockedrect as _, prect, flags).ok() }
    }
    pub unsafe fn UnlockRect(&self, level: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockRect)(windows_core::Interface::as_raw(self), level).ok() }
    }
    pub unsafe fn AddDirtyRect(&self, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyRect)(windows_core::Interface::as_raw(self), pdirtyrect).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DSURFACE_DESC) -> windows_core::HRESULT,
    pub GetSurfaceLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DLOCKED_RECT, *const super::super::Foundation::RECT, u32) -> windows_core::HRESULT,
    pub UnlockRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AddDirtyRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
}
pub trait IDirect3DTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::Result<()>;
    fn GetSurfaceLevel(&self, level: u32) -> windows_core::Result<IDirect3DSurface9>;
    fn LockRect(&self, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::Result<()>;
    fn UnlockRect(&self, level: u32) -> windows_core::Result<()>;
    fn AddDirtyRect(&self, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl IDirect3DTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut D3DSURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceLevel<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, ppsurfacelevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DTexture9_Impl::GetSurfaceLevel(this, core::mem::transmute_copy(&level)) {
                    Ok(ok__) => {
                        ppsurfacelevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, plockedrect: *mut D3DLOCKED_RECT, prect: *const super::super::Foundation::RECT, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::LockRect(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::UnlockRect(this, core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyRect<Identity: IDirect3DTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DTexture9_Impl::AddDirtyRect(this, core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetSurfaceLevel: GetSurfaceLevel::<Identity, OFFSET>,
            LockRect: LockRect::<Identity, OFFSET>,
            UnlockRect: UnlockRect::<Identity, OFFSET>,
            AddDirtyRect: AddDirtyRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DTexture9 {}
windows_core::imp::define_interface!(IDirect3DVertexBuffer9, IDirect3DVertexBuffer9_Vtbl, 0xb64bb1b5_fd70_4df6_bf91_19d0a12455e3);
impl core::ops::Deref for IDirect3DVertexBuffer9 {
    type Target = IDirect3DResource9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DVertexBuffer9, windows_core::IUnknown, IDirect3DResource9);
impl IDirect3DVertexBuffer9 {
    pub unsafe fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), offsettolock, sizetolock, ppbdata as _, flags).ok() }
    }
    pub unsafe fn Unlock(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DVERTEXBUFFER_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexBuffer9_Vtbl {
    pub base__: IDirect3DResource9_Vtbl,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT,
}
pub trait IDirect3DVertexBuffer9_Impl: IDirect3DResource9_Impl {
    fn Lock(&self, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DVERTEXBUFFER_DESC) -> windows_core::Result<()>;
}
impl IDirect3DVertexBuffer9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsettolock: u32, sizetolock: u32, ppbdata: *mut *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::Lock(this, core::mem::transmute_copy(&offsettolock), core::mem::transmute_copy(&sizetolock), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::Unlock(this).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DVertexBuffer9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DVERTEXBUFFER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexBuffer9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        Self {
            base__: IDirect3DResource9_Vtbl::new::<Identity, OFFSET>(),
            Lock: Lock::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexBuffer9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DVertexBuffer9 {}
windows_core::imp::define_interface!(IDirect3DVertexDeclaration9, IDirect3DVertexDeclaration9_Vtbl, 0xdd13c59c_36fa_4098_a8fb_c7ed39dc8546);
windows_core::imp::interface_hierarchy!(IDirect3DVertexDeclaration9, windows_core::IUnknown);
impl IDirect3DVertexDeclaration9 {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeclaration(&self, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeclaration)(windows_core::Interface::as_raw(self), pelement as _, pnumelements as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexDeclaration9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeclaration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVERTEXELEMENT9, *mut u32) -> windows_core::HRESULT,
}
pub trait IDirect3DVertexDeclaration9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetDeclaration(&self, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> windows_core::Result<()>;
}
impl IDirect3DVertexDeclaration9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVertexDeclaration9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeclaration<Identity: IDirect3DVertexDeclaration9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut D3DVERTEXELEMENT9, pnumelements: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexDeclaration9_Impl::GetDeclaration(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&pnumelements)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetDeclaration: GetDeclaration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexDeclaration9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DVertexDeclaration9 {}
windows_core::imp::define_interface!(IDirect3DVertexShader9, IDirect3DVertexShader9_Vtbl, 0xefc5557e_6265_4613_8a94_43857889eb36);
windows_core::imp::interface_hierarchy!(IDirect3DVertexShader9, windows_core::IUnknown);
impl IDirect3DVertexShader9 {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFunction)(windows_core::Interface::as_raw(self), param0 as _, psizeofdata as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVertexShader9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDirect3DVertexShader9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn GetFunction(&self, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
}
impl IDirect3DVertexShader9_Vtbl {
    pub const fn new<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVertexShader9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFunction<Identity: IDirect3DVertexShader9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVertexShader9_Impl::GetFunction(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetFunction: GetFunction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVertexShader9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DVertexShader9 {}
windows_core::imp::define_interface!(IDirect3DVolume9, IDirect3DVolume9_Vtbl, 0x24f416e6_1f67_4aa7_b88e_d33f6f3128a1);
windows_core::imp::interface_hierarchy!(IDirect3DVolume9, windows_core::IUnknown);
impl IDirect3DVolume9 {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata, sizeofdata, flags).ok() }
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), refguid, pdata as _, psizeofdata as _).ok() }
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FreePrivateData)(windows_core::Interface::as_raw(self), refguid).ok() }
    }
    pub unsafe fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetContainer)(windows_core::Interface::as_raw(self), riid, ppcontainer as _).ok() }
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3DVOLUME_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _).ok() }
    }
    pub unsafe fn LockBox(&self, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockBox)(windows_core::Interface::as_raw(self), plockedvolume as _, pbox, flags).ok() }
    }
    pub unsafe fn UnlockBox(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockBox)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVolume9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVOLUME_DESC) -> windows_core::HRESULT,
    pub LockBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DLOCKED_BOX, *const D3DBOX, u32) -> windows_core::HRESULT,
    pub UnlockBox: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirect3DVolume9_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self) -> windows_core::Result<IDirect3DDevice9>;
    fn SetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::Result<()>;
    fn FreePrivateData(&self, refguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetContainer(&self, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDesc(&self, pdesc: *mut D3DVOLUME_DESC) -> windows_core::Result<()>;
    fn LockBox(&self, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::Result<()>;
    fn UnlockBox(&self) -> windows_core::Result<()>;
}
impl IDirect3DVolume9_Vtbl {
    pub const fn new<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVolume9_Impl::GetDevice(this) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *const core::ffi::c_void, sizeofdata: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::SetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&sizeofdata), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID, pdata: *mut core::ffi::c_void, psizeofdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetPrivateData(this, core::mem::transmute_copy(&refguid), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&psizeofdata)).into()
            }
        }
        unsafe extern "system" fn FreePrivateData<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::FreePrivateData(this, core::mem::transmute_copy(&refguid)).into()
            }
        }
        unsafe extern "system" fn GetContainer<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetContainer(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcontainer)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3DVOLUME_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn LockBox<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::LockBox(this, core::mem::transmute_copy(&plockedvolume), core::mem::transmute_copy(&pbox), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockBox<Identity: IDirect3DVolume9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolume9_Impl::UnlockBox(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            FreePrivateData: FreePrivateData::<Identity, OFFSET>,
            GetContainer: GetContainer::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            LockBox: LockBox::<Identity, OFFSET>,
            UnlockBox: UnlockBox::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVolume9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DVolume9 {}
windows_core::imp::define_interface!(IDirect3DVolumeTexture9, IDirect3DVolumeTexture9_Vtbl, 0x2518526c_e789_4111_a7b9_47ef328d13e6);
impl core::ops::Deref for IDirect3DVolumeTexture9 {
    type Target = IDirect3DBaseTexture9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirect3DVolumeTexture9, windows_core::IUnknown, IDirect3DResource9, IDirect3DBaseTexture9);
impl IDirect3DVolumeTexture9 {
    pub unsafe fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DVOLUME_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLevelDesc)(windows_core::Interface::as_raw(self), level, pdesc as _).ok() }
    }
    pub unsafe fn GetVolumeLevel(&self, level: u32) -> windows_core::Result<IDirect3DVolume9> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolumeLevel)(windows_core::Interface::as_raw(self), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LockBox(&self, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockBox)(windows_core::Interface::as_raw(self), level, plockedvolume as _, pbox, flags).ok() }
    }
    pub unsafe fn UnlockBox(&self, level: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockBox)(windows_core::Interface::as_raw(self), level).ok() }
    }
    pub unsafe fn AddDirtyBox(&self, pdirtybox: *const D3DBOX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDirtyBox)(windows_core::Interface::as_raw(self), pdirtybox).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DVolumeTexture9_Vtbl {
    pub base__: IDirect3DBaseTexture9_Vtbl,
    pub GetLevelDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DVOLUME_DESC) -> windows_core::HRESULT,
    pub GetVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockBox: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3DLOCKED_BOX, *const D3DBOX, u32) -> windows_core::HRESULT,
    pub UnlockBox: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AddDirtyBox: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3DBOX) -> windows_core::HRESULT,
}
pub trait IDirect3DVolumeTexture9_Impl: IDirect3DBaseTexture9_Impl {
    fn GetLevelDesc(&self, level: u32, pdesc: *mut D3DVOLUME_DESC) -> windows_core::Result<()>;
    fn GetVolumeLevel(&self, level: u32) -> windows_core::Result<IDirect3DVolume9>;
    fn LockBox(&self, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::Result<()>;
    fn UnlockBox(&self, level: u32) -> windows_core::Result<()>;
    fn AddDirtyBox(&self, pdirtybox: *const D3DBOX) -> windows_core::Result<()>;
}
impl IDirect3DVolumeTexture9_Vtbl {
    pub const fn new<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLevelDesc<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, pdesc: *mut D3DVOLUME_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::GetLevelDesc(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, ppvolumelevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DVolumeTexture9_Impl::GetVolumeLevel(this, core::mem::transmute_copy(&level)) {
                    Ok(ok__) => {
                        ppvolumelevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32, plockedvolume: *mut D3DLOCKED_BOX, pbox: *const D3DBOX, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::LockBox(this, core::mem::transmute_copy(&level), core::mem::transmute_copy(&plockedvolume), core::mem::transmute_copy(&pbox), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn UnlockBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::UnlockBox(this, core::mem::transmute_copy(&level)).into()
            }
        }
        unsafe extern "system" fn AddDirtyBox<Identity: IDirect3DVolumeTexture9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtybox: *const D3DBOX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DVolumeTexture9_Impl::AddDirtyBox(this, core::mem::transmute_copy(&pdirtybox)).into()
            }
        }
        Self {
            base__: IDirect3DBaseTexture9_Vtbl::new::<Identity, OFFSET>(),
            GetLevelDesc: GetLevelDesc::<Identity, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, OFFSET>,
            LockBox: LockBox::<Identity, OFFSET>,
            UnlockBox: UnlockBox::<Identity, OFFSET>,
            AddDirtyBox: AddDirtyBox::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DVolumeTexture9 as windows_core::Interface>::IID || iid == &<IDirect3DResource9 as windows_core::Interface>::IID || iid == &<IDirect3DBaseTexture9 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirect3DVolumeTexture9 {}
pub type LPD3DENUMDEVICESCALLBACK = Option<unsafe extern "system" fn(lpguid: *mut windows_core::GUID, lpdevicedescription: windows_core::PCSTR, lpdevicename: windows_core::PCSTR, param3: *mut D3DDEVICEDESC, param4: *mut D3DDEVICEDESC, param5: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type LPD3DENUMDEVICESCALLBACK7 = Option<unsafe extern "system" fn(lpdevicedescription: windows_core::PCSTR, lpdevicename: windows_core::PCSTR, param2: *mut D3DDEVICEDESC7, param3: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type LPD3DENUMPIXELFORMATSCALLBACK = Option<unsafe extern "system" fn(lpddpixfmt: *mut super::DirectDraw::DDPIXELFORMAT, lpcontext: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type LPD3DENUMTEXTUREFORMATSCALLBACK = Option<unsafe extern "system" fn(lpddsd: *mut super::DirectDraw::DDSURFACEDESC, lpcontext: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type LPD3DVALIDATECALLBACK = Option<unsafe extern "system" fn(lpuserarg: *mut core::ffi::c_void, dwoffset: u32) -> windows_core::HRESULT>;
pub const MAXD3DDECLLENGTH: u32 = 64u32;
pub const MAXD3DDECLUSAGEINDEX: u32 = 15u32;
pub const MAX_DEVICE_IDENTIFIER_STRING: u32 = 512u32;
pub const PROCESSIDTYPE_DWM: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(1i32);
pub const PROCESSIDTYPE_HANDLE: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(2i32);
pub const PROCESSIDTYPE_UNKNOWN: D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE = D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(0i32);
pub const _FACD3D: u32 = 2166u32;
