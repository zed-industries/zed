#[cfg(feature = "ApplicationModel_Activation")]
pub mod Activation;
#[cfg(feature = "ApplicationModel_AppExtensions")]
pub mod AppExtensions;
#[cfg(feature = "ApplicationModel_AppService")]
pub mod AppService;
#[cfg(feature = "ApplicationModel_Appointments")]
pub mod Appointments;
#[cfg(feature = "ApplicationModel_Background")]
pub mod Background;
#[cfg(feature = "ApplicationModel_Calls")]
pub mod Calls;
#[cfg(feature = "ApplicationModel_Chat")]
pub mod Chat;
#[cfg(feature = "ApplicationModel_CommunicationBlocking")]
pub mod CommunicationBlocking;
#[cfg(feature = "ApplicationModel_Contacts")]
pub mod Contacts;
#[cfg(feature = "ApplicationModel_ConversationalAgent")]
pub mod ConversationalAgent;
#[cfg(feature = "ApplicationModel_Core")]
pub mod Core;
#[cfg(feature = "ApplicationModel_DataTransfer")]
pub mod DataTransfer;
#[cfg(feature = "ApplicationModel_Email")]
pub mod Email;
#[cfg(feature = "ApplicationModel_ExtendedExecution")]
pub mod ExtendedExecution;
#[cfg(feature = "ApplicationModel_Holographic")]
pub mod Holographic;
#[cfg(feature = "ApplicationModel_LockScreen")]
pub mod LockScreen;
#[cfg(feature = "ApplicationModel_Payments")]
pub mod Payments;
#[cfg(feature = "ApplicationModel_Preview")]
pub mod Preview;
#[cfg(feature = "ApplicationModel_Resources")]
pub mod Resources;
#[cfg(feature = "ApplicationModel_Search")]
pub mod Search;
#[cfg(feature = "ApplicationModel_UserActivities")]
pub mod UserActivities;
#[cfg(feature = "ApplicationModel_UserDataAccounts")]
pub mod UserDataAccounts;
#[cfg(feature = "ApplicationModel_UserDataTasks")]
pub mod UserDataTasks;
#[cfg(feature = "ApplicationModel_VoiceCommands")]
pub mod VoiceCommands;
#[cfg(feature = "ApplicationModel_Wallet")]
pub mod Wallet;
windows_core::imp::define_interface!(IAppDisplayInfo, IAppDisplayInfo_Vtbl, 0x1aeb1103_e4d4_41aa_a4f6_c4a276e79eac);
impl windows_core::RuntimeType for IAppDisplayInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppDisplayInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetLogo: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetLogo: usize,
}
windows_core::imp::define_interface!(IAppInfo, IAppInfo_Vtbl, 0xcf7f59b3_6a09_4de8_a6c0_5792d56880d1);
impl windows_core::RuntimeType for IAppInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInfo2, IAppInfo2_Vtbl, 0xbe4b1f5a_2098_431b_bd25_b30878748d47);
impl windows_core::RuntimeType for IAppInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInfo3, IAppInfo3_Vtbl, 0x09a78e46_93a4_46de_9397_0843b57115ea);
impl windows_core::RuntimeType for IAppInfo3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInfo3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExecutionContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppExecutionContext) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInfo4, IAppInfo4_Vtbl, 0x2f34bdeb_1609_4554_9f33_12e1e803e0d4);
impl windows_core::RuntimeType for IAppInfo4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInfo4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SupportedFileExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInfoStatics, IAppInfoStatics_Vtbl, 0xcf1f782a_e48b_4f0c_9b0b_79c3f8957dd7);
impl windows_core::RuntimeType for IAppInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFromAppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetFromAppUserModelIdForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFromAppUserModelIdForUser: usize,
}
windows_core::imp::define_interface!(IAppInstallerInfo, IAppInstallerInfo_Vtbl, 0x29ab2ac0_d4f6_42a3_adcd_d6583c659508);
impl windows_core::RuntimeType for IAppInstallerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallerInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallerInfo2, IAppInstallerInfo2_Vtbl, 0xd20f1388_8256_597c_8511_c84ec50d5e2b);
impl windows_core::RuntimeType for IAppInstallerInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstallerInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnLaunch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HoursBetweenUpdateChecks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ShowPrompt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub UpdateBlocksActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AutomaticBackgroundTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAutoRepairEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PackageVersion) -> windows_core::HRESULT,
    pub LastChecked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::DateTime) -> windows_core::HRESULT,
    pub PausedUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RepairUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RepairUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DependencyPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DependencyPackageUris: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionalPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionalPackageUris: usize,
    pub PolicySource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppInstallerPolicySource) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstance, IAppInstance_Vtbl, 0x675f2b47_f25f_4532_9fd6_3633e0634d01);
impl windows_core::RuntimeType for IAppInstance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstance_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsCurrentInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RedirectActivationTo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstanceStatics, IAppInstanceStatics_Vtbl, 0x9d11e77f_9ea6_47af_a6ec_46784c5ba254);
impl windows_core::RuntimeType for IAppInstanceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAppInstanceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RecommendedInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub GetActivatedEventArgs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    GetActivatedEventArgs: usize,
    pub FindOrRegisterInstanceForKey: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInstances: usize,
}
windows_core::imp::define_interface!(ICameraApplicationManagerStatics, ICameraApplicationManagerStatics_Vtbl, 0x9599ddce_9bd3_435c_8054_c1add50028fe);
impl windows_core::RuntimeType for ICameraApplicationManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICameraApplicationManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShowInstalledApplicationsUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDesignModeStatics, IDesignModeStatics_Vtbl, 0x2c3893cc_f81a_4e7a_b857_76a80887e185);
impl windows_core::RuntimeType for IDesignModeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDesignModeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesignModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDesignModeStatics2, IDesignModeStatics2_Vtbl, 0x80cf8137_b064_4858_bec8_3eba22357535);
impl windows_core::RuntimeType for IDesignModeStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDesignModeStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesignMode2Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnteredBackgroundEventArgs, IEnteredBackgroundEventArgs_Vtbl, 0xf722dcc2_9827_403d_aaed_ecca9ac17398);
impl core::ops::Deref for IEnteredBackgroundEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnteredBackgroundEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IEnteredBackgroundEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IEnteredBackgroundEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEnteredBackgroundEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFindRelatedPackagesOptions, IFindRelatedPackagesOptions_Vtbl, 0x41dd7eea_b335_521f_b96c_5ea07f5b7329);
impl windows_core::RuntimeType for IFindRelatedPackagesOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFindRelatedPackagesOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Relationship: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PackageRelationship) -> windows_core::HRESULT,
    pub SetRelationship: unsafe extern "system" fn(*mut core::ffi::c_void, PackageRelationship) -> windows_core::HRESULT,
    pub IncludeFrameworks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeFrameworks: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncludeHostRuntimes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeHostRuntimes: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncludeOptionals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeOptionals: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IncludeResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeResources: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFindRelatedPackagesOptionsFactory, IFindRelatedPackagesOptionsFactory_Vtbl, 0xd7d17254_a4fd_55c4_98cf_f2710b7d8be2);
