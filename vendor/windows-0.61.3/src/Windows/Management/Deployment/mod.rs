#[cfg(feature = "Management_Deployment_Preview")]
pub mod Preview;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AddPackageByAppInstallerOptions(pub u32);
impl AddPackageByAppInstallerOptions {
    pub const None: Self = Self(0u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetAppShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const LimitToExistingPackages: Self = Self(512u32);
}
impl windows_core::TypeKind for AddPackageByAppInstallerOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AddPackageByAppInstallerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.AddPackageByAppInstallerOptions;u4)");
}
impl AddPackageByAppInstallerOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AddPackageByAppInstallerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AddPackageByAppInstallerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddPackageOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AddPackageOptions, windows_core::IUnknown, windows_core::IInspectable);
impl AddPackageOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AddPackageOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DependencyPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DependencyPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TargetVolume(&self) -> windows_core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetVolume)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTargetVolume<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PackageVolume>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTargetVolume)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OptionalPackageFamilyNames(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalPackageFamilyNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OptionalPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RelatedPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelatedPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExternalLocationUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExternalLocationUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExternalLocationUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExternalLocationUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StubPackageOption(&self) -> windows_core::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StubPackageOption)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStubPackageOption)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeveloperMode(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeveloperMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeveloperMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceAppShutdown(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceAppShutdown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceAppShutdown)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceTargetAppShutdown(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceTargetAppShutdown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceTargetAppShutdown)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallAllResources)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInstallAllResources)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequiredContentGroupOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiredContentGroupOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequiredContentGroupOnly)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RetainFilesOnFailure(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetainFilesOnFailure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRetainFilesOnFailure(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRetainFilesOnFailure)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StageInPlace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStageInPlace)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowUnsigned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowUnsigned)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeferRegistrationWhenPackagesAreInUse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeferRegistrationWhenPackagesAreInUse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExpectedDigests(&self) -> windows_core::Result<windows_collections::IMap<super::super::Foundation::Uri, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IAddPackageOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpectedDigests)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LimitToExistingPackages(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAddPackageOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LimitToExistingPackages)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLimitToExistingPackages(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAddPackageOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLimitToExistingPackages)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for AddPackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAddPackageOptions>();
}
unsafe impl windows_core::Interface for AddPackageOptions {
    type Vtable = <IAddPackageOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAddPackageOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AddPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AddPackageOptions";
}
unsafe impl Send for AddPackageOptions {}
unsafe impl Sync for AddPackageOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppInstallerManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppInstallerManager, windows_core::IUnknown, windows_core::IInspectable);
impl AppInstallerManager {
    pub fn SetAutoUpdateSettings<P1>(&self, packagefamilyname: &windows_core::HSTRING, appinstallerinfo: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<AutoUpdateSettingsOptions>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoUpdateSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), appinstallerinfo.param().abi()).ok() }
    }
    pub fn ClearAutoUpdateSettings(&self, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearAutoUpdateSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname)).ok() }
    }
    pub fn PauseAutoUpdatesUntil(&self, packagefamilyname: &windows_core::HSTRING, datetime: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PauseAutoUpdatesUntil)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), datetime).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForSystem() -> windows_core::Result<AppInstallerManager> {
        Self::IAppInstallerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAppInstallerManagerStatics<R, F: FnOnce(&IAppInstallerManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppInstallerManager, IAppInstallerManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppInstallerManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppInstallerManager>();
}
unsafe impl windows_core::Interface for AppInstallerManager {
    type Vtable = <IAppInstallerManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppInstallerManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppInstallerManager {
    const NAME: &'static str = "Windows.Management.Deployment.AppInstallerManager";
}
unsafe impl Send for AppInstallerManager {}
unsafe impl Sync for AppInstallerManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutoUpdateSettingsOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AutoUpdateSettingsOptions, windows_core::IUnknown, windows_core::IInspectable);
impl AutoUpdateSettingsOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AutoUpdateSettingsOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Version(&self) -> windows_core::Result<super::super::ApplicationModel::PackageVersion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn SetVersion(&self, value: super::super::ApplicationModel::PackageVersion) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVersion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppInstallerUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppInstallerUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppInstallerUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppInstallerUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OnLaunch(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OnLaunch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOnLaunch(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOnLaunch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HoursBetweenUpdateChecks(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HoursBetweenUpdateChecks)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHoursBetweenUpdateChecks(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHoursBetweenUpdateChecks)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowPrompt(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowPrompt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShowPrompt(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShowPrompt)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateBlocksActivation(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateBlocksActivation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUpdateBlocksActivation(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUpdateBlocksActivation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticBackgroundTask(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutomaticBackgroundTask)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutomaticBackgroundTask(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutomaticBackgroundTask)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutoRepairEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAutoRepairEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAutoRepairEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsAutoRepairEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RepairUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepairUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DependencyPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DependencyPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OptionalPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn CreateFromAppInstallerInfo<P0>(appinstallerinfo: P0) -> windows_core::Result<AutoUpdateSettingsOptions>
    where
        P0: windows_core::Param<super::super::ApplicationModel::AppInstallerInfo>,
    {
        Self::IAutoUpdateSettingsOptionsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromAppInstallerInfo)(windows_core::Interface::as_raw(this), appinstallerinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAutoUpdateSettingsOptionsStatics<R, F: FnOnce(&IAutoUpdateSettingsOptionsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AutoUpdateSettingsOptions, IAutoUpdateSettingsOptionsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AutoUpdateSettingsOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAutoUpdateSettingsOptions>();
}
unsafe impl windows_core::Interface for AutoUpdateSettingsOptions {
    type Vtable = <IAutoUpdateSettingsOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAutoUpdateSettingsOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AutoUpdateSettingsOptions {
    const NAME: &'static str = "Windows.Management.Deployment.AutoUpdateSettingsOptions";
}
unsafe impl Send for AutoUpdateSettingsOptions {}
unsafe impl Sync for AutoUpdateSettingsOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateSharedPackageContainerOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CreateSharedPackageContainerOptions, windows_core::IUnknown, windows_core::IInspectable);
impl CreateSharedPackageContainerOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CreateSharedPackageContainerOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Members(&self) -> windows_core::Result<windows_collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Members)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ForceAppShutdown(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceAppShutdown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceAppShutdown)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateCollisionOption(&self) -> windows_core::Result<SharedPackageContainerCreationCollisionOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCollisionOption)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCreateCollisionOption(&self, value: SharedPackageContainerCreationCollisionOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCreateCollisionOption)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CreateSharedPackageContainerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICreateSharedPackageContainerOptions>();
}
unsafe impl windows_core::Interface for CreateSharedPackageContainerOptions {
    type Vtable = <ICreateSharedPackageContainerOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICreateSharedPackageContainerOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CreateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerOptions";
}
unsafe impl Send for CreateSharedPackageContainerOptions {}
unsafe impl Sync for CreateSharedPackageContainerOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateSharedPackageContainerResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CreateSharedPackageContainerResult, windows_core::IUnknown, windows_core::IInspectable);
impl CreateSharedPackageContainerResult {
    pub fn Container(&self) -> windows_core::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Container)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for CreateSharedPackageContainerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICreateSharedPackageContainerResult>();
}
unsafe impl windows_core::Interface for CreateSharedPackageContainerResult {
    type Vtable = <ICreateSharedPackageContainerResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICreateSharedPackageContainerResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CreateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.CreateSharedPackageContainerResult";
}
unsafe impl Send for CreateSharedPackageContainerResult {}
unsafe impl Sync for CreateSharedPackageContainerResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteSharedPackageContainerOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeleteSharedPackageContainerOptions, windows_core::IUnknown, windows_core::IInspectable);
impl DeleteSharedPackageContainerOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DeleteSharedPackageContainerOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ForceAppShutdown(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceAppShutdown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceAppShutdown)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllUsers(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllUsers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllUsers(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllUsers)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for DeleteSharedPackageContainerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeleteSharedPackageContainerOptions>();
}
unsafe impl windows_core::Interface for DeleteSharedPackageContainerOptions {
    type Vtable = <IDeleteSharedPackageContainerOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDeleteSharedPackageContainerOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeleteSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerOptions";
}
unsafe impl Send for DeleteSharedPackageContainerOptions {}
unsafe impl Sync for DeleteSharedPackageContainerOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteSharedPackageContainerResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeleteSharedPackageContainerResult, windows_core::IUnknown, windows_core::IInspectable);
impl DeleteSharedPackageContainerResult {
    pub fn Status(&self) -> windows_core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for DeleteSharedPackageContainerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeleteSharedPackageContainerResult>();
}
unsafe impl windows_core::Interface for DeleteSharedPackageContainerResult {
    type Vtable = <IDeleteSharedPackageContainerResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDeleteSharedPackageContainerResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeleteSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeleteSharedPackageContainerResult";
}
unsafe impl Send for DeleteSharedPackageContainerResult {}
unsafe impl Sync for DeleteSharedPackageContainerResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeploymentOptions(pub u32);
impl DeploymentOptions {
    pub const None: Self = Self(0u32);
    pub const ForceApplicationShutdown: Self = Self(1u32);
    pub const DevelopmentMode: Self = Self(2u32);
    pub const InstallAllResources: Self = Self(32u32);
    pub const ForceTargetApplicationShutdown: Self = Self(64u32);
    pub const RequiredContentGroupOnly: Self = Self(256u32);
    pub const ForceUpdateFromAnyVersion: Self = Self(262144u32);
    pub const RetainFilesOnFailure: Self = Self(2097152u32);
    pub const StageInPlace: Self = Self(4194304u32);
}
impl windows_core::TypeKind for DeploymentOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DeploymentOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentOptions;u4)");
}
impl DeploymentOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DeploymentOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DeploymentOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DeploymentOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DeploymentOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DeploymentOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DeploymentProgress {
    pub state: DeploymentProgressState,
    pub percentage: u32,
}
impl windows_core::TypeKind for DeploymentProgress {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DeploymentProgress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Management.Deployment.DeploymentProgress;enum(Windows.Management.Deployment.DeploymentProgressState;i4);u4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeploymentProgressState(pub i32);
impl DeploymentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
}
impl windows_core::TypeKind for DeploymentProgressState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DeploymentProgressState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.DeploymentProgressState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeploymentResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DeploymentResult, windows_core::IUnknown, windows_core::IInspectable);
impl DeploymentResult {
    pub fn ErrorText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ActivityId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRegistered(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDeploymentResult2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRegistered)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DeploymentResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeploymentResult>();
}
unsafe impl windows_core::Interface for DeploymentResult {
    type Vtable = <IDeploymentResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDeploymentResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DeploymentResult {
    const NAME: &'static str = "Windows.Management.Deployment.DeploymentResult";
}
unsafe impl Send for DeploymentResult {}
unsafe impl Sync for DeploymentResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FindSharedPackageContainerOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FindSharedPackageContainerOptions, windows_core::IUnknown, windows_core::IInspectable);
impl FindSharedPackageContainerOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FindSharedPackageContainerOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetPackageFamilyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPackageFamilyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for FindSharedPackageContainerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFindSharedPackageContainerOptions>();
}
unsafe impl windows_core::Interface for FindSharedPackageContainerOptions {
    type Vtable = <IFindSharedPackageContainerOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFindSharedPackageContainerOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FindSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.FindSharedPackageContainerOptions";
}
unsafe impl Send for FindSharedPackageContainerOptions {}
unsafe impl Sync for FindSharedPackageContainerOptions {}
windows_core::imp::define_interface!(IAddPackageOptions, IAddPackageOptions_Vtbl, 0x05cee018_f68f_422b_95a4_66679ec77fc0);
impl windows_core::RuntimeType for IAddPackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DependencyPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelatedPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExternalLocationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExternalLocationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StubPackageOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StubPackageOption) -> windows_core::HRESULT,
    pub SetStubPackageOption: unsafe extern "system" fn(*mut core::ffi::c_void, StubPackageOption) -> windows_core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForceTargetAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceTargetAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RequiredContentGroupOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRequiredContentGroupOnly: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RetainFilesOnFailure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRetainFilesOnFailure: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAddPackageOptions2, IAddPackageOptions2_Vtbl, 0xee515828_bf33_40f7_84af_1b6fad2919d7);
