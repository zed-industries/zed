pub trait II2cDeviceStatics_Impl: Sized {
    fn GetDeviceSelector(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &windows_core::HSTRING, settings: Option<&I2cConnectionSettings>) -> windows_core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>>;
}
impl windows_core::RuntimeName for II2cDeviceStatics {
    const NAME: &'static str = "Windows.Devices.I2c.II2cDeviceStatics";
}
impl II2cDeviceStatics_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> II2cDeviceStatics_Vtbl
    where
        Identity: II2cDeviceStatics_Impl,
    {
        unsafe extern "system" fn GetDeviceSelector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: II2cDeviceStatics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cDeviceStatics_Impl::GetDeviceSelector(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, friendlyname: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: II2cDeviceStatics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cDeviceStatics_Impl::GetDeviceSelectorFromFriendlyName(this, core::mem::transmute(&friendlyname)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: core::mem::MaybeUninit<windows_core::HSTRING>, settings: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: II2cDeviceStatics_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cDeviceStatics_Impl::FromIdAsync(this, core::mem::transmute(&deviceid), windows_core::from_raw_borrowed(&settings)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, II2cDeviceStatics, OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Identity, OFFSET>,
            GetDeviceSelectorFromFriendlyName: GetDeviceSelectorFromFriendlyName::<Identity, OFFSET>,
            FromIdAsync: FromIdAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<II2cDeviceStatics as windows_core::Interface>::IID
    }
}
