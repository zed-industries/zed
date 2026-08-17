#[cfg(feature = "Devices_Spi_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(ISpiBusInfo, ISpiBusInfo_Vtbl, 0x9929444a_54f2_48c6_b952_9c32fc02c669);
impl windows_core::RuntimeType for ISpiBusInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiBusInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChipSelectLineCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MinClockFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxClockFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SupportedDataBitLengths: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpiConnectionSettings, ISpiConnectionSettings_Vtbl, 0x5283a37f_f935_4b9f_a7a7_3a7890afa5ce);
impl windows_core::RuntimeType for ISpiConnectionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiConnectionSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChipSelectLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetChipSelectLine: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpiMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, SpiMode) -> windows_core::HRESULT,
    pub DataBitLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDataBitLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClockFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetClockFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpiSharingMode) -> windows_core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, SpiSharingMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpiConnectionSettingsFactory, ISpiConnectionSettingsFactory_Vtbl, 0xff99081e_10c4_44b7_9fea_a748b5a46f31);
impl windows_core::RuntimeType for ISpiConnectionSettingsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiConnectionSettingsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpiController, ISpiController_Vtbl, 0xa8d3c829_9895_4159_a934_8741f1ee6d27);
impl windows_core::RuntimeType for ISpiController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpiControllerStatics, ISpiControllerStatics_Vtbl, 0x0d5229e2_138b_4e48_b964_4f2f79b9c5a2);
impl windows_core::RuntimeType for ISpiControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Spi_Provider")]
    pub GetControllersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Spi_Provider"))]
    GetControllersAsync: usize,
}
windows_core::imp::define_interface!(ISpiDevice, ISpiDevice_Vtbl, 0x05d5356d_11b6_4d39_84d5_95dfb4c9f2ce);
impl windows_core::RuntimeType for ISpiDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub TransferSequential: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8) -> windows_core::HRESULT,
    pub TransferFullDuplex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpiDeviceStatics, ISpiDeviceStatics_Vtbl, 0xa278e559_5720_4d3f_bd93_56f5ff5a5879);
