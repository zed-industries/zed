// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::basetsd::UINT32;
use shared::dxgiformat::DXGI_FORMAT;
use shared::guiddef::{GUID, REFCLSID, REFGUID};
use shared::minwindef::{BOOL, BYTE, FLOAT};
use shared::ntdef::{HRESULT, PCSTR, PCWSTR};
use um::d2d1::D2D1_EXTEND_MODE;
use um::d2d1_1::{
    D2D1_BUFFER_PRECISION, D2D1_COLOR_SPACE, ID2D1Bitmap1, ID2D1ColorContext, ID2D1Effect,
};
use um::d2dbasetypes::{D2D_POINT_2L, D2D_POINT_2U, D2D_RECT_L};
use um::d3dcommon::D3D_FEATURE_LEVEL;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wincodec::IWICColorContext;
FN!{stdcall PD2D1_PROPERTY_SET_FUNCTION(
    effect: *const IUnknown,
    data: *const BYTE,
    dataSize: UINT32,
) -> HRESULT}
FN!{stdcall PD2D1_PROPERTY_GET_FUNCTION(
    effect: *const IUnknown,
    data: *mut BYTE,
    dataSize: UINT32,
    actualSize: *mut UINT32,
) -> HRESULT}
ENUM!{enum D2D1_CHANGE_TYPE {
    D2D1_CHANGE_TYPE_NONE = 0,
    D2D1_CHANGE_TYPE_PROPERTIES = 1,
    D2D1_CHANGE_TYPE_CONTEXT = 2,
    D2D1_CHANGE_TYPE_GRAPH = 3,
}}
ENUM!{enum D2D1_PIXEL_OPTIONS {
    D2D1_PIXEL_OPTIONS_NONE = 0,
    D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING = 1,
}}
ENUM!{enum D2D1_VERTEX_OPTIONS {
    D2D1_VERTEX_OPTIONS_NONE = 0,
    D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR = 1,
    D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER = 2,
    D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP = 4,
}}
ENUM!{enum D2D1_VERTEX_USAGE {
    D2D1_VERTEX_USAGE_STATIC = 0,
    D2D1_VERTEX_USAGE_DYNAMIC = 1,
}}
ENUM!{enum D2D1_BLEND_OPERATION {
    D2D1_BLEND_OPERATION_ADD = 1,
    D2D1_BLEND_OPERATION_SUBTRACT = 2,
    D2D1_BLEND_OPERATION_REV_SUBTRACT = 3,
    D2D1_BLEND_OPERATION_MIN = 4,
    D2D1_BLEND_OPERATION_MAX = 5,
}}
ENUM!{enum D2D1_BLEND {
    D2D1_BLEND_ZERO = 1,
    D2D1_BLEND_ONE = 2,
    D2D1_BLEND_SRC_COLOR = 3,
    D2D1_BLEND_INV_SRC_COLOR = 4,
    D2D1_BLEND_SRC_ALPHA = 5,
    D2D1_BLEND_INV_SRC_ALPHA = 6,
    D2D1_BLEND_DEST_ALPHA = 7,
    D2D1_BLEND_INV_DEST_ALPHA = 8,
    D2D1_BLEND_DEST_COLOR = 9,
    D2D1_BLEND_INV_DEST_COLOR = 10,
    D2D1_BLEND_SRC_ALPHA_SAT = 11,
    D2D1_BLEND_BLEND_FACTOR = 14,
    D2D1_BLEND_INV_BLEND_FACTOR = 15,
}}
ENUM!{enum D2D1_CHANNEL_DEPTH {
    D2D1_CHANNEL_DEPTH_DEFAULT = 0,
    D2D1_CHANNEL_DEPTH_1 = 1,
    D2D1_CHANNEL_DEPTH_4 = 4,
}}
ENUM!{enum D2D1_FILTER {
    D2D1_FILTER_MIN_MAG_MIP_POINT = 0x00,
    D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR = 0x01,
    D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT = 0x04,
    D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR = 0x05,
    D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT = 0x10,
    D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR = 0x11,
    D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT = 0x14,
    D2D1_FILTER_MIN_MAG_MIP_LINEAR = 0x15,
    D2D1_FILTER_ANISOTROPIC = 0x55,
}}
ENUM!{enum D2D1_FEATURE {
    D2D1_FEATURE_DOUBLES = 0,
    D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS = 1,
}}
STRUCT!{struct D2D1_PROPERTY_BINDING {
    propertyName: PCWSTR,
    setFunction: PD2D1_PROPERTY_SET_FUNCTION,
    getFunction: PD2D1_PROPERTY_GET_FUNCTION,
}}
STRUCT!{struct D2D1_RESOURCE_TEXTURE_PROPERTIES {
    extents: *const UINT32,
    dimensions: UINT32,
    bufferPrecision: D2D1_BUFFER_PRECISION,
    channelDepth: D2D1_CHANNEL_DEPTH,
    filter: D2D1_FILTER,
    extendModes: *const D2D1_EXTEND_MODE,
}}
STRUCT!{struct D2D1_INPUT_ELEMENT_DESC {
    semanticName: PCSTR,
    semanticIndex: UINT32,
    format: DXGI_FORMAT,
    inputSlot: UINT32,
    alignedByteOffset: UINT32,
}}
pub const D2D1_APPEND_ALIGNED_ELEMENT: UINT32 = 0xffffffff;
STRUCT!{struct D2D1_VERTEX_BUFFER_PROPERTIES {
    inputCount: UINT32,
    usage: D2D1_VERTEX_USAGE,
    data: *const BYTE,
    byteWidth: UINT32,
}}
STRUCT!{struct D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    shaderBufferWithInputSignature: *const BYTE,
    shaderBufferSize: UINT32,
    inputElements: *const D2D1_INPUT_ELEMENT_DESC,
    elementCount: UINT32,
    stride: UINT32,
}}
STRUCT!{struct D2D1_VERTEX_RANGE {
    startVertex: UINT32,
    vertexCount: UINT32,
}}
STRUCT!{struct D2D1_BLEND_DESCRIPTION {
    sourceBlend: D2D1_BLEND,
    destinationBlend: D2D1_BLEND,
    blendOperation: D2D1_BLEND_OPERATION,
    sourceBlendAlpha: D2D1_BLEND,
    destinationBlendAlpha: D2D1_BLEND,
    blendOperationAlpha: D2D1_BLEND_OPERATION,
    blendFactor: [FLOAT; 4],
}}
STRUCT!{struct D2D1_INPUT_DESCRIPTION {
    filter: D2D1_FILTER,
    leveOfDetailCount: UINT32,
}}
STRUCT!{struct D2D1_FEATURE_DATA_DOUBLES {
    doublePrecisionFloatShaderOps: BOOL,
}}
STRUCT!{struct D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: BOOL,
}}
DEFINE_GUID!{IID_ID2D1VertexBuffer,
    0x9b8b1336, 0x00a5, 0x4668, 0x92, 0xb7, 0xce, 0xd5, 0xd8, 0xbf, 0x9b, 0x7b}
