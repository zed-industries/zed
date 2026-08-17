windows_core::imp::define_interface!(IBattery, IBattery_Vtbl, 0xbc894fc6_0072_47c8_8b5d_614aaa7a437e);
impl windows_core::RuntimeType for IBattery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBattery_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReportUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBatteryReport, IBatteryReport_Vtbl, 0xc9858c3a_4e13_420a_a8d0_24f18f395401);
impl windows_core::RuntimeType for IBatteryReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBatteryReport_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChargeRateInMilliwatts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DesignCapacityInMilliwattHours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FullChargeCapacityInMilliwattHours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemainingCapacityInMilliwattHours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System_Power")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Power::BatteryStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "System_Power"))]
    Status: usize,
}
windows_core::imp::define_interface!(IBatteryStatics, IBatteryStatics_Vtbl, 0x79cd72b6_9e5e_4452_bea6_dfcd541e597f);
impl windows_core::RuntimeType for IBatteryStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBatteryStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AggregateBattery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerGridData, IPowerGridData_Vtbl, 0xc360fb17_fc92_5f6e_999d_16a4cf9d6c40);
impl windows_core::RuntimeType for IPowerGridData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPowerGridData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Severity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsLowUserExperienceImpact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerGridForecast, IPowerGridForecast_Vtbl, 0x077e4de9_ed60_58bb_a850_003c6a138685);
impl windows_core::RuntimeType for IPowerGridForecast {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPowerGridForecast_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub BlockDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Forecast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Forecast: usize,
}
windows_core::imp::define_interface!(IPowerGridForecastStatics, IPowerGridForecastStatics_Vtbl, 0x5b78c806_2e4e_5bcc_bb34_cb81c60f9e12);
impl windows_core::RuntimeType for IPowerGridForecastStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPowerGridForecastStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForecast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForecastUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveForecastUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Battery(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Battery, windows_core::IUnknown, windows_core::IInspectable);
impl Battery {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetReport(&self) -> windows_core::Result<BatteryReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportUpdated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Battery, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReportUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReportUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AggregateBattery() -> windows_core::Result<Battery> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AggregateBattery)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Battery>> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBatteryStatics<R, F: FnOnce(&IBatteryStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Battery, IBatteryStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Battery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBattery>();
}
unsafe impl windows_core::Interface for Battery {
    type Vtable = IBattery_Vtbl;
    const IID: windows_core::GUID = <IBattery as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Battery {
    const NAME: &'static str = "Windows.Devices.Power.Battery";
}
unsafe impl Send for Battery {}
unsafe impl Sync for Battery {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BatteryReport(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BatteryReport, windows_core::IUnknown, windows_core::IInspectable);
impl BatteryReport {
    pub fn ChargeRateInMilliwatts(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChargeRateInMilliwatts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DesignCapacityInMilliwattHours(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesignCapacityInMilliwattHours)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FullChargeCapacityInMilliwattHours(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullChargeCapacityInMilliwattHours)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemainingCapacityInMilliwattHours(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingCapacityInMilliwattHours)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System_Power")]
    pub fn Status(&self) -> windows_core::Result<super::super::System::Power::BatteryStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BatteryReport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBatteryReport>();
}
unsafe impl windows_core::Interface for BatteryReport {
    type Vtable = IBatteryReport_Vtbl;
    const IID: windows_core::GUID = <IBatteryReport as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BatteryReport {
    const NAME: &'static str = "Windows.Devices.Power.BatteryReport";
}
unsafe impl Send for BatteryReport {}
unsafe impl Sync for BatteryReport {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PowerGridData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PowerGridData, windows_core::IUnknown, windows_core::IInspectable);
impl PowerGridData {
    pub fn Severity(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Severity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLowUserExperienceImpact(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLowUserExperienceImpact)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PowerGridData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPowerGridData>();
}
unsafe impl windows_core::Interface for PowerGridData {
    type Vtable = IPowerGridData_Vtbl;
    const IID: windows_core::GUID = <IPowerGridData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PowerGridData {
    const NAME: &'static str = "Windows.Devices.Power.PowerGridData";
}
unsafe impl Send for PowerGridData {}
unsafe impl Sync for PowerGridData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PowerGridForecast(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PowerGridForecast, windows_core::IUnknown, windows_core::IInspectable);
impl PowerGridForecast {
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BlockDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BlockDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Forecast(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PowerGridData>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Forecast)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForecast() -> windows_core::Result<PowerGridForecast> {
        Self::IPowerGridForecastStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForecast)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ForecastUpdated<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IPowerGridForecastStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForecastUpdated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveForecastUpdated(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPowerGridForecastStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveForecastUpdated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IPowerGridForecastStatics<R, F: FnOnce(&IPowerGridForecastStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PowerGridForecast, IPowerGridForecastStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PowerGridForecast {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPowerGridForecast>();
}
unsafe impl windows_core::Interface for PowerGridForecast {
    type Vtable = IPowerGridForecast_Vtbl;
    const IID: windows_core::GUID = <IPowerGridForecast as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PowerGridForecast {
    const NAME: &'static str = "Windows.Devices.Power.PowerGridForecast";
}
unsafe impl Send for PowerGridForecast {}
unsafe impl Sync for PowerGridForecast {}
