#[cfg(feature = "Win32_System_Com")]
pub trait ContextInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn IsInTransaction(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetTransaction(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetTransactionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetActivityId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetContextId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ContextInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ContextInfo_Vtbl
    where
        Identity: ContextInfo_Impl,
    {
        unsafe extern "system" fn IsInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ContextInfo_Impl::IsInTransaction(this) {
                Ok(ok__) => {
                    pbisintx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptx: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ContextInfo_Impl::GetTransaction(this) {
                Ok(ok__) => {
                    pptx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtxid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ContextInfo_Impl::GetTransactionId(this) {
                Ok(ok__) => {
                    pbstrtxid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstractivityid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ContextInfo_Impl::GetActivityId(this) {
                Ok(ok__) => {
                    pbstractivityid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrctxid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ContextInfo_Impl::GetContextId(this) {
                Ok(ok__) => {
                    pbstrctxid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, OFFSET>,
            GetTransaction: GetTransaction::<Identity, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, OFFSET>,
            GetActivityId: GetActivityId::<Identity, OFFSET>,
            GetContextId: GetContextId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ContextInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ContextInfo2_Impl: Sized + ContextInfo_Impl {
    fn GetPartitionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetApplicationId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetApplicationInstanceId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ContextInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ContextInfo2_Vtbl
    where
        Identity: ContextInfo2_Impl,
    {
        unsafe extern "system" fn GetPartitionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20000: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ContextInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ContextInfo2_Impl::GetPartitionId(this) {
                Ok(ok__) => {
                    __midl__contextinfo20000.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20001: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ContextInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ContextInfo2_Impl::GetApplicationId(this) {
                Ok(ok__) => {
                    __midl__contextinfo20001.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20002: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ContextInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ContextInfo2_Impl::GetApplicationInstanceId(this) {
                Ok(ok__) => {
                    __midl__contextinfo20002.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ContextInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPartitionId: GetPartitionId::<Identity, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ContextInfo2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ContextInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppDomainHelper_Impl: Sized + super::Com::IDispatch_Impl {
    fn Initialize(&self, punkad: Option<&windows_core::IUnknown>, __midl__iappdomainhelper0000: isize, ppool: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DoCallback(&self, punkad: Option<&windows_core::IUnknown>, __midl__iappdomainhelper0001: isize, ppool: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAppDomainHelper {}
#[cfg(feature = "Win32_System_Com")]
impl IAppDomainHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppDomainHelper_Vtbl
    where
        Identity: IAppDomainHelper_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkad: *mut core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppDomainHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppDomainHelper_Impl::Initialize(this, windows_core::from_raw_borrowed(&punkad), core::mem::transmute_copy(&__midl__iappdomainhelper0000), core::mem::transmute_copy(&ppool)).into()
        }
        unsafe extern "system" fn DoCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkad: *mut core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppDomainHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppDomainHelper_Impl::DoCallback(this, windows_core::from_raw_borrowed(&punkad), core::mem::transmute_copy(&__midl__iappdomainhelper0001), core::mem::transmute_copy(&ppool)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            DoCallback: DoCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppDomainHelper as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAssemblyLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetModules(&self, applicationdir: &windows_core::BSTR, applicationname: &windows_core::BSTR, assemblyname: &windows_core::BSTR) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAssemblyLocator {}
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyLocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAssemblyLocator_Vtbl
    where
        Identity: IAssemblyLocator_Impl,
    {
        unsafe extern "system" fn GetModules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationdir: core::mem::MaybeUninit<windows_core::BSTR>, applicationname: core::mem::MaybeUninit<windows_core::BSTR>, assemblyname: core::mem::MaybeUninit<windows_core::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: IAssemblyLocator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAssemblyLocator_Impl::GetModules(this, core::mem::transmute(&applicationdir), core::mem::transmute(&applicationname), core::mem::transmute(&assemblyname)) {
                Ok(ok__) => {
                    pmodules.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetModules: GetModules::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAssemblyLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IAsyncErrorNotify_Impl: Sized {
    fn OnError(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsyncErrorNotify {}
impl IAsyncErrorNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAsyncErrorNotify_Vtbl
    where
        Identity: IAsyncErrorNotify_Impl,
    {
        unsafe extern "system" fn OnError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IAsyncErrorNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncErrorNotify_Impl::OnError(this, core::mem::transmute_copy(&hr)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnError: OnError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncErrorNotify as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICOMAdminCatalog_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrcatalogservername: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn GetCollectionByQuery(&self, bstrcollname: &windows_core::BSTR, ppsavarquery: *const *const super::Com::SAFEARRAY) -> windows_core::Result<super::Com::IDispatch>;
    fn ImportComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InstallComponent(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ShutdownApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ExportApplication(&self, bstrapplidorname: &windows_core::BSTR, bstrapplicationfile: &windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> windows_core::Result<()>;
    fn InstallApplication(&self, bstrapplicationfile: &windows_core::BSTR, bstrdestinationdirectory: &windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopRouter(&self) -> windows_core::Result<()>;
    fn RefreshRouter(&self) -> windows_core::Result<()>;
    fn StartRouter(&self) -> windows_core::Result<()>;
    fn Reserved1(&self) -> windows_core::Result<()>;
    fn Reserved2(&self) -> windows_core::Result<()>;
    fn InstallMultipleComponents(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn GetMultipleComponentsInfo(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn RefreshComponents(&self) -> windows_core::Result<()>;
    fn BackupREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RestoreREGDB(&self, bstrbackupfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryApplicationFile(&self, bstrapplicationfile: &windows_core::BSTR, pbstrapplicationname: *mut windows_core::BSTR, pbstrapplicationdescription: *mut windows_core::BSTR, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn StartApplication(&self, bstrapplidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceCheck(&self, lservice: i32) -> windows_core::Result<i32>;
    fn InstallMultipleEventClasses(&self, bstrapplidorname: &windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn InstallEventClass(&self, bstrapplidorname: &windows_core::BSTR, bstrdll: &windows_core::BSTR, bstrtlb: &windows_core::BSTR, bstrpsdll: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetEventClassesForIID(&self, bstriid: &windows_core::BSTR, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICOMAdminCatalog {}
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICOMAdminCatalog_Vtbl
    where
        Identity: ICOMAdminCatalog_Impl,
    {
        unsafe extern "system" fn GetCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: core::mem::MaybeUninit<windows_core::BSTR>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog_Impl::GetCollection(this, core::mem::transmute(&bstrcollname)) {
                Ok(ok__) => {
                    ppcatalogcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcatalogservername: core::mem::MaybeUninit<windows_core::BSTR>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog_Impl::Connect(this, core::mem::transmute(&bstrcatalogservername)) {
                Ok(ok__) => {
                    ppcatalogcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog_Impl::MajorVersion(this) {
                Ok(ok__) => {
                    plmajorversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog_Impl::MinorVersion(this) {
                Ok(ok__) => {
                    plminorversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionByQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog_Impl::GetCollectionByQuery(this, core::mem::transmute(&bstrcollname), core::mem::transmute_copy(&ppsavarquery)) {
                Ok(ok__) => {
                    ppcatalogcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsidorprogid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::ImportComponent(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrclsidorprogid)).into()
        }
        unsafe extern "system" fn InstallComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdll: core::mem::MaybeUninit<windows_core::BSTR>, bstrtlb: core::mem::MaybeUninit<windows_core::BSTR>, bstrpsdll: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::InstallComponent(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrdll), core::mem::transmute(&bstrtlb), core::mem::transmute(&bstrpsdll)).into()
        }
        unsafe extern "system" fn ShutdownApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::ShutdownApplication(this, core::mem::transmute(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ExportApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrapplicationfile: core::mem::MaybeUninit<windows_core::BSTR>, loptions: COMAdminApplicationExportOptions) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::ExportApplication(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrapplicationfile), core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: core::mem::MaybeUninit<windows_core::BSTR>, bstrdestinationdirectory: core::mem::MaybeUninit<windows_core::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstrrsn: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::InstallApplication(this, core::mem::transmute(&bstrapplicationfile), core::mem::transmute(&bstrdestinationdirectory), core::mem::transmute_copy(&loptions), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrrsn)).into()
        }
        unsafe extern "system" fn StopRouter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::StopRouter(this).into()
        }
        unsafe extern "system" fn RefreshRouter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::RefreshRouter(this).into()
        }
        unsafe extern "system" fn StartRouter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::StartRouter(this).into()
        }
        unsafe extern "system" fn Reserved1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::Reserved1(this).into()
        }
        unsafe extern "system" fn Reserved2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::Reserved2(this).into()
        }
        unsafe extern "system" fn InstallMultipleComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::InstallMultipleComponents(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::GetMultipleComponentsInfo(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids), core::mem::transmute_copy(&ppsavarclassnames), core::mem::transmute_copy(&ppsavarfileflags), core::mem::transmute_copy(&ppsavarcomponentflags)).into()
        }
        unsafe extern "system" fn RefreshComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::RefreshComponents(this).into()
        }
        unsafe extern "system" fn BackupREGDB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupfilepath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::BackupREGDB(this, core::mem::transmute(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn RestoreREGDB<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupfilepath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::RestoreREGDB(this, core::mem::transmute(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn QueryApplicationFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: core::mem::MaybeUninit<windows_core::BSTR>, pbstrapplicationname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrapplicationdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::QueryApplicationFile(this, core::mem::transmute(&bstrapplicationfile), core::mem::transmute_copy(&pbstrapplicationname), core::mem::transmute_copy(&pbstrapplicationdescription), core::mem::transmute_copy(&pbhasusers), core::mem::transmute_copy(&pbisproxy), core::mem::transmute_copy(&ppsavarfilenames)).into()
        }
        unsafe extern "system" fn StartApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::StartApplication(this, core::mem::transmute(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ServiceCheck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog_Impl::ServiceCheck(this, core::mem::transmute_copy(&lservice)) {
                Ok(ok__) => {
                    plstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::InstallMultipleEventClasses(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn InstallEventClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdll: core::mem::MaybeUninit<windows_core::BSTR>, bstrtlb: core::mem::MaybeUninit<windows_core::BSTR>, bstrpsdll: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::InstallEventClass(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrdll), core::mem::transmute(&bstrtlb), core::mem::transmute(&bstrpsdll)).into()
        }
        unsafe extern "system" fn GetEventClassesForIID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstriid: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog_Impl::GetEventClassesForIID(this, core::mem::transmute(&bstriid), core::mem::transmute_copy(&ppsavarclsids), core::mem::transmute_copy(&ppsavarprogids), core::mem::transmute_copy(&ppsavardescriptions)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetCollection: GetCollection::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
            GetCollectionByQuery: GetCollectionByQuery::<Identity, OFFSET>,
            ImportComponent: ImportComponent::<Identity, OFFSET>,
            InstallComponent: InstallComponent::<Identity, OFFSET>,
            ShutdownApplication: ShutdownApplication::<Identity, OFFSET>,
            ExportApplication: ExportApplication::<Identity, OFFSET>,
            InstallApplication: InstallApplication::<Identity, OFFSET>,
            StopRouter: StopRouter::<Identity, OFFSET>,
            RefreshRouter: RefreshRouter::<Identity, OFFSET>,
            StartRouter: StartRouter::<Identity, OFFSET>,
            Reserved1: Reserved1::<Identity, OFFSET>,
            Reserved2: Reserved2::<Identity, OFFSET>,
            InstallMultipleComponents: InstallMultipleComponents::<Identity, OFFSET>,
            GetMultipleComponentsInfo: GetMultipleComponentsInfo::<Identity, OFFSET>,
            RefreshComponents: RefreshComponents::<Identity, OFFSET>,
            BackupREGDB: BackupREGDB::<Identity, OFFSET>,
            RestoreREGDB: RestoreREGDB::<Identity, OFFSET>,
            QueryApplicationFile: QueryApplicationFile::<Identity, OFFSET>,
            StartApplication: StartApplication::<Identity, OFFSET>,
            ServiceCheck: ServiceCheck::<Identity, OFFSET>,
            InstallMultipleEventClasses: InstallMultipleEventClasses::<Identity, OFFSET>,
            InstallEventClass: InstallEventClass::<Identity, OFFSET>,
            GetEventClassesForIID: GetEventClassesForIID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICOMAdminCatalog as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICOMAdminCatalog2_Impl: Sized + ICOMAdminCatalog_Impl {
    fn GetCollectionByQuery2(&self, bstrcollectionname: &windows_core::BSTR, pvarquerystrings: *const windows_core::VARIANT) -> windows_core::Result<super::Com::IDispatch>;
    fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const windows_core::VARIANT, lreasoncode: i32) -> windows_core::Result<()>;
    fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DumpApplicationInstance(&self, bstrapplicationinstanceid: &windows_core::BSTR, bstrdirectory: &windows_core::BSTR, lmaximages: i32) -> windows_core::Result<windows_core::BSTR>;
    fn IsApplicationInstanceDumpSupported(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CreateServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR, bstrservicename: &windows_core::BSTR, bstrstarttype: &windows_core::BSTR, bstrerrorcontrol: &windows_core::BSTR, bstrdependencies: &windows_core::BSTR, bstrrunas: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteServiceForApplication(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPartitionID(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetPartitionName(&self, bstrapplicationidorname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetCurrentPartition(&self, bstrpartitionidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CurrentPartitionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentPartitionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GlobalPartitionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FlushPartitionCache(&self) -> windows_core::Result<()>;
    fn CopyApplications(&self, bstrsourcepartitionidorname: &windows_core::BSTR, pvarapplicationid: *const windows_core::VARIANT, bstrdestinationpartitionidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CopyComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const windows_core::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MoveComponents(&self, bstrsourceapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const windows_core::VARIANT, bstrdestinationapplicationidorname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AliasComponent(&self, bstrsrcapplicationidorname: &windows_core::BSTR, bstrclsidorprogid: &windows_core::BSTR, bstrdestapplicationidorname: &windows_core::BSTR, bstrnewprogid: &windows_core::BSTR, bstrnewclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsSafeToDelete(&self, bstrdllname: &windows_core::BSTR) -> windows_core::Result<COMAdminInUse>;
    fn ImportUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const windows_core::VARIANT, pvarcomponenttype: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn PromoteUnconfiguredComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const windows_core::VARIANT, pvarcomponenttype: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn ImportComponents(&self, bstrapplicationidorname: &windows_core::BSTR, pvarclsidorprogid: *const windows_core::VARIANT, pvarcomponenttype: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Is64BitCatalogServer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExportPartition(&self, bstrpartitionidorname: &windows_core::BSTR, bstrpartitionfilename: &windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> windows_core::Result<()>;
    fn InstallPartition(&self, bstrfilename: &windows_core::BSTR, bstrdestdirectory: &windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrrsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryApplicationFile2(&self, bstrapplicationfile: &windows_core::BSTR) -> windows_core::Result<super::Com::IDispatch>;
    fn GetComponentVersionCount(&self, bstrclsidorprogid: &windows_core::BSTR) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICOMAdminCatalog2 {}
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICOMAdminCatalog2_Vtbl
    where
        Identity: ICOMAdminCatalog2_Impl,
    {
        unsafe extern "system" fn GetCollectionByQuery2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollectionname: core::mem::MaybeUninit<windows_core::BSTR>, pvarquerystrings: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::GetCollectionByQuery2(this, core::mem::transmute(&bstrcollectionname), core::mem::transmute_copy(&pvarquerystrings)) {
                Ok(ok__) => {
                    ppcatalogcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::GetApplicationInstanceIDFromProcessID(this, core::mem::transmute_copy(&lprocessid)) {
                Ok(ok__) => {
                    pbstrapplicationinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::ShutdownApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn PauseApplicationInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::PauseApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn ResumeApplicationInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::ResumeApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn RecycleApplicationInstances<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>, lreasoncode: i32) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::RecycleApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid), core::mem::transmute_copy(&lreasoncode)).into()
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarboolpaused: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::AreApplicationInstancesPaused(this, core::mem::transmute_copy(&pvarapplicationinstanceid)) {
                Ok(ok__) => {
                    pvarboolpaused.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpApplicationInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationinstanceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrdirectory: core::mem::MaybeUninit<windows_core::BSTR>, lmaximages: i32, pbstrdumpfile: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::DumpApplicationInstance(this, core::mem::transmute(&bstrapplicationinstanceid), core::mem::transmute(&bstrdirectory), core::mem::transmute_copy(&lmaximages)) {
                Ok(ok__) => {
                    pbstrdumpfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbooldumpsupported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::IsApplicationInstanceDumpSupported(this) {
                Ok(ok__) => {
                    pvarbooldumpsupported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServiceForApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrservicename: core::mem::MaybeUninit<windows_core::BSTR>, bstrstarttype: core::mem::MaybeUninit<windows_core::BSTR>, bstrerrorcontrol: core::mem::MaybeUninit<windows_core::BSTR>, bstrdependencies: core::mem::MaybeUninit<windows_core::BSTR>, bstrrunas: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::CreateServiceForApplication(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute(&bstrservicename), core::mem::transmute(&bstrstarttype), core::mem::transmute(&bstrerrorcontrol), core::mem::transmute(&bstrdependencies), core::mem::transmute(&bstrrunas), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&bdesktopok)).into()
        }
        unsafe extern "system" fn DeleteServiceForApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::DeleteServiceForApplication(this, core::mem::transmute(&bstrapplicationidorname)).into()
        }
        unsafe extern "system" fn GetPartitionID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrpartitionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::GetPartitionID(this, core::mem::transmute(&bstrapplicationidorname)) {
                Ok(ok__) => {
                    pbstrpartitionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartitionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrpartitionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::GetPartitionName(this, core::mem::transmute(&bstrapplicationidorname)) {
                Ok(ok__) => {
                    pbstrpartitionname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPartition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpartitionidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::SetCurrentPartition(this, core::mem::transmute(&bstrpartitionidorname)).into()
        }
        unsafe extern "system" fn CurrentPartitionID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpartitionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::CurrentPartitionID(this) {
                Ok(ok__) => {
                    pbstrpartitionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPartitionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpartitionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::CurrentPartitionName(this) {
                Ok(ok__) => {
                    pbstrpartitionname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalPartitionID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrglobalpartitionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::GlobalPartitionID(this) {
                Ok(ok__) => {
                    pbstrglobalpartitionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushPartitionCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::FlushPartitionCache(this).into()
        }
        unsafe extern "system" fn CopyApplications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourcepartitionidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarapplicationid: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrdestinationpartitionidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::CopyApplications(this, core::mem::transmute(&bstrsourcepartitionidorname), core::mem::transmute_copy(&pvarapplicationid), core::mem::transmute(&bstrdestinationpartitionidorname)).into()
        }
        unsafe extern "system" fn CopyComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrdestinationapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::CopyComponents(this, core::mem::transmute(&bstrsourceapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn MoveComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrdestinationapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::MoveComponents(this, core::mem::transmute(&bstrsourceapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn AliasComponent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsrcapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsidorprogid: core::mem::MaybeUninit<windows_core::BSTR>, bstrdestapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrnewprogid: core::mem::MaybeUninit<windows_core::BSTR>, bstrnewclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::AliasComponent(this, core::mem::transmute(&bstrsrcapplicationidorname), core::mem::transmute(&bstrclsidorprogid), core::mem::transmute(&bstrdestapplicationidorname), core::mem::transmute(&bstrnewprogid), core::mem::transmute(&bstrnewclsid)).into()
        }
        unsafe extern "system" fn IsSafeToDelete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdllname: core::mem::MaybeUninit<windows_core::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::IsSafeToDelete(this, core::mem::transmute(&bstrdllname)) {
                Ok(ok__) => {
                    pcomadmininuse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarcomponenttype: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::ImportUnconfiguredComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarcomponenttype: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::PromoteUnconfiguredComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn ImportComponents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarcomponenttype: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::ImportComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn Is64BitCatalogServer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbis64bit: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::Is64BitCatalogServer(this) {
                Ok(ok__) => {
                    pbis64bit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPartition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpartitionidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrpartitionfilename: core::mem::MaybeUninit<windows_core::BSTR>, loptions: COMAdminApplicationExportOptions) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::ExportPartition(this, core::mem::transmute(&bstrpartitionidorname), core::mem::transmute(&bstrpartitionfilename), core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallPartition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, bstrdestdirectory: core::mem::MaybeUninit<windows_core::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstrrsn: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMAdminCatalog2_Impl::InstallPartition(this, core::mem::transmute(&bstrfilename), core::mem::transmute(&bstrdestdirectory), core::mem::transmute_copy(&loptions), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrrsn)).into()
        }
        unsafe extern "system" fn QueryApplicationFile2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: core::mem::MaybeUninit<windows_core::BSTR>, ppfilesforimport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::QueryApplicationFile2(this, core::mem::transmute(&bstrapplicationfile)) {
                Ok(ok__) => {
                    ppfilesforimport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentVersionCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsidorprogid: core::mem::MaybeUninit<windows_core::BSTR>, plversioncount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICOMAdminCatalog2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICOMAdminCatalog2_Impl::GetComponentVersionCount(this, core::mem::transmute(&bstrclsidorprogid)) {
                Ok(ok__) => {
                    plversioncount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICOMAdminCatalog_Vtbl::new::<Identity, OFFSET>(),
            GetCollectionByQuery2: GetCollectionByQuery2::<Identity, OFFSET>,
            GetApplicationInstanceIDFromProcessID: GetApplicationInstanceIDFromProcessID::<Identity, OFFSET>,
            ShutdownApplicationInstances: ShutdownApplicationInstances::<Identity, OFFSET>,
            PauseApplicationInstances: PauseApplicationInstances::<Identity, OFFSET>,
            ResumeApplicationInstances: ResumeApplicationInstances::<Identity, OFFSET>,
            RecycleApplicationInstances: RecycleApplicationInstances::<Identity, OFFSET>,
            AreApplicationInstancesPaused: AreApplicationInstancesPaused::<Identity, OFFSET>,
            DumpApplicationInstance: DumpApplicationInstance::<Identity, OFFSET>,
            IsApplicationInstanceDumpSupported: IsApplicationInstanceDumpSupported::<Identity, OFFSET>,
            CreateServiceForApplication: CreateServiceForApplication::<Identity, OFFSET>,
            DeleteServiceForApplication: DeleteServiceForApplication::<Identity, OFFSET>,
            GetPartitionID: GetPartitionID::<Identity, OFFSET>,
            GetPartitionName: GetPartitionName::<Identity, OFFSET>,
            SetCurrentPartition: SetCurrentPartition::<Identity, OFFSET>,
            CurrentPartitionID: CurrentPartitionID::<Identity, OFFSET>,
            CurrentPartitionName: CurrentPartitionName::<Identity, OFFSET>,
            GlobalPartitionID: GlobalPartitionID::<Identity, OFFSET>,
            FlushPartitionCache: FlushPartitionCache::<Identity, OFFSET>,
            CopyApplications: CopyApplications::<Identity, OFFSET>,
            CopyComponents: CopyComponents::<Identity, OFFSET>,
            MoveComponents: MoveComponents::<Identity, OFFSET>,
            AliasComponent: AliasComponent::<Identity, OFFSET>,
            IsSafeToDelete: IsSafeToDelete::<Identity, OFFSET>,
            ImportUnconfiguredComponents: ImportUnconfiguredComponents::<Identity, OFFSET>,
            PromoteUnconfiguredComponents: PromoteUnconfiguredComponents::<Identity, OFFSET>,
            ImportComponents: ImportComponents::<Identity, OFFSET>,
            Is64BitCatalogServer: Is64BitCatalogServer::<Identity, OFFSET>,
            ExportPartition: ExportPartition::<Identity, OFFSET>,
            InstallPartition: InstallPartition::<Identity, OFFSET>,
            QueryApplicationFile2: QueryApplicationFile2::<Identity, OFFSET>,
            GetComponentVersionCount: GetComponentVersionCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICOMAdminCatalog2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICOMAdminCatalog as windows_core::Interface>::IID
    }
}
pub trait ICOMLBArguments_Impl: Sized {
    fn GetCLSID(&self, pclsid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn SetCLSID(&self, pclsid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetMachineName(&self, cchsvr: u32, szservername: windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetMachineName(&self, cchsvr: u32, szservername: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICOMLBArguments {}
impl ICOMLBArguments_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICOMLBArguments_Vtbl
    where
        Identity: ICOMLBArguments_Impl,
    {
        unsafe extern "system" fn GetCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICOMLBArguments_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMLBArguments_Impl::GetCLSID(this, core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICOMLBArguments_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMLBArguments_Impl::SetCLSID(this, core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn GetMachineName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchsvr: u32, szservername: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICOMLBArguments_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMLBArguments_Impl::GetMachineName(this, core::mem::transmute_copy(&cchsvr), core::mem::transmute_copy(&szservername)).into()
        }
        unsafe extern "system" fn SetMachineName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchsvr: u32, szservername: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICOMLBArguments_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICOMLBArguments_Impl::SetMachineName(this, core::mem::transmute_copy(&cchsvr), core::mem::transmute(&szservername)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLSID: GetCLSID::<Identity, OFFSET>,
            SetCLSID: SetCLSID::<Identity, OFFSET>,
            GetMachineName: GetMachineName::<Identity, OFFSET>,
            SetMachineName: SetMachineName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICOMLBArguments as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICatalogCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<super::Com::IDispatch>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn Add(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Populate(&self) -> windows_core::Result<()>;
    fn SaveChanges(&self) -> windows_core::Result<i32>;
    fn GetCollection(&self, bstrcollname: &windows_core::BSTR, varobjectkey: &windows_core::VARIANT) -> windows_core::Result<super::Com::IDispatch>;
    fn Name(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn AddEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RemoveEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetUtilInterface(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn DataStoreMajorVersion(&self) -> windows_core::Result<i32>;
    fn DataStoreMinorVersion(&self) -> windows_core::Result<i32>;
    fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn PopulateByQuery(&self, bstrquerystring: &windows_core::BSTR, lquerytype: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICatalogCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ICatalogCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICatalogCollection_Vtbl
    where
        Identity: ICatalogCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppcatalogobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    ppcatalogobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plobjectcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::Count(this) {
                Ok(ok__) => {
                    plobjectcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalogCollection_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcatalogobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::Add(this) {
                Ok(ok__) => {
                    ppcatalogobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Populate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalogCollection_Impl::Populate(this).into()
        }
        unsafe extern "system" fn SaveChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchanges: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::SaveChanges(this) {
                Ok(ok__) => {
                    pcchanges.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: core::mem::MaybeUninit<windows_core::BSTR>, varobjectkey: core::mem::MaybeUninit<windows_core::VARIANT>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::GetCollection(this, core::mem::transmute(&bstrcollname), core::mem::transmute(&varobjectkey)) {
                Ok(ok__) => {
                    ppcatalogcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarnamel: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::Name(this) {
                Ok(ok__) => {
                    pvarnamel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::AddEnabled(this) {
                Ok(ok__) => {
                    pvarbool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::RemoveEnabled(this) {
                Ok(ok__) => {
                    pvarbool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUtilInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::GetUtilInterface(this) {
                Ok(ok__) => {
                    ppidispatch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMajorVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::DataStoreMajorVersion(this) {
                Ok(ok__) => {
                    plmajorversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMinorVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversionl: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogCollection_Impl::DataStoreMinorVersion(this) {
                Ok(ok__) => {
                    plminorversionl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulateByKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalogCollection_Impl::PopulateByKey(this, core::mem::transmute_copy(&psakeys)).into()
        }
        unsafe extern "system" fn PopulateByQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrquerystring: core::mem::MaybeUninit<windows_core::BSTR>, lquerytype: i32) -> windows_core::HRESULT
        where
            Identity: ICatalogCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalogCollection_Impl::PopulateByQuery(this, core::mem::transmute(&bstrquerystring), core::mem::transmute_copy(&lquerytype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Populate: Populate::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            GetCollection: GetCollection::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            AddEnabled: AddEnabled::<Identity, OFFSET>,
            RemoveEnabled: RemoveEnabled::<Identity, OFFSET>,
            GetUtilInterface: GetUtilInterface::<Identity, OFFSET>,
            DataStoreMajorVersion: DataStoreMajorVersion::<Identity, OFFSET>,
            DataStoreMinorVersion: DataStoreMinorVersion::<Identity, OFFSET>,
            PopulateByKey: PopulateByKey::<Identity, OFFSET>,
            PopulateByQuery: PopulateByQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICatalogObject_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Value(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn put_Value(&self, bstrpropname: &windows_core::BSTR, val: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Key(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Name(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsPropertyReadOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Valid(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPropertyWriteOnly(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICatalogObject {}
#[cfg(feature = "Win32_System_Com")]
impl ICatalogObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICatalogObject_Vtbl
    where
        Identity: ICatalogObject_Impl,
    {
        unsafe extern "system" fn get_Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, pvarretval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICatalogObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogObject_Impl::get_Value(this, core::mem::transmute(&bstrpropname)) {
                Ok(ok__) => {
                    pvarretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, val: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICatalogObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalogObject_Impl::put_Value(this, core::mem::transmute(&bstrpropname), core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn Key<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarretval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICatalogObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogObject_Impl::Key(this) {
                Ok(ok__) => {
                    pvarretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarretval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICatalogObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogObject_Impl::Name(this) {
                Ok(ok__) => {
                    pvarretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICatalogObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogObject_Impl::IsPropertyReadOnly(this, core::mem::transmute(&bstrpropname)) {
                Ok(ok__) => {
                    pbretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICatalogObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogObject_Impl::Valid(this) {
                Ok(ok__) => {
                    pbretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICatalogObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICatalogObject_Impl::IsPropertyWriteOnly(this, core::mem::transmute(&bstrpropname)) {
                Ok(ok__) => {
                    pbretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
            Key: Key::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            IsPropertyReadOnly: IsPropertyReadOnly::<Identity, OFFSET>,
            Valid: Valid::<Identity, OFFSET>,
            IsPropertyWriteOnly: IsPropertyWriteOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogObject as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ICheckSxsConfig_Impl: Sized {
    fn IsSameSxsConfig(&self, wszsxsname: &windows_core::PCWSTR, wszsxsdirectory: &windows_core::PCWSTR, wszsxsappname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICheckSxsConfig {}
impl ICheckSxsConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICheckSxsConfig_Vtbl
    where
        Identity: ICheckSxsConfig_Impl,
    {
        unsafe extern "system" fn IsSameSxsConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszsxsname: windows_core::PCWSTR, wszsxsdirectory: windows_core::PCWSTR, wszsxsappname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICheckSxsConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICheckSxsConfig_Impl::IsSameSxsConfig(this, core::mem::transmute(&wszsxsname), core::mem::transmute(&wszsxsdirectory), core::mem::transmute(&wszsxsappname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSameSxsConfig: IsSameSxsConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICheckSxsConfig as windows_core::Interface>::IID
    }
}
pub trait IComActivityEvents_Impl: Sized {
    fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32) -> windows_core::Result<()>;
    fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32, dwtimeout: u32) -> windows_core::Result<()>;
    fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> windows_core::Result<()>;
    fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidleft: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwcalldepth: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComActivityEvents {}
impl IComActivityEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComActivityEvents_Vtbl
    where
        Identity: IComActivityEvents_Impl,
    {
        unsafe extern "system" fn OnActivityCreate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComActivityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComActivityEvents_Impl::OnActivityCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityDestroy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComActivityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComActivityEvents_Impl::OnActivityDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32) -> windows_core::HRESULT
        where
            Identity: IComActivityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComActivityEvents_Impl::OnActivityEnter(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidentered), core::mem::transmute_copy(&dwthread)).into()
        }
        unsafe extern "system" fn OnActivityTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32, dwtimeout: u32) -> windows_core::HRESULT
        where
            Identity: IComActivityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComActivityEvents_Impl::OnActivityTimeout(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidentered), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnActivityReenter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> windows_core::HRESULT
        where
            Identity: IComActivityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComActivityEvents_Impl::OnActivityReenter(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwcalldepth)).into()
        }
        unsafe extern "system" fn OnActivityLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidleft: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComActivityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComActivityEvents_Impl::OnActivityLeave(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidleft)).into()
        }
        unsafe extern "system" fn OnActivityLeaveSame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwcalldepth: u32) -> windows_core::HRESULT
        where
            Identity: IComActivityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComActivityEvents_Impl::OnActivityLeaveSame(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&dwcalldepth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnActivityCreate: OnActivityCreate::<Identity, OFFSET>,
            OnActivityDestroy: OnActivityDestroy::<Identity, OFFSET>,
            OnActivityEnter: OnActivityEnter::<Identity, OFFSET>,
            OnActivityTimeout: OnActivityTimeout::<Identity, OFFSET>,
            OnActivityReenter: OnActivityReenter::<Identity, OFFSET>,
            OnActivityLeave: OnActivityLeave::<Identity, OFFSET>,
            OnActivityLeaveSame: OnActivityLeaveSame::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComActivityEvents as windows_core::Interface>::IID
    }
}
pub trait IComApp2Events_Impl: Sized {
    fn OnAppActivation2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID, guidprocess: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppForceShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppPaused2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID, bpaused: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnAppRecycle2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID, guidprocess: &windows_core::GUID, lreason: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComApp2Events {}
impl IComApp2Events_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComApp2Events_Vtbl
    where
        Identity: IComApp2Events_Impl,
    {
        unsafe extern "system" fn OnAppActivation2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComApp2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComApp2Events_Impl::OnAppActivation2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute(&guidprocess)).into()
        }
        unsafe extern "system" fn OnAppShutdown2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComApp2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComApp2Events_Impl::OnAppShutdown2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComApp2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComApp2Events_Impl::OnAppForceShutdown2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppPaused2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, bpaused: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComApp2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComApp2Events_Impl::OnAppPaused2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute_copy(&bpaused)).into()
        }
        unsafe extern "system" fn OnAppRecycle2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID, lreason: i32) -> windows_core::HRESULT
        where
            Identity: IComApp2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComApp2Events_Impl::OnAppRecycle2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute(&guidprocess), core::mem::transmute_copy(&lreason)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppActivation2: OnAppActivation2::<Identity, OFFSET>,
            OnAppShutdown2: OnAppShutdown2::<Identity, OFFSET>,
            OnAppForceShutdown2: OnAppForceShutdown2::<Identity, OFFSET>,
            OnAppPaused2: OnAppPaused2::<Identity, OFFSET>,
            OnAppRecycle2: OnAppRecycle2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComApp2Events as windows_core::Interface>::IID
    }
}
pub trait IComAppEvents_Impl: Sized {
    fn OnAppActivation(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnAppForceShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComAppEvents {}
impl IComAppEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComAppEvents_Vtbl
    where
        Identity: IComAppEvents_Impl,
    {
        unsafe extern "system" fn OnAppActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComAppEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComAppEvents_Impl::OnAppActivation(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppShutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComAppEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComAppEvents_Impl::OnAppShutdown(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComAppEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComAppEvents_Impl::OnAppForceShutdown(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppActivation: OnAppActivation::<Identity, OFFSET>,
            OnAppShutdown: OnAppShutdown::<Identity, OFFSET>,
            OnAppForceShutdown: OnAppForceShutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComAppEvents as windows_core::Interface>::IID
    }
}
pub trait IComCRMEvents_Impl: Sized {
    fn OnCRMRecoveryStart(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMRecoveryDone(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMCheckpoint(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMBegin(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID, guidactivity: &windows_core::GUID, guidtx: &windows_core::GUID, szprogidcompensator: &windows_core::PCWSTR, szdescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnCRMPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMIndoubt(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMDone(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMRelease(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMAnalyze(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> windows_core::Result<()>;
    fn OnCRMWrite(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> windows_core::Result<()>;
    fn OnCRMForget(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMForce(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnCRMDeliver(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComCRMEvents {}
impl IComCRMEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComCRMEvents_Vtbl
    where
        Identity: IComCRMEvents_Impl,
    {
        unsafe extern "system" fn OnCRMRecoveryStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMRecoveryStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMRecoveryDone(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMCheckpoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMCheckpoint(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMBegin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, guidactivity: windows_core::GUID, guidtx: windows_core::GUID, szprogidcompensator: windows_core::PCWSTR, szdescription: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMBegin(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute(&guidactivity), core::mem::transmute(&guidtx), core::mem::transmute(&szprogidcompensator), core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn OnCRMPrepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMIndoubt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMIndoubt(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMDone(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMRelease<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMRelease(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAnalyze<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMAnalyze(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&dwcrmrecordtype), core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMWrite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMWrite(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&fvariants), core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMForget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMForget(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMForce<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMForce(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDeliver<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> windows_core::HRESULT
        where
            Identity: IComCRMEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComCRMEvents_Impl::OnCRMDeliver(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&fvariants), core::mem::transmute_copy(&dwrecordsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCRMRecoveryStart: OnCRMRecoveryStart::<Identity, OFFSET>,
            OnCRMRecoveryDone: OnCRMRecoveryDone::<Identity, OFFSET>,
            OnCRMCheckpoint: OnCRMCheckpoint::<Identity, OFFSET>,
            OnCRMBegin: OnCRMBegin::<Identity, OFFSET>,
            OnCRMPrepare: OnCRMPrepare::<Identity, OFFSET>,
            OnCRMCommit: OnCRMCommit::<Identity, OFFSET>,
            OnCRMAbort: OnCRMAbort::<Identity, OFFSET>,
            OnCRMIndoubt: OnCRMIndoubt::<Identity, OFFSET>,
            OnCRMDone: OnCRMDone::<Identity, OFFSET>,
            OnCRMRelease: OnCRMRelease::<Identity, OFFSET>,
            OnCRMAnalyze: OnCRMAnalyze::<Identity, OFFSET>,
            OnCRMWrite: OnCRMWrite::<Identity, OFFSET>,
            OnCRMForget: OnCRMForget::<Identity, OFFSET>,
            OnCRMForce: OnCRMForce::<Identity, OFFSET>,
            OnCRMDeliver: OnCRMDeliver::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComCRMEvents as windows_core::Interface>::IID
    }
}
pub trait IComExceptionEvents_Impl: Sized {
    fn OnExceptionUser(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComExceptionEvents {}
impl IComExceptionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComExceptionEvents_Vtbl
    where
        Identity: IComExceptionEvents_Impl,
    {
        unsafe extern "system" fn OnExceptionUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IComExceptionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComExceptionEvents_Impl::OnExceptionUser(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&code), core::mem::transmute_copy(&address), core::mem::transmute(&pszstacktrace)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnExceptionUser: OnExceptionUser::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComExceptionEvents as windows_core::Interface>::IID
    }
}
pub trait IComIdentityEvents_Impl: Sized {
    fn OnIISRequestInfo(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: &windows_core::PCWSTR, pszserverip: &windows_core::PCWSTR, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComIdentityEvents {}
impl IComIdentityEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComIdentityEvents_Vtbl
    where
        Identity: IComIdentityEvents_Impl,
    {
        unsafe extern "system" fn OnIISRequestInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: windows_core::PCWSTR, pszserverip: windows_core::PCWSTR, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IComIdentityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComIdentityEvents_Impl::OnIISRequestInfo(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute(&pszclientip), core::mem::transmute(&pszserverip), core::mem::transmute(&pszurl)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIISRequestInfo: OnIISRequestInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComIdentityEvents as windows_core::Interface>::IID
    }
}
pub trait IComInstance2Events_Impl: Sized {
    fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComInstance2Events {}
impl IComInstance2Events_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComInstance2Events_Vtbl
    where
        Identity: IComInstance2Events_Impl,
    {
        unsafe extern "system" fn OnObjectCreate2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComInstance2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComInstance2Events_Impl::OnObjectCreate2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjectDestroy2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT
        where
            Identity: IComInstance2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComInstance2Events_Impl::OnObjectDestroy2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectCreate2: OnObjectCreate2::<Identity, OFFSET>,
            OnObjectDestroy2: OnObjectDestroy2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComInstance2Events as windows_core::Interface>::IID
    }
}
pub trait IComInstanceEvents_Impl: Sized {
    fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64) -> windows_core::Result<()>;
    fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComInstanceEvents {}
impl IComInstanceEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComInstanceEvents_Vtbl
    where
        Identity: IComInstanceEvents_Impl,
    {
        unsafe extern "system" fn OnObjectCreate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64) -> windows_core::HRESULT
        where
            Identity: IComInstanceEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComInstanceEvents_Impl::OnObjectCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDestroy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT
        where
            Identity: IComInstanceEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComInstanceEvents_Impl::OnObjectDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectCreate: OnObjectCreate::<Identity, OFFSET>,
            OnObjectDestroy: OnObjectDestroy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComInstanceEvents as windows_core::Interface>::IID
    }
}
pub trait IComLTxEvents_Impl: Sized {
    fn OnLtxTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID, tsid: &windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> windows_core::Result<()>;
    fn OnLtxTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID, fvote: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnLtxTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnLtxTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID) -> windows_core::Result<()>;
    fn OnLtxTransactionPromote(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &windows_core::GUID, txnid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComLTxEvents {}
impl IComLTxEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComLTxEvents_Vtbl
    where
        Identity: IComLTxEvents_Impl,
    {
        unsafe extern "system" fn OnLtxTransactionStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, tsid: windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> windows_core::HRESULT
        where
            Identity: IComLTxEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComLTxEvents_Impl::OnLtxTransactionStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute(&tsid), core::mem::transmute_copy(&froot), core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, fvote: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComLTxEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComLTxEvents_Impl::OnLtxTransactionPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute_copy(&fvote)).into()
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComLTxEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComLTxEvents_Impl::OnLtxTransactionAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComLTxEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComLTxEvents_Impl::OnLtxTransactionCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, txnid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComLTxEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComLTxEvents_Impl::OnLtxTransactionPromote(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute(&txnid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLtxTransactionStart: OnLtxTransactionStart::<Identity, OFFSET>,
            OnLtxTransactionPrepare: OnLtxTransactionPrepare::<Identity, OFFSET>,
            OnLtxTransactionAbort: OnLtxTransactionAbort::<Identity, OFFSET>,
            OnLtxTransactionCommit: OnLtxTransactionCommit::<Identity, OFFSET>,
            OnLtxTransactionPromote: OnLtxTransactionPromote::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComLTxEvents as windows_core::Interface>::IID
    }
}
pub trait IComMethod2Events_Impl: Sized {
    fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::Result<()>;
    fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComMethod2Events {}
impl IComMethod2Events_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComMethod2Events_Vtbl
    where
        Identity: IComMethod2Events_Impl,
    {
        unsafe extern "system" fn OnMethodCall2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::HRESULT
        where
            Identity: IComMethod2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComMethod2Events_Impl::OnMethodCall2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IComMethod2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComMethod2Events_Impl::OnMethodReturn2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::HRESULT
        where
            Identity: IComMethod2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComMethod2Events_Impl::OnMethodException2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMethodCall2: OnMethodCall2::<Identity, OFFSET>,
            OnMethodReturn2: OnMethodReturn2::<Identity, OFFSET>,
            OnMethodException2: OnMethodException2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComMethod2Events as windows_core::Interface>::IID
    }
}
pub trait IComMethodEvents_Impl: Sized {
    fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::Result<()>;
    fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComMethodEvents {}
impl IComMethodEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComMethodEvents_Vtbl
    where
        Identity: IComMethodEvents_Impl,
    {
        unsafe extern "system" fn OnMethodCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::HRESULT
        where
            Identity: IComMethodEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComMethodEvents_Impl::OnMethodCall(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IComMethodEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComMethodEvents_Impl::OnMethodReturn(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::HRESULT
        where
            Identity: IComMethodEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComMethodEvents_Impl::OnMethodException(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMethodCall: OnMethodCall::<Identity, OFFSET>,
            OnMethodReturn: OnMethodReturn::<Identity, OFFSET>,
            OnMethodException: OnMethodException::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComMethodEvents as windows_core::Interface>::IID
    }
}
pub trait IComMtaThreadPoolKnobs_Impl: Sized {
    fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> windows_core::Result<()>;
    fn MTAGetMaxThreadCount(&self) -> windows_core::Result<u32>;
    fn MTASetThrottleValue(&self, dwthrottle: u32) -> windows_core::Result<()>;
    fn MTAGetThrottleValue(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IComMtaThreadPoolKnobs {}
impl IComMtaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComMtaThreadPoolKnobs_Vtbl
    where
        Identity: IComMtaThreadPoolKnobs_Impl,
    {
        unsafe extern "system" fn MTASetMaxThreadCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxthreads: u32) -> windows_core::HRESULT
        where
            Identity: IComMtaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComMtaThreadPoolKnobs_Impl::MTASetMaxThreadCount(this, core::mem::transmute_copy(&dwmaxthreads)).into()
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComMtaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComMtaThreadPoolKnobs_Impl::MTAGetMaxThreadCount(this) {
                Ok(ok__) => {
                    pdwmaxthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MTASetThrottleValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthrottle: u32) -> windows_core::HRESULT
        where
            Identity: IComMtaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComMtaThreadPoolKnobs_Impl::MTASetThrottleValue(this, core::mem::transmute_copy(&dwthrottle)).into()
        }
        unsafe extern "system" fn MTAGetThrottleValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthrottle: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComMtaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComMtaThreadPoolKnobs_Impl::MTAGetThrottleValue(this) {
                Ok(ok__) => {
                    pdwthrottle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MTASetMaxThreadCount: MTASetMaxThreadCount::<Identity, OFFSET>,
            MTAGetMaxThreadCount: MTAGetMaxThreadCount::<Identity, OFFSET>,
            MTASetThrottleValue: MTASetThrottleValue::<Identity, OFFSET>,
            MTAGetThrottleValue: MTAGetThrottleValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComMtaThreadPoolKnobs as windows_core::Interface>::IID
    }
}
pub trait IComObjectConstruction2Events_Impl: Sized {
    fn OnObjectConstruct2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: &windows_core::PCWSTR, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComObjectConstruction2Events {}
impl IComObjectConstruction2Events_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComObjectConstruction2Events_Vtbl
    where
        Identity: IComObjectConstruction2Events_Impl,
    {
        unsafe extern "system" fn OnObjectConstruct2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: windows_core::PCWSTR, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComObjectConstruction2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectConstruction2Events_Impl::OnObjectConstruct2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute(&sconstructstring), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidpartition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnObjectConstruct2: OnObjectConstruct2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectConstruction2Events as windows_core::Interface>::IID
    }
}
pub trait IComObjectConstructionEvents_Impl: Sized {
    fn OnObjectConstruct(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: &windows_core::PCWSTR, oid: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComObjectConstructionEvents {}
impl IComObjectConstructionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComObjectConstructionEvents_Vtbl
    where
        Identity: IComObjectConstructionEvents_Impl,
    {
        unsafe extern "system" fn OnObjectConstruct<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: windows_core::PCWSTR, oid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectConstructionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectConstructionEvents_Impl::OnObjectConstruct(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute(&sconstructstring), core::mem::transmute_copy(&oid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnObjectConstruct: OnObjectConstruct::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectConstructionEvents as windows_core::Interface>::IID
    }
}
pub trait IComObjectEvents_Impl: Sized {
    fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::Result<()>;
    fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::Result<()>;
    fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
    fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
    fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
    fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComObjectEvents {}
impl IComObjectEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComObjectEvents_Vtbl
    where
        Identity: IComObjectEvents_Impl,
    {
        unsafe extern "system" fn OnObjectActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectEvents_Impl::OnObjectActivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDeactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectEvents_Impl::OnObjectDeactivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnDisableCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectEvents_Impl::OnDisableCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnEnableCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectEvents_Impl::OnEnableCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectEvents_Impl::OnSetComplete(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectEvents_Impl::OnSetAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectActivate: OnObjectActivate::<Identity, OFFSET>,
            OnObjectDeactivate: OnObjectDeactivate::<Identity, OFFSET>,
            OnDisableCommit: OnDisableCommit::<Identity, OFFSET>,
            OnEnableCommit: OnEnableCommit::<Identity, OFFSET>,
            OnSetComplete: OnSetComplete::<Identity, OFFSET>,
            OnSetAbort: OnSetAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectEvents as windows_core::Interface>::IID
    }
}
pub trait IComObjectPool2Events_Impl: Sized {
    fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()>;
    fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64, guidpartition: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComObjectPool2Events {}
impl IComObjectPool2Events_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComObjectPool2Events_Vtbl
    where
        Identity: IComObjectPool2Events_Impl,
    {
        unsafe extern "system" fn OnObjPoolPutObject2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectPool2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPool2Events_Impl::OnObjPoolPutObject2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&nreason), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComObjectPool2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPool2Events_Impl::OnObjPoolGetObject2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectPool2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPool2Events_Impl::OnObjPoolRecycleToTx2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComObjectPool2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPool2Events_Impl::OnObjPoolGetFromTx2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid), core::mem::transmute_copy(&guidpartition)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject2: OnObjPoolPutObject2::<Identity, OFFSET>,
            OnObjPoolGetObject2: OnObjPoolGetObject2::<Identity, OFFSET>,
            OnObjPoolRecycleToTx2: OnObjPoolRecycleToTx2::<Identity, OFFSET>,
            OnObjPoolGetFromTx2: OnObjPoolGetFromTx2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectPool2Events as windows_core::Interface>::IID
    }
}
pub trait IComObjectPoolEvents_Impl: Sized {
    fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()>;
    fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComObjectPoolEvents {}
impl IComObjectPoolEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComObjectPoolEvents_Vtbl
    where
        Identity: IComObjectPoolEvents_Impl,
    {
        unsafe extern "system" fn OnObjPoolPutObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents_Impl::OnObjPoolPutObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&nreason), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents_Impl::OnObjPoolGetObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents_Impl::OnObjPoolRecycleToTx(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents_Impl::OnObjPoolGetFromTx(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject: OnObjPoolPutObject::<Identity, OFFSET>,
            OnObjPoolGetObject: OnObjPoolGetObject::<Identity, OFFSET>,
            OnObjPoolRecycleToTx: OnObjPoolRecycleToTx::<Identity, OFFSET>,
            OnObjPoolGetFromTx: OnObjPoolGetFromTx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectPoolEvents as windows_core::Interface>::IID
    }
}
pub trait IComObjectPoolEvents2_Impl: Sized {
    fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::Result<()>;
    fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> windows_core::Result<()>;
    fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, guidactivity: *const windows_core::GUID, dwtimeout: u32) -> windows_core::Result<()>;
    fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComObjectPoolEvents2 {}
impl IComObjectPoolEvents2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComObjectPoolEvents2_Vtbl
    where
        Identity: IComObjectPoolEvents2_Impl,
    {
        unsafe extern "system" fn OnObjPoolCreateObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents2_Impl::OnObjPoolCreateObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwobjscreated), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents2_Impl::OnObjPoolDestroyObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwobjscreated), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents2_Impl::OnObjPoolCreateDecision(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&dwthreadswaiting), core::mem::transmute_copy(&dwavail), core::mem::transmute_copy(&dwcreated), core::mem::transmute_copy(&dwmin), core::mem::transmute_copy(&dwmax)).into()
        }
        unsafe extern "system" fn OnObjPoolTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, guidactivity: *const windows_core::GUID, dwtimeout: u32) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents2_Impl::OnObjPoolTimeout(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> windows_core::HRESULT
        where
            Identity: IComObjectPoolEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComObjectPoolEvents2_Impl::OnObjPoolCreatePool(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwmin), core::mem::transmute_copy(&dwmax), core::mem::transmute_copy(&dwtimeout)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolCreateObject: OnObjPoolCreateObject::<Identity, OFFSET>,
            OnObjPoolDestroyObject: OnObjPoolDestroyObject::<Identity, OFFSET>,
            OnObjPoolCreateDecision: OnObjPoolCreateDecision::<Identity, OFFSET>,
            OnObjPoolTimeout: OnObjPoolTimeout::<Identity, OFFSET>,
            OnObjPoolCreatePool: OnObjPoolCreatePool::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComObjectPoolEvents2 as windows_core::Interface>::IID
    }
}
pub trait IComQCEvents_Impl: Sized {
    fn OnQCRecord(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &windows_core::PCWSTR, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, msmqhr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnQCQueueOpen(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: &windows_core::PCWSTR, queueid: u64, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, retryindex: u32) -> windows_core::Result<()>;
    fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComQCEvents {}
impl IComQCEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComQCEvents_Vtbl
    where
        Identity: IComQCEvents_Impl,
    {
        unsafe extern "system" fn OnQCRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: windows_core::PCWSTR, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, msmqhr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IComQCEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComQCEvents_Impl::OnQCRecord(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute(&szqueue), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCQueueOpen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: windows_core::PCWSTR, queueid: u64, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IComQCEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComQCEvents_Impl::OnQCQueueOpen(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&szqueue), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IComQCEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComQCEvents_Impl::OnQCReceive(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceiveFail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IComQCEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComQCEvents_Impl::OnQCReceiveFail(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, retryindex: u32) -> windows_core::HRESULT
        where
            Identity: IComQCEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComQCEvents_Impl::OnQCMoveToReTryQueue(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&retryindex)).into()
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComQCEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComQCEvents_Impl::OnQCMoveToDeadQueue(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid)).into()
        }
        unsafe extern "system" fn OnQCPlayback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IComQCEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComQCEvents_Impl::OnQCPlayback(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnQCRecord: OnQCRecord::<Identity, OFFSET>,
            OnQCQueueOpen: OnQCQueueOpen::<Identity, OFFSET>,
            OnQCReceive: OnQCReceive::<Identity, OFFSET>,
            OnQCReceiveFail: OnQCReceiveFail::<Identity, OFFSET>,
            OnQCMoveToReTryQueue: OnQCMoveToReTryQueue::<Identity, OFFSET>,
            OnQCMoveToDeadQueue: OnQCMoveToDeadQueue::<Identity, OFFSET>,
            OnQCPlayback: OnQCPlayback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComQCEvents as windows_core::Interface>::IID
    }
}
pub trait IComResourceEvents_Impl: Sized {
    fn OnResourceCreate(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnResourceAllocate(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> windows_core::Result<()>;
    fn OnResourceRecycle(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &windows_core::PCWSTR, resid: u64) -> windows_core::Result<()>;
    fn OnResourceDestroy(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: windows_core::HRESULT, psztype: &windows_core::PCWSTR, resid: u64) -> windows_core::Result<()>;
    fn OnResourceTrack(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComResourceEvents {}
impl IComResourceEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComResourceEvents_Vtbl
    where
        Identity: IComResourceEvents_Impl,
    {
        unsafe extern "system" fn OnResourceCreate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComResourceEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComResourceEvents_Impl::OnResourceCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted)).into()
        }
        unsafe extern "system" fn OnResourceAllocate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> windows_core::HRESULT
        where
            Identity: IComResourceEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComResourceEvents_Impl::OnResourceAllocate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted), core::mem::transmute_copy(&numrated), core::mem::transmute_copy(&rating)).into()
        }
        unsafe extern "system" fn OnResourceRecycle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64) -> windows_core::HRESULT
        where
            Identity: IComResourceEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComResourceEvents_Impl::OnResourceRecycle(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceDestroy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: windows_core::HRESULT, psztype: windows_core::PCWSTR, resid: u64) -> windows_core::HRESULT
        where
            Identity: IComResourceEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComResourceEvents_Impl::OnResourceDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&hr), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceTrack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComResourceEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComResourceEvents_Impl::OnResourceTrack(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnResourceCreate: OnResourceCreate::<Identity, OFFSET>,
            OnResourceAllocate: OnResourceAllocate::<Identity, OFFSET>,
            OnResourceRecycle: OnResourceRecycle::<Identity, OFFSET>,
            OnResourceDestroy: OnResourceDestroy::<Identity, OFFSET>,
            OnResourceTrack: OnResourceTrack::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComResourceEvents as windows_core::Interface>::IID
    }
}
pub trait IComSecurityEvents_Impl: Sized {
    fn OnAuthenticate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnAuthenticateFail(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComSecurityEvents {}
impl IComSecurityEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComSecurityEvents_Vtbl
    where
        Identity: IComSecurityEvents_Impl,
    {
        unsafe extern "system" fn OnAuthenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComSecurityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComSecurityEvents_Impl::OnAuthenticate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidiid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&cbbyteorig), core::mem::transmute_copy(&psidoriginaluser), core::mem::transmute_copy(&cbbytecur), core::mem::transmute_copy(&psidcurrentuser), core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        unsafe extern "system" fn OnAuthenticateFail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComSecurityEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComSecurityEvents_Impl::OnAuthenticateFail(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidiid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&cbbyteorig), core::mem::transmute_copy(&psidoriginaluser), core::mem::transmute_copy(&cbbytecur), core::mem::transmute_copy(&psidcurrentuser), core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAuthenticate: OnAuthenticate::<Identity, OFFSET>,
            OnAuthenticateFail: OnAuthenticateFail::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComSecurityEvents as windows_core::Interface>::IID
    }
}
pub trait IComStaThreadPoolKnobs_Impl: Sized {
    fn SetMinThreadCount(&self, minthreads: u32) -> windows_core::Result<()>;
    fn GetMinThreadCount(&self) -> windows_core::Result<u32>;
    fn SetMaxThreadCount(&self, maxthreads: u32) -> windows_core::Result<()>;
    fn GetMaxThreadCount(&self) -> windows_core::Result<u32>;
    fn SetActivityPerThread(&self, activitiesperthread: u32) -> windows_core::Result<()>;
    fn GetActivityPerThread(&self) -> windows_core::Result<u32>;
    fn SetActivityRatio(&self, activityratio: f64) -> windows_core::Result<()>;
    fn GetActivityRatio(&self) -> windows_core::Result<f64>;
    fn GetThreadCount(&self) -> windows_core::Result<u32>;
    fn GetQueueDepth(&self) -> windows_core::Result<u32>;
    fn SetQueueDepth(&self, dwqdepth: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComStaThreadPoolKnobs {}
impl IComStaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComStaThreadPoolKnobs_Vtbl
    where
        Identity: IComStaThreadPoolKnobs_Impl,
    {
        unsafe extern "system" fn SetMinThreadCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minthreads: u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs_Impl::SetMinThreadCount(this, core::mem::transmute_copy(&minthreads)).into()
        }
        unsafe extern "system" fn GetMinThreadCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs_Impl::GetMinThreadCount(this) {
                Ok(ok__) => {
                    minthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxThreadCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxthreads: u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs_Impl::SetMaxThreadCount(this, core::mem::transmute_copy(&maxthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreadCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs_Impl::GetMaxThreadCount(this) {
                Ok(ok__) => {
                    maxthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityPerThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitiesperthread: u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs_Impl::SetActivityPerThread(this, core::mem::transmute_copy(&activitiesperthread)).into()
        }
        unsafe extern "system" fn GetActivityPerThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitiesperthread: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs_Impl::GetActivityPerThread(this) {
                Ok(ok__) => {
                    activitiesperthread.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityRatio<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activityratio: f64) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs_Impl::SetActivityRatio(this, core::mem::transmute_copy(&activityratio)).into()
        }
        unsafe extern "system" fn GetActivityRatio<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activityratio: *mut f64) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs_Impl::GetActivityRatio(this) {
                Ok(ok__) => {
                    activityratio.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs_Impl::GetThreadCount(this) {
                Ok(ok__) => {
                    pdwthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueueDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwqdepth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs_Impl::GetQueueDepth(this) {
                Ok(ok__) => {
                    pdwqdepth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueueDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwqdepth: i32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs_Impl::SetQueueDepth(this, core::mem::transmute_copy(&dwqdepth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMinThreadCount: SetMinThreadCount::<Identity, OFFSET>,
            GetMinThreadCount: GetMinThreadCount::<Identity, OFFSET>,
            SetMaxThreadCount: SetMaxThreadCount::<Identity, OFFSET>,
            GetMaxThreadCount: GetMaxThreadCount::<Identity, OFFSET>,
            SetActivityPerThread: SetActivityPerThread::<Identity, OFFSET>,
            GetActivityPerThread: GetActivityPerThread::<Identity, OFFSET>,
            SetActivityRatio: SetActivityRatio::<Identity, OFFSET>,
            GetActivityRatio: GetActivityRatio::<Identity, OFFSET>,
            GetThreadCount: GetThreadCount::<Identity, OFFSET>,
            GetQueueDepth: GetQueueDepth::<Identity, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs as windows_core::Interface>::IID
    }
}
pub trait IComStaThreadPoolKnobs2_Impl: Sized + IComStaThreadPoolKnobs_Impl {
    fn GetMaxCPULoad(&self) -> windows_core::Result<u32>;
    fn SetMaxCPULoad(&self, pdwload: i32) -> windows_core::Result<()>;
    fn GetCPUMetricEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCPUMetricEnabled(&self, bmetricenabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetCreateThreadsAggressively(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCreateThreadsAggressively(&self, bmetricenabled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetMaxCSR(&self) -> windows_core::Result<u32>;
    fn SetMaxCSR(&self, dwcsr: i32) -> windows_core::Result<()>;
    fn GetWaitTimeForThreadCleanup(&self) -> windows_core::Result<u32>;
    fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComStaThreadPoolKnobs2 {}
impl IComStaThreadPoolKnobs2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComStaThreadPoolKnobs2_Vtbl
    where
        Identity: IComStaThreadPoolKnobs2_Impl,
    {
        unsafe extern "system" fn GetMaxCPULoad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwload: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs2_Impl::GetMaxCPULoad(this) {
                Ok(ok__) => {
                    pdwload.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCPULoad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwload: i32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs2_Impl::SetMaxCPULoad(this, core::mem::transmute_copy(&pdwload)).into()
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs2_Impl::GetCPUMetricEnabled(this) {
                Ok(ok__) => {
                    pbmetricenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs2_Impl::SetCPUMetricEnabled(this, core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs2_Impl::GetCreateThreadsAggressively(this) {
                Ok(ok__) => {
                    pbmetricenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs2_Impl::SetCreateThreadsAggressively(this, core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetMaxCSR<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcsr: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs2_Impl::GetMaxCSR(this) {
                Ok(ok__) => {
                    pdwcsr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCSR<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcsr: i32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs2_Impl::SetMaxCSR(this, core::mem::transmute_copy(&dwcsr)).into()
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComStaThreadPoolKnobs2_Impl::GetWaitTimeForThreadCleanup(this) {
                Ok(ok__) => {
                    pdwthreadcleanupwaittime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadcleanupwaittime: i32) -> windows_core::HRESULT
        where
            Identity: IComStaThreadPoolKnobs2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComStaThreadPoolKnobs2_Impl::SetWaitTimeForThreadCleanup(this, core::mem::transmute_copy(&dwthreadcleanupwaittime)).into()
        }
        Self {
            base__: IComStaThreadPoolKnobs_Vtbl::new::<Identity, OFFSET>(),
            GetMaxCPULoad: GetMaxCPULoad::<Identity, OFFSET>,
            SetMaxCPULoad: SetMaxCPULoad::<Identity, OFFSET>,
            GetCPUMetricEnabled: GetCPUMetricEnabled::<Identity, OFFSET>,
            SetCPUMetricEnabled: SetCPUMetricEnabled::<Identity, OFFSET>,
            GetCreateThreadsAggressively: GetCreateThreadsAggressively::<Identity, OFFSET>,
            SetCreateThreadsAggressively: SetCreateThreadsAggressively::<Identity, OFFSET>,
            GetMaxCSR: GetMaxCSR::<Identity, OFFSET>,
            SetMaxCSR: SetMaxCSR::<Identity, OFFSET>,
            GetWaitTimeForThreadCleanup: GetWaitTimeForThreadCleanup::<Identity, OFFSET>,
            SetWaitTimeForThreadCleanup: SetWaitTimeForThreadCleanup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs2 as windows_core::Interface>::IID || iid == &<IComStaThreadPoolKnobs as windows_core::Interface>::IID
    }
}
pub trait IComThreadEvents_Impl: Sized {
    fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::Result<()>;
    fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::Result<()>;
    fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> windows_core::Result<()>;
    fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> windows_core::Result<()>;
    fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()>;
    fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> windows_core::Result<()>;
    fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()>;
    fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> windows_core::Result<()>;
    fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::Result<()>;
    fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, aptid: u64) -> windows_core::Result<()>;
    fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComThreadEvents {}
impl IComThreadEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComThreadEvents_Vtbl
    where
        Identity: IComThreadEvents_Impl,
    {
        unsafe extern "system" fn OnThreadStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadTerminate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadBindToApartment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadBindToApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&aptid), core::mem::transmute_copy(&dwactcnt), core::mem::transmute_copy(&dwlowcnt)).into()
        }
        unsafe extern "system" fn OnThreadUnBind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadUnBind(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&aptid), core::mem::transmute_copy(&dwactcnt)).into()
        }
        unsafe extern "system" fn OnThreadWorkEnque<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadWorkEnque(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadWorkPrivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid)).into()
        }
        unsafe extern "system" fn OnThreadWorkPublic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadWorkPublic(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadWorkRedirect(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen), core::mem::transmute_copy(&threadnum)).into()
        }
        unsafe extern "system" fn OnThreadWorkReject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadWorkReject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadAssignApartment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, aptid: u64) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadAssignApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&aptid)).into()
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> windows_core::HRESULT
        where
            Identity: IComThreadEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComThreadEvents_Impl::OnThreadUnassignApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&aptid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnThreadStart: OnThreadStart::<Identity, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, OFFSET>,
            OnThreadBindToApartment: OnThreadBindToApartment::<Identity, OFFSET>,
            OnThreadUnBind: OnThreadUnBind::<Identity, OFFSET>,
            OnThreadWorkEnque: OnThreadWorkEnque::<Identity, OFFSET>,
            OnThreadWorkPrivate: OnThreadWorkPrivate::<Identity, OFFSET>,
            OnThreadWorkPublic: OnThreadWorkPublic::<Identity, OFFSET>,
            OnThreadWorkRedirect: OnThreadWorkRedirect::<Identity, OFFSET>,
            OnThreadWorkReject: OnThreadWorkReject::<Identity, OFFSET>,
            OnThreadAssignApartment: OnThreadAssignApartment::<Identity, OFFSET>,
            OnThreadUnassignApartment: OnThreadUnassignApartment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComThreadEvents as windows_core::Interface>::IID
    }
}
pub trait IComTrackingInfoCollection_Impl: Sized {
    fn Type(&self) -> windows_core::Result<TRACKING_COLL_TYPE>;
    fn Count(&self) -> windows_core::Result<u32>;
    fn Item(&self, ulindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComTrackingInfoCollection {}
impl IComTrackingInfoCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComTrackingInfoCollection_Vtbl
    where
        Identity: IComTrackingInfoCollection_Impl,
    {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> windows_core::HRESULT
        where
            Identity: IComTrackingInfoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComTrackingInfoCollection_Impl::Type(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComTrackingInfoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComTrackingInfoCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IComTrackingInfoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTrackingInfoCollection_Impl::Item(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoCollection as windows_core::Interface>::IID
    }
}
pub trait IComTrackingInfoEvents_Impl: Sized {
    fn OnNewTrackingInfo(&self, ptoplevelcollection: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComTrackingInfoEvents {}
impl IComTrackingInfoEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComTrackingInfoEvents_Vtbl
    where
        Identity: IComTrackingInfoEvents_Impl,
    {
        unsafe extern "system" fn OnNewTrackingInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoplevelcollection: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IComTrackingInfoEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTrackingInfoEvents_Impl::OnNewTrackingInfo(this, windows_core::from_raw_borrowed(&ptoplevelcollection)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNewTrackingInfo: OnNewTrackingInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoEvents as windows_core::Interface>::IID
    }
}
pub trait IComTrackingInfoObject_Impl: Sized {
    fn GetValue(&self, szpropertyname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for IComTrackingInfoObject {}
impl IComTrackingInfoObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComTrackingInfoObject_Vtbl
    where
        Identity: IComTrackingInfoObject_Impl,
    {
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szpropertyname: windows_core::PCWSTR, pvarout: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IComTrackingInfoObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComTrackingInfoObject_Impl::GetValue(this, core::mem::transmute(&szpropertyname)) {
                Ok(ok__) => {
                    pvarout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoObject as windows_core::Interface>::IID
    }
}
pub trait IComTrackingInfoProperties_Impl: Sized {
    fn PropCount(&self) -> windows_core::Result<u32>;
    fn GetPropName(&self, ulindex: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IComTrackingInfoProperties {}
impl IComTrackingInfoProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComTrackingInfoProperties_Vtbl
    where
        Identity: IComTrackingInfoProperties_Impl,
    {
        unsafe extern "system" fn PropCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IComTrackingInfoProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComTrackingInfoProperties_Impl::PropCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppszpropname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IComTrackingInfoProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IComTrackingInfoProperties_Impl::GetPropName(this, core::mem::transmute_copy(&ulindex)) {
                Ok(ok__) => {
                    ppszpropname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropCount: PropCount::<Identity, OFFSET>,
            GetPropName: GetPropName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoProperties as windows_core::Interface>::IID
    }
}
pub trait IComTransaction2Events_Impl: Sized {
    fn OnTransactionStart2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> windows_core::Result<()>;
    fn OnTransactionPrepare2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComTransaction2Events {}
impl IComTransaction2Events_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComTransaction2Events_Vtbl
    where
        Identity: IComTransaction2Events_Impl,
    {
        unsafe extern "system" fn OnTransactionStart2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> windows_core::HRESULT
        where
            Identity: IComTransaction2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTransaction2Events_Impl::OnTransactionStart2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&froot), core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComTransaction2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTransaction2Events_Impl::OnTransactionPrepare2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComTransaction2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTransaction2Events_Impl::OnTransactionAbort2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComTransaction2Events_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTransaction2Events_Impl::OnTransactionCommit2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransactionStart2: OnTransactionStart2::<Identity, OFFSET>,
            OnTransactionPrepare2: OnTransactionPrepare2::<Identity, OFFSET>,
            OnTransactionAbort2: OnTransactionAbort2::<Identity, OFFSET>,
            OnTransactionCommit2: OnTransactionCommit2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTransaction2Events as windows_core::Interface>::IID
    }
}
pub trait IComTransactionEvents_Impl: Sized {
    fn OnTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComTransactionEvents {}
impl IComTransactionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComTransactionEvents_Vtbl
    where
        Identity: IComTransactionEvents_Impl,
    {
        unsafe extern "system" fn OnTransactionStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComTransactionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTransactionEvents_Impl::OnTransactionStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&froot)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IComTransactionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTransactionEvents_Impl::OnTransactionPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComTransactionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTransactionEvents_Impl::OnTransactionAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IComTransactionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComTransactionEvents_Impl::OnTransactionCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransactionStart: OnTransactionStart::<Identity, OFFSET>,
            OnTransactionPrepare: OnTransactionPrepare::<Identity, OFFSET>,
            OnTransactionAbort: OnTransactionAbort::<Identity, OFFSET>,
            OnTransactionCommit: OnTransactionCommit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComTransactionEvents as windows_core::Interface>::IID
    }
}
pub trait IComUserEvent_Impl: Sized {
    fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IComUserEvent {}
impl IComUserEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IComUserEvent_Vtbl
    where
        Identity: IComUserEvent_Impl,
    {
        unsafe extern "system" fn OnUserEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IComUserEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IComUserEvent_Impl::OnUserEvent(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pvarevent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUserEvent: OnUserEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComUserEvent as windows_core::Interface>::IID
    }
}
pub trait IContextProperties_Impl: Sized {
    fn Count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, name: &windows_core::BSTR, pproperty: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn EnumNames(&self) -> windows_core::Result<IEnumNames>;
    fn SetProperty(&self, name: &windows_core::BSTR, property: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn RemoveProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContextProperties {}
impl IContextProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContextProperties_Vtbl
    where
        Identity: IContextProperties_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IContextProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextProperties_Impl::Count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pproperty: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IContextProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextProperties_Impl::GetProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContextProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContextProperties_Impl::EnumNames(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, property: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IContextProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextProperties_Impl::SetProperty(this, core::mem::transmute(&name), core::mem::transmute(&property)).into()
        }
        unsafe extern "system" fn RemoveProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IContextProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextProperties_Impl::RemoveProperty(this, core::mem::transmute(&name)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            EnumNames: EnumNames::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextProperties as windows_core::Interface>::IID
    }
}
pub trait IContextSecurityPerimeter_Impl: Sized {
    fn GetPerimeterFlag(&self, pflag: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetPerimeterFlag(&self, fflag: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContextSecurityPerimeter {}
impl IContextSecurityPerimeter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContextSecurityPerimeter_Vtbl
    where
        Identity: IContextSecurityPerimeter_Impl,
    {
        unsafe extern "system" fn GetPerimeterFlag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContextSecurityPerimeter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextSecurityPerimeter_Impl::GetPerimeterFlag(this, core::mem::transmute_copy(&pflag)).into()
        }
        unsafe extern "system" fn SetPerimeterFlag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContextSecurityPerimeter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextSecurityPerimeter_Impl::SetPerimeterFlag(this, core::mem::transmute_copy(&fflag)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPerimeterFlag: GetPerimeterFlag::<Identity, OFFSET>,
            SetPerimeterFlag: SetPerimeterFlag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextSecurityPerimeter as windows_core::Interface>::IID
    }
}
pub trait IContextState_Impl: Sized {
    fn SetDeactivateOnReturn(&self, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetDeactivateOnReturn(&self, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetMyTransactionVote(&self, txvote: TransactionVote) -> windows_core::Result<()>;
    fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContextState {}
impl IContextState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContextState_Vtbl
    where
        Identity: IContextState_Impl,
    {
        unsafe extern "system" fn SetDeactivateOnReturn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IContextState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextState_Impl::SetDeactivateOnReturn(this, core::mem::transmute_copy(&bdeactivate)).into()
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IContextState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextState_Impl::GetDeactivateOnReturn(this, core::mem::transmute_copy(&pbdeactivate)).into()
        }
        unsafe extern "system" fn SetMyTransactionVote<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, txvote: TransactionVote) -> windows_core::HRESULT
        where
            Identity: IContextState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextState_Impl::SetMyTransactionVote(this, core::mem::transmute_copy(&txvote)).into()
        }
        unsafe extern "system" fn GetMyTransactionVote<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxvote: *mut TransactionVote) -> windows_core::HRESULT
        where
            Identity: IContextState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContextState_Impl::GetMyTransactionVote(this, core::mem::transmute_copy(&ptxvote)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDeactivateOnReturn: SetDeactivateOnReturn::<Identity, OFFSET>,
            GetDeactivateOnReturn: GetDeactivateOnReturn::<Identity, OFFSET>,
            SetMyTransactionVote: SetMyTransactionVote::<Identity, OFFSET>,
            GetMyTransactionVote: GetMyTransactionVote::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextState as windows_core::Interface>::IID
    }
}
pub trait ICreateWithLocalTransaction_Impl: Sized {
    fn CreateInstanceWithSysTx(&self, ptransaction: Option<&windows_core::IUnknown>, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICreateWithLocalTransaction {}
impl ICreateWithLocalTransaction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateWithLocalTransaction_Vtbl
    where
        Identity: ICreateWithLocalTransaction_Impl,
    {
        unsafe extern "system" fn CreateInstanceWithSysTx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateWithLocalTransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateWithLocalTransaction_Impl::CreateInstanceWithSysTx(this, windows_core::from_raw_borrowed(&ptransaction), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstanceWithSysTx: CreateInstanceWithSysTx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateWithLocalTransaction as windows_core::Interface>::IID
    }
}
pub trait ICreateWithTipTransactionEx_Impl: Sized {
    fn CreateInstance(&self, bstrtipurl: &windows_core::BSTR, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICreateWithTipTransactionEx {}
impl ICreateWithTipTransactionEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateWithTipTransactionEx_Vtbl
    where
        Identity: ICreateWithTipTransactionEx_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtipurl: core::mem::MaybeUninit<windows_core::BSTR>, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateWithTipTransactionEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateWithTipTransactionEx_Impl::CreateInstance(this, core::mem::transmute(&bstrtipurl), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateWithTipTransactionEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ICreateWithTransactionEx_Impl: Sized {
    fn CreateInstance(&self, ptransaction: Option<&super::DistributedTransactionCoordinator::ITransaction>, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for ICreateWithTransactionEx {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ICreateWithTransactionEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateWithTransactionEx_Vtbl
    where
        Identity: ICreateWithTransactionEx_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateWithTransactionEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateWithTransactionEx_Impl::CreateInstance(this, windows_core::from_raw_borrowed(&ptransaction), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateWithTransactionEx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmCompensator_Impl: Sized {
    fn SetLogControl(&self, plogcontrol: Option<&ICrmLogControl>) -> windows_core::Result<()>;
    fn BeginPrepare(&self) -> windows_core::Result<()>;
    fn PrepareRecord(&self, crmlogrec: &CrmLogRecordRead) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn EndPrepare(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn BeginCommit(&self, frecovery: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CommitRecord(&self, crmlogrec: &CrmLogRecordRead) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn EndCommit(&self) -> windows_core::Result<()>;
    fn BeginAbort(&self, frecovery: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn AbortRecord(&self, crmlogrec: &CrmLogRecordRead) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn EndAbort(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICrmCompensator {}
#[cfg(feature = "Win32_System_Com")]
impl ICrmCompensator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICrmCompensator_Vtbl
    where
        Identity: ICrmCompensator_Impl,
    {
        unsafe extern "system" fn SetLogControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensator_Impl::SetLogControl(this, windows_core::from_raw_borrowed(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensator_Impl::BeginPrepare(this).into()
        }
        unsafe extern "system" fn PrepareRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmCompensator_Impl::PrepareRecord(this, core::mem::transmute(&crmlogrec)) {
                Ok(ok__) => {
                    pfforget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmCompensator_Impl::EndPrepare(this) {
                Ok(ok__) => {
                    pfoktoprepare.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensator_Impl::BeginCommit(this, core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn CommitRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmCompensator_Impl::CommitRecord(this, core::mem::transmute(&crmlogrec)) {
                Ok(ok__) => {
                    pfforget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensator_Impl::EndCommit(this).into()
        }
        unsafe extern "system" fn BeginAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensator_Impl::BeginAbort(this, core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn AbortRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmCompensator_Impl::AbortRecord(this, core::mem::transmute(&crmlogrec)) {
                Ok(ok__) => {
                    pfforget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmCompensator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensator_Impl::EndAbort(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLogControl: SetLogControl::<Identity, OFFSET>,
            BeginPrepare: BeginPrepare::<Identity, OFFSET>,
            PrepareRecord: PrepareRecord::<Identity, OFFSET>,
            EndPrepare: EndPrepare::<Identity, OFFSET>,
            BeginCommit: BeginCommit::<Identity, OFFSET>,
            CommitRecord: CommitRecord::<Identity, OFFSET>,
            EndCommit: EndCommit::<Identity, OFFSET>,
            BeginAbort: BeginAbort::<Identity, OFFSET>,
            AbortRecord: AbortRecord::<Identity, OFFSET>,
            EndAbort: EndAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmCompensator as windows_core::Interface>::IID
    }
}
pub trait ICrmCompensatorVariants_Impl: Sized {
    fn SetLogControlVariants(&self, plogcontrol: Option<&ICrmLogControl>) -> windows_core::Result<()>;
    fn BeginPrepareVariants(&self) -> windows_core::Result<()>;
    fn PrepareRecordVariants(&self, plogrecord: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndPrepareVariants(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BeginCommitVariants(&self, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CommitRecordVariants(&self, plogrecord: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndCommitVariants(&self) -> windows_core::Result<()>;
    fn BeginAbortVariants(&self, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AbortRecordVariants(&self, plogrecord: *const windows_core::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndAbortVariants(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICrmCompensatorVariants {}
impl ICrmCompensatorVariants_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICrmCompensatorVariants_Vtbl
    where
        Identity: ICrmCompensatorVariants_Impl,
    {
        unsafe extern "system" fn SetLogControlVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensatorVariants_Impl::SetLogControlVariants(this, windows_core::from_raw_borrowed(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepareVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensatorVariants_Impl::BeginPrepareVariants(this).into()
        }
        unsafe extern "system" fn PrepareRecordVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const core::mem::MaybeUninit<windows_core::VARIANT>, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmCompensatorVariants_Impl::PrepareRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                Ok(ok__) => {
                    pbforget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepareVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboktoprepare: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmCompensatorVariants_Impl::EndPrepareVariants(this) {
                Ok(ok__) => {
                    pboktoprepare.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommitVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensatorVariants_Impl::BeginCommitVariants(this, core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn CommitRecordVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const core::mem::MaybeUninit<windows_core::VARIANT>, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmCompensatorVariants_Impl::CommitRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                Ok(ok__) => {
                    pbforget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommitVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensatorVariants_Impl::EndCommitVariants(this).into()
        }
        unsafe extern "system" fn BeginAbortVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensatorVariants_Impl::BeginAbortVariants(this, core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn AbortRecordVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const core::mem::MaybeUninit<windows_core::VARIANT>, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmCompensatorVariants_Impl::AbortRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                Ok(ok__) => {
                    pbforget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbortVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmCompensatorVariants_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmCompensatorVariants_Impl::EndAbortVariants(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLogControlVariants: SetLogControlVariants::<Identity, OFFSET>,
            BeginPrepareVariants: BeginPrepareVariants::<Identity, OFFSET>,
            PrepareRecordVariants: PrepareRecordVariants::<Identity, OFFSET>,
            EndPrepareVariants: EndPrepareVariants::<Identity, OFFSET>,
            BeginCommitVariants: BeginCommitVariants::<Identity, OFFSET>,
            CommitRecordVariants: CommitRecordVariants::<Identity, OFFSET>,
            EndCommitVariants: EndCommitVariants::<Identity, OFFSET>,
            BeginAbortVariants: BeginAbortVariants::<Identity, OFFSET>,
            AbortRecordVariants: AbortRecordVariants::<Identity, OFFSET>,
            EndAbortVariants: EndAbortVariants::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmCompensatorVariants as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmFormatLogRecords_Impl: Sized {
    fn GetColumnCount(&self) -> windows_core::Result<i32>;
    fn GetColumnHeaders(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn GetColumn(&self, crmlogrec: &CrmLogRecordRead) -> windows_core::Result<windows_core::VARIANT>;
    fn GetColumnVariants(&self, logrecord: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICrmFormatLogRecords {}
#[cfg(feature = "Win32_System_Com")]
impl ICrmFormatLogRecords_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICrmFormatLogRecords_Vtbl
    where
        Identity: ICrmFormatLogRecords_Impl,
    {
        unsafe extern "system" fn GetColumnCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcolumncount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICrmFormatLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmFormatLogRecords_Impl::GetColumnCount(this) {
                Ok(ok__) => {
                    plcolumncount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheaders: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmFormatLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmFormatLogRecords_Impl::GetColumnHeaders(this) {
                Ok(ok__) => {
                    pheaders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmFormatLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmFormatLogRecords_Impl::GetColumn(this, core::mem::transmute(&crmlogrec)) {
                Ok(ok__) => {
                    pformattedlogrecord.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecord: core::mem::MaybeUninit<windows_core::VARIANT>, pformattedlogrecord: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmFormatLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmFormatLogRecords_Impl::GetColumnVariants(this, core::mem::transmute(&logrecord)) {
                Ok(ok__) => {
                    pformattedlogrecord.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumnCount: GetColumnCount::<Identity, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, OFFSET>,
            GetColumn: GetColumn::<Identity, OFFSET>,
            GetColumnVariants: GetColumnVariants::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmFormatLogRecords as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmLogControl_Impl: Sized {
    fn TransactionUOW(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RegisterCompensator(&self, lpcwstrprogidcompensator: &windows_core::PCWSTR, lpcwstrdescription: &windows_core::PCWSTR, lcrmregflags: i32) -> windows_core::Result<()>;
    fn WriteLogRecordVariants(&self, plogrecord: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn ForceLog(&self) -> windows_core::Result<()>;
    fn ForgetLogRecord(&self) -> windows_core::Result<()>;
    fn ForceTransactionToAbort(&self) -> windows_core::Result<()>;
    fn WriteLogRecord(&self, rgblob: *const super::Com::BLOB, cblob: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICrmLogControl {}
#[cfg(feature = "Win32_System_Com")]
impl ICrmLogControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICrmLogControl_Vtbl
    where
        Identity: ICrmLogControl_Impl,
    {
        unsafe extern "system" fn TransactionUOW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ICrmLogControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmLogControl_Impl::TransactionUOW(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCompensator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcwstrprogidcompensator: windows_core::PCWSTR, lpcwstrdescription: windows_core::PCWSTR, lcrmregflags: i32) -> windows_core::HRESULT
        where
            Identity: ICrmLogControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmLogControl_Impl::RegisterCompensator(this, core::mem::transmute(&lpcwstrprogidcompensator), core::mem::transmute(&lpcwstrdescription), core::mem::transmute_copy(&lcrmregflags)).into()
        }
        unsafe extern "system" fn WriteLogRecordVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmLogControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmLogControl_Impl::WriteLogRecordVariants(this, core::mem::transmute_copy(&plogrecord)).into()
        }
        unsafe extern "system" fn ForceLog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmLogControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmLogControl_Impl::ForceLog(this).into()
        }
        unsafe extern "system" fn ForgetLogRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmLogControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmLogControl_Impl::ForgetLogRecord(this).into()
        }
        unsafe extern "system" fn ForceTransactionToAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmLogControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmLogControl_Impl::ForceTransactionToAbort(this).into()
        }
        unsafe extern "system" fn WriteLogRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> windows_core::HRESULT
        where
            Identity: ICrmLogControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmLogControl_Impl::WriteLogRecord(this, core::mem::transmute_copy(&rgblob), core::mem::transmute_copy(&cblob)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransactionUOW: TransactionUOW::<Identity, OFFSET>,
            RegisterCompensator: RegisterCompensator::<Identity, OFFSET>,
            WriteLogRecordVariants: WriteLogRecordVariants::<Identity, OFFSET>,
            ForceLog: ForceLog::<Identity, OFFSET>,
            ForgetLogRecord: ForgetLogRecord::<Identity, OFFSET>,
            ForceTransactionToAbort: ForceTransactionToAbort::<Identity, OFFSET>,
            WriteLogRecord: WriteLogRecord::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmLogControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmMonitor_Impl: Sized {
    fn GetClerks(&self) -> windows_core::Result<ICrmMonitorClerks>;
    fn HoldClerk(&self, index: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICrmMonitor {}
#[cfg(feature = "Win32_System_Com")]
impl ICrmMonitor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICrmMonitor_Vtbl
    where
        Identity: ICrmMonitor_Impl,
    {
        unsafe extern "system" fn GetClerks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclerks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitor_Impl::GetClerks(this) {
                Ok(ok__) => {
                    pclerks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldClerk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitor_Impl::HoldClerk(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClerks: GetClerks::<Identity, OFFSET>,
            HoldClerk: HoldClerk::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmMonitor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmMonitorClerks_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ProgIdCompensator(&self, index: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn Description(&self, index: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn TransactionUOW(&self, index: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn ActivityId(&self, index: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICrmMonitorClerks {}
#[cfg(feature = "Win32_System_Com")]
impl ICrmMonitorClerks_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICrmMonitorClerks_Vtbl
    where
        Identity: ICrmMonitorClerks_Impl,
    {
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorClerks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorClerks_Impl::Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorClerks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorClerks_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorClerks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorClerks_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgIdCompensator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorClerks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorClerks_Impl::ProgIdCompensator(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorClerks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorClerks_Impl::Description(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionUOW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorClerks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorClerks_Impl::TransactionUOW(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorClerks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorClerks_Impl::ActivityId(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ProgIdCompensator: ProgIdCompensator::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            TransactionUOW: TransactionUOW::<Identity, OFFSET>,
            ActivityId: ActivityId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmMonitorClerks as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmMonitorLogRecords_Impl: Sized {
    fn Count(&self) -> windows_core::Result<i32>;
    fn TransactionState(&self) -> windows_core::Result<CrmTransactionState>;
    fn StructuredRecords(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> windows_core::Result<()>;
    fn GetLogRecordVariants(&self, indexnumber: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICrmMonitorLogRecords {}
#[cfg(feature = "Win32_System_Com")]
impl ICrmMonitorLogRecords_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICrmMonitorLogRecords_Vtbl
    where
        Identity: ICrmMonitorLogRecords_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorLogRecords_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut CrmTransactionState) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorLogRecords_Impl::TransactionState(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StructuredRecords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorLogRecords_Impl::StructuredRecords(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICrmMonitorLogRecords_Impl::GetLogRecord(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcrmlogrec)).into()
        }
        unsafe extern "system" fn GetLogRecordVariants<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexnumber: core::mem::MaybeUninit<windows_core::VARIANT>, plogrecord: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICrmMonitorLogRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICrmMonitorLogRecords_Impl::GetLogRecordVariants(this, core::mem::transmute(&indexnumber)) {
                Ok(ok__) => {
                    plogrecord.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            TransactionState: TransactionState::<Identity, OFFSET>,
            StructuredRecords: StructuredRecords::<Identity, OFFSET>,
            GetLogRecord: GetLogRecord::<Identity, OFFSET>,
            GetLogRecordVariants: GetLogRecordVariants::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrmMonitorLogRecords as windows_core::Interface>::IID
    }
}
pub trait IDispenserDriver_Impl: Sized {
    fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> windows_core::Result<()>;
    fn RateResource(&self, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> windows_core::Result<()>;
    fn EnlistResource(&self, resid: usize, transid: usize) -> windows_core::Result<()>;
    fn ResetResource(&self, resid: usize) -> windows_core::Result<()>;
    fn DestroyResource(&self, resid: usize) -> windows_core::Result<()>;
    fn DestroyResourceS(&self, resid: *mut u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDispenserDriver {}
impl IDispenserDriver_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDispenserDriver_Vtbl
    where
        Identity: IDispenserDriver_Impl,
    {
        unsafe extern "system" fn CreateResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDispenserDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispenserDriver_Impl::CreateResource(this, core::mem::transmute_copy(&restypid), core::mem::transmute_copy(&presid), core::mem::transmute_copy(&psecsfreebeforedestroy)).into()
        }
        unsafe extern "system" fn RateResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDispenserDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispenserDriver_Impl::RateResource(this, core::mem::transmute_copy(&restypid), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&frequirestransactionenlistment), core::mem::transmute_copy(&prating)).into()
        }
        unsafe extern "system" fn EnlistResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize, transid: usize) -> windows_core::HRESULT
        where
            Identity: IDispenserDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispenserDriver_Impl::EnlistResource(this, core::mem::transmute_copy(&resid), core::mem::transmute_copy(&transid)).into()
        }
        unsafe extern "system" fn ResetResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize) -> windows_core::HRESULT
        where
            Identity: IDispenserDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispenserDriver_Impl::ResetResource(this, core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize) -> windows_core::HRESULT
        where
            Identity: IDispenserDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispenserDriver_Impl::DestroyResource(this, core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResourceS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: *mut u16) -> windows_core::HRESULT
        where
            Identity: IDispenserDriver_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispenserDriver_Impl::DestroyResourceS(this, core::mem::transmute_copy(&resid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateResource: CreateResource::<Identity, OFFSET>,
            RateResource: RateResource::<Identity, OFFSET>,
            EnlistResource: EnlistResource::<Identity, OFFSET>,
            ResetResource: ResetResource::<Identity, OFFSET>,
            DestroyResource: DestroyResource::<Identity, OFFSET>,
            DestroyResourceS: DestroyResourceS::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispenserDriver as windows_core::Interface>::IID
    }
}
pub trait IDispenserManager_Impl: Sized {
    fn RegisterDispenser(&self, __midl__idispensermanager0000: Option<&IDispenserDriver>, szdispensername: &windows_core::PCWSTR) -> windows_core::Result<IHolder>;
    fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDispenserManager {}
impl IDispenserManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDispenserManager_Vtbl
    where
        Identity: IDispenserManager_Impl,
    {
        unsafe extern "system" fn RegisterDispenser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__idispensermanager0000: *mut core::ffi::c_void, szdispensername: windows_core::PCWSTR, __midl__idispensermanager0001: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDispenserManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDispenserManager_Impl::RegisterDispenser(this, windows_core::from_raw_borrowed(&__midl__idispensermanager0000), core::mem::transmute(&szdispensername)) {
                Ok(ok__) => {
                    __midl__idispensermanager0001.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> windows_core::HRESULT
        where
            Identity: IDispenserManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDispenserManager_Impl::GetContext(this, core::mem::transmute_copy(&__midl__idispensermanager0002), core::mem::transmute_copy(&__midl__idispensermanager0003)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDispenser: RegisterDispenser::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispenserManager as windows_core::Interface>::IID
    }
}
pub trait IEnumNames_Impl: Sized {
    fn Next(&self, celt: u32, rgname: *mut windows_core::BSTR, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNames>;
}
impl windows_core::RuntimeName for IEnumNames {}
impl IEnumNames_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNames_Vtbl
    where
        Identity: IEnumNames_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNames_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgname), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNames_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNames_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumNames_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNames as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEventServerTrace_Impl: Sized + super::Com::IDispatch_Impl {
    fn StartTraceGuid(&self, bstrguidevent: &windows_core::BSTR, bstrguidfilter: &windows_core::BSTR, lpidfilter: i32) -> windows_core::Result<()>;
    fn StopTraceGuid(&self, bstrguidevent: &windows_core::BSTR, bstrguidfilter: &windows_core::BSTR, lpidfilter: i32) -> windows_core::Result<()>;
    fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEventServerTrace {}
#[cfg(feature = "Win32_System_Com")]
impl IEventServerTrace_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventServerTrace_Vtbl
    where
        Identity: IEventServerTrace_Impl,
    {
        unsafe extern "system" fn StartTraceGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidevent: core::mem::MaybeUninit<windows_core::BSTR>, bstrguidfilter: core::mem::MaybeUninit<windows_core::BSTR>, lpidfilter: i32) -> windows_core::HRESULT
        where
            Identity: IEventServerTrace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventServerTrace_Impl::StartTraceGuid(this, core::mem::transmute(&bstrguidevent), core::mem::transmute(&bstrguidfilter), core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn StopTraceGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidevent: core::mem::MaybeUninit<windows_core::BSTR>, bstrguidfilter: core::mem::MaybeUninit<windows_core::BSTR>, lpidfilter: i32) -> windows_core::HRESULT
        where
            Identity: IEventServerTrace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventServerTrace_Impl::StopTraceGuid(this, core::mem::transmute(&bstrguidevent), core::mem::transmute(&bstrguidfilter), core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn EnumTraceGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventServerTrace_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventServerTrace_Impl::EnumTraceGuid(this, core::mem::transmute_copy(&plcntguids), core::mem::transmute_copy(&pbstrguidlist)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartTraceGuid: StartTraceGuid::<Identity, OFFSET>,
            StopTraceGuid: StopTraceGuid::<Identity, OFFSET>,
            EnumTraceGuid: EnumTraceGuid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventServerTrace as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IGetAppTrackerData_Impl: Sized {
    fn GetApplicationProcesses(&self, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> windows_core::Result<()>;
    fn GetApplicationProcessDetails(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetApplicationsInProcess(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> windows_core::Result<()>;
    fn GetComponentsInProcess(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> windows_core::Result<()>;
    fn GetComponentDetails(&self, applicationinstanceid: *const windows_core::GUID, processid: u32, clsid: *const windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> windows_core::Result<()>;
    fn GetTrackerDataAsCollectionObject(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetSuggestedPollingInterval(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IGetAppTrackerData {}
impl IGetAppTrackerData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetAppTrackerData_Vtbl
    where
        Identity: IGetAppTrackerData_Impl,
    {
        unsafe extern "system" fn GetApplicationProcesses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> windows_core::HRESULT
        where
            Identity: IGetAppTrackerData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetAppTrackerData_Impl::GetApplicationProcesses(this, core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&applicationid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numapplicationprocesses), core::mem::transmute_copy(&applicationprocesses)).into()
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IGetAppTrackerData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetAppTrackerData_Impl::GetApplicationProcessDetails(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&summary), core::mem::transmute_copy(&statistics), core::mem::transmute_copy(&recycleinfo), core::mem::transmute_copy(&anycomponentshangmonitored)).into()
        }
        unsafe extern "system" fn GetApplicationsInProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> windows_core::HRESULT
        where
            Identity: IGetAppTrackerData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetAppTrackerData_Impl::GetApplicationsInProcess(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numapplicationsinprocess), core::mem::transmute_copy(&applications)).into()
        }
        unsafe extern "system" fn GetComponentsInProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> windows_core::HRESULT
        where
            Identity: IGetAppTrackerData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetAppTrackerData_Impl::GetComponentsInProcess(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&applicationid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numcomponentsinprocess), core::mem::transmute_copy(&components)).into()
        }
        unsafe extern "system" fn GetComponentDetails<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, clsid: *const windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> windows_core::HRESULT
        where
            Identity: IGetAppTrackerData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetAppTrackerData_Impl::GetComponentDetails(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&summary), core::mem::transmute_copy(&statistics), core::mem::transmute_copy(&hangmonitorinfo)).into()
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, toplevelcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetAppTrackerData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetAppTrackerData_Impl::GetTrackerDataAsCollectionObject(this) {
                Ok(ok__) => {
                    toplevelcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pollingintervalinseconds: *mut u32) -> windows_core::HRESULT
        where
            Identity: IGetAppTrackerData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetAppTrackerData_Impl::GetSuggestedPollingInterval(this) {
                Ok(ok__) => {
                    pollingintervalinseconds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetApplicationProcesses: GetApplicationProcesses::<Identity, OFFSET>,
            GetApplicationProcessDetails: GetApplicationProcessDetails::<Identity, OFFSET>,
            GetApplicationsInProcess: GetApplicationsInProcess::<Identity, OFFSET>,
            GetComponentsInProcess: GetComponentsInProcess::<Identity, OFFSET>,
            GetComponentDetails: GetComponentDetails::<Identity, OFFSET>,
            GetTrackerDataAsCollectionObject: GetTrackerDataAsCollectionObject::<Identity, OFFSET>,
            GetSuggestedPollingInterval: GetSuggestedPollingInterval::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetAppTrackerData as windows_core::Interface>::IID
    }
}
pub trait IGetContextProperties_Impl: Sized {
    fn Count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn GetProperty(&self, name: &windows_core::BSTR, pproperty: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn EnumNames(&self) -> windows_core::Result<IEnumNames>;
}
impl windows_core::RuntimeName for IGetContextProperties {}
impl IGetContextProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetContextProperties_Vtbl
    where
        Identity: IGetContextProperties_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IGetContextProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetContextProperties_Impl::Count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pproperty: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IGetContextProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetContextProperties_Impl::GetProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetContextProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetContextProperties_Impl::EnumNames(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            EnumNames: EnumNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetContextProperties as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGetSecurityCallContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetSecurityCallContext(&self) -> windows_core::Result<ISecurityCallContext>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IGetSecurityCallContext {}
#[cfg(feature = "Win32_System_Com")]
impl IGetSecurityCallContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetSecurityCallContext_Vtbl
    where
        Identity: IGetSecurityCallContext_Impl,
    {
        unsafe extern "system" fn GetSecurityCallContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetSecurityCallContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetSecurityCallContext_Impl::GetSecurityCallContext(this) {
                Ok(ok__) => {
                    ppobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetSecurityCallContext: GetSecurityCallContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetSecurityCallContext as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IHolder_Impl: Sized {
    fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> windows_core::Result<()>;
    fn FreeResource(&self, __midl__iholder0002: usize) -> windows_core::Result<()>;
    fn TrackResource(&self, __midl__iholder0003: usize) -> windows_core::Result<()>;
    fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> windows_core::Result<()>;
    fn UntrackResource(&self, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UntrackResourceS(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHolder {}
impl IHolder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHolder_Vtbl
    where
        Identity: IHolder_Impl,
    {
        unsafe extern "system" fn AllocResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> windows_core::HRESULT
        where
            Identity: IHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolder_Impl::AllocResource(this, core::mem::transmute_copy(&__midl__iholder0000), core::mem::transmute_copy(&__midl__iholder0001)).into()
        }
        unsafe extern "system" fn FreeResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0002: usize) -> windows_core::HRESULT
        where
            Identity: IHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolder_Impl::FreeResource(this, core::mem::transmute_copy(&__midl__iholder0002)).into()
        }
        unsafe extern "system" fn TrackResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0003: usize) -> windows_core::HRESULT
        where
            Identity: IHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolder_Impl::TrackResource(this, core::mem::transmute_copy(&__midl__iholder0003)).into()
        }
        unsafe extern "system" fn TrackResourceS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0004: *mut u16) -> windows_core::HRESULT
        where
            Identity: IHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolder_Impl::TrackResourceS(this, core::mem::transmute_copy(&__midl__iholder0004)).into()
        }
        unsafe extern "system" fn UntrackResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolder_Impl::UntrackResource(this, core::mem::transmute_copy(&__midl__iholder0005), core::mem::transmute_copy(&__midl__iholder0006)).into()
        }
        unsafe extern "system" fn UntrackResourceS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolder_Impl::UntrackResourceS(this, core::mem::transmute_copy(&__midl__iholder0007), core::mem::transmute_copy(&__midl__iholder0008)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolder_Impl::Close(this).into()
        }
        unsafe extern "system" fn RequestDestroyResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0009: usize) -> windows_core::HRESULT
        where
            Identity: IHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHolder_Impl::RequestDestroyResource(this, core::mem::transmute_copy(&__midl__iholder0009)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocResource: AllocResource::<Identity, OFFSET>,
            FreeResource: FreeResource::<Identity, OFFSET>,
            TrackResource: TrackResource::<Identity, OFFSET>,
            TrackResourceS: TrackResourceS::<Identity, OFFSET>,
            UntrackResource: UntrackResource::<Identity, OFFSET>,
            UntrackResourceS: UntrackResourceS::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            RequestDestroyResource: RequestDestroyResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolder as windows_core::Interface>::IID
    }
}
pub trait ILBEvents_Impl: Sized {
    fn TargetUp(&self, bstrservername: &windows_core::BSTR, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TargetDown(&self, bstrservername: &windows_core::BSTR, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EngineDefined(&self, bstrpropname: &windows_core::BSTR, varpropvalue: *const windows_core::VARIANT, bstrclsideng: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILBEvents {}
impl ILBEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILBEvents_Vtbl
    where
        Identity: ILBEvents_Impl,
    {
        unsafe extern "system" fn TargetUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsideng: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILBEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILBEvents_Impl::TargetUp(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrclsideng)).into()
        }
        unsafe extern "system" fn TargetDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsideng: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILBEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILBEvents_Impl::TargetDown(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrclsideng)).into()
        }
        unsafe extern "system" fn EngineDefined<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, varpropvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrclsideng: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ILBEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILBEvents_Impl::EngineDefined(this, core::mem::transmute(&bstrpropname), core::mem::transmute_copy(&varpropvalue), core::mem::transmute(&bstrclsideng)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TargetUp: TargetUp::<Identity, OFFSET>,
            TargetDown: TargetDown::<Identity, OFFSET>,
            EngineDefined: EngineDefined::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILBEvents as windows_core::Interface>::IID
    }
}
pub trait IMTSActivity_Impl: Sized {
    fn SynchronousCall(&self, pcall: Option<&IMTSCall>) -> windows_core::Result<()>;
    fn AsyncCall(&self, pcall: Option<&IMTSCall>) -> windows_core::Result<()>;
    fn Reserved1(&self);
    fn BindToCurrentThread(&self) -> windows_core::Result<()>;
    fn UnbindFromThread(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMTSActivity {}
impl IMTSActivity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMTSActivity_Vtbl
    where
        Identity: IMTSActivity_Impl,
    {
        unsafe extern "system" fn SynchronousCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMTSActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMTSActivity_Impl::SynchronousCall(this, windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn AsyncCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMTSActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMTSActivity_Impl::AsyncCall(this, windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn Reserved1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IMTSActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMTSActivity_Impl::Reserved1(this)
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMTSActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMTSActivity_Impl::BindToCurrentThread(this).into()
        }
        unsafe extern "system" fn UnbindFromThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMTSActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMTSActivity_Impl::UnbindFromThread(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, OFFSET>,
            AsyncCall: AsyncCall::<Identity, OFFSET>,
            Reserved1: Reserved1::<Identity, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMTSActivity as windows_core::Interface>::IID
    }
}
pub trait IMTSCall_Impl: Sized {
    fn OnCall(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMTSCall {}
impl IMTSCall_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMTSCall_Vtbl
    where
        Identity: IMTSCall_Impl,
    {
        unsafe extern "system" fn OnCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMTSCall_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMTSCall_Impl::OnCall(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMTSCall as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMTSLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetEventDispatcher(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMTSLocator {}
#[cfg(feature = "Win32_System_Com")]
impl IMTSLocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMTSLocator_Vtbl
    where
        Identity: IMTSLocator_Impl,
    {
        unsafe extern "system" fn GetEventDispatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMTSLocator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMTSLocator_Impl::GetEventDispatcher(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetEventDispatcher: GetEventDispatcher::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMTSLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IManagedActivationEvents_Impl: Sized {
    fn CreateManagedStub(&self, pinfo: Option<&IManagedObjectInfo>, fdist: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DestroyManagedStub(&self, pinfo: Option<&IManagedObjectInfo>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IManagedActivationEvents {}
impl IManagedActivationEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IManagedActivationEvents_Vtbl
    where
        Identity: IManagedActivationEvents_Impl,
    {
        unsafe extern "system" fn CreateManagedStub<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void, fdist: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IManagedActivationEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManagedActivationEvents_Impl::CreateManagedStub(this, windows_core::from_raw_borrowed(&pinfo), core::mem::transmute_copy(&fdist)).into()
        }
        unsafe extern "system" fn DestroyManagedStub<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IManagedActivationEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManagedActivationEvents_Impl::DestroyManagedStub(this, windows_core::from_raw_borrowed(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateManagedStub: CreateManagedStub::<Identity, OFFSET>,
            DestroyManagedStub: DestroyManagedStub::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedActivationEvents as windows_core::Interface>::IID
    }
}
pub trait IManagedObjectInfo_Impl: Sized {
    fn GetIUnknown(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetIObjectControl(&self) -> windows_core::Result<IObjectControl>;
    fn SetInPool(&self, binpool: super::super::Foundation::BOOL, ppooledobj: Option<&IManagedPooledObj>) -> windows_core::Result<()>;
    fn SetWrapperStrength(&self, bstrong: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IManagedObjectInfo {}
impl IManagedObjectInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IManagedObjectInfo_Vtbl
    where
        Identity: IManagedObjectInfo_Impl,
    {
        unsafe extern "system" fn GetIUnknown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IManagedObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManagedObjectInfo_Impl::GetIUnknown(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIObjectControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IManagedObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManagedObjectInfo_Impl::GetIObjectControl(this) {
                Ok(ok__) => {
                    pctrl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IManagedObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManagedObjectInfo_Impl::SetInPool(this, core::mem::transmute_copy(&binpool), windows_core::from_raw_borrowed(&ppooledobj)).into()
        }
        unsafe extern "system" fn SetWrapperStrength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IManagedObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManagedObjectInfo_Impl::SetWrapperStrength(this, core::mem::transmute_copy(&bstrong)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIUnknown: GetIUnknown::<Identity, OFFSET>,
            GetIObjectControl: GetIObjectControl::<Identity, OFFSET>,
            SetInPool: SetInPool::<Identity, OFFSET>,
            SetWrapperStrength: SetWrapperStrength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedObjectInfo as windows_core::Interface>::IID
    }
}
pub trait IManagedPoolAction_Impl: Sized {
    fn LastRelease(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IManagedPoolAction {}
impl IManagedPoolAction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IManagedPoolAction_Vtbl
    where
        Identity: IManagedPoolAction_Impl,
    {
        unsafe extern "system" fn LastRelease<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IManagedPoolAction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManagedPoolAction_Impl::LastRelease(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LastRelease: LastRelease::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedPoolAction as windows_core::Interface>::IID
    }
}
pub trait IManagedPooledObj_Impl: Sized {
    fn SetHeld(&self, m_bheld: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IManagedPooledObj {}
impl IManagedPooledObj_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IManagedPooledObj_Vtbl
    where
        Identity: IManagedPooledObj_Impl,
    {
        unsafe extern "system" fn SetHeld<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IManagedPooledObj_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManagedPooledObj_Impl::SetHeld(this, core::mem::transmute_copy(&m_bheld)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetHeld: SetHeld::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedPooledObj as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMessageMover_Impl: Sized + super::Com::IDispatch_Impl {
    fn SourcePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSourcePath(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DestPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDestPath(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CommitBatchSize(&self) -> windows_core::Result<i32>;
    fn SetCommitBatchSize(&self, newval: i32) -> windows_core::Result<()>;
    fn MoveMessages(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMessageMover {}
#[cfg(feature = "Win32_System_Com")]
impl IMessageMover_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMessageMover_Vtbl
    where
        Identity: IMessageMover_Impl,
    {
        unsafe extern "system" fn SourcePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMessageMover_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMessageMover_Impl::SourcePath(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourcePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMessageMover_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessageMover_Impl::SetSourcePath(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn DestPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMessageMover_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMessageMover_Impl::DestPath(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMessageMover_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessageMover_Impl::SetDestPath(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn CommitBatchSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMessageMover_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMessageMover_Impl::CommitBatchSize(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitBatchSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT
        where
            Identity: IMessageMover_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessageMover_Impl::SetCommitBatchSize(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MoveMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmessagesmoved: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMessageMover_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMessageMover_Impl::MoveMessages(this) {
                Ok(ok__) => {
                    plmessagesmoved.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SourcePath: SourcePath::<Identity, OFFSET>,
            SetSourcePath: SetSourcePath::<Identity, OFFSET>,
            DestPath: DestPath::<Identity, OFFSET>,
            SetDestPath: SetDestPath::<Identity, OFFSET>,
            CommitBatchSize: CommitBatchSize::<Identity, OFFSET>,
            SetCommitBatchSize: SetCommitBatchSize::<Identity, OFFSET>,
            MoveMessages: MoveMessages::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessageMover as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsEventInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Names(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EventID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Value(&self, skey: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMtsEventInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IMtsEventInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMtsEventInfo_Vtbl
    where
        Identity: IMtsEventInfo_Impl,
    {
        unsafe extern "system" fn Names<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMtsEventInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEventInfo_Impl::Names(this) {
                Ok(ok__) => {
                    punk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMtsEventInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEventInfo_Impl::DisplayName(this) {
                Ok(ok__) => {
                    sdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sguideventid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMtsEventInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEventInfo_Impl::EventID(this) {
                Ok(ok__) => {
                    sguideventid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMtsEventInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEventInfo_Impl::Count(this) {
                Ok(ok__) => {
                    lcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, skey: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMtsEventInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEventInfo_Impl::get_Value(this, core::mem::transmute(&skey)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Names: Names::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            EventID: EventID::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMtsEventInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsEvents_Impl: Sized + super::Com::IDispatch_Impl {
    fn PackageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PackageGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PostEvent(&self, vevent: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn FireEvents(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetProcessID(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMtsEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IMtsEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMtsEvents_Vtbl
    where
        Identity: IMtsEvents_Impl,
    {
        unsafe extern "system" fn PackageName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMtsEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEvents_Impl::PackageName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMtsEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEvents_Impl::PackageGuid(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vevent: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMtsEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMtsEvents_Impl::PostEvent(this, core::mem::transmute_copy(&vevent)).into()
        }
        unsafe extern "system" fn FireEvents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMtsEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEvents_Impl::FireEvents(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMtsEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsEvents_Impl::GetProcessID(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PackageName: PackageName::<Identity, OFFSET>,
            PackageGuid: PackageGuid::<Identity, OFFSET>,
            PostEvent: PostEvent::<Identity, OFFSET>,
            FireEvents: FireEvents::<Identity, OFFSET>,
            GetProcessID: GetProcessID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMtsEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsGrp_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, lindex: i32) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMtsGrp {}
#[cfg(feature = "Win32_System_Com")]
impl IMtsGrp_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMtsGrp_Vtbl
    where
        Identity: IMtsGrp_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMtsGrp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsGrp_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMtsGrp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMtsGrp_Impl::Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    ppunkdispatcher.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMtsGrp_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMtsGrp_Impl::Refresh(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMtsGrp as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IObjPool_Impl: Sized {
    fn Reserved1(&self);
    fn Reserved2(&self);
    fn Reserved3(&self);
    fn Reserved4(&self);
    fn PutEndTx(&self, pobj: Option<&windows_core::IUnknown>);
    fn Reserved5(&self);
    fn Reserved6(&self);
}
impl windows_core::RuntimeName for IObjPool {}
impl IObjPool_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjPool_Vtbl
    where
        Identity: IObjPool_Impl,
    {
        unsafe extern "system" fn Reserved1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IObjPool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjPool_Impl::Reserved1(this)
        }
        unsafe extern "system" fn Reserved2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IObjPool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjPool_Impl::Reserved2(this)
        }
        unsafe extern "system" fn Reserved3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IObjPool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjPool_Impl::Reserved3(this)
        }
        unsafe extern "system" fn Reserved4<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IObjPool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjPool_Impl::Reserved4(this)
        }
        unsafe extern "system" fn PutEndTx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobj: *mut core::ffi::c_void)
        where
            Identity: IObjPool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjPool_Impl::PutEndTx(this, windows_core::from_raw_borrowed(&pobj))
        }
        unsafe extern "system" fn Reserved5<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IObjPool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjPool_Impl::Reserved5(this)
        }
        unsafe extern "system" fn Reserved6<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IObjPool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjPool_Impl::Reserved6(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, OFFSET>,
            Reserved2: Reserved2::<Identity, OFFSET>,
            Reserved3: Reserved3::<Identity, OFFSET>,
            Reserved4: Reserved4::<Identity, OFFSET>,
            PutEndTx: PutEndTx::<Identity, OFFSET>,
            Reserved5: Reserved5::<Identity, OFFSET>,
            Reserved6: Reserved6::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjPool as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstruct_Impl: Sized {
    fn Construct(&self, pctorobj: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IObjectConstruct {}
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstruct_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectConstruct_Vtbl
    where
        Identity: IObjectConstruct_Impl,
    {
        unsafe extern "system" fn Construct<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctorobj: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectConstruct_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectConstruct_Impl::Construct(this, windows_core::from_raw_borrowed(&pctorobj)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Construct: Construct::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectConstruct as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstructString_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConstructString(&self, pval: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IObjectConstructString {}
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstructString_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectConstructString_Vtbl
    where
        Identity: IObjectConstructString_Impl,
    {
        unsafe extern "system" fn ConstructString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IObjectConstructString_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectConstructString_Impl::ConstructString(this, core::mem::transmute_copy(&pval)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ConstructString: ConstructString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectConstructString as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IObjectContext_Impl: Sized {
    fn CreateInstance(&self, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetComplete(&self) -> windows_core::Result<()>;
    fn SetAbort(&self) -> windows_core::Result<()>;
    fn EnableCommit(&self) -> windows_core::Result<()>;
    fn DisableCommit(&self) -> windows_core::Result<()>;
    fn IsInTransaction(&self) -> super::super::Foundation::BOOL;
    fn IsSecurityEnabled(&self) -> super::super::Foundation::BOOL;
    fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectContext {}
impl IObjectContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectContext_Vtbl
    where
        Identity: IObjectContext_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContext_Impl::CreateInstance(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn SetComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContext_Impl::SetComplete(this).into()
        }
        unsafe extern "system" fn SetAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContext_Impl::SetAbort(this).into()
        }
        unsafe extern "system" fn EnableCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContext_Impl::EnableCommit(this).into()
        }
        unsafe extern "system" fn DisableCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContext_Impl::DisableCommit(this).into()
        }
        unsafe extern "system" fn IsInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContext_Impl::IsInTransaction(this)
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContext_Impl::IsSecurityEnabled(this)
        }
        unsafe extern "system" fn IsCallerInRole<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole), core::mem::transmute_copy(&pfisinrole)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            SetComplete: SetComplete::<Identity, OFFSET>,
            SetAbort: SetAbort::<Identity, OFFSET>,
            EnableCommit: EnableCommit::<Identity, OFFSET>,
            DisableCommit: DisableCommit::<Identity, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContext as windows_core::Interface>::IID
    }
}
pub trait IObjectContextActivity_Impl: Sized {
    fn GetActivityId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectContextActivity {}
impl IObjectContextActivity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectContextActivity_Vtbl
    where
        Identity: IObjectContextActivity_Impl,
    {
        unsafe extern "system" fn GetActivityId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IObjectContextActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextActivity_Impl::GetActivityId(this, core::mem::transmute_copy(&pguid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetActivityId: GetActivityId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContextActivity as windows_core::Interface>::IID
    }
}
pub trait IObjectContextInfo_Impl: Sized {
    fn IsInTransaction(&self) -> super::super::Foundation::BOOL;
    fn GetTransaction(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetTransactionId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetActivityId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetContextId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectContextInfo {}
impl IObjectContextInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectContextInfo_Vtbl
    where
        Identity: IObjectContextInfo_Impl,
    {
        unsafe extern "system" fn IsInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IObjectContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextInfo_Impl::IsInTransaction(this)
        }
        unsafe extern "system" fn GetTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptrans: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectContextInfo_Impl::GetTransaction(this) {
                Ok(ok__) => {
                    pptrans.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IObjectContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextInfo_Impl::GetTransactionId(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetActivityId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IObjectContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextInfo_Impl::GetActivityId(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetContextId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IObjectContextInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextInfo_Impl::GetContextId(this, core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, OFFSET>,
            GetTransaction: GetTransaction::<Identity, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, OFFSET>,
            GetActivityId: GetActivityId::<Identity, OFFSET>,
            GetContextId: GetContextId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContextInfo as windows_core::Interface>::IID
    }
}
pub trait IObjectContextInfo2_Impl: Sized + IObjectContextInfo_Impl {
    fn GetPartitionId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetApplicationId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetApplicationInstanceId(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectContextInfo2 {}
impl IObjectContextInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectContextInfo2_Vtbl
    where
        Identity: IObjectContextInfo2_Impl,
    {
        unsafe extern "system" fn GetPartitionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IObjectContextInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextInfo2_Impl::GetPartitionId(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IObjectContextInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextInfo2_Impl::GetApplicationId(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IObjectContextInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextInfo2_Impl::GetApplicationInstanceId(this, core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base__: IObjectContextInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPartitionId: GetPartitionId::<Identity, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContextInfo2 as windows_core::Interface>::IID || iid == &<IObjectContextInfo as windows_core::Interface>::IID
    }
}
pub trait IObjectContextTip_Impl: Sized {
    fn GetTipUrl(&self, ptipurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IObjectContextTip {}
impl IObjectContextTip_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectContextTip_Vtbl
    where
        Identity: IObjectContextTip_Impl,
    {
        unsafe extern "system" fn GetTipUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptipurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IObjectContextTip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectContextTip_Impl::GetTipUrl(this, core::mem::transmute_copy(&ptipurl)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTipUrl: GetTipUrl::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectContextTip as windows_core::Interface>::IID
    }
}
pub trait IObjectControl_Impl: Sized {
    fn Activate(&self) -> windows_core::Result<()>;
    fn Deactivate(&self);
    fn CanBePooled(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IObjectControl {}
impl IObjectControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectControl_Vtbl
    where
        Identity: IObjectControl_Impl,
    {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IObjectControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectControl_Impl::Activate(this).into()
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IObjectControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectControl_Impl::Deactivate(this)
        }
        unsafe extern "system" fn CanBePooled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: IObjectControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectControl_Impl::CanBePooled(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CanBePooled: CanBePooled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectControl as windows_core::Interface>::IID
    }
}
pub trait IPlaybackControl_Impl: Sized {
    fn FinalClientRetry(&self) -> windows_core::Result<()>;
    fn FinalServerRetry(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPlaybackControl {}
impl IPlaybackControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPlaybackControl_Vtbl
    where
        Identity: IPlaybackControl_Impl,
    {
        unsafe extern "system" fn FinalClientRetry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPlaybackControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPlaybackControl_Impl::FinalClientRetry(this).into()
        }
        unsafe extern "system" fn FinalServerRetry<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPlaybackControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPlaybackControl_Impl::FinalServerRetry(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinalClientRetry: FinalClientRetry::<Identity, OFFSET>,
            FinalServerRetry: FinalServerRetry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlaybackControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPoolManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn ShutdownPool(&self, clsidorprogid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPoolManager {}
#[cfg(feature = "Win32_System_Com")]
impl IPoolManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPoolManager_Vtbl
    where
        Identity: IPoolManager_Impl,
    {
        unsafe extern "system" fn ShutdownPool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidorprogid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IPoolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPoolManager_Impl::ShutdownPool(this, core::mem::transmute(&clsidorprogid)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ShutdownPool: ShutdownPool::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPoolManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IProcessInitializer_Impl: Sized {
    fn Startup(&self, punkprocesscontrol: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProcessInitializer {}
impl IProcessInitializer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProcessInitializer_Vtbl
    where
        Identity: IProcessInitializer_Impl,
    {
        unsafe extern "system" fn Startup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkprocesscontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProcessInitializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProcessInitializer_Impl::Startup(this, windows_core::from_raw_borrowed(&punkprocesscontrol)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProcessInitializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProcessInitializer_Impl::Shutdown(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Startup: Startup::<Identity, OFFSET>, Shutdown: Shutdown::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessInitializer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityCallContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsSecurityEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsUserInRole(&self, puser: *const windows_core::VARIANT, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISecurityCallContext {}
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISecurityCallContext_Vtbl
    where
        Identity: ISecurityCallContext_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISecurityCallContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallContext_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISecurityCallContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallContext_Impl::get_Item(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISecurityCallContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallContext_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISecurityCallContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole)) {
                Ok(ok__) => {
                    pfinrole.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISecurityCallContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallContext_Impl::IsSecurityEnabled(this) {
                Ok(ok__) => {
                    pfisenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserInRole<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISecurityCallContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallContext_Impl::IsUserInRole(this, core::mem::transmute_copy(&puser), core::mem::transmute(&bstrrole)) {
                Ok(ok__) => {
                    pfinrole.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, OFFSET>,
            IsUserInRole: IsUserInRole::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityCallContext as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityCallersColl_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> windows_core::Result<ISecurityIdentityColl>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISecurityCallersColl {}
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallersColl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISecurityCallersColl_Vtbl
    where
        Identity: ISecurityCallersColl_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISecurityCallersColl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallersColl_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISecurityCallersColl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallersColl_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    pobj.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISecurityCallersColl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityCallersColl_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityCallersColl as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityIdentityColl_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISecurityIdentityColl {}
#[cfg(feature = "Win32_System_Com")]
impl ISecurityIdentityColl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISecurityIdentityColl_Vtbl
    where
        Identity: ISecurityIdentityColl_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISecurityIdentityColl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityIdentityColl_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISecurityIdentityColl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityIdentityColl_Impl::get_Item(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISecurityIdentityColl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityIdentityColl_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityIdentityColl as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security")]
pub trait ISecurityProperty_Impl: Sized {
    fn GetDirectCreatorSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()>;
    fn GetOriginalCreatorSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()>;
    fn GetDirectCallerSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()>;
    fn GetOriginalCallerSID(&self, psid: *mut super::super::Security::PSID) -> windows_core::Result<()>;
    fn ReleaseSID(&self, psid: super::super::Security::PSID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for ISecurityProperty {}
#[cfg(feature = "Win32_Security")]
impl ISecurityProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISecurityProperty_Vtbl
    where
        Identity: ISecurityProperty_Impl,
    {
        unsafe extern "system" fn GetDirectCreatorSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Security::PSID) -> windows_core::HRESULT
        where
            Identity: ISecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISecurityProperty_Impl::GetDirectCreatorSID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Security::PSID) -> windows_core::HRESULT
        where
            Identity: ISecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISecurityProperty_Impl::GetOriginalCreatorSID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetDirectCallerSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Security::PSID) -> windows_core::HRESULT
        where
            Identity: ISecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISecurityProperty_Impl::GetDirectCallerSID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCallerSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Security::PSID) -> windows_core::HRESULT
        where
            Identity: ISecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISecurityProperty_Impl::GetOriginalCallerSID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn ReleaseSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: super::super::Security::PSID) -> windows_core::HRESULT
        where
            Identity: ISecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISecurityProperty_Impl::ReleaseSID(this, core::mem::transmute_copy(&psid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDirectCreatorSID: GetDirectCreatorSID::<Identity, OFFSET>,
            GetOriginalCreatorSID: GetOriginalCreatorSID::<Identity, OFFSET>,
            GetDirectCallerSID: GetDirectCallerSID::<Identity, OFFSET>,
            GetOriginalCallerSID: GetOriginalCallerSID::<Identity, OFFSET>,
            ReleaseSID: ReleaseSID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityProperty as windows_core::Interface>::IID
    }
}
pub trait ISelectCOMLBServer_Impl: Sized {
    fn Init(&self) -> windows_core::Result<()>;
    fn GetLBServer(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISelectCOMLBServer {}
impl ISelectCOMLBServer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISelectCOMLBServer_Vtbl
    where
        Identity: ISelectCOMLBServer_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISelectCOMLBServer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISelectCOMLBServer_Impl::Init(this).into()
        }
        unsafe extern "system" fn GetLBServer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISelectCOMLBServer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISelectCOMLBServer_Impl::GetLBServer(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Init: Init::<Identity, OFFSET>, GetLBServer: GetLBServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectCOMLBServer as windows_core::Interface>::IID
    }
}
pub trait ISendMethodEvents_Impl: Sized {
    fn SendMethodCall(&self, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32) -> windows_core::Result<()>;
    fn SendMethodReturn(&self, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32, hrcall: windows_core::HRESULT, hrserver: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISendMethodEvents {}
impl ISendMethodEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISendMethodEvents_Vtbl
    where
        Identity: ISendMethodEvents_Impl,
    {
        unsafe extern "system" fn SendMethodCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32) -> windows_core::HRESULT
        where
            Identity: ISendMethodEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISendMethodEvents_Impl::SendMethodCall(this, core::mem::transmute_copy(&pidentity), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&dwmeth)).into()
        }
        unsafe extern "system" fn SendMethodReturn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32, hrcall: windows_core::HRESULT, hrserver: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ISendMethodEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISendMethodEvents_Impl::SendMethodReturn(this, core::mem::transmute_copy(&pidentity), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&dwmeth), core::mem::transmute_copy(&hrcall), core::mem::transmute_copy(&hrserver)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendMethodCall: SendMethodCall::<Identity, OFFSET>,
            SendMethodReturn: SendMethodReturn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISendMethodEvents as windows_core::Interface>::IID
    }
}
pub trait IServiceActivity_Impl: Sized {
    fn SynchronousCall(&self, piservicecall: Option<&IServiceCall>) -> windows_core::Result<()>;
    fn AsynchronousCall(&self, piservicecall: Option<&IServiceCall>) -> windows_core::Result<()>;
    fn BindToCurrentThread(&self) -> windows_core::Result<()>;
    fn UnbindFromThread(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceActivity {}
impl IServiceActivity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceActivity_Vtbl
    where
        Identity: IServiceActivity_Impl,
    {
        unsafe extern "system" fn SynchronousCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piservicecall: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceActivity_Impl::SynchronousCall(this, windows_core::from_raw_borrowed(&piservicecall)).into()
        }
        unsafe extern "system" fn AsynchronousCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piservicecall: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceActivity_Impl::AsynchronousCall(this, windows_core::from_raw_borrowed(&piservicecall)).into()
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceActivity_Impl::BindToCurrentThread(this).into()
        }
        unsafe extern "system" fn UnbindFromThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServiceActivity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceActivity_Impl::UnbindFromThread(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, OFFSET>,
            AsynchronousCall: AsynchronousCall::<Identity, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceActivity as windows_core::Interface>::IID
    }
}
pub trait IServiceCall_Impl: Sized {
    fn OnCall(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceCall {}
impl IServiceCall_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceCall_Vtbl
    where
        Identity: IServiceCall_Impl,
    {
        unsafe extern "system" fn OnCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServiceCall_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceCall_Impl::OnCall(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceCall as windows_core::Interface>::IID
    }
}
pub trait IServiceComTIIntrinsicsConfig_Impl: Sized {
    fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceComTIIntrinsicsConfig {}
impl IServiceComTIIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceComTIIntrinsicsConfig_Vtbl
    where
        Identity: IServiceComTIIntrinsicsConfig_Impl,
    {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> windows_core::HRESULT
        where
            Identity: IServiceComTIIntrinsicsConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceComTIIntrinsicsConfig_Impl::ComTIIntrinsicsConfig(this, core::mem::transmute_copy(&comtiintrinsicsconfig)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ComTIIntrinsicsConfig: ComTIIntrinsicsConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceComTIIntrinsicsConfig as windows_core::Interface>::IID
    }
}
pub trait IServiceIISIntrinsicsConfig_Impl: Sized {
    fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceIISIntrinsicsConfig {}
impl IServiceIISIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceIISIntrinsicsConfig_Vtbl
    where
        Identity: IServiceIISIntrinsicsConfig_Impl,
    {
        unsafe extern "system" fn IISIntrinsicsConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> windows_core::HRESULT
        where
            Identity: IServiceIISIntrinsicsConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceIISIntrinsicsConfig_Impl::IISIntrinsicsConfig(this, core::mem::transmute_copy(&iisintrinsicsconfig)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IISIntrinsicsConfig: IISIntrinsicsConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceIISIntrinsicsConfig as windows_core::Interface>::IID
    }
}
pub trait IServiceInheritanceConfig_Impl: Sized {
    fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceInheritanceConfig {}
impl IServiceInheritanceConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceInheritanceConfig_Vtbl
    where
        Identity: IServiceInheritanceConfig_Impl,
    {
        unsafe extern "system" fn ContainingContextTreatment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> windows_core::HRESULT
        where
            Identity: IServiceInheritanceConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceInheritanceConfig_Impl::ContainingContextTreatment(this, core::mem::transmute_copy(&inheritanceconfig)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ContainingContextTreatment: ContainingContextTreatment::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceInheritanceConfig as windows_core::Interface>::IID
    }
}
pub trait IServicePartitionConfig_Impl: Sized {
    fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> windows_core::Result<()>;
    fn PartitionID(&self, guidpartitionid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServicePartitionConfig {}
impl IServicePartitionConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServicePartitionConfig_Vtbl
    where
        Identity: IServicePartitionConfig_Impl,
    {
        unsafe extern "system" fn PartitionConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> windows_core::HRESULT
        where
            Identity: IServicePartitionConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePartitionConfig_Impl::PartitionConfig(this, core::mem::transmute_copy(&partitionconfig)).into()
        }
        unsafe extern "system" fn PartitionID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpartitionid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IServicePartitionConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePartitionConfig_Impl::PartitionID(this, core::mem::transmute_copy(&guidpartitionid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PartitionConfig: PartitionConfig::<Identity, OFFSET>,
            PartitionID: PartitionID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServicePartitionConfig as windows_core::Interface>::IID
    }
}
pub trait IServicePool_Impl: Sized {
    fn Initialize(&self, ppoolconfig: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServicePool {}
impl IServicePool_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServicePool_Vtbl
    where
        Identity: IServicePool_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoolconfig: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServicePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePool_Impl::Initialize(this, windows_core::from_raw_borrowed(&ppoolconfig)).into()
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServicePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePool_Impl::GetObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServicePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePool_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServicePool as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IServicePoolConfig_Impl: Sized {
    fn SetMaxPoolSize(&self, dwmaxpool: u32) -> windows_core::Result<()>;
    fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> windows_core::Result<()>;
    fn SetMinPoolSize(&self, dwminpool: u32) -> windows_core::Result<()>;
    fn MinPoolSize(&self, pdwminpool: *mut u32) -> windows_core::Result<()>;
    fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> windows_core::Result<()>;
    fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> windows_core::Result<()>;
    fn SetTransactionAffinity(&self, ftxaffinity: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn TransactionAffinity(&self, pftxaffinity: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetClassFactory(&self, pfactory: Option<&super::Com::IClassFactory>) -> windows_core::Result<()>;
    fn ClassFactory(&self) -> windows_core::Result<super::Com::IClassFactory>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IServicePoolConfig {}
#[cfg(feature = "Win32_System_Com")]
impl IServicePoolConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServicePoolConfig_Vtbl
    where
        Identity: IServicePoolConfig_Impl,
    {
        unsafe extern "system" fn SetMaxPoolSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxpool: u32) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::SetMaxPoolSize(this, core::mem::transmute_copy(&dwmaxpool)).into()
        }
        unsafe extern "system" fn MaxPoolSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxpool: *mut u32) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::MaxPoolSize(this, core::mem::transmute_copy(&pdwmaxpool)).into()
        }
        unsafe extern "system" fn SetMinPoolSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminpool: u32) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::SetMinPoolSize(this, core::mem::transmute_copy(&dwminpool)).into()
        }
        unsafe extern "system" fn MinPoolSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminpool: *mut u32) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::MinPoolSize(this, core::mem::transmute_copy(&pdwminpool)).into()
        }
        unsafe extern "system" fn SetCreationTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreationtimeout: u32) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::SetCreationTimeout(this, core::mem::transmute_copy(&dwcreationtimeout)).into()
        }
        unsafe extern "system" fn CreationTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcreationtimeout: *mut u32) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::CreationTimeout(this, core::mem::transmute_copy(&pdwcreationtimeout)).into()
        }
        unsafe extern "system" fn SetTransactionAffinity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::SetTransactionAffinity(this, core::mem::transmute_copy(&ftxaffinity)).into()
        }
        unsafe extern "system" fn TransactionAffinity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::TransactionAffinity(this, core::mem::transmute_copy(&pftxaffinity)).into()
        }
        unsafe extern "system" fn SetClassFactory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfactory: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServicePoolConfig_Impl::SetClassFactory(this, windows_core::from_raw_borrowed(&pfactory)).into()
        }
        unsafe extern "system" fn ClassFactory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServicePoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IServicePoolConfig_Impl::ClassFactory(this) {
                Ok(ok__) => {
                    pfactory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMaxPoolSize: SetMaxPoolSize::<Identity, OFFSET>,
            MaxPoolSize: MaxPoolSize::<Identity, OFFSET>,
            SetMinPoolSize: SetMinPoolSize::<Identity, OFFSET>,
            MinPoolSize: MinPoolSize::<Identity, OFFSET>,
            SetCreationTimeout: SetCreationTimeout::<Identity, OFFSET>,
            CreationTimeout: CreationTimeout::<Identity, OFFSET>,
            SetTransactionAffinity: SetTransactionAffinity::<Identity, OFFSET>,
            TransactionAffinity: TransactionAffinity::<Identity, OFFSET>,
            SetClassFactory: SetClassFactory::<Identity, OFFSET>,
            ClassFactory: ClassFactory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServicePoolConfig as windows_core::Interface>::IID
    }
}
pub trait IServiceSxsConfig_Impl: Sized {
    fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> windows_core::Result<()>;
    fn SxsName(&self, szsxsname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SxsDirectory(&self, szsxsdirectory: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceSxsConfig {}
impl IServiceSxsConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceSxsConfig_Vtbl
    where
        Identity: IServiceSxsConfig_Impl,
    {
        unsafe extern "system" fn SxsConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scsconfig: CSC_SxsConfig) -> windows_core::HRESULT
        where
            Identity: IServiceSxsConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceSxsConfig_Impl::SxsConfig(this, core::mem::transmute_copy(&scsconfig)).into()
        }
        unsafe extern "system" fn SxsName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsxsname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IServiceSxsConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceSxsConfig_Impl::SxsName(this, core::mem::transmute(&szsxsname)).into()
        }
        unsafe extern "system" fn SxsDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsxsdirectory: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IServiceSxsConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceSxsConfig_Impl::SxsDirectory(this, core::mem::transmute(&szsxsdirectory)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SxsConfig: SxsConfig::<Identity, OFFSET>,
            SxsName: SxsName::<Identity, OFFSET>,
            SxsDirectory: SxsDirectory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceSxsConfig as windows_core::Interface>::IID
    }
}
pub trait IServiceSynchronizationConfig_Impl: Sized {
    fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceSynchronizationConfig {}
impl IServiceSynchronizationConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceSynchronizationConfig_Vtbl
    where
        Identity: IServiceSynchronizationConfig_Impl,
    {
        unsafe extern "system" fn ConfigureSynchronization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> windows_core::HRESULT
        where
            Identity: IServiceSynchronizationConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceSynchronizationConfig_Impl::ConfigureSynchronization(this, core::mem::transmute_copy(&synchconfig)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConfigureSynchronization: ConfigureSynchronization::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceSynchronizationConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceSysTxnConfig_Impl: Sized + IServiceTransactionConfig_Impl {
    fn ConfigureBYOTSysTxn(&self, ptxproxy: Option<&ITransactionProxy>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for IServiceSysTxnConfig {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl IServiceSysTxnConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceSysTxnConfig_Vtbl
    where
        Identity: IServiceSysTxnConfig_Impl,
    {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxproxy: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServiceSysTxnConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceSysTxnConfig_Impl::ConfigureBYOTSysTxn(this, windows_core::from_raw_borrowed(&ptxproxy)).into()
        }
        Self { base__: IServiceTransactionConfig_Vtbl::new::<Identity, OFFSET>(), ConfigureBYOTSysTxn: ConfigureBYOTSysTxn::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceSysTxnConfig as windows_core::Interface>::IID || iid == &<IServiceTransactionConfigBase as windows_core::Interface>::IID || iid == &<IServiceTransactionConfig as windows_core::Interface>::IID
    }
}
pub trait IServiceThreadPoolConfig_Impl: Sized {
    fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> windows_core::Result<()>;
    fn SetBindingInfo(&self, binding: CSC_Binding) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceThreadPoolConfig {}
impl IServiceThreadPoolConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceThreadPoolConfig_Vtbl
    where
        Identity: IServiceThreadPoolConfig_Impl,
    {
        unsafe extern "system" fn SelectThreadPool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadpool: CSC_ThreadPool) -> windows_core::HRESULT
        where
            Identity: IServiceThreadPoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceThreadPoolConfig_Impl::SelectThreadPool(this, core::mem::transmute_copy(&threadpool)).into()
        }
        unsafe extern "system" fn SetBindingInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, binding: CSC_Binding) -> windows_core::HRESULT
        where
            Identity: IServiceThreadPoolConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceThreadPoolConfig_Impl::SetBindingInfo(this, core::mem::transmute_copy(&binding)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SelectThreadPool: SelectThreadPool::<Identity, OFFSET>,
            SetBindingInfo: SetBindingInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceThreadPoolConfig as windows_core::Interface>::IID
    }
}
pub trait IServiceTrackerConfig_Impl: Sized {
    fn TrackerConfig(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: &windows_core::PCWSTR, sztrackerctxname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceTrackerConfig {}
impl IServiceTrackerConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceTrackerConfig_Vtbl
    where
        Identity: IServiceTrackerConfig_Impl,
    {
        unsafe extern "system" fn TrackerConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: windows_core::PCWSTR, sztrackerctxname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IServiceTrackerConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceTrackerConfig_Impl::TrackerConfig(this, core::mem::transmute_copy(&trackerconfig), core::mem::transmute(&sztrackerappname), core::mem::transmute(&sztrackerctxname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TrackerConfig: TrackerConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceTrackerConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceTransactionConfig_Impl: Sized + IServiceTransactionConfigBase_Impl {
    fn ConfigureBYOT(&self, pitxbyot: Option<&super::DistributedTransactionCoordinator::ITransaction>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for IServiceTransactionConfig {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl IServiceTransactionConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceTransactionConfig_Vtbl
    where
        Identity: IServiceTransactionConfig_Impl,
    {
        unsafe extern "system" fn ConfigureBYOT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitxbyot: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IServiceTransactionConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceTransactionConfig_Impl::ConfigureBYOT(this, windows_core::from_raw_borrowed(&pitxbyot)).into()
        }
        Self { base__: IServiceTransactionConfigBase_Vtbl::new::<Identity, OFFSET>(), ConfigureBYOT: ConfigureBYOT::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceTransactionConfig as windows_core::Interface>::IID || iid == &<IServiceTransactionConfigBase as windows_core::Interface>::IID
    }
}
pub trait IServiceTransactionConfigBase_Impl: Sized {
    fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> windows_core::Result<()>;
    fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> windows_core::Result<()>;
    fn TransactionTimeout(&self, ultimeoutsec: u32) -> windows_core::Result<()>;
    fn BringYourOwnTransaction(&self, sztipurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn NewTransactionDescription(&self, sztxdesc: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IServiceTransactionConfigBase {}
impl IServiceTransactionConfigBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IServiceTransactionConfigBase_Vtbl
    where
        Identity: IServiceTransactionConfigBase_Impl,
    {
        unsafe extern "system" fn ConfigureTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> windows_core::HRESULT
        where
            Identity: IServiceTransactionConfigBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceTransactionConfigBase_Impl::ConfigureTransaction(this, core::mem::transmute_copy(&transactionconfig)).into()
        }
        unsafe extern "system" fn IsolationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> windows_core::HRESULT
        where
            Identity: IServiceTransactionConfigBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceTransactionConfigBase_Impl::IsolationLevel(this, core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn TransactionTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultimeoutsec: u32) -> windows_core::HRESULT
        where
            Identity: IServiceTransactionConfigBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceTransactionConfigBase_Impl::TransactionTimeout(this, core::mem::transmute_copy(&ultimeoutsec)).into()
        }
        unsafe extern "system" fn BringYourOwnTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztipurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IServiceTransactionConfigBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceTransactionConfigBase_Impl::BringYourOwnTransaction(this, core::mem::transmute(&sztipurl)).into()
        }
        unsafe extern "system" fn NewTransactionDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztxdesc: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IServiceTransactionConfigBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IServiceTransactionConfigBase_Impl::NewTransactionDescription(this, core::mem::transmute(&sztxdesc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConfigureTransaction: ConfigureTransaction::<Identity, OFFSET>,
            IsolationLevel: IsolationLevel::<Identity, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, OFFSET>,
            BringYourOwnTransaction: BringYourOwnTransaction::<Identity, OFFSET>,
            NewTransactionDescription: NewTransactionDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceTransactionConfigBase as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, val: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISharedProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ISharedProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISharedProperty_Vtbl
    where
        Identity: ISharedProperty_Impl,
    {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISharedProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISharedProperty_Impl::Value(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISharedProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISharedProperty_Impl::SetValue(this, core::mem::transmute(&val)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Value: Value::<Identity, OFFSET>, SetValue: SetValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISharedProperty as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedPropertyGroup_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreatePropertyByPosition(&self, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut Option<ISharedProperty>) -> windows_core::Result<()>;
    fn get_PropertyByPosition(&self, index: i32) -> windows_core::Result<ISharedProperty>;
    fn CreateProperty(&self, name: &windows_core::BSTR, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut Option<ISharedProperty>) -> windows_core::Result<()>;
    fn get_Property(&self, name: &windows_core::BSTR) -> windows_core::Result<ISharedProperty>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISharedPropertyGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISharedPropertyGroup_Vtbl
    where
        Identity: ISharedPropertyGroup_Impl,
    {
        unsafe extern "system" fn CreatePropertyByPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISharedPropertyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISharedPropertyGroup_Impl::CreatePropertyByPosition(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn get_PropertyByPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISharedPropertyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISharedPropertyGroup_Impl::get_PropertyByPosition(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISharedPropertyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISharedPropertyGroup_Impl::CreateProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn get_Property<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISharedPropertyGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISharedPropertyGroup_Impl::get_Property(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    ppproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyByPosition: CreatePropertyByPosition::<Identity, OFFSET>,
            get_PropertyByPosition: get_PropertyByPosition::<Identity, OFFSET>,
            CreateProperty: CreateProperty::<Identity, OFFSET>,
            get_Property: get_Property::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISharedPropertyGroup as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedPropertyGroupManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreatePropertyGroup(&self, name: &windows_core::BSTR, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut Option<ISharedPropertyGroup>) -> windows_core::Result<()>;
    fn get_Group(&self, name: &windows_core::BSTR) -> windows_core::Result<ISharedPropertyGroup>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISharedPropertyGroupManager {}
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroupManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISharedPropertyGroupManager_Vtbl
    where
        Identity: ISharedPropertyGroupManager_Impl,
    {
        unsafe extern "system" fn CreatePropertyGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISharedPropertyGroupManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISharedPropertyGroupManager_Impl::CreatePropertyGroup(this, core::mem::transmute(&name), core::mem::transmute_copy(&dwisomode), core::mem::transmute_copy(&dwrelmode), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppgroup)).into()
        }
        unsafe extern "system" fn get_Group<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISharedPropertyGroupManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISharedPropertyGroupManager_Impl::get_Group(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISharedPropertyGroupManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISharedPropertyGroupManager_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreatePropertyGroup: CreatePropertyGroup::<Identity, OFFSET>,
            get_Group: get_Group::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISharedPropertyGroupManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ISystemAppEventData_Impl: Sized {
    fn Startup(&self) -> windows_core::Result<()>;
    fn OnDataChanged(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: &windows_core::BSTR, dwreason: u32, u64tracehandle: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISystemAppEventData {}
impl ISystemAppEventData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISystemAppEventData_Vtbl
    where
        Identity: ISystemAppEventData_Impl,
    {
        unsafe extern "system" fn Startup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISystemAppEventData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemAppEventData_Impl::Startup(this).into()
        }
        unsafe extern "system" fn OnDataChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: core::mem::MaybeUninit<windows_core::BSTR>, dwreason: u32, u64tracehandle: u64) -> windows_core::HRESULT
        where
            Identity: ISystemAppEventData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISystemAppEventData_Impl::OnDataChanged(this, core::mem::transmute_copy(&dwpid), core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&dwnumbersinks), core::mem::transmute(&bstrdwmethodmask), core::mem::transmute_copy(&dwreason), core::mem::transmute_copy(&u64tracehandle)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Startup: Startup::<Identity, OFFSET>,
            OnDataChanged: OnDataChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemAppEventData as windows_core::Interface>::IID
    }
}
pub trait IThreadPoolKnobs_Impl: Sized {
    fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> windows_core::Result<()>;
    fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> windows_core::Result<()>;
    fn SetMaxThreads(&self, lcmaxthreads: i32) -> windows_core::Result<()>;
    fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> windows_core::Result<()>;
    fn SetDeleteDelay(&self, msecdeletedelay: i32) -> windows_core::Result<()>;
    fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> windows_core::Result<()>;
    fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> windows_core::Result<()>;
    fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> windows_core::Result<()>;
    fn SetMinThreads(&self, lcminthreads: i32) -> windows_core::Result<()>;
    fn SetQueueDepth(&self, lcqueuedepth: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IThreadPoolKnobs {}
impl IThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IThreadPoolKnobs_Vtbl
    where
        Identity: IThreadPoolKnobs_Impl,
    {
        unsafe extern "system" fn GetMaxThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcmaxthreads: *mut i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::GetMaxThreads(this, core::mem::transmute_copy(&plcmaxthreads)).into()
        }
        unsafe extern "system" fn GetCurrentThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plccurrentthreads: *mut i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::GetCurrentThreads(this, core::mem::transmute_copy(&plccurrentthreads)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcmaxthreads: i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::SetMaxThreads(this, core::mem::transmute_copy(&lcmaxthreads)).into()
        }
        unsafe extern "system" fn GetDeleteDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsecdeletedelay: *mut i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::GetDeleteDelay(this, core::mem::transmute_copy(&pmsecdeletedelay)).into()
        }
        unsafe extern "system" fn SetDeleteDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msecdeletedelay: i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::SetDeleteDelay(this, core::mem::transmute_copy(&msecdeletedelay)).into()
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::GetMaxQueuedRequests(this, core::mem::transmute_copy(&plcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::GetCurrentQueuedRequests(this, core::mem::transmute_copy(&plccurrentqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcmaxqueuedrequests: i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::SetMaxQueuedRequests(this, core::mem::transmute_copy(&lcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMinThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcminthreads: i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::SetMinThreads(this, core::mem::transmute_copy(&lcminthreads)).into()
        }
        unsafe extern "system" fn SetQueueDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcqueuedepth: i32) -> windows_core::HRESULT
        where
            Identity: IThreadPoolKnobs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IThreadPoolKnobs_Impl::SetQueueDepth(this, core::mem::transmute_copy(&lcqueuedepth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxThreads: GetMaxThreads::<Identity, OFFSET>,
            GetCurrentThreads: GetCurrentThreads::<Identity, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, OFFSET>,
            GetDeleteDelay: GetDeleteDelay::<Identity, OFFSET>,
            SetDeleteDelay: SetDeleteDelay::<Identity, OFFSET>,
            GetMaxQueuedRequests: GetMaxQueuedRequests::<Identity, OFFSET>,
            GetCurrentQueuedRequests: GetCurrentQueuedRequests::<Identity, OFFSET>,
            SetMaxQueuedRequests: SetMaxQueuedRequests::<Identity, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IThreadPoolKnobs as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITransactionContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateInstance(&self, pszprogid: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITransactionContext {}
#[cfg(feature = "Win32_System_Com")]
impl ITransactionContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionContext_Vtbl
    where
        Identity: ITransactionContext_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprogid: core::mem::MaybeUninit<windows_core::BSTR>, pobject: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ITransactionContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionContext_Impl::CreateInstance(this, core::mem::transmute(&pszprogid)) {
                Ok(ok__) => {
                    pobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionContext_Impl::Commit(this).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionContext_Impl::Abort(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionContext as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ITransactionContextEx_Impl: Sized {
    fn CreateInstance(&self, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionContextEx {}
impl ITransactionContextEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionContextEx_Vtbl
    where
        Identity: ITransactionContextEx_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionContextEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionContextEx_Impl::CreateInstance(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionContextEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionContextEx_Impl::Commit(this).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionContextEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionContextEx_Impl::Abort(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionContextEx as windows_core::Interface>::IID
    }
}
pub trait ITransactionProperty_Impl: Sized {
    fn Reserved1(&self);
    fn Reserved2(&self);
    fn Reserved3(&self);
    fn Reserved4(&self);
    fn Reserved5(&self);
    fn Reserved6(&self);
    fn Reserved7(&self);
    fn Reserved8(&self);
    fn Reserved9(&self);
    fn GetTransactionResourcePool(&self) -> windows_core::Result<ITransactionResourcePool>;
    fn Reserved10(&self);
    fn Reserved11(&self);
    fn Reserved12(&self);
    fn Reserved13(&self);
    fn Reserved14(&self);
    fn Reserved15(&self);
    fn Reserved16(&self);
    fn Reserved17(&self);
}
impl windows_core::RuntimeName for ITransactionProperty {}
impl ITransactionProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionProperty_Vtbl
    where
        Identity: ITransactionProperty_Impl,
    {
        unsafe extern "system" fn Reserved1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved1(this)
        }
        unsafe extern "system" fn Reserved2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved2(this)
        }
        unsafe extern "system" fn Reserved3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved3(this)
        }
        unsafe extern "system" fn Reserved4<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved4(this)
        }
        unsafe extern "system" fn Reserved5<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved5(this)
        }
        unsafe extern "system" fn Reserved6<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved6(this)
        }
        unsafe extern "system" fn Reserved7<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved7(this)
        }
        unsafe extern "system" fn Reserved8<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved8(this)
        }
        unsafe extern "system" fn Reserved9<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved9(this)
        }
        unsafe extern "system" fn GetTransactionResourcePool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptxpool: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionProperty_Impl::GetTransactionResourcePool(this) {
                Ok(ok__) => {
                    pptxpool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved10<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved10(this)
        }
        unsafe extern "system" fn Reserved11<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved11(this)
        }
        unsafe extern "system" fn Reserved12<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved12(this)
        }
        unsafe extern "system" fn Reserved13<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved13(this)
        }
        unsafe extern "system" fn Reserved14<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved14(this)
        }
        unsafe extern "system" fn Reserved15<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved15(this)
        }
        unsafe extern "system" fn Reserved16<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved16(this)
        }
        unsafe extern "system" fn Reserved17<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ITransactionProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProperty_Impl::Reserved17(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, OFFSET>,
            Reserved2: Reserved2::<Identity, OFFSET>,
            Reserved3: Reserved3::<Identity, OFFSET>,
            Reserved4: Reserved4::<Identity, OFFSET>,
            Reserved5: Reserved5::<Identity, OFFSET>,
            Reserved6: Reserved6::<Identity, OFFSET>,
            Reserved7: Reserved7::<Identity, OFFSET>,
            Reserved8: Reserved8::<Identity, OFFSET>,
            Reserved9: Reserved9::<Identity, OFFSET>,
            GetTransactionResourcePool: GetTransactionResourcePool::<Identity, OFFSET>,
            Reserved10: Reserved10::<Identity, OFFSET>,
            Reserved11: Reserved11::<Identity, OFFSET>,
            Reserved12: Reserved12::<Identity, OFFSET>,
            Reserved13: Reserved13::<Identity, OFFSET>,
            Reserved14: Reserved14::<Identity, OFFSET>,
            Reserved15: Reserved15::<Identity, OFFSET>,
            Reserved16: Reserved16::<Identity, OFFSET>,
            Reserved17: Reserved17::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionProperty as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionProxy_Impl: Sized {
    fn Commit(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn Promote(&self) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransaction>;
    fn CreateVoter(&self, ptxasync: Option<&super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>;
    fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> windows_core::Result<()>;
    fn GetIdentifier(&self, pbstridentifier: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn IsReusable(&self, pfisreusable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for ITransactionProxy {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionProxy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionProxy_Vtbl
    where
        Identity: ITransactionProxy_Impl,
    {
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITransactionProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProxy_Impl::Commit(this, core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProxy_Impl::Abort(this).into()
        }
        unsafe extern "system" fn Promote<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionProxy_Impl::Promote(this) {
                Ok(ok__) => {
                    ptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVoter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxasync: *mut core::ffi::c_void, ppballot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionProxy_Impl::CreateVoter(this, windows_core::from_raw_borrowed(&ptxasync)) {
                Ok(ok__) => {
                    ppballot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsolationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> windows_core::HRESULT
        where
            Identity: ITransactionProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProxy_Impl::GetIsolationLevel(this, core::mem::transmute_copy(&__midl__itransactionproxy0000)).into()
        }
        unsafe extern "system" fn GetIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstridentifier: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ITransactionProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProxy_Impl::GetIdentifier(this, core::mem::transmute_copy(&pbstridentifier)).into()
        }
        unsafe extern "system" fn IsReusable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITransactionProxy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionProxy_Impl::IsReusable(this, core::mem::transmute_copy(&pfisreusable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            Promote: Promote::<Identity, OFFSET>,
            CreateVoter: CreateVoter::<Identity, OFFSET>,
            GetIsolationLevel: GetIsolationLevel::<Identity, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, OFFSET>,
            IsReusable: IsReusable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionProxy as windows_core::Interface>::IID
    }
}
pub trait ITransactionResourcePool_Impl: Sized {
    fn PutResource(&self, ppool: Option<&IObjPool>, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetResource(&self, ppool: Option<&IObjPool>) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ITransactionResourcePool {}
impl ITransactionResourcePool_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionResourcePool_Vtbl
    where
        Identity: ITransactionResourcePool_Impl,
    {
        unsafe extern "system" fn PutResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppool: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionResourcePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionResourcePool_Impl::PutResource(this, windows_core::from_raw_borrowed(&ppool), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppool: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionResourcePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionResourcePool_Impl::GetResource(this, windows_core::from_raw_borrowed(&ppool)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutResource: PutResource::<Identity, OFFSET>,
            GetResource: GetResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionResourcePool as windows_core::Interface>::IID
    }
}
pub trait ITransactionStatus_Impl: Sized {
    fn SetTransactionStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetTransactionStatus(&self, phrstatus: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITransactionStatus {}
impl ITransactionStatus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionStatus_Vtbl
    where
        Identity: ITransactionStatus_Impl,
    {
        unsafe extern "system" fn SetTransactionStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITransactionStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionStatus_Impl::SetTransactionStatus(this, core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn GetTransactionStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ITransactionStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionStatus_Impl::GetTransactionStatus(this, core::mem::transmute_copy(&phrstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTransactionStatus: SetTransactionStatus::<Identity, OFFSET>,
            GetTransactionStatus: GetTransactionStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionStatus as windows_core::Interface>::IID
    }
}
pub trait ITxProxyHolder_Impl: Sized {
    fn GetIdentifier(&self, pguidltx: *mut windows_core::GUID);
}
impl windows_core::RuntimeName for ITxProxyHolder {}
impl ITxProxyHolder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITxProxyHolder_Vtbl
    where
        Identity: ITxProxyHolder_Impl,
    {
        unsafe extern "system" fn GetIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidltx: *mut windows_core::GUID)
        where
            Identity: ITxProxyHolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITxProxyHolder_Impl::GetIdentifier(this, core::mem::transmute_copy(&pguidltx))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdentifier: GetIdentifier::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITxProxyHolder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ObjectContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateInstance(&self, bstrprogid: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn SetComplete(&self) -> windows_core::Result<()>;
    fn SetAbort(&self) -> windows_core::Result<()>;
    fn EnableCommit(&self) -> windows_core::Result<()>;
    fn DisableCommit(&self) -> windows_core::Result<()>;
    fn IsInTransaction(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsSecurityEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsCallerInRole(&self, bstrrole: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, name: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Security(&self) -> windows_core::Result<SecurityProperty>;
    fn ContextInfo(&self) -> windows_core::Result<ContextInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ObjectContext {}
#[cfg(feature = "Win32_System_Com")]
impl ObjectContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ObjectContext_Vtbl
    where
        Identity: ObjectContext_Impl,
    {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprogid: core::mem::MaybeUninit<windows_core::BSTR>, pobject: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::CreateInstance(this, core::mem::transmute(&bstrprogid)) {
                Ok(ok__) => {
                    pobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ObjectContext_Impl::SetComplete(this).into()
        }
        unsafe extern "system" fn SetAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ObjectContext_Impl::SetAbort(this).into()
        }
        unsafe extern "system" fn EnableCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ObjectContext_Impl::EnableCommit(this).into()
        }
        unsafe extern "system" fn DisableCommit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ObjectContext_Impl::DisableCommit(this).into()
        }
        unsafe extern "system" fn IsInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::IsInTransaction(this) {
                Ok(ok__) => {
                    pbisintx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::IsSecurityEnabled(this) {
                Ok(ok__) => {
                    pbisenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>, pbinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole)) {
                Ok(ok__) => {
                    pbinrole.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::get_Item(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::Security(this) {
                Ok(ok__) => {
                    ppsecurityproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontextinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ObjectContext_Impl::ContextInfo(this) {
                Ok(ok__) => {
                    ppcontextinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            SetComplete: SetComplete::<Identity, OFFSET>,
            SetAbort: SetAbort::<Identity, OFFSET>,
            EnableCommit: EnableCommit::<Identity, OFFSET>,
            DisableCommit: DisableCommit::<Identity, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            ContextInfo: ContextInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ObjectContext as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ObjectControl_Impl: Sized {
    fn Activate(&self) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn CanBePooled(&self, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ObjectControl {}
impl ObjectControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ObjectControl_Vtbl
    where
        Identity: ObjectControl_Impl,
    {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ObjectControl_Impl::Activate(this).into()
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ObjectControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ObjectControl_Impl::Deactivate(this).into()
        }
        unsafe extern "system" fn CanBePooled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ObjectControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ObjectControl_Impl::CanBePooled(this, core::mem::transmute_copy(&pbpoolable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            CanBePooled: CanBePooled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ObjectControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait SecurityProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDirectCallerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDirectCreatorName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetOriginalCallerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetOriginalCreatorName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for SecurityProperty {}
#[cfg(feature = "Win32_System_Com")]
impl SecurityProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> SecurityProperty_Vtbl
    where
        Identity: SecurityProperty_Impl,
    {
        unsafe extern "system" fn GetDirectCallerName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: SecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match SecurityProperty_Impl::GetDirectCallerName(this) {
                Ok(ok__) => {
                    bstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectCreatorName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: SecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match SecurityProperty_Impl::GetDirectCreatorName(this) {
                Ok(ok__) => {
                    bstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCallerName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: SecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match SecurityProperty_Impl::GetOriginalCallerName(this) {
                Ok(ok__) => {
                    bstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCreatorName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: SecurityProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match SecurityProperty_Impl::GetOriginalCreatorName(this) {
                Ok(ok__) => {
                    bstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDirectCallerName: GetDirectCallerName::<Identity, OFFSET>,
            GetDirectCreatorName: GetDirectCreatorName::<Identity, OFFSET>,
            GetOriginalCallerName: GetOriginalCallerName::<Identity, OFFSET>,
            GetOriginalCreatorName: GetOriginalCreatorName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<SecurityProperty as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
