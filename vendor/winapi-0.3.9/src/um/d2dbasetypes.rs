// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of d2dbasetypes.h
use shared::d3d9types::D3DCOLORVALUE;
// FIXME: Remove in next major version
pub use um::dcommon::{
    D2D1_MATRIX_3X2_F, D2D1_POINT_2F, D2D1_POINT_2L, D2D1_POINT_2U, D2D1_RECT_F, D2D1_RECT_L,
    D2D1_RECT_U, D2D1_SIZE_F, D2D1_SIZE_U, D2D_MATRIX_3X2_F, D2D_MATRIX_4X3_F, D2D_MATRIX_4X4_F,
    D2D_MATRIX_5X4_F, D2D_POINT_2F, D2D_POINT_2L, D2D_POINT_2U, D2D_RECT_F, D2D_RECT_L, D2D_RECT_U,
    D2D_SIZE_F, D2D_SIZE_U, D2D_VECTOR_2F, D2D_VECTOR_3F, D2D_VECTOR_4F,
};
pub type D2D_COLOR_F = D3DCOLORVALUE;
