// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of d2d1_3.h
use ctypes::c_void;
use shared::basetsd::{UINT16, UINT32, UINT64};
use shared::dxgi::{IDXGIDevice, IDXGISurface};
use shared::dxgitype::DXGI_COLOR_SPACE_TYPE;
use shared::minwindef::{BOOL, BYTE, DWORD, FLOAT};
use shared::ntdef::WCHAR;
use shared::winerror::HRESULT;
use um::d2d1::{
    D2D1_BITMAP_INTERPOLATION_MODE, D2D1_COLOR_F, D2D1_DRAW_TEXT_OPTIONS, D2D1_GAMMA_1_0,
    D2D1_GAMMA_2_2, D2D1_MATRIX_3X2_F, D2D1_POINT_2F, D2D1_RECT_F, D2D1_RECT_U, D2D1_SIZE_F,
    ID2D1Bitmap, ID2D1Brush, ID2D1Image, ID2D1ImageVtbl, ID2D1Resource, ID2D1ResourceVtbl,
    ID2D1SimplifiedGeometrySink,
};
use um::d2d1_1::{
    D2D1_BUFFER_PRECISION, D2D1_DEVICE_CONTEXT_OPTIONS, D2D1_INTERPOLATION_MODE,
    D2D1_PRIMITIVE_BLEND, ID2D1ColorContext, ID2D1ColorContextVtbl, ID2D1CommandList,
    ID2D1GdiMetafile, ID2D1GdiMetafileSink, ID2D1GdiMetafileSinkVtbl, ID2D1GdiMetafileVtbl,
};
use um::d2d1_2::{
    ID2D1CommandSink1, ID2D1CommandSink1Vtbl, ID2D1Device1, ID2D1Device1Vtbl, ID2D1DeviceContext1,
    ID2D1DeviceContext1Vtbl, ID2D1Factory2, ID2D1Factory2Vtbl,
};
use um::d2d1effects::D2D1_BLEND_MODE;
use um::d2d1svg::ID2D1SvgDocument;
use um::dcommon::{D2D1_ALPHA_MODE, DWRITE_GLYPH_IMAGE_FORMATS, DWRITE_MEASURING_MODE};
use um::dwrite::{DWRITE_GLYPH_RUN, IDWriteFontFace, IDWriteTextFormat, IDWriteTextLayout};
use um::objidlbase::IStream;
use um::wincodec::IWICBitmapSource;
ENUM!{enum D2D1_INK_NIB_SHAPE {
    D2D1_INK_NIB_SHAPE_ROUND = 0,
    D2D1_INK_NIB_SHAPE_SQUARE = 1,
}}
ENUM!{enum D2D1_ORIENTATION {
    D2D1_ORIENTATION_DEFAULT = 1,
    D2D1_ORIENTATION_FLIP_HORIZONTAL = 2,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE180 = 3,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL = 4,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL = 5,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE270 = 6,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL = 7,
    D2D1_ORIENTATION_ROTATE_CLOCKWISE90 = 8,
}}
ENUM!{enum D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE = 0,
    D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE = 1,
    D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND = 2,
}}
ENUM!{enum D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE = 0,
    D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION = 1,
}}
ENUM!{enum D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE = 0,
    D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE = 1,
}}
STRUCT!{struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    orientation: D2D1_ORIENTATION,
    scaleX: FLOAT,
    scaleY: FLOAT,
    interpolationMode: D2D1_INTERPOLATION_MODE,
    options: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS,
}}
STRUCT!{struct D2D1_INK_POINT {
    x: FLOAT,
    y: FLOAT,
    radius: FLOAT,
}}
STRUCT!{struct D2D1_INK_BEZIER_SEGMENT {
    point1: D2D1_INK_POINT,
    point2: D2D1_INK_POINT,
    point3: D2D1_INK_POINT,
}}
STRUCT!{struct D2D1_INK_STYLE_PROPERTIES {
    nibShape: D2D1_INK_NIB_SHAPE,
    nibTransform: D2D1_MATRIX_3X2_F,
}}
ENUM!{enum D2D1_PATCH_EDGE_MODE {
    D2D1_PATCH_EDGE_MODE_ALIASED = 0,
    D2D1_PATCH_EDGE_MODE_ANTIALIASED = 1,
    D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED = 2,
}}
STRUCT!{struct D2D1_GRADIENT_MESH_PATCH {
    point00: D2D1_POINT_2F,
    point01: D2D1_POINT_2F,
    point02: D2D1_POINT_2F,
    point03: D2D1_POINT_2F,
    point10: D2D1_POINT_2F,
    point11: D2D1_POINT_2F,
    point12: D2D1_POINT_2F,
    point13: D2D1_POINT_2F,
    point20: D2D1_POINT_2F,
    point21: D2D1_POINT_2F,
    point22: D2D1_POINT_2F,
    point23: D2D1_POINT_2F,
    point30: D2D1_POINT_2F,
    point31: D2D1_POINT_2F,
    point32: D2D1_POINT_2F,
    point33: D2D1_POINT_2F,
    color00: D2D1_COLOR_F,
    color03: D2D1_COLOR_F,
    color30: D2D1_COLOR_F,
    color33: D2D1_COLOR_F,
    topEdgeMode: D2D1_PATCH_EDGE_MODE,
    leftEdgeMode: D2D1_PATCH_EDGE_MODE,
    bottomEdgeMode: D2D1_PATCH_EDGE_MODE,
    rightEdgeMode: D2D1_PATCH_EDGE_MODE,
}}
ENUM!{enum D2D1_SPRITE_OPTIONS {
    D2D1_SPRITE_OPTIONS_NONE = 0,
    D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE = 1,
}}
ENUM!{enum D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION {
    D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT = 0,
    D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE = 1,
}}
ENUM!{enum D2D1_GAMMA1 {
    D2D1_GAMMA1_G22 = D2D1_GAMMA_2_2,
    D2D1_GAMMA1_G10 = D2D1_GAMMA_1_0,
    D2D1_GAMMA1_G2084 = 2,
}}
STRUCT!{struct D2D1_SIMPLE_COLOR_PROFILE {
    redPrimary: D2D1_POINT_2F,
    greenPrimary: D2D1_POINT_2F,
    bluePrimary: D2D1_POINT_2F,
    whitePointXZ: D2D1_POINT_2F,
    gamma: D2D1_GAMMA1,
}}
ENUM!{enum D2D1_COLOR_CONTEXT_TYPE {
    D2D1_COLOR_CONTEXT_TYPE_ICC = 0,
    D2D1_COLOR_CONTEXT_TYPE_SIMPLE = 1,
    D2D1_COLOR_CONTEXT_TYPE_DXGI = 2,
}}
DEFINE_GUID!{IID_ID2D1InkStyle,
    0xbae8b344, 0x23fc, 0x4071, 0x8c, 0xb5, 0xd0, 0x5d, 0x6f, 0x07, 0x38, 0x48}
