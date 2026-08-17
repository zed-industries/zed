// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Direct3D include file
use shared::basetsd::UINT32;
use shared::d3d9caps::{D3DCAPS9, D3DCONTENTPROTECTIONCAPS, D3DOVERLAYCAPS};
use shared::d3d9types::{
    D3DADAPTER_IDENTIFIER9, D3DAUTHENTICATEDCHANNELTYPE, D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT,
    D3DBACKBUFFER_TYPE, D3DBOX, D3DCLIPSTATUS9, D3DCOLOR, D3DCOMPOSERECTSOP, D3DCUBEMAP_FACES,
    D3DDEVICE_CREATION_PARAMETERS, D3DDEVTYPE, D3DDISPLAYMODE, D3DDISPLAYMODEEX,
    D3DDISPLAYMODEFILTER, D3DDISPLAYROTATION, D3DENCRYPTED_BLOCK_INFO, D3DFORMAT, D3DGAMMARAMP,
    D3DINDEXBUFFER_DESC, D3DLIGHT9, D3DLOCKED_BOX, D3DLOCKED_RECT, D3DMATERIAL9, D3DMATRIX,
    D3DMULTISAMPLE_TYPE, D3DPOOL, D3DPRESENTSTATS, D3DPRESENT_PARAMETERS, D3DPRIMITIVETYPE,
    D3DQUERYTYPE, D3DRASTER_STATUS, D3DRECT, D3DRECTPATCH_INFO, D3DRENDERSTATETYPE,
    D3DRESOURCETYPE, D3DSAMPLERSTATETYPE, D3DSTATEBLOCKTYPE, D3DSURFACE_DESC, D3DTEXTUREFILTERTYPE,
    D3DTEXTURESTAGESTATETYPE, D3DTRANSFORMSTATETYPE, D3DTRIPATCH_INFO, D3DVERTEXBUFFER_DESC,
    D3DVERTEXELEMENT9, D3DVIEWPORT9, D3DVOLUME_DESC,
};
use shared::guiddef::{GUID, IID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, INT, UINT};
use shared::windef::{HDC, HMONITOR, HWND, POINT, RECT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wingdi::{PALETTEENTRY, RGNDATA};
use um::winnt::{HANDLE, HRESULT, LPCWSTR, LUID, VOID};
pub const D3D_SDK_VERSION: DWORD = 32;
pub const D3D9b_SDK_VERSION: DWORD = 31;
DEFINE_GUID!{IID_IDirect3D9,
    0x81bdcbca, 0x64d4, 0x426d, 0xae, 0x8d, 0xad, 0x01, 0x47, 0xf4, 0x27, 0x5c}
DEFINE_GUID!{IID_IDirect3DDevice9,
    0xd0223b96, 0xbf7a, 0x43fd, 0x92, 0xbd, 0xa4, 0x3b, 0x0d, 0x82, 0xb9, 0xeb}
DEFINE_GUID!{IID_IDirect3DResource9,
    0x05eec05d, 0x8f7d, 0x4362, 0xb9, 0x99, 0xd1, 0xba, 0xf3, 0x57, 0xc7, 0x04}
DEFINE_GUID!{IID_IDirect3DBaseTexture9,
    0x580ca87e, 0x1d3c, 0x4d54, 0x99, 0x1d, 0xb7, 0xd3, 0xe3, 0xc2, 0x98, 0xce}
DEFINE_GUID!{IID_IDirect3DTexture9,
    0x85c31227, 0x3de5, 0x4f00, 0x9b, 0x3a, 0xf1, 0x1a, 0xc3, 0x8c, 0x18, 0xb5}
DEFINE_GUID!{IID_IDirect3DCubeTexture9,
    0xfff32f81, 0xd953, 0x473a, 0x92, 0x23, 0x93, 0xd6, 0x52, 0xab, 0xa9, 0x3f}
DEFINE_GUID!{IID_IDirect3DVolumeTexture9,
    0x2518526c, 0xe789, 0x4111, 0xa7, 0xb9, 0x47, 0xef, 0x32, 0x8d, 0x13, 0xe6}
DEFINE_GUID!{IID_IDirect3DVertexBuffer9,
    0xb64bb1b5, 0xfd70, 0x4df6, 0xbf, 0x91, 0x19, 0xd0, 0xa1, 0x24, 0x55, 0xe3}
DEFINE_GUID!{IID_IDirect3DIndexBuffer9,
    0x7c9dd65e, 0xd3f7, 0x4529, 0xac, 0xee, 0x78, 0x58, 0x30, 0xac, 0xde, 0x35}
DEFINE_GUID!{IID_IDirect3DSurface9,
    0x0cfbaf3a, 0x9ff6, 0x429a, 0x99, 0xb3, 0xa2, 0x79, 0x6a, 0xf8, 0xb8, 0x9b}
DEFINE_GUID!{IID_IDirect3DVolume9,
    0x24f416e6, 0x1f67, 0x4aa7, 0xb8, 0x8e, 0xd3, 0x3f, 0x6f, 0x31, 0x28, 0xa1}
DEFINE_GUID!{IID_IDirect3DSwapChain9,
    0x794950f2, 0xadfc, 0x458a, 0x90, 0x5e, 0x10, 0xa1, 0x0b, 0x0b, 0x50, 0x3b}
DEFINE_GUID!{IID_IDirect3DVertexDeclaration9,
    0xdd13c59c, 0x36fa, 0x4098, 0xa8, 0xfb, 0xc7, 0xed, 0x39, 0xdc, 0x85, 0x46}
DEFINE_GUID!{IID_IDirect3DVertexShader9,
    0xefc5557e, 0x6265, 0x4613, 0x8a, 0x94, 0x43, 0x85, 0x78, 0x89, 0xeb, 0x36}
DEFINE_GUID!{IID_IDirect3DPixelShader9,
    0x6d3bdbdc, 0x5b02, 0x4415, 0xb8, 0x52, 0xce, 0x5e, 0x8b, 0xcc, 0xb2, 0x89}
DEFINE_GUID!{IID_IDirect3DStateBlock9,
    0xb07c4fe5, 0x310d, 0x4ba8, 0xa2, 0x3c, 0x4f, 0x0f, 0x20, 0x6f, 0x21, 0x8b}
DEFINE_GUID!{IID_IDirect3DQuery9,
    0xd9771460, 0xa695, 0x4f26, 0xbb, 0xd3, 0x27, 0xb8, 0x40, 0xb5, 0x41, 0xcc}
DEFINE_GUID!{IID_HelperName,
    0xe4a36723, 0xfdfe, 0x4b22, 0xb1, 0x46, 0x3c, 0x04, 0xc0, 0x7f, 0x4c, 0xc8}
DEFINE_GUID!{IID_IDirect3D9Ex,
    0x02177241, 0x69fc, 0x400c, 0x8f, 0xf1, 0x93, 0xa4, 0x4d, 0xf6, 0x86, 0x1d}
DEFINE_GUID!{IID_IDirect3DDevice9Ex,
    0xb18b10ce, 0x2649, 0x405a, 0x87, 0x0f, 0x95, 0xf7, 0x77, 0xd4, 0x31, 0x3a}
DEFINE_GUID!{IID_IDirect3DSwapChain9Ex,
    0x91886caf, 0x1c3d, 0x4d2e, 0xa0, 0xab, 0x3e, 0x4c, 0x7d, 0x8d, 0x33, 0x03}
DEFINE_GUID!{IID_IDirect3D9ExOverlayExtension,
    0x187aeb13, 0xaaf5, 0x4c59, 0x87, 0x6d, 0xe0, 0x59, 0x08, 0x8c, 0x0d, 0xf8}
DEFINE_GUID!{IID_IDirect3DDevice9Video,
    0x26dc4561, 0xa1ee, 0x4ae7, 0x96, 0xda, 0x11, 0x8a, 0x36, 0xc0, 0xec, 0x95}
DEFINE_GUID!{IID_IDirect3DAuthenticatedChannel9,
    0xff24beee, 0xda21, 0x4beb, 0x98, 0xb5, 0xd2, 0xf8, 0x99, 0xf9, 0x8a, 0xf9}
DEFINE_GUID!{IID_IDirect3DCryptoSession9,
    0xfa0ab799, 0x7a9c, 0x48ca, 0x8c, 0x5b, 0x23, 0x7e, 0x71, 0xa5, 0x44, 0x34}
extern "system" {
    pub fn Direct3DCreate9(
        SDKVersion: UINT,
    ) -> *mut IDirect3D9;
    pub fn D3DPERF_BeginEvent(
        col: D3DCOLOR,
        wszName: LPCWSTR,
    ) -> INT;
    pub fn D3DPERF_EndEvent() -> INT;
    pub fn D3DPERF_SetMarker(
        col: D3DCOLOR,
        wszName: LPCWSTR,
    ) -> ();
    pub fn D3DPERF_SetRegion(
        col: D3DCOLOR,
        wszName: LPCWSTR,
    ) -> ();
    pub fn D3DPERF_QueryRepeatFrame() -> BOOL;
    pub fn D3DPERF_SetOptions(
        dwOptions: DWORD,
    ) -> ();
    pub fn D3DPERF_GetStatus() -> DWORD;
}
RIDL!{#[uuid(0x81bdcbca, 0x64d4, 0x426d, 0xae, 0x8d, 0xad, 0x1, 0x47, 0xf4, 0x27, 0x5c)]
interface IDirect3D9(IDirect3D9Vtbl): IUnknown(IUnknownVtbl) {
    fn RegisterSoftwareDevice(
        pInitializeFunction: *mut VOID,
    ) -> HRESULT,
    fn GetAdapterCount() -> UINT,
    fn GetAdapterIdentifier(
        Adapter: UINT,
        Flags: DWORD,
        pIdentifier: *mut D3DADAPTER_IDENTIFIER9,
    ) -> HRESULT,
    fn GetAdapterModeCount(
        Adapter: UINT,
        Format: D3DFORMAT,
    ) -> UINT,
    fn EnumAdapterModes(
        Adapter: UINT,
        Format: D3DFORMAT,
        Mode: UINT,
        pMode: *mut D3DDISPLAYMODE,
    ) -> HRESULT,
    fn GetAdapterDisplayMode(
        Adapter: UINT,
        pMode: *mut D3DDISPLAYMODE,
    ) -> HRESULT,
    fn CheckDeviceType(
        Adapter: UINT,
        DevType: D3DDEVTYPE,
        AdapterFormat: D3DFORMAT,
        BackBufferFormat: D3DFORMAT,
        bWindowed: BOOL,
    ) -> HRESULT,
    fn CheckDeviceFormat(
        Adapter: UINT,
        DeviceType: D3DDEVTYPE,
        AdapterFormat: D3DFORMAT,
        Usage: DWORD,
        RType: D3DRESOURCETYPE,
        CheckFormat: D3DFORMAT,
    ) -> HRESULT,
    fn CheckDeviceMultiSampleType(
        Adapter: UINT,
        DeviceType: D3DDEVTYPE,
        SurfaceFormat: D3DFORMAT,
        Windowed: BOOL,
        MultiSampleType: D3DMULTISAMPLE_TYPE,
        pQualityLevels: *mut DWORD,
    ) -> HRESULT,
    fn CheckDepthStencilMatch(
        Adapter: UINT,
        DeviceType: D3DDEVTYPE,
        AdapterFormat: D3DFORMAT,
        RenderTargetFormat: D3DFORMAT,
        DepthStencilFormat: D3DFORMAT,
    ) -> HRESULT,
    fn CheckDeviceFormatConversion(
        Adapter: UINT,
        DeviceType: D3DDEVTYPE,
        SourceFormat: D3DFORMAT,
        TargetFormat: D3DFORMAT,
    ) -> HRESULT,
    fn GetDeviceCaps(
        Adapter: UINT,
        DeviceType: D3DDEVTYPE,
        pCaps: *mut D3DCAPS9,
    ) -> HRESULT,
    fn GetAdapterMonitor(
        Adapter: UINT,
    ) -> HMONITOR,
    fn CreateDevice(
        Adapter: UINT,
        DeviceType: D3DDEVTYPE,
        hFocusWindow: HWND,
        BehaviorFlags: DWORD,
        pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
        ppReturnedDeviceInterface: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
}}
pub type LPDIRECT3D9 = *mut IDirect3D9;
pub type PDIRECT3D9 = *mut IDirect3D9;
RIDL!{#[uuid(0xd0223b96, 0xbf7a, 0x43fd, 0x92, 0xbd, 0xa4, 0x3b, 0xd, 0x82, 0xb9, 0xeb)]
interface IDirect3DDevice9(IDirect3DDevice9Vtbl): IUnknown(IUnknownVtbl) {
    fn TestCooperativeLevel() -> HRESULT,
    fn GetAvailableTextureMem() -> UINT,
    fn EvictManagedResources() -> HRESULT,
    fn GetDirect3D(
        ppD3D9: *mut *mut IDirect3D9,
    ) -> HRESULT,
    fn GetDeviceCaps(
        pCaps: *mut D3DCAPS9,
    ) -> HRESULT,
    fn GetDisplayMode(
        iSwapChain: UINT,
        pMode: *mut D3DDISPLAYMODE,
    ) -> HRESULT,
    fn GetCreationParameters(
        pParameters: *mut D3DDEVICE_CREATION_PARAMETERS,
    ) -> HRESULT,
    fn SetCursorProperties(
        XHotSpot: UINT,
        YHotSpot: UINT,
        pCursorBitmap: *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn SetCursorPosition(
        X: INT,
        Y: INT,
        Flags: DWORD,
    ) -> (),
    fn ShowCursor(
        bShow: BOOL,
    ) -> BOOL,
    fn CreateAdditionalSwapChain(
        pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
        pSwapChain: *mut *mut IDirect3DSwapChain9,
    ) -> HRESULT,
    fn GetSwapChain(
        iSwapChain: UINT,
        pSwapChain: *mut *mut IDirect3DSwapChain9,
    ) -> HRESULT,
    fn GetNumberOfSwapChains() -> UINT,
    fn Reset(
        pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
    ) -> HRESULT,
    fn Present(
        pSourceRect: *const RECT,
        pDestRect: *const RECT,
        hDestWindowOverride: HWND,
        pDirtyRegion: *const RGNDATA,
    ) -> HRESULT,
    fn GetBackBuffer(
        iSwapChain: UINT,
        iBackBuffer: UINT,
        Type: D3DBACKBUFFER_TYPE,
        ppBackBuffer: *mut *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn GetRasterStatus(
        iSwapChain: UINT,
        pRasterStatus: *mut D3DRASTER_STATUS,
    ) -> HRESULT,
    fn SetDialogBoxMode(
        bEnableDialogs: BOOL,
    ) -> HRESULT,
    fn SetGammaRamp(
        iSwapChain: UINT,
        Flags: DWORD,
        pRamp: *const D3DGAMMARAMP,
    ) -> (),
    fn GetGammaRamp(
        iSwapChain: UINT,
        pRamp: *mut D3DGAMMARAMP,
    ) -> (),
    fn CreateTexture(
        Width: UINT,
        Height: UINT,
        Levels: UINT,
        Usage: DWORD,
        Format: D3DFORMAT,
        Pool: D3DPOOL,
        ppTexture: *mut *mut IDirect3DTexture9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn CreateVolumeTexture(
        Width: UINT,
        Height: UINT,
        Depth: UINT,
        Levels: UINT,
        Usage: DWORD,
        Format: D3DFORMAT,
        Pool: D3DPOOL,
        ppVolumeTexture: *mut *mut IDirect3DVolumeTexture9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn CreateCubeTexture(
        EdgeLength: UINT,
        Levels: UINT,
        Usage: DWORD,
        Format: D3DFORMAT,
        Pool: D3DPOOL,
        ppCubeTexture: *mut *mut IDirect3DCubeTexture9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn CreateVertexBuffer(
        Length: UINT,
        Usage: DWORD,
        FVF: DWORD,
        Pool: D3DPOOL,
        ppVertexBuffer: *mut *mut IDirect3DVertexBuffer9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn CreateIndexBuffer(
        Length: UINT,
        Usage: DWORD,
        Format: D3DFORMAT,
        Pool: D3DPOOL,
        ppIndexBuffer: *mut *mut IDirect3DIndexBuffer9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn CreateRenderTarget(
        Width: UINT,
        Height: UINT,
        Format: D3DFORMAT,
        MultiSample: D3DMULTISAMPLE_TYPE,
        MultisampleQuality: DWORD,
        Lockable: BOOL,
        ppSurface: *mut *mut IDirect3DSurface9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn CreateDepthStencilSurface(
        Width: UINT,
        Height: UINT,
        Format: D3DFORMAT,
        MultiSample: D3DMULTISAMPLE_TYPE,
        MultisampleQuality: DWORD,
        Discard: BOOL,
        ppSurface: *mut *mut IDirect3DSurface9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn UpdateSurface(
        pSourceSurface: *mut IDirect3DSurface9,
        pSourceRect: *const RECT,
        pDestinationSurface: *mut IDirect3DSurface9,
        pDestPoint: *const POINT,
    ) -> HRESULT,
    fn UpdateTexture(
        pSourceTexture: *mut IDirect3DBaseTexture9,
        pDestinationTexture: *mut IDirect3DBaseTexture9,
    ) -> HRESULT,
    fn GetRenderTargetData(
        pRenderTarget: *mut IDirect3DSurface9,
        pDestSurface: *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn GetFrontBufferData(
        iSwapChain: UINT,
        pDestSurface: *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn StretchRect(
        pSourceSurface: *mut IDirect3DSurface9,
        pSourceRect: *const RECT,
        pDestSurface: *mut IDirect3DSurface9,
        pDestRect: *const RECT,
        Filter: D3DTEXTUREFILTERTYPE,
    ) -> HRESULT,
    fn ColorFill(
        pSurface: *mut IDirect3DSurface9,
        pRect: *const RECT,
        color: D3DCOLOR,
    ) -> HRESULT,
    fn CreateOffscreenPlainSurface(
        Width: UINT,
        Height: UINT,
        Format: D3DFORMAT,
        Pool: D3DPOOL,
        ppSurface: *mut *mut IDirect3DSurface9,
        pSharedHandle: *mut HANDLE,
    ) -> HRESULT,
    fn SetRenderTarget(
        RenderTargetIndex: DWORD,
        pRenderTarget: *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn GetRenderTarget(
        RenderTargetIndex: DWORD,
        ppRenderTarget: *mut *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn SetDepthStencilSurface(
        pNewZStencil: *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn GetDepthStencilSurface(
        ppZStencilSurface: *mut *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn BeginScene() -> HRESULT,
    fn EndScene() -> HRESULT,
    fn Clear(
        Count: DWORD,
        pRects: *const D3DRECT,
        Flags: DWORD,
        Color: D3DCOLOR,
        Z: FLOAT,
        Stencil: DWORD,
    ) -> HRESULT,
    fn SetTransform(
        State: D3DTRANSFORMSTATETYPE,
        pMatrix: *const D3DMATRIX,
    ) -> HRESULT,
    fn GetTransform(
        State: D3DTRANSFORMSTATETYPE,
        pMatrix: *mut D3DMATRIX,
    ) -> HRESULT,
    fn MultiplyTransform(
        arg1: D3DTRANSFORMSTATETYPE,
        arg2: *const D3DMATRIX,
    ) -> HRESULT,
    fn SetViewport(
        pViewport: *const D3DVIEWPORT9,
    ) -> HRESULT,
    fn GetViewport(
        pViewport: *mut D3DVIEWPORT9,
    ) -> HRESULT,
    fn SetMaterial(
        pMaterial: *const D3DMATERIAL9,
    ) -> HRESULT,
    fn GetMaterial(
        pMaterial: *mut D3DMATERIAL9,
    ) -> HRESULT,
    fn SetLight(
        Index: DWORD,
        arg1: *const D3DLIGHT9,
    ) -> HRESULT,
    fn GetLight(
        Index: DWORD,
        arg1: *mut D3DLIGHT9,
    ) -> HRESULT,
    fn LightEnable(
        Index: DWORD,
        Enable: BOOL,
    ) -> HRESULT,
    fn GetLightEnable(
        Index: DWORD,
        pEnable: *mut BOOL,
    ) -> HRESULT,
    fn SetClipPlane(
        Index: DWORD,
        pPlane: *const FLOAT,
    ) -> HRESULT,
    fn GetClipPlane(
        Index: DWORD,
        pPlane: *mut FLOAT,
    ) -> HRESULT,
    fn SetRenderState(
        State: D3DRENDERSTATETYPE,
        Value: DWORD,
    ) -> HRESULT,
    fn GetRenderState(
        State: D3DRENDERSTATETYPE,
        pValue: *mut DWORD,
    ) -> HRESULT,
    fn CreateStateBlock(
        Type: D3DSTATEBLOCKTYPE,
        ppSB: *mut *mut IDirect3DStateBlock9,
    ) -> HRESULT,
    fn BeginStateBlock() -> HRESULT,
    fn EndStateBlock(
        ppSB: *mut *mut IDirect3DStateBlock9,
    ) -> HRESULT,
    fn SetClipStatus(
        pClipStatus: *const D3DCLIPSTATUS9,
    ) -> HRESULT,
    fn GetClipStatus(
        pClipStatus: *mut D3DCLIPSTATUS9,
    ) -> HRESULT,
    fn GetTexture(
        Stage: DWORD,
        ppTexture: *mut *mut IDirect3DBaseTexture9,
    ) -> HRESULT,
    fn SetTexture(
        Stage: DWORD,
        pTexture: *mut IDirect3DBaseTexture9,
    ) -> HRESULT,
    fn GetTextureStageState(
        Stage: DWORD,
        Type: D3DTEXTURESTAGESTATETYPE,
        pValue: *mut DWORD,
    ) -> HRESULT,
    fn SetTextureStageState(
        Stage: DWORD,
        Type: D3DTEXTURESTAGESTATETYPE,
        Value: DWORD,
    ) -> HRESULT,
    fn GetSamplerState(
        Sampler: DWORD,
        Type: D3DSAMPLERSTATETYPE,
        pValue: *mut DWORD,
    ) -> HRESULT,
    fn SetSamplerState(
        Sampler: DWORD,
        Type: D3DSAMPLERSTATETYPE,
        Value: DWORD,
    ) -> HRESULT,
    fn ValidateDevice(
        pNumPasses: *mut DWORD,
    ) -> HRESULT,
    fn SetPaletteEntries(
        PaletteNumber: UINT,
        pEntries: *const PALETTEENTRY,
    ) -> HRESULT,
    fn GetPaletteEntries(
        PaletteNumber: UINT,
        pEntries: *mut PALETTEENTRY,
    ) -> HRESULT,
    fn SetCurrentTexturePalette(
        PaletteNumber: UINT,
    ) -> HRESULT,
    fn GetCurrentTexturePalette(
        PaletteNumber: *mut UINT,
    ) -> HRESULT,
    fn SetScissorRect(
        pRect: *const RECT,
    ) -> HRESULT,
    fn GetScissorRect(
        pRect: *mut RECT,
    ) -> HRESULT,
    fn SetSoftwareVertexProcessing(
        bSoftware: BOOL,
    ) -> HRESULT,
    fn GetSoftwareVertexProcessing() -> BOOL,
    fn SetNPatchMode(
        nSegments: FLOAT,
    ) -> HRESULT,
    fn GetNPatchMode() -> FLOAT,
    fn DrawPrimitive(
        PrimitiveType: D3DPRIMITIVETYPE,
        StartVertex: UINT,
        PrimitiveCount: UINT,
    ) -> HRESULT,
    fn DrawIndexedPrimitive(
        arg1: D3DPRIMITIVETYPE,
        BaseVertexIndex: INT,
        MinVertexIndex: UINT,
        NumVertices: UINT,
        startIndex: UINT,
        primCount: UINT,
    ) -> HRESULT,
    fn DrawPrimitiveUP(
        PrimitiveType: D3DPRIMITIVETYPE,
        PrimitiveCount: UINT,
        pVertexStreamZeroData: *const VOID,
        VertexStreamZeroStride: UINT,
    ) -> HRESULT,
    fn DrawIndexedPrimitiveUP(
        PrimitiveType: D3DPRIMITIVETYPE,
        MinVertexIndex: UINT,
        NumVertices: UINT,
        PrimitiveCount: UINT,
        pIndexData: *const VOID,
        IndexDataFormat: D3DFORMAT,
        pVertexStreamZeroData: *const VOID,
        VertexStreamZeroStride: UINT,
    ) -> HRESULT,
    fn ProcessVertices(
        SrcStartIndex: UINT,
        DestIndex: UINT,
        VertexCount: UINT,
        pDestBuffer: *mut IDirect3DVertexBuffer9,
        pVertexDecl: *mut IDirect3DVertexDeclaration9,
        Flags: DWORD,
    ) -> HRESULT,
    fn CreateVertexDeclaration(
        pVertexElements: *const D3DVERTEXELEMENT9,
        ppDecl: *mut *mut IDirect3DVertexDeclaration9,
    ) -> HRESULT,
    fn SetVertexDeclaration(
        pDecl: *mut IDirect3DVertexDeclaration9,
    ) -> HRESULT,
    fn GetVertexDeclaration(
        ppDecl: *mut *mut IDirect3DVertexDeclaration9,
    ) -> HRESULT,
    fn SetFVF(
        FVF: DWORD,
    ) -> HRESULT,
    fn GetFVF(
        pFVF: *mut DWORD,
    ) -> HRESULT,
    fn CreateVertexShader(
        pFunction: *const DWORD,
        ppShader: *mut *mut IDirect3DVertexShader9,
    ) -> HRESULT,
    fn SetVertexShader(
        pShader: *mut IDirect3DVertexShader9,
    ) -> HRESULT,
    fn GetVertexShader(
        ppShader: *mut *mut IDirect3DVertexShader9,
    ) -> HRESULT,
    fn SetVertexShaderConstantF(
        StartRegister: UINT,
        pConstantData: *const FLOAT,
        Vector4fCount: UINT,
    ) -> HRESULT,
    fn GetVertexShaderConstantF(
        StartRegister: UINT,
        pConstantData: *mut FLOAT,
        Vector4fCount: UINT,
    ) -> HRESULT,
    fn SetVertexShaderConstantI(
        StartRegister: UINT,
        pConstantData: *const INT,
        Vector4iCount: UINT,
    ) -> HRESULT,
    fn GetVertexShaderConstantI(
        StartRegister: UINT,
        pConstantData: *mut INT,
        Vector4iCount: UINT,
    ) -> HRESULT,
    fn SetVertexShaderConstantB(
        StartRegister: UINT,
        pConstantData: *const BOOL,
        BoolCount: UINT,
    ) -> HRESULT,
    fn GetVertexShaderConstantB(
        StartRegister: UINT,
        pConstantData: *mut BOOL,
        BoolCount: UINT,
    ) -> HRESULT,
    fn SetStreamSource(
        StreamNumber: UINT,
        pStreamData: *mut IDirect3DVertexBuffer9,
        OffsetInBytes: UINT,
        Stride: UINT,
    ) -> HRESULT,
    fn GetStreamSource(
        StreamNumber: UINT,
        ppStreamData: *mut *mut IDirect3DVertexBuffer9,
        pOffsetInBytes: *mut UINT,
        pStride: *mut UINT,
    ) -> HRESULT,
    fn SetStreamSourceFreq(
        StreamNumber: UINT,
        Setting: UINT,
    ) -> HRESULT,
    fn GetStreamSourceFreq(
        StreamNumber: UINT,
        pSetting: *mut UINT,
    ) -> HRESULT,
    fn SetIndices(
        pIndexData: *mut IDirect3DIndexBuffer9,
    ) -> HRESULT,
    fn GetIndices(
        ppIndexData: *mut *mut IDirect3DIndexBuffer9,
    ) -> HRESULT,
    fn CreatePixelShader(
        pFunction: *const DWORD,
        ppShader: *mut *mut IDirect3DPixelShader9,
    ) -> HRESULT,
    fn SetPixelShader(
        pShader: *mut IDirect3DPixelShader9,
    ) -> HRESULT,
    fn GetPixelShader(
        ppShader: *mut *mut IDirect3DPixelShader9,
    ) -> HRESULT,
    fn SetPixelShaderConstantF(
        StartRegister: UINT,
        pConstantData: *const FLOAT,
        Vector4fCount: UINT,
    ) -> HRESULT,
    fn GetPixelShaderConstantF(
        StartRegister: UINT,
        pConstantData: *mut FLOAT,
        Vector4fCount: UINT,
    ) -> HRESULT,
    fn SetPixelShaderConstantI(
        StartRegister: UINT,
        pConstantData: *const INT,
        Vector4iCount: UINT,
    ) -> HRESULT,
    fn GetPixelShaderConstantI(
        StartRegister: UINT,
        pConstantData: *mut INT,
        Vector4iCount: UINT,
    ) -> HRESULT,
    fn SetPixelShaderConstantB(
        StartRegister: UINT,
        pConstantData: *const BOOL,
        BoolCount: UINT,
    ) -> HRESULT,
    fn GetPixelShaderConstantB(
        StartRegister: UINT,
        pConstantData: *mut BOOL,
        BoolCount: UINT,
    ) -> HRESULT,
    fn DrawRectPatch(
        Handle: UINT,
        pNumSegs: *const FLOAT,
        pRectPatchInfo: *const D3DRECTPATCH_INFO,
    ) -> HRESULT,
    fn DrawTriPatch(
        Handle: UINT,
        pNumSegs: *const FLOAT,
        pTriPatchInfo: *const D3DTRIPATCH_INFO,
    ) -> HRESULT,
    fn DeletePatch(
        Handle: UINT,
    ) -> HRESULT,
    fn CreateQuery(
        Type: D3DQUERYTYPE,
        ppQuery: *mut *mut IDirect3DQuery9,
    ) -> HRESULT,
}}
pub type LPDIRECT3DDEVICE9 = *mut IDirect3DDevice9;
pub type PDIRECT3DDEVICE9 = *mut IDirect3DDevice9;
RIDL!{#[uuid(0xb07c4fe5, 0x310d, 0x4ba8, 0xa2, 0x3c, 0x4f, 0xf, 0x20, 0x6f, 0x21, 0x8b)]
interface IDirect3DStateBlock9(IDirect3DStateBlock9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(
        ppDevice: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
    fn Capture() -> HRESULT,
    fn Apply() -> HRESULT,
}}
pub type LPDIRECT3DSTATEBLOCK9 = *mut IDirect3DStateBlock9;
pub type PDIRECT3DSTATEBLOCK9 = *mut IDirect3DStateBlock9;
RIDL!{#[uuid(0x794950f2, 0xadfc, 0x458a, 0x90, 0x5e, 0x10, 0xa1, 0xb, 0xb, 0x50, 0x3b)]
interface IDirect3DSwapChain9(IDirect3DSwapChain9Vtbl): IUnknown(IUnknownVtbl) {
    fn Present(
        pSourceRect: *const RECT,
        pDestRect: *const RECT,
        hDestWindowOverride: HWND,
        pDirtyRegion: *const RGNDATA,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetFrontBufferData(
        pDestSurface: *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn GetBackBuffer(
        iBackBuffer: UINT,
        Type: D3DBACKBUFFER_TYPE,
        ppBackBuffer: *mut *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn GetRasterStatus(
        pRasterStatus: *mut D3DRASTER_STATUS,
    ) -> HRESULT,
    fn GetDisplayMode(
        pMode: *mut D3DDISPLAYMODE,
    ) -> HRESULT,
    fn GetDevice(
        ppDevice: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
    fn GetPresentParameters(
        pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
    ) -> HRESULT,
}}
pub type LPDIRECT3DSWAPCHAIN9 = *mut IDirect3DSwapChain9;
pub type PDIRECT3DSWAPCHAIN9 = *mut IDirect3DSwapChain9;
RIDL!{#[uuid(0x5eec05d, 0x8f7d, 0x4362, 0xb9, 0x99, 0xd1, 0xba, 0xf3, 0x57, 0xc7, 0x4)]
interface IDirect3DResource9(IDirect3DResource9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(
        ppDevice: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
    fn SetPrivateData(
        refguid: *const GUID,
        pData: *const VOID,
        SizeOfData: DWORD,
        Flags: DWORD,
    ) -> HRESULT,
    fn GetPrivateData(
        refguid: *const GUID,
        pData: *mut VOID,
        pSizeOfData: *mut DWORD,
    ) -> HRESULT,
    fn FreePrivateData(
        refguid: *const GUID,
    ) -> HRESULT,
    fn SetPriority(
        PriorityNew: DWORD,
    ) -> DWORD,
    fn GetPriority() -> DWORD,
    fn PreLoad() -> (),
    fn GetType() -> D3DRESOURCETYPE,
}}
pub type LPDIRECT3DRESOURCE9 = *mut IDirect3DResource9;
pub type PDIRECT3DRESOURCE9 = *mut IDirect3DResource9;
RIDL!{#[uuid(0xdd13c59c, 0x36fa, 0x4098, 0xa8, 0xfb, 0xc7, 0xed, 0x39, 0xdc, 0x85, 0x46)]
interface IDirect3DVertexDeclaration9(IDirect3DVertexDeclaration9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(
        ppDevice: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
    fn GetDeclaration(
        pElement: *mut D3DVERTEXELEMENT9,
        pNumElements: *mut UINT,
    ) -> HRESULT,
}}
pub type LPDIRECT3DVERTEXDECLARATION9 = *mut IDirect3DVertexDeclaration9;
pub type PDIRECT3DVERTEXDECLARATION9 = *mut IDirect3DVertexDeclaration9;
RIDL!{#[uuid(0xefc5557e, 0x6265, 0x4613, 0x8a, 0x94, 0x43, 0x85, 0x78, 0x89, 0xeb, 0x36)]
interface IDirect3DVertexShader9(IDirect3DVertexShader9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(
        ppDevice: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
    fn GetFunction(
        arg1: *mut VOID,
        pSizeOfData: *mut UINT,
    ) -> HRESULT,
}}
pub type LPDIRECT3DVERTEXSHADER9 = *mut IDirect3DVertexShader9;
pub type PDIRECT3DVERTEXSHADER9 = *mut IDirect3DVertexShader9;
RIDL!{#[uuid(0x6d3bdbdc, 0x5b02, 0x4415, 0xb8, 0x52, 0xce, 0x5e, 0x8b, 0xcc, 0xb2, 0x89)]
interface IDirect3DPixelShader9(IDirect3DPixelShader9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(
        ppDevice: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
    fn GetFunction(
        arg1: *mut VOID,
        pSizeOfData: *mut UINT,
    ) -> HRESULT,
}}
pub type LPDIRECT3DPIXELSHADER9 = *mut IDirect3DPixelShader9;
pub type PDIRECT3DPIXELSHADER9 = *mut IDirect3DPixelShader9;
RIDL!{#[uuid(0x580ca87e, 0x1d3c, 0x4d54, 0x99, 0x1d, 0xb7, 0xd3, 0xe3, 0xc2, 0x98, 0xce)]
interface IDirect3DBaseTexture9(IDirect3DBaseTexture9Vtbl):
    IDirect3DResource9(IDirect3DResource9Vtbl) {
    fn SetLOD(
        LODNew: DWORD,
    ) -> DWORD,
    fn GetLOD() -> DWORD,
    fn GetLevelCount() -> DWORD,
    fn SetAutoGenFilterType(
        FilterType: D3DTEXTUREFILTERTYPE,
    ) -> HRESULT,
    fn GetAutoGenFilterType() -> D3DTEXTUREFILTERTYPE,
    fn GenerateMipSubLevels() -> (),
}}
pub type LPDIRECT3DBASETEXTURE9 = *mut IDirect3DBaseTexture9;
pub type PDIRECT3DBASETEXTURE9 = *mut IDirect3DBaseTexture9;
RIDL!{#[uuid(0x85c31227, 0x3de5, 0x4f00, 0x9b, 0x3a, 0xf1, 0x1a, 0xc3, 0x8c, 0x18, 0xb5)]
interface IDirect3DTexture9(IDirect3DTexture9Vtbl):
    IDirect3DBaseTexture9(IDirect3DBaseTexture9Vtbl) {
    fn GetLevelDesc(
        Level: UINT,
        pDesc: *mut D3DSURFACE_DESC,
    ) -> HRESULT,
    fn GetSurfaceLevel(
        Level: UINT,
        ppSurfaceLevel: *mut *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn LockRect(
        Level: UINT,
        pLockedRect: *mut D3DLOCKED_RECT,
        pRect: *const RECT,
        Flags: DWORD,
    ) -> HRESULT,
    fn UnlockRect(
        Level: UINT,
    ) -> HRESULT,
    fn AddDirtyRect(
        pDirtyRect: *const RECT,
    ) -> HRESULT,
}}
pub type LPDIRECT3DTEXTURE9 = *mut IDirect3DTexture9;
pub type PDIRECT3DTEXTURE9 = *mut IDirect3DTexture9;
RIDL!{#[uuid(0x2518526c, 0xe789, 0x4111, 0xa7, 0xb9, 0x47, 0xef, 0x32, 0x8d, 0x13, 0xe6)]
interface IDirect3DVolumeTexture9(IDirect3DVolumeTexture9Vtbl):
    IDirect3DBaseTexture9(IDirect3DBaseTexture9Vtbl) {
    fn GetLevelDesc(
        Level: UINT,
        pDesc: *mut D3DVOLUME_DESC,
    ) -> HRESULT,
    fn GetVolumeLevel(
        Level: UINT,
        ppVolumeLevel: *mut *mut IDirect3DVolume9,
    ) -> HRESULT,
    fn LockBox(
        Level: UINT,
        pLockedVolume: *mut D3DLOCKED_BOX,
        pBox: *const D3DBOX,
        Flags: DWORD,
    ) -> HRESULT,
    fn UnlockBox(
        Level: UINT,
    ) -> HRESULT,
    fn AddDirtyBox(
        pDirtyBox: *const D3DBOX,
    ) -> HRESULT,
}}
pub type LPDIRECT3DVOLUMETEXTURE9 = *mut IDirect3DVolumeTexture9;
pub type PDIRECT3DVOLUMETEXTURE9 = *mut IDirect3DVolumeTexture9;
RIDL!{#[uuid(0xfff32f81, 0xd953, 0x473a, 0x92, 0x23, 0x93, 0xd6, 0x52, 0xab, 0xa9, 0x3f)]
interface IDirect3DCubeTexture9(IDirect3DCubeTexture9Vtbl):
    IDirect3DBaseTexture9(IDirect3DBaseTexture9Vtbl) {
    fn GetLevelDesc(
        Level: UINT,
        pDesc: *mut D3DSURFACE_DESC,
    ) -> HRESULT,
    fn GetCubeMapSurface(
        FaceType: D3DCUBEMAP_FACES,
        Level: UINT,
        ppCubeMapSurface: *mut *mut IDirect3DSurface9,
    ) -> HRESULT,
    fn LockRect(
        FaceType: D3DCUBEMAP_FACES,
        Level: UINT,
        pLockedRect: *mut D3DLOCKED_RECT,
        pRect: *const RECT,
        Flags: DWORD,
    ) -> HRESULT,
    fn UnlockRect(
        FaceType: D3DCUBEMAP_FACES,
        Level: UINT,
    ) -> HRESULT,
    fn AddDirtyRect(
        FaceType: D3DCUBEMAP_FACES,
        pDirtyRect: *const RECT,
    ) -> HRESULT,
}}
pub type LPDIRECT3DCUBETEXTURE9 = *mut IDirect3DCubeTexture9;
pub type PDIRECT3DCUBETEXTURE9 = *mut IDirect3DCubeTexture9;
RIDL!{#[uuid(0xb64bb1b5, 0xfd70, 0x4df6, 0xbf, 0x91, 0x19, 0xd0, 0xa1, 0x24, 0x55, 0xe3)]
interface IDirect3DVertexBuffer9(IDirect3DVertexBuffer9Vtbl):
    IDirect3DResource9(IDirect3DResource9Vtbl) {
    fn Lock(
        OffsetToLock: UINT,
        SizeToLock: UINT,
        ppbData: *mut *mut VOID,
        Flags: DWORD,
    ) -> HRESULT,
    fn Unlock() -> HRESULT,
    fn GetDesc(
        pDesc: *mut D3DVERTEXBUFFER_DESC,
    ) -> HRESULT,
}}
pub type LPDIRECT3DVERTEXBUFFER9 = *mut IDirect3DVertexBuffer9;
pub type PDIRECT3DVERTEXBUFFER9 = *mut IDirect3DVertexBuffer9;
RIDL!{#[uuid(0x7c9dd65e, 0xd3f7, 0x4529, 0xac, 0xee, 0x78, 0x58, 0x30, 0xac, 0xde, 0x35)]
interface IDirect3DIndexBuffer9(IDirect3DIndexBuffer9Vtbl):
    IDirect3DResource9(IDirect3DResource9Vtbl) {
    fn Lock(
        OffsetToLock: UINT,
        SizeToLock: UINT,
        ppbData: *mut *mut VOID,
        Flags: DWORD,
    ) -> HRESULT,
    fn Unlock() -> HRESULT,
    fn GetDesc(
        pDesc: *mut D3DINDEXBUFFER_DESC,
    ) -> HRESULT,
}}
pub type LPDIRECT3DINDEXBUFFER9 = *mut IDirect3DIndexBuffer9;
pub type PDIRECT3DINDEXBUFFER9 = *mut IDirect3DIndexBuffer9;
RIDL!{#[uuid(0xcfbaf3a, 0x9ff6, 0x429a, 0x99, 0xb3, 0xa2, 0x79, 0x6a, 0xf8, 0xb8, 0x9b)]
interface IDirect3DSurface9(IDirect3DSurface9Vtbl): IDirect3DResource9(IDirect3DResource9Vtbl) {
    fn GetContainer(
        riid: *const IID,
        ppContainer: *mut *mut VOID,
    ) -> HRESULT,
    fn GetDesc(
        pDesc: *mut D3DSURFACE_DESC,
    ) -> HRESULT,
    fn LockRect(
        pLockedRect: *mut D3DLOCKED_RECT,
        pRect: *const RECT,
        Flags: DWORD,
    ) -> HRESULT,
    fn UnlockRect() -> HRESULT,
    fn GetDC(
        phdc: *mut HDC,
    ) -> HRESULT,
    fn ReleaseDC(
        hdc: HDC,
    ) -> HRESULT,
}}
pub type LPDIRECT3DSURFACE9 = *mut IDirect3DSurface9;
pub type PDIRECT3DSURFACE9 = *mut IDirect3DSurface9;
RIDL!{#[uuid(0x24f416e6, 0x1f67, 0x4aa7, 0xb8, 0x8e, 0xd3, 0x3f, 0x6f, 0x31, 0x28, 0xa1)]
interface IDirect3DVolume9(IDirect3DVolume9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(
        ppDevice: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
    fn SetPrivateData(
        refguid: *const GUID,
        pData: *const VOID,
        SizeOfData: DWORD,
        Flags: DWORD,
    ) -> HRESULT,
    fn GetPrivateData(
        refguid: *const GUID,
        pData: *mut VOID,
        pSizeOfData: *mut DWORD,
    ) -> HRESULT,
    fn FreePrivateData(
        refguid: *const GUID,
    ) -> HRESULT,
    fn GetContainer(
        riid: *const IID,
        ppContainer: *mut *mut VOID,
    ) -> HRESULT,
    fn GetDesc(
        pDesc: *mut D3DVOLUME_DESC,
    ) -> HRESULT,
    fn LockBox(
        pLockedVolume: *mut D3DLOCKED_BOX,
        pBox: *const D3DBOX,
        Flags: DWORD,
    ) -> HRESULT,
    fn UnlockBox() -> HRESULT,
}}
pub type LPDIRECT3DVOLUME9 = *mut IDirect3DVolume9;
pub type PDIRECT3DVOLUME9 = *mut IDirect3DVolume9;
RIDL!{#[uuid(0xd9771460, 0xa695, 0x4f26, 0xbb, 0xd3, 0x27, 0xb8, 0x40, 0xb5, 0x41, 0xcc)]
interface IDirect3DQuery9(IDirect3DQuery9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetDevice(
        ppDevice: *mut *mut IDirect3DDevice9,
    ) -> HRESULT,
    fn GetType() -> D3DRESOURCETYPE,
    fn GetDataSize() -> DWORD,
    fn Issue(
        dwIssueFlags: DWORD,
    ) -> HRESULT,
    fn GetData(
        pData: *mut VOID,
        dwSize: DWORD,
        dwGetDataFlags: DWORD,
    ) -> HRESULT,
}}
pub type LPDIRECT3DQUERY9 = *mut IDirect3DQuery9;
pub type PDIRECT3DQUERY9 = *mut IDirect3DQuery9;
pub const D3DCREATE_FPU_PRESERVE: DWORD = 0x2;
pub const D3DCREATE_MULTITHREADED: DWORD = 0x4;
pub const D3DCREATE_PUREDEVICE: DWORD = 0x10;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: DWORD = 0x20;
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: DWORD = 0x40;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: DWORD = 0x80;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: DWORD = 0x100;
pub const D3DCREATE_ADAPTERGROUP_DEVICE: DWORD = 0x200;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: DWORD = 0x400;
pub const D3DCREATE_NOWINDOWCHANGES: DWORD = 0x800;
pub const D3DCREATE_DISABLE_PSGP_THREADING: DWORD = 0x2000;
pub const D3DCREATE_ENABLE_PRESENTSTATS: DWORD = 0x4000;
pub const D3DCREATE_DISABLE_PRESENTSTATS: DWORD = 0x8000;
pub const D3DCREATE_SCREENSAVER: DWORD = 0x10000000;
pub const D3DADAPTER_DEFAULT: DWORD = 0;
extern "system" {
    pub fn Direct3DCreate9Ex(
        SDKVersion: UINT,
        arg1: *mut *mut IDirect3D9Ex,
    ) -> HRESULT;
}
RIDL!{#[uuid(0x02177241, 0x69fc, 0x400c, 0x8f, 0xf1, 0x93, 0xa4, 0x4d, 0xf6, 0x86, 0x1d)]
interface IDirect3D9Ex(IDirect3D9ExVtbl): IDirect3D9(IDirect3D9Vtbl) {
    fn GetAdapterModeCountEx(
        Adapter: UINT,
        pFilter: *const D3DDISPLAYMODEFILTER,
    ) -> UINT,
    fn EnumAdapterModesEx(
        Adapter: UINT,
        pFilter: *const D3DDISPLAYMODEFILTER,
        Mode: UINT,
        pMode: *mut D3DDISPLAYMODEEX,
    ) -> HRESULT,
    fn GetAdapterDisplayModeEx(
        Adapter: UINT,
        pMode: *mut D3DDISPLAYMODEEX,
        pRotation: *mut D3DDISPLAYROTATION,
    ) -> HRESULT,
    fn CreateDeviceEx(
        Adapter: UINT,
        DeviceType: D3DDEVTYPE,
        hFocusWindow: HWND,
        BehaviorFlags: DWORD,
        pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
        pFullscreenDisplayMode: *mut D3DDISPLAYMODEEX,
        ppReturnedDeviceInterface: *mut *mut IDirect3DDevice9Ex,
    ) -> HRESULT,
    fn GetAdapterLUID(
        Adapter: UINT,
        pLUID: *mut LUID,
    ) -> HRESULT,
}}
pub type LPDIRECT3D9EX = *mut IDirect3D9Ex;
pub type PDIRECT3D9EX = *mut IDirect3D9Ex;
RIDL!{#[uuid(0xb18b10ce, 0x2649, 0x405a, 0x87, 0xf, 0x95, 0xf7, 0x77, 0xd4, 0x31, 0x3a)]
interface IDirect3DDevice9Ex(IDirect3DDevice9ExVtbl): IDirect3DDevice9(IDirect3DDevice9Vtbl) {
    fn SetConvolutionMonoKernel(
        width: UINT,
        height: UINT,
        rows: *mut FLOAT,
        columns: *mut FLOAT,
    ) -> HRESULT,
    fn ComposeRects(
        pSrc: *mut IDirect3DSurface9,
        pDst: *mut IDirect3DSurface9,
        pSrcRectDescs: *mut IDirect3DVertexBuffer9,
        NumRects: UINT,
        pDstRectDescs: *mut IDirect3DVertexBuffer9,
        Operation: D3DCOMPOSERECTSOP,
        Xoffset: INT,
        Yoffset: INT,
    ) -> HRESULT,
    fn PresentEx(
        pSourceRect: *const RECT,
        pDestRect: *const RECT,
        hDestWindowOverride: HWND,
        pDirtyRegion: *const RGNDATA,
        dwFlags: DWORD,
    ) -> HRESULT,
    fn GetGPUThreadPriority(
        pPriority: *mut INT,
    ) -> HRESULT,
    fn SetGPUThreadPriority(
        Priority: INT,
    ) -> HRESULT,
    fn WaitForVBlank(
        iSwapChain: UINT,
    ) -> HRESULT,
    fn CheckResourceResidency(
        pResourceArray: *mut *mut IDirect3DResource9,
        NumResources: UINT32,
    ) -> HRESULT,
    fn SetMaximumFrameLatency(
        MaxLatency: UINT,
    ) -> HRESULT,
    fn GetMaximumFrameLatency(
        pMaxLatency: *mut UINT,
    ) -> HRESULT,
    fn CheckDeviceState(
        hDestinationWindow: HWND,
    ) -> HRESULT,
    fn CreateRenderTargetEx(
        Width: UINT,
        Height: UINT,
        Format: D3DFORMAT,
        MultiSample: D3DMULTISAMPLE_TYPE,
        MultisampleQuality: DWORD,
        Lockable: BOOL,
        ppSurface: *mut *mut IDirect3DSurface9,
        pSharedHandle: *mut HANDLE,
        Usage: DWORD,
    ) -> HRESULT,
    fn CreateOffscreenPlainSurfaceEx(
        Width: UINT,
        Height: UINT,
        Format: D3DFORMAT,
        Pool: D3DPOOL,
        ppSurface: *mut *mut IDirect3DSurface9,
        pSharedHandle: *mut HANDLE,
        Usage: DWORD,
    ) -> HRESULT,
    fn CreateDepthStencilSurfaceEx(
        Width: UINT,
        Height: UINT,
        Format: D3DFORMAT,
        MultiSample: D3DMULTISAMPLE_TYPE,
        MultisampleQuality: DWORD,
        Discard: BOOL,
        ppSurface: *mut *mut IDirect3DSurface9,
        pSharedHandle: *mut HANDLE,
        Usage: DWORD,
    ) -> HRESULT,
    fn ResetEx(
        pPresentationParameters: *mut D3DPRESENT_PARAMETERS,
        pFullscreenDisplayMode: *mut D3DDISPLAYMODEEX,
    ) -> HRESULT,
    fn GetDisplayModeEx(
        iSwapChain: UINT,
        pMode: *mut D3DDISPLAYMODEEX,
        pRotation: *mut D3DDISPLAYROTATION,
    ) -> HRESULT,
}}
pub type LPDIRECT3DDEVICE9EX = *mut IDirect3DDevice9Ex;
pub type PDIRECT3DDEVICE9EX = *mut IDirect3DDevice9Ex;
RIDL!{#[uuid(0x91886caf, 0x1c3d, 0x4d2e, 0xa0, 0xab, 0x3e, 0x4c, 0x7d, 0x8d, 0x33, 0x3)]
interface IDirect3DSwapChain9Ex(IDirect3DSwapChain9ExVtbl):
    IDirect3DSwapChain9(IDirect3DSwapChain9Vtbl) {
    fn GetLastPresentCount(
        pLastPresentCount: *mut UINT,
    ) -> HRESULT,
    fn GetPresentStats(
        pPresentationStatistics: *mut D3DPRESENTSTATS,
    ) -> HRESULT,
    fn GetDisplayModeEx(
        pMode: *mut D3DDISPLAYMODEEX,
        pRotation: *mut D3DDISPLAYROTATION,
    ) -> HRESULT,
}}
pub type LPDIRECT3DSWAPCHAIN9EX = *mut IDirect3DSwapChain9Ex;
pub type PDIRECT3DSWAPCHAIN9EX = *mut IDirect3DSwapChain9Ex;
RIDL!{#[uuid(0x187aeb13, 0xaaf5, 0x4c59, 0x87, 0x6d, 0xe0, 0x59, 0x8, 0x8c, 0xd, 0xf8)]
interface IDirect3D9ExOverlayExtension(IDirect3D9ExOverlayExtensionVtbl): IUnknown(IUnknownVtbl) {
    fn CheckDeviceOverlayType(
        Adapter: UINT,
        DevType: D3DDEVTYPE,
        OverlayWidth: UINT,
        OverlayHeight: UINT,
        OverlayFormat: D3DFORMAT,
        pDisplayMode: *mut D3DDISPLAYMODEEX,
        DisplayRotation: D3DDISPLAYROTATION,
        pOverlayCaps: *mut D3DOVERLAYCAPS,
    ) -> HRESULT,
}}
pub type LPDIRECT3D9EXOVERLAYEXTENSION = *mut IDirect3D9ExOverlayExtension;
pub type PDIRECT3D9EXOVERLAYEXTENSION = *mut IDirect3D9ExOverlayExtension;
RIDL!{#[uuid(0x26dc4561, 0xa1ee, 0x4ae7, 0x96, 0xda, 0x11, 0x8a, 0x36, 0xc0, 0xec, 0x95)]
interface IDirect3DDevice9Video(IDirect3DDevice9VideoVtbl): IUnknown(IUnknownVtbl) {
    fn GetContentProtectionCaps(
        pCryptoType: *const GUID,
        pDecodeProfile: *const GUID,
        pCaps: *mut D3DCONTENTPROTECTIONCAPS,
    ) -> HRESULT,
    fn CreateAuthenticatedChannel(
        ChannelType: D3DAUTHENTICATEDCHANNELTYPE,
        ppAuthenticatedChannel: *mut *mut IDirect3DAuthenticatedChannel9,
        pChannelHandle: *mut HANDLE,
    ) -> HRESULT,
    fn CreateCryptoSession(
        pCryptoType: *const GUID,
        pDecodeProfile: *const GUID,
        ppCryptoSession: *mut *mut IDirect3DCryptoSession9,
        pCryptoHandle: *mut HANDLE,
    ) -> HRESULT,
}}
pub type LPDIRECT3DDEVICE9VIDEO = *mut IDirect3DDevice9Video;
pub type PDIRECT3DDEVICE9VIDEO = *mut IDirect3DDevice9Video;
RIDL!{#[uuid(0xff24beee, 0xda21, 0x4beb, 0x98, 0xb5, 0xd2, 0xf8, 0x99, 0xf9, 0x8a, 0xf9)]
interface IDirect3DAuthenticatedChannel9(IDirect3DAuthenticatedChannel9Vtbl):
    IUnknown(IUnknownVtbl) {
    fn GetCertificateSize(
        pCertificateSize: *mut UINT,
    ) -> HRESULT,
    fn GetCertificate(
        CertifacteSize: UINT,
        ppCertificate: *mut BYTE,
    ) -> HRESULT,
    fn NegotiateKeyExchange(
        DataSize: UINT,
        pData: *mut VOID,
    ) -> HRESULT,
    fn Query(
        InputSize: UINT,
        pInput: *const VOID,
        OutputSize: UINT,
        pOutput: *mut VOID,
    ) -> HRESULT,
    fn Configure(
        InputSize: UINT,
        pInput: *const VOID,
        pOutput: *mut D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT,
    ) -> HRESULT,
}}
pub type LPDIRECT3DAUTHENTICATEDCHANNEL9 = *mut IDirect3DAuthenticatedChannel9;
pub type PDIRECT3DAUTHENTICATEDCHANNEL9 = *mut IDirect3DAuthenticatedChannel9;
RIDL!{#[uuid(0xfa0ab799, 0x7a9c, 0x48ca, 0x8c, 0x5b, 0x23, 0x7e, 0x71, 0xa5, 0x44, 0x34)]
interface IDirect3DCryptoSession9(IDirect3DCryptoSession9Vtbl): IUnknown(IUnknownVtbl) {
    fn GetCertificateSize(
        pCertificateSize: *mut UINT,
    ) -> HRESULT,
    fn GetCertificate(
        CertifacteSize: UINT,
        ppCertificate: *mut BYTE,
    ) -> HRESULT,
    fn NegotiateKeyExchange(
        DataSize: UINT,
        pData: *mut VOID,
    ) -> HRESULT,
    fn EncryptionBlt(
        pSrcSurface: *mut IDirect3DSurface9,
        pDstSurface: *mut IDirect3DSurface9,
        DstSurfaceSize: UINT,
        pIV: *mut VOID,
    ) -> HRESULT,
    fn DecryptionBlt(
        pSrcSurface: *mut IDirect3DSurface9,
        pDstSurface: *mut IDirect3DSurface9,
        SrcSurfaceSize: UINT,
        pEncryptedBlockInfo: *mut D3DENCRYPTED_BLOCK_INFO,
        pContentKey: *mut VOID,
        pIV: *mut VOID,
    ) -> HRESULT,
    fn GetSurfacePitch(
        pSrcSurface: *mut IDirect3DSurface9,
        pSurfacePitch: *mut UINT,
    ) -> HRESULT,
    fn StartSessionKeyRefresh(
        pRandomNumber: *mut VOID,
        RandomNumberSize: UINT,
    ) -> HRESULT,
    fn FinishSessionKeyRefresh() -> HRESULT,
    fn GetEncryptionBltKey(
        pReadbackKey: *mut VOID,
        KeySize: UINT,
    ) -> HRESULT,
}}
pub type LPDIRECT3DCRYPTOSESSION9 = *mut IDirect3DCryptoSession9;
pub type PDIRECT3DCRYPTOSESSION9 = *mut IDirect3DCryptoSession9;
