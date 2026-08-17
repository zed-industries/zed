#[cfg(feature = "Win32_System_Registry")]
pub trait IGPEInformation_Impl: Sized {
    fn GetName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> windows_core::Result<()>;
    fn GetDSPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetOptions(&self, dwoptions: *mut u32) -> windows_core::Result<()>;
    fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::Result<()>;
    fn GetHint(&self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> windows_core::Result<()>;
    fn PolicyChanged(&self, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IGPEInformation {}
#[cfg(feature = "Win32_System_Registry")]
impl IGPEInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>() -> IGPEInformation_Vtbl {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::GetDisplayName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetRegistryKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::GetRegistryKey(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&hkey)).into()
        }
        unsafe extern "system" fn GetDSPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::GetDSPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetFileSysPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::GetFileSysPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::GetOptions(this, core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::GetType(this, core::mem::transmute_copy(&gpotype)).into()
        }
        unsafe extern "system" fn GetHint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::GetHint(this, core::mem::transmute_copy(&gphint)).into()
        }
        unsafe extern "system" fn PolicyChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPEInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut windows_core::GUID, pguidsnapin: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPEInformation_Impl::PolicyChanged(this, core::mem::transmute_copy(&bmachine), core::mem::transmute_copy(&badd), core::mem::transmute_copy(&pguidextension), core::mem::transmute_copy(&pguidsnapin)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, Impl, OFFSET>,
            GetDSPath: GetDSPath::<Identity, Impl, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetHint: GetHint::<Identity, Impl, OFFSET>,
            PolicyChanged: PolicyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPEInformation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPM_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDomain(&self, bstrdomain: &windows_core::BSTR, bstrdomaincontroller: &windows_core::BSTR, ldcflags: i32) -> windows_core::Result<IGPMDomain>;
    fn GetBackupDir(&self, bstrbackupdir: &windows_core::BSTR) -> windows_core::Result<IGPMBackupDir>;
    fn GetSitesContainer(&self, bstrforest: &windows_core::BSTR, bstrdomain: &windows_core::BSTR, bstrdomaincontroller: &windows_core::BSTR, ldcflags: i32) -> windows_core::Result<IGPMSitesContainer>;
    fn GetRSOP(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: &windows_core::BSTR, lflags: i32) -> windows_core::Result<IGPMRSOP>;
    fn CreatePermission(&self, bstrtrustee: &windows_core::BSTR, perm: GPMPermissionType, binheritable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IGPMPermission>;
    fn CreateSearchCriteria(&self) -> windows_core::Result<IGPMSearchCriteria>;
    fn CreateTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<IGPMTrustee>;
    fn GetClientSideExtensions(&self) -> windows_core::Result<IGPMCSECollection>;
    fn GetConstants(&self) -> windows_core::Result<IGPMConstants>;
    fn GetMigrationTable(&self, bstrmigrationtablepath: &windows_core::BSTR) -> windows_core::Result<IGPMMigrationTable>;
    fn CreateMigrationTable(&self) -> windows_core::Result<IGPMMigrationTable>;
    fn InitializeReporting(&self, bstradmpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPM {}
#[cfg(feature = "Win32_System_Com")]
impl IGPM_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>() -> IGPM_Vtbl {
        unsafe extern "system" fn GetDomain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdomain: core::mem::MaybeUninit<windows_core::BSTR>, bstrdomaincontroller: core::mem::MaybeUninit<windows_core::BSTR>, ldcflags: i32, pigpmdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::GetDomain(this, core::mem::transmute(&bstrdomain), core::mem::transmute(&bstrdomaincontroller), core::mem::transmute_copy(&ldcflags)) {
                Ok(ok__) => {
                    core::ptr::write(pigpmdomain, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackupDir<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupdir: core::mem::MaybeUninit<windows_core::BSTR>, pigpmbackupdir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::GetBackupDir(this, core::mem::transmute(&bstrbackupdir)) {
                Ok(ok__) => {
                    core::ptr::write(pigpmbackupdir, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSitesContainer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrforest: core::mem::MaybeUninit<windows_core::BSTR>, bstrdomain: core::mem::MaybeUninit<windows_core::BSTR>, bstrdomaincontroller: core::mem::MaybeUninit<windows_core::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::GetSitesContainer(this, core::mem::transmute(&bstrforest), core::mem::transmute(&bstrdomain), core::mem::transmute(&bstrdomaincontroller), core::mem::transmute_copy(&ldcflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmsitescontainer, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRSOP<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: core::mem::MaybeUninit<windows_core::BSTR>, lflags: i32, ppigpmrsop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::GetRSOP(this, core::mem::transmute_copy(&gpmrsopmode), core::mem::transmute(&bstrnamespace), core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmrsop, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePermission<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: core::mem::MaybeUninit<windows_core::BSTR>, perm: GPMPermissionType, binheritable: super::super::Foundation::VARIANT_BOOL, ppperm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::CreatePermission(this, core::mem::transmute(&bstrtrustee), core::mem::transmute_copy(&perm), core::mem::transmute_copy(&binheritable)) {
                Ok(ok__) => {
                    core::ptr::write(ppperm, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSearchCriteria<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmsearchcriteria: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::CreateSearchCriteria(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmsearchcriteria, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrustee<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: core::mem::MaybeUninit<windows_core::BSTR>, ppigpmtrustee: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::CreateTrustee(this, core::mem::transmute(&bstrtrustee)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmtrustee, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientSideExtensions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmcsecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::GetClientSideExtensions(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmcsecollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmconstants: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::GetConstants(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmconstants, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMigrationTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmigrationtablepath: core::mem::MaybeUninit<windows_core::BSTR>, ppmigrationtable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::GetMigrationTable(this, core::mem::transmute(&bstrmigrationtablepath)) {
                Ok(ok__) => {
                    core::ptr::write(ppmigrationtable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMigrationTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmigrationtable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM_Impl::CreateMigrationTable(this) {
                Ok(ok__) => {
                    core::ptr::write(ppmigrationtable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeReporting<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPM_Impl::InitializeReporting(this, core::mem::transmute(&bstradmpath)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDomain: GetDomain::<Identity, Impl, OFFSET>,
            GetBackupDir: GetBackupDir::<Identity, Impl, OFFSET>,
            GetSitesContainer: GetSitesContainer::<Identity, Impl, OFFSET>,
            GetRSOP: GetRSOP::<Identity, Impl, OFFSET>,
            CreatePermission: CreatePermission::<Identity, Impl, OFFSET>,
            CreateSearchCriteria: CreateSearchCriteria::<Identity, Impl, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, Impl, OFFSET>,
            GetClientSideExtensions: GetClientSideExtensions::<Identity, Impl, OFFSET>,
            GetConstants: GetConstants::<Identity, Impl, OFFSET>,
            GetMigrationTable: GetMigrationTable::<Identity, Impl, OFFSET>,
            CreateMigrationTable: CreateMigrationTable::<Identity, Impl, OFFSET>,
            InitializeReporting: InitializeReporting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPM as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPM2_Impl: Sized + IGPM_Impl {
    fn GetBackupDirEx(&self, bstrbackupdir: &windows_core::BSTR, backupdirtype: GPMBackupType) -> windows_core::Result<IGPMBackupDirEx>;
    fn InitializeReportingEx(&self, bstradmpath: &windows_core::BSTR, reportingoptions: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPM2 {}
#[cfg(feature = "Win32_System_Com")]
impl IGPM2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: isize>() -> IGPM2_Vtbl {
        unsafe extern "system" fn GetBackupDirEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupdir: core::mem::MaybeUninit<windows_core::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPM2_Impl::GetBackupDirEx(this, core::mem::transmute(&bstrbackupdir), core::mem::transmute_copy(&backupdirtype)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmbackupdirex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeReportingEx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPM2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradmpath: core::mem::MaybeUninit<windows_core::BSTR>, reportingoptions: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPM2_Impl::InitializeReportingEx(this, core::mem::transmute(&bstradmpath), core::mem::transmute_copy(&reportingoptions)).into()
        }
        Self {
            base__: IGPM_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetBackupDirEx: GetBackupDirEx::<Identity, Impl, OFFSET>,
            InitializeReportingEx: InitializeReportingEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPM2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPM as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMAsyncCancel_Impl: Sized + super::Com::IDispatch_Impl {
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMAsyncCancel {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMAsyncCancel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMAsyncCancel_Impl, const OFFSET: isize>() -> IGPMAsyncCancel_Vtbl {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMAsyncCancel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMAsyncCancel_Impl::Cancel(this).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Cancel: Cancel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMAsyncCancel as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMAsyncProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn Status(&self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: windows_core::HRESULT, presult: *const windows_core::VARIANT, ppigpmstatusmsgcollection: Option<&IGPMStatusMsgCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMAsyncProgress {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMAsyncProgress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMAsyncProgress_Impl, const OFFSET: isize>() -> IGPMAsyncProgress_Vtbl {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMAsyncProgress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: windows_core::HRESULT, presult: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmstatusmsgcollection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMAsyncProgress_Impl::Status(this, core::mem::transmute_copy(&lprogressnumerator), core::mem::transmute_copy(&lprogressdenominator), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&presult), windows_core::from_raw_borrowed(&ppigpmstatusmsgcollection)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Status: Status::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMAsyncProgress as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMBackup_Impl: Sized + super::Com::IDispatch_Impl {
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GPOID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GPODomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GPODisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Timestamp(&self) -> windows_core::Result<f64>;
    fn Comment(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMBackup {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>() -> IGPMBackup_Vtbl {
        unsafe extern "system" fn ID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::ID(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPOID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::GPOID(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODomain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::GPODomain(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::GPODisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::Timestamp(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::Comment(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupDir<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::BackupDir(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMBackup_Impl::Delete(this).into()
        }
        unsafe extern "system" fn GenerateReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: core::mem::MaybeUninit<windows_core::BSTR>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackup_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ID: ID::<Identity, Impl, OFFSET>,
            GPOID: GPOID::<Identity, Impl, OFFSET>,
            GPODomain: GPODomain::<Identity, Impl, OFFSET>,
            GPODisplayName: GPODisplayName::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMBackup as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMBackupCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMBackupCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMBackupCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>() -> IGPMBackupCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmbackup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMBackupCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMBackupDir_Impl: Sized + super::Com::IDispatch_Impl {
    fn BackupDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetBackup(&self, bstrid: &windows_core::BSTR) -> windows_core::Result<IGPMBackup>;
    fn SearchBackups(&self, pigpmsearchcriteria: Option<&IGPMSearchCriteria>) -> windows_core::Result<IGPMBackupCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMBackupDir {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupDir_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: isize>() -> IGPMBackupDir_Vtbl {
        unsafe extern "system" fn BackupDirectory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupDir_Impl::BackupDirectory(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrid: core::mem::MaybeUninit<windows_core::BSTR>, ppbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupDir_Impl::GetBackup(this, core::mem::transmute(&bstrid)) {
                Ok(ok__) => {
                    core::ptr::write(ppbackup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBackups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDir_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmbackupcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupDir_Impl::SearchBackups(this, windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmbackupcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupDirectory: BackupDirectory::<Identity, Impl, OFFSET>,
            GetBackup: GetBackup::<Identity, Impl, OFFSET>,
            SearchBackups: SearchBackups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMBackupDir as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMBackupDirEx_Impl: Sized + super::Com::IDispatch_Impl {
    fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BackupType(&self) -> windows_core::Result<GPMBackupType>;
    fn GetBackup(&self, bstrid: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn SearchBackups(&self, pigpmsearchcriteria: Option<&IGPMSearchCriteria>) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMBackupDirEx {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupDirEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>() -> IGPMBackupDirEx_Vtbl {
        unsafe extern "system" fn BackupDir<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupdir: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupDirEx_Impl::BackupDir(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrbackupdir, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupDirEx_Impl::BackupType(this) {
                Ok(ok__) => {
                    core::ptr::write(pgpmbackuptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrid: core::mem::MaybeUninit<windows_core::BSTR>, pvarbackup: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupDirEx_Impl::GetBackup(this, core::mem::transmute(&bstrid)) {
                Ok(ok__) => {
                    core::ptr::write(pvarbackup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBackups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMBackupDirEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, pvarbackupcollection: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMBackupDirEx_Impl::SearchBackups(this, windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                Ok(ok__) => {
                    core::ptr::write(pvarbackupcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            BackupType: BackupType::<Identity, Impl, OFFSET>,
            GetBackup: GetBackup::<Identity, Impl, OFFSET>,
            SearchBackups: SearchBackups::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMBackupDirEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMCSECollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMCSECollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMCSECollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: isize>() -> IGPMCSECollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMCSECollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMCSECollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMCSECollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmcses: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMCSECollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmcses, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMCSECollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMClientSideExtension_Impl: Sized + super::Com::IDispatch_Impl {
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsUserEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsComputerEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMClientSideExtension {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMClientSideExtension_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>() -> IGPMClientSideExtension_Vtbl {
        unsafe extern "system" fn ID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMClientSideExtension_Impl::ID(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMClientSideExtension_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMClientSideExtension_Impl::IsUserEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pvbenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMClientSideExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMClientSideExtension_Impl::IsComputerEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pvbenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ID: ID::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            IsUserEnabled: IsUserEnabled::<Identity, Impl, OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMClientSideExtension as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMConstants_Impl: Sized + super::Com::IDispatch_Impl {
    fn PermGPOApply(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermGPORead(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermGPOEdit(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermGPOEditSecurityAndDelete(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermGPOCustom(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterEdit(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterFullControl(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermWMIFilterCustom(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMLink(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMLogging(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMPlanning(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMGPOCreate(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMWMICreate(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermSOMWMIFullControl(&self) -> windows_core::Result<GPMPermissionType>;
    fn SearchPropertyGPOPermissions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOEffectivePermissions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODisplayName(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOWMIFilter(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOID(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOComputerExtensions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPOUserExtensions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertySOMLinks(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyGPODomain(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyBackupMostRecent(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchOpEquals(&self) -> windows_core::Result<GPMSearchOperation>;
    fn SearchOpContains(&self) -> windows_core::Result<GPMSearchOperation>;
    fn SearchOpNotContains(&self) -> windows_core::Result<GPMSearchOperation>;
    fn SearchOpNotEquals(&self) -> windows_core::Result<GPMSearchOperation>;
    fn UsePDC(&self) -> windows_core::Result<i32>;
    fn UseAnyDC(&self) -> windows_core::Result<i32>;
    fn DoNotUseW2KDC(&self) -> windows_core::Result<i32>;
    fn SOMSite(&self) -> windows_core::Result<GPMSOMType>;
    fn SOMDomain(&self) -> windows_core::Result<GPMSOMType>;
    fn SOMOU(&self) -> windows_core::Result<GPMSOMType>;
    fn get_SecurityFlags(&self, vbowner: super::super::Foundation::VARIANT_BOOL, vbgroup: super::super::Foundation::VARIANT_BOOL, vbdacl: super::super::Foundation::VARIANT_BOOL, vbsacl: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<i32>;
    fn DoNotValidateDC(&self) -> windows_core::Result<i32>;
    fn ReportHTML(&self) -> windows_core::Result<GPMReportType>;
    fn ReportXML(&self) -> windows_core::Result<GPMReportType>;
    fn RSOPModeUnknown(&self) -> windows_core::Result<GPMRSOPMode>;
    fn RSOPModePlanning(&self) -> windows_core::Result<GPMRSOPMode>;
    fn RSOPModeLogging(&self) -> windows_core::Result<GPMRSOPMode>;
    fn EntryTypeUser(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeComputer(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeLocalGroup(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeGlobalGroup(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeUniversalGroup(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeUNCPath(&self) -> windows_core::Result<GPMEntryType>;
    fn EntryTypeUnknown(&self) -> windows_core::Result<GPMEntryType>;
    fn DestinationOptionSameAsSource(&self) -> windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionNone(&self) -> windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionByRelativeName(&self) -> windows_core::Result<GPMDestinationOption>;
    fn DestinationOptionSet(&self) -> windows_core::Result<GPMDestinationOption>;
    fn MigrationTableOnly(&self) -> windows_core::Result<i32>;
    fn ProcessSecurity(&self) -> windows_core::Result<i32>;
    fn RsopLoggingNoComputer(&self) -> windows_core::Result<i32>;
    fn RsopLoggingNoUser(&self) -> windows_core::Result<i32>;
    fn RsopPlanningAssumeSlowLink(&self) -> windows_core::Result<i32>;
    fn get_RsopPlanningLoopbackOption(&self, vbmerge: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<i32>;
    fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> windows_core::Result<i32>;
    fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMConstants {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMConstants_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>() -> IGPMConstants_Vtbl {
        unsafe extern "system" fn PermGPOApply<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermGPOApply(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPORead<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermGPORead(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOEdit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermGPOEdit(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOEditSecurityAndDelete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermGPOEditSecurityAndDelete(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermGPOCustom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermGPOCustom(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterEdit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermWMIFilterEdit(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterFullControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermWMIFilterFullControl(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermWMIFilterCustom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermWMIFilterCustom(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMLink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermSOMLink(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMLogging<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermSOMLogging(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMPlanning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermSOMPlanning(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMGPOCreate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermSOMGPOCreate(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMWMICreate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermSOMWMICreate(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermSOMWMIFullControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::PermSOMWMIFullControl(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOPermissions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyGPOPermissions(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOEffectivePermissions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyGPOEffectivePermissions(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPODisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyGPODisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOWMIFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyGPOWMIFilter(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyGPOID(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOComputerExtensions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyGPOComputerExtensions(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPOUserExtensions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyGPOUserExtensions(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertySOMLinks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertySOMLinks(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyGPODomain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyGPODomain(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyBackupMostRecent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchPropertyBackupMostRecent(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpEquals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchOperation) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchOpEquals(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpContains<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchOperation) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchOpContains(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpNotContains<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchOperation) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchOpNotContains(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchOpNotEquals<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchOperation) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SearchOpNotEquals(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsePDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::UsePDC(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseAnyDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::UseAnyDC(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotUseW2KDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::DoNotUseW2KDC(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMSite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSOMType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SOMSite(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMDomain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSOMType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SOMDomain(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOMOU<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSOMType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::SOMOU(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_SecurityFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbowner: super::super::Foundation::VARIANT_BOOL, vbgroup: super::super::Foundation::VARIANT_BOOL, vbdacl: super::super::Foundation::VARIANT_BOOL, vbsacl: super::super::Foundation::VARIANT_BOOL, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::get_SecurityFlags(this, core::mem::transmute_copy(&vbowner), core::mem::transmute_copy(&vbgroup), core::mem::transmute_copy(&vbdacl), core::mem::transmute_copy(&vbsacl)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotValidateDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::DoNotValidateDC(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportHTML<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMReportType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::ReportHTML(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportXML<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMReportType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::ReportXML(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModeUnknown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMRSOPMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::RSOPModeUnknown(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModePlanning<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMRSOPMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::RSOPModePlanning(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RSOPModeLogging<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMRSOPMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::RSOPModeLogging(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::EntryTypeUser(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::EntryTypeComputer(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeLocalGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::EntryTypeLocalGroup(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeGlobalGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::EntryTypeGlobalGroup(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUniversalGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::EntryTypeUniversalGroup(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUNCPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::EntryTypeUNCPath(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryTypeUnknown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMEntryType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::EntryTypeUnknown(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionSameAsSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMDestinationOption) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::DestinationOptionSameAsSource(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionNone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMDestinationOption) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::DestinationOptionNone(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionByRelativeName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMDestinationOption) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::DestinationOptionByRelativeName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOptionSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMDestinationOption) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::DestinationOptionSet(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MigrationTableOnly<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::MigrationTableOnly(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessSecurity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::ProcessSecurity(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopLoggingNoComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::RsopLoggingNoComputer(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopLoggingNoUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::RsopLoggingNoUser(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeSlowLink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::RsopPlanningAssumeSlowLink(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RsopPlanningLoopbackOption<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbmerge: super::super::Foundation::VARIANT_BOOL, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::get_RsopPlanningLoopbackOption(this, core::mem::transmute_copy(&vbmerge)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeUserWQLFilterTrue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::RsopPlanningAssumeUserWQLFilterTrue(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RsopPlanningAssumeCompWQLFilterTrue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants_Impl::RsopPlanningAssumeCompWQLFilterTrue(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PermGPOApply: PermGPOApply::<Identity, Impl, OFFSET>,
            PermGPORead: PermGPORead::<Identity, Impl, OFFSET>,
            PermGPOEdit: PermGPOEdit::<Identity, Impl, OFFSET>,
            PermGPOEditSecurityAndDelete: PermGPOEditSecurityAndDelete::<Identity, Impl, OFFSET>,
            PermGPOCustom: PermGPOCustom::<Identity, Impl, OFFSET>,
            PermWMIFilterEdit: PermWMIFilterEdit::<Identity, Impl, OFFSET>,
            PermWMIFilterFullControl: PermWMIFilterFullControl::<Identity, Impl, OFFSET>,
            PermWMIFilterCustom: PermWMIFilterCustom::<Identity, Impl, OFFSET>,
            PermSOMLink: PermSOMLink::<Identity, Impl, OFFSET>,
            PermSOMLogging: PermSOMLogging::<Identity, Impl, OFFSET>,
            PermSOMPlanning: PermSOMPlanning::<Identity, Impl, OFFSET>,
            PermSOMGPOCreate: PermSOMGPOCreate::<Identity, Impl, OFFSET>,
            PermSOMWMICreate: PermSOMWMICreate::<Identity, Impl, OFFSET>,
            PermSOMWMIFullControl: PermSOMWMIFullControl::<Identity, Impl, OFFSET>,
            SearchPropertyGPOPermissions: SearchPropertyGPOPermissions::<Identity, Impl, OFFSET>,
            SearchPropertyGPOEffectivePermissions: SearchPropertyGPOEffectivePermissions::<Identity, Impl, OFFSET>,
            SearchPropertyGPODisplayName: SearchPropertyGPODisplayName::<Identity, Impl, OFFSET>,
            SearchPropertyGPOWMIFilter: SearchPropertyGPOWMIFilter::<Identity, Impl, OFFSET>,
            SearchPropertyGPOID: SearchPropertyGPOID::<Identity, Impl, OFFSET>,
            SearchPropertyGPOComputerExtensions: SearchPropertyGPOComputerExtensions::<Identity, Impl, OFFSET>,
            SearchPropertyGPOUserExtensions: SearchPropertyGPOUserExtensions::<Identity, Impl, OFFSET>,
            SearchPropertySOMLinks: SearchPropertySOMLinks::<Identity, Impl, OFFSET>,
            SearchPropertyGPODomain: SearchPropertyGPODomain::<Identity, Impl, OFFSET>,
            SearchPropertyBackupMostRecent: SearchPropertyBackupMostRecent::<Identity, Impl, OFFSET>,
            SearchOpEquals: SearchOpEquals::<Identity, Impl, OFFSET>,
            SearchOpContains: SearchOpContains::<Identity, Impl, OFFSET>,
            SearchOpNotContains: SearchOpNotContains::<Identity, Impl, OFFSET>,
            SearchOpNotEquals: SearchOpNotEquals::<Identity, Impl, OFFSET>,
            UsePDC: UsePDC::<Identity, Impl, OFFSET>,
            UseAnyDC: UseAnyDC::<Identity, Impl, OFFSET>,
            DoNotUseW2KDC: DoNotUseW2KDC::<Identity, Impl, OFFSET>,
            SOMSite: SOMSite::<Identity, Impl, OFFSET>,
            SOMDomain: SOMDomain::<Identity, Impl, OFFSET>,
            SOMOU: SOMOU::<Identity, Impl, OFFSET>,
            get_SecurityFlags: get_SecurityFlags::<Identity, Impl, OFFSET>,
            DoNotValidateDC: DoNotValidateDC::<Identity, Impl, OFFSET>,
            ReportHTML: ReportHTML::<Identity, Impl, OFFSET>,
            ReportXML: ReportXML::<Identity, Impl, OFFSET>,
            RSOPModeUnknown: RSOPModeUnknown::<Identity, Impl, OFFSET>,
            RSOPModePlanning: RSOPModePlanning::<Identity, Impl, OFFSET>,
            RSOPModeLogging: RSOPModeLogging::<Identity, Impl, OFFSET>,
            EntryTypeUser: EntryTypeUser::<Identity, Impl, OFFSET>,
            EntryTypeComputer: EntryTypeComputer::<Identity, Impl, OFFSET>,
            EntryTypeLocalGroup: EntryTypeLocalGroup::<Identity, Impl, OFFSET>,
            EntryTypeGlobalGroup: EntryTypeGlobalGroup::<Identity, Impl, OFFSET>,
            EntryTypeUniversalGroup: EntryTypeUniversalGroup::<Identity, Impl, OFFSET>,
            EntryTypeUNCPath: EntryTypeUNCPath::<Identity, Impl, OFFSET>,
            EntryTypeUnknown: EntryTypeUnknown::<Identity, Impl, OFFSET>,
            DestinationOptionSameAsSource: DestinationOptionSameAsSource::<Identity, Impl, OFFSET>,
            DestinationOptionNone: DestinationOptionNone::<Identity, Impl, OFFSET>,
            DestinationOptionByRelativeName: DestinationOptionByRelativeName::<Identity, Impl, OFFSET>,
            DestinationOptionSet: DestinationOptionSet::<Identity, Impl, OFFSET>,
            MigrationTableOnly: MigrationTableOnly::<Identity, Impl, OFFSET>,
            ProcessSecurity: ProcessSecurity::<Identity, Impl, OFFSET>,
            RsopLoggingNoComputer: RsopLoggingNoComputer::<Identity, Impl, OFFSET>,
            RsopLoggingNoUser: RsopLoggingNoUser::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeSlowLink: RsopPlanningAssumeSlowLink::<Identity, Impl, OFFSET>,
            get_RsopPlanningLoopbackOption: get_RsopPlanningLoopbackOption::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeUserWQLFilterTrue: RsopPlanningAssumeUserWQLFilterTrue::<Identity, Impl, OFFSET>,
            RsopPlanningAssumeCompWQLFilterTrue: RsopPlanningAssumeCompWQLFilterTrue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMConstants as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMConstants2_Impl: Sized + IGPMConstants_Impl {
    fn BackupTypeGPO(&self) -> windows_core::Result<GPMBackupType>;
    fn BackupTypeStarterGPO(&self) -> windows_core::Result<GPMBackupType>;
    fn StarterGPOTypeSystem(&self) -> windows_core::Result<GPMStarterGPOType>;
    fn StarterGPOTypeCustom(&self) -> windows_core::Result<GPMStarterGPOType>;
    fn SearchPropertyStarterGPOPermissions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOEffectivePermissions(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODisplayName(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPOID(&self) -> windows_core::Result<GPMSearchProperty>;
    fn SearchPropertyStarterGPODomain(&self) -> windows_core::Result<GPMSearchProperty>;
    fn PermStarterGPORead(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOEdit(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOFullControl(&self) -> windows_core::Result<GPMPermissionType>;
    fn PermStarterGPOCustom(&self) -> windows_core::Result<GPMPermissionType>;
    fn ReportLegacy(&self) -> windows_core::Result<GPMReportingOptions>;
    fn ReportComments(&self) -> windows_core::Result<GPMReportingOptions>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMConstants2 {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMConstants2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>() -> IGPMConstants2_Vtbl {
        unsafe extern "system" fn BackupTypeGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMBackupType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::BackupTypeGPO(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupTypeStarterGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMBackupType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::BackupTypeStarterGPO(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOTypeSystem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMStarterGPOType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::StarterGPOTypeSystem(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOTypeCustom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMStarterGPOType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::StarterGPOTypeCustom(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOPermissions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::SearchPropertyStarterGPOPermissions(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOEffectivePermissions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::SearchPropertyStarterGPOEffectivePermissions(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::SearchPropertyStarterGPODisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPOID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::SearchPropertyStarterGPOID(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchPropertyStarterGPODomain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSearchProperty) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::SearchPropertyStarterGPODomain(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPORead<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::PermStarterGPORead(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOEdit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::PermStarterGPOEdit(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOFullControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::PermStarterGPOFullControl(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermStarterGPOCustom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::PermStarterGPOCustom(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportLegacy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMReportingOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::ReportLegacy(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportComments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMConstants2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMReportingOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMConstants2_Impl::ReportComments(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IGPMConstants_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupTypeGPO: BackupTypeGPO::<Identity, Impl, OFFSET>,
            BackupTypeStarterGPO: BackupTypeStarterGPO::<Identity, Impl, OFFSET>,
            StarterGPOTypeSystem: StarterGPOTypeSystem::<Identity, Impl, OFFSET>,
            StarterGPOTypeCustom: StarterGPOTypeCustom::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOPermissions: SearchPropertyStarterGPOPermissions::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOEffectivePermissions: SearchPropertyStarterGPOEffectivePermissions::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPODisplayName: SearchPropertyStarterGPODisplayName::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPOID: SearchPropertyStarterGPOID::<Identity, Impl, OFFSET>,
            SearchPropertyStarterGPODomain: SearchPropertyStarterGPODomain::<Identity, Impl, OFFSET>,
            PermStarterGPORead: PermStarterGPORead::<Identity, Impl, OFFSET>,
            PermStarterGPOEdit: PermStarterGPOEdit::<Identity, Impl, OFFSET>,
            PermStarterGPOFullControl: PermStarterGPOFullControl::<Identity, Impl, OFFSET>,
            PermStarterGPOCustom: PermStarterGPOCustom::<Identity, Impl, OFFSET>,
            ReportLegacy: ReportLegacy::<Identity, Impl, OFFSET>,
            ReportComments: ReportComments::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMConstants2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMConstants as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMDomain_Impl: Sized + super::Com::IDispatch_Impl {
    fn DomainController(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreateGPO(&self) -> windows_core::Result<IGPMGPO>;
    fn GetGPO(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<IGPMGPO>;
    fn SearchGPOs(&self, pigpmsearchcriteria: Option<&IGPMSearchCriteria>) -> windows_core::Result<IGPMGPOCollection>;
    fn RestoreGPO(&self, pigpmbackup: Option<&IGPMBackup>, ldcflags: i32, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GetSOM(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<IGPMSOM>;
    fn SearchSOMs(&self, pigpmsearchcriteria: Option<&IGPMSearchCriteria>) -> windows_core::Result<IGPMSOMCollection>;
    fn GetWMIFilter(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<IGPMWMIFilter>;
    fn SearchWMIFilters(&self, pigpmsearchcriteria: Option<&IGPMSearchCriteria>) -> windows_core::Result<IGPMWMIFilterCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMDomain {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>() -> IGPMDomain_Vtbl {
        unsafe extern "system" fn DomainController<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::DomainController(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::Domain(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewgpo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::CreateGPO(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnewgpo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: core::mem::MaybeUninit<windows_core::BSTR>, ppgpo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::GetGPO(this, core::mem::transmute(&bstrguid)) {
                Ok(ok__) => {
                    core::ptr::write(ppgpo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchGPOs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmgpocollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::SearchGPOs(this, windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmgpocollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmbackup: *mut core::ffi::c_void, ldcflags: i32, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::RestoreGPO(this, windows_core::from_raw_borrowed(&pigpmbackup), core::mem::transmute_copy(&ldcflags), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSOM<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: core::mem::MaybeUninit<windows_core::BSTR>, ppsom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::GetSOM(this, core::mem::transmute(&bstrpath)) {
                Ok(ok__) => {
                    core::ptr::write(ppsom, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchSOMs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmsomcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::SearchSOMs(this, windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmsomcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: core::mem::MaybeUninit<windows_core::BSTR>, ppwmifilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::GetWMIFilter(this, core::mem::transmute(&bstrpath)) {
                Ok(ok__) => {
                    core::ptr::write(ppwmifilter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchWMIFilters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmwmifiltercollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain_Impl::SearchWMIFilters(this, windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmwmifiltercollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DomainController: DomainController::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            CreateGPO: CreateGPO::<Identity, Impl, OFFSET>,
            GetGPO: GetGPO::<Identity, Impl, OFFSET>,
            SearchGPOs: SearchGPOs::<Identity, Impl, OFFSET>,
            RestoreGPO: RestoreGPO::<Identity, Impl, OFFSET>,
            GetSOM: GetSOM::<Identity, Impl, OFFSET>,
            SearchSOMs: SearchSOMs::<Identity, Impl, OFFSET>,
            GetWMIFilter: GetWMIFilter::<Identity, Impl, OFFSET>,
            SearchWMIFilters: SearchWMIFilters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMDomain as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMDomain2_Impl: Sized + IGPMDomain_Impl {
    fn CreateStarterGPO(&self) -> windows_core::Result<IGPMStarterGPO>;
    fn CreateGPOFromStarterGPO(&self, pgpotemplate: Option<&IGPMStarterGPO>) -> windows_core::Result<IGPMGPO>;
    fn GetStarterGPO(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<IGPMStarterGPO>;
    fn SearchStarterGPOs(&self, pigpmsearchcriteria: Option<&IGPMSearchCriteria>) -> windows_core::Result<IGPMStarterGPOCollection>;
    fn LoadStarterGPO(&self, bstrloadfile: &windows_core::BSTR, boverwrite: super::super::Foundation::VARIANT_BOOL, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn RestoreStarterGPO(&self, pigpmtmplbackup: Option<&IGPMStarterGPOBackup>, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMDomain2 {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>() -> IGPMDomain2_Vtbl {
        unsafe extern "system" fn CreateStarterGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewtemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain2_Impl::CreateStarterGPO(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnewtemplate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPOFromStarterGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgpotemplate: *mut core::ffi::c_void, ppnewgpo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain2_Impl::CreateGPOFromStarterGPO(this, windows_core::from_raw_borrowed(&pgpotemplate)) {
                Ok(ok__) => {
                    core::ptr::write(ppnewgpo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStarterGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: core::mem::MaybeUninit<windows_core::BSTR>, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain2_Impl::GetStarterGPO(this, core::mem::transmute(&bstrguid)) {
                Ok(ok__) => {
                    core::ptr::write(pptemplate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchStarterGPOs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmtemplatecollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain2_Impl::SearchStarterGPOs(this, windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmtemplatecollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadStarterGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrloadfile: core::mem::MaybeUninit<windows_core::BSTR>, boverwrite: super::super::Foundation::VARIANT_BOOL, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain2_Impl::LoadStarterGPO(this, core::mem::transmute(&bstrloadfile), core::mem::transmute_copy(&boverwrite), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreStarterGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmtmplbackup: *mut core::ffi::c_void, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain2_Impl::RestoreStarterGPO(this, windows_core::from_raw_borrowed(&pigpmtmplbackup), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IGPMDomain_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateStarterGPO: CreateStarterGPO::<Identity, Impl, OFFSET>,
            CreateGPOFromStarterGPO: CreateGPOFromStarterGPO::<Identity, Impl, OFFSET>,
            GetStarterGPO: GetStarterGPO::<Identity, Impl, OFFSET>,
            SearchStarterGPOs: SearchStarterGPOs::<Identity, Impl, OFFSET>,
            LoadStarterGPO: LoadStarterGPO::<Identity, Impl, OFFSET>,
            RestoreStarterGPO: RestoreStarterGPO::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMDomain2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMDomain as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMDomain3_Impl: Sized + IGPMDomain2_Impl {
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn InfrastructureDC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInfrastructureDC(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetInfrastructureFlags(&self, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMDomain3 {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>() -> IGPMDomain3_Vtbl {
        unsafe extern "system" fn GenerateReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain3_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfrastructureDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMDomain3_Impl::InfrastructureDC(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMDomain3_Impl::SetInfrastructureDC(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMDomain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMDomain3_Impl::SetInfrastructureFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: IGPMDomain2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            InfrastructureDC: InfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMDomain3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMDomain as windows_core::Interface>::IID || iid == &<IGPMDomain2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPO_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DomainName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreationTime(&self) -> windows_core::Result<f64>;
    fn ModificationTime(&self) -> windows_core::Result<f64>;
    fn UserDSVersionNumber(&self) -> windows_core::Result<i32>;
    fn ComputerDSVersionNumber(&self) -> windows_core::Result<i32>;
    fn UserSysvolVersionNumber(&self) -> windows_core::Result<i32>;
    fn ComputerSysvolVersionNumber(&self) -> windows_core::Result<i32>;
    fn GetWMIFilter(&self) -> windows_core::Result<IGPMWMIFilter>;
    fn SetWMIFilter(&self, pigpmwmifilter: Option<&IGPMWMIFilter>) -> windows_core::Result<()>;
    fn SetUserEnabled(&self, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetComputerEnabled(&self, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsUserEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsComputerEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: Option<&IGPMSecurityInfo>) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Backup(&self, bstrbackupdir: &windows_core::BSTR, bstrcomment: &windows_core::BSTR, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn Import(&self, lflags: i32, pigpmbackup: Option<&IGPMBackup>, pvarmigrationtable: *const windows_core::VARIANT, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
    fn CopyTo(&self, lflags: i32, pigpmdomain: Option<&IGPMDomain>, pvarnewdisplayname: *const windows_core::VARIANT, pvarmigrationtable: *const windows_core::VARIANT, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn SetSecurityDescriptor(&self, lflags: i32, psd: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn GetSecurityDescriptor(&self, lflags: i32) -> windows_core::Result<super::Com::IDispatch>;
    fn IsACLConsistent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MakeACLConsistent(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMGPO {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>() -> IGPMGPO_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO_Impl::SetDisplayName(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::Path(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::ID(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::DomainName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::CreationTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModificationTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::ModificationTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pdate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDSVersionNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::UserDSVersionNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerDSVersionNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::ComputerDSVersionNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSysvolVersionNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::UserSysvolVersionNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerSysvolVersionNumber<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::ComputerSysvolVersionNumber(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWMIFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmwmifilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::GetWMIFilter(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmwmifilter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWMIFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmwmifilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO_Impl::SetWMIFilter(this, windows_core::from_raw_borrowed(&pigpmwmifilter)).into()
        }
        unsafe extern "system" fn SetUserEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO_Impl::SetUserEnabled(this, core::mem::transmute_copy(&vbenabled)).into()
        }
        unsafe extern "system" fn SetComputerEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO_Impl::SetComputerEnabled(this, core::mem::transmute_copy(&vbenabled)).into()
        }
        unsafe extern "system" fn IsUserEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::IsUserEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pvbenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComputerEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::IsComputerEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pvbenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::GetSecurityInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsecurityinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecurityinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO_Impl::SetSecurityInfo(this, windows_core::from_raw_borrowed(&psecurityinfo)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Backup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupdir: core::mem::MaybeUninit<windows_core::BSTR>, bstrcomment: core::mem::MaybeUninit<windows_core::BSTR>, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::Backup(this, core::mem::transmute(&bstrbackupdir), core::mem::transmute(&bstrcomment), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Import<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pigpmbackup: *mut core::ffi::c_void, pvarmigrationtable: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::Import(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pigpmbackup), core::mem::transmute_copy(&pvarmigrationtable), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: core::mem::MaybeUninit<windows_core::BSTR>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pigpmdomain: *mut core::ffi::c_void, pvarnewdisplayname: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarmigrationtable: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::CopyTo(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&pigpmdomain), core::mem::transmute_copy(&pvarnewdisplayname), core::mem::transmute_copy(&pvarmigrationtable), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, psd: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO_Impl::SetSecurityDescriptor(this, core::mem::transmute_copy(&lflags), windows_core::from_raw_borrowed(&psd)).into()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppsd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&lflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppsd, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsACLConsistent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbconsistent: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO_Impl::IsACLConsistent(this) {
                Ok(ok__) => {
                    core::ptr::write(pvbconsistent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeACLConsistent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO_Impl::MakeACLConsistent(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            DomainName: DomainName::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            ModificationTime: ModificationTime::<Identity, Impl, OFFSET>,
            UserDSVersionNumber: UserDSVersionNumber::<Identity, Impl, OFFSET>,
            ComputerDSVersionNumber: ComputerDSVersionNumber::<Identity, Impl, OFFSET>,
            UserSysvolVersionNumber: UserSysvolVersionNumber::<Identity, Impl, OFFSET>,
            ComputerSysvolVersionNumber: ComputerSysvolVersionNumber::<Identity, Impl, OFFSET>,
            GetWMIFilter: GetWMIFilter::<Identity, Impl, OFFSET>,
            SetWMIFilter: SetWMIFilter::<Identity, Impl, OFFSET>,
            SetUserEnabled: SetUserEnabled::<Identity, Impl, OFFSET>,
            SetComputerEnabled: SetComputerEnabled::<Identity, Impl, OFFSET>,
            IsUserEnabled: IsUserEnabled::<Identity, Impl, OFFSET>,
            IsComputerEnabled: IsComputerEnabled::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            IsACLConsistent: IsACLConsistent::<Identity, Impl, OFFSET>,
            MakeACLConsistent: MakeACLConsistent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPO as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPO2_Impl: Sized + IGPMGPO_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMGPO2 {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: isize>() -> IGPMGPO2_Vtbl {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO2_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO2_Impl::SetDescription(this, core::mem::transmute(&newval)).into()
        }
        Self {
            base__: IGPMGPO_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPO2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMGPO as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPO3_Impl: Sized + IGPMGPO2_Impl {
    fn InfrastructureDC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInfrastructureDC(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetInfrastructureFlags(&self, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMGPO3 {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: isize>() -> IGPMGPO3_Vtbl {
        unsafe extern "system" fn InfrastructureDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPO3_Impl::InfrastructureDC(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInfrastructureDC<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO3_Impl::SetInfrastructureDC(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn SetInfrastructureFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPO3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPO3_Impl::SetInfrastructureFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: IGPMGPO2_Vtbl::new::<Identity, Impl, OFFSET>(),
            InfrastructureDC: InfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureDC: SetInfrastructureDC::<Identity, Impl, OFFSET>,
            SetInfrastructureFlags: SetInfrastructureFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPO3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IGPMGPO as windows_core::Interface>::IID || iid == &<IGPMGPO2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMGPOCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>() -> IGPMGPOCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmgpos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmgpos, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPOCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMGPOLink_Impl: Sized + super::Com::IDispatch_Impl {
    fn GPOID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GPODomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Enforced(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnforced(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SOMLinkOrder(&self) -> windows_core::Result<i32>;
    fn SOM(&self) -> windows_core::Result<IGPMSOM>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMGPOLink {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOLink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>() -> IGPMGPOLink_Vtbl {
        unsafe extern "system" fn GPOID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLink_Impl::GPOID(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPODomain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLink_Impl::GPODomain(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLink_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPOLink_Impl::SetEnabled(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Enforced<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLink_Impl::Enforced(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnforced<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPOLink_Impl::SetEnforced(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SOMLinkOrder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLink_Impl::SOMLinkOrder(this) {
                Ok(ok__) => {
                    core::ptr::write(lval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SOM<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmsom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLink_Impl::SOM(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmsom, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMGPOLink_Impl::Delete(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GPOID: GPOID::<Identity, Impl, OFFSET>,
            GPODomain: GPODomain::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Enforced: Enforced::<Identity, Impl, OFFSET>,
            SetEnforced: SetEnforced::<Identity, Impl, OFFSET>,
            SOMLinkOrder: SOMLinkOrder::<Identity, Impl, OFFSET>,
            SOM: SOM::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPOLink as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMGPOLinksCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMGPOLinksCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMGPOLinksCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>() -> IGPMGPOLinksCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLinksCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLinksCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMGPOLinksCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmlinks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMGPOLinksCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmlinks, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMGPOLinksCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMMapEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn Source(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Destination(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DestinationOption(&self) -> windows_core::Result<GPMDestinationOption>;
    fn EntryType(&self) -> windows_core::Result<GPMEntryType>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMMapEntry {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMMapEntry_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>() -> IGPMMapEntry_Vtbl {
        unsafe extern "system" fn Source<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMapEntry_Impl::Source(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsource, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdestination: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMapEntry_Impl::Destination(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdestination, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationOption<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMapEntry_Impl::DestinationOption(this) {
                Ok(ok__) => {
                    core::ptr::write(pgpmdestoption, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMapEntry_Impl::EntryType(this) {
                Ok(ok__) => {
                    core::ptr::write(pgpmentrytype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Source: Source::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            DestinationOption: DestinationOption::<Identity, Impl, OFFSET>,
            EntryType: EntryType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMMapEntry as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMMapEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMMapEntryCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMMapEntryCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>() -> IGPMMapEntryCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMapEntryCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMapEntryCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMapEntryCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMapEntryCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMMapEntryCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMMigrationTable_Impl: Sized + super::Com::IDispatch_Impl {
    fn Save(&self, bstrmigrationtablepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Add(&self, lflags: i32, var: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AddEntry(&self, bstrsource: &windows_core::BSTR, gpmentrytype: GPMEntryType, pvardestination: *const windows_core::VARIANT) -> windows_core::Result<IGPMMapEntry>;
    fn GetEntry(&self, bstrsource: &windows_core::BSTR) -> windows_core::Result<IGPMMapEntry>;
    fn DeleteEntry(&self, bstrsource: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UpdateDestination(&self, bstrsource: &windows_core::BSTR, pvardestination: *const windows_core::VARIANT) -> windows_core::Result<IGPMMapEntry>;
    fn Validate(&self) -> windows_core::Result<IGPMResult>;
    fn GetEntries(&self) -> windows_core::Result<IGPMMapEntryCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMMigrationTable {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMMigrationTable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>() -> IGPMMigrationTable_Vtbl {
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmigrationtablepath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMMigrationTable_Impl::Save(this, core::mem::transmute(&bstrmigrationtablepath)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, var: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMMigrationTable_Impl::Add(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn AddEntry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsource: core::mem::MaybeUninit<windows_core::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMigrationTable_Impl::AddEntry(this, core::mem::transmute(&bstrsource), core::mem::transmute_copy(&gpmentrytype), core::mem::transmute_copy(&pvardestination)) {
                Ok(ok__) => {
                    core::ptr::write(ppentry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsource: core::mem::MaybeUninit<windows_core::BSTR>, ppentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMigrationTable_Impl::GetEntry(this, core::mem::transmute(&bstrsource)) {
                Ok(ok__) => {
                    core::ptr::write(ppentry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteEntry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsource: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMMigrationTable_Impl::DeleteEntry(this, core::mem::transmute(&bstrsource)).into()
        }
        unsafe extern "system" fn UpdateDestination<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsource: core::mem::MaybeUninit<windows_core::BSTR>, pvardestination: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMigrationTable_Impl::UpdateDestination(this, core::mem::transmute(&bstrsource), core::mem::transmute_copy(&pvardestination)) {
                Ok(ok__) => {
                    core::ptr::write(ppentry, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMigrationTable_Impl::Validate(this) {
                Ok(ok__) => {
                    core::ptr::write(ppresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMMigrationTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppentries: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMMigrationTable_Impl::GetEntries(this) {
                Ok(ok__) => {
                    core::ptr::write(ppentries, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Save: Save::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            AddEntry: AddEntry::<Identity, Impl, OFFSET>,
            GetEntry: GetEntry::<Identity, Impl, OFFSET>,
            DeleteEntry: DeleteEntry::<Identity, Impl, OFFSET>,
            UpdateDestination: UpdateDestination::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
            GetEntries: GetEntries::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMMigrationTable as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMPermission_Impl: Sized + super::Com::IDispatch_Impl {
    fn Inherited(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Inheritable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Denied(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Permission(&self) -> windows_core::Result<GPMPermissionType>;
    fn Trustee(&self) -> windows_core::Result<IGPMTrustee>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMPermission {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMPermission_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>() -> IGPMPermission_Vtbl {
        unsafe extern "system" fn Inherited<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMPermission_Impl::Inherited(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inheritable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMPermission_Impl::Inheritable(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Denied<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMPermission_Impl::Denied(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Permission<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMPermissionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMPermission_Impl::Permission(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trustee<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMPermission_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmtrustee: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMPermission_Impl::Trustee(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmtrustee, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Inherited: Inherited::<Identity, Impl, OFFSET>,
            Inheritable: Inheritable::<Identity, Impl, OFFSET>,
            Denied: Denied::<Identity, Impl, OFFSET>,
            Permission: Permission::<Identity, Impl, OFFSET>,
            Trustee: Trustee::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMPermission as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMRSOP_Impl: Sized + super::Com::IDispatch_Impl {
    fn Mode(&self) -> windows_core::Result<GPMRSOPMode>;
    fn Namespace(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoggingComputer(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoggingComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoggingUser(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LoggingUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLoggingFlags(&self, lval: i32) -> windows_core::Result<()>;
    fn LoggingFlags(&self) -> windows_core::Result<i32>;
    fn SetPlanningFlags(&self, lval: i32) -> windows_core::Result<()>;
    fn PlanningFlags(&self) -> windows_core::Result<i32>;
    fn SetPlanningDomainController(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningDomainController(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningSiteName(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningSiteName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningUser(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningUserSOM(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningUserSOM(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningUserWMIFilters(&self, varval: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PlanningUserWMIFilters(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPlanningUserSecurityGroups(&self, varval: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PlanningUserSecurityGroups(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPlanningComputer(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningComputerSOM(&self, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlanningComputerSOM(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPlanningComputerWMIFilters(&self, varval: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PlanningComputerWMIFilters(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetPlanningComputerSecurityGroups(&self, varval: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn PlanningComputerSecurityGroups(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn LoggingEnumerateUsers(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn CreateQueryResults(&self) -> windows_core::Result<()>;
    fn ReleaseQueryResults(&self) -> windows_core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMRSOP {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMRSOP_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>() -> IGPMRSOP_Vtbl {
        unsafe extern "system" fn Mode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMRSOPMode) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::Mode(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Namespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::Namespace(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetLoggingComputer(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn LoggingComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::LoggingComputer(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetLoggingUser(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn LoggingUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::LoggingUser(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLoggingFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetLoggingFlags(this, core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn LoggingFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::LoggingFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(lval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningFlags(this, core::mem::transmute_copy(&lval)).into()
        }
        unsafe extern "system" fn PlanningFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(lval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningDomainController<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningDomainController(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningDomainController<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningDomainController(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningSiteName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningSiteName(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningSiteName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningSiteName(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningUser(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningUser(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserSOM<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningUserSOM(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningUserSOM<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningUserSOM(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserWMIFilters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningUserWMIFilters(this, core::mem::transmute(&varval)).into()
        }
        unsafe extern "system" fn PlanningUserWMIFilters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningUserWMIFilters(this) {
                Ok(ok__) => {
                    core::ptr::write(varval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningUserSecurityGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningUserSecurityGroups(this, core::mem::transmute(&varval)).into()
        }
        unsafe extern "system" fn PlanningUserSecurityGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningUserSecurityGroups(this) {
                Ok(ok__) => {
                    core::ptr::write(varval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningComputer(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningComputer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningComputer(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerSOM<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningComputerSOM(this, core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn PlanningComputerSOM<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningComputerSOM(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerWMIFilters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningComputerWMIFilters(this, core::mem::transmute(&varval)).into()
        }
        unsafe extern "system" fn PlanningComputerWMIFilters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningComputerWMIFilters(this) {
                Ok(ok__) => {
                    core::ptr::write(varval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlanningComputerSecurityGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::SetPlanningComputerSecurityGroups(this, core::mem::transmute(&varval)).into()
        }
        unsafe extern "system" fn PlanningComputerSecurityGroups<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::PlanningComputerSecurityGroups(this) {
                Ok(ok__) => {
                    core::ptr::write(varval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoggingEnumerateUsers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::LoggingEnumerateUsers(this) {
                Ok(ok__) => {
                    core::ptr::write(varval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateQueryResults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::CreateQueryResults(this).into()
        }
        unsafe extern "system" fn ReleaseQueryResults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMRSOP_Impl::ReleaseQueryResults(this).into()
        }
        unsafe extern "system" fn GenerateReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMRSOP_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: core::mem::MaybeUninit<windows_core::BSTR>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMRSOP_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Mode: Mode::<Identity, Impl, OFFSET>,
            Namespace: Namespace::<Identity, Impl, OFFSET>,
            SetLoggingComputer: SetLoggingComputer::<Identity, Impl, OFFSET>,
            LoggingComputer: LoggingComputer::<Identity, Impl, OFFSET>,
            SetLoggingUser: SetLoggingUser::<Identity, Impl, OFFSET>,
            LoggingUser: LoggingUser::<Identity, Impl, OFFSET>,
            SetLoggingFlags: SetLoggingFlags::<Identity, Impl, OFFSET>,
            LoggingFlags: LoggingFlags::<Identity, Impl, OFFSET>,
            SetPlanningFlags: SetPlanningFlags::<Identity, Impl, OFFSET>,
            PlanningFlags: PlanningFlags::<Identity, Impl, OFFSET>,
            SetPlanningDomainController: SetPlanningDomainController::<Identity, Impl, OFFSET>,
            PlanningDomainController: PlanningDomainController::<Identity, Impl, OFFSET>,
            SetPlanningSiteName: SetPlanningSiteName::<Identity, Impl, OFFSET>,
            PlanningSiteName: PlanningSiteName::<Identity, Impl, OFFSET>,
            SetPlanningUser: SetPlanningUser::<Identity, Impl, OFFSET>,
            PlanningUser: PlanningUser::<Identity, Impl, OFFSET>,
            SetPlanningUserSOM: SetPlanningUserSOM::<Identity, Impl, OFFSET>,
            PlanningUserSOM: PlanningUserSOM::<Identity, Impl, OFFSET>,
            SetPlanningUserWMIFilters: SetPlanningUserWMIFilters::<Identity, Impl, OFFSET>,
            PlanningUserWMIFilters: PlanningUserWMIFilters::<Identity, Impl, OFFSET>,
            SetPlanningUserSecurityGroups: SetPlanningUserSecurityGroups::<Identity, Impl, OFFSET>,
            PlanningUserSecurityGroups: PlanningUserSecurityGroups::<Identity, Impl, OFFSET>,
            SetPlanningComputer: SetPlanningComputer::<Identity, Impl, OFFSET>,
            PlanningComputer: PlanningComputer::<Identity, Impl, OFFSET>,
            SetPlanningComputerSOM: SetPlanningComputerSOM::<Identity, Impl, OFFSET>,
            PlanningComputerSOM: PlanningComputerSOM::<Identity, Impl, OFFSET>,
            SetPlanningComputerWMIFilters: SetPlanningComputerWMIFilters::<Identity, Impl, OFFSET>,
            PlanningComputerWMIFilters: PlanningComputerWMIFilters::<Identity, Impl, OFFSET>,
            SetPlanningComputerSecurityGroups: SetPlanningComputerSecurityGroups::<Identity, Impl, OFFSET>,
            PlanningComputerSecurityGroups: PlanningComputerSecurityGroups::<Identity, Impl, OFFSET>,
            LoggingEnumerateUsers: LoggingEnumerateUsers::<Identity, Impl, OFFSET>,
            CreateQueryResults: CreateQueryResults::<Identity, Impl, OFFSET>,
            ReleaseQueryResults: ReleaseQueryResults::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMRSOP as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn Status(&self) -> windows_core::Result<IGPMStatusMsgCollection>;
    fn Result(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn OverallStatus(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMResult {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMResult_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: isize>() -> IGPMResult_Vtbl {
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmstatusmsgcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMResult_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmstatusmsgcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarresult: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMResult_Impl::Result(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMResult_Impl::OverallStatus(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Result: Result::<Identity, Impl, OFFSET>,
            OverallStatus: OverallStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMResult as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMSOM_Impl: Sized + super::Com::IDispatch_Impl {
    fn GPOInheritanceBlocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetGPOInheritanceBlocked(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreateGPOLink(&self, llinkpos: i32, pgpo: Option<&IGPMGPO>) -> windows_core::Result<IGPMGPOLink>;
    fn Type(&self) -> windows_core::Result<GPMSOMType>;
    fn GetGPOLinks(&self) -> windows_core::Result<IGPMGPOLinksCollection>;
    fn GetInheritedGPOLinks(&self) -> windows_core::Result<IGPMGPOLinksCollection>;
    fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: Option<&IGPMSecurityInfo>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMSOM {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMSOM_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>() -> IGPMSOM_Vtbl {
        unsafe extern "system" fn GPOInheritanceBlocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOM_Impl::GPOInheritanceBlocked(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGPOInheritanceBlocked<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMSOM_Impl::SetGPOInheritanceBlocked(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOM_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOM_Impl::Path(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGPOLink<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llinkpos: i32, pgpo: *mut core::ffi::c_void, ppnewgpolink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOM_Impl::CreateGPOLink(this, core::mem::transmute_copy(&llinkpos), windows_core::from_raw_borrowed(&pgpo)) {
                Ok(ok__) => {
                    core::ptr::write(ppnewgpolink, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMSOMType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOM_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPOLinks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgpolinks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOM_Impl::GetGPOLinks(this) {
                Ok(ok__) => {
                    core::ptr::write(ppgpolinks, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInheritedGPOLinks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgpolinks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOM_Impl::GetInheritedGPOLinks(this) {
                Ok(ok__) => {
                    core::ptr::write(ppgpolinks, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOM_Impl::GetSecurityInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsecurityinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecurityinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMSOM_Impl::SetSecurityInfo(this, windows_core::from_raw_borrowed(&psecurityinfo)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GPOInheritanceBlocked: GPOInheritanceBlocked::<Identity, Impl, OFFSET>,
            SetGPOInheritanceBlocked: SetGPOInheritanceBlocked::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            CreateGPOLink: CreateGPOLink::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            GetGPOLinks: GetGPOLinks::<Identity, Impl, OFFSET>,
            GetInheritedGPOLinks: GetInheritedGPOLinks::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSOM as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSOMCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMSOMCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSOMCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>() -> IGPMSOMCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOMCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOMCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSOMCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmsom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSOMCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmsom, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSOMCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMSearchCriteria_Impl: Sized + super::Com::IDispatch_Impl {
    fn Add(&self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMSearchCriteria {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMSearchCriteria_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSearchCriteria_Impl, const OFFSET: isize>() -> IGPMSearchCriteria_Vtbl {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSearchCriteria_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMSearchCriteria_Impl::Add(this, core::mem::transmute_copy(&searchproperty), core::mem::transmute_copy(&searchoperation), core::mem::transmute(&varvalue)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Add: Add::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSearchCriteria as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMSecurityInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
    fn Add(&self, pperm: Option<&IGPMPermission>) -> windows_core::Result<()>;
    fn Remove(&self, pperm: Option<&IGPMPermission>) -> windows_core::Result<()>;
    fn RemoveTrustee(&self, bstrtrustee: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMSecurityInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMSecurityInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>() -> IGPMSecurityInfo_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSecurityInfo_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSecurityInfo_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSecurityInfo_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMSecurityInfo_Impl::Add(this, windows_core::from_raw_borrowed(&pperm)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMSecurityInfo_Impl::Remove(this, windows_core::from_raw_borrowed(&pperm)).into()
        }
        unsafe extern "system" fn RemoveTrustee<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSecurityInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtrustee: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMSecurityInfo_Impl::RemoveTrustee(this, core::mem::transmute(&bstrtrustee)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveTrustee: RemoveTrustee::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSecurityInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMSitesContainer_Impl: Sized + super::Com::IDispatch_Impl {
    fn DomainController(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Forest(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSite(&self, bstrsitename: &windows_core::BSTR) -> windows_core::Result<IGPMSOM>;
    fn SearchSites(&self, pigpmsearchcriteria: Option<&IGPMSearchCriteria>) -> windows_core::Result<IGPMSOMCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMSitesContainer {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMSitesContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>() -> IGPMSitesContainer_Vtbl {
        unsafe extern "system" fn DomainController<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSitesContainer_Impl::DomainController(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSitesContainer_Impl::Domain(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSitesContainer_Impl::Forest(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsitename: core::mem::MaybeUninit<windows_core::BSTR>, ppsom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSitesContainer_Impl::GetSite(this, core::mem::transmute(&bstrsitename)) {
                Ok(ok__) => {
                    core::ptr::write(ppsom, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchSites<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMSitesContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pigpmsearchcriteria: *mut core::ffi::c_void, ppigpmsomcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMSitesContainer_Impl::SearchSites(this, windows_core::from_raw_borrowed(&pigpmsearchcriteria)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmsomcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DomainController: DomainController::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            Forest: Forest::<Identity, Impl, OFFSET>,
            GetSite: GetSite::<Identity, Impl, OFFSET>,
            SearchSites: SearchSites::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMSitesContainer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStarterGPO_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Author(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Product(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreationTime(&self) -> windows_core::Result<f64>;
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ModifiedTime(&self) -> windows_core::Result<f64>;
    fn Type(&self) -> windows_core::Result<GPMStarterGPOType>;
    fn ComputerVersion(&self) -> windows_core::Result<u16>;
    fn UserVersion(&self) -> windows_core::Result<u16>;
    fn StarterGPOVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self, bstrsavefile: &windows_core::BSTR, boverwrite: super::super::Foundation::VARIANT_BOOL, bsaveassystem: super::super::Foundation::VARIANT_BOOL, bstrlanguage: *const windows_core::VARIANT, bstrauthor: *const windows_core::VARIANT, bstrproduct: *const windows_core::VARIANT, bstruniqueid: *const windows_core::VARIANT, bstrversion: *const windows_core::VARIANT, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn Backup(&self, bstrbackupdir: &windows_core::BSTR, bstrcomment: &windows_core::BSTR, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn CopyTo(&self, pvarnewdisplayname: *const windows_core::VARIANT, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *const windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *const windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
    fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: Option<&IGPMSecurityInfo>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMStarterGPO {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPO_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>() -> IGPMStarterGPO_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMStarterGPO_Impl::SetDisplayName(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMStarterGPO_Impl::SetDescription(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Author<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::Author(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Product<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::Product(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::CreationTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::ID(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiedTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::ModifiedTime(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut GPMStarterGPOType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputerVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::ComputerVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::UserVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::StarterGPOVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMStarterGPO_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            bstrsavefile: core::mem::MaybeUninit<windows_core::BSTR>,
            boverwrite: super::super::Foundation::VARIANT_BOOL,
            bsaveassystem: super::super::Foundation::VARIANT_BOOL,
            bstrlanguage: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            bstrauthor: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            bstrproduct: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            bstruniqueid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            bstrversion: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>,
            ppigpmresult: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::Save(this, core::mem::transmute(&bstrsavefile), core::mem::transmute_copy(&boverwrite), core::mem::transmute_copy(&bsaveassystem), core::mem::transmute_copy(&bstrlanguage), core::mem::transmute_copy(&bstrauthor), core::mem::transmute_copy(&bstrproduct), core::mem::transmute_copy(&bstruniqueid), core::mem::transmute_copy(&bstrversion), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Backup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupdir: core::mem::MaybeUninit<windows_core::BSTR>, bstrcomment: core::mem::MaybeUninit<windows_core::BSTR>, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::Backup(this, core::mem::transmute(&bstrbackupdir), core::mem::transmute(&bstrcomment), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarnewdisplayname: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::CopyTo(this, core::mem::transmute_copy(&pvarnewdisplayname), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: core::mem::MaybeUninit<windows_core::BSTR>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPO_Impl::GetSecurityInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsecurityinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecurityinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMStarterGPO_Impl::SetSecurityInfo(this, windows_core::from_raw_borrowed(&psecurityinfo)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Product: Product::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            ModifiedTime: ModifiedTime::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            ComputerVersion: ComputerVersion::<Identity, Impl, OFFSET>,
            UserVersion: UserVersion::<Identity, Impl, OFFSET>,
            StarterGPOVersion: StarterGPOVersion::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Backup: Backup::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStarterGPO as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStarterGPOBackup_Impl: Sized + super::Com::IDispatch_Impl {
    fn BackupDir(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Comment(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StarterGPOID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Timestamp(&self) -> windows_core::Result<f64>;
    fn Type(&self) -> windows_core::Result<GPMStarterGPOType>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const windows_core::VARIANT, pvargpmcancel: *mut windows_core::VARIANT) -> windows_core::Result<IGPMResult>;
    fn GenerateReportToFile(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: &windows_core::BSTR) -> windows_core::Result<IGPMResult>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMStarterGPOBackup {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOBackup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>() -> IGPMStarterGPOBackup_Vtbl {
        unsafe extern "system" fn BackupDir<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupdir: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::BackupDir(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrbackupdir, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcomment: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::Comment(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcomment, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdisplayname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtemplatedomain: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::Domain(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtemplatedomain, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StarterGPOID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtemplateid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::StarterGPOID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtemplateid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::ID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimestamp: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::Timestamp(this) {
                Ok(ok__) => {
                    core::ptr::write(ptimestamp, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(ptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMStarterGPOBackup_Impl::Delete(this).into()
        }
        unsafe extern "system" fn GenerateReport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvargpmcancel: *mut core::mem::MaybeUninit<windows_core::VARIANT>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::GenerateReport(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute_copy(&pvargpmprogress), core::mem::transmute_copy(&pvargpmcancel)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateReportToFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: core::mem::MaybeUninit<windows_core::BSTR>, ppigpmresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackup_Impl::GenerateReportToFile(this, core::mem::transmute_copy(&gpmreporttype), core::mem::transmute(&bstrtargetfilepath)) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmresult, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BackupDir: BackupDir::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            Domain: Domain::<Identity, Impl, OFFSET>,
            StarterGPOID: StarterGPOID::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GenerateReport: GenerateReport::<Identity, Impl, OFFSET>,
            GenerateReportToFile: GenerateReportToFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackup as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOBackupCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMStarterGPOBackupCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOBackupCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>() -> IGPMStarterGPOBackupCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackupCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackupCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOBackupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmtmplbackup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOBackupCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmtmplbackup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStarterGPOBackupCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStarterGPOCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMStarterGPOCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStarterGPOCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>() -> IGPMStarterGPOCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStarterGPOCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppigpmtemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStarterGPOCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppigpmtemplates, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStarterGPOCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMStatusMessage_Impl: Sized + super::Com::IDispatch_Impl {
    fn ObjectPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ErrorCode(&self) -> windows_core::Result<()>;
    fn ExtensionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettingsName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OperationCode(&self) -> windows_core::Result<()>;
    fn Message(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMStatusMessage {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMStatusMessage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>() -> IGPMStatusMessage_Vtbl {
        unsafe extern "system" fn ObjectPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStatusMessage_Impl::ObjectPath(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMStatusMessage_Impl::ErrorCode(this).into()
        }
        unsafe extern "system" fn ExtensionName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStatusMessage_Impl::ExtensionName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettingsName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStatusMessage_Impl::SettingsName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OperationCode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMStatusMessage_Impl::OperationCode(this).into()
        }
        unsafe extern "system" fn Message<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStatusMessage_Impl::Message(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectPath: ObjectPath::<Identity, Impl, OFFSET>,
            ErrorCode: ErrorCode::<Identity, Impl, OFFSET>,
            ExtensionName: ExtensionName::<Identity, Impl, OFFSET>,
            SettingsName: SettingsName::<Identity, Impl, OFFSET>,
            OperationCode: OperationCode::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStatusMessage as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMStatusMsgCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMStatusMsgCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMStatusMsgCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>() -> IGPMStatusMsgCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStatusMsgCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStatusMsgCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMStatusMsgCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMStatusMsgCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMStatusMsgCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMTrustee_Impl: Sized + super::Com::IDispatch_Impl {
    fn TrusteeSid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TrusteeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TrusteeDomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TrusteeDSPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TrusteeType(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMTrustee {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMTrustee_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>() -> IGPMTrustee_Vtbl {
        unsafe extern "system" fn TrusteeSid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMTrustee_Impl::TrusteeSid(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMTrustee_Impl::TrusteeName(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeDomain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMTrustee_Impl::TrusteeDomain(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeDSPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMTrustee_Impl::TrusteeDSPath(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrusteeType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMTrustee_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMTrustee_Impl::TrusteeType(this) {
                Ok(ok__) => {
                    core::ptr::write(lval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TrusteeSid: TrusteeSid::<Identity, Impl, OFFSET>,
            TrusteeName: TrusteeName::<Identity, Impl, OFFSET>,
            TrusteeDomain: TrusteeDomain::<Identity, Impl, OFFSET>,
            TrusteeDSPath: TrusteeDSPath::<Identity, Impl, OFFSET>,
            TrusteeType: TrusteeType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMTrustee as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGPMWMIFilter_Impl: Sized + super::Com::IDispatch_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetQueryList(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn GetSecurityInfo(&self) -> windows_core::Result<IGPMSecurityInfo>;
    fn SetSecurityInfo(&self, psecurityinfo: Option<&IGPMSecurityInfo>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGPMWMIFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IGPMWMIFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>() -> IGPMWMIFilter_Vtbl {
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMWMIFilter_Impl::Path(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMWMIFilter_Impl::SetName(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMWMIFilter_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMWMIFilter_Impl::SetDescription(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMWMIFilter_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqrylist: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMWMIFilter_Impl::GetQueryList(this) {
                Ok(ok__) => {
                    core::ptr::write(pqrylist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMWMIFilter_Impl::GetSecurityInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsecurityinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psecurityinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGPMWMIFilter_Impl::SetSecurityInfo(this, windows_core::from_raw_borrowed(&psecurityinfo)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            GetQueryList: GetQueryList::<Identity, Impl, OFFSET>,
            GetSecurityInfo: GetSecurityInfo::<Identity, Impl, OFFSET>,
            SetSecurityInfo: SetSecurityInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMWMIFilter as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGPMWMIFilterCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<super::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IGPMWMIFilterCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGPMWMIFilterCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>() -> IGPMWMIFilterCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMWMIFilterCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMWMIFilterCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGPMWMIFilterCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGPMWMIFilterCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGPMWMIFilterCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
pub trait IGroupPolicyObject_Impl: Sized {
    fn New(&self, pszdomainname: &windows_core::PCWSTR, pszdisplayname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn OpenDSGPO(&self, pszpath: &windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()>;
    fn OpenLocalMachineGPO(&self, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()>;
    fn OpenRemoteMachineGPO(&self, pszcomputername: &windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> windows_core::Result<()>;
    fn Save(&self, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn GetName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn SetDisplayName(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPath(&self, pszpath: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetDSPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetFileSysPath(&self, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::Result<()>;
    fn GetRegistryKey(&self, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> windows_core::Result<()>;
    fn GetOptions(&self, dwoptions: *mut GPO_OPTIONS) -> windows_core::Result<()>;
    fn SetOptions(&self, dwoptions: GPO_OPTIONS, dwmask: u32) -> windows_core::Result<()>;
    fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::Result<()>;
    fn GetMachineName(&self, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetPropertySheetPages(&self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl windows_core::RuntimeName for IGroupPolicyObject {}
#[cfg(all(feature = "Win32_System_Registry", feature = "Win32_UI_Controls"))]
impl IGroupPolicyObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>() -> IGroupPolicyObject_Vtbl {
        unsafe extern "system" fn New<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdomainname: windows_core::PCWSTR, pszdisplayname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::New(this, core::mem::transmute(&pszdomainname), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenDSGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::OpenDSGPO(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenLocalMachineGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: GPO_OPEN_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::OpenLocalMachineGPO(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn OpenRemoteMachineGPO<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcomputername: windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::OpenRemoteMachineGPO(this, core::mem::transmute(&pszcomputername), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut windows_core::GUID, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::Save(this, core::mem::transmute_copy(&bmachine), core::mem::transmute_copy(&badd), core::mem::transmute_copy(&pguidextension), core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::Delete(this).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetDisplayName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::SetDisplayName(this, core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetPath(this, core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetDSPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetDSPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetFileSysPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszpath: windows_core::PWSTR, cchmaxpath: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetFileSysPath(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchmaxpath)).into()
        }
        unsafe extern "system" fn GetRegistryKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetRegistryKey(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&hkey)).into()
        }
        unsafe extern "system" fn GetOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: *mut GPO_OPTIONS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetOptions(this, core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn SetOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: GPO_OPTIONS, dwmask: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::SetOptions(this, core::mem::transmute_copy(&dwoptions), core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetType(this, core::mem::transmute_copy(&gpotype)).into()
        }
        unsafe extern "system" fn GetMachineName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetMachineName(this, core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetPropertySheetPages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGroupPolicyObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGroupPolicyObject_Impl::GetPropertySheetPages(this, core::mem::transmute_copy(&hpages), core::mem::transmute_copy(&upagecount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            New: New::<Identity, Impl, OFFSET>,
            OpenDSGPO: OpenDSGPO::<Identity, Impl, OFFSET>,
            OpenLocalMachineGPO: OpenLocalMachineGPO::<Identity, Impl, OFFSET>,
            OpenRemoteMachineGPO: OpenRemoteMachineGPO::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetDSPath: GetDSPath::<Identity, Impl, OFFSET>,
            GetFileSysPath: GetFileSysPath::<Identity, Impl, OFFSET>,
            GetRegistryKey: GetRegistryKey::<Identity, Impl, OFFSET>,
            GetOptions: GetOptions::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetMachineName: GetMachineName::<Identity, Impl, OFFSET>,
            GetPropertySheetPages: GetPropertySheetPages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGroupPolicyObject as windows_core::Interface>::IID
    }
}
pub trait IRSOPInformation_Impl: Sized {
    fn GetNamespace(&self, dwsection: u32, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::Result<()>;
    fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
    fn GetEventLogEntryText(&self, pszeventsource: &windows_core::PCWSTR, pszeventlogname: &windows_core::PCWSTR, pszeventtime: &windows_core::PCWSTR, dweventid: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IRSOPInformation {}
impl IRSOPInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: isize>() -> IRSOPInformation_Vtbl {
        unsafe extern "system" fn GetNamespace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsection: u32, pszname: windows_core::PWSTR, cchmaxlength: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRSOPInformation_Impl::GetNamespace(this, core::mem::transmute_copy(&dwsection), core::mem::transmute_copy(&pszname), core::mem::transmute_copy(&cchmaxlength)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRSOPInformation_Impl::GetFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetEventLogEntryText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRSOPInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszeventsource: windows_core::PCWSTR, pszeventlogname: windows_core::PCWSTR, pszeventtime: windows_core::PCWSTR, dweventid: u32, ppsztext: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRSOPInformation_Impl::GetEventLogEntryText(this, core::mem::transmute(&pszeventsource), core::mem::transmute(&pszeventlogname), core::mem::transmute(&pszeventtime), core::mem::transmute_copy(&dweventid)) {
                Ok(ok__) => {
                    core::ptr::write(ppsztext, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNamespace: GetNamespace::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetEventLogEntryText: GetEventLogEntryText::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRSOPInformation as windows_core::Interface>::IID
    }
}
