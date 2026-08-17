windows_core::imp::define_interface!(IAdaptiveCard, IAdaptiveCard_Vtbl, 0x72d0568c_a274_41cd_82a8_989d40b9b05e);
impl core::ops::Deref for IAdaptiveCard {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAdaptiveCard, windows_core::IUnknown, windows_core::IInspectable);
impl IAdaptiveCard {
    pub fn ToJson(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToJson)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IAdaptiveCard {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveCard_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToJson: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdaptiveCardBuilderStatics, IAdaptiveCardBuilderStatics_Vtbl, 0x766d8f08_d3fe_4347_a0bc_b9ea9a6dc28e);
impl core::ops::Deref for IAdaptiveCardBuilderStatics {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAdaptiveCardBuilderStatics, windows_core::IUnknown, windows_core::IInspectable);
impl IAdaptiveCardBuilderStatics {
    pub fn CreateAdaptiveCardFromJson(&self, value: &windows_core::HSTRING) -> windows_core::Result<IAdaptiveCard> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAdaptiveCardFromJson)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IAdaptiveCardBuilderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdaptiveCardBuilderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAdaptiveCardFromJson: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusSession, IFocusSession_Vtbl, 0x069fbab8_0e84_5f2f_8614_9b6544326277);
impl windows_core::RuntimeType for IFocusSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFocusSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusSessionManager, IFocusSessionManager_Vtbl, 0xe7ffbaa9_d8be_5dbf_bac6_49364842e37e);
impl windows_core::RuntimeType for IFocusSessionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFocusSessionManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsFocusActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetSession: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryStartFocusSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryStartFocusSession2: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeactivateFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFocusActiveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsFocusActiveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusSessionManagerStatics, IFocusSessionManagerStatics_Vtbl, 0x834df764_cb9a_5d0a_aa9f_73df4f249395);
impl windows_core::RuntimeType for IFocusSessionManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFocusSessionManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISecurityAppManager, ISecurityAppManager_Vtbl, 0x96ac500c_aed4_561d_bde8_953520343a2d);
impl windows_core::RuntimeType for ISecurityAppManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISecurityAppManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, SecurityAppKind, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, bool, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, SecurityAppKind, windows_core::GUID) -> windows_core::HRESULT,
    pub UpdateState: unsafe extern "system" fn(*mut core::ffi::c_void, SecurityAppKind, windows_core::GUID, SecurityAppState, SecurityAppSubstatus, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShareWindowCommandEventArgs, IShareWindowCommandEventArgs_Vtbl, 0x4578dc09_a523_5756_a995_e4feb991fff0);
impl windows_core::RuntimeType for IShareWindowCommandEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IShareWindowCommandEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowId) -> windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ShareWindowCommand) -> windows_core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(*mut core::ffi::c_void, ShareWindowCommand) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShareWindowCommandSource, IShareWindowCommandSource_Vtbl, 0xcb3b7ae3_6b9c_561e_bccc_61e68e0abfef);
impl windows_core::RuntimeType for IShareWindowCommandSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IShareWindowCommandSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportCommandChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommandRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCommandRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CommandInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCommandInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IShareWindowCommandSourceStatics, IShareWindowCommandSourceStatics_Vtbl, 0xb0eb6656_9cac_517c_b6c7_8ef715084295);
impl windows_core::RuntimeType for IShareWindowCommandSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IShareWindowCommandSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITaskbarManager, ITaskbarManager_Vtbl, 0x87490a19_1ad9_49f4_b2e8_86738dc5ac40);
impl windows_core::RuntimeType for ITaskbarManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITaskbarManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPinningAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCurrentAppPinnedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Core")]
    pub IsAppListEntryPinnedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    IsAppListEntryPinnedAsync: usize,
    pub RequestPinCurrentAppAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Core")]
    pub RequestPinAppListEntryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    RequestPinAppListEntryAsync: usize,
}
windows_core::imp::define_interface!(ITaskbarManager2, ITaskbarManager2_Vtbl, 0x79f0a06e_7b02_4911_918c_dee0bbd20ba4);
impl windows_core::RuntimeType for ITaskbarManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITaskbarManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSecondaryTilePinnedAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_StartScreen")]
    pub RequestPinSecondaryTileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_StartScreen"))]
    RequestPinSecondaryTileAsync: usize,
    pub TryUnpinSecondaryTileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITaskbarManagerDesktopAppSupportStatics, ITaskbarManagerDesktopAppSupportStatics_Vtbl, 0xcdfefd63_e879_4134_b9a7_8283f05f9480);
