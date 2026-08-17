#[cfg(feature = "UI_Notifications_Management")]
pub mod Management;
#[cfg(feature = "UI_Notifications_Preview")]
pub mod Preview;
windows_core::imp::define_interface!(IAdaptiveNotificationContent, IAdaptiveNotificationContent_Vtbl, 0xeb0dbe66_7448_448d_9db8_d78acd2abba9);
impl core::ops::Deref for IAdaptiveNotificationContent {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAdaptiveNotificationContent, windows_core::IUnknown, windows_core::IInspectable);
impl IAdaptiveNotificationContent {
    pub fn Kind(&self) -> windows_core::Result<AdaptiveNotificationContentKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IAdaptiveNotificationContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveNotificationContent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdaptiveNotificationContentKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Hints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Hints: usize,
}
windows_core::imp::define_interface!(IAdaptiveNotificationText, IAdaptiveNotificationText_Vtbl, 0x46d4a3be_609a_4326_a40b_bfde872034a3);
impl windows_core::RuntimeType for IAdaptiveNotificationText {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveNotificationText_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBadgeNotification, IBadgeNotification_Vtbl, 0x075cb4ca_d08a_4e2f_9233_7e289c1f7722);
impl windows_core::RuntimeType for IBadgeNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBadgeNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBadgeNotificationFactory, IBadgeNotificationFactory_Vtbl, 0xedf255ce_0618_4d59_948a_5a61040c52f9);
impl windows_core::RuntimeType for IBadgeNotificationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBadgeNotificationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateBadgeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateBadgeNotification: usize,
}
windows_core::imp::define_interface!(IBadgeUpdateManagerForUser, IBadgeUpdateManagerForUser_Vtbl, 0x996b21bc_0386_44e5_ba8d_0c1077a62e92);
impl windows_core::RuntimeType for IBadgeUpdateManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBadgeUpdateManagerForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateBadgeUpdaterForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBadgeUpdaterForApplicationWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBadgeUpdaterForSecondaryTile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IBadgeUpdateManagerStatics, IBadgeUpdateManagerStatics_Vtbl, 0x33400faa_6dd5_4105_aebc_9b50fca492da);
impl windows_core::RuntimeType for IBadgeUpdateManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBadgeUpdateManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateBadgeUpdaterForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBadgeUpdaterForApplicationWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBadgeUpdaterForSecondaryTile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(*mut core::ffi::c_void, BadgeTemplateType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
windows_core::imp::define_interface!(IBadgeUpdateManagerStatics2, IBadgeUpdateManagerStatics2_Vtbl, 0x979a35ce_f940_48bf_94e8_ca244d400b41);
impl windows_core::RuntimeType for IBadgeUpdateManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBadgeUpdateManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IBadgeUpdater, IBadgeUpdater_Vtbl, 0xb5fa1fd4_7562_4f6c_bfa3_1b6ed2e57f2f);
impl windows_core::RuntimeType for IBadgeUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBadgeUpdater_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPeriodicUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PeriodicUpdateRecurrence) -> windows_core::HRESULT,
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, PeriodicUpdateRecurrence) -> windows_core::HRESULT,
    pub StopPeriodicUpdate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKnownAdaptiveNotificationHintsStatics, IKnownAdaptiveNotificationHintsStatics_Vtbl, 0x06206598_d496_497d_8692_4f7d7c2770df);
impl windows_core::RuntimeType for IKnownAdaptiveNotificationHintsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownAdaptiveNotificationHintsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Style: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Wrap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MinLines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TextStacking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Align: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKnownAdaptiveNotificationTextStylesStatics, IKnownAdaptiveNotificationTextStylesStatics_Vtbl, 0x202192d7_8996_45aa_8ba1_d461d72c2a1b);
impl windows_core::RuntimeType for IKnownAdaptiveNotificationTextStylesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownAdaptiveNotificationTextStylesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Caption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Base: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subheader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Header: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TitleNumeral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SubheaderNumeral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HeaderNumeral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CaptionSubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BodySubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub BaseSubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SubtitleSubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TitleSubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SubheaderSubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SubheaderNumeralSubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HeaderSubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub HeaderNumeralSubtle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKnownNotificationBindingsStatics, IKnownNotificationBindingsStatics_Vtbl, 0x79427bae_a8b7_4d58_89ea_76a7b7bccded);
impl windows_core::RuntimeType for IKnownNotificationBindingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownNotificationBindingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToastGeneric: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INotification, INotification_Vtbl, 0x108037fe_eb76_4f82_97bc_da07530a2e20);
impl windows_core::RuntimeType for INotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Visual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INotificationBinding, INotificationBinding_Vtbl, 0xf29e4b85_0370_4ad3_b4ea_da9e35e7eabf);
impl windows_core::RuntimeType for INotificationBinding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotificationBinding_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Hints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Hints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextElements: usize,
}
windows_core::imp::define_interface!(INotificationData, INotificationData_Vtbl, 0x9ffd2312_9d6a_4aaf_b6ac_ff17f0c1f280);
impl windows_core::RuntimeType for INotificationData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotificationData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
    pub SequenceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSequenceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INotificationDataFactory, INotificationDataFactory_Vtbl, 0x23c1e33a_1c10_46fb_8040_dec384621cf8);
impl windows_core::RuntimeType for INotificationDataFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotificationDataFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNotificationDataWithValuesAndSequenceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNotificationDataWithValuesAndSequenceNumber: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNotificationDataWithValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNotificationDataWithValues: usize,
}
windows_core::imp::define_interface!(INotificationVisual, INotificationVisual_Vtbl, 0x68835b8e_aa56_4e11_86d3_5f9a6957bc5b);
impl windows_core::RuntimeType for INotificationVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INotificationVisual_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Bindings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bindings: usize,
    pub GetBinding: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScheduledTileNotification, IScheduledTileNotification_Vtbl, 0x0abca6d5_99dc_4c78_a11c_c9e7f86d7ef7);
impl windows_core::RuntimeType for IScheduledTileNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScheduledTileNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub DeliveryTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScheduledTileNotificationFactory, IScheduledTileNotificationFactory_Vtbl, 0x3383138a_98c0_4c3b_bbd6_4a633c7cfc29);
impl windows_core::RuntimeType for IScheduledTileNotificationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScheduledTileNotificationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateScheduledTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateScheduledTileNotification: usize,
}
windows_core::imp::define_interface!(IScheduledToastNotification, IScheduledToastNotification_Vtbl, 0x79f577f8_0de7_48cd_9740_9b370490c838);
impl windows_core::RuntimeType for IScheduledToastNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScheduledToastNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub DeliveryTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub SnoozeInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaximumSnoozeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScheduledToastNotification2, IScheduledToastNotification2_Vtbl, 0xa66ea09c_31b4_43b0_b5dd_7a40e85363b1);
impl windows_core::RuntimeType for IScheduledToastNotification2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScheduledToastNotification2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSuppressPopup: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScheduledToastNotification3, IScheduledToastNotification3_Vtbl, 0x98429e8b_bd32_4a3b_9d15_22aea49462a1);
impl windows_core::RuntimeType for IScheduledToastNotification3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScheduledToastNotification3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotificationMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NotificationMirroring) -> windows_core::HRESULT,
    pub SetNotificationMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, NotificationMirroring) -> windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScheduledToastNotification4, IScheduledToastNotification4_Vtbl, 0x1d4761fd_bdef_4e4a_96be_0101369b58d2);
