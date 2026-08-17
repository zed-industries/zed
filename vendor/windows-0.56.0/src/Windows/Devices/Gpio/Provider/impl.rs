pub trait IGpioControllerProvider_Impl: Sized {
    fn PinCount(&self) -> windows_core::Result<i32>;
    fn OpenPinProvider(&self, pin: i32, sharingmode: ProviderGpioSharingMode) -> windows_core::Result<IGpioPinProvider>;
}
impl windows_core::RuntimeName for IGpioControllerProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioControllerProvider";
}
impl IGpioControllerProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: isize>() -> IGpioControllerProvider_Vtbl {
        unsafe extern "system" fn PinCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioControllerProvider_Impl::PinCount(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPinProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioControllerProvider_Impl::OpenPinProvider(this, pin, sharingmode) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGpioControllerProvider, OFFSET>(),
            PinCount: PinCount::<Identity, Impl, OFFSET>,
            OpenPinProvider: OpenPinProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGpioControllerProvider as windows_core::Interface>::IID
    }
}
pub trait IGpioPinProvider_Impl: Sized {
    fn ValueChanged(&self, handler: Option<&super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()>;
    fn DebounceTimeout(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDebounceTimeout(&self, value: &super::super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
    fn PinNumber(&self) -> windows_core::Result<i32>;
    fn SharingMode(&self) -> windows_core::Result<ProviderGpioSharingMode>;
    fn IsDriveModeSupported(&self, drivemode: ProviderGpioPinDriveMode) -> windows_core::Result<bool>;
    fn GetDriveMode(&self) -> windows_core::Result<ProviderGpioPinDriveMode>;
    fn SetDriveMode(&self, value: ProviderGpioPinDriveMode) -> windows_core::Result<()>;
    fn Write(&self, value: ProviderGpioPinValue) -> windows_core::Result<()>;
    fn Read(&self) -> windows_core::Result<ProviderGpioPinValue>;
}
impl windows_core::RuntimeName for IGpioPinProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioPinProvider";
}
impl IGpioPinProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>() -> IGpioPinProvider_Vtbl {
        unsafe extern "system" fn ValueChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioPinProvider_Impl::ValueChanged(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGpioPinProvider_Impl::RemoveValueChanged(this, core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn DebounceTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioPinProvider_Impl::DebounceTimeout(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDebounceTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGpioPinProvider_Impl::SetDebounceTimeout(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn PinNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioPinProvider_Impl::PinNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProviderGpioSharingMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioPinProvider_Impl::SharingMode(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDriveModeSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioPinProvider_Impl::IsDriveModeSupported(this, drivemode) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDriveMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProviderGpioPinDriveMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioPinProvider_Impl::GetDriveMode(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDriveMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ProviderGpioPinDriveMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGpioPinProvider_Impl::SetDriveMode(this, value).into()
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ProviderGpioPinValue) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGpioPinProvider_Impl::Write(this, value).into()
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioPinProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProviderGpioPinValue) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioPinProvider_Impl::Read(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGpioPinProvider, OFFSET>(),
            ValueChanged: ValueChanged::<Identity, Impl, OFFSET>,
            RemoveValueChanged: RemoveValueChanged::<Identity, Impl, OFFSET>,
            DebounceTimeout: DebounceTimeout::<Identity, Impl, OFFSET>,
            SetDebounceTimeout: SetDebounceTimeout::<Identity, Impl, OFFSET>,
            PinNumber: PinNumber::<Identity, Impl, OFFSET>,
            SharingMode: SharingMode::<Identity, Impl, OFFSET>,
            IsDriveModeSupported: IsDriveModeSupported::<Identity, Impl, OFFSET>,
            GetDriveMode: GetDriveMode::<Identity, Impl, OFFSET>,
            SetDriveMode: SetDriveMode::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGpioPinProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IGpioProvider_Impl: Sized {
    fn GetControllers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IGpioProvider {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.IGpioProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IGpioProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioProvider_Impl, const OFFSET: isize>() -> IGpioProvider_Vtbl {
        unsafe extern "system" fn GetControllers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGpioProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGpioProvider_Impl::GetControllers(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IGpioProvider, OFFSET>(), GetControllers: GetControllers::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGpioProvider as windows_core::Interface>::IID
    }
}
