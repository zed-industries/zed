windows_core::imp::define_interface!(IAppInstallItem, IAppInstallItem_Vtbl, 0x49d3dfab_168a_4cbf_a93a_9e448c82737d);
impl windows_core::RuntimeType for IAppInstallItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub InstallType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppInstallType) -> windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Restart: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallItem2, IAppInstallItem2_Vtbl, 0xd3972af8_40c0_4fd7_aa6c_0aa13ca6188c);
impl windows_core::RuntimeType for IAppInstallItem2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallItem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CancelWithTelemetry: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallItem3, IAppInstallItem3_Vtbl, 0x6f3dc998_dd47_433c_9234_560172d67a45);
impl windows_core::RuntimeType for IAppInstallItem3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallItem3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub ItemOperationsMightAffectOtherItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallItem4, IAppInstallItem4_Vtbl, 0xc2d1ce12_71ff_4fc8_b540_453d4b37e1d1);
impl windows_core::RuntimeType for IAppInstallItem4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallItem4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LaunchAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallItem5, IAppInstallItem5_Vtbl, 0x5510e7cc_4076_4a0b_9472_c21d9d380e55);
impl windows_core::RuntimeType for IAppInstallItem5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallItem5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppInstallationToastNotificationMode) -> windows_core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppInstallationToastNotificationMode) -> windows_core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppInstallationToastNotificationMode) -> windows_core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppInstallationToastNotificationMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallManager, IAppInstallManager_Vtbl, 0x9353e170_8441_4b45_bd72_7c2fa925beee);
impl windows_core::RuntimeType for IAppInstallManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItems: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Restart: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ItemCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveItemCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ItemStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveItemStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AutoUpdateSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoUpdateSetting) -> windows_core::HRESULT,
    pub SetAutoUpdateSetting: unsafe extern "system" fn(*mut core::ffi::c_void, AutoUpdateSetting) -> windows_core::HRESULT,
    pub AcquisitionIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAcquisitionIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetIsApplicableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartAppInstallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, bool, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateAppByPackageFamilyNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchForUpdatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesAsync: usize,
    pub IsStoreBlockedByPolicyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIsAppAllowedToInstallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallManager2, IAppInstallManager2_Vtbl, 0x16937851_ed37_480d_8314_52e27c03f04a);
impl windows_core::RuntimeType for IAppInstallManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAppInstallWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, bool, bool, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateAppByPackageFamilyNameWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchForUpdatesWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithTelemetryAsync: usize,
    pub GetIsAppAllowedToInstallWithTelemetryAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelWithTelemetry: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallManager3, IAppInstallManager3_Vtbl, 0x95b24b17_e96a_4d0e_84e1_c8cb417a0178);
impl windows_core::RuntimeType for IAppInstallManager3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallManager3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub StartProductInstallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, bool, bool, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    StartProductInstallAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub StartProductInstallForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, bool, bool, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System")))]
    StartProductInstallForUserAsync: usize,
    #[cfg(feature = "System")]
    pub UpdateAppByPackageFamilyNameForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    UpdateAppByPackageFamilyNameForUserAsync: usize,
    #[cfg(feature = "System")]
    pub SearchForUpdatesForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SearchForUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesForUserAsync: usize,
    #[cfg(feature = "System")]
    pub GetIsAppAllowedToInstallForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetIsAppAllowedToInstallForUserAsync: usize,
    #[cfg(feature = "System")]
    pub GetIsApplicableForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetIsApplicableForUserAsync: usize,
    pub MoveToFrontOfDownloadQueue: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallManager4, IAppInstallManager4_Vtbl, 0x260a2a16_5a9e_4ebd_b944_f2ba75c31159);
impl windows_core::RuntimeType for IAppInstallManager4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallManager4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFreeUserEntitlementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetFreeUserEntitlementForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFreeUserEntitlementForUserAsync: usize,
    pub GetFreeDeviceEntitlementAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallManager5, IAppInstallManager5_Vtbl, 0x3cd7be4c_1be9_4f7f_b675_aa1d64a529b2);