impl windows_core::RuntimeType for IAddPackageOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPackageOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExpectedDigests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LimitToExistingPackages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetLimitToExistingPackages: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallerManager, IAppInstallerManager_Vtbl, 0xe7ee21c3_2103_53ee_9b18_68afeab0033d);
impl windows_core::RuntimeType for IAppInstallerManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAutoUpdateSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearAutoUpdateSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PauseAutoUpdatesUntil: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppInstallerManagerStatics, IAppInstallerManagerStatics_Vtbl, 0xc95a6ed5_fc59_5336_9b2e_2b07c5e61434);
impl windows_core::RuntimeType for IAppInstallerManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallerManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAutoUpdateSettingsOptions, IAutoUpdateSettingsOptions_Vtbl, 0x67491d87_35e1_512a_8968_1ae88d1be6d3);
impl windows_core::RuntimeType for IAutoUpdateSettingsOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::ApplicationModel::PackageVersion) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Version: usize,
    #[cfg(feature = "ApplicationModel")]
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::ApplicationModel::PackageVersion) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SetVersion: usize,
    pub AppInstallerUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAppInstallerUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnLaunch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetOnLaunch: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub HoursBetweenUpdateChecks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHoursBetweenUpdateChecks: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ShowPrompt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowPrompt: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub UpdateBlocksActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUpdateBlocksActivation: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AutomaticBackgroundTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutomaticBackgroundTask: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAutoRepairEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAutoRepairEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub UpdateUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RepairUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DependencyPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAutoUpdateSettingsOptionsStatics, IAutoUpdateSettingsOptionsStatics_Vtbl, 0x887b337d_0c05_54d0_bd49_3bb7a2c084cb);
impl windows_core::RuntimeType for IAutoUpdateSettingsOptionsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoUpdateSettingsOptionsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub CreateFromAppInstallerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    CreateFromAppInstallerInfo: usize,
}
windows_core::imp::define_interface!(ICreateSharedPackageContainerOptions, ICreateSharedPackageContainerOptions_Vtbl, 0xc2ab6ece_f664_5c8e_a4b3_2a33276d3dde);
impl windows_core::RuntimeType for ICreateSharedPackageContainerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CreateCollisionOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SharedPackageContainerCreationCollisionOptions) -> windows_core::HRESULT,
    pub SetCreateCollisionOption: unsafe extern "system" fn(*mut core::ffi::c_void, SharedPackageContainerCreationCollisionOptions) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateSharedPackageContainerResult, ICreateSharedPackageContainerResult_Vtbl, 0xce8810bf_151c_5707_b936_497e564afc7a);
