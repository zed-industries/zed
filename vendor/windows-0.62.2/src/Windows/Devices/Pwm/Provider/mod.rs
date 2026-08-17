windows_core::imp::define_interface!(IPwmControllerProvider, IPwmControllerProvider_Vtbl, 0x1300593b_e2e3_40a4_b7d9_48dff0377a52);
impl windows_core::RuntimeType for IPwmControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPwmControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IPwmControllerProvider {
    pub fn PinCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualFrequency(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualFrequency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredFrequency(&self, frequency: f64) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetDesiredFrequency)(windows_core::Interface::as_raw(this), frequency, &mut result__).map(|| result__)
        }
    }
    pub fn MaxFrequency(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxFrequency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinFrequency(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinFrequency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AcquirePin(&self, pin: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcquirePin)(windows_core::Interface::as_raw(this), pin).ok() }
    }
    pub fn ReleasePin(&self, pin: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReleasePin)(windows_core::Interface::as_raw(this), pin).ok() }
    }
    pub fn EnablePin(&self, pin: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EnablePin)(windows_core::Interface::as_raw(this), pin).ok() }
    }
    pub fn DisablePin(&self, pin: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DisablePin)(windows_core::Interface::as_raw(this), pin).ok() }
    }
    pub fn SetPulseParameters(&self, pin: i32, dutycycle: f64, invertpolarity: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPulseParameters)(windows_core::Interface::as_raw(this), pin, dutycycle, invertpolarity).ok() }
    }
}
impl windows_core::RuntimeName for IPwmControllerProvider {
    const NAME: &'static str = "Windows.Devices.Pwm.Provider.IPwmControllerProvider";
}
pub trait IPwmControllerProvider_Impl: windows_core::IUnknownImpl {
    fn PinCount(&self) -> windows_core::Result<i32>;
    fn ActualFrequency(&self) -> windows_core::Result<f64>;
    fn SetDesiredFrequency(&self, frequency: f64) -> windows_core::Result<f64>;
    fn MaxFrequency(&self) -> windows_core::Result<f64>;
    fn MinFrequency(&self) -> windows_core::Result<f64>;
    fn AcquirePin(&self, pin: i32) -> windows_core::Result<()>;
    fn ReleasePin(&self, pin: i32) -> windows_core::Result<()>;
    fn EnablePin(&self, pin: i32) -> windows_core::Result<()>;
    fn DisablePin(&self, pin: i32) -> windows_core::Result<()>;
    fn SetPulseParameters(&self, pin: i32, dutyCycle: f64, invertPolarity: bool) -> windows_core::Result<()>;
}
impl IPwmControllerProvider_Vtbl {
    pub const fn new<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PinCount<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPwmControllerProvider_Impl::PinCount(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActualFrequency<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPwmControllerProvider_Impl::ActualFrequency(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredFrequency<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequency: f64, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPwmControllerProvider_Impl::SetDesiredFrequency(this, frequency) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaxFrequency<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPwmControllerProvider_Impl::MaxFrequency(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MinFrequency<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPwmControllerProvider_Impl::MinFrequency(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AcquirePin<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPwmControllerProvider_Impl::AcquirePin(this, pin).into()
            }
        }
        unsafe extern "system" fn ReleasePin<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPwmControllerProvider_Impl::ReleasePin(this, pin).into()
            }
        }
        unsafe extern "system" fn EnablePin<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPwmControllerProvider_Impl::EnablePin(this, pin).into()
            }
        }
        unsafe extern "system" fn DisablePin<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPwmControllerProvider_Impl::DisablePin(this, pin).into()
            }
        }
        unsafe extern "system" fn SetPulseParameters<Identity: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPwmControllerProvider_Impl::SetPulseParameters(this, pin, dutycycle, invertpolarity).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPwmControllerProvider, OFFSET>(),
            PinCount: PinCount::<Identity, OFFSET>,
            ActualFrequency: ActualFrequency::<Identity, OFFSET>,
            SetDesiredFrequency: SetDesiredFrequency::<Identity, OFFSET>,
            MaxFrequency: MaxFrequency::<Identity, OFFSET>,
            MinFrequency: MinFrequency::<Identity, OFFSET>,
            AcquirePin: AcquirePin::<Identity, OFFSET>,
            ReleasePin: ReleasePin::<Identity, OFFSET>,
            EnablePin: EnablePin::<Identity, OFFSET>,
            DisablePin: DisablePin::<Identity, OFFSET>,
            SetPulseParameters: SetPulseParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPwmControllerProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ActualFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDesiredFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut f64) -> windows_core::HRESULT,
    pub MaxFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MinFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AcquirePin: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReleasePin: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnablePin: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisablePin: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetPulseParameters: unsafe extern "system" fn(*mut core::ffi::c_void, i32, f64, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPwmProvider, IPwmProvider_Vtbl, 0xa3301228_52f1_47b0_9349_66ba43d25902);
impl windows_core::RuntimeType for IPwmProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPwmProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IPwmProvider {
    pub fn GetControllers(&self) -> windows_core::Result<windows_collections::IVectorView<IPwmControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IPwmProvider {
    const NAME: &'static str = "Windows.Devices.Pwm.Provider.IPwmProvider";
}
pub trait IPwmProvider_Impl: windows_core::IUnknownImpl {
    fn GetControllers(&self) -> windows_core::Result<windows_collections::IVectorView<IPwmControllerProvider>>;
}
impl IPwmProvider_Vtbl {
    pub const fn new<Identity: IPwmProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetControllers<Identity: IPwmProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPwmProvider_Impl::GetControllers(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPwmProvider, OFFSET>(), GetControllers: GetControllers::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPwmProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
