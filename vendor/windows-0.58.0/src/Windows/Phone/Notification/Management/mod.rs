windows_core::imp::define_interface!(IAccessoryManager, IAccessoryManager_Vtbl, 0x0d04a12c_883d_4aa7_bca7_fa4bb8bffee6);
impl windows_core::RuntimeType for IAccessoryManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccessoryManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RegisterAccessoryApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetNextTriggerDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessTriggerDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PhoneLineDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PhoneLineDetails: usize,
    pub GetPhoneLineDetails: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptPhoneCall: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AcceptPhoneCallOnEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, PhoneCallAudioEndpoint) -> windows_core::HRESULT,
    pub AcceptPhoneCallWithVideo: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub AcceptPhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, PhoneCallAudioEndpoint) -> windows_core::HRESULT,
    pub RejectPhoneCall: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RejectPhoneCallWithText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub MakePhoneCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MakePhoneCallOnAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, core::mem::MaybeUninit<windows_core::HSTRING>, PhoneCallAudioEndpoint) -> windows_core::HRESULT,
    pub MakePhoneCallWithVideo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MakePhoneCallWithVideoOnAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, core::mem::MaybeUninit<windows_core::HSTRING>, PhoneCallAudioEndpoint) -> windows_core::HRESULT,
    pub SwapPhoneCalls: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub HoldPhoneCall: unsafe extern "system" fn(*mut core::ffi::c_void, u32, bool) -> windows_core::HRESULT,
    pub EndPhoneCall: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetPhoneMute: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PhoneMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPhoneCallAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, PhoneCallAudioEndpoint) -> windows_core::HRESULT,
    pub PhoneCallAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallAudioEndpoint) -> windows_core::HRESULT,
    pub SnoozeAlarm: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SnoozeAlarmForSpecifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub DismissAlarm: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SnoozeReminder: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SnoozeReminderForSpecifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub DismissReminder: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub GetMediaMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaPlaybackCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlaybackCapability) -> windows_core::HRESULT,
    pub MediaPlaybackStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlaybackStatus) -> windows_core::HRESULT,
    pub PerformMediaPlaybackCommand: unsafe extern "system" fn(*mut core::ffi::c_void, PlaybackCommand) -> windows_core::HRESULT,
    pub DoNotDisturbEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DrivingModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub BatterySaverState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetApps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetApps: usize,
    pub EnableNotificationsForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisableNotificationsForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsNotificationEnabledForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub GetEnabledAccessoryNotificationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnableAccessoryNotificationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DisableAllAccessoryNotificationTypes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserConsent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetAppIcon: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetAppIcon: usize,
}
windows_core::imp::define_interface!(IAccessoryManager2, IAccessoryManager2_Vtbl, 0xbacad44d_d393_46c6_b80c_15fdf44d5386);
impl windows_core::RuntimeType for IAccessoryManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccessoryManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RingDevice: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SpeedDialList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SpeedDialList: usize,
    pub ClearToast: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsPhonePinLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IncreaseVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DecreaseVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetRingerVibrate: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool) -> windows_core::HRESULT,
    pub VolumeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllEmailAccounts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllEmailAccounts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFolders: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFolders: usize,
    pub EnableEmailNotificationEmailAccount: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisableEmailNotificationEmailAccount: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EnableEmailNotificationFolderFilter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EnableEmailNotificationFolderFilter: usize,
    pub UpdateEmailReadStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccessoryManager3, IAccessoryManager3_Vtbl, 0x81f75137_edc7_47e0_b2f7_7e577c833f7d);
impl windows_core::RuntimeType for IAccessoryManager3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccessoryManager3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SnoozeAlarmByInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DismissAlarmByInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SnoozeReminderByInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DismissReminderByInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccessoryNotificationTriggerDetails, IAccessoryNotificationTriggerDetails_Vtbl, 0x6968a7d4_e3ca_49cb_8c87_2c11cdff9646);
impl core::ops::Deref for IAccessoryNotificationTriggerDetails {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccessoryNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl IAccessoryNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for IAccessoryNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAccessoryNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TimeCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub AppDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AccessoryNotificationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AccessoryNotificationType) -> windows_core::HRESULT,
    pub StartedProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetStartedProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAlarmNotificationTriggerDetails, IAlarmNotificationTriggerDetails_Vtbl, 0x38f5fa30_c738_4da2_908c_775d83c36abb);
