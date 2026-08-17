windows_core::imp::define_interface!(IActivitySensorTrigger, IActivitySensorTrigger_Vtbl, 0xd0dd4342_e37b_4823_a5fe_6b31dfefdeb0);
impl windows_core::RuntimeType for IActivitySensorTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivitySensorTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub SubscribedActivities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))]
    SubscribedActivities: usize,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub SupportedActivities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))]
    SupportedActivities: usize,
    pub MinimumReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivitySensorTriggerFactory, IActivitySensorTriggerFactory_Vtbl, 0xa72691c3_3837_44f7_831b_0132cc872bc3);
impl windows_core::RuntimeType for IActivitySensorTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IActivitySensorTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAlarmApplicationManagerStatics, IAlarmApplicationManagerStatics_Vtbl, 0xca03fa3b_cce6_4de2_b09b_9628bd33bbbe);
impl windows_core::RuntimeType for IAlarmApplicationManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAlarmApplicationManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAccessStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AlarmAccessStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastTrigger, IAppBroadcastTrigger_Vtbl, 0x74d4f496_8d37_44ec_9481_2a0b9854eb48);
impl windows_core::RuntimeType for IAppBroadcastTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetProviderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProviderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastTriggerFactory, IAppBroadcastTriggerFactory_Vtbl, 0x280b9f44_22f4_4618_a02e_e7e411eb7238);
impl windows_core::RuntimeType for IAppBroadcastTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAppBroadcastTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppBroadcastTriggerProviderInfo, IAppBroadcastTriggerProviderInfo_Vtbl, 0xf219352d_9de8_4420_9ce2_5eff8f17376b);
impl windows_core::RuntimeType for IAppBroadcastTriggerProviderInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppBroadcastTriggerProviderInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetDisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLogoResource: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LogoResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetVideoKeyFrameInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub VideoKeyFrameInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetMaxVideoBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxVideoBitrate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxVideoWidth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxVideoWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxVideoHeight: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxVideoHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationTrigger, IApplicationTrigger_Vtbl, 0x0b468630_9574_492c_9e93_1a3ae6335fe9);
impl windows_core::RuntimeType for IApplicationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAsyncWithArguments: usize,
}
windows_core::imp::define_interface!(IApplicationTriggerDetails, IApplicationTriggerDetails_Vtbl, 0x97dc6ab2_2219_4a9e_9c5e_41d047f76e82);
impl windows_core::RuntimeType for IApplicationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
windows_core::imp::define_interface!(IAppointmentStoreNotificationTrigger, IAppointmentStoreNotificationTrigger_Vtbl, 0x64d4040c_c201_42ad_aa2a_e21ba3425b6d);
impl windows_core::RuntimeType for IAppointmentStoreNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppointmentStoreNotificationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IBackgroundCondition, IBackgroundCondition_Vtbl, 0xae48a1ee_8951_400a_8302_9c9c9a2a3a3b);
impl core::ops::Deref for IBackgroundCondition {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundCondition, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundCondition {}
impl windows_core::RuntimeType for IBackgroundCondition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundCondition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IBackgroundExecutionManagerStatics, IBackgroundExecutionManagerStatics_Vtbl, 0xe826ea58_66a9_4d41_83d4_b4c18c87b846);
impl windows_core::RuntimeType for IBackgroundExecutionManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundExecutionManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAccessForApplicationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAccessForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetAccessStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundAccessStatus) -> windows_core::HRESULT,
    pub GetAccessStatusForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut BackgroundAccessStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundExecutionManagerStatics2, IBackgroundExecutionManagerStatics2_Vtbl, 0x469b24ef_9bbb_4e18_999a_fd6512931be9);
impl windows_core::RuntimeType for IBackgroundExecutionManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundExecutionManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessKindAsync: unsafe extern "system" fn(*mut core::ffi::c_void, BackgroundAccessRequestKind, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundExecutionManagerStatics3, IBackgroundExecutionManagerStatics3_Vtbl, 0x98a5d3f6_5a25_5b6c_9192_d77a43dfedc4);
impl windows_core::RuntimeType for IBackgroundExecutionManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundExecutionManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessKindForModernStandbyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, BackgroundAccessRequestKind, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAccessStatusForModernStandby: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundAccessStatus) -> windows_core::HRESULT,
    pub GetAccessStatusForModernStandbyForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut BackgroundAccessStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTask, IBackgroundTask_Vtbl, 0x7d13d534_fd12_43ce_8c22_ea1ff13c06df);
impl core::ops::Deref for IBackgroundTask {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTask, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundTask {
    pub fn Run<P0>(&self, taskinstance: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundTaskInstance>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Run)(windows_core::Interface::as_raw(this), taskinstance.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IBackgroundTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTask_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskBuilder, IBackgroundTaskBuilder_Vtbl, 0x0351550e_3e64_4572_a93a_84075a37c917);
impl windows_core::RuntimeType for IBackgroundTaskBuilder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskBuilder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTaskEntryPoint: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TaskEntryPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskBuilder2, IBackgroundTaskBuilder2_Vtbl, 0x6ae7cfb1_104f_406d_8db6_844a570f42bb);
impl windows_core::RuntimeType for IBackgroundTaskBuilder2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskBuilder2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCancelOnConditionLoss: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CancelOnConditionLoss: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskBuilder3, IBackgroundTaskBuilder3_Vtbl, 0x28c74f4a_8ba9_4c09_a24f_19683e2c924c);
impl windows_core::RuntimeType for IBackgroundTaskBuilder3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskBuilder3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsNetworkRequested: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsNetworkRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskBuilder4, IBackgroundTaskBuilder4_Vtbl, 0x4755e522_cba2_4e35_bd16_a6da7f1c19aa);
impl windows_core::RuntimeType for IBackgroundTaskBuilder4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskBuilder4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TaskGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskBuilder5, IBackgroundTaskBuilder5_Vtbl, 0x077103f6_99f5_4af4_bcad_4731d0330d43);
impl windows_core::RuntimeType for IBackgroundTaskBuilder5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskBuilder5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTaskEntryPointClsid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskCompletedEventArgs, IBackgroundTaskCompletedEventArgs_Vtbl, 0x565d25cf_f209_48f4_9967_2b184f7bfbf0);
impl windows_core::RuntimeType for IBackgroundTaskCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CheckResult: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskDeferral, IBackgroundTaskDeferral_Vtbl, 0x93cc156d_af27_4dd3_846e_24ee40cadd25);
impl windows_core::RuntimeType for IBackgroundTaskDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskInstance, IBackgroundTaskInstance_Vtbl, 0x865bda7a_21d8_4573_8f32_928a1b0641f6);
impl core::ops::Deref for IBackgroundTaskInstance {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTaskInstance, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundTaskInstance {
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Task(&self) -> windows_core::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Task)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProgress)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Canceled<P0>(&self, cancelhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskCanceledEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Canceled)(windows_core::Interface::as_raw(this), cancelhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCanceled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuspendedCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<BackgroundTaskDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IBackgroundTaskInstance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskInstance_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TriggerDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Canceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SuspendedCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskInstance2, IBackgroundTaskInstance2_Vtbl, 0x4f7d0176_0c76_4fb4_896d_5de1864122f6);
impl core::ops::Deref for IBackgroundTaskInstance2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTaskInstance2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IBackgroundTaskInstance2, IBackgroundTaskInstance);
impl IBackgroundTaskInstance2 {
    pub fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThrottleCount)(windows_core::Interface::as_raw(this), counter, &mut result__).map(|| result__)
        }
    }
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Task(&self) -> windows_core::Result<BackgroundTaskRegistration> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Task)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProgress)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Canceled<P0>(&self, cancelhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskCanceledEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Canceled)(windows_core::Interface::as_raw(this), cancelhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCanceled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuspendedCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<BackgroundTaskDeferral> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IBackgroundTaskInstance2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskInstance2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetThrottleCount: unsafe extern "system" fn(*mut core::ffi::c_void, BackgroundTaskThrottleCounter, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskInstance4, IBackgroundTaskInstance4_Vtbl, 0x7f29f23c_aa04_4b08_97b0_06d874cdabf5);
impl core::ops::Deref for IBackgroundTaskInstance4 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTaskInstance4, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IBackgroundTaskInstance4, IBackgroundTaskInstance);
impl IBackgroundTaskInstance4 {
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Task(&self) -> windows_core::Result<BackgroundTaskRegistration> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Task)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProgress)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Canceled<P0>(&self, cancelhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskCanceledEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Canceled)(windows_core::Interface::as_raw(this), cancelhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCanceled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuspendedCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<BackgroundTaskDeferral> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IBackgroundTaskInstance4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskInstance4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IBackgroundTaskProgressEventArgs, IBackgroundTaskProgressEventArgs_Vtbl, 0xfb1468ac_8332_4d0a_9532_03eae684da31);
impl windows_core::RuntimeType for IBackgroundTaskProgressEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskProgressEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskRegistration, IBackgroundTaskRegistration_Vtbl, 0x10654cc2_a26e_43bf_8c12_1fb40dbfbfa0);
impl core::ops::Deref for IBackgroundTaskRegistration {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTaskRegistration, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundTaskRegistration {
    pub fn TaskId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskProgressEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProgress)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Completed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskCompletedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), canceltask).ok() }
    }
}
impl windows_core::RuntimeType for IBackgroundTaskRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskRegistration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProgress: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskRegistration2, IBackgroundTaskRegistration2_Vtbl, 0x6138c703_bb86_4112_afc3_7f939b166e3b);
impl core::ops::Deref for IBackgroundTaskRegistration2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTaskRegistration2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IBackgroundTaskRegistration2, IBackgroundTaskRegistration);
impl IBackgroundTaskRegistration2 {
    pub fn Trigger(&self) -> windows_core::Result<IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Trigger)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TaskId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskProgressEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveProgress)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Completed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskCompletedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), canceltask).ok() }
    }
}
impl windows_core::RuntimeType for IBackgroundTaskRegistration2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskRegistration2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Trigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskRegistration3, IBackgroundTaskRegistration3_Vtbl, 0xfe338195_9423_4d8b_830d_b1dd2c7badd5);
impl core::ops::Deref for IBackgroundTaskRegistration3 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTaskRegistration3, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IBackgroundTaskRegistration3, IBackgroundTaskRegistration);
impl IBackgroundTaskRegistration3 {
    pub fn TaskGroup(&self) -> windows_core::Result<BackgroundTaskRegistrationGroup> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TaskId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskProgressEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveProgress)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Completed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskCompletedEventHandler>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), canceltask).ok() }
    }
}
impl windows_core::RuntimeType for IBackgroundTaskRegistration3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskRegistration3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TaskGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskRegistrationGroup, IBackgroundTaskRegistrationGroup_Vtbl, 0x2ab1919a_871b_4167_8a76_055cd67b5b23);
impl windows_core::RuntimeType for IBackgroundTaskRegistrationGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub BackgroundActivated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    BackgroundActivated: usize,
    pub RemoveBackgroundActivated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