DEFINE_GUID!{IID_ID2D1Ink,
    0xb499923b, 0x7029, 0x478f, 0xa8, 0xb3, 0x43, 0x2c, 0x7c, 0x5f, 0x53, 0x12}
DEFINE_GUID!{IID_ID2D1GradientMesh,
    0xf292e401, 0xc050, 0x4cde, 0x83, 0xd7, 0x04, 0x96, 0x2d, 0x3b, 0x23, 0xc2}
DEFINE_GUID!{IID_ID2D1ImageSource,
    0xc9b664e5, 0x74a1, 0x4378, 0x9a, 0xc2, 0xee, 0xfc, 0x37, 0xa3, 0xf4, 0xd8}
DEFINE_GUID!{IID_ID2D1ImageSourceFromWic,
    0x77395441, 0x1c8f, 0x4555, 0x86, 0x83, 0xf5, 0x0d, 0xab, 0x0f, 0xe7, 0x92}
DEFINE_GUID!{IID_ID2D1TransformedImageSource,
    0x7f1f79e5, 0x2796, 0x416c, 0x8f, 0x55, 0x70, 0x0f, 0x91, 0x14, 0x45, 0xe5}
DEFINE_GUID!{IID_ID2D1LookupTable3D,
    0x53dd9855, 0xa3b0, 0x4d5b, 0x82, 0xe1, 0x26, 0xe2, 0x5c, 0x5e, 0x57, 0x97}
DEFINE_GUID!{IID_ID2D1DeviceContext2,
    0x394ea6a3, 0x0c34, 0x4321, 0x95, 0x0b, 0x6c, 0xa2, 0x0f, 0x0b, 0xe6, 0xc7}
DEFINE_GUID!{IID_ID2D1Device2,
    0xa44472e1, 0x8dfb, 0x4e60, 0x84, 0x92, 0x6e, 0x28, 0x61, 0xc9, 0xca, 0x8b}
DEFINE_GUID!{IID_ID2D1Factory3,
    0x0869759f, 0x4f00, 0x413f, 0xb0, 0x3e, 0x2b, 0xda, 0x45, 0x40, 0x4d, 0x0f}
DEFINE_GUID!{IID_ID2D1CommandSink2,
    0x3bab440e, 0x417e, 0x47df, 0xa2, 0xe2, 0xbc, 0x0b, 0xe6, 0xa0, 0x09, 0x16}
DEFINE_GUID!{IID_ID2D1GdiMetafile1,
    0x2e69f9e8, 0xdd3f, 0x4bf9, 0x95, 0xba, 0xc0, 0x4f, 0x49, 0xd7, 0x88, 0xdf}
DEFINE_GUID!{IID_ID2D1GdiMetafileSink1,
    0xfd0ecb6b, 0x91e6, 0x411e, 0x86, 0x55, 0x39, 0x5e, 0x76, 0x0f, 0x91, 0xb4}
DEFINE_GUID!{IID_ID2D1SpriteBatch,
    0x4dc583bf, 0x3a10, 0x438a, 0x87, 0x22, 0xe9, 0x76, 0x52, 0x24, 0xf1, 0xf1}