impl windows_core::RuntimeType for IScheduledToastNotification4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScheduledToastNotification4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScheduledToastNotificationFactory, IScheduledToastNotificationFactory_Vtbl, 0xe7bed191_0bb9_4189_8394_31761b476fd7);
impl windows_core::RuntimeType for IScheduledToastNotificationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScheduledToastNotificationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateScheduledToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateScheduledToastNotification: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateScheduledToastNotificationRecurring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::TimeSpan, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateScheduledToastNotificationRecurring: usize,
}
windows_core::imp::define_interface!(IScheduledToastNotificationShowingEventArgs, IScheduledToastNotificationShowingEventArgs_Vtbl, 0x6173f6b4_412a_5e2c_a6ed_a0209aef9a09);
impl windows_core::RuntimeType for IScheduledToastNotificationShowingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IScheduledToastNotificationShowingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ScheduledToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShownTileNotification, IShownTileNotification_Vtbl, 0x342d8988_5af2_481a_a6a3_f2fdc78de88e);
impl windows_core::RuntimeType for IShownTileNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IShownTileNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITileFlyoutNotification, ITileFlyoutNotification_Vtbl, 0x9a53b261_c70c_42be_b2f3_f42aa97d34e5);
impl windows_core::RuntimeType for ITileFlyoutNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileFlyoutNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITileFlyoutNotificationFactory, ITileFlyoutNotificationFactory_Vtbl, 0xef556ff5_5226_4f2b_b278_88a35dfe569f);
impl windows_core::RuntimeType for ITileFlyoutNotificationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileFlyoutNotificationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateTileFlyoutNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateTileFlyoutNotification: usize,
}
windows_core::imp::define_interface!(ITileFlyoutUpdateManagerStatics, ITileFlyoutUpdateManagerStatics_Vtbl, 0x04363b0b_1ac0_4b99_88e7_ada83e953d48);
impl windows_core::RuntimeType for ITileFlyoutUpdateManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileFlyoutUpdateManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTileFlyoutUpdaterForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTileFlyoutUpdaterForApplicationWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTileFlyoutUpdaterForSecondaryTile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(*mut core::ffi::c_void, TileFlyoutTemplateType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
windows_core::imp::define_interface!(ITileFlyoutUpdater, ITileFlyoutUpdater_Vtbl, 0x8d40c76a_c465_4052_a740_5c2654c1a089);
impl windows_core::RuntimeType for ITileFlyoutUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileFlyoutUpdater_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPeriodicUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PeriodicUpdateRecurrence) -> windows_core::HRESULT,
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, PeriodicUpdateRecurrence) -> windows_core::HRESULT,
    pub StopPeriodicUpdate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NotificationSetting) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITileNotification, ITileNotification_Vtbl, 0xebaec8fa_50ec_4c18_b4d0_3af02e5540ab);
impl windows_core::RuntimeType for ITileNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITileNotificationFactory, ITileNotificationFactory_Vtbl, 0xc6abdd6e_4928_46c8_bdbf_81a047dea0d4);
impl windows_core::RuntimeType for ITileNotificationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileNotificationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateTileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateTileNotification: usize,
}
windows_core::imp::define_interface!(ITileUpdateManagerForUser, ITileUpdateManagerForUser_Vtbl, 0x55141348_2ee2_4e2d_9cc1_216a20decc9f);
impl windows_core::RuntimeType for ITileUpdateManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileUpdateManagerForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTileUpdaterForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTileUpdaterForApplicationWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTileUpdaterForSecondaryTile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(ITileUpdateManagerStatics, ITileUpdateManagerStatics_Vtbl, 0xda159e5d_3ea9_4986_8d84_b09d5e12276d);
impl windows_core::RuntimeType for ITileUpdateManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileUpdateManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTileUpdaterForApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTileUpdaterForApplicationWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTileUpdaterForSecondaryTile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(*mut core::ffi::c_void, TileTemplateType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
windows_core::imp::define_interface!(ITileUpdateManagerStatics2, ITileUpdateManagerStatics2_Vtbl, 0x731c1ddc_8e14_4b7c_a34b_9d22de76c84d);
impl windows_core::RuntimeType for ITileUpdateManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileUpdateManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(ITileUpdater, ITileUpdater_Vtbl, 0x0942a48b_1d91_44ec_9243_c1e821c29a20);
impl windows_core::RuntimeType for ITileUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileUpdater_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableNotificationQueue: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Setting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NotificationSetting) -> windows_core::HRESULT,
    pub AddToSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScheduledTileNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScheduledTileNotifications: usize,
    pub StartPeriodicUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PeriodicUpdateRecurrence) -> windows_core::HRESULT,
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, PeriodicUpdateRecurrence) -> windows_core::HRESULT,
    pub StopPeriodicUpdate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartPeriodicUpdateBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PeriodicUpdateRecurrence) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartPeriodicUpdateBatch: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartPeriodicUpdateBatchAtTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime, PeriodicUpdateRecurrence) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartPeriodicUpdateBatchAtTime: usize,
}
windows_core::imp::define_interface!(ITileUpdater2, ITileUpdater2_Vtbl, 0xa2266e12_15ee_43ed_83f5_65b352bb1a84);
impl windows_core::RuntimeType for ITileUpdater2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITileUpdater2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnableNotificationQueueForSquare150x150: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub EnableNotificationQueueForWide310x150: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub EnableNotificationQueueForSquare310x310: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastActivatedEventArgs, IToastActivatedEventArgs_Vtbl, 0xe3bf92f3_c197_436f_8265_0625824f8dac);
impl windows_core::RuntimeType for IToastActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastActivatedEventArgs2, IToastActivatedEventArgs2_Vtbl, 0xab7da512_cc61_568e_81be_304ac31038fa);
impl windows_core::RuntimeType for IToastActivatedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastActivatedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
windows_core::imp::define_interface!(IToastCollection, IToastCollection_Vtbl, 0x0a8bc3b0_e0be_4858_bc2a_89dfe0b32863);
impl windows_core::RuntimeType for IToastCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LaunchArgs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLaunchArgs: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Icon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastCollectionFactory, IToastCollectionFactory_Vtbl, 0x164dd3d7_73c4_44f7_b4ff_fb6d4bf1f4c6);
impl windows_core::RuntimeType for IToastCollectionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastCollectionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastCollectionManager, IToastCollectionManager_Vtbl, 0x2a1821fe_179d_49bc_b79d_a527920d3665);
impl windows_core::RuntimeType for IToastCollectionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastCollectionManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SaveToastCollectionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllToastCollectionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllToastCollectionsAsync: usize,
    pub GetToastCollectionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveToastCollectionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllToastCollectionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastDismissedEventArgs, IToastDismissedEventArgs_Vtbl, 0x3f89d935_d9cb_4538_a0f0_ffe7659938f8);
impl windows_core::RuntimeType for IToastDismissedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastDismissedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToastDismissalReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastFailedEventArgs, IToastFailedEventArgs_Vtbl, 0x35176862_cfd4_44f8_ad64_f500fd896c3b);
impl windows_core::RuntimeType for IToastFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotification, IToastNotification_Vtbl, 0x997e2675_059e_4e60_8b06_1760917c8b80);
impl windows_core::RuntimeType for IToastNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub SetExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dismissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Activated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Failed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFailed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotification2, IToastNotification2_Vtbl, 0x9dfb9fd1_143a_490e_90bf_b9fba7132de7);
impl windows_core::RuntimeType for IToastNotification2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotification2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetSuppressPopup: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotification3, IToastNotification3_Vtbl, 0x31e8aed8_8141_4f99_bc0a_c4ed21297d77);
impl windows_core::RuntimeType for IToastNotification3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotification3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotificationMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NotificationMirroring) -> windows_core::HRESULT,
    pub SetNotificationMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, NotificationMirroring) -> windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotification4, IToastNotification4_Vtbl, 0x15154935_28ea_4727_88e9_c58680e2d118);
impl windows_core::RuntimeType for IToastNotification4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotification4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToastNotificationPriority) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, ToastNotificationPriority) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotification6, IToastNotification6_Vtbl, 0x43ebfe53_89ae_5c1e_a279_3aecfe9b6f54);
impl windows_core::RuntimeType for IToastNotification6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotification6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExpiresOnReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetExpiresOnReboot: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationActionTriggerDetail, IToastNotificationActionTriggerDetail_Vtbl, 0x9445135a_38f3_42f6_96aa_7955b0f03da2);
impl windows_core::RuntimeType for IToastNotificationActionTriggerDetail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationActionTriggerDetail_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Argument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
windows_core::imp::define_interface!(IToastNotificationFactory, IToastNotificationFactory_Vtbl, 0x04124b20_82c6_4229_b109_fd9ed4662b53);
impl windows_core::RuntimeType for IToastNotificationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateToastNotification: usize,
}
windows_core::imp::define_interface!(IToastNotificationHistory, IToastNotificationHistory_Vtbl, 0x5caddc63_01d3_4c97_986f_0533483fee14);
impl windows_core::RuntimeType for IToastNotificationHistory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationHistory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoveGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoveGroupWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoveGroupedTagWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoveGroupedTag: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationHistory2, IToastNotificationHistory2_Vtbl, 0x3bc3d253_2f31_4092_9129_8ad5abf067da);
impl windows_core::RuntimeType for IToastNotificationHistory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationHistory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHistory: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHistoryWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHistoryWithId: usize,
}
windows_core::imp::define_interface!(IToastNotificationHistoryChangedTriggerDetail, IToastNotificationHistoryChangedTriggerDetail_Vtbl, 0xdb037ffa_0068_412c_9c83_267c37f65670);
impl windows_core::RuntimeType for IToastNotificationHistoryChangedTriggerDetail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationHistoryChangedTriggerDetail_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToastHistoryChangedType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationHistoryChangedTriggerDetail2, IToastNotificationHistoryChangedTriggerDetail2_Vtbl, 0x0b36e982_c871_49fb_babb_25bdbc4cc45b);
impl windows_core::RuntimeType for IToastNotificationHistoryChangedTriggerDetail2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationHistoryChangedTriggerDetail2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CollectionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationManagerForUser, IToastNotificationManagerForUser_Vtbl, 0x79ab57f6_43fe_487b_8a7f_99567200ae94);
impl windows_core::RuntimeType for IToastNotificationManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationManagerForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateToastNotifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub History: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IToastNotificationManagerForUser2, IToastNotificationManagerForUser2_Vtbl, 0x679c64b7_81ab_42c2_8819_c958767753f4);
impl windows_core::RuntimeType for IToastNotificationManagerForUser2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationManagerForUser2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetToastNotifierForToastCollectionIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHistoryForToastCollectionIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetToastCollectionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetToastCollectionManagerWithAppId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationManagerForUser3, IToastNotificationManagerForUser3_Vtbl, 0x3efcb176_6cc1_56dc_973b_251f7aacb1c5);
impl windows_core::RuntimeType for IToastNotificationManagerForUser3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationManagerForUser3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToastNotificationMode) -> windows_core::HRESULT,
    pub NotificationModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNotificationModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationManagerStatics, IToastNotificationManagerStatics_Vtbl, 0x50ac103f_d235_4598_bbef_98fe4d1a3ad4);
