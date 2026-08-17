windows_core::imp::define_interface!(ICustomSensor, ICustomSensor_Vtbl, 0xa136f9ad_4034_4b4d_99dd_531aac649c09);
impl windows_core::RuntimeType for ICustomSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomSensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveReadingChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICustomSensor2, ICustomSensor2_Vtbl, 0x20db3111_ec58_4d9f_bfbd_e77825088510);
impl windows_core::RuntimeType for ICustomSensor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomSensor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReportLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MaxBatchSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICustomSensorReading, ICustomSensorReading_Vtbl, 0x64004f4d_446a_4366_a87a_5f963268ec53);
impl windows_core::RuntimeType for ICustomSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomSensorReading_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(ICustomSensorReading2, ICustomSensorReading2_Vtbl, 0x223c98ea_bf73_4992_9a48_d3c897594ccb);
impl windows_core::RuntimeType for ICustomSensorReading2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomSensorReading2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PerformanceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICustomSensorReadingChangedEventArgs, ICustomSensorReadingChangedEventArgs_Vtbl, 0x6b202023_cffd_4cc1_8ff0_e21823d76fcc);
impl windows_core::RuntimeType for ICustomSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomSensorReadingChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICustomSensorStatics, ICustomSensorStatics_Vtbl, 0x992052cf_f422_4c7d_836b_e7dc74a7124b);
impl windows_core::RuntimeType for ICustomSensorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomSensorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CustomSensor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CustomSensor, windows_core::IUnknown, windows_core::IInspectable);
impl CustomSensor {
    pub fn GetCurrentReading(&self) -> windows_core::Result<CustomSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReportInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadingChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CustomSensor, CustomSensorReadingChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadingChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveReadingChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveReadingChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetReportLatency(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICustomSensor2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReportLatency)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReportLatency(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICustomSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportLatency)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxBatchSize(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<ICustomSensor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxBatchSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeviceSelector(interfaceid: windows_core::GUID) -> windows_core::Result<windows_core::HSTRING> {
        Self::ICustomSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), interfaceid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(sensorid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<CustomSensor>> {
        Self::ICustomSensorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sensorid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICustomSensorStatics<R, F: FnOnce(&ICustomSensorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CustomSensor, ICustomSensorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CustomSensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICustomSensor>();
}
unsafe impl windows_core::Interface for CustomSensor {
    type Vtable = ICustomSensor_Vtbl;
    const IID: windows_core::GUID = <ICustomSensor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomSensor {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensor";
}
unsafe impl Send for CustomSensor {}
unsafe impl Sync for CustomSensor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CustomSensorReading(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CustomSensorReading, windows_core::IUnknown, windows_core::IInspectable);
impl CustomSensorReading {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PerformanceCount(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &windows_core::Interface::cast::<ICustomSensorReading2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PerformanceCount)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CustomSensorReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICustomSensorReading>();
}
unsafe impl windows_core::Interface for CustomSensorReading {
    type Vtable = ICustomSensorReading_Vtbl;
    const IID: windows_core::GUID = <ICustomSensorReading as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomSensorReading {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensorReading";
}
unsafe impl Send for CustomSensorReading {}
unsafe impl Sync for CustomSensorReading {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CustomSensorReadingChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CustomSensorReadingChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CustomSensorReadingChangedEventArgs {
    pub fn Reading(&self) -> windows_core::Result<CustomSensorReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reading)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CustomSensorReadingChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICustomSensorReadingChangedEventArgs>();
}
unsafe impl windows_core::Interface for CustomSensorReadingChangedEventArgs {
    type Vtable = ICustomSensorReadingChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICustomSensorReadingChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomSensorReadingChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sensors.Custom.CustomSensorReadingChangedEventArgs";
}
unsafe impl Send for CustomSensorReadingChangedEventArgs {}
unsafe impl Sync for CustomSensorReadingChangedEventArgs {}