DEFINE_GUID!{IID_ID2D1DeviceContext3,
    0x235a7496, 0x8351, 0x414c, 0xbc, 0xd4, 0x66, 0x72, 0xab, 0x2d, 0x8e, 0x00}
DEFINE_GUID!{IID_ID2D1Device3,
    0x852f2087, 0x802c, 0x4037, 0xab, 0x60, 0xff, 0x2e, 0x7e, 0xe6, 0xfc, 0x01}
DEFINE_GUID!{IID_ID2D1Factory4,
    0xbd4ec2d2, 0x0662, 0x4bee, 0xba, 0x8e, 0x6f, 0x29, 0xf0, 0x32, 0xe0, 0x96}
DEFINE_GUID!{IID_ID2D1CommandSink3,
    0x18079135, 0x4cf3, 0x4868, 0xbc, 0x8e, 0x06, 0x06, 0x7e, 0x6d, 0x24, 0x2d}
DEFINE_GUID!{IID_ID2D1SvgGlyphStyle,
    0xaf671749, 0xd241, 0x4db8, 0x8e, 0x41, 0xdc, 0xc2, 0xe5, 0xc1, 0xa4, 0x38}
DEFINE_GUID!{IID_ID2D1DeviceContext4,
    0x8c427831, 0x3d90, 0x4476, 0xb6, 0x47, 0xc4, 0xfa, 0xe3, 0x49, 0xe4, 0xdb}
DEFINE_GUID!{IID_ID2D1Device4,
    0xd7bdb159, 0x5683, 0x4a46, 0xbc, 0x9c, 0x72, 0xdc, 0x72, 0x0b, 0x85, 0x8b}
DEFINE_GUID!{IID_ID2D1Factory5,
    0xc4349994, 0x838e, 0x4b0f, 0x8c, 0xab, 0x44, 0x99, 0x7d, 0x9e, 0xea, 0xcc}
DEFINE_GUID!{IID_ID2D1CommandSink4,
    0xc78a6519, 0x40d6, 0x4218, 0xb2, 0xde, 0xbe, 0xee, 0xb7, 0x44, 0xbb, 0x3e}
DEFINE_GUID!{IID_ID2D1ColorContext1,
    0x1ab42875, 0xc57f, 0x4be9, 0xbd, 0x85, 0x9c, 0xd7, 0x8d, 0x6f, 0x55, 0xee}
DEFINE_GUID!{IID_ID2D1DeviceContext5,
    0x7836d248, 0x68cc, 0x4df6, 0xb9, 0xe8, 0xde, 0x99, 0x1b, 0xf6, 0x2e, 0xb7}
DEFINE_GUID!{IID_ID2D1Device5,
    0xd55ba0a4, 0x6405, 0x4694, 0xae, 0xf5, 0x08, 0xee, 0x1a, 0x43, 0x58, 0xb4}
DEFINE_GUID!{IID_ID2D1Factory6,
    0xf9976f46, 0xf642, 0x44c1, 0x97, 0xca, 0xda, 0x32, 0xea, 0x2a, 0x26, 0x35}
DEFINE_GUID!{IID_ID2D1CommandSink5,
    0x7047dd26, 0xb1e7, 0x44a7, 0x95, 0x9a, 0x83, 0x49, 0xe2, 0x14, 0x4f, 0xa8}
DEFINE_GUID!{IID_ID2D1DeviceContext6,
    0x985f7e37, 0x4ed0, 0x4a19, 0x98, 0xa3, 0x15, 0xb0, 0xed, 0xfd, 0xe3, 0x06}
DEFINE_GUID!{IID_ID2D1Device6,
    0x7bfef914, 0x2d75, 0x4bad, 0xbe, 0x87, 0xe1, 0x8d, 0xdb, 0x07, 0x7b, 0x6d}
DEFINE_GUID!{IID_ID2D1Factory7,
    0xbdc2bdd3, 0xb96c, 0x4de6, 0xbd, 0xf7, 0x99, 0xd4, 0x74, 0x54, 0x54, 0xde}
