#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(IGpioChangeCounter, IGpioChangeCounter_Vtbl, 0xcb5ec0de_6801_43ff_803d_4576628a8b26);
impl windows_core::RuntimeType for IGpioChangeCounter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioChangeCounter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPolarity: unsafe extern "system" fn(*mut core::ffi::c_void, GpioChangePolarity) -> windows_core::HRESULT,
    pub Polarity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioChangePolarity) -> windows_core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioChangeCount) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioChangeCount) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioChangeCounterFactory, IGpioChangeCounterFactory_Vtbl, 0x147d94b6_0a9e_410c_b4fa_f89f4052084d);
impl windows_core::RuntimeType for IGpioChangeCounterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioChangeCounterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioChangeReader, IGpioChangeReader_Vtbl, 0x0abc885f_e031_48e8_8590_70de78363c6d);
impl windows_core::RuntimeType for IGpioChangeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioChangeReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsOverflowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPolarity: unsafe extern "system" fn(*mut core::ffi::c_void, GpioChangePolarity) -> windows_core::HRESULT,
    pub Polarity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioChangePolarity) -> windows_core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioChangeRecord) -> windows_core::HRESULT,
    pub PeekNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioChangeRecord) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllItems: usize,
    pub WaitForItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioChangeReaderFactory, IGpioChangeReaderFactory_Vtbl, 0xa9598ef3_390e_441a_9d1c_e8de0b2df0df);
impl windows_core::RuntimeType for IGpioChangeReaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioChangeReaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithCapacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioController, IGpioController_Vtbl, 0x284012e3_7461_469c_a8bc_61d69d08a53c);
impl windows_core::RuntimeType for IGpioController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OpenPin: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenPinWithSharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32, GpioSharingMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryOpenPin: unsafe extern "system" fn(*mut core::ffi::c_void, i32, GpioSharingMode, *mut *mut core::ffi::c_void, *mut GpioOpenStatus, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioControllerStatics, IGpioControllerStatics_Vtbl, 0x2ed6f42e_7af7_4116_9533_c43d99a1fb64);
impl windows_core::RuntimeType for IGpioControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioControllerStatics2, IGpioControllerStatics2_Vtbl, 0x912b7d20_6ca4_4106_a373_fffd346b0e5b);
impl windows_core::RuntimeType for IGpioControllerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioControllerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioPin, IGpioPin_Vtbl, 0x11d9b087_afae_4790_9ee9_e0eac942d201);
impl windows_core::RuntimeType for IGpioPin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioPin_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DebounceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDebounceTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub PinNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioSharingMode) -> windows_core::HRESULT,
    pub IsDriveModeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, GpioPinDriveMode, *mut bool) -> windows_core::HRESULT,
    pub GetDriveMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioPinDriveMode) -> windows_core::HRESULT,
    pub SetDriveMode: unsafe extern "system" fn(*mut core::ffi::c_void, GpioPinDriveMode) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, GpioPinValue) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioPinValue) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGpioPinValueChangedEventArgs, IGpioPinValueChangedEventArgs_Vtbl, 0x3137aae1_703d_4059_bd24_b5b25dffb84e);