DEFINE_GUID!{IID_ID2D1ResourceTexture,
    0x688d15c3, 0x02b0, 0x438d, 0xb1, 0x3a, 0xd1, 0xb4, 0x4c, 0x32, 0xc3, 0x9a}
DEFINE_GUID!{IID_ID2D1RenderInfo,
    0x519ae1bd, 0xd19a, 0x420d, 0xb8, 0x49, 0x36, 0x4f, 0x59, 0x47, 0x76, 0xb7}
DEFINE_GUID!{IID_ID2D1DrawInfo,
    0x693ce632, 0x7f2f, 0x45de, 0x93, 0xfe, 0x18, 0xd8, 0x8b, 0x37, 0xaa, 0x21}
DEFINE_GUID!{IID_ID2D1ComputeInfo,
    0x5598b14b, 0x9fd7, 0x48b7, 0x9b, 0xdb, 0x8f, 0x09, 0x64, 0xeb, 0x38, 0xbc}
DEFINE_GUID!{IID_ID2D1TransformNode,
    0xb2efe1e7, 0x729f, 0x4102, 0x94, 0x9f, 0x50, 0x5f, 0xa2, 0x1b, 0xf6, 0x66}
DEFINE_GUID!{IID_ID2D1TransformGraph,
    0x13d29038, 0xc3e6, 0x4034, 0x90, 0x81, 0x13, 0xb5, 0x3a, 0x41, 0x79, 0x92}
