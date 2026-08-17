// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of d2d1_1.h
use ctypes::c_void;
use shared::basetsd::{UINT32, UINT64};
use shared::dxgi::{IDXGIDevice, IDXGISurface};
use shared::dxgiformat::DXGI_FORMAT;
use shared::guiddef::{CLSID, REFCLSID};
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT};
use um::d2d1::{
    D2D1_ANTIALIAS_MODE, D2D1_BRUSH_PROPERTIES, D2D1_CAP_STYLE, D2D1_COLOR_F,
    D2D1_DASH_STYLE, D2D1_DEBUG_LEVEL, D2D1_EXTEND_MODE, D2D1_GRADIENT_STOP,
    D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC, D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC,
    D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC,
    D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR,
    D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR,
    D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR, D2D1_LINE_JOIN, D2D1_MATRIX_3X2_F,
    D2D1_POINT_2F, D2D1_RECT_F, D2D1_SIZE_U, D2D1_TAG, D2D1_TEXT_ANTIALIAS_MODE, ID2D1Bitmap,
    ID2D1BitmapBrush, ID2D1BitmapBrushVtbl, ID2D1BitmapVtbl, ID2D1Brush, ID2D1BrushVtbl,
    ID2D1DrawingStateBlock, ID2D1DrawingStateBlockVtbl, ID2D1Factory, ID2D1FactoryVtbl,
    ID2D1Geometry, ID2D1GradientStopCollection, ID2D1GradientStopCollectionVtbl, ID2D1Image,
    ID2D1ImageVtbl, ID2D1Layer, ID2D1Mesh, ID2D1PathGeometry, ID2D1PathGeometryVtbl,
    ID2D1RenderTarget, ID2D1RenderTargetVtbl, ID2D1Resource, ID2D1ResourceVtbl, ID2D1StrokeStyle,
    ID2D1StrokeStyleVtbl,
};
use um::d2d1effectauthor::D2D1_PROPERTY_BINDING;
use um::d2dbasetypes::D2D_SIZE_F;
use um::dcommon::{D2D1_PIXEL_FORMAT, DWRITE_MEASURING_MODE};
use um::documenttarget::IPrintDocumentPackageTarget;
use um::dwrite::{DWRITE_GLYPH_RUN, DWRITE_GLYPH_RUN_DESCRIPTION, IDWriteRenderingParams};
use um::objidlbase::IStream;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wincodec::{IWICBitmapSource, IWICColorContext, IWICImagingFactory};
use um::winnt::{HRESULT, PCWSTR, PWSTR};
FN!{stdcall PD2D1_EFFECT_FACTORY(
    effectImpl: *mut *mut IUnknown,
) -> HRESULT}
pub use um::d2dbasetypes::D2D_RECT_L as D2D1_RECT_L;
pub use um::d2dbasetypes::D2D_POINT_2L as D2D1_POINT_2L;
ENUM!{enum D2D1_PROPERTY_TYPE {
    D2D1_PROPERTY_TYPE_UNKNOWN = 0,
    D2D1_PROPERTY_TYPE_STRING = 1,
    D2D1_PROPERTY_TYPE_BOOL = 2,
    D2D1_PROPERTY_TYPE_UINT32 = 3,
    D2D1_PROPERTY_TYPE_INT32 = 4,
    D2D1_PROPERTY_TYPE_FLOAT = 5,
    D2D1_PROPERTY_TYPE_VECTOR2 = 6,
    D2D1_PROPERTY_TYPE_VECTOR3 = 7,
    D2D1_PROPERTY_TYPE_VECTOR4 = 8,
    D2D1_PROPERTY_TYPE_BLOB = 9,
    D2D1_PROPERTY_TYPE_IUNKNOWN = 10,
    D2D1_PROPERTY_TYPE_ENUM = 11,
    D2D1_PROPERTY_TYPE_ARRAY = 12,
    D2D1_PROPERTY_TYPE_CLSID = 13,
    D2D1_PROPERTY_TYPE_MATRIX_3X2 = 14,
    D2D1_PROPERTY_TYPE_MATRIX_4X3 = 15,
    D2D1_PROPERTY_TYPE_MATRIX_4X4 = 16,
    D2D1_PROPERTY_TYPE_MATRIX_5X4 = 17,
    D2D1_PROPERTY_TYPE_COLOR_CONTEXT = 18,
    D2D1_PROPERTY_TYPE_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_PROPERTY {
    D2D1_PROPERTY_CLSID = 0x80000000,
    D2D1_PROPERTY_DISPLAYNAME = 0x80000001,
    D2D1_PROPERTY_AUTHOR = 0x80000002,
    D2D1_PROPERTY_CATEGORY = 0x80000003,
    D2D1_PROPERTY_DESCRIPTION = 0x80000004,
    D2D1_PROPERTY_INPUTS = 0x80000005,
    D2D1_PROPERTY_CACHED = 0x80000006,
    D2D1_PROPERTY_PRECISION = 0x80000007,
    D2D1_PROPERTY_MIN_INPUTS = 0x80000008,
    D2D1_PROPERTY_MAX_INPUTS = 0x80000009,
    D2D1_PROPERTY_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_SUBPROPERTY {
    D2D1_SUBPROPERTY_DISPLAYNAME = 0x80000000,
    D2D1_SUBPROPERTY_ISREADONLY = 0x80000001,
    D2D1_SUBPROPERTY_MIN = 0x80000002,
    D2D1_SUBPROPERTY_MAX = 0x80000003,
    D2D1_SUBPROPERTY_DEFAULT = 0x80000004,
    D2D1_SUBPROPERTY_FIELDS = 0x80000005,
    D2D1_SUBPROPERTY_INDEX = 0x80000006,
    D2D1_SUBPROPERTY_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_BITMAP_OPTIONS {
    D2D1_BITMAP_OPTIONS_NONE = 0x00000000,
    D2D1_BITMAP_OPTIONS_TARGET = 0x00000001,
    D2D1_BITMAP_OPTIONS_CANNOT_DRAW = 0x00000002,
    D2D1_BITMAP_OPTIONS_CPU_READ = 0x00000004,
    D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE = 0x00000008,
    D2D1_BITMAP_OPTIONS_FORCE_DWORD = 0xffffffff,
}}
// DEFINE_ENUM_FLAG_OPERATORS(D2D1_BITMAP_OPTIONS);
ENUM!{enum D2D1_COMPOSITE_MODE {
    D2D1_COMPOSITE_MODE_SOURCE_OVER = 0,
    D2D1_COMPOSITE_MODE_DESTINATION_OVER = 1,
    D2D1_COMPOSITE_MODE_SOURCE_IN = 2,
    D2D1_COMPOSITE_MODE_DESTINATION_IN = 3,
    D2D1_COMPOSITE_MODE_SOURCE_OUT = 4,
    D2D1_COMPOSITE_MODE_DESTINATION_OUT = 5,
    D2D1_COMPOSITE_MODE_SOURCE_ATOP = 6,
    D2D1_COMPOSITE_MODE_DESTINATION_ATOP = 7,
    D2D1_COMPOSITE_MODE_XOR = 8,
    D2D1_COMPOSITE_MODE_PLUS = 9,
    D2D1_COMPOSITE_MODE_SOURCE_COPY = 10,
    D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY = 11,
    D2D1_COMPOSITE_MODE_MASK_INVERT = 12,
    D2D1_COMPOSITE_MODE_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_BUFFER_PRECISION {
    D2D1_BUFFER_PRECISION_UNKNOWN = 0,
    D2D1_BUFFER_PRECISION_8BPC_UNORM = 1,
    D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB = 2,
    D2D1_BUFFER_PRECISION_16BPC_UNORM = 3,
    D2D1_BUFFER_PRECISION_16BPC_FLOAT = 4,
    D2D1_BUFFER_PRECISION_32BPC_FLOAT = 5,
    D2D1_BUFFER_PRECISION_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_MAP_OPTIONS {
    D2D1_MAP_OPTIONS_NONE = 0,
    D2D1_MAP_OPTIONS_READ = 1,
    D2D1_MAP_OPTIONS_WRITE = 2,
    D2D1_MAP_OPTIONS_DISCARD = 4,
    D2D1_MAP_OPTIONS_FORCE_DWORD = 0xffffffff,
}}
//DEFINE_ENUM_FLAG_OPERATORS(D2D1_MAP_OPTIONS);
ENUM!{enum D2D1_INTERPOLATION_MODE {
    D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR = D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR,
    D2D1_INTERPOLATION_MODE_LINEAR = D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR,
    D2D1_INTERPOLATION_MODE_CUBIC = D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC,
    D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR
        = D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR,
    D2D1_INTERPOLATION_MODE_ANISOTROPIC = D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC,
    D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC
        = D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC,
    D2D1_INTERPOLATION_MODE_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_UNIT_MODE {
    D2D1_UNIT_MODE_DIPS = 0,
    D2D1_UNIT_MODE_PIXELS = 1,
    D2D1_UNIT_MODE_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_COLOR_SPACE {
    D2D1_COLOR_SPACE_CUSTOM = 0,
    D2D1_COLOR_SPACE_SRGB = 1,
    D2D1_COLOR_SPACE_SCRGB = 2,
    D2D1_COLOR_SPACE_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_DEVICE_CONTEXT_OPTIONS {
    D2D1_DEVICE_CONTEXT_OPTIONS_NONE = 0,
    D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS = 1,
    D2D1_DEVICE_CONTEXT_OPTIONS_FORCE_DWORD = 0xffffffff,
}}
//DEFINE_ENUM_FLAG_OPERATORS(D2D1_DEVICE_CONTEXT_OPTIONS);
ENUM!{enum D2D1_STROKE_TRANSFORM_TYPE {
    D2D1_STROKE_TRANSFORM_TYPE_NORMAL = 0,
    D2D1_STROKE_TRANSFORM_TYPE_FIXED = 1,
    D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE = 2,
    D2D1_STROKE_TRANSFORM_TYPE_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_PRIMITIVE_BLEND {
    D2D1_PRIMITIVE_BLEND_SOURCE_OVER = 0,
    D2D1_PRIMITIVE_BLEND_COPY = 1,
    D2D1_PRIMITIVE_BLEND_MIN = 2,
    D2D1_PRIMITIVE_BLEND_ADD = 3,
    D2D1_PRIMITIVE_BLEND_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_THREADING_MODE {
    D2D1_THREADING_MODE_SINGLE_THREADED = super::d2d1::D2D1_FACTORY_TYPE_SINGLE_THREADED,
    D2D1_THREADING_MODE_MULTI_THREADED = super::d2d1::D2D1_FACTORY_TYPE_MULTI_THREADED,
    D2D1_THREADING_MODE_FORCE_DWORD = 0xffffffff,
}}
ENUM!{enum D2D1_COLOR_INTERPOLATION_MODE {
    D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT = 0,
    D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED = 1,
    D2D1_COLOR_INTERPOLATION_MODE_FORCE_DWORD = 0xffffffff,
}}
pub use um::d2dbasetypes::D2D_VECTOR_2F as D2D1_VECTOR_2F;
pub use um::d2dbasetypes::D2D_VECTOR_3F as D2D1_VECTOR_3F;
pub use um::d2dbasetypes::D2D_VECTOR_4F as D2D1_VECTOR_4F;
STRUCT!{struct D2D1_BITMAP_PROPERTIES1 {
    pixelFormat: D2D1_PIXEL_FORMAT,
    dpiX: FLOAT,
    dpiY: FLOAT,
    bitmapOptions: D2D1_BITMAP_OPTIONS,
    colorContext: *const ID2D1ColorContext,
}}
STRUCT!{struct D2D1_MAPPED_RECT {
    pitch: UINT32,
    bits: *const BYTE,
}}
STRUCT!{struct D2D1_RENDERING_CONTROLS {
    bufferPrecision: D2D1_BUFFER_PRECISION,
    tileSize: D2D1_SIZE_U,
}}
STRUCT!{struct D2D1_EFFECT_INPUT_DESCRIPTION {
    effect: *const ID2D1Effect,
    inputIndex: UINT32,
    inputRectangle: D2D1_RECT_F,
}}
pub use um::d2dbasetypes::D2D_MATRIX_4X3_F as D2D1_MATRIX_4X3_F;
pub use um::d2dbasetypes::D2D_MATRIX_4X4_F as D2D1_MATRIX_4X4_F;
pub use um::d2dbasetypes::D2D_MATRIX_5X4_F as D2D1_MATRIX_5X4_F;
STRUCT!{struct D2D1_POINT_DESCRIPTION {
    point: D2D1_POINT_2F,
    unitTangentVector: D2D1_POINT_2F,
    endSegment: UINT32,
    endFigure: UINT32,
    lengthToEndSegment: FLOAT,
}}
STRUCT!{struct D2D1_IMAGE_BRUSH_PROPERTIES {
    sourceRectangle: D2D1_RECT_F,
    extendModeX: D2D1_EXTEND_MODE,
    extendModeY: D2D1_EXTEND_MODE,
    interpolationMode: D2D1_INTERPOLATION_MODE,
}}
STRUCT!{struct D2D1_BITMAP_BRUSH_PROPERTIES1 {
    extendModeX: D2D1_EXTEND_MODE,
    extendModeY: D2D1_EXTEND_MODE,
    interpolationMode: D2D1_INTERPOLATION_MODE,
}}
STRUCT!{struct D2D1_STROKE_STYLE_PROPERTIES1 {
    startCap: D2D1_CAP_STYLE,
    endCap: D2D1_CAP_STYLE,
    dashCap: D2D1_CAP_STYLE,
    lineJoin: D2D1_LINE_JOIN,
    miterLimit: FLOAT,
    dashStyle: D2D1_DASH_STYLE,
    dashOffset: FLOAT,
    transformType: D2D1_STROKE_TRANSFORM_TYPE,
}}
ENUM!{enum D2D1_LAYER_OPTIONS1 {
    D2D1_LAYER_OPTIONS1_NONE = 0,
    D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND = 1,
    D2D1_LAYER_OPTIONS1_IGNORE_ALPHA = 2,
    D2D1_LAYER_OPTIONS1_FORCE_DWORD = 0xffffffff,
}}
//DEFINE_ENUM_FLAG_OPERATORS(D2D1_LAYER_OPTIONS1);
STRUCT!{struct D2D1_LAYER_PARAMETERS1 {
    contentBounds: D2D1_RECT_F,
    geometricMask: *const ID2D1Geometry,
    maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    maskTransform: D2D1_MATRIX_3X2_F,
    opacity: FLOAT,
    opacityBrush: *const ID2D1Brush,
    layerOptions: D2D1_LAYER_OPTIONS1,
}}
ENUM!{enum D2D1_PRINT_FONT_SUBSET_MODE {
    D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT = 0,
    D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE = 1,
    D2D1_PRINT_FONT_SUBSET_MODE_NONE = 2,
    D2D1_PRINT_FONT_SUBSET_MODE_FORCE_DWORD = 0xffffffff,
}}
STRUCT!{struct D2D1_DRAWING_STATE_DESCRIPTION1 {
    antialiasMode: D2D1_ANTIALIAS_MODE,
    textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    tag1: D2D1_TAG,
    tag2: D2D1_TAG,
    transform: D2D1_MATRIX_3X2_F,
    primitiveBlend: D2D1_PRIMITIVE_BLEND,
    unitMode: D2D1_UNIT_MODE,
}}
STRUCT!{struct D2D1_PRINT_CONTROL_PROPERTIES {
    fontSubset: D2D1_PRINT_FONT_SUBSET_MODE,
    rasterDPI: FLOAT,
    colorSpace: D2D1_COLOR_SPACE,
}}
STRUCT!{struct D2D1_CREATION_PROPERTIES {
    threadingMode: D2D1_THREADING_MODE,
    debugLevel: D2D1_DEBUG_LEVEL,
    options: D2D1_DEVICE_CONTEXT_OPTIONS,
}}
RIDL!{#[uuid(0x82237326, 0x8111, 0x4f7c, 0xbc, 0xf4, 0xb5, 0xc1, 0x17, 0x55, 0x64, 0xfe)]
interface ID2D1GdiMetafileSink(ID2D1GdiMetafileSinkVtbl): IUnknown(IUnknownVtbl) {
    fn ProcessRecord(
        recordType: DWORD,
        recordData: *const c_void,
        recordDataSize: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2f543dc3, 0xcfc1, 0x4211, 0x86, 0x4f, 0xcf, 0xd9, 0x1c, 0x6f, 0x33, 0x95)]
interface ID2D1GdiMetafile(ID2D1GdiMetafileVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn Stream(
        sink: *const ID2D1GdiMetafileSink,
    ) -> HRESULT,
    fn GetBounds(
        bounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x54d7898a, 0xa061, 0x40a7, 0xbe, 0xc7, 0xe4, 0x65, 0xbc, 0xba, 0x2c, 0x4f)]
interface ID2D1CommandSink(ID2D1CommandSinkVtbl): IUnknown(IUnknownVtbl) {
    fn BeginDraw() -> HRESULT,
    fn EndDraw() -> HRESULT,
    fn SetAntialiasMode(
        antialiasMode: D2D1_ANTIALIAS_MODE,
    ) -> HRESULT,
    fn SetTags(
        tag1: D2D1_TAG,
        tag2: D2D1_TAG,
    ) -> HRESULT,
    fn SetTextAntialiasMode(
        textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    ) -> HRESULT,
    fn SetTextRenderingParams(
        textRenderingParams: *const IDWriteRenderingParams,
    ) -> HRESULT,
    fn SetTransform(
        transform: *const D2D1_MATRIX_3X2_F,
    ) -> HRESULT,
    fn SetPrimitiveBlend(
        primitiveBlend: D2D1_PRIMITIVE_BLEND,
    ) -> HRESULT,
    fn SetUnitMode(
        unitMode: D2D1_UNIT_MODE,
    ) -> HRESULT,
    fn Clear(
        color: *const D2D1_COLOR_F,
    ) -> HRESULT,
    fn DrawGlyphRun(
        baselineOrigin: D2D1_POINT_2F,
        glyphRun: *const DWRITE_GLYPH_RUN,
        glyphRunDescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        foregroundBrush: *const ID2D1Brush,
        measuringMode: DWRITE_MEASURING_MODE,
    ) -> HRESULT,
    fn DrawLine(
        point0: D2D1_POINT_2F,
        point1: D2D1_POINT_2F,
        brush: *const ID2D1Brush,
        strokeWidth: FLOAT,
        strokeStyle: *const ID2D1StrokeStyle,
    ) -> HRESULT,
    fn DrawGeometry(
        geometry: *const ID2D1Geometry,
        brush: *const ID2D1Brush,
        strokeWidth: FLOAT,
        strokeStyle: *const ID2D1StrokeStyle,
    ) -> HRESULT,
    fn DrawRectangle(
        rect: *const D2D1_RECT_F,
        brush: *const ID2D1Brush,
        strokeWidth: FLOAT,
        strokeStyle: *const ID2D1StrokeStyle,
    ) -> HRESULT,
    fn DrawBitmap(
        bitmap: *const ID2D1Bitmap,
        destinationRectangle: *const D2D1_RECT_F,
        opacity: FLOAT,
        interpolationMode: D2D1_INTERPOLATION_MODE,
        sourceRectangle: *const D2D1_RECT_F,
        perspectiveTransform: *const D2D1_MATRIX_4X4_F,
    ) -> HRESULT,
    fn DrawImage(
        image: *const ID2D1Image,
        targetOffset: *const D2D1_POINT_2F,
        imageRectangle: *const D2D1_RECT_F,
        interpolationMode: D2D1_INTERPOLATION_MODE,
        compositeMode: D2D1_COMPOSITE_MODE,
    ) -> HRESULT,
    fn DrawGdiMetafile(
        gdiMetafile: *const ID2D1GdiMetafile,
        targetOffset: *const D2D1_POINT_2F,
    ) -> HRESULT,
    fn FillMesh(
        mesh: *const ID2D1Mesh,
        brush: *const ID2D1Brush,
    ) -> HRESULT,
    fn FillOpacityMask(
        opacityMask: *const ID2D1Bitmap,
        brush: *const ID2D1Brush,
        destinationRectangle: *const D2D1_RECT_F,
        sourceRectangle: *const D2D1_RECT_F,
    ) -> HRESULT,
    fn FillGeometry(
        geometry: *const ID2D1Geometry,
        brush: *const ID2D1Brush,
        opacityBrush: *const ID2D1Brush,
    ) -> HRESULT,
    fn FillRectangle(
        rect: *const D2D1_RECT_F,
        brush: *const ID2D1Brush,
    ) -> HRESULT,
    fn PushAxisAlignedClip(
        clipRect: *const D2D1_RECT_F,
        antialiasMode: D2D1_ANTIALIAS_MODE,
    ) -> HRESULT,
    fn PushLayer(
        layerParameters1: *const D2D1_LAYER_PARAMETERS1,
        layer: *const ID2D1Layer,
    ) -> HRESULT,
    fn PopAxisAlignedClip() -> HRESULT,
    fn PopLayer() -> HRESULT,
}}
RIDL!{#[uuid(0xb4f34a19, 0x2383, 0x4d76, 0x94, 0xf6, 0xec, 0x34, 0x36, 0x57, 0xc3, 0xdc)]
interface ID2D1CommandList(ID2D1CommandListVtbl): ID2D1Image(ID2D1ImageVtbl) {
    fn Stream(
        sink: *const ID2D1CommandSink,
    ) -> HRESULT,
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0x2c1d867d, 0xc290, 0x41c8, 0xae, 0x7e, 0x34, 0xa9, 0x87, 0x02, 0xe9, 0xa5)]
interface ID2D1PrintControl(ID2D1PrintControlVtbl): IUnknown(IUnknownVtbl) {
    fn AddPage(
        commandList: *const ID2D1CommandList,
        pageSize: D2D_SIZE_F,
        pagePrintTicketStream: *const IStream,
        tag1: *mut D2D1_TAG,
        tag2: *mut D2D1_TAG,
    ) -> HRESULT,
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0xfe9e984d, 0x3f95, 0x407c, 0xb5, 0xdb, 0xcb, 0x94, 0xd4, 0xe8, 0xf8, 0x7c)]
interface ID2D1ImageBrush(ID2D1ImageBrushVtbl): ID2D1Brush(ID2D1BrushVtbl) {
    fn SetImage(
        image: *const ID2D1Image,
    ) -> (),
    fn SetExtendModeX(
        extendModeX: D2D1_EXTEND_MODE,
    ) -> (),
    fn SetExtendModeY(
        extendModeY: D2D1_EXTEND_MODE,
    ) -> (),
    fn SetInterpolationMode(
        interpolationMode: D2D1_INTERPOLATION_MODE,
    ) -> (),
    fn SetSourceRectangle(
        sourceRectangle: *const D2D1_RECT_F,
    ) -> (),
    fn GetImage(
        image: *mut *mut ID2D1Image,
    ) -> (),
    fn GetExtendModeX() -> D2D1_EXTEND_MODE,
    fn GetExtendModeY() -> D2D1_EXTEND_MODE,
    fn GetInterpolationMode() -> D2D1_INTERPOLATION_MODE,
    fn GetSourceRectangle(
        sourceRectangle: *mut D2D1_RECT_F,
    ) -> (),
}}
RIDL!{#[uuid(0x41343a53, 0xe41a, 0x49a2, 0x91, 0xcd, 0x21, 0x79, 0x3b, 0xbb, 0x62, 0xe5)]
interface ID2D1BitmapBrush1(ID2D1BitmapBrush1Vtbl): ID2D1BitmapBrush(ID2D1BitmapBrushVtbl) {
    fn SetInterpolationMode1(
        interpolationMode: D2D1_INTERPOLATION_MODE,
    ) -> (),
    fn GetInterpolationMode1() -> D2D1_INTERPOLATION_MODE,
}}
RIDL!{#[uuid(0x10a72a66, 0xe91c, 0x43f4, 0x99, 0x3f, 0xdd, 0xf4, 0xb8, 0x2b, 0x0b, 0x4a)]
interface ID2D1StrokeStyle1(ID2D1StrokeStyle1Vtbl): ID2D1StrokeStyle(ID2D1StrokeStyleVtbl) {
    fn GetStrokeTransformType() -> D2D1_STROKE_TRANSFORM_TYPE,
}}
RIDL!{#[uuid(0x62baa2d2, 0xab54, 0x41b7, 0xb8, 0x72, 0x78, 0x7e, 0x01, 0x06, 0xa4, 0x21)]
interface ID2D1PathGeometry1(ID2D1PathGeometry1Vtbl): ID2D1PathGeometry(ID2D1PathGeometryVtbl) {
    fn ComputePointAndSegmentAtLength(
        length: FLOAT,
        startSegment: UINT32,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        pointDescription: *mut D2D1_POINT_DESCRIPTION,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x483473d7, 0xcd46, 0x4f9d, 0x9d, 0x3a, 0x31, 0x12, 0xaa, 0x80, 0x15, 0x9d)]
interface ID2D1Properties(ID2D1PropertiesVtbl): IUnknown(IUnknownVtbl) {
    fn GetPropertyCount() -> UINT32,
    fn GetPropertyName(
        index: UINT32,
        name: PWSTR,
        nameCount: UINT32,
    ) -> HRESULT,
    fn GetPropertyNameLength(
        index: UINT32,
    ) -> UINT32,
    fn GetType(
        index: UINT32,
    ) -> D2D1_PROPERTY_TYPE,
    fn GetPropertyIndex(
        name: PCWSTR,
    ) -> UINT32,
    fn SetValueByName(
        name: PCWSTR,
        prop_type: D2D1_PROPERTY_TYPE,
        data: *const BYTE,
        dataSize: UINT32,
    ) -> HRESULT,
    fn SetValue(
        index: UINT32,
        prop_type: D2D1_PROPERTY_TYPE,
        data: *const BYTE,
        dataSize: UINT32,
    ) -> HRESULT,
    fn GetValueByName(
        name: PCWSTR,
        prop_type: D2D1_PROPERTY_TYPE,
        data: *mut BYTE,
        dataSize: UINT32,
    ) -> HRESULT,
    fn GetValue(
        index: UINT32,
        prop_type: D2D1_PROPERTY_TYPE,
        data: *mut BYTE,
        dataSize: UINT32,
    ) -> HRESULT,
    fn GetValueSize(
        index: UINT32,
    ) -> UINT32,
    fn GetSubProperties(
        index: UINT32,
        subProperties: *mut *mut ID2D1Properties,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x28211a43, 0x7d89, 0x476f, 0x81, 0x81, 0x2d, 0x61, 0x59, 0xb2, 0x20, 0xad)]
interface ID2D1Effect(ID2D1EffectVtbl): ID2D1Properties(ID2D1PropertiesVtbl) {
    fn SetInput(
        index: UINT32,
        input: *const ID2D1Image,
        invalidate: BOOL,
    ) -> (),
    fn SetInputCount(
        inputCount: UINT32,
    ) -> HRESULT,
    fn GetInput(
        index: UINT32,
        input: *mut *mut ID2D1Image,
    ) -> (),
    fn GetInputCount() -> UINT32,
    fn GetOutput(
        outputImage: *mut *mut ID2D1Image,
    ) -> (),
}}
RIDL!{#[uuid(0xa898a84c, 0x3873, 0x4588, 0xb0, 0x8b, 0xeb, 0xbf, 0x97, 0x8d, 0xf0, 0x41)]
interface ID2D1Bitmap1(ID2D1Bitmap1Vtbl): ID2D1Bitmap(ID2D1BitmapVtbl) {
    fn GetColorContext(
        colorContext: *mut *mut ID2D1ColorContext,
    ) -> (),
    fn GetOptions() -> D2D1_BITMAP_OPTIONS,
    fn GetSurface(
        dxgiSurface: *mut *mut IDXGISurface,
    ) -> HRESULT,
    fn Map(
        options: D2D1_MAP_OPTIONS,
        mappedRect: *mut D2D1_MAPPED_RECT,
    ) -> HRESULT,
    fn Unmap() -> HRESULT,
}}
RIDL!{#[uuid(0x1c4820bb, 0x5771, 0x4518, 0xa5, 0x81, 0x2f, 0xe4, 0xdd, 0x0e, 0xc6, 0x57)]
interface ID2D1ColorContext(ID2D1ColorContextVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetColorSpace() -> D2D1_COLOR_SPACE,
    fn GetProfileSize() -> UINT32,
    fn GetProfile(
        profile: *mut BYTE,
        profileSize: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xae1572f4, 0x5dd0, 0x4777, 0x99, 0x8b, 0x92, 0x79, 0x47, 0x2a, 0xe6, 0x3b)]
interface ID2D1GradientStopCollection1(ID2D1GradientStopCollection1Vtbl):
    ID2D1GradientStopCollection(ID2D1GradientStopCollectionVtbl) {
    fn GetGradientStops1(
        gradientStops: *mut D2D1_GRADIENT_STOP,
        gradientStopsCount: UINT32,
    ) -> (),
    fn GetPreInterpolationSpace() -> D2D1_COLOR_SPACE,
    fn GetPostInterpolationSpace() -> D2D1_COLOR_SPACE,
    fn GetBufferPrecision() -> D2D1_BUFFER_PRECISION,
    fn GetColorInterpolationMode() -> D2D1_COLOR_INTERPOLATION_MODE,
}}
RIDL!{#[uuid(0x689f1f85, 0xc72e, 0x4e33, 0x8f, 0x19, 0x85, 0x75, 0x4e, 0xfd, 0x5a, 0xce)]
interface ID2D1DrawingStateBlock1(ID2D1DrawingStateBlock1Vtbl):
    ID2D1DrawingStateBlock(ID2D1DrawingStateBlockVtbl) {
    fn GetDescription(
        stateDescription: *mut D2D1_DRAWING_STATE_DESCRIPTION1,
    ) -> (),
    fn SetDescription(
        stateDescription: *const D2D1_DRAWING_STATE_DESCRIPTION1,
    ) -> (),
}}
RIDL!{#[uuid(0xe8f7fe7a, 0x191c, 0x466d, 0xad, 0x95, 0x97, 0x56, 0x78, 0xbd, 0xa9, 0x98)]
interface ID2D1DeviceContext(ID2D1DeviceContextVtbl): ID2D1RenderTarget(ID2D1RenderTargetVtbl) {
    fn CreateBitmap(
        size: D2D1_SIZE_U,
        sourceData: *const c_void,
        pitch: UINT32,
        bitmapProperties: *const D2D1_BITMAP_PROPERTIES1,
        bitmap: *mut *mut ID2D1Bitmap1,
    ) -> HRESULT,
    fn CreateBitmapFromWicBitmap(
        wicBitmapSource: *const IWICBitmapSource,
        bitmapProperties: *const D2D1_BITMAP_PROPERTIES1,
        bitmap: *mut *mut ID2D1Bitmap1,
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
        wicColorContext: *const IWICColorContext,
        colorContext: *mut *mut ID2D1ColorContext,
    ) -> HRESULT,
    fn CreateBitmapFromDxgiSurface(
        surface: *const IDXGISurface,
        bitmapProperties: *const D2D1_BITMAP_PROPERTIES1,
        bitmap: *mut *mut ID2D1Bitmap1,
    ) -> HRESULT,
    fn CreateEffect(
        effectId: REFCLSID,
        effect: *mut *mut ID2D1Effect,
    ) -> HRESULT,
    fn CreateGradientStopCollection(
        straightAlphaGradientStops: *const D2D1_GRADIENT_STOP,
        straightAlphaGradientStopsCount: UINT32,
        preInterpolationSpace: D2D1_COLOR_SPACE,
        postInterpolationSpace: D2D1_COLOR_SPACE,
        bufferPrecision: D2D1_BUFFER_PRECISION,
        extendMode: D2D1_EXTEND_MODE,
        colorInterpolationMode: D2D1_COLOR_INTERPOLATION_MODE,
        gradientStopCollection1: *mut *mut ID2D1GradientStopCollection1,
    ) -> HRESULT,
    fn CreateImageBrush(
        image: *const ID2D1Image,
        imageBrushProperties: *const D2D1_IMAGE_BRUSH_PROPERTIES,
        brushProperties: *const D2D1_BRUSH_PROPERTIES,
        imageBrush: *mut *mut ID2D1ImageBrush,
    ) -> HRESULT,
    fn CreateBitmapBrush(
        bitmap: *const ID2D1Bitmap,
        bitmapBrushProperties: *const D2D1_BITMAP_BRUSH_PROPERTIES1,
        brushProperties: *const D2D1_BRUSH_PROPERTIES,
        bitmapBrush: *mut *mut ID2D1BitmapBrush1,
    ) -> HRESULT,
    fn CreateCommandList(
        commandList: *mut *mut ID2D1CommandList,
    ) -> HRESULT,
    fn IsDxgiFormatSupported(
        format: DXGI_FORMAT,
    ) -> BOOL,
    fn IsBufferPrecisionSupported(
        bufferPrecision: D2D1_BUFFER_PRECISION,
    ) -> BOOL,
    fn GetImageLocalBounds(
        image: *const ID2D1Image,
        localBounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
    fn GetImageWorldBounds(
        image: *const ID2D1Image,
        worldBounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
    fn GetGlyphRunWorldBounds(
        baselineOrigin: D2D1_POINT_2F,
        glyphRun: *const DWRITE_GLYPH_RUN,
        measuringMode: DWRITE_MEASURING_MODE,
        bounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
    fn GetDevice(
        device: *mut *mut ID2D1Device,
    ) -> (),
    fn SetTarget(
        image: *const ID2D1Image,
    ) -> (),
    fn GetTarget(
        image: *mut *mut ID2D1Image,
    ) -> (),
    fn SetRenderingControls(
        renderingControls: *const D2D1_RENDERING_CONTROLS,
    ) -> (),
    fn GetRenderingControls(
        renderingControls: *mut D2D1_RENDERING_CONTROLS,
    ) -> (),
    fn SetPrimitiveBlend(
        primitiveBlend: D2D1_PRIMITIVE_BLEND,
    ) -> (),
    fn GetPrimitiveBlend() -> D2D1_PRIMITIVE_BLEND,
    fn SetUnitMode(
        unitMode: D2D1_UNIT_MODE,
    ) -> (),
    fn GetUnitMode() -> D2D1_UNIT_MODE,
    fn DrawGlyphRun(
        baselineOrigin: D2D1_POINT_2F,
        glyphRun: *const DWRITE_GLYPH_RUN,
        glyphRunDescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        foregroundBrush: *const ID2D1Brush,
        measuringMode: DWRITE_MEASURING_MODE,
    ) -> (),
    fn DrawImage(
        image: *const ID2D1Image,
        targetOffset: *const D2D1_POINT_2F,
        imageRectangle: *const D2D1_RECT_F,
        interpolationMode: D2D1_INTERPOLATION_MODE,
        compositeMode: D2D1_COMPOSITE_MODE,
    ) -> (),
    fn DrawGdiMetafile(
        gdiMetafile: *const ID2D1GdiMetafile,
        targetOffset: *const D2D1_POINT_2F,
    ) -> (),
    fn DrawBitmap(
        bitmap: *const ID2D1Bitmap,
        destinationRectangle: *const D2D1_RECT_F,
        opacity: FLOAT,
        interpolationMode: D2D1_INTERPOLATION_MODE,
        sourceRectangle: *const D2D1_RECT_F,
        perspectiveTransform: *const D2D1_MATRIX_4X4_F,
    ) -> (),
    fn PushLayer(
        layerParameters: *const D2D1_LAYER_PARAMETERS1,
        layer: *const ID2D1Layer,
    ) -> (),
    fn InvalidateEffectInputRectangle(
        effect: *const ID2D1Effect,
        input: UINT32,
        inputRectangle: *const D2D1_RECT_F,
    ) -> HRESULT,
    fn GetEffectInvalidRectangleCount(
        effect: *const ID2D1Effect,
        rectangleCount: *mut UINT32,
    ) -> HRESULT,
    fn GetEffectInvalidRectangles(
        effect: *const ID2D1Effect,
        rectangles: *mut D2D1_RECT_F,
        rectanglesCount: UINT32,
    ) -> HRESULT,
    fn GetEffectRequiredInputRectangles(
        renderEffect: *const ID2D1Effect,
        renderImageRectangle: *const D2D1_RECT_F,
        inputDescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION,
        requiredInputRects: *mut D2D1_RECT_F,
        inputCount: UINT32,
    ) -> HRESULT,
    fn FillOpacityMask(
        opacityMask: *const ID2D1Bitmap,
        brush: *const ID2D1Brush,
        destinationRectangle: *const D2D1_RECT_F,
        sourceRectangle: *const D2D1_RECT_F,
    ) -> (),
}}
RIDL!{#[uuid(0x47dd575d, 0xac05, 0x4cdd, 0x80, 0x49, 0x9b, 0x02, 0xcd, 0x16, 0xf4, 0x4c)]
interface ID2D1Device(ID2D1DeviceVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn CreateDeviceContext(
        options: D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext: *mut *mut ID2D1DeviceContext,
    ) -> HRESULT,
    fn CreatePrintControl(
        wicFactory: *const IWICImagingFactory,
        documentTarget: *const IPrintDocumentPackageTarget,
        printControlProperties: *const D2D1_PRINT_CONTROL_PROPERTIES,
        printControl: *mut *mut ID2D1PrintControl,
    ) -> HRESULT,
    fn SetMaximumTextureMemory(
        maximumInBytes: UINT64,
    ) -> (),
    fn GetMaximumTextureMemory() -> UINT64,
    fn ClearResources(
        millisecondsSinceUse: UINT32,
    ) -> (),
}}
RIDL!{#[uuid(0xbb12d362, 0xdaee, 0x4b9a, 0xaa, 0x1d, 0x14, 0xba, 0x40, 0x1c, 0xfa, 0x1f)]
interface ID2D1Factory1(ID2D1Factory1Vtbl): ID2D1Factory(ID2D1FactoryVtbl) {
    fn CreateDevice(
        dxgiDevice: *const IDXGIDevice,
        d2dDevice: *mut *mut ID2D1Device,
    ) -> HRESULT,
    fn CreateStrokeStyle(
        strokeStyleProperties: *const D2D1_STROKE_STYLE_PROPERTIES1,
        dashes: *const FLOAT,
        dashesCount: UINT32,
        strokeStyle: *mut *mut ID2D1StrokeStyle1,
    ) -> HRESULT,
    fn CreatePathGeometry(
        pathGeometry: *mut *mut ID2D1PathGeometry1,
    ) -> HRESULT,
    fn CreateDrawingStateBlock(
        drawingStateDescription: *const D2D1_DRAWING_STATE_DESCRIPTION1,
        textRenderingParams: *const IDWriteRenderingParams,
        drawingStateBlock: *mut *mut ID2D1DrawingStateBlock1,
    ) -> HRESULT,
    fn CreateGdiMetafile(
        metafileStream: *const IStream,
        metafile: *mut *mut ID2D1GdiMetafile,
    ) -> HRESULT,
    fn RegisterEffectFromStream(
        classId: REFCLSID,
        propertyXml: *const IStream,
        bindings: *const D2D1_PROPERTY_BINDING,
        bindingsCount: UINT32,
        effectFactory: PD2D1_EFFECT_FACTORY,
    ) -> HRESULT,
    fn RegisterEffectFromString(
        classId: REFCLSID,
        propertyXml: PCWSTR,
        bindings: *const D2D1_PROPERTY_BINDING,
        bindingsCount: UINT32,
        effectFactory: PD2D1_EFFECT_FACTORY,
    ) -> HRESULT,
    fn UnregisterEffect(
        classId: REFCLSID,
    ) -> HRESULT,
    fn GetRegisteredEffects(
        effects: *mut CLSID,
        effectsCount: UINT32,
        effectsReturned: *mut UINT32,
        effectsRegistered: *mut UINT32,
    ) -> HRESULT,
    fn GetEffectProperties(
        effectId: REFCLSID,
        properties: *mut *mut ID2D1Properties,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x31e6e7bc, 0xe0ff, 0x4d46, 0x8c, 0x64, 0xa0, 0xa8, 0xc4, 0x1c, 0x15, 0xd3)]
interface ID2D1Multithread(ID2D1MultithreadVtbl): IUnknown(IUnknownVtbl) {
    fn GetMultithreadProtected() -> BOOL,
    fn Enter() -> (),
    fn Leave() -> (),
}}
extern "system" {
    pub fn D2D1CreateDevice(
        dxgiDevice: *const IDXGIDevice,
        creationProperties: *const D2D1_CREATION_PROPERTIES,
        d2dDevice: *mut *mut ID2D1Device,
    ) -> HRESULT;
    pub fn D2D1CreateDeviceContext(
        dxgiSurface: *const IDXGISurface,
        creationProperties: *const D2D1_CREATION_PROPERTIES,
        d2dDeviceContext: *mut *mut ID2D1DeviceContext,
    ) -> HRESULT;
    pub fn D2D1ConvertColorSpace(
        sourceColorSpace: D2D1_COLOR_SPACE,
        destinationColorSpace: D2D1_COLOR_SPACE,
        color: *const D2D1_COLOR_F,
    ) -> D2D1_COLOR_F;
    pub fn D2D1SinCos(
        angle: FLOAT,
        s: *mut FLOAT,
        c: *mut FLOAT,
    ) -> ();
    pub fn D2D1Tan(
        angle: FLOAT,
    ) -> FLOAT;
    pub fn D2D1Vec3Length(
        x: FLOAT,
        y: FLOAT,
        z: FLOAT,
    ) -> FLOAT;
}
