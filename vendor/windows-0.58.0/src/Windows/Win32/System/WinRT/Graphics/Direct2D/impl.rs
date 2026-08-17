#[cfg(feature = "Win32_Graphics_Direct2D")]
pub trait IGeometrySource2DInterop_Impl: Sized {
    fn GetGeometry(&self) -> windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
    fn TryGetGeometryUsingFactory(&self, factory: Option<&super::super::super::super::Graphics::Direct2D::ID2D1Factory>) -> windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>;
}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl windows_core::RuntimeName for IGeometrySource2DInterop {}
#[cfg(feature = "Win32_Graphics_Direct2D")]
impl IGeometrySource2DInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGeometrySource2DInterop_Vtbl
    where
        Identity: IGeometrySource2DInterop_Impl,
    {
        unsafe extern "system" fn GetGeometry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGeometrySource2DInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGeometrySource2DInterop_Impl::GetGeometry(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetGeometryUsingFactory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGeometrySource2DInterop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGeometrySource2DInterop_Impl::TryGetGeometryUsingFactory(this, windows_core::from_raw_borrowed(&factory)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
pub trait IGraphicsEffectD2D1Interop_Impl: Sized {
    fn GetEffectId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetNamedPropertyMapping(&self, name: &windows_core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> windows_core::Result<()>;
    fn GetPropertyCount(&self) -> windows_core::Result<u32>;
    fn GetProperty(&self, index: u32) -> windows_core::Result<super::super::super::super::super::Foundation::IPropertyValue>;
    fn GetSource(&self, index: u32) -> windows_core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource>;
    fn GetSourceCount(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
impl windows_core::RuntimeName for IGraphicsEffectD2D1Interop {}
#[cfg(all(feature = "Foundation", feature = "Graphics_Effects"))]
impl IGraphicsEffectD2D1Interop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGraphicsEffectD2D1Interop_Vtbl
    where
        Identity: IGraphicsEffectD2D1Interop_Impl,
    {
        unsafe extern "system" fn GetEffectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IGraphicsEffectD2D1Interop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGraphicsEffectD2D1Interop_Impl::GetEffectId(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNamedPropertyMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> windows_core::HRESULT
        where
            Identity: IGraphicsEffectD2D1Interop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGraphicsEffectD2D1Interop_Impl::GetNamedPropertyMapping(this, core::mem::transmute(&name), core::mem::transmute_copy(&index), core::mem::transmute_copy(&mapping)).into()
        }
        unsafe extern "system" fn GetPropertyCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGraphicsEffectD2D1Interop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGraphicsEffectD2D1Interop_Impl::GetPropertyCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGraphicsEffectD2D1Interop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGraphicsEffectD2D1Interop_Impl::GetProperty(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, source: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGraphicsEffectD2D1Interop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGraphicsEffectD2D1Interop_Impl::GetSource(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    source.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGraphicsEffectD2D1Interop_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGraphicsEffectD2D1Interop_Impl::GetSourceCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
