// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of dcommon.h
use ctypes::c_void;
use shared::basetsd::UINT32;
use shared::dxgiformat::DXGI_FORMAT;
use shared::minwindef::FLOAT;
use shared::windef::{POINT, RECT};
ENUM!{enum DWRITE_MEASURING_MODE {
    DWRITE_MEASURING_MODE_NATURAL = 0,
    DWRITE_MEASURING_MODE_GDI_CLASSIC = 1,
    DWRITE_MEASURING_MODE_GDI_NATURAL = 2,
}}
ENUM!{enum DWRITE_GLYPH_IMAGE_FORMATS {
    DWRITE_GLYPH_IMAGE_FORMATS_NONE = 0x00000000,
    DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE = 0x00000001,
    DWRITE_GLYPH_IMAGE_FORMATS_CFF = 0x00000002,
    DWRITE_GLYPH_IMAGE_FORMATS_COLR = 0x00000004,
    DWRITE_GLYPH_IMAGE_FORMATS_SVG = 0x00000008,
    DWRITE_GLYPH_IMAGE_FORMATS_PNG = 0x00000010,
    DWRITE_GLYPH_IMAGE_FORMATS_JPEG = 0x00000020,
    DWRITE_GLYPH_IMAGE_FORMATS_TIFF = 0x00000040,
    DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8 = 0x00000080,
}}
STRUCT!{struct DWRITE_GLYPH_IMAGE_DATA {
    imageData: *const c_void,
    imageDataSize: UINT32,
    uniqueDataId: UINT32,
    pixelsPerEm: UINT32,
    pixelSize: D2D1_SIZE_U,
    horizontalLeftOrigin: D2D1_POINT_2L,
    horizontalRightOrigin: D2D1_POINT_2L,
    verticalTopOrigin: D2D1_POINT_2L,
    verticalBottomOrigin: D2D1_POINT_2L,
}}
ENUM!{enum D2D1_ALPHA_MODE {
    D2D1_ALPHA_MODE_UNKNOWN = 0,
    D2D1_ALPHA_MODE_PREMULTIPLIED = 1,
    D2D1_ALPHA_MODE_STRAIGHT = 2,
    D2D1_ALPHA_MODE_IGNORE = 3,
}}
STRUCT!{struct D2D1_PIXEL_FORMAT {
    format: DXGI_FORMAT,
    alphaMode: D2D1_ALPHA_MODE,
}}
STRUCT!{struct D2D_POINT_2U {
    x: UINT32,
    y: UINT32,
}}
STRUCT!{struct D2D_POINT_2F {
    x: FLOAT,
    y: FLOAT,
}}
pub type D2D_POINT_2L = POINT;
STRUCT!{struct D2D_VECTOR_2F {
    x: FLOAT,
    y: FLOAT,
}}
STRUCT!{struct D2D_VECTOR_3F {
    x: FLOAT,
    y: FLOAT,
    z: FLOAT,
}}
STRUCT!{struct D2D_VECTOR_4F {
    x: FLOAT,
    y: FLOAT,
    z: FLOAT,
    w: FLOAT,
}}
STRUCT!{struct D2D_RECT_F {
    left: FLOAT,
    top: FLOAT,
    right: FLOAT,
    bottom: FLOAT,
}}
STRUCT!{struct D2D_RECT_U {
    left: UINT32,
    top: UINT32,
    right: UINT32,
    bottom: UINT32,
}}
pub type D2D_RECT_L = RECT;
STRUCT!{struct D2D_SIZE_F {
    width: FLOAT,
    height: FLOAT,
}}
STRUCT!{struct D2D_SIZE_U {
    width: UINT32,
    height: UINT32,
}}
STRUCT!{struct D2D_MATRIX_3X2_F {
    matrix: [[FLOAT; 2]; 3],
}}
STRUCT!{struct D2D_MATRIX_4X3_F {
    matrix: [[FLOAT; 3]; 4],
}}
STRUCT!{struct D2D_MATRIX_4X4_F {
    matrix: [[FLOAT; 4]; 4],
}}
STRUCT!{struct D2D_MATRIX_5X4_F {
    matrix: [[FLOAT; 4]; 5],
}}
pub type D2D1_POINT_2F = D2D_POINT_2F;
pub type D2D1_POINT_2U = D2D_POINT_2U;
pub type D2D1_POINT_2L = D2D_POINT_2L;
pub type D2D1_RECT_F = D2D_RECT_F;
pub type D2D1_RECT_U = D2D_RECT_U;
pub type D2D1_RECT_L = D2D_RECT_L;
pub type D2D1_SIZE_F = D2D_SIZE_F;
pub type D2D1_SIZE_U = D2D_SIZE_U;
pub type D2D1_MATRIX_3X2_F = D2D_MATRIX_3X2_F;