impl windows_core::RuntimeType for ITaskbarManagerDesktopAppSupportStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITaskbarManagerDesktopAppSupportStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ITaskbarManagerStatics, ITaskbarManagerStatics_Vtbl, 0xdb32ab74_de52_4fe6_b7b6_95ff9f8395df);
impl windows_core::RuntimeType for ITaskbarManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITaskbarManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTab, IWindowTab_Vtbl, 0x551e776a_7928_4d60_bdd9_672b5a5758eb);
impl windows_core::RuntimeType for IWindowTab {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTab_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Icon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TreatAsSecondaryTileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTreatAsSecondaryTileId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportThumbnailAvailable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTabCloseRequestedEventArgs, IWindowTabCloseRequestedEventArgs_Vtbl, 0x477282e9_eec4_5882_9889_2dd64d0f9fb6);
impl windows_core::RuntimeType for IWindowTabCloseRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabCloseRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Tab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTabCollection, IWindowTabCollection_Vtbl, 0xaccd0d6c_ed07_519a_8c33_17e02e7e9b0f);
impl windows_core::RuntimeType for IWindowTabCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MoveTab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTabGroup, IWindowTabGroup_Vtbl, 0xa9c2c4fe_6cfe_449c_8b57_5756771abe56);
impl windows_core::RuntimeType for IWindowTabGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Icon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTabIcon, IWindowTabIcon_Vtbl, 0xf92f398f_3669_4d0c_a183_14ddae6f6538);
impl windows_core::RuntimeType for IWindowTabIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabIcon_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IWindowTabIconStatics, IWindowTabIconStatics_Vtbl, 0x2e18d95e_2cbb_4084_af0c_36ee1c2d54b1);
impl windows_core::RuntimeType for IWindowTabIconStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabIconStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromFontGlyph: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromFontGlyphWithUri: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromImage: usize,
}
windows_core::imp::define_interface!(IWindowTabManager, IWindowTabManager_Vtbl, 0x97b3c697_f43a_43e7_b3a2_e889a9835599);
impl windows_core::RuntimeType for IWindowTabManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Tabs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActiveTab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TabSwitchRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTabSwitchRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TabCloseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTabCloseRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TabTearOutRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTabTearOutRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TabThumbnailRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTabThumbnailRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTabManagerStatics, IWindowTabManagerStatics_Vtbl, 0x76755668_45f0_4e0b_8172_4e6d9d0f87bd);
impl windows_core::RuntimeType for IWindowTabManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsTabTearOutSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTabSwitchRequestedEventArgs, IWindowTabSwitchRequestedEventArgs_Vtbl, 0x7cbc421a_58a4_568b_a351_f8a947a5aad8);
impl windows_core::RuntimeType for IWindowTabSwitchRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabSwitchRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Tab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTabTearOutRequestedEventArgs, IWindowTabTearOutRequestedEventArgs_Vtbl, 0x17d66659_5005_5ece_99af_566306e73642);
impl windows_core::RuntimeType for IWindowTabTearOutRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabTearOutRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Tab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowTabThumbnailRequestedEventArgs, IWindowTabThumbnailRequestedEventArgs_Vtbl, 0x2d558e54_9c4e_5abc_ab72_3350fb4937a0);
impl windows_core::RuntimeType for IWindowTabThumbnailRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowTabThumbnailRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Tab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub RequestedSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Imaging::BitmapSize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    RequestedSize: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Image: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetImage: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCompositedOnWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct AdaptiveCardBuilder;
impl AdaptiveCardBuilder {
    pub fn CreateAdaptiveCardFromJson(value: &windows_core::HSTRING) -> windows_core::Result<IAdaptiveCard> {
        Self::IAdaptiveCardBuilderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAdaptiveCardFromJson)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAdaptiveCardBuilderStatics<R, F: FnOnce(&IAdaptiveCardBuilderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AdaptiveCardBuilder, IAdaptiveCardBuilderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AdaptiveCardBuilder {
    const NAME: &'static str = "Windows.UI.Shell.AdaptiveCardBuilder";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FocusSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusSession, windows_core::IUnknown, windows_core::IInspectable);
impl FocusSession {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn End(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).End)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for FocusSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusSession>();
}
unsafe impl windows_core::Interface for FocusSession {
    type Vtable = IFocusSession_Vtbl;
    const IID: windows_core::GUID = <IFocusSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusSession {
    const NAME: &'static str = "Windows.UI.Shell.FocusSession";
}
unsafe impl Send for FocusSession {}
unsafe impl Sync for FocusSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FocusSessionManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusSessionManager, windows_core::IUnknown, windows_core::IInspectable);
impl FocusSessionManager {
    pub fn IsFocusActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetSession(&self, id: &windows_core::HSTRING) -> windows_core::Result<FocusSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSession)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryStartFocusSession(&self) -> windows_core::Result<FocusSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStartFocusSession)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryStartFocusSession2(&self, endtime: super::super::Foundation::DateTime) -> windows_core::Result<FocusSession> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStartFocusSession2)(windows_core::Interface::as_raw(this), endtime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeactivateFocus(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DeactivateFocus)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsFocusActiveChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<FocusSessionManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusActiveChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsFocusActiveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsFocusActiveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<FocusSessionManager> {
        Self::IFocusSessionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IFocusSessionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IFocusSessionManagerStatics<R, F: FnOnce(&IFocusSessionManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FocusSessionManager, IFocusSessionManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FocusSessionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusSessionManager>();
}
unsafe impl windows_core::Interface for FocusSessionManager {
    type Vtable = IFocusSessionManager_Vtbl;
    const IID: windows_core::GUID = <IFocusSessionManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusSessionManager {
    const NAME: &'static str = "Windows.UI.Shell.FocusSessionManager";
}
unsafe impl Send for FocusSessionManager {}
unsafe impl Sync for FocusSessionManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SecurityAppManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SecurityAppManager, windows_core::IUnknown, windows_core::IInspectable);
impl SecurityAppManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SecurityAppManager, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Register<P0>(&self, kind: SecurityAppKind, displayname: &windows_core::HSTRING, detailsuri: P0, registerperuser: bool) -> windows_core::Result<windows_core::GUID>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Register)(windows_core::Interface::as_raw(this), kind, core::mem::transmute_copy(displayname), detailsuri.param().abi(), registerperuser, &mut result__).map(|| result__)
        }
    }
    pub fn Unregister(&self, kind: SecurityAppKind, guidregistration: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this), kind, guidregistration).ok() }
    }
    pub fn UpdateState<P0>(&self, kind: SecurityAppKind, guidregistration: windows_core::GUID, state: SecurityAppState, substatus: SecurityAppSubstatus, detailsuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateState)(windows_core::Interface::as_raw(this), kind, guidregistration, state, substatus, detailsuri.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for SecurityAppManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISecurityAppManager>();
}
unsafe impl windows_core::Interface for SecurityAppManager {
    type Vtable = ISecurityAppManager_Vtbl;
    const IID: windows_core::GUID = <ISecurityAppManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SecurityAppManager {
    const NAME: &'static str = "Windows.UI.Shell.SecurityAppManager";
}
unsafe impl Send for SecurityAppManager {}
unsafe impl Sync for SecurityAppManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ShareWindowCommandEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ShareWindowCommandEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ShareWindowCommandEventArgs {
    pub fn WindowId(&self) -> windows_core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Command(&self) -> windows_core::Result<ShareWindowCommand> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Command)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCommand(&self, value: ShareWindowCommand) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCommand)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ShareWindowCommandEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IShareWindowCommandEventArgs>();
}
unsafe impl windows_core::Interface for ShareWindowCommandEventArgs {
    type Vtable = IShareWindowCommandEventArgs_Vtbl;
    const IID: windows_core::GUID = <IShareWindowCommandEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ShareWindowCommandEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandEventArgs";
}
unsafe impl Send for ShareWindowCommandEventArgs {}
unsafe impl Sync for ShareWindowCommandEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ShareWindowCommandSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ShareWindowCommandSource, windows_core::IUnknown, windows_core::IInspectable);
impl ShareWindowCommandSource {
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCommandChanged(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCommandChanged)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CommandRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCommandRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCommandRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CommandInvoked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ShareWindowCommandSource, ShareWindowCommandEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandInvoked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCommandInvoked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCommandInvoked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<ShareWindowCommandSource> {
        Self::IShareWindowCommandSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IShareWindowCommandSourceStatics<R, F: FnOnce(&IShareWindowCommandSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ShareWindowCommandSource, IShareWindowCommandSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ShareWindowCommandSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IShareWindowCommandSource>();
}
unsafe impl windows_core::Interface for ShareWindowCommandSource {
    type Vtable = IShareWindowCommandSource_Vtbl;
    const IID: windows_core::GUID = <IShareWindowCommandSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ShareWindowCommandSource {
    const NAME: &'static str = "Windows.UI.Shell.ShareWindowCommandSource";
}
unsafe impl Send for ShareWindowCommandSource {}
unsafe impl Sync for ShareWindowCommandSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TaskbarManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TaskbarManager, windows_core::IUnknown, windows_core::IInspectable);
impl TaskbarManager {
    pub fn IsSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPinningAllowed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPinningAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCurrentAppPinnedAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCurrentAppPinnedAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn IsAppListEntryPinnedAsync<P0>(&self, applistentry: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Core::AppListEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAppListEntryPinnedAsync)(windows_core::Interface::as_raw(this), applistentry.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestPinCurrentAppAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPinCurrentAppAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn RequestPinAppListEntryAsync<P0>(&self, applistentry: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Core::AppListEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPinAppListEntryAsync)(windows_core::Interface::as_raw(this), applistentry.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSecondaryTilePinnedAsync(&self, tileid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSecondaryTilePinnedAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_StartScreen")]
    pub fn RequestPinSecondaryTileAsync<P0>(&self, secondarytile: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::StartScreen::SecondaryTile>,
    {
        let this = &windows_core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestPinSecondaryTileAsync)(windows_core::Interface::as_raw(this), secondarytile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryUnpinSecondaryTileAsync(&self, tileid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<ITaskbarManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUnpinSecondaryTileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<TaskbarManager> {
        Self::ITaskbarManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITaskbarManagerStatics<R, F: FnOnce(&ITaskbarManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TaskbarManager, ITaskbarManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TaskbarManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITaskbarManager>();
}
unsafe impl windows_core::Interface for TaskbarManager {
    type Vtable = ITaskbarManager_Vtbl;
    const IID: windows_core::GUID = <ITaskbarManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TaskbarManager {
    const NAME: &'static str = "Windows.UI.Shell.TaskbarManager";
}
unsafe impl Send for TaskbarManager {}
unsafe impl Sync for TaskbarManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTab(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTab, windows_core::IUnknown, windows_core::IInspectable);
impl WindowTab {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowTab, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Icon(&self) -> windows_core::Result<WindowTabIcon> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Icon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowTabIcon>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIcon)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TreatAsSecondaryTileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TreatAsSecondaryTileId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTreatAsSecondaryTileId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTreatAsSecondaryTileId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Group(&self) -> windows_core::Result<WindowTabGroup> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Group)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowTabGroup>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReportThumbnailAvailable(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportThumbnailAvailable)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for WindowTab {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTab>();
}
unsafe impl windows_core::Interface for WindowTab {
    type Vtable = IWindowTab_Vtbl;
    const IID: windows_core::GUID = <IWindowTab as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTab {
    const NAME: &'static str = "Windows.UI.Shell.WindowTab";
}
unsafe impl Send for WindowTab {}
unsafe impl Sync for WindowTab {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTabCloseRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTabCloseRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowTabCloseRequestedEventArgs {
    pub fn Tab(&self) -> windows_core::Result<WindowTab> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tab)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowTabCloseRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTabCloseRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WindowTabCloseRequestedEventArgs {
    type Vtable = IWindowTabCloseRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowTabCloseRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTabCloseRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.WindowTabCloseRequestedEventArgs";
}
unsafe impl Send for WindowTabCloseRequestedEventArgs {}
unsafe impl Sync for WindowTabCloseRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTabCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTabCollection, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(WindowTabCollection, super::super::Foundation::Collections::IIterable::<WindowTab>, super::super::Foundation::Collections::IVector::<WindowTab>);
impl WindowTabCollection {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<WindowTab>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<WindowTab>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<WindowTab> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowTab>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<WindowTab>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowTab>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowTab>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowTab>,
    {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<WindowTab>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[Option<WindowTab>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IVector<WindowTab>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
    pub fn MoveTab<P0>(&self, tab: P0, index: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowTab>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).MoveTab)(windows_core::Interface::as_raw(this), tab.param().abi(), index).ok() }
    }
}
impl windows_core::RuntimeType for WindowTabCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTabCollection>();
}
unsafe impl windows_core::Interface for WindowTabCollection {
    type Vtable = IWindowTabCollection_Vtbl;
    const IID: windows_core::GUID = <IWindowTabCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTabCollection {
    const NAME: &'static str = "Windows.UI.Shell.WindowTabCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for WindowTabCollection {
    type Item = WindowTab;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &WindowTabCollection {
    type Item = WindowTab;
    type IntoIter = super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorIterator::new(windows_core::Interface::cast(self).ok())
    }
}
unsafe impl Send for WindowTabCollection {}
unsafe impl Sync for WindowTabCollection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTabGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTabGroup, windows_core::IUnknown, windows_core::IInspectable);
impl WindowTabGroup {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowTabGroup, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Icon(&self) -> windows_core::Result<WindowTabIcon> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Icon)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowTabIcon>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIcon)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for WindowTabGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTabGroup>();
}
unsafe impl windows_core::Interface for WindowTabGroup {
    type Vtable = IWindowTabGroup_Vtbl;
    const IID: windows_core::GUID = <IWindowTabGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTabGroup {
    const NAME: &'static str = "Windows.UI.Shell.WindowTabGroup";
}
unsafe impl Send for WindowTabGroup {}
unsafe impl Sync for WindowTabGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTabIcon(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTabIcon, windows_core::IUnknown, windows_core::IInspectable);
impl WindowTabIcon {
    pub fn CreateFromFontGlyph(glyph: &windows_core::HSTRING, fontfamily: &windows_core::HSTRING) -> windows_core::Result<WindowTabIcon> {
        Self::IWindowTabIconStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromFontGlyph)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(glyph), core::mem::transmute_copy(fontfamily), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromFontGlyphWithUri<P0>(glyph: &windows_core::HSTRING, fontfamily: &windows_core::HSTRING, fonturi: P0) -> windows_core::Result<WindowTabIcon>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IWindowTabIconStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromFontGlyphWithUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(glyph), core::mem::transmute_copy(fontfamily), fonturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromImage<P0>(image: P0) -> windows_core::Result<WindowTabIcon>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IWindowTabIconStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromImage)(windows_core::Interface::as_raw(this), image.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWindowTabIconStatics<R, F: FnOnce(&IWindowTabIconStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowTabIcon, IWindowTabIconStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowTabIcon {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTabIcon>();
}
unsafe impl windows_core::Interface for WindowTabIcon {
    type Vtable = IWindowTabIcon_Vtbl;
    const IID: windows_core::GUID = <IWindowTabIcon as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTabIcon {
    const NAME: &'static str = "Windows.UI.Shell.WindowTabIcon";
}
unsafe impl Send for WindowTabIcon {}
unsafe impl Sync for WindowTabIcon {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTabManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTabManager, windows_core::IUnknown, windows_core::IInspectable);
impl WindowTabManager {
    pub fn Tabs(&self) -> windows_core::Result<WindowTabCollection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tabs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetActiveTab<P0>(&self, tab: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowTab>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetActiveTab)(windows_core::Interface::as_raw(this), tab.param().abi()).ok() }
    }
    pub fn TabSwitchRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowTabManager, WindowTabSwitchRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabSwitchRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTabSwitchRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTabSwitchRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TabCloseRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowTabManager, WindowTabCloseRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabCloseRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTabCloseRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTabCloseRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TabTearOutRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowTabManager, WindowTabTearOutRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabTearOutRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTabTearOutRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTabTearOutRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TabThumbnailRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowTabManager, WindowTabThumbnailRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabThumbnailRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTabThumbnailRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTabThumbnailRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForWindow(id: super::WindowId) -> windows_core::Result<WindowTabManager> {
        Self::IWindowTabManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindow)(windows_core::Interface::as_raw(this), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IWindowTabManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsTabTearOutSupported() -> windows_core::Result<bool> {
        Self::IWindowTabManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabTearOutSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowTabManagerStatics<R, F: FnOnce(&IWindowTabManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowTabManager, IWindowTabManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowTabManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTabManager>();
}
unsafe impl windows_core::Interface for WindowTabManager {
    type Vtable = IWindowTabManager_Vtbl;
    const IID: windows_core::GUID = <IWindowTabManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTabManager {
    const NAME: &'static str = "Windows.UI.Shell.WindowTabManager";
}
unsafe impl Send for WindowTabManager {}
unsafe impl Sync for WindowTabManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTabSwitchRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTabSwitchRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowTabSwitchRequestedEventArgs {
    pub fn Tab(&self) -> windows_core::Result<WindowTab> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tab)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowTabSwitchRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTabSwitchRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WindowTabSwitchRequestedEventArgs {
    type Vtable = IWindowTabSwitchRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowTabSwitchRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTabSwitchRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.WindowTabSwitchRequestedEventArgs";
}
unsafe impl Send for WindowTabSwitchRequestedEventArgs {}
unsafe impl Sync for WindowTabSwitchRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTabTearOutRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTabTearOutRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowTabTearOutRequestedEventArgs {
    pub fn Tab(&self) -> windows_core::Result<WindowTab> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tab)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WindowId(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WindowId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWindowId(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetWindowId)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowTabTearOutRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTabTearOutRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WindowTabTearOutRequestedEventArgs {
    type Vtable = IWindowTabTearOutRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowTabTearOutRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTabTearOutRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.WindowTabTearOutRequestedEventArgs";
}
unsafe impl Send for WindowTabTearOutRequestedEventArgs {}
unsafe impl Sync for WindowTabTearOutRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WindowTabThumbnailRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowTabThumbnailRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowTabThumbnailRequestedEventArgs {
    pub fn Tab(&self) -> windows_core::Result<WindowTab> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tab)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn RequestedSize(&self) -> windows_core::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestedSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Image(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Image)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetImage<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetImage)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsCompositedOnWindow(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCompositedOnWindow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowTabThumbnailRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowTabThumbnailRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WindowTabThumbnailRequestedEventArgs {
    type Vtable = IWindowTabThumbnailRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowTabThumbnailRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowTabThumbnailRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Shell.WindowTabThumbnailRequestedEventArgs";
}
unsafe impl Send for WindowTabThumbnailRequestedEventArgs {}
unsafe impl Sync for WindowTabThumbnailRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: Self = Self(0i32);
}
impl windows_core::TypeKind for SecurityAppKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SecurityAppKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SecurityAppKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SecurityAppKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl windows_core::TypeKind for SecurityAppState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SecurityAppState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SecurityAppState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SecurityAppState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: Self = Self(0i32);
    pub const NoActionNeeded: Self = Self(1i32);
    pub const ActionRecommended: Self = Self(2i32);
    pub const ActionNeeded: Self = Self(3i32);
}
impl windows_core::TypeKind for SecurityAppSubstatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SecurityAppSubstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SecurityAppSubstatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SecurityAppSubstatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.SecurityAppSubstatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: Self = Self(0i32);
    pub const StartSharing: Self = Self(1i32);
    pub const StopSharing: Self = Self(2i32);
}
impl windows_core::TypeKind for ShareWindowCommand {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ShareWindowCommand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ShareWindowCommand").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ShareWindowCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.ShareWindowCommand;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
