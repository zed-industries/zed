windows_core::imp::define_interface!(IBattery, IBattery_Vtbl, 0x972adbdd_6720_4702_a476_b9d38a0070e3);
impl windows_core::RuntimeType for IBattery {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBattery_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemainingChargePercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingDischargeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub RemainingChargePercentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemainingChargePercentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBatteryStatics, IBatteryStatics_Vtbl, 0xfaf5bc70_6369_11e1_b86c_0800200c9a66);
impl windows_core::RuntimeType for IBatteryStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBatteryStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Battery(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Battery, windows_core::IUnknown, windows_core::IInspectable);
impl Battery {
    pub fn RemainingChargePercent(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingChargePercent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RemainingDischargeTime(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingDischargeTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RemainingChargePercentChanged<P0>(&self, changehandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemainingChargePercentChanged)(windows_core::Interface::as_raw(this), changehandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemainingChargePercentChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemainingChargePercentChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<Battery> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    const NAME: &'static str = "Windows.Phone.Devices.Power.Battery";
}
unsafe impl Send for Battery {}
unsafe impl Sync for Battery {}
