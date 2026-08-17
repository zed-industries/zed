// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dxgi.h
use ctypes::c_void;
use shared::basetsd::{SIZE_T, UINT64};
use shared::dxgiformat::DXGI_FORMAT;
use shared::dxgitype::{
    DXGI_GAMMA_CONTROL, DXGI_GAMMA_CONTROL_CAPABILITIES, DXGI_MODE_DESC, DXGI_MODE_ROTATION,
    DXGI_SAMPLE_DESC, DXGI_USAGE,
};
use shared::guiddef::{REFGUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, HMODULE, UINT};
use shared::windef::{HDC, HMONITOR, HWND, RECT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, INT, LARGE_INTEGER, LUID, WCHAR};
STRUCT!{struct DXGI_FRAME_STATISTICS {
    PresentCount: UINT,
    PresentRefreshCount: UINT,
    SyncRefreshCount: UINT,
    SyncQPCTime: LARGE_INTEGER,
    SyncGPUTime: LARGE_INTEGER,
}}
STRUCT!{struct DXGI_MAPPED_RECT {
    Pitch: INT,
    pBits: *mut BYTE,
}}
STRUCT!{struct DXGI_ADAPTER_DESC {
    Description: [WCHAR; 128],
    VendorId: UINT,
    DeviceId: UINT,
    SubSysId: UINT,
    Revision: UINT,
    DedicatedVideoMemory: SIZE_T,
    DedicatedSystemMemory: SIZE_T,
    SharedSystemMemory: SIZE_T,
    AdapterLuid: LUID,
}}
STRUCT!{struct DXGI_OUTPUT_DESC {
    DeviceName: [WCHAR; 32],
    DesktopCoordinates: RECT,
    AttachedToDesktop: BOOL,
    Rotation: DXGI_MODE_ROTATION,
    Monitor: HMONITOR,
}}
STRUCT!{struct DXGI_SHARED_RESOURCE {
    Handle: HANDLE,
}}
pub const DXGI_RESOURCE_PRIORITY_MINIMUM: DWORD = 0x28000000;
pub const DXGI_RESOURCE_PRIORITY_LOW: DWORD = 0x50000000;
pub const DXGI_RESOURCE_PRIORITY_NORMAL: DWORD = 0x78000000;
pub const DXGI_RESOURCE_PRIORITY_HIGH: DWORD = 0xa0000000;
pub const DXGI_RESOURCE_PRIORITY_MAXIMUM: DWORD = 0xc8000000;
ENUM!{enum DXGI_RESIDENCY {
    DXGI_RESIDENCY_FULLY_RESIDENT = 1,
    DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY = 2,
    DXGI_RESIDENCY_EVICTED_TO_DISK = 3,
}}
STRUCT!{struct DXGI_SURFACE_DESC {
    Width: UINT,
    Height: UINT,
    Format: DXGI_FORMAT,
    SampleDesc: DXGI_SAMPLE_DESC,
}}
ENUM!{enum DXGI_SWAP_EFFECT {
    DXGI_SWAP_EFFECT_DISCARD = 0,
    DXGI_SWAP_EFFECT_SEQUENTIAL = 1,
    DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL = 3,
    DXGI_SWAP_EFFECT_FLIP_DISCARD = 4,
}}
ENUM!{enum DXGI_SWAP_CHAIN_FLAG {
    DXGI_SWAP_CHAIN_FLAG_NONPREROTATED = 1,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH = 2,
    DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE = 4,
    DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT = 8,
    DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER = 16,
    DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY = 32,
    DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT = 64,
    DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER = 128,
    DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO = 256,
    DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO = 512,
    DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED = 1024,
    DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING = 2048,
}}
STRUCT!{struct DXGI_SWAP_CHAIN_DESC {
    BufferDesc: DXGI_MODE_DESC,
    SampleDesc: DXGI_SAMPLE_DESC,
    BufferUsage: DXGI_USAGE,
    BufferCount: UINT,
    OutputWindow: HWND,
    Windowed: BOOL,
    SwapEffect: DXGI_SWAP_EFFECT,
    Flags: UINT,
}}
RIDL!{#[uuid(0xaec22fb8, 0x76f3, 0x4639, 0x9b, 0xe0, 0x28, 0xeb, 0x43, 0xa6, 0x7a, 0x2e)]
interface IDXGIObject(IDXGIObjectVtbl): IUnknown(IUnknownVtbl) {
    fn SetPrivateData(
        Name: REFGUID,
        DataSize: UINT,
        pData: *const c_void,
    ) -> HRESULT,
    fn SetPrivateDataInterface(
        Name: REFGUID,
        pUnknown: *const IUnknown,
    ) -> HRESULT,
    fn GetPrivateData(
        Name: REFGUID,
        pDataSize: *mut UINT,
        pData: *mut c_void,
    ) -> HRESULT,
    fn GetParent(
        riid: REFIID,
        ppParent: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3d3e0379, 0xf9de, 0x4d58, 0xbb, 0x6c, 0x18, 0xd6, 0x29, 0x92, 0xf1, 0xa6)]
interface IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDevice(
        riid: REFIID,
        ppDevice: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x035f3ab4, 0x482e, 0x4e50, 0xb4, 0x1f, 0x8a, 0x7f, 0x8b, 0xd8, 0x96, 0x0b)]
