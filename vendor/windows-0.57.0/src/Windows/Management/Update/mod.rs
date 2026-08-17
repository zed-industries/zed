windows_core::imp::define_interface!(IPreviewBuildsManager, IPreviewBuildsManager_Vtbl, 0xfa07dd61_7e4f_59f7_7c9f_def9051c5f62);
impl windows_core::RuntimeType for IPreviewBuildsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPreviewBuildsManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ArePreviewBuildsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetArePreviewBuildsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SyncAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPreviewBuildsManagerStatics, IPreviewBuildsManagerStatics_Vtbl, 0x3e422887_b112_5a70_7da1_97d78d32aa29);
impl windows_core::RuntimeType for IPreviewBuildsManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPreviewBuildsManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPreviewBuildsState, IPreviewBuildsState_Vtbl, 0xa2f2903e_b223_5f63_7546_3e8eac070a2e);
impl windows_core::RuntimeType for IPreviewBuildsState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPreviewBuildsState_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IWindowsUpdate, IWindowsUpdate_Vtbl, 0xc3c88dd7_0ef3_52b2_a9ad_66bfc6bd9582);
impl windows_core::RuntimeType for IWindowsUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsFeatureUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMinorImpact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCritical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsForOS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDriver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMandatory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsUrgent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSeeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEulaAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EulaText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AttentionRequiredInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptEula: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateActionCompletedEventArgs, IWindowsUpdateActionCompletedEventArgs_Vtbl, 0x2c44b950_a655_5321_aec1_aee762922131);
impl windows_core::RuntimeType for IWindowsUpdateActionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateActionCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateActionProgress, IWindowsUpdateActionProgress_Vtbl, 0x83b22d8a_4bb0_549f_ba39_59724882d137);
impl windows_core::RuntimeType for IWindowsUpdateActionProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateActionProgress_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateActionResult, IWindowsUpdateActionResult_Vtbl, 0xe6692c62_f697_51b7_ab7f_e73e5e688f12);
impl windows_core::RuntimeType for IWindowsUpdateActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateActionResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateAdministrator, IWindowsUpdateAdministrator_Vtbl, 0x7a60181c_ba1e_5cf9_aa65_304120b73d72);
impl windows_core::RuntimeType for IWindowsUpdateAdministrator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateAdministrator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartAdministratorScan: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApproveWindowsUpdateAction: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RevokeWindowsUpdateActionApproval: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ApproveWindowsUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeWindowsUpdateApproval: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUpdates: usize,
}
windows_core::imp::define_interface!(IWindowsUpdateAdministratorStatics, IWindowsUpdateAdministratorStatics_Vtbl, 0x013e6d36_ef69_53bc_8db8_c403bca550ed);
impl windows_core::RuntimeType for IWindowsUpdateAdministratorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateAdministratorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetRegisteredAdministrator: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterForAdministration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, WindowsUpdateAdministratorOptions, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
    pub UnregisterForAdministration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
    pub GetRegisteredAdministratorName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RequestRestart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CancelRestartRequest: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateApprovalData, IWindowsUpdateApprovalData_Vtbl, 0xaadf5bfd_84db_59bc_85e2_ad4fc1f62f7c);
