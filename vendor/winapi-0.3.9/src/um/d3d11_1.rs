// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::basetsd::{UINT64, UINT8};
use shared::dxgiformat::DXGI_FORMAT;
use shared::dxgitype::{DXGI_COLOR_SPACE_TYPE, DXGI_RATIONAL};
use shared::guiddef::{GUID, REFIID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT, INT, UINT};
use um::d3d11::{
    D3D11_BLEND, D3D11_BLEND_OP, D3D11_BOX, D3D11_CULL_MODE, D3D11_FILL_MODE, D3D11_RECT,
    D3D11_VIDEO_DECODER_BUFFER_TYPE, D3D11_VIDEO_DECODER_CONFIG, D3D11_VIDEO_DECODER_DESC,
    ID3D11BlendState, ID3D11BlendStateVtbl, ID3D11Buffer, ID3D11CryptoSession, ID3D11Device,
    ID3D11DeviceChild, ID3D11DeviceChildVtbl, ID3D11DeviceContext, ID3D11DeviceContextVtbl,
    ID3D11DeviceVtbl, ID3D11RasterizerState, ID3D11RasterizerStateVtbl, ID3D11Resource,
    ID3D11VideoContext, ID3D11VideoContextVtbl, ID3D11VideoDecoder, ID3D11VideoDevice,
    ID3D11VideoDeviceVtbl, ID3D11VideoProcessor, ID3D11VideoProcessorEnumerator,
    ID3D11VideoProcessorEnumeratorVtbl, ID3D11View
};
use um::d3dcommon::D3D_FEATURE_LEVEL;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HANDLE, HRESULT, LPCWSTR};
DEFINE_GUID!{IID_ID3D11BlendState1,
    0xcc86fabe, 0xda55, 0x401d, 0x85, 0xe7, 0xe3, 0xc9, 0xde, 0x28, 0x77, 0xe9}
DEFINE_GUID!{IID_ID3D11RasterizerState1,
    0x1217d7a6, 0x5039, 0x418c, 0xb0, 0x42, 0x9c, 0xbe, 0x25, 0x6a, 0xfd, 0x6e}
DEFINE_GUID!{IID_ID3DDeviceContextState,
    0x5c1e0d8a, 0x7c23, 0x48f9, 0x8c, 0x59, 0xa9, 0x29, 0x58, 0xce, 0xff, 0x11}
DEFINE_GUID!{IID_ID3D11DeviceContext1,
    0xbb2c6faa, 0xb5fb, 0x4082, 0x8e, 0x6b, 0x38, 0x8b, 0x8c, 0xfa, 0x90, 0xe1}
DEFINE_GUID!{IID_ID3D11VideoContext1,
    0xa7f026da, 0xa5f8, 0x4487, 0xa5, 0x64, 0x15, 0xe3, 0x43, 0x57, 0x65, 0x1e}
DEFINE_GUID!{IID_ID3D11VideoDevice1,
    0x29da1d51, 0x1321, 0x4454, 0x80, 0x4b, 0xf5, 0xfc, 0x9f, 0x86, 0x1f, 0x0f}
DEFINE_GUID!{IID_ID3D11VideoProcessorEnumerator1,
    0x465217f2, 0x5568, 0x43cf, 0xb5, 0xb9, 0xf6, 0x1d, 0x54, 0x53, 0x1c, 0xa1}
DEFINE_GUID!{IID_ID3D11Device1,
    0xa04bfb29, 0x08ef, 0x43d6, 0xa4, 0x9c, 0xa9, 0xbd, 0xbd, 0xcb, 0xe6, 0x86}
DEFINE_GUID!{IID_ID3DUserDefinedAnnotation,
    0xb2daad8b, 0x03d4, 0x4dbf, 0x95, 0xeb, 0x32, 0xab, 0x4b, 0x63, 0xd0, 0xab}