interface IDXGIResource(IDXGIResourceVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn GetSharedHandle(
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn GetUsage(
        pUsage: *mut DXGI_USAGE,
    ) -> HRESULT,
    fn SetEvictionPriority(
        EvictionPriority: UINT,
    ) -> HRESULT,
    fn GetEvictionPriority(
        pEvictionPriority: *mut UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9d8e1289, 0xd7b3, 0x465f, 0x81, 0x26, 0x25, 0x0e, 0x34, 0x9a, 0xf8, 0x5d)]
interface IDXGIKeyedMutex(IDXGIKeyedMutexVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn AcquireSync(
        Key: UINT64,
        dwMilliseconds: DWORD,
    ) -> HRESULT,
    fn ReleaseSync(
        Key: UINT64,
    ) -> HRESULT,
}}
pub const DXGI_MAP_READ: UINT = 1;
pub const DXGI_MAP_WRITE: UINT = 2;
pub const DXGI_MAP_DISCARD: UINT = 4;
RIDL!{#[uuid(0xcafcb56c, 0x6ac3, 0x4889, 0xbf, 0x47, 0x9e, 0x23, 0xbb, 0xd2, 0x60, 0xec)]
interface IDXGISurface(IDXGISurfaceVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn GetDesc(
        pDesc: *mut DXGI_SURFACE_DESC,
    ) -> HRESULT,
    fn Map(
        pLockedRect: *mut DXGI_MAPPED_RECT,
        MapFlags: UINT,
    ) -> HRESULT,
    fn Unmap() -> HRESULT,
}}
RIDL!{#[uuid(0x4ae63092, 0x6327, 0x4c1b, 0x80, 0xae, 0xbf, 0xe1, 0x2e, 0xa3, 0x2b, 0x86)]
interface IDXGISurface1(IDXGISurface1Vtbl): IDXGISurface(IDXGISurfaceVtbl) {
    fn GetDC(
        Discard: BOOL,
        phdc: *mut HDC,
    ) -> HRESULT,
    fn ReleaseDC(
        pDirtyRect: *mut RECT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2411e7e1, 0x12ac, 0x4ccf, 0xbd, 0x14, 0x97, 0x98, 0xe8, 0x53, 0x4d, 0xc0)]
interface IDXGIAdapter(IDXGIAdapterVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn EnumOutputs(
        Output: UINT,
        ppOutput: *mut *mut IDXGIOutput,
    ) -> HRESULT,
    fn GetDesc(
        pDesc: *mut DXGI_ADAPTER_DESC,
    ) -> HRESULT,
    fn CheckInterfaceSupport(
        InterfaceName: REFGUID,
        pUMDVersion: *mut LARGE_INTEGER,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xae02eedb, 0xc735, 0x4690, 0x8d, 0x52, 0x5a, 0x8d, 0xc2, 0x02, 0x13, 0xaa)]
interface IDXGIOutput(IDXGIOutputVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDesc(
        pDesc: *mut DXGI_OUTPUT_DESC,
    ) -> HRESULT,
    fn GetDisplayModeList(
        EnumFormat: DXGI_FORMAT,
        Flags: UINT,
        pNumModes: *mut UINT,
        pDesc: *mut DXGI_MODE_DESC,
    ) -> HRESULT,
    fn FindClosestMatchingMode(
        pModeToMatch: *const DXGI_MODE_DESC,
        pClosestMatch: *mut DXGI_MODE_DESC,
        pConcernedDevice: *mut IUnknown,
    ) -> HRESULT,
    fn WaitForVBlank() -> HRESULT,
    fn TakeOwnership(
        pDevice: *mut IUnknown,
        Exclusive: BOOL,
    ) -> HRESULT,
    fn ReleaseOwnership() -> (),
    fn GetGammaControlCapabilities(
        pGammaCaps: *mut DXGI_GAMMA_CONTROL_CAPABILITIES,
    ) -> HRESULT,
    fn SetGammaControl(
        pArray: *const DXGI_GAMMA_CONTROL,
    ) -> HRESULT,
    fn GetGammaControl(
        pArray: *mut DXGI_GAMMA_CONTROL,
    ) -> HRESULT,
    fn SetDisplaySurface(
        pScanoutSurface: *mut IDXGISurface,
    ) -> HRESULT,
    fn GetDisplaySurfaceData(
        pDestination: *mut IDXGISurface,
    ) -> HRESULT,
    fn GetFrameStatistics(
        pStats: *mut DXGI_FRAME_STATISTICS,
    ) -> HRESULT,
}}
pub const DXGI_MAX_SWAP_CHAIN_BUFFERS: DWORD = 16;
pub const DXGI_PRESENT_TEST: DWORD = 0x00000001;
pub const DXGI_PRESENT_DO_NOT_SEQUENCE: DWORD = 0x00000002;
pub const DXGI_PRESENT_RESTART: DWORD = 0x00000004;
pub const DXGI_PRESENT_DO_NOT_WAIT: DWORD = 0x00000008;
pub const DXGI_PRESENT_STEREO_PREFER_RIGHT: DWORD = 0x00000010;
pub const DXGI_PRESENT_STEREO_TEMPORARY_MONO: DWORD = 0x00000020;
pub const DXGI_PRESENT_RESTRICT_TO_OUTPUT: DWORD = 0x00000040;
pub const DXGI_PRESENT_USE_DURATION: DWORD = 0x00000100;
pub const DXGI_PRESENT_ALLOW_TEARING: DWORD = 0x00000200;
pub const DXGI_ENUM_MODES_INTERLACED: UINT = 1;
pub const DXGI_ENUM_MODES_SCALING: UINT = 2;
RIDL!{#[uuid(0x310d36a0, 0xd2e7, 0x4c0a, 0xaa, 0x04, 0x6a, 0x9d, 0x23, 0xb8, 0x88, 0x6a)]
interface IDXGISwapChain(IDXGISwapChainVtbl): IDXGIDeviceSubObject(IDXGIDeviceSubObjectVtbl) {
    fn Present(
        SyncInterval: UINT,
        Flags: UINT,
    ) -> HRESULT,
    fn GetBuffer(
        Buffer: UINT,
        riid: REFIID,
        ppSurface: *mut *mut c_void,
    ) -> HRESULT,
    fn SetFullscreenState(
        Fullscreen: BOOL,
        pTarget: *mut IDXGIOutput,
    ) -> HRESULT,
    fn GetFullscreenState(
        pFullscreen: *mut BOOL,
        ppTarget: *mut *mut IDXGIOutput,
    ) -> HRESULT,
    fn GetDesc(
        pDesc: *mut DXGI_SWAP_CHAIN_DESC,
    ) -> HRESULT,
    fn ResizeBuffers(
        BufferCount: UINT,
        Width: UINT,
        Height: UINT,
        NewFormat: DXGI_FORMAT,
        SwapChainFlags: UINT,
    ) -> HRESULT,
    fn ResizeTarget(
        pNewTargetParameters: *const DXGI_MODE_DESC,
    ) -> HRESULT,
    fn GetContainingOutput(
        ppOutput: *mut *mut IDXGIOutput,
    ) -> HRESULT,
    fn GetFrameStatistics(
        pStats: *mut DXGI_FRAME_STATISTICS,
    ) -> HRESULT,
    fn GetLastPresentCount(
        pLastPresentCount: *mut UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7b7166ec, 0x21c7, 0x44ae, 0xb2, 0x1a, 0xc9, 0xae, 0x32, 0x1a, 0xe3, 0x69)]
interface IDXGIFactory(IDXGIFactoryVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn EnumAdapters(
        Adapter: UINT,
        ppAdapter: *mut *mut IDXGIAdapter,
    ) -> HRESULT,
    fn MakeWindowAssociation(
        WindowHandle: HWND,
        Flags: UINT,
    ) -> HRESULT,
    fn GetWindowAssociation(
        pWindowHandle: *mut HWND,
    ) -> HRESULT,
    fn CreateSwapChain(
        pDevice: *mut IUnknown,
        pDesc: *mut DXGI_SWAP_CHAIN_DESC,
        ppSwapChain: *mut *mut IDXGISwapChain,
    ) -> HRESULT,
    fn CreateSoftwareAdapter(
        Module: HMODULE,
        ppAdapter: *mut *mut IDXGIAdapter,
    ) -> HRESULT,
}}
extern "system" {
    pub fn CreateDXGIFactory(
        riid: REFIID,
        ppFactory: *mut *mut c_void,
    ) -> HRESULT;
    pub fn CreateDXGIFactory1(
        riid: REFIID,
        ppFactory: *mut *mut c_void,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x54ec77fa, 0x1377, 0x44e6, 0x8c, 0x32, 0x88, 0xfd, 0x5f, 0x44, 0xc8, 0x4c)]