impl windows_core::RuntimeType for IFindRelatedPackagesOptionsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFindRelatedPackagesOptionsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, PackageRelationship, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFullTrustProcessLaunchResult, IFullTrustProcessLaunchResult_Vtbl, 0x8917d888_edfb_515f_8e22_5ebceb69dfd9);
impl windows_core::RuntimeType for IFullTrustProcessLaunchResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFullTrustProcessLaunchResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LaunchResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FullTrustLaunchResult) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFullTrustProcessLauncherStatics, IFullTrustProcessLauncherStatics_Vtbl, 0xd784837f_1100_3c6b_a455_f6262cc331b6);
impl windows_core::RuntimeType for IFullTrustProcessLauncherStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFullTrustProcessLauncherStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LaunchFullTrustProcessForCurrentAppAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LaunchFullTrustProcessForCurrentAppWithParametersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LaunchFullTrustProcessForAppAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LaunchFullTrustProcessForAppWithParametersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFullTrustProcessLauncherStatics2, IFullTrustProcessLauncherStatics2_Vtbl, 0x8b8ed72f_b65c_56cf_a1a7_2bf77cbc6ea8);
impl windows_core::RuntimeType for IFullTrustProcessLauncherStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFullTrustProcessLauncherStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LaunchFullTrustProcessForAppWithArgumentsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILeavingBackgroundEventArgs, ILeavingBackgroundEventArgs_Vtbl, 0x39c6ec9a_ae6e_46f9_a07a_cfc23f88733e);
impl core::ops::Deref for ILeavingBackgroundEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILeavingBackgroundEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ILeavingBackgroundEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ILeavingBackgroundEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILeavingBackgroundEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILimitedAccessFeatureRequestResult, ILimitedAccessFeatureRequestResult_Vtbl, 0xd45156a6_1e24_5ddd_abb4_6188aba4d5bf);
impl windows_core::RuntimeType for ILimitedAccessFeatureRequestResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILimitedAccessFeatureRequestResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeatureId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LimitedAccessFeatureStatus) -> windows_core::HRESULT,
    pub EstimatedRemovalDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILimitedAccessFeaturesStatics, ILimitedAccessFeaturesStatics_Vtbl, 0x8be612d4_302b_5fbf_a632_1a99e43e8925);
impl windows_core::RuntimeType for ILimitedAccessFeaturesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILimitedAccessFeaturesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryUnlockFeature: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackage, IPackage_Vtbl, 0x163c792f_bd75_413c_bf23_b1fe7b95d825);
impl windows_core::RuntimeType for IPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub InstalledLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    InstalledLocation: usize,
    pub IsFramework: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Dependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Dependencies: usize,
}
windows_core::imp::define_interface!(IPackage2, IPackage2_Vtbl, 0xa6612fb6_7688_4ace_95fb_359538e7aa01);
impl windows_core::RuntimeType for IPackage2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PublisherDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Logo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsResourcePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBundle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsDevelopmentMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackage3, IPackage3_Vtbl, 0x5f738b61_f86a_4917_93d1_f1ee9d3b35d9);
impl windows_core::RuntimeType for IPackage3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstalledDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::DateTime) -> windows_core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntriesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntriesAsync: usize,
}
windows_core::imp::define_interface!(IPackage4, IPackage4_Vtbl, 0x65aed1ae_b95b_450c_882b_6255187f397e);
impl windows_core::RuntimeType for IPackage4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SignatureKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PackageSignatureKind) -> windows_core::HRESULT,
    pub IsOptional: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub VerifyContentIntegrityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackage5, IPackage5_Vtbl, 0x0e842dd4_d9ac_45ed_9a1e_74ce056b2635);
impl windows_core::RuntimeType for IPackage5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetContentGroupsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetContentGroupsAsync: usize,
    pub GetContentGroupAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StageContentGroupsWithPriorityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StageContentGroupsWithPriorityAsync: usize,
    pub SetInUseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackage6, IPackage6_Vtbl, 0x8b1ad942_12d7_4754_ae4e_638cbc0e3a2e);
impl windows_core::RuntimeType for IPackage6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAppInstallerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckUpdateAvailabilityAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackage7, IPackage7_Vtbl, 0x86ff8d31_a2e4_45e0_9732_283a6d88fde1);
impl windows_core::RuntimeType for IPackage7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub MutableLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MutableLocation: usize,
    #[cfg(feature = "Storage")]
    pub EffectiveLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveLocation: usize,
}
windows_core::imp::define_interface!(IPackage8, IPackage8_Vtbl, 0x2c584f7b_ce2a_4be6_a093_77cfbb2a7ea1);
impl windows_core::RuntimeType for IPackage8 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage8_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub EffectiveExternalLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    EffectiveExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub MachineExternalLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    MachineExternalLocation: usize,
    #[cfg(feature = "Storage")]
    pub UserExternalLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    UserExternalLocation: usize,
    pub InstalledPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EffectivePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub EffectiveExternalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub MachineExternalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserExternalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetLogoAsRandomAccessStreamReference: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetLogoAsRandomAccessStreamReference: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub GetAppListEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections")))]
    GetAppListEntries: usize,
    pub IsStub: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackage9, IPackage9_Vtbl, 0xd5ab224f_d7e1_49ec_90ce_720cdbd02e9c);
impl windows_core::RuntimeType for IPackage9 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackage9_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindRelatedPackages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindRelatedPackages: usize,
    pub SourceUriSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageCatalog, IPackageCatalog_Vtbl, 0x230a3751_9de3_4445_be74_91fb325abefe);
impl windows_core::RuntimeType for IPackageCatalog {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalog_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageStaging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageStaging: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PackageInstalling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageInstalling: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PackageUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageUpdating: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PackageUninstalling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageUninstalling: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub PackageStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageCatalog2, IPackageCatalog2_Vtbl, 0x96a60c36_8ff7_4344_b6bf_ee64c2207ed2);
impl windows_core::RuntimeType for IPackageCatalog2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalog2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageContentGroupStaging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePackageContentGroupStaging: unsafe extern "system" fn(*mut core::ffi::c_void, super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AddOptionalPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageCatalog3, IPackageCatalog3_Vtbl, 0x96dd5c88_8837_43f9_9015_033434ba14f3);
impl windows_core::RuntimeType for IPackageCatalog3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalog3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveOptionalPackagesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveOptionalPackagesAsync: usize,
}
windows_core::imp::define_interface!(IPackageCatalog4, IPackageCatalog4_Vtbl, 0xc37c399b_44cc_4b7b_8baf_796c04ead3b9);
impl windows_core::RuntimeType for IPackageCatalog4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalog4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddResourcePackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, AddResourcePackageOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveResourcePackagesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveResourcePackagesAsync: usize,
}
windows_core::imp::define_interface!(IPackageCatalogAddOptionalPackageResult, IPackageCatalogAddOptionalPackageResult_Vtbl, 0x3bf10cd4_b4df_47b3_a963_e2fa832f7dd3);
impl windows_core::RuntimeType for IPackageCatalogAddOptionalPackageResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalogAddOptionalPackageResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageCatalogAddResourcePackageResult, IPackageCatalogAddResourcePackageResult_Vtbl, 0x9636ce0d_3e17_493f_aa08_ccec6fdef699);
impl windows_core::RuntimeType for IPackageCatalogAddResourcePackageResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalogAddResourcePackageResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageCatalogRemoveOptionalPackagesResult, IPackageCatalogRemoveOptionalPackagesResult_Vtbl, 0x29d2f97b_d974_4e64_9359_22cadfd79828);
impl windows_core::RuntimeType for IPackageCatalogRemoveOptionalPackagesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalogRemoveOptionalPackagesResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageCatalogRemoveResourcePackagesResult, IPackageCatalogRemoveResourcePackagesResult_Vtbl, 0xae719709_1a52_4321_87b3_e5a1a17981a7);
impl windows_core::RuntimeType for IPackageCatalogRemoveResourcePackagesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalogRemoveResourcePackagesResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PackagesRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PackagesRemoved: usize,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageCatalogStatics, IPackageCatalogStatics_Vtbl, 0xa18c9696_e65b_4634_ba21_5e63eb7244a7);
impl windows_core::RuntimeType for IPackageCatalogStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalogStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenForCurrentPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenForCurrentUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageCatalogStatics2, IPackageCatalogStatics2_Vtbl, 0x4c11c159_9a28_598c_b185_55e1899b2be4);
impl core::ops::Deref for IPackageCatalogStatics2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPackageCatalogStatics2, windows_core::IUnknown, windows_core::IInspectable);
impl IPackageCatalogStatics2 {
    pub fn OpenForPackage<P0>(&self, package: P0) -> windows_core::Result<PackageCatalog>
    where
        P0: windows_core::Param<Package>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenForPackage)(windows_core::Interface::as_raw(this), package.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IPackageCatalogStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageCatalogStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenForPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageContentGroup, IPackageContentGroup_Vtbl, 0x8f62695d_120a_4798_b5e1_5800dda8f2e1);
