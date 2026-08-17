windows_core::imp::define_interface!(IPwmControllerProvider, IPwmControllerProvider_Vtbl, 0x1300593b_e2e3_40a4_b7d9_48dff0377a52);
impl core::ops::Deref for IPwmControllerProvider {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
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
impl windows_core::RuntimeType for IPwmControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
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
impl core::ops::Deref for IPwmProvider {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPwmProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IPwmProvider {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IPwmProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPwmProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