interface IDXGIDevice(IDXGIDeviceVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetAdapter(
        pAdapter: *mut *mut IDXGIAdapter,
    ) -> HRESULT,
    fn CreateSurface(
        pDesc: *const DXGI_SURFACE_DESC,
        NumSurfaces: UINT,
        Usage: DXGI_USAGE,
        pSharedResource: *const DXGI_SHARED_RESOURCE,
        ppSurface: *mut *mut IDXGISurface,
    ) -> HRESULT,
    fn QueryResourceResidency(
        ppResources: *const *mut IUnknown,
        pResidencyStatus: *mut DXGI_RESIDENCY,
        NumResources: UINT,
    ) -> HRESULT,
    fn SetGPUThreadPriority(
        Priority: INT,
    ) -> HRESULT,
    fn GetGPUThreadPriority(
        pPriority: *mut INT,
    ) -> HRESULT,
}}
ENUM!{enum DXGI_ADAPTER_FLAG {
    DXGI_ADAPTER_FLAG_NONE,
    DXGI_ADAPTER_FLAG_REMOTE,
    DXGI_ADAPTER_FLAG_SOFTWARE,
}}
STRUCT!{struct DXGI_ADAPTER_DESC1 {
    Description: [WCHAR; 128],
    VendorId: UINT,
    DeviceId: UINT,
    SubSysId: UINT,
    Revision: UINT,
    DedicatedVideoMemory: SIZE_T,
    DedicatedSystemMemory: SIZE_T,
    SharedSystemMemory: SIZE_T,
    AdapterLuid: LUID,
    Flags: UINT,
}}
STRUCT!{struct DXGI_DISPLAY_COLOR_SPACE {
    PrimaryCoordinates: [[FLOAT; 2]; 8],
    WhitePoints: [[FLOAT; 2]; 16],
}}
RIDL!{#[uuid(0x770aae78, 0xf26f, 0x4dba, 0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87)]
interface IDXGIFactory1(IDXGIFactory1Vtbl): IDXGIFactory(IDXGIFactoryVtbl) {
    fn EnumAdapters1(
        Adapter: UINT,
        ppAdapter: *mut *mut IDXGIAdapter1,
    ) -> HRESULT,
    fn IsCurrent() -> BOOL,
}}
RIDL!{#[uuid(0x29038f61, 0x3839, 0x4626, 0x91, 0xfd, 0x08, 0x68, 0x79, 0x01, 0x1a, 0x05)]
interface IDXGIAdapter1(IDXGIAdapter1Vtbl): IDXGIAdapter(IDXGIAdapterVtbl) {
    fn GetDesc1(
        pDesc: *mut DXGI_ADAPTER_DESC1,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x77db970f, 0x6276, 0x48ba, 0xba, 0x28, 0x07, 0x01, 0x43, 0xb4, 0x39, 0x2c)]
interface IDXGIDevice1(IDXGIDevice1Vtbl): IDXGIDevice(IDXGIDeviceVtbl) {
    fn SetMaximumFrameLatency(
        MaxLatency: UINT,
    ) -> HRESULT,
    fn GetMaximumFrameLatency(
        pMaxLatency: *mut UINT,
    ) -> HRESULT,
}}
DEFINE_GUID!{IID_IDXGIObject,
    0xaec22fb8, 0x76f3, 0x4639, 0x9b, 0xe0, 0x28, 0xeb, 0x43, 0xa6, 0x7a, 0x2e}