impl windows_core::RuntimeType for ICreateSharedPackageContainerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateSharedPackageContainerResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SharedPackageContainerOperationStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeleteSharedPackageContainerOptions, IDeleteSharedPackageContainerOptions_Vtbl, 0x9d81865f_986e_5138_8b5d_384d8e66ed6c);
impl windows_core::RuntimeType for IDeleteSharedPackageContainerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllUsers: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeleteSharedPackageContainerResult, IDeleteSharedPackageContainerResult_Vtbl, 0x35398884_5736_517b_85bc_e598c81ab284);
impl windows_core::RuntimeType for IDeleteSharedPackageContainerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteSharedPackageContainerResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SharedPackageContainerOperationStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeploymentResult, IDeploymentResult_Vtbl, 0x2563b9ae_b77d_4c1f_8a7b_20e6ad515ef3);
impl windows_core::RuntimeType for IDeploymentResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ErrorText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeploymentResult2, IDeploymentResult2_Vtbl, 0xfc0e715c_5a01_4bd7_bcf1_381c8c82e04a);
impl windows_core::RuntimeType for IDeploymentResult2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeploymentResult2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRegistered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFindSharedPackageContainerOptions, IFindSharedPackageContainerOptions_Vtbl, 0xb40fc8fe_8384_54cc_817d_ae09d3b6a606);
impl windows_core::RuntimeType for IFindSharedPackageContainerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindSharedPackageContainerOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageAllUserProvisioningOptions, IPackageAllUserProvisioningOptions_Vtbl, 0xda35aa22_1de0_5d3e_99ff_d24f3118bf5e);
impl windows_core::RuntimeType for IPackageAllUserProvisioningOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageAllUserProvisioningOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProjectionOrderPackageFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageAllUserProvisioningOptions2, IPackageAllUserProvisioningOptions2_Vtbl, 0xb9e3cab5_2d97_579f_9368_d10bb4d4542b);
impl windows_core::RuntimeType for IPackageAllUserProvisioningOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageAllUserProvisioningOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeferAutomaticRegistration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDeferAutomaticRegistration: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager, IPackageManager_Vtbl, 0x9a7d4b65_5e8f_4fc7_a2e5_7f6925cb8b53);
impl windows_core::RuntimeType for IPackageManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdatePackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovePackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StagePackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackages: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityId: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByNamePublisher: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdNamePublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdNamePublisher: usize,
    pub FindUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPackageState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageState) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackageByPackageFullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackageByPackageFullName: usize,
    pub CleanupPackageForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByPackageFamilyName: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdPackageFamilyName: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackageByUserSecurityIdPackageFullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackageByUserSecurityIdPackageFullName: usize,
}
windows_core::imp::define_interface!(IPackageManager10, IPackageManager10_Vtbl, 0xa7d7d07e_2e66_4093_aed5_e093ed87b3bb);
impl windows_core::RuntimeType for IPackageManager10 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager10_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProvisionPackageForAllUsersWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager11, IPackageManager11_Vtbl, 0x12950b24_c77e_4ea7_8859_325318074e15);
impl windows_core::RuntimeType for IPackageManager11 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager11_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemovePackageByUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager12, IPackageManager12_Vtbl, 0x5d233adf_f9e3_4d96_b40d_96788e39539f);
impl windows_core::RuntimeType for IPackageManager12 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager12_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPackageRemovalPending: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPackageRemovalPendingForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPackageRemovalPendingByUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPackageRemovalPendingByUriForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager2, IPackageManager2_Vtbl, 0xf7aad08d_0840_46f2_b5d8_cad47693a095);
impl windows_core::RuntimeType for IPackageManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemovePackageWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RemovalOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StagePackageWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterPackageByFullNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PackageTypes, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageTypes, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByNamePublisherWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, PackageTypes, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByNamePublisherWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, PackageTypes, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageTypes, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByPackageFamilyNameWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, PackageTypes, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: usize,
    pub StageUserDataAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager3, IPackageManager3_Vtbl, 0xdaad9948_36f1_41a7_9188_bc263e0dcb72);
impl windows_core::RuntimeType for IPackageManager3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddPackageVolumeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPackageToVolumeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearPackageStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageStatus) -> windows_core::HRESULT,
    pub RegisterPackageWithAppDataVolumeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindPackageVolumeByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindPackageVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultPackageVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MovePackageToVolumeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovePackageVolumeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultPackageVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPackageStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageStatus) -> windows_core::HRESULT,
    pub SetPackageVolumeOfflineAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPackageVolumeOnlineAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StagePackageToVolumeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StageUserDataWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager4, IPackageManager4_Vtbl, 0x3c719963_bab6_46bf_8ff7_da4719230ae6);
impl windows_core::RuntimeType for IPackageManager4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPackageVolumesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager5, IPackageManager5_Vtbl, 0x711f3117_1afd_4313_978c_9bb6e1b864a7);
impl windows_core::RuntimeType for IPackageManager5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddPackageToVolumeAndOptionalPackagesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StagePackageToVolumeAndOptionalPackagesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterPackageByFamilyNameAndOptionalPackagesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DebugSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager6, IPackageManager6_Vtbl, 0x0847e909_53cd_4e4f_832e_57d180f6e447);
impl windows_core::RuntimeType for IPackageManager6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProvisionPackageForAllUsersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPackageByAppInstallerFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AddPackageByAppInstallerOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAddPackageByAppInstallerFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, AddPackageByAppInstallerOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPackageToVolumeAndRelatedSetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StagePackageToVolumeAndRelatedSetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAddPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager7, IPackageManager7_Vtbl, 0xf28654f4_2ba7_4b80_88d6_be15f9a23fba);
impl windows_core::RuntimeType for IPackageManager7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAddPackageAndRelatedSetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, DeploymentOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager8, IPackageManager8_Vtbl, 0xb8575330_1298_4ee2_80ee_7f659c5d2782);
impl windows_core::RuntimeType for IPackageManager8 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager8_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeprovisionPackageForAllUsersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManager9, IPackageManager9_Vtbl, 0x1aa79035_cc71_4b2e_80a6_c7041d8579a7);
impl windows_core::RuntimeType for IPackageManager9 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManager9_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub FindProvisionedPackages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindProvisionedPackages: usize,
    pub AddPackageByUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StagePackageByUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterPackageByUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterPackagesByFullNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPackageStubPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageStubPreference) -> windows_core::HRESULT,
    pub GetPackageStubPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut PackageStubPreference) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageManagerDebugSettings, IPackageManagerDebugSettings_Vtbl, 0x1a611683_a988_4fcf_8f0f_ce175898e8eb);