impl windows_core::RuntimeType for IWindowsUpdateApprovalData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateApprovalData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Seeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSeeker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowDownloadOnMetered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAllowDownloadOnMetered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateAttentionRequiredInfo, IWindowsUpdateAttentionRequiredInfo_Vtbl, 0x44df2579_74d3_5ffa_b6ce_09e187e1e0ed);
impl windows_core::RuntimeType for IWindowsUpdateAttentionRequiredInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateAttentionRequiredInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAttentionRequiredReason) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateAttentionRequiredReasonChangedEventArgs, IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl, 0x0627abca_dbb8_524a_b1d2_d9df004eeb31);
impl windows_core::RuntimeType for IWindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAttentionRequiredReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateGetAdministratorResult, IWindowsUpdateGetAdministratorResult_Vtbl, 0xbb39ffc4_2c42_5b1c_8995_343341c92c50);
impl windows_core::RuntimeType for IWindowsUpdateGetAdministratorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateGetAdministratorResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Administrator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowsUpdateAdministratorStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateItem, IWindowsUpdateItem_Vtbl, 0xb222e44a_49b6_59bf_a033_ef617cd73a98);
impl windows_core::RuntimeType for IWindowsUpdateItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateManager, IWindowsUpdateManager_Vtbl, 0x5dd966c0_a71a_5602_bbd0_09a70e4573fa);
impl windows_core::RuntimeType for IWindowsUpdateManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScanningStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveScanningStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub WorkingStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveWorkingStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProgressChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AttentionRequiredReasonChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAttentionRequiredReasonChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ActionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveActionCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ScanCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveScanCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsScanning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsWorking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub LastSuccessfulScanTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetApplicableUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetApplicableUpdates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMostRecentCompletedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMostRecentCompletedUpdates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMostRecentCompletedUpdatesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMostRecentCompletedUpdatesAsync: usize,
    pub StartScan: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateManagerFactory, IWindowsUpdateManagerFactory_Vtbl, 0x1b394df8_decb_5f44_b47c_6ccf3bcfdb37);