DEFINE_GUID!{IID_IDXGIDeviceSubObject,
    0x3d3e0379, 0xf9de, 0x4d58, 0xbb, 0x6c, 0x18, 0xd6, 0x29, 0x92, 0xf1, 0xa6}
DEFINE_GUID!{IID_IDXGIResource,
    0x035f3ab4, 0x482e, 0x4e50, 0xb4, 0x1f, 0x8a, 0x7f, 0x8b, 0xd8, 0x96, 0x0b}
DEFINE_GUID!{IID_IDXGIKeyedMutex,
    0x9d8e1289, 0xd7b3, 0x465f, 0x81, 0x26, 0x25, 0x0e, 0x34, 0x9a, 0xf8, 0x5d}
DEFINE_GUID!{IID_IDXGISurface,
    0xcafcb56c, 0x6ac3, 0x4889, 0xbf, 0x47, 0x9e, 0x23, 0xbb, 0xd2, 0x60, 0xec}
DEFINE_GUID!{IID_IDXGISurface1,
    0x4ae63092, 0x6327, 0x4c1b, 0x80, 0xae, 0xbf, 0xe1, 0x2e, 0xa3, 0x2b, 0x86}
DEFINE_GUID!{IID_IDXGIAdapter,
    0x2411e7e1, 0x12ac, 0x4ccf, 0xbd, 0x14, 0x97, 0x98, 0xe8, 0x53, 0x4d, 0xc0}
