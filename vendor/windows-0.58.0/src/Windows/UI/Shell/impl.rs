pub trait IAdaptiveCard_Impl: Sized {
    fn ToJson(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IAdaptiveCard {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCard";
}
impl IAdaptiveCard_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAdaptiveCard_Vtbl
    where
        Identity: IAdaptiveCard_Impl,
    {
        unsafe extern "system" fn ToJson<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IAdaptiveCard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAdaptiveCard_Impl::ToJson(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAdaptiveCard, OFFSET>(), ToJson: ToJson::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdaptiveCard as windows_core::Interface>::IID
    }
}
pub trait IAdaptiveCardBuilderStatics_Impl: Sized {
    fn CreateAdaptiveCardFromJson(&self, value: &windows_core::HSTRING) -> windows_core::Result<IAdaptiveCard>;
}
impl windows_core::RuntimeName for IAdaptiveCardBuilderStatics {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCardBuilderStatics";
}
impl IAdaptiveCardBuilderStatics_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAdaptiveCardBuilderStatics_Vtbl
    where
        Identity: IAdaptiveCardBuilderStatics_Impl,
    {
        unsafe extern "system" fn CreateAdaptiveCardFromJson<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAdaptiveCardBuilderStatics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAdaptiveCardBuilderStatics_Impl::CreateAdaptiveCardFromJson(this, core::mem::transmute(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAdaptiveCardBuilderStatics, OFFSET>(),
            CreateAdaptiveCardFromJson: CreateAdaptiveCardFromJson::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdaptiveCardBuilderStatics as windows_core::Interface>::IID
    }
}