impl windows_core::RuntimeType for IAppInstallManager5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallManager5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItemsWithGroupSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItemsWithGroupSupport: usize,
}
windows_core::imp::define_interface!(IAppInstallManager6, IAppInstallManager6_Vtbl, 0xc9e7d408_f27a_4471_b2f4_e76efcbebcca);
impl windows_core::RuntimeType for IAppInstallManager6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallManager6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesWithUpdateOptionsForUserAsync: usize,
    pub SearchForUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub SearchForUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SearchForUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartProductInstallWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartProductInstallWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub StartProductInstallWithOptionsForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    StartProductInstallWithOptionsForUserAsync: usize,
    pub GetIsPackageIdentityAllowedToInstallAsync: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetIsPackageIdentityAllowedToInstallForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, std::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetIsPackageIdentityAllowedToInstallForUserAsync: usize,
}
windows_core::imp::define_interface!(IAppInstallManager7, IAppInstallManager7_Vtbl, 0xa5ee7b30_d5e4_49a3_9853_3db03203321d);
impl windows_core::RuntimeType for IAppInstallManager7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallManager7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanInstallForAllUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallManagerItemEventArgs, IAppInstallManagerItemEventArgs_Vtbl, 0xbc505743_4674_4dd1_957e_c25682086a14);
impl windows_core::RuntimeType for IAppInstallManagerItemEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallManagerItemEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallOptions, IAppInstallOptions_Vtbl, 0xc9808300_1cb8_4eb6_8c9f_6a30c64a5b51);
impl windows_core::RuntimeType for IAppInstallOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CatalogId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ForceUseOfNonRemovableStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceUseOfNonRemovableStorage: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Repair: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRepair: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub TargetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    TargetVolume: usize,
    #[cfg(feature = "Management_Deployment")]
    pub SetTargetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    SetTargetVolume: usize,
    pub LaunchAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallOptions2, IAppInstallOptions2_Vtbl, 0x8a04c0d7_c94b_425e_95b4_bf27faeaee89);
impl windows_core::RuntimeType for IAppInstallOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppInstallationToastNotificationMode) -> windows_core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppInstallationToastNotificationMode) -> windows_core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppInstallationToastNotificationMode) -> windows_core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(*mut core::ffi::c_void, AppInstallationToastNotificationMode) -> windows_core::HRESULT,
    pub InstallForAllUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetInstallForAllUsers: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub StageButDoNotInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetStageButDoNotInstall: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCampaignId: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ExtendedCampaignId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetExtendedCampaignId: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallStatus, IAppInstallStatus_Vtbl, 0x936dccfa_2450_4126_88b1_6127a644dd5c);
impl windows_core::RuntimeType for IAppInstallStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstallState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppInstallState) -> windows_core::HRESULT,
    pub DownloadSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub BytesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallStatus2, IAppInstallStatus2_Vtbl, 0x96e7818a_5e92_4aa9_8edc_58fed4b87e00);