impl windows_core::RuntimeType for IWindowsUpdateManagerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateManagerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateProgressChangedEventArgs, IWindowsUpdateProgressChangedEventArgs_Vtbl, 0xbbfbdeeb_94c8_5aa7_b0fb_66c67c233b0a);
impl windows_core::RuntimeType for IWindowsUpdateProgressChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateProgressChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateRestartRequestOptions, IWindowsUpdateRestartRequestOptions_Vtbl, 0x38cfb7d3_4188_5222_905c_6c4443c951ee);
impl windows_core::RuntimeType for IWindowsUpdateRestartRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateRestartRequestOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMoreInfoUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetComplianceDeadlineInDays: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetComplianceGracePeriodInDays: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub OrganizationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetOrganizationName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub OptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetOptOutOfAutoReboot: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateRestartRequestOptionsFactory, IWindowsUpdateRestartRequestOptionsFactory_Vtbl, 0x75f41d04_0e17_50d0_8c15_6b9d0539b3a9);
impl windows_core::RuntimeType for IWindowsUpdateRestartRequestOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateRestartRequestOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowsUpdateScanCompletedEventArgs, IWindowsUpdateScanCompletedEventArgs_Vtbl, 0x95b6953e_ba5c_5fe8_b115_12de184a6bb0);
impl windows_core::RuntimeType for IWindowsUpdateScanCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWindowsUpdateScanCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Updates: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PreviewBuildsManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PreviewBuildsManager, windows_core::IUnknown, windows_core::IInspectable);
impl PreviewBuildsManager {
    pub fn ArePreviewBuildsAllowed(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArePreviewBuildsAllowed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetArePreviewBuildsAllowed(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetArePreviewBuildsAllowed)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrentState(&self) -> windows_core::Result<PreviewBuildsState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentState)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SyncAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SyncAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<PreviewBuildsManager> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IPreviewBuildsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IPreviewBuildsManagerStatics<R, F: FnOnce(&IPreviewBuildsManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PreviewBuildsManager, IPreviewBuildsManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PreviewBuildsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPreviewBuildsManager>();
}
unsafe impl windows_core::Interface for PreviewBuildsManager {
    type Vtable = IPreviewBuildsManager_Vtbl;
    const IID: windows_core::GUID = <IPreviewBuildsManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PreviewBuildsManager {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsManager";
}
unsafe impl Send for PreviewBuildsManager {}
unsafe impl Sync for PreviewBuildsManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PreviewBuildsState(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PreviewBuildsState, windows_core::IUnknown, windows_core::IInspectable);
impl PreviewBuildsState {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PreviewBuildsState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPreviewBuildsState>();
}
unsafe impl windows_core::Interface for PreviewBuildsState {
    type Vtable = IPreviewBuildsState_Vtbl;
    const IID: windows_core::GUID = <IPreviewBuildsState as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PreviewBuildsState {
    const NAME: &'static str = "Windows.Management.Update.PreviewBuildsState";
}
unsafe impl Send for PreviewBuildsState {}
unsafe impl Sync for PreviewBuildsState {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdate, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdate {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn IsFeatureUpdate(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFeatureUpdate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMinorImpact(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMinorImpact)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSecurity(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSecurity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsCritical(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCritical)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsForOS(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsForOS)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDriver(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDriver)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsMandatory(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMandatory)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsUrgent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsUrgent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSeeker(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSeeker)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SupportUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsEulaAccepted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEulaAccepted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EulaText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EulaText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AttentionRequiredInfo(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttentionRequiredInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActionResult(&self) -> windows_core::Result<WindowsUpdateActionResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionResult)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentAction(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentAction)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActionProgress(&self) -> windows_core::Result<WindowsUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionProgress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPropertyValue(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertyValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AcceptEula(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AcceptEula)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for WindowsUpdate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdate>();
}
unsafe impl windows_core::Interface for WindowsUpdate {
    type Vtable = IWindowsUpdate_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdate {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdate";
}
unsafe impl Send for WindowsUpdate {}
unsafe impl Sync for WindowsUpdate {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateActionCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionCompletedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionCompletedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionCompletedEventArgs {
    type Vtable = IWindowsUpdateActionCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateActionCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateActionCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionCompletedEventArgs";
}
unsafe impl Send for WindowsUpdateActionCompletedEventArgs {}
unsafe impl Sync for WindowsUpdateActionCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateActionProgress(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionProgress, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionProgress {
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionProgress>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionProgress {
    type Vtable = IWindowsUpdateActionProgress_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateActionProgress as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateActionProgress {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionProgress";
}
unsafe impl Send for WindowsUpdateActionProgress {}
unsafe impl Sync for WindowsUpdateActionProgress {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateActionResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateActionResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateActionResult {
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Action(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Action)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateActionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateActionResult>();
}
unsafe impl windows_core::Interface for WindowsUpdateActionResult {
    type Vtable = IWindowsUpdateActionResult_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateActionResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateActionResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateActionResult";
}
unsafe impl Send for WindowsUpdateActionResult {}
unsafe impl Sync for WindowsUpdateActionResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateAdministrator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAdministrator, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAdministrator {
    pub fn StartAdministratorScan(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartAdministratorScan)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApproveWindowsUpdateAction(&self, updateid: &windows_core::HSTRING, action: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ApproveWindowsUpdateAction)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), core::mem::transmute_copy(action)).ok() }
    }
    pub fn RevokeWindowsUpdateActionApproval(&self, updateid: &windows_core::HSTRING, action: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RevokeWindowsUpdateActionApproval)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), core::mem::transmute_copy(action)).ok() }
    }
    pub fn ApproveWindowsUpdate<P0>(&self, updateid: &windows_core::HSTRING, approvaldata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<WindowsUpdateApprovalData>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ApproveWindowsUpdate)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), approvaldata.param().abi()).ok() }
    }
    pub fn RevokeWindowsUpdateApproval(&self, updateid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RevokeWindowsUpdateApproval)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUpdates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUpdates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRegisteredAdministrator(organizationname: &windows_core::HSTRING) -> windows_core::Result<WindowsUpdateGetAdministratorResult> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRegisteredAdministrator)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RegisterForAdministration(organizationname: &windows_core::HSTRING, options: WindowsUpdateAdministratorOptions) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterForAdministration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), options, &mut result__).map(|| result__)
        })
    }
    pub fn UnregisterForAdministration(organizationname: &windows_core::HSTRING) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnregisterForAdministration)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(organizationname), &mut result__).map(|| result__)
        })
    }
    pub fn GetRegisteredAdministratorName() -> windows_core::Result<windows_core::HSTRING> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRegisteredAdministratorName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn RequestRestart<P0>(restartoptions: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<WindowsUpdateRestartRequestOptions>,
    {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestRestart)(windows_core::Interface::as_raw(this), restartoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CancelRestartRequest(requestrestarttoken: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IWindowsUpdateAdministratorStatics(|this| unsafe { (windows_core::Interface::vtable(this).CancelRestartRequest)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(requestrestarttoken)).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateAdministratorStatics<R, F: FnOnce(&IWindowsUpdateAdministratorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateAdministrator, IWindowsUpdateAdministratorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateAdministrator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAdministrator>();
}
unsafe impl windows_core::Interface for WindowsUpdateAdministrator {
    type Vtable = IWindowsUpdateAdministrator_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateAdministrator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateAdministrator {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAdministrator";
}
unsafe impl Send for WindowsUpdateAdministrator {}
unsafe impl Sync for WindowsUpdateAdministrator {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateApprovalData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateApprovalData, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateApprovalData {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateApprovalData, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Seeker(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Seeker)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSeeker<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSeeker)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowDownloadOnMetered(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDownloadOnMetered)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAllowDownloadOnMetered<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDownloadOnMetered)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ComplianceDeadlineInDays(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComplianceDeadlineInDays<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComplianceGracePeriodInDays<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OptOutOfAutoReboot(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptOutOfAutoReboot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOptOutOfAutoReboot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOptOutOfAutoReboot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for WindowsUpdateApprovalData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateApprovalData>();
}
unsafe impl windows_core::Interface for WindowsUpdateApprovalData {
    type Vtable = IWindowsUpdateApprovalData_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateApprovalData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateApprovalData {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateApprovalData";
}
unsafe impl Send for WindowsUpdateApprovalData {}
unsafe impl Sync for WindowsUpdateApprovalData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateAttentionRequiredInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAttentionRequiredInfo, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAttentionRequiredInfo {
    pub fn Reason(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAttentionRequiredInfo>();
}
unsafe impl windows_core::Interface for WindowsUpdateAttentionRequiredInfo {
    type Vtable = IWindowsUpdateAttentionRequiredInfo_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateAttentionRequiredInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateAttentionRequiredInfo {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredInfo";
}
unsafe impl Send for WindowsUpdateAttentionRequiredInfo {}
unsafe impl Sync for WindowsUpdateAttentionRequiredInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateAttentionRequiredReasonChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateAttentionRequiredReasonChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reason(&self) -> windows_core::Result<WindowsUpdateAttentionRequiredReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateAttentionRequiredReasonChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    type Vtable = IWindowsUpdateAttentionRequiredReasonChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateAttentionRequiredReasonChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs";
}
unsafe impl Send for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
unsafe impl Sync for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateGetAdministratorResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateGetAdministratorResult, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateGetAdministratorResult {
    pub fn Administrator(&self) -> windows_core::Result<WindowsUpdateAdministrator> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Administrator)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<WindowsUpdateAdministratorStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateGetAdministratorResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateGetAdministratorResult>();
}
unsafe impl windows_core::Interface for WindowsUpdateGetAdministratorResult {
    type Vtable = IWindowsUpdateGetAdministratorResult_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateGetAdministratorResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateGetAdministratorResult {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateGetAdministratorResult";
}
unsafe impl Send for WindowsUpdateGetAdministratorResult {}
unsafe impl Sync for WindowsUpdateGetAdministratorResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateItem, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateItem {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Timestamp)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Category(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Category)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Operation(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Operation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateItem>();
}
unsafe impl windows_core::Interface for WindowsUpdateItem {
    type Vtable = IWindowsUpdateItem_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateItem {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateItem";
}
unsafe impl Send for WindowsUpdateItem {}
unsafe impl Sync for WindowsUpdateItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateManager, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateManager {
    pub fn ScanningStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanningStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScanningStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScanningStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn WorkingStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WorkingStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWorkingStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWorkingStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ProgressChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateProgressChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProgressChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProgressChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProgressChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AttentionRequiredReasonChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateAttentionRequiredReasonChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttentionRequiredReasonChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAttentionRequiredReasonChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAttentionRequiredReasonChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ActionCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateActionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveActionCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveActionCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ScanCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<WindowsUpdateManager, WindowsUpdateScanCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveScanCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveScanCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsScanning(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsScanning)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsWorking(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWorking)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LastSuccessfulScanTimestamp(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastSuccessfulScanTimestamp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetApplicableUpdates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetApplicableUpdates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMostRecentCompletedUpdates(&self, count: i32) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdateItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMostRecentCompletedUpdates)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMostRecentCompletedUpdatesAsync(&self, count: i32) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WindowsUpdateItem>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMostRecentCompletedUpdatesAsync)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartScan(&self, userinitiated: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartScan)(windows_core::Interface::as_raw(this), userinitiated).ok() }
    }
    pub fn CreateInstance(clientid: &windows_core::HSTRING) -> windows_core::Result<WindowsUpdateManager> {
        Self::IWindowsUpdateManagerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(clientid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateManagerFactory<R, F: FnOnce(&IWindowsUpdateManagerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateManager, IWindowsUpdateManagerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateManager>();
}
unsafe impl windows_core::Interface for WindowsUpdateManager {
    type Vtable = IWindowsUpdateManager_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateManager {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateManager";
}
unsafe impl Send for WindowsUpdateManager {}
unsafe impl Sync for WindowsUpdateManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateProgressChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateProgressChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateProgressChangedEventArgs {
    pub fn Update(&self) -> windows_core::Result<WindowsUpdate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActionProgress(&self) -> windows_core::Result<WindowsUpdateActionProgress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionProgress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateProgressChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateProgressChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateProgressChangedEventArgs {
    type Vtable = IWindowsUpdateProgressChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateProgressChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateProgressChangedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateProgressChangedEventArgs";
}
unsafe impl Send for WindowsUpdateProgressChangedEventArgs {}
unsafe impl Sync for WindowsUpdateProgressChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateRestartRequestOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateRestartRequestOptions, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateRestartRequestOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateRestartRequestOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
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
    pub fn MoreInfoUrl(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoreInfoUrl)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMoreInfoUrl<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMoreInfoUrl)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ComplianceDeadlineInDays(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetComplianceDeadlineInDays(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComplianceDeadlineInDays)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ComplianceGracePeriodInDays(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetComplianceGracePeriodInDays(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComplianceGracePeriodInDays)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OrganizationName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrganizationName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetOrganizationName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrganizationName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn OptOutOfAutoReboot(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptOutOfAutoReboot)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOptOutOfAutoReboot(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOptOutOfAutoReboot)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<P0>(title: &windows_core::HSTRING, description: &windows_core::HSTRING, moreinfourl: P0, compliancedeadlineindays: i32, compliancegraceperiodindays: i32) -> windows_core::Result<WindowsUpdateRestartRequestOptions>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IWindowsUpdateRestartRequestOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(title), core::mem::transmute_copy(description), moreinfourl.param().abi(), compliancedeadlineindays, compliancegraceperiodindays, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IWindowsUpdateRestartRequestOptionsFactory<R, F: FnOnce(&IWindowsUpdateRestartRequestOptionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WindowsUpdateRestartRequestOptions, IWindowsUpdateRestartRequestOptionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WindowsUpdateRestartRequestOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateRestartRequestOptions>();
}
unsafe impl windows_core::Interface for WindowsUpdateRestartRequestOptions {
    type Vtable = IWindowsUpdateRestartRequestOptions_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateRestartRequestOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateRestartRequestOptions {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateRestartRequestOptions";
}
unsafe impl Send for WindowsUpdateRestartRequestOptions {}
unsafe impl Sync for WindowsUpdateRestartRequestOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct WindowsUpdateScanCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowsUpdateScanCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowsUpdateScanCompletedEventArgs {
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Updates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<WindowsUpdate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WindowsUpdateScanCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowsUpdateScanCompletedEventArgs>();
}
unsafe impl windows_core::Interface for WindowsUpdateScanCompletedEventArgs {
    type Vtable = IWindowsUpdateScanCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWindowsUpdateScanCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowsUpdateScanCompletedEventArgs {
    const NAME: &'static str = "Windows.Management.Update.WindowsUpdateScanCompletedEventArgs";
}
unsafe impl Send for WindowsUpdateScanCompletedEventArgs {}
unsafe impl Sync for WindowsUpdateScanCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WindowsUpdateAdministratorOptions(pub u32);
impl WindowsUpdateAdministratorOptions {
    pub const None: Self = Self(0u32);
    pub const RequireAdministratorApprovalForScans: Self = Self(1u32);
    pub const RequireAdministratorApprovalForUpdates: Self = Self(2u32);
    pub const RequireAdministratorApprovalForActions: Self = Self(4u32);
}
impl windows_core::TypeKind for WindowsUpdateAdministratorOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WindowsUpdateAdministratorOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministratorOptions").field(&self.0).finish()
    }
}
impl WindowsUpdateAdministratorOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WindowsUpdateAdministratorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WindowsUpdateAdministratorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for WindowsUpdateAdministratorOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WindowsUpdateAdministratorStatus(pub i32);
impl WindowsUpdateAdministratorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoAdministratorRegistered: Self = Self(1i32);
    pub const OtherAdministratorIsRegistered: Self = Self(2i32);
}
impl windows_core::TypeKind for WindowsUpdateAdministratorStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WindowsUpdateAdministratorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministratorStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WindowsUpdateAdministratorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAdministratorStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WindowsUpdateAttentionRequiredReason(pub i32);
impl WindowsUpdateAttentionRequiredReason {
    pub const None: Self = Self(0i32);
    pub const SeekerUpdate: Self = Self(1i32);
    pub const ReadyToReboot: Self = Self(2i32);
    pub const NeedNonMeteredNetwork: Self = Self(3i32);
    pub const NeedUserAgreementForMeteredNetwork: Self = Self(4i32);
    pub const NeedNetwork: Self = Self(5i32);
    pub const NeedMoreSpace: Self = Self(6i32);
    pub const BatterySaverEnabled: Self = Self(7i32);
    pub const NeedUserInteraction: Self = Self(8i32);
    pub const NeedUserAgreementForPolicy: Self = Self(9i32);
    pub const CompatibilityError: Self = Self(10i32);
    pub const NeedUserInteractionForEula: Self = Self(11i32);
    pub const NeedUserInteractionForCta: Self = Self(12i32);
    pub const Regulated: Self = Self(13i32);
    pub const ExternalReboot: Self = Self(14i32);
    pub const OtherUpdate: Self = Self(15i32);
    pub const BlockedByProvider: Self = Self(16i32);
    pub const BlockedByPostRebootFailure: Self = Self(17i32);
    pub const UserEngaged: Self = Self(18i32);
    pub const BlockedByBattery: Self = Self(19i32);
    pub const Exclusivity: Self = Self(20i32);
    pub const BlockedBySerialization: Self = Self(21i32);
    pub const ConflictClass: Self = Self(22i32);
    pub const BlockedByAdminApproval: Self = Self(23i32);
    pub const BlockedByTooManyAttempts: Self = Self(24i32);
    pub const BlockedByFailure: Self = Self(25i32);
    pub const Demotion: Self = Self(26i32);
    pub const BlockedByActiveHours: Self = Self(27i32);
    pub const ScheduledForMaintenance: Self = Self(28i32);
    pub const PolicyScheduledInstallTime: Self = Self(29i32);
    pub const BlockedByOobe: Self = Self(30i32);
    pub const DeferredDuringOobe: Self = Self(31i32);
    pub const DeferredForSustainableTime: Self = Self(32i32);
}
impl windows_core::TypeKind for WindowsUpdateAttentionRequiredReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WindowsUpdateAttentionRequiredReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WindowsUpdateAttentionRequiredReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WindowsUpdateAttentionRequiredReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.WindowsUpdateAttentionRequiredReason;i4)");
}
