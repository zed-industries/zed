pub trait IGeoshape_Impl: Sized {
    fn GeoshapeType(&self) -> windows_core::Result<GeoshapeType>;
    fn SpatialReferenceId(&self) -> windows_core::Result<u32>;
    fn AltitudeReferenceSystem(&self) -> windows_core::Result<AltitudeReferenceSystem>;
}
impl windows_core::RuntimeName for IGeoshape {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoshape";
}
impl IGeoshape_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGeoshape_Vtbl
    where
        Identity: IGeoshape_Impl,
    {
        unsafe extern "system" fn GeoshapeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut GeoshapeType) -> windows_core::HRESULT
        where
            Identity: IGeoshape_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGeoshape_Impl::GeoshapeType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialReferenceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGeoshape_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGeoshape_Impl::SpatialReferenceId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeReferenceSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> windows_core::HRESULT
        where
            Identity: IGeoshape_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGeoshape_Impl::AltitudeReferenceSystem(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGeoshape, OFFSET>(),
            GeoshapeType: GeoshapeType::<Identity, OFFSET>,
            SpatialReferenceId: SpatialReferenceId::<Identity, OFFSET>,
            AltitudeReferenceSystem: AltitudeReferenceSystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGeoshape as windows_core::Interface>::IID
    }
}