impl windows_core::RuntimeType for IGpioPinValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGpioPinValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Edge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GpioPinEdge) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GpioChangeCounter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GpioChangeCounter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GpioChangeCounter, super::super::Foundation::IClosable);
impl GpioChangeCounter {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPolarity(&self, value: GpioChangePolarity) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPolarity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Polarity(&self) -> windows_core::Result<GpioChangePolarity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Polarity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStarted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStarted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Read(&self) -> windows_core::Result<GpioChangeCount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Read)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Reset(&self) -> windows_core::Result<GpioChangeCount> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create<P0>(pin: P0) -> windows_core::Result<GpioChangeCounter>
    where
        P0: windows_core::Param<GpioPin>,
    {
        Self::IGpioChangeCounterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), pin.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGpioChangeCounterFactory<R, F: FnOnce(&IGpioChangeCounterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GpioChangeCounter, IGpioChangeCounterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GpioChangeCounter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGpioChangeCounter>();
}
unsafe impl windows_core::Interface for GpioChangeCounter {
    type Vtable = IGpioChangeCounter_Vtbl;
    const IID: windows_core::GUID = <IGpioChangeCounter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GpioChangeCounter {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioChangeCounter";
}
unsafe impl Send for GpioChangeCounter {}
unsafe impl Sync for GpioChangeCounter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GpioChangeReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GpioChangeReader, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GpioChangeReader, super::super::Foundation::IClosable);
impl GpioChangeReader {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Capacity(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEmpty(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEmpty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOverflowed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOverflowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPolarity(&self, value: GpioChangePolarity) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPolarity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Polarity(&self) -> windows_core::Result<GpioChangePolarity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Polarity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStarted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStarted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetNextItem(&self) -> windows_core::Result<GpioChangeRecord> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNextItem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PeekNextItem(&self) -> windows_core::Result<GpioChangeRecord> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeekNextItem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllItems(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<GpioChangeRecord>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllItems)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WaitForItemsAsync(&self, count: i32) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WaitForItemsAsync)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(pin: P0) -> windows_core::Result<GpioChangeReader>
    where
        P0: windows_core::Param<GpioPin>,
    {
        Self::IGpioChangeReaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), pin.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithCapacity<P0>(pin: P0, mincapacity: i32) -> windows_core::Result<GpioChangeReader>
    where
        P0: windows_core::Param<GpioPin>,
    {
        Self::IGpioChangeReaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithCapacity)(windows_core::Interface::as_raw(this), pin.param().abi(), mincapacity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGpioChangeReaderFactory<R, F: FnOnce(&IGpioChangeReaderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GpioChangeReader, IGpioChangeReaderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GpioChangeReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGpioChangeReader>();
}
unsafe impl windows_core::Interface for GpioChangeReader {
    type Vtable = IGpioChangeReader_Vtbl;
    const IID: windows_core::GUID = <IGpioChangeReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GpioChangeReader {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioChangeReader";
}
unsafe impl Send for GpioChangeReader {}
unsafe impl Sync for GpioChangeReader {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GpioController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GpioController, windows_core::IUnknown, windows_core::IInspectable);
impl GpioController {
    pub fn PinCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OpenPin(&self, pinnumber: i32) -> windows_core::Result<GpioPin> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenPin)(windows_core::Interface::as_raw(this), pinnumber, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenPinWithSharingMode(&self, pinnumber: i32, sharingmode: GpioSharingMode) -> windows_core::Result<GpioPin> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenPinWithSharingMode)(windows_core::Interface::as_raw(this), pinnumber, sharingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryOpenPin(&self, pinnumber: i32, sharingmode: GpioSharingMode, pin: &mut Option<GpioPin>, openstatus: &mut GpioOpenStatus) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryOpenPin)(windows_core::Interface::as_raw(this), pinnumber, sharingmode, pin as *mut _ as _, openstatus, &mut result__).map(|| result__)
        }
    }
    pub fn GetDefault() -> windows_core::Result<GpioController> {
        Self::IGpioControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<P0>(provider: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<GpioController>>>
    where
        P0: windows_core::Param<Provider::IGpioProvider>,
    {
        Self::IGpioControllerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetControllersAsync)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<GpioController>> {
        Self::IGpioControllerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGpioControllerStatics<R, F: FnOnce(&IGpioControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GpioController, IGpioControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGpioControllerStatics2<R, F: FnOnce(&IGpioControllerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GpioController, IGpioControllerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GpioController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGpioController>();
}
unsafe impl windows_core::Interface for GpioController {
    type Vtable = IGpioController_Vtbl;
    const IID: windows_core::GUID = <IGpioController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GpioController {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioController";
}
unsafe impl Send for GpioController {}
unsafe impl Sync for GpioController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GpioPin(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GpioPin, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GpioPin, super::super::Foundation::IClosable);
impl GpioPin {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ValueChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GpioPin, GpioPinValueChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveValueChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveValueChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DebounceTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DebounceTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDebounceTimeout(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDebounceTimeout)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinNumber(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PinNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SharingMode(&self) -> windows_core::Result<GpioSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SharingMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDriveModeSupported(&self, drivemode: GpioPinDriveMode) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDriveModeSupported)(windows_core::Interface::as_raw(this), drivemode, &mut result__).map(|| result__)
        }
    }
    pub fn GetDriveMode(&self) -> windows_core::Result<GpioPinDriveMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDriveMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDriveMode(&self, value: GpioPinDriveMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDriveMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Write(&self, value: GpioPinValue) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Write)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Read(&self) -> windows_core::Result<GpioPinValue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Read)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GpioPin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGpioPin>();
}
unsafe impl windows_core::Interface for GpioPin {
    type Vtable = IGpioPin_Vtbl;
    const IID: windows_core::GUID = <IGpioPin as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GpioPin {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioPin";
}
unsafe impl Send for GpioPin {}
unsafe impl Sync for GpioPin {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GpioPinValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GpioPinValueChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GpioPinValueChangedEventArgs {
    pub fn Edge(&self) -> windows_core::Result<GpioPinEdge> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Edge)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GpioPinValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGpioPinValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for GpioPinValueChangedEventArgs {
    type Vtable = IGpioPinValueChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGpioPinValueChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GpioPinValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioPinValueChangedEventArgs";
}
unsafe impl Send for GpioPinValueChangedEventArgs {}
unsafe impl Sync for GpioPinValueChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GpioChangePolarity(pub i32);
impl GpioChangePolarity {
    pub const Falling: Self = Self(0i32);
    pub const Rising: Self = Self(1i32);
    pub const Both: Self = Self(2i32);
}
impl windows_core::TypeKind for GpioChangePolarity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GpioChangePolarity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GpioChangePolarity").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GpioChangePolarity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioChangePolarity;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GpioOpenStatus(pub i32);
impl GpioOpenStatus {
    pub const PinOpened: Self = Self(0i32);
    pub const PinUnavailable: Self = Self(1i32);
    pub const SharingViolation: Self = Self(2i32);
    pub const MuxingConflict: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl windows_core::TypeKind for GpioOpenStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GpioOpenStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GpioOpenStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GpioOpenStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioOpenStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GpioPinDriveMode(pub i32);
impl GpioPinDriveMode {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const InputPullUp: Self = Self(2i32);
    pub const InputPullDown: Self = Self(3i32);
    pub const OutputOpenDrain: Self = Self(4i32);
    pub const OutputOpenDrainPullUp: Self = Self(5i32);
    pub const OutputOpenSource: Self = Self(6i32);
    pub const OutputOpenSourcePullDown: Self = Self(7i32);
}
impl windows_core::TypeKind for GpioPinDriveMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GpioPinDriveMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GpioPinDriveMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GpioPinDriveMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinDriveMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GpioPinEdge(pub i32);
impl GpioPinEdge {
    pub const FallingEdge: Self = Self(0i32);
    pub const RisingEdge: Self = Self(1i32);
}
impl windows_core::TypeKind for GpioPinEdge {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GpioPinEdge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GpioPinEdge").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GpioPinEdge {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinEdge;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GpioPinValue(pub i32);
impl GpioPinValue {
    pub const Low: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl windows_core::TypeKind for GpioPinValue {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GpioPinValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GpioPinValue").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GpioPinValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinValue;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GpioSharingMode(pub i32);
impl GpioSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl windows_core::TypeKind for GpioSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GpioSharingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GpioSharingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GpioSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioSharingMode;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpioChangeCount {
    pub Count: u64,
    pub RelativeTime: super::super::Foundation::TimeSpan,
}
impl windows_core::TypeKind for GpioChangeCount {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GpioChangeCount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Gpio.GpioChangeCount;u8;struct(Windows.Foundation.TimeSpan;i8))");
}
impl Default for GpioChangeCount {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpioChangeRecord {
    pub RelativeTime: super::super::Foundation::TimeSpan,
    pub Edge: GpioPinEdge,
}
impl windows_core::TypeKind for GpioChangeRecord {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GpioChangeRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Gpio.GpioChangeRecord;struct(Windows.Foundation.TimeSpan;i8);enum(Windows.Devices.Gpio.GpioPinEdge;i4))");
}
impl Default for GpioChangeRecord {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
