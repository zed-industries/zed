// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! Mappings for the contents of d2d1.h
use ctypes::c_void;
use shared::basetsd::{UINT32, UINT64};
use shared::dxgi::IDXGISurface;
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, DWORD, FLOAT};
use shared::windef::{HDC, HWND, RECT};
use um::d2dbasetypes::{
    D2D_COLOR_F, D2D_MATRIX_3X2_F, D2D_POINT_2F, D2D_POINT_2U, D2D_RECT_F, D2D_RECT_U, D2D_SIZE_F,
    D2D_SIZE_U,
};
use um::d3dcommon::{D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_9_1};
use um::dcommon::{D2D1_PIXEL_FORMAT, DWRITE_MEASURING_MODE};
use um::dwrite::{DWRITE_GLYPH_RUN, IDWriteRenderingParams, IDWriteTextFormat, IDWriteTextLayout};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::wincodec::{IWICBitmap, IWICBitmapSource};
use um::winnt::{HRESULT, WCHAR};
// Types confirmed affected by the ABI issue:
// D2D1_SIZE_F, D2D1_SIZE_U, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D1_POINT_2F
pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: FLOAT = 0.25;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR: DWORD = 0;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR: DWORD = 1;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_CUBIC: DWORD = 2;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MULTI_SAMPLE_LINEAR: DWORD = 3;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_ANISOTROPIC: DWORD = 4;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_HIGH_QUALITY_CUBIC: DWORD = 5;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_FANT: DWORD = 6;
pub const D2D1_INTERPOLATION_MODE_DEFINITION_MIPMAP_LINEAR: DWORD = 7;
ENUM!{enum D2D1_GAMMA {
    D2D1_GAMMA_2_2 = 0,
    D2D1_GAMMA_1_0 = 1,
}}
ENUM!{enum D2D1_OPACITY_MASK_CONTENT {
    D2D1_OPACITY_MASK_CONTENT_GRAPHICS = 0,
    D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL = 1,
    D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE = 2,
}}
ENUM!{enum D2D1_EXTEND_MODE {
    D2D1_EXTEND_MODE_CLAMP = 0,
    D2D1_EXTEND_MODE_WRAP = 1,
    D2D1_EXTEND_MODE_MIRROR = 2,
}}
ENUM!{enum D2D1_ANTIALIAS_MODE {
    D2D1_ANTIALIAS_MODE_PER_PRIMITIVE = 0,
    D2D1_ANTIALIAS_MODE_ALIASED = 1,
}}
ENUM!{enum D2D1_TEXT_ANTIALIAS_MODE {
    D2D1_TEXT_ANTIALIAS_MODE_DEFAULT = 0,
    D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE = 1,
    D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE = 2,
    D2D1_TEXT_ANTIALIAS_MODE_ALIASED = 3,
}}
ENUM!{enum D2D1_BITMAP_INTERPOLATION_MODE {
    D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR =
        D2D1_INTERPOLATION_MODE_DEFINITION_NEAREST_NEIGHBOR,
    D2D1_BITMAP_INTERPOLATION_MODE_LINEAR =
        D2D1_INTERPOLATION_MODE_DEFINITION_LINEAR,
}}
ENUM!{enum D2D1_DRAW_TEXT_OPTIONS {
    D2D1_DRAW_TEXT_OPTIONS_NO_SNAP = 0x00000001,
    D2D1_DRAW_TEXT_OPTIONS_CLIP = 0x00000002,
    D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT = 0x00000004,
    D2D1_DRAW_TEXT_OPTIONS_NONE = 0x00000000,
}}
pub type D2D1_POINT_2U = D2D_POINT_2U;
pub type D2D1_POINT_2F = D2D_POINT_2F;
pub type D2D1_RECT_F = D2D_RECT_F;
pub type D2D1_RECT_U = D2D_RECT_U;
pub type D2D1_SIZE_F = D2D_SIZE_F;
pub type D2D1_SIZE_U = D2D_SIZE_U;
pub type D2D1_COLOR_F = D2D_COLOR_F;
pub type D2D1_MATRIX_3X2_F = D2D_MATRIX_3X2_F;
pub type D2D1_TAG = UINT64;
STRUCT!{struct D2D1_BITMAP_PROPERTIES {
    pixelFormat: D2D1_PIXEL_FORMAT,
    dpiX: FLOAT,
    dpiY: FLOAT,
}}
STRUCT!{struct D2D1_GRADIENT_STOP {
    position: FLOAT,
    color: D2D1_COLOR_F,
}}
STRUCT!{struct D2D1_BRUSH_PROPERTIES {
    opacity: FLOAT,
    transform: D2D1_MATRIX_3X2_F,
}}
STRUCT!{struct D2D1_BITMAP_BRUSH_PROPERTIES {
    extendModeX: D2D1_EXTEND_MODE,
    extendModeY: D2D1_EXTEND_MODE,
    interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
}}
STRUCT!{struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    startPoint: D2D1_POINT_2F,
    endPoint: D2D1_POINT_2F,
}}
STRUCT!{struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    center: D2D1_POINT_2F,
    gradientOriginOffset: D2D1_POINT_2F,
    radiusX: FLOAT,
    radiusY: FLOAT,
}}
ENUM!{enum D2D1_ARC_SIZE {
    D2D1_ARC_SIZE_SMALL = 0,
    D2D1_ARC_SIZE_LARGE = 1,
}}
ENUM!{enum D2D1_CAP_STYLE {
    D2D1_CAP_STYLE_FLAT = 0,
    D2D1_CAP_STYLE_SQUARE = 1,
    D2D1_CAP_STYLE_ROUND = 2,
    D2D1_CAP_STYLE_TRIANGLE = 3,
}}
ENUM!{enum D2D1_DASH_STYLE {
    D2D1_DASH_STYLE_SOLID = 0,
    D2D1_DASH_STYLE_DASH = 1,
    D2D1_DASH_STYLE_DOT = 2,
    D2D1_DASH_STYLE_DASH_DOT = 3,
    D2D1_DASH_STYLE_DASH_DOT_DOT = 4,
    D2D1_DASH_STYLE_CUSTOM = 5,
}}
ENUM!{enum D2D1_LINE_JOIN {
    D2D1_LINE_JOIN_MITER = 0,
    D2D1_LINE_JOIN_BEVEL = 1,
    D2D1_LINE_JOIN_ROUND = 2,
    D2D1_LINE_JOIN_MITER_OR_BEVEL = 3,
}}
ENUM!{enum D2D1_COMBINE_MODE {
    D2D1_COMBINE_MODE_UNION = 0,
    D2D1_COMBINE_MODE_INTERSECT = 1,
    D2D1_COMBINE_MODE_XOR = 2,
    D2D1_COMBINE_MODE_EXCLUDE = 3,
}}
ENUM!{enum D2D1_GEOMETRY_RELATION {
    D2D1_GEOMETRY_RELATION_UNKNOWN = 0,
    D2D1_GEOMETRY_RELATION_DISJOINT = 1,
    D2D1_GEOMETRY_RELATION_IS_CONTAINED = 2,
    D2D1_GEOMETRY_RELATION_CONTAINS = 3,
    D2D1_GEOMETRY_RELATION_OVERLAP = 4,
}}
ENUM!{enum D2D1_GEOMETRY_SIMPLIFICATION_OPTION {
    D2D1_GEOMETRY_SIMPLIFICATION_OPTION_CUBICS_AND_LINES = 0,
    D2D1_GEOMETRY_SIMPLIFICATION_OPTION_LINES = 1,
}}
ENUM!{enum D2D1_FIGURE_BEGIN {
    D2D1_FIGURE_BEGIN_FILLED = 0,
    D2D1_FIGURE_BEGIN_HOLLOW = 1,
}}
ENUM!{enum D2D1_FIGURE_END {
    D2D1_FIGURE_END_OPEN = 0,
    D2D1_FIGURE_END_CLOSED = 1,
}}
STRUCT!{struct D2D1_BEZIER_SEGMENT {
    point1: D2D1_POINT_2F,
    point2: D2D1_POINT_2F,
    point3: D2D1_POINT_2F,
}}
STRUCT!{struct D2D1_TRIANGLE {
    point1: D2D1_POINT_2F,
    point2: D2D1_POINT_2F,
    point3: D2D1_POINT_2F,
}}
ENUM!{enum D2D1_PATH_SEGMENT {
    D2D1_PATH_SEGMENT_NONE = 0x00000000,
    D2D1_PATH_SEGMENT_FORCE_UNSTROKED = 0x00000001,
    D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN = 0x00000002,
}}
ENUM!{enum D2D1_SWEEP_DIRECTION {
    D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE = 0,
    D2D1_SWEEP_DIRECTION_CLOCKWISE = 1,
}}
ENUM!{enum D2D1_FILL_MODE {
    D2D1_FILL_MODE_ALTERNATE = 0,
    D2D1_FILL_MODE_WINDING = 1,
}}
STRUCT!{struct D2D1_ARC_SEGMENT {
    point: D2D1_POINT_2F,
    size: D2D1_SIZE_F,
    rotationAngle: FLOAT,
    sweepDirection: D2D1_SWEEP_DIRECTION,
    arcSize: D2D1_ARC_SIZE,
}}
STRUCT!{struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    point1: D2D1_POINT_2F,
    point2: D2D1_POINT_2F,
}}
STRUCT!{struct D2D1_ELLIPSE {
    point: D2D1_POINT_2F,
    radiusX: FLOAT,
    radiusY: FLOAT,
}}
STRUCT!{struct D2D1_ROUNDED_RECT {
    rect: D2D1_RECT_F,
    radiusX: FLOAT,
    radiusY: FLOAT,
}}
STRUCT!{struct D2D1_STROKE_STYLE_PROPERTIES {
    startCap: D2D1_CAP_STYLE,
    endCap: D2D1_CAP_STYLE,
    dashCap: D2D1_CAP_STYLE,
    lineJoin: D2D1_LINE_JOIN,
    miterLimit: FLOAT,
    dashStyle: D2D1_DASH_STYLE,
    dashOffset: FLOAT,
}}
ENUM!{enum D2D1_LAYER_OPTIONS {
    D2D1_LAYER_OPTIONS_NONE = 0x00000000,
    D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE = 0x00000001,
}}
STRUCT!{struct D2D1_LAYER_PARAMETERS {
    contentBounds: D2D1_RECT_F,
    geometricMask: *mut ID2D1Geometry,
    maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    maskTransform: D2D1_MATRIX_3X2_F,
    opacity: FLOAT,
    opacityBrush: *mut ID2D1Brush,
    layerOptions: D2D1_LAYER_OPTIONS,
}}
ENUM!{enum D2D1_WINDOW_STATE {
    D2D1_WINDOW_STATE_NONE = 0x0000000,
    D2D1_WINDOW_STATE_OCCLUDED = 0x0000001,
}}
ENUM!{enum D2D1_RENDER_TARGET_TYPE {
    D2D1_RENDER_TARGET_TYPE_DEFAULT = 0,
    D2D1_RENDER_TARGET_TYPE_SOFTWARE = 1,
    D2D1_RENDER_TARGET_TYPE_HARDWARE = 2,
}}
ENUM!{enum D2D1_FEATURE_LEVEL {
    D2D1_FEATURE_LEVEL_DEFAULT = 0,
    D2D1_FEATURE_LEVEL_9 = D3D_FEATURE_LEVEL_9_1,
    D2D1_FEATURE_LEVEL_10 = D3D_FEATURE_LEVEL_10_0,
}}
ENUM!{enum D2D1_RENDER_TARGET_USAGE {
    D2D1_RENDER_TARGET_USAGE_NONE = 0x00000000,
    D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING = 0x00000001,
    D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE = 0x00000002,
}}
ENUM!{enum D2D1_PRESENT_OPTIONS {
    D2D1_PRESENT_OPTIONS_NONE = 0x00000000,
    D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS = 0x00000001,
    D2D1_PRESENT_OPTIONS_IMMEDIATELY = 0x00000002,
}}
STRUCT!{struct D2D1_RENDER_TARGET_PROPERTIES {
    _type: D2D1_RENDER_TARGET_TYPE,
    pixelFormat: D2D1_PIXEL_FORMAT,
    dpiX: FLOAT,
    dpiY: FLOAT,
    usage: D2D1_RENDER_TARGET_USAGE,
    minLevel: D2D1_FEATURE_LEVEL,
}}
STRUCT!{struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    hwnd: HWND,
    pixelSize: D2D1_SIZE_U,
    presentOptions: D2D1_PRESENT_OPTIONS,
}}
ENUM!{enum D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE = 0x00000000,
    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE = 0x00000001,
}}
STRUCT!{struct D2D1_DRAWING_STATE_DESCRIPTION {
    antialiasMode: D2D1_ANTIALIAS_MODE,
    textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    tag1: D2D1_TAG,
    tag2: D2D1_TAG,
    transform: D2D1_MATRIX_3X2_F,
}}
ENUM!{enum D2D1_DC_INITIALIZE_MODE {
    D2D1_DC_INITIALIZE_MODE_COPY = 0,
    D2D1_DC_INITIALIZE_MODE_CLEAR = 1,
}}
ENUM!{enum D2D1_DEBUG_LEVEL {
    D2D1_DEBUG_LEVEL_NONE = 0,
    D2D1_DEBUG_LEVEL_ERROR = 1,
    D2D1_DEBUG_LEVEL_WARNING = 2,
    D2D1_DEBUG_LEVEL_INFORMATION = 3,
}}
ENUM!{enum D2D1_FACTORY_TYPE {
    D2D1_FACTORY_TYPE_SINGLE_THREADED = 0,
    D2D1_FACTORY_TYPE_MULTI_THREADED = 1,
}}
STRUCT!{struct D2D1_FACTORY_OPTIONS {
    debugLevel: D2D1_DEBUG_LEVEL,
}}
RIDL!{#[uuid(0x2cd90691, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1Resource(ID2D1ResourceVtbl): IUnknown(IUnknownVtbl) {
    fn GetFactory(
        factory: *mut *mut ID2D1Factory,
    ) -> (),
}}
RIDL!{#[uuid(0x65019f75, 0x8da2, 0x497c, 0xb3, 0x2c, 0xdf, 0xa3, 0x4e, 0x48, 0xed, 0xe6)]
interface ID2D1Image(ID2D1ImageVtbl): ID2D1Resource(ID2D1ResourceVtbl) {}}
RIDL!{#[uuid(0xa2296057, 0xea42, 0x4099, 0x98, 0x3b, 0x53, 0x9f, 0xb6, 0x50, 0x54, 0x26)]
interface ID2D1Bitmap(ID2D1BitmapVtbl): ID2D1Image(ID2D1ImageVtbl) {
    #[fixme] fn GetSize() -> D2D1_SIZE_F,
    #[fixme] fn GetPixelSize() -> D2D1_SIZE_U,
    #[fixme] fn GetPixelFormat() -> D2D1_PIXEL_FORMAT,
    fn GetDpi(
        dpiX: *mut FLOAT,
        dpiY: *mut FLOAT,
    ) -> (),
    fn CopyFromBitmap(
        destPoint: *const D2D1_POINT_2U,
        bitmap: *mut ID2D1Bitmap,
        srcRect: *const D2D1_RECT_U,
    ) -> HRESULT,
    fn CopyFromRenderTarget(
        destPoint: *const D2D1_POINT_2U,
        renderTarget: *mut ID2D1RenderTarget,
        srcRect: *const D2D1_RECT_U,
    ) -> HRESULT,
    fn CopyFromMemory(
        dstRect: *const D2D1_RECT_U,
        srcData: *const c_void,
        pitch: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2cd906a7, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1GradientStopCollection(ID2D1GradientStopCollectionVtbl):
    ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetGradientStopCount() -> UINT32,
    fn GetGradientStops(
        gradientStops: *mut D2D1_GRADIENT_STOP,
        gradientStopsCount: UINT32,
    ) -> (),
    fn GetColorInterpolationGamma() -> D2D1_GAMMA,
    fn GetExtendMode() -> D2D1_EXTEND_MODE,
}}
RIDL!{#[uuid(0x2cd906a8, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1Brush(ID2D1BrushVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn SetOpacity(
        opacity: FLOAT,
    ) -> (),
    fn SetTransform(
        transform: *const D2D1_MATRIX_3X2_F,
    ) -> (),
    fn GetOpacity() -> FLOAT,
    fn GetTransform(
        transform: *mut D2D1_MATRIX_3X2_F,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906aa, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1BitmapBrush(ID2D1BitmapBrushVtbl): ID2D1Brush(ID2D1BrushVtbl) {
    fn SetExtendModeX(
        extendModeX: D2D1_EXTEND_MODE,
    ) -> (),
    fn SetExtendModeY(
        extendModeY: D2D1_EXTEND_MODE,
    ) -> (),
    fn SetInterpolationMode(
        interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
    ) -> (),
    fn SetBitmap(
        bitmap: *mut ID2D1Bitmap,
    ) -> (),
    fn GetExtendModeX() -> D2D1_EXTEND_MODE,
    fn GetExtendModeY() -> D2D1_EXTEND_MODE,
    fn GetInterpolationMode() -> D2D1_BITMAP_INTERPOLATION_MODE,
    fn GetBitmap(
        bitmap: *mut *mut ID2D1Bitmap,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906a9, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1SolidColorBrush(ID2D1SolidColorBrushVtbl): ID2D1Brush(ID2D1BrushVtbl) {
    fn SetColor(
        color: *const D2D1_COLOR_F,
    ) -> (),
    #[fixme] fn GetColor() -> D2D1_COLOR_F,
}}
RIDL!{#[uuid(0x2cd906ab, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1LinearGradientBrush(ID2D1LinearGradientBrushVtbl): ID2D1Brush(ID2D1BrushVtbl) {
    fn SetStartPoint(
        startPoint: D2D1_POINT_2F,
    ) -> (),
    fn SetEndPoint(
        endPoint: D2D1_POINT_2F,
    ) -> (),
    #[fixme] fn GetStartPoint() -> D2D1_POINT_2F,
    #[fixme] fn GetEndPoint() -> D2D1_POINT_2F,
    fn GetGradientStopCollection(
        gradientStopCollection: *mut *mut ID2D1GradientStopCollection,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906ac, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1RadialGradientBrush(ID2D1RadialGradientBrushVtbl): ID2D1Brush(ID2D1BrushVtbl) {
    fn SetCenter(
        center: D2D1_POINT_2F,
    ) -> (),
    fn SetGradientOriginOffset(
        gradientOriginOffset: D2D1_POINT_2F,
    ) -> (),
    fn SetRadiusX(
        radiusX: FLOAT,
    ) -> (),
    fn SetRadiusY(
        radiusY: FLOAT,
    ) -> (),
    #[fixme] fn GetCenter() -> D2D1_POINT_2F,
    #[fixme] fn GetGradientOriginOffset() -> D2D1_POINT_2F,
    fn GetRadiusX() -> FLOAT,
    fn GetRadiusY() -> FLOAT,
    fn GetGradientStopCollection(
        gradientStopCollection: *mut *mut ID2D1GradientStopCollection,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd9069d, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1StrokeStyle(ID2D1StrokeStyleVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetStartCap() -> D2D1_CAP_STYLE,
    fn GetEndCap() -> D2D1_CAP_STYLE,
    fn GetDashCap() -> D2D1_CAP_STYLE,
    fn GetMiterLimit() -> FLOAT,
    fn GetLineJoin() -> D2D1_LINE_JOIN,
    fn GetDashOffset() -> FLOAT,
    fn GetDashStyle() -> D2D1_DASH_STYLE,
    fn GetDashesCount() -> UINT32,
    fn GetDashes(
        dashes: *mut FLOAT,
        dashesCount: UINT32,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906a1, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1Geometry(ID2D1GeometryVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetBounds(
        worldTransform: *const D2D1_MATRIX_3X2_F,
        bounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
    fn GetWidenedBounds(
        strokeWidth: FLOAT,
        strokeStyle: *mut ID2D1StrokeStyle,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        bounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
    fn StrokeContainsPoint(
        point: D2D1_POINT_2F,
        strokeWidth: FLOAT,
        strokeStyle: *mut ID2D1StrokeStyle,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        contains: *mut BOOL,
    ) -> HRESULT,
    fn FillContainsPoint(
        point: D2D1_POINT_2F,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        contains: *mut BOOL,
    ) -> HRESULT,
    fn CompareWithGeometry(
        inputGeometry: *mut ID2D1Geometry,
        inputGeometryTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        relation: *mut D2D1_GEOMETRY_RELATION,
    ) -> HRESULT,
    fn Simplify(
        simplificationOption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        geometrySink: *mut ID2D1SimplifiedGeometrySink,
    ) -> HRESULT,
    fn Tessellate(
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        tessellationSink: *mut ID2D1TessellationSink,
    ) -> HRESULT,
    fn CombineWithGeometry(
        inputGeometry: *mut ID2D1Geometry,
        combineMode: D2D1_COMBINE_MODE,
        inputGeometryTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        geometrySink: *mut ID2D1SimplifiedGeometrySink,
    ) -> HRESULT,
    fn Outline(
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        geometrySink: *mut ID2D1SimplifiedGeometrySink,
    ) -> HRESULT,
    fn ComputeArea(
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        area: *mut FLOAT,
    ) -> HRESULT,
    fn ComputeLength(
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        length: *mut FLOAT,
    ) -> HRESULT,
    fn ComputePointAtLength(
        length: FLOAT,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        point: *mut D2D1_POINT_2F,
        unitTangentVector: *mut D2D1_POINT_2F,
    ) -> HRESULT,
    fn Widen(
        strokeWidth: FLOAT,
        strokeStyle: *mut ID2D1StrokeStyle,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        geometrySink: *mut ID2D1SimplifiedGeometrySink,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2cd906a2, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1RectangleGeometry(ID2D1RectangleGeometryVtbl): ID2D1Geometry(ID2D1GeometryVtbl) {
    fn GetRect(
        rect: *mut D2D1_RECT_F,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906a3, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1RoundedRectangleGeometry(ID2D1RoundedRectangleGeometryVtbl):
    ID2D1Geometry(ID2D1GeometryVtbl) {
    fn GetRoundedRect(
        roundedRect: *mut D2D1_ROUNDED_RECT,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906a4, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1EllipseGeometry(ID2D1EllipseGeometryVtbl): ID2D1Geometry(ID2D1GeometryVtbl) {
    fn GetEllipse(
        ellipse: *mut D2D1_ELLIPSE,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906a6, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1GeometryGroup(ID2D1GeometryGroupVtbl): ID2D1Geometry(ID2D1GeometryVtbl) {
    fn GetFillMode() -> D2D1_FILL_MODE,
    fn GetSourceGeometryCount() -> UINT32,
    fn GetSourceGeometries(
        geometries: *mut *mut ID2D1Geometry,
        geometriesCount: UINT32,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906bb, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1TransformedGeometry(ID2D1TransformedGeometryVtbl):
    ID2D1Geometry(ID2D1GeometryVtbl) {
    fn GetSourceGeometry(
        sourceGeometry: *mut *mut ID2D1Geometry,
    ) -> (),
    fn GetTransform(
        transform: *mut D2D1_MATRIX_3X2_F,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd9069e, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1SimplifiedGeometrySink(ID2D1SimplifiedGeometrySinkVtbl): IUnknown(IUnknownVtbl) {
    fn SetFillMode(
        fillMode: D2D1_FILL_MODE,
    ) -> (),
    fn SetSegmentFlags(
        vertexFlags: D2D1_PATH_SEGMENT,
    ) -> (),
    fn BeginFigure(
        startPoint: D2D1_POINT_2F,
        figureBegin: D2D1_FIGURE_BEGIN,
    ) -> (),
    fn AddLines(
        points: *const D2D1_POINT_2F,
        pointsCount: UINT32,
    ) -> (),
    fn AddBeziers(
        beziers: *const D2D1_BEZIER_SEGMENT,
        beziersCount: UINT32,
    ) -> (),
    fn EndFigure(
        figureEnd: D2D1_FIGURE_END,
    ) -> (),
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0x2cd9069f, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1GeometrySink(ID2D1GeometrySinkVtbl):
    ID2D1SimplifiedGeometrySink(ID2D1SimplifiedGeometrySinkVtbl) {
    fn AddLine(
        point: D2D1_POINT_2F,
    ) -> (),
    fn AddBezier(
        bezier: *const D2D1_BEZIER_SEGMENT,
    ) -> (),
    fn AddQuadraticBezier(
        bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT,
    ) -> (),
    fn AddQuadraticBeziers(
        beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT,
        beziersCount: UINT32,
    ) -> (),
    fn AddArc(
        arc: *const D2D1_ARC_SEGMENT,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd906c1, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1TessellationSink(ID2D1TessellationSinkVtbl): IUnknown(IUnknownVtbl) {
    fn AddTriangles(
        triangles: *const D2D1_TRIANGLE,
        triangleCount: UINT32,
    ) -> (),
    fn Close() -> HRESULT,
}}
RIDL!{#[uuid(0x2cd906a5, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1PathGeometry(ID2D1PathGeometryVtbl): ID2D1Geometry(ID2D1GeometryVtbl) {
    fn Open(
        geometrySink: *mut *mut ID2D1GeometrySink,
    ) -> HRESULT,
    fn Stream(
        geometrySink: *mut ID2D1GeometrySink,
    ) -> HRESULT,
    fn GetSegmentCount(
        count: *mut UINT32,
    ) -> HRESULT,
    fn GetFigureCount(
        count: *mut UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2cd906c2, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1Mesh(ID2D1MeshVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn Open(
        tessellationSink: *mut *mut ID2D1TessellationSink,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2cd9069b, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1Layer(ID2D1LayerVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    #[fixme] fn GetSize() -> D2D1_SIZE_F,
}}
RIDL!{#[uuid(0x28506e39, 0xebf6, 0x46a1, 0xbb, 0x47, 0xfd, 0x85, 0x56, 0x5a, 0xb9, 0x57)]
interface ID2D1DrawingStateBlock(ID2D1DrawingStateBlockVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetDescription(
        stateDescription: *mut D2D1_DRAWING_STATE_DESCRIPTION,
    ) -> (),
    fn SetDescription(
        stateDescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
    ) -> (),
    fn SetTextRenderingParams(
        textRenderingParams: *mut IDWriteRenderingParams,
    ) -> (),
    fn GetTextRenderingParams(
        textRenderingParams: *mut *mut IDWriteRenderingParams,
    ) -> (),
}}
RIDL!{#[uuid(0x2cd90694, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1RenderTarget(ID2D1RenderTargetVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn CreateBitmap(
        size: D2D1_SIZE_U,
        srcData: *const c_void,
        pitch: UINT32,
        bitmapProperties: *const D2D1_BITMAP_PROPERTIES,
        bitmap: *mut *mut ID2D1Bitmap,
    ) -> HRESULT,
    fn CreateBitmapFromWicBitmap(
        wicBitmapSource: *mut IWICBitmapSource,
        bitmapProperties: *const D2D1_BITMAP_PROPERTIES,
        bitmap: *mut *mut ID2D1Bitmap,
    ) -> HRESULT,
    fn CreateSharedBitmap(
        riid: REFIID,
        data: *const c_void,
        bitmapProperties: *const D2D1_BITMAP_PROPERTIES,
        bitmap: *mut *mut ID2D1Bitmap,
    ) -> HRESULT,
    fn CreateBitmapBrush(
        bitmap: *mut ID2D1Bitmap,
        bitmapBrushProperties: *const D2D1_BITMAP_BRUSH_PROPERTIES,
        brushProperties: *const D2D1_BRUSH_PROPERTIES,
        bitmapBrush: *mut *mut ID2D1BitmapBrush,
    ) -> HRESULT,
    fn CreateSolidColorBrush(
        color: *const D2D1_COLOR_F,
        brushProperties: *const D2D1_BRUSH_PROPERTIES,
        solidColorBrush: *mut *mut ID2D1SolidColorBrush,
    ) -> HRESULT,
    fn CreateGradientStopCollection(
        gradientStops: *const D2D1_GRADIENT_STOP,
        gradientStopsCount: UINT32,
        colorInterpolationGamma: D2D1_GAMMA,
        extendMode: D2D1_EXTEND_MODE,
        gradientStopCollection: *mut *mut ID2D1GradientStopCollection,
    ) -> HRESULT,
    fn CreateLinearGradientBrush(
        linearGradientBrushProperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
        brushProperties: *const D2D1_BRUSH_PROPERTIES,
        gradientStopCollection: *mut ID2D1GradientStopCollection,
        linearGradientBrush: *mut *mut ID2D1LinearGradientBrush,
    ) -> HRESULT,
    fn CreateRadialGradientBrush(
        radialGradientBrushProperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
        brushProperties: *const D2D1_BRUSH_PROPERTIES,
        gradientStopCollection: *mut ID2D1GradientStopCollection,
        radialGradientBrush: *mut *mut ID2D1RadialGradientBrush,
    ) -> HRESULT,
    fn CreateCompatibleRenderTarget(
        desiredSize: *const D2D1_SIZE_F,
        desiredPixelSize: *const D2D1_SIZE_U,
        desiredFormat: *const D2D1_PIXEL_FORMAT,
        options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
        bitmapRenderTarget: *mut *mut ID2D1BitmapRenderTarget,
    ) -> HRESULT,
    fn CreateLayer(
        size: *const D2D1_SIZE_F,
        layer: *mut *mut ID2D1Layer,
    ) -> HRESULT,
    fn CreateMesh(
        mesh: *mut *mut ID2D1Mesh,
    ) -> HRESULT,
    fn DrawLine(
        point0: D2D1_POINT_2F,
        point1: D2D1_POINT_2F,
        brush: *mut ID2D1Brush,
        strokeWidth: FLOAT,
        strokeStype: *mut ID2D1StrokeStyle,
    ) -> (),
    fn DrawRectangle(
        rect: *const D2D1_RECT_F,
        brush: *mut ID2D1Brush,
        strokeWidth: FLOAT,
        strokeStyle: *mut ID2D1StrokeStyle,
    ) -> (),
    fn FillRectangle(
        rect: *const D2D1_RECT_F,
        brush: *mut ID2D1Brush,
    ) -> (),
    fn DrawRoundedRectangle(
        roundedRect: *const D2D1_ROUNDED_RECT,
        brush: *mut ID2D1Brush,
        strokeWidth: FLOAT,
        strokeStyle: *mut ID2D1StrokeStyle,
    ) -> (),
    fn FillRoundedRectangle(
        roundedRect: *const D2D1_ROUNDED_RECT,
        brush: *mut ID2D1Brush,
    ) -> (),
    fn DrawEllipse(
        ellipse: *const D2D1_ELLIPSE,
        brush: *mut ID2D1Brush,
        strokeWidth: FLOAT,
        strokeStyle: *mut ID2D1StrokeStyle,
    ) -> (),
    fn FillEllipse(
        ellipse: *const D2D1_ELLIPSE,
        brush: *mut ID2D1Brush,
    ) -> (),
    fn DrawGeometry(
        geometry: *mut ID2D1Geometry,
        brush: *mut ID2D1Brush,
        strokeWidth: FLOAT,
        strokeStyle: *mut ID2D1StrokeStyle,
    ) -> (),
    fn FillGeometry(
        geometry: *mut ID2D1Geometry,
        brush: *mut ID2D1Brush,
        opacityBrush: *mut ID2D1Brush,
    ) -> (),
    fn FillMesh(
        mesh: *mut ID2D1Mesh,
        brush: *const ID2D1Brush,
    ) -> (),
    fn FillOpacityMask(
        opacityMask: *mut ID2D1Bitmap,
        brush: *mut ID2D1Brush,
        content: D2D1_OPACITY_MASK_CONTENT,
        destinationRectangle: *const D2D1_RECT_F,
        sourceRectangle: *const D2D1_RECT_F,
    ) -> (),
    fn DrawBitmap(
        bitmap: *mut ID2D1Bitmap,
        destinationRectangle: *const D2D1_RECT_F,
        opacity: FLOAT,
        interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
        sourceRectangle: *const D2D1_RECT_F,
    ) -> (),
    fn DrawText(
        string: *const WCHAR,
        stringLength: UINT32,
        textFormat: *mut IDWriteTextFormat,
        layoutRect: *const D2D1_RECT_F,
        defaultForegroundBrush: *mut ID2D1Brush,
        options: D2D1_DRAW_TEXT_OPTIONS,
        measuringMode: DWRITE_MEASURING_MODE,
    ) -> (),
    fn DrawTextLayout(
        origin: D2D1_POINT_2F,
        textLayout: *mut IDWriteTextLayout,
        defaultForegroundBrush: *mut ID2D1Brush,
        options: D2D1_DRAW_TEXT_OPTIONS,
    ) -> (),
    fn DrawGlyphRun(
        baselineOrigin: D2D1_POINT_2F,
        glyphRun: *const DWRITE_GLYPH_RUN,
        foregroundBrush: *mut ID2D1Brush,
        measuringMode: DWRITE_MEASURING_MODE,
    ) -> (),
    fn SetTransform(
        transform: *const D2D1_MATRIX_3X2_F,
    ) -> (),
    fn GetTransform(
        transform: *mut D2D1_MATRIX_3X2_F,
    ) -> (),
    fn SetAntialiasMode(
        antialiasMode: D2D1_ANTIALIAS_MODE,
    ) -> (),
    fn GetAntialiasMode() -> D2D1_ANTIALIAS_MODE,
    fn SetTextAntialiasMode(
        textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    ) -> (),
    fn GetTextAntialiasMode() -> D2D1_TEXT_ANTIALIAS_MODE,
    fn SetTextRenderingParams(
        textRenderingParams: *mut IDWriteRenderingParams,
    ) -> (),
    fn GetTextRenderingParams(
        textRenderingParams: *mut *mut IDWriteRenderingParams,
    ) -> (),
    fn SetTags(
        tag1: D2D1_TAG,
        tag2: D2D1_TAG,
    ) -> (),
    fn GetTags(
        tag1: *mut D2D1_TAG,
        tag2: *mut D2D1_TAG,
    ) -> (),
    fn PushLayer(
        layerParameters: *const D2D1_LAYER_PARAMETERS,
        layer: *mut ID2D1Layer,
    ) -> (),
    fn PopLayer() -> (),
    fn Flush(
        tag1: *mut D2D1_TAG,
        tag2: *mut D2D1_TAG,
    ) -> HRESULT,
    fn SaveDrawingState(
        drawingStateBlock: *mut ID2D1DrawingStateBlock,
    ) -> (),
    fn RestoreDrawingState(
        drawingStateBlock: *mut ID2D1DrawingStateBlock,
    ) -> (),
    fn PushAxisAlignedClip(
        clipRect: *const D2D1_RECT_F,
        antialiasMode: D2D1_ANTIALIAS_MODE,
    ) -> (),
    fn PopAxisAlignedClip() -> (),
    fn Clear(
        clearColor: *const D2D1_COLOR_F,
    ) -> (),
    fn BeginDraw() -> (),
    fn EndDraw(
        tag1: *mut D2D1_TAG,
        tag2: *mut D2D1_TAG,
    ) -> HRESULT,
    #[fixme] fn GetPixelFormat() -> D2D1_PIXEL_FORMAT,
    fn SetDpi(
        dpiX: FLOAT,
        dpiY: FLOAT,
    ) -> (),
    fn GetDpi(
        dpiX: *mut FLOAT,
        dpiY: *mut FLOAT,
    ) -> (),
    #[fixme] fn GetSize() -> D2D1_SIZE_F,
    #[fixme] fn GetPixelSize() -> D2D1_SIZE_U,
    fn GetMaximumBitmapSize() -> UINT32,
    fn IsSupported(
        renderTargetProperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> BOOL,
}}
RIDL!{#[uuid(0x2cd90695, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1BitmapRenderTarget(ID2D1BitmapRenderTargetVtbl):
    ID2D1RenderTarget(ID2D1RenderTargetVtbl) {
    fn GetBitmap(
        bitmap: *mut *mut ID2D1Bitmap,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x2cd90698, 0x12e2, 0x11dc, 0x9f, 0xed, 0x00, 0x11, 0x43, 0xa0, 0x55, 0xf9)]
interface ID2D1HwndRenderTarget(ID2D1HwndRenderTargetVtbl):
    ID2D1RenderTarget(ID2D1RenderTargetVtbl) {
    fn CheckWindowState() -> D2D1_WINDOW_STATE,
    fn Resize(
        pixelSize: *const D2D1_SIZE_U,
    ) -> HRESULT,
    fn GetHwnd() -> HWND,
}}
RIDL!{#[uuid(0xe0db51c3, 0x6f77, 0x4bae, 0xb3, 0xd5, 0xe4, 0x75, 0x09, 0xb3, 0x58, 0x38)]
interface ID2D1GdiInteropRenderTarget(ID2D1GdiInteropRenderTargetVtbl): IUnknown(IUnknownVtbl) {
    fn GetDC(
        mode: D2D1_DC_INITIALIZE_MODE,
        hdc: *mut HDC,
    ) -> HRESULT,
    fn ReleaseDC(
        update: *const RECT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1c51bc64, 0xde61, 0x46fd, 0x98, 0x99, 0x63, 0xa5, 0xd8, 0xf0, 0x39, 0x50)]
interface ID2D1DCRenderTarget(ID2D1DCRenderTargetVtbl): ID2D1RenderTarget(ID2D1RenderTargetVtbl) {
    fn BindDC(
        hDC: HDC,
        pSubRect: *const RECT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x06152247, 0x6f50, 0x465a, 0x92, 0x45, 0x11, 0x8b, 0xfd, 0x3b, 0x60, 0x07)]
interface ID2D1Factory(ID2D1FactoryVtbl): IUnknown(IUnknownVtbl) {
    fn ReloadSystemMetrics() -> HRESULT,
    fn GetDesktopDpi(
        dpiX: *mut FLOAT,
        dpiY: *mut FLOAT,
    ) -> (),
    fn CreateRectangleGeometry(
        rectangle: *const D2D1_RECT_F,
        rectangleGeometry: *mut *mut ID2D1RectangleGeometry,
    ) -> HRESULT,
    fn CreateRoundedRectangleGeometry(
        roundedRectangle: *const D2D1_ROUNDED_RECT,
        roundedRectangleGeometry: *mut *mut ID2D1RoundedRectangleGeometry,
    ) -> HRESULT,
    fn CreateEllipseGeometry(
        ellipse: *const D2D1_ELLIPSE,
        ellipseGeometry: *mut *mut ID2D1EllipseGeometry,
    ) -> HRESULT,
    fn CreateGeometryGroup(
        fillMode: D2D1_FILL_MODE,
        geometries: *mut *mut ID2D1Geometry,
        geometriesCount: UINT32,
        geometryGroup: *mut *mut ID2D1GeometryGroup,
    ) -> HRESULT,
    fn CreateTransformedGeometry(
        sourceGeometry: *mut ID2D1Geometry,
        transform: *const D2D1_MATRIX_3X2_F,
        transformedGeometry: *mut *mut ID2D1TransformedGeometry,
    ) -> HRESULT,
    fn CreatePathGeometry(
        pathGeometry: *mut *mut ID2D1PathGeometry,
    ) -> HRESULT,
    fn CreateStrokeStyle(
        strokeStyleProperties: *const D2D1_STROKE_STYLE_PROPERTIES,
        dashes: *const FLOAT,
        dashesCount: UINT32,
        strokeStyle: *mut *mut ID2D1StrokeStyle,
    ) -> HRESULT,
    fn CreateDrawingStateBlock(
        drawingStateDescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
        textRenderingParams: *mut IDWriteRenderingParams,
        drawingStateBlock: *mut *mut ID2D1DrawingStateBlock,
    ) -> HRESULT,
    fn CreateWicBitmapRenderTarget(
        target: *mut IWICBitmap,
        renderTargetProperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        renderTarget: *mut *mut ID2D1RenderTarget,
    ) -> HRESULT,
    fn CreateHwndRenderTarget(
        renderTargetProperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        hwndRenderTargetProperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
        hwndRenderTarget: *mut *mut ID2D1HwndRenderTarget,
    ) -> HRESULT,
    fn CreateDxgiSurfaceRenderTarget(
        dxgiSurface: *mut IDXGISurface,
        renderTargetProperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        renderTarget: *mut *mut ID2D1RenderTarget,
    ) -> HRESULT,
    fn CreateDCRenderTarget(
        renderTargetProperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        dcRenderTarget: *mut *mut ID2D1DCRenderTarget,
    ) -> HRESULT,
}}
extern "system" {
    pub fn D2D1CreateFactory(
        factoryType: D2D1_FACTORY_TYPE,
        riid: REFIID,
        pFactoryOptions: *const D2D1_FACTORY_OPTIONS,
        ppIFactory: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D2D1MakeRotateMatrix(
        angle: FLOAT,
        center: D2D1_POINT_2F,
        matrix: *mut D2D1_MATRIX_3X2_F,
    );
    pub fn D2D1MakeSkewMatrix(
        angleX: FLOAT,
        angleY: FLOAT,
        center: D2D1_POINT_2F,
        matrix: *mut D2D1_MATRIX_3X2_F,
    );
    pub fn D2D1IsMatrixInvertible(
        matrix: *const D2D1_MATRIX_3X2_F,
    ) -> BOOL;
    pub fn D2D1InvertMatrix(
        matrix: *mut D2D1_MATRIX_3X2_F,
    ) -> BOOL;
    pub fn D2D1ComputeMaximumScaleFactor(
        matrix: *const D2D1_MATRIX_3X2_F,
    ) -> FLOAT;
}