DEFINE_GUID!{IID_ID2D1Transform,
    0xef1a287d, 0x342a, 0x4f76, 0x8f, 0xdb, 0xda, 0x0d, 0x6e, 0xa9, 0xf9, 0x2b}
DEFINE_GUID!{IID_ID2D1DrawTransform,
    0x36bfdcb6, 0x9739, 0x435d, 0xa3, 0x0d, 0xa6, 0x53, 0xbe, 0xff, 0x6a, 0x6f}
DEFINE_GUID!{IID_ID2D1ComputeTransform,
    0x0d85573c, 0x01e3, 0x4f7d, 0xbf, 0xd9, 0x0d, 0x60, 0x60, 0x8b, 0xf3, 0xc3}
DEFINE_GUID!{IID_ID2D1AnalysisTransform,
    0x0359dc30, 0x95e6, 0x4568, 0x90, 0x55, 0x27, 0x72, 0x0d, 0x13, 0x0e, 0x93}
DEFINE_GUID!{IID_ID2D1SourceTransform,
    0xdb1800dd, 0x0c34, 0x4cf9, 0xbe, 0x90, 0x31, 0xcc, 0x0a, 0x56, 0x53, 0xe1}
DEFINE_GUID!{IID_ID2D1ConcreteTransform,
    0x1a799d8a, 0x69f7, 0x4e4c, 0x9f, 0xed, 0x43, 0x7c, 0xcc, 0x66, 0x84, 0xcc}
DEFINE_GUID!{IID_ID2D1BlendTransform,
    0x63ac0b32, 0xba44, 0x450f, 0x88, 0x06, 0x7f, 0x4c, 0xa1, 0xff, 0x2f, 0x1b}
DEFINE_GUID!{IID_ID2D1BorderTransform,
    0x4998735c, 0x3a19, 0x473c, 0x97, 0x81, 0x65, 0x68, 0x47, 0xe3, 0xa3, 0x47}
DEFINE_GUID!{IID_ID2D1OffsetTransform,
    0x3fe6adea, 0x7643, 0x4f53, 0xbd, 0x14, 0xa0, 0xce, 0x63, 0xf2, 0x40, 0x42}
DEFINE_GUID!{IID_ID2D1BoundsAdjustmentTransform,
    0x90f732e2, 0x5092, 0x4606, 0xa8, 0x19, 0x86, 0x51, 0x97, 0x0b, 0xac, 0xcd}
DEFINE_GUID!{IID_ID2D1EffectImpl,
    0xa248fd3f, 0x3e6c, 0x4e63, 0x9f, 0x03, 0x7f, 0x68, 0xec, 0xc9, 0x1d, 0xb9}
DEFINE_GUID!{IID_ID2D1EffectContext,
    0x3d9f916b, 0x27dc, 0x4ad7, 0xb4, 0xf1, 0x64, 0x94, 0x53, 0x40, 0xf5, 0x63}