windows_core::imp::define_interface!(IBackgroundTaskRegistrationGroupFactory, IBackgroundTaskRegistrationGroupFactory_Vtbl, 0x83d92b69_44cf_4631_9740_03c7d8741bc5);
impl windows_core::RuntimeType for IBackgroundTaskRegistrationGroupFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationGroupFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTaskRegistrationStatics, IBackgroundTaskRegistrationStatics_Vtbl, 0x4c542f69_b000_42ba_a093_6a563c65e3f8);
impl windows_core::RuntimeType for IBackgroundTaskRegistrationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
windows_core::imp::define_interface!(IBackgroundTaskRegistrationStatics2, IBackgroundTaskRegistrationStatics2_Vtbl, 0x174b671e_b20d_4fa9_ad9a_e93ad6c71e01);
impl windows_core::RuntimeType for IBackgroundTaskRegistrationStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTaskRegistrationStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTaskGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTaskGroups: usize,
    pub GetTaskGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBackgroundTrigger, IBackgroundTrigger_Vtbl, 0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
impl core::ops::Deref for IBackgroundTrigger {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBackgroundTrigger, windows_core::IUnknown, windows_core::IInspectable);
impl IBackgroundTrigger {}
impl windows_core::RuntimeType for IBackgroundTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IBackgroundWorkCostStatics, IBackgroundWorkCostStatics_Vtbl, 0xc740a662_c310_4b82_b3e3_3bcfb9e4c77d);
impl windows_core::RuntimeType for IBackgroundWorkCostStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBackgroundWorkCostStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentBackgroundWorkCost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BackgroundWorkCostValue) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisherTrigger, IBluetoothLEAdvertisementPublisherTrigger_Vtbl, 0xab3e2612_25d3_48ae_8724_d81877ae6129);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisherTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub Advertisement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    Advertisement: usize,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementPublisherTrigger2, IBluetoothLEAdvertisementPublisherTrigger2_Vtbl, 0xaa28d064_38f4_597d_b597_4e55588c6503);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementPublisherTrigger2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UseExtendedFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseExtendedFormat: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAnonymous: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncludeTransmitPowerLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeTransmitPowerLevel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementWatcherTrigger, IBluetoothLEAdvertisementWatcherTrigger_Vtbl, 0x1aab1819_bce1_48eb_a827_59fb7cee52a6);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcherTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MinSamplingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MaxSamplingInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MinOutOfRangeTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth")]
    pub SignalStrengthFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    SignalStrengthFilter: usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub SetSignalStrengthFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    SetSignalStrengthFilter: usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub AdvertisementFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    AdvertisementFilter: usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub SetAdvertisementFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    SetAdvertisementFilter: usize,
}
windows_core::imp::define_interface!(IBluetoothLEAdvertisementWatcherTrigger2, IBluetoothLEAdvertisementWatcherTrigger2_Vtbl, 0x39b56799_eb39_5ab6_9932_aa9e4549604d);
impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcherTrigger2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICachedFileUpdaterTrigger, ICachedFileUpdaterTrigger_Vtbl, 0xe21caeeb_32f2_4d31_b553_b9e01bde37e0);
impl windows_core::RuntimeType for ICachedFileUpdaterTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICachedFileUpdaterTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICachedFileUpdaterTriggerDetails, ICachedFileUpdaterTriggerDetails_Vtbl, 0x71838c13_1314_47b4_9597_dc7e248c17cc);
impl windows_core::RuntimeType for ICachedFileUpdaterTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICachedFileUpdaterTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Provider")]
    pub UpdateTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Storage::Provider::CachedFileTarget) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    UpdateTarget: usize,
    #[cfg(feature = "Storage_Provider")]
    pub UpdateRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    UpdateRequest: usize,
    pub CanRequestUserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChatMessageNotificationTrigger, IChatMessageNotificationTrigger_Vtbl, 0x513b43bf_1d40_5c5d_78f5_c923fee3739e);
impl windows_core::RuntimeType for IChatMessageNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageNotificationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IChatMessageReceivedNotificationTrigger, IChatMessageReceivedNotificationTrigger_Vtbl, 0x3ea3760e_baf5_4077_88e9_060cf6f0c6d5);
impl windows_core::RuntimeType for IChatMessageReceivedNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IChatMessageReceivedNotificationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICommunicationBlockingAppSetAsActiveTrigger, ICommunicationBlockingAppSetAsActiveTrigger_Vtbl, 0xfb91f28a_16a5_486d_974c_7835a8477be2);
impl windows_core::RuntimeType for ICommunicationBlockingAppSetAsActiveTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IContactStoreNotificationTrigger, IContactStoreNotificationTrigger_Vtbl, 0xc833419b_4705_4571_9a16_06b997bf9c96);
impl windows_core::RuntimeType for IContactStoreNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactStoreNotificationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IContentPrefetchTrigger, IContentPrefetchTrigger_Vtbl, 0x710627ee_04fa_440b_80c0_173202199e5d);
impl windows_core::RuntimeType for IContentPrefetchTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentPrefetchTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WaitInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContentPrefetchTriggerFactory, IContentPrefetchTriggerFactory_Vtbl, 0xc2643eda_8a03_409e_b8c4_88814c28ccb6);
impl windows_core::RuntimeType for IContentPrefetchTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContentPrefetchTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICustomSystemEventTrigger, ICustomSystemEventTrigger_Vtbl, 0xf3596798_cf6b_4ef4_a0ca_29cf4a278c87);
impl windows_core::RuntimeType for ICustomSystemEventTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomSystemEventTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TriggerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CustomSystemEventTriggerRecurrence) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICustomSystemEventTriggerFactory, ICustomSystemEventTriggerFactory_Vtbl, 0x6bcb16c5_f2dc_41b2_9efd_b96bdcd13ced);
impl windows_core::RuntimeType for ICustomSystemEventTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomSystemEventTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, CustomSystemEventTriggerRecurrence, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceConnectionChangeTrigger, IDeviceConnectionChangeTrigger_Vtbl, 0x90875e64_3cdd_4efb_ab1c_5b3b6a60ce34);
impl windows_core::RuntimeType for IDeviceConnectionChangeTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceConnectionChangeTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CanMaintainConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaintainConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetMaintainConnection: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceConnectionChangeTriggerStatics, IDeviceConnectionChangeTriggerStatics_Vtbl, 0xc3ea246a_4efd_4498_aa60_a4e4e3b17ab9);
impl windows_core::RuntimeType for IDeviceConnectionChangeTriggerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceConnectionChangeTriggerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IDeviceManufacturerNotificationTrigger, IDeviceManufacturerNotificationTrigger_Vtbl, 0x81278ab5_41ab_16da_86c2_7f7bf0912f5b);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IDeviceManufacturerNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IDeviceManufacturerNotificationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub TriggerQualifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TriggerQualifier: usize,
    #[cfg(feature = "deprecated")]
    pub OneShot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OneShot: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IDeviceManufacturerNotificationTriggerFactory, IDeviceManufacturerNotificationTriggerFactory_Vtbl, 0x7955de75_25bb_4153_a1a2_3029fcabb652);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IDeviceManufacturerNotificationTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IDeviceManufacturerNotificationTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
windows_core::imp::define_interface!(IDeviceServicingTrigger, IDeviceServicingTrigger_Vtbl, 0x1ab217ad_6e34_49d3_9e6f_17f1b6dfa881);
impl windows_core::RuntimeType for IDeviceServicingTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceServicingTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAsyncSimple: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAsyncWithArguments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::TimeSpan, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceUseTrigger, IDeviceUseTrigger_Vtbl, 0x0da68011_334f_4d57_b6ec_6dca64b412e4);
impl windows_core::RuntimeType for IDeviceUseTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceUseTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAsyncSimple: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAsyncWithArguments: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceWatcherTrigger, IDeviceWatcherTrigger_Vtbl, 0xa4617fdd_8573_4260_befc_5bec89cb693d);
impl windows_core::RuntimeType for IDeviceWatcherTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeviceWatcherTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IEmailStoreNotificationTrigger, IEmailStoreNotificationTrigger_Vtbl, 0x986d06da_47eb_4268_a4f2_f3f77188388a);
impl windows_core::RuntimeType for IEmailStoreNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailStoreNotificationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IGattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTrigger_Vtbl, 0xe25f8fc8_0696_474f_a732_f292b0cebc5d);
impl windows_core::RuntimeType for IGattCharacteristicNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
}
windows_core::imp::define_interface!(IGattCharacteristicNotificationTrigger2, IGattCharacteristicNotificationTrigger2_Vtbl, 0x9322a2c4_ae0e_42f2_b28c_f51372e69245);
impl windows_core::RuntimeType for IGattCharacteristicNotificationTrigger2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTrigger2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub EventTriggeringMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    EventTriggeringMode: usize,
}
windows_core::imp::define_interface!(IGattCharacteristicNotificationTriggerFactory, IGattCharacteristicNotificationTriggerFactory_Vtbl, 0x57ba1995_b143_4575_9f6b_fd59d93ace1a);
impl windows_core::RuntimeType for IGattCharacteristicNotificationTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Create: usize,
}
windows_core::imp::define_interface!(IGattCharacteristicNotificationTriggerFactory2, IGattCharacteristicNotificationTriggerFactory2_Vtbl, 0x5998e91f_8a53_4e9f_a32c_23cd33664cee);
impl windows_core::RuntimeType for IGattCharacteristicNotificationTriggerFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattCharacteristicNotificationTriggerFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub CreateWithEventTriggeringMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile")))]
    CreateWithEventTriggeringMode: usize,
}
windows_core::imp::define_interface!(IGattServiceProviderTrigger, IGattServiceProviderTrigger_Vtbl, 0xddc6a3e9_1557_4bd8_8542_468aa0c696f6);
impl windows_core::RuntimeType for IGattServiceProviderTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TriggerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Service: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Service: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub SetAdvertisingParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    SetAdvertisingParameters: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub AdvertisingParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    AdvertisingParameters: usize,
}
windows_core::imp::define_interface!(IGattServiceProviderTriggerResult, IGattServiceProviderTriggerResult_Vtbl, 0x3c4691b1_b198_4e84_bad4_cf4ad299ed3a);
impl windows_core::RuntimeType for IGattServiceProviderTriggerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderTriggerResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Trigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth")]
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Bluetooth::BluetoothError) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    Error: usize,
}
windows_core::imp::define_interface!(IGattServiceProviderTriggerStatics, IGattServiceProviderTriggerStatics_Vtbl, 0xb413a36a_e294_4591_a5a6_64891a828153);
impl windows_core::RuntimeType for IGattServiceProviderTriggerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGattServiceProviderTriggerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeovisitTrigger, IGeovisitTrigger_Vtbl, 0x4818edaa_04e1_4127_9a4c_19351b8a80a4);
impl windows_core::RuntimeType for IGeovisitTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGeovisitTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub MonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::Geolocation::VisitMonitoringScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    MonitoringScope: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetMonitoringScope: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Devices::Geolocation::VisitMonitoringScope) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetMonitoringScope: usize,
}
windows_core::imp::define_interface!(ILocationTrigger, ILocationTrigger_Vtbl, 0x47666a1c_6877_481e_8026_ff7e14a811a0);
impl windows_core::RuntimeType for ILocationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TriggerType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LocationTriggerType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILocationTriggerFactory, ILocationTriggerFactory_Vtbl, 0x1106bb07_ff69_4e09_aa8b_1384ea475e98);
impl windows_core::RuntimeType for ILocationTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILocationTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, LocationTriggerType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMaintenanceTrigger, IMaintenanceTrigger_Vtbl, 0x68184c83_fc22_4ce5_841a_7239a9810047);
impl windows_core::RuntimeType for IMaintenanceTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMaintenanceTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FreshnessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OneShot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMaintenanceTriggerFactory, IMaintenanceTriggerFactory_Vtbl, 0x4b3ddb2e_97dd_4629_88b0_b06cf9482ae5);
impl windows_core::RuntimeType for IMaintenanceTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMaintenanceTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaProcessingTrigger, IMediaProcessingTrigger_Vtbl, 0x9a95be65_8a52_4b30_9011_cf38040ea8b0);
impl windows_core::RuntimeType for IMediaProcessingTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaProcessingTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAsyncWithArguments: usize,
}
windows_core::imp::define_interface!(INetworkOperatorHotspotAuthenticationTrigger, INetworkOperatorHotspotAuthenticationTrigger_Vtbl, 0xe756c791_3001_4de5_83c7_de61d88831d0);
impl windows_core::RuntimeType for INetworkOperatorHotspotAuthenticationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorHotspotAuthenticationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(INetworkOperatorNotificationTrigger, INetworkOperatorNotificationTrigger_Vtbl, 0x90089cc6_63cd_480c_95d1_6e6aef801e4a);
impl windows_core::RuntimeType for INetworkOperatorNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorNotificationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkOperatorNotificationTriggerFactory, INetworkOperatorNotificationTriggerFactory_Vtbl, 0x0a223e00_27d7_4353_adb9_9265aaea579d);
impl windows_core::RuntimeType for INetworkOperatorNotificationTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkOperatorNotificationTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneTrigger, IPhoneTrigger_Vtbl, 0x8dcfe99b_d4c5_49f1_b7d3_82e87a0e9dde);
impl windows_core::RuntimeType for IPhoneTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OneShot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub TriggerType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Calls::Background::PhoneTriggerType) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    TriggerType: usize,
}
windows_core::imp::define_interface!(IPhoneTriggerFactory, IPhoneTriggerFactory_Vtbl, 0xa0d93cda_5fc1_48fb_a546_32262040157b);
impl windows_core::RuntimeType for IPhoneTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::Calls::Background::PhoneTriggerType, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    Create: usize,
}
windows_core::imp::define_interface!(IPushNotificationTriggerFactory, IPushNotificationTriggerFactory_Vtbl, 0x6dd8ed1b_458e_4fc2_bc2e_d5664f77ed19);
impl windows_core::RuntimeType for IPushNotificationTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRcsEndUserMessageAvailableTrigger, IRcsEndUserMessageAvailableTrigger_Vtbl, 0x986d0d6a_b2f6_467f_a978_a44091c11a66);
impl windows_core::RuntimeType for IRcsEndUserMessageAvailableTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRcsEndUserMessageAvailableTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IRfcommConnectionTrigger, IRfcommConnectionTrigger_Vtbl, 0xe8c4cae2_0b53_4464_9394_fd875654de64);
impl windows_core::RuntimeType for IRfcommConnectionTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommConnectionTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub InboundConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    InboundConnection: usize,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub OutboundConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    OutboundConnection: usize,
    pub AllowMultipleConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowMultipleConnections: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub ProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Networking::Sockets::SocketProtectionLevel) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ProtectionLevel: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub SetProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Networking::Sockets::SocketProtectionLevel) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    SetProtectionLevel: usize,
    #[cfg(feature = "Networking")]
    pub RemoteHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    RemoteHostName: usize,
    #[cfg(feature = "Networking")]
    pub SetRemoteHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetRemoteHostName: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(ISecondaryAuthenticationFactorAuthenticationTrigger, ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl, 0xf237f327_5181_4f24_96a7_700a4e5fac62);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for ISecondaryAuthenticationFactorAuthenticationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISensorDataThresholdTrigger, ISensorDataThresholdTrigger_Vtbl, 0x5bc0f372_d48b_4b7f_abec_15f9bacc12e2);
