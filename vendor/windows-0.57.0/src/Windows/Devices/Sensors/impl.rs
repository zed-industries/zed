pub trait IHumanPresenceSensorExtension_Impl: Sized {
    fn Initialize(&self, deviceinterface: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn ProcessReading(&self, reading: Option<&HumanPresenceSensorReading>) -> windows_core::Result<HumanPresenceSensorReadingUpdate>;
    fn ProcessReadingTimeoutExpired(&self, reading: Option<&HumanPresenceSensorReading>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHumanPresenceSensorExtension {
    const NAME: &'static str = "Windows.Devices.Sensors.IHumanPresenceSensorExtension";
}
impl IHumanPresenceSensorExtension_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>() -> IHumanPresenceSensorExtension_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceinterface: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHumanPresenceSensorExtension_Impl::Initialize(this, core::mem::transmute(&deviceinterface)).into()
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHumanPresenceSensorExtension_Impl::Start(this).into()
        }
        unsafe extern "system" fn ProcessReading<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reading: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IHumanPresenceSensorExtension_Impl::ProcessReading(this, windows_core::from_raw_borrowed(&reading)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessReadingTimeoutExpired<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reading: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHumanPresenceSensorExtension_Impl::ProcessReadingTimeoutExpired(this, windows_core::from_raw_borrowed(&reading)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHumanPresenceSensorExtension_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHumanPresenceSensorExtension_Impl::Uninitialize(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHumanPresenceSensorExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHumanPresenceSensorExtension_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHumanPresenceSensorExtension, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            ProcessReading: ProcessReading::<Identity, Impl, OFFSET>,
            ProcessReadingTimeoutExpired: ProcessReadingTimeoutExpired::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHumanPresenceSensorExtension as windows_core::Interface>::IID
    }
}
pub trait ISensorDataThreshold_Impl: Sized {}
impl windows_core::RuntimeName for ISensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorDataThreshold";
}
impl ISensorDataThreshold_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorDataThreshold_Impl, const OFFSET: isize>() -> ISensorDataThreshold_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ISensorDataThreshold, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorDataThreshold as windows_core::Interface>::IID
    }
}
