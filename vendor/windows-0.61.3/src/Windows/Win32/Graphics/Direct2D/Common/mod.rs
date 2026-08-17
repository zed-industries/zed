#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(pub i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(4i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(2i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(5i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(1i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(3i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_ALPHA_MODE(pub i32);
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(3i32);
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(1i32);
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(2i32);
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: windows_numerics::Vector2,
    pub point2: windows_numerics::Vector2,
    pub point3: windows_numerics::Vector2,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_BLEND_MODE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_BORDER_MODE(pub i32);
pub const D2D1_BORDER_MODE_HARD: D2D1_BORDER_MODE = D2D1_BORDER_MODE(1i32);
pub const D2D1_BORDER_MODE_SOFT: D2D1_BORDER_MODE = D2D1_BORDER_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_COLORMATRIX_ALPHA_MODE(pub i32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(1i32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_STRAIGHT: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_COMPOSITE_MODE(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_FIGURE_BEGIN(pub i32);
pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(0i32);
pub const D2D1_FIGURE_BEGIN_HOLLOW: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_FIGURE_END(pub i32);
pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = D2D1_FIGURE_END(1i32);
pub const D2D1_FIGURE_END_OPEN: D2D1_FIGURE_END = D2D1_FIGURE_END(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_FILL_MODE(pub i32);
pub const D2D1_FILL_MODE_ALTERNATE: D2D1_FILL_MODE = D2D1_FILL_MODE(0i32);
pub const D2D1_FILL_MODE_WINDING: D2D1_FILL_MODE = D2D1_FILL_MODE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: D2D1_COLOR_F,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_PATH_SEGMENT(pub i32);
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
pub const D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(2i32);
pub const D2D1_PATH_SEGMENT_FORCE_UNSTROKED: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(1i32);
pub const D2D1_PATH_SEGMENT_NONE: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(0i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::super::Dxgi::Common::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_TURBULENCE_NOISE(pub i32);
pub const D2D1_TURBULENCE_NOISE_FRACTAL_SUM: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(0i32);
pub const D2D1_TURBULENCE_NOISE_TURBULENCE: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_4X3_F {
    pub Anonymous: D2D_MATRIX_4X3_F_0,
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
impl Default for D2D_MATRIX_4X3_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_5X4_F {
    pub Anonymous: D2D_MATRIX_5X4_F_0,
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
impl Default for D2D_MATRIX_5X4_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_VECTOR_3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
windows_core::imp::define_interface!(ID2D1SimplifiedGeometrySink, ID2D1SimplifiedGeometrySink_Vtbl, 0x2cd9069e_12e2_11dc_9fed_001143a055f9);
windows_core::imp::interface_hierarchy!(ID2D1SimplifiedGeometrySink, windows_core::IUnknown);
impl ID2D1SimplifiedGeometrySink {
    pub unsafe fn SetFillMode(&self, fillmode: D2D1_FILL_MODE) {
        unsafe { (windows_core::Interface::vtable(self).SetFillMode)(windows_core::Interface::as_raw(self), fillmode) }
    }
    pub unsafe fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT) {
        unsafe { (windows_core::Interface::vtable(self).SetSegmentFlags)(windows_core::Interface::as_raw(self), vertexflags) }
    }
    pub unsafe fn BeginFigure(&self, startpoint: windows_numerics::Vector2, figurebegin: D2D1_FIGURE_BEGIN) {
        unsafe { (windows_core::Interface::vtable(self).BeginFigure)(windows_core::Interface::as_raw(self), core::mem::transmute(startpoint), figurebegin) }
    }
    pub unsafe fn AddLines(&self, points: &[windows_numerics::Vector2]) {
        unsafe { (windows_core::Interface::vtable(self).AddLines)(windows_core::Interface::as_raw(self), core::mem::transmute(points.as_ptr()), points.len().try_into().unwrap()) }
    }
    pub unsafe fn AddBeziers(&self, beziers: &[D2D1_BEZIER_SEGMENT]) {
        unsafe { (windows_core::Interface::vtable(self).AddBeziers)(windows_core::Interface::as_raw(self), core::mem::transmute(beziers.as_ptr()), beziers.len().try_into().unwrap()) }
    }
    pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
        unsafe { (windows_core::Interface::vtable(self).EndFigure)(windows_core::Interface::as_raw(self), figureend) }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SimplifiedGeometrySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFillMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FILL_MODE),
    pub SetSegmentFlags: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PATH_SEGMENT),
    pub BeginFigure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, D2D1_FIGURE_BEGIN),
    pub AddLines: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2, u32),
    pub AddBeziers: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_BEZIER_SEGMENT, u32),
    pub EndFigure: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FIGURE_END),
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1SimplifiedGeometrySink {}
unsafe impl Sync for ID2D1SimplifiedGeometrySink {}
pub trait ID2D1SimplifiedGeometrySink_Impl: windows_core::IUnknownImpl {
    fn SetFillMode(&self, fillmode: D2D1_FILL_MODE);
    fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT);
    fn BeginFigure(&self, startpoint: &windows_numerics::Vector2, figurebegin: D2D1_FIGURE_BEGIN);
    fn AddLines(&self, points: *const windows_numerics::Vector2, pointscount: u32);
    fn AddBeziers(&self, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32);
    fn EndFigure(&self, figureend: D2D1_FIGURE_END);
    fn Close(&self) -> windows_core::Result<()>;
}
impl ID2D1SimplifiedGeometrySink_Vtbl {
    pub const fn new<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFillMode<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillmode: D2D1_FILL_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::SetFillMode(this, core::mem::transmute_copy(&fillmode))
            }
        }
        unsafe extern "system" fn SetSegmentFlags<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexflags: D2D1_PATH_SEGMENT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::SetSegmentFlags(this, core::mem::transmute_copy(&vertexflags))
            }
        }
        unsafe extern "system" fn BeginFigure<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: windows_numerics::Vector2, figurebegin: D2D1_FIGURE_BEGIN) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::BeginFigure(this, core::mem::transmute(&startpoint), core::mem::transmute_copy(&figurebegin))
            }
        }
        unsafe extern "system" fn AddLines<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *const windows_numerics::Vector2, pointscount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::AddLines(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount))
            }
        }
        unsafe extern "system" fn AddBeziers<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::AddBeziers(this, core::mem::transmute_copy(&beziers), core::mem::transmute_copy(&bezierscount))
            }
        }
        unsafe extern "system" fn EndFigure<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, figureend: D2D1_FIGURE_END) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::EndFigure(this, core::mem::transmute_copy(&figureend))
            }
        }
        unsafe extern "system" fn Close<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFillMode: SetFillMode::<Identity, OFFSET>,
            SetSegmentFlags: SetSegmentFlags::<Identity, OFFSET>,
            BeginFigure: BeginFigure::<Identity, OFFSET>,
            AddLines: AddLines::<Identity, OFFSET>,
            AddBeziers: AddBeziers::<Identity, OFFSET>,
            EndFigure: EndFigure::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SimplifiedGeometrySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1SimplifiedGeometrySink {}