ENUM!{enum D3D11_COPY_FLAGS {
    D3D11_COPY_NO_OVERWRITE = 0x00000001,
    D3D11_COPY_DISCARD = 0x00000002,
}}
ENUM!{enum D3D11_LOGIC_OP {
    D3D11_LOGIC_OP_CLEAR = 0,
    D3D11_LOGIC_OP_SET = 1,
    D3D11_LOGIC_OP_COPY = 2,
    D3D11_LOGIC_OP_COPY_INVERTED = 3,
    D3D11_LOGIC_OP_NOOP = 4,
    D3D11_LOGIC_OP_INVERT = 5,
    D3D11_LOGIC_OP_AND = 6,
    D3D11_LOGIC_OP_NAND = 7,
    D3D11_LOGIC_OP_OR = 8,
    D3D11_LOGIC_OP_NOR = 9,
    D3D11_LOGIC_OP_XOR = 10,
    D3D11_LOGIC_OP_EQUIV = 11,
    D3D11_LOGIC_OP_AND_REVERSE = 12,
    D3D11_LOGIC_OP_AND_INVERTED = 13,
    D3D11_LOGIC_OP_OR_REVERSE = 14,
    D3D11_LOGIC_OP_OR_INVERTED = 15,
}}
STRUCT!{struct D3D11_RENDER_TARGET_BLEND_DESC1 {
    BlendEnable: BOOL,
    LogicOpEnable: BOOL,
    SrcBlend: D3D11_BLEND,
    DestBlend: D3D11_BLEND,
    BlendOp: D3D11_BLEND_OP,
    SrcBlendAlpha: D3D11_BLEND,
    DestBlendAlpha: D3D11_BLEND,
    BlendOpAlpha: D3D11_BLEND_OP,
    LogicOp: D3D11_LOGIC_OP,
    RenderTargetWriteMask: UINT8,
}}
STRUCT!{struct D3D11_BLEND_DESC1 {
    AlphaToCoverageEnable: BOOL,
    IndependentBlendEnable: BOOL,
    RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1; 8],
}}
RIDL!{#[uuid(0xcc86fabe, 0xda55, 0x401d, 0x85, 0xe7, 0xe3, 0xc9, 0xde, 0x28, 0x77, 0xe9)]
interface ID3D11BlendState1(ID3D11BlendState1Vtbl): ID3D11BlendState(ID3D11BlendStateVtbl) {
    fn GetDesc1(
        pDesc: *mut D3D11_BLEND_DESC1,
    ) -> (),
}}
STRUCT!{struct D3D11_RASTERIZER_DESC1 {
    FillMode: D3D11_FILL_MODE,
    CullMode: D3D11_CULL_MODE,
    FrontCounterClockwise: BOOL,
    DepthBias: INT,
    DepthBiasClamp: FLOAT,
    SlopeScaledDepthBias: FLOAT,
    DepthClipEnable: BOOL,
    ScissorEnable: BOOL,
    MultisampleEnable: BOOL,
    AntialiasedLineEnable: BOOL,
    ForcedSampleCount: UINT,
}}
RIDL!{#[uuid(0x1217d7a6, 0x5039, 0x418c, 0xb0, 0x42, 0x9c, 0xbe, 0x25, 0x6a, 0xfd, 0x6e)]
interface ID3D11RasterizerState1(ID3D11RasterizerState1Vtbl):
    ID3D11RasterizerState(ID3D11RasterizerStateVtbl) {
    fn GetDesc1(
        pDesc: *mut D3D11_RASTERIZER_DESC1,
    ) -> (),
}}
ENUM!{enum D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG {
    D3D11_1_CREATE_DEVICE_CONTEXT_STATE_SINGLETHREADED = 0x1,
}}
RIDL!{#[uuid(0x5c1e0d8a, 0x7c23, 0x48f9, 0x8c, 0x59, 0xa9, 0x29, 0x58, 0xce, 0xff, 0x11)]
interface ID3DDeviceContextState(ID3DDeviceContextStateVtbl):
    ID3D11DeviceChild(ID3D11DeviceChildVtbl) {}}
