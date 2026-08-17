pub trait IGraphicsEffect_Impl: Sized + IGraphicsEffectSource_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetName(&self, name: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGraphicsEffect {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffect";
}
impl IGraphicsEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGraphicsEffect_Vtbl
    where
        Identity: IGraphicsEffect_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IGraphicsEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGraphicsEffect_Impl::Name(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IGraphicsEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGraphicsEffect_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGraphicsEffect, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGraphicsEffect as windows_core::Interface>::IID
    }
}
pub trait IGraphicsEffectSource_Impl: Sized {}
impl windows_core::RuntimeName for IGraphicsEffectSource {
    const NAME: &'static str = "Windows.Graphics.Effects.IGraphicsEffectSource";
}
impl IGraphicsEffectSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGraphicsEffectSource_Vtbl
    where
        Identity: IGraphicsEffectSource_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IGraphicsEffectSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGraphicsEffectSource as windows_core::Interface>::IID
    }
}