impl windows_core::RuntimeType for IAppInstallStatus2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallStatus2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub ReadyForLaunch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallStatus3, IAppInstallStatus3_Vtbl, 0xcb880c56_837b_4b4c_9ebb_6d44a0a96307);
impl windows_core::RuntimeType for IAppInstallStatus3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallStatus3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsStaged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppUpdateOptions, IAppUpdateOptions_Vtbl, 0x26f0b02f_c2f3_4aea_af8c_6308dd9db85f);
impl windows_core::RuntimeType for IAppUpdateOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppUpdateOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CatalogId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(*mut core::ffi::c_void, std::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppUpdateOptions2, IAppUpdateOptions2_Vtbl, 0xf4646e08_ed26_4bf9_9679_48f628e53df8);
impl windows_core::RuntimeType for IAppUpdateOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppUpdateOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetEntitlementResult, IGetEntitlementResult_Vtbl, 0x74fc843f_1a9e_4609_8e4d_819086d08a3d);
impl windows_core::RuntimeType for IGetEntitlementResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGetEntitlementResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GetEntitlementStatus) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppInstallItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInstallItem, windows_core::IUnknown, windows_core::IInspectable);
impl AppInstallItem {
    pub fn ProductId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstallType(&self) -> windows_core::Result<AppInstallType> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUserInitiated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUserInitiated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentStatus(&self) -> windows_core::Result<AppInstallStatus> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentStatus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Restart(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Restart)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Completed<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStatusChanged(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CancelWithTelemetry(&self, correlationvector: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CancelWithTelemetry)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn PauseWithTelemetry(&self, correlationvector: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PauseWithTelemetry)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn RestartWithTelemetry(&self, correlationvector: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RestartWithTelemetry)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(correlationvector)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &windows_core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ItemOperationsMightAffectOtherItems(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemOperationsMightAffectOtherItems)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LaunchAfterInstall(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchAfterInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLaunchAfterInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PinToDesktopAfterInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPinToDesktopAfterInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PinToStartAfterInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPinToStartAfterInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PinToTaskbarAfterInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPinToTaskbarAfterInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CompletedInstallToastNotificationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompletedInstallToastNotificationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallInProgressToastNotificationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallItem5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInstallInProgressToastNotificationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AppInstallItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallItem>();
}
unsafe impl windows_core::Interface for AppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
    const IID: windows_core::GUID = <IAppInstallItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
}
unsafe impl Send for AppInstallItem {}
unsafe impl Sync for AppInstallItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppInstallManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInstallManager, windows_core::IUnknown, windows_core::IInspectable);
impl AppInstallManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppInstallManager, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItems(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AppInstallItems)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Cancel(&self, productid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid)).ok() }
    }
    pub fn Pause(&self, productid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid)).ok() }
    }
    pub fn Restart(&self, productid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Restart)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid)).ok() }
    }
    pub fn ItemCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveItemCompleted(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveItemCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ItemStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveItemStatusChanged(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveItemStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AutoUpdateSetting(&self) -> windows_core::Result<AutoUpdateSetting> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoUpdateSetting)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoUpdateSetting)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AcquisitionIdentity(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AcquisitionIdentity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAcquisitionIdentity(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAcquisitionIdentity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetIsApplicableAsync(&self, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsApplicableAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAppInstallAsync(&self, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAppInstallAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), repair, forceuseofnonremovablestorage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateAppByPackageFamilyNameAsync(&self, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SearchForUpdatesAsync(&self, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForUpdatesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesAsync(&self) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForAllUpdatesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsStoreBlockedByPolicyAsync(&self, storeclientname: &windows_core::HSTRING, storeclientpublisher: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStoreBlockedByPolicyAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(storeclientname), core::mem::transmute_copy(storeclientpublisher), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIsAppAllowedToInstallAsync(&self, productid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsAppAllowedToInstallAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartAppInstallWithTelemetryAsync(&self, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, catalogid: &windows_core::HSTRING, bundleid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAppInstallWithTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), repair, forceuseofnonremovablestorage, core::mem::transmute_copy(catalogid), core::mem::transmute_copy(bundleid), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateAppByPackageFamilyNameWithTelemetryAsync(&self, packagefamilyname: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameWithTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SearchForUpdatesWithTelemetryAsync(&self, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING, catalogid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForUpdatesWithTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), core::mem::transmute_copy(catalogid), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesWithTelemetryAsync(&self, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForAllUpdatesWithTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIsAppAllowedToInstallWithTelemetryAsync(&self, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING, catalogid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsAppAllowedToInstallWithTelemetryAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), core::mem::transmute_copy(catalogid), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CancelWithTelemetry(&self, productid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CancelWithTelemetry)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn PauseWithTelemetry(&self, productid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PauseWithTelemetry)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn RestartWithTelemetry(&self, productid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RestartWithTelemetry)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(correlationvector)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn StartProductInstallAsync<P0>(&self, productid: &windows_core::HSTRING, catalogid: &windows_core::HSTRING, flightid: &windows_core::HSTRING, clientid: &windows_core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &windows_core::HSTRING, targetvolume: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: windows_core::Param<super::super::super::super::Management::Deployment::PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProductInstallAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(catalogid), core::mem::transmute_copy(flightid), core::mem::transmute_copy(clientid), repair, forceuseofnonremovablestorage, core::mem::transmute_copy(correlationvector), targetvolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub fn StartProductInstallForUserAsync<P0, P1>(&self, user: P0, productid: &windows_core::HSTRING, catalogid: &windows_core::HSTRING, flightid: &windows_core::HSTRING, clientid: &windows_core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &windows_core::HSTRING, targetvolume: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
        P1: windows_core::Param<super::super::super::super::Management::Deployment::PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProductInstallForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(productid), core::mem::transmute_copy(catalogid), core::mem::transmute_copy(flightid), core::mem::transmute_copy(clientid), repair, forceuseofnonremovablestorage, core::mem::transmute_copy(correlationvector), targetvolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn UpdateAppByPackageFamilyNameForUserAsync<P0>(&self, user: P0, packagefamilyname: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(packagefamilyname), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn SearchForUpdatesForUserAsync<P0>(&self, user: P0, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING, catalogid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForUpdatesForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), core::mem::transmute_copy(catalogid), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesForUserAsync<P0>(&self, user: P0, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForAllUpdatesForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn GetIsAppAllowedToInstallForUserAsync<P0>(&self, user: P0, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING, catalogid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsAppAllowedToInstallForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), core::mem::transmute_copy(catalogid), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn GetIsApplicableForUserAsync<P0>(&self, user: P0, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsApplicableForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MoveToFrontOfDownloadQueue(&self, productid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallManager3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).MoveToFrontOfDownloadQueue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn GetFreeUserEntitlementAsync(&self, storeid: &windows_core::HSTRING, campaignid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFreeUserEntitlementAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(storeid), core::mem::transmute_copy(campaignid), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn GetFreeUserEntitlementForUserAsync<P0>(&self, user: P0, storeid: &windows_core::HSTRING, campaignid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFreeUserEntitlementForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(storeid), core::mem::transmute_copy(campaignid), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFreeDeviceEntitlementAsync(&self, storeid: &windows_core::HSTRING, campaignid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFreeDeviceEntitlementAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(storeid), core::mem::transmute_copy(campaignid), core::mem::transmute_copy(correlationvector), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItemsWithGroupSupport(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager5>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AppInstallItemsWithGroupSupport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesWithUpdateOptionsAsync<P0>(&self, correlationvector: &windows_core::HSTRING, clientid: &windows_core::HSTRING, updateoptions: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: windows_core::Param<AppUpdateOptions>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForAllUpdatesWithUpdateOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(correlationvector), core::mem::transmute_copy(clientid), updateoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsForUserAsync<P0, P1>(&self, user: P0, correlationvector: &windows_core::HSTRING, clientid: &windows_core::HSTRING, updateoptions: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
        P1: windows_core::Param<AppUpdateOptions>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForAllUpdatesWithUpdateOptionsForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(correlationvector), core::mem::transmute_copy(clientid), updateoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SearchForUpdatesWithUpdateOptionsAsync<P0>(&self, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING, clientid: &windows_core::HSTRING, updateoptions: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>
    where
        P0: windows_core::Param<AppUpdateOptions>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForUpdatesWithUpdateOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), core::mem::transmute_copy(correlationvector), core::mem::transmute_copy(clientid), updateoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn SearchForUpdatesWithUpdateOptionsForUserAsync<P0, P1>(&self, user: P0, productid: &windows_core::HSTRING, skuid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING, clientid: &windows_core::HSTRING, updateoptions: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
        P1: windows_core::Param<AppUpdateOptions>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).SearchForUpdatesWithUpdateOptionsForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(productid), core::mem::transmute_copy(skuid), core::mem::transmute_copy(correlationvector), core::mem::transmute_copy(clientid), updateoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartProductInstallWithOptionsAsync<P0>(&self, productid: &windows_core::HSTRING, flightid: &windows_core::HSTRING, clientid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING, installoptions: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: windows_core::Param<AppInstallOptions>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProductInstallWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(productid), core::mem::transmute_copy(flightid), core::mem::transmute_copy(clientid), core::mem::transmute_copy(correlationvector), installoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn StartProductInstallWithOptionsForUserAsync<P0, P1>(&self, user: P0, productid: &windows_core::HSTRING, flightid: &windows_core::HSTRING, clientid: &windows_core::HSTRING, correlationvector: &windows_core::HSTRING, installoptions: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
        P1: windows_core::Param<AppInstallOptions>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StartProductInstallWithOptionsForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(productid), core::mem::transmute_copy(flightid), core::mem::transmute_copy(clientid), core::mem::transmute_copy(correlationvector), installoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetIsPackageIdentityAllowedToInstallAsync(&self, correlationvector: &windows_core::HSTRING, packageidentityname: &windows_core::HSTRING, publishercertificatename: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsPackageIdentityAllowedToInstallAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(correlationvector), core::mem::transmute_copy(packageidentityname), core::mem::transmute_copy(publishercertificatename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn GetIsPackageIdentityAllowedToInstallForUserAsync<P0>(&self, user: P0, correlationvector: &windows_core::HSTRING, packageidentityname: &windows_core::HSTRING, publishercertificatename: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<super::super::super::super::System::User>,
    {
        let this = &windows_core::Interface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsPackageIdentityAllowedToInstallForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(correlationvector), core::mem::transmute_copy(packageidentityname), core::mem::transmute_copy(publishercertificatename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanInstallForAllUsers(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallManager7>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CanInstallForAllUsers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppInstallManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallManager>();
}
unsafe impl windows_core::Interface for AppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
    const IID: windows_core::GUID = <IAppInstallManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
}
unsafe impl Send for AppInstallManager {}
unsafe impl Sync for AppInstallManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppInstallManagerItemEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInstallManagerItemEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AppInstallManagerItemEventArgs {
    pub fn Item(&self) -> windows_core::Result<AppInstallItem> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Item)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppInstallManagerItemEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallManagerItemEventArgs>();
}
unsafe impl windows_core::Interface for AppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
    const IID: windows_core::GUID = <IAppInstallManagerItemEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
}
unsafe impl Send for AppInstallManagerItemEventArgs {}
unsafe impl Sync for AppInstallManagerItemEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppInstallOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInstallOptions, windows_core::IUnknown, windows_core::IInspectable);
impl AppInstallOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppInstallOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CatalogId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CatalogId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCatalogId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCatalogId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ForceUseOfNonRemovableStorage(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceUseOfNonRemovableStorage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceUseOfNonRemovableStorage)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowForcedAppRestart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowForcedAppRestart)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Repair(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Repair)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRepair(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRepair)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Management_Deployment")]
    pub fn TargetVolume(&self) -> windows_core::Result<super::super::super::super::Management::Deployment::PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetVolume)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Management_Deployment")]
    pub fn SetTargetVolume<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Management::Deployment::PackageVolume>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTargetVolume)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LaunchAfterInstall(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchAfterInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLaunchAfterInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PinToDesktopAfterInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPinToDesktopAfterInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PinToStartAfterInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPinToStartAfterInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PinToTaskbarAfterInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPinToTaskbarAfterInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CompletedInstallToastNotificationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompletedInstallToastNotificationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallInProgressToastNotificationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInstallInProgressToastNotificationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallForAllUsers(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallForAllUsers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInstallForAllUsers(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInstallForAllUsers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageButDoNotInstall(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).StageButDoNotInstall)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStageButDoNotInstall(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStageButDoNotInstall)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CampaignId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CampaignId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCampaignId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCampaignId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ExtendedCampaignId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedCampaignId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExtendedCampaignId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExtendedCampaignId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for AppInstallOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallOptions>();
}
unsafe impl windows_core::Interface for AppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
    const IID: windows_core::GUID = <IAppInstallOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
}
unsafe impl Send for AppInstallOptions {}
unsafe impl Sync for AppInstallOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppInstallStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInstallStatus, windows_core::IUnknown, windows_core::IInspectable);
impl AppInstallStatus {
    pub fn InstallState(&self) -> windows_core::Result<AppInstallState> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DownloadSizeInBytes(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).DownloadSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BytesDownloaded(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesDownloaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PercentComplete(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).PercentComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::super::super::System::User> {
        let this = &windows_core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadyForLaunch(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadyForLaunch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallStatus3>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppInstallStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallStatus>();
}
unsafe impl windows_core::Interface for AppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
    const IID: windows_core::GUID = <IAppInstallStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
}
unsafe impl Send for AppInstallStatus {}
unsafe impl Sync for AppInstallStatus {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppUpdateOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppUpdateOptions, windows_core::IUnknown, windows_core::IInspectable);
impl AppUpdateOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppUpdateOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CatalogId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).CatalogId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCatalogId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCatalogId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowForcedAppRestart)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowForcedAppRestart)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomaticallyDownloadAndInstallUpdateIfFound)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAppUpdateOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAutomaticallyDownloadAndInstallUpdateIfFound)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AppUpdateOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppUpdateOptions>();
}
unsafe impl windows_core::Interface for AppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
    const IID: windows_core::GUID = <IAppUpdateOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
}
unsafe impl Send for AppUpdateOptions {}
unsafe impl Sync for AppUpdateOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GetEntitlementResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GetEntitlementResult, windows_core::IUnknown, windows_core::IInspectable);
impl GetEntitlementResult {
    pub fn Status(&self) -> windows_core::Result<GetEntitlementStatus> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GetEntitlementResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGetEntitlementResult>();
}
unsafe impl windows_core::Interface for GetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
    const IID: windows_core::GUID = <IGetEntitlementResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
}
unsafe impl Send for GetEntitlementResult {}
unsafe impl Sync for GetEntitlementResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const AcquiringLicense: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const RestoringData: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedLowBattery: Self = Self(10i32);
    pub const PausedWiFiRecommended: Self = Self(11i32);
    pub const PausedWiFiRequired: Self = Self(12i32);
    pub const ReadyToDownload: Self = Self(13i32);
}
impl windows_core::TypeKind for AppInstallState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppInstallState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppInstallState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppInstallState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl windows_core::TypeKind for AppInstallType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppInstallType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppInstallType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppInstallType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
impl windows_core::TypeKind for AppInstallationToastNotificationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppInstallationToastNotificationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppInstallationToastNotificationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppInstallationToastNotificationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
impl windows_core::TypeKind for AutoUpdateSetting {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutoUpdateSetting {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutoUpdateSetting").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AutoUpdateSetting {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl windows_core::TypeKind for GetEntitlementStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GetEntitlementStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GetEntitlementStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GetEntitlementStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus;i4)");
}