RIDL!{#[uuid(0xbb2c6faa, 0xb5fb, 0x4082, 0x8e, 0x6b, 0x38, 0x8b, 0x8c, 0xfa, 0x90, 0xe1)]
interface ID3D11DeviceContext1(ID3D11DeviceContext1Vtbl):
    ID3D11DeviceContext(ID3D11DeviceContextVtbl) {
    fn CopySubresourceRegion1(
        pDstResource: *mut ID3D11Resource,
        DstSubresource: UINT,
        DstX: UINT,
        DstY: UINT,
        DstZ: UINT,
        pSrcResource: *mut ID3D11Resource,
        SrcSubresource: UINT,
        pSrcBox: *const D3D11_BOX,
        CopyFlags: UINT,
    ) -> (),
    fn UpdateSubresource1(
        pDstResource: *mut ID3D11Resource,
        DstSubresource: UINT,
        pDstBox: *const D3D11_BOX,
        pSrcData: *mut c_void,
        SrcRowPitch: UINT,
        SrcDepthPitch: UINT,
        CopyFlags: UINT,
    ) -> (),
    fn DiscardResource(
        pResource: *mut ID3D11Resource,
    ) -> (),
    fn DiscardView(
        pResource: *mut ID3D11Resource,
    ) -> (),
    fn VSSetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *const *mut ID3D11Buffer,
        pFirstConstant: *const UINT,
        pNumConstants: *const UINT,
    ) -> (),
    fn HSSetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *const *mut ID3D11Buffer,
        pFirstConstant: *const UINT,
        pNumConstants: *const UINT,
    ) -> (),
    fn DSSetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *const *mut ID3D11Buffer,
        pFirstConstant: *const UINT,
        pNumConstants: *const UINT,
    ) -> (),
    fn GSSetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *const *mut ID3D11Buffer,
        pFirstConstant: *const UINT,
        pNumConstants: *const UINT,
    ) -> (),
    fn PSSetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *const *mut ID3D11Buffer,
        pFirstConstant: *const UINT,
        pNumConstants: *const UINT,
    ) -> (),
    fn CSSetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *const *mut ID3D11Buffer,
        pFirstConstant: *const UINT,
        pNumConstants: *const UINT,
    ) -> (),
    fn VSGetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
        pFirstConstant: *mut UINT,
        pNumConstants: *mut UINT,
    ) -> (),
    fn HSGetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
        pFirstConstant: *mut UINT,
        pNumConstants: *mut UINT,
    ) -> (),
    fn DSGetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
        pFirstConstant: *mut UINT,
        pNumConstants: *mut UINT,
    ) -> (),
    fn GSGetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
        pFirstConstant: *mut UINT,
        pNumConstants: *mut UINT,
    ) -> (),
    fn PSGetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
        pFirstConstant: *mut UINT,
        pNumConstants: *mut UINT,
    ) -> (),
    fn CSGetConstantBuffers1(
        StartSlot: UINT,
        NumBuffers: UINT,
        ppConstantBuffers: *mut *mut ID3D11Buffer,
        pFirstConstant: *mut UINT,
        pNumConstants: *mut UINT,
    ) -> (),
    fn SwapDeviceContextState(
        pState: *mut ID3DDeviceContextState,
        ppPreviousState: *mut *mut ID3DDeviceContextState,
    ) -> (),
    fn ClearView(
        pView: *mut ID3D11View,
        Color: [FLOAT; 4],
        pRect: *const D3D11_RECT,
        NumRects: UINT,
    ) -> (),
    fn DiscardView1(
        pResourceView: *mut ID3D11View,
        pRects: *const D3D11_RECT,
        NumRects: UINT,
    ) -> (),
}}
STRUCT!{struct D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    ClearSize: UINT,
    EncryptedSize: UINT,
}}
STRUCT!{struct D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    DataOffset: UINT,
    DataSize: UINT,
    pIV: *mut c_void,
    IVSize: UINT,
    pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    SubSampleMappingCount: UINT,
}}
STRUCT!{struct D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    pCryptoSession: *mut ID3D11CryptoSession,
    BlobSize: UINT,
    pBlob: *mut c_void,
    pKeyInfoId: *mut GUID,
    PrivateDataSize: UINT,
    pPrivateData: *mut c_void,
}}
ENUM!{enum D3D11_VIDEO_DECODER_CAPS {
    D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE = 0x1,
    D3D11_VIDEO_DECODER_CAPS_NON_REAL_TIME = 0x02,
    D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_DYNAMIC = 0x04,
    D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_REQUIRED = 0x08,
    D3D11_VIDEO_DECODER_CAPS_UNSUPPORTED = 0x10,
}}
ENUM!{enum D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS {
    D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_ROTATION = 0x01,
    D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_RESIZE = 0x02,
    D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_COLOR_SPACE_CONVERSION = 0x04,
    D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_TRIPLE_BUFFER_OUTPUT = 0x08,
}}
STRUCT!{struct D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    Enable: BOOL,
    Width: UINT,
    Height: UINT,
    Format: DXGI_FORMAT,
}}
ENUM!{enum D3D11_CRYPTO_SESSION_STATUS {
    D3D11_CRYPTO_SESSION_STATUS_OK = 0,
    D3D11_CRYPTO_SESSION_STATUS_KEY_LOST = 1,
    D3D11_CRYPTO_SESSION_STATUS_KEY_AND_CONTENT_LOST = 2,
}}
STRUCT!{struct D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    PrivateDataSize: UINT,
    HWProtectionDataSize: UINT,
    pbInput: [BYTE; 4],
}}
STRUCT!{struct D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    PrivateDataSize: UINT,
    MaxHWProtectionDataSize: UINT,
    HWProtectionDataSize: UINT,
    TransportTime: UINT64,
    ExecutionTime: UINT64,
    pbOutput: [BYTE; 4],
}}
STRUCT!{struct D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    HWProtectionFunctionID: UINT,
    pInputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA,
    pOutputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA,
    Status: HRESULT,
}}
STRUCT!{struct D3D11_VIDEO_SAMPLE_DESC {
    Width: UINT,
    Height: UINT,
    Format: DXGI_FORMAT,
    ColorSpace: DXGI_COLOR_SPACE_TYPE,
}}
RIDL!{#[uuid(0xa7f026da, 0xa5f8, 0x4487, 0xa5, 0x64, 0x15, 0xe3, 0x43, 0x57, 0x65, 0x1e)]
interface ID3D11VideoContext1(ID3D11VideoContext1Vtbl):
    ID3D11VideoContext(ID3D11VideoContextVtbl) {
    fn SubmitDecoderBuffers1(
        pDecoder: *mut ID3D11VideoDecoder,
        NumBuffers: UINT,
        pBufferDesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1,
    ) -> HRESULT,
    fn GetDataForNewHardwareKey(
        pCryptoSession: *mut ID3D11CryptoSession,
        PrivateInputSize: UINT,
        pPrivateInputData: *const c_void,
        pPrivateOutputData: *mut UINT64,
    ) -> HRESULT,
    fn CheckCryptoSessionStatus(
        pCryptoSession: *mut ID3D11CryptoSession,
        pStatus: *mut D3D11_CRYPTO_SESSION_STATUS,
    ) -> HRESULT,
    fn DecoderEnableDownsampling(
        pDecoder: *mut ID3D11VideoDecoder,
        InputColorSpace: DXGI_COLOR_SPACE_TYPE,
        pOutputDesc: *const D3D11_VIDEO_SAMPLE_DESC,
        ReferenceFrameCount: UINT,
    ) -> HRESULT,
    fn DecoderUpdateDownsampling(
        pDecoder: *mut ID3D11VideoDecoder,
        pOutputDesc: *const D3D11_VIDEO_SAMPLE_DESC,
    ) -> HRESULT,
    fn VideoProcessorSetOutputColorSpace1(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        ColorSpace: DXGI_COLOR_SPACE_TYPE,
    ) -> (),
    fn VideoProcessorSetOutputShaderUsage(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        ShaderUsage: BOOL,
    ) -> (),
    fn VideoProcessorGetOutputColorSpace1(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        pColorSpace: *mut DXGI_COLOR_SPACE_TYPE,
    ) -> (),
    fn VideoProcessorGetOutputShaderUsage(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        pShaderUsage: *mut BOOL,
    ) -> (),
    fn VideoProcessorSetStreamColorSpace1(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        StreamIndex: UINT,
        ColorSpace: DXGI_COLOR_SPACE_TYPE,
    ) -> (),
    fn VideoProcessorSetStreamMirror(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        StreamIndex: UINT,
        Enable: BOOL,
        FlipHorizontal: BOOL,
        FlipVertical: BOOL,
    ) -> (),
    fn VideoProcessorGetStreamColorSpace1(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        StreamIndex: UINT,
        pColorSpace: *mut DXGI_COLOR_SPACE_TYPE,
    ) -> (),
    fn VideoProcessorGetStreamMirror(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        StreamIndex: UINT,
        pEnable: *mut BOOL,
        pFlipHorizontal: *mut BOOL,
        pFlipVertical: *mut BOOL,
    ) -> (),
    fn VideoProcessorGetBehaviorHints(
        pVideoProcessor: *mut ID3D11VideoProcessor,
        OutputWidth: UINT,
        OutputHeight: UINT,
        OutputFormat: DXGI_FORMAT,
        StreamCount: UINT,
        pStreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT,
        pBehaviorHints: *mut UINT,
    ) -> (),
}}
RIDL!{#[uuid(0x29da1d51, 0x1321, 0x4454, 0x80, 0x4b, 0xf5, 0xfc, 0x9f, 0x86, 0x1f, 0x0f)]
interface ID3D11VideoDevice1(ID3D11VideoDevice1Vtbl): ID3D11VideoDevice(ID3D11VideoDeviceVtbl) {
    fn GetCryptoSessionPrivateDataSize(
        pCryptoType: *const GUID,
        pDecoderProfile: *const GUID,
        pKeyExchangeType: *const GUID,
        pPrivateInputSize: *mut UINT,
        pPrivateOutputSize: *mut UINT,
    ) -> HRESULT,
    fn GetVideoDecoderCaps(
        pDecoderProfile: *const GUID,
        SampleWidth: UINT,
        SampleHeight: UINT,
        pFrameRate: *const DXGI_RATIONAL,
        BitRate: UINT,
        pCryptoType: *const GUID,
        pDecoderCaps: *mut UINT,
    ) -> HRESULT,
    fn CheckVideoDecoderDownsampling(
        pInputDesc: *const D3D11_VIDEO_DECODER_DESC,
        InputColorSpace: DXGI_COLOR_SPACE_TYPE,
        pInputConfig: *const D3D11_VIDEO_DECODER_CONFIG,
        pFrameRate: *const DXGI_RATIONAL,
        pOutputDesc: *const D3D11_VIDEO_SAMPLE_DESC,
        pSupported: *mut BOOL,
        pRealTimeHint: *mut BOOL,
    ) -> HRESULT,
    fn RecommendVideoDecoderDownsampleParameters(
        pInputDesc: *const D3D11_VIDEO_DECODER_DESC,
        InputColorSpace: DXGI_COLOR_SPACE_TYPE,
        pInputConfig: *const D3D11_VIDEO_DECODER_CONFIG,
        pRecommendedOutputDesc: *mut D3D11_VIDEO_SAMPLE_DESC,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x465217f2, 0x5568, 0x43cf, 0xb5, 0xb9, 0xf6, 0x1d, 0x54, 0x53, 0x1c, 0xa1)]
interface ID3D11VideoProcessorEnumerator1(ID3D11VideoProcessorEnumerator1Vtbl):
    ID3D11VideoProcessorEnumerator(ID3D11VideoProcessorEnumeratorVtbl) {
    fn CheckVideoProcessorFormatConversion(
        InputFormat: DXGI_FORMAT,
        InputCOlorSpace: DXGI_COLOR_SPACE_TYPE,
        OutputFormat: DXGI_FORMAT,
        OutputColorSpace: DXGI_COLOR_SPACE_TYPE,
        pSupported: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa04bfb29, 0x08ef, 0x43d6, 0xa4, 0x9c, 0xa9, 0xbd, 0xbd, 0xcb, 0xe6, 0x86)]
interface ID3D11Device1(ID3D11Device1Vtbl): ID3D11Device(ID3D11DeviceVtbl) {
    fn GetImmediateContext1(
        ppImmediateContext: *mut *mut ID3D11DeviceContext1,
    ) -> (),
    fn CreateDeferredContext1(
        ContextFlags: UINT,
        ppDeferredContext: *mut *mut ID3D11DeviceContext1,
    ) -> HRESULT,
    fn CreateBlendState(
        pBlendStateDesc: *const D3D11_BLEND_DESC1,
        ppBlendState: *mut *mut ID3D11BlendState1,
    ) -> HRESULT,
    fn CreateRasterizerState(
        pRasterizerDesc: *const D3D11_RASTERIZER_DESC1,
        ppRasterizerState: *mut *mut ID3D11RasterizerState1,
    ) -> HRESULT,
    fn CreateDeviceContextState(
        Flags: UINT,
        pFeatureLevels: *const D3D_FEATURE_LEVEL,
        FeatureLevels: UINT,
        SDKVersion: UINT,
        EmulatedInterface: REFIID,
        pChosenFeatureLevel: *mut D3D_FEATURE_LEVEL,
        ppContextState: *mut *mut ID3DDeviceContextState,
    ) -> HRESULT,
    fn OpenSharedResource1(
        hResource: HANDLE,
        returnedInterface: REFIID,
        ppResource: *mut *mut c_void,
    ) -> HRESULT,
    fn OpenSharedResourceByName(
        Name: LPCWSTR,
        dwDesiredAccess: DWORD,
        returnedInterface: REFIID,
        ppResource: *mut *mut c_void,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb2daad8b, 0x03d4, 0x4dbf, 0x95, 0xeb, 0x32, 0xab, 0x4b, 0x63, 0xd0, 0xab)]
interface ID3DUserDefinedAnnotation(ID3DUserDefinedAnnotationVtbl): IUnknown(IUnknownVtbl) {
    fn BeginEvent(
        Name: LPCWSTR,
    ) -> INT,
    fn EndEvent() -> INT,
    fn SetMarker(
        Name: LPCWSTR,
    ) -> (),
    fn GetStatus() -> BOOL,
}}
