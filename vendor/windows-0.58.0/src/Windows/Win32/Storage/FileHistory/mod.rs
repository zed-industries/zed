#[inline]
pub unsafe fn FhServiceBlockBackup<P0>(pipe: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<FH_SERVICE_PIPE_HANDLE>,
{
    windows_targets::link!("fhsvcctl.dll" "system" fn FhServiceBlockBackup(pipe : FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    FhServiceBlockBackup(pipe.param().abi()).ok()
}
#[inline]
pub unsafe fn FhServiceClosePipe<P0>(pipe: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<FH_SERVICE_PIPE_HANDLE>,
{
    windows_targets::link!("fhsvcctl.dll" "system" fn FhServiceClosePipe(pipe : FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    FhServiceClosePipe(pipe.param().abi()).ok()
}
#[inline]
pub unsafe fn FhServiceOpenPipe<P0>(startserviceifstopped: P0) -> windows_core::Result<FH_SERVICE_PIPE_HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("fhsvcctl.dll" "system" fn FhServiceOpenPipe(startserviceifstopped : super::super::Foundation:: BOOL, pipe : *mut FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    FhServiceOpenPipe(startserviceifstopped.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FhServiceReloadConfiguration<P0>(pipe: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<FH_SERVICE_PIPE_HANDLE>,
{
    windows_targets::link!("fhsvcctl.dll" "system" fn FhServiceReloadConfiguration(pipe : FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    FhServiceReloadConfiguration(pipe.param().abi()).ok()
}
#[inline]
pub unsafe fn FhServiceStartBackup<P0, P1>(pipe: P0, lowpriorityio: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<FH_SERVICE_PIPE_HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("fhsvcctl.dll" "system" fn FhServiceStartBackup(pipe : FH_SERVICE_PIPE_HANDLE, lowpriorityio : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    FhServiceStartBackup(pipe.param().abi(), lowpriorityio.param().abi()).ok()
}
#[inline]
pub unsafe fn FhServiceStopBackup<P0, P1>(pipe: P0, stoptracking: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<FH_SERVICE_PIPE_HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("fhsvcctl.dll" "system" fn FhServiceStopBackup(pipe : FH_SERVICE_PIPE_HANDLE, stoptracking : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    FhServiceStopBackup(pipe.param().abi(), stoptracking.param().abi()).ok()
}
#[inline]
pub unsafe fn FhServiceUnblockBackup<P0>(pipe: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<FH_SERVICE_PIPE_HANDLE>,
{
    windows_targets::link!("fhsvcctl.dll" "system" fn FhServiceUnblockBackup(pipe : FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    FhServiceUnblockBackup(pipe.param().abi()).ok()
}
windows_core::imp::define_interface!(IFhConfigMgr, IFhConfigMgr_Vtbl, 0x6a5fea5b_bf8f_4ee5_b8c3_44d8a0d7331c);
impl core::ops::Deref for IFhConfigMgr {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFhConfigMgr, windows_core::IUnknown);
impl IFhConfigMgr {
    pub unsafe fn LoadConfiguration(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadConfiguration)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDefaultConfiguration<P0>(&self, overwriteifexists: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).CreateDefaultConfiguration)(windows_core::Interface::as_raw(self), overwriteifexists.param().abi()).ok()
    }
    pub unsafe fn SaveConfiguration(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveConfiguration)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddRemoveExcludeRule<P0, P1>(&self, add: P0, category: FH_PROTECTED_ITEM_CATEGORY, item: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddRemoveExcludeRule)(windows_core::Interface::as_raw(self), add.param().abi(), category, item.param().abi()).ok()
    }
    pub unsafe fn GetIncludeExcludeRules<P0>(&self, include: P0, category: FH_PROTECTED_ITEM_CATEGORY) -> windows_core::Result<IFhScopeIterator>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIncludeExcludeRules)(windows_core::Interface::as_raw(self), include.param().abi(), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocalPolicy)(windows_core::Interface::as_raw(self), localpolicytype, &mut result__).map(|| result__)
    }
    pub unsafe fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLocalPolicy)(windows_core::Interface::as_raw(self), localpolicytype, policyvalue).ok()
    }
    pub unsafe fn GetBackupStatus(&self) -> windows_core::Result<FH_BACKUP_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBackupStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBackupStatus)(windows_core::Interface::as_raw(self), backupstatus).ok()
    }
    pub unsafe fn GetDefaultTarget(&self) -> windows_core::Result<IFhTarget> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ValidateTarget<P0>(&self, targeturl: P0) -> windows_core::Result<FH_DEVICE_VALIDATION_RESULT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValidateTarget)(windows_core::Interface::as_raw(self), targeturl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ProvisionAndSetNewTarget<P0, P1>(&self, targeturl: P0, targetname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ProvisionAndSetNewTarget)(windows_core::Interface::as_raw(self), targeturl.param().abi(), targetname.param().abi()).ok()
    }
    pub unsafe fn ChangeDefaultTargetRecommendation<P0>(&self, recommend: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ChangeDefaultTargetRecommendation)(windows_core::Interface::as_raw(self), recommend.param().abi()).ok()
    }
    pub unsafe fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryProtectionStatus)(windows_core::Interface::as_raw(self), protectionstate, core::mem::transmute(protecteduntiltime)).ok()
    }
}
#[repr(C)]
pub struct IFhConfigMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDefaultConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SaveConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRemoveExcludeRule: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, FH_PROTECTED_ITEM_CATEGORY, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetIncludeExcludeRules: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, FH_PROTECTED_ITEM_CATEGORY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, FH_LOCAL_POLICY_TYPE, *mut u64) -> windows_core::HRESULT,
    pub SetLocalPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, FH_LOCAL_POLICY_TYPE, u64) -> windows_core::HRESULT,
    pub GetBackupStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FH_BACKUP_STATUS) -> windows_core::HRESULT,
    pub SetBackupStatus: unsafe extern "system" fn(*mut core::ffi::c_void, FH_BACKUP_STATUS) -> windows_core::HRESULT,
    pub GetDefaultTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateTarget: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut FH_DEVICE_VALIDATION_RESULT) -> windows_core::HRESULT,
    pub ProvisionAndSetNewTarget: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ChangeDefaultTargetRecommendation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub QueryProtectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFhReassociation, IFhReassociation_Vtbl, 0x6544a28a_f68d_47ac_91ef_16b2b36aa3ee);
impl core::ops::Deref for IFhReassociation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFhReassociation, windows_core::IUnknown);
impl IFhReassociation {
    pub unsafe fn ValidateTarget<P0>(&self, targeturl: P0) -> windows_core::Result<FH_DEVICE_VALIDATION_RESULT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ValidateTarget)(windows_core::Interface::as_raw(self), targeturl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ScanTargetForConfigurations<P0>(&self, targeturl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ScanTargetForConfigurations)(windows_core::Interface::as_raw(self), targeturl.param().abi()).ok()
    }
    pub unsafe fn GetConfigurationDetails(&self, index: u32, username: *mut windows_core::BSTR, pcname: *mut windows_core::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConfigurationDetails)(windows_core::Interface::as_raw(self), index, core::mem::transmute(username), core::mem::transmute(pcname), backuptime).ok()
    }
    pub unsafe fn SelectConfiguration(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SelectConfiguration)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn PerformReassociation<P0>(&self, overwriteifexists: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PerformReassociation)(windows_core::Interface::as_raw(self), overwriteifexists.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFhReassociation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ValidateTarget: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut FH_DEVICE_VALIDATION_RESULT) -> windows_core::HRESULT,
    pub ScanTargetForConfigurations: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetConfigurationDetails: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SelectConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PerformReassociation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFhScopeIterator, IFhScopeIterator_Vtbl, 0x3197abce_532a_44c6_8615_f3666566a720);