impl windows_core::RuntimeType for IToastNotificationManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateToastNotifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(*mut core::ffi::c_void, ToastTemplateType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
windows_core::imp::define_interface!(IToastNotificationManagerStatics2, IToastNotificationManagerStatics2_Vtbl, 0x7ab93c52_0e48_4750_ba9d_1a4113981847);
impl windows_core::RuntimeType for IToastNotificationManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub History: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationManagerStatics4, IToastNotificationManagerStatics4_Vtbl, 0x8f993fd3_e516_45fb_8130_398e93fa52c3);
impl windows_core::RuntimeType for IToastNotificationManagerStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationManagerStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub ConfigureNotificationMirroring: unsafe extern "system" fn(*mut core::ffi::c_void, NotificationMirroring) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotificationManagerStatics5, IToastNotificationManagerStatics5_Vtbl, 0xd6f5f569_d40d_407c_8989_88cab42cfd14);
impl windows_core::RuntimeType for IToastNotificationManagerStatics5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotificationManagerStatics5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotifier, IToastNotifier_Vtbl, 0x75927b93_03f3_41ec_91d3_6e5bac1b38e7);
impl windows_core::RuntimeType for IToastNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NotificationSetting) -> windows_core::HRESULT,
    pub AddToSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScheduledToastNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScheduledToastNotifications: usize,
}
windows_core::imp::define_interface!(IToastNotifier2, IToastNotifier2_Vtbl, 0x354389c6_7c01_4bd5_9c20_604340cd2b74);
impl windows_core::RuntimeType for IToastNotifier2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotifier2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateWithTagAndGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut NotificationUpdateResult) -> windows_core::HRESULT,
    pub UpdateWithTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut NotificationUpdateResult) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToastNotifier3, IToastNotifier3_Vtbl, 0xae75a04a_3b0c_51ad_b7e8_b08ab6052549);
impl windows_core::RuntimeType for IToastNotifier3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IToastNotifier3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScheduledToastNotificationShowing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveScheduledToastNotificationShowing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserNotification, IUserNotification_Vtbl, 0xadf7e52f_4e53_42d5_9c33_eb5ea515b23e);
impl windows_core::RuntimeType for IUserNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Notification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserNotificationChangedEventArgs, IUserNotificationChangedEventArgs_Vtbl, 0xb6bd6839_79cf_4b25_82c0_0ce1eef81f8c);
impl windows_core::RuntimeType for IUserNotificationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserNotificationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangeKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserNotificationChangedKind) -> windows_core::HRESULT,
    pub UserNotificationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AdaptiveNotificationText(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdaptiveNotificationText, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AdaptiveNotificationText, IAdaptiveNotificationContent);