impl windows_core::RuntimeType for ISensorDataThresholdTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISensorDataThresholdTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISensorDataThresholdTriggerFactory, ISensorDataThresholdTriggerFactory_Vtbl, 0x921fe675_7df0_4da3_97b3_e544ee857fe6);
impl windows_core::RuntimeType for ISensorDataThresholdTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISensorDataThresholdTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Sensors")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))]
    Create: usize,
}
windows_core::imp::define_interface!(ISmartCardTrigger, ISmartCardTrigger_Vtbl, 0xf53bc5ac_84ca_4972_8ce9_e58f97b37a50);
impl windows_core::RuntimeType for ISmartCardTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_SmartCards")]
    pub TriggerType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Devices::SmartCards::SmartCardTriggerType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    TriggerType: usize,
}
windows_core::imp::define_interface!(ISmartCardTriggerFactory, ISmartCardTriggerFactory_Vtbl, 0x63bf54c3_89c1_4e00_a9d3_97c629269dad);
impl windows_core::RuntimeType for ISmartCardTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmartCardTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_SmartCards")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Devices::SmartCards::SmartCardTriggerType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    Create: usize,
}
windows_core::imp::define_interface!(ISmsMessageReceivedTriggerFactory, ISmsMessageReceivedTriggerFactory_Vtbl, 0xea3ad8c8_6ba4_4ab2_8d21_bc6b09c77564);
impl windows_core::RuntimeType for ISmsMessageReceivedTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISmsMessageReceivedTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Sms")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    Create: usize,
}
windows_core::imp::define_interface!(ISocketActivityTrigger, ISocketActivityTrigger_Vtbl, 0xa9bbf810_9dde_4f8a_83e3_b0e0e7a50d70);
impl windows_core::RuntimeType for ISocketActivityTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISocketActivityTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageLibraryChangeTrackerTriggerFactory, IStorageLibraryChangeTrackerTriggerFactory_Vtbl, 0x1eb0ffd0_5a85_499e_a888_824607124f50);
impl windows_core::RuntimeType for IStorageLibraryChangeTrackerTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageLibraryChangeTrackerTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
}
windows_core::imp::define_interface!(IStorageLibraryContentChangedTrigger, IStorageLibraryContentChangedTrigger_Vtbl, 0x1637e0a7_829c_45bc_929b_a1e7ea78d89b);
impl windows_core::RuntimeType for IStorageLibraryContentChangedTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageLibraryContentChangedTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IStorageLibraryContentChangedTriggerStatics, IStorageLibraryContentChangedTriggerStatics_Vtbl, 0x7f9f1b39_5f90_4e12_914e_a7d8e0bbfb18);
impl windows_core::RuntimeType for IStorageLibraryContentChangedTriggerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStorageLibraryContentChangedTriggerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub CreateFromLibraries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    CreateFromLibraries: usize,
}
windows_core::imp::define_interface!(ISystemCondition, ISystemCondition_Vtbl, 0xc15fb476_89c5_420b_abd3_fb3030472128);
impl windows_core::RuntimeType for ISystemCondition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemCondition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConditionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemConditionType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemConditionFactory, ISystemConditionFactory_Vtbl, 0xd269d1f1_05a7_49ae_87d7_16b2b8b9a553);
impl windows_core::RuntimeType for ISystemConditionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemConditionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, SystemConditionType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemTrigger, ISystemTrigger_Vtbl, 0x1d80c776_3748_4463_8d7e_276dc139ac1c);
impl windows_core::RuntimeType for ISystemTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OneShot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TriggerType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemTriggerType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemTriggerFactory, ISystemTriggerFactory_Vtbl, 0xe80423d4_8791_4579_8126_87ec8aaa407a);
impl windows_core::RuntimeType for ISystemTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, SystemTriggerType, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimeTrigger, ITimeTrigger_Vtbl, 0x656e5556_0b2a_4377_ba70_3b45a935547f);
impl windows_core::RuntimeType for ITimeTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITimeTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FreshnessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OneShot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimeTriggerFactory, ITimeTriggerFactory_Vtbl, 0x38c682fe_9b54_45e6_b2f3_269b87a6f734);
impl windows_core::RuntimeType for ITimeTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITimeTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationActionTriggerFactory, IToastNotificationActionTriggerFactory_Vtbl, 0xb09dfc27_6480_4349_8125_97b3efaa0a3a);
impl windows_core::RuntimeType for IToastNotificationActionTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationActionTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationHistoryChangedTriggerFactory, IToastNotificationHistoryChangedTriggerFactory_Vtbl, 0x81c6faad_8797_4785_81b4_b0cccb73d1d9);
impl windows_core::RuntimeType for IToastNotificationHistoryChangedTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationHistoryChangedTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserNotificationChangedTriggerFactory, IUserNotificationChangedTriggerFactory_Vtbl, 0xcad4436c_69ab_4e18_a48a_5ed2ac435957);
impl windows_core::RuntimeType for IUserNotificationChangedTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserNotificationChangedTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Notifications")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Notifications::NotificationKinds, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    Create: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ActivitySensorTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActivitySensorTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActivitySensorTrigger, IBackgroundTrigger);
impl ActivitySensorTrigger {
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SubscribedActivities(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscribedActivities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SupportedActivities(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedActivities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MinimumReportInterval(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinimumReportInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(reportintervalinmilliseconds: u32) -> windows_core::Result<ActivitySensorTrigger> {
        Self::IActivitySensorTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), reportintervalinmilliseconds, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IActivitySensorTriggerFactory<R, F: FnOnce(&IActivitySensorTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ActivitySensorTrigger, IActivitySensorTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ActivitySensorTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActivitySensorTrigger>();
}
unsafe impl windows_core::Interface for ActivitySensorTrigger {
    type Vtable = IActivitySensorTrigger_Vtbl;
    const IID: windows_core::GUID = <IActivitySensorTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActivitySensorTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ActivitySensorTrigger";
}
unsafe impl Send for ActivitySensorTrigger {}
unsafe impl Sync for ActivitySensorTrigger {}
pub struct AlarmApplicationManager;
impl AlarmApplicationManager {
    pub fn RequestAccessAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetAccessStatus() -> windows_core::Result<AlarmAccessStatus> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAccessStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IAlarmApplicationManagerStatics<R, F: FnOnce(&IAlarmApplicationManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AlarmApplicationManager, IAlarmApplicationManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AlarmApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AlarmApplicationManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppBroadcastTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AppBroadcastTrigger, IBackgroundTrigger);
impl AppBroadcastTrigger {
    pub fn SetProviderInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AppBroadcastTriggerProviderInfo>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ProviderInfo(&self) -> windows_core::Result<AppBroadcastTriggerProviderInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAppBroadcastTrigger(providerkey: &windows_core::HSTRING) -> windows_core::Result<AppBroadcastTrigger> {
        Self::IAppBroadcastTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAppBroadcastTrigger)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(providerkey), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastTriggerFactory<R, F: FnOnce(&IAppBroadcastTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppBroadcastTrigger, IAppBroadcastTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppBroadcastTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastTrigger>();
}
unsafe impl windows_core::Interface for AppBroadcastTrigger {
    type Vtable = IAppBroadcastTrigger_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTrigger";
}
unsafe impl Send for AppBroadcastTrigger {}
unsafe impl Sync for AppBroadcastTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppBroadcastTriggerProviderInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppBroadcastTriggerProviderInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppBroadcastTriggerProviderInfo {
    pub fn SetDisplayNameResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayNameResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayNameResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayNameResource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLogoResource(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLogoResource)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn LogoResource(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogoResource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVideoKeyFrameInterval(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVideoKeyFrameInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoKeyFrameInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoKeyFrameInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxVideoBitrate(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxVideoBitrate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoBitrate(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxVideoBitrate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxVideoWidth(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxVideoWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoWidth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxVideoWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxVideoHeight(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxVideoHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoHeight(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxVideoHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppBroadcastTriggerProviderInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppBroadcastTriggerProviderInfo>();
}
unsafe impl windows_core::Interface for AppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfo_Vtbl;
    const IID: windows_core::GUID = <IAppBroadcastTriggerProviderInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppBroadcastTriggerProviderInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo";
}
unsafe impl Send for AppBroadcastTriggerProviderInfo {}
unsafe impl Sync for AppBroadcastTriggerProviderInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ApplicationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ApplicationTrigger, IBackgroundTrigger);
impl ApplicationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAsyncWithArguments<P0>(&self, arguments: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsyncWithArguments)(windows_core::Interface::as_raw(this), arguments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ApplicationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationTrigger>();
}
unsafe impl windows_core::Interface for ApplicationTrigger {
    type Vtable = IApplicationTrigger_Vtbl;
    const IID: windows_core::GUID = <IApplicationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTrigger";
}
unsafe impl Send for ApplicationTrigger {}
unsafe impl Sync for ApplicationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ApplicationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl ApplicationTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ApplicationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationTriggerDetails>();
}
unsafe impl windows_core::Interface for ApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IApplicationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTriggerDetails";
}
unsafe impl Send for ApplicationTriggerDetails {}
unsafe impl Sync for ApplicationTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppointmentStoreNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentStoreNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AppointmentStoreNotificationTrigger, IBackgroundTrigger);
impl AppointmentStoreNotificationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppointmentStoreNotificationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppointmentStoreNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentStoreNotificationTrigger>();
}
unsafe impl windows_core::Interface for AppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTrigger_Vtbl;
    const IID: windows_core::GUID = <IAppointmentStoreNotificationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger";
}
unsafe impl Send for AppointmentStoreNotificationTrigger {}
unsafe impl Sync for AppointmentStoreNotificationTrigger {}
pub struct BackgroundExecutionManager;
impl BackgroundExecutionManager {
    pub fn RequestAccessAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestAccessForApplicationAsync(applicationid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessForApplicationAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RemoveAccess() -> windows_core::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveAccess)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn RemoveAccessForApplication(applicationid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveAccessForApplication)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid)).ok() })
    }
    pub fn GetAccessStatus() -> windows_core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAccessStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GetAccessStatusForApplication(applicationid: &windows_core::HSTRING) -> windows_core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAccessStatusForApplication)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).map(|| result__)
        })
    }
    pub fn RequestAccessKindAsync(requestedaccess: BackgroundAccessRequestKind, reason: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessKindAsync)(windows_core::Interface::as_raw(this), requestedaccess, core::mem::transmute_copy(reason), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestAccessKindForModernStandbyAsync(requestedaccess: BackgroundAccessRequestKind, reason: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessKindForModernStandbyAsync)(windows_core::Interface::as_raw(this), requestedaccess, core::mem::transmute_copy(reason), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetAccessStatusForModernStandby() -> windows_core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAccessStatusForModernStandby)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GetAccessStatusForModernStandbyForApplication(applicationid: &windows_core::HSTRING) -> windows_core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAccessStatusForModernStandbyForApplication)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics<R, F: FnOnce(&IBackgroundExecutionManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics2<R, F: FnOnce(&IBackgroundExecutionManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics3<R, F: FnOnce(&IBackgroundExecutionManagerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BackgroundExecutionManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundExecutionManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTaskBuilder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTaskBuilder, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTaskBuilder {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTaskBuilder, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetTaskEntryPoint(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTaskEntryPoint)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn TaskEntryPoint(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskEntryPoint)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTrigger<P0>(&self, trigger: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundTrigger>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTrigger)(windows_core::Interface::as_raw(this), trigger.param().abi()).ok() }
    }
    pub fn AddCondition<P0>(&self, condition: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundCondition>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddCondition)(windows_core::Interface::as_raw(this), condition.param().abi()).ok() }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Register(&self) -> windows_core::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Register)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCancelOnConditionLoss(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCancelOnConditionLoss)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CancelOnConditionLoss(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelOnConditionLoss)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsNetworkRequested(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsNetworkRequested)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsNetworkRequested(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNetworkRequested)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TaskGroup(&self) -> windows_core::Result<BackgroundTaskRegistrationGroup> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTaskGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BackgroundTaskRegistrationGroup>,
    {
        let this = &windows_core::Interface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTaskGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetTaskEntryPointClsid(&self, taskentrypoint: windows_core::GUID) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskBuilder5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTaskEntryPointClsid)(windows_core::Interface::as_raw(this), taskentrypoint).ok() }
    }
}
impl windows_core::RuntimeType for BackgroundTaskBuilder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTaskBuilder>();
}
unsafe impl windows_core::Interface for BackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilder_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTaskBuilder as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTaskBuilder {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskBuilder";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTaskCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTaskCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTaskCompletedEventArgs {
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CheckResult(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CheckResult)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BackgroundTaskCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTaskCompletedEventArgs>();
}
unsafe impl windows_core::Interface for BackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTaskCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs";
}
unsafe impl Send for BackgroundTaskCompletedEventArgs {}
unsafe impl Sync for BackgroundTaskCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTaskDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTaskDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTaskDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BackgroundTaskDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTaskDeferral>();
}
unsafe impl windows_core::Interface for BackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferral_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTaskDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTaskDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskDeferral";
}
unsafe impl Send for BackgroundTaskDeferral {}
unsafe impl Sync for BackgroundTaskDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTaskProgressEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTaskProgressEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTaskProgressEventArgs {
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Progress(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BackgroundTaskProgressEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTaskProgressEventArgs>();
}
unsafe impl windows_core::Interface for BackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgs_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTaskProgressEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTaskProgressEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs";
}
unsafe impl Send for BackgroundTaskProgressEventArgs {}
unsafe impl Sync for BackgroundTaskProgressEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTaskRegistration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTaskRegistration, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BackgroundTaskRegistration, IBackgroundTaskRegistration, IBackgroundTaskRegistration2, IBackgroundTaskRegistration3);
impl BackgroundTaskRegistration {
    pub fn TaskId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskProgressEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProgress)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Completed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<BackgroundTaskCompletedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), canceltask).ok() }
    }
    pub fn Trigger(&self) -> windows_core::Result<IBackgroundTrigger> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Trigger)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TaskGroup(&self) -> windows_core::Result<BackgroundTaskRegistrationGroup> {
        let this = &windows_core::Interface::cast::<IBackgroundTaskRegistration3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks() -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::GUID, IBackgroundTaskRegistration>> {
        Self::IBackgroundTaskRegistrationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllTasks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTaskGroups() -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, BackgroundTaskRegistrationGroup>> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllTaskGroups)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetTaskGroup(groupid: &windows_core::HSTRING) -> windows_core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTaskGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(groupid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationStatics<R, F: FnOnce(&IBackgroundTaskRegistrationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTaskRegistration, IBackgroundTaskRegistrationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationStatics2<R, F: FnOnce(&IBackgroundTaskRegistrationStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTaskRegistration, IBackgroundTaskRegistrationStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BackgroundTaskRegistration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTaskRegistration>();
}
unsafe impl windows_core::Interface for BackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistration_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTaskRegistration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistration";
}
unsafe impl Send for BackgroundTaskRegistration {}
unsafe impl Sync for BackgroundTaskRegistration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BackgroundTaskRegistrationGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BackgroundTaskRegistrationGroup, windows_core::IUnknown, windows_core::IInspectable);
impl BackgroundTaskRegistrationGroup {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn BackgroundActivated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundActivated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBackgroundActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBackgroundActivated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::GUID, BackgroundTaskRegistration>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllTasks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(id: &windows_core::HSTRING) -> windows_core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithName(id: &windows_core::HSTRING, name: &windows_core::HSTRING) -> windows_core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationGroupFactory<R, F: FnOnce(&IBackgroundTaskRegistrationGroupFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundTaskRegistrationGroup, IBackgroundTaskRegistrationGroupFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BackgroundTaskRegistrationGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTaskRegistrationGroup>();
}
unsafe impl windows_core::Interface for BackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroup_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTaskRegistrationGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BackgroundTaskRegistrationGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup";
}
unsafe impl Send for BackgroundTaskRegistrationGroup {}
unsafe impl Sync for BackgroundTaskRegistrationGroup {}
pub struct BackgroundWorkCost;
impl BackgroundWorkCost {
    pub fn CurrentBackgroundWorkCost() -> windows_core::Result<BackgroundWorkCostValue> {
        Self::IBackgroundWorkCostStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentBackgroundWorkCost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundWorkCostStatics<R, F: FnOnce(&IBackgroundWorkCostStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BackgroundWorkCost, IBackgroundWorkCostStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BackgroundWorkCost {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundWorkCost";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BluetoothLEAdvertisementPublisherTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementPublisherTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BluetoothLEAdvertisementPublisherTrigger, IBackgroundTrigger);
impl BluetoothLEAdvertisementPublisherTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementPublisherTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn Advertisement(&self) -> windows_core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Advertisement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> windows_core::Result<super::super::Foundation::IReference<i16>> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredTransmitPowerLevelInDBm)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPreferredTransmitPowerLevelInDBm<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i16>>,
    {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPreferredTransmitPowerLevelInDBm)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn UseExtendedFormat(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseExtendedFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseExtendedFormat(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseExtendedFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAnonymous(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAnonymous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAnonymous(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAnonymous)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeTransmitPowerLevel(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeTransmitPowerLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeTransmitPowerLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementPublisherTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementPublisherTrigger>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementPublisherTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementPublisherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger";
}
unsafe impl Send for BluetoothLEAdvertisementPublisherTrigger {}
unsafe impl Sync for BluetoothLEAdvertisementPublisherTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BluetoothLEAdvertisementWatcherTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementWatcherTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BluetoothLEAdvertisementWatcherTrigger, IBackgroundTrigger);
impl BluetoothLEAdvertisementWatcherTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementWatcherTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MinSamplingInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinSamplingInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxSamplingInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSamplingInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinOutOfRangeTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinOutOfRangeTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxOutOfRangeTimeout(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxOutOfRangeTimeout)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SignalStrengthFilter(&self) -> windows_core::Result<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignalStrengthFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SetSignalStrengthFilter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSignalStrengthFilter)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn AdvertisementFilter(&self) -> windows_core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisementFilter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn SetAdvertisementFilter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdvertisementFilter)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowExtendedAdvertisements(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowExtendedAdvertisements)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowExtendedAdvertisements)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for BluetoothLEAdvertisementWatcherTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementWatcherTrigger>();
}
unsafe impl windows_core::Interface for BluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger_Vtbl;
    const IID: windows_core::GUID = <IBluetoothLEAdvertisementWatcherTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BluetoothLEAdvertisementWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger";
}
unsafe impl Send for BluetoothLEAdvertisementWatcherTrigger {}
unsafe impl Sync for BluetoothLEAdvertisementWatcherTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CachedFileUpdaterTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CachedFileUpdaterTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CachedFileUpdaterTrigger, IBackgroundTrigger);
impl CachedFileUpdaterTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CachedFileUpdaterTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CachedFileUpdaterTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICachedFileUpdaterTrigger>();
}
unsafe impl windows_core::Interface for CachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTrigger_Vtbl;
    const IID: windows_core::GUID = <ICachedFileUpdaterTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CachedFileUpdaterTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTrigger";
}
unsafe impl Send for CachedFileUpdaterTrigger {}
unsafe impl Sync for CachedFileUpdaterTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CachedFileUpdaterTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CachedFileUpdaterTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl CachedFileUpdaterTriggerDetails {
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateTarget(&self) -> windows_core::Result<super::super::Storage::Provider::CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateTarget)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateRequest(&self) -> windows_core::Result<super::super::Storage::Provider::FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateRequest)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanRequestUserInput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRequestUserInput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CachedFileUpdaterTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICachedFileUpdaterTriggerDetails>();
}
unsafe impl windows_core::Interface for CachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <ICachedFileUpdaterTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CachedFileUpdaterTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails";
}
unsafe impl Send for CachedFileUpdaterTriggerDetails {}
unsafe impl Sync for CachedFileUpdaterTriggerDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ChatMessageNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ChatMessageNotificationTrigger, IBackgroundTrigger);
impl ChatMessageNotificationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatMessageNotificationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ChatMessageNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageNotificationTrigger>();
}
unsafe impl windows_core::Interface for ChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTrigger_Vtbl;
    const IID: windows_core::GUID = <IChatMessageNotificationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageNotificationTrigger";
}
unsafe impl Send for ChatMessageNotificationTrigger {}
unsafe impl Sync for ChatMessageNotificationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ChatMessageReceivedNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ChatMessageReceivedNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ChatMessageReceivedNotificationTrigger, IBackgroundTrigger);
impl ChatMessageReceivedNotificationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ChatMessageReceivedNotificationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ChatMessageReceivedNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IChatMessageReceivedNotificationTrigger>();
}
unsafe impl windows_core::Interface for ChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTrigger_Vtbl;
    const IID: windows_core::GUID = <IChatMessageReceivedNotificationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ChatMessageReceivedNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger";
}
unsafe impl Send for ChatMessageReceivedNotificationTrigger {}
unsafe impl Sync for ChatMessageReceivedNotificationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CommunicationBlockingAppSetAsActiveTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CommunicationBlockingAppSetAsActiveTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CommunicationBlockingAppSetAsActiveTrigger, IBackgroundTrigger);
impl CommunicationBlockingAppSetAsActiveTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CommunicationBlockingAppSetAsActiveTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CommunicationBlockingAppSetAsActiveTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICommunicationBlockingAppSetAsActiveTrigger>();
}
unsafe impl windows_core::Interface for CommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTrigger_Vtbl;
    const IID: windows_core::GUID = <ICommunicationBlockingAppSetAsActiveTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CommunicationBlockingAppSetAsActiveTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger";
}
unsafe impl Send for CommunicationBlockingAppSetAsActiveTrigger {}
unsafe impl Sync for CommunicationBlockingAppSetAsActiveTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContactStoreNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactStoreNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ContactStoreNotificationTrigger, IBackgroundTrigger);
impl ContactStoreNotificationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContactStoreNotificationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ContactStoreNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactStoreNotificationTrigger>();
}
unsafe impl windows_core::Interface for ContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTrigger_Vtbl;
    const IID: windows_core::GUID = <IContactStoreNotificationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContactStoreNotificationTrigger";
}
unsafe impl Send for ContactStoreNotificationTrigger {}
unsafe impl Sync for ContactStoreNotificationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ContentPrefetchTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContentPrefetchTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ContentPrefetchTrigger, IBackgroundTrigger);
impl ContentPrefetchTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentPrefetchTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn WaitInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WaitInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(waitinterval: super::super::Foundation::TimeSpan) -> windows_core::Result<ContentPrefetchTrigger> {
        Self::IContentPrefetchTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), waitinterval, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IContentPrefetchTriggerFactory<R, F: FnOnce(&IContentPrefetchTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContentPrefetchTrigger, IContentPrefetchTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ContentPrefetchTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContentPrefetchTrigger>();
}
unsafe impl windows_core::Interface for ContentPrefetchTrigger {
    type Vtable = IContentPrefetchTrigger_Vtbl;
    const IID: windows_core::GUID = <IContentPrefetchTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContentPrefetchTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContentPrefetchTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ConversationalAgentTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConversationalAgentTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ConversationalAgentTrigger, IBackgroundTrigger);