impl windows_core::RuntimeType for IPackageContentGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageContentGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PackageContentGroupState) -> windows_core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageContentGroupStagingEventArgs, IPackageContentGroupStagingEventArgs_Vtbl, 0x3d7bc27e_6f27_446c_986e_d4733d4d9113);
impl windows_core::RuntimeType for IPackageContentGroupStagingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageContentGroupStagingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub ContentGroupName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsContentGroupRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageContentGroupStatics, IPackageContentGroupStatics_Vtbl, 0x70ee7619_5f12_4b92_b9ea_6ccada13bc75);
impl windows_core::RuntimeType for IPackageContentGroupStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageContentGroupStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequiredGroupName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageId, IPackageId_Vtbl, 0x1adb665e_37c7_4790_9980_dd7ae74e8bb2);
impl windows_core::RuntimeType for IPackageId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PackageVersion) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub Architecture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::System::ProcessorArchitecture) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    Architecture: usize,
    pub ResourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Publisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PublisherId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageIdWithMetadata, IPackageIdWithMetadata_Vtbl, 0x40577a7c_0c9e_443d_9074_855f5ce0a08d);
impl windows_core::RuntimeType for IPackageIdWithMetadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageIdWithMetadata_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageInstallingEventArgs, IPackageInstallingEventArgs_Vtbl, 0x97741eb7_ab7a_401a_8b61_eb0e7faff237);
impl windows_core::RuntimeType for IPackageInstallingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageInstallingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageStagingEventArgs, IPackageStagingEventArgs_Vtbl, 0x1041682d_54e2_4f51_b828_9ef7046c210f);
impl windows_core::RuntimeType for IPackageStagingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageStagingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageStatics, IPackageStatics_Vtbl, 0x4e534bdf_2960_4878_97a4_9624deb72f2d);
impl windows_core::RuntimeType for IPackageStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageStatus, IPackageStatus_Vtbl, 0x5fe74f71_a365_4c09_a02d_046d525ea1da);
impl windows_core::RuntimeType for IPackageStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VerifyIsOK: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub NotAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PackageOffline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DataOffline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Disabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub NeedsRemediation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub LicenseIssue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Modified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Tampered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DependencyIssue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Servicing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DeploymentInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageStatus2, IPackageStatus2_Vtbl, 0xf428fa93_7c56_4862_acfa_abaedcc0694d);
impl windows_core::RuntimeType for IPackageStatus2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageStatus2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPartiallyStaged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageStatusChangedEventArgs, IPackageStatusChangedEventArgs_Vtbl, 0x437d714d_bd80_4a70_bc50_f6e796509575);
impl windows_core::RuntimeType for IPackageStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageStatusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageUninstallingEventArgs, IPackageUninstallingEventArgs_Vtbl, 0x4443aa52_ab22_44cd_82bb_4ec9b827367a);
impl windows_core::RuntimeType for IPackageUninstallingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageUninstallingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Package: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageUpdateAvailabilityResult, IPackageUpdateAvailabilityResult_Vtbl, 0x114e5009_199a_48a1_a079_313c45634a71);
impl windows_core::RuntimeType for IPackageUpdateAvailabilityResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageUpdateAvailabilityResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Availability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PackageUpdateAvailability) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageUpdatingEventArgs, IPackageUpdatingEventArgs_Vtbl, 0xcd7b4228_fd74_443e_b114_23e677b0e86f);
impl windows_core::RuntimeType for IPackageUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageUpdatingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SourcePackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetPackage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageWithMetadata, IPackageWithMetadata_Vtbl, 0x95949780_1de9_40f2_b452_0de9f1910012);
impl windows_core::RuntimeType for IPackageWithMetadata {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPackageWithMetadata_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InstallDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::DateTime) -> windows_core::HRESULT,
    pub GetThumbnailToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub Launch: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Launch: usize,
}
windows_core::imp::define_interface!(IStartupTask, IStartupTask_Vtbl, 0xf75c23c8_b5f2_4f6c_88dd_36cb1d599d17);
impl windows_core::RuntimeType for IStartupTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStartupTask_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestEnableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StartupTaskState) -> windows_core::HRESULT,
    pub TaskId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStartupTaskStatics, IStartupTaskStatics_Vtbl, 0xee5b60bd_a148_41a7_b26e_e8b88a1e62f8);
impl windows_core::RuntimeType for IStartupTaskStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStartupTaskStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetForCurrentPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetForCurrentPackageAsync: usize,
    pub GetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISuspendingDeferral, ISuspendingDeferral_Vtbl, 0x59140509_8bc9_4eb4_b636_dabdc4f46f66);
impl core::ops::Deref for ISuspendingDeferral {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISuspendingDeferral, windows_core::IUnknown, windows_core::IInspectable);
impl ISuspendingDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ISuspendingDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISuspendingDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISuspendingEventArgs, ISuspendingEventArgs_Vtbl, 0x96061c05_2dba_4d08_b0bd_2b30a131c6aa);
impl core::ops::Deref for ISuspendingEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISuspendingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ISuspendingEventArgs {
    pub fn SuspendingOperation(&self) -> windows_core::Result<SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuspendingOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ISuspendingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISuspendingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SuspendingOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISuspendingOperation, ISuspendingOperation_Vtbl, 0x9da4ca41_20e1_4e9b_9f65_a9f435340c3a);