DEFINE_GUID!{IID_IDXGIOutput,
    0xae02eedb, 0xc735, 0x4690, 0x8d, 0x52, 0x5a, 0x8d, 0xc2, 0x02, 0x13, 0xaa}
DEFINE_GUID!{IID_IDXGISwapChain,
    0x310d36a0, 0xd2e7, 0x4c0a, 0xaa, 0x04, 0x6a, 0x9d, 0x23, 0xb8, 0x88, 0x6a}
DEFINE_GUID!{IID_IDXGIFactory,
    0x7b7166ec, 0x21c7, 0x44ae, 0xb2, 0x1a, 0xc9, 0xae, 0x32, 0x1a, 0xe3, 0x69}
DEFINE_GUID!{IID_IDXGIDevice,
    0x54ec77fa, 0x1377, 0x44e6, 0x8c, 0x32, 0x88, 0xfd, 0x5f, 0x44, 0xc8, 0x4c}
DEFINE_GUID!{IID_IDXGIFactory1,
    0x770aae78, 0xf26f, 0x4dba, 0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87}
DEFINE_GUID!{IID_IDXGIAdapter1,
    0x29038f61, 0x3839, 0x4626, 0x91, 0xfd, 0x08, 0x68, 0x79, 0x01, 0x1a, 0x05}
DEFINE_GUID!{IID_IDXGIDevice1,
    0x77db970f, 0x6276, 0x48ba, 0xba, 0x28, 0x07, 0x01, 0x43, 0xb4, 0x39, 0x2c}