impl ConversationalAgentTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ConversationalAgentTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ConversationalAgentTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for ConversationalAgentTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConversationalAgentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ConversationalAgentTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CustomSystemEventTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CustomSystemEventTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CustomSystemEventTrigger, IBackgroundTrigger);
impl CustomSystemEventTrigger {
    pub fn TriggerId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Recurrence(&self) -> windows_core::Result<CustomSystemEventTriggerRecurrence> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Recurrence)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(triggerid: &windows_core::HSTRING, recurrence: CustomSystemEventTriggerRecurrence) -> windows_core::Result<CustomSystemEventTrigger> {
        Self::ICustomSystemEventTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(triggerid), recurrence, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICustomSystemEventTriggerFactory<R, F: FnOnce(&ICustomSystemEventTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CustomSystemEventTrigger, ICustomSystemEventTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CustomSystemEventTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICustomSystemEventTrigger>();
}
unsafe impl windows_core::Interface for CustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTrigger_Vtbl;
    const IID: windows_core::GUID = <ICustomSystemEventTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomSystemEventTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CustomSystemEventTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DeviceConnectionChangeTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceConnectionChangeTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DeviceConnectionChangeTrigger, IBackgroundTrigger);
impl DeviceConnectionChangeTrigger {
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanMaintainConnection(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanMaintainConnection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaintainConnection(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaintainConnection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaintainConnection(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaintainConnection)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>> {
        Self::IDeviceConnectionChangeTriggerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDeviceConnectionChangeTriggerStatics<R, F: FnOnce(&IDeviceConnectionChangeTriggerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceConnectionChangeTrigger, IDeviceConnectionChangeTriggerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DeviceConnectionChangeTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceConnectionChangeTrigger>();
}
unsafe impl windows_core::Interface for DeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTrigger_Vtbl;
    const IID: windows_core::GUID = <IDeviceConnectionChangeTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceConnectionChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger";
}
unsafe impl Send for DeviceConnectionChangeTrigger {}
unsafe impl Sync for DeviceConnectionChangeTrigger {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DeviceManufacturerNotificationTrigger(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(DeviceManufacturerNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(DeviceManufacturerNotificationTrigger, IBackgroundTrigger);
#[cfg(feature = "deprecated")]
impl DeviceManufacturerNotificationTrigger {
    #[cfg(feature = "deprecated")]
    pub fn TriggerQualifier(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerQualifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn OneShot(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OneShot)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Create(triggerqualifier: &windows_core::HSTRING, oneshot: bool) -> windows_core::Result<DeviceManufacturerNotificationTrigger> {
        Self::IDeviceManufacturerNotificationTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(triggerqualifier), oneshot, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IDeviceManufacturerNotificationTriggerFactory<R, F: FnOnce(&IDeviceManufacturerNotificationTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceManufacturerNotificationTrigger, IDeviceManufacturerNotificationTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for DeviceManufacturerNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceManufacturerNotificationTrigger>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for DeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTrigger_Vtbl;
    const IID: windows_core::GUID = <IDeviceManufacturerNotificationTrigger as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for DeviceManufacturerNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DeviceServicingTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceServicingTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DeviceServicingTrigger, IBackgroundTrigger);
impl DeviceServicingTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceServicingTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestAsyncSimple(&self, deviceid: &windows_core::HSTRING, expectedduration: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsyncSimple)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), expectedduration, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAsyncWithArguments(&self, deviceid: &windows_core::HSTRING, expectedduration: super::super::Foundation::TimeSpan, arguments: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsyncWithArguments)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), expectedduration, core::mem::transmute_copy(arguments), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceServicingTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceServicingTrigger>();
}
unsafe impl windows_core::Interface for DeviceServicingTrigger {
    type Vtable = IDeviceServicingTrigger_Vtbl;
    const IID: windows_core::GUID = <IDeviceServicingTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceServicingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceServicingTrigger";
}
unsafe impl Send for DeviceServicingTrigger {}
unsafe impl Sync for DeviceServicingTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DeviceUseTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceUseTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DeviceUseTrigger, IBackgroundTrigger);
impl DeviceUseTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeviceUseTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestAsyncSimple(&self, deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsyncSimple)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAsyncWithArguments(&self, deviceid: &windows_core::HSTRING, arguments: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsyncWithArguments)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), core::mem::transmute_copy(arguments), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DeviceUseTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceUseTrigger>();
}
unsafe impl windows_core::Interface for DeviceUseTrigger {
    type Vtable = IDeviceUseTrigger_Vtbl;
    const IID: windows_core::GUID = <IDeviceUseTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceUseTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceUseTrigger";
}
unsafe impl Send for DeviceUseTrigger {}
unsafe impl Sync for DeviceUseTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct DeviceWatcherTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeviceWatcherTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DeviceWatcherTrigger, IBackgroundTrigger);
impl DeviceWatcherTrigger {}
impl windows_core::RuntimeType for DeviceWatcherTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeviceWatcherTrigger>();
}
unsafe impl windows_core::Interface for DeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTrigger_Vtbl;
    const IID: windows_core::GUID = <IDeviceWatcherTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeviceWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceWatcherTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct EmailStoreNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailStoreNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(EmailStoreNotificationTrigger, IBackgroundTrigger);
impl EmailStoreNotificationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<EmailStoreNotificationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for EmailStoreNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailStoreNotificationTrigger>();
}
unsafe impl windows_core::Interface for EmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTrigger_Vtbl;
    const IID: windows_core::GUID = <IEmailStoreNotificationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.EmailStoreNotificationTrigger";
}
unsafe impl Send for EmailStoreNotificationTrigger {}
unsafe impl Sync for EmailStoreNotificationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GattCharacteristicNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattCharacteristicNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GattCharacteristicNotificationTrigger, IBackgroundTrigger);
impl GattCharacteristicNotificationTrigger {
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Characteristic(&self) -> windows_core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Characteristic)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn EventTriggeringMode(&self) -> windows_core::Result<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode> {
        let this = &windows_core::Interface::cast::<IGattCharacteristicNotificationTrigger2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventTriggeringMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Create<P0>(characteristic: P0) -> windows_core::Result<GattCharacteristicNotificationTrigger>
    where
        P0: windows_core::Param<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>,
    {
        Self::IGattCharacteristicNotificationTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), characteristic.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub fn CreateWithEventTriggeringMode<P0>(characteristic: P0, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> windows_core::Result<GattCharacteristicNotificationTrigger>
    where
        P0: windows_core::Param<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>,
    {
        Self::IGattCharacteristicNotificationTriggerFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithEventTriggeringMode)(windows_core::Interface::as_raw(this), characteristic.param().abi(), eventtriggeringmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicNotificationTriggerFactory<R, F: FnOnce(&IGattCharacteristicNotificationTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicNotificationTriggerFactory2<R, F: FnOnce(&IGattCharacteristicNotificationTriggerFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTriggerFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattCharacteristicNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattCharacteristicNotificationTrigger>();
}
unsafe impl windows_core::Interface for GattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTrigger_Vtbl;
    const IID: windows_core::GUID = <IGattCharacteristicNotificationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattCharacteristicNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger";
}
unsafe impl Send for GattCharacteristicNotificationTrigger {}
unsafe impl Sync for GattCharacteristicNotificationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GattServiceProviderTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattServiceProviderTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GattServiceProviderTrigger, IBackgroundTrigger);
impl GattServiceProviderTrigger {
    pub fn TriggerId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Service(&self) -> windows_core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Service)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn SetAdvertisingParameters<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdvertisingParameters)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn AdvertisingParameters(&self) -> windows_core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvertisingParameters)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAsync(triggerid: &windows_core::HSTRING, serviceuuid: windows_core::GUID) -> windows_core::Result<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>> {
        Self::IGattServiceProviderTriggerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(triggerid), serviceuuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderTriggerStatics<R, F: FnOnce(&IGattServiceProviderTriggerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GattServiceProviderTrigger, IGattServiceProviderTriggerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GattServiceProviderTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattServiceProviderTrigger>();
}
unsafe impl windows_core::Interface for GattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTrigger_Vtbl;
    const IID: windows_core::GUID = <IGattServiceProviderTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattServiceProviderTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTrigger";
}
unsafe impl Send for GattServiceProviderTrigger {}
unsafe impl Sync for GattServiceProviderTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GattServiceProviderTriggerResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GattServiceProviderTriggerResult, windows_core::IUnknown, windows_core::IInspectable);
impl GattServiceProviderTriggerResult {
    pub fn Trigger(&self) -> windows_core::Result<GattServiceProviderTrigger> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Trigger)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn Error(&self) -> windows_core::Result<super::super::Devices::Bluetooth::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GattServiceProviderTriggerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGattServiceProviderTriggerResult>();
}
unsafe impl windows_core::Interface for GattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResult_Vtbl;
    const IID: windows_core::GUID = <IGattServiceProviderTriggerResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GattServiceProviderTriggerResult {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTriggerResult";
}
unsafe impl Send for GattServiceProviderTriggerResult {}
unsafe impl Sync for GattServiceProviderTriggerResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GeovisitTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GeovisitTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GeovisitTrigger, IBackgroundTrigger);
impl GeovisitTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GeovisitTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn MonitoringScope(&self) -> windows_core::Result<super::super::Devices::Geolocation::VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonitoringScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetMonitoringScope(&self, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMonitoringScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for GeovisitTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeovisitTrigger>();
}
unsafe impl windows_core::Interface for GeovisitTrigger {
    type Vtable = IGeovisitTrigger_Vtbl;
    const IID: windows_core::GUID = <IGeovisitTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GeovisitTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GeovisitTrigger";
}
unsafe impl Send for GeovisitTrigger {}
unsafe impl Sync for GeovisitTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LocationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LocationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LocationTrigger, IBackgroundTrigger);
impl LocationTrigger {
    pub fn TriggerType(&self) -> windows_core::Result<LocationTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(triggertype: LocationTriggerType) -> windows_core::Result<LocationTrigger> {
        Self::ILocationTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), triggertype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILocationTriggerFactory<R, F: FnOnce(&ILocationTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LocationTrigger, ILocationTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LocationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILocationTrigger>();
}
unsafe impl windows_core::Interface for LocationTrigger {
    type Vtable = ILocationTrigger_Vtbl;
    const IID: windows_core::GUID = <ILocationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LocationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.LocationTrigger";
}
unsafe impl Send for LocationTrigger {}
unsafe impl Sync for LocationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MaintenanceTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MaintenanceTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MaintenanceTrigger, IBackgroundTrigger);
impl MaintenanceTrigger {
    pub fn FreshnessTime(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FreshnessTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OneShot(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OneShot)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(freshnesstime: u32, oneshot: bool) -> windows_core::Result<MaintenanceTrigger> {
        Self::IMaintenanceTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), freshnesstime, oneshot, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMaintenanceTriggerFactory<R, F: FnOnce(&IMaintenanceTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MaintenanceTrigger, IMaintenanceTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MaintenanceTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMaintenanceTrigger>();
}
unsafe impl windows_core::Interface for MaintenanceTrigger {
    type Vtable = IMaintenanceTrigger_Vtbl;
    const IID: windows_core::GUID = <IMaintenanceTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MaintenanceTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MaintenanceTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaProcessingTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaProcessingTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaProcessingTrigger, IBackgroundTrigger);
impl MediaProcessingTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaProcessingTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RequestAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAsyncWithArguments<P0>(&self, arguments: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsyncWithArguments)(windows_core::Interface::as_raw(this), arguments.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaProcessingTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaProcessingTrigger>();
}
unsafe impl windows_core::Interface for MediaProcessingTrigger {
    type Vtable = IMediaProcessingTrigger_Vtbl;
    const IID: windows_core::GUID = <IMediaProcessingTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaProcessingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MediaProcessingTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandDeviceServiceNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MobileBroadbandDeviceServiceNotificationTrigger, IBackgroundTrigger);
impl MobileBroadbandDeviceServiceNotificationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandDeviceServiceNotificationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandDeviceServiceNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for MobileBroadbandDeviceServiceNotificationTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandDeviceServiceNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger";
}
unsafe impl Send for MobileBroadbandDeviceServiceNotificationTrigger {}
unsafe impl Sync for MobileBroadbandDeviceServiceNotificationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPcoDataChangeTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPcoDataChangeTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MobileBroadbandPcoDataChangeTrigger, IBackgroundTrigger);
impl MobileBroadbandPcoDataChangeTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandPcoDataChangeTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandPcoDataChangeTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for MobileBroadbandPcoDataChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPcoDataChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger";
}
unsafe impl Send for MobileBroadbandPcoDataChangeTrigger {}
unsafe impl Sync for MobileBroadbandPcoDataChangeTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandPinLockStateChangeTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandPinLockStateChangeTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MobileBroadbandPinLockStateChangeTrigger, IBackgroundTrigger);
impl MobileBroadbandPinLockStateChangeTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandPinLockStateChangeTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandPinLockStateChangeTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for MobileBroadbandPinLockStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandPinLockStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger";
}
unsafe impl Send for MobileBroadbandPinLockStateChangeTrigger {}
unsafe impl Sync for MobileBroadbandPinLockStateChangeTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandRadioStateChangeTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandRadioStateChangeTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MobileBroadbandRadioStateChangeTrigger, IBackgroundTrigger);
impl MobileBroadbandRadioStateChangeTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandRadioStateChangeTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandRadioStateChangeTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for MobileBroadbandRadioStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandRadioStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger";
}
unsafe impl Send for MobileBroadbandRadioStateChangeTrigger {}
unsafe impl Sync for MobileBroadbandRadioStateChangeTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MobileBroadbandRegistrationStateChangeTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MobileBroadbandRegistrationStateChangeTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MobileBroadbandRegistrationStateChangeTrigger, IBackgroundTrigger);
impl MobileBroadbandRegistrationStateChangeTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MobileBroadbandRegistrationStateChangeTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MobileBroadbandRegistrationStateChangeTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for MobileBroadbandRegistrationStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MobileBroadbandRegistrationStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger";
}
unsafe impl Send for MobileBroadbandRegistrationStateChangeTrigger {}
unsafe impl Sync for MobileBroadbandRegistrationStateChangeTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorDataUsageTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorDataUsageTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(NetworkOperatorDataUsageTrigger, IBackgroundTrigger);
impl NetworkOperatorDataUsageTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkOperatorDataUsageTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NetworkOperatorDataUsageTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for NetworkOperatorDataUsageTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorDataUsageTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger";
}
unsafe impl Send for NetworkOperatorDataUsageTrigger {}
unsafe impl Sync for NetworkOperatorDataUsageTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorHotspotAuthenticationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorHotspotAuthenticationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(NetworkOperatorHotspotAuthenticationTrigger, IBackgroundTrigger);
impl NetworkOperatorHotspotAuthenticationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkOperatorHotspotAuthenticationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NetworkOperatorHotspotAuthenticationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkOperatorHotspotAuthenticationTrigger>();
}
unsafe impl windows_core::Interface for NetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTrigger_Vtbl;
    const IID: windows_core::GUID = <INetworkOperatorHotspotAuthenticationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorHotspotAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NetworkOperatorNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkOperatorNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(NetworkOperatorNotificationTrigger, IBackgroundTrigger);
