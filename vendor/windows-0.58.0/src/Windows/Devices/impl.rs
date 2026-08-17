#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
pub trait ILowLevelDevicesAggregateProvider_Impl: Sized {
    fn AdcControllerProvider(&self) -> windows_core::Result<Adc::Provider::IAdcControllerProvider>;
    fn PwmControllerProvider(&self) -> windows_core::Result<Pwm::Provider::IPwmControllerProvider>;
    fn GpioControllerProvider(&self) -> windows_core::Result<Gpio::Provider::IGpioControllerProvider>;
    fn I2cControllerProvider(&self) -> windows_core::Result<I2c::Provider::II2cControllerProvider>;
    fn SpiControllerProvider(&self) -> windows_core::Result<Spi::Provider::ISpiControllerProvider>;
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl windows_core::RuntimeName for ILowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesAggregateProvider";
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl ILowLevelDevicesAggregateProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILowLevelDevicesAggregateProvider_Vtbl
    where
        Identity: ILowLevelDevicesAggregateProvider_Impl,
    {
        unsafe extern "system" fn AdcControllerProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILowLevelDevicesAggregateProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILowLevelDevicesAggregateProvider_Impl::AdcControllerProvider(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PwmControllerProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILowLevelDevicesAggregateProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILowLevelDevicesAggregateProvider_Impl::PwmControllerProvider(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GpioControllerProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILowLevelDevicesAggregateProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILowLevelDevicesAggregateProvider_Impl::GpioControllerProvider(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn I2cControllerProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILowLevelDevicesAggregateProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILowLevelDevicesAggregateProvider_Impl::I2cControllerProvider(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpiControllerProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILowLevelDevicesAggregateProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILowLevelDevicesAggregateProvider_Impl::SpiControllerProvider(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILowLevelDevicesAggregateProvider, OFFSET>(),
            AdcControllerProvider: AdcControllerProvider::<Identity, OFFSET>,
            PwmControllerProvider: PwmControllerProvider::<Identity, OFFSET>,
            GpioControllerProvider: GpioControllerProvider::<Identity, OFFSET>,
            I2cControllerProvider: I2cControllerProvider::<Identity, OFFSET>,
            SpiControllerProvider: SpiControllerProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILowLevelDevicesAggregateProvider as windows_core::Interface>::IID
    }
}
