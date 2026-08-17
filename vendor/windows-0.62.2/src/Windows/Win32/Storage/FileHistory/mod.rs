#[inline]
pub unsafe fn FhServiceBlockBackup(pipe: FH_SERVICE_PIPE_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("fhsvcctl.dll" "system" fn FhServiceBlockBackup(pipe : FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    unsafe { FhServiceBlockBackup(pipe).ok() }
}
#[inline]
pub unsafe fn FhServiceClosePipe(pipe: FH_SERVICE_PIPE_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("fhsvcctl.dll" "system" fn FhServiceClosePipe(pipe : FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    unsafe { FhServiceClosePipe(pipe).ok() }
}
#[inline]
pub unsafe fn FhServiceOpenPipe(startserviceifstopped: bool) -> windows_core::Result<FH_SERVICE_PIPE_HANDLE> {
    windows_core::link!("fhsvcctl.dll" "system" fn FhServiceOpenPipe(startserviceifstopped : windows_core::BOOL, pipe : *mut FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        FhServiceOpenPipe(startserviceifstopped.into(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn FhServiceReloadConfiguration(pipe: FH_SERVICE_PIPE_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("fhsvcctl.dll" "system" fn FhServiceReloadConfiguration(pipe : FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    unsafe { FhServiceReloadConfiguration(pipe).ok() }
}
#[inline]
pub unsafe fn FhServiceStartBackup(pipe: FH_SERVICE_PIPE_HANDLE, lowpriorityio: bool) -> windows_core::Result<()> {
    windows_core::link!("fhsvcctl.dll" "system" fn FhServiceStartBackup(pipe : FH_SERVICE_PIPE_HANDLE, lowpriorityio : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { FhServiceStartBackup(pipe, lowpriorityio.into()).ok() }
}
#[inline]
pub unsafe fn FhServiceStopBackup(pipe: FH_SERVICE_PIPE_HANDLE, stoptracking: bool) -> windows_core::Result<()> {
    windows_core::link!("fhsvcctl.dll" "system" fn FhServiceStopBackup(pipe : FH_SERVICE_PIPE_HANDLE, stoptracking : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { FhServiceStopBackup(pipe, stoptracking.into()).ok() }
}
#[inline]
pub unsafe fn FhServiceUnblockBackup(pipe: FH_SERVICE_PIPE_HANDLE) -> windows_core::Result<()> {
    windows_core::link!("fhsvcctl.dll" "system" fn FhServiceUnblockBackup(pipe : FH_SERVICE_PIPE_HANDLE) -> windows_core::HRESULT);
    unsafe { FhServiceUnblockBackup(pipe).ok() }
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FH_BACKUP_STATUS(pub i32);
pub const FH_CURRENT_DEFAULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FH_DEVICE_VALIDATION_RESULT(pub i32);
pub const FH_DRIVE_FIXED: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(3i32);
pub const FH_DRIVE_REMOTE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(4i32);
pub const FH_DRIVE_REMOVABLE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(2i32);
pub const FH_DRIVE_UNKNOWN: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(0i32);
pub const FH_FOLDER: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(0i32);
pub const FH_FREQUENCY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(0i32);
pub const FH_INVALID_DRIVE_TYPE: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(1i32);
pub const FH_LIBRARY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FH_LOCAL_POLICY_TYPE(pub i32);
pub const FH_NAMESPACE_EXISTS: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FH_PROTECTED_ITEM_CATEGORY(pub i32);
pub const FH_READ_ONLY_PERMISSION: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(2i32);
pub const FH_RETENTION_AGE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(2i32);
pub const FH_RETENTION_AGE_BASED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(2i32);
pub const FH_RETENTION_DISABLED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(0i32);
pub const FH_RETENTION_TYPE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FH_RETENTION_TYPES(pub i32);
pub const FH_RETENTION_UNLIMITED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(1i32);
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
            windows_core::link!("fhsvcctl.dll" "system" fn FhServiceClosePipe(pipe : *mut core::ffi::c_void) -> i32);
            unsafe {
                FhServiceClosePipe(self.0);
            }
        }
    }
}
impl Default for FH_SERVICE_PIPE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FH_TARGET_DRIVE_TYPES(pub i32);
pub const FH_TARGET_NAME: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(0i32);
pub const FH_TARGET_PART_OF_LIBRARY: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FH_TARGET_PROPERTY_TYPE(pub i32);
pub const FH_TARGET_URL: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(1i32);
pub const FH_VALID_TARGET: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FhBackupStopReason(pub i32);
pub const FhConfigMgr: windows_core::GUID = windows_core::GUID::from_u128(0xed43bb3c_09e9_498a_9df6_2177244c6db4);
pub const FhReassociation: windows_core::GUID = windows_core::GUID::from_u128(0x4d728e35_16fa_4320_9e8b_bfd7100a8846);
windows_core::imp::define_interface!(IFhConfigMgr, IFhConfigMgr_Vtbl, 0x6a5fea5b_bf8f_4ee5_b8c3_44d8a0d7331c);
windows_core::imp::interface_hierarchy!(IFhConfigMgr, windows_core::IUnknown);
impl IFhConfigMgr {
    pub unsafe fn LoadConfiguration(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LoadConfiguration)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn CreateDefaultConfiguration(&self, overwriteifexists: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateDefaultConfiguration)(windows_core::Interface::as_raw(self), overwriteifexists.into()).ok() }
    }
    pub unsafe fn SaveConfiguration(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveConfiguration)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AddRemoveExcludeRule(&self, add: bool, category: FH_PROTECTED_ITEM_CATEGORY, item: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddRemoveExcludeRule)(windows_core::Interface::as_raw(self), add.into(), category, core::mem::transmute_copy(item)).ok() }
    }
    pub unsafe fn GetIncludeExcludeRules(&self, include: bool, category: FH_PROTECTED_ITEM_CATEGORY) -> windows_core::Result<IFhScopeIterator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIncludeExcludeRules)(windows_core::Interface::as_raw(self), include.into(), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalPolicy)(windows_core::Interface::as_raw(self), localpolicytype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalPolicy)(windows_core::Interface::as_raw(self), localpolicytype, policyvalue).ok() }
    }
    pub unsafe fn GetBackupStatus(&self) -> windows_core::Result<FH_BACKUP_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBackupStatus)(windows_core::Interface::as_raw(self), backupstatus).ok() }
    }
    pub unsafe fn GetDefaultTarget(&self) -> windows_core::Result<IFhTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ValidateTarget(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<FH_DEVICE_VALIDATION_RESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateTarget)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targeturl), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProvisionAndSetNewTarget(&self, targeturl: &windows_core::BSTR, targetname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProvisionAndSetNewTarget)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targeturl), core::mem::transmute_copy(targetname)).ok() }
    }
    pub unsafe fn ChangeDefaultTargetRecommendation(&self, recommend: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ChangeDefaultTargetRecommendation)(windows_core::Interface::as_raw(self), recommend.into()).ok() }
    }
    pub unsafe fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryProtectionStatus)(windows_core::Interface::as_raw(self), protectionstate as _, core::mem::transmute(protecteduntiltime)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhConfigMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDefaultConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SaveConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRemoveExcludeRule: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, FH_PROTECTED_ITEM_CATEGORY, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIncludeExcludeRules: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, FH_PROTECTED_ITEM_CATEGORY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocalPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, FH_LOCAL_POLICY_TYPE, *mut u64) -> windows_core::HRESULT,
    pub SetLocalPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, FH_LOCAL_POLICY_TYPE, u64) -> windows_core::HRESULT,
    pub GetBackupStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FH_BACKUP_STATUS) -> windows_core::HRESULT,
    pub SetBackupStatus: unsafe extern "system" fn(*mut core::ffi::c_void, FH_BACKUP_STATUS) -> windows_core::HRESULT,
    pub GetDefaultTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FH_DEVICE_VALIDATION_RESULT) -> windows_core::HRESULT,
    pub ProvisionAndSetNewTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangeDefaultTargetRecommendation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub QueryProtectionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFhConfigMgr_Impl: windows_core::IUnknownImpl {
    fn LoadConfiguration(&self) -> windows_core::Result<()>;
    fn CreateDefaultConfiguration(&self, overwriteifexists: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveConfiguration(&self) -> windows_core::Result<()>;
    fn AddRemoveExcludeRule(&self, add: windows_core::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetIncludeExcludeRules(&self, include: windows_core::BOOL, category: FH_PROTECTED_ITEM_CATEGORY) -> windows_core::Result<IFhScopeIterator>;
    fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> windows_core::Result<u64>;
    fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> windows_core::Result<()>;
    fn GetBackupStatus(&self) -> windows_core::Result<FH_BACKUP_STATUS>;
    fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> windows_core::Result<()>;
    fn GetDefaultTarget(&self) -> windows_core::Result<IFhTarget>;
    fn ValidateTarget(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ProvisionAndSetNewTarget(&self, targeturl: &windows_core::BSTR, targetname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangeDefaultTargetRecommendation(&self, recommend: windows_core::BOOL) -> windows_core::Result<()>;
    fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IFhConfigMgr_Vtbl {
    pub const fn new<Identity: IFhConfigMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadConfiguration<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::LoadConfiguration(this).into()
            }
        }
        unsafe extern "system" fn CreateDefaultConfiguration<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwriteifexists: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::CreateDefaultConfiguration(this, core::mem::transmute_copy(&overwriteifexists)).into()
            }
        }
        unsafe extern "system" fn SaveConfiguration<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::SaveConfiguration(this).into()
            }
        }
        unsafe extern "system" fn AddRemoveExcludeRule<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, add: windows_core::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::AddRemoveExcludeRule(this, core::mem::transmute_copy(&add), core::mem::transmute_copy(&category), core::mem::transmute(&item)).into()
            }
        }
        unsafe extern "system" fn GetIncludeExcludeRules<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, include: windows_core::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhConfigMgr_Impl::GetIncludeExcludeRules(this, core::mem::transmute_copy(&include), core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        iterator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalPolicy<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhConfigMgr_Impl::GetLocalPolicy(this, core::mem::transmute_copy(&localpolicytype)) {
                    Ok(ok__) => {
                        policyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalPolicy<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::SetLocalPolicy(this, core::mem::transmute_copy(&localpolicytype), core::mem::transmute_copy(&policyvalue)).into()
            }
        }
        unsafe extern "system" fn GetBackupStatus<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhConfigMgr_Impl::GetBackupStatus(this) {
                    Ok(ok__) => {
                        backupstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackupStatus<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::SetBackupStatus(this, core::mem::transmute_copy(&backupstatus)).into()
            }
        }
        unsafe extern "system" fn GetDefaultTarget<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaulttarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhConfigMgr_Impl::GetDefaultTarget(this) {
                    Ok(ok__) => {
                        defaulttarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValidateTarget<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturl: *mut core::ffi::c_void, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhConfigMgr_Impl::ValidateTarget(this, core::mem::transmute(&targeturl)) {
                    Ok(ok__) => {
                        validationresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProvisionAndSetNewTarget<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturl: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::ProvisionAndSetNewTarget(this, core::mem::transmute(&targeturl), core::mem::transmute(&targetname)).into()
            }
        }
        unsafe extern "system" fn ChangeDefaultTargetRecommendation<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recommend: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::ChangeDefaultTargetRecommendation(this, core::mem::transmute_copy(&recommend)).into()
            }
        }
        unsafe extern "system" fn QueryProtectionStatus<Identity: IFhConfigMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhConfigMgr_Impl::QueryProtectionStatus(this, core::mem::transmute_copy(&protectionstate), core::mem::transmute_copy(&protecteduntiltime)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadConfiguration: LoadConfiguration::<Identity, OFFSET>,
            CreateDefaultConfiguration: CreateDefaultConfiguration::<Identity, OFFSET>,
            SaveConfiguration: SaveConfiguration::<Identity, OFFSET>,
            AddRemoveExcludeRule: AddRemoveExcludeRule::<Identity, OFFSET>,
            GetIncludeExcludeRules: GetIncludeExcludeRules::<Identity, OFFSET>,
            GetLocalPolicy: GetLocalPolicy::<Identity, OFFSET>,
            SetLocalPolicy: SetLocalPolicy::<Identity, OFFSET>,
            GetBackupStatus: GetBackupStatus::<Identity, OFFSET>,
            SetBackupStatus: SetBackupStatus::<Identity, OFFSET>,
            GetDefaultTarget: GetDefaultTarget::<Identity, OFFSET>,
            ValidateTarget: ValidateTarget::<Identity, OFFSET>,
            ProvisionAndSetNewTarget: ProvisionAndSetNewTarget::<Identity, OFFSET>,
            ChangeDefaultTargetRecommendation: ChangeDefaultTargetRecommendation::<Identity, OFFSET>,
            QueryProtectionStatus: QueryProtectionStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFhConfigMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFhConfigMgr {}
windows_core::imp::define_interface!(IFhReassociation, IFhReassociation_Vtbl, 0x6544a28a_f68d_47ac_91ef_16b2b36aa3ee);
windows_core::imp::interface_hierarchy!(IFhReassociation, windows_core::IUnknown);
impl IFhReassociation {
    pub unsafe fn ValidateTarget(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<FH_DEVICE_VALIDATION_RESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateTarget)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targeturl), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ScanTargetForConfigurations(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ScanTargetForConfigurations)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targeturl)).ok() }
    }
    pub unsafe fn GetConfigurationDetails(&self, index: u32, username: *mut windows_core::BSTR, pcname: *mut windows_core::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConfigurationDetails)(windows_core::Interface::as_raw(self), index, core::mem::transmute(username), core::mem::transmute(pcname), backuptime as _).ok() }
    }
    pub unsafe fn SelectConfiguration(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectConfiguration)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub unsafe fn PerformReassociation(&self, overwriteifexists: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PerformReassociation)(windows_core::Interface::as_raw(self), overwriteifexists.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhReassociation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ValidateTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FH_DEVICE_VALIDATION_RESULT) -> windows_core::HRESULT,
    pub ScanTargetForConfigurations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConfigurationDetails: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SelectConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PerformReassociation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IFhReassociation_Impl: windows_core::IUnknownImpl {
    fn ValidateTarget(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ScanTargetForConfigurations(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetConfigurationDetails(&self, index: u32, username: *mut windows_core::BSTR, pcname: *mut windows_core::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SelectConfiguration(&self, index: u32) -> windows_core::Result<()>;
    fn PerformReassociation(&self, overwriteifexists: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IFhReassociation_Vtbl {
    pub const fn new<Identity: IFhReassociation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ValidateTarget<Identity: IFhReassociation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturl: *mut core::ffi::c_void, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhReassociation_Impl::ValidateTarget(this, core::mem::transmute(&targeturl)) {
                    Ok(ok__) => {
                        validationresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ScanTargetForConfigurations<Identity: IFhReassociation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhReassociation_Impl::ScanTargetForConfigurations(this, core::mem::transmute(&targeturl)).into()
            }
        }
        unsafe extern "system" fn GetConfigurationDetails<Identity: IFhReassociation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, username: *mut *mut core::ffi::c_void, pcname: *mut *mut core::ffi::c_void, backuptime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhReassociation_Impl::GetConfigurationDetails(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&username), core::mem::transmute_copy(&pcname), core::mem::transmute_copy(&backuptime)).into()
            }
        }
        unsafe extern "system" fn SelectConfiguration<Identity: IFhReassociation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhReassociation_Impl::SelectConfiguration(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn PerformReassociation<Identity: IFhReassociation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwriteifexists: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhReassociation_Impl::PerformReassociation(this, core::mem::transmute_copy(&overwriteifexists)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ValidateTarget: ValidateTarget::<Identity, OFFSET>,
            ScanTargetForConfigurations: ScanTargetForConfigurations::<Identity, OFFSET>,
            GetConfigurationDetails: GetConfigurationDetails::<Identity, OFFSET>,
            SelectConfiguration: SelectConfiguration::<Identity, OFFSET>,
            PerformReassociation: PerformReassociation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFhReassociation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFhReassociation {}
windows_core::imp::define_interface!(IFhScopeIterator, IFhScopeIterator_Vtbl, 0x3197abce_532a_44c6_8615_f3666566a720);
windows_core::imp::interface_hierarchy!(IFhScopeIterator, windows_core::IUnknown);
impl IFhScopeIterator {
    pub unsafe fn MoveToNextItem(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MoveToNextItem)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetItem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhScopeIterator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MoveToNextItem: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFhScopeIterator_Impl: windows_core::IUnknownImpl {
    fn MoveToNextItem(&self) -> windows_core::Result<()>;
    fn GetItem(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IFhScopeIterator_Vtbl {
    pub const fn new<Identity: IFhScopeIterator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MoveToNextItem<Identity: IFhScopeIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFhScopeIterator_Impl::MoveToNextItem(this).into()
            }
        }
        unsafe extern "system" fn GetItem<Identity: IFhScopeIterator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhScopeIterator_Impl::GetItem(this) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MoveToNextItem: MoveToNextItem::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFhScopeIterator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFhScopeIterator {}
windows_core::imp::define_interface!(IFhTarget, IFhTarget_Vtbl, 0xd87965fd_2bad_4657_bd3b_9567eb300ced);
windows_core::imp::interface_hierarchy!(IFhTarget, windows_core::IUnknown);
impl IFhTarget {
    pub unsafe fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringProperty)(windows_core::Interface::as_raw(self), propertytype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumericalProperty)(windows_core::Interface::as_raw(self), propertytype, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStringProperty: unsafe extern "system" fn(*mut core::ffi::c_void, FH_TARGET_PROPERTY_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumericalProperty: unsafe extern "system" fn(*mut core::ffi::c_void, FH_TARGET_PROPERTY_TYPE, *mut u64) -> windows_core::HRESULT,
}
pub trait IFhTarget_Impl: windows_core::IUnknownImpl {
    fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> windows_core::Result<windows_core::BSTR>;
    fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> windows_core::Result<u64>;
}
impl IFhTarget_Vtbl {
    pub const fn new<Identity: IFhTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStringProperty<Identity: IFhTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhTarget_Impl::GetStringProperty(this, core::mem::transmute_copy(&propertytype)) {
                    Ok(ok__) => {
                        propertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumericalProperty<Identity: IFhTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFhTarget_Impl::GetNumericalProperty(this, core::mem::transmute_copy(&propertytype)) {
                    Ok(ok__) => {
                        propertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStringProperty: GetStringProperty::<Identity, OFFSET>,
            GetNumericalProperty: GetNumericalProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFhTarget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFhTarget {}
pub const MAX_BACKUP_STATUS: FH_BACKUP_STATUS = FH_BACKUP_STATUS(4i32);
pub const MAX_LOCAL_POLICY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(3i32);
pub const MAX_PROTECTED_ITEM_CATEGORY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(2i32);
pub const MAX_RETENTION_TYPE: FH_RETENTION_TYPES = FH_RETENTION_TYPES(3i32);
pub const MAX_TARGET_PROPERTY: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(3i32);
pub const MAX_VALIDATION_RESULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(7i32);