RIDL!{#[uuid(0x9b8b1336, 0x00a5, 0x4668, 0x92, 0xb7, 0xce, 0xd5, 0xd8, 0xbf, 0x9b, 0x7b)]
interface ID2D1VertexBuffer(ID2D1VertexBufferVtbl): IUnknown(IUnknownVtbl) {
    fn Map(
        data: *mut *mut BYTE,
    ) -> HRESULT,
    fn Unmap() -> HRESULT,
}}
RIDL!{#[uuid(0x688d15c3, 0x02b0, 0x438d, 0xb1, 0x3a, 0xd1, 0xb4, 0x4c, 0x32, 0xc3, 0x9a)]
interface ID2D1ResourceTexture(ID2D1ResourceTextureVtbl): IUnknown(IUnknownVtbl) {
    fn Update(
        minimumExtents: *const UINT32,
        maximumExtents: *const UINT32,
        strides: *const UINT32,
        dimensions: UINT32,
        data: *const BYTE,
        dataCount: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x519ae1bd, 0xd19a, 0x420d, 0xb8, 0x49, 0x36, 0x4f, 0x59, 0x47, 0x76, 0xb7)]
interface ID2D1RenderInfo(ID2D1RenderInfoVtbl): IUnknown(IUnknownVtbl) {
    fn SetInputDescription(
        inputIndex: UINT32,
        inputDescription: D2D1_INPUT_DESCRIPTION,
    ) -> HRESULT,
    fn SetOutputBuffer(
        bufferPrecision: D2D1_BUFFER_PRECISION,
        channelDepth: D2D1_CHANNEL_DEPTH,
    ) -> HRESULT,
    fn SetCached(
        isCached: BOOL,
    ) -> (),
    fn SetInstructionCountHint(
        instructionCount: UINT32,
    ) -> (),
}}
RIDL!{#[uuid(0x693ce632, 0x7f2f, 0x45de, 0x93, 0xfe, 0x18, 0xd8, 0x8b, 0x37, 0xaa, 0x21)]
interface ID2D1DrawInfo(ID2D1DrawInfoVtbl): ID2D1RenderInfo(ID2D1RenderInfoVtbl) {
    fn SetPixelShaderConstantBuffer(
        buffer: *const BYTE,
        bufferCount: UINT32,
    ) -> HRESULT,
    fn SetResourceTexture(
        textureIndex: UINT32,
        resourceTexture: *mut ID2D1ResourceTexture,
    ) -> HRESULT,
    fn SetVertexShaderConstantBuffer(
        buffer: *const BYTE,
        bufferCount: UINT32,
    ) -> HRESULT,
    fn SetPixelShader(
        shaderId: REFGUID,
        pixelOptions: D2D1_PIXEL_OPTIONS,
    ) -> HRESULT,
    fn SetVertexProcessing(
        vertexBuffer: *mut ID2D1VertexBuffer,
        vertexOptions: D2D1_VERTEX_OPTIONS,
        blendDescription: *const D2D1_BLEND_DESCRIPTION,
        vertexRange: *const D2D1_VERTEX_RANGE,
        vertexShader: *const GUID,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x5598b14b, 0x9fd7, 0x48b7, 0x9b, 0xdb, 0x8f, 0x09, 0x64, 0xeb, 0x38, 0xbc)]
interface ID2D1ComputeInfo(ID2D1ComputeInfoVtbl): ID2D1RenderInfo(ID2D1RenderInfoVtbl) {
    fn SetComputeShaderConstantBuffer(
        buffer: *const BYTE,
        bufferCount: UINT32,
    ) -> HRESULT,
    fn SetComputeShader(
        shaderId: REFGUID,
    ) -> HRESULT,
    fn SetResourceTexture(
        textureIndex: UINT32,
        resourceTexture: *mut ID2D1ResourceTexture,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb2efe1e7, 0x729f, 0x4102, 0x94, 0x9f, 0x50, 0x5f, 0xa2, 0x1b, 0xf6, 0x66)]
interface ID2D1TransformNode(ID2D1TransformNodeVtbl): IUnknown(IUnknownVtbl) {
    fn GetInputCount() -> UINT32,
}}
RIDL!{#[uuid(0x13d29038, 0xc3e6, 0x4034, 0x90, 0x81, 0x13, 0xb5, 0x3a, 0x41, 0x79, 0x92)]
interface ID2D1TransformGraph(ID2D1TransformGraphVtbl): IUnknown(IUnknownVtbl) {
    fn GetInputCount() -> UINT32,
    fn SetSingleTransformNode(
        node: *mut ID2D1TransformNode,
    ) -> HRESULT,
    fn AddNode(
        node: *mut ID2D1TransformNode,
    ) -> HRESULT,
    fn RemoveNode(
        node: *mut ID2D1TransformNode,
    ) -> HRESULT,
    fn SetOutputNode(
        node: *mut ID2D1TransformNode,
    ) -> HRESULT,
    fn ConnectNode(
        fromNode: *mut ID2D1TransformNode,
        toNode: *mut ID2D1TransformNode,
        toNodeInputIndex: UINT32,
    ) -> HRESULT,
    fn ConnectToEffectInput(
        toEffectInputIndex: UINT32,
        node: *mut ID2D1TransformNode,
        toNodeInputIndex: UINT32,
    ) -> HRESULT,
    fn Clear() -> (),
    fn SetPassthroughGraph(
        effectInputIndex: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xef1a287d, 0x342a, 0x4f76, 0x8f, 0xdb, 0xda, 0x0d, 0x6e, 0xa9, 0xf9, 0x2b)]
interface ID2D1Transform(ID2D1TransformVtbl): ID2D1TransformNode(ID2D1TransformNodeVtbl) {
    fn MapOutputRectToInputRects(
        outputRect: *const D2D_RECT_L,
        inputRects: *mut D2D_RECT_L,
        inputRectsCount: UINT32,
    ) -> HRESULT,
    fn MapInputRectsToOutputRect(
        inputRects: *const D2D_RECT_L,
        inputOpaqueSubRects: *const D2D_RECT_L,
        inputRectCount: UINT32,
        outputRect: *mut D2D_RECT_L,
        outputOpaqueSubRect: *mut D2D_RECT_L,
    ) -> HRESULT,
    fn MapInvalidRect(
        inputIndex: UINT32,
        invalidInputRect: D2D_RECT_L,
        invalidOutputRect: *mut D2D_RECT_L,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x36bfdcb6, 0x9739, 0x435d, 0xa3, 0x0d, 0xa6, 0x53, 0xbe, 0xff, 0x6a, 0x6f)]
interface ID2D1DrawTransform(ID2D1DrawTransformVtbl): ID2D1Transform(ID2D1TransformVtbl) {
    fn SetDrawInfo(
        drawInfo: *mut ID2D1DrawInfo,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0d85573c, 0x01e3, 0x4f7d, 0xbf, 0xd9, 0x0d, 0x60, 0x60, 0x8b, 0xf3, 0xc3)]
interface ID2D1ComputeTransform(ID2D1ComputeTransformVtbl): ID2D1Transform(ID2D1TransformVtbl) {
    fn SetComputeInfo(
        computeInfo: *mut ID2D1ComputeInfo,
    ) -> HRESULT,
    fn CalculateThreadgroups(
        outputRect: *const D2D_RECT_L,
        dimensionX: *mut UINT32,
        dimensionY: *mut UINT32,
        dimensionZ: *mut UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0359dc30, 0x95e6, 0x4568, 0x90, 0x55, 0x27, 0x72, 0x0d, 0x13, 0x0e, 0x93)]
interface ID2D1AnalysisTransform(ID2D1AnalysisTransformVtbl): IUnknown(IUnknownVtbl) {
    fn ProcessAnalysisResults(
        analysisData: *const BYTE,
        analysisDataCount: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xdb1800dd, 0x0c34, 0x4cf9, 0xbe, 0x90, 0x31, 0xcc, 0x0a, 0x56, 0x53, 0xe1)]
interface ID2D1SourceTransform(ID2D1SourceTransformVtbl): ID2D1Transform(ID2D1TransformVtbl) {
    fn SetRenderInfo(
        renderInfo: *mut ID2D1RenderInfo,
    ) -> HRESULT,
    fn Draw(
        target: *mut ID2D1Bitmap1,
        drawRect: *mut D2D_RECT_L,
        targetOrigin: D2D_POINT_2U,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1a799d8a, 0x69f7, 0x4e4c, 0x9f, 0xed, 0x43, 0x7c, 0xcc, 0x66, 0x84, 0xcc)]
interface ID2D1ConcreteTransform(ID2D1ConcreteTransformVtbl):
    ID2D1TransformNode(ID2D1TransformNodeVtbl) {
    fn SetOutputBuffer(
        bufferPrecision: D2D1_BUFFER_PRECISION,
        channelDepth: D2D1_CHANNEL_DEPTH,
    ) -> HRESULT,
    fn SetCached(
        isCached: BOOL,
    ) -> (),
}}
RIDL!{#[uuid(0x63ac0b32, 0xba44, 0x450f, 0x88, 0x06, 0x7f, 0x4c, 0xa1, 0xff, 0x2f, 0x1b)]
interface ID2D1BlendTransform(ID2D1BlendTransformVtbl):
    ID2D1ConcreteTransform(ID2D1ConcreteTransformVtbl) {
    fn SetDescription(
        description: *const D2D1_BLEND_DESCRIPTION,
    ) -> (),
    fn GetDescription(
        description: *mut D2D1_BLEND_DESCRIPTION,
    ) -> (),
}}
RIDL!{#[uuid(0x4998735c, 0x3a19, 0x473c, 0x97, 0x81, 0x65, 0x68, 0x47, 0xe3, 0xa3, 0x47)]
interface ID2D1BorderTransform(ID2D1BorderTransformVtbl):
    ID2D1ConcreteTransform(ID2D1ConcreteTransformVtbl) {
    fn SetExtendModeX(
        extendMode: D2D1_EXTEND_MODE,
    ) -> (),
    fn SetExtendModeY(
        extendMode: D2D1_EXTEND_MODE,
    ) -> (),
    fn GetExtendModeX() -> D2D1_EXTEND_MODE,
    fn GetExtendModeY() -> D2D1_EXTEND_MODE,
}}
RIDL!{#[uuid(0x3fe6adea, 0x7643, 0x4f53, 0xbd, 0x14, 0xa0, 0xce, 0x63, 0xf2, 0x40, 0x42)]
interface ID2D1OffsetTransform(ID2D1OffsetTransformVtbl):
    ID2D1TransformNode(ID2D1TransformNodeVtbl) {
    fn SetOffset(
        offset: D2D_POINT_2L,
    ) -> (),
    fn GetOffset() -> D2D_POINT_2L,
}}
RIDL!{#[uuid(0x90f732e2, 0x5092, 0x4606, 0xa8, 0x19, 0x86, 0x51, 0x97, 0x0b, 0xac, 0xcd)]
interface ID2D1BoundsAdjustmentTransform(ID2D1BoundsAdjustmentTransformVtbl):
    ID2D1TransformNode(ID2D1TransformNodeVtbl) {
    fn SetOutputBounds(
        outputBounds: *const D2D_RECT_L,
    ) -> (),
    fn GetOutputBounds(
        outputBounds: *mut D2D_RECT_L,
    ) -> (),
}}
RIDL!{#[uuid(0xa248fd3f, 0x3e6c, 0x4e63, 0x9f, 0x03, 0x7f, 0x68, 0xec, 0xc9, 0x1d, 0xb9)]
interface ID2D1EffectImpl(ID2D1EffectImplVtbl): IUnknown(IUnknownVtbl) {
    fn Initialize(
        effectContext: *mut ID2D1EffectContext,
        transformGraph: *mut ID2D1TransformGraph,
    ) -> HRESULT,
    fn PrepareForRender(
        changeType: D2D1_CHANGE_TYPE,
    ) -> HRESULT,
    fn SetGraph(
        transformGraph: *mut ID2D1TransformGraph,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3d9f916b, 0x27dc, 0x4ad7, 0xb4, 0xf1, 0x64, 0x94, 0x53, 0x40, 0xf5, 0x63)]
interface ID2D1EffectContext(ID2D1EffectContextVtbl): IUnknown(IUnknownVtbl) {
    fn GetDpi(
        dpiX: *mut FLOAT,
        dpiY: *mut FLOAT,
    ) -> (),
    fn CreateEffect(
        effectId: REFCLSID,
        effect: *mut *mut ID2D1Effect,
    ) -> HRESULT,
    fn GetMaximumSupportedFeatureLevel(
        featureLevels: *const D3D_FEATURE_LEVEL,
        featureLevelsCount: UINT32,
        maximumSupportedFeatureLevel: *mut D3D_FEATURE_LEVEL,
    ) -> HRESULT,
    fn CreateTransformNodeFromEffect(
        effect: *mut ID2D1Effect,
        transformNode: *mut *mut ID2D1TransformNode,
    ) -> HRESULT,
    fn CreateBlendTransform(
        numInputs: UINT32,
        blendDescription: D2D1_BLEND_DESCRIPTION,
        transform: *mut *mut ID2D1BlendTransform,
    ) -> HRESULT,
    fn CreateBorderTransform(
        extendModeX: D2D1_EXTEND_MODE,
        extendModeY: D2D1_EXTEND_MODE,
        transform: *mut *mut ID2D1BorderTransform,
    ) -> HRESULT,
    fn CreateOffsetTransform(
        offset: D2D_POINT_2L,
        transform: *mut *mut ID2D1OffsetTransform,
    ) -> HRESULT,
    fn CreateBoundsAdjustmentTransform(
        outputRectangle: *mut D2D_RECT_L,
        transform: ID2D1BoundsAdjustmentTransform,
    ) -> HRESULT,
    fn LoadPixelShader(
        shaderId: REFGUID,
        shaderBuffer: *const BYTE,
        shaderBufferCount: UINT32,
    ) -> HRESULT,
    fn LoadVertexShader(
        resourceId: REFGUID,
        shaderBuffer: *const BYTE,
        shaderBufferCount: UINT32,
    ) -> HRESULT,
    fn LoadComputeShader(
        resourceId: REFGUID,
        shaderBuffer: *const BYTE,
        shaderBufferCount: UINT32,
    ) -> HRESULT,
    fn IsShaderLoaded(
        shaderId: REFGUID,
    ) -> BOOL,
    fn CreateResourceTexture(
        resourceId: *const GUID,
        resourceTextureProperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES,
        data: *const BYTE,
        strides: *const UINT32,
        dataSize: UINT32,
        resourceTexture: *mut *mut ID2D1ResourceTexture,
    ) -> HRESULT,
    fn FindResourceTexture(
        resourceId: *const GUID,
        resourceTexture: *mut *mut ID2D1ResourceTexture,
    ) -> HRESULT,
    fn CreateVertexBuffer(
        vertexBufferProperties: *const D2D1_VERTEX_BUFFER_PROPERTIES,
        resourceId: *const GUID,
        customVertexBufferProperties: *const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES,
        buffer: *mut *mut ID2D1VertexBuffer,
    ) -> HRESULT,
    fn FindVertexBuffer(
        resourceId: *const GUID,
        buffer: *mut *mut ID2D1VertexBuffer,
    ) -> HRESULT,
    fn CreateColorContext(
        space: D2D1_COLOR_SPACE,
        profile: *const BYTE,
        profileSize: UINT32,
        colorContext: *mut *mut ID2D1ColorContext,
    ) -> HRESULT,
    fn CreateColorContextFromFilename(
        filename: PCWSTR,
        colorContext: *mut *mut ID2D1ColorContext,
    ) -> HRESULT,
    fn CreateColorContextFromWicColorContext(
        wicColorContext: *mut IWICColorContext,
        colorContext: *mut *mut ID2D1ColorContext,
    ) -> HRESULT,
    fn CheckFeatureSupport(
        feature: D2D1_FEATURE,
        featureSupportData: *mut c_void,
        featureSupportDataSize: UINT32,
    ) -> HRESULT,
    fn IsBufferPrecisionSupported(
        bufferPrecision: D2D1_BUFFER_PRECISION,
    ) -> BOOL,
}}