RIDL!{#[uuid(0xbae8b344, 0x23fc, 0x4071, 0x8c, 0xb5, 0xd0, 0x5d, 0x6f, 0x07, 0x38, 0x48)]
interface ID2D1InkStyle(ID2D1InkStyleVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn SetNibTransform(
        transform: *const D2D1_MATRIX_3X2_F,
    ) -> (),
    fn GetNibTransform(
        transform: *mut D2D1_MATRIX_3X2_F,
    ) -> (),
    fn SetNibShape(
        nibShape: D2D1_INK_NIB_SHAPE,
    ) -> (),
    fn GetNibShape() -> D2D1_INK_NIB_SHAPE,
}}
RIDL!{#[uuid(0xb499923b, 0x7029, 0x478f, 0xa8, 0xb3, 0x43, 0x2c, 0x7c, 0x5f, 0x53, 0x12)]
interface ID2D1Ink(ID2D1InkVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn SetStartPoint(
        startPoint: *const D2D1_INK_POINT,
    ) -> (),
    fn GetStartPoint() -> D2D1_INK_POINT,
    fn AddSegments(
        segments: *const D2D1_INK_BEZIER_SEGMENT,
        segmentsCount: UINT32,
    ) -> HRESULT,
    fn RemoveSegmentsAtEnd(
        segmentsCount: UINT32,
    ) -> HRESULT,
    fn SetSegments(
        startSegment: UINT32,
        segments: *const D2D1_INK_BEZIER_SEGMENT,
        segmentsCount: UINT32,
    ) -> HRESULT,
    fn SetSegmentAtEnd(
        segment: *const D2D1_INK_BEZIER_SEGMENT,
    ) -> HRESULT,
    fn GetSegmentCount() -> UINT32,
    fn GetSegments(
        startSegment: UINT32,
        segments: *mut D2D1_INK_BEZIER_SEGMENT,
        segmentsCount: UINT32,
    ) -> HRESULT,
    fn StreamAsGeometry(
        inkStyle: *mut ID2D1InkStyle,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        flatteningTolerance: FLOAT,
        geometrySink: *mut ID2D1SimplifiedGeometrySink,
    ) -> HRESULT,
    fn GetBounds(
        inkStyle: *mut ID2D1InkStyle,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        bounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf292e401, 0xc050, 0x4cde, 0x83, 0xd7, 0x04, 0x96, 0x2d, 0x3b, 0x23, 0xc2)]
interface ID2D1GradientMesh(ID2D1GradientMeshVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetPatchCount() -> UINT32,
    fn GetPatches(
        startIndex: UINT32,
        patches: *mut D2D1_GRADIENT_MESH_PATCH,
        patchesCount: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc9b664e5, 0x74a1, 0x4378, 0x9a, 0xc2, 0xee, 0xfc, 0x37, 0xa3, 0xf4, 0xd8)]
interface ID2D1ImageSource(ID2D1ImageSourceVtbl): ID2D1Image(ID2D1ImageVtbl) {
    fn OfferResources() -> HRESULT,
    fn TryReclaimResources(
        resourcesDiscarded: *mut BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x77395441, 0x1c8f, 0x4555, 0x86, 0x83, 0xf5, 0x0d, 0xab, 0x0f, 0xe7, 0x92)]
interface ID2D1ImageSourceFromWic(ID2D1ImageSourceFromWicVtbl):
    ID2D1ImageSource(ID2D1ImageSourceVtbl) {
    fn EnsureCached(
        rectangleToFill: *const D2D1_RECT_U,
    ) -> HRESULT,
    fn TrimCache(
        rectangleToPreserve: *const D2D1_RECT_U,
    ) -> HRESULT,
    fn GetSource(
        wicBitmapSource: *mut *mut IWICBitmapSource,
    ) -> (),
}}
RIDL!{#[uuid(0x7f1f79e5, 0x2796, 0x416c, 0x8f, 0x55, 0x70, 0x0f, 0x91, 0x14, 0x45, 0xe5)]
interface ID2D1TransformedImageSource(ID2D1TransformedImageSourceVtbl):
    ID2D1Image(ID2D1ImageVtbl) {
    fn GetSource(
        imageSource: *mut *mut ID2D1ImageSource,
    ) -> (),
    fn GetProperties(
        properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES,
    ) -> (),
}}
RIDL!{#[uuid(0x53dd9855, 0xa3b0, 0x4d5b, 0x82, 0xe1, 0x26, 0xe2, 0x5c, 0x5e, 0x57, 0x97)]
interface ID2D1LookupTable3D(ID2D1LookupTable3DVtbl): ID2D1Resource(ID2D1ResourceVtbl) {}}
RIDL!{#[uuid(0x394ea6a3, 0x0c34, 0x4321, 0x95, 0x0b, 0x6c, 0xa2, 0x0f, 0x0b, 0xe6, 0xc7)]
interface ID2D1DeviceContext2(ID2D1DeviceContext2Vtbl):
    ID2D1DeviceContext1(ID2D1DeviceContext1Vtbl) {
    fn CreateInk(
        startPoint: *const D2D1_INK_POINT,
        ink: *mut *mut ID2D1Ink,
    ) -> HRESULT,
    fn CreateInkStyle(
        inkStyleProperties: *const D2D1_INK_STYLE_PROPERTIES,
        inkStyle: *mut *mut ID2D1InkStyle,
    ) -> HRESULT,
    fn CreateGradientMesh(
        patches: *const D2D1_GRADIENT_MESH_PATCH,
        patchesCount: UINT32,
        gradientMesh: *mut *mut ID2D1GradientMesh,
    ) -> HRESULT,
    fn CreateImageSourceFromWic(
        wicBitmapSource: *mut IWICBitmapSource,
        loadingOptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS,
        alphaMode: D2D1_ALPHA_MODE,
        imageSource: *mut *mut ID2D1ImageSourceFromWic,
    ) -> HRESULT,
    fn CreateLookupTable3D(
        precision: D2D1_BUFFER_PRECISION,
        extents: *const UINT32,
        data: *const BYTE,
        dataCount: UINT32,
        strides: *const UINT32,
        lookupTable: *mut *mut ID2D1LookupTable3D,
    ) -> HRESULT,
    fn CreateImageSourceFromDxgi(
        surfaces: *const *mut IDXGISurface,
        surfaceCount: UINT32,
        colorSpace: DXGI_COLOR_SPACE_TYPE,
        options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS,
        imageSource: *mut *mut ID2D1ImageSource,
    ) -> HRESULT,
    fn GetGradientMeshWorldBounds(
        gradientMesh: *mut ID2D1GradientMesh,
        pBounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
    fn DrawInk(
        ink: *mut ID2D1Ink,
        brush: *mut ID2D1Brush,
        inkStyle: *mut ID2D1InkStyle,
    ) -> (),
    fn DrawGradientMesh(
        gradientMesh: *mut ID2D1GradientMesh,
    ) -> (),
    fn DrawGdiMetafile(
        gdiMetafile: *mut ID2D1GdiMetafile,
        destinationRectangle: *const D2D1_RECT_F,
        sourceRectangle: *const D2D1_RECT_F,
    ) -> (),
    fn CreateTransformedImageSource(
        imageSource: *mut ID2D1ImageSource,
        properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES,
        transformedImageSource: *mut *mut ID2D1TransformedImageSource,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa44472e1, 0x8dfb, 0x4e60, 0x84, 0x92, 0x6e, 0x28, 0x61, 0xc9, 0xca, 0x8b)]
interface ID2D1Device2(ID2D1Device2Vtbl): ID2D1Device1(ID2D1Device1Vtbl) {
    fn CreateDeviceContext(
        options: D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext2: *mut *mut ID2D1DeviceContext2,
    ) -> HRESULT,
    fn FlushDeviceContexts(
        bitmap: *mut ID2D1Bitmap,
    ) -> (),
    fn GetDxgiDevice(
        dxgiDevice: *mut *mut IDXGIDevice,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0869759f, 0x4f00, 0x413f, 0xb0, 0x3e, 0x2b, 0xda, 0x45, 0x40, 0x4d, 0x0f)]
interface ID2D1Factory3(ID2D1Factory3Vtbl): ID2D1Factory2(ID2D1Factory2Vtbl) {
    fn CreateDevice(
        dxgiDevice: *mut IDXGIDevice,
        d2dDevice2: *mut *mut ID2D1Device2,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x3bab440e, 0x417e, 0x47df, 0xa2, 0xe2, 0xbc, 0x0b, 0xe6, 0xa0, 0x09, 0x16)]
interface ID2D1CommandSink2(ID2D1CommandSink2Vtbl): ID2D1CommandSink1(ID2D1CommandSink1Vtbl) {
    fn DrawInk(
        ink: *mut ID2D1Ink,
        brush: *mut ID2D1Brush,
        inkStyle: *mut ID2D1InkStyle,
    ) -> (),
    fn DrawGradientMesh(
        gradientMesh: *mut ID2D1GradientMesh,
    ) -> (),
    fn DrawGdiMetafile(
        gdiMetafile: *mut ID2D1GdiMetafile,
        destinationRectangle: *const D2D1_RECT_F,
        sourceRectangle: *const D2D1_RECT_F,
    ) -> (),
}}
RIDL!{#[uuid(0x2e69f9e8, 0xdd3f, 0x4bf9, 0x95, 0xba, 0xc0, 0x4f, 0x49, 0xd7, 0x88, 0xdf)]
interface ID2D1GdiMetafile1(ID2D1GdiMetafile1Vtbl): ID2D1GdiMetafile(ID2D1GdiMetafileVtbl) {
    fn GetDpi(
        dpiX: *mut FLOAT,
        dpiY: *mut FLOAT,
    ) -> HRESULT,
    fn GetSourceBounds(
        bounds: *mut D2D1_RECT_F,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xfd0ecb6b, 0x91e6, 0x411e, 0x86, 0x55, 0x39, 0x5e, 0x76, 0x0f, 0x91, 0xb4)]
interface ID2D1GdiMetafileSink1(ID2D1GdiMetafileSink1Vtbl):
    ID2D1GdiMetafileSink(ID2D1GdiMetafileSinkVtbl) {
    fn ProcessRecord(
        recordType: DWORD,
        recordData: *const c_void,
        recordDataSize: DWORD,
        flags: UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x4dc583bf, 0x3a10, 0x438a, 0x87, 0x22, 0xe9, 0x76, 0x52, 0x24, 0xf1, 0xf1)]
interface ID2D1SpriteBatch(ID2D1SpriteBatchVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn AddSprites(
        spriteCount: UINT32,
        destinationRectangle: *const D2D1_RECT_F,
        sourceRectangles: *const D2D1_RECT_U,
        colors: *const D2D1_COLOR_F,
        transforms: *const D2D1_MATRIX_3X2_F,
        destinationRectanglesStride: UINT32,
        sourceRectanglesStride: UINT32,
        colorsStride: UINT32,
        transformsStride: D2D1_MATRIX_3X2_F,
    ) -> HRESULT,
    fn SetSprites(
        startIndex: UINT32,
        spriteCount: UINT32,
        destinationRectangle: *const D2D1_RECT_F,
        sourceRectangles: *const D2D1_RECT_U,
        colors: *const D2D1_COLOR_F,
        transforms: *const D2D1_MATRIX_3X2_F,
        destinationRectanglesStride: UINT32,
        sourceRectanglesStride: UINT32,
        colorsStride: UINT32,
        transformsStride: D2D1_MATRIX_3X2_F,
    ) -> HRESULT,
    fn GetSprites(
        startIndex: UINT32,
        spriteCount: UINT32,
        destinationRectangle: *mut D2D1_RECT_F,
        sourceRectangles: *mut D2D1_RECT_U,
        colors: *mut D2D1_COLOR_F,
        transforms: *mut D2D1_MATRIX_3X2_F,
    ) -> HRESULT,
    fn GetSpriteCount() -> UINT32,
    fn Clear() -> (),
}}
RIDL!{#[uuid(0x235a7496, 0x8351, 0x414c, 0xbc, 0xd4, 0x66, 0x72, 0xab, 0x2d, 0x8e, 0x00)]
interface ID2D1DeviceContext3(ID2D1DeviceContext3Vtbl):
    ID2D1DeviceContext2(ID2D1DeviceContext2Vtbl) {
    fn CreateSpriteBatch(
        spriteBatch: *mut *mut ID2D1SpriteBatch,
    ) -> HRESULT,
    fn DrawSpriteBatch(
        spriteBatch: *mut ID2D1SpriteBatch,
        startIndex: UINT32,
        spriteCount: UINT32,
        bitmap: *mut ID2D1Bitmap,
        interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
        spriteOptions: D2D1_SPRITE_OPTIONS,
    ) -> (),
}}
RIDL!{#[uuid(0x852f2087, 0x802c, 0x4037, 0xab, 0x60, 0xff, 0x2e, 0x7e, 0xe6, 0xfc, 0x01)]
interface ID2D1Device3(ID2D1Device3Vtbl): ID2D1Device2(ID2D1Device2Vtbl) {
    fn CreateDeviceContext(
        options: D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext3: *mut *mut ID2D1DeviceContext3,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbd4ec2d2, 0x0662, 0x4bee, 0xba, 0x8e, 0x6f, 0x29, 0xf0, 0x32, 0xe0, 0x96)]
interface ID2D1Factory4(ID2D1Factory4Vtbl): ID2D1Factory3(ID2D1Factory3Vtbl) {
    fn CreateDevice(
        dxgiDevice: *mut IDXGIDevice,
        d2dDevice3: *mut *mut ID2D1Device3,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x18079135, 0x4cf3, 0x4868, 0xbc, 0x8e, 0x06, 0x06, 0x7e, 0x6d, 0x24, 0x2d)]
interface ID2D1CommandSink3(ID2D1CommandSink3Vtbl): ID2D1CommandSink2(ID2D1CommandSink2Vtbl) {
    fn DrawSpriteBatch(
        spriteBatch: *mut ID2D1SpriteBatch,
        startIndex: UINT32,
        spriteCount: UINT32,
        bitmap: *mut ID2D1Bitmap,
        interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
        spriteOptions: D2D1_SPRITE_OPTIONS,
    ) -> (),
}}
RIDL!{#[uuid(0xaf671749, 0xd241, 0x4db8, 0x8e, 0x41, 0xdc, 0xc2, 0xe5, 0xc1, 0xa4, 0x38)]
interface ID2D1SvgGlyphStyle(ID2D1SvgGlyphStyleVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn SetFill(
        brush: *mut ID2D1Brush,
    ) -> HRESULT,
    fn GetFill(
        brush: *mut *mut ID2D1Brush,
    ) -> (),
    fn SetStroke(
        brush: *mut ID2D1Brush,
        strokeWidth: FLOAT,
        dashes: *const FLOAT,
        dashesCount: UINT32,
        dashOffset: FLOAT,
    ) -> HRESULT,
    fn GetStrokeDashesCount() -> UINT32,
    fn GetStroke(
        brush: *mut *mut ID2D1Brush,
        strokeWidth: *mut FLOAT,
        dashes: *mut FLOAT,
        dashesCount: UINT32,
        dashOffset: *mut FLOAT,
    ) -> (),
}}
RIDL!{#[uuid(0x8c427831, 0x3d90, 0x4476, 0xb6, 0x47, 0xc4, 0xfa, 0xe3, 0x49, 0xe4, 0xdb)]
interface ID2D1DeviceContext4(ID2D1DeviceContext4Vtbl):
    ID2D1DeviceContext3(ID2D1DeviceContext3Vtbl) {
    fn CreateSvgGlyphStyle(
        svgGlyphStyle: *mut *mut ID2D1SvgGlyphStyle,
    ) -> HRESULT,
    fn DrawText(
        string: *const WCHAR,
        stringLength: UINT32,
        textFormat: *mut IDWriteTextFormat,
        layoutRect: *const D2D1_RECT_F,
        defaultFillBrush: *mut ID2D1Brush,
        svgGlyphStyle: *mut ID2D1SvgGlyphStyle,
        colorPaletteIndex: UINT32,
        options: D2D1_DRAW_TEXT_OPTIONS,
        measuringMode: DWRITE_MEASURING_MODE,
    ) -> (),
    fn DrawTextLayout(
        origin: D2D1_POINT_2F,
        textLayout: *mut IDWriteTextLayout,
        defaultFillBrush: *mut ID2D1Brush,
        svgGlyphStyle: *mut ID2D1SvgGlyphStyle,
        colorPaletteIndex: UINT32,
        options: D2D1_DRAW_TEXT_OPTIONS,
    ) -> (),
    fn DrawColorBitmapGlyphRun(
        glyphImageFormat: DWRITE_GLYPH_IMAGE_FORMATS,
        baselineOrigin: D2D1_POINT_2F,
        glyphRun: *const DWRITE_GLYPH_RUN,
        measuringMode: DWRITE_MEASURING_MODE,
        bitmapSnapOption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION,
    ) -> (),
    fn DrawSvgGlyphRun(
        baselineOrigin: D2D1_POINT_2F,
        glyphRun: *const DWRITE_GLYPH_RUN,
        defaultFillBrush: *mut ID2D1Brush,
        svgGlyphStyle: *mut ID2D1SvgGlyphStyle,
        colorPaletteIndex: UINT32,
        measuringMode: DWRITE_MEASURING_MODE,
    ) -> (),
    fn GetColorBitmapGlyphImage(
        glyphImageFormat: DWRITE_GLYPH_IMAGE_FORMATS,
        glyphOrigin: D2D1_POINT_2F,
        fontFace: *mut IDWriteFontFace,
        fontEmSize: FLOAT,
        glyphIndex: UINT16,
        isSideways: BOOL,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        dpiX: FLOAT,
        dpiY: FLOAT,
        glyphTransform: *mut D2D1_MATRIX_3X2_F,
        glyphImage: *mut *mut ID2D1Image,
    ) -> HRESULT,
    fn GetSvgGlyphImage(
        glyphImageFormat: DWRITE_GLYPH_IMAGE_FORMATS,
        glyphOrigin: D2D1_POINT_2F,
        fontFace: *mut IDWriteFontFace,
        fontEmSize: FLOAT,
        glyphIndex: UINT16,
        isSideways: BOOL,
        worldTransform: *const D2D1_MATRIX_3X2_F,
        defaultFillBrush: *mut ID2D1Brush,
        svgGlyphStyle: *mut ID2D1SvgGlyphStyle,
        colorPaletteIndex: UINT32,
        glyphTransform: *mut D2D1_MATRIX_3X2_F,
        glyphImage: *mut *mut ID2D1CommandList,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd7bdb159, 0x5683, 0x4a46, 0xbc, 0x9c, 0x72, 0xdc, 0x72, 0x0b, 0x85, 0x8b)]
interface ID2D1Device4(ID2D1Device4Vtbl): ID2D1Device3(ID2D1Device3Vtbl) {
    fn CreateDeviceContext(
        options: D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext4: *mut *mut ID2D1DeviceContext4,
    ) -> HRESULT,
    fn SetMaximumColorGlyphCacheMemory(
        maximumInBytes: UINT64,
    ) -> (),
    fn GetMaximumColorGlyphCacheMemory() -> UINT64,
}}
RIDL!{#[uuid(0xc4349994, 0x838e, 0x4b0f, 0x8c, 0xab, 0x44, 0x99, 0x7d, 0x9e, 0xea, 0xcc)]
interface ID2D1Factory5(ID2D1Factory5Vtbl): ID2D1Factory4(ID2D1Factory4Vtbl) {
    fn CreateDevice(
        dxgiDevice: *mut IDXGIDevice,
        d2dDevice4: *mut *mut ID2D1Device4,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc78a6519, 0x40d6, 0x4218, 0xb2, 0xde, 0xbe, 0xee, 0xb7, 0x44, 0xbb, 0x3e)]
interface ID2D1CommandSink4(ID2D1CommandSink4Vtbl): ID2D1CommandSink3(ID2D1CommandSink3Vtbl) {
    fn SetPrimitiveBlend2(
        primitiveBlend: D2D1_PRIMITIVE_BLEND,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x1ab42875, 0xc57f, 0x4be9, 0xbd, 0x85, 0x9c, 0xd7, 0x8d, 0x6f, 0x55, 0xee)]
interface ID2D1ColorContext1(ID2D1ColorContext1Vtbl): ID2D1ColorContext(ID2D1ColorContextVtbl) {
    fn GetColorContextType() -> D2D1_COLOR_CONTEXT_TYPE,
    fn GetDXGIColorSpace() -> DXGI_COLOR_SPACE_TYPE,
    fn GetSimpleColorProfile(
        simpleProfile: *mut D2D1_SIMPLE_COLOR_PROFILE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7836d248, 0x68cc, 0x4df6, 0xb9, 0xe8, 0xde, 0x99, 0x1b, 0xf6, 0x2e, 0xb7)]
interface ID2D1DeviceContext5(ID2D1DeviceContext5Vtbl):
    ID2D1DeviceContext4(ID2D1DeviceContext4Vtbl) {
    fn CreateSvgDocument(
        inputXmlStream: *mut IStream,
        viewportSize: D2D1_SIZE_F,
        svgDocument: *mut *mut ID2D1SvgDocument,
    ) -> HRESULT,
    fn DrawSvgDocument(
        svgDocument: *mut ID2D1SvgDocument,
    ) -> (),
    fn CreateColorContextFromDxgiColorSpace(
        colorSpace: DXGI_COLOR_SPACE_TYPE,
        colorContext: *mut *mut ID2D1ColorContext1,
    ) -> HRESULT,
    fn CreateColorContextFromSimpleColorProfile(
        simpleProfile: *const D2D1_SIMPLE_COLOR_PROFILE,
        colorContext: *mut *mut ID2D1ColorContext1,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd55ba0a4, 0x6405, 0x4694, 0xae, 0xf5, 0x08, 0xee, 0x1a, 0x43, 0x58, 0xb4)]
interface ID2D1Device5(ID2D1Device5Vtbl): ID2D1Device4(ID2D1Device4Vtbl) {
    fn CreateDeviceContext(
        options: D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext5: *mut *mut ID2D1DeviceContext5,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xf9976f46, 0xf642, 0x44c1, 0x97, 0xca, 0xda, 0x32, 0xea, 0x2a, 0x26, 0x35)]
interface ID2D1Factory6(ID2D1Factory6Vtbl): ID2D1Factory5(ID2D1Factory5Vtbl) {
    fn CreateDevice(
        dxgiDevice: *mut IDXGIDevice,
        d2dDevice5: *mut *mut ID2D1Device5,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7047dd26, 0xb1e7, 0x44a7, 0x95, 0x9a, 0x83, 0x49, 0xe2, 0x14, 0x4f, 0xa8)]
interface ID2D1CommandSink5(ID2D1CommandSink5Vtbl): ID2D1CommandSink4(ID2D1CommandSink4Vtbl) {
    fn BlendImage(
        image: *mut ID2D1Image,
        blendMode: D2D1_BLEND_MODE,
        targetOffset: *const D2D1_POINT_2F,
        imageRectangle: *const D2D1_RECT_F,
        interpolationMode: D2D1_INTERPOLATION_MODE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x985f7e37, 0x4ed0, 0x4a19, 0x98, 0xa3, 0x15, 0xb0, 0xed, 0xfd, 0xe3, 0x06)]
interface ID2D1DeviceContext6(ID2D1DeviceContext6Vtbl):
    ID2D1DeviceContext5(ID2D1DeviceContext5Vtbl) {
    fn BlendImage(
        image: *mut ID2D1Image,
        blendMode: D2D1_BLEND_MODE,
        targetOffset: *const D2D1_POINT_2F,
        imageRectangle: *const D2D1_RECT_F,
        interpolationMode: D2D1_INTERPOLATION_MODE,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x7bfef914, 0x2d75, 0x4bad, 0xbe, 0x87, 0xe1, 0x8d, 0xdb, 0x07, 0x7b, 0x6d)]
interface ID2D1Device6(ID2D1Device6Vtbl): ID2D1Device5(ID2D1Device5Vtbl) {
    fn CreateDeviceContext(
        options: D2D1_DEVICE_CONTEXT_OPTIONS,
        deviceContext6: *mut *mut ID2D1DeviceContext6,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xbdc2bdd3, 0xb96c, 0x4de6, 0xbd, 0xf7, 0x99, 0xd4, 0x74, 0x54, 0x54, 0xde)]
interface ID2D1Factory7(ID2D1Factory7Vtbl): ID2D1Factory6(ID2D1Factory6Vtbl) {
    fn CreateDevice(
        dxgiDevice: *mut IDXGIDevice,
        d2dDevice6: *mut *mut ID2D1Device6,
    ) -> HRESULT,
}}
extern "system" {
    pub fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(
        pPoint0: *const D2D1_POINT_2F,
        pPoint1: *const D2D1_POINT_2F,
        pPoint2: *const D2D1_POINT_2F,
        pPoint3: *const D2D1_POINT_2F,
        pPoint4: *const D2D1_POINT_2F,
        pPoint5: *const D2D1_POINT_2F,
        pPoint6: *const D2D1_POINT_2F,
        pPoint7: *const D2D1_POINT_2F,
        pPoint8: *const D2D1_POINT_2F,
        pPoint9: *const D2D1_POINT_2F,
        pPoint10: *const D2D1_POINT_2F,
        pPoint11: *const D2D1_POINT_2F,
        pTensorPoint11: *mut D2D1_POINT_2F,
        pTensorPoint12: *mut D2D1_POINT_2F,
        pTensorPoint21: *mut D2D1_POINT_2F,
        pTensorPoint22: *mut D2D1_POINT_2F,
    );
}
