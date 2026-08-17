pub trait IFhConfigMgr_Impl: Sized {
    fn LoadConfiguration(&self) -> windows_core::Result<()>;
    fn CreateDefaultConfiguration(&self, overwriteifexists: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SaveConfiguration(&self) -> windows_core::Result<()>;
    fn AddRemoveExcludeRule(&self, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetIncludeExcludeRules(&self, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY) -> windows_core::Result<IFhScopeIterator>;
    fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> windows_core::Result<u64>;
    fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> windows_core::Result<()>;
    fn GetBackupStatus(&self) -> windows_core::Result<FH_BACKUP_STATUS>;
    fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> windows_core::Result<()>;
    fn GetDefaultTarget(&self) -> windows_core::Result<IFhTarget>;
    fn ValidateTarget(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ProvisionAndSetNewTarget(&self, targeturl: &windows_core::BSTR, targetname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangeDefaultTargetRecommendation(&self, recommend: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFhConfigMgr {}
impl IFhConfigMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFhConfigMgr_Vtbl
    where
        Identity: IFhConfigMgr_Impl,
    {
        unsafe extern "system" fn LoadConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::LoadConfiguration(this).into()
        }
        unsafe extern "system" fn CreateDefaultConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::CreateDefaultConfiguration(this, core::mem::transmute_copy(&overwriteifexists)).into()
        }
        unsafe extern "system" fn SaveConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::SaveConfiguration(this).into()
        }
        unsafe extern "system" fn AddRemoveExcludeRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::AddRemoveExcludeRule(this, core::mem::transmute_copy(&add), core::mem::transmute_copy(&category), core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn GetIncludeExcludeRules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhConfigMgr_Impl::GetIncludeExcludeRules(this, core::mem::transmute_copy(&include), core::mem::transmute_copy(&category)) {
                Ok(ok__) => {
                    iterator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhConfigMgr_Impl::GetLocalPolicy(this, core::mem::transmute_copy(&localpolicytype)) {
                Ok(ok__) => {
                    policyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::SetLocalPolicy(this, core::mem::transmute_copy(&localpolicytype), core::mem::transmute_copy(&policyvalue)).into()
        }
        unsafe extern "system" fn GetBackupStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhConfigMgr_Impl::GetBackupStatus(this) {
                Ok(ok__) => {
                    backupstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackupStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::SetBackupStatus(this, core::mem::transmute_copy(&backupstatus)).into()
        }
        unsafe extern "system" fn GetDefaultTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, defaulttarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhConfigMgr_Impl::GetDefaultTarget(this) {
                Ok(ok__) => {
                    defaulttarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturl: core::mem::MaybeUninit<windows_core::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhConfigMgr_Impl::ValidateTarget(this, core::mem::transmute(&targeturl)) {
                Ok(ok__) => {
                    validationresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvisionAndSetNewTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturl: core::mem::MaybeUninit<windows_core::BSTR>, targetname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::ProvisionAndSetNewTarget(this, core::mem::transmute(&targeturl), core::mem::transmute(&targetname)).into()
        }
        unsafe extern "system" fn ChangeDefaultTargetRecommendation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, recommend: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::ChangeDefaultTargetRecommendation(this, core::mem::transmute_copy(&recommend)).into()
        }
        unsafe extern "system" fn QueryProtectionStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFhConfigMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhConfigMgr_Impl::QueryProtectionStatus(this, core::mem::transmute_copy(&protectionstate), core::mem::transmute_copy(&protecteduntiltime)).into()
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
pub trait IFhReassociation_Impl: Sized {
    fn ValidateTarget(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<FH_DEVICE_VALIDATION_RESULT>;
    fn ScanTargetForConfigurations(&self, targeturl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetConfigurationDetails(&self, index: u32, username: *mut windows_core::BSTR, pcname: *mut windows_core::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SelectConfiguration(&self, index: u32) -> windows_core::Result<()>;
    fn PerformReassociation(&self, overwriteifexists: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFhReassociation {}
impl IFhReassociation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFhReassociation_Vtbl
    where
        Identity: IFhReassociation_Impl,
    {
        unsafe extern "system" fn ValidateTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturl: core::mem::MaybeUninit<windows_core::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> windows_core::HRESULT
        where
            Identity: IFhReassociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhReassociation_Impl::ValidateTarget(this, core::mem::transmute(&targeturl)) {
                Ok(ok__) => {
                    validationresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanTargetForConfigurations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targeturl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFhReassociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhReassociation_Impl::ScanTargetForConfigurations(this, core::mem::transmute(&targeturl)).into()
        }
        unsafe extern "system" fn GetConfigurationDetails<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, username: *mut core::mem::MaybeUninit<windows_core::BSTR>, pcname: *mut core::mem::MaybeUninit<windows_core::BSTR>, backuptime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IFhReassociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhReassociation_Impl::GetConfigurationDetails(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&username), core::mem::transmute_copy(&pcname), core::mem::transmute_copy(&backuptime)).into()
        }
        unsafe extern "system" fn SelectConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IFhReassociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhReassociation_Impl::SelectConfiguration(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn PerformReassociation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFhReassociation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhReassociation_Impl::PerformReassociation(this, core::mem::transmute_copy(&overwriteifexists)).into()
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
pub trait IFhScopeIterator_Impl: Sized {
    fn MoveToNextItem(&self) -> windows_core::Result<()>;
    fn GetItem(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for IFhScopeIterator {}
impl IFhScopeIterator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFhScopeIterator_Vtbl
    where
        Identity: IFhScopeIterator_Impl,
    {
        unsafe extern "system" fn MoveToNextItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFhScopeIterator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFhScopeIterator_Impl::MoveToNextItem(this).into()
        }
        unsafe extern "system" fn GetItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFhScopeIterator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhScopeIterator_Impl::GetItem(this) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IFhTarget_Impl: Sized {
    fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> windows_core::Result<windows_core::BSTR>;
    fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IFhTarget {}
impl IFhTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFhTarget_Vtbl
    where
        Identity: IFhTarget_Impl,
    {
        unsafe extern "system" fn GetStringProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFhTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhTarget_Impl::GetStringProperty(this, core::mem::transmute_copy(&propertytype)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumericalProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> windows_core::HRESULT
        where
            Identity: IFhTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFhTarget_Impl::GetNumericalProperty(this, core::mem::transmute_copy(&propertytype)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
