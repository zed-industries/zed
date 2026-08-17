// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dxgi1_2.h
use ctypes::c_void;
use shared::basetsd::SIZE_T;
use shared::dxgi::{
    DXGI_MAPPED_RECT, DXGI_SWAP_EFFECT, IDXGIAdapter1, IDXGIAdapter1Vtbl, IDXGIDevice1,
    IDXGIDevice1Vtbl, IDXGIFactory1, IDXGIFactory1Vtbl, IDXGIObject, IDXGIObjectVtbl, IDXGIOutput,
    IDXGIOutputVtbl, IDXGIResource, IDXGIResourceVtbl, IDXGISurface1, IDXGISurface1Vtbl,
    IDXGISwapChain, IDXGISwapChainVtbl,
};
use shared::dxgiformat::DXGI_FORMAT;
use shared::dxgitype::{
    DXGI_MODE_DESC, DXGI_MODE_ROTATION, DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL,
    DXGI_RGBA, DXGI_SAMPLE_DESC, DXGI_USAGE,
};
use shared::guiddef::REFGUID;
use shared::minwindef::{BOOL, DWORD, UINT};
use shared::windef::{HWND, POINT, RECT};
use um::minwinbase::SECURITY_ATTRIBUTES;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LARGE_INTEGER, LPCWSTR, LUID, WCHAR};
ENUM!{enum DXGI_ALPHA_MODE {
    DXGI_ALPHA_MODE_UNSPECIFIED = 0,
    DXGI_ALPHA_MODE_PREMULTIPLIED = 1,
    DXGI_ALPHA_MODE_STRAIGHT = 2,
    DXGI_ALPHA_MODE_IGNORE = 3,
    DXGI_ALPHA_MODE_FORCE_DWORD = 0xFFFFFFFF,
}}
ENUM!{enum DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY = 0,
    DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY = 1,
    DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY = 2,
    DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY = 3,
    DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY = 4,
}}
ENUM!{enum DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY = 0,
    DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY = 1,
    DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY = 2,
    DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY = 3,
    DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY = 4,
}}
ENUM!{enum DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME = 1,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR = 2,
    DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR = 4,
}}
ENUM!{enum DXGI_SCALING {
    DXGI_SCALING_STRETCH = 0,
    DXGI_SCALING_NONE = 1,
    DXGI_SCALING_ASPECT_RATIO_STRETCH = 2,
}}
ENUM!{enum _DXGI_OFFER_RESOURCE_PRIORITY {
    DXGI_OFFER_RESOURCE_PRIORITY_LOW = 1,
    DXGI_OFFER_RESOURCE_PRIORITY_NORMAL = 2,
    DXGI_OFFER_RESOURCE_PRIORITY_HIGH = 3,
}}
STRUCT!{struct DXGI_ADAPTER_DESC2 {
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
    GraphicsPreemptionGranularity: DXGI_GRAPHICS_PREEMPTION_GRANULARITY,
    ComputePreemptionGranularity: DXGI_COMPUTE_PREEMPTION_GRANULARITY,
}}
STRUCT!{struct DXGI_MODE_DESC1 {
    Width: UINT,
    Height: UINT,
    RefreshRate: DXGI_RATIONAL,
    Format: DXGI_FORMAT,
    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    Scaling: DXGI_MODE_SCALING,
    Stereo: BOOL,
}}
STRUCT!{struct DXGI_OUTDUPL_DESC {
    ModeDesc: DXGI_MODE_DESC,
    Rotation: DXGI_MODE_ROTATION,
    DesktopImageInSystemMemory: BOOL,
}}
STRUCT!{struct DXGI_OUTDUPL_FRAME_INFO {
    LastPresentTime: LARGE_INTEGER,
    LastMouseUpdateTime: LARGE_INTEGER,
    AccumulatedFrames: UINT,
    RectsCoalesced: BOOL,
    ProtectedContentMaskedOut: BOOL,
    PointerPosition: DXGI_OUTDUPL_POINTER_POSITION,
    TotalMetadataBufferSize: UINT,
    PointerShapeBufferSize: UINT,
}}
STRUCT!{struct DXGI_OUTDUPL_MOVE_RECT {
    SourcePoint: POINT,
    DestinationRect: RECT,
}}
STRUCT!{struct DXGI_OUTDUPL_POINTER_POSITION {
    Position: POINT,
    Visible: BOOL,
}}
STRUCT!{struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    Type: UINT,
    Width: UINT,
    Height: UINT,
    Pitch: UINT,
    HotSpot: POINT,
}}
STRUCT!{struct DXGI_PRESENT_PARAMETERS {
    DirtyRectsCount: UINT,
    pDirtyRects: *mut RECT,
    pScrollRect: *mut RECT,
    pScrollOffset: *mut POINT,
}}
STRUCT!{struct DXGI_SWAP_CHAIN_DESC1 {
    Width: UINT,
    Height: UINT,
    Format: DXGI_FORMAT,
    Stereo: BOOL,
    SampleDesc: DXGI_SAMPLE_DESC,
    BufferUsage: DXGI_USAGE,
    BufferCount: UINT,
    Scaling: DXGI_SCALING,
    SwapEffect: DXGI_SWAP_EFFECT,
    AlphaMode: DXGI_ALPHA_MODE,
    Flags: UINT,
}}
STRUCT!{struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    RefreshRate: DXGI_RATIONAL,
    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    Scaling: DXGI_MODE_SCALING,
    Windowed: BOOL,
}}
RIDL!{#[uuid(0x0aa1ae0a, 0xfa0e, 0x4b84, 0x86, 0x44, 0xe0, 0x5f, 0xf8, 0xe5, 0xac, 0xb5)]
interface IDXGIAdapter2(IDXGIAdapter2Vtbl): IDXGIAdapter1(IDXGIAdapter1Vtbl) {
    fn GetDesc2(
        pDesc: *mut DXGI_ADAPTER_DESC2,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x05008617, 0xfbfd, 0x4051, 0xa7, 0x90, 0x14, 0x48, 0x84, 0xb4, 0xf6, 0xa9)]
interface IDXGIDevice2(IDXGIDevice2Vtbl): IDXGIDevice1(IDXGIDevice1Vtbl) {
    fn OfferResources(
        NumResources: UINT,
        ppResources: *mut *mut IDXGIResource,
        Priority: DXGI_OFFER_RESOURCE_PRIORITY,
    ) -> HRESULT,
    fn ReclaimResources(
        NumResources: UINT,
        ppResources: *mut *mut IDXGIResource,
        pDiscarded: *mut BOOL,
    ) -> HRESULT,
    fn EnqueueSetEvent(
        hEvent: HANDLE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xea9dbf1a, 0xc88e, 0x4486, 0x85, 0x4a, 0x98, 0xaa, 0x01, 0x38, 0xf3, 0x0c)]
interface IDXGIDisplayControl(IDXGIDisplayControlVtbl): IUnknown(IUnknownVtbl) {
    fn IsStereoEnabled() -> BOOL,
    fn SetStereoEnabled(
        enabled: BOOL,
    ) -> (),
}}
RIDL!{#[uuid(0x50c83a1c, 0xe072, 0x4c48, 0x87, 0xb0, 0x36, 0x30, 0xfa, 0x36, 0xa6, 0xd0)]
interface IDXGIFactory2(IDXGIFactory2Vtbl): IDXGIFactory1(IDXGIFactory1Vtbl) {
    fn IsWindowedStereoEnabled() -> BOOL,
    fn CreateSwapChainForHwnd(
        pDevice: *mut IUnknown,
        hWnd: HWND,
        pDesc: *const DXGI_SWAP_CHAIN_DESC1,
        pFullscreenDesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        pRestrictToOutput: *mut IDXGIOutput,
        ppSwapChain: *mut *mut IDXGISwapChain1,
    ) -> HRESULT,
    fn CreateSwapChainForCoreWindow(
        pDevice: *mut IUnknown,
        pWindow: *mut IUnknown,
        pDesc: *const DXGI_SWAP_CHAIN_DESC1,
        pRestrictToOutput: *mut IDXGIOutput,
        ppSwapChain: *mut *mut IDXGISwapChain1,
    ) -> HRESULT,
    fn GetSharedResourceAdapterLuid(
        hResource: HANDLE,
        pLuid: *mut LUID,
    ) -> HRESULT,
    fn RegisterStereoStatusWindow(
        WindowHandle: HWND,
        wMsg: UINT,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn RegisterStereoStatusEvent(
        hEvent: HANDLE,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn UnregisterStereoStatus(
        dwCookie: DWORD,
    ) -> (),
    fn RegisterOcclusionStatusWindow(
        WindowHandle: HWND,
        wMsg: UINT,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn RegisterOcclusionStatusEvent(
        hEvent: HANDLE,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn UnregisterOcclusionStatus(
        dwCookie: DWORD,
    ) -> (),
    fn CreateSwapChainForComposition(
        pDevice: *mut IUnknown,
        pDesc: *const DXGI_SWAP_CHAIN_DESC1,
        pRestrictToOutput: *mut IDXGIOutput,
        ppSwapChain: *mut *mut IDXGISwapChain1,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x00cddea8, 0x939b, 0x4b83, 0xa3, 0x40, 0xa6, 0x85, 0x22, 0x66, 0x66, 0xcc)]
interface IDXGIOutput1(IDXGIOutput1Vtbl): IDXGIOutput(IDXGIOutputVtbl) {
    fn GetDisplayModeList1(
        EnumFormat: DXGI_FORMAT,
        Flags: UINT,
        pNumModes: *mut UINT,
        pDesc: *mut DXGI_MODE_DESC1,
    ) -> HRESULT,
    fn FindClosestMatchingMode1(
        pModeToMatch: *const DXGI_MODE_DESC1,
        pClosestMatch: *mut DXGI_MODE_DESC1,
        pConcernedDevice: *mut IUnknown,
    ) -> HRESULT,
    fn GetDisplaySurfaceData1(
        pDestination: *mut IDXGIResource,
    ) -> HRESULT,
    fn DuplicateOutput(
        pDevice: *mut IUnknown,
        ppOutputDuplication: *mut *mut IDXGIOutputDuplication,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x191cfac3, 0xa341, 0x470d, 0xb2, 0x6e, 0xa8, 0x64, 0xf4, 0x28, 0x31, 0x9c)]
interface IDXGIOutputDuplication(IDXGIOutputDuplicationVtbl): IDXGIObject(IDXGIObjectVtbl) {
    fn GetDesc(
        pDesc: *mut DXGI_OUTDUPL_DESC,
    ) -> (),
    fn AcquireNextFrame(
        TimeoutInMilliseconds: UINT,
        pFrameInfo: *mut DXGI_OUTDUPL_FRAME_INFO,
        ppDesktopResource: *mut *mut IDXGIResource,
    ) -> HRESULT,
    fn GetFrameDirtyRects(
        DirtyRectsBufferSize: UINT,
        pDirtyRectsBuffer: *mut RECT,
        pDirtyRectsBufferSizeRequired: *mut UINT,
    ) -> HRESULT,
    fn GetFrameMoveRects(
        MoveRectsBufferSize: UINT,
        pMoveRectBuffer: *mut DXGI_OUTDUPL_MOVE_RECT,
        pMoveRectsBufferSizeRequired: *mut UINT,
    ) -> HRESULT,
    fn GetFramePointerShape(
        PointerShapeBufferSize: UINT,
        pPointerShapeBuffer: *mut c_void,
        pPointerShapeBufferSizeRequired: *mut UINT,
        pPointerShapeInfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO,
    ) -> HRESULT,
    fn MapDesktopSurface(
        pLockedRect: *mut DXGI_MAPPED_RECT,
    ) -> HRESULT,
    fn UnMapDesktopSurface() -> HRESULT,
    fn ReleaseFrame() -> HRESULT,
}}
RIDL!{#[uuid(0x30961379, 0x4609, 0x4a41, 0x99, 0x8e, 0x54, 0xfe, 0x56, 0x7e, 0xe0, 0xc1)]
interface IDXGIResource1(IDXGIResource1Vtbl): IDXGIResource(IDXGIResourceVtbl) {
    fn CreateSubresourceSurface(
        index: UINT,
        ppSurface: *mut *mut IDXGISurface2,
    ) -> HRESULT,
    fn CreateSharedHandle(
        pAttributes: *const SECURITY_ATTRIBUTES,
        dwAccess: DWORD,
        lpName: LPCWSTR,
        pHandle: *mut HANDLE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xaba496dd, 0xb617, 0x4cb8, 0xa8, 0x66, 0xbc, 0x44, 0xd7, 0xeb, 0x1f, 0xa2)]
interface IDXGISurface2(IDXGISurface2Vtbl): IDXGISurface1(IDXGISurface1Vtbl) {
    fn GetResource(
        riid: REFGUID,
        ppParentResource: *mut *mut c_void,
        pSubresourceIndex: *mut UINT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x790a45f7, 0x0d42, 0x4876, 0x98, 0x3a, 0x0a, 0x55, 0xcf, 0xe6, 0xf4, 0xaa)]
interface IDXGISwapChain1(IDXGISwapChain1Vtbl): IDXGISwapChain(IDXGISwapChainVtbl) {
    fn GetDesc1(
        pDesc: *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> HRESULT,
    fn GetFullscreenDesc(
        pDesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> HRESULT,
    fn GetHwnd(
        pHwnd: *mut HWND,
    ) -> HRESULT,
    fn GetCoreWindow(
        refiid: REFGUID,
        ppUnk: *mut *mut c_void,
    ) -> HRESULT,
    fn Present1(
        SyncInterval: UINT,
        PresentFlags: UINT,
        pPresentParameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> HRESULT,
    fn IsTemporaryMonoSupported() -> BOOL,
    fn GetRestrictToOutput(
        ppRestrictToOutput: *mut *mut IDXGIOutput,
    ) -> HRESULT,
    fn SetBackgroundColor(
        pColor: *const DXGI_RGBA,
    ) -> HRESULT,
    fn GetBackgroundColor(
        pColor: *mut DXGI_RGBA,
    ) -> HRESULT,
    fn SetRotation(
        Rotation: DXGI_MODE_ROTATION,
    ) -> HRESULT,
    fn GetRotation(
        pRotation: *mut DXGI_MODE_ROTATION,
    ) -> HRESULT,
}}
pub type DXGI_OFFER_RESOURCE_PRIORITY = _DXGI_OFFER_RESOURCE_PRIORITY;
pub const DXGI_ENUM_MODES_DISABLED_STEREO: UINT = 8;
pub const DXGI_ENUM_MODES_STEREO: UINT = 4;
pub const DXGI_SHARED_RESOURCE_READ: UINT = 0x80000000;
pub const DXGI_SHARED_RESOURCE_WRITE: UINT = 1;
DEFINE_GUID!{IID_IDXGIDisplayControl,
    0xea9dbf1a, 0xc88e, 0x4486, 0x85, 0x4a, 0x98, 0xaa, 0x01, 0x38, 0xf3, 0x0c}
DEFINE_GUID!{IID_IDXGIOutputDuplication,
    0x191cfac3, 0xa341, 0x470d, 0xb2, 0x6e, 0xa8, 0x64, 0xf4, 0x28, 0x31, 0x9c}
DEFINE_GUID!{IID_IDXGISurface2,
    0xaba496dd, 0xb617, 0x4cb8, 0xa8, 0x66, 0xbc, 0x44, 0xd7, 0xeb, 0x1f, 0xa2}
DEFINE_GUID!{IID_IDXGIResource1,
    0x30961379, 0x4609, 0x4a41, 0x99, 0x8e, 0x54, 0xfe, 0x56, 0x7e, 0xe0, 0xc1}
DEFINE_GUID!{IID_IDXGIDevice2,
    0x05008617, 0xfbfd, 0x4051, 0xa7, 0x90, 0x14, 0x48, 0x84, 0xb4, 0xf6, 0xa9}
DEFINE_GUID!{IID_IDXGISwapChain1,
    0x790a45f7, 0x0d42, 0x4876, 0x98, 0x3a, 0x0a, 0x55, 0xcf, 0xe6, 0xf4, 0xaa}
DEFINE_GUID!{IID_IDXGIFactory2,
    0x50c83a1c, 0xe072, 0x4c48, 0x87, 0xb0, 0x36, 0x30, 0xfa, 0x36, 0xa6, 0xd0}
DEFINE_GUID!{IID_IDXGIAdapter2,
    0x0aa1ae0a, 0xfa0e, 0x4b84, 0x86, 0x44, 0xe0, 0x5f, 0xf8, 0xe5, 0xac, 0xb5}
DEFINE_GUID!{IID_IDXGIOutput1,
    0x00cddea8, 0x939b, 0x4b83, 0xa3, 0x40, 0xa6, 0x85, 0x22, 0x66, 0x66, 0xcc}