impl windows_core::RuntimeType for IAlarmNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAlarmNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AlarmId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub ReminderState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ReminderState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAlarmNotificationTriggerDetails2, IAlarmNotificationTriggerDetails2_Vtbl, 0xcf16e06a_7155_40fe_a9c2_7bd2127ef853);
impl windows_core::RuntimeType for IAlarmNotificationTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAlarmNotificationTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppNotificationInfo, IAppNotificationInfo_Vtbl, 0x2157bea5_e286_45d3_9bea_f790fc216e0e);
impl windows_core::RuntimeType for IAppNotificationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppNotificationInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBinaryId, IBinaryId_Vtbl, 0x4f0da531_5595_44b4_9181_ce4efa3fc168);
impl windows_core::RuntimeType for IBinaryId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBinaryId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICalendarChangedNotificationTriggerDetails, ICalendarChangedNotificationTriggerDetails_Vtbl, 0x4b8a3bfc_279d_42ab_9c68_3e87977bf216);
impl windows_core::RuntimeType for ICalendarChangedNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICalendarChangedNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EventType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CalendarChangedEvent) -> windows_core::HRESULT,
    pub ItemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICortanaTileNotificationTriggerDetails, ICortanaTileNotificationTriggerDetails_Vtbl, 0xdc0f01d5_1489_46bb_b73b_7f90067ecf27);
impl windows_core::RuntimeType for ICortanaTileNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICortanaTileNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LargeContent1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LargeContent2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EmphasizedText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NonWrappedSmallContent1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NonWrappedSmallContent2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NonWrappedSmallContent3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NonWrappedSmallContent4: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailAccountInfo, IEmailAccountInfo_Vtbl, 0xdfbc02ab_bda0_4568_927e_b2ede35818a1);
impl windows_core::RuntimeType for IEmailAccountInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailAccountInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailFolderInfo, IEmailFolderInfo_Vtbl, 0xc207150e_e237_46d6_90e6_4f529eeac1e2);
impl windows_core::RuntimeType for IEmailFolderInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailFolderInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsNotificationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailNotificationTriggerDetails, IEmailNotificationTriggerDetails_Vtbl, 0xf3b82612_46cf_4e70_8e0d_7b2e04ab492b);
impl windows_core::RuntimeType for IEmailNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SenderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SenderAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Email")]
    pub EmailMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Email"))]
    EmailMessage: usize,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailNotificationTriggerDetails2, IEmailNotificationTriggerDetails2_Vtbl, 0x168067e3_c56f_4ec7_bed1_f734e08de5b2);
impl windows_core::RuntimeType for IEmailNotificationTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailNotificationTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageEntryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEmailReadNotificationTriggerDetails, IEmailReadNotificationTriggerDetails_Vtbl, 0xf5b7a087_06f3_4e3e_8c42_325e67010413);
impl windows_core::RuntimeType for IEmailReadNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEmailReadNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ParentFolderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MessageEntryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaControlsTriggerDetails, IMediaControlsTriggerDetails_Vtbl, 0xfab4648b_ae45_4548_91ca_4ab0548e33b5);
impl windows_core::RuntimeType for IMediaControlsTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaControlsTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PlaybackStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlaybackStatus) -> windows_core::HRESULT,
    pub MediaMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaMetadata, IMediaMetadata_Vtbl, 0x9b50ddf7_bb6c_4330_b3cd_0704a54cdb80);
impl windows_core::RuntimeType for IMediaMetadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaMetadata_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Album: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Track: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
windows_core::imp::define_interface!(IPhoneCallDetails, IPhoneCallDetails_Vtbl, 0x0c1b6f53_f071_483e_bf33_ebd44b724447);
impl windows_core::RuntimeType for IPhoneCallDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneCallDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PhoneLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub CallId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CallTransport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallTransport) -> windows_core::HRESULT,
    pub CallMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneMediaType) -> windows_core::HRESULT,
    pub CallDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallDirection) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneCallState) -> windows_core::HRESULT,
    pub ConferenceCallId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContactName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PresetTextResponses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PresetTextResponses: usize,
}
windows_core::imp::define_interface!(IPhoneLineDetails, IPhoneLineDetails_Vtbl, 0x47eb32dc_33ed_49b9_995c_a296bac82b77);
impl windows_core::RuntimeType for IPhoneLineDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LineNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DefaultOutgoingLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub VoicemailCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RegistrationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneLineRegistrationState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneLineDetails2, IPhoneLineDetails2_Vtbl, 0xb30cd77d_0147_498c_8241_bf0cabc60a25);
impl windows_core::RuntimeType for IPhoneLineDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneLineDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MissedCallCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhoneNotificationTriggerDetails, IPhoneNotificationTriggerDetails_Vtbl, 0xccc2fdf7_09c3_4118_91bc_ca6323a8d383);
impl windows_core::RuntimeType for IPhoneNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPhoneNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PhoneNotificationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhoneNotificationType) -> windows_core::HRESULT,
    pub CallDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhoneLineChangedId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReminderNotificationTriggerDetails, IReminderNotificationTriggerDetails_Vtbl, 0x5bddaa5d_9f61_4bf0_9feb_10502bc0b0c2);
