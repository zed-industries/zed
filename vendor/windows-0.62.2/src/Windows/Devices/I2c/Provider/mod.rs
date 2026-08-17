windows_core::imp::define_interface!(II2cControllerProvider, II2cControllerProvider_Vtbl, 0x61c2bb82_4510_4163_a87c_4e15a9558980);
impl windows_core::RuntimeType for II2cControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(II2cControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
impl II2cControllerProvider {
    pub fn GetDeviceProvider<P0>(&self, settings: P0) -> windows_core::Result<II2cDeviceProvider>
    where
        P0: windows_core::Param<ProviderI2cConnectionSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceProvider)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for II2cControllerProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cControllerProvider";
}
pub trait II2cControllerProvider_Impl: windows_core::IUnknownImpl {
    fn GetDeviceProvider(&self, settings: windows_core::Ref<ProviderI2cConnectionSettings>) -> windows_core::Result<II2cDeviceProvider>;
}
impl II2cControllerProvider_Vtbl {
    pub const fn new<Identity: II2cControllerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceProvider<Identity: II2cControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settings: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match II2cControllerProvider_Impl::GetDeviceProvider(this, core::mem::transmute_copy(&settings)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, II2cControllerProvider, OFFSET>(),
            GetDeviceProvider: GetDeviceProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<II2cControllerProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(II2cDeviceProvider, II2cDeviceProvider_Vtbl, 0xad342654_57e8_453e_8329_d1e447d103a9);
impl windows_core::RuntimeType for II2cDeviceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(II2cDeviceProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(II2cDeviceProvider, super::super::super::Foundation::IClosable);
impl II2cDeviceProvider {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Write(&self, buffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Write)(windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_ptr()).ok() }
    }
    pub fn WritePartial(&self, buffer: &[u8]) -> windows_core::Result<ProviderI2cTransferResult> {
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
    pub fn ReadPartial(&self, buffer: &mut [u8]) -> windows_core::Result<ProviderI2cTransferResult> {
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
    pub fn WriteReadPartial(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> windows_core::Result<ProviderI2cTransferResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteReadPartial)(windows_core::Interface::as_raw(this), writebuffer.len().try_into().unwrap(), writebuffer.as_ptr(), readbuffer.len().try_into().unwrap(), readbuffer.as_mut_ptr(), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for II2cDeviceProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cDeviceProvider";
}
pub trait II2cDeviceProvider_Impl: super::super::super::Foundation::IClosable_Impl {
    fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Write(&self, buffer: &[u8]) -> windows_core::Result<()>;
    fn WritePartial(&self, buffer: &[u8]) -> windows_core::Result<ProviderI2cTransferResult>;
    fn Read(&self, buffer: &mut [u8]) -> windows_core::Result<()>;
    fn ReadPartial(&self, buffer: &mut [u8]) -> windows_core::Result<ProviderI2cTransferResult>;
    fn WriteRead(&self, writeBuffer: &[u8], readBuffer: &mut [u8]) -> windows_core::Result<()>;
    fn WriteReadPartial(&self, writeBuffer: &[u8], readBuffer: &mut [u8]) -> windows_core::Result<ProviderI2cTransferResult>;
}
impl II2cDeviceProvider_Vtbl {
    pub const fn new<Identity: II2cDeviceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceId<Identity: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn Write<Identity: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                II2cDeviceProvider_Impl::Write(this, core::slice::from_raw_parts(core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn WritePartial<Identity: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ProviderI2cTransferResult) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match II2cDeviceProvider_Impl::WritePartial(this, core::slice::from_raw_parts(core::mem::transmute_copy(&buffer), buffer_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Read<Identity: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                II2cDeviceProvider_Impl::Read(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn ReadPartial<Identity: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match II2cDeviceProvider_Impl::ReadPartial(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&buffer), buffer_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteRead<Identity: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writebuffer_array_size: u32, writebuffer: *const u8, readbuffer_array_size: u32, readbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                II2cDeviceProvider_Impl::WriteRead(this, core::slice::from_raw_parts(core::mem::transmute_copy(&writebuffer), writebuffer_array_size as usize), core::slice::from_raw_parts_mut(core::mem::transmute_copy(&readbuffer), readbuffer_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn WriteReadPartial<Identity: II2cDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writebuffer_array_size: u32, writebuffer: *const u8, readbuffer_array_size: u32, readbuffer: *mut u8, result__: *mut ProviderI2cTransferResult) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match II2cDeviceProvider_Impl::WriteReadPartial(this, core::slice::from_raw_parts(core::mem::transmute_copy(&writebuffer), writebuffer_array_size as usize), core::slice::from_raw_parts_mut(core::mem::transmute_copy(&readbuffer), readbuffer_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[repr(C)]
#[doc(hidden)]
pub struct II2cDeviceProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub WritePartial: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut ProviderI2cTransferResult) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub ReadPartial: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut ProviderI2cTransferResult) -> windows_core::HRESULT,
    pub WriteRead: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8) -> windows_core::HRESULT,
    pub WriteReadPartial: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, *mut ProviderI2cTransferResult) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(II2cProvider, II2cProvider_Vtbl, 0x6f13083e_bf62_4fe2_a95a_f08999669818);
impl windows_core::RuntimeType for II2cProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(II2cProvider, windows_core::IUnknown, windows_core::IInspectable);
impl II2cProvider {
    pub fn GetControllersAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<II2cControllerProvider>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllersAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for II2cProvider {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.II2cProvider";
}
pub trait II2cProvider_Impl: windows_core::IUnknownImpl {
    fn GetControllersAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<II2cControllerProvider>>>;
}
impl II2cProvider_Vtbl {
    pub const fn new<Identity: II2cProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetControllersAsync<Identity: II2cProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, II2cProvider, OFFSET>(), GetControllersAsync: GetControllersAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<II2cProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct II2cProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetControllersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProviderI2cConnectionSettings, IProviderI2cConnectionSettings_Vtbl, 0xe9db4e34_e510_44b7_809d_f2f85b555339);
impl windows_core::RuntimeType for IProviderI2cConnectionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderI2cConnectionSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SlaveAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSlaveAddress: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BusSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderI2cBusSpeed) -> windows_core::HRESULT,
    pub SetBusSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderI2cBusSpeed) -> windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderI2cSharingMode) -> windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderI2cSharingMode) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderI2cBusSpeed(pub i32);
impl ProviderI2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl windows_core::TypeKind for ProviderI2cBusSpeed {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderI2cBusSpeed {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cBusSpeed;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderI2cConnectionSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProviderI2cConnectionSettings, windows_core::IUnknown, windows_core::IInspectable);
impl ProviderI2cConnectionSettings {
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
    pub fn BusSpeed(&self) -> windows_core::Result<ProviderI2cBusSpeed> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BusSpeed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBusSpeed(&self, value: ProviderI2cBusSpeed) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBusSpeed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> windows_core::Result<ProviderI2cSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSharingMode(&self, value: ProviderI2cSharingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSharingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ProviderI2cConnectionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProviderI2cConnectionSettings>();
}
unsafe impl windows_core::Interface for ProviderI2cConnectionSettings {
    type Vtable = <IProviderI2cConnectionSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProviderI2cConnectionSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProviderI2cConnectionSettings {
    const NAME: &'static str = "Windows.Devices.I2c.Provider.ProviderI2cConnectionSettings";
}
unsafe impl Send for ProviderI2cConnectionSettings {}
unsafe impl Sync for ProviderI2cConnectionSettings {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderI2cSharingMode(pub i32);
impl ProviderI2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl windows_core::TypeKind for ProviderI2cSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderI2cSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cSharingMode;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ProviderI2cTransferResult {
    pub Status: ProviderI2cTransferStatus,
    pub BytesTransferred: u32,
}
impl windows_core::TypeKind for ProviderI2cTransferResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderI2cTransferResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.I2c.Provider.ProviderI2cTransferResult;enum(Windows.Devices.I2c.Provider.ProviderI2cTransferStatus;i4);u4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderI2cTransferStatus(pub i32);
impl ProviderI2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
}
impl windows_core::TypeKind for ProviderI2cTransferStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderI2cTransferStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.I2c.Provider.ProviderI2cTransferStatus;i4)");
}
