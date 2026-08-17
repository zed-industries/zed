pub trait II2cControllerProvider_Impl: Sized {
    fn GetDeviceProvider(&self, settings: Option<&ProviderI2cConnectionSettings>) -> windows_core::Result<II2cDeviceProvider>;
}
impl windows_core::RuntimeName for II2cControllerProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cControllerProvider";
}
impl II2cControllerProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> II2cControllerProvider_Vtbl
    where
        Identity: II2cControllerProvider_Impl,
    {
        unsafe extern "system" fn GetDeviceProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, settings: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: II2cControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cControllerProvider_Impl::GetDeviceProvider(this, windows_core::from_raw_borrowed(&settings)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, II2cControllerProvider, OFFSET>(),
            GetDeviceProvider: GetDeviceProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<II2cControllerProvider as windows_core::Interface>::IID
    }
}
pub trait II2cDeviceProvider_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Write(&self, buffer: &[u8]) -> windows_core::Result<()>;
    fn WritePartial(&self, buffer: &[u8]) -> windows_core::Result<ProviderI2cTransferResult>;
    fn Read(&self, buffer: &mut [u8]) -> windows_core::Result<()>;
    fn ReadPartial(&self, buffer: &mut [u8]) -> windows_core::Result<ProviderI2cTransferResult>;
    fn WriteRead(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> windows_core::Result<()>;
    fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> windows_core::Result<ProviderI2cTransferResult>;
}
impl windows_core::RuntimeName for II2cDeviceProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cDeviceProvider";
}
impl II2cDeviceProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> II2cDeviceProvider_Vtbl
    where
        Identity: II2cDeviceProvider_Impl,
    {
        unsafe extern "system" fn DeviceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: II2cDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cDeviceProvider_Impl::DeviceId(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> windows_core::HRESULT
        where
            Identity: II2cDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            II2cDeviceProvider_Impl::Write(this, core::slice::from_raw_parts(core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
        }
        unsafe extern "system" fn WritePartial<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ProviderI2cTransferResult) -> windows_core::HRESULT
        where
            Identity: II2cDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cDeviceProvider_Impl::WritePartial(this, core::slice::from_raw_parts(core::mem::transmute_copy(&buffer), buffer_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Read<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> windows_core::HRESULT
        where
            Identity: II2cDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            II2cDeviceProvider_Impl::Read(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
        }
        unsafe extern "system" fn ReadPartial<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> windows_core::HRESULT
        where
            Identity: II2cDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cDeviceProvider_Impl::ReadPartial(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&buffer), buffer_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8) -> windows_core::HRESULT
        where
            Identity: II2cDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            II2cDeviceProvider_Impl::WriteRead(this, core::slice::from_raw_parts(core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as usize), core::slice::from_raw_parts_mut(core::mem::transmute_copy(&readbuffer), readBuffer_array_size as usize)).into()
        }
        unsafe extern "system" fn WriteReadPartial<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, writeBuffer_array_size: u32, writebuffer: *const u8, readBuffer_array_size: u32, readbuffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> windows_core::HRESULT
        where
            Identity: II2cDeviceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cDeviceProvider_Impl::WriteReadPartial(this, core::slice::from_raw_parts(core::mem::transmute_copy(&writebuffer), writeBuffer_array_size as usize), core::slice::from_raw_parts_mut(core::mem::transmute_copy(&readbuffer), readBuffer_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, II2cDeviceProvider, OFFSET>(),
            DeviceId: DeviceId::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            WritePartial: WritePartial::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            ReadPartial: ReadPartial::<Identity, OFFSET>,
            WriteRead: WriteRead::<Identity, OFFSET>,
            WriteReadPartial: WriteReadPartial::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<II2cDeviceProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait II2cProvider_Impl: Sized {
    fn GetControllersAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<II2cControllerProvider>>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for II2cProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl II2cProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> II2cProvider_Vtbl
    where
        Identity: II2cProvider_Impl,
    {
        unsafe extern "system" fn GetControllersAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: II2cProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match II2cProvider_Impl::GetControllersAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, II2cProvider, OFFSET>(), GetControllersAsync: GetControllersAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<II2cProvider as windows_core::Interface>::IID
    }
}