impl core::ops::Deref for ISuspendingOperation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISuspendingOperation, windows_core::IUnknown, windows_core::IInspectable);
impl ISuspendingOperation {
    pub fn GetDeferral(&self) -> windows_core::Result<SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ISuspendingOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISuspendingOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Foundation::DateTime) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppDisplayInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppDisplayInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppDisplayInfo {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetLogo(&self, size: super::Foundation::Size) -> windows_core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLogo)(windows_core::Interface::as_raw(this), size, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AppDisplayInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppDisplayInfo>();
}
unsafe impl windows_core::Interface for AppDisplayInfo {
    type Vtable = IAppDisplayInfo_Vtbl;
    const IID: windows_core::GUID = <IAppDisplayInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppDisplayInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppDisplayInfo";
}
unsafe impl Send for AppDisplayInfo {}
unsafe impl Sync for AppDisplayInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppInfo {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppUserModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppUserModelId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<AppDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = &windows_core::Interface::cast::<IAppInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExecutionContext(&self) -> windows_core::Result<AppExecutionContext> {
        let this = &windows_core::Interface::cast::<IAppInfo3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecutionContext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportedFileExtensions(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IAppInfo4>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).SupportedFileExtensions)(windows_core::Interface::as_raw(this), windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn Current() -> windows_core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetFromAppUserModelId(appusermodelid: &windows_core::HSTRING) -> windows_core::Result<AppInfo> {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFromAppUserModelId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appusermodelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetFromAppUserModelIdForUser<P0>(user: P0, appusermodelid: &windows_core::HSTRING) -> windows_core::Result<AppInfo>
    where
        P0: windows_core::Param<super::System::User>,
    {
        Self::IAppInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFromAppUserModelIdForUser)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(appusermodelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppInfoStatics<R, F: FnOnce(&IAppInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppInfo, IAppInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInfo>();
}
unsafe impl windows_core::Interface for AppInfo {
    type Vtable = IAppInfo_Vtbl;
    const IID: windows_core::GUID = <IAppInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppInfo";
}
unsafe impl Send for AppInfo {}
unsafe impl Sync for AppInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppInstallerInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInstallerInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppInstallerInfo {
    pub fn Uri(&self) -> windows_core::Result<super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OnLaunch(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnLaunch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HoursBetweenUpdateChecks(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoursBetweenUpdateChecks)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ShowPrompt(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowPrompt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateBlocksActivation(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateBlocksActivation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AutomaticBackgroundTask(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomaticBackgroundTask)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAutoRepairEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutoRepairEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Version(&self) -> windows_core::Result<PackageVersion> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LastChecked(&self) -> windows_core::Result<super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChecked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PausedUntil(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::DateTime>> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PausedUntil)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateUris(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RepairUris(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepairUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DependencyPackageUris(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DependencyPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionalPackageUris(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PolicySource(&self) -> windows_core::Result<AppInstallerPolicySource> {
        let this = &windows_core::Interface::cast::<IAppInstallerInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PolicySource)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AppInstallerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallerInfo>();
}
unsafe impl windows_core::Interface for AppInstallerInfo {
    type Vtable = IAppInstallerInfo_Vtbl;
    const IID: windows_core::GUID = <IAppInstallerInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.AppInstallerInfo";
}
unsafe impl Send for AppInstallerInfo {}
unsafe impl Sync for AppInstallerInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct AppInstance(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInstance, windows_core::IUnknown, windows_core::IInspectable);
impl AppInstance {
    pub fn Key(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Key)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsCurrentInstance(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCurrentInstance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RedirectActivationTo(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RedirectActivationTo)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RecommendedInstance() -> windows_core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RecommendedInstance)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn GetActivatedEventArgs() -> windows_core::Result<Activation::IActivatedEventArgs> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActivatedEventArgs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FindOrRegisterInstanceForKey(key: &windows_core::HSTRING) -> windows_core::Result<AppInstance> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindOrRegisterInstanceForKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Unregister() -> windows_core::Result<()> {
        Self::IAppInstanceStatics(|this| unsafe { (windows_core::Interface::vtable(this).Unregister)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInstances() -> windows_core::Result<super::Foundation::Collections::IVector<AppInstance>> {
        Self::IAppInstanceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInstances)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAppInstanceStatics<R, F: FnOnce(&IAppInstanceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppInstance, IAppInstanceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppInstance {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInstance>();
}
unsafe impl windows_core::Interface for AppInstance {
    type Vtable = IAppInstance_Vtbl;
    const IID: windows_core::GUID = <IAppInstance as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstance {
    const NAME: &'static str = "Windows.ApplicationModel.AppInstance";
}
unsafe impl Send for AppInstance {}
unsafe impl Sync for AppInstance {}
pub struct CameraApplicationManager;
impl CameraApplicationManager {
    pub fn ShowInstalledApplicationsUI() -> windows_core::Result<()> {
        Self::ICameraApplicationManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ShowInstalledApplicationsUI)(windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ICameraApplicationManagerStatics<R, F: FnOnce(&ICameraApplicationManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CameraApplicationManager, ICameraApplicationManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for CameraApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.CameraApplicationManager";
}
pub struct DesignMode;
impl DesignMode {
    pub fn DesignModeEnabled() -> windows_core::Result<bool> {
        Self::IDesignModeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesignModeEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn DesignMode2Enabled() -> windows_core::Result<bool> {
        Self::IDesignModeStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesignMode2Enabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IDesignModeStatics<R, F: FnOnce(&IDesignModeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DesignMode, IDesignModeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDesignModeStatics2<R, F: FnOnce(&IDesignModeStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DesignMode, IDesignModeStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for DesignMode {
    const NAME: &'static str = "Windows.ApplicationModel.DesignMode";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct EnteredBackgroundEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EnteredBackgroundEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(EnteredBackgroundEventArgs, IEnteredBackgroundEventArgs);
impl EnteredBackgroundEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EnteredBackgroundEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEnteredBackgroundEventArgs>();
}
unsafe impl windows_core::Interface for EnteredBackgroundEventArgs {
    type Vtable = IEnteredBackgroundEventArgs_Vtbl;
    const IID: windows_core::GUID = <IEnteredBackgroundEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.EnteredBackgroundEventArgs";
}
unsafe impl Send for EnteredBackgroundEventArgs {}
unsafe impl Sync for EnteredBackgroundEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FindRelatedPackagesOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FindRelatedPackagesOptions, windows_core::IUnknown, windows_core::IInspectable);
impl FindRelatedPackagesOptions {
    pub fn Relationship(&self) -> windows_core::Result<PackageRelationship> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Relationship)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRelationship(&self, value: PackageRelationship) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRelationship)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeFrameworks(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeFrameworks)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeFrameworks(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeFrameworks)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeHostRuntimes(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeHostRuntimes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeHostRuntimes(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeHostRuntimes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeOptionals(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeOptionals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeOptionals(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeOptionals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeResources(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeResources)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeResources(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeResources)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance(relationship: PackageRelationship) -> windows_core::Result<FindRelatedPackagesOptions> {
        Self::IFindRelatedPackagesOptionsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), relationship, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IFindRelatedPackagesOptionsFactory<R, F: FnOnce(&IFindRelatedPackagesOptionsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FindRelatedPackagesOptions, IFindRelatedPackagesOptionsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FindRelatedPackagesOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFindRelatedPackagesOptions>();
}
unsafe impl windows_core::Interface for FindRelatedPackagesOptions {
    type Vtable = IFindRelatedPackagesOptions_Vtbl;
    const IID: windows_core::GUID = <IFindRelatedPackagesOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FindRelatedPackagesOptions {
    const NAME: &'static str = "Windows.ApplicationModel.FindRelatedPackagesOptions";
}
unsafe impl Send for FindRelatedPackagesOptions {}
unsafe impl Sync for FindRelatedPackagesOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct FullTrustProcessLaunchResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FullTrustProcessLaunchResult, windows_core::IUnknown, windows_core::IInspectable);
impl FullTrustProcessLaunchResult {
    pub fn LaunchResult(&self) -> windows_core::Result<FullTrustLaunchResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchResult)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for FullTrustProcessLaunchResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFullTrustProcessLaunchResult>();
}
unsafe impl windows_core::Interface for FullTrustProcessLaunchResult {
    type Vtable = IFullTrustProcessLaunchResult_Vtbl;
    const IID: windows_core::GUID = <IFullTrustProcessLaunchResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FullTrustProcessLaunchResult {
    const NAME: &'static str = "Windows.ApplicationModel.FullTrustProcessLaunchResult";
}
unsafe impl Send for FullTrustProcessLaunchResult {}
unsafe impl Sync for FullTrustProcessLaunchResult {}
pub struct FullTrustProcessLauncher;
impl FullTrustProcessLauncher {
    pub fn LaunchFullTrustProcessForCurrentAppAsync() -> windows_core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LaunchFullTrustProcessForCurrentAppWithParametersAsync(parametergroupid: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppWithParametersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(parametergroupid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LaunchFullTrustProcessForAppAsync(fulltrustpackagerelativeappid: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchFullTrustProcessForAppAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(fulltrustpackagerelativeappid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LaunchFullTrustProcessForAppWithParametersAsync(fulltrustpackagerelativeappid: &windows_core::HSTRING, parametergroupid: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::IAsyncAction> {
        Self::IFullTrustProcessLauncherStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchFullTrustProcessForAppWithParametersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(fulltrustpackagerelativeappid), core::mem::transmute_copy(parametergroupid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(commandline: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>> {
        Self::IFullTrustProcessLauncherStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchFullTrustProcessForCurrentAppWithArgumentsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(commandline), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LaunchFullTrustProcessForAppWithArgumentsAsync(fulltrustpackagerelativeappid: &windows_core::HSTRING, commandline: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>> {
        Self::IFullTrustProcessLauncherStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LaunchFullTrustProcessForAppWithArgumentsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(fulltrustpackagerelativeappid), core::mem::transmute_copy(commandline), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IFullTrustProcessLauncherStatics<R, F: FnOnce(&IFullTrustProcessLauncherStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FullTrustProcessLauncher, IFullTrustProcessLauncherStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFullTrustProcessLauncherStatics2<R, F: FnOnce(&IFullTrustProcessLauncherStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FullTrustProcessLauncher, IFullTrustProcessLauncherStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for FullTrustProcessLauncher {
    const NAME: &'static str = "Windows.ApplicationModel.FullTrustProcessLauncher";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LeavingBackgroundEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LeavingBackgroundEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LeavingBackgroundEventArgs, ILeavingBackgroundEventArgs);
impl LeavingBackgroundEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LeavingBackgroundEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILeavingBackgroundEventArgs>();
}
unsafe impl windows_core::Interface for LeavingBackgroundEventArgs {
    type Vtable = ILeavingBackgroundEventArgs_Vtbl;
    const IID: windows_core::GUID = <ILeavingBackgroundEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.LeavingBackgroundEventArgs";
}
unsafe impl Send for LeavingBackgroundEventArgs {}
unsafe impl Sync for LeavingBackgroundEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LimitedAccessFeatureRequestResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LimitedAccessFeatureRequestResult, windows_core::IUnknown, windows_core::IInspectable);
impl LimitedAccessFeatureRequestResult {
    pub fn FeatureId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FeatureId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<LimitedAccessFeatureStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EstimatedRemovalDate(&self) -> windows_core::Result<super::Foundation::IReference<super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EstimatedRemovalDate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LimitedAccessFeatureRequestResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILimitedAccessFeatureRequestResult>();
}
unsafe impl windows_core::Interface for LimitedAccessFeatureRequestResult {
    type Vtable = ILimitedAccessFeatureRequestResult_Vtbl;
    const IID: windows_core::GUID = <ILimitedAccessFeatureRequestResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LimitedAccessFeatureRequestResult {
    const NAME: &'static str = "Windows.ApplicationModel.LimitedAccessFeatureRequestResult";
}
unsafe impl Send for LimitedAccessFeatureRequestResult {}
unsafe impl Sync for LimitedAccessFeatureRequestResult {}
pub struct LimitedAccessFeatures;
impl LimitedAccessFeatures {
    pub fn TryUnlockFeature(featureid: &windows_core::HSTRING, token: &windows_core::HSTRING, attestation: &windows_core::HSTRING) -> windows_core::Result<LimitedAccessFeatureRequestResult> {
        Self::ILimitedAccessFeaturesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUnlockFeature)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(featureid), core::mem::transmute_copy(token), core::mem::transmute_copy(attestation), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILimitedAccessFeaturesStatics<R, F: FnOnce(&ILimitedAccessFeaturesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LimitedAccessFeatures, ILimitedAccessFeaturesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for LimitedAccessFeatures {
    const NAME: &'static str = "Windows.ApplicationModel.LimitedAccessFeatures";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Package(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Package, windows_core::IUnknown, windows_core::IInspectable);
impl Package {
    pub fn Id(&self) -> windows_core::Result<PackageId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn InstalledLocation(&self) -> windows_core::Result<super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstalledLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsFramework(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFramework)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Dependencies(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dependencies)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PublisherDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublisherDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Logo(&self) -> windows_core::Result<super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Logo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsResourcePackage(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsResourcePackage)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBundle(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBundle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsDevelopmentMode(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDevelopmentMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<PackageStatus> {
        let this = &windows_core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstalledDate(&self) -> windows_core::Result<super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstalledDate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub fn GetAppListEntriesAsync(&self) -> windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<Core::AppListEntry>>> {
        let this = &windows_core::Interface::cast::<IPackage3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppListEntriesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SignatureKind(&self) -> windows_core::Result<PackageSignatureKind> {
        let this = &windows_core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignatureKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOptional(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOptional)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VerifyContentIntegrityAsync(&self) -> windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IPackage4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerifyContentIntegrityAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetContentGroupsAsync(&self) -> windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>> {
        let this = &windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContentGroupsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetContentGroupAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::IAsyncOperation<PackageContentGroup>> {
        let this = &windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContentGroupAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StageContentGroupsAsync<P0>(&self, names: P0) -> windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>
    where
        P0: windows_core::Param<super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StageContentGroupsAsync)(windows_core::Interface::as_raw(this), names.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StageContentGroupsWithPriorityAsync<P0>(&self, names: P0, movetoheadofqueue: bool) -> windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>
    where
        P0: windows_core::Param<super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StageContentGroupsWithPriorityAsync)(windows_core::Interface::as_raw(this), names.param().abi(), movetoheadofqueue, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInUseAsync(&self, inuse: bool) -> windows_core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IPackage5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetInUseAsync)(windows_core::Interface::as_raw(this), inuse, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAppInstallerInfo(&self) -> windows_core::Result<AppInstallerInfo> {
        let this = &windows_core::Interface::cast::<IPackage6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppInstallerInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CheckUpdateAvailabilityAsync(&self) -> windows_core::Result<super::Foundation::IAsyncOperation<PackageUpdateAvailabilityResult>> {
        let this = &windows_core::Interface::cast::<IPackage6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CheckUpdateAvailabilityAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn MutableLocation(&self) -> windows_core::Result<super::Storage::StorageFolder> {
        let this = &windows_core::Interface::cast::<IPackage7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MutableLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn EffectiveLocation(&self) -> windows_core::Result<super::Storage::StorageFolder> {
        let this = &windows_core::Interface::cast::<IPackage7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectiveLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn EffectiveExternalLocation(&self) -> windows_core::Result<super::Storage::StorageFolder> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectiveExternalLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn MachineExternalLocation(&self) -> windows_core::Result<super::Storage::StorageFolder> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MachineExternalLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn UserExternalLocation(&self) -> windows_core::Result<super::Storage::StorageFolder> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserExternalLocation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstalledPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstalledPath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MutablePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MutablePath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EffectivePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectivePath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EffectiveExternalPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectiveExternalPath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MachineExternalPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MachineExternalPath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserExternalPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserExternalPath)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetLogoAsRandomAccessStreamReference(&self, size: super::Foundation::Size) -> windows_core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLogoAsRandomAccessStreamReference)(windows_core::Interface::as_raw(this), size, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation_Collections"))]
    pub fn GetAppListEntries(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<Core::AppListEntry>> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAppListEntries)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsStub(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackage8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsStub)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindRelatedPackages<P0>(&self, options: P0) -> windows_core::Result<super::Foundation::Collections::IVector<Package>>
    where
        P0: windows_core::Param<FindRelatedPackagesOptions>,
    {
        let this = &windows_core::Interface::cast::<IPackage9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindRelatedPackages)(windows_core::Interface::as_raw(this), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SourceUriSchemeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackage9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceUriSchemeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Current() -> windows_core::Result<Package> {
        Self::IPackageStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn InstallDate(&self) -> windows_core::Result<super::Foundation::DateTime> {
        let this = &windows_core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallDate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetThumbnailToken(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetThumbnailToken)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Launch(&self, parameters: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPackageWithMetadata>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Launch)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(parameters)).ok() }
    }
    #[doc(hidden)]
    pub fn IPackageStatics<R, F: FnOnce(&IPackageStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Package, IPackageStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Package {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackage>();
}
unsafe impl windows_core::Interface for Package {
    type Vtable = IPackage_Vtbl;
    const IID: windows_core::GUID = <IPackage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Package {
    const NAME: &'static str = "Windows.ApplicationModel.Package";
}
unsafe impl Send for Package {}
unsafe impl Sync for Package {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageCatalog(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageCatalog, windows_core::IUnknown, windows_core::IInspectable);
impl PackageCatalog {
    pub fn PackageStaging<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageStaging)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageStaging(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageStaging)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageInstalling<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageInstalling)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageInstalling(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageInstalling)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageUpdating<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageUpdating)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageUpdating(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageUpdating)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageUninstalling<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageUninstalling)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageUninstalling(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageUninstalling)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageStatusChanged<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageStatusChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageStatusChanged(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageStatusChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PackageContentGroupStaging<P0>(&self, handler: P0) -> windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::Foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageContentGroupStaging)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageContentGroupStaging(&self, token: super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePackageContentGroupStaging)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AddOptionalPackageAsync(&self, optionalpackagefamilyname: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>> {
        let this = &windows_core::Interface::cast::<IPackageCatalog2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddOptionalPackageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(optionalpackagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveOptionalPackagesAsync<P0>(&self, optionalpackagefamilynames: P0) -> windows_core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>>
    where
        P0: windows_core::Param<super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IPackageCatalog3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveOptionalPackagesAsync)(windows_core::Interface::as_raw(this), optionalpackagefamilynames.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddResourcePackageAsync(&self, resourcepackagefamilyname: &windows_core::HSTRING, resourceid: &windows_core::HSTRING, options: AddResourcePackageOptions) -> windows_core::Result<super::Foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>> {
        let this = &windows_core::Interface::cast::<IPackageCatalog4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddResourcePackageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(resourcepackagefamilyname), core::mem::transmute_copy(resourceid), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveResourcePackagesAsync<P0>(&self, resourcepackages: P0) -> windows_core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>>
    where
        P0: windows_core::Param<super::Foundation::Collections::IIterable<Package>>,
    {
        let this = &windows_core::Interface::cast::<IPackageCatalog4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveResourcePackagesAsync)(windows_core::Interface::as_raw(this), resourcepackages.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenForCurrentPackage() -> windows_core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenForCurrentPackage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OpenForCurrentUser() -> windows_core::Result<PackageCatalog> {
        Self::IPackageCatalogStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenForCurrentUser)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OpenForPackage<P0>(package: P0) -> windows_core::Result<PackageCatalog>
    where
        P0: windows_core::Param<Package>,
    {
        Self::IPackageCatalogStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenForPackage)(windows_core::Interface::as_raw(this), package.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPackageCatalogStatics<R, F: FnOnce(&IPackageCatalogStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PackageCatalog, IPackageCatalogStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPackageCatalogStatics2<R, F: FnOnce(&IPackageCatalogStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PackageCatalog, IPackageCatalogStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PackageCatalog {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageCatalog>();
}
unsafe impl windows_core::Interface for PackageCatalog {
    type Vtable = IPackageCatalog_Vtbl;
    const IID: windows_core::GUID = <IPackageCatalog as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalog";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageCatalogAddOptionalPackageResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageCatalogAddOptionalPackageResult, windows_core::IUnknown, windows_core::IInspectable);
impl PackageCatalogAddOptionalPackageResult {
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PackageCatalogAddOptionalPackageResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageCatalogAddOptionalPackageResult>();
}
unsafe impl windows_core::Interface for PackageCatalogAddOptionalPackageResult {
    type Vtable = IPackageCatalogAddOptionalPackageResult_Vtbl;
    const IID: windows_core::GUID = <IPackageCatalogAddOptionalPackageResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageCatalogAddOptionalPackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageCatalogAddResourcePackageResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageCatalogAddResourcePackageResult, windows_core::IUnknown, windows_core::IInspectable);
impl PackageCatalogAddResourcePackageResult {
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for PackageCatalogAddResourcePackageResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageCatalogAddResourcePackageResult>();
}
unsafe impl windows_core::Interface for PackageCatalogAddResourcePackageResult {
    type Vtable = IPackageCatalogAddResourcePackageResult_Vtbl;
    const IID: windows_core::GUID = <IPackageCatalogAddResourcePackageResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageCatalogAddResourcePackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogAddResourcePackageResult";
}
unsafe impl Send for PackageCatalogAddResourcePackageResult {}
unsafe impl Sync for PackageCatalogAddResourcePackageResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageCatalogRemoveOptionalPackagesResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageCatalogRemoveOptionalPackagesResult, windows_core::IUnknown, windows_core::IInspectable);
impl PackageCatalogRemoveOptionalPackagesResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn PackagesRemoved(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackagesRemoved)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PackageCatalogRemoveOptionalPackagesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageCatalogRemoveOptionalPackagesResult>();
}
unsafe impl windows_core::Interface for PackageCatalogRemoveOptionalPackagesResult {
    type Vtable = IPackageCatalogRemoveOptionalPackagesResult_Vtbl;
    const IID: windows_core::GUID = <IPackageCatalogRemoveOptionalPackagesResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageCatalogRemoveOptionalPackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageCatalogRemoveResourcePackagesResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageCatalogRemoveResourcePackagesResult, windows_core::IUnknown, windows_core::IInspectable);
impl PackageCatalogRemoveResourcePackagesResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn PackagesRemoved(&self) -> windows_core::Result<super::Foundation::Collections::IVectorView<Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackagesRemoved)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
impl windows_core::RuntimeType for PackageCatalogRemoveResourcePackagesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageCatalogRemoveResourcePackagesResult>();
}
unsafe impl windows_core::Interface for PackageCatalogRemoveResourcePackagesResult {
    type Vtable = IPackageCatalogRemoveResourcePackagesResult_Vtbl;
    const IID: windows_core::GUID = <IPackageCatalogRemoveResourcePackagesResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageCatalogRemoveResourcePackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult";
}
unsafe impl Send for PackageCatalogRemoveResourcePackagesResult {}
unsafe impl Sync for PackageCatalogRemoveResourcePackagesResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageContentGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageContentGroup, windows_core::IUnknown, windows_core::IInspectable);
impl PackageContentGroup {
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<PackageContentGroupState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RequiredGroupName() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPackageContentGroupStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiredGroupName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPackageContentGroupStatics<R, F: FnOnce(&IPackageContentGroupStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PackageContentGroup, IPackageContentGroupStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PackageContentGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageContentGroup>();
}
unsafe impl windows_core::Interface for PackageContentGroup {
    type Vtable = IPackageContentGroup_Vtbl;
    const IID: windows_core::GUID = <IPackageContentGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageContentGroup {
    const NAME: &'static str = "Windows.ApplicationModel.PackageContentGroup";
}
unsafe impl Send for PackageContentGroup {}
unsafe impl Sync for PackageContentGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageContentGroupStagingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageContentGroupStagingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageContentGroupStagingEventArgs {
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContentGroupName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentGroupName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsContentGroupRequired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsContentGroupRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PackageContentGroupStagingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageContentGroupStagingEventArgs>();
}
unsafe impl windows_core::Interface for PackageContentGroupStagingEventArgs {
    type Vtable = IPackageContentGroupStagingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageContentGroupStagingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageContentGroupStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageContentGroupStagingEventArgs";
}
unsafe impl Send for PackageContentGroupStagingEventArgs {}
unsafe impl Sync for PackageContentGroupStagingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageId(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageId, windows_core::IUnknown, windows_core::IInspectable);
impl PackageId {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Version(&self) -> windows_core::Result<PackageVersion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn Architecture(&self) -> windows_core::Result<super::System::ProcessorArchitecture> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Architecture)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResourceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResourceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Publisher(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Publisher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PublisherId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublisherId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FullName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FamilyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProductId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackageIdWithMetadata>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProductId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Author(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPackageIdWithMetadata>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Author)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageId>();
}
unsafe impl windows_core::Interface for PackageId {
    type Vtable = IPackageId_Vtbl;
    const IID: windows_core::GUID = <IPackageId as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageId {
    const NAME: &'static str = "Windows.ApplicationModel.PackageId";
}
unsafe impl Send for PackageId {}
unsafe impl Sync for PackageId {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageInstallingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageInstallingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageInstallingEventArgs {
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PackageInstallingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageInstallingEventArgs>();
}
unsafe impl windows_core::Interface for PackageInstallingEventArgs {
    type Vtable = IPackageInstallingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageInstallingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageInstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageInstallingEventArgs";
}
unsafe impl Send for PackageInstallingEventArgs {}
unsafe impl Sync for PackageInstallingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageStagingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageStagingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageStagingEventArgs {
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PackageStagingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageStagingEventArgs>();
}
unsafe impl windows_core::Interface for PackageStagingEventArgs {
    type Vtable = IPackageStagingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageStagingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStagingEventArgs";
}
unsafe impl Send for PackageStagingEventArgs {}
unsafe impl Sync for PackageStagingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageStatus, windows_core::IUnknown, windows_core::IInspectable);
impl PackageStatus {
    pub fn VerifyIsOK(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerifyIsOK)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NotAvailable(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PackageOffline(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageOffline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DataOffline(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataOffline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Disabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Disabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NeedsRemediation(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NeedsRemediation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LicenseIssue(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LicenseIssue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Modified(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Modified)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Tampered(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tampered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DependencyIssue(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DependencyIssue)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Servicing(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Servicing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeploymentInProgress(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeploymentInProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPartiallyStaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackageStatus2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPartiallyStaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PackageStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageStatus>();
}
unsafe impl windows_core::Interface for PackageStatus {
    type Vtable = IPackageStatus_Vtbl;
    const IID: windows_core::GUID = <IPackageStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageStatus {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStatus";
}
unsafe impl Send for PackageStatus {}
unsafe impl Sync for PackageStatus {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageStatusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageStatusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageStatusChangedEventArgs {
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageStatusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageStatusChangedEventArgs>();
}
unsafe impl windows_core::Interface for PackageStatusChangedEventArgs {
    type Vtable = IPackageStatusChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageStatusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageStatusChangedEventArgs";
}
unsafe impl Send for PackageStatusChangedEventArgs {}
unsafe impl Sync for PackageStatusChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageUninstallingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageUninstallingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageUninstallingEventArgs {
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Package(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Package)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PackageUninstallingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageUninstallingEventArgs>();
}
unsafe impl windows_core::Interface for PackageUninstallingEventArgs {
    type Vtable = IPackageUninstallingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageUninstallingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUninstallingEventArgs";
}
unsafe impl Send for PackageUninstallingEventArgs {}
unsafe impl Sync for PackageUninstallingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageUpdateAvailabilityResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageUpdateAvailabilityResult, windows_core::IUnknown, windows_core::IInspectable);
impl PackageUpdateAvailabilityResult {
    pub fn Availability(&self) -> windows_core::Result<PackageUpdateAvailability> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Availability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for PackageUpdateAvailabilityResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageUpdateAvailabilityResult>();
}
unsafe impl windows_core::Interface for PackageUpdateAvailabilityResult {
    type Vtable = IPackageUpdateAvailabilityResult_Vtbl;
    const IID: windows_core::GUID = <IPackageUpdateAvailabilityResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageUpdateAvailabilityResult {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUpdateAvailabilityResult";
}
unsafe impl Send for PackageUpdateAvailabilityResult {}
unsafe impl Sync for PackageUpdateAvailabilityResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PackageUpdatingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageUpdatingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PackageUpdatingEventArgs {
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SourcePackage(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourcePackage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TargetPackage(&self) -> windows_core::Result<Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetPackage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Progress(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PackageUpdatingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageUpdatingEventArgs>();
}
unsafe impl windows_core::Interface for PackageUpdatingEventArgs {
    type Vtable = IPackageUpdatingEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPackageUpdatingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.PackageUpdatingEventArgs";
}
unsafe impl Send for PackageUpdatingEventArgs {}
unsafe impl Sync for PackageUpdatingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct StartupTask(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StartupTask, windows_core::IUnknown, windows_core::IInspectable);
impl StartupTask {
    pub fn RequestEnableAsync(&self) -> windows_core::Result<super::Foundation::IAsyncOperation<StartupTaskState>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestEnableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Disable(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Disable)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn State(&self) -> windows_core::Result<StartupTaskState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TaskId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetForCurrentPackageAsync() -> windows_core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StartupTask>>> {
        Self::IStartupTaskStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentPackageAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetAsync(taskid: &windows_core::HSTRING) -> windows_core::Result<super::Foundation::IAsyncOperation<StartupTask>> {
        Self::IStartupTaskStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(taskid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStartupTaskStatics<R, F: FnOnce(&IStartupTaskStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StartupTask, IStartupTaskStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StartupTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStartupTask>();
}
unsafe impl windows_core::Interface for StartupTask {
    type Vtable = IStartupTask_Vtbl;
    const IID: windows_core::GUID = <IStartupTask as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StartupTask {
    const NAME: &'static str = "Windows.ApplicationModel.StartupTask";
}
unsafe impl Send for StartupTask {}
unsafe impl Sync for StartupTask {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SuspendingDeferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SuspendingDeferral, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SuspendingDeferral, ISuspendingDeferral);
impl SuspendingDeferral {
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for SuspendingDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISuspendingDeferral>();
}
unsafe impl windows_core::Interface for SuspendingDeferral {
    type Vtable = ISuspendingDeferral_Vtbl;
    const IID: windows_core::GUID = <ISuspendingDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingDeferral";
}
unsafe impl Send for SuspendingDeferral {}
unsafe impl Sync for SuspendingDeferral {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SuspendingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SuspendingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SuspendingEventArgs, ISuspendingEventArgs);
impl SuspendingEventArgs {
    pub fn SuspendingOperation(&self) -> windows_core::Result<SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuspendingOperation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SuspendingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISuspendingEventArgs>();
}
unsafe impl windows_core::Interface for SuspendingEventArgs {
    type Vtable = ISuspendingEventArgs_Vtbl;
    const IID: windows_core::GUID = <ISuspendingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingEventArgs";
}
unsafe impl Send for SuspendingEventArgs {}
unsafe impl Sync for SuspendingEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SuspendingOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SuspendingOperation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SuspendingOperation, ISuspendingOperation);
impl SuspendingOperation {
    pub fn GetDeferral(&self) -> windows_core::Result<SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Deadline(&self) -> windows_core::Result<super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Deadline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SuspendingOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISuspendingOperation>();
}
unsafe impl windows_core::Interface for SuspendingOperation {
    type Vtable = ISuspendingOperation_Vtbl;
    const IID: windows_core::GUID = <ISuspendingOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.ApplicationModel.SuspendingOperation";
}
unsafe impl Send for SuspendingOperation {}
unsafe impl Sync for SuspendingOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AddResourcePackageOptions(pub u32);
impl AddResourcePackageOptions {
    pub const None: Self = Self(0u32);
    pub const ForceTargetAppShutdown: Self = Self(1u32);
    pub const ApplyUpdateIfAvailable: Self = Self(2u32);
}
impl windows_core::TypeKind for AddResourcePackageOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AddResourcePackageOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AddResourcePackageOptions").field(&self.0).finish()
    }
}
impl AddResourcePackageOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AddResourcePackageOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AddResourcePackageOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AddResourcePackageOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AddResourcePackageOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AddResourcePackageOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for AddResourcePackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AddResourcePackageOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppExecutionContext(pub i32);
impl AppExecutionContext {
    pub const Unknown: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Guest: Self = Self(2i32);
}
impl windows_core::TypeKind for AppExecutionContext {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppExecutionContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppExecutionContext").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppExecutionContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppExecutionContext;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AppInstallerPolicySource(pub i32);
impl AppInstallerPolicySource {
    pub const Default: Self = Self(0i32);
    pub const System: Self = Self(1i32);
}
impl windows_core::TypeKind for AppInstallerPolicySource {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AppInstallerPolicySource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AppInstallerPolicySource").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AppInstallerPolicySource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.AppInstallerPolicySource;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FullTrustLaunchResult(pub i32);
impl FullTrustLaunchResult {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl windows_core::TypeKind for FullTrustLaunchResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FullTrustLaunchResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FullTrustLaunchResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FullTrustLaunchResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.FullTrustLaunchResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LimitedAccessFeatureStatus(pub i32);
impl LimitedAccessFeatureStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
    pub const AvailableWithoutToken: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl windows_core::TypeKind for LimitedAccessFeatureStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LimitedAccessFeatureStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LimitedAccessFeatureStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LimitedAccessFeatureStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.LimitedAccessFeatureStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackageContentGroupState(pub i32);
impl PackageContentGroupState {
    pub const NotStaged: Self = Self(0i32);
    pub const Queued: Self = Self(1i32);
    pub const Staging: Self = Self(2i32);
    pub const Staged: Self = Self(3i32);
}
impl windows_core::TypeKind for PackageContentGroupState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackageContentGroupState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackageContentGroupState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PackageContentGroupState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageContentGroupState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackageRelationship(pub i32);
impl PackageRelationship {
    pub const Dependencies: Self = Self(0i32);
    pub const Dependents: Self = Self(1i32);
    pub const All: Self = Self(2i32);
}
impl windows_core::TypeKind for PackageRelationship {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackageRelationship {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackageRelationship").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PackageRelationship {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageRelationship;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackageSignatureKind(pub i32);
impl PackageSignatureKind {
    pub const None: Self = Self(0i32);
    pub const Developer: Self = Self(1i32);
    pub const Enterprise: Self = Self(2i32);
    pub const Store: Self = Self(3i32);
    pub const System: Self = Self(4i32);
}
impl windows_core::TypeKind for PackageSignatureKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackageSignatureKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackageSignatureKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PackageSignatureKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageSignatureKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PackageUpdateAvailability(pub i32);
impl PackageUpdateAvailability {
    pub const Unknown: Self = Self(0i32);
    pub const NoUpdates: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Required: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl windows_core::TypeKind for PackageUpdateAvailability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PackageUpdateAvailability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PackageUpdateAvailability").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PackageUpdateAvailability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.PackageUpdateAvailability;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StartupTaskState(pub i32);
impl StartupTaskState {
    pub const Disabled: Self = Self(0i32);
    pub const DisabledByUser: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const EnabledByPolicy: Self = Self(4i32);
}
impl windows_core::TypeKind for StartupTaskState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StartupTaskState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StartupTaskState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for StartupTaskState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.StartupTaskState;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackageInstallProgress {
    pub PercentComplete: u32,
}
impl windows_core::TypeKind for PackageInstallProgress {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PackageInstallProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.PackageInstallProgress;u4)");
}
impl Default for PackageInstallProgress {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackageVersion {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl windows_core::TypeKind for PackageVersion {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PackageVersion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.PackageVersion;u2;u2;u2;u2)");
}
impl Default for PackageVersion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