impl windows_core::RuntimeType for IPackageManagerDebugSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageManagerDebugSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub SetContentGroupStateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::ApplicationModel::PackageContentGroupState, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SetContentGroupStateAsync: usize,
    #[cfg(feature = "ApplicationModel")]
    pub SetContentGroupStateWithPercentageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::ApplicationModel::PackageContentGroupState, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SetContentGroupStateWithPercentageAsync: usize,
}
windows_core::imp::define_interface!(IPackageUserInformation, IPackageUserInformation_Vtbl, 0xf6383423_fa09_4cbc_9055_15ca275e2e7e);
impl windows_core::RuntimeType for IPackageUserInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageUserInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UserSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PackageInstallState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPackageVolume, IPackageVolume_Vtbl, 0xcf2672c3_1a40_4450_9739_2ace2e898853);
impl windows_core::RuntimeType for IPackageVolume {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsOffline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSystemVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MountPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PackageStorePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportsHardLinks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackages: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByNamePublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByNamePublisher: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByPackageFamilyName: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PackageTypes, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByNamePublisherWithPackagesTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PackageTypes, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByNamePublisherWithPackagesTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByPackageFamilyNameWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, PackageTypes, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByPackageFamilyNameWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackageByPackageFullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackageByPackageFullName: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityId: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdNamePublisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdNamePublisher: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdPackageFamilyName: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageTypes, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageTypes, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PackageTypes, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: usize,
    #[cfg(feature = "ApplicationModel")]
    pub FindPackageByUserSecurityIdPackageFullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    FindPackageByUserSecurityIdPackageFullName: usize,
}
windows_core::imp::define_interface!(IPackageVolume2, IPackageVolume2_Vtbl, 0x46abcf2e_9dd4_47a2_ab8c_c6408349bcd8);
impl windows_core::RuntimeType for IPackageVolume2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackageVolume2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsFullTrustPackageSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsAppxInstallSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAvailableSpaceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRegisterPackageOptions, IRegisterPackageOptions_Vtbl, 0x677112a7_50d4_496c_8415_0602b4c6d3bf);
impl windows_core::RuntimeType for IRegisterPackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisterPackageOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DependencyPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppDataVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAppDataVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExternalLocationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExternalLocationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForceTargetAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceTargetAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDeferRegistrationWhenPackagesAreInUse: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRegisterPackageOptions2, IRegisterPackageOptions2_Vtbl, 0x3dfa9743_86ff_4a11_bc93_434eb6be3a0b);
impl windows_core::RuntimeType for IRegisterPackageOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisterPackageOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExpectedDigests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemovePackageOptions, IRemovePackageOptions_Vtbl, 0x13cf01f3_c450_4f7c_a5a3_5e3c631b7462);
impl windows_core::RuntimeType for IRemovePackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemovePackageOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreserveApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPreserveApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PreserveRoamableApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetPreserveRoamableApplicationData: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RemoveForAllUsers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRemoveForAllUsers: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemovePackageOptions2, IRemovePackageOptions2_Vtbl, 0x3fcc61e5_22c5_423b_b4b4_cf10bb50830c);
impl windows_core::RuntimeType for IRemovePackageOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemovePackageOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeferRemovalWhenPackagesAreInUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDeferRemovalWhenPackagesAreInUse: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISharedPackageContainer, ISharedPackageContainer_Vtbl, 0x177f1aa9_151e_5ef7_b1d9_2fba0b4b0d17);
impl windows_core::RuntimeType for ISharedPackageContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMembers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemovePackageFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISharedPackageContainerManager, ISharedPackageContainerManager_Vtbl, 0xbe353068_1ef7_5ac8_ab3f_0b9f612f0274);
impl windows_core::RuntimeType for ISharedPackageContainerManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindContainers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindContainersWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISharedPackageContainerManagerStatics, ISharedPackageContainerManagerStatics_Vtbl, 0x2ef56348_838a_5f55_a89e_1198a2c627e6);
impl windows_core::RuntimeType for ISharedPackageContainerManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForProvisioning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISharedPackageContainerMember, ISharedPackageContainerMember_Vtbl, 0xfe0d0438_43c9_5426_b89c_f79bf85ddff4);
impl windows_core::RuntimeType for ISharedPackageContainerMember {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMember_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISharedPackageContainerMemberFactory, ISharedPackageContainerMemberFactory_Vtbl, 0x49b0ceeb_498f_5a62_b738_b3ca0d436704);
impl windows_core::RuntimeType for ISharedPackageContainerMemberFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedPackageContainerMemberFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStagePackageOptions, IStagePackageOptions_Vtbl, 0x0b110c9c_b95d_4c56_bd36_6d656800d06b);
impl windows_core::RuntimeType for IStagePackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStagePackageOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DependencyPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalPackageFamilyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OptionalPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelatedPackageUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExternalLocationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExternalLocationUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StubPackageOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StubPackageOption) -> windows_core::HRESULT,
    pub SetStubPackageOption: unsafe extern "system" fn(*mut core::ffi::c_void, StubPackageOption) -> windows_core::HRESULT,
    pub DeveloperMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDeveloperMode: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceUpdateFromAnyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InstallAllResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetInstallAllResources: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RequiredContentGroupOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRequiredContentGroupOnly: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub StageInPlace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetStageInPlace: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowUnsigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowUnsigned: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStagePackageOptions2, IStagePackageOptions2_Vtbl, 0x990c4ccc_6226_4192_ba92_79875fce0d9c);
