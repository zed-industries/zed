windows_core::imp::define_interface!(ID2D1SimplifiedGeometrySink, ID2D1SimplifiedGeometrySink_Vtbl, 0x2cd9069e_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1SimplifiedGeometrySink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1SimplifiedGeometrySink, windows_core::IUnknown);
impl ID2D1SimplifiedGeometrySink {
    pub unsafe fn SetFillMode(&self, fillmode: D2D1_FILL_MODE) {
        (windows_core::Interface::vtable(self).SetFillMode)(windows_core::Interface::as_raw(self), fillmode)
    }
    pub unsafe fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT) {
        (windows_core::Interface::vtable(self).SetSegmentFlags)(windows_core::Interface::as_raw(self), vertexflags)
    }
    pub unsafe fn BeginFigure(&self, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
        (windows_core::Interface::vtable(self).BeginFigure)(windows_core::Interface::as_raw(self), core::mem::transmute(startpoint), figurebegin)
    }
    pub unsafe fn AddLines(&self, points: &[D2D_POINT_2F]) {
        (windows_core::Interface::vtable(self).AddLines)(windows_core::Interface::as_raw(self), core::mem::transmute(points.as_ptr()), points.len().try_into().unwrap())
    }
    pub unsafe fn AddBeziers(&self, beziers: &[D2D1_BEZIER_SEGMENT]) {
        (windows_core::Interface::vtable(self).AddBeziers)(windows_core::Interface::as_raw(self), core::mem::transmute(beziers.as_ptr()), beziers.len().try_into().unwrap())
    }
    pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
        (windows_core::Interface::vtable(self).EndFigure)(windows_core::Interface::as_raw(self), figureend)
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl Send for ID2D1SimplifiedGeometrySink {}
unsafe impl Sync for ID2D1SimplifiedGeometrySink {}
#[repr(C)]
pub struct ID2D1SimplifiedGeometrySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFillMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FILL_MODE),
    pub SetSegmentFlags: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PATH_SEGMENT),
    pub BeginFigure: unsafe extern "system" fn(*mut core::ffi::c_void, D2D_POINT_2F, D2D1_FIGURE_BEGIN),
    pub AddLines: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_POINT_2F, u32),
    pub AddBeziers: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_BEZIER_SEGMENT, u32),
    pub EndFigure: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FIGURE_END),
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(4i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(2i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(5i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(1i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(3i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(0i32);
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(3i32);
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(1i32);
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(2i32);
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(0i32);
pub const D2D1_BLEND_MODE_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(22i32);
pub const D2D1_BLEND_MODE_COLOR_BURN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(5i32);
pub const D2D1_BLEND_MODE_COLOR_DODGE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(9i32);
pub const D2D1_BLEND_MODE_DARKEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(2i32);
pub const D2D1_BLEND_MODE_DARKER_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(7i32);
pub const D2D1_BLEND_MODE_DIFFERENCE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(18i32);
pub const D2D1_BLEND_MODE_DISSOLVE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(4i32);
pub const D2D1_BLEND_MODE_DIVISION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(25i32);
pub const D2D1_BLEND_MODE_EXCLUSION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(19i32);
pub const D2D1_BLEND_MODE_HARD_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(13i32);
pub const D2D1_BLEND_MODE_HARD_MIX: D2D1_BLEND_MODE = D2D1_BLEND_MODE(17i32);
pub const D2D1_BLEND_MODE_HUE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(20i32);
pub const D2D1_BLEND_MODE_LIGHTEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(3i32);
pub const D2D1_BLEND_MODE_LIGHTER_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(8i32);
pub const D2D1_BLEND_MODE_LINEAR_BURN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(6i32);
pub const D2D1_BLEND_MODE_LINEAR_DODGE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(10i32);
pub const D2D1_BLEND_MODE_LINEAR_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(15i32);
pub const D2D1_BLEND_MODE_LUMINOSITY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(23i32);
pub const D2D1_BLEND_MODE_MULTIPLY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(0i32);
pub const D2D1_BLEND_MODE_OVERLAY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(11i32);
pub const D2D1_BLEND_MODE_PIN_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(16i32);
pub const D2D1_BLEND_MODE_SATURATION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(21i32);
pub const D2D1_BLEND_MODE_SCREEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(1i32);
pub const D2D1_BLEND_MODE_SOFT_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(12i32);
pub const D2D1_BLEND_MODE_SUBTRACT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(24i32);
pub const D2D1_BLEND_MODE_VIVID_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(14i32);
pub const D2D1_BORDER_MODE_HARD: D2D1_BORDER_MODE = D2D1_BORDER_MODE(1i32);
pub const D2D1_BORDER_MODE_SOFT: D2D1_BORDER_MODE = D2D1_BORDER_MODE(0i32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(1i32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_STRAIGHT: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(2i32);
pub const D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(11i32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_ATOP: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(7i32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_IN: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(3i32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_OUT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(5i32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_OVER: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(1i32);
pub const D2D1_COMPOSITE_MODE_MASK_INVERT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(12i32);
pub const D2D1_COMPOSITE_MODE_PLUS: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(9i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_ATOP: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(6i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_COPY: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(10i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_IN: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(2i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_OUT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(4i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_OVER: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(0i32);
pub const D2D1_COMPOSITE_MODE_XOR: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(8i32);
pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(0i32);
pub const D2D1_FIGURE_BEGIN_HOLLOW: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(1i32);
pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = D2D1_FIGURE_END(1i32);
pub const D2D1_FIGURE_END_OPEN: D2D1_FIGURE_END = D2D1_FIGURE_END(0i32);
pub const D2D1_FILL_MODE_ALTERNATE: D2D1_FILL_MODE = D2D1_FILL_MODE(0i32);
pub const D2D1_FILL_MODE_WINDING: D2D1_FILL_MODE = D2D1_FILL_MODE(1i32);
pub const D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(2i32);
pub const D2D1_PATH_SEGMENT_FORCE_UNSTROKED: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(1i32);
pub const D2D1_PATH_SEGMENT_NONE: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(0i32);
pub const D2D1_TURBULENCE_NOISE_FRACTAL_SUM: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(0i32);
pub const D2D1_TURBULENCE_NOISE_TURBULENCE: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(pub i32);
impl windows_core::TypeKind for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_ALPHA_MODE(pub i32);
impl windows_core::TypeKind for D2D1_ALPHA_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_ALPHA_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_ALPHA_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BLEND_MODE(pub i32);
impl windows_core::TypeKind for D2D1_BLEND_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BLEND_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BLEND_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_BORDER_MODE(pub i32);
impl windows_core::TypeKind for D2D1_BORDER_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_BORDER_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_BORDER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COLORMATRIX_ALPHA_MODE(pub i32);
impl windows_core::TypeKind for D2D1_COLORMATRIX_ALPHA_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COLORMATRIX_ALPHA_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COLORMATRIX_ALPHA_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_COMPOSITE_MODE(pub i32);
impl windows_core::TypeKind for D2D1_COMPOSITE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_COMPOSITE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_COMPOSITE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_FIGURE_BEGIN(pub i32);
impl windows_core::TypeKind for D2D1_FIGURE_BEGIN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_FIGURE_BEGIN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_FIGURE_BEGIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_FIGURE_END(pub i32);
impl windows_core::TypeKind for D2D1_FIGURE_END {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_FIGURE_END {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_FIGURE_END").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_FILL_MODE(pub i32);
impl windows_core::TypeKind for D2D1_FILL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_FILL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_FILL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_PATH_SEGMENT(pub i32);
impl windows_core::TypeKind for D2D1_PATH_SEGMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_PATH_SEGMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_PATH_SEGMENT").field(&self.0).finish()
    }
}
impl D2D1_PATH_SEGMENT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for D2D1_PATH_SEGMENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for D2D1_PATH_SEGMENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct D2D1_TURBULENCE_NOISE(pub i32);
impl windows_core::TypeKind for D2D1_TURBULENCE_NOISE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for D2D1_TURBULENCE_NOISE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("D2D1_TURBULENCE_NOISE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: D2D_POINT_2F,
    pub point2: D2D_POINT_2F,
    pub point3: D2D_POINT_2F,
}
impl windows_core::TypeKind for D2D1_BEZIER_SEGMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl windows_core::TypeKind for D2D1_COLOR_F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_COLOR_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: D2D1_COLOR_F,
}
impl windows_core::TypeKind for D2D1_GRADIENT_STOP {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D1_GRADIENT_STOP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::super::Dxgi::Common::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for D2D1_PIXEL_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for D2D1_PIXEL_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl windows_core::TypeKind for D2D_COLOR_F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_COLOR_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_4X3_F {
    pub Anonymous: D2D_MATRIX_4X3_F_0,
}
impl windows_core::TypeKind for D2D_MATRIX_4X3_F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_4X3_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_4X3_F_0 {
    pub Anonymous: D2D_MATRIX_4X3_F_0_0,
    pub m: [f32; 12],
}
impl windows_core::TypeKind for D2D_MATRIX_4X3_F_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_4X3_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_MATRIX_4X3_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
}
impl windows_core::TypeKind for D2D_MATRIX_4X3_F_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_4X3_F_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_4X4_F {
    pub Anonymous: D2D_MATRIX_4X4_F_0,
}
impl windows_core::TypeKind for D2D_MATRIX_4X4_F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_4X4_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_4X4_F_0 {
    pub Anonymous: D2D_MATRIX_4X4_F_0_0,
    pub m: [f32; 16],
}
impl windows_core::TypeKind for D2D_MATRIX_4X4_F_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_4X4_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_MATRIX_4X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl windows_core::TypeKind for D2D_MATRIX_4X4_F_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_4X4_F_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_5X4_F {
    pub Anonymous: D2D_MATRIX_5X4_F_0,
}
impl windows_core::TypeKind for D2D_MATRIX_5X4_F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_5X4_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_5X4_F_0 {
    pub Anonymous: D2D_MATRIX_5X4_F_0_0,
    pub m: [f32; 20],
}
impl windows_core::TypeKind for D2D_MATRIX_5X4_F_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_5X4_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_MATRIX_5X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
    pub _51: f32,
    pub _52: f32,
    pub _53: f32,
    pub _54: f32,
}
impl windows_core::TypeKind for D2D_MATRIX_5X4_F_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_MATRIX_5X4_F_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
impl windows_core::TypeKind for D2D_POINT_2F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_POINT_2F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
}
impl windows_core::TypeKind for D2D_POINT_2U {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_POINT_2U {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl windows_core::TypeKind for D2D_RECT_F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_RECT_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
impl windows_core::TypeKind for D2D_RECT_U {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_RECT_U {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
impl windows_core::TypeKind for D2D_SIZE_F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_SIZE_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
impl windows_core::TypeKind for D2D_SIZE_U {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_SIZE_U {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_VECTOR_2F {
    pub x: f32,
    pub y: f32,
}
impl windows_core::TypeKind for D2D_VECTOR_2F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_VECTOR_2F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_VECTOR_3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl windows_core::TypeKind for D2D_VECTOR_3F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_VECTOR_3F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D2D_VECTOR_4F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl windows_core::TypeKind for D2D_VECTOR_4F {
    type TypeKind = windows_core::CopyType;
}
impl Default for D2D_VECTOR_4F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