impl windows_core::RuntimeType for IReminderNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReminderNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReminderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub Appointment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    Appointment: usize,
    pub ReminderState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ReminderState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReminderNotificationTriggerDetails2, IReminderNotificationTriggerDetails2_Vtbl, 0xe715f9c0_504d_4c0f_a6b3_bcb9722c6cdd);
impl windows_core::RuntimeType for IReminderNotificationTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IReminderNotificationTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpeedDialEntry, ISpeedDialEntry_Vtbl, 0x9240b6db_872c_46dc_b62a_be4541b166f8);
impl windows_core::RuntimeType for ISpeedDialEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpeedDialEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PhoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub NumberType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ContactName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextResponse, ITextResponse_Vtbl, 0xe9cb74c3_2457_4cdb_8110_72f5e8e883e8);
impl windows_core::RuntimeType for ITextResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITextResponse_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationTriggerDetails, IToastNotificationTriggerDetails_Vtbl, 0xc9314895_4e6d_4e9d_afec_9e921b875ae8);
impl windows_core::RuntimeType for IToastNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text3: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Text4: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationTriggerDetails2, IToastNotificationTriggerDetails2_Vtbl, 0x3e0479dd_cac4_4f60_afa3_b925d9d83c93);
impl windows_core::RuntimeType for IToastNotificationTriggerDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationTriggerDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVolumeInfo, IVolumeInfo_Vtbl, 0x944dd118_7704_4481_b92e_d3ed3ece6322);
impl windows_core::RuntimeType for IVolumeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVolumeInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SystemVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CallVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MediaVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsVibrateEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VibrateState) -> windows_core::HRESULT,
}
pub struct AccessoryManager;
impl AccessoryManager {
    pub fn RegisterAccessoryApp() -> windows_core::Result<windows_core::HSTRING> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterAccessoryApp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetNextTriggerDetails() -> windows_core::Result<IAccessoryNotificationTriggerDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNextTriggerDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ProcessTriggerDetails<P0>(pdetails: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAccessoryNotificationTriggerDetails>,
    {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).ProcessTriggerDetails)(windows_core::Interface::as_raw(this), pdetails.param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PhoneLineDetails() -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<PhoneLineDetails>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneLineDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetPhoneLineDetails(phoneline: windows_core::GUID) -> windows_core::Result<PhoneLineDetails> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPhoneLineDetails)(windows_core::Interface::as_raw(this), phoneline, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AcceptPhoneCall(phonecallid: u32) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).AcceptPhoneCall)(windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn AcceptPhoneCallOnEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).AcceptPhoneCallOnEndpoint)(windows_core::Interface::as_raw(this), phonecallid, endpoint).ok() })
    }
    pub fn AcceptPhoneCallWithVideo(phonecallid: u32) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).AcceptPhoneCallWithVideo)(windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn AcceptPhoneCallWithVideoOnAudioEndpoint(phonecallid: u32, endpoint: PhoneCallAudioEndpoint) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).AcceptPhoneCallWithVideoOnAudioEndpoint)(windows_core::Interface::as_raw(this), phonecallid, endpoint).ok() })
    }
    pub fn RejectPhoneCall(phonecallid: u32) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).RejectPhoneCall)(windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn RejectPhoneCallWithText(phonecallid: u32, textresponseid: u32) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).RejectPhoneCallWithText)(windows_core::Interface::as_raw(this), phonecallid, textresponseid).ok() })
    }
    pub fn MakePhoneCall(phoneline: windows_core::GUID, phonenumber: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).MakePhoneCall)(windows_core::Interface::as_raw(this), phoneline, core::mem::transmute_copy(phonenumber)).ok() })
    }
    pub fn MakePhoneCallOnAudioEndpoint(phoneline: windows_core::GUID, phonenumber: &windows_core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).MakePhoneCallOnAudioEndpoint)(windows_core::Interface::as_raw(this), phoneline, core::mem::transmute_copy(phonenumber), endpoint).ok() })
    }
    pub fn MakePhoneCallWithVideo(phoneline: windows_core::GUID, phonenumber: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).MakePhoneCallWithVideo)(windows_core::Interface::as_raw(this), phoneline, core::mem::transmute_copy(phonenumber)).ok() })
    }
    pub fn MakePhoneCallWithVideoOnAudioEndpoint(phoneline: windows_core::GUID, phonenumber: &windows_core::HSTRING, endpoint: PhoneCallAudioEndpoint) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).MakePhoneCallWithVideoOnAudioEndpoint)(windows_core::Interface::as_raw(this), phoneline, core::mem::transmute_copy(phonenumber), endpoint).ok() })
    }
    pub fn SwapPhoneCalls(phonecallidtohold: u32, phonecallidonhold: u32) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).SwapPhoneCalls)(windows_core::Interface::as_raw(this), phonecallidtohold, phonecallidonhold).ok() })
    }
    pub fn HoldPhoneCall(phonecallid: u32, holdcall: bool) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).HoldPhoneCall)(windows_core::Interface::as_raw(this), phonecallid, holdcall).ok() })
    }
    pub fn EndPhoneCall(phonecallid: u32) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).EndPhoneCall)(windows_core::Interface::as_raw(this), phonecallid).ok() })
    }
    pub fn SetPhoneMute(value: bool) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).SetPhoneMute)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn PhoneMute() -> windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneMute)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetPhoneCallAudioEndpoint(value: PhoneCallAudioEndpoint) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).SetPhoneCallAudioEndpoint)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn PhoneCallAudioEndpoint() -> windows_core::Result<PhoneCallAudioEndpoint> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneCallAudioEndpoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SnoozeAlarm(alarmid: windows_core::GUID) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).SnoozeAlarm)(windows_core::Interface::as_raw(this), alarmid).ok() })
    }
    pub fn SnoozeAlarmForSpecifiedTime(alarmid: windows_core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).SnoozeAlarmForSpecifiedTime)(windows_core::Interface::as_raw(this), alarmid, timespan).ok() })
    }
    pub fn DismissAlarm(alarmid: windows_core::GUID) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).DismissAlarm)(windows_core::Interface::as_raw(this), alarmid).ok() })
    }
    pub fn SnoozeReminder(reminderid: windows_core::GUID) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).SnoozeReminder)(windows_core::Interface::as_raw(this), reminderid).ok() })
    }
    pub fn SnoozeReminderForSpecifiedTime(reminderid: windows_core::GUID, timespan: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).SnoozeReminderForSpecifiedTime)(windows_core::Interface::as_raw(this), reminderid, timespan).ok() })
    }
    pub fn DismissReminder(reminderid: windows_core::GUID) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).DismissReminder)(windows_core::Interface::as_raw(this), reminderid).ok() })
    }
    pub fn GetMediaMetadata() -> windows_core::Result<MediaMetadata> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMediaMetadata)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MediaPlaybackCapabilities() -> windows_core::Result<PlaybackCapability> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaPlaybackCapabilities)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MediaPlaybackStatus() -> windows_core::Result<PlaybackStatus> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaPlaybackStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PerformMediaPlaybackCommand(command: PlaybackCommand) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).PerformMediaPlaybackCommand)(windows_core::Interface::as_raw(this), command).ok() })
    }
    pub fn DoNotDisturbEnabled() -> windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DoNotDisturbEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DrivingModeEnabled() -> windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DrivingModeEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BatterySaverState() -> windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatterySaverState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetApps() -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::HSTRING, AppNotificationInfo>> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetApps)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn EnableNotificationsForApplication(appid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).EnableNotificationsForApplication)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appid)).ok() })
    }
    pub fn DisableNotificationsForApplication(appid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).DisableNotificationsForApplication)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appid)).ok() })
    }
    pub fn IsNotificationEnabledForApplication(appid: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNotificationEnabledForApplication)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appid), &mut result__).map(|| result__)
        })
    }
    pub fn GetEnabledAccessoryNotificationTypes() -> windows_core::Result<i32> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEnabledAccessoryNotificationTypes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn EnableAccessoryNotificationTypes(accessorynotificationtypes: i32) -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).EnableAccessoryNotificationTypes)(windows_core::Interface::as_raw(this), accessorynotificationtypes).ok() })
    }
    pub fn DisableAllAccessoryNotificationTypes() -> windows_core::Result<()> {
        Self::IAccessoryManager(|this| unsafe { (windows_core::Interface::vtable(this).DisableAllAccessoryNotificationTypes)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn GetUserConsent() -> windows_core::Result<bool> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUserConsent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetAppIcon(appid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        Self::IAccessoryManager(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppIcon)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RingDevice() -> windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).RingDevice)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SpeedDialList() -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<SpeedDialEntry>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpeedDialList)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ClearToast(instanceid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).ClearToast)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn IsPhonePinLocked() -> windows_core::Result<bool> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPhonePinLocked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IncreaseVolume(step: i32) -> windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).IncreaseVolume)(windows_core::Interface::as_raw(this), step).ok() })
    }
    pub fn DecreaseVolume(step: i32) -> windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).DecreaseVolume)(windows_core::Interface::as_raw(this), step).ok() })
    }
    pub fn SetMute(mute: bool) -> windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).SetMute)(windows_core::Interface::as_raw(this), mute).ok() })
    }
    pub fn SetRingerVibrate(ringer: bool, vibrate: bool) -> windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).SetRingerVibrate)(windows_core::Interface::as_raw(this), ringer, vibrate).ok() })
    }
    pub fn VolumeInfo() -> windows_core::Result<VolumeInfo> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VolumeInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllEmailAccounts() -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<EmailAccountInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAllEmailAccounts)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFolders(emailaccount: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<EmailFolderInfo>> {
        Self::IAccessoryManager2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolders)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(emailaccount), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn EnableEmailNotificationEmailAccount(emailaccount: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).EnableEmailNotificationEmailAccount)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(emailaccount)).ok() })
    }
    pub fn DisableEmailNotificationEmailAccount(emailaccount: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).DisableEmailNotificationEmailAccount)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(emailaccount)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EnableEmailNotificationFolderFilter<P0>(emailaccount: &windows_core::HSTRING, folders: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>>,
    {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).EnableEmailNotificationFolderFilter)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(emailaccount), folders.param().abi()).ok() })
    }
    pub fn UpdateEmailReadStatus<P0>(messageentryid: P0, isread: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BinaryId>,
    {
        Self::IAccessoryManager2(|this| unsafe { (windows_core::Interface::vtable(this).UpdateEmailReadStatus)(windows_core::Interface::as_raw(this), messageentryid.param().abi(), isread).ok() })
    }
    pub fn SnoozeAlarmByInstanceId(instanceid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (windows_core::Interface::vtable(this).SnoozeAlarmByInstanceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn DismissAlarmByInstanceId(instanceid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (windows_core::Interface::vtable(this).DismissAlarmByInstanceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn SnoozeReminderByInstanceId(instanceid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (windows_core::Interface::vtable(this).SnoozeReminderByInstanceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(instanceid)).ok() })
    }
    pub fn DismissReminderByInstanceId(instanceid: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IAccessoryManager3(|this| unsafe { (windows_core::Interface::vtable(this).DismissReminderByInstanceId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(instanceid)).ok() })
    }
    #[doc(hidden)]
    pub fn IAccessoryManager<R, F: FnOnce(&IAccessoryManager) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessoryManager, IAccessoryManager> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccessoryManager2<R, F: FnOnce(&IAccessoryManager2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessoryManager, IAccessoryManager2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAccessoryManager3<R, F: FnOnce(&IAccessoryManager3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessoryManager, IAccessoryManager3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AccessoryManager {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AccessoryManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AlarmNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AlarmNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AlarmNotificationTriggerDetails, IAccessoryNotificationTriggerDetails);
impl AlarmNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlarmId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlarmId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReminderState(&self) -> windows_core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReminderState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAlarmNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AlarmNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAlarmNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for AlarmNotificationTriggerDetails {
    type Vtable = IAlarmNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IAlarmNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AlarmNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AlarmNotificationTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AppNotificationInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppNotificationInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppNotificationInfo {
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
}
impl windows_core::RuntimeType for AppNotificationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppNotificationInfo>();
}
unsafe impl windows_core::Interface for AppNotificationInfo {
    type Vtable = IAppNotificationInfo_Vtbl;
    const IID: windows_core::GUID = <IAppNotificationInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppNotificationInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.AppNotificationInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BinaryId(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BinaryId, windows_core::IUnknown, windows_core::IInspectable);
impl BinaryId {
    pub fn Id(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for BinaryId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBinaryId>();
}
unsafe impl windows_core::Interface for BinaryId {
    type Vtable = IBinaryId_Vtbl;
    const IID: windows_core::GUID = <IBinaryId as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BinaryId {
    const NAME: &'static str = "Windows.Phone.Notification.Management.BinaryId";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CalendarChangedNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CalendarChangedNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CalendarChangedNotificationTriggerDetails, IAccessoryNotificationTriggerDetails);
impl CalendarChangedNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EventType(&self) -> windows_core::Result<CalendarChangedEvent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EventType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ItemId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CalendarChangedNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICalendarChangedNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for CalendarChangedNotificationTriggerDetails {
    type Vtable = ICalendarChangedNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <ICalendarChangedNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CalendarChangedNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CalendarChangedNotificationTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CortanaTileNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CortanaTileNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CortanaTileNotificationTriggerDetails, IAccessoryNotificationTriggerDetails);
impl CortanaTileNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LargeContent1(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LargeContent1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LargeContent2(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LargeContent2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EmphasizedText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmphasizedText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NonWrappedSmallContent1(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NonWrappedSmallContent1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NonWrappedSmallContent2(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NonWrappedSmallContent2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NonWrappedSmallContent3(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NonWrappedSmallContent3)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NonWrappedSmallContent4(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NonWrappedSmallContent4)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Source(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CortanaTileNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICortanaTileNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for CortanaTileNotificationTriggerDetails {
    type Vtable = ICortanaTileNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <ICortanaTileNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CortanaTileNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.CortanaTileNotificationTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailAccountInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailAccountInfo, windows_core::IUnknown, windows_core::IInspectable);
impl EmailAccountInfo {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsNotificationEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNotificationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for EmailAccountInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailAccountInfo>();
}
unsafe impl windows_core::Interface for EmailAccountInfo {
    type Vtable = IEmailAccountInfo_Vtbl;
    const IID: windows_core::GUID = <IEmailAccountInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailAccountInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailAccountInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailFolderInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailFolderInfo, windows_core::IUnknown, windows_core::IInspectable);
impl EmailFolderInfo {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsNotificationEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNotificationEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for EmailFolderInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailFolderInfo>();
}
unsafe impl windows_core::Interface for EmailFolderInfo {
    type Vtable = IEmailFolderInfo_Vtbl;
    const IID: windows_core::GUID = <IEmailFolderInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailFolderInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailFolderInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(EmailNotificationTriggerDetails, IAccessoryNotificationTriggerDetails);
impl EmailNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccountName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentFolderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentFolderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SenderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SenderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SenderAddress(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SenderAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Email")]
    pub fn EmailMessage(&self) -> windows_core::Result<super::super::super::ApplicationModel::Email::EmailMessage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EmailMessage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MessageEntryId(&self) -> windows_core::Result<BinaryId> {
        let this = &windows_core::Interface::cast::<IEmailNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageEntryId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EmailNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for EmailNotificationTriggerDetails {
    type Vtable = IEmailNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IEmailNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailNotificationTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EmailReadNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EmailReadNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(EmailReadNotificationTriggerDetails, IAccessoryNotificationTriggerDetails);
impl EmailReadNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccountName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentFolderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentFolderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MessageEntryId(&self) -> windows_core::Result<BinaryId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageEntryId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsRead(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for EmailReadNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEmailReadNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for EmailReadNotificationTriggerDetails {
    type Vtable = IEmailReadNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IEmailReadNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EmailReadNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.EmailReadNotificationTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaControlsTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaControlsTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaControlsTriggerDetails, IAccessoryNotificationTriggerDetails);
impl MediaControlsTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackStatus(&self) -> windows_core::Result<PlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PlaybackStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MediaMetadata(&self) -> windows_core::Result<MediaMetadata> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaMetadata)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaControlsTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaControlsTriggerDetails>();
}
unsafe impl windows_core::Interface for MediaControlsTriggerDetails {
    type Vtable = IMediaControlsTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IMediaControlsTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaControlsTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaControlsTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MediaMetadata(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaMetadata, windows_core::IUnknown, windows_core::IInspectable);
impl MediaMetadata {
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subtitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Artist(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Artist)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Album(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Album)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Track(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Track)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> windows_core::Result<super::super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaMetadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaMetadata>();
}
unsafe impl windows_core::Interface for MediaMetadata {
    type Vtable = IMediaMetadata_Vtbl;
    const IID: windows_core::GUID = <IMediaMetadata as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaMetadata {
    const NAME: &'static str = "Windows.Phone.Notification.Management.MediaMetadata";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneCallDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneCallDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneCallDetails {
    pub fn PhoneLine(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneLine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CallId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CallTransport(&self) -> windows_core::Result<PhoneCallTransport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallTransport)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CallMediaType(&self) -> windows_core::Result<PhoneMediaType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallMediaType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CallDirection(&self) -> windows_core::Result<PhoneCallDirection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallDirection)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<PhoneCallState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConferenceCallId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConferenceCallId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContactName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PresetTextResponses(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<TextResponse>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PresetTextResponses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PhoneCallDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneCallDetails>();
}
unsafe impl windows_core::Interface for PhoneCallDetails {
    type Vtable = IPhoneCallDetails_Vtbl;
    const IID: windows_core::GUID = <IPhoneCallDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneCallDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneCallDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneLineDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneLineDetails, windows_core::IUnknown, windows_core::IInspectable);
impl PhoneLineDetails {
    pub fn LineId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LineNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LineNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultOutgoingLine(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultOutgoingLine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VoicemailCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VoicemailCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RegistrationState(&self) -> windows_core::Result<PhoneLineRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegistrationState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MissedCallCount(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPhoneLineDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MissedCallCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneLineDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneLineDetails>();
}
unsafe impl windows_core::Interface for PhoneLineDetails {
    type Vtable = IPhoneLineDetails_Vtbl;
    const IID: windows_core::GUID = <IPhoneLineDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneLineDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneLineDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PhoneNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhoneNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PhoneNotificationTriggerDetails, IAccessoryNotificationTriggerDetails);
impl PhoneNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PhoneNotificationType(&self) -> windows_core::Result<PhoneNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CallDetails(&self) -> windows_core::Result<PhoneCallDetails> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PhoneLineChangedId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneLineChangedId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PhoneNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhoneNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for PhoneNotificationTriggerDetails {
    type Vtable = IPhoneNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IPhoneNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhoneNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.PhoneNotificationTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ReminderNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ReminderNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ReminderNotificationTriggerDetails, IAccessoryNotificationTriggerDetails);
impl ReminderNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReminderId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReminderId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Details(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Details)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn Appointment(&self) -> windows_core::Result<super::super::super::ApplicationModel::Appointments::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appointment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReminderState(&self) -> windows_core::Result<ReminderState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReminderState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IReminderNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ReminderNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IReminderNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for ReminderNotificationTriggerDetails {
    type Vtable = IReminderNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IReminderNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ReminderNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ReminderNotificationTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpeedDialEntry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpeedDialEntry, windows_core::IUnknown, windows_core::IInspectable);
impl SpeedDialEntry {
    pub fn PhoneNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NumberType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContactName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContactName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpeedDialEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpeedDialEntry>();
}
unsafe impl windows_core::Interface for SpeedDialEntry {
    type Vtable = ISpeedDialEntry_Vtbl;
    const IID: windows_core::GUID = <ISpeedDialEntry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpeedDialEntry {
    const NAME: &'static str = "Windows.Phone.Notification.Management.SpeedDialEntry";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TextResponse(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextResponse, windows_core::IUnknown, windows_core::IInspectable);
impl TextResponse {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TextResponse {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextResponse>();
}
unsafe impl windows_core::Interface for TextResponse {
    type Vtable = ITextResponse_Vtbl;
    const IID: windows_core::GUID = <ITextResponse as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextResponse {
    const NAME: &'static str = "Windows.Phone.Notification.Management.TextResponse";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ToastNotificationTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotificationTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ToastNotificationTriggerDetails, IAccessoryNotificationTriggerDetails);
impl ToastNotificationTriggerDetails {
    pub fn TimeCreated(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessoryNotificationType(&self) -> windows_core::Result<AccessoryNotificationType> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessoryNotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartedProcessing(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartedProcessing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartedProcessing(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAccessoryNotificationTriggerDetails>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStartedProcessing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Text1(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text1)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text2(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text2)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text3(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text3)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text4(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text4)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SuppressPopup(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuppressPopup)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InstanceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IToastNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstanceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ToastNotificationTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastNotificationTriggerDetails>();
}
unsafe impl windows_core::Interface for ToastNotificationTriggerDetails {
    type Vtable = IToastNotificationTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <IToastNotificationTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.ToastNotificationTriggerDetails";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VolumeInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VolumeInfo, windows_core::IUnknown, windows_core::IInspectable);
impl VolumeInfo {
    pub fn SystemVolume(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemVolume)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CallVolume(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CallVolume)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MediaVolume(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaVolume)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMuted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMuted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsVibrateEnabled(&self) -> windows_core::Result<VibrateState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVibrateEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for VolumeInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVolumeInfo>();
}
unsafe impl windows_core::Interface for VolumeInfo {
    type Vtable = IVolumeInfo_Vtbl;
    const IID: windows_core::GUID = <IVolumeInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VolumeInfo {
    const NAME: &'static str = "Windows.Phone.Notification.Management.VolumeInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AccessoryNotificationType(pub u32);
impl AccessoryNotificationType {
    pub const None: Self = Self(0u32);
    pub const Phone: Self = Self(1u32);
    pub const Email: Self = Self(2u32);
    pub const Reminder: Self = Self(4u32);
    pub const Alarm: Self = Self(8u32);
    pub const Toast: Self = Self(16u32);
    pub const AppUninstalled: Self = Self(32u32);
    pub const Dnd: Self = Self(64u32);
    pub const DrivingMode: Self = Self(128u32);
    pub const BatterySaver: Self = Self(256u32);
    pub const Media: Self = Self(512u32);
    pub const CortanaTile: Self = Self(1024u32);
    pub const ToastCleared: Self = Self(2048u32);
    pub const CalendarChanged: Self = Self(4096u32);
    pub const VolumeChanged: Self = Self(8192u32);
    pub const EmailReadStatusChanged: Self = Self(16384u32);
}
impl windows_core::TypeKind for AccessoryNotificationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AccessoryNotificationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AccessoryNotificationType").field(&self.0).finish()
    }
}
impl AccessoryNotificationType {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AccessoryNotificationType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AccessoryNotificationType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AccessoryNotificationType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AccessoryNotificationType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AccessoryNotificationType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for AccessoryNotificationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.AccessoryNotificationType;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CalendarChangedEvent(pub i32);
impl CalendarChangedEvent {
    pub const LostEvents: Self = Self(0i32);
    pub const AppointmentAdded: Self = Self(1i32);
    pub const AppointmentChanged: Self = Self(2i32);
    pub const AppointmentDeleted: Self = Self(3i32);
    pub const CalendarAdded: Self = Self(4i32);
    pub const CalendarChanged: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl windows_core::TypeKind for CalendarChangedEvent {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CalendarChangedEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CalendarChangedEvent").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CalendarChangedEvent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.CalendarChangedEvent;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallAudioEndpoint(pub i32);
impl PhoneCallAudioEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Speaker: Self = Self(1i32);
    pub const Handsfree: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneCallAudioEndpoint {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallAudioEndpoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallAudioEndpoint").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallAudioEndpoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallAudioEndpoint;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallDirection(pub i32);
impl PhoneCallDirection {
    pub const Incoming: Self = Self(0i32);
    pub const Outgoing: Self = Self(1i32);
}
impl windows_core::TypeKind for PhoneCallDirection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallDirection").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallDirection;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallState(pub i32);
impl PhoneCallState {
    pub const Unknown: Self = Self(0i32);
    pub const Ringing: Self = Self(1i32);
    pub const Talking: Self = Self(2i32);
    pub const Held: Self = Self(3i32);
    pub const Ended: Self = Self(4i32);
}
impl windows_core::TypeKind for PhoneCallState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneCallTransport(pub i32);
impl PhoneCallTransport {
    pub const Cellular: Self = Self(0i32);
    pub const Voip: Self = Self(1i32);
}
impl windows_core::TypeKind for PhoneCallTransport {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneCallTransport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneCallTransport").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneCallTransport {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneCallTransport;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneLineRegistrationState(pub i32);
impl PhoneLineRegistrationState {
    pub const Disconnected: Self = Self(0i32);
    pub const Home: Self = Self(1i32);
    pub const Roaming: Self = Self(2i32);
}
impl windows_core::TypeKind for PhoneLineRegistrationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneLineRegistrationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneLineRegistrationState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneLineRegistrationState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneLineRegistrationState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneMediaType(pub i32);
impl PhoneMediaType {
    pub const AudioOnly: Self = Self(0i32);
    pub const AudioVideo: Self = Self(1i32);
}
impl windows_core::TypeKind for PhoneMediaType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneMediaType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneMediaType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneMediaType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneMediaType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PhoneNotificationType(pub i32);
impl PhoneNotificationType {
    pub const NewCall: Self = Self(0i32);
    pub const CallChanged: Self = Self(1i32);
    pub const LineChanged: Self = Self(2i32);
    pub const PhoneCallAudioEndpointChanged: Self = Self(3i32);
    pub const PhoneMuteChanged: Self = Self(4i32);
}
impl windows_core::TypeKind for PhoneNotificationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PhoneNotificationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PhoneNotificationType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PhoneNotificationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PhoneNotificationType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlaybackCapability(pub u32);
impl PlaybackCapability {
    pub const None: Self = Self(0u32);
    pub const Play: Self = Self(1u32);
    pub const Pause: Self = Self(2u32);
    pub const Stop: Self = Self(4u32);
    pub const Record: Self = Self(8u32);
    pub const FastForward: Self = Self(16u32);
    pub const Rewind: Self = Self(32u32);
    pub const Next: Self = Self(64u32);
    pub const Previous: Self = Self(128u32);
    pub const ChannelUp: Self = Self(256u32);
    pub const ChannelDown: Self = Self(512u32);
}
impl windows_core::TypeKind for PlaybackCapability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlaybackCapability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlaybackCapability").field(&self.0).finish()
    }
}
impl PlaybackCapability {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PlaybackCapability {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PlaybackCapability {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PlaybackCapability {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PlaybackCapability {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PlaybackCapability {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PlaybackCapability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCapability;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlaybackCommand(pub i32);
impl PlaybackCommand {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl windows_core::TypeKind for PlaybackCommand {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlaybackCommand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlaybackCommand").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PlaybackCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackCommand;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PlaybackStatus(pub i32);
impl PlaybackStatus {
    pub const None: Self = Self(0i32);
    pub const TrackChanged: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl windows_core::TypeKind for PlaybackStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PlaybackStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PlaybackStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PlaybackStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.PlaybackStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ReminderState(pub i32);
impl ReminderState {
    pub const Active: Self = Self(0i32);
    pub const Snoozed: Self = Self(1i32);
    pub const Dismissed: Self = Self(2i32);
}
impl windows_core::TypeKind for ReminderState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ReminderState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ReminderState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ReminderState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.ReminderState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VibrateState(pub i32);
impl VibrateState {
    pub const RingerOffVibrateOff: Self = Self(0i32);
    pub const RingerOffVibrateOn: Self = Self(1i32);
    pub const RingerOnVibrateOff: Self = Self(2i32);
    pub const RingerOnVibrateOn: Self = Self(3i32);
}
impl windows_core::TypeKind for VibrateState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VibrateState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VibrateState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for VibrateState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Notification.Management.VibrateState;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
