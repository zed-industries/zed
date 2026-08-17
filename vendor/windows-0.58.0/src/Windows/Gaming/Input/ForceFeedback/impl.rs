pub trait IForceFeedbackEffect_Impl: Sized {
    fn Gain(&self) -> windows_core::Result<f64>;
    fn SetGain(&self, value: f64) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<ForceFeedbackEffectState>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IForceFeedbackEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.IForceFeedbackEffect";
}
impl IForceFeedbackEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IForceFeedbackEffect_Vtbl
    where
        Identity: IForceFeedbackEffect_Impl,
    {
        unsafe extern "system" fn Gain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT
        where
            Identity: IForceFeedbackEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IForceFeedbackEffect_Impl::Gain(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT
        where
            Identity: IForceFeedbackEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IForceFeedbackEffect_Impl::SetGain(this, value).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ForceFeedbackEffectState) -> windows_core::HRESULT
        where
            Identity: IForceFeedbackEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IForceFeedbackEffect_Impl::State(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IForceFeedbackEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IForceFeedbackEffect_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IForceFeedbackEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IForceFeedbackEffect_Impl::Stop(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IForceFeedbackEffect, OFFSET>(),
            Gain: Gain::<Identity, OFFSET>,
            SetGain: SetGain::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IForceFeedbackEffect as windows_core::Interface>::IID
    }
}
