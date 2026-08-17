#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct I2cBusSpeed(pub i32);
impl I2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl windows_core::TypeKind for I2cBusSpeed {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for I2cBusSpeed {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cBusSpeed;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct I2cConnectionSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(I2cConnectionSettings, windows_core::IUnknown, windows_core::IInspectable);
impl I2cConnectionSettings {
    pub fn SlaveAddress(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SlaveAddress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSlaveAddress(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSlaveAddress)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BusSpeed(&self) -> windows_core::Result<I2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BusSpeed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBusSpeed(&self, value: I2cBusSpeed) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBusSpeed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> windows_core::Result<I2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSharingMode(&self, value: I2cSharingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSharingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(slaveaddress: i32) -> windows_core::Result<I2cConnectionSettings> {
        Self::II2cConnectionSettingsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), slaveaddress, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn II2cConnectionSettingsFactory<R, F: FnOnce(&II2cConnectionSettingsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<I2cConnectionSettings, II2cConnectionSettingsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for I2cConnectionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, II2cConnectionSettings>();
}
unsafe impl windows_core::Interface for I2cConnectionSettings {
    type Vtable = <II2cConnectionSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <II2cConnectionSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for I2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.I2cConnectionSettings";
}
unsafe impl Send for I2cConnectionSettings {}
unsafe impl Sync for I2cConnectionSettings {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct I2cController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(I2cController, windows_core::IUnknown, windows_core::IInspectable);
impl I2cController {
    pub fn GetDevice<P0>(&self, settings: P0) -> windows_core::Result<I2cDevice>
    where
        P0: windows_core::Param<I2cConnectionSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDevice)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn GetControllersAsync<P0>(provider: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<I2cController>>>
    where
        P0: windows_core::Param<Provider::II2cProvider>,
    {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllersAsync)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<I2cController>> {
        Self::II2cControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn II2cControllerStatics<R, F: FnOnce(&II2cControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<I2cController, II2cControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for I2cController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, II2cController>();
}
unsafe impl windows_core::Interface for I2cController {
    type Vtable = <II2cController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <II2cController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for I2cController {
    const NAME: &'static str = "Windows.Devices.I2c.I2cController";
}
unsafe impl Send for I2cController {}
unsafe impl Sync for I2cController {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct I2cDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(I2cDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(I2cDevice, super::super::Foundation::IClosable, II2cDeviceStatics);
impl I2cDevice {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ConnectionSettings(&self) -> windows_core::Result<I2cConnectionSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Write)(windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_ptr()).ok() }
    }
    pub fn WritePartial(&self, buffer: &[u8]) -> windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WritePartial)(windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_ptr(), &mut result__).map(|| result__)
        }
    }
    pub fn Read(&self, buffer: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Read)(windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_mut_ptr()).ok() }
    }
    pub fn ReadPartial(&self, buffer: &mut [u8]) -> windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadPartial)(windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_mut_ptr(), &mut result__).map(|| result__)
        }
    }
    pub fn WriteRead(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteRead)(windows_core::Interface::as_raw(this), writebuffer.len().try_into().unwrap(), writebuffer.as_ptr(), readbuffer.len().try_into().unwrap(), readbuffer.as_mut_ptr()).ok() }
    }
    pub fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> windows_core::Result<I2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteReadPartial)(windows_core::Interface::as_raw(this), writebuffer.len().try_into().unwrap(), writebuffer.as_ptr(), readbuffer.len().try_into().unwrap(), readbuffer.as_mut_ptr(), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromFriendlyName(friendlyname: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(friendlyname), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync<P1>(deviceid: &windows_core::HSTRING, settings: P1) -> windows_core::Result<windows_future::IAsyncOperation<I2cDevice>>
    where
        P1: windows_core::Param<I2cConnectionSettings>,
    {
        Self::II2cDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn II2cDeviceStatics<R, F: FnOnce(&II2cDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<I2cDevice, II2cDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for I2cDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, II2cDevice>();
}
unsafe impl windows_core::Interface for I2cDevice {
    type Vtable = <II2cDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <II2cDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for I2cDevice {
    const NAME: &'static str = "Windows.Devices.I2c.I2cDevice";
}
unsafe impl Send for I2cDevice {}
unsafe impl Sync for I2cDevice {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct I2cSharingMode(pub i32);
impl I2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl windows_core::TypeKind for I2cSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for I2cSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cSharingMode;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct I2cTransferResult {
    pub Status: I2cTransferStatus,
    pub BytesTransferred: u32,
}
impl windows_core::TypeKind for I2cTransferResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for I2cTransferResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.I2c.I2cTransferResult;enum(Windows.Devices.I2c.I2cTransferStatus;i4);u4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
    pub const ClockStretchTimeout: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl windows_core::TypeKind for I2cTransferStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for I2cTransferStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.I2cTransferStatus;i4)");
}
windows_core::imp::define_interface!(II2cConnectionSettings, II2cConnectionSettings_Vtbl, 0xf2db1307_ab6f_4639_a767_54536dc3460f);
impl windows_core::RuntimeType for II2cConnectionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SlaveAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut I2cBusSpeed) -> windows_core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, I2cBusSpeed) -> windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut I2cSharingMode) -> windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, I2cSharingMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(II2cConnectionSettingsFactory, II2cConnectionSettingsFactory_Vtbl, 0x81b586b3_9693_41b1_a243_ded4f6e66926);
impl windows_core::RuntimeType for II2cConnectionSettingsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cConnectionSettingsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(II2cController, II2cController_Vtbl, 0xc48ab1b2_87a0_4166_8e3e_b4b8f97cd729);
impl windows_core::RuntimeType for II2cController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(II2cControllerStatics, II2cControllerStatics_Vtbl, 0x40fc0365_5f05_4e7e_84bd_100db8e0aec5);
impl windows_core::RuntimeType for II2cControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_I2c_Provider")]
    pub GetControllersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_I2c_Provider"))]
    GetControllersAsync: usize,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(II2cDevice, II2cDevice_Vtbl, 0x8636c136_b9c5_4f70_9449_cc46dc6f57eb);
