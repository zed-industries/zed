#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GRAPHICS_EFFECT_PROPERTY_MAPPING(pub i32);
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
windows_core::imp::define_interface!(IGeometrySource2DInterop, IGeometrySource2DInterop_Vtbl, 0x0657af73_53fd_47cf_84ff_c8492d2a80a3);
windows_core::imp::interface_hierarchy!(IGeometrySource2DInterop, windows_core::IUnknown);
impl IGeometrySource2DInterop {
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn GetGeometry(&self) -> windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn TryGetGeometryUsingFactory<P0>(&self, factory: P0) -> windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Direct2D::ID2D1Factory>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryGetGeometryUsingFactory)(windows_core::Interface::as_raw(self), factory.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Direct2D")]
pub trait IGeometrySource2DInterop_Impl: windows_core::IUnknownImpl {
    fn GetGeometry(&self) -> windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
    fn TryGetGeometryUsingFactory(&self, factory: windows_core::Ref<super::super::super::super::Graphics::Direct2D::ID2D1Factory>) -> windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl IGeometrySource2DInterop_Vtbl {
    pub const fn new<Identity: IGeometrySource2DInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGeometry<Identity: IGeometrySource2DInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGeometrySource2DInterop_Impl::GetGeometry(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TryGetGeometryUsingFactory<Identity: IGeometrySource2DInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGeometrySource2DInterop_Impl::TryGetGeometryUsingFactory(this, core::mem::transmute_copy(&factory)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGeometry: GetGeometry::<Identity, OFFSET>,
            TryGetGeometryUsingFactory: TryGetGeometryUsingFactory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGeometrySource2DInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl windows_core::RuntimeName for IGeometrySource2DInterop {}
windows_core::imp::define_interface!(IGraphicsEffectD2D1Interop, IGraphicsEffectD2D1Interop_Vtbl, 0x2fc57384_a068_44d7_a331_30982fcf7177);
windows_core::imp::interface_hierarchy!(IGraphicsEffectD2D1Interop, windows_core::IUnknown);
impl IGraphicsEffectD2D1Interop {
    pub unsafe fn GetEffectId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNamedPropertyMapping<P0>(&self, name: P0, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetNamedPropertyMapping)(windows_core::Interface::as_raw(self), name.param().abi(), index as _, mapping as _).ok() }
    }
    pub unsafe fn GetPropertyCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self, index: u32) -> windows_core::Result<super::super::super::super::super::Foundation::IPropertyValue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Effects")]
    pub unsafe fn GetSource(&self, index: u32) -> windows_core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSourceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectD2D1Interop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEffectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetNamedPropertyMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> windows_core::HRESULT,
    pub GetPropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Effects")]
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    GetSource: usize,
    pub GetSourceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Graphics_Effects")]
pub trait IGraphicsEffectD2D1Interop_Impl: windows_core::IUnknownImpl {
    fn GetEffectId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetNamedPropertyMapping(&self, name: &windows_core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> windows_core::Result<()>;
    fn GetPropertyCount(&self) -> windows_core::Result<u32>;
    fn GetProperty(&self, index: u32) -> windows_core::Result<super::super::super::super::super::Foundation::IPropertyValue>;
    fn GetSource(&self, index: u32) -> windows_core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource>;
    fn GetSourceCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Graphics_Effects")]
impl IGraphicsEffectD2D1Interop_Vtbl {
    pub const fn new<Identity: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEffectId<Identity: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2D1Interop_Impl::GetEffectId(this) {
                    Ok(ok__) => {
                        id.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNamedPropertyMapping<Identity: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGraphicsEffectD2D1Interop_Impl::GetNamedPropertyMapping(this, core::mem::transmute(&name), core::mem::transmute_copy(&index), core::mem::transmute_copy(&mapping)).into()
            }
        }
        unsafe extern "system" fn GetPropertyCount<Identity: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2D1Interop_Impl::GetPropertyCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2D1Interop_Impl::GetProperty(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSource<Identity: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, source: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2D1Interop_Impl::GetSource(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        source.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceCount<Identity: IGraphicsEffectD2D1Interop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGraphicsEffectD2D1Interop_Impl::GetSourceCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEffectId: GetEffectId::<Identity, OFFSET>,
            GetNamedPropertyMapping: GetNamedPropertyMapping::<Identity, OFFSET>,
            GetPropertyCount: GetPropertyCount::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGraphicsEffectD2D1Interop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Graphics_Effects")]
impl windows_core::RuntimeName for IGraphicsEffectD2D1Interop {}
