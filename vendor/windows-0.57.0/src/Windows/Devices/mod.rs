#[cfg(feature = "Devices_Adc")]
pub mod Adc;
#[cfg(feature = "Devices_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth")]
pub mod Bluetooth;
#[cfg(feature = "Devices_Custom")]
pub mod Custom;
#[cfg(feature = "Devices_Display")]
pub mod Display;
#[cfg(feature = "Devices_Enumeration")]
pub mod Enumeration;
#[cfg(feature = "Devices_Geolocation")]
pub mod Geolocation;
#[cfg(feature = "Devices_Gpio")]
pub mod Gpio;
#[cfg(feature = "Devices_Haptics")]
pub mod Haptics;
#[cfg(feature = "Devices_HumanInterfaceDevice")]
pub mod HumanInterfaceDevice;
#[cfg(feature = "Devices_I2c")]
pub mod I2c;
#[cfg(feature = "Devices_Input")]
pub mod Input;
#[cfg(feature = "Devices_Lights")]
pub mod Lights;
#[cfg(feature = "Devices_Midi")]
pub mod Midi;
#[cfg(feature = "Devices_PointOfService")]
pub mod PointOfService;
#[cfg(feature = "Devices_Portable")]
pub mod Portable;
#[cfg(feature = "Devices_Power")]
pub mod Power;
#[cfg(feature = "Devices_Printers")]
pub mod Printers;
#[cfg(feature = "Devices_Pwm")]
pub mod Pwm;
#[cfg(feature = "Devices_Radios")]
pub mod Radios;
#[cfg(feature = "Devices_Scanners")]
pub mod Scanners;
#[cfg(feature = "Devices_Sensors")]
pub mod Sensors;
#[cfg(feature = "Devices_SerialCommunication")]
pub mod SerialCommunication;
#[cfg(feature = "Devices_SmartCards")]
pub mod SmartCards;
#[cfg(feature = "Devices_Sms")]
pub mod Sms;
#[cfg(feature = "Devices_Spi")]
pub mod Spi;
#[cfg(feature = "Devices_Usb")]
pub mod Usb;
#[cfg(feature = "Devices_WiFi")]
pub mod WiFi;
#[cfg(feature = "Devices_WiFiDirect")]
pub mod WiFiDirect;
windows_core::imp::define_interface!(ILowLevelDevicesAggregateProvider, ILowLevelDevicesAggregateProvider_Vtbl, 0xa73e561c_aac1_4ec7_a852_479f7060d01f);
impl core::ops::Deref for ILowLevelDevicesAggregateProvider {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILowLevelDevicesAggregateProvider, windows_core::IUnknown, windows_core::IInspectable);
impl ILowLevelDevicesAggregateProvider {
    #[cfg(feature = "Devices_Adc_Provider")]
    pub fn AdcControllerProvider(&self) -> windows_core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdcControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub fn PwmControllerProvider(&self) -> windows_core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PwmControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub fn GpioControllerProvider(&self) -> windows_core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GpioControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn I2cControllerProvider(&self) -> windows_core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).I2cControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn SpiControllerProvider(&self) -> windows_core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpiControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ILowLevelDevicesAggregateProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLevelDevicesAggregateProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Adc_Provider")]
    pub AdcControllerProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Adc_Provider"))]
    AdcControllerProvider: usize,
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub PwmControllerProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Pwm_Provider"))]
    PwmControllerProvider: usize,
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub GpioControllerProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Gpio_Provider"))]
    GpioControllerProvider: usize,
    #[cfg(feature = "Devices_I2c_Provider")]
    pub I2cControllerProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_I2c_Provider"))]
    I2cControllerProvider: usize,
    #[cfg(feature = "Devices_Spi_Provider")]
    pub SpiControllerProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Spi_Provider"))]
    SpiControllerProvider: usize,
}
windows_core::imp::define_interface!(ILowLevelDevicesAggregateProviderFactory, ILowLevelDevicesAggregateProviderFactory_Vtbl, 0x9ac4aaf6_3473_465e_96d5_36281a2c57af);
impl windows_core::RuntimeType for ILowLevelDevicesAggregateProviderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLevelDevicesAggregateProviderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider")))]
    Create: usize,
}
windows_core::imp::define_interface!(ILowLevelDevicesController, ILowLevelDevicesController_Vtbl, 0x2ec23dd4_179b_45de_9b39_3ae02527de52);
impl windows_core::RuntimeType for ILowLevelDevicesController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLevelDevicesController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ILowLevelDevicesControllerStatics, ILowLevelDevicesControllerStatics_Vtbl, 0x093e926a_fccb_4394_a697_19de637c2db3);
impl windows_core::RuntimeType for ILowLevelDevicesControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILowLevelDevicesControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DefaultProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LowLevelDevicesAggregateProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LowLevelDevicesAggregateProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LowLevelDevicesAggregateProvider, ILowLevelDevicesAggregateProvider);
impl LowLevelDevicesAggregateProvider {
    #[cfg(feature = "Devices_Adc_Provider")]
    pub fn AdcControllerProvider(&self) -> windows_core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdcControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub fn PwmControllerProvider(&self) -> windows_core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PwmControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub fn GpioControllerProvider(&self) -> windows_core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GpioControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn I2cControllerProvider(&self) -> windows_core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).I2cControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn SpiControllerProvider(&self) -> windows_core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpiControllerProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    pub fn Create<P0, P1, P2, P3, P4>(adc: P0, pwm: P1, gpio: P2, i2c: P3, spi: P4) -> windows_core::Result<LowLevelDevicesAggregateProvider>
    where
        P0: windows_core::Param<Adc::Provider::IAdcControllerProvider>,
        P1: windows_core::Param<Pwm::Provider::IPwmControllerProvider>,
        P2: windows_core::Param<Gpio::Provider::IGpioControllerProvider>,
        P3: windows_core::Param<I2c::Provider::II2cControllerProvider>,
        P4: windows_core::Param<Spi::Provider::ISpiControllerProvider>,
    {
        Self::ILowLevelDevicesAggregateProviderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), adc.param().abi(), pwm.param().abi(), gpio.param().abi(), i2c.param().abi(), spi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILowLevelDevicesAggregateProviderFactory<R, F: FnOnce(&ILowLevelDevicesAggregateProviderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LowLevelDevicesAggregateProvider, ILowLevelDevicesAggregateProviderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LowLevelDevicesAggregateProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILowLevelDevicesAggregateProvider>();
}
unsafe impl windows_core::Interface for LowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_Vtbl;
    const IID: windows_core::GUID = <ILowLevelDevicesAggregateProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesAggregateProvider";
}
unsafe impl Send for LowLevelDevicesAggregateProvider {}
unsafe impl Sync for LowLevelDevicesAggregateProvider {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LowLevelDevicesController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LowLevelDevicesController, windows_core::IUnknown, windows_core::IInspectable);
impl LowLevelDevicesController {
    pub fn DefaultProvider() -> windows_core::Result<ILowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultProvider)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetDefaultProvider<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILowLevelDevicesAggregateProvider>,
    {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetDefaultProvider)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ILowLevelDevicesControllerStatics<R, F: FnOnce(&ILowLevelDevicesControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LowLevelDevicesController, ILowLevelDevicesControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LowLevelDevicesController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILowLevelDevicesController>();
}
unsafe impl windows_core::Interface for LowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_Vtbl;
    const IID: windows_core::GUID = <ILowLevelDevicesController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LowLevelDevicesController {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesController";
}
unsafe impl Send for LowLevelDevicesController {}
unsafe impl Sync for LowLevelDevicesController {}
#[cfg(feature = "implement")]
core::include!("impl.rs");
