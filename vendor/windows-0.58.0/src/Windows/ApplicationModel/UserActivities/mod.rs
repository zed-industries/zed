#[cfg(feature = "ApplicationModel_UserActivities_Core")]
pub mod Core;
windows_core::imp::define_interface!(IUserActivity, IUserActivity_Vtbl, 0xfc103e9e_2cab_4d36_aea2_b4bb556cef0f);
impl windows_core::RuntimeType for IUserActivity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserActivityState) -> windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub VisualElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActivationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivity2, IUserActivity2_Vtbl, 0x9dc40c62_08c4_47ac_aa9c_2bb2221c55fd);
impl windows_core::RuntimeType for IUserActivity2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivity2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToJson: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivity3, IUserActivity3_Vtbl, 0xe7697744_e1a2_5147_8e06_55f1eeef271c);
impl windows_core::RuntimeType for IUserActivity3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivity3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRoamable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRoamable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityAttribution, IUserActivityAttribution_Vtbl, 0x34a5c8b5_86dd_4aec_a491_6a4faea5d22e);
impl windows_core::RuntimeType for IUserActivityAttribution {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityAttribution_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IconUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIconUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AlternateText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAlternateText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AddImageQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAddImageQuery: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityAttributionFactory, IUserActivityAttributionFactory_Vtbl, 0xe62bd252_c566_4f42_9974_916c4d76377e);
impl windows_core::RuntimeType for IUserActivityAttributionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityAttributionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityChannel, IUserActivityChannel_Vtbl, 0xbac0f8b8_a0e4_483b_b948_9cbabd06070c);
impl windows_core::RuntimeType for IUserActivityChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityChannel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetOrCreateUserActivityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteActivityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteAllActivitiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityChannel2, IUserActivityChannel2_Vtbl, 0x1698e35b_eb7e_4ea0_bf17_a459e8be706c);
impl windows_core::RuntimeType for IUserActivityChannel2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityChannel2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRecentUserActivitiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRecentUserActivitiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessionHistoryItemsForUserActivityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessionHistoryItemsForUserActivityAsync: usize,
}
windows_core::imp::define_interface!(IUserActivityChannelStatics, IUserActivityChannelStatics_Vtbl, 0xc8c005ab_198d_4d80_abb2_c9775ec4a729);
impl windows_core::RuntimeType for IUserActivityChannelStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityChannelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityChannelStatics2, IUserActivityChannelStatics2_Vtbl, 0x8e87de30_aa4f_4624_9ad0_d40f3ba0317c);
impl windows_core::RuntimeType for IUserActivityChannelStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityChannelStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisableAutoSessionCreation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub TryGetForWebAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    TryGetForWebAccount: usize,
}
windows_core::imp::define_interface!(IUserActivityChannelStatics3, IUserActivityChannelStatics3_Vtbl, 0x53bc4ddb_bbdf_5984_802a_5305874e205c);
impl windows_core::RuntimeType for IUserActivityChannelStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityChannelStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IUserActivityContentInfo, IUserActivityContentInfo_Vtbl, 0xb399e5ad_137f_409d_822d_e1af27ce08dc);
impl core::ops::Deref for IUserActivityContentInfo {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUserActivityContentInfo, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivityContentInfo {
    pub fn ToJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToJson)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IUserActivityContentInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityContentInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToJson: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityContentInfoStatics, IUserActivityContentInfoStatics_Vtbl, 0x9988c34b_0386_4bc9_968a_8200b004144f);