impl windows_core::RuntimeType for II2cDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut I2cTransferResult) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut I2cTransferResult) -> windows_core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8) -> windows_core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, *mut I2cTransferResult) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(II2cDeviceStatics, II2cDeviceStatics_Vtbl, 0x91a33be3_7334_4512_96bc_fbae9459f5f6);
impl windows_core::RuntimeType for II2cDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(II2cDeviceStatics, windows_core::IUnknown, windows_core::IInspectable);
impl II2cDeviceStatics {
    pub fn GetDeviceSelector(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(friendlyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FromIdAsync<P1>(&self, deviceid: &windows_core::HSTRING, settings: P1) -> windows_core::Result<windows_future::IAsyncOperation<I2cDevice>>
    where
        P1: windows_core::Param<I2cConnectionSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for II2cDeviceStatics {
    const NAME: &'static str = "Windows.Devices.I2c.II2cDeviceStatics";
}
pub trait II2cDeviceStatics_Impl: windows_core::IUnknownImpl {
    fn GetDeviceSelector(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyName: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>;
    fn FromIdAsync(&self, deviceId: &windows_core::HSTRING, settings: windows_core::Ref<I2cConnectionSettings>) -> windows_core::Result<windows_future::IAsyncOperation<I2cDevice>>;
}
impl II2cDeviceStatics_Vtbl {
    pub const fn new<Identity: II2cDeviceStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceSelector<Identity: II2cDeviceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Identity: II2cDeviceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, friendlyname: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn FromIdAsync<Identity: II2cDeviceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut core::ffi::c_void, settings: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match II2cDeviceStatics_Impl::FromIdAsync(this, core::mem::transmute(&deviceid), core::mem::transmute_copy(&settings)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