impl windows_core::RuntimeType for ISpiDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ISpiDeviceStatics, windows_core::IUnknown, windows_core::IInspectable);
impl ISpiDeviceStatics {
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
    pub fn GetBusInfo(&self, busid: &windows_core::HSTRING) -> windows_core::Result<SpiBusInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBusInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(busid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync<P1>(&self, busid: &windows_core::HSTRING, settings: P1) -> windows_core::Result<windows_future::IAsyncOperation<SpiDevice>>
    where
        P1: windows_core::Param<SpiConnectionSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(busid), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ISpiDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Spi.ISpiDeviceStatics";
}
pub trait ISpiDeviceStatics_Impl: windows_core::IUnknownImpl {
    fn GetDeviceSelector(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyName: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>;
    fn GetBusInfo(&self, busId: &windows_core::HSTRING) -> windows_core::Result<SpiBusInfo>;
    fn FromIdAsync(&self, busId: &windows_core::HSTRING, settings: windows_core::Ref<SpiConnectionSettings>) -> windows_core::Result<windows_future::IAsyncOperation<SpiDevice>>;
}
impl ISpiDeviceStatics_Vtbl {
    pub const fn new<Identity: ISpiDeviceStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceSelector<Identity: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpiDeviceStatics_Impl::GetDeviceSelector(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Identity: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, friendlyname: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpiDeviceStatics_Impl::GetDeviceSelectorFromFriendlyName(this, core::mem::transmute(&friendlyname)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBusInfo<Identity: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpiDeviceStatics_Impl::GetBusInfo(this, core::mem::transmute(&busid)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FromIdAsync<Identity: ISpiDeviceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, busid: *mut core::ffi::c_void, settings: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpiDeviceStatics_Impl::FromIdAsync(this, core::mem::transmute(&busid), core::mem::transmute_copy(&settings)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpiDeviceStatics, OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Identity, OFFSET>,
            GetDeviceSelectorFromFriendlyName: GetDeviceSelectorFromFriendlyName::<Identity, OFFSET>,
            GetBusInfo: GetBusInfo::<Identity, OFFSET>,
            FromIdAsync: FromIdAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpiDeviceStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpiDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBusInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpiBusInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpiBusInfo, windows_core::IUnknown, windows_core::IInspectable);
impl SpiBusInfo {
    pub fn ChipSelectLineCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChipSelectLineCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinClockFrequency(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinClockFrequency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxClockFrequency(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxClockFrequency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportedDataBitLengths(&self) -> windows_core::Result<windows_collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedDataBitLengths)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpiBusInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpiBusInfo>();
}
unsafe impl windows_core::Interface for SpiBusInfo {
    type Vtable = <ISpiBusInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpiBusInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpiBusInfo {
    const NAME: &'static str = "Windows.Devices.Spi.SpiBusInfo";
}
unsafe impl Send for SpiBusInfo {}
unsafe impl Sync for SpiBusInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpiConnectionSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpiConnectionSettings, windows_core::IUnknown, windows_core::IInspectable);
impl SpiConnectionSettings {
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
    pub fn Mode(&self) -> windows_core::Result<SpiMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: SpiMode) -> windows_core::Result<()> {
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
    pub fn SharingMode(&self) -> windows_core::Result<SpiSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSharingMode(&self, value: SpiSharingMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSharingMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(chipselectline: i32) -> windows_core::Result<SpiConnectionSettings> {
        Self::ISpiConnectionSettingsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), chipselectline, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISpiConnectionSettingsFactory<R, F: FnOnce(&ISpiConnectionSettingsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpiConnectionSettings, ISpiConnectionSettingsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpiConnectionSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpiConnectionSettings>();
}
unsafe impl windows_core::Interface for SpiConnectionSettings {
    type Vtable = <ISpiConnectionSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpiConnectionSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpiConnectionSettings {
    const NAME: &'static str = "Windows.Devices.Spi.SpiConnectionSettings";
}
unsafe impl Send for SpiConnectionSettings {}
unsafe impl Sync for SpiConnectionSettings {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpiController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpiController, windows_core::IUnknown, windows_core::IInspectable);
impl SpiController {
    pub fn GetDevice<P0>(&self, settings: P0) -> windows_core::Result<SpiDevice>
    where
        P0: windows_core::Param<SpiConnectionSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDevice)(windows_core::Interface::as_raw(this), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<SpiController>> {
        Self::ISpiControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn GetControllersAsync<P0>(provider: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<SpiController>>>
    where
        P0: windows_core::Param<Provider::ISpiProvider>,
    {
        Self::ISpiControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllersAsync)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISpiControllerStatics<R, F: FnOnce(&ISpiControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpiController, ISpiControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpiController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpiController>();
}
unsafe impl windows_core::Interface for SpiController {
    type Vtable = <ISpiController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpiController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpiController {
    const NAME: &'static str = "Windows.Devices.Spi.SpiController";
}
unsafe impl Send for SpiController {}
unsafe impl Sync for SpiController {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpiDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpiDevice, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SpiDevice, super::super::Foundation::IClosable, ISpiDeviceStatics);
impl SpiDevice {
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
    pub fn ConnectionSettings(&self) -> windows_core::Result<SpiConnectionSettings> {
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
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpiDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetDeviceSelectorFromFriendlyName(friendlyname: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::ISpiDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(friendlyname), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn GetBusInfo(busid: &windows_core::HSTRING) -> windows_core::Result<SpiBusInfo> {
        Self::ISpiDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBusInfo)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(busid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync<P1>(busid: &windows_core::HSTRING, settings: P1) -> windows_core::Result<windows_future::IAsyncOperation<SpiDevice>>
    where
        P1: windows_core::Param<SpiConnectionSettings>,
    {
        Self::ISpiDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(busid), settings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISpiDeviceStatics<R, F: FnOnce(&ISpiDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpiDevice, ISpiDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpiDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpiDevice>();
}
unsafe impl windows_core::Interface for SpiDevice {
    type Vtable = <ISpiDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISpiDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpiDevice {
    const NAME: &'static str = "Windows.Devices.Spi.SpiDevice";
}
unsafe impl Send for SpiDevice {}
unsafe impl Sync for SpiDevice {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpiMode(pub i32);
impl SpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl windows_core::TypeKind for SpiMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpiMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.SpiMode;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpiSharingMode(pub i32);
impl SpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl windows_core::TypeKind for SpiSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SpiSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Spi.SpiSharingMode;i4)");
}
