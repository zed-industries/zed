windows_core::imp::define_interface!(IProviderSpiConnectionSettings, IProviderSpiConnectionSettings_Vtbl, 0xf6034550_a542_4ec0_9601_a4dd68f8697b);
impl windows_core::RuntimeType for IProviderSpiConnectionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderSpiConnectionSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChipSelectLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetChipSelectLine: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderSpiMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderSpiMode) -> windows_core::HRESULT,
    pub DataBitLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDataBitLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClockFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetClockFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderSpiSharingMode) -> windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, ProviderSpiSharingMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProviderSpiConnectionSettingsFactory, IProviderSpiConnectionSettingsFactory_Vtbl, 0x66456b5a_0c79_43e3_9f3c_e59780ac18fa);
impl windows_core::RuntimeType for IProviderSpiConnectionSettingsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderSpiConnectionSettingsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpiControllerProvider, ISpiControllerProvider_Vtbl, 0xc1686504_02ce_4226_a385_4f11fb04b41b);
impl windows_core::RuntimeType for ISpiControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISpiControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
impl ISpiControllerProvider {
    pub fn GetDeviceProvider<P0>(&self, settings: P0) -> windows_core::Result<ISpiDeviceProvider>
    where
        P0: windows_core::Param<ProviderSpiConnectionSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceProvider)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ISpiControllerProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiControllerProvider";
}
pub trait ISpiControllerProvider_Impl: windows_core::IUnknownImpl {
    fn GetDeviceProvider(&self, settings: windows_core::Ref<ProviderSpiConnectionSettings>) -> windows_core::Result<ISpiDeviceProvider>;
}
impl ISpiControllerProvider_Vtbl {
    pub const fn new<Identity: ISpiControllerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceProvider<Identity: ISpiControllerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settings: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpiControllerProvider_Impl::GetDeviceProvider(this, core::mem::transmute_copy(&settings)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpiControllerProvider, OFFSET>(),
            GetDeviceProvider: GetDeviceProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpiControllerProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpiDeviceProvider, ISpiDeviceProvider_Vtbl, 0x0d1c3443_304b_405c_b4f7_f5ab1074461e);
impl windows_core::RuntimeType for ISpiDeviceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISpiDeviceProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ISpiDeviceProvider, super::super::super::Foundation::IClosable);
impl ISpiDeviceProvider {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ConnectionSettings(&self) -> windows_core::Result<ProviderSpiConnectionSettings> {
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
    pub fn Read(&self, buffer: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Read)(windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_mut_ptr()).ok() }
    }
    pub fn TransferSequential(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).TransferSequential)(windows_core::Interface::as_raw(this), writebuffer.len().try_into().unwrap(), writebuffer.as_ptr(), readbuffer.len().try_into().unwrap(), readbuffer.as_mut_ptr()).ok() }
    }
    pub fn TransferFullDuplex(&self, writebuffer: &[u8], readbuffer: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).TransferFullDuplex)(windows_core::Interface::as_raw(this), writebuffer.len().try_into().unwrap(), writebuffer.as_ptr(), readbuffer.len().try_into().unwrap(), readbuffer.as_mut_ptr()).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for ISpiDeviceProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiDeviceProvider";
}
pub trait ISpiDeviceProvider_Impl: super::super::super::Foundation::IClosable_Impl {
    fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ConnectionSettings(&self) -> windows_core::Result<ProviderSpiConnectionSettings>;
    fn Write(&self, buffer: &[u8]) -> windows_core::Result<()>;
    fn Read(&self, buffer: &mut [u8]) -> windows_core::Result<()>;
    fn TransferSequential(&self, writeBuffer: &[u8], readBuffer: &mut [u8]) -> windows_core::Result<()>;
    fn TransferFullDuplex(&self, writeBuffer: &[u8], readBuffer: &mut [u8]) -> windows_core::Result<()>;
}
impl ISpiDeviceProvider_Vtbl {
    pub const fn new<Identity: ISpiDeviceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceId<Identity: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpiDeviceProvider_Impl::DeviceId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectionSettings<Identity: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpiDeviceProvider_Impl::ConnectionSettings(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Write<Identity: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpiDeviceProvider_Impl::Write(this, core::slice::from_raw_parts(core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpiDeviceProvider_Impl::Read(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&buffer), buffer_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn TransferSequential<Identity: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writebuffer_array_size: u32, writebuffer: *const u8, readbuffer_array_size: u32, readbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpiDeviceProvider_Impl::TransferSequential(this, core::slice::from_raw_parts(core::mem::transmute_copy(&writebuffer), writebuffer_array_size as usize), core::slice::from_raw_parts_mut(core::mem::transmute_copy(&readbuffer), readbuffer_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn TransferFullDuplex<Identity: ISpiDeviceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writebuffer_array_size: u32, writebuffer: *const u8, readbuffer_array_size: u32, readbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpiDeviceProvider_Impl::TransferFullDuplex(this, core::slice::from_raw_parts(core::mem::transmute_copy(&writebuffer), writebuffer_array_size as usize), core::slice::from_raw_parts_mut(core::mem::transmute_copy(&readbuffer), readbuffer_array_size as usize)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpiDeviceProvider, OFFSET>(),
            DeviceId: DeviceId::<Identity, OFFSET>,
            ConnectionSettings: ConnectionSettings::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            TransferSequential: TransferSequential::<Identity, OFFSET>,
            TransferFullDuplex: TransferFullDuplex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpiDeviceProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiDeviceProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub TransferSequential: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8) -> windows_core::HRESULT,
    pub TransferFullDuplex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpiProvider, ISpiProvider_Vtbl, 0x96b461e2_77d4_48ce_aaa0_75715a8362cf);
impl windows_core::RuntimeType for ISpiProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISpiProvider, windows_core::IUnknown, windows_core::IInspectable);
impl ISpiProvider {
    pub fn GetControllersAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<ISpiControllerProvider>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllersAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ISpiProvider {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ISpiProvider";
}
pub trait ISpiProvider_Impl: windows_core::IUnknownImpl {
    fn GetControllersAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<ISpiControllerProvider>>>;
}
impl ISpiProvider_Vtbl {
    pub const fn new<Identity: ISpiProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetControllersAsync<Identity: ISpiProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpiProvider_Impl::GetControllersAsync(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpiProvider, OFFSET>(), GetControllersAsync: GetControllersAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpiProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetControllersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSpiConnectionSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProviderSpiConnectionSettings, windows_core::IUnknown, windows_core::IInspectable);
impl ProviderSpiConnectionSettings {
    pub fn ChipSelectLine(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChipSelectLine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetChipSelectLine(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetChipSelectLine)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Mode(&self) -> windows_core::Result<ProviderSpiMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: ProviderSpiMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DataBitLength(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataBitLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDataBitLength(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDataBitLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClockFrequency(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClockFrequency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetClockFrequency(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetClockFrequency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharingMode(&self) -> windows_core::Result<ProviderSpiSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSharingMode(&self, value: ProviderSpiSharingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSharingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(chipselectline: i32) -> windows_core::Result<ProviderSpiConnectionSettings> {
        Self::IProviderSpiConnectionSettingsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), chipselectline, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IProviderSpiConnectionSettingsFactory<R, F: FnOnce(&IProviderSpiConnectionSettingsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProviderSpiConnectionSettings, IProviderSpiConnectionSettingsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProviderSpiConnectionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProviderSpiConnectionSettings>();
}
unsafe impl windows_core::Interface for ProviderSpiConnectionSettings {
    type Vtable = <IProviderSpiConnectionSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProviderSpiConnectionSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProviderSpiConnectionSettings {
    const NAME: &'static str = "Windows.Devices.Spi.Provider.ProviderSpiConnectionSettings";
}
unsafe impl Send for ProviderSpiConnectionSettings {}
unsafe impl Sync for ProviderSpiConnectionSettings {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderSpiMode(pub i32);
impl ProviderSpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl windows_core::TypeKind for ProviderSpiMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderSpiMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.Provider.ProviderSpiMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProviderSpiSharingMode(pub i32);
impl ProviderSpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl windows_core::TypeKind for ProviderSpiSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ProviderSpiSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.Provider.ProviderSpiSharingMode;i4)");
}