impl windows_core::RuntimeType for IStagePackageOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStagePackageOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ExpectedDigests: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateSharedPackageContainerOptions, IUpdateSharedPackageContainerOptions_Vtbl, 0x80672e83_7194_59f9_b5b9_daa5375f130a);
impl windows_core::RuntimeType for IUpdateSharedPackageContainerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetForceAppShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub RequirePackagesPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRequirePackagesPresent: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateSharedPackageContainerResult, IUpdateSharedPackageContainerResult_Vtbl, 0xaa407df7_c72d_5458_aea3_4645b6a8ee99);
impl windows_core::RuntimeType for IUpdateSharedPackageContainerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSharedPackageContainerResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SharedPackageContainerOperationStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageAllUserProvisioningOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageAllUserProvisioningOptions, windows_core::IUnknown, windows_core::IInspectable);
impl PackageAllUserProvisioningOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PackageAllUserProvisioningOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OptionalPackageFamilyNames(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalPackageFamilyNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProjectionOrderPackageFamilyNames(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProjectionOrderPackageFamilyNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeferAutomaticRegistration(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackageAllUserProvisioningOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeferAutomaticRegistration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeferAutomaticRegistration(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPackageAllUserProvisioningOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDeferAutomaticRegistration)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for PackageAllUserProvisioningOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageAllUserProvisioningOptions>();
}
unsafe impl windows_core::Interface for PackageAllUserProvisioningOptions {
    type Vtable = <IPackageAllUserProvisioningOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPackageAllUserProvisioningOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageAllUserProvisioningOptions {
    const NAME: &'static str = "Windows.Management.Deployment.PackageAllUserProvisioningOptions";
}
unsafe impl Send for PackageAllUserProvisioningOptions {}
unsafe impl Sync for PackageAllUserProvisioningOptions {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageInstallState(pub i32);
impl PackageInstallState {
    pub const NotInstalled: Self = Self(0i32);
    pub const Staged: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
    pub const Paused: Self = Self(6i32);
}
impl windows_core::TypeKind for PackageInstallState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PackageInstallState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageInstallState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageManager, windows_core::IUnknown, windows_core::IInspectable);
impl PackageManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PackageManager, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AddPackageAsync<P0, P1>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddPackageAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpdatePackageAsync<P0, P1>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdatePackageAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemovePackageAsync(&self, packagefullname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovePackageAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StagePackageAsync<P0, P1>(&self, packageuri: P0, dependencypackageuris: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StagePackageAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPackageAsync<P0, P1>(&self, manifesturi: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPackageAsync)(windows_core::Interface::as_raw(this), manifesturi.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackages(&self) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityId(&self, usersecurityid: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByNamePublisher(&self, packagename: &windows_core::HSTRING, packagepublisher: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByNamePublisher)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagename), core::mem::transmute_copy(packagepublisher), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdNamePublisher(&self, usersecurityid: &windows_core::HSTRING, packagename: &windows_core::HSTRING, packagepublisher: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisher)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), core::mem::transmute_copy(packagename), core::mem::transmute_copy(packagepublisher), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindUsers(&self, packagefullname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IIterable<PackageUserInformation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindUsers)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPackageState(&self, packagefullname: &windows_core::HSTRING, packagestate: PackageState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPackageState)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), packagestate).ok() }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackageByPackageFullName(&self, packagefullname: &windows_core::HSTRING) -> windows_core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackageByPackageFullName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CleanupPackageForUserAsync(&self, packagename: &windows_core::HSTRING, usersecurityid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CleanupPackageForUserAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagename), core::mem::transmute_copy(usersecurityid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByPackageFamilyName(&self, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByPackageFamilyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName(&self, usersecurityid: &windows_core::HSTRING, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackageByUserSecurityIdPackageFullName(&self, usersecurityid: &windows_core::HSTRING, packagefullname: &windows_core::HSTRING) -> windows_core::Result<super::super::ApplicationModel::Package> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackageByUserSecurityIdPackageFullName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProvisionPackageForAllUsersWithOptionsAsync<P1>(&self, mainpackagefamilyname: &windows_core::HSTRING, options: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P1: windows_core::Param<PackageAllUserProvisioningOptions>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager10>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProvisionPackageForAllUsersWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(mainpackagefamilyname), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemovePackageByUriAsync<P0, P1>(&self, packageuri: P0, options: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<RemovePackageOptions>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager11>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovePackageByUriAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPackageRemovalPending(&self, packagefullname: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackageManager12>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPackageRemovalPending)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), &mut result__).map(|| result__)
        }
    }
    pub fn IsPackageRemovalPendingForUser(&self, packagefullname: &windows_core::HSTRING, usersecurityid: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackageManager12>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPackageRemovalPendingForUser)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), core::mem::transmute_copy(usersecurityid), &mut result__).map(|| result__)
        }
    }
    pub fn IsPackageRemovalPendingByUri<P0>(&self, packageuri: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager12>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPackageRemovalPendingByUri)(windows_core::Interface::as_raw(this), packageuri.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsPackageRemovalPendingByUriForUser<P0>(&self, packageuri: P0, usersecurityid: &windows_core::HSTRING) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager12>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPackageRemovalPendingByUriForUser)(windows_core::Interface::as_raw(this), packageuri.param().abi(), core::mem::transmute_copy(usersecurityid), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePackageWithOptionsAsync(&self, packagefullname: &windows_core::HSTRING, removaloptions: RemovalOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovePackageWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), removaloptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StagePackageWithOptionsAsync<P0, P1>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StagePackageWithOptionsAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPackageByFullNameAsync<P1>(&self, mainpackagefullname: &windows_core::HSTRING, dependencypackagefullnames: P1, deploymentoptions: DeploymentOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPackageByFullNameAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(mainpackagefullname), dependencypackagefullnames.param().abi(), deploymentoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesWithPackageTypes)(windows_core::Interface::as_raw(this), packagetypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes(&self, usersecurityid: &windows_core::HSTRING, packagetypes: PackageTypes) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdWithPackageTypes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), packagetypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByNamePublisherWithPackageTypes(&self, packagename: &windows_core::HSTRING, packagepublisher: &windows_core::HSTRING, packagetypes: PackageTypes) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByNamePublisherWithPackageTypes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagename), core::mem::transmute_copy(packagepublisher), packagetypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, usersecurityid: &windows_core::HSTRING, packagename: &windows_core::HSTRING, packagepublisher: &windows_core::HSTRING, packagetypes: PackageTypes) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), core::mem::transmute_copy(packagename), core::mem::transmute_copy(packagepublisher), packagetypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packagefamilyname: &windows_core::HSTRING, packagetypes: PackageTypes) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByPackageFamilyNameWithPackageTypes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), packagetypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes(&self, usersecurityid: &windows_core::HSTRING, packagefamilyname: &windows_core::HSTRING, packagetypes: PackageTypes) -> windows_core::Result<windows_collections::IIterable<super::super::ApplicationModel::Package>> {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), core::mem::transmute_copy(packagefamilyname), packagetypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StageUserDataAsync(&self, packagefullname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &windows_core::Interface::cast::<IPackageManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StageUserDataAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddPackageVolumeAsync(&self, packagestorepath: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<PackageVolume>> {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddPackageVolumeAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagestorepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddPackageToVolumeAsync<P0, P1, P3>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions, targetvolume: P3) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddPackageToVolumeAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, targetvolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ClearPackageStatus(&self, packagefullname: &windows_core::HSTRING, status: PackageStatus) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ClearPackageStatus)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), status).ok() }
    }
    pub fn RegisterPackageWithAppDataVolumeAsync<P0, P1, P3>(&self, manifesturi: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions, appdatavolume: P3) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPackageWithAppDataVolumeAsync)(windows_core::Interface::as_raw(this), manifesturi.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, appdatavolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindPackageVolumeByName(&self, volumename: &windows_core::HSTRING) -> windows_core::Result<PackageVolume> {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackageVolumeByName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(volumename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindPackageVolumes(&self) -> windows_core::Result<windows_collections::IIterable<PackageVolume>> {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackageVolumes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefaultPackageVolume(&self) -> windows_core::Result<PackageVolume> {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultPackageVolume)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MovePackageToVolumeAsync<P2>(&self, packagefullname: &windows_core::HSTRING, deploymentoptions: DeploymentOptions, targetvolume: P2) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P2: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MovePackageToVolumeAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), deploymentoptions, targetvolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemovePackageVolumeAsync<P0>(&self, volume: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovePackageVolumeAsync)(windows_core::Interface::as_raw(this), volume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultPackageVolume<P0>(&self, volume: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultPackageVolume)(windows_core::Interface::as_raw(this), volume.param().abi()).ok() }
    }
    pub fn SetPackageStatus(&self, packagefullname: &windows_core::HSTRING, status: PackageStatus) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPackageStatus)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), status).ok() }
    }
    pub fn SetPackageVolumeOfflineAsync<P0>(&self, packagevolume: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPackageVolumeOfflineAsync)(windows_core::Interface::as_raw(this), packagevolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPackageVolumeOnlineAsync<P0>(&self, packagevolume: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetPackageVolumeOnlineAsync)(windows_core::Interface::as_raw(this), packagevolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StagePackageToVolumeAsync<P0, P1, P3>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions, targetvolume: P3) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StagePackageToVolumeAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, targetvolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StageUserDataWithOptionsAsync(&self, packagefullname: &windows_core::HSTRING, deploymentoptions: DeploymentOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &windows_core::Interface::cast::<IPackageManager3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StageUserDataWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), deploymentoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPackageVolumesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<PackageVolume>>> {
        let this = &windows_core::Interface::cast::<IPackageManager4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPackageVolumesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddPackageToVolumeAndOptionalPackagesAsync<P0, P1, P3, P4, P5>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions, targetvolume: P3, optionalpackagefamilynames: P4, externalpackageuris: P5) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
        P4: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P5: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddPackageToVolumeAndOptionalPackagesAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, targetvolume.param().abi(), optionalpackagefamilynames.param().abi(), externalpackageuris.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StagePackageToVolumeAndOptionalPackagesAsync<P0, P1, P3, P4, P5>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions, targetvolume: P3, optionalpackagefamilynames: P4, externalpackageuris: P5) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
        P4: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P5: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StagePackageToVolumeAndOptionalPackagesAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, targetvolume.param().abi(), optionalpackagefamilynames.param().abi(), externalpackageuris.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPackageByFamilyNameAndOptionalPackagesAsync<P1, P3, P4>(&self, mainpackagefamilyname: &windows_core::HSTRING, dependencypackagefamilynames: P1, deploymentoptions: DeploymentOptions, appdatavolume: P3, optionalpackagefamilynames: P4) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P1: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P3: windows_core::Param<PackageVolume>,
        P4: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPackageByFamilyNameAndOptionalPackagesAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(mainpackagefamilyname), dependencypackagefamilynames.param().abi(), deploymentoptions, appdatavolume.param().abi(), optionalpackagefamilynames.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DebugSettings(&self) -> windows_core::Result<PackageManagerDebugSettings> {
        let this = &windows_core::Interface::cast::<IPackageManager5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DebugSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProvisionPackageForAllUsersAsync(&self, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProvisionPackageForAllUsersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddPackageByAppInstallerFileAsync<P0, P2>(&self, appinstallerfileuri: P0, options: AddPackageByAppInstallerOptions, targetvolume: P2) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P2: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddPackageByAppInstallerFileAsync)(windows_core::Interface::as_raw(this), appinstallerfileuri.param().abi(), options, targetvolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAddPackageByAppInstallerFileAsync<P0, P2>(&self, appinstallerfileuri: P0, options: AddPackageByAppInstallerOptions, targetvolume: P2) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P2: windows_core::Param<PackageVolume>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAddPackageByAppInstallerFileAsync)(windows_core::Interface::as_raw(this), appinstallerfileuri.param().abi(), options, targetvolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddPackageToVolumeAndRelatedSetAsync<P0, P1, P3, P4, P5, P6>(&self, packageuri: P0, dependencypackageuris: P1, options: DeploymentOptions, targetvolume: P3, optionalpackagefamilynames: P4, packageuristoinstall: P5, relatedpackageuris: P6) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
        P4: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P5: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P6: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddPackageToVolumeAndRelatedSetAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), options, targetvolume.param().abi(), optionalpackagefamilynames.param().abi(), packageuristoinstall.param().abi(), relatedpackageuris.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StagePackageToVolumeAndRelatedSetAsync<P0, P1, P3, P4, P5, P6>(&self, packageuri: P0, dependencypackageuris: P1, options: DeploymentOptions, targetvolume: P3, optionalpackagefamilynames: P4, packageuristoinstall: P5, relatedpackageuris: P6) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
        P4: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P5: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P6: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StagePackageToVolumeAndRelatedSetAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), options, targetvolume.param().abi(), optionalpackagefamilynames.param().abi(), packageuristoinstall.param().abi(), relatedpackageuris.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAddPackageAsync<P0, P1, P3, P4, P5>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions, targetvolume: P3, optionalpackagefamilynames: P4, relatedpackageuris: P5) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
        P4: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P5: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAddPackageAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, targetvolume.param().abi(), optionalpackagefamilynames.param().abi(), relatedpackageuris.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAddPackageAndRelatedSetAsync<P0, P1, P3, P4, P5, P6>(&self, packageuri: P0, dependencypackageuris: P1, deploymentoptions: DeploymentOptions, targetvolume: P3, optionalpackagefamilynames: P4, relatedpackageuris: P5, packageuristoinstall: P6) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P3: windows_core::Param<PackageVolume>,
        P4: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P5: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
        P6: windows_core::Param<windows_collections::IIterable<super::super::Foundation::Uri>>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAddPackageAndRelatedSetAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), dependencypackageuris.param().abi(), deploymentoptions, targetvolume.param().abi(), optionalpackagefamilynames.param().abi(), relatedpackageuris.param().abi(), packageuristoinstall.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeprovisionPackageForAllUsersAsync(&self, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> {
        let this = &windows_core::Interface::cast::<IPackageManager8>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeprovisionPackageForAllUsersAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindProvisionedPackages(&self) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = &windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindProvisionedPackages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddPackageByUriAsync<P0, P1>(&self, packageuri: P0, options: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<AddPackageOptions>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddPackageByUriAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StagePackageByUriAsync<P0, P1>(&self, packageuri: P0, options: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<StagePackageOptions>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StagePackageByUriAsync)(windows_core::Interface::as_raw(this), packageuri.param().abi(), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPackageByUriAsync<P0, P1>(&self, manifesturi: P0, options: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<RegisterPackageOptions>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPackageByUriAsync)(windows_core::Interface::as_raw(this), manifesturi.param().abi(), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RegisterPackagesByFullNameAsync<P0, P1>(&self, packagefullnames: P0, options: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
        P1: windows_core::Param<RegisterPackageOptions>,
    {
        let this = &windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RegisterPackagesByFullNameAsync)(windows_core::Interface::as_raw(this), packagefullnames.param().abi(), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPackageStubPreference(&self, packagefamilyname: &windows_core::HSTRING, usestub: PackageStubPreference) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPackageStubPreference)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), usestub).ok() }
    }
    pub fn GetPackageStubPreference(&self, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<PackageStubPreference> {
        let this = &windows_core::Interface::cast::<IPackageManager9>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPackageStubPreference)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PackageManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageManager>();
}
unsafe impl windows_core::Interface for PackageManager {
    type Vtable = <IPackageManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPackageManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageManager {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManager";
}
unsafe impl Send for PackageManager {}
unsafe impl Sync for PackageManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageManagerDebugSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageManagerDebugSettings, windows_core::IUnknown, windows_core::IInspectable);
impl PackageManagerDebugSettings {
    #[cfg(feature = "ApplicationModel")]
    pub fn SetContentGroupStateAsync<P0>(&self, package: P0, contentgroupname: &windows_core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Package>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetContentGroupStateAsync)(windows_core::Interface::as_raw(this), package.param().abi(), core::mem::transmute_copy(contentgroupname), state, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn SetContentGroupStateWithPercentageAsync<P0>(&self, package: P0, contentgroupname: &windows_core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Package>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetContentGroupStateWithPercentageAsync)(windows_core::Interface::as_raw(this), package.param().abi(), core::mem::transmute_copy(contentgroupname), state, completionpercentage, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageManagerDebugSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageManagerDebugSettings>();
}
unsafe impl windows_core::Interface for PackageManagerDebugSettings {
    type Vtable = <IPackageManagerDebugSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPackageManagerDebugSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageManagerDebugSettings {
    const NAME: &'static str = "Windows.Management.Deployment.PackageManagerDebugSettings";
}
unsafe impl Send for PackageManagerDebugSettings {}
unsafe impl Sync for PackageManagerDebugSettings {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageState(pub i32);
impl PackageState {
    pub const Normal: Self = Self(0i32);
    pub const LicenseInvalid: Self = Self(1i32);
    pub const Modified: Self = Self(2i32);
    pub const Tampered: Self = Self(3i32);
}
impl windows_core::TypeKind for PackageState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PackageState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageStatus(pub u32);
impl PackageStatus {
    pub const OK: Self = Self(0u32);
    pub const LicenseIssue: Self = Self(1u32);
    pub const Modified: Self = Self(2u32);
    pub const Tampered: Self = Self(4u32);
    pub const Disabled: Self = Self(8u32);
}
impl windows_core::TypeKind for PackageStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PackageStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStatus;u4)");
}
impl PackageStatus {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PackageStatus {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PackageStatus {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PackageStatus {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PackageStatus {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PackageStatus {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageStubPreference(pub i32);
impl PackageStubPreference {
    pub const Full: Self = Self(0i32);
    pub const Stub: Self = Self(1i32);
}
impl windows_core::TypeKind for PackageStubPreference {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PackageStubPreference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageStubPreference;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PackageTypes(pub u32);
impl PackageTypes {
    pub const None: Self = Self(0u32);
    pub const Main: Self = Self(1u32);
    pub const Framework: Self = Self(2u32);
    pub const Resource: Self = Self(4u32);
    pub const Bundle: Self = Self(8u32);
    pub const Xap: Self = Self(16u32);
    pub const Optional: Self = Self(32u32);
    pub const All: Self = Self(4294967295u32);
}
impl windows_core::TypeKind for PackageTypes {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PackageTypes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.PackageTypes;u4)");
}
impl PackageTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PackageTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PackageTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PackageTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PackageTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PackageTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageUserInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageUserInformation, windows_core::IUnknown, windows_core::IInspectable);
impl PackageUserInformation {
    pub fn UserSecurityId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserSecurityId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn InstallState(&self) -> windows_core::Result<PackageInstallState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PackageUserInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageUserInformation>();
}
unsafe impl windows_core::Interface for PackageUserInformation {
    type Vtable = <IPackageUserInformation as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPackageUserInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageUserInformation {
    const NAME: &'static str = "Windows.Management.Deployment.PackageUserInformation";
}
unsafe impl Send for PackageUserInformation {}
unsafe impl Sync for PackageUserInformation {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageVolume(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PackageVolume, windows_core::IUnknown, windows_core::IInspectable);
impl PackageVolume {
    pub fn IsOffline(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOffline)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSystemVolume(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSystemVolume)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MountPoint(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MountPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PackageStorePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageStorePath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SupportsHardLinks(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportsHardLinks)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackages(&self) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByNamePublisher(&self, packagename: &windows_core::HSTRING, packagepublisher: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByNamePublisher)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagename), core::mem::transmute_copy(packagepublisher), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByPackageFamilyName(&self, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByPackageFamilyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesWithPackageTypes)(windows_core::Interface::as_raw(this), packagetypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByNamePublisherWithPackagesTypes(&self, packagetypes: PackageTypes, packagename: &windows_core::HSTRING, packagepublisher: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByNamePublisherWithPackagesTypes)(windows_core::Interface::as_raw(this), packagetypes, core::mem::transmute_copy(packagename), core::mem::transmute_copy(packagepublisher), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packagetypes: PackageTypes, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByPackageFamilyNameWithPackageTypes)(windows_core::Interface::as_raw(this), packagetypes, core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackageByPackageFullName(&self, packagefullname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackageByPackageFullName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityId(&self, usersecurityid: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdNamePublisher(&self, usersecurityid: &windows_core::HSTRING, packagename: &windows_core::HSTRING, packagepublisher: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisher)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), core::mem::transmute_copy(packagename), core::mem::transmute_copy(packagepublisher), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdPackageFamilyName(&self, usersecurityid: &windows_core::HSTRING, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdWithPackageTypes(&self, usersecurityid: &windows_core::HSTRING, packagetypes: PackageTypes) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdWithPackageTypes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), packagetypes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, usersecurityid: &windows_core::HSTRING, packagetypes: PackageTypes, packagename: &windows_core::HSTRING, packagepublisher: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), packagetypes, core::mem::transmute_copy(packagename), core::mem::transmute_copy(packagepublisher), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes(&self, usersecurityid: &windows_core::HSTRING, packagetypes: PackageTypes, packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), packagetypes, core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn FindPackageByUserSecurityIdPackageFullName(&self, usersecurityid: &windows_core::HSTRING, packagefullname: &windows_core::HSTRING) -> windows_core::Result<windows_collections::IVector<super::super::ApplicationModel::Package>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindPackageByUserSecurityIdPackageFullName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersecurityid), core::mem::transmute_copy(packagefullname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsFullTrustPackageSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFullTrustPackageSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsAppxInstallSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAppxInstallSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetAvailableSpaceAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<u64>> {
        let this = &windows_core::Interface::cast::<IPackageVolume2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAvailableSpaceAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PackageVolume {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPackageVolume>();
}
unsafe impl windows_core::Interface for PackageVolume {
    type Vtable = <IPackageVolume as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPackageVolume as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PackageVolume {
    const NAME: &'static str = "Windows.Management.Deployment.PackageVolume";
}
unsafe impl Send for PackageVolume {}
unsafe impl Sync for PackageVolume {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisterPackageOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RegisterPackageOptions, windows_core::IUnknown, windows_core::IInspectable);
impl RegisterPackageOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RegisterPackageOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DependencyPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DependencyPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppDataVolume(&self) -> windows_core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDataVolume)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppDataVolume<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PackageVolume>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAppDataVolume)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OptionalPackageFamilyNames(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalPackageFamilyNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExternalLocationUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExternalLocationUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExternalLocationUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExternalLocationUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DeveloperMode(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeveloperMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeveloperMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceAppShutdown(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceAppShutdown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceAppShutdown)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceTargetAppShutdown(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceTargetAppShutdown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceTargetAppShutdown(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceTargetAppShutdown)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallAllResources)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInstallAllResources)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StageInPlace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStageInPlace)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowUnsigned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowUnsigned)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeferRegistrationWhenPackagesAreInUse(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeferRegistrationWhenPackagesAreInUse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeferRegistrationWhenPackagesAreInUse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExpectedDigests(&self) -> windows_core::Result<windows_collections::IMap<super::super::Foundation::Uri, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IRegisterPackageOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpectedDigests)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RegisterPackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRegisterPackageOptions>();
}
unsafe impl windows_core::Interface for RegisterPackageOptions {
    type Vtable = <IRegisterPackageOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRegisterPackageOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RegisterPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.RegisterPackageOptions";
}
unsafe impl Send for RegisterPackageOptions {}
unsafe impl Sync for RegisterPackageOptions {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RemovalOptions(pub u32);
impl RemovalOptions {
    pub const None: Self = Self(0u32);
    pub const PreserveApplicationData: Self = Self(4096u32);
    pub const PreserveRoamableApplicationData: Self = Self(128u32);
    pub const DeferRemovalWhenPackagesAreInUse: Self = Self(8192u32);
    pub const RemoveForAllUsers: Self = Self(524288u32);
}
impl windows_core::TypeKind for RemovalOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RemovalOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.RemovalOptions;u4)");
}
impl RemovalOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RemovalOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RemovalOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RemovalOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RemovalOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RemovalOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemovePackageOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemovePackageOptions, windows_core::IUnknown, windows_core::IInspectable);
impl RemovePackageOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RemovePackageOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PreserveApplicationData(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreserveApplicationData)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPreserveApplicationData(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPreserveApplicationData)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PreserveRoamableApplicationData(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreserveRoamableApplicationData)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPreserveRoamableApplicationData(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPreserveRoamableApplicationData)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RemoveForAllUsers(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveForAllUsers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRemoveForAllUsers(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoveForAllUsers)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeferRemovalWhenPackagesAreInUse(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRemovePackageOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeferRemovalWhenPackagesAreInUse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeferRemovalWhenPackagesAreInUse(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRemovePackageOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDeferRemovalWhenPackagesAreInUse)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for RemovePackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemovePackageOptions>();
}
unsafe impl windows_core::Interface for RemovePackageOptions {
    type Vtable = <IRemovePackageOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRemovePackageOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemovePackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.RemovePackageOptions";
}
unsafe impl Send for RemovePackageOptions {}
unsafe impl Sync for RemovePackageOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedPackageContainer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SharedPackageContainer, windows_core::IUnknown, windows_core::IInspectable);
impl SharedPackageContainer {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetMembers(&self) -> windows_core::Result<windows_collections::IVector<SharedPackageContainerMember>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMembers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemovePackageFamily<P1>(&self, packagefamilyname: &windows_core::HSTRING, options: P1) -> windows_core::Result<UpdateSharedPackageContainerResult>
    where
        P1: windows_core::Param<UpdateSharedPackageContainerOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemovePackageFamily)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResetData(&self) -> windows_core::Result<UpdateSharedPackageContainerResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResetData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SharedPackageContainer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISharedPackageContainer>();
}
unsafe impl windows_core::Interface for SharedPackageContainer {
    type Vtable = <ISharedPackageContainer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISharedPackageContainer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SharedPackageContainer {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainer";
}
unsafe impl Send for SharedPackageContainer {}
unsafe impl Sync for SharedPackageContainer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SharedPackageContainerCreationCollisionOptions(pub i32);
impl SharedPackageContainerCreationCollisionOptions {
    pub const FailIfExists: Self = Self(0i32);
    pub const MergeWithExisting: Self = Self(1i32);
    pub const ReplaceExisting: Self = Self(2i32);
}
impl windows_core::TypeKind for SharedPackageContainerCreationCollisionOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SharedPackageContainerCreationCollisionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerCreationCollisionOptions;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedPackageContainerManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SharedPackageContainerManager, windows_core::IUnknown, windows_core::IInspectable);
impl SharedPackageContainerManager {
    pub fn CreateContainer<P1>(&self, name: &windows_core::HSTRING, options: P1) -> windows_core::Result<CreateSharedPackageContainerResult>
    where
        P1: windows_core::Param<CreateSharedPackageContainerOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateContainer)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DeleteContainer<P1>(&self, id: &windows_core::HSTRING, options: P1) -> windows_core::Result<DeleteSharedPackageContainerResult>
    where
        P1: windows_core::Param<DeleteSharedPackageContainerOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteContainer)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetContainer(&self, id: &windows_core::HSTRING) -> windows_core::Result<SharedPackageContainer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContainer)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(id), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindContainers(&self) -> windows_core::Result<windows_collections::IVector<SharedPackageContainer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindContainers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindContainersWithOptions<P0>(&self, options: P0) -> windows_core::Result<windows_collections::IVector<SharedPackageContainer>>
    where
        P0: windows_core::Param<FindSharedPackageContainerOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindContainersWithOptions)(windows_core::Interface::as_raw(this), options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForUser(usersid: &windows_core::HSTRING) -> windows_core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(usersid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForProvisioning() -> windows_core::Result<SharedPackageContainerManager> {
        Self::ISharedPackageContainerManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForProvisioning)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISharedPackageContainerManagerStatics<R, F: FnOnce(&ISharedPackageContainerManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SharedPackageContainerManager, ISharedPackageContainerManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SharedPackageContainerManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISharedPackageContainerManager>();
}
unsafe impl windows_core::Interface for SharedPackageContainerManager {
    type Vtable = <ISharedPackageContainerManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISharedPackageContainerManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SharedPackageContainerManager {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerManager";
}
unsafe impl Send for SharedPackageContainerManager {}
unsafe impl Sync for SharedPackageContainerManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedPackageContainerMember(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SharedPackageContainerMember, windows_core::IUnknown, windows_core::IInspectable);
impl SharedPackageContainerMember {
    pub fn PackageFamilyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PackageFamilyName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<SharedPackageContainerMember> {
        Self::ISharedPackageContainerMemberFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ISharedPackageContainerMemberFactory<R, F: FnOnce(&ISharedPackageContainerMemberFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SharedPackageContainerMember, ISharedPackageContainerMemberFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SharedPackageContainerMember {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISharedPackageContainerMember>();
}
unsafe impl windows_core::Interface for SharedPackageContainerMember {
    type Vtable = <ISharedPackageContainerMember as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISharedPackageContainerMember as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SharedPackageContainerMember {
    const NAME: &'static str = "Windows.Management.Deployment.SharedPackageContainerMember";
}
unsafe impl Send for SharedPackageContainerMember {}
unsafe impl Sync for SharedPackageContainerMember {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SharedPackageContainerOperationStatus(pub i32);
impl SharedPackageContainerOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const BlockedByPolicy: Self = Self(1i32);
    pub const AlreadyExists: Self = Self(2i32);
    pub const PackageFamilyExistsInAnotherContainer: Self = Self(3i32);
    pub const NotFound: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl windows_core::TypeKind for SharedPackageContainerOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for SharedPackageContainerOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.SharedPackageContainerOperationStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StagePackageOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StagePackageOptions, windows_core::IUnknown, windows_core::IInspectable);
impl StagePackageOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StagePackageOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DependencyPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DependencyPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TargetVolume(&self) -> windows_core::Result<PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetVolume)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTargetVolume<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PackageVolume>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTargetVolume)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OptionalPackageFamilyNames(&self) -> windows_core::Result<windows_collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalPackageFamilyNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OptionalPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OptionalPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RelatedPackageUris(&self) -> windows_core::Result<windows_collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RelatedPackageUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExternalLocationUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExternalLocationUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetExternalLocationUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExternalLocationUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StubPackageOption(&self) -> windows_core::Result<StubPackageOption> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StubPackageOption)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStubPackageOption(&self, value: StubPackageOption) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStubPackageOption)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeveloperMode(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeveloperMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeveloperMode(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeveloperMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForceUpdateFromAnyVersion(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceUpdateFromAnyVersion(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceUpdateFromAnyVersion)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallAllResources(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InstallAllResources)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInstallAllResources(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInstallAllResources)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequiredContentGroupOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiredContentGroupOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequiredContentGroupOnly(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequiredContentGroupOnly)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageInPlace(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StageInPlace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStageInPlace(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStageInPlace)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowUnsigned(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowUnsigned)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowUnsigned(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAllowUnsigned)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExpectedDigests(&self) -> windows_core::Result<windows_collections::IMap<super::super::Foundation::Uri, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IStagePackageOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpectedDigests)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StagePackageOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStagePackageOptions>();
}
unsafe impl windows_core::Interface for StagePackageOptions {
    type Vtable = <IStagePackageOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStagePackageOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StagePackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.StagePackageOptions";
}
unsafe impl Send for StagePackageOptions {}
unsafe impl Sync for StagePackageOptions {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StubPackageOption(pub i32);
impl StubPackageOption {
    pub const Default: Self = Self(0i32);
    pub const InstallFull: Self = Self(1i32);
    pub const InstallStub: Self = Self(2i32);
    pub const UsePreference: Self = Self(3i32);
}
impl windows_core::TypeKind for StubPackageOption {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StubPackageOption {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Deployment.StubPackageOption;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateSharedPackageContainerOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateSharedPackageContainerOptions, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateSharedPackageContainerOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateSharedPackageContainerOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ForceAppShutdown(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceAppShutdown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetForceAppShutdown(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetForceAppShutdown)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequirePackagesPresent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequirePackagesPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequirePackagesPresent(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequirePackagesPresent)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for UpdateSharedPackageContainerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateSharedPackageContainerOptions>();
}
unsafe impl windows_core::Interface for UpdateSharedPackageContainerOptions {
    type Vtable = <IUpdateSharedPackageContainerOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateSharedPackageContainerOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerOptions";
}
unsafe impl Send for UpdateSharedPackageContainerOptions {}
unsafe impl Sync for UpdateSharedPackageContainerOptions {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateSharedPackageContainerResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateSharedPackageContainerResult, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateSharedPackageContainerResult {
    pub fn Status(&self) -> windows_core::Result<SharedPackageContainerOperationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for UpdateSharedPackageContainerResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateSharedPackageContainerResult>();
}
unsafe impl windows_core::Interface for UpdateSharedPackageContainerResult {
    type Vtable = <IUpdateSharedPackageContainerResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateSharedPackageContainerResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.UpdateSharedPackageContainerResult";
}
unsafe impl Send for UpdateSharedPackageContainerResult {}
unsafe impl Sync for UpdateSharedPackageContainerResult {}