impl AdaptiveNotificationText {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AdaptiveNotificationText, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Kind(&self) -> windows_core::Result<AdaptiveNotificationContentKind> {
        let this = &windows_core::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for AdaptiveNotificationText {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdaptiveNotificationText>();
}
unsafe impl windows_core::Interface for AdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_Vtbl;
    const IID: windows_core::GUID = <IAdaptiveNotificationText as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdaptiveNotificationText {
    const NAME: &'static str = "Windows.UI.Notifications.AdaptiveNotificationText";
}
unsafe impl Send for AdaptiveNotificationText {}
unsafe impl Sync for AdaptiveNotificationText {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BadgeNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BadgeNotification, windows_core::IUnknown, windows_core::IInspectable);
impl BadgeNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExpirationTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExpirationTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateBadgeNotification<P0>(content: P0) -> windows_core::Result<BadgeNotification>
    where
        P0: windows_core::Param<super::super::Data::Xml::Dom::XmlDocument>,
    {
        Self::IBadgeNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeNotification)(windows_core::Interface::as_raw(this), content.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBadgeNotificationFactory<R, F: FnOnce(&IBadgeNotificationFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BadgeNotification, IBadgeNotificationFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BadgeNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBadgeNotification>();
}
unsafe impl windows_core::Interface for BadgeNotification {
    type Vtable = IBadgeNotification_Vtbl;
    const IID: windows_core::GUID = <IBadgeNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BadgeNotification {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeNotification";
}
unsafe impl Send for BadgeNotification {}
unsafe impl Sync for BadgeNotification {}
pub struct BadgeUpdateManager;
impl BadgeUpdateManager {
    pub fn CreateBadgeUpdaterForApplication() -> windows_core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeUpdaterForApplication)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateBadgeUpdaterForApplicationWithId(applicationid: &windows_core::HSTRING) -> windows_core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeUpdaterForApplicationWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateBadgeUpdaterForSecondaryTile(tileid: &windows_core::HSTRING) -> windows_core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeUpdaterForSecondaryTile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: BadgeTemplateType) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateContent)(windows_core::Interface::as_raw(this), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<BadgeUpdateManagerForUser>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IBadgeUpdateManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBadgeUpdateManagerStatics<R, F: FnOnce(&IBadgeUpdateManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BadgeUpdateManager, IBadgeUpdateManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBadgeUpdateManagerStatics2<R, F: FnOnce(&IBadgeUpdateManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BadgeUpdateManager, IBadgeUpdateManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for BadgeUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BadgeUpdateManagerForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BadgeUpdateManagerForUser, windows_core::IUnknown, windows_core::IInspectable);
impl BadgeUpdateManagerForUser {
    pub fn CreateBadgeUpdaterForApplication(&self) -> windows_core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeUpdaterForApplication)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateBadgeUpdaterForApplicationWithId(&self, applicationid: &windows_core::HSTRING) -> windows_core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeUpdaterForApplicationWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateBadgeUpdaterForSecondaryTile(&self, tileid: &windows_core::HSTRING) -> windows_core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBadgeUpdaterForSecondaryTile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BadgeUpdateManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBadgeUpdateManagerForUser>();
}
unsafe impl windows_core::Interface for BadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_Vtbl;
    const IID: windows_core::GUID = <IBadgeUpdateManagerForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BadgeUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManagerForUser";
}
unsafe impl Send for BadgeUpdateManagerForUser {}
unsafe impl Sync for BadgeUpdateManagerForUser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct BadgeUpdater(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BadgeUpdater, windows_core::IUnknown, windows_core::IInspectable);
impl BadgeUpdater {
    pub fn Update<P0>(&self, notification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BadgeNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), notification.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartPeriodicUpdate<P0>(&self, badgecontent: P0, requestedinterval: PeriodicUpdateRecurrence) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartPeriodicUpdate)(windows_core::Interface::as_raw(this), badgecontent.param().abi(), requestedinterval).ok() }
    }
    pub fn StartPeriodicUpdateAtTime<P0>(&self, badgecontent: P0, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartPeriodicUpdateAtTime)(windows_core::Interface::as_raw(this), badgecontent.param().abi(), starttime, requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopPeriodicUpdate)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for BadgeUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBadgeUpdater>();
}
unsafe impl windows_core::Interface for BadgeUpdater {
    type Vtable = IBadgeUpdater_Vtbl;
    const IID: windows_core::GUID = <IBadgeUpdater as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BadgeUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdater";
}
unsafe impl Send for BadgeUpdater {}
unsafe impl Sync for BadgeUpdater {}
pub struct KnownAdaptiveNotificationHints;
impl KnownAdaptiveNotificationHints {
    pub fn Style() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Wrap() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Wrap)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MaxLines() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxLines)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MinLines() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinLines)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TextStacking() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextStacking)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Align() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Align)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownAdaptiveNotificationHintsStatics<R, F: FnOnce(&IKnownAdaptiveNotificationHintsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownAdaptiveNotificationHints, IKnownAdaptiveNotificationHintsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownAdaptiveNotificationHints {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationHints";
}
pub struct KnownAdaptiveNotificationTextStyles;
impl KnownAdaptiveNotificationTextStyles {
    pub fn Caption() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Caption)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Body() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Body)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Base() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Base)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Subtitle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subtitle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Title() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Subheader() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subheader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Header() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Header)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TitleNumeral() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TitleNumeral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SubheaderNumeral() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubheaderNumeral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeaderNumeral() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderNumeral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CaptionSubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptionSubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn BodySubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BodySubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn BaseSubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseSubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SubtitleSubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubtitleSubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TitleSubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TitleSubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SubheaderSubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubheaderSubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SubheaderNumeralSubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubheaderNumeralSubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeaderSubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderSubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeaderNumeralSubtle() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderNumeralSubtle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownAdaptiveNotificationTextStylesStatics<R, F: FnOnce(&IKnownAdaptiveNotificationTextStylesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownAdaptiveNotificationTextStyles, IKnownAdaptiveNotificationTextStylesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownAdaptiveNotificationTextStyles {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles";
}
pub struct KnownNotificationBindings;
impl KnownNotificationBindings {
    pub fn ToastGeneric() -> windows_core::Result<windows_core::HSTRING> {
        Self::IKnownNotificationBindingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToastGeneric)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IKnownNotificationBindingsStatics<R, F: FnOnce(&IKnownNotificationBindingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownNotificationBindings, IKnownNotificationBindingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownNotificationBindings {
    const NAME: &'static str = "Windows.UI.Notifications.KnownNotificationBindings";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Notification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Notification, windows_core::IUnknown, windows_core::IInspectable);
impl Notification {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Notification, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExpirationTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExpirationTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Visual(&self) -> windows_core::Result<NotificationVisual> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visual)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVisual<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<NotificationVisual>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVisual)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for Notification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INotification>();
}
unsafe impl windows_core::Interface for Notification {
    type Vtable = INotification_Vtbl;
    const IID: windows_core::GUID = <INotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Notification {
    const NAME: &'static str = "Windows.UI.Notifications.Notification";
}
unsafe impl Send for Notification {}
unsafe impl Sync for Notification {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NotificationBinding(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NotificationBinding, windows_core::IUnknown, windows_core::IInspectable);
impl NotificationBinding {
    pub fn Template(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Template)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTemplate(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTemplate)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextElements(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AdaptiveNotificationText>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTextElements)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NotificationBinding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INotificationBinding>();
}
unsafe impl windows_core::Interface for NotificationBinding {
    type Vtable = INotificationBinding_Vtbl;
    const IID: windows_core::GUID = <INotificationBinding as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NotificationBinding {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationBinding";
}
unsafe impl Send for NotificationBinding {}
unsafe impl Sync for NotificationBinding {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NotificationData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NotificationData, windows_core::IUnknown, windows_core::IInspectable);
impl NotificationData {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NotificationData, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Values(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Values)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SequenceNumber(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SequenceNumber)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSequenceNumber(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSequenceNumber)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValuesAndSequenceNumber<P0>(initialvalues: P0, sequencenumber: u32) -> windows_core::Result<NotificationData>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>,
    {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNotificationDataWithValuesAndSequenceNumber)(windows_core::Interface::as_raw(this), initialvalues.param().abi(), sequencenumber, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValues<P0>(initialvalues: P0) -> windows_core::Result<NotificationData>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>,
    {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNotificationDataWithValues)(windows_core::Interface::as_raw(this), initialvalues.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn INotificationDataFactory<R, F: FnOnce(&INotificationDataFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NotificationData, INotificationDataFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for NotificationData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INotificationData>();
}
unsafe impl windows_core::Interface for NotificationData {
    type Vtable = INotificationData_Vtbl;
    const IID: windows_core::GUID = <INotificationData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NotificationData {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationData";
}
unsafe impl Send for NotificationData {}
unsafe impl Sync for NotificationData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct NotificationVisual(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NotificationVisual, windows_core::IUnknown, windows_core::IInspectable);
impl NotificationVisual {
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bindings(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<NotificationBinding>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bindings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetBinding(&self, templatename: &windows_core::HSTRING) -> windows_core::Result<NotificationBinding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBinding)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(templatename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NotificationVisual {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INotificationVisual>();
}
unsafe impl windows_core::Interface for NotificationVisual {
    type Vtable = INotificationVisual_Vtbl;
    const IID: windows_core::GUID = <INotificationVisual as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NotificationVisual {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationVisual";
}
unsafe impl Send for NotificationVisual {}
unsafe impl Sync for NotificationVisual {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ScheduledTileNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScheduledTileNotification, windows_core::IUnknown, windows_core::IInspectable);
impl ScheduledTileNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeliveryTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeliveryTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExpirationTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExpirationTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateScheduledTileNotification<P0>(content: P0, deliverytime: super::super::Foundation::DateTime) -> windows_core::Result<ScheduledTileNotification>
    where
        P0: windows_core::Param<super::super::Data::Xml::Dom::XmlDocument>,
    {
        Self::IScheduledTileNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateScheduledTileNotification)(windows_core::Interface::as_raw(this), content.param().abi(), deliverytime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IScheduledTileNotificationFactory<R, F: FnOnce(&IScheduledTileNotificationFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScheduledTileNotification, IScheduledTileNotificationFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScheduledTileNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScheduledTileNotification>();
}
unsafe impl windows_core::Interface for ScheduledTileNotification {
    type Vtable = IScheduledTileNotification_Vtbl;
    const IID: windows_core::GUID = <IScheduledTileNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScheduledTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledTileNotification";
}
unsafe impl Send for ScheduledTileNotification {}
unsafe impl Sync for ScheduledTileNotification {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ScheduledToastNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScheduledToastNotification, windows_core::IUnknown, windows_core::IInspectable);
impl ScheduledToastNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeliveryTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeliveryTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SnoozeInterval(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SnoozeInterval)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaximumSnoozeCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaximumSnoozeCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetGroup(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSuppressPopup)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SuppressPopup(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuppressPopup)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NotificationMirroring(&self) -> windows_core::Result<NotificationMirroring> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotificationMirroring)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNotificationMirroring(&self, value: NotificationMirroring) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNotificationMirroring)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RemoteId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRemoteId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExpirationTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = &windows_core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExpirationTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateScheduledToastNotification<P0>(content: P0, deliverytime: super::super::Foundation::DateTime) -> windows_core::Result<ScheduledToastNotification>
    where
        P0: windows_core::Param<super::super::Data::Xml::Dom::XmlDocument>,
    {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateScheduledToastNotification)(windows_core::Interface::as_raw(this), content.param().abi(), deliverytime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateScheduledToastNotificationRecurring<P0>(content: P0, deliverytime: super::super::Foundation::DateTime, snoozeinterval: super::super::Foundation::TimeSpan, maximumsnoozecount: u32) -> windows_core::Result<ScheduledToastNotification>
    where
        P0: windows_core::Param<super::super::Data::Xml::Dom::XmlDocument>,
    {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateScheduledToastNotificationRecurring)(windows_core::Interface::as_raw(this), content.param().abi(), deliverytime, snoozeinterval, maximumsnoozecount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IScheduledToastNotificationFactory<R, F: FnOnce(&IScheduledToastNotificationFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScheduledToastNotification, IScheduledToastNotificationFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScheduledToastNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScheduledToastNotification>();
}
unsafe impl windows_core::Interface for ScheduledToastNotification {
    type Vtable = IScheduledToastNotification_Vtbl;
    const IID: windows_core::GUID = <IScheduledToastNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScheduledToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotification";
}
unsafe impl Send for ScheduledToastNotification {}
unsafe impl Sync for ScheduledToastNotification {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ScheduledToastNotificationShowingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScheduledToastNotificationShowingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ScheduledToastNotificationShowingEventArgs {
    pub fn Cancel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCancel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScheduledToastNotification(&self) -> windows_core::Result<ScheduledToastNotification> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScheduledToastNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for ScheduledToastNotificationShowingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScheduledToastNotificationShowingEventArgs>();
}
unsafe impl windows_core::Interface for ScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IScheduledToastNotificationShowingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScheduledToastNotificationShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs";
}
unsafe impl Send for ScheduledToastNotificationShowingEventArgs {}
unsafe impl Sync for ScheduledToastNotificationShowingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ShownTileNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ShownTileNotification, windows_core::IUnknown, windows_core::IInspectable);
impl ShownTileNotification {
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ShownTileNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IShownTileNotification>();
}
unsafe impl windows_core::Interface for ShownTileNotification {
    type Vtable = IShownTileNotification_Vtbl;
    const IID: windows_core::GUID = <IShownTileNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ShownTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ShownTileNotification";
}
unsafe impl Send for ShownTileNotification {}
unsafe impl Sync for ShownTileNotification {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TileFlyoutNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TileFlyoutNotification, windows_core::IUnknown, windows_core::IInspectable);
impl TileFlyoutNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExpirationTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExpirationTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileFlyoutNotification<P0>(content: P0) -> windows_core::Result<TileFlyoutNotification>
    where
        P0: windows_core::Param<super::super::Data::Xml::Dom::XmlDocument>,
    {
        Self::ITileFlyoutNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileFlyoutNotification)(windows_core::Interface::as_raw(this), content.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITileFlyoutNotificationFactory<R, F: FnOnce(&ITileFlyoutNotificationFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TileFlyoutNotification, ITileFlyoutNotificationFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TileFlyoutNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITileFlyoutNotification>();
}
unsafe impl windows_core::Interface for TileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_Vtbl;
    const IID: windows_core::GUID = <ITileFlyoutNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TileFlyoutNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutNotification";
}
unsafe impl Send for TileFlyoutNotification {}
unsafe impl Sync for TileFlyoutNotification {}
pub struct TileFlyoutUpdateManager;
impl TileFlyoutUpdateManager {
    pub fn CreateTileFlyoutUpdaterForApplication() -> windows_core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileFlyoutUpdaterForApplication)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTileFlyoutUpdaterForApplicationWithId(applicationid: &windows_core::HSTRING) -> windows_core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileFlyoutUpdaterForApplicationWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTileFlyoutUpdaterForSecondaryTile(tileid: &windows_core::HSTRING) -> windows_core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileFlyoutUpdaterForSecondaryTile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: TileFlyoutTemplateType) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateContent)(windows_core::Interface::as_raw(this), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITileFlyoutUpdateManagerStatics<R, F: FnOnce(&ITileFlyoutUpdateManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TileFlyoutUpdateManager, ITileFlyoutUpdateManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for TileFlyoutUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdateManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TileFlyoutUpdater(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TileFlyoutUpdater, windows_core::IUnknown, windows_core::IInspectable);
impl TileFlyoutUpdater {
    pub fn Update<P0>(&self, notification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<TileFlyoutNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), notification.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartPeriodicUpdate<P0>(&self, tileflyoutcontent: P0, requestedinterval: PeriodicUpdateRecurrence) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartPeriodicUpdate)(windows_core::Interface::as_raw(this), tileflyoutcontent.param().abi(), requestedinterval).ok() }
    }
    pub fn StartPeriodicUpdateAtTime<P0>(&self, tileflyoutcontent: P0, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartPeriodicUpdateAtTime)(windows_core::Interface::as_raw(this), tileflyoutcontent.param().abi(), starttime, requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopPeriodicUpdate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Setting(&self) -> windows_core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Setting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TileFlyoutUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITileFlyoutUpdater>();
}
unsafe impl windows_core::Interface for TileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_Vtbl;
    const IID: windows_core::GUID = <ITileFlyoutUpdater as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TileFlyoutUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdater";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TileNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TileNotification, windows_core::IUnknown, windows_core::IInspectable);
impl TileNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExpirationTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExpirationTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileNotification<P0>(content: P0) -> windows_core::Result<TileNotification>
    where
        P0: windows_core::Param<super::super::Data::Xml::Dom::XmlDocument>,
    {
        Self::ITileNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileNotification)(windows_core::Interface::as_raw(this), content.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITileNotificationFactory<R, F: FnOnce(&ITileNotificationFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TileNotification, ITileNotificationFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TileNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITileNotification>();
}
unsafe impl windows_core::Interface for TileNotification {
    type Vtable = ITileNotification_Vtbl;
    const IID: windows_core::GUID = <ITileNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileNotification";
}
unsafe impl Send for TileNotification {}
unsafe impl Sync for TileNotification {}
pub struct TileUpdateManager;
impl TileUpdateManager {
    pub fn CreateTileUpdaterForApplication() -> windows_core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileUpdaterForApplication)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTileUpdaterForApplicationWithId(applicationid: &windows_core::HSTRING) -> windows_core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileUpdaterForApplicationWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTileUpdaterForSecondaryTile(tileid: &windows_core::HSTRING) -> windows_core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileUpdaterForSecondaryTile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: TileTemplateType) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateContent)(windows_core::Interface::as_raw(this), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<TileUpdateManagerForUser>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::ITileUpdateManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITileUpdateManagerStatics<R, F: FnOnce(&ITileUpdateManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TileUpdateManager, ITileUpdateManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITileUpdateManagerStatics2<R, F: FnOnce(&ITileUpdateManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TileUpdateManager, ITileUpdateManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for TileUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TileUpdateManagerForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TileUpdateManagerForUser, windows_core::IUnknown, windows_core::IInspectable);
impl TileUpdateManagerForUser {
    pub fn CreateTileUpdaterForApplication(&self) -> windows_core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileUpdaterForApplication)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTileUpdaterForApplicationWithId(&self, applicationid: &windows_core::HSTRING) -> windows_core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileUpdaterForApplicationWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTileUpdaterForSecondaryTile(&self, tileid: &windows_core::HSTRING) -> windows_core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTileUpdaterForSecondaryTile)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TileUpdateManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITileUpdateManagerForUser>();
}
unsafe impl windows_core::Interface for TileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_Vtbl;
    const IID: windows_core::GUID = <ITileUpdateManagerForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TileUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManagerForUser";
}
unsafe impl Send for TileUpdateManagerForUser {}
unsafe impl Sync for TileUpdateManagerForUser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TileUpdater(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TileUpdater, windows_core::IUnknown, windows_core::IInspectable);
impl TileUpdater {
    pub fn Update<P0>(&self, notification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<TileNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), notification.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EnableNotificationQueue(&self, enable: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).EnableNotificationQueue)(windows_core::Interface::as_raw(this), enable).ok() }
    }
    pub fn Setting(&self) -> windows_core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Setting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddToSchedule<P0>(&self, scheduledtile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ScheduledTileNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddToSchedule)(windows_core::Interface::as_raw(this), scheduledtile.param().abi()).ok() }
    }
    pub fn RemoveFromSchedule<P0>(&self, scheduledtile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ScheduledTileNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFromSchedule)(windows_core::Interface::as_raw(this), scheduledtile.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledTileNotifications(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ScheduledTileNotification>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetScheduledTileNotifications)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPeriodicUpdate<P0>(&self, tilecontent: P0, requestedinterval: PeriodicUpdateRecurrence) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartPeriodicUpdate)(windows_core::Interface::as_raw(this), tilecontent.param().abi(), requestedinterval).ok() }
    }
    pub fn StartPeriodicUpdateAtTime<P0>(&self, tilecontent: P0, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartPeriodicUpdateAtTime)(windows_core::Interface::as_raw(this), tilecontent.param().abi(), starttime, requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopPeriodicUpdate)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartPeriodicUpdateBatch<P0>(&self, tilecontents: P0, requestedinterval: PeriodicUpdateRecurrence) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartPeriodicUpdateBatch)(windows_core::Interface::as_raw(this), tilecontents.param().abi(), requestedinterval).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartPeriodicUpdateBatchAtTime<P0>(&self, tilecontents: P0, starttime: super::super::Foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartPeriodicUpdateBatchAtTime)(windows_core::Interface::as_raw(this), tilecontents.param().abi(), starttime, requestedinterval).ok() }
    }
    pub fn EnableNotificationQueueForSquare150x150(&self, enable: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableNotificationQueueForSquare150x150)(windows_core::Interface::as_raw(this), enable).ok() }
    }
    pub fn EnableNotificationQueueForWide310x150(&self, enable: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableNotificationQueueForWide310x150)(windows_core::Interface::as_raw(this), enable).ok() }
    }
    pub fn EnableNotificationQueueForSquare310x310(&self, enable: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableNotificationQueueForSquare310x310)(windows_core::Interface::as_raw(this), enable).ok() }
    }
}
impl windows_core::RuntimeType for TileUpdater {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITileUpdater>();
}
unsafe impl windows_core::Interface for TileUpdater {
    type Vtable = ITileUpdater_Vtbl;
    const IID: windows_core::GUID = <ITileUpdater as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TileUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdater";
}
unsafe impl Send for TileUpdater {}
unsafe impl Sync for TileUpdater {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ToastActivatedEventArgs {
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &windows_core::Interface::cast::<IToastActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ToastActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastActivatedEventArgs>();
}
unsafe impl windows_core::Interface for ToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IToastActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastActivatedEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastCollection, windows_core::IUnknown, windows_core::IInspectable);
impl ToastCollection {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn LaunchArgs(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchArgs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLaunchArgs(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLaunchArgs)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Icon(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Icon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIcon)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CreateInstance<P0>(collectionid: &windows_core::HSTRING, displayname: &windows_core::HSTRING, launchargs: &windows_core::HSTRING, iconuri: P0) -> windows_core::Result<ToastCollection>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IToastCollectionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(collectionid), core::mem::transmute_copy(displayname), core::mem::transmute_copy(launchargs), iconuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IToastCollectionFactory<R, F: FnOnce(&IToastCollectionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastCollection, IToastCollectionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToastCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastCollection>();
}
unsafe impl windows_core::Interface for ToastCollection {
    type Vtable = IToastCollection_Vtbl;
    const IID: windows_core::GUID = <IToastCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastCollection {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollection";
}
unsafe impl Send for ToastCollection {}
unsafe impl Sync for ToastCollection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastCollectionManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastCollectionManager, windows_core::IUnknown, windows_core::IInspectable);
impl ToastCollectionManager {
    pub fn SaveToastCollectionAsync<P0>(&self, collection: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<ToastCollection>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveToastCollectionAsync)(windows_core::Interface::as_raw(this), collection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllToastCollectionsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ToastCollection>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllToastCollectionsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetToastCollectionAsync(&self, collectionid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ToastCollection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetToastCollectionAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(collectionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveToastCollectionAsync(&self, collectionid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveToastCollectionAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(collectionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveAllToastCollectionsAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveAllToastCollectionsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ToastCollectionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastCollectionManager>();
}
unsafe impl windows_core::Interface for ToastCollectionManager {
    type Vtable = IToastCollectionManager_Vtbl;
    const IID: windows_core::GUID = <IToastCollectionManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastCollectionManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollectionManager";
}
unsafe impl Send for ToastCollectionManager {}
unsafe impl Sync for ToastCollectionManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastDismissedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastDismissedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ToastDismissedEventArgs {
    pub fn Reason(&self) -> windows_core::Result<ToastDismissalReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ToastDismissedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastDismissedEventArgs>();
}
unsafe impl windows_core::Interface for ToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IToastDismissedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastDismissedEventArgs";
}
unsafe impl Send for ToastDismissedEventArgs {}
unsafe impl Sync for ToastDismissedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ToastFailedEventArgs {
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ToastFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastFailedEventArgs>();
}
unsafe impl windows_core::Interface for ToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IToastFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastFailedEventArgs";
}
unsafe impl Send for ToastFailedEventArgs {}
unsafe impl Sync for ToastFailedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotification, windows_core::IUnknown, windows_core::IInspectable);
impl ToastNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExpirationTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExpirationTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Dismissed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dismissed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDismissed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDismissed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Activated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ToastNotification, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Activated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActivated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Failed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Failed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFailed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetGroup(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Group(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSuppressPopup)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SuppressPopup(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuppressPopup)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NotificationMirroring(&self) -> windows_core::Result<NotificationMirroring> {
        let this = &windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotificationMirroring)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNotificationMirroring(&self, value: NotificationMirroring) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNotificationMirroring)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RemoteId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRemoteId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Data(&self) -> windows_core::Result<NotificationData> {
        let this = &windows_core::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<NotificationData>,
    {
        let this = &windows_core::Interface::cast::<IToastNotification4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Priority(&self) -> windows_core::Result<ToastNotificationPriority> {
        let this = &windows_core::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Priority)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPriority(&self, value: ToastNotificationPriority) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPriority)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExpiresOnReboot(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IToastNotification6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpiresOnReboot)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExpiresOnReboot(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotification6>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExpiresOnReboot)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateToastNotification<P0>(content: P0) -> windows_core::Result<ToastNotification>
    where
        P0: windows_core::Param<super::super::Data::Xml::Dom::XmlDocument>,
    {
        Self::IToastNotificationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotification)(windows_core::Interface::as_raw(this), content.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationFactory<R, F: FnOnce(&IToastNotificationFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotification, IToastNotificationFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ToastNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastNotification>();
}
unsafe impl windows_core::Interface for ToastNotification {
    type Vtable = IToastNotification_Vtbl;
    const IID: windows_core::GUID = <IToastNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotification";
}
unsafe impl Send for ToastNotification {}
unsafe impl Sync for ToastNotification {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastNotificationActionTriggerDetail(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotificationActionTriggerDetail, windows_core::IUnknown, windows_core::IInspectable);
impl ToastNotificationActionTriggerDetail {
    pub fn Argument(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Argument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserInput)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ToastNotificationActionTriggerDetail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastNotificationActionTriggerDetail>();
}
unsafe impl windows_core::Interface for ToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_Vtbl;
    const IID: windows_core::GUID = <IToastNotificationActionTriggerDetail as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotificationActionTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationActionTriggerDetail";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastNotificationHistory(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotificationHistory, windows_core::IUnknown, windows_core::IInspectable);
impl ToastNotificationHistory {
    pub fn RemoveGroup(&self, group: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGroup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(group)).ok() }
    }
    pub fn RemoveGroupWithId(&self, group: &windows_core::HSTRING, applicationid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGroupWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(group), core::mem::transmute_copy(applicationid)).ok() }
    }
    pub fn RemoveGroupedTagWithId(&self, tag: &windows_core::HSTRING, group: &windows_core::HSTRING, applicationid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGroupedTagWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tag), core::mem::transmute_copy(group), core::mem::transmute_copy(applicationid)).ok() }
    }
    pub fn RemoveGroupedTag(&self, tag: &windows_core::HSTRING, group: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveGroupedTag)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tag), core::mem::transmute_copy(group)).ok() }
    }
    pub fn Remove(&self, tag: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tag)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ClearWithId(&self, applicationid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistory(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>> {
        let this = &windows_core::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHistory)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistoryWithId(&self, applicationid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ToastNotification>> {
        let this = &windows_core::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHistoryWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ToastNotificationHistory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastNotificationHistory>();
}
unsafe impl windows_core::Interface for ToastNotificationHistory {
    type Vtable = IToastNotificationHistory_Vtbl;
    const IID: windows_core::GUID = <IToastNotificationHistory as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotificationHistory {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistory";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastNotificationHistoryChangedTriggerDetail(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotificationHistoryChangedTriggerDetail, windows_core::IUnknown, windows_core::IInspectable);
impl ToastNotificationHistoryChangedTriggerDetail {
    pub fn ChangeType(&self) -> windows_core::Result<ToastHistoryChangedType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CollectionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IToastNotificationHistoryChangedTriggerDetail2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CollectionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ToastNotificationHistoryChangedTriggerDetail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastNotificationHistoryChangedTriggerDetail>();
}
unsafe impl windows_core::Interface for ToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_Vtbl;
    const IID: windows_core::GUID = <IToastNotificationHistoryChangedTriggerDetail as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotificationHistoryChangedTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail";
}
pub struct ToastNotificationManager;
impl ToastNotificationManager {
    pub fn CreateToastNotifier() -> windows_core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateToastNotifierWithId(applicationid: &windows_core::HSTRING) -> windows_core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifierWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: ToastTemplateType) -> windows_core::Result<super::super::Data::Xml::Dom::XmlDocument> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateContent)(windows_core::Interface::as_raw(this), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn History() -> windows_core::Result<ToastNotificationHistory> {
        Self::IToastNotificationManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).History)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<ToastNotificationManagerForUser>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IToastNotificationManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ConfigureNotificationMirroring(value: NotificationMirroring) -> windows_core::Result<()> {
        Self::IToastNotificationManagerStatics4(|this| unsafe { (windows_core::Interface::vtable(this).ConfigureNotificationMirroring)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn GetDefault() -> windows_core::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics5(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationManagerStatics<R, F: FnOnce(&IToastNotificationManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IToastNotificationManagerStatics2<R, F: FnOnce(&IToastNotificationManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IToastNotificationManagerStatics4<R, F: FnOnce(&IToastNotificationManagerStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IToastNotificationManagerStatics5<R, F: FnOnce(&IToastNotificationManagerStatics5) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics5> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ToastNotificationManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastNotificationManagerForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotificationManagerForUser, windows_core::IUnknown, windows_core::IInspectable);
impl ToastNotificationManagerForUser {
    pub fn CreateToastNotifier(&self) -> windows_core::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifier)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateToastNotifierWithId(&self, applicationid: &windows_core::HSTRING) -> windows_core::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateToastNotifierWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn History(&self) -> windows_core::Result<ToastNotificationHistory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).History)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetToastNotifierForToastCollectionIdAsync(&self, collectionid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ToastNotifier>> {
        let this = &windows_core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetToastNotifierForToastCollectionIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(collectionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetHistoryForToastCollectionIdAsync(&self, collectionid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ToastNotificationHistory>> {
        let this = &windows_core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHistoryForToastCollectionIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(collectionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetToastCollectionManager(&self) -> windows_core::Result<ToastCollectionManager> {
        let this = &windows_core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetToastCollectionManager)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetToastCollectionManagerWithAppId(&self, appid: &windows_core::HSTRING) -> windows_core::Result<ToastCollectionManager> {
        let this = &windows_core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetToastCollectionManagerWithAppId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NotificationMode(&self) -> windows_core::Result<ToastNotificationMode> {
        let this = &windows_core::Interface::cast::<IToastNotificationManagerForUser3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotificationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NotificationModeChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ToastNotificationManagerForUser, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IToastNotificationManagerForUser3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotificationModeChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveNotificationModeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotificationManagerForUser3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveNotificationModeChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ToastNotificationManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastNotificationManagerForUser>();
}
unsafe impl windows_core::Interface for ToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_Vtbl;
    const IID: windows_core::GUID = <IToastNotificationManagerForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotificationManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManagerForUser";
}
unsafe impl Send for ToastNotificationManagerForUser {}
unsafe impl Sync for ToastNotificationManagerForUser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ToastNotifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ToastNotifier, windows_core::IUnknown, windows_core::IInspectable);
impl ToastNotifier {
    pub fn Show<P0>(&self, notification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ToastNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Show)(windows_core::Interface::as_raw(this), notification.param().abi()).ok() }
    }
    pub fn Hide<P0>(&self, notification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ToastNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Hide)(windows_core::Interface::as_raw(this), notification.param().abi()).ok() }
    }
    pub fn Setting(&self) -> windows_core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Setting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AddToSchedule<P0>(&self, scheduledtoast: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ScheduledToastNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddToSchedule)(windows_core::Interface::as_raw(this), scheduledtoast.param().abi()).ok() }
    }
    pub fn RemoveFromSchedule<P0>(&self, scheduledtoast: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ScheduledToastNotification>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFromSchedule)(windows_core::Interface::as_raw(this), scheduledtoast.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledToastNotifications(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ScheduledToastNotification>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetScheduledToastNotifications)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateWithTagAndGroup<P0>(&self, data: P0, tag: &windows_core::HSTRING, group: &windows_core::HSTRING) -> windows_core::Result<NotificationUpdateResult>
    where
        P0: windows_core::Param<NotificationData>,
    {
        let this = &windows_core::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateWithTagAndGroup)(windows_core::Interface::as_raw(this), data.param().abi(), core::mem::transmute_copy(tag), core::mem::transmute_copy(group), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateWithTag<P0>(&self, data: P0, tag: &windows_core::HSTRING) -> windows_core::Result<NotificationUpdateResult>
    where
        P0: windows_core::Param<NotificationData>,
    {
        let this = &windows_core::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateWithTag)(windows_core::Interface::as_raw(this), data.param().abi(), core::mem::transmute_copy(tag), &mut result__).map(|| result__)
        }
    }
    pub fn ScheduledToastNotificationShowing<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IToastNotifier3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScheduledToastNotificationShowing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScheduledToastNotificationShowing(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToastNotifier3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveScheduledToastNotificationShowing)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ToastNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToastNotifier>();
}
unsafe impl windows_core::Interface for ToastNotifier {
    type Vtable = IToastNotifier_Vtbl;
    const IID: windows_core::GUID = <IToastNotifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ToastNotifier {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotifier";
}
unsafe impl Send for ToastNotifier {}
unsafe impl Sync for ToastNotifier {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UserNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserNotification, windows_core::IUnknown, windows_core::IInspectable);
impl UserNotification {
    pub fn Notification(&self) -> windows_core::Result<Notification> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Notification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> windows_core::Result<super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreationTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreationTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for UserNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserNotification>();
}
unsafe impl windows_core::Interface for UserNotification {
    type Vtable = IUserNotification_Vtbl;
    const IID: windows_core::GUID = <IUserNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserNotification {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotification";
}
unsafe impl Send for UserNotification {}
unsafe impl Sync for UserNotification {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UserNotificationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserNotificationChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UserNotificationChangedEventArgs {
    pub fn ChangeKind(&self) -> windows_core::Result<UserNotificationChangedKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChangeKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UserNotificationId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserNotificationId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for UserNotificationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserNotificationChangedEventArgs>();
}
unsafe impl windows_core::Interface for UserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUserNotificationChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserNotificationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotificationChangedEventArgs";
}
unsafe impl Send for UserNotificationChangedEventArgs {}
unsafe impl Sync for UserNotificationChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: Self = Self(0i32);
}
impl windows_core::TypeKind for AdaptiveNotificationContentKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdaptiveNotificationContentKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdaptiveNotificationContentKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AdaptiveNotificationContentKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.AdaptiveNotificationContentKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: Self = Self(0i32);
    pub const BadgeNumber: Self = Self(1i32);
}
impl windows_core::TypeKind for BadgeTemplateType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BadgeTemplateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BadgeTemplateType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for BadgeTemplateType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.BadgeTemplateType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: Self = Self(0u32);
    pub const Toast: Self = Self(1u32);
}
impl windows_core::TypeKind for NotificationKinds {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NotificationKinds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NotificationKinds").field(&self.0).finish()
    }
}
impl NotificationKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NotificationKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NotificationKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NotificationKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NotificationKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NotificationKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for NotificationKinds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationKinds;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl windows_core::TypeKind for NotificationMirroring {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NotificationMirroring {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NotificationMirroring").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NotificationMirroring {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationMirroring;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledForApplication: Self = Self(1i32);
    pub const DisabledForUser: Self = Self(2i32);
    pub const DisabledByGroupPolicy: Self = Self(3i32);
    pub const DisabledByManifest: Self = Self(4i32);
}
impl windows_core::TypeKind for NotificationSetting {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NotificationSetting {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NotificationSetting").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NotificationSetting {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationSetting;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const NotificationNotFound: Self = Self(2i32);
}
impl windows_core::TypeKind for NotificationUpdateResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NotificationUpdateResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NotificationUpdateResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NotificationUpdateResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationUpdateResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const SixHours: Self = Self(2i32);
    pub const TwelveHours: Self = Self(3i32);
    pub const Daily: Self = Self(4i32);
}
impl windows_core::TypeKind for PeriodicUpdateRecurrence {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PeriodicUpdateRecurrence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PeriodicUpdateRecurrence").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PeriodicUpdateRecurrence {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.PeriodicUpdateRecurrence;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: Self = Self(0i32);
}
impl windows_core::TypeKind for TileFlyoutTemplateType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TileFlyoutTemplateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TileFlyoutTemplateType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TileFlyoutTemplateType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.TileFlyoutTemplateType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TileTemplateType(pub i32);
impl TileTemplateType {
    pub const TileSquareImage: Self = Self(0i32);
    pub const TileSquareBlock: Self = Self(1i32);
    pub const TileSquareText01: Self = Self(2i32);
    pub const TileSquareText02: Self = Self(3i32);
    pub const TileSquareText03: Self = Self(4i32);
    pub const TileSquareText04: Self = Self(5i32);
    pub const TileSquarePeekImageAndText01: Self = Self(6i32);
    pub const TileSquarePeekImageAndText02: Self = Self(7i32);
    pub const TileSquarePeekImageAndText03: Self = Self(8i32);
    pub const TileSquarePeekImageAndText04: Self = Self(9i32);
    pub const TileWideImage: Self = Self(10i32);
    pub const TileWideImageCollection: Self = Self(11i32);
    pub const TileWideImageAndText01: Self = Self(12i32);
    pub const TileWideImageAndText02: Self = Self(13i32);
    pub const TileWideBlockAndText01: Self = Self(14i32);
    pub const TileWideBlockAndText02: Self = Self(15i32);
    pub const TileWidePeekImageCollection01: Self = Self(16i32);
    pub const TileWidePeekImageCollection02: Self = Self(17i32);
    pub const TileWidePeekImageCollection03: Self = Self(18i32);
    pub const TileWidePeekImageCollection04: Self = Self(19i32);
    pub const TileWidePeekImageCollection05: Self = Self(20i32);
    pub const TileWidePeekImageCollection06: Self = Self(21i32);
    pub const TileWidePeekImageAndText01: Self = Self(22i32);
    pub const TileWidePeekImageAndText02: Self = Self(23i32);
    pub const TileWidePeekImage01: Self = Self(24i32);
    pub const TileWidePeekImage02: Self = Self(25i32);
    pub const TileWidePeekImage03: Self = Self(26i32);
    pub const TileWidePeekImage04: Self = Self(27i32);
    pub const TileWidePeekImage05: Self = Self(28i32);
    pub const TileWidePeekImage06: Self = Self(29i32);
    pub const TileWideSmallImageAndText01: Self = Self(30i32);
    pub const TileWideSmallImageAndText02: Self = Self(31i32);
    pub const TileWideSmallImageAndText03: Self = Self(32i32);
    pub const TileWideSmallImageAndText04: Self = Self(33i32);
    pub const TileWideSmallImageAndText05: Self = Self(34i32);
    pub const TileWideText01: Self = Self(35i32);
    pub const TileWideText02: Self = Self(36i32);
    pub const TileWideText03: Self = Self(37i32);
    pub const TileWideText04: Self = Self(38i32);
    pub const TileWideText05: Self = Self(39i32);
    pub const TileWideText06: Self = Self(40i32);
    pub const TileWideText07: Self = Self(41i32);
    pub const TileWideText08: Self = Self(42i32);
    pub const TileWideText09: Self = Self(43i32);
    pub const TileWideText10: Self = Self(44i32);
    pub const TileWideText11: Self = Self(45i32);
    pub const TileSquare150x150Image: Self = Self(0i32);
    pub const TileSquare150x150Block: Self = Self(1i32);
    pub const TileSquare150x150Text01: Self = Self(2i32);
    pub const TileSquare150x150Text02: Self = Self(3i32);
    pub const TileSquare150x150Text03: Self = Self(4i32);
    pub const TileSquare150x150Text04: Self = Self(5i32);
    pub const TileSquare150x150PeekImageAndText01: Self = Self(6i32);
    pub const TileSquare150x150PeekImageAndText02: Self = Self(7i32);
    pub const TileSquare150x150PeekImageAndText03: Self = Self(8i32);
    pub const TileSquare150x150PeekImageAndText04: Self = Self(9i32);
    pub const TileWide310x150Image: Self = Self(10i32);
    pub const TileWide310x150ImageCollection: Self = Self(11i32);
    pub const TileWide310x150ImageAndText01: Self = Self(12i32);
    pub const TileWide310x150ImageAndText02: Self = Self(13i32);
    pub const TileWide310x150BlockAndText01: Self = Self(14i32);
    pub const TileWide310x150BlockAndText02: Self = Self(15i32);
    pub const TileWide310x150PeekImageCollection01: Self = Self(16i32);
    pub const TileWide310x150PeekImageCollection02: Self = Self(17i32);
    pub const TileWide310x150PeekImageCollection03: Self = Self(18i32);
    pub const TileWide310x150PeekImageCollection04: Self = Self(19i32);
    pub const TileWide310x150PeekImageCollection05: Self = Self(20i32);
    pub const TileWide310x150PeekImageCollection06: Self = Self(21i32);
    pub const TileWide310x150PeekImageAndText01: Self = Self(22i32);
    pub const TileWide310x150PeekImageAndText02: Self = Self(23i32);
    pub const TileWide310x150PeekImage01: Self = Self(24i32);
    pub const TileWide310x150PeekImage02: Self = Self(25i32);
    pub const TileWide310x150PeekImage03: Self = Self(26i32);
    pub const TileWide310x150PeekImage04: Self = Self(27i32);
    pub const TileWide310x150PeekImage05: Self = Self(28i32);
    pub const TileWide310x150PeekImage06: Self = Self(29i32);
    pub const TileWide310x150SmallImageAndText01: Self = Self(30i32);
    pub const TileWide310x150SmallImageAndText02: Self = Self(31i32);
    pub const TileWide310x150SmallImageAndText03: Self = Self(32i32);
    pub const TileWide310x150SmallImageAndText04: Self = Self(33i32);
    pub const TileWide310x150SmallImageAndText05: Self = Self(34i32);
    pub const TileWide310x150Text01: Self = Self(35i32);
    pub const TileWide310x150Text02: Self = Self(36i32);
    pub const TileWide310x150Text03: Self = Self(37i32);
    pub const TileWide310x150Text04: Self = Self(38i32);
    pub const TileWide310x150Text05: Self = Self(39i32);
    pub const TileWide310x150Text06: Self = Self(40i32);
    pub const TileWide310x150Text07: Self = Self(41i32);
    pub const TileWide310x150Text08: Self = Self(42i32);
    pub const TileWide310x150Text09: Self = Self(43i32);
    pub const TileWide310x150Text10: Self = Self(44i32);
    pub const TileWide310x150Text11: Self = Self(45i32);
    pub const TileSquare310x310BlockAndText01: Self = Self(46i32);
    pub const TileSquare310x310BlockAndText02: Self = Self(47i32);
    pub const TileSquare310x310Image: Self = Self(48i32);
    pub const TileSquare310x310ImageAndText01: Self = Self(49i32);
    pub const TileSquare310x310ImageAndText02: Self = Self(50i32);
    pub const TileSquare310x310ImageAndTextOverlay01: Self = Self(51i32);
    pub const TileSquare310x310ImageAndTextOverlay02: Self = Self(52i32);
    pub const TileSquare310x310ImageAndTextOverlay03: Self = Self(53i32);
    pub const TileSquare310x310ImageCollectionAndText01: Self = Self(54i32);
    pub const TileSquare310x310ImageCollectionAndText02: Self = Self(55i32);
    pub const TileSquare310x310ImageCollection: Self = Self(56i32);
    pub const TileSquare310x310SmallImagesAndTextList01: Self = Self(57i32);
    pub const TileSquare310x310SmallImagesAndTextList02: Self = Self(58i32);
    pub const TileSquare310x310SmallImagesAndTextList03: Self = Self(59i32);
    pub const TileSquare310x310SmallImagesAndTextList04: Self = Self(60i32);
    pub const TileSquare310x310Text01: Self = Self(61i32);
    pub const TileSquare310x310Text02: Self = Self(62i32);
    pub const TileSquare310x310Text03: Self = Self(63i32);
    pub const TileSquare310x310Text04: Self = Self(64i32);
    pub const TileSquare310x310Text05: Self = Self(65i32);
    pub const TileSquare310x310Text06: Self = Self(66i32);
    pub const TileSquare310x310Text07: Self = Self(67i32);
    pub const TileSquare310x310Text08: Self = Self(68i32);
    pub const TileSquare310x310TextList01: Self = Self(69i32);
    pub const TileSquare310x310TextList02: Self = Self(70i32);
    pub const TileSquare310x310TextList03: Self = Self(71i32);
    pub const TileSquare310x310SmallImageAndText01: Self = Self(72i32);
    pub const TileSquare310x310SmallImagesAndTextList05: Self = Self(73i32);
    pub const TileSquare310x310Text09: Self = Self(74i32);
    pub const TileSquare71x71IconWithBadge: Self = Self(75i32);
    pub const TileSquare150x150IconWithBadge: Self = Self(76i32);
    pub const TileWide310x150IconWithBadgeAndText: Self = Self(77i32);
    pub const TileSquare71x71Image: Self = Self(78i32);
    pub const TileTall150x310Image: Self = Self(79i32);
}
impl windows_core::TypeKind for TileTemplateType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TileTemplateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TileTemplateType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TileTemplateType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.TileTemplateType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: Self = Self(0i32);
    pub const ApplicationHidden: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
}
impl windows_core::TypeKind for ToastDismissalReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ToastDismissalReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ToastDismissalReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ToastDismissalReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastDismissalReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const Expired: Self = Self(2i32);
    pub const Added: Self = Self(3i32);
}
impl windows_core::TypeKind for ToastHistoryChangedType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ToastHistoryChangedType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ToastHistoryChangedType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ToastHistoryChangedType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastHistoryChangedType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ToastNotificationMode(pub i32);
impl ToastNotificationMode {
    pub const Unrestricted: Self = Self(0i32);
    pub const PriorityOnly: Self = Self(1i32);
    pub const AlarmsOnly: Self = Self(2i32);
}
impl windows_core::TypeKind for ToastNotificationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ToastNotificationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ToastNotificationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ToastNotificationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastNotificationMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl windows_core::TypeKind for ToastNotificationPriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ToastNotificationPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ToastNotificationPriority").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ToastNotificationPriority {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastNotificationPriority;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ToastTemplateType(pub i32);
impl ToastTemplateType {
    pub const ToastImageAndText01: Self = Self(0i32);
    pub const ToastImageAndText02: Self = Self(1i32);
    pub const ToastImageAndText03: Self = Self(2i32);
    pub const ToastImageAndText04: Self = Self(3i32);
    pub const ToastText01: Self = Self(4i32);
    pub const ToastText02: Self = Self(5i32);
    pub const ToastText03: Self = Self(6i32);
    pub const ToastText04: Self = Self(7i32);
}
impl windows_core::TypeKind for ToastTemplateType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ToastTemplateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ToastTemplateType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ToastTemplateType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastTemplateType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
}
impl windows_core::TypeKind for UserNotificationChangedKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserNotificationChangedKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserNotificationChangedKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserNotificationChangedKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.UserNotificationChangedKind;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