impl NetworkOperatorNotificationTrigger {
    pub fn NetworkAccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAccountId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(networkaccountid: &windows_core::HSTRING) -> windows_core::Result<NetworkOperatorNotificationTrigger> {
        Self::INetworkOperatorNotificationTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn INetworkOperatorNotificationTriggerFactory<R, F: FnOnce(&INetworkOperatorNotificationTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkOperatorNotificationTrigger, INetworkOperatorNotificationTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NetworkOperatorNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkOperatorNotificationTrigger>();
}
unsafe impl windows_core::Interface for NetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTrigger_Vtbl;
    const IID: windows_core::GUID = <INetworkOperatorNotificationTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkOperatorNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PaymentAppCanMakePaymentTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PaymentAppCanMakePaymentTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PaymentAppCanMakePaymentTrigger, IBackgroundTrigger);
impl PaymentAppCanMakePaymentTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PaymentAppCanMakePaymentTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PaymentAppCanMakePaymentTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for PaymentAppCanMakePaymentTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PaymentAppCanMakePaymentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger";
}
unsafe impl Send for PaymentAppCanMakePaymentTrigger {}
unsafe impl Sync for PaymentAppCanMakePaymentTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PhoneTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PhoneTrigger, IBackgroundTrigger);
impl PhoneTrigger {
    pub fn OneShot(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OneShot)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn TriggerType(&self) -> windows_core::Result<super::Calls::Background::PhoneTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn Create(r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool) -> windows_core::Result<PhoneTrigger> {
        Self::IPhoneTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), r#type, oneshot, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPhoneTriggerFactory<R, F: FnOnce(&IPhoneTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PhoneTrigger, IPhoneTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PhoneTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneTrigger>();
}
unsafe impl windows_core::Interface for PhoneTrigger {
    type Vtable = IPhoneTrigger_Vtbl;
    const IID: windows_core::GUID = <IPhoneTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PhoneTrigger";
}
unsafe impl Send for PhoneTrigger {}
unsafe impl Sync for PhoneTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PushNotificationTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PushNotificationTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PushNotificationTrigger, IBackgroundTrigger);
impl PushNotificationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PushNotificationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &windows_core::HSTRING) -> windows_core::Result<PushNotificationTrigger> {
        Self::IPushNotificationTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPushNotificationTriggerFactory<R, F: FnOnce(&IPushNotificationTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PushNotificationTrigger, IPushNotificationTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PushNotificationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for PushNotificationTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PushNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PushNotificationTrigger";
}
unsafe impl Send for PushNotificationTrigger {}
unsafe impl Sync for PushNotificationTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RcsEndUserMessageAvailableTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RcsEndUserMessageAvailableTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RcsEndUserMessageAvailableTrigger, IBackgroundTrigger);
impl RcsEndUserMessageAvailableTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RcsEndUserMessageAvailableTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RcsEndUserMessageAvailableTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRcsEndUserMessageAvailableTrigger>();
}
unsafe impl windows_core::Interface for RcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTrigger_Vtbl;
    const IID: windows_core::GUID = <IRcsEndUserMessageAvailableTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RcsEndUserMessageAvailableTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger";
}
unsafe impl Send for RcsEndUserMessageAvailableTrigger {}
unsafe impl Sync for RcsEndUserMessageAvailableTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RfcommConnectionTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RfcommConnectionTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RfcommConnectionTrigger, IBackgroundTrigger);
impl RfcommConnectionTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RfcommConnectionTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn InboundConnection(&self) -> windows_core::Result<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InboundConnection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn OutboundConnection(&self) -> windows_core::Result<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundConnection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AllowMultipleConnections(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowMultipleConnections)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowMultipleConnections(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowMultipleConnections)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn ProtectionLevel(&self) -> windows_core::Result<super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn SetProtectionLevel(&self, value: super::super::Networking::Sockets::SocketProtectionLevel) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Networking")]
    pub fn RemoteHostName(&self) -> windows_core::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteHostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking")]
    pub fn SetRemoteHostName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Networking::HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteHostName)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for RfcommConnectionTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRfcommConnectionTrigger>();
}
unsafe impl windows_core::Interface for RfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTrigger_Vtbl;
    const IID: windows_core::GUID = <IRfcommConnectionTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RfcommConnectionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RfcommConnectionTrigger";
}
unsafe impl Send for RfcommConnectionTrigger {}
unsafe impl Sync for RfcommConnectionTrigger {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SecondaryAuthenticationFactorAuthenticationTrigger(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(SecondaryAuthenticationFactorAuthenticationTrigger, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
windows_core::imp::required_hierarchy!(SecondaryAuthenticationFactorAuthenticationTrigger, IBackgroundTrigger);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SecondaryAuthenticationFactorAuthenticationTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for SecondaryAuthenticationFactorAuthenticationTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISecondaryAuthenticationFactorAuthenticationTrigger>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for SecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl;
    const IID: windows_core::GUID = <ISecondaryAuthenticationFactorAuthenticationTrigger as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for SecondaryAuthenticationFactorAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SensorDataThresholdTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SensorDataThresholdTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SensorDataThresholdTrigger, IBackgroundTrigger);
impl SensorDataThresholdTrigger {
    #[cfg(feature = "Devices_Sensors")]
    pub fn Create<P0>(threshold: P0) -> windows_core::Result<SensorDataThresholdTrigger>
    where
        P0: windows_core::Param<super::super::Devices::Sensors::ISensorDataThreshold>,
    {
        Self::ISensorDataThresholdTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), threshold.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISensorDataThresholdTriggerFactory<R, F: FnOnce(&ISensorDataThresholdTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SensorDataThresholdTrigger, ISensorDataThresholdTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SensorDataThresholdTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISensorDataThresholdTrigger>();
}
unsafe impl windows_core::Interface for SensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTrigger_Vtbl;
    const IID: windows_core::GUID = <ISensorDataThresholdTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SensorDataThresholdTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SensorDataThresholdTrigger";
}
unsafe impl Send for SensorDataThresholdTrigger {}
unsafe impl Sync for SensorDataThresholdTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SmartCardTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmartCardTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmartCardTrigger, IBackgroundTrigger);
impl SmartCardTrigger {
    #[cfg(feature = "Devices_SmartCards")]
    pub fn TriggerType(&self) -> windows_core::Result<super::super::Devices::SmartCards::SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_SmartCards")]
    pub fn Create(triggertype: super::super::Devices::SmartCards::SmartCardTriggerType) -> windows_core::Result<SmartCardTrigger> {
        Self::ISmartCardTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), triggertype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardTriggerFactory<R, F: FnOnce(&ISmartCardTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmartCardTrigger, ISmartCardTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmartCardTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISmartCardTrigger>();
}
unsafe impl windows_core::Interface for SmartCardTrigger {
    type Vtable = ISmartCardTrigger_Vtbl;
    const IID: windows_core::GUID = <ISmartCardTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmartCardTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmartCardTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SmsMessageReceivedTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SmsMessageReceivedTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SmsMessageReceivedTrigger, IBackgroundTrigger);
impl SmsMessageReceivedTrigger {
    #[cfg(feature = "Devices_Sms")]
    pub fn Create<P0>(filterrules: P0) -> windows_core::Result<SmsMessageReceivedTrigger>
    where
        P0: windows_core::Param<super::super::Devices::Sms::SmsFilterRules>,
    {
        Self::ISmsMessageReceivedTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), filterrules.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISmsMessageReceivedTriggerFactory<R, F: FnOnce(&ISmsMessageReceivedTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SmsMessageReceivedTrigger, ISmsMessageReceivedTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SmsMessageReceivedTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for SmsMessageReceivedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SmsMessageReceivedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmsMessageReceivedTrigger";
}
unsafe impl Send for SmsMessageReceivedTrigger {}
unsafe impl Sync for SmsMessageReceivedTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SocketActivityTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SocketActivityTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SocketActivityTrigger, IBackgroundTrigger);
impl SocketActivityTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SocketActivityTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsWakeFromLowPowerSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ISocketActivityTrigger>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWakeFromLowPowerSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SocketActivityTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for SocketActivityTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SocketActivityTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SocketActivityTrigger";
}
unsafe impl Send for SocketActivityTrigger {}
unsafe impl Sync for SocketActivityTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct StorageLibraryChangeTrackerTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageLibraryChangeTrackerTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageLibraryChangeTrackerTrigger, IBackgroundTrigger);
impl StorageLibraryChangeTrackerTrigger {
    #[cfg(feature = "Storage")]
    pub fn Create<P0>(tracker: P0) -> windows_core::Result<StorageLibraryChangeTrackerTrigger>
    where
        P0: windows_core::Param<super::super::Storage::StorageLibraryChangeTracker>,
    {
        Self::IStorageLibraryChangeTrackerTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), tracker.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryChangeTrackerTriggerFactory<R, F: FnOnce(&IStorageLibraryChangeTrackerTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageLibraryChangeTrackerTrigger, IStorageLibraryChangeTrackerTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StorageLibraryChangeTrackerTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for StorageLibraryChangeTrackerTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageLibraryChangeTrackerTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger";
}
unsafe impl Send for StorageLibraryChangeTrackerTrigger {}
unsafe impl Sync for StorageLibraryChangeTrackerTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct StorageLibraryContentChangedTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageLibraryContentChangedTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageLibraryContentChangedTrigger, IBackgroundTrigger);
impl StorageLibraryContentChangedTrigger {
    #[cfg(feature = "Storage")]
    pub fn Create<P0>(storagelibrary: P0) -> windows_core::Result<StorageLibraryContentChangedTrigger>
    where
        P0: windows_core::Param<super::super::Storage::StorageLibrary>,
    {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), storagelibrary.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn CreateFromLibraries<P0>(storagelibraries: P0) -> windows_core::Result<StorageLibraryContentChangedTrigger>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary>>,
    {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromLibraries)(windows_core::Interface::as_raw(this), storagelibraries.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryContentChangedTriggerStatics<R, F: FnOnce(&IStorageLibraryContentChangedTriggerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageLibraryContentChangedTrigger, IStorageLibraryContentChangedTriggerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StorageLibraryContentChangedTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageLibraryContentChangedTrigger>();
}
unsafe impl windows_core::Interface for StorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTrigger_Vtbl;
    const IID: windows_core::GUID = <IStorageLibraryContentChangedTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageLibraryContentChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemCondition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemCondition, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SystemCondition, IBackgroundCondition);
impl SystemCondition {
    pub fn ConditionType(&self) -> windows_core::Result<SystemConditionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConditionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(conditiontype: SystemConditionType) -> windows_core::Result<SystemCondition> {
        Self::ISystemConditionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), conditiontype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISystemConditionFactory<R, F: FnOnce(&ISystemConditionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemCondition, ISystemConditionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SystemCondition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemCondition>();
}
unsafe impl windows_core::Interface for SystemCondition {
    type Vtable = ISystemCondition_Vtbl;
    const IID: windows_core::GUID = <ISystemCondition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemCondition";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SystemTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SystemTrigger, IBackgroundTrigger);
impl SystemTrigger {
    pub fn OneShot(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OneShot)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TriggerType(&self) -> windows_core::Result<SystemTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggerType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(triggertype: SystemTriggerType, oneshot: bool) -> windows_core::Result<SystemTrigger> {
        Self::ISystemTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), triggertype, oneshot, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISystemTriggerFactory<R, F: FnOnce(&ISystemTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemTrigger, ISystemTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SystemTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemTrigger>();
}
unsafe impl windows_core::Interface for SystemTrigger {
    type Vtable = ISystemTrigger_Vtbl;
    const IID: windows_core::GUID = <ISystemTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TetheringEntitlementCheckTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TetheringEntitlementCheckTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TetheringEntitlementCheckTrigger, IBackgroundTrigger);
impl TetheringEntitlementCheckTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TetheringEntitlementCheckTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TetheringEntitlementCheckTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for TetheringEntitlementCheckTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TetheringEntitlementCheckTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger";
}
unsafe impl Send for TetheringEntitlementCheckTrigger {}
unsafe impl Sync for TetheringEntitlementCheckTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TimeTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TimeTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TimeTrigger, IBackgroundTrigger);
impl TimeTrigger {
    pub fn FreshnessTime(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FreshnessTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OneShot(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OneShot)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(freshnesstime: u32, oneshot: bool) -> windows_core::Result<TimeTrigger> {
        Self::ITimeTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), freshnesstime, oneshot, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITimeTriggerFactory<R, F: FnOnce(&ITimeTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TimeTrigger, ITimeTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TimeTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITimeTrigger>();
}
unsafe impl windows_core::Interface for TimeTrigger {
    type Vtable = ITimeTrigger_Vtbl;
    const IID: windows_core::GUID = <ITimeTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TimeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TimeTrigger";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastNotificationActionTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotificationActionTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ToastNotificationActionTrigger, IBackgroundTrigger);
impl ToastNotificationActionTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotificationActionTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &windows_core::HSTRING) -> windows_core::Result<ToastNotificationActionTrigger> {
        Self::IToastNotificationActionTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationActionTriggerFactory<R, F: FnOnce(&IToastNotificationActionTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotificationActionTrigger, IToastNotificationActionTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToastNotificationActionTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for ToastNotificationActionTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotificationActionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationActionTrigger";
}
unsafe impl Send for ToastNotificationActionTrigger {}
unsafe impl Sync for ToastNotificationActionTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastNotificationHistoryChangedTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotificationHistoryChangedTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ToastNotificationHistoryChangedTrigger, IBackgroundTrigger);
impl ToastNotificationHistoryChangedTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotificationHistoryChangedTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &windows_core::HSTRING) -> windows_core::Result<ToastNotificationHistoryChangedTrigger> {
        Self::IToastNotificationHistoryChangedTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationHistoryChangedTriggerFactory<R, F: FnOnce(&IToastNotificationHistoryChangedTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotificationHistoryChangedTrigger, IToastNotificationHistoryChangedTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToastNotificationHistoryChangedTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for ToastNotificationHistoryChangedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotificationHistoryChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger";
}
unsafe impl Send for ToastNotificationHistoryChangedTrigger {}
unsafe impl Sync for ToastNotificationHistoryChangedTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UserNotificationChangedTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserNotificationChangedTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UserNotificationChangedTrigger, IBackgroundTrigger);
impl UserNotificationChangedTrigger {
    #[cfg(feature = "UI_Notifications")]
    pub fn Create(notificationkinds: super::super::UI::Notifications::NotificationKinds) -> windows_core::Result<UserNotificationChangedTrigger> {
        Self::IUserNotificationChangedTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), notificationkinds, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserNotificationChangedTriggerFactory<R, F: FnOnce(&IUserNotificationChangedTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserNotificationChangedTrigger, IUserNotificationChangedTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserNotificationChangedTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for UserNotificationChangedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserNotificationChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.UserNotificationChangedTrigger";
}
unsafe impl Send for UserNotificationChangedTrigger {}
unsafe impl Sync for UserNotificationChangedTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiOnDemandHotspotConnectTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiOnDemandHotspotConnectTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WiFiOnDemandHotspotConnectTrigger, IBackgroundTrigger);
impl WiFiOnDemandHotspotConnectTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiOnDemandHotspotConnectTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WiFiOnDemandHotspotConnectTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for WiFiOnDemandHotspotConnectTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiOnDemandHotspotConnectTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.WiFiOnDemandHotspotConnectTrigger";
}
unsafe impl Send for WiFiOnDemandHotspotConnectTrigger {}
unsafe impl Sync for WiFiOnDemandHotspotConnectTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WiFiOnDemandHotspotUpdateMetadataTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WiFiOnDemandHotspotUpdateMetadataTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WiFiOnDemandHotspotUpdateMetadataTrigger, IBackgroundTrigger);
impl WiFiOnDemandHotspotUpdateMetadataTrigger {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WiFiOnDemandHotspotUpdateMetadataTrigger, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WiFiOnDemandHotspotUpdateMetadataTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBackgroundTrigger>();
}
unsafe impl windows_core::Interface for WiFiOnDemandHotspotUpdateMetadataTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
    const IID: windows_core::GUID = <IBackgroundTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WiFiOnDemandHotspotUpdateMetadataTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.WiFiOnDemandHotspotUpdateMetadataTrigger";
}
unsafe impl Send for WiFiOnDemandHotspotUpdateMetadataTrigger {}
unsafe impl Sync for WiFiOnDemandHotspotUpdateMetadataTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AlarmAccessStatus(pub i32);
impl AlarmAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithWakeupCapability: Self = Self(1i32);
    pub const AllowedWithoutWakeupCapability: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
}
impl windows_core::TypeKind for AlarmAccessStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AlarmAccessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AlarmAccessStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AlarmAccessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.AlarmAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ApplicationTriggerResult(pub i32);
impl ApplicationTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl windows_core::TypeKind for ApplicationTriggerResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ApplicationTriggerResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ApplicationTriggerResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ApplicationTriggerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.ApplicationTriggerResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundAccessRequestKind(pub i32);
impl BackgroundAccessRequestKind {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(1i32);
}
impl windows_core::TypeKind for BackgroundAccessRequestKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundAccessRequestKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundAccessRequestKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundAccessRequestKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundAccessRequestKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundAccessStatus(pub i32);
impl BackgroundAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithAlwaysOnRealTimeConnectivity: Self = Self(1i32);
    pub const AllowedMayUseActiveRealTimeConnectivity: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const AlwaysAllowed: Self = Self(4i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(5i32);
    pub const DeniedBySystemPolicy: Self = Self(6i32);
    pub const DeniedByUser: Self = Self(7i32);
}
impl windows_core::TypeKind for BackgroundAccessStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundAccessStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundAccessStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundAccessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundTaskCancellationReason(pub i32);
impl BackgroundTaskCancellationReason {
    pub const Abort: Self = Self(0i32);
    pub const Terminating: Self = Self(1i32);
    pub const LoggingOff: Self = Self(2i32);
    pub const ServicingUpdate: Self = Self(3i32);
    pub const IdleTask: Self = Self(4i32);
    pub const Uninstall: Self = Self(5i32);
    pub const ConditionLoss: Self = Self(6i32);
    pub const SystemPolicy: Self = Self(7i32);
    pub const QuietHoursEntered: Self = Self(8i32);
    pub const ExecutionTimeExceeded: Self = Self(9i32);
    pub const ResourceRevocation: Self = Self(10i32);
    pub const EnergySaver: Self = Self(11i32);
}
impl windows_core::TypeKind for BackgroundTaskCancellationReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundTaskCancellationReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundTaskCancellationReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundTaskCancellationReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundTaskCancellationReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundTaskThrottleCounter(pub i32);
impl BackgroundTaskThrottleCounter {
    pub const All: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl windows_core::TypeKind for BackgroundTaskThrottleCounter {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundTaskThrottleCounter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundTaskThrottleCounter").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundTaskThrottleCounter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundTaskThrottleCounter;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BackgroundWorkCostValue(pub i32);
impl BackgroundWorkCostValue {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl windows_core::TypeKind for BackgroundWorkCostValue {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BackgroundWorkCostValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BackgroundWorkCostValue").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BackgroundWorkCostValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundWorkCostValue;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CustomSystemEventTriggerRecurrence(pub i32);
impl CustomSystemEventTriggerRecurrence {
    pub const Once: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
}
impl windows_core::TypeKind for CustomSystemEventTriggerRecurrence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CustomSystemEventTriggerRecurrence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CustomSystemEventTriggerRecurrence").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CustomSystemEventTriggerRecurrence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.CustomSystemEventTriggerRecurrence;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceTriggerResult(pub i32);
impl DeviceTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const LowBattery: Self = Self(3i32);
}
impl windows_core::TypeKind for DeviceTriggerResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceTriggerResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceTriggerResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceTriggerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.DeviceTriggerResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LocationTriggerType(pub i32);
impl LocationTriggerType {
    pub const Geofence: Self = Self(0i32);
}
impl windows_core::TypeKind for LocationTriggerType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LocationTriggerType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LocationTriggerType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LocationTriggerType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.LocationTriggerType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaProcessingTriggerResult(pub i32);
impl MediaProcessingTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl windows_core::TypeKind for MediaProcessingTriggerResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaProcessingTriggerResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaProcessingTriggerResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaProcessingTriggerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.MediaProcessingTriggerResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemConditionType(pub i32);
impl SystemConditionType {
    pub const Invalid: Self = Self(0i32);
    pub const UserPresent: Self = Self(1i32);
    pub const UserNotPresent: Self = Self(2i32);
    pub const InternetAvailable: Self = Self(3i32);
    pub const InternetNotAvailable: Self = Self(4i32);
    pub const SessionConnected: Self = Self(5i32);
    pub const SessionDisconnected: Self = Self(6i32);
    pub const FreeNetworkAvailable: Self = Self(7i32);
    pub const BackgroundWorkCostNotHigh: Self = Self(8i32);
}
impl windows_core::TypeKind for SystemConditionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemConditionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemConditionType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SystemConditionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.SystemConditionType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SystemTriggerType(pub i32);
impl SystemTriggerType {
    pub const Invalid: Self = Self(0i32);
    pub const SmsReceived: Self = Self(1i32);
    pub const UserPresent: Self = Self(2i32);
    pub const UserAway: Self = Self(3i32);
    pub const NetworkStateChange: Self = Self(4i32);
    pub const ControlChannelReset: Self = Self(5i32);
    pub const InternetAvailable: Self = Self(6i32);
    pub const SessionConnected: Self = Self(7i32);
    pub const ServicingComplete: Self = Self(8i32);
    pub const LockScreenApplicationAdded: Self = Self(9i32);
    pub const LockScreenApplicationRemoved: Self = Self(10i32);
    pub const TimeZoneChange: Self = Self(11i32);
    pub const OnlineIdConnectedStateChange: Self = Self(12i32);
    pub const BackgroundWorkCostChange: Self = Self(13i32);
    pub const PowerStateChange: Self = Self(14i32);
    pub const DefaultSignInAccountChange: Self = Self(15i32);
}
impl windows_core::TypeKind for SystemTriggerType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SystemTriggerType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SystemTriggerType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SystemTriggerType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.SystemTriggerType;i4)");
}
windows_core::imp::define_interface!(BackgroundTaskCanceledEventHandler, BackgroundTaskCanceledEventHandler_Vtbl, 0xa6c4bac0_51f8_4c57_ac3f_156dd1680c4f);
impl BackgroundTaskCanceledEventHandler {
    pub fn new<F: FnMut(Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskCanceledEventHandlerBox::<F> { vtable: &BackgroundTaskCanceledEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0, reason: BackgroundTaskCancellationReason) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBackgroundTaskInstance>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), reason).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskCanceledEventHandlerBox<F: FnMut(Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const BackgroundTaskCanceledEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> windows_core::Result<()> + Send + 'static> BackgroundTaskCanceledEventHandlerBox<F> {
    const VTABLE: BackgroundTaskCanceledEventHandler_Vtbl = BackgroundTaskCanceledEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <BackgroundTaskCanceledEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, reason: BackgroundTaskCancellationReason) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), reason).into()
    }
}
impl windows_core::RuntimeType for BackgroundTaskCanceledEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct BackgroundTaskCanceledEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, BackgroundTaskCancellationReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(BackgroundTaskCompletedEventHandler, BackgroundTaskCompletedEventHandler_Vtbl, 0x5b38e929_a086_46a7_a678_439135822bcf);
impl BackgroundTaskCompletedEventHandler {
    pub fn new<F: FnMut(Option<&BackgroundTaskRegistration>, Option<&BackgroundTaskCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskCompletedEventHandlerBox::<F> { vtable: &BackgroundTaskCompletedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BackgroundTaskRegistration>,
        P1: windows_core::Param<BackgroundTaskCompletedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskCompletedEventHandlerBox<F: FnMut(Option<&BackgroundTaskRegistration>, Option<&BackgroundTaskCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const BackgroundTaskCompletedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&BackgroundTaskRegistration>, Option<&BackgroundTaskCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static> BackgroundTaskCompletedEventHandlerBox<F> {
    const VTABLE: BackgroundTaskCompletedEventHandler_Vtbl = BackgroundTaskCompletedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <BackgroundTaskCompletedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&args)).into()
    }
}
impl windows_core::RuntimeType for BackgroundTaskCompletedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct BackgroundTaskCompletedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(BackgroundTaskProgressEventHandler, BackgroundTaskProgressEventHandler_Vtbl, 0x46e0683c_8a88_4c99_804c_76897f6277a6);
impl BackgroundTaskProgressEventHandler {
    pub fn new<F: FnMut(Option<&BackgroundTaskRegistration>, Option<&BackgroundTaskProgressEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskProgressEventHandlerBox::<F> { vtable: &BackgroundTaskProgressEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BackgroundTaskRegistration>,
        P1: windows_core::Param<BackgroundTaskProgressEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskProgressEventHandlerBox<F: FnMut(Option<&BackgroundTaskRegistration>, Option<&BackgroundTaskProgressEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const BackgroundTaskProgressEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&BackgroundTaskRegistration>, Option<&BackgroundTaskProgressEventArgs>) -> windows_core::Result<()> + Send + 'static> BackgroundTaskProgressEventHandlerBox<F> {
    const VTABLE: BackgroundTaskProgressEventHandler_Vtbl = BackgroundTaskProgressEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <BackgroundTaskProgressEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&args)).into()
    }
}
impl windows_core::RuntimeType for BackgroundTaskProgressEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct BackgroundTaskProgressEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
