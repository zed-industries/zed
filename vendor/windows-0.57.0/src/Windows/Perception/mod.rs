#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
windows_core::imp::define_interface!(IPerceptionTimestamp, IPerceptionTimestamp_Vtbl, 0x87c24804_a22e_4adb_ba26_d78ef639bcf4);
impl windows_core::RuntimeType for IPerceptionTimestamp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPerceptionTimestamp_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TargetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PredictionAmount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPerceptionTimestamp2, IPerceptionTimestamp2_Vtbl, 0xe354b7ed_2bd1_41b7_9ed0_74a15c354537);
impl windows_core::RuntimeType for IPerceptionTimestamp2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPerceptionTimestamp2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SystemRelativeTargetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPerceptionTimestampHelperStatics, IPerceptionTimestampHelperStatics_Vtbl, 0x47a611d4_a9df_4edc_855d_f4d339d967ac);
impl windows_core::RuntimeType for IPerceptionTimestampHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPerceptionTimestampHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromHistoricalTargetTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPerceptionTimestampHelperStatics2, IPerceptionTimestampHelperStatics2_Vtbl, 0x73d1a7fe_3fb9_4571_87d4_3c920a5e86eb);
impl windows_core::RuntimeType for IPerceptionTimestampHelperStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPerceptionTimestampHelperStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromSystemRelativeTargetTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PerceptionTimestamp(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PerceptionTimestamp, windows_core::IUnknown, windows_core::IInspectable);
impl PerceptionTimestamp {
    pub fn TargetTime(&self) -> windows_core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PredictionAmount(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PredictionAmount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SystemRelativeTargetTime(&self) -> windows_core::Result<super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IPerceptionTimestamp2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeTargetTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PerceptionTimestamp {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPerceptionTimestamp>();
}
unsafe impl windows_core::Interface for PerceptionTimestamp {
    type Vtable = IPerceptionTimestamp_Vtbl;
    const IID: windows_core::GUID = <IPerceptionTimestamp as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PerceptionTimestamp {
    const NAME: &'static str = "Windows.Perception.PerceptionTimestamp";
}
unsafe impl Send for PerceptionTimestamp {}
unsafe impl Sync for PerceptionTimestamp {}
pub struct PerceptionTimestampHelper;
impl PerceptionTimestampHelper {
    pub fn FromHistoricalTargetTime(targettime: super::Foundation::DateTime) -> windows_core::Result<PerceptionTimestamp> {
        Self::IPerceptionTimestampHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromHistoricalTargetTime)(windows_core::Interface::as_raw(this), targettime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromSystemRelativeTargetTime(targettime: super::Foundation::TimeSpan) -> windows_core::Result<PerceptionTimestamp> {
        Self::IPerceptionTimestampHelperStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromSystemRelativeTargetTime)(windows_core::Interface::as_raw(this), targettime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPerceptionTimestampHelperStatics<R, F: FnOnce(&IPerceptionTimestampHelperStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PerceptionTimestampHelper, IPerceptionTimestampHelperStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPerceptionTimestampHelperStatics2<R, F: FnOnce(&IPerceptionTimestampHelperStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PerceptionTimestampHelper, IPerceptionTimestampHelperStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PerceptionTimestampHelper {
    const NAME: &'static str = "Windows.Perception.PerceptionTimestampHelper";
}