impl windows_core::RuntimeType for IUserActivityContentInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityContentInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromJson: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityFactory, IUserActivityFactory_Vtbl, 0x7c385758_361d_4a67_8a3b_34ca2978f9a3);
impl windows_core::RuntimeType for IUserActivityFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWithActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityRequest, IUserActivityRequest_Vtbl, 0xa0ef6355_cf35_4ff0_8833_50cb4b72e06d);
impl windows_core::RuntimeType for IUserActivityRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetUserActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityRequestManager, IUserActivityRequestManager_Vtbl, 0x0c30be4e_903d_48d6_82d4_4043ed57791b);
impl windows_core::RuntimeType for IUserActivityRequestManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityRequestManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserActivityRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUserActivityRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityRequestManagerStatics, IUserActivityRequestManagerStatics_Vtbl, 0xc0392df1_224a_432c_81e5_0c76b4c4cefa);
impl windows_core::RuntimeType for IUserActivityRequestManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityRequestManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityRequestedEventArgs, IUserActivityRequestedEventArgs_Vtbl, 0xa4cc7a4c_8229_4cfd_a3bc_c61d318575a4);
impl windows_core::RuntimeType for IUserActivityRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivitySession, IUserActivitySession_Vtbl, 0xae434d78_24fa_44a3_ad48_6eda61aa1924);
impl windows_core::RuntimeType for IUserActivitySession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivitySession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivitySessionHistoryItem, IUserActivitySessionHistoryItem_Vtbl, 0xe8d59bd3_3e5d_49fd_98d7_6da97521e255);
impl windows_core::RuntimeType for IUserActivitySessionHistoryItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivitySessionHistoryItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserActivityStatics, IUserActivityStatics_Vtbl, 0x8c8fd333_0e09_47f6_9ac7_95cf5c39367b);
impl windows_core::RuntimeType for IUserActivityStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryParseFromJson: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryParseFromJsonArray: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryParseFromJsonArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ToJsonArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ToJsonArray: usize,
}
windows_core::imp::define_interface!(IUserActivityVisualElements, IUserActivityVisualElements_Vtbl, 0x94757513_262f_49ef_bbbf_9b75d2e85250);
impl windows_core::RuntimeType for IUserActivityVisualElements {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityVisualElements_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    pub Attribution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAttribution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Shell")]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    SetContent: usize,
    #[cfg(feature = "UI_Shell")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    Content: usize,
}
windows_core::imp::define_interface!(IUserActivityVisualElements2, IUserActivityVisualElements2_Vtbl, 0xcaae7fc7_3eef_4359_825c_9d51b9220de3);
impl windows_core::RuntimeType for IUserActivityVisualElements2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserActivityVisualElements2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AttributionDisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAttributionDisplayText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivity, windows_core::IUnknown, windows_core::IInspectable);
impl UserActivity {
    pub fn State(&self) -> windows_core::Result<UserActivityState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VisualElements(&self) -> windows_core::Result<UserActivityVisualElements> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VisualElements)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContentUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentType(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentType)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FallbackUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FallbackUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFallbackUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFallbackUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ActivationUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivationUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetActivationUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetActivationUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentInfo(&self) -> windows_core::Result<IUserActivityContentInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetContentInfo<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUserActivityContentInfo>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentInfo)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SaveAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateSession(&self) -> windows_core::Result<UserActivitySession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ToJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUserActivity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToJson)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsRoamable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUserActivity3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRoamable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRoamable(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUserActivity3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRoamable)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateWithActivityId(activityid: &windows_core::HSTRING) -> windows_core::Result<UserActivity> {
        Self::IUserActivityFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithActivityId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activityid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryParseFromJson(json: &windows_core::HSTRING) -> windows_core::Result<UserActivity> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseFromJson)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(json), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryParseFromJsonArray(json: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVector<UserActivity>> {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryParseFromJsonArray)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(json), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ToJsonArray<P0>(activities: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<UserActivity>>,
    {
        Self::IUserActivityStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToJsonArray)(windows_core::Interface::as_raw(this), activities.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityFactory<R, F: FnOnce(&IUserActivityFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivity, IUserActivityFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityStatics<R, F: FnOnce(&IUserActivityStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivity, IUserActivityStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserActivity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivity>();
}
unsafe impl windows_core::Interface for UserActivity {
    type Vtable = IUserActivity_Vtbl;
    const IID: windows_core::GUID = <IUserActivity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivity {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivity";
}
unsafe impl Send for UserActivity {}
unsafe impl Sync for UserActivity {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivityAttribution(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivityAttribution, windows_core::IUnknown, windows_core::IInspectable);
impl UserActivityAttribution {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivityAttribution, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IconUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIconUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIconUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AlternateText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlternateText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAlternateText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAlternateText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AddImageQuery(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddImageQuery)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAddImageQuery(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAddImageQuery)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateWithUri<P0>(iconuri: P0) -> windows_core::Result<UserActivityAttribution>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IUserActivityAttributionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithUri)(windows_core::Interface::as_raw(this), iconuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityAttributionFactory<R, F: FnOnce(&IUserActivityAttributionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivityAttribution, IUserActivityAttributionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserActivityAttribution {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivityAttribution>();
}
unsafe impl windows_core::Interface for UserActivityAttribution {
    type Vtable = IUserActivityAttribution_Vtbl;
    const IID: windows_core::GUID = <IUserActivityAttribution as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivityAttribution {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityAttribution";
}
unsafe impl Send for UserActivityAttribution {}
unsafe impl Sync for UserActivityAttribution {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivityChannel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivityChannel, windows_core::IUnknown, windows_core::IInspectable);
impl UserActivityChannel {
    pub fn GetOrCreateUserActivityAsync(&self, activityid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserActivity>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOrCreateUserActivityAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activityid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteActivityAsync(&self, activityid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteActivityAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activityid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteAllActivitiesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAllActivitiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecentUserActivitiesAsync(&self, maxuniqueactivities: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &windows_core::Interface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRecentUserActivitiesAsync)(windows_core::Interface::as_raw(this), maxuniqueactivities, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessionHistoryItemsForUserActivityAsync(&self, activityid: &windows_core::HSTRING, starttime: super::super::Foundation::DateTime) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<UserActivitySessionHistoryItem>>> {
        let this = &windows_core::Interface::cast::<IUserActivityChannel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSessionHistoryItemsForUserActivityAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activityid), starttime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<UserActivityChannel> {
        Self::IUserActivityChannelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn DisableAutoSessionCreation() -> windows_core::Result<()> {
        Self::IUserActivityChannelStatics2(|this| unsafe { (windows_core::Interface::vtable(this).DisableAutoSessionCreation)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn TryGetForWebAccount<P0>(account: P0) -> windows_core::Result<UserActivityChannel>
    where
        P0: windows_core::Param<super::super::Security::Credentials::WebAccount>,
    {
        Self::IUserActivityChannelStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetForWebAccount)(windows_core::Interface::as_raw(this), account.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<UserActivityChannel>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IUserActivityChannelStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics<R, F: FnOnce(&IUserActivityChannelStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivityChannel, IUserActivityChannelStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics2<R, F: FnOnce(&IUserActivityChannelStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivityChannel, IUserActivityChannelStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUserActivityChannelStatics3<R, F: FnOnce(&IUserActivityChannelStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivityChannel, IUserActivityChannelStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserActivityChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivityChannel>();
}
unsafe impl windows_core::Interface for UserActivityChannel {
    type Vtable = IUserActivityChannel_Vtbl;
    const IID: windows_core::GUID = <IUserActivityChannel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivityChannel {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityChannel";
}
unsafe impl Send for UserActivityChannel {}
unsafe impl Sync for UserActivityChannel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivityContentInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivityContentInfo, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UserActivityContentInfo, IUserActivityContentInfo);
impl UserActivityContentInfo {
    pub fn ToJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToJson)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromJson(value: &windows_core::HSTRING) -> windows_core::Result<UserActivityContentInfo> {
        Self::IUserActivityContentInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromJson)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityContentInfoStatics<R, F: FnOnce(&IUserActivityContentInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivityContentInfo, IUserActivityContentInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserActivityContentInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivityContentInfo>();
}
unsafe impl windows_core::Interface for UserActivityContentInfo {
    type Vtable = IUserActivityContentInfo_Vtbl;
    const IID: windows_core::GUID = <IUserActivityContentInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityContentInfo";
}
unsafe impl Send for UserActivityContentInfo {}
unsafe impl Sync for UserActivityContentInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivityRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivityRequest, windows_core::IUnknown, windows_core::IInspectable);
impl UserActivityRequest {
    pub fn SetUserActivity<P0>(&self, activity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UserActivity>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUserActivity)(windows_core::Interface::as_raw(this), activity.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for UserActivityRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivityRequest>();
}
unsafe impl windows_core::Interface for UserActivityRequest {
    type Vtable = IUserActivityRequest_Vtbl;
    const IID: windows_core::GUID = <IUserActivityRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivityRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequest";
}
unsafe impl Send for UserActivityRequest {}
unsafe impl Sync for UserActivityRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivityRequestManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivityRequestManager, windows_core::IUnknown, windows_core::IInspectable);
impl UserActivityRequestManager {
    pub fn UserActivityRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UserActivityRequestManager, UserActivityRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserActivityRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserActivityRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserActivityRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<UserActivityRequestManager> {
        Self::IUserActivityRequestManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserActivityRequestManagerStatics<R, F: FnOnce(&IUserActivityRequestManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserActivityRequestManager, IUserActivityRequestManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserActivityRequestManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivityRequestManager>();
}
unsafe impl windows_core::Interface for UserActivityRequestManager {
    type Vtable = IUserActivityRequestManager_Vtbl;
    const IID: windows_core::GUID = <IUserActivityRequestManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivityRequestManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivityRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivityRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UserActivityRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<UserActivityRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserActivityRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivityRequestedEventArgs>();
}
unsafe impl windows_core::Interface for UserActivityRequestedEventArgs {
    type Vtable = IUserActivityRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUserActivityRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivityRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityRequestedEventArgs";
}
unsafe impl Send for UserActivityRequestedEventArgs {}
unsafe impl Sync for UserActivityRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivitySession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivitySession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UserActivitySession, super::super::Foundation::IClosable);
impl UserActivitySession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserActivitySession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivitySession>();
}
unsafe impl windows_core::Interface for UserActivitySession {
    type Vtable = IUserActivitySession_Vtbl;
    const IID: windows_core::GUID = <IUserActivitySession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivitySession {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySession";
}
unsafe impl Send for UserActivitySession {}
unsafe impl Sync for UserActivitySession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivitySessionHistoryItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivitySessionHistoryItem, windows_core::IUnknown, windows_core::IInspectable);
impl UserActivitySessionHistoryItem {
    pub fn UserActivity(&self) -> windows_core::Result<UserActivity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserActivity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserActivitySessionHistoryItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivitySessionHistoryItem>();
}
unsafe impl windows_core::Interface for UserActivitySessionHistoryItem {
    type Vtable = IUserActivitySessionHistoryItem_Vtbl;
    const IID: windows_core::GUID = <IUserActivitySessionHistoryItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivitySessionHistoryItem {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivitySessionHistoryItem";
}
unsafe impl Send for UserActivitySessionHistoryItem {}
unsafe impl Sync for UserActivitySessionHistoryItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UserActivityVisualElements(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserActivityVisualElements, windows_core::IUnknown, windows_core::IInspectable);
impl UserActivityVisualElements {
    pub fn DisplayText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Attribution(&self) -> windows_core::Result<UserActivityAttribution> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attribution)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAttribution<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UserActivityAttribution>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAttribution)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Shell")]
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::Shell::IAdaptiveCard>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Shell")]
    pub fn Content(&self) -> windows_core::Result<super::super::UI::Shell::IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AttributionDisplayText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributionDisplayText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAttributionDisplayText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUserActivityVisualElements2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAttributionDisplayText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for UserActivityVisualElements {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserActivityVisualElements>();
}
unsafe impl windows_core::Interface for UserActivityVisualElements {
    type Vtable = IUserActivityVisualElements_Vtbl;
    const IID: windows_core::GUID = <IUserActivityVisualElements as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserActivityVisualElements {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.UserActivityVisualElements";
}
unsafe impl Send for UserActivityVisualElements {}
unsafe impl Sync for UserActivityVisualElements {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserActivityState(pub i32);
impl UserActivityState {
    pub const New: Self = Self(0i32);
    pub const Published: Self = Self(1i32);
}
impl windows_core::TypeKind for UserActivityState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserActivityState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserActivityState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserActivityState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserActivities.UserActivityState;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
