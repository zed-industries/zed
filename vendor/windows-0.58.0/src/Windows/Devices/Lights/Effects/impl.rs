pub trait ILampArrayEffect_Impl: Sized {
    fn ZIndex(&self) -> windows_core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILampArrayEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.ILampArrayEffect";
}
impl ILampArrayEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILampArrayEffect_Vtbl
    where
        Identity: ILampArrayEffect_Impl,
    {
        unsafe extern "system" fn ZIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT
        where
            Identity: ILampArrayEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILampArrayEffect_Impl::ZIndex(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT
        where
            Identity: ILampArrayEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILampArrayEffect_Impl::SetZIndex(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILampArrayEffect, OFFSET>(),
            ZIndex: ZIndex::<Identity, OFFSET>,
            SetZIndex: SetZIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILampArrayEffect as windows_core::Interface>::IID
    }
}
