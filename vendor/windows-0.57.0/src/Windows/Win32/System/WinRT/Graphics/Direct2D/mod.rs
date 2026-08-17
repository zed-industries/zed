windows_core::imp::define_interface!(IGeometrySource2DInterop, IGeometrySource2DInterop_Vtbl, 0x0657af73_53fd_47cf_84ff_c8492d2a80a3);
impl core::ops::Deref for IGeometrySource2DInterop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGeometrySource2DInterop, windows_core::IUnknown);
impl IGeometrySource2DInterop {
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn GetGeometry(&self) -> windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn TryGetGeometryUsingFactory<P0>(&self, factory: P0) -> windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Direct2D::ID2D1Factory>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TryGetGeometryUsingFactory)(windows_core::Interface::as_raw(self), factory.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IGeometrySource2DInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub GetGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    GetGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub TryGetGeometryUsingFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    TryGetGeometryUsingFactory: usize,
}
windows_core::imp::define_interface!(IGraphicsEffectD2D1Interop, IGraphicsEffectD2D1Interop_Vtbl, 0x2fc57384_a068_44d7_a331_30982fcf7177);
impl core::ops::Deref for IGraphicsEffectD2D1Interop {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGraphicsEffectD2D1Interop, windows_core::IUnknown);
impl IGraphicsEffectD2D1Interop {
    pub unsafe fn GetEffectId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEffectId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNamedPropertyMapping<P0>(&self, name: P0, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetNamedPropertyMapping)(windows_core::Interface::as_raw(self), name.param().abi(), index, mapping).ok()
    }
    pub unsafe fn GetPropertyCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Foundation")]
    pub unsafe fn GetProperty(&self, index: u32) -> windows_core::Result<super::super::super::super::super::Foundation::IPropertyValue> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Graphics_Effects")]
    pub unsafe fn GetSource(&self, index: u32) -> windows_core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSourceCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGraphicsEffectD2D1Interop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEffectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetNamedPropertyMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> windows_core::HRESULT,
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProperty: usize,
    #[cfg(feature = "Graphics_Effects")]
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    GetSource: usize,
    pub GetSourceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLORMATRIX_ALPHA_MODE: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(8i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR3: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(9i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(10i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(1i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RADIANS_TO_DEGREES: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(7i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RECT_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(6i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_UNKNOWN: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(0i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORW: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(5i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORX: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(2i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORY: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(3i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORZ: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(4i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GRAPHICS_EFFECT_PROPERTY_MAPPING(pub i32);
impl windows_core::TypeKind for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GRAPHICS_EFFECT_PROPERTY_MAPPING").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