impl core::ops::Deref for IFhScopeIterator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFhScopeIterator, windows_core::IUnknown);
impl IFhScopeIterator {
    pub unsafe fn MoveToNextItem(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MoveToNextItem)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetItem(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IFhScopeIterator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveToNextItem: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFhTarget, IFhTarget_Vtbl, 0xd87965fd_2bad_4657_bd3b_9567eb300ced);
impl core::ops::Deref for IFhTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFhTarget, windows_core::IUnknown);
impl IFhTarget {
    pub unsafe fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStringProperty)(windows_core::Interface::as_raw(self), propertytype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumericalProperty)(windows_core::Interface::as_raw(self), propertytype, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IFhTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStringProperty: unsafe extern "system" fn(*mut core::ffi::c_void, FH_TARGET_PROPERTY_TYPE, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetNumericalProperty: unsafe extern "system" fn(*mut core::ffi::c_void, FH_TARGET_PROPERTY_TYPE, *mut u64) -> windows_core::HRESULT,
}
pub const BackupCancelled: FhBackupStopReason = FhBackupStopReason(4i32);
pub const BackupInvalidStopReason: FhBackupStopReason = FhBackupStopReason(0i32);
pub const BackupLimitUserBusyMachineOnAC: FhBackupStopReason = FhBackupStopReason(1i32);
pub const BackupLimitUserBusyMachineOnDC: FhBackupStopReason = FhBackupStopReason(3i32);
pub const BackupLimitUserIdleMachineOnDC: FhBackupStopReason = FhBackupStopReason(2i32);
pub const FHCFG_E_CONFIGURATION_PREVIOUSLY_LOADED: windows_core::HRESULT = windows_core::HRESULT(0x80040305_u32 as _);
pub const FHCFG_E_CONFIG_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x80040302_u32 as _);
pub const FHCFG_E_CONFIG_FILE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80040301_u32 as _);
pub const FHCFG_E_CORRUPT_CONFIG_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80040300_u32 as _);
pub const FHCFG_E_INVALID_REHYDRATION_STATE: windows_core::HRESULT = windows_core::HRESULT(0x8004030A_u32 as _);
pub const FHCFG_E_LEGACY_BACKUP_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80040315_u32 as _);
pub const FHCFG_E_LEGACY_BACKUP_USER_EXCLUDED: windows_core::HRESULT = windows_core::HRESULT(0x80040314_u32 as _);
pub const FHCFG_E_LEGACY_TARGET_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040312_u32 as _);
pub const FHCFG_E_LEGACY_TARGET_VALIDATION_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80040313_u32 as _);
pub const FHCFG_E_NO_VALID_CONFIGURATION_LOADED: windows_core::HRESULT = windows_core::HRESULT(0x80040303_u32 as _);
pub const FHCFG_E_RECOMMENDATION_CHANGE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80040310_u32 as _);
pub const FHCFG_E_TARGET_CANNOT_BE_USED: windows_core::HRESULT = windows_core::HRESULT(0x80040309_u32 as _);
pub const FHCFG_E_TARGET_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x80040307_u32 as _);
pub const FHCFG_E_TARGET_NOT_CONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x80040304_u32 as _);
pub const FHCFG_E_TARGET_NOT_ENOUGH_FREE_SPACE: windows_core::HRESULT = windows_core::HRESULT(0x80040308_u32 as _);
pub const FHCFG_E_TARGET_REHYDRATED_ELSEWHERE: windows_core::HRESULT = windows_core::HRESULT(0x80040311_u32 as _);
pub const FHCFG_E_TARGET_VERIFICATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80040306_u32 as _);
pub const FHSVC_E_BACKUP_BLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80040600_u32 as _);
pub const FHSVC_E_CONFIG_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80040602_u32 as _);
pub const FHSVC_E_CONFIG_DISABLED_GP: windows_core::HRESULT = windows_core::HRESULT(0x80040603_u32 as _);
pub const FHSVC_E_CONFIG_REHYDRATING: windows_core::HRESULT = windows_core::HRESULT(0x80040605_u32 as _);
pub const FHSVC_E_FATAL_CONFIG_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80040604_u32 as _);
pub const FHSVC_E_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x80040601_u32 as _);
pub const FH_ACCESS_DENIED: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(0i32);
pub const FH_CURRENT_DEFAULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(3i32);
pub const FH_DRIVE_FIXED: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(3i32);
pub const FH_DRIVE_REMOTE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(4i32);
pub const FH_DRIVE_REMOVABLE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(2i32);
pub const FH_DRIVE_UNKNOWN: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(0i32);
pub const FH_FOLDER: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(0i32);
pub const FH_FREQUENCY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(0i32);
pub const FH_INVALID_DRIVE_TYPE: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(1i32);
pub const FH_LIBRARY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(1i32);
pub const FH_NAMESPACE_EXISTS: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(4i32);
pub const FH_READ_ONLY_PERMISSION: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(2i32);
pub const FH_RETENTION_AGE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(2i32);
pub const FH_RETENTION_AGE_BASED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(2i32);
pub const FH_RETENTION_DISABLED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(0i32);
pub const FH_RETENTION_TYPE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(1i32);
pub const FH_RETENTION_UNLIMITED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(1i32);
pub const FH_STATE_BACKUP_NOT_SUPPORTED: u32 = 2064u32;
pub const FH_STATE_DISABLED_BY_GP: u32 = 2u32;
pub const FH_STATE_FATAL_CONFIG_ERROR: u32 = 3u32;
pub const FH_STATE_MIGRATING: u32 = 4u32;
pub const FH_STATE_NOT_TRACKED: u32 = 0u32;
pub const FH_STATE_NO_ERROR: u32 = 255u32;
pub const FH_STATE_OFF: u32 = 1u32;
pub const FH_STATE_REHYDRATING: u32 = 5u32;
pub const FH_STATE_RUNNING: u32 = 256u32;
pub const FH_STATE_STAGING_FULL: u32 = 18u32;
pub const FH_STATE_TARGET_ABSENT: u32 = 21u32;
pub const FH_STATE_TARGET_ACCESS_DENIED: u32 = 14u32;
pub const FH_STATE_TARGET_FS_LIMITATION: u32 = 13u32;
pub const FH_STATE_TARGET_FULL: u32 = 17u32;
pub const FH_STATE_TARGET_FULL_RETENTION_MAX: u32 = 16u32;
pub const FH_STATE_TARGET_LOW_SPACE: u32 = 20u32;
pub const FH_STATE_TARGET_LOW_SPACE_RETENTION_MAX: u32 = 19u32;
pub const FH_STATE_TARGET_VOLUME_DIRTY: u32 = 15u32;
pub const FH_STATE_TOO_MUCH_BEHIND: u32 = 240u32;
pub const FH_STATUS_DISABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(0i32);
pub const FH_STATUS_DISABLED_BY_GP: FH_BACKUP_STATUS = FH_BACKUP_STATUS(1i32);
pub const FH_STATUS_ENABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(2i32);
pub const FH_STATUS_REHYDRATING: FH_BACKUP_STATUS = FH_BACKUP_STATUS(3i32);
pub const FH_TARGET_DRIVE_TYPE: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(2i32);
pub const FH_TARGET_NAME: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(0i32);
pub const FH_TARGET_PART_OF_LIBRARY: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(5i32);
pub const FH_TARGET_URL: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(1i32);
pub const FH_VALID_TARGET: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(6i32);
pub const MAX_BACKUP_STATUS: FH_BACKUP_STATUS = FH_BACKUP_STATUS(4i32);
pub const MAX_LOCAL_POLICY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(3i32);
pub const MAX_PROTECTED_ITEM_CATEGORY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(2i32);
pub const MAX_RETENTION_TYPE: FH_RETENTION_TYPES = FH_RETENTION_TYPES(3i32);
pub const MAX_TARGET_PROPERTY: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(3i32);
pub const MAX_VALIDATION_RESULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(7i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FH_BACKUP_STATUS(pub i32);
impl windows_core::TypeKind for FH_BACKUP_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FH_BACKUP_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FH_BACKUP_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FH_DEVICE_VALIDATION_RESULT(pub i32);
impl windows_core::TypeKind for FH_DEVICE_VALIDATION_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FH_DEVICE_VALIDATION_RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FH_DEVICE_VALIDATION_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FH_LOCAL_POLICY_TYPE(pub i32);
impl windows_core::TypeKind for FH_LOCAL_POLICY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FH_LOCAL_POLICY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FH_LOCAL_POLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FH_PROTECTED_ITEM_CATEGORY(pub i32);
impl windows_core::TypeKind for FH_PROTECTED_ITEM_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FH_PROTECTED_ITEM_CATEGORY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FH_PROTECTED_ITEM_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FH_RETENTION_TYPES(pub i32);
impl windows_core::TypeKind for FH_RETENTION_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FH_RETENTION_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FH_RETENTION_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FH_TARGET_DRIVE_TYPES(pub i32);
impl windows_core::TypeKind for FH_TARGET_DRIVE_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FH_TARGET_DRIVE_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FH_TARGET_DRIVE_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FH_TARGET_PROPERTY_TYPE(pub i32);
impl windows_core::TypeKind for FH_TARGET_PROPERTY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FH_TARGET_PROPERTY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FH_TARGET_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FhBackupStopReason(pub i32);
impl windows_core::TypeKind for FhBackupStopReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FhBackupStopReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FhBackupStopReason").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FH_SERVICE_PIPE_HANDLE(pub *mut core::ffi::c_void);
impl FH_SERVICE_PIPE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for FH_SERVICE_PIPE_HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = FhServiceClosePipe(*self);
        }
    }
}
impl Default for FH_SERVICE_PIPE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FH_SERVICE_PIPE_HANDLE {
    type TypeKind = windows_core::CopyType;
}
pub const FhConfigMgr: windows_core::GUID = windows_core::GUID::from_u128(0xed43bb3c_09e9_498a_9df6_2177244c6db4);
pub const FhReassociation: windows_core::GUID = windows_core::GUID::from_u128(0x4d728e35_16fa_4320_9e8b_bfd7100a8846);
#[cfg(feature = "implement")]
core::include!("impl.rs");
