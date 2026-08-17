#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[inline]
pub unsafe fn CreateDXGIFactory<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory(riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateDXGIFactory1<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory1(riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory1(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CreateDXGIFactory2<T>(flags: DXGI_CREATE_FACTORY_FLAGS) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn CreateDXGIFactory2(flags : DXGI_CREATE_FACTORY_FLAGS, riid : *const windows_core::GUID, ppfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreateDXGIFactory2(flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn DXGIDeclareAdapterRemovalSupport() -> windows_core::Result<()> {
    windows_core::link!("dxgi.dll" "system" fn DXGIDeclareAdapterRemovalSupport() -> windows_core::HRESULT);
    unsafe { DXGIDeclareAdapterRemovalSupport().ok() }
}
#[inline]
pub unsafe fn DXGIDisableVBlankVirtualization() -> windows_core::Result<()> {
    windows_core::link!("dxgi.dll" "system" fn DXGIDisableVBlankVirtualization() -> windows_core::HRESULT);
    unsafe { DXGIDisableVBlankVirtualization().ok() }
}
#[inline]
pub unsafe fn DXGIGetDebugInterface1<T>(flags: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dxgi.dll" "system" fn DXGIGetDebugInterface1(flags : u32, riid : *const windows_core::GUID, pdebug : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DXGIGetDebugInterface1(flags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
}
impl Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC1 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
}
impl Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC2 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
impl Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_ADAPTER_DESC3 {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: super::super::Foundation::LUID,
    pub Flags: DXGI_ADAPTER_FLAG3,
    pub GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    pub ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}
impl Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_ADAPTER_FLAG(pub i32);
impl DXGI_ADAPTER_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_ADAPTER_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_ADAPTER_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_ADAPTER_FLAG3(pub i32);
impl DXGI_ADAPTER_FLAG3 {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_ADAPTER_FLAG3 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_ADAPTER_FLAG3 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(4i32);
pub const DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(32i32);
pub const DXGI_ADAPTER_FLAG3_NONE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(0i32);
pub const DXGI_ADAPTER_FLAG3_REMOTE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(1i32);
pub const DXGI_ADAPTER_FLAG3_SOFTWARE: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(2i32);
pub const DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(8i32);
pub const DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES: DXGI_ADAPTER_FLAG3 = DXGI_ADAPTER_FLAG3(16i32);
pub const DXGI_ADAPTER_FLAG_NONE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(0i32);
pub const DXGI_ADAPTER_FLAG_REMOTE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(1i32);
pub const DXGI_ADAPTER_FLAG_SOFTWARE: DXGI_ADAPTER_FLAG = DXGI_ADAPTER_FLAG(2i32);
pub const DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(1i32);
pub const DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_COMPUTE_PREEMPTION_GRANULARITY(pub i32);
pub const DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(4i32);
pub const DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(3i32);
pub const DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY: DXGI_COMPUTE_PREEMPTION_GRANULARITY = DXGI_COMPUTE_PREEMPTION_GRANULARITY(2i32);
pub const DXGI_CREATE_FACTORY_DEBUG: DXGI_CREATE_FACTORY_FLAGS = DXGI_CREATE_FACTORY_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_CREATE_FACTORY_FLAGS(pub u32);
impl DXGI_CREATE_FACTORY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_CREATE_FACTORY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_CREATE_FACTORY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_CREATE_FACTORY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_CREATE_FACTORY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_CREATE_FACTORY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_DEBUG_ALL: windows_core::GUID = windows_core::GUID::from_u128(0xe48ae283_da80_490b_87e6_43e9a9cfda08);
pub const DXGI_DEBUG_APP: windows_core::GUID = windows_core::GUID::from_u128(0x06cd6e01_4219_4ebd_8709_27ed23360c62);
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1u32;
pub const DXGI_DEBUG_DX: windows_core::GUID = windows_core::GUID::from_u128(0x35cdd7fc_13b2_421d_a5d7_7e4451287d64);
pub const DXGI_DEBUG_DXGI: windows_core::GUID = windows_core::GUID::from_u128(0x25cddaa4_b1c6_47e1_ac3e_98875b5a2e2a);
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(7i32);
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_DEBUG_RLO_FLAGS(pub i32);
impl DXGI_DEBUG_RLO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(4i32);
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = DXGI_DEBUG_RLO_FLAGS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    pub PrimaryCoordinates: [f32; 16],
    pub WhitePoints: [f32; 32],
}
impl Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_ENUM_MODES(pub u32);
impl DXGI_ENUM_MODES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_ENUM_MODES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_ENUM_MODES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_ENUM_MODES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_ENUM_MODES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_ENUM_MODES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_ENUM_MODES_DISABLED_STEREO: DXGI_ENUM_MODES = DXGI_ENUM_MODES(8u32);
pub const DXGI_ENUM_MODES_INTERLACED: DXGI_ENUM_MODES = DXGI_ENUM_MODES(1u32);
pub const DXGI_ENUM_MODES_SCALING: DXGI_ENUM_MODES = DXGI_ENUM_MODES(2u32);
pub const DXGI_ENUM_MODES_STEREO: DXGI_ENUM_MODES = DXGI_ENUM_MODES(4u32);
pub const DXGI_ERROR_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x887A002B_u32 as _);
pub const DXGI_ERROR_ACCESS_LOST: windows_core::HRESULT = windows_core::HRESULT(0x887A0026_u32 as _);
pub const DXGI_ERROR_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x887A0036_u32 as _);
pub const DXGI_ERROR_CACHE_CORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x887A0033_u32 as _);
pub const DXGI_ERROR_CACHE_FULL: windows_core::HRESULT = windows_core::HRESULT(0x887A0034_u32 as _);
pub const DXGI_ERROR_CACHE_HASH_COLLISION: windows_core::HRESULT = windows_core::HRESULT(0x887A0035_u32 as _);
pub const DXGI_ERROR_CANNOT_PROTECT_CONTENT: windows_core::HRESULT = windows_core::HRESULT(0x887A002A_u32 as _);
pub const DXGI_ERROR_DEVICE_HUNG: windows_core::HRESULT = windows_core::HRESULT(0x887A0006_u32 as _);
pub const DXGI_ERROR_DEVICE_REMOVED: windows_core::HRESULT = windows_core::HRESULT(0x887A0005_u32 as _);
pub const DXGI_ERROR_DEVICE_RESET: windows_core::HRESULT = windows_core::HRESULT(0x887A0007_u32 as _);
pub const DXGI_ERROR_DRIVER_INTERNAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x887A0020_u32 as _);
pub const DXGI_ERROR_DYNAMIC_CODE_POLICY_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0x887A0031_u32 as _);
pub const DXGI_ERROR_FRAME_STATISTICS_DISJOINT: windows_core::HRESULT = windows_core::HRESULT(0x887A000B_u32 as _);
pub const DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x887A000C_u32 as _);
pub const DXGI_ERROR_HW_PROTECTION_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x887A0030_u32 as _);
pub const DXGI_ERROR_INVALID_CALL: windows_core::HRESULT = windows_core::HRESULT(0x887A0001_u32 as _);
pub const DXGI_ERROR_MODE_CHANGE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x887A0025_u32 as _);
pub const DXGI_ERROR_MORE_DATA: windows_core::HRESULT = windows_core::HRESULT(0x887A0003_u32 as _);
pub const DXGI_ERROR_MPO_UNPINNED: windows_core::HRESULT = windows_core::HRESULT(0x887A0064_u32 as _);
pub const DXGI_ERROR_NAME_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x887A002C_u32 as _);
pub const DXGI_ERROR_NONEXCLUSIVE: windows_core::HRESULT = windows_core::HRESULT(0x887A0021_u32 as _);
pub const DXGI_ERROR_NON_COMPOSITED_UI: windows_core::HRESULT = windows_core::HRESULT(0x887A0032_u32 as _);
pub const DXGI_ERROR_NOT_CURRENT: windows_core::HRESULT = windows_core::HRESULT(0x887A002E_u32 as _);
pub const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x887A0022_u32 as _);
pub const DXGI_ERROR_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x887A0002_u32 as _);
pub const DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x887A0023_u32 as _);
pub const DXGI_ERROR_REMOTE_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x887A0024_u32 as _);
pub const DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE: windows_core::HRESULT = windows_core::HRESULT(0x887A0029_u32 as _);
pub const DXGI_ERROR_SDK_COMPONENT_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x887A002D_u32 as _);
pub const DXGI_ERROR_SESSION_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x887A0028_u32 as _);
pub const DXGI_ERROR_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x887A0004_u32 as _);
pub const DXGI_ERROR_WAIT_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x887A0027_u32 as _);
pub const DXGI_ERROR_WAS_STILL_DRAWING: windows_core::HRESULT = windows_core::HRESULT(0x887A000A_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_FEATURE(pub i32);
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = DXGI_FEATURE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_FRAME_PRESENTATION_MODE(pub i32);
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSED: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(0i32);
pub const DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(3i32);
pub const DXGI_FRAME_PRESENTATION_MODE_NONE: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(2i32);
pub const DXGI_FRAME_PRESENTATION_MODE_OVERLAY: DXGI_FRAME_PRESENTATION_MODE = DXGI_FRAME_PRESENTATION_MODE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_FRAME_STATISTICS_MEDIA {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
    pub CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    pub ApprovedPresentDuration: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_GPU_PREFERENCE(pub i32);
pub const DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(2i32);
pub const DXGI_GPU_PREFERENCE_MINIMUM_POWER: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(1i32);
pub const DXGI_GPU_PREFERENCE_UNSPECIFIED: DXGI_GPU_PREFERENCE = DXGI_GPU_PREFERENCE(0i32);
pub const DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_GRAPHICS_PREEMPTION_GRANULARITY(pub i32);
pub const DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(4i32);
pub const DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(3i32);
pub const DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(1i32);
pub const DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY: DXGI_GRAPHICS_PREEMPTION_GRANULARITY = DXGI_GRAPHICS_PREEMPTION_GRANULARITY(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(pub i32);
impl DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(4i32);
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(1i32);
pub const DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED: DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS = DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_HDR_METADATA_TYPE(pub i32);
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(1i32);
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(2i32);
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = DXGI_HDR_METADATA_TYPE(0i32);
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut i32,
}
impl Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: windows_core::GUID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: i32,
    pub pDescription: *const u8,
    pub DescriptionByteLength: usize,
}
impl Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_INFO_QUEUE_MESSAGE_CATEGORY(pub i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(3i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(4i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(9i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(2i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(1i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(8i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(10i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(5i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(7i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(6i32);
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = DXGI_INFO_QUEUE_MESSAGE_CATEGORY(0i32);
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_INFO_QUEUE_MESSAGE_SEVERITY(pub i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(0i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(1i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(3i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(4i32);
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = DXGI_INFO_QUEUE_MESSAGE_SEVERITY(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
impl Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_MAP_DISCARD: DXGI_MAP_FLAGS = DXGI_MAP_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_MAP_FLAGS(pub u32);
impl DXGI_MAP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_MAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_MAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_MAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_MAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_MAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_MAP_READ: DXGI_MAP_FLAGS = DXGI_MAP_FLAGS(1u32);
pub const DXGI_MAP_WRITE: DXGI_MAP_FLAGS = DXGI_MAP_FLAGS(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_MEMORY_SEGMENT_GROUP(pub i32);
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = DXGI_MEMORY_SEGMENT_GROUP(0i32);
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = DXGI_MEMORY_SEGMENT_GROUP(1i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_MODE_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub Format: Common::DXGI_FORMAT,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Stereo: windows_core::BOOL,
}
pub const DXGI_MSG_DXGIGetDebugInterface1_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(231i32);
pub const DXGI_MSG_DXGIGetDebugInterface1_NULL_ppDebug: DXGI_Message_Id = DXGI_Message_Id(230i32);
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs2_InvalidEnumOutputs2Flag: DXGI_Message_Id = DXGI_Message_Id(257i32);
pub const DXGI_MSG_IDXGIAdapter_EnumOutputs_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(127i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetDestSize_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(245i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetSourceRect_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(243i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_GetTargetRect_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(244i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetColorSpace_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(239i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetDestSize_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(242i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetSourceRect_InvalidRect: DXGI_Message_Id = DXGI_Message_Id(240i32);
pub const DXGI_MSG_IDXGIDecodeSwapChain_SetTargetRect_InvalidRect: DXGI_Message_Id = DXGI_Message_Id(241i32);
pub const DXGI_MSG_IDXGIDevice_CreateSurface_InvalidParametersWithpSharedResource: DXGI_Message_Id = DXGI_Message_Id(63i32);
pub const DXGI_MSG_IDXGIDisplayControl_IsStereoEnabled_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(195i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCompositionSurface_InvalidHandle: DXGI_Message_Id = DXGI_Message_Id(173i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_ForegroundUnsupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(220i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidAlphaMode: DXGI_Message_Id = DXGI_Message_Id(222i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(221i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_UnsupportedOnWindows7: DXGI_Message_Id = DXGI_Message_Id(164i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsInvalid: DXGI_Message_Id = DXGI_Message_Id(172i32);
pub const DXGI_MSG_IDXGIFactory2_CreateSwapChainForCoreWindow_pWindowIsNULL: DXGI_Message_Id = DXGI_Message_Id(165i32);
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusEvent_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(193i32);
pub const DXGI_MSG_IDXGIFactory2_RegisterOcclusionStatusWindow_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(192i32);
pub const DXGI_MSG_IDXGIFactory2_UnregisterStatus_CookieNotFound: DXGI_Message_Id = DXGI_Message_Id(129i32);
pub const DXGI_MSG_IDXGIFactory7_UnregisterAdaptersChangedEvent_CookieNotFound: DXGI_Message_Id = DXGI_Message_Id(293i32);
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidFeature: DXGI_Message_Id = DXGI_Message_Id(287i32);
pub const DXGI_MSG_IDXGIFactory_CheckFeatureSupport_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(288i32);
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ModuleIsNULL: DXGI_Message_Id = DXGI_Message_Id(68i32);
pub const DXGI_MSG_IDXGIFactory_CreateSoftwareAdapter_ppAdapterInterfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(28i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidAlphaMode: DXGI_Message_Id = DXGI_Message_Id(196i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(189i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_OnlyFlipSequentialSupported: DXGI_Message_Id = DXGI_Message_Id(135i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(136i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_UnsupportedOnWindows7: DXGI_Message_Id = DXGI_Message_Id(137i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_WidthOrHeightIsZero: DXGI_Message_Id = DXGI_Message_Id(134i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForCoreWindow_InvalidSwapEffect: DXGI_Message_Id = DXGI_Message_Id(190i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainForHwnd_InvalidScaling: DXGI_Message_Id = DXGI_Message_Id(278i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChainOrRegisterOcclusionStatus_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = DXGI_Message_Id(208i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_10BitFormatNotSupported: DXGI_Message_Id = DXGI_Message_Id(272i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AllowTearingFlagIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(286i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(184i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaIsWindowlessOnly: DXGI_Message_Id = DXGI_Message_Id(183i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_AlphaUnrecognized: DXGI_Message_Id = DXGI_Message_Id(182i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_BufferCountOOBForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(100i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = DXGI_Message_Id(143i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyOnLegacy: DXGI_Message_Id = DXGI_Message_Id(186i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(144i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FSUnsupportedForModernApps: DXGI_Message_Id = DXGI_Message_Id(166i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FailedToGoFSButNonPreRotated: DXGI_Message_Id = DXGI_Message_Id(207i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSequentialNotSupportedOnD3D10: DXGI_Message_Id = DXGI_Message_Id(99i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_FlipSwapEffectRequired: DXGI_Message_Id = DXGI_Message_Id(273i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ForegroundIsCoreWindowOnly: DXGI_Message_Id = DXGI_Message_Id(219i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_HwProtectUnsupported: DXGI_Message_Id = DXGI_Message_Id(264i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidDevice: DXGI_Message_Id = DXGI_Message_Id(274i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(33i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidFormatForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(101i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidHwProtect: DXGI_Message_Id = DXGI_Message_Id(263i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_InvalidQueue: DXGI_Message_Id = DXGI_Message_Id(276i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_LegacyBltModelSwapEffect: DXGI_Message_Id = DXGI_Message_Id(294i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultiSamplingNotSupportedForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(102i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_MultipleSwapchainRefToSurface_DeferredDtr: DXGI_Message_Id = DXGI_Message_Id(297i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = DXGI_Message_Id(86i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NonPreRotatedFlagAndWindowed: DXGI_Message_Id = DXGI_Message_Id(34i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_NullDeviceInterface: DXGI_Message_Id = DXGI_Message_Id(35i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictOutputNotSupportedOnAdapter: DXGI_Message_Id = DXGI_Message_Id(119i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_RestrictToOutputAdapterMismatch: DXGI_Message_Id = DXGI_Message_Id(185i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneIsFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(141i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingNoneRequiresWindows8OrNewer: DXGI_Message_Id = DXGI_Message_Id(175i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ScalingUnrecognized: DXGI_Message_Id = DXGI_Message_Id(142i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ShaderInputUnsupported_YUV: DXGI_Message_Id = DXGI_Message_Id(254i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(124i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnknownSwapEffect: DXGI_Message_Id = DXGI_Message_Id(32i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_UnsupportedBufferUsageFlags: DXGI_Message_Id = DXGI_Message_Id(114i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreFlipModelOnly: DXGI_Message_Id = DXGI_Message_Id(210i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_WaitableSwapChainsAreNotFullscreen: DXGI_Message_Id = DXGI_Message_Id(211i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(31i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pDeviceHasMismatchedDXGIFactory: DXGI_Message_Id = DXGI_Message_Id(97i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_pRestrictToOutputFromOtherIDXGIFactory: DXGI_Message_Id = DXGI_Message_Id(118i32);
pub const DXGI_MSG_IDXGIFactory_CreateSwapChain_ppSwapChainIsNULL: DXGI_Message_Id = DXGI_Message_Id(30i32);
pub const DXGI_MSG_IDXGIFactory_Creation_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(76i32);
pub const DXGI_MSG_IDXGIFactory_EnumAdapters_ppAdapterInterfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(29i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidLUID: DXGI_Message_Id = DXGI_Message_Id(198i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_InvalidResource: DXGI_Message_Id = DXGI_Message_Id(197i32);
pub const DXGI_MSG_IDXGIFactory_GetSharedResourceAdapterLuid_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(199i32);
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(126i32);
pub const DXGI_MSG_IDXGIFactory_GetWindowAssociation_phWndIsNULL: DXGI_Message_Id = DXGI_Message_Id(36i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_InvalidFlags: DXGI_Message_Id = DXGI_Message_Id(37i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_ModernApp: DXGI_Message_Id = DXGI_Message_Id(167i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_NoOpBehavior: DXGI_Message_Id = DXGI_Message_Id(298i32);
pub const DXGI_MSG_IDXGIFactory_MakeWindowAssociation_UnavailableInSession0: DXGI_Message_Id = DXGI_Message_Id(125i32);
pub const DXGI_MSG_IDXGIFactory_Release_CalledFromDllMain: DXGI_Message_Id = DXGI_Message_Id(83i32);
pub const DXGI_MSG_IDXGIObject_GetPrivateData_puiDataSizeIsNULL: DXGI_Message_Id = DXGI_Message_Id(64i32);
pub const DXGI_MSG_IDXGIOutput1_DuplicateOutput_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(194i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_2DOnly: DXGI_Message_Id = DXGI_Message_Id(200i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_MappedOrOfferedResource: DXGI_Message_Id = DXGI_Message_Id(205i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NeedCPUAccessWrite: DXGI_Message_Id = DXGI_Message_Id(202i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_NoShared: DXGI_Message_Id = DXGI_Message_Id(203i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_OnlyMipLevels1: DXGI_Message_Id = DXGI_Message_Id(204i32);
pub const DXGI_MSG_IDXGIOutput1_GetDisplaySurfaceData1_StagingOnly: DXGI_Message_Id = DXGI_Message_Id(201i32);
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(256i32);
pub const DXGI_MSG_IDXGIOutput3_CheckOverlaySupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(255i32);
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(260i32);
pub const DXGI_MSG_IDXGIOutput4_CheckOverlayColorSpaceSupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(259i32);
pub const DXGI_MSG_IDXGIOutput6_CheckHardwareCompositionSupport_NullPointer: DXGI_Message_Id = DXGI_Message_Id(289i32);
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput1_PerMonitorDpiRequired: DXGI_Message_Id = DXGI_Message_Id(292i32);
pub const DXGI_MSG_IDXGIOutput_DuplicateOutput_PerMonitorDpiShimApplied: DXGI_Message_Id = DXGI_Message_Id(291i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_IDXGIDeviceNotSupportedBypConcernedDevice: DXGI_Message_Id = DXGI_Message_Id(69i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeFormatAndDeviceCombination: DXGI_Message_Id = DXGI_Message_Id(75i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScaling: DXGI_Message_Id = DXGI_Message_Id(74i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_InvalidDisplayModeScanlineOrdering: DXGI_Message_Id = DXGI_Message_Id(73i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasInvalidWidthOrHeight: DXGI_Message_Id = DXGI_Message_Id(46i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_ModeHasRefreshRateDenominatorZero: DXGI_Message_Id = DXGI_Message_Id(71i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(62i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(95i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_UnknownFormatIsInvalidForConfiguration: DXGI_Message_Id = DXGI_Message_Id(72i32);
pub const DXGI_MSG_IDXGIOutput_FindClosestMatchingMode_pModeToMatchOrpClosestMatchIsNULL: DXGI_Message_Id = DXGI_Message_Id(70i32);
pub const DXGI_MSG_IDXGIOutput_GetCammaControlCapabilities_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(47i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(61i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(96i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplayModeList_pNumModesIsNULL: DXGI_Message_Id = DXGI_Message_Id(45i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_ArraySizeMismatch: DXGI_Message_Id = DXGI_Message_Id(180i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_InvalidTargetSurfaceFormat: DXGI_Message_Id = DXGI_Message_Id(67i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_MapOfDestinationFailed: DXGI_Message_Id = DXGI_Message_Id(51i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(49i32);
pub const DXGI_MSG_IDXGIOutput_GetDisplaySurfaceData_pDestinationIsNULL: DXGI_Message_Id = DXGI_Message_Id(50i32);
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(52i32);
pub const DXGI_MSG_IDXGIOutput_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = DXGI_Message_Id(53i32);
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoGammaControls: DXGI_Message_Id = DXGI_Message_Id(56i32);
pub const DXGI_MSG_IDXGIOutput_GetGammaControl_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(55i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_IDXGIResourceNotSupportedBypPrimary: DXGI_Message_Id = DXGI_Message_Id(57i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_ModernApp: DXGI_Message_Id = DXGI_Message_Id(170i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(59i32);
pub const DXGI_MSG_IDXGIOutput_SetDisplaySurface_pPrimaryIsInvalid: DXGI_Message_Id = DXGI_Message_Id(58i32);
pub const DXGI_MSG_IDXGIOutput_SetGammaControl_NoOwnerDevice: DXGI_Message_Id = DXGI_Message_Id(54i32);
pub const DXGI_MSG_IDXGIOutput_SetOrGetGammaControl_pArrayIsNULL: DXGI_Message_Id = DXGI_Message_Id(81i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_FailedToAcquireFullscreenMutex: DXGI_Message_Id = DXGI_Message_Id(27i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_ModernApp: DXGI_Message_Id = DXGI_Message_Id(171i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteDeviceNotSupported: DXGI_Message_Id = DXGI_Message_Id(60i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_RemoteOutputNotSupported: DXGI_Message_Id = DXGI_Message_Id(94i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_Unsupported: DXGI_Message_Id = DXGI_Message_Id(275i32);
pub const DXGI_MSG_IDXGIOutput_TakeOwnership_pDeviceIsNULL: DXGI_Message_Id = DXGI_Message_Id(48i32);
pub const DXGI_MSG_IDXGIResource1_CreateSharedHandle_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(191i32);
pub const DXGI_MSG_IDXGIResource1_CreateSubresourceSurface_InvalidIndex: DXGI_Message_Id = DXGI_Message_Id(188i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_GDICompatibleFlagNotSet: DXGI_Message_Id = DXGI_Message_Id(89i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_ModernApp: DXGI_Message_Id = DXGI_Message_Id(174i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_SurfaceNotTexture2D: DXGI_Message_Id = DXGI_Message_Id(88i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(90i32);
pub const DXGI_MSG_IDXGISurface1_GetDC_pHdcIsNULL: DXGI_Message_Id = DXGI_Message_Id(87i32);
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_GetDCNotCalled: DXGI_Message_Id = DXGI_Message_Id(92i32);
pub const DXGI_MSG_IDXGISurface1_ReleaseDC_InvalidRectangleDimensions: DXGI_Message_Id = DXGI_Message_Id(93i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardAndReadFlagSet: DXGI_Message_Id = DXGI_Message_Id(40i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardButNotWriteFlagSet: DXGI_Message_Id = DXGI_Message_Id(41i32);
pub const DXGI_MSG_IDXGISurface_Map_DiscardFlagSetButCPUAccessIsNotDynamic: DXGI_Message_Id = DXGI_Message_Id(44i32);
pub const DXGI_MSG_IDXGISurface_Map_FlagsSetToZero: DXGI_Message_Id = DXGI_Message_Id(39i32);
pub const DXGI_MSG_IDXGISurface_Map_InvalidSurface: DXGI_Message_Id = DXGI_Message_Id(38i32);
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess: DXGI_Message_Id = DXGI_Message_Id(42i32);
pub const DXGI_MSG_IDXGISurface_Map_NoCPUAccess2: DXGI_Message_Id = DXGI_Message_Id(91i32);
pub const DXGI_MSG_IDXGISurface_Map_ReadFlagSetButCPUAccessIsDynamic: DXGI_Message_Id = DXGI_Message_Id(43i32);
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(158i32);
pub const DXGI_MSG_IDXGISwapChain1_GetRotation_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(153i32);
pub const DXGI_MSG_IDXGISwapChain1_SetBackgroundColor_OutOfRange: DXGI_Message_Id = DXGI_Message_Id(148i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(156i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_InvalidRotation: DXGI_Message_Id = DXGI_Message_Id(157i32);
pub const DXGI_MSG_IDXGISwapChain1_SetRotation_UnsupportedOS: DXGI_Message_Id = DXGI_Message_Id(152i32);
pub const DXGI_MSG_IDXGISwapChain3_CheckColorSpaceSupport_NullPointers: DXGI_Message_Id = DXGI_Message_Id(261i32);
pub const DXGI_MSG_IDXGISwapChain3_ResizeBuffers1_InvalidQueue: DXGI_Message_Id = DXGI_Message_Id(277i32);
pub const DXGI_MSG_IDXGISwapChain3_SetColorSpace1_InvalidColorSpace: DXGI_Message_Id = DXGI_Message_Id(262i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidPointer: DXGI_Message_Id = DXGI_Message_Id(280i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidSize: DXGI_Message_Id = DXGI_Message_Id(279i32);
pub const DXGI_MSG_IDXGISwapChain3_SetHDRMetaData_InvalidType: DXGI_Message_Id = DXGI_Message_Id(281i32);
pub const DXGI_MSG_IDXGISwapChain4_SetHDRMetaData_MetadataUnchanged: DXGI_Message_Id = DXGI_Message_Id(295i32);
pub const DXGI_MSG_IDXGISwapChain_CreateSwapChain_InvalidHwProtectGdiFlag: DXGI_Message_Id = DXGI_Message_Id(270i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferHeightInferred: DXGI_Message_Id = DXGI_Message_Id(2i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_BufferWidthInferred: DXGI_Message_Id = DXGI_Message_Id(1i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_InvalidOutputWindow: DXGI_Message_Id = DXGI_Message_Id(0i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrResizeBuffers_NoScanoutFlagChanged: DXGI_Message_Id = DXGI_Message_Id(3i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_FSUnsupportedForFlipDiscard: DXGI_Message_Id = DXGI_Message_Id(258i32);
pub const DXGI_MSG_IDXGISwapChain_CreationOrSetFullscreenState_StereoDisabled: DXGI_Message_Id = DXGI_Message_Id(128i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidOutputWindow: DXGI_Message_Id = DXGI_Message_Id(65i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_InvalidWindowStyle: DXGI_Message_Id = DXGI_Message_Id(78i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_MaxBufferCountExceeded: DXGI_Message_Id = DXGI_Message_Id(4i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_NoOutputWindow: DXGI_Message_Id = DXGI_Message_Id(6i32);
pub const DXGI_MSG_IDXGISwapChain_Creation_TooFewBuffers: DXGI_Message_Id = DXGI_Message_Id(5i32);
pub const DXGI_MSG_IDXGISwapChain_Destruction_OtherMethodsCalled: DXGI_Message_Id = DXGI_Message_Id(7i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_NoAllocatedBuffers: DXGI_Message_Id = DXGI_Message_Id(10i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferMustBeZero: DXGI_Message_Id = DXGI_Message_Id(11i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_iBufferOOB: DXGI_Message_Id = DXGI_Message_Id(12i32);
pub const DXGI_MSG_IDXGISwapChain_GetBuffer_ppSurfaceIsNULL: DXGI_Message_Id = DXGI_Message_Id(9i32);
pub const DXGI_MSG_IDXGISwapChain_GetCompositionSurface_WrongType: DXGI_Message_Id = DXGI_Message_Id(160i32);
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_SwapchainAdapterDoesNotControlOutput: DXGI_Message_Id = DXGI_Message_Id(80i32);
pub const DXGI_MSG_IDXGISwapChain_GetContainingOutput_ppOutputIsNULL: DXGI_Message_Id = DXGI_Message_Id(13i32);
pub const DXGI_MSG_IDXGISwapChain_GetCoreWindow_WrongType: DXGI_Message_Id = DXGI_Message_Id(161i32);
pub const DXGI_MSG_IDXGISwapChain_GetDesc_pDescIsNULL: DXGI_Message_Id = DXGI_Message_Id(8i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameLatencyWaitableObject_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(214i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_UnsupportedStatistics: DXGI_Message_Id = DXGI_Message_Id(79i32);
pub const DXGI_MSG_IDXGISwapChain_GetFrameStatistics_pStatsIsNULL: DXGI_Message_Id = DXGI_Message_Id(24i32);
pub const DXGI_MSG_IDXGISwapChain_GetFullscreenDesc_NonHwnd: DXGI_Message_Id = DXGI_Message_Id(162i32);
pub const DXGI_MSG_IDXGISwapChain_GetHwnd_WrongType: DXGI_Message_Id = DXGI_Message_Id(159i32);
pub const DXGI_MSG_IDXGISwapChain_GetLastPresentCount_pLastPresentCountIsNULL: DXGI_Message_Id = DXGI_Message_Id(25i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = DXGI_Message_Id(228i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(229i32);
pub const DXGI_MSG_IDXGISwapChain_GetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(250i32);
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(215i32);
pub const DXGI_MSG_IDXGISwapChain_GetMaximumFrameLatency_pMaxLatencyIsNULL: DXGI_Message_Id = DXGI_Message_Id(216i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(238i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_NullPointers: DXGI_Message_Id = DXGI_Message_Id(237i32);
pub const DXGI_MSG_IDXGISwapChain_GetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(248i32);
pub const DXGI_MSG_IDXGISwapChain_PresentBuffer_YUV: DXGI_Message_Id = DXGI_Message_Id(246i32);
pub const DXGI_MSG_IDXGISwapChain_Present_11On12_Released_Resource: DXGI_Message_Id = DXGI_Message_Id(296i32);
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresCreationFlag: DXGI_Message_Id = DXGI_Message_Id(284i32);
pub const DXGI_MSG_IDXGISwapChain_Present_AllowTearingRequiresPresentIntervalZero: DXGI_Message_Id = DXGI_Message_Id(283i32);
pub const DXGI_MSG_IDXGISwapChain_Present_BlitModelUsedWhileRegisteredForOcclusionStatusEvents: DXGI_Message_Id = DXGI_Message_Id(209i32);
pub const DXGI_MSG_IDXGISwapChain_Present_Decode: DXGI_Message_Id = DXGI_Message_Id(232i32);
pub const DXGI_MSG_IDXGISwapChain_Present_DirtyRectOutOfBackbufferBounds: DXGI_Message_Id = DXGI_Message_Id(113i32);
pub const DXGI_MSG_IDXGISwapChain_Present_DoNotSequenceFlagSetButPreviousBufferIsUndefined: DXGI_Message_Id = DXGI_Message_Id(115i32);
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyDirtyRect: DXGI_Message_Id = DXGI_Message_Id(112i32);
pub const DXGI_MSG_IDXGISwapChain_Present_EmptyScrollRect: DXGI_Message_Id = DXGI_Message_Id(109i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FlipModelChainMustResizeOrCreateOnFSTransition: DXGI_Message_Id = DXGI_Message_Id(117i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenAllowTearingIsInvalid: DXGI_Message_Id = DXGI_Message_Id(282i32);
pub const DXGI_MSG_IDXGISwapChain_Present_FullscreenPartialPresentIsInvalid: DXGI_Message_Id = DXGI_Message_Id(106i32);
pub const DXGI_MSG_IDXGISwapChain_Present_GetDXGIAdapterFailed: DXGI_Message_Id = DXGI_Message_Id(17i32);
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidNonPreRotatedFlag: DXGI_Message_Id = DXGI_Message_Id(15i32);
pub const DXGI_MSG_IDXGISwapChain_Present_InvalidPresentTestOrDoNotSequenceFlag: DXGI_Message_Id = DXGI_Message_Id(107i32);
pub const DXGI_MSG_IDXGISwapChain_Present_NoAllocatedBuffers: DXGI_Message_Id = DXGI_Message_Id(16i32);
pub const DXGI_MSG_IDXGISwapChain_Present_NonOptimalFSConfiguration: DXGI_Message_Id = DXGI_Message_Id(98i32);
pub const DXGI_MSG_IDXGISwapChain_Present_OtherFlagsCausingInvalidPresentTestFlag: DXGI_Message_Id = DXGI_Message_Id(123i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationBeforeStandardPresentation: DXGI_Message_Id = DXGI_Message_Id(105i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithMSAABuffers: DXGI_Message_Id = DXGI_Message_Id(155i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentationWithSwapEffectDiscard: DXGI_Message_Id = DXGI_Message_Id(181i32);
pub const DXGI_MSG_IDXGISwapChain_Present_PartialPresentation_YUV: DXGI_Message_Id = DXGI_Message_Id(251i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithDWMOffOrInvalidDisplayAffinity: DXGI_Message_Id = DXGI_Message_Id(133i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFSOrOverlay: DXGI_Message_Id = DXGI_Message_Id(130i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentInWindowedModeWithoutFlipSequential: DXGI_Message_Id = DXGI_Message_Id(131i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedContentWithRDPDriver: DXGI_Message_Id = DXGI_Message_Id(132i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ProtectedWindowlessPresentationRequiresDisplayOnly: DXGI_Message_Id = DXGI_Message_Id(146i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestartIsFullscreenOnly: DXGI_Message_Id = DXGI_Message_Id(145i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictOutputFlagWithStaleSwapChain: DXGI_Message_Id = DXGI_Message_Id(122i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagSetButInvalidpRestrictToOutput: DXGI_Message_Id = DXGI_Message_Id(120i32);
pub const DXGI_MSG_IDXGISwapChain_Present_RestrictToOutputFlagdWithFullscreen: DXGI_Message_Id = DXGI_Message_Id(121i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollInfoWithNoDirtyRectsSpecified: DXGI_Message_Id = DXGI_Message_Id(108i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBounds: DXGI_Message_Id = DXGI_Message_Id(110i32);
pub const DXGI_MSG_IDXGISwapChain_Present_ScrollRectOutOfBackbufferBoundsWithOffset: DXGI_Message_Id = DXGI_Message_Id(111i32);
pub const DXGI_MSG_IDXGISwapChain_Present_SyncIntervalOOB: DXGI_Message_Id = DXGI_Message_Id(14i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoAndPreferRight: DXGI_Message_Id = DXGI_Message_Id(176i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithDoNotSequence: DXGI_Message_Id = DXGI_Message_Id(177i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoOrPreferRightWithoutStereo: DXGI_Message_Id = DXGI_Message_Id(178i32);
pub const DXGI_MSG_IDXGISwapChain_Present_TemporaryMonoUnsupported: DXGI_Message_Id = DXGI_Message_Id(179i32);
pub const DXGI_MSG_IDXGISwapChain_Present_UnreleasedHDC: DXGI_Message_Id = DXGI_Message_Id(84i32);
pub const DXGI_MSG_IDXGISwapChain_Present_UnsupportedFlags: DXGI_Message_Id = DXGI_Message_Id(116i32);
pub const DXGI_MSG_IDXGISwapChain_Release_SwapChainIsFullscreen: DXGI_Message_Id = DXGI_Message_Id(66i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_D3D12Only: DXGI_Message_Id = DXGI_Message_Id(267i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_FlipModel: DXGI_Message_Id = DXGI_Message_Id(268i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers1_NodeMaskAndQueueRequired: DXGI_Message_Id = DXGI_Message_Id(269i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Alignment_YUV: DXGI_Message_Id = DXGI_Message_Id(253i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOB: DXGI_Message_Id = DXGI_Message_Id(18i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_BufferCountOOBForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(103i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveAllowTearingFlag: DXGI_Message_Id = DXGI_Message_Id(285i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveFlag_YUV: DXGI_Message_Id = DXGI_Message_Id(252i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveForegroundFlag: DXGI_Message_Id = DXGI_Message_Id(223i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_CannotAddOrRemoveWaitableFlag: DXGI_Message_Id = DXGI_Message_Id(213i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_Decode: DXGI_Message_Id = DXGI_Message_Id(233i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyFullscreenUnsupported: DXGI_Message_Id = DXGI_Message_Id(149i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyOnLegacy: DXGI_Message_Id = DXGI_Message_Id(187i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(150i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_HwProtectUnsupported: DXGI_Message_Id = DXGI_Message_Id(266i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidFormatForFlipSequential: DXGI_Message_Id = DXGI_Message_Id(104i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtect: DXGI_Message_Id = DXGI_Message_Id(265i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidHwProtectGdiFlag: DXGI_Message_Id = DXGI_Message_Id(271i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidNonPreRotatedFlag: DXGI_Message_Id = DXGI_Message_Id(21i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_InvalidSwapChainFlag: DXGI_Message_Id = DXGI_Message_Id(20i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_NonPreRotatedAndGDICompatibleFlags: DXGI_Message_Id = DXGI_Message_Id(85i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_UnreleasedReferences: DXGI_Message_Id = DXGI_Message_Id(19i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeBuffers_WidthOrHeightIsZero: DXGI_Message_Id = DXGI_Message_Id(140i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_InvalidWithCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(139i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_ModernApp: DXGI_Message_Id = DXGI_Message_Id(168i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_RefreshRateDivideByZero: DXGI_Message_Id = DXGI_Message_Id(22i32);
pub const DXGI_MSG_IDXGISwapChain_ResizeTarget_pNewTargetParametersIsNULL: DXGI_Message_Id = DXGI_Message_Id(169i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_CoreWindow: DXGI_Message_Id = DXGI_Message_Id(163i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_DisplayOnlyUnsupported: DXGI_Message_Id = DXGI_Message_Id(147i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSTransitionWithCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(138i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FSUnsupportedForModernApps: DXGI_Message_Id = DXGI_Message_Id(206i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_FullscreenInvalidForChildWindows: DXGI_Message_Id = DXGI_Message_Id(82i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_InvalidTarget: DXGI_Message_Id = DXGI_Message_Id(23i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_OutputNotOwnedBySwapChainDevice: DXGI_Message_Id = DXGI_Message_Id(77i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_PerMonitorDpiShimApplied: DXGI_Message_Id = DXGI_Message_Id(290i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_RemoteNotSupported: DXGI_Message_Id = DXGI_Message_Id(26i32);
pub const DXGI_MSG_IDXGISwapChain_SetFullscreenState_Waitable: DXGI_Message_Id = DXGI_Message_Id(212i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeFinite: DXGI_Message_Id = DXGI_Message_Id(226i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixMustBeTranslateAndOrScale: DXGI_Message_Id = DXGI_Message_Id(227i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_MatrixPointerCannotBeNull: DXGI_Message_Id = DXGI_Message_Id(224i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_RequiresCompositionSwapChain: DXGI_Message_Id = DXGI_Message_Id(225i32);
pub const DXGI_MSG_IDXGISwapChain_SetMatrixTransform_YUV: DXGI_Message_Id = DXGI_Message_Id(249i32);
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_MaxLatencyIsOutOfBounds: DXGI_Message_Id = DXGI_Message_Id(218i32);
pub const DXGI_MSG_IDXGISwapChain_SetMaximumFrameLatency_OnlyWaitable: DXGI_Message_Id = DXGI_Message_Id(217i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_Decode: DXGI_Message_Id = DXGI_Message_Id(235i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_FlipModel: DXGI_Message_Id = DXGI_Message_Id(234i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_WidthHeight: DXGI_Message_Id = DXGI_Message_Id(236i32);
pub const DXGI_MSG_IDXGISwapChain_SetSourceSize_YUV: DXGI_Message_Id = DXGI_Message_Id(247i32);
pub const DXGI_MSG_IDXGISwapchain_Present_FullscreenRotation: DXGI_Message_Id = DXGI_Message_Id(154i32);
pub const DXGI_MSG_IDXGISwapchain_Present_ScrollUnsupported: DXGI_Message_Id = DXGI_Message_Id(151i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_DISCARD_BufferCount: DXGI_Message_Id = DXGI_Message_Id(1001i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_Modern_CoreWindow_Only: DXGI_Message_Id = DXGI_Message_Id(1028i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FLIP_SEQUENTIAL_BufferCount: DXGI_Message_Id = DXGI_Message_Id(1027i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_FailedRegisterWithCompositor: DXGI_Message_Id = DXGI_Message_Id(1025i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_MSAA_NotSupported: DXGI_Message_Id = DXGI_Message_Id(1021i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow: DXGI_Message_Id = DXGI_Message_Id(1000i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_NotForegroundWindow_AtRendering: DXGI_Message_Id = DXGI_Message_Id(1026i32);
pub const DXGI_MSG_Phone_IDXGIFactory_CreateSwapChain_ScalingAspectRatioStretch_Supported_ModernApp: DXGI_Message_Id = DXGI_Message_Id(1022i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_GetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(1031i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_GetFrameStatistics_NotAvailable_ModernApp: DXGI_Message_Id = DXGI_Message_Id(1023i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present1_RequiresOverlays: DXGI_Message_Id = DXGI_Message_Id(1029i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidBlend: DXGI_Message_Id = DXGI_Message_Id(1009i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidDestinationRect: DXGI_Message_Id = DXGI_Message_Id(1016i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidFlag: DXGI_Message_Id = DXGI_Message_Id(1019i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForOverlay: DXGI_Message_Id = DXGI_Message_Id(1013i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidIndexForPrimary: DXGI_Message_Id = DXGI_Message_Id(1012i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidInterval: DXGI_Message_Id = DXGI_Message_Id(1020i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerFlag: DXGI_Message_Id = DXGI_Message_Id(1007i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidLayerIndex: DXGI_Message_Id = DXGI_Message_Id(1005i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidMultiPlaneOverlayResource: DXGI_Message_Id = DXGI_Message_Id(1011i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidResource: DXGI_Message_Id = DXGI_Message_Id(1010i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidRotation: DXGI_Message_Id = DXGI_Message_Id(1008i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSourceRect: DXGI_Message_Id = DXGI_Message_Id(1015i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_InvalidSubResourceIndex: DXGI_Message_Id = DXGI_Message_Id(1014i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleLayerIndex: DXGI_Message_Id = DXGI_Message_Id(1006i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_MultipleResource: DXGI_Message_Id = DXGI_Message_Id(1017i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_NotSharedResource: DXGI_Message_Id = DXGI_Message_Id(1018i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_Present_ReplaceInterval0With1: DXGI_Message_Id = DXGI_Message_Id(1024i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeBuffers_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1003i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_ResizeTarget_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1004i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_SetBackgroundColor_FlipSequentialRequired: DXGI_Message_Id = DXGI_Message_Id(1030i32);
pub const DXGI_MSG_Phone_IDXGISwapChain_SetFullscreenState_NotAvailable: DXGI_Message_Id = DXGI_Message_Id(1002i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(pub i32);
impl DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(2i32);
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(1i32);
pub const DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS = DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_MWA_FLAGS(pub u32);
impl DXGI_MWA_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_MWA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_MWA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_MWA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_MWA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_MWA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_MWA_NO_ALT_ENTER: DXGI_MWA_FLAGS = DXGI_MWA_FLAGS(2u32);
pub const DXGI_MWA_NO_PRINT_SCREEN: DXGI_MWA_FLAGS = DXGI_MWA_FLAGS(4u32);
pub const DXGI_MWA_NO_WINDOW_CHANGES: DXGI_MWA_FLAGS = DXGI_MWA_FLAGS(1u32);
pub const DXGI_MWA_VALID: DXGI_MWA_FLAGS = DXGI_MWA_FLAGS(7u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_Message_Id(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_OFFER_RESOURCE_FLAGS(pub i32);
impl DXGI_OFFER_RESOURCE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_OFFER_RESOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_OFFER_RESOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_OFFER_RESOURCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_OFFER_RESOURCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_OFFER_RESOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = DXGI_OFFER_RESOURCE_FLAGS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_OFFER_RESOURCE_PRIORITY(pub i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_HIGH: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(3i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_LOW: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(1i32);
pub const DXGI_OFFER_RESOURCE_PRIORITY_NORMAL: DXGI_OFFER_RESOURCE_PRIORITY = DXGI_OFFER_RESOURCE_PRIORITY(2i32);
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = DXGI_OUTDUPL_FLAG(1i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_DESC {
    pub ModeDesc: Common::DXGI_MODE_DESC,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub DesktopImageInSystemMemory: windows_core::BOOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_OUTDUPL_FLAG(pub i32);
impl DXGI_OUTDUPL_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_OUTDUPL_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_OUTDUPL_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_OUTDUPL_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_OUTDUPL_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_OUTDUPL_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_FRAME_INFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: windows_core::BOOL,
    pub ProtectedContentMaskedOut: windows_core::BOOL,
    pub PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    pub SourcePoint: super::super::Foundation::POINT,
    pub DestinationRect: super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_POINTER_POSITION {
    pub Position: super::super::Foundation::POINT,
    pub Visible: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: u32,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::super::Foundation::POINT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_TYPE(pub i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(2i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(4i32);
pub const DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: DXGI_OUTDUPL_POINTER_SHAPE_TYPE = DXGI_OUTDUPL_POINTER_SHAPE_TYPE(1i32);
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_OUTPUT_DESC {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: windows_core::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_OUTPUT_DESC1 {
    pub DeviceName: [u16; 32],
    pub DesktopCoordinates: super::super::Foundation::RECT,
    pub AttachedToDesktop: windows_core::BOOL,
    pub Rotation: Common::DXGI_MODE_ROTATION,
    pub Monitor: super::Gdi::HMONITOR,
    pub BitsPerColor: u32,
    pub ColorSpace: Common::DXGI_COLOR_SPACE_TYPE,
    pub RedPrimary: [f32; 2],
    pub GreenPrimary: [f32; 2],
    pub BluePrimary: [f32; 2],
    pub WhitePoint: [f32; 2],
    pub MinLuminance: f32,
    pub MaxLuminance: f32,
    pub MaxFullFrameLuminance: f32,
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(pub i32);
impl DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_OVERLAY_SUPPORT_FLAG(pub i32);
impl DXGI_OVERLAY_SUPPORT_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_OVERLAY_SUPPORT_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_OVERLAY_SUPPORT_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_OVERLAY_SUPPORT_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_OVERLAY_SUPPORT_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_OVERLAY_SUPPORT_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT: DXGI_OVERLAY_SUPPORT_FLAG = DXGI_OVERLAY_SUPPORT_FLAG(1i32);
pub const DXGI_OVERLAY_SUPPORT_FLAG_SCALING: DXGI_OVERLAY_SUPPORT_FLAG = DXGI_OVERLAY_SUPPORT_FLAG(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_PRESENT(pub u32);
impl DXGI_PRESENT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_PRESENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_PRESENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_PRESENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_PRESENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_PRESENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_PRESENT_ALLOW_TEARING: DXGI_PRESENT = DXGI_PRESENT(512u32);
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: DXGI_PRESENT = DXGI_PRESENT(2u32);
pub const DXGI_PRESENT_DO_NOT_WAIT: DXGI_PRESENT = DXGI_PRESENT(8u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut super::super::Foundation::RECT,
    pub pScrollRect: *mut super::super::Foundation::RECT,
    pub pScrollOffset: *mut super::super::Foundation::POINT,
}
impl Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGI_PRESENT_RESTART: DXGI_PRESENT = DXGI_PRESENT(4u32);
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: DXGI_PRESENT = DXGI_PRESENT(64u32);
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: DXGI_PRESENT = DXGI_PRESENT(16u32);
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: DXGI_PRESENT = DXGI_PRESENT(32u32);
pub const DXGI_PRESENT_TEST: DXGI_PRESENT = DXGI_PRESENT(1u32);
pub const DXGI_PRESENT_USE_DURATION: DXGI_PRESENT = DXGI_PRESENT(256u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_RECLAIM_RESOURCE_RESULTS(pub i32);
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(1i32);
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(2i32);
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS = DXGI_RECLAIM_RESOURCE_RESULTS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_RESIDENCY(pub i32);
pub const DXGI_RESIDENCY_EVICTED_TO_DISK: DXGI_RESIDENCY = DXGI_RESIDENCY(3i32);
pub const DXGI_RESIDENCY_FULLY_RESIDENT: DXGI_RESIDENCY = DXGI_RESIDENCY(1i32);
pub const DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY: DXGI_RESIDENCY = DXGI_RESIDENCY(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_RESOURCE_PRIORITY(pub u32);
pub const DXGI_RESOURCE_PRIORITY_HIGH: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(2684354560u32);
pub const DXGI_RESOURCE_PRIORITY_LOW: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(1342177280u32);
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(3355443200u32);
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(671088640u32);
pub const DXGI_RESOURCE_PRIORITY_NORMAL: DXGI_RESOURCE_PRIORITY = DXGI_RESOURCE_PRIORITY(2013265920u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SCALING(pub i32);
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = DXGI_SCALING(2i32);
pub const DXGI_SCALING_NONE: DXGI_SCALING = DXGI_SCALING(1i32);
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = DXGI_SCALING(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: super::super::Foundation::HANDLE,
}
pub const DXGI_SHARED_RESOURCE_READ: DXGI_SHARED_RESOURCE_RW = DXGI_SHARED_RESOURCE_RW(2147483648u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SHARED_RESOURCE_RW(pub u32);
impl DXGI_SHARED_RESOURCE_RW {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_SHARED_RESOURCE_RW {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_SHARED_RESOURCE_RW {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_SHARED_RESOURCE_RW {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_SHARED_RESOURCE_RW {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_SHARED_RESOURCE_RW {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_SHARED_RESOURCE_WRITE: DXGI_SHARED_RESOURCE_RW = DXGI_SHARED_RESOURCE_RW(1u32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(pub i32);
impl DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(2i32);
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG(1i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: Common::DXGI_MODE_DESC,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: super::super::Foundation::HWND,
    pub Windowed: windows_core::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub Stereo: windows_core::BOOL,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: Common::DXGI_ALPHA_MODE,
    pub Flags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_FLAG(pub i32);
impl DXGI_SWAP_CHAIN_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_SWAP_CHAIN_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_SWAP_CHAIN_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_SWAP_CHAIN_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_SWAP_CHAIN_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_SWAP_CHAIN_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2i32);
pub const DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(2048i32);
pub const DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(32i32);
pub const DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(128i32);
pub const DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(64i32);
pub const DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(256i32);
pub const DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4i32);
pub const DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1024i32);
pub const DXGI_SWAP_CHAIN_FLAG_NONPREROTATED: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(1i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(8i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(4096i32);
pub const DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(16i32);
pub const DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO: DXGI_SWAP_CHAIN_FLAG = DXGI_SWAP_CHAIN_FLAG(512i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: Common::DXGI_RATIONAL,
    pub ScanlineOrdering: Common::DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: Common::DXGI_MODE_SCALING,
    pub Windowed: windows_core::BOOL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_EFFECT(pub i32);
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(0i32);
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(4i32);
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(3i32);
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = DXGI_SWAP_EFFECT(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_USAGE(pub u32);
impl DXGI_USAGE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DXGI_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DXGI_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DXGI_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DXGI_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DXGI_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DXGI_USAGE_BACK_BUFFER: DXGI_USAGE = DXGI_USAGE(64u32);
pub const DXGI_USAGE_DISCARD_ON_PRESENT: DXGI_USAGE = DXGI_USAGE(512u32);
pub const DXGI_USAGE_READ_ONLY: DXGI_USAGE = DXGI_USAGE(256u32);
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: DXGI_USAGE = DXGI_USAGE(32u32);
pub const DXGI_USAGE_SHADER_INPUT: DXGI_USAGE = DXGI_USAGE(16u32);
pub const DXGI_USAGE_SHARED: DXGI_USAGE = DXGI_USAGE(128u32);
pub const DXGI_USAGE_UNORDERED_ACCESS: DXGI_USAGE = DXGI_USAGE(1024u32);
windows_core::imp::define_interface!(IDXGIAdapter, IDXGIAdapter_Vtbl, 0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
impl core::ops::Deref for IDXGIAdapter {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter, windows_core::IUnknown, IDXGIObject);
impl IDXGIAdapter {
    pub unsafe fn EnumOutputs(&self, output: u32) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumOutputs)(windows_core::Interface::as_raw(self), output, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDesc(&self) -> windows_core::Result<DXGI_ADAPTER_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckInterfaceSupport)(windows_core::Interface::as_raw(self), interfacename, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumOutputs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT,
    pub CheckInterfaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut i64) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIAdapter {}
unsafe impl Sync for IDXGIAdapter {}
pub trait IDXGIAdapter_Impl: IDXGIObject_Impl {
    fn EnumOutputs(&self, output: u32) -> windows_core::Result<IDXGIOutput>;
    fn GetDesc(&self) -> windows_core::Result<DXGI_ADAPTER_DESC>;
    fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64>;
}
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: IDXGIAdapter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumOutputs<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: u32, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::EnumOutputs(this, core::mem::transmute_copy(&output)) {
                    Ok(ok__) => {
                        ppoutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::GetDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacename: *const windows_core::GUID, pumdversion: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::CheckInterfaceSupport(this, core::mem::transmute_copy(&interfacename)) {
                    Ok(ok__) => {
                        pumdversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIAdapter {}
windows_core::imp::define_interface!(IDXGIAdapter1, IDXGIAdapter1_Vtbl, 0x29038f61_3839_4626_91fd_086879011a05);
impl core::ops::Deref for IDXGIAdapter1 {
    type Target = IDXGIAdapter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter1, windows_core::IUnknown, IDXGIObject, IDXGIAdapter);
impl IDXGIAdapter1 {
    pub unsafe fn GetDesc1(&self) -> windows_core::Result<DXGI_ADAPTER_DESC1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter1_Vtbl {
    pub base__: IDXGIAdapter_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIAdapter1 {}
unsafe impl Sync for IDXGIAdapter1 {}
pub trait IDXGIAdapter1_Impl: IDXGIAdapter_Impl {
    fn GetDesc1(&self) -> windows_core::Result<DXGI_ADAPTER_DESC1>;
}
impl IDXGIAdapter1_Vtbl {
    pub const fn new<Identity: IDXGIAdapter1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGIAdapter1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter1_Impl::GetDesc1(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIAdapter_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIAdapter1 {}
windows_core::imp::define_interface!(IDXGIAdapter2, IDXGIAdapter2_Vtbl, 0x0aa1ae0a_fa0e_4b84_8644_e05ff8e5acb5);
impl core::ops::Deref for IDXGIAdapter2 {
    type Target = IDXGIAdapter1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter2, windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1);
impl IDXGIAdapter2 {
    pub unsafe fn GetDesc2(&self) -> windows_core::Result<DXGI_ADAPTER_DESC2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc2)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter2_Vtbl {
    pub base__: IDXGIAdapter1_Vtbl,
    pub GetDesc2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIAdapter2 {}
unsafe impl Sync for IDXGIAdapter2 {}
pub trait IDXGIAdapter2_Impl: IDXGIAdapter1_Impl {
    fn GetDesc2(&self) -> windows_core::Result<DXGI_ADAPTER_DESC2>;
}
impl IDXGIAdapter2_Vtbl {
    pub const fn new<Identity: IDXGIAdapter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc2<Identity: IDXGIAdapter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter2_Impl::GetDesc2(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIAdapter1_Vtbl::new::<Identity, OFFSET>(), GetDesc2: GetDesc2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIAdapter2 {}
windows_core::imp::define_interface!(IDXGIAdapter3, IDXGIAdapter3_Vtbl, 0x645967a4_1392_4310_a798_8053ce3e93fd);
impl core::ops::Deref for IDXGIAdapter3 {
    type Target = IDXGIAdapter2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter3, windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter2);
impl IDXGIAdapter3 {
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterHardwareContentProtectionTeardownStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        unsafe { (windows_core::Interface::vtable(self).UnregisterHardwareContentProtectionTeardownStatus)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryVideoMemoryInfo)(windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, pvideomemoryinfo as _).ok() }
    }
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVideoMemoryReservation)(windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, reservation).ok() }
    }
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterVideoMemoryBudgetChangeNotificationEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        unsafe { (windows_core::Interface::vtable(self).UnregisterVideoMemoryBudgetChangeNotification)(windows_core::Interface::as_raw(self), dwcookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter3_Vtbl {
    pub base__: IDXGIAdapter2_Vtbl,
    pub RegisterHardwareContentProtectionTeardownStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
    pub UnregisterHardwareContentProtectionTeardownStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub QueryVideoMemoryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_MEMORY_SEGMENT_GROUP, *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT,
    pub SetVideoMemoryReservation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_MEMORY_SEGMENT_GROUP, u64) -> windows_core::HRESULT,
    pub RegisterVideoMemoryBudgetChangeNotificationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
    pub UnregisterVideoMemoryBudgetChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
unsafe impl Send for IDXGIAdapter3 {}
unsafe impl Sync for IDXGIAdapter3 {}
pub trait IDXGIAdapter3_Impl: IDXGIAdapter2_Impl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32);
    fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::Result<()>;
    fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::Result<()>;
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32);
}
impl IDXGIAdapter3_Vtbl {
    pub const fn new<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter3_Impl::RegisterHardwareContentProtectionTeardownStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::UnregisterHardwareContentProtectionTeardownStatus(this, core::mem::transmute_copy(&dwcookie))
            }
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::QueryVideoMemoryInfo(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&pvideomemoryinfo)).into()
            }
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::SetVideoMemoryReservation(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&reservation)).into()
            }
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter3_Impl::RegisterVideoMemoryBudgetChangeNotificationEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::UnregisterVideoMemoryBudgetChangeNotification(this, core::mem::transmute_copy(&dwcookie))
            }
        }
        Self {
            base__: IDXGIAdapter2_Vtbl::new::<Identity, OFFSET>(),
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Identity, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Identity, OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Identity, OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Identity, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Identity, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIAdapter2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIAdapter3 {}
windows_core::imp::define_interface!(IDXGIAdapter4, IDXGIAdapter4_Vtbl, 0x3c8d99d1_4fbf_4181_a82c_af66bf7bd24e);
impl core::ops::Deref for IDXGIAdapter4 {
    type Target = IDXGIAdapter3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter4, windows_core::IUnknown, IDXGIObject, IDXGIAdapter, IDXGIAdapter1, IDXGIAdapter2, IDXGIAdapter3);
impl IDXGIAdapter4 {
    pub unsafe fn GetDesc3(&self) -> windows_core::Result<DXGI_ADAPTER_DESC3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc3)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter4_Vtbl {
    pub base__: IDXGIAdapter3_Vtbl,
    pub GetDesc3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIAdapter4 {}
unsafe impl Sync for IDXGIAdapter4 {}
pub trait IDXGIAdapter4_Impl: IDXGIAdapter3_Impl {
    fn GetDesc3(&self) -> windows_core::Result<DXGI_ADAPTER_DESC3>;
}
impl IDXGIAdapter4_Vtbl {
    pub const fn new<Identity: IDXGIAdapter4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc3<Identity: IDXGIAdapter4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter4_Impl::GetDesc3(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIAdapter3_Vtbl::new::<Identity, OFFSET>(), GetDesc3: GetDesc3::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIAdapter as windows_core::Interface>::IID || iid == &<IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<IDXGIAdapter2 as windows_core::Interface>::IID || iid == &<IDXGIAdapter3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIAdapter4 {}
windows_core::imp::define_interface!(IDXGIDebug, IDXGIDebug_Vtbl, 0x119e7452_de9e_40fe_8806_88f90c12b441);
windows_core::imp::interface_hierarchy!(IDXGIDebug, windows_core::IUnknown);
impl IDXGIDebug {
    pub unsafe fn ReportLiveObjects(&self, apiid: windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportLiveObjects)(windows_core::Interface::as_raw(self), core::mem::transmute(apiid), flags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportLiveObjects: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDebug {}
unsafe impl Sync for IDXGIDebug {}
pub trait IDXGIDebug_Impl: windows_core::IUnknownImpl {
    fn ReportLiveObjects(&self, apiid: &windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::Result<()>;
}
impl IDXGIDebug_Vtbl {
    pub const fn new<Identity: IDXGIDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportLiveObjects<Identity: IDXGIDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apiid: windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug_Impl::ReportLiveObjects(this, core::mem::transmute(&apiid), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportLiveObjects: ReportLiveObjects::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDebug {}
windows_core::imp::define_interface!(IDXGIDebug1, IDXGIDebug1_Vtbl, 0xc5a05f0c_16f2_4adf_9f4d_a8c4d58ac550);
impl core::ops::Deref for IDXGIDebug1 {
    type Target = IDXGIDebug;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDebug1, windows_core::IUnknown, IDXGIDebug);
impl IDXGIDebug1 {
    pub unsafe fn EnableLeakTrackingForThread(&self) {
        unsafe { (windows_core::Interface::vtable(self).EnableLeakTrackingForThread)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DisableLeakTrackingForThread(&self) {
        unsafe { (windows_core::Interface::vtable(self).DisableLeakTrackingForThread)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsLeakTrackingEnabledForThread)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug1_Vtbl {
    pub base__: IDXGIDebug_Vtbl,
    pub EnableLeakTrackingForThread: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub DisableLeakTrackingForThread: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub IsLeakTrackingEnabledForThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
unsafe impl Send for IDXGIDebug1 {}
unsafe impl Sync for IDXGIDebug1 {}
pub trait IDXGIDebug1_Impl: IDXGIDebug_Impl {
    fn EnableLeakTrackingForThread(&self);
    fn DisableLeakTrackingForThread(&self);
    fn IsLeakTrackingEnabledForThread(&self) -> windows_core::BOOL;
}
impl IDXGIDebug1_Vtbl {
    pub const fn new<Identity: IDXGIDebug1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableLeakTrackingForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::EnableLeakTrackingForThread(this)
            }
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::DisableLeakTrackingForThread(this)
            }
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::IsLeakTrackingEnabledForThread(this)
            }
        }
        Self {
            base__: IDXGIDebug_Vtbl::new::<Identity, OFFSET>(),
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Identity, OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Identity, OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug1 as windows_core::Interface>::IID || iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDebug1 {}
windows_core::imp::define_interface!(IDXGIDecodeSwapChain, IDXGIDecodeSwapChain_Vtbl, 0x2633066b_4514_4c7a_8fd8_12ea98059d18);
windows_core::imp::interface_hierarchy!(IDXGIDecodeSwapChain, windows_core::IUnknown);
impl IDXGIDecodeSwapChain {
    pub unsafe fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PresentBuffer)(windows_core::Interface::as_raw(self), buffertopresent, syncinterval, flags) }
    }
    pub unsafe fn SetSourceRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourceRect)(windows_core::Interface::as_raw(self), prect).ok() }
    }
    pub unsafe fn SetTargetRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetRect)(windows_core::Interface::as_raw(self), prect).ok() }
    }
    pub unsafe fn SetDestSize(&self, width: u32, height: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDestSize)(windows_core::Interface::as_raw(self), width, height).ok() }
    }
    pub unsafe fn GetSourceRect(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTargetRect(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDestSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _).ok() }
    }
    pub unsafe fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColorSpace)(windows_core::Interface::as_raw(self), colorspace).ok() }
    }
    pub unsafe fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
        unsafe { (windows_core::Interface::vtable(self).GetColorSpace)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDecodeSwapChain_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PresentBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, DXGI_PRESENT) -> windows_core::HRESULT,
    pub SetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetDestSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetDestSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT,
    pub GetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS,
}
unsafe impl Send for IDXGIDecodeSwapChain {}
unsafe impl Sync for IDXGIDecodeSwapChain {}
pub trait IDXGIDecodeSwapChain_Impl: windows_core::IUnknownImpl {
    fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT;
    fn SetSourceRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetTargetRect(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetDestSize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn GetSourceRect(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetTargetRect(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()>;
    fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::Result<()>;
    fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS;
}
impl IDXGIDecodeSwapChain_Vtbl {
    pub const fn new<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PresentBuffer<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::PresentBuffer(this, core::mem::transmute_copy(&buffertopresent), core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags))
            }
        }
        unsafe extern "system" fn SetSourceRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetSourceRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn SetTargetRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetTargetRect(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn SetDestSize<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetDestSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetSourceRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDecodeSwapChain_Impl::GetSourceRect(this) {
                    Ok(ok__) => {
                        prect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetRect<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDecodeSwapChain_Impl::GetTargetRect(this) {
                    Ok(ok__) => {
                        prect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDestSize<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::GetDestSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn SetColorSpace<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
            }
        }
        unsafe extern "system" fn GetColorSpace<Identity: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDecodeSwapChain_Impl::GetColorSpace(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PresentBuffer: PresentBuffer::<Identity, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, OFFSET>,
            SetTargetRect: SetTargetRect::<Identity, OFFSET>,
            SetDestSize: SetDestSize::<Identity, OFFSET>,
            GetSourceRect: GetSourceRect::<Identity, OFFSET>,
            GetTargetRect: GetTargetRect::<Identity, OFFSET>,
            GetDestSize: GetDestSize::<Identity, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, OFFSET>,
            GetColorSpace: GetColorSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDecodeSwapChain as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDecodeSwapChain {}
windows_core::imp::define_interface!(IDXGIDevice, IDXGIDevice_Vtbl, 0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
impl core::ops::Deref for IDXGIDevice {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice, windows_core::IUnknown, IDXGIObject);
impl IDXGIDevice {
    pub unsafe fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [Option<IDXGISurface>]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), pdesc, ppsurface.len().try_into().unwrap(), usage, psharedresource.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppsurface.as_ptr())).ok() }
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const Option<windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryResourceResidency)(windows_core::Interface::as_raw(self), core::mem::transmute(ppresources), presidencystatus as _, numresources).ok() }
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGPUThreadPriority)(windows_core::Interface::as_raw(self), priority).ok() }
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPUThreadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_SURFACE_DESC, u32, DXGI_USAGE, *const DXGI_SHARED_RESOURCE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    pub QueryResourceResidency: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, *mut DXGI_RESIDENCY, u32) -> windows_core::HRESULT,
    pub SetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDevice {}
unsafe impl Sync for IDXGIDevice {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice_Impl: IDXGIObject_Impl {
    fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter>;
    fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut Option<IDXGISurface>) -> windows_core::Result<()>;
    fn QueryResourceResidency(&self, ppresources: *const Option<windows_core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice_Vtbl {
    pub const fn new<Identity: IDXGIDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdapter<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetAdapter(this) {
                    Ok(ok__) => {
                        padapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::CreateSurface(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&numsurfaces), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&psharedresource), core::mem::transmute_copy(&ppsurface)).into()
            }
        }
        unsafe extern "system" fn QueryResourceResidency<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::QueryResourceResidency(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presidencystatus), core::mem::transmute_copy(&numresources)).into()
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetGPUThreadPriority(this) {
                    Ok(ok__) => {
                        ppriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice {}
windows_core::imp::define_interface!(IDXGIDevice1, IDXGIDevice1_Vtbl, 0x77db970f_6276_48ba_ba28_070143b4392c);
impl core::ops::Deref for IDXGIDevice1 {
    type Target = IDXGIDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice1, windows_core::IUnknown, IDXGIObject, IDXGIDevice);
impl IDXGIDevice1 {
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency).ok() }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_Vtbl {
    pub base__: IDXGIDevice_Vtbl,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDevice1 {}
unsafe impl Sync for IDXGIDevice1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice1_Impl: IDXGIDevice_Impl {
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice1_Vtbl {
    pub const fn new<Identity: IDXGIDevice1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice1_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice1_Impl::GetMaximumFrameLatency(this) {
                    Ok(ok__) => {
                        pmaxlatency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDevice_Vtbl::new::<Identity, OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice1 {}
windows_core::imp::define_interface!(IDXGIDevice2, IDXGIDevice2_Vtbl, 0x05008617_fbfd_4051_a790_144884b4f6a9);
impl core::ops::Deref for IDXGIDevice2 {
    type Target = IDXGIDevice1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice2, windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1);
impl IDXGIDevice2 {
    pub unsafe fn OfferResources(&self, ppresources: &[Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OfferResources)(windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), core::mem::transmute(ppresources.as_ptr()), priority).ok() }
    }
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, pdiscarded: Option<*mut windows_core::BOOL>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReclaimResources)(windows_core::Interface::as_raw(self), numresources, core::mem::transmute(ppresources), pdiscarded.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn EnqueueSetEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnqueueSetEvent)(windows_core::Interface::as_raw(self), hevent).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice2_Vtbl {
    pub base__: IDXGIDevice1_Vtbl,
    pub OfferResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT,
    pub ReclaimResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub EnqueueSetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDevice2 {}
unsafe impl Sync for IDXGIDevice2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice2_Impl: IDXGIDevice1_Impl {
    fn OfferResources(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::Result<()>;
    fn ReclaimResources(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, pdiscarded: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn EnqueueSetEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice2_Vtbl {
    pub const fn new<Identity: IDXGIDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OfferResources<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::OfferResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn ReclaimResources<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, pdiscarded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::ReclaimResources(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&pdiscarded)).into()
            }
        }
        unsafe extern "system" fn EnqueueSetEvent<Identity: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice2_Impl::EnqueueSetEvent(this, core::mem::transmute_copy(&hevent)).into()
            }
        }
        Self {
            base__: IDXGIDevice1_Vtbl::new::<Identity, OFFSET>(),
            OfferResources: OfferResources::<Identity, OFFSET>,
            ReclaimResources: ReclaimResources::<Identity, OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice2 {}
windows_core::imp::define_interface!(IDXGIDevice3, IDXGIDevice3_Vtbl, 0x6007896c_3244_4afd_bf18_a6d3beda5023);
impl core::ops::Deref for IDXGIDevice3 {
    type Target = IDXGIDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice3, windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1, IDXGIDevice2);
impl IDXGIDevice3 {
    pub unsafe fn Trim(&self) {
        unsafe { (windows_core::Interface::vtable(self).Trim)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice3_Vtbl {
    pub base__: IDXGIDevice2_Vtbl,
    pub Trim: unsafe extern "system" fn(*mut core::ffi::c_void),
}
unsafe impl Send for IDXGIDevice3 {}
unsafe impl Sync for IDXGIDevice3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice3_Impl: IDXGIDevice2_Impl {
    fn Trim(&self);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice3_Vtbl {
    pub const fn new<Identity: IDXGIDevice3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Trim<Identity: IDXGIDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice3_Impl::Trim(this)
            }
        }
        Self { base__: IDXGIDevice2_Vtbl::new::<Identity, OFFSET>(), Trim: Trim::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice3 {}
windows_core::imp::define_interface!(IDXGIDevice4, IDXGIDevice4_Vtbl, 0x95b4f95f_d8da_4ca4_9ee6_3b76d5968a10);
impl core::ops::Deref for IDXGIDevice4 {
    type Target = IDXGIDevice3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice4, windows_core::IUnknown, IDXGIObject, IDXGIDevice, IDXGIDevice1, IDXGIDevice2, IDXGIDevice3);
impl IDXGIDevice4 {
    pub unsafe fn OfferResources1(&self, ppresources: &[Option<IDXGIResource>], priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: DXGI_OFFER_RESOURCE_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OfferResources1)(windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), core::mem::transmute(ppresources.as_ptr()), priority, flags.0 as _).ok() }
    }
    pub unsafe fn ReclaimResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReclaimResources1)(windows_core::Interface::as_raw(self), numresources, core::mem::transmute(ppresources), presults as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice4_Vtbl {
    pub base__: IDXGIDevice3_Vtbl,
    pub OfferResources1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, DXGI_OFFER_RESOURCE_PRIORITY, u32) -> windows_core::HRESULT,
    pub ReclaimResources1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDevice4 {}
unsafe impl Sync for IDXGIDevice4 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIDevice4_Impl: IDXGIDevice3_Impl {
    fn OfferResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: &DXGI_OFFER_RESOURCE_FLAGS) -> windows_core::Result<()>;
    fn ReclaimResources1(&self, numresources: u32, ppresources: *const Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIDevice4_Vtbl {
    pub const fn new<Identity: IDXGIDevice4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OfferResources1<Identity: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice4_Impl::OfferResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority), core::mem::transmute(&flags)).into()
            }
        }
        unsafe extern "system" fn ReclaimResources1<Identity: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice4_Impl::ReclaimResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presults)).into()
            }
        }
        Self {
            base__: IDXGIDevice3_Vtbl::new::<Identity, OFFSET>(),
            OfferResources1: OfferResources1::<Identity, OFFSET>,
            ReclaimResources1: ReclaimResources1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDevice as windows_core::Interface>::IID || iid == &<IDXGIDevice1 as windows_core::Interface>::IID || iid == &<IDXGIDevice2 as windows_core::Interface>::IID || iid == &<IDXGIDevice3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIDevice4 {}
windows_core::imp::define_interface!(IDXGIDeviceSubObject, IDXGIDeviceSubObject_Vtbl, 0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
impl core::ops::Deref for IDXGIDeviceSubObject {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDeviceSubObject, windows_core::IUnknown, IDXGIObject);
impl IDXGIDeviceSubObject {
    pub unsafe fn GetDevice<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDeviceSubObject {}
unsafe impl Sync for IDXGIDeviceSubObject {}
pub trait IDXGIDeviceSubObject_Impl: IDXGIObject_Impl {
    fn GetDevice(&self, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXGIDeviceSubObject_Vtbl {
    pub const fn new<Identity: IDXGIDeviceSubObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDXGIDeviceSubObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDeviceSubObject_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        Self { base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(), GetDevice: GetDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDeviceSubObject {}
windows_core::imp::define_interface!(IDXGIDisplayControl, IDXGIDisplayControl_Vtbl, 0xea9dbf1a_c88e_4486_854a_98aa0138f30c);
windows_core::imp::interface_hierarchy!(IDXGIDisplayControl, windows_core::IUnknown);
impl IDXGIDisplayControl {
    pub unsafe fn IsStereoEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsStereoEnabled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetStereoEnabled(&self, enabled: bool) {
        unsafe { (windows_core::Interface::vtable(self).SetStereoEnabled)(windows_core::Interface::as_raw(self), enabled.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDisplayControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
}
unsafe impl Send for IDXGIDisplayControl {}
unsafe impl Sync for IDXGIDisplayControl {}
pub trait IDXGIDisplayControl_Impl: windows_core::IUnknownImpl {
    fn IsStereoEnabled(&self) -> windows_core::BOOL;
    fn SetStereoEnabled(&self, enabled: windows_core::BOOL);
}
impl IDXGIDisplayControl_Vtbl {
    pub const fn new<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsStereoEnabled<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDisplayControl_Impl::IsStereoEnabled(this)
            }
        }
        unsafe extern "system" fn SetStereoEnabled<Identity: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDisplayControl_Impl::SetStereoEnabled(this, core::mem::transmute_copy(&enabled))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStereoEnabled: IsStereoEnabled::<Identity, OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDisplayControl {}
windows_core::imp::define_interface!(IDXGIFactory, IDXGIFactory_Vtbl, 0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
impl core::ops::Deref for IDXGIFactory {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory, windows_core::IUnknown, IDXGIObject);
impl IDXGIFactory {
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapters)(windows_core::Interface::as_raw(self), adapter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MakeWindowAssociation(&self, windowhandle: super::super::Foundation::HWND, flags: DXGI_MWA_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MakeWindowAssociation)(windows_core::Interface::as_raw(self), windowhandle, flags).ok() }
    }
    pub unsafe fn GetWindowAssociation(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindowAssociation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut Option<IDXGISwapChain>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateSwapChain)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pdesc, core::mem::transmute(ppswapchain)) }
    }
    pub unsafe fn CreateSoftwareAdapter(&self, module: super::super::Foundation::HMODULE) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSoftwareAdapter)(windows_core::Interface::as_raw(self), module, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MakeWindowAssociation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, DXGI_MWA_FLAGS) -> windows_core::HRESULT,
    pub GetWindowAssociation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChain: usize,
    pub CreateSoftwareAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HMODULE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIFactory {}
unsafe impl Sync for IDXGIFactory {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory_Impl: IDXGIObject_Impl {
    fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(&self, windowhandle: super::super::Foundation::HWND, flags: DXGI_MWA_FLAGS) -> windows_core::Result<()>;
    fn GetWindowAssociation(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn CreateSwapChain(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: windows_core::OutRef<IDXGISwapChain>) -> windows_core::HRESULT;
    fn CreateSoftwareAdapter(&self, module: super::super::Foundation::HMODULE) -> windows_core::Result<IDXGIAdapter>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory_Vtbl {
    pub const fn new<Identity: IDXGIFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapters<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::EnumAdapters(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: DXGI_MWA_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory_Impl::MakeWindowAssociation(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetWindowAssociation<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::GetWindowAssociation(this) {
                    Ok(ok__) => {
                        pwindowhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSwapChain<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory_Impl::CreateSwapChain(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppswapchain))
            }
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Identity: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, module: super::super::Foundation::HMODULE, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory_Impl::CreateSoftwareAdapter(this, core::mem::transmute_copy(&module)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapters: EnumAdapters::<Identity, OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Identity, OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Identity, OFFSET>,
            CreateSwapChain: CreateSwapChain::<Identity, OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory {}
windows_core::imp::define_interface!(IDXGIFactory1, IDXGIFactory1_Vtbl, 0x770aae78_f26f_4dba_a829_253c83d1b387);
impl core::ops::Deref for IDXGIFactory1 {
    type Target = IDXGIFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory1, windows_core::IUnknown, IDXGIObject, IDXGIFactory);
impl IDXGIFactory1 {
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapters1)(windows_core::Interface::as_raw(self), adapter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsCurrent(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsCurrent)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_Vtbl {
    pub base__: IDXGIFactory_Vtbl,
    pub EnumAdapters1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCurrent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
unsafe impl Send for IDXGIFactory1 {}
unsafe impl Sync for IDXGIFactory1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory1_Impl: IDXGIFactory_Impl {
    fn EnumAdapters1(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter1>;
    fn IsCurrent(&self) -> windows_core::BOOL;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory1_Vtbl {
    pub const fn new<Identity: IDXGIFactory1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapters1<Identity: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, ppadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory1_Impl::EnumAdapters1(this, core::mem::transmute_copy(&adapter)) {
                    Ok(ok__) => {
                        ppadapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCurrent<Identity: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory1_Impl::IsCurrent(this)
            }
        }
        Self { base__: IDXGIFactory_Vtbl::new::<Identity, OFFSET>(), EnumAdapters1: EnumAdapters1::<Identity, OFFSET>, IsCurrent: IsCurrent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory1 {}
windows_core::imp::define_interface!(IDXGIFactory2, IDXGIFactory2_Vtbl, 0x50c83a1c_e072_4c48_87b0_3630fa36a6d0);
impl core::ops::Deref for IDXGIFactory2 {
    type Target = IDXGIFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory2, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1);
impl IDXGIFactory2 {
    pub unsafe fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsWindowedStereoEnabled)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForHwnd<P0, P4>(&self, pdevice: P0, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P4) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForHwnd)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hwnd, pdesc, pfullscreendesc.unwrap_or(core::mem::zeroed()) as _, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P3>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P3) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForCoreWindow)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pwindow.param().abi(), pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSharedResourceAdapterLuid(&self, hresource: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::LUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharedResourceAdapterLuid)(windows_core::Interface::as_raw(self), hresource, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterStereoStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusWindow)(windows_core::Interface::as_raw(self), windowhandle, wmsg, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterStereoStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        unsafe { (windows_core::Interface::vtable(self).UnregisterStereoStatus)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    pub unsafe fn RegisterOcclusionStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusWindow)(windows_core::Interface::as_raw(self), windowhandle, wmsg, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterOcclusionStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        unsafe { (windows_core::Interface::vtable(self).UnregisterOcclusionStatus)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForComposition<P0, P2>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForComposition)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_Vtbl {
    pub base__: IDXGIFactory1_Vtbl,
    pub IsWindowedStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, *const DXGI_SWAP_CHAIN_DESC1, *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChainForHwnd: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChainForCoreWindow: usize,
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut super::super::Foundation::LUID) -> windows_core::HRESULT,
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, *mut u32) -> windows_core::HRESULT,
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
    pub UnregisterStereoStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, *mut u32) -> windows_core::HRESULT,
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChainForComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChainForComposition: usize,
}
unsafe impl Send for IDXGIFactory2 {}
unsafe impl Sync for IDXGIFactory2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory2_Impl: IDXGIFactory1_Impl {
    fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL;
    fn CreateSwapChainForHwnd(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn CreateSwapChainForCoreWindow(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pwindow: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn GetSharedResourceAdapterLuid(&self, hresource: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::LUID>;
    fn RegisterStereoStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> windows_core::Result<u32>;
    fn RegisterStereoStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterStereoStatus(&self, dwcookie: u32);
    fn RegisterOcclusionStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> windows_core::Result<u32>;
    fn RegisterOcclusionStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterOcclusionStatus(&self, dwcookie: u32);
    fn CreateSwapChainForComposition(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory2_Vtbl {
    pub const fn new<Identity: IDXGIFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsWindowedStereoEnabled<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::IsWindowedStereoEnabled(this)
            }
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForHwnd(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pfullscreendesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pwindow: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForCoreWindow(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pwindow), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::GetSharedResourceAdapterLuid(this, core::mem::transmute_copy(&hresource)) {
                    Ok(ok__) => {
                        pluid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterStereoStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterStereoStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::UnregisterStereoStatus(this, core::mem::transmute_copy(&dwcookie))
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterOcclusionStatusWindow(this, core::mem::transmute_copy(&windowhandle), core::mem::transmute_copy(&wmsg)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::RegisterOcclusionStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory2_Impl::UnregisterOcclusionStatus(this, core::mem::transmute_copy(&dwcookie))
            }
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Identity: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory2_Impl::CreateSwapChainForComposition(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIFactory1_Vtbl::new::<Identity, OFFSET>(),
            IsWindowedStereoEnabled: IsWindowedStereoEnabled::<Identity, OFFSET>,
            CreateSwapChainForHwnd: CreateSwapChainForHwnd::<Identity, OFFSET>,
            CreateSwapChainForCoreWindow: CreateSwapChainForCoreWindow::<Identity, OFFSET>,
            GetSharedResourceAdapterLuid: GetSharedResourceAdapterLuid::<Identity, OFFSET>,
            RegisterStereoStatusWindow: RegisterStereoStatusWindow::<Identity, OFFSET>,
            RegisterStereoStatusEvent: RegisterStereoStatusEvent::<Identity, OFFSET>,
            UnregisterStereoStatus: UnregisterStereoStatus::<Identity, OFFSET>,
            RegisterOcclusionStatusWindow: RegisterOcclusionStatusWindow::<Identity, OFFSET>,
            RegisterOcclusionStatusEvent: RegisterOcclusionStatusEvent::<Identity, OFFSET>,
            UnregisterOcclusionStatus: UnregisterOcclusionStatus::<Identity, OFFSET>,
            CreateSwapChainForComposition: CreateSwapChainForComposition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory2 {}
windows_core::imp::define_interface!(IDXGIFactory3, IDXGIFactory3_Vtbl, 0x25483823_cd46_4c7d_86ca_47aa95b837bd);
impl core::ops::Deref for IDXGIFactory3 {
    type Target = IDXGIFactory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory3, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2);
impl IDXGIFactory3 {
    pub unsafe fn GetCreationFlags(&self) -> DXGI_CREATE_FACTORY_FLAGS {
        unsafe { (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory3_Vtbl {
    pub base__: IDXGIFactory2_Vtbl,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> DXGI_CREATE_FACTORY_FLAGS,
}
unsafe impl Send for IDXGIFactory3 {}
unsafe impl Sync for IDXGIFactory3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory3_Impl: IDXGIFactory2_Impl {
    fn GetCreationFlags(&self) -> DXGI_CREATE_FACTORY_FLAGS;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory3_Vtbl {
    pub const fn new<Identity: IDXGIFactory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCreationFlags<Identity: IDXGIFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> DXGI_CREATE_FACTORY_FLAGS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory3_Impl::GetCreationFlags(this)
            }
        }
        Self { base__: IDXGIFactory2_Vtbl::new::<Identity, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory3 {}
windows_core::imp::define_interface!(IDXGIFactory4, IDXGIFactory4_Vtbl, 0x1bc6ea02_ef36_464f_bf0c_21ca39e5168a);
impl core::ops::Deref for IDXGIFactory4 {
    type Target = IDXGIFactory3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory4, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3);
impl IDXGIFactory4 {
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterByLuid)(windows_core::Interface::as_raw(self), core::mem::transmute(adapterluid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumWarpAdapter)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory4_Vtbl {
    pub base__: IDXGIFactory3_Vtbl,
    pub EnumAdapterByLuid: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::LUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumWarpAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIFactory4 {}
unsafe impl Sync for IDXGIFactory4 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory4_Impl: IDXGIFactory3_Impl {
    fn EnumAdapterByLuid(&self, adapterluid: &super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumWarpAdapter(&self, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory4_Vtbl {
    pub const fn new<Identity: IDXGIFactory4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapterByLuid<Identity: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory4_Impl::EnumAdapterByLuid(this, core::mem::transmute(&adapterluid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        unsafe extern "system" fn EnumWarpAdapter<Identity: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory4_Impl::EnumWarpAdapter(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        Self {
            base__: IDXGIFactory3_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapterByLuid: EnumAdapterByLuid::<Identity, OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory4 {}
windows_core::imp::define_interface!(IDXGIFactory5, IDXGIFactory5_Vtbl, 0x7632e1f5_ee65_4dca_87fd_84cd75f8838d);
impl core::ops::Deref for IDXGIFactory5 {
    type Target = IDXGIFactory4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory5, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4);
impl IDXGIFactory5 {
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, pfeaturesupportdata as _, featuresupportdatasize).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory5_Vtbl {
    pub base__: IDXGIFactory4_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_FEATURE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIFactory5 {}
unsafe impl Sync for IDXGIFactory5 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory5_Impl: IDXGIFactory4_Impl {
    fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory5_Vtbl {
    pub const fn new<Identity: IDXGIFactory5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckFeatureSupport<Identity: IDXGIFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory5_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
            }
        }
        Self { base__: IDXGIFactory4_Vtbl::new::<Identity, OFFSET>(), CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory5 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory5 {}
windows_core::imp::define_interface!(IDXGIFactory6, IDXGIFactory6_Vtbl, 0xc1b6694f_ff09_44a9_b03c_77900a0a1d17);
impl core::ops::Deref for IDXGIFactory6 {
    type Target = IDXGIFactory5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory6, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5);
impl IDXGIFactory6 {
    pub unsafe fn EnumAdapterByGpuPreference<T>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterByGpuPreference)(windows_core::Interface::as_raw(self), adapter, gpupreference, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory6_Vtbl {
    pub base__: IDXGIFactory5_Vtbl,
    pub EnumAdapterByGpuPreference: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_GPU_PREFERENCE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIFactory6 {}
unsafe impl Sync for IDXGIFactory6 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory6_Impl: IDXGIFactory5_Impl {
    fn EnumAdapterByGpuPreference(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory6_Vtbl {
    pub const fn new<Identity: IDXGIFactory6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Identity: IDXGIFactory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory6_Impl::EnumAdapterByGpuPreference(this, core::mem::transmute_copy(&adapter), core::mem::transmute_copy(&gpupreference), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        Self { base__: IDXGIFactory5_Vtbl::new::<Identity, OFFSET>(), EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory6 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIFactory5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory6 {}
windows_core::imp::define_interface!(IDXGIFactory7, IDXGIFactory7_Vtbl, 0xa4966eed_76db_44da_84c1_ee9a7afb20a8);
impl core::ops::Deref for IDXGIFactory7 {
    type Target = IDXGIFactory6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory7, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1, IDXGIFactory2, IDXGIFactory3, IDXGIFactory4, IDXGIFactory5, IDXGIFactory6);
impl IDXGIFactory7 {
    pub unsafe fn RegisterAdaptersChangedEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterAdaptersChangedEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterAdaptersChangedEvent)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory7_Vtbl {
    pub base__: IDXGIFactory6_Vtbl,
    pub RegisterAdaptersChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
    pub UnregisterAdaptersChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIFactory7 {}
unsafe impl Sync for IDXGIFactory7 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactory7_Impl: IDXGIFactory6_Impl {
    fn RegisterAdaptersChangedEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactory7_Vtbl {
    pub const fn new<Identity: IDXGIFactory7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Identity: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactory7_Impl::RegisterAdaptersChangedEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Identity: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory7_Impl::UnregisterAdaptersChangedEvent(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: IDXGIFactory6_Vtbl::new::<Identity, OFFSET>(),
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Identity, OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory7 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIFactory as windows_core::Interface>::IID || iid == &<IDXGIFactory1 as windows_core::Interface>::IID || iid == &<IDXGIFactory2 as windows_core::Interface>::IID || iid == &<IDXGIFactory3 as windows_core::Interface>::IID || iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<IDXGIFactory5 as windows_core::Interface>::IID || iid == &<IDXGIFactory6 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactory7 {}
windows_core::imp::define_interface!(IDXGIFactoryMedia, IDXGIFactoryMedia_Vtbl, 0x41e7d1f2_a591_4f7b_a2e5_fa9c843e1c12);
windows_core::imp::interface_hierarchy!(IDXGIFactoryMedia, windows_core::IUnknown);
impl IDXGIFactoryMedia {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSwapChainForCompositionSurfaceHandle<P0, P3>(&self, pdevice: P0, hsurface: Option<super::super::Foundation::HANDLE>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P3) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForCompositionSurfaceHandle)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hsurface.unwrap_or(core::mem::zeroed()) as _, pdesc, prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDecodeSwapChainForCompositionSurfaceHandle<P0, P3, P4>(&self, pdevice: P0, hsurface: Option<super::super::Foundation::HANDLE>, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: P3, prestricttooutput: P4) -> windows_core::Result<IDXGIDecodeSwapChain>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IDXGIResource>,
        P4: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDecodeSwapChainForCompositionSurfaceHandle)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hsurface.unwrap_or(core::mem::zeroed()) as _, pdesc, pyuvdecodebuffers.param().abi(), prestricttooutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactoryMedia_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HANDLE, *const DXGI_SWAP_CHAIN_DESC1, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSwapChainForCompositionSurfaceHandle: usize,
    pub CreateDecodeSwapChainForCompositionSurfaceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HANDLE, *const DXGI_DECODE_SWAP_CHAIN_DESC, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIFactoryMedia {}
unsafe impl Sync for IDXGIFactoryMedia {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIFactoryMedia_Impl: windows_core::IUnknownImpl {
    fn CreateSwapChainForCompositionSurfaceHandle(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGISwapChain1>;
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: windows_core::Ref<IDXGIResource>, prestricttooutput: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<IDXGIDecodeSwapChain>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIFactoryMedia_Vtbl {
    pub const fn new<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactoryMedia_Impl::CreateSwapChainForCompositionSurfaceHandle(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Identity: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut core::ffi::c_void, prestricttooutput: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIFactoryMedia_Impl::CreateDecodeSwapChainForCompositionSurfaceHandle(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hsurface), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pyuvdecodebuffers), core::mem::transmute_copy(&prestricttooutput)) {
                    Ok(ok__) => {
                        ppswapchain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Identity, OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactoryMedia as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIFactoryMedia {}
windows_core::imp::define_interface!(IDXGIInfoQueue, IDXGIInfoQueue_Vtbl, 0xd67441c7_672a_476f_9e82_cd55b44949ce);
windows_core::imp::interface_hierarchy!(IDXGIInfoQueue, windows_core::IUnknown);
impl IDXGIInfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, producer: windows_core::GUID, messagecountlimit: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), messagecountlimit).ok() }
    }
    pub unsafe fn ClearStoredMessages(&self, producer: windows_core::GUID) {
        unsafe { (windows_core::Interface::vtable(self).ClearStoredMessages)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetMessage(&self, producer: windows_core::GUID, messageindex: u64, pmessage: Option<*mut DXGI_INFO_QUEUE_MESSAGE>, pmessagebytelength: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), messageindex, pmessage.unwrap_or(core::mem::zeroed()) as _, pmessagebytelength as _).ok() }
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: windows_core::GUID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilters)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumStoredMessages(&self, producer: windows_core::GUID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumStoredMessages)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: windows_core::GUID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetMessageCountLimit(&self, producer: windows_core::GUID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self, producer: windows_core::GUID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self, producer: windows_core::GUID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddStorageFilterEntries(&self, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddStorageFilterEntries)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter).ok() }
    }
    pub unsafe fn GetStorageFilter(&self, producer: windows_core::GUID, pfilter: Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter.unwrap_or(core::mem::zeroed()) as _, pfilterbytelength as _).ok() }
    }
    pub unsafe fn ClearStorageFilter(&self, producer: windows_core::GUID) {
        unsafe { (windows_core::Interface::vtable(self).ClearStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushEmptyStorageFilter(&self, producer: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushEmptyStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)).ok() }
    }
    pub unsafe fn PushDenyAllStorageFilter(&self, producer: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushDenyAllStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)).ok() }
    }
    pub unsafe fn PushCopyOfStorageFilter(&self, producer: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushCopyOfStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)).ok() }
    }
    pub unsafe fn PushStorageFilter(&self, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter).ok() }
    }
    pub unsafe fn PopStorageFilter(&self, producer: windows_core::GUID) {
        unsafe { (windows_core::Interface::vtable(self).PopStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetStorageFilterStackSize(&self, producer: windows_core::GUID) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetStorageFilterStackSize)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRetrievalFilterEntries)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter).ok() }
    }
    pub unsafe fn GetRetrievalFilter(&self, producer: windows_core::GUID, pfilter: Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter.unwrap_or(core::mem::zeroed()) as _, pfilterbytelength as _).ok() }
    }
    pub unsafe fn ClearRetrievalFilter(&self, producer: windows_core::GUID) {
        unsafe { (windows_core::Interface::vtable(self).ClearRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self, producer: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushEmptyRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)).ok() }
    }
    pub unsafe fn PushDenyAllRetrievalFilter(&self, producer: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushDenyAllRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)).ok() }
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self, producer: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushCopyOfRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)).ok() }
    }
    pub unsafe fn PushRetrievalFilter(&self, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PushRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter).ok() }
    }
    pub unsafe fn PopRetrievalFilter(&self, producer: windows_core::GUID) {
        unsafe { (windows_core::Interface::vtable(self).PopRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self, producer: windows_core::GUID) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetRetrievalFilterStackSize)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddMessage<P4>(&self, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: P4) -> windows_core::Result<()>
    where
        P4: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category, severity, id, pdescription.param().abi()).ok() }
    }
    pub unsafe fn AddApplicationMessage<P1>(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddApplicationMessage)(windows_core::Interface::as_raw(self), severity, pdescription.param().abi()).ok() }
    }
    pub unsafe fn SetBreakOnCategory(&self, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnCategory)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category, benable.into()).ok() }
    }
    pub unsafe fn SetBreakOnSeverity(&self, producer: windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnSeverity)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), severity, benable.into()).ok() }
    }
    pub unsafe fn SetBreakOnID(&self, producer: windows_core::GUID, id: i32, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnID)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), id, benable.into()).ok() }
    }
    pub unsafe fn GetBreakOnCategory(&self, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnCategory)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category) }
    }
    pub unsafe fn GetBreakOnSeverity(&self, producer: windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnSeverity)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), severity) }
    }
    pub unsafe fn GetBreakOnID(&self, producer: windows_core::GUID, id: i32) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnID)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), id) }
    }
    pub unsafe fn SetMuteDebugOutput(&self, producer: windows_core::GUID, bmute: bool) {
        unsafe { (windows_core::Interface::vtable(self).SetMuteDebugOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), bmute.into()) }
    }
    pub unsafe fn GetMuteDebugOutput(&self, producer: windows_core::GUID) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMuteDebugOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIInfoQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u64) -> windows_core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID),
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u64, *mut DXGI_INFO_QUEUE_MESSAGE, *mut usize) -> windows_core::HRESULT,
    pub GetNumStoredMessagesAllowedByRetrievalFilters: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> u64,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut DXGI_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PushDenyAllStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut DXGI_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PushDenyAllRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> u32,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, i32, windows_core::PCSTR) -> windows_core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL,
    pub GetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32) -> windows_core::BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, windows_core::BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::BOOL,
}
unsafe impl Send for IDXGIInfoQueue {}
unsafe impl Sync for IDXGIInfoQueue {}
pub trait IDXGIInfoQueue_Impl: windows_core::IUnknownImpl {
    fn SetMessageCountLimit(&self, producer: &windows_core::GUID, messagecountlimit: u64) -> windows_core::Result<()>;
    fn ClearStoredMessages(&self, producer: &windows_core::GUID);
    fn GetMessage(&self, producer: &windows_core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::Result<()>;
    fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: &windows_core::GUID) -> u64;
    fn GetNumStoredMessages(&self, producer: &windows_core::GUID) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: &windows_core::GUID) -> u64;
    fn GetMessageCountLimit(&self, producer: &windows_core::GUID) -> u64;
    fn GetNumMessagesAllowedByStorageFilter(&self, producer: &windows_core::GUID) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self, producer: &windows_core::GUID) -> u64;
    fn AddStorageFilterEntries(&self, producer: &windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetStorageFilter(&self, producer: &windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearStorageFilter(&self, producer: &windows_core::GUID);
    fn PushEmptyStorageFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushDenyAllStorageFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushCopyOfStorageFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushStorageFilter(&self, producer: &windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopStorageFilter(&self, producer: &windows_core::GUID);
    fn GetStorageFilterStackSize(&self, producer: &windows_core::GUID) -> u32;
    fn AddRetrievalFilterEntries(&self, producer: &windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetRetrievalFilter(&self, producer: &windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearRetrievalFilter(&self, producer: &windows_core::GUID);
    fn PushEmptyRetrievalFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushDenyAllRetrievalFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self, producer: &windows_core::GUID) -> windows_core::Result<()>;
    fn PushRetrievalFilter(&self, producer: &windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopRetrievalFilter(&self, producer: &windows_core::GUID);
    fn GetRetrievalFilterStackSize(&self, producer: &windows_core::GUID) -> u32;
    fn AddMessage(&self, producer: &windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn AddApplicationMessage(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetBreakOnCategory(&self, producer: &windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnSeverity(&self, producer: &windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnID(&self, producer: &windows_core::GUID, id: i32, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetBreakOnCategory(&self, producer: &windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL;
    fn GetBreakOnSeverity(&self, producer: &windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL;
    fn GetBreakOnID(&self, producer: &windows_core::GUID, id: i32) -> windows_core::BOOL;
    fn SetMuteDebugOutput(&self, producer: &windows_core::GUID, bmute: windows_core::BOOL);
    fn GetMuteDebugOutput(&self, producer: &windows_core::GUID) -> windows_core::BOOL;
}
impl IDXGIInfoQueue_Vtbl {
    pub const fn new<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, messagecountlimit: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messagecountlimit)).into()
            }
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearStoredMessages(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
            }
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilters(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumStoredMessages(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMessageCountLimit(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn GetStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
            }
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearStorageFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushEmptyStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushDenyAllStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushCopyOfStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn PopStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PopStorageFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetStorageFilterStackSize(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
            }
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearRetrievalFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushEmptyRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushDenyAllRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushCopyOfRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PopRetrievalFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetRetrievalFilterStackSize(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
            }
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnID<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, id: i32, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category))
            }
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity))
            }
        }
        unsafe extern "system" fn GetBreakOnID<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, id: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id))
            }
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID, bmute: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute(&producer), core::mem::transmute_copy(&bmute))
            }
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: windows_core::GUID) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMuteDebugOutput(this, core::mem::transmute(&producer))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters: GetNumStoredMessagesAllowedByRetrievalFilters::<Identity, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, OFFSET>,
            PushDenyAllStorageFilter: PushDenyAllStorageFilter::<Identity, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, OFFSET>,
            PushDenyAllRetrievalFilter: PushDenyAllRetrievalFilter::<Identity, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, OFFSET>,
            AddMessage: AddMessage::<Identity, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIInfoQueue as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIInfoQueue {}
windows_core::imp::define_interface!(IDXGIKeyedMutex, IDXGIKeyedMutex_Vtbl, 0x9d8e1289_d7b3_465f_8126_250e349af85d);
impl core::ops::Deref for IDXGIKeyedMutex {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIKeyedMutex, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGIKeyedMutex {
    pub unsafe fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AcquireSync)(windows_core::Interface::as_raw(self), key, dwmilliseconds).ok() }
    }
    pub unsafe fn ReleaseSync(&self, key: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseSync)(windows_core::Interface::as_raw(self), key).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIKeyedMutex_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub AcquireSync: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32) -> windows_core::HRESULT,
    pub ReleaseSync: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIKeyedMutex {}
unsafe impl Sync for IDXGIKeyedMutex {}
pub trait IDXGIKeyedMutex_Impl: IDXGIDeviceSubObject_Impl {
    fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn ReleaseSync(&self, key: u64) -> windows_core::Result<()>;
}
impl IDXGIKeyedMutex_Vtbl {
    pub const fn new<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcquireSync<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64, dwmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIKeyedMutex_Impl::AcquireSync(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn ReleaseSync<Identity: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIKeyedMutex_Impl::ReleaseSync(this, core::mem::transmute_copy(&key)).into()
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            AcquireSync: AcquireSync::<Identity, OFFSET>,
            ReleaseSync: ReleaseSync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIKeyedMutex as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIKeyedMutex {}
windows_core::imp::define_interface!(IDXGIObject, IDXGIObject_Vtbl, 0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
windows_core::imp::interface_hierarchy!(IDXGIObject, windows_core::IUnknown);
impl IDXGIObject {
    pub unsafe fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), name, datasize, pdata).ok() }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, name: *const windows_core::GUID, punknown: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), name, punknown.param().abi()).ok() }
    }
    pub unsafe fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), name, pdatasize as _, pdata as _).ok() }
    }
    pub unsafe fn GetParent<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIObject {}
unsafe impl Sync for IDXGIObject {}
pub trait IDXGIObject_Impl: windows_core::IUnknownImpl {
    fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, name: *const windows_core::GUID, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParent(&self, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: IDXGIObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrivateData<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetParent<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetParent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIObject {}
windows_core::imp::define_interface!(IDXGIOutput, IDXGIOutput_Vtbl, 0xae02eedb_c735_4690_8d52_5a8dc20213aa);
impl core::ops::Deref for IDXGIOutput {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput, windows_core::IUnknown, IDXGIObject);
impl IDXGIOutput {
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self) -> windows_core::Result<DXGI_OUTPUT_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: Option<*mut Common::DXGI_MODE_DESC>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeList)(windows_core::Interface::as_raw(self), enumformat, flags, pnummodes as _, pdesc.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P2>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindClosestMatchingMode)(windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch as _, pconcerneddevice.param().abi()).ok() }
    }
    pub unsafe fn WaitForVBlank(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WaitForVBlank)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn TakeOwnership<P0>(&self, pdevice: P0, exclusive: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).TakeOwnership)(windows_core::Interface::as_raw(self), pdevice.param().abi(), exclusive.into()).ok() }
    }
    pub unsafe fn ReleaseOwnership(&self) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseOwnership)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGammaControlCapabilities)(windows_core::Interface::as_raw(self), pgammacaps as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGammaControl)(windows_core::Interface::as_raw(self), parray).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGammaControl)(windows_core::Interface::as_raw(self), parray as _).ok() }
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplaySurface)(windows_core::Interface::as_raw(self), pscanoutsurface.param().abi()).ok() }
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDisplaySurfaceData)(windows_core::Interface::as_raw(self), pdestination.param().abi()).ok() }
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), pstats as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayModeList: unsafe extern "system" fn(*mut core::ffi::c_void, Common::DXGI_FORMAT, DXGI_ENUM_MODES, *mut u32, *mut Common::DXGI_MODE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayModeList: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub FindClosestMatchingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::DXGI_MODE_DESC, *mut Common::DXGI_MODE_DESC, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    FindClosestMatchingMode: usize,
    pub WaitForVBlank: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ReleaseOwnership: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetGammaControlCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetGammaControlCapabilities: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetGammaControl: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetGammaControl: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetGammaControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetGammaControl: usize,
    pub SetDisplaySurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplaySurfaceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIOutput {}
unsafe impl Sync for IDXGIOutput {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput_Impl: IDXGIObject_Impl {
    fn GetDesc(&self) -> windows_core::Result<DXGI_OUTPUT_DESC>;
    fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn FindClosestMatchingMode(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn WaitForVBlank(&self) -> windows_core::Result<()>;
    fn TakeOwnership(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, exclusive: windows_core::BOOL) -> windows_core::Result<()>;
    fn ReleaseOwnership(&self);
    fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::Result<()>;
    fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> windows_core::Result<()>;
    fn SetDisplaySurface(&self, pscanoutsurface: windows_core::Ref<IDXGISurface>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData(&self, pdestination: windows_core::Ref<IDXGISurface>) -> windows_core::Result<()>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput_Vtbl {
    pub const fn new<Identity: IDXGIOutput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput_Impl::GetDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayModeList<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDisplayModeList(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::FindClosestMatchingMode(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), core::mem::transmute_copy(&pconcerneddevice)).into()
            }
        }
        unsafe extern "system" fn WaitForVBlank<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::WaitForVBlank(this).into()
            }
        }
        unsafe extern "system" fn TakeOwnership<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, exclusive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::TakeOwnership(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&exclusive)).into()
            }
        }
        unsafe extern "system" fn ReleaseOwnership<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::ReleaseOwnership(this)
            }
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetGammaControlCapabilities(this, core::mem::transmute_copy(&pgammacaps)).into()
            }
        }
        unsafe extern "system" fn SetGammaControl<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::SetGammaControl(this, core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn GetGammaControl<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetGammaControl(this, core::mem::transmute_copy(&parray)).into()
            }
        }
        unsafe extern "system" fn SetDisplaySurface<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscanoutsurface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::SetDisplaySurface(this, core::mem::transmute_copy(&pscanoutsurface)).into()
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetDisplaySurfaceData(this, core::mem::transmute_copy(&pdestination)).into()
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Identity, OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Identity, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Identity, OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Identity, OFFSET>,
            SetGammaControl: SetGammaControl::<Identity, OFFSET>,
            GetGammaControl: GetGammaControl::<Identity, OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Identity, OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput {}
windows_core::imp::define_interface!(IDXGIOutput1, IDXGIOutput1_Vtbl, 0x00cddea8_939b_4b83_a340_a685226666cc);
impl core::ops::Deref for IDXGIOutput1 {
    type Target = IDXGIOutput;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput1, windows_core::IUnknown, IDXGIObject, IDXGIOutput);
impl IDXGIOutput1 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: Option<*mut DXGI_MODE_DESC1>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayModeList1)(windows_core::Interface::as_raw(self), enumformat, flags, pnummodes as _, pdesc.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode1<P2>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindClosestMatchingMode1)(windows_core::Interface::as_raw(self), pmodetomatch, pclosestmatch as _, pconcerneddevice.param().abi()).ok() }
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDXGIResource>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDisplaySurfaceData1)(windows_core::Interface::as_raw(self), pdestination.param().abi()).ok() }
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> windows_core::Result<IDXGIOutputDuplication>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateOutput)(windows_core::Interface::as_raw(self), pdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput1_Vtbl {
    pub base__: IDXGIOutput_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayModeList1: unsafe extern "system" fn(*mut core::ffi::c_void, Common::DXGI_FORMAT, DXGI_ENUM_MODES, *mut u32, *mut DXGI_MODE_DESC1) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayModeList1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub FindClosestMatchingMode1: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MODE_DESC1, *mut DXGI_MODE_DESC1, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    FindClosestMatchingMode1: usize,
    pub GetDisplaySurfaceData1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DuplicateOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIOutput1 {}
unsafe impl Sync for IDXGIOutput1 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput1_Impl: IDXGIOutput_Impl {
    fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::Result<()>;
    fn FindClosestMatchingMode1(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDisplaySurfaceData1(&self, pdestination: windows_core::Ref<IDXGIResource>) -> windows_core::Result<()>;
    fn DuplicateOutput(&self, pdevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput1_Vtbl {
    pub const fn new<Identity: IDXGIOutput1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayModeList1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: DXGI_ENUM_MODES, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::GetDisplayModeList1(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pnummodes), core::mem::transmute_copy(&pdesc)).into()
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::FindClosestMatchingMode1(this, core::mem::transmute_copy(&pmodetomatch), core::mem::transmute_copy(&pclosestmatch), core::mem::transmute_copy(&pconcerneddevice)).into()
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput1_Impl::GetDisplaySurfaceData1(this, core::mem::transmute_copy(&pdestination)).into()
            }
        }
        unsafe extern "system" fn DuplicateOutput<Identity: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput1_Impl::DuplicateOutput(this, core::mem::transmute_copy(&pdevice)) {
                    Ok(ok__) => {
                        ppoutputduplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIOutput_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayModeList1: GetDisplayModeList1::<Identity, OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Identity, OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Identity, OFFSET>,
            DuplicateOutput: DuplicateOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput1 {}
windows_core::imp::define_interface!(IDXGIOutput2, IDXGIOutput2_Vtbl, 0x595e39d1_2724_4663_99b1_da969de28364);
impl core::ops::Deref for IDXGIOutput2 {
    type Target = IDXGIOutput1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput2, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1);
impl IDXGIOutput2 {
    pub unsafe fn SupportsOverlays(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).SupportsOverlays)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput2_Vtbl {
    pub base__: IDXGIOutput1_Vtbl,
    pub SupportsOverlays: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
unsafe impl Send for IDXGIOutput2 {}
unsafe impl Sync for IDXGIOutput2 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput2_Impl: IDXGIOutput1_Impl {
    fn SupportsOverlays(&self) -> windows_core::BOOL;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput2_Vtbl {
    pub const fn new<Identity: IDXGIOutput2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportsOverlays<Identity: IDXGIOutput2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutput2_Impl::SupportsOverlays(this)
            }
        }
        Self { base__: IDXGIOutput1_Vtbl::new::<Identity, OFFSET>(), SupportsOverlays: SupportsOverlays::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput2 {}
windows_core::imp::define_interface!(IDXGIOutput3, IDXGIOutput3_Vtbl, 0x8a6bb301_7e7e_41f4_a8e0_5b32f7f99b18);
impl core::ops::Deref for IDXGIOutput3 {
    type Target = IDXGIOutput2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput3, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2);
impl IDXGIOutput3 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P1>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckOverlaySupport)(windows_core::Interface::as_raw(self), enumformat, pconcerneddevice.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput3_Vtbl {
    pub base__: IDXGIOutput2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlaySupport: unsafe extern "system" fn(*mut core::ffi::c_void, Common::DXGI_FORMAT, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlaySupport: usize,
}
unsafe impl Send for IDXGIOutput3 {}
unsafe impl Sync for IDXGIOutput3 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput3_Impl: IDXGIOutput2_Impl {
    fn CheckOverlaySupport(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput3_Vtbl {
    pub const fn new<Identity: IDXGIOutput3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckOverlaySupport<Identity: IDXGIOutput3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput3_Impl::CheckOverlaySupport(this, core::mem::transmute_copy(&enumformat), core::mem::transmute_copy(&pconcerneddevice)) {
                    Ok(ok__) => {
                        pflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIOutput2_Vtbl::new::<Identity, OFFSET>(), CheckOverlaySupport: CheckOverlaySupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput3 {}
windows_core::imp::define_interface!(IDXGIOutput4, IDXGIOutput4_Vtbl, 0xdc7dca35_2196_414d_9f53_617884032a60);
impl core::ops::Deref for IDXGIOutput4 {
    type Target = IDXGIOutput3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput4, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3);
impl IDXGIOutput4 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P2>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P2) -> windows_core::Result<u32>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckOverlayColorSpaceSupport)(windows_core::Interface::as_raw(self), format, colorspace, pconcerneddevice.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput4_Vtbl {
    pub base__: IDXGIOutput3_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckOverlayColorSpaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, Common::DXGI_FORMAT, Common::DXGI_COLOR_SPACE_TYPE, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckOverlayColorSpaceSupport: usize,
}
unsafe impl Send for IDXGIOutput4 {}
unsafe impl Sync for IDXGIOutput4 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput4_Impl: IDXGIOutput3_Impl {
    fn CheckOverlayColorSpaceSupport(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput4_Vtbl {
    pub const fn new<Identity: IDXGIOutput4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Identity: IDXGIOutput4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput4_Impl::CheckOverlayColorSpaceSupport(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&colorspace), core::mem::transmute_copy(&pconcerneddevice)) {
                    Ok(ok__) => {
                        pflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIOutput3_Vtbl::new::<Identity, OFFSET>(), CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput4 {}
windows_core::imp::define_interface!(IDXGIOutput5, IDXGIOutput5_Vtbl, 0x80a07424_ab52_42eb_833c_0c42fd282d98);
impl core::ops::Deref for IDXGIOutput5 {
    type Target = IDXGIOutput4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput5, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3, IDXGIOutput4);
impl IDXGIOutput5 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DuplicateOutput1<P0>(&self, pdevice: P0, flags: u32, psupportedformats: &[Common::DXGI_FORMAT]) -> windows_core::Result<IDXGIOutputDuplication>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateOutput1)(windows_core::Interface::as_raw(self), pdevice.param().abi(), flags, psupportedformats.len().try_into().unwrap(), core::mem::transmute(psupportedformats.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput5_Vtbl {
    pub base__: IDXGIOutput4_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub DuplicateOutput1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const Common::DXGI_FORMAT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    DuplicateOutput1: usize,
}
unsafe impl Send for IDXGIOutput5 {}
unsafe impl Sync for IDXGIOutput5 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput5_Impl: IDXGIOutput4_Impl {
    fn DuplicateOutput1(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT) -> windows_core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput5_Vtbl {
    pub const fn new<Identity: IDXGIOutput5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DuplicateOutput1<Identity: IDXGIOutput5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput5_Impl::DuplicateOutput1(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&supportedformatscount), core::mem::transmute_copy(&psupportedformats)) {
                    Ok(ok__) => {
                        ppoutputduplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDXGIOutput4_Vtbl::new::<Identity, OFFSET>(), DuplicateOutput1: DuplicateOutput1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput5 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIOutput4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput5 {}
windows_core::imp::define_interface!(IDXGIOutput6, IDXGIOutput6_Vtbl, 0x068346e8_aaec_4b84_add7_137f513f77a1);
impl core::ops::Deref for IDXGIOutput6 {
    type Target = IDXGIOutput5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput6, windows_core::IUnknown, IDXGIObject, IDXGIOutput, IDXGIOutput1, IDXGIOutput2, IDXGIOutput3, IDXGIOutput4, IDXGIOutput5);
impl IDXGIOutput6 {
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc1(&self) -> windows_core::Result<DXGI_OUTPUT_DESC1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CheckHardwareCompositionSupport(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckHardwareCompositionSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput6_Vtbl {
    pub base__: IDXGIOutput5_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi")))]
    GetDesc1: usize,
    pub CheckHardwareCompositionSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIOutput6 {}
unsafe impl Sync for IDXGIOutput6 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput6_Impl: IDXGIOutput5_Impl {
    fn GetDesc1(&self) -> windows_core::Result<DXGI_OUTPUT_DESC1>;
    fn CheckHardwareCompositionSupport(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput6_Vtbl {
    pub const fn new<Identity: IDXGIOutput6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput6_Impl::GetDesc1(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Identity: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput6_Impl::CheckHardwareCompositionSupport(this) {
                    Ok(ok__) => {
                        pflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIOutput5_Vtbl::new::<Identity, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput6 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIOutput as windows_core::Interface>::IID || iid == &<IDXGIOutput1 as windows_core::Interface>::IID || iid == &<IDXGIOutput2 as windows_core::Interface>::IID || iid == &<IDXGIOutput3 as windows_core::Interface>::IID || iid == &<IDXGIOutput4 as windows_core::Interface>::IID || iid == &<IDXGIOutput5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGIOutput6 {}
windows_core::imp::define_interface!(IDXGIOutputDuplication, IDXGIOutputDuplication_Vtbl, 0x191cfac3_a341_470d_b26e_a864f428319c);
impl core::ops::Deref for IDXGIOutputDuplication {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutputDuplication, windows_core::IUnknown, IDXGIObject);
impl IDXGIOutputDuplication {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> DXGI_OUTDUPL_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut Option<IDXGIResource>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AcquireNextFrame)(windows_core::Interface::as_raw(self), timeoutinmilliseconds, pframeinfo as _, core::mem::transmute(ppdesktopresource)).ok() }
    }
    pub unsafe fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFrameDirtyRects)(windows_core::Interface::as_raw(self), dirtyrectsbuffersize, pdirtyrectsbuffer as _, pdirtyrectsbuffersizerequired as _).ok() }
    }
    pub unsafe fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFrameMoveRects)(windows_core::Interface::as_raw(self), moverectsbuffersize, pmoverectbuffer as _, pmoverectsbuffersizerequired as _).ok() }
    }
    pub unsafe fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFramePointerShape)(windows_core::Interface::as_raw(self), pointershapebuffersize, ppointershapebuffer as _, ppointershapebuffersizerequired as _, ppointershapeinfo as _).ok() }
    }
    pub unsafe fn MapDesktopSurface(&self) -> windows_core::Result<DXGI_MAPPED_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MapDesktopSurface)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnMapDesktopSurface(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnMapDesktopSurface)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ReleaseFrame(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseFrame)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutputDuplication_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_OUTDUPL_DESC),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub AcquireNextFrame: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXGI_OUTDUPL_FRAME_INFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameDirtyRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::RECT, *mut u32) -> windows_core::HRESULT,
    pub GetFrameMoveRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXGI_OUTDUPL_MOVE_RECT, *mut u32) -> windows_core::HRESULT,
    pub GetFramePointerShape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT,
    pub MapDesktopSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MAPPED_RECT) -> windows_core::HRESULT,
    pub UnMapDesktopSurface: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIOutputDuplication {}
unsafe impl Sync for IDXGIOutputDuplication {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGIOutputDuplication_Impl: IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC);
    fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: windows_core::OutRef<IDXGIResource>) -> windows_core::Result<()>;
    fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::Result<()>;
    fn MapDesktopSurface(&self) -> windows_core::Result<DXGI_MAPPED_RECT>;
    fn UnMapDesktopSurface(&self) -> windows_core::Result<()>;
    fn ReleaseFrame(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGIOutputDuplication_Vtbl {
    pub const fn new<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
            }
        }
        unsafe extern "system" fn AcquireNextFrame<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::AcquireNextFrame(this, core::mem::transmute_copy(&timeoutinmilliseconds), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&ppdesktopresource)).into()
            }
        }
        unsafe extern "system" fn GetFrameDirtyRects<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFrameDirtyRects(this, core::mem::transmute_copy(&dirtyrectsbuffersize), core::mem::transmute_copy(&pdirtyrectsbuffer), core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into()
            }
        }
        unsafe extern "system" fn GetFrameMoveRects<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFrameMoveRects(this, core::mem::transmute_copy(&moverectsbuffersize), core::mem::transmute_copy(&pmoverectbuffer), core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into()
            }
        }
        unsafe extern "system" fn GetFramePointerShape<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::GetFramePointerShape(this, core::mem::transmute_copy(&pointershapebuffersize), core::mem::transmute_copy(&ppointershapebuffer), core::mem::transmute_copy(&ppointershapebuffersizerequired), core::mem::transmute_copy(&ppointershapeinfo)).into()
            }
        }
        unsafe extern "system" fn MapDesktopSurface<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutputDuplication_Impl::MapDesktopSurface(this) {
                    Ok(ok__) => {
                        plockedrect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::UnMapDesktopSurface(this).into()
            }
        }
        unsafe extern "system" fn ReleaseFrame<Identity: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIOutputDuplication_Impl::ReleaseFrame(this).into()
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            AcquireNextFrame: AcquireNextFrame::<Identity, OFFSET>,
            GetFrameDirtyRects: GetFrameDirtyRects::<Identity, OFFSET>,
            GetFrameMoveRects: GetFrameMoveRects::<Identity, OFFSET>,
            GetFramePointerShape: GetFramePointerShape::<Identity, OFFSET>,
            MapDesktopSurface: MapDesktopSurface::<Identity, OFFSET>,
            UnMapDesktopSurface: UnMapDesktopSurface::<Identity, OFFSET>,
            ReleaseFrame: ReleaseFrame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutputDuplication as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGIOutputDuplication {}
windows_core::imp::define_interface!(IDXGIResource, IDXGIResource_Vtbl, 0x035f3ab4_482e_4e50_b41f_8a7f8bd8960b);
impl core::ops::Deref for IDXGIResource {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIResource, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGIResource {
    pub unsafe fn GetSharedHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharedHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUsage(&self) -> windows_core::Result<DXGI_USAGE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUsage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEvictionPriority)(windows_core::Interface::as_raw(self), evictionpriority).ok() }
    }
    pub unsafe fn GetEvictionPriority(&self) -> windows_core::Result<DXGI_RESOURCE_PRIORITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEvictionPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub GetSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_USAGE) -> windows_core::HRESULT,
    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_RESOURCE_PRIORITY) -> windows_core::HRESULT,
    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_RESOURCE_PRIORITY) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIResource {}
unsafe impl Sync for IDXGIResource {}
pub trait IDXGIResource_Impl: IDXGIDeviceSubObject_Impl {
    fn GetSharedHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetUsage(&self) -> windows_core::Result<DXGI_USAGE>;
    fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> windows_core::Result<()>;
    fn GetEvictionPriority(&self) -> windows_core::Result<DXGI_RESOURCE_PRIORITY>;
}
impl IDXGIResource_Vtbl {
    pub const fn new<Identity: IDXGIResource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSharedHandle<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetSharedHandle(this) {
                    Ok(ok__) => {
                        psharedhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUsage<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusage: *mut DXGI_USAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetUsage(this) {
                    Ok(ok__) => {
                        pusage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIResource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority)).into()
            }
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: IDXGIResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevictionpriority: *mut DXGI_RESOURCE_PRIORITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource_Impl::GetEvictionPriority(this) {
                    Ok(ok__) => {
                        pevictionpriority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            GetSharedHandle: GetSharedHandle::<Identity, OFFSET>,
            GetUsage: GetUsage::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIResource {}
windows_core::imp::define_interface!(IDXGIResource1, IDXGIResource1_Vtbl, 0x30961379_4609_4a41_998e_54fe567ee0c1);
impl core::ops::Deref for IDXGIResource1 {
    type Target = IDXGIResource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIResource1, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGIResource);
impl IDXGIResource1 {
    pub unsafe fn CreateSubresourceSurface(&self, index: u32) -> windows_core::Result<IDXGISurface2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSubresourceSurface)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Security")]
    pub unsafe fn CreateSharedHandle<P2>(&self, pattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwaccess: u32, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSharedHandle)(windows_core::Interface::as_raw(self), pattributes.unwrap_or(core::mem::zeroed()) as _, dwaccess, lpname.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIResource1_Vtbl {
    pub base__: IDXGIResource_Vtbl,
    pub CreateSubresourceSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Security")]
    pub CreateSharedHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Security::SECURITY_ATTRIBUTES, u32, windows_core::PCWSTR, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Security"))]
    CreateSharedHandle: usize,
}
unsafe impl Send for IDXGIResource1 {}
unsafe impl Sync for IDXGIResource1 {}
#[cfg(feature = "Win32_Security")]
pub trait IDXGIResource1_Impl: IDXGIResource_Impl {
    fn CreateSubresourceSurface(&self, index: u32) -> windows_core::Result<IDXGISurface2>;
    fn CreateSharedHandle(&self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Security")]
impl IDXGIResource1_Vtbl {
    pub const fn new<Identity: IDXGIResource1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSubresourceSurface<Identity: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource1_Impl::CreateSubresourceSurface(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIResource1_Impl::CreateSharedHandle(this, core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&dwaccess), core::mem::transmute(&lpname)) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIResource_Vtbl::new::<Identity, OFFSET>(),
            CreateSubresourceSurface: CreateSubresourceSurface::<Identity, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIResource1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIResource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for IDXGIResource1 {}
windows_core::imp::define_interface!(IDXGISurface, IDXGISurface_Vtbl, 0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
impl core::ops::Deref for IDXGISurface {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISurface {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> windows_core::Result<DXGI_SURFACE_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: DXGI_MAP_FLAGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Map)(windows_core::Interface::as_raw(self), plockedrect as _, mapflags).ok() }
    }
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    pub Map: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MAPPED_RECT, DXGI_MAP_FLAGS) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISurface {}
unsafe impl Sync for IDXGISurface {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISurface_Impl: IDXGIDeviceSubObject_Impl {
    fn GetDesc(&self) -> windows_core::Result<DXGI_SURFACE_DESC>;
    fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: DXGI_MAP_FLAGS) -> windows_core::Result<()>;
    fn Unmap(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISurface_Vtbl {
    pub const fn new<Identity: IDXGISurface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISurface_Impl::GetDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Map<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: DXGI_MAP_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface_Impl::Map(this, core::mem::transmute_copy(&plockedrect), core::mem::transmute_copy(&mapflags)).into()
            }
        }
        unsafe extern "system" fn Unmap<Identity: IDXGISurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface_Impl::Unmap(this).into()
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISurface {}
windows_core::imp::define_interface!(IDXGISurface1, IDXGISurface1_Vtbl, 0x4ae63092_6327_4c1b_80ae_bfe12ea32b86);
impl core::ops::Deref for IDXGISurface1 {
    type Target = IDXGISurface;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface1, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface);
impl IDXGISurface1 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, discard: bool) -> windows_core::Result<super::Gdi::HDC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), discard.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReleaseDC(&self, pdirtyrect: Option<*const super::super::Foundation::RECT>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), pdirtyrect.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface1_Vtbl {
    pub base__: IDXGISurface_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut super::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISurface1 {}
unsafe impl Sync for IDXGISurface1 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface1_Impl: IDXGISurface_Impl {
    fn GetDC(&self, discard: windows_core::BOOL) -> windows_core::Result<super::Gdi::HDC>;
    fn ReleaseDC(&self, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface1_Vtbl {
    pub const fn new<Identity: IDXGISurface1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDC<Identity: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discard: windows_core::BOOL, phdc: *mut super::Gdi::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISurface1_Impl::GetDC(this, core::mem::transmute_copy(&discard)) {
                    Ok(ok__) => {
                        phdc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface1_Impl::ReleaseDC(this, core::mem::transmute_copy(&pdirtyrect)).into()
            }
        }
        Self { base__: IDXGISurface_Vtbl::new::<Identity, OFFSET>(), GetDC: GetDC::<Identity, OFFSET>, ReleaseDC: ReleaseDC::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISurface as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGISurface1 {}
windows_core::imp::define_interface!(IDXGISurface2, IDXGISurface2_Vtbl, 0xaba496dd_b617_4cb8_a866_bc44d7eb1fa2);
impl core::ops::Deref for IDXGISurface2 {
    type Target = IDXGISurface1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface2, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISurface, IDXGISurface1);
impl IDXGISurface2 {
    pub unsafe fn GetResource<T>(&self, psubresourceindex: *mut u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), &T::IID, &mut result__, psubresourceindex as _).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface2_Vtbl {
    pub base__: IDXGISurface1_Vtbl,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISurface2 {}
unsafe impl Sync for IDXGISurface2 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface2_Impl: IDXGISurface1_Impl {
    fn GetResource(&self, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface2_Vtbl {
    pub const fn new<Identity: IDXGISurface2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource<Identity: IDXGISurface2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparentresource: *mut *mut core::ffi::c_void, psubresourceindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISurface2_Impl::GetResource(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparentresource), core::mem::transmute_copy(&psubresourceindex)).into()
            }
        }
        Self { base__: IDXGISurface1_Vtbl::new::<Identity, OFFSET>(), GetResource: GetResource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISurface2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISurface as windows_core::Interface>::IID || iid == &<IDXGISurface1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl windows_core::RuntimeName for IDXGISurface2 {}
windows_core::imp::define_interface!(IDXGISwapChain, IDXGISwapChain_Vtbl, 0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
impl core::ops::Deref for IDXGISwapChain {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISwapChain {
    pub unsafe fn Present(&self, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self), syncinterval, flags) }
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn SetFullscreenState<P1>(&self, fullscreen: bool, ptarget: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDXGIOutput>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFullscreenState)(windows_core::Interface::as_raw(self), fullscreen.into(), ptarget.param().abi()).ok() }
    }
    pub unsafe fn GetFullscreenState(&self, pfullscreen: Option<*mut windows_core::BOOL>, pptarget: Option<*mut Option<IDXGIOutput>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFullscreenState)(windows_core::Interface::as_raw(self), pfullscreen.unwrap_or(core::mem::zeroed()) as _, pptarget.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: DXGI_SWAP_CHAIN_FLAG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResizeBuffers)(windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags.0 as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResizeTarget)(windows_core::Interface::as_raw(self), pnewtargetparameters).ok() }
    }
    pub unsafe fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainingOutput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatistics)(windows_core::Interface::as_raw(self), pstats as _).ok() }
    }
    pub unsafe fn GetLastPresentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPresentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_PRESENT) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFullscreenState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFullscreenState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, Common::DXGI_FORMAT, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::DXGI_MODE_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeTarget: usize,
    pub GetContainingOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT,
    pub GetLastPresentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChain {}
unsafe impl Sync for IDXGISwapChain {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain_Impl: IDXGIDeviceSubObject_Impl {
    fn Present(&self, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT;
    fn GetBuffer(&self, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetFullscreenState(&self, fullscreen: windows_core::BOOL, ptarget: windows_core::Ref<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetFullscreenState(&self, pfullscreen: *mut windows_core::BOOL, pptarget: windows_core::OutRef<IDXGIOutput>) -> windows_core::Result<()>;
    fn GetDesc(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC>;
    fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: &DXGI_SWAP_CHAIN_FLAG) -> windows_core::Result<()>;
    fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> windows_core::Result<()>;
    fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::Result<()>;
    fn GetLastPresentCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain_Vtbl {
    pub const fn new<Identity: IDXGISwapChain_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Present<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::Present(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&flags))
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: u32, riid: *const windows_core::GUID, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsurface)).into()
            }
        }
        unsafe extern "system" fn SetFullscreenState<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullscreen: windows_core::BOOL, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::SetFullscreenState(this, core::mem::transmute_copy(&fullscreen), core::mem::transmute_copy(&ptarget)).into()
            }
        }
        unsafe extern "system" fn GetFullscreenState<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfullscreen: *mut windows_core::BOOL, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetFullscreenState(this, core::mem::transmute_copy(&pfullscreen), core::mem::transmute_copy(&pptarget)).into()
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain_Impl::GetDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResizeBuffers<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::ResizeBuffers(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&newformat), core::mem::transmute(&swapchainflags)).into()
            }
        }
        unsafe extern "system" fn ResizeTarget<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::ResizeTarget(this, core::mem::transmute_copy(&pnewtargetparameters)).into()
            }
        }
        unsafe extern "system" fn GetContainingOutput<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain_Impl::GetContainingOutput(this) {
                    Ok(ok__) => {
                        ppoutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain_Impl::GetFrameStatistics(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastpresentcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain_Impl::GetLastPresentCount(this) {
                    Ok(ok__) => {
                        plastpresentcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SetFullscreenState: SetFullscreenState::<Identity, OFFSET>,
            GetFullscreenState: GetFullscreenState::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            ResizeBuffers: ResizeBuffers::<Identity, OFFSET>,
            ResizeTarget: ResizeTarget::<Identity, OFFSET>,
            GetContainingOutput: GetContainingOutput::<Identity, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain {}
windows_core::imp::define_interface!(IDXGISwapChain1, IDXGISwapChain1_Vtbl, 0x790a45f7_0d42_4876_983a_0a55cfe6f4aa);
impl core::ops::Deref for IDXGISwapChain1 {
    type Target = IDXGISwapChain;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain1, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain);
impl IDXGISwapChain1 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFullscreenDesc(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFullscreenDesc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHwnd(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCoreWindow)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: DXGI_PRESENT, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Present1)(windows_core::Interface::as_raw(self), syncinterval, presentflags, ppresentparameters) }
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsTemporaryMonoSupported)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRestrictToOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictToOutput)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackgroundColor)(windows_core::Interface::as_raw(self), pcolor).ok() }
    }
    pub unsafe fn GetBackgroundColor(&self) -> windows_core::Result<DXGI_RGBA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackgroundColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), rotation).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> windows_core::Result<Common::DXGI_MODE_ROTATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_Vtbl {
    pub base__: IDXGISwapChain_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFullscreenDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFullscreenDesc: usize,
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Present1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_PRESENT, *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT,
    pub IsTemporaryMonoSupported: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetRestrictToOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_RGBA) -> windows_core::HRESULT,
    pub GetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_RGBA) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetRotation: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRotation: usize,
}
unsafe impl Send for IDXGISwapChain1 {}
unsafe impl Sync for IDXGISwapChain1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain1_Impl: IDXGISwapChain_Impl {
    fn GetDesc1(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC1>;
    fn GetFullscreenDesc(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>;
    fn GetHwnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn GetCoreWindow(&self, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Present1(&self, syncinterval: u32, presentflags: DXGI_PRESENT, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT;
    fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL;
    fn GetRestrictToOutput(&self) -> windows_core::Result<IDXGIOutput>;
    fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> windows_core::Result<()>;
    fn GetBackgroundColor(&self) -> windows_core::Result<DXGI_RGBA>;
    fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> windows_core::Result<()>;
    fn GetRotation(&self) -> windows_core::Result<Common::DXGI_MODE_ROTATION>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain1_Vtbl {
    pub const fn new<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc1<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetDesc1(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFullscreenDesc<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetFullscreenDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHwnd<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetHwnd(this) {
                    Ok(ok__) => {
                        phwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCoreWindow<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refiid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::GetCoreWindow(this, core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&ppunk)).into()
            }
        }
        unsafe extern "system" fn Present1<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncinterval: u32, presentflags: DXGI_PRESENT, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::Present1(this, core::mem::transmute_copy(&syncinterval), core::mem::transmute_copy(&presentflags), core::mem::transmute_copy(&ppresentparameters))
            }
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::IsTemporaryMonoSupported(this)
            }
        }
        unsafe extern "system" fn GetRestrictToOutput<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprestricttooutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetRestrictToOutput(this) {
                    Ok(ok__) => {
                        pprestricttooutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *const DXGI_RGBA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::SetBackgroundColor(this, core::mem::transmute_copy(&pcolor)).into()
            }
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetBackgroundColor(this) {
                    Ok(ok__) => {
                        pcolor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain1_Impl::SetRotation(this, core::mem::transmute_copy(&rotation)).into()
            }
        }
        unsafe extern "system" fn GetRotation<Identity: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain1_Impl::GetRotation(this) {
                    Ok(ok__) => {
                        protation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGISwapChain_Vtbl::new::<Identity, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, OFFSET>,
            GetFullscreenDesc: GetFullscreenDesc::<Identity, OFFSET>,
            GetHwnd: GetHwnd::<Identity, OFFSET>,
            GetCoreWindow: GetCoreWindow::<Identity, OFFSET>,
            Present1: Present1::<Identity, OFFSET>,
            IsTemporaryMonoSupported: IsTemporaryMonoSupported::<Identity, OFFSET>,
            GetRestrictToOutput: GetRestrictToOutput::<Identity, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            GetRotation: GetRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain1 {}
windows_core::imp::define_interface!(IDXGISwapChain2, IDXGISwapChain2_Vtbl, 0xa8be2ac4_199f_4946_b331_79599fb98de7);
impl core::ops::Deref for IDXGISwapChain2 {
    type Target = IDXGISwapChain1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain2, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1);
impl IDXGISwapChain2 {
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourceSize)(windows_core::Interface::as_raw(self), width, height).ok() }
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSourceSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _).ok() }
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency).ok() }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).GetFrameLatencyWaitableObject)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixTransform)(windows_core::Interface::as_raw(self), pmatrix).ok() }
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixTransform)(windows_core::Interface::as_raw(self), pmatrix as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_Vtbl {
    pub base__: IDXGISwapChain1_Vtbl,
    pub SetSourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub SetMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChain2 {}
unsafe impl Sync for IDXGISwapChain2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain2_Impl: IDXGISwapChain1_Impl {
    fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
    fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE;
    fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::Result<()>;
    fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain2_Vtbl {
    pub const fn new<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSourceSize<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetSourceSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn GetSourceSize<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetSourceSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight)).into()
            }
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetMaximumFrameLatency(this, core::mem::transmute_copy(&maxlatency)).into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlatency: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain2_Impl::GetMaximumFrameLatency(this) {
                    Ok(ok__) => {
                        pmaxlatency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetFrameLatencyWaitableObject(this)
            }
        }
        unsafe extern "system" fn SetMatrixTransform<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::SetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetMatrixTransform<Identity: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain2_Impl::GetMatrixTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        Self {
            base__: IDXGISwapChain1_Vtbl::new::<Identity, OFFSET>(),
            SetSourceSize: SetSourceSize::<Identity, OFFSET>,
            GetSourceSize: GetSourceSize::<Identity, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
            GetFrameLatencyWaitableObject: GetFrameLatencyWaitableObject::<Identity, OFFSET>,
            SetMatrixTransform: SetMatrixTransform::<Identity, OFFSET>,
            GetMatrixTransform: GetMatrixTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain2 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain2 {}
windows_core::imp::define_interface!(IDXGISwapChain3, IDXGISwapChain3_Vtbl, 0x94d99bdb_f1f8_4ab0_b236_7da0170edab1);
impl core::ops::Deref for IDXGISwapChain3 {
    type Target = IDXGISwapChain2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain3, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2);
impl IDXGISwapChain3 {
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentBackBufferIndex)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckColorSpaceSupport)(windows_core::Interface::as_raw(self), colorspace, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColorSpace1)(windows_core::Interface::as_raw(self), colorspace).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: DXGI_SWAP_CHAIN_FLAG, pcreationnodemask: *const u32, pppresentqueue: *const Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResizeBuffers1)(windows_core::Interface::as_raw(self), buffercount, width, height, format, swapchainflags.0 as _, pcreationnodemask, core::mem::transmute(pppresentqueue)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain3_Vtbl {
    pub base__: IDXGISwapChain2_Vtbl,
    pub GetCurrentBackBufferIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckColorSpaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, Common::DXGI_COLOR_SPACE_TYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckColorSpaceSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace1: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, Common::DXGI_FORMAT, u32, *const u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers1: usize,
}
unsafe impl Send for IDXGISwapChain3 {}
unsafe impl Sync for IDXGISwapChain3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain3_Impl: IDXGISwapChain2_Impl {
    fn GetCurrentBackBufferIndex(&self) -> u32;
    fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<u32>;
    fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()>;
    fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: &DXGI_SWAP_CHAIN_FLAG, pcreationnodemask: *const u32, pppresentqueue: *const Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain3_Vtbl {
    pub const fn new<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::GetCurrentBackBufferIndex(this)
            }
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain3_Impl::CheckColorSpaceSupport(this, core::mem::transmute_copy(&colorspace)) {
                    Ok(ok__) => {
                        pcolorspacesupport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColorSpace1<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::SetColorSpace1(this, core::mem::transmute_copy(&colorspace)).into()
            }
        }
        unsafe extern "system" fn ResizeBuffers1<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::ResizeBuffers1(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute(&swapchainflags), core::mem::transmute_copy(&pcreationnodemask), core::mem::transmute_copy(&pppresentqueue)).into()
            }
        }
        Self {
            base__: IDXGISwapChain2_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Identity, OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Identity, OFFSET>,
            SetColorSpace1: SetColorSpace1::<Identity, OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain3 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGISwapChain2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain3 {}
windows_core::imp::define_interface!(IDXGISwapChain4, IDXGISwapChain4_Vtbl, 0x3d585d5a_bd4a_489e_b1f4_3dbcb6452ffb);
impl core::ops::Deref for IDXGISwapChain4 {
    type Target = IDXGISwapChain3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain4, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1, IDXGISwapChain2, IDXGISwapChain3);
impl IDXGISwapChain4 {
    pub unsafe fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, pmetadata: Option<&[u8]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHDRMetaData)(windows_core::Interface::as_raw(self), r#type, pmetadata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pmetadata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain4_Vtbl {
    pub base__: IDXGISwapChain3_Vtbl,
    pub SetHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_HDR_METADATA_TYPE, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChain4 {}
unsafe impl Sync for IDXGISwapChain4 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISwapChain4_Impl: IDXGISwapChain3_Impl {
    fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISwapChain4_Vtbl {
    pub const fn new<Identity: IDXGISwapChain4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHDRMetaData<Identity: IDXGISwapChain4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain4_Impl::SetHDRMetaData(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        Self { base__: IDXGISwapChain3_Vtbl::new::<Identity, OFFSET>(), SetHDRMetaData: SetHDRMetaData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain4 as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID || iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGISwapChain as windows_core::Interface>::IID || iid == &<IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<IDXGISwapChain2 as windows_core::Interface>::IID || iid == &<IDXGISwapChain3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain4 {}
windows_core::imp::define_interface!(IDXGISwapChainMedia, IDXGISwapChainMedia_Vtbl, 0xdd95b90b_f05f_4f6a_bd65_25bfb264bd84);
windows_core::imp::interface_hierarchy!(IDXGISwapChainMedia, windows_core::IUnknown);
impl IDXGISwapChainMedia {
    pub unsafe fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFrameStatisticsMedia)(windows_core::Interface::as_raw(self), pstats as _).ok() }
    }
    pub unsafe fn SetPresentDuration(&self, duration: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPresentDuration)(windows_core::Interface::as_raw(self), duration).ok() }
    }
    pub unsafe fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckPresentDurationSupport)(windows_core::Interface::as_raw(self), desiredpresentduration, pclosestsmallerpresentduration as _, pclosestlargerpresentduration as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChainMedia_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrameStatisticsMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT,
    pub SetPresentDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CheckPresentDurationSupport: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChainMedia {}
unsafe impl Sync for IDXGISwapChainMedia {}
pub trait IDXGISwapChainMedia_Impl: windows_core::IUnknownImpl {
    fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::Result<()>;
    fn SetPresentDuration(&self, duration: u32) -> windows_core::Result<()>;
    fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::Result<()>;
}
impl IDXGISwapChainMedia_Vtbl {
    pub const fn new<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrameStatisticsMedia<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::GetFrameStatisticsMedia(this, core::mem::transmute_copy(&pstats)).into()
            }
        }
        unsafe extern "system" fn SetPresentDuration<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::SetPresentDuration(this, core::mem::transmute_copy(&duration)).into()
            }
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Identity: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChainMedia_Impl::CheckPresentDurationSupport(this, core::mem::transmute_copy(&desiredpresentduration), core::mem::transmute_copy(&pclosestsmallerpresentduration), core::mem::transmute_copy(&pclosestlargerpresentduration)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Identity, OFFSET>,
            SetPresentDuration: SetPresentDuration::<Identity, OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChainMedia as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGISwapChainMedia {}
windows_core::imp::define_interface!(IDXGraphicsAnalysis, IDXGraphicsAnalysis_Vtbl, 0x9f251514_9d4d_4902_9d60_18988ab7d4b5);
windows_core::imp::interface_hierarchy!(IDXGraphicsAnalysis, windows_core::IUnknown);
impl IDXGraphicsAnalysis {
    pub unsafe fn BeginCapture(&self) {
        unsafe { (windows_core::Interface::vtable(self).BeginCapture)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndCapture(&self) {
        unsafe { (windows_core::Interface::vtable(self).EndCapture)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGraphicsAnalysis_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginCapture: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EndCapture: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IDXGraphicsAnalysis_Impl: windows_core::IUnknownImpl {
    fn BeginCapture(&self);
    fn EndCapture(&self);
}
impl IDXGraphicsAnalysis_Vtbl {
    pub const fn new<Identity: IDXGraphicsAnalysis_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginCapture<Identity: IDXGraphicsAnalysis_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGraphicsAnalysis_Impl::BeginCapture(this)
            }
        }
        unsafe extern "system" fn EndCapture<Identity: IDXGraphicsAnalysis_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGraphicsAnalysis_Impl::EndCapture(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCapture: BeginCapture::<Identity, OFFSET>,
            EndCapture: EndCapture::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGraphicsAnalysis as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGraphicsAnalysis {}
