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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>() -> ContextInfo_Vtbl {
        unsafe extern "system" fn IsInTransaction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ContextInfo_Impl::IsInTransaction(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisintx, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransaction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptx: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ContextInfo_Impl::GetTransaction(this) {
                Ok(ok__) => {
                    core::ptr::write(pptx, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtxid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ContextInfo_Impl::GetTransactionId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtxid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstractivityid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ContextInfo_Impl::GetActivityId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstractivityid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrctxid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ContextInfo_Impl::GetContextId(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrctxid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, Impl, OFFSET>,
            GetActivityId: GetActivityId::<Identity, Impl, OFFSET>,
            GetContextId: GetContextId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>() -> ContextInfo2_Vtbl {
        unsafe extern "system" fn GetPartitionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20000: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ContextInfo2_Impl::GetPartitionId(this) {
                Ok(ok__) => {
                    core::ptr::write(__midl__contextinfo20000, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20001: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ContextInfo2_Impl::GetApplicationId(this) {
                Ok(ok__) => {
                    core::ptr::write(__midl__contextinfo20001, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__contextinfo20002: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ContextInfo2_Impl::GetApplicationInstanceId(this) {
                Ok(ok__) => {
                    core::ptr::write(__midl__contextinfo20002, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ContextInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPartitionId: GetPartitionId::<Identity, Impl, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, Impl, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>() -> IAppDomainHelper_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkad: *mut core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAppDomainHelper_Impl::Initialize(this, windows_core::from_raw_borrowed(&punkad), core::mem::transmute_copy(&__midl__iappdomainhelper0000), core::mem::transmute_copy(&ppool)).into()
        }
        unsafe extern "system" fn DoCallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkad: *mut core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAppDomainHelper_Impl::DoCallback(this, windows_core::from_raw_borrowed(&punkad), core::mem::transmute_copy(&__midl__iappdomainhelper0001), core::mem::transmute_copy(&ppool)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            DoCallback: DoCallback::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyLocator_Impl, const OFFSET: isize>() -> IAssemblyLocator_Vtbl {
        unsafe extern "system" fn GetModules<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationdir: core::mem::MaybeUninit<windows_core::BSTR>, applicationname: core::mem::MaybeUninit<windows_core::BSTR>, assemblyname: core::mem::MaybeUninit<windows_core::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IAssemblyLocator_Impl::GetModules(this, core::mem::transmute(&applicationdir), core::mem::transmute(&applicationname), core::mem::transmute(&assemblyname)) {
                Ok(ok__) => {
                    core::ptr::write(pmodules, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetModules: GetModules::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncErrorNotify_Impl, const OFFSET: isize>() -> IAsyncErrorNotify_Vtbl {
        unsafe extern "system" fn OnError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncErrorNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAsyncErrorNotify_Impl::OnError(this, core::mem::transmute_copy(&hr)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnError: OnError::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>() -> ICOMAdminCatalog_Vtbl {
        unsafe extern "system" fn GetCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: core::mem::MaybeUninit<windows_core::BSTR>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog_Impl::GetCollection(this, core::mem::transmute(&bstrcollname)) {
                Ok(ok__) => {
                    core::ptr::write(ppcatalogcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcatalogservername: core::mem::MaybeUninit<windows_core::BSTR>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog_Impl::Connect(this, core::mem::transmute(&bstrcatalogservername)) {
                Ok(ok__) => {
                    core::ptr::write(ppcatalogcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog_Impl::MajorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plmajorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog_Impl::MinorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plminorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionByQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog_Impl::GetCollectionByQuery(this, core::mem::transmute(&bstrcollname), core::mem::transmute_copy(&ppsavarquery)) {
                Ok(ok__) => {
                    core::ptr::write(ppcatalogcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsidorprogid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::ImportComponent(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrclsidorprogid)).into()
        }
        unsafe extern "system" fn InstallComponent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdll: core::mem::MaybeUninit<windows_core::BSTR>, bstrtlb: core::mem::MaybeUninit<windows_core::BSTR>, bstrpsdll: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::InstallComponent(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrdll), core::mem::transmute(&bstrtlb), core::mem::transmute(&bstrpsdll)).into()
        }
        unsafe extern "system" fn ShutdownApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::ShutdownApplication(this, core::mem::transmute(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ExportApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrapplicationfile: core::mem::MaybeUninit<windows_core::BSTR>, loptions: COMAdminApplicationExportOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::ExportApplication(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrapplicationfile), core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: core::mem::MaybeUninit<windows_core::BSTR>, bstrdestinationdirectory: core::mem::MaybeUninit<windows_core::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstrrsn: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::InstallApplication(this, core::mem::transmute(&bstrapplicationfile), core::mem::transmute(&bstrdestinationdirectory), core::mem::transmute_copy(&loptions), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrrsn)).into()
        }
        unsafe extern "system" fn StopRouter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::StopRouter(this).into()
        }
        unsafe extern "system" fn RefreshRouter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::RefreshRouter(this).into()
        }
        unsafe extern "system" fn StartRouter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::StartRouter(this).into()
        }
        unsafe extern "system" fn Reserved1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::Reserved1(this).into()
        }
        unsafe extern "system" fn Reserved2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::Reserved2(this).into()
        }
        unsafe extern "system" fn InstallMultipleComponents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::InstallMultipleComponents(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::GetMultipleComponentsInfo(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids), core::mem::transmute_copy(&ppsavarclassnames), core::mem::transmute_copy(&ppsavarfileflags), core::mem::transmute_copy(&ppsavarcomponentflags)).into()
        }
        unsafe extern "system" fn RefreshComponents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::RefreshComponents(this).into()
        }
        unsafe extern "system" fn BackupREGDB<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupfilepath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::BackupREGDB(this, core::mem::transmute(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn RestoreREGDB<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbackupfilepath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::RestoreREGDB(this, core::mem::transmute(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn QueryApplicationFile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: core::mem::MaybeUninit<windows_core::BSTR>, pbstrapplicationname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrapplicationdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::QueryApplicationFile(this, core::mem::transmute(&bstrapplicationfile), core::mem::transmute_copy(&pbstrapplicationname), core::mem::transmute_copy(&pbstrapplicationdescription), core::mem::transmute_copy(&pbhasusers), core::mem::transmute_copy(&pbisproxy), core::mem::transmute_copy(&ppsavarfilenames)).into()
        }
        unsafe extern "system" fn StartApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::StartApplication(this, core::mem::transmute(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ServiceCheck<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog_Impl::ServiceCheck(this, core::mem::transmute_copy(&lservice)) {
                Ok(ok__) => {
                    core::ptr::write(plstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::InstallMultipleEventClasses(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute_copy(&ppsavarfilenames), core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn InstallEventClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrdll: core::mem::MaybeUninit<windows_core::BSTR>, bstrtlb: core::mem::MaybeUninit<windows_core::BSTR>, bstrpsdll: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::InstallEventClass(this, core::mem::transmute(&bstrapplidorname), core::mem::transmute(&bstrdll), core::mem::transmute(&bstrtlb), core::mem::transmute(&bstrpsdll)).into()
        }
        unsafe extern "system" fn GetEventClassesForIID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstriid: core::mem::MaybeUninit<windows_core::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog_Impl::GetEventClassesForIID(this, core::mem::transmute(&bstriid), core::mem::transmute_copy(&ppsavarclsids), core::mem::transmute_copy(&ppsavarprogids), core::mem::transmute_copy(&ppsavardescriptions)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            GetCollectionByQuery: GetCollectionByQuery::<Identity, Impl, OFFSET>,
            ImportComponent: ImportComponent::<Identity, Impl, OFFSET>,
            InstallComponent: InstallComponent::<Identity, Impl, OFFSET>,
            ShutdownApplication: ShutdownApplication::<Identity, Impl, OFFSET>,
            ExportApplication: ExportApplication::<Identity, Impl, OFFSET>,
            InstallApplication: InstallApplication::<Identity, Impl, OFFSET>,
            StopRouter: StopRouter::<Identity, Impl, OFFSET>,
            RefreshRouter: RefreshRouter::<Identity, Impl, OFFSET>,
            StartRouter: StartRouter::<Identity, Impl, OFFSET>,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            InstallMultipleComponents: InstallMultipleComponents::<Identity, Impl, OFFSET>,
            GetMultipleComponentsInfo: GetMultipleComponentsInfo::<Identity, Impl, OFFSET>,
            RefreshComponents: RefreshComponents::<Identity, Impl, OFFSET>,
            BackupREGDB: BackupREGDB::<Identity, Impl, OFFSET>,
            RestoreREGDB: RestoreREGDB::<Identity, Impl, OFFSET>,
            QueryApplicationFile: QueryApplicationFile::<Identity, Impl, OFFSET>,
            StartApplication: StartApplication::<Identity, Impl, OFFSET>,
            ServiceCheck: ServiceCheck::<Identity, Impl, OFFSET>,
            InstallMultipleEventClasses: InstallMultipleEventClasses::<Identity, Impl, OFFSET>,
            InstallEventClass: InstallEventClass::<Identity, Impl, OFFSET>,
            GetEventClassesForIID: GetEventClassesForIID::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>() -> ICOMAdminCatalog2_Vtbl {
        unsafe extern "system" fn GetCollectionByQuery2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollectionname: core::mem::MaybeUninit<windows_core::BSTR>, pvarquerystrings: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::GetCollectionByQuery2(this, core::mem::transmute(&bstrcollectionname), core::mem::transmute_copy(&pvarquerystrings)) {
                Ok(ok__) => {
                    core::ptr::write(ppcatalogcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::GetApplicationInstanceIDFromProcessID(this, core::mem::transmute_copy(&lprocessid)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrapplicationinstanceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::ShutdownApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn PauseApplicationInstances<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::PauseApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn ResumeApplicationInstances<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::ResumeApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn RecycleApplicationInstances<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>, lreasoncode: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::RecycleApplicationInstances(this, core::mem::transmute_copy(&pvarapplicationinstanceid), core::mem::transmute_copy(&lreasoncode)).into()
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarapplicationinstanceid: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarboolpaused: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::AreApplicationInstancesPaused(this, core::mem::transmute_copy(&pvarapplicationinstanceid)) {
                Ok(ok__) => {
                    core::ptr::write(pvarboolpaused, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpApplicationInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationinstanceid: core::mem::MaybeUninit<windows_core::BSTR>, bstrdirectory: core::mem::MaybeUninit<windows_core::BSTR>, lmaximages: i32, pbstrdumpfile: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::DumpApplicationInstance(this, core::mem::transmute(&bstrapplicationinstanceid), core::mem::transmute(&bstrdirectory), core::mem::transmute_copy(&lmaximages)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdumpfile, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbooldumpsupported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::IsApplicationInstanceDumpSupported(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarbooldumpsupported, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServiceForApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrservicename: core::mem::MaybeUninit<windows_core::BSTR>, bstrstarttype: core::mem::MaybeUninit<windows_core::BSTR>, bstrerrorcontrol: core::mem::MaybeUninit<windows_core::BSTR>, bstrdependencies: core::mem::MaybeUninit<windows_core::BSTR>, bstrrunas: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::CreateServiceForApplication(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute(&bstrservicename), core::mem::transmute(&bstrstarttype), core::mem::transmute(&bstrerrorcontrol), core::mem::transmute(&bstrdependencies), core::mem::transmute(&bstrrunas), core::mem::transmute(&bstrpassword), core::mem::transmute_copy(&bdesktopok)).into()
        }
        unsafe extern "system" fn DeleteServiceForApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::DeleteServiceForApplication(this, core::mem::transmute(&bstrapplicationidorname)).into()
        }
        unsafe extern "system" fn GetPartitionID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrpartitionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::GetPartitionID(this, core::mem::transmute(&bstrapplicationidorname)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpartitionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartitionName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrpartitionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::GetPartitionName(this, core::mem::transmute(&bstrapplicationidorname)) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpartitionname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPartition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpartitionidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::SetCurrentPartition(this, core::mem::transmute(&bstrpartitionidorname)).into()
        }
        unsafe extern "system" fn CurrentPartitionID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpartitionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::CurrentPartitionID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpartitionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPartitionName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpartitionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::CurrentPartitionName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpartitionname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalPartitionID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrglobalpartitionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::GlobalPartitionID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrglobalpartitionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushPartitionCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::FlushPartitionCache(this).into()
        }
        unsafe extern "system" fn CopyApplications<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourcepartitionidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarapplicationid: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrdestinationpartitionidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::CopyApplications(this, core::mem::transmute(&bstrsourcepartitionidorname), core::mem::transmute_copy(&pvarapplicationid), core::mem::transmute(&bstrdestinationpartitionidorname)).into()
        }
        unsafe extern "system" fn CopyComponents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrdestinationapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::CopyComponents(this, core::mem::transmute(&bstrsourceapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn MoveComponents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrdestinationapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::MoveComponents(this, core::mem::transmute(&bstrsourceapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn AliasComponent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsrcapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsidorprogid: core::mem::MaybeUninit<windows_core::BSTR>, bstrdestapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrnewprogid: core::mem::MaybeUninit<windows_core::BSTR>, bstrnewclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::AliasComponent(this, core::mem::transmute(&bstrsrcapplicationidorname), core::mem::transmute(&bstrclsidorprogid), core::mem::transmute(&bstrdestapplicationidorname), core::mem::transmute(&bstrnewprogid), core::mem::transmute(&bstrnewclsid)).into()
        }
        unsafe extern "system" fn IsSafeToDelete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdllname: core::mem::MaybeUninit<windows_core::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::IsSafeToDelete(this, core::mem::transmute(&bstrdllname)) {
                Ok(ok__) => {
                    core::ptr::write(pcomadmininuse, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarcomponenttype: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::ImportUnconfiguredComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarcomponenttype: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::PromoteUnconfiguredComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn ImportComponents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationidorname: core::mem::MaybeUninit<windows_core::BSTR>, pvarclsidorprogid: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarcomponenttype: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::ImportComponents(this, core::mem::transmute(&bstrapplicationidorname), core::mem::transmute_copy(&pvarclsidorprogid), core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn Is64BitCatalogServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbis64bit: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::Is64BitCatalogServer(this) {
                Ok(ok__) => {
                    core::ptr::write(pbis64bit, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPartition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpartitionidorname: core::mem::MaybeUninit<windows_core::BSTR>, bstrpartitionfilename: core::mem::MaybeUninit<windows_core::BSTR>, loptions: COMAdminApplicationExportOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::ExportPartition(this, core::mem::transmute(&bstrpartitionidorname), core::mem::transmute(&bstrpartitionfilename), core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallPartition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, bstrdestdirectory: core::mem::MaybeUninit<windows_core::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: core::mem::MaybeUninit<windows_core::BSTR>, bstrpassword: core::mem::MaybeUninit<windows_core::BSTR>, bstrrsn: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMAdminCatalog2_Impl::InstallPartition(this, core::mem::transmute(&bstrfilename), core::mem::transmute(&bstrdestdirectory), core::mem::transmute_copy(&loptions), core::mem::transmute(&bstruserid), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrrsn)).into()
        }
        unsafe extern "system" fn QueryApplicationFile2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrapplicationfile: core::mem::MaybeUninit<windows_core::BSTR>, ppfilesforimport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::QueryApplicationFile2(this, core::mem::transmute(&bstrapplicationfile)) {
                Ok(ok__) => {
                    core::ptr::write(ppfilesforimport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentVersionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsidorprogid: core::mem::MaybeUninit<windows_core::BSTR>, plversioncount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICOMAdminCatalog2_Impl::GetComponentVersionCount(this, core::mem::transmute(&bstrclsidorprogid)) {
                Ok(ok__) => {
                    core::ptr::write(plversioncount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICOMAdminCatalog_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCollectionByQuery2: GetCollectionByQuery2::<Identity, Impl, OFFSET>,
            GetApplicationInstanceIDFromProcessID: GetApplicationInstanceIDFromProcessID::<Identity, Impl, OFFSET>,
            ShutdownApplicationInstances: ShutdownApplicationInstances::<Identity, Impl, OFFSET>,
            PauseApplicationInstances: PauseApplicationInstances::<Identity, Impl, OFFSET>,
            ResumeApplicationInstances: ResumeApplicationInstances::<Identity, Impl, OFFSET>,
            RecycleApplicationInstances: RecycleApplicationInstances::<Identity, Impl, OFFSET>,
            AreApplicationInstancesPaused: AreApplicationInstancesPaused::<Identity, Impl, OFFSET>,
            DumpApplicationInstance: DumpApplicationInstance::<Identity, Impl, OFFSET>,
            IsApplicationInstanceDumpSupported: IsApplicationInstanceDumpSupported::<Identity, Impl, OFFSET>,
            CreateServiceForApplication: CreateServiceForApplication::<Identity, Impl, OFFSET>,
            DeleteServiceForApplication: DeleteServiceForApplication::<Identity, Impl, OFFSET>,
            GetPartitionID: GetPartitionID::<Identity, Impl, OFFSET>,
            GetPartitionName: GetPartitionName::<Identity, Impl, OFFSET>,
            SetCurrentPartition: SetCurrentPartition::<Identity, Impl, OFFSET>,
            CurrentPartitionID: CurrentPartitionID::<Identity, Impl, OFFSET>,
            CurrentPartitionName: CurrentPartitionName::<Identity, Impl, OFFSET>,
            GlobalPartitionID: GlobalPartitionID::<Identity, Impl, OFFSET>,
            FlushPartitionCache: FlushPartitionCache::<Identity, Impl, OFFSET>,
            CopyApplications: CopyApplications::<Identity, Impl, OFFSET>,
            CopyComponents: CopyComponents::<Identity, Impl, OFFSET>,
            MoveComponents: MoveComponents::<Identity, Impl, OFFSET>,
            AliasComponent: AliasComponent::<Identity, Impl, OFFSET>,
            IsSafeToDelete: IsSafeToDelete::<Identity, Impl, OFFSET>,
            ImportUnconfiguredComponents: ImportUnconfiguredComponents::<Identity, Impl, OFFSET>,
            PromoteUnconfiguredComponents: PromoteUnconfiguredComponents::<Identity, Impl, OFFSET>,
            ImportComponents: ImportComponents::<Identity, Impl, OFFSET>,
            Is64BitCatalogServer: Is64BitCatalogServer::<Identity, Impl, OFFSET>,
            ExportPartition: ExportPartition::<Identity, Impl, OFFSET>,
            InstallPartition: InstallPartition::<Identity, Impl, OFFSET>,
            QueryApplicationFile2: QueryApplicationFile2::<Identity, Impl, OFFSET>,
            GetComponentVersionCount: GetComponentVersionCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>() -> ICOMLBArguments_Vtbl {
        unsafe extern "system" fn GetCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMLBArguments_Impl::GetCLSID(this, core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMLBArguments_Impl::SetCLSID(this, core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn GetMachineName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchsvr: u32, szservername: windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMLBArguments_Impl::GetMachineName(this, core::mem::transmute_copy(&cchsvr), core::mem::transmute_copy(&szservername)).into()
        }
        unsafe extern "system" fn SetMachineName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchsvr: u32, szservername: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICOMLBArguments_Impl::SetMachineName(this, core::mem::transmute_copy(&cchsvr), core::mem::transmute(&szservername)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLSID: GetCLSID::<Identity, Impl, OFFSET>,
            SetCLSID: SetCLSID::<Identity, Impl, OFFSET>,
            GetMachineName: GetMachineName::<Identity, Impl, OFFSET>,
            SetMachineName: SetMachineName::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>() -> ICatalogCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenumvariant, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppcatalogobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppcatalogobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plobjectcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plobjectcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICatalogCollection_Impl::Remove(this, core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcatalogobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::Add(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcatalogobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Populate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICatalogCollection_Impl::Populate(this).into()
        }
        unsafe extern "system" fn SaveChanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchanges: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::SaveChanges(this) {
                Ok(ok__) => {
                    core::ptr::write(pcchanges, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcollname: core::mem::MaybeUninit<windows_core::BSTR>, varobjectkey: core::mem::MaybeUninit<windows_core::VARIANT>, ppcatalogcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::GetCollection(this, core::mem::transmute(&bstrcollname), core::mem::transmute(&varobjectkey)) {
                Ok(ok__) => {
                    core::ptr::write(ppcatalogcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarnamel: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarnamel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::AddEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarbool, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::RemoveEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarbool, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUtilInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::GetUtilInterface(this) {
                Ok(ok__) => {
                    core::ptr::write(ppidispatch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMajorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::DataStoreMajorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plmajorversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMinorVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plminorversionl: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogCollection_Impl::DataStoreMinorVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(plminorversionl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulateByKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICatalogCollection_Impl::PopulateByKey(this, core::mem::transmute_copy(&psakeys)).into()
        }
        unsafe extern "system" fn PopulateByQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrquerystring: core::mem::MaybeUninit<windows_core::BSTR>, lquerytype: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICatalogCollection_Impl::PopulateByQuery(this, core::mem::transmute(&bstrquerystring), core::mem::transmute_copy(&lquerytype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Populate: Populate::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            AddEnabled: AddEnabled::<Identity, Impl, OFFSET>,
            RemoveEnabled: RemoveEnabled::<Identity, Impl, OFFSET>,
            GetUtilInterface: GetUtilInterface::<Identity, Impl, OFFSET>,
            DataStoreMajorVersion: DataStoreMajorVersion::<Identity, Impl, OFFSET>,
            DataStoreMinorVersion: DataStoreMinorVersion::<Identity, Impl, OFFSET>,
            PopulateByKey: PopulateByKey::<Identity, Impl, OFFSET>,
            PopulateByQuery: PopulateByQuery::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>() -> ICatalogObject_Vtbl {
        unsafe extern "system" fn get_Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, pvarretval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogObject_Impl::get_Value(this, core::mem::transmute(&bstrpropname)) {
                Ok(ok__) => {
                    core::ptr::write(pvarretval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, val: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICatalogObject_Impl::put_Value(this, core::mem::transmute(&bstrpropname), core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn Key<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarretval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogObject_Impl::Key(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarretval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarretval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogObject_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pvarretval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyReadOnly<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogObject_Impl::IsPropertyReadOnly(this, core::mem::transmute(&bstrpropname)) {
                Ok(ok__) => {
                    core::ptr::write(pbretval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogObject_Impl::Valid(this) {
                Ok(ok__) => {
                    core::ptr::write(pbretval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICatalogObject_Impl::IsPropertyWriteOnly(this, core::mem::transmute(&bstrpropname)) {
                Ok(ok__) => {
                    core::ptr::write(pbretval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Value: get_Value::<Identity, Impl, OFFSET>,
            put_Value: put_Value::<Identity, Impl, OFFSET>,
            Key: Key::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsPropertyReadOnly: IsPropertyReadOnly::<Identity, Impl, OFFSET>,
            Valid: Valid::<Identity, Impl, OFFSET>,
            IsPropertyWriteOnly: IsPropertyWriteOnly::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICheckSxsConfig_Impl, const OFFSET: isize>() -> ICheckSxsConfig_Vtbl {
        unsafe extern "system" fn IsSameSxsConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICheckSxsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszsxsname: windows_core::PCWSTR, wszsxsdirectory: windows_core::PCWSTR, wszsxsappname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICheckSxsConfig_Impl::IsSameSxsConfig(this, core::mem::transmute(&wszsxsname), core::mem::transmute(&wszsxsdirectory), core::mem::transmute(&wszsxsappname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSameSxsConfig: IsSameSxsConfig::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>() -> IComActivityEvents_Vtbl {
        unsafe extern "system" fn OnActivityCreate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComActivityEvents_Impl::OnActivityCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityDestroy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComActivityEvents_Impl::OnActivityDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityEnter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComActivityEvents_Impl::OnActivityEnter(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidentered), core::mem::transmute_copy(&dwthread)).into()
        }
        unsafe extern "system" fn OnActivityTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidentered: *const windows_core::GUID, dwthread: u32, dwtimeout: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComActivityEvents_Impl::OnActivityTimeout(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidentered), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnActivityReenter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComActivityEvents_Impl::OnActivityReenter(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwcalldepth)).into()
        }
        unsafe extern "system" fn OnActivityLeave<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, guidleft: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComActivityEvents_Impl::OnActivityLeave(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&guidleft)).into()
        }
        unsafe extern "system" fn OnActivityLeaveSame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const windows_core::GUID, dwcalldepth: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComActivityEvents_Impl::OnActivityLeaveSame(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidcurrent), core::mem::transmute_copy(&dwcalldepth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnActivityCreate: OnActivityCreate::<Identity, Impl, OFFSET>,
            OnActivityDestroy: OnActivityDestroy::<Identity, Impl, OFFSET>,
            OnActivityEnter: OnActivityEnter::<Identity, Impl, OFFSET>,
            OnActivityTimeout: OnActivityTimeout::<Identity, Impl, OFFSET>,
            OnActivityReenter: OnActivityReenter::<Identity, Impl, OFFSET>,
            OnActivityLeave: OnActivityLeave::<Identity, Impl, OFFSET>,
            OnActivityLeaveSame: OnActivityLeaveSame::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>() -> IComApp2Events_Vtbl {
        unsafe extern "system" fn OnAppActivation2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComApp2Events_Impl::OnAppActivation2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute(&guidprocess)).into()
        }
        unsafe extern "system" fn OnAppShutdown2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComApp2Events_Impl::OnAppShutdown2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComApp2Events_Impl::OnAppForceShutdown2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppPaused2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, bpaused: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComApp2Events_Impl::OnAppPaused2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute_copy(&bpaused)).into()
        }
        unsafe extern "system" fn OnAppRecycle2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID, guidprocess: windows_core::GUID, lreason: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComApp2Events_Impl::OnAppRecycle2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp), core::mem::transmute(&guidprocess), core::mem::transmute_copy(&lreason)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppActivation2: OnAppActivation2::<Identity, Impl, OFFSET>,
            OnAppShutdown2: OnAppShutdown2::<Identity, Impl, OFFSET>,
            OnAppForceShutdown2: OnAppForceShutdown2::<Identity, Impl, OFFSET>,
            OnAppPaused2: OnAppPaused2::<Identity, Impl, OFFSET>,
            OnAppRecycle2: OnAppRecycle2::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>() -> IComAppEvents_Vtbl {
        unsafe extern "system" fn OnAppActivation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComAppEvents_Impl::OnAppActivation(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppShutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComAppEvents_Impl::OnAppShutdown(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComAppEvents_Impl::OnAppForceShutdown(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppActivation: OnAppActivation::<Identity, Impl, OFFSET>,
            OnAppShutdown: OnAppShutdown::<Identity, Impl, OFFSET>,
            OnAppForceShutdown: OnAppForceShutdown::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>() -> IComCRMEvents_Vtbl {
        unsafe extern "system" fn OnCRMRecoveryStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMRecoveryStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMRecoveryDone(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMCheckpoint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMCheckpoint(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMBegin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, guidactivity: windows_core::GUID, guidtx: windows_core::GUID, szprogidcompensator: windows_core::PCWSTR, szdescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMBegin(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute(&guidactivity), core::mem::transmute(&guidtx), core::mem::transmute(&szprogidcompensator), core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn OnCRMPrepare<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMIndoubt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMIndoubt(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMDone(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMRelease<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMRelease(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAnalyze<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMAnalyze(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&dwcrmrecordtype), core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMWrite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMWrite(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&fvariants), core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMForget<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMForget(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMForce<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMForce(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDeliver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComCRMEvents_Impl::OnCRMDeliver(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidclerkclsid), core::mem::transmute_copy(&fvariants), core::mem::transmute_copy(&dwrecordsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCRMRecoveryStart: OnCRMRecoveryStart::<Identity, Impl, OFFSET>,
            OnCRMRecoveryDone: OnCRMRecoveryDone::<Identity, Impl, OFFSET>,
            OnCRMCheckpoint: OnCRMCheckpoint::<Identity, Impl, OFFSET>,
            OnCRMBegin: OnCRMBegin::<Identity, Impl, OFFSET>,
            OnCRMPrepare: OnCRMPrepare::<Identity, Impl, OFFSET>,
            OnCRMCommit: OnCRMCommit::<Identity, Impl, OFFSET>,
            OnCRMAbort: OnCRMAbort::<Identity, Impl, OFFSET>,
            OnCRMIndoubt: OnCRMIndoubt::<Identity, Impl, OFFSET>,
            OnCRMDone: OnCRMDone::<Identity, Impl, OFFSET>,
            OnCRMRelease: OnCRMRelease::<Identity, Impl, OFFSET>,
            OnCRMAnalyze: OnCRMAnalyze::<Identity, Impl, OFFSET>,
            OnCRMWrite: OnCRMWrite::<Identity, Impl, OFFSET>,
            OnCRMForget: OnCRMForget::<Identity, Impl, OFFSET>,
            OnCRMForce: OnCRMForce::<Identity, Impl, OFFSET>,
            OnCRMDeliver: OnCRMDeliver::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComExceptionEvents_Impl, const OFFSET: isize>() -> IComExceptionEvents_Vtbl {
        unsafe extern "system" fn OnExceptionUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComExceptionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComExceptionEvents_Impl::OnExceptionUser(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&code), core::mem::transmute_copy(&address), core::mem::transmute(&pszstacktrace)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnExceptionUser: OnExceptionUser::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComIdentityEvents_Impl, const OFFSET: isize>() -> IComIdentityEvents_Vtbl {
        unsafe extern "system" fn OnIISRequestInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComIdentityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: windows_core::PCWSTR, pszserverip: windows_core::PCWSTR, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComIdentityEvents_Impl::OnIISRequestInfo(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute(&pszclientip), core::mem::transmute(&pszserverip), core::mem::transmute(&pszurl)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIISRequestInfo: OnIISRequestInfo::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>() -> IComInstance2Events_Vtbl {
        unsafe extern "system" fn OnObjectCreate2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComInstance2Events_Impl::OnObjectCreate2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjectDestroy2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComInstance2Events_Impl::OnObjectDestroy2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectCreate2: OnObjectCreate2::<Identity, Impl, OFFSET>,
            OnObjectDestroy2: OnObjectDestroy2::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>() -> IComInstanceEvents_Vtbl {
        unsafe extern "system" fn OnObjectCreate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, clsid: *const windows_core::GUID, tsid: *const windows_core::GUID, ctxtid: u64, objectid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComInstanceEvents_Impl::OnObjectCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDestroy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComInstanceEvents_Impl::OnObjectDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectCreate: OnObjectCreate::<Identity, Impl, OFFSET>,
            OnObjectDestroy: OnObjectDestroy::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>() -> IComLTxEvents_Vtbl {
        unsafe extern "system" fn OnLtxTransactionStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, tsid: windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComLTxEvents_Impl::OnLtxTransactionStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute(&tsid), core::mem::transmute_copy(&froot), core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, fvote: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComLTxEvents_Impl::OnLtxTransactionPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute_copy(&fvote)).into()
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComLTxEvents_Impl::OnLtxTransactionAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComLTxEvents_Impl::OnLtxTransactionCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: windows_core::GUID, txnid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComLTxEvents_Impl::OnLtxTransactionPromote(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&guidltx), core::mem::transmute(&txnid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLtxTransactionStart: OnLtxTransactionStart::<Identity, Impl, OFFSET>,
            OnLtxTransactionPrepare: OnLtxTransactionPrepare::<Identity, Impl, OFFSET>,
            OnLtxTransactionAbort: OnLtxTransactionAbort::<Identity, Impl, OFFSET>,
            OnLtxTransactionCommit: OnLtxTransactionCommit::<Identity, Impl, OFFSET>,
            OnLtxTransactionPromote: OnLtxTransactionPromote::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>() -> IComMethod2Events_Vtbl {
        unsafe extern "system" fn OnMethodCall2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComMethod2Events_Impl::OnMethodCall2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComMethod2Events_Impl::OnMethodReturn2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, dwthread: u32, imeth: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComMethod2Events_Impl::OnMethodException2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMethodCall2: OnMethodCall2::<Identity, Impl, OFFSET>,
            OnMethodReturn2: OnMethodReturn2::<Identity, Impl, OFFSET>,
            OnMethodException2: OnMethodException2::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>() -> IComMethodEvents_Vtbl {
        unsafe extern "system" fn OnMethodCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComMethodEvents_Impl::OnMethodCall(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComMethodEvents_Impl::OnMethodReturn(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const windows_core::GUID, guidrid: *const windows_core::GUID, imeth: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComMethodEvents_Impl::OnMethodException(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidcid), core::mem::transmute_copy(&guidrid), core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMethodCall: OnMethodCall::<Identity, Impl, OFFSET>,
            OnMethodReturn: OnMethodReturn::<Identity, Impl, OFFSET>,
            OnMethodException: OnMethodException::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>() -> IComMtaThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn MTASetMaxThreadCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxthreads: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComMtaThreadPoolKnobs_Impl::MTASetMaxThreadCount(this, core::mem::transmute_copy(&dwmaxthreads)).into()
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxthreads: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComMtaThreadPoolKnobs_Impl::MTAGetMaxThreadCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwmaxthreads, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MTASetThrottleValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthrottle: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComMtaThreadPoolKnobs_Impl::MTASetThrottleValue(this, core::mem::transmute_copy(&dwthrottle)).into()
        }
        unsafe extern "system" fn MTAGetThrottleValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthrottle: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComMtaThreadPoolKnobs_Impl::MTAGetThrottleValue(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwthrottle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MTASetMaxThreadCount: MTASetMaxThreadCount::<Identity, Impl, OFFSET>,
            MTAGetMaxThreadCount: MTAGetMaxThreadCount::<Identity, Impl, OFFSET>,
            MTASetThrottleValue: MTASetThrottleValue::<Identity, Impl, OFFSET>,
            MTAGetThrottleValue: MTAGetThrottleValue::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstruction2Events_Impl, const OFFSET: isize>() -> IComObjectConstruction2Events_Vtbl {
        unsafe extern "system" fn OnObjectConstruct2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstruction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: windows_core::PCWSTR, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectConstruction2Events_Impl::OnObjectConstruct2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute(&sconstructstring), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidpartition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnObjectConstruct2: OnObjectConstruct2::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstructionEvents_Impl, const OFFSET: isize>() -> IComObjectConstructionEvents_Vtbl {
        unsafe extern "system" fn OnObjectConstruct<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstructionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, sconstructstring: windows_core::PCWSTR, oid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectConstructionEvents_Impl::OnObjectConstruct(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute(&sconstructstring), core::mem::transmute_copy(&oid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnObjectConstruct: OnObjectConstruct::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>() -> IComObjectEvents_Vtbl {
        unsafe extern "system" fn OnObjectActivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectEvents_Impl::OnObjectActivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDeactivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectEvents_Impl::OnObjectDeactivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid), core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnDisableCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectEvents_Impl::OnDisableCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnEnableCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectEvents_Impl::OnEnableCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectEvents_Impl::OnSetComplete(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectEvents_Impl::OnSetAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectActivate: OnObjectActivate::<Identity, Impl, OFFSET>,
            OnObjectDeactivate: OnObjectDeactivate::<Identity, Impl, OFFSET>,
            OnDisableCommit: OnDisableCommit::<Identity, Impl, OFFSET>,
            OnEnableCommit: OnEnableCommit::<Identity, Impl, OFFSET>,
            OnSetComplete: OnSetComplete::<Identity, Impl, OFFSET>,
            OnSetAbort: OnSetAbort::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>() -> IComObjectPool2Events_Vtbl {
        unsafe extern "system" fn OnObjPoolPutObject2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPool2Events_Impl::OnObjPoolPutObject2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&nreason), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPool2Events_Impl::OnObjPoolGetObject2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid), core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPool2Events_Impl::OnObjPoolRecycleToTx2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64, guidpartition: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPool2Events_Impl::OnObjPoolGetFromTx2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid), core::mem::transmute_copy(&guidpartition)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject2: OnObjPoolPutObject2::<Identity, Impl, OFFSET>,
            OnObjPoolGetObject2: OnObjPoolGetObject2::<Identity, Impl, OFFSET>,
            OnObjPoolRecycleToTx2: OnObjPoolRecycleToTx2::<Identity, Impl, OFFSET>,
            OnObjPoolGetFromTx2: OnObjPoolGetFromTx2::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>() -> IComObjectPoolEvents_Vtbl {
        unsafe extern "system" fn OnObjPoolPutObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents_Impl::OnObjPoolPutObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&nreason), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, dwavailable: u32, oid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents_Impl::OnObjPoolGetObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwavailable), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents_Impl::OnObjPoolRecycleToTx(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, guidobject: *const windows_core::GUID, guidtx: *const windows_core::GUID, objid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents_Impl::OnObjPoolGetFromTx(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&objid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject: OnObjPoolPutObject::<Identity, Impl, OFFSET>,
            OnObjPoolGetObject: OnObjPoolGetObject::<Identity, Impl, OFFSET>,
            OnObjPoolRecycleToTx: OnObjPoolRecycleToTx::<Identity, Impl, OFFSET>,
            OnObjPoolGetFromTx: OnObjPoolGetFromTx::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>() -> IComObjectPoolEvents2_Vtbl {
        unsafe extern "system" fn OnObjPoolCreateObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents2_Impl::OnObjPoolCreateObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwobjscreated), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwobjscreated: u32, oid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents2_Impl::OnObjPoolDestroyObject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwobjscreated), core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents2_Impl::OnObjPoolCreateDecision(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&dwthreadswaiting), core::mem::transmute_copy(&dwavail), core::mem::transmute_copy(&dwcreated), core::mem::transmute_copy(&dwmin), core::mem::transmute_copy(&dwmax)).into()
        }
        unsafe extern "system" fn OnObjPoolTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, guidactivity: *const windows_core::GUID, dwtimeout: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents2_Impl::OnObjPoolTimeout(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComObjectPoolEvents2_Impl::OnObjPoolCreatePool(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidobject), core::mem::transmute_copy(&dwmin), core::mem::transmute_copy(&dwmax), core::mem::transmute_copy(&dwtimeout)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolCreateObject: OnObjPoolCreateObject::<Identity, Impl, OFFSET>,
            OnObjPoolDestroyObject: OnObjPoolDestroyObject::<Identity, Impl, OFFSET>,
            OnObjPoolCreateDecision: OnObjPoolCreateDecision::<Identity, Impl, OFFSET>,
            OnObjPoolTimeout: OnObjPoolTimeout::<Identity, Impl, OFFSET>,
            OnObjPoolCreatePool: OnObjPoolCreatePool::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>() -> IComQCEvents_Vtbl {
        unsafe extern "system" fn OnQCRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: windows_core::PCWSTR, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, msmqhr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComQCEvents_Impl::OnQCRecord(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute(&szqueue), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCQueueOpen<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: windows_core::PCWSTR, queueid: u64, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComQCEvents_Impl::OnQCQueueOpen(this, core::mem::transmute_copy(&pinfo), core::mem::transmute(&szqueue), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComQCEvents_Impl::OnQCReceive(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceiveFail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComQCEvents_Impl::OnQCReceiveFail(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&queueid), core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, retryindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComQCEvents_Impl::OnQCMoveToReTryQueue(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&retryindex)).into()
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComQCEvents_Impl::OnQCMoveToDeadQueue(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid)).into()
        }
        unsafe extern "system" fn OnQCPlayback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const windows_core::GUID, guidworkflowid: *const windows_core::GUID, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComQCEvents_Impl::OnQCPlayback(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objid), core::mem::transmute_copy(&guidmsgid), core::mem::transmute_copy(&guidworkflowid), core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnQCRecord: OnQCRecord::<Identity, Impl, OFFSET>,
            OnQCQueueOpen: OnQCQueueOpen::<Identity, Impl, OFFSET>,
            OnQCReceive: OnQCReceive::<Identity, Impl, OFFSET>,
            OnQCReceiveFail: OnQCReceiveFail::<Identity, Impl, OFFSET>,
            OnQCMoveToReTryQueue: OnQCMoveToReTryQueue::<Identity, Impl, OFFSET>,
            OnQCMoveToDeadQueue: OnQCMoveToDeadQueue::<Identity, Impl, OFFSET>,
            OnQCPlayback: OnQCPlayback::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>() -> IComResourceEvents_Vtbl {
        unsafe extern "system" fn OnResourceCreate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComResourceEvents_Impl::OnResourceCreate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted)).into()
        }
        unsafe extern "system" fn OnResourceAllocate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComResourceEvents_Impl::OnResourceAllocate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted), core::mem::transmute_copy(&numrated), core::mem::transmute_copy(&rating)).into()
        }
        unsafe extern "system" fn OnResourceRecycle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComResourceEvents_Impl::OnResourceRecycle(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceDestroy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: windows_core::HRESULT, psztype: windows_core::PCWSTR, resid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComResourceEvents_Impl::OnResourceDestroy(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&hr), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceTrack<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComResourceEvents_Impl::OnResourceTrack(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&objectid), core::mem::transmute(&psztype), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&enlisted)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnResourceCreate: OnResourceCreate::<Identity, Impl, OFFSET>,
            OnResourceAllocate: OnResourceAllocate::<Identity, Impl, OFFSET>,
            OnResourceRecycle: OnResourceRecycle::<Identity, Impl, OFFSET>,
            OnResourceDestroy: OnResourceDestroy::<Identity, Impl, OFFSET>,
            OnResourceTrack: OnResourceTrack::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>() -> IComSecurityEvents_Vtbl {
        unsafe extern "system" fn OnAuthenticate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComSecurityEvents_Impl::OnAuthenticate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidiid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&cbbyteorig), core::mem::transmute_copy(&psidoriginaluser), core::mem::transmute_copy(&cbbytecur), core::mem::transmute_copy(&psidcurrentuser), core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        unsafe extern "system" fn OnAuthenticateFail<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, objectid: u64, guidiid: *const windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComSecurityEvents_Impl::OnAuthenticateFail(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&guidiid), core::mem::transmute_copy(&imeth), core::mem::transmute_copy(&cbbyteorig), core::mem::transmute_copy(&psidoriginaluser), core::mem::transmute_copy(&cbbytecur), core::mem::transmute_copy(&psidcurrentuser), core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAuthenticate: OnAuthenticate::<Identity, Impl, OFFSET>,
            OnAuthenticateFail: OnAuthenticateFail::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>() -> IComStaThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn SetMinThreadCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minthreads: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs_Impl::SetMinThreadCount(this, core::mem::transmute_copy(&minthreads)).into()
        }
        unsafe extern "system" fn GetMinThreadCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minthreads: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs_Impl::GetMinThreadCount(this) {
                Ok(ok__) => {
                    core::ptr::write(minthreads, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxThreadCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxthreads: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs_Impl::SetMaxThreadCount(this, core::mem::transmute_copy(&maxthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreadCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxthreads: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs_Impl::GetMaxThreadCount(this) {
                Ok(ok__) => {
                    core::ptr::write(maxthreads, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityPerThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitiesperthread: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs_Impl::SetActivityPerThread(this, core::mem::transmute_copy(&activitiesperthread)).into()
        }
        unsafe extern "system" fn GetActivityPerThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitiesperthread: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs_Impl::GetActivityPerThread(this) {
                Ok(ok__) => {
                    core::ptr::write(activitiesperthread, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityRatio<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activityratio: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs_Impl::SetActivityRatio(this, core::mem::transmute_copy(&activityratio)).into()
        }
        unsafe extern "system" fn GetActivityRatio<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activityratio: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs_Impl::GetActivityRatio(this) {
                Ok(ok__) => {
                    core::ptr::write(activityratio, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreads: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs_Impl::GetThreadCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwthreads, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueueDepth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwqdepth: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs_Impl::GetQueueDepth(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwqdepth, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueueDepth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwqdepth: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs_Impl::SetQueueDepth(this, core::mem::transmute_copy(&dwqdepth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMinThreadCount: SetMinThreadCount::<Identity, Impl, OFFSET>,
            GetMinThreadCount: GetMinThreadCount::<Identity, Impl, OFFSET>,
            SetMaxThreadCount: SetMaxThreadCount::<Identity, Impl, OFFSET>,
            GetMaxThreadCount: GetMaxThreadCount::<Identity, Impl, OFFSET>,
            SetActivityPerThread: SetActivityPerThread::<Identity, Impl, OFFSET>,
            GetActivityPerThread: GetActivityPerThread::<Identity, Impl, OFFSET>,
            SetActivityRatio: SetActivityRatio::<Identity, Impl, OFFSET>,
            GetActivityRatio: GetActivityRatio::<Identity, Impl, OFFSET>,
            GetThreadCount: GetThreadCount::<Identity, Impl, OFFSET>,
            GetQueueDepth: GetQueueDepth::<Identity, Impl, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>() -> IComStaThreadPoolKnobs2_Vtbl {
        unsafe extern "system" fn GetMaxCPULoad<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwload: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs2_Impl::GetMaxCPULoad(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwload, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCPULoad<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwload: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs2_Impl::SetMaxCPULoad(this, core::mem::transmute_copy(&pdwload)).into()
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs2_Impl::GetCPUMetricEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pbmetricenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs2_Impl::SetCPUMetricEnabled(this, core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs2_Impl::GetCreateThreadsAggressively(this) {
                Ok(ok__) => {
                    core::ptr::write(pbmetricenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs2_Impl::SetCreateThreadsAggressively(this, core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetMaxCSR<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcsr: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs2_Impl::GetMaxCSR(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwcsr, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCSR<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcsr: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs2_Impl::SetMaxCSR(this, core::mem::transmute_copy(&dwcsr)).into()
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComStaThreadPoolKnobs2_Impl::GetWaitTimeForThreadCleanup(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwthreadcleanupwaittime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwthreadcleanupwaittime: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComStaThreadPoolKnobs2_Impl::SetWaitTimeForThreadCleanup(this, core::mem::transmute_copy(&dwthreadcleanupwaittime)).into()
        }
        Self {
            base__: IComStaThreadPoolKnobs_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMaxCPULoad: GetMaxCPULoad::<Identity, Impl, OFFSET>,
            SetMaxCPULoad: SetMaxCPULoad::<Identity, Impl, OFFSET>,
            GetCPUMetricEnabled: GetCPUMetricEnabled::<Identity, Impl, OFFSET>,
            SetCPUMetricEnabled: SetCPUMetricEnabled::<Identity, Impl, OFFSET>,
            GetCreateThreadsAggressively: GetCreateThreadsAggressively::<Identity, Impl, OFFSET>,
            SetCreateThreadsAggressively: SetCreateThreadsAggressively::<Identity, Impl, OFFSET>,
            GetMaxCSR: GetMaxCSR::<Identity, Impl, OFFSET>,
            SetMaxCSR: SetMaxCSR::<Identity, Impl, OFFSET>,
            GetWaitTimeForThreadCleanup: GetWaitTimeForThreadCleanup::<Identity, Impl, OFFSET>,
            SetWaitTimeForThreadCleanup: SetWaitTimeForThreadCleanup::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>() -> IComThreadEvents_Vtbl {
        unsafe extern "system" fn OnThreadStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadTerminate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&dwthread), core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadBindToApartment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadBindToApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&aptid), core::mem::transmute_copy(&dwactcnt), core::mem::transmute_copy(&dwlowcnt)).into()
        }
        unsafe extern "system" fn OnThreadUnBind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadUnBind(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&aptid), core::mem::transmute_copy(&dwactcnt)).into()
        }
        unsafe extern "system" fn OnThreadWorkEnque<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadWorkEnque(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadWorkPrivate(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid)).into()
        }
        unsafe extern "system" fn OnThreadWorkPublic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadWorkPublic(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadWorkRedirect(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen), core::mem::transmute_copy(&threadnum)).into()
        }
        unsafe extern "system" fn OnThreadWorkReject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadWorkReject(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&msgworkid), core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadAssignApartment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const windows_core::GUID, aptid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadAssignApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidactivity), core::mem::transmute_copy(&aptid)).into()
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComThreadEvents_Impl::OnThreadUnassignApartment(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&aptid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnThreadStart: OnThreadStart::<Identity, Impl, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, Impl, OFFSET>,
            OnThreadBindToApartment: OnThreadBindToApartment::<Identity, Impl, OFFSET>,
            OnThreadUnBind: OnThreadUnBind::<Identity, Impl, OFFSET>,
            OnThreadWorkEnque: OnThreadWorkEnque::<Identity, Impl, OFFSET>,
            OnThreadWorkPrivate: OnThreadWorkPrivate::<Identity, Impl, OFFSET>,
            OnThreadWorkPublic: OnThreadWorkPublic::<Identity, Impl, OFFSET>,
            OnThreadWorkRedirect: OnThreadWorkRedirect::<Identity, Impl, OFFSET>,
            OnThreadWorkReject: OnThreadWorkReject::<Identity, Impl, OFFSET>,
            OnThreadAssignApartment: OnThreadAssignApartment::<Identity, Impl, OFFSET>,
            OnThreadUnassignApartment: OnThreadUnassignApartment::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>() -> IComTrackingInfoCollection_Vtbl {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComTrackingInfoCollection_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(ptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComTrackingInfoCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTrackingInfoCollection_Impl::Item(this, core::mem::transmute_copy(&ulindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoEvents_Impl, const OFFSET: isize>() -> IComTrackingInfoEvents_Vtbl {
        unsafe extern "system" fn OnNewTrackingInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoplevelcollection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTrackingInfoEvents_Impl::OnNewTrackingInfo(this, windows_core::from_raw_borrowed(&ptoplevelcollection)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNewTrackingInfo: OnNewTrackingInfo::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoObject_Impl, const OFFSET: isize>() -> IComTrackingInfoObject_Vtbl {
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szpropertyname: windows_core::PCWSTR, pvarout: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComTrackingInfoObject_Impl::GetValue(this, core::mem::transmute(&szpropertyname)) {
                Ok(ok__) => {
                    core::ptr::write(pvarout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>() -> IComTrackingInfoProperties_Vtbl {
        unsafe extern "system" fn PropCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComTrackingInfoProperties_Impl::PropCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppszpropname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IComTrackingInfoProperties_Impl::GetPropName(this, core::mem::transmute_copy(&ulindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppszpropname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropCount: PropCount::<Identity, Impl, OFFSET>,
            GetPropName: GetPropName::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>() -> IComTransaction2Events_Vtbl {
        unsafe extern "system" fn OnTransactionStart2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTransaction2Events_Impl::OnTransactionStart2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&froot), core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTransaction2Events_Impl::OnTransactionPrepare2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTransaction2Events_Impl::OnTransactionAbort2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTransaction2Events_Impl::OnTransactionCommit2(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransactionStart2: OnTransactionStart2::<Identity, Impl, OFFSET>,
            OnTransactionPrepare2: OnTransactionPrepare2::<Identity, Impl, OFFSET>,
            OnTransactionAbort2: OnTransactionAbort2::<Identity, Impl, OFFSET>,
            OnTransactionCommit2: OnTransactionCommit2::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>() -> IComTransactionEvents_Vtbl {
        unsafe extern "system" fn OnTransactionStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, tsid: *const windows_core::GUID, froot: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTransactionEvents_Impl::OnTransactionStart(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&tsid), core::mem::transmute_copy(&froot)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTransactionEvents_Impl::OnTransactionPrepare(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx), core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTransactionEvents_Impl::OnTransactionAbort(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComTransactionEvents_Impl::OnTransactionCommit(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransactionStart: OnTransactionStart::<Identity, Impl, OFFSET>,
            OnTransactionPrepare: OnTransactionPrepare::<Identity, Impl, OFFSET>,
            OnTransactionAbort: OnTransactionAbort::<Identity, Impl, OFFSET>,
            OnTransactionCommit: OnTransactionCommit::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComUserEvent_Impl, const OFFSET: isize>() -> IComUserEvent_Vtbl {
        unsafe extern "system" fn OnUserEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IComUserEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IComUserEvent_Impl::OnUserEvent(this, core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pvarevent)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUserEvent: OnUserEvent::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>() -> IContextProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextProperties_Impl::Count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pproperty: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextProperties_Impl::GetProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContextProperties_Impl::EnumNames(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, property: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextProperties_Impl::SetProperty(this, core::mem::transmute(&name), core::mem::transmute(&property)).into()
        }
        unsafe extern "system" fn RemoveProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextProperties_Impl::RemoveProperty(this, core::mem::transmute(&name)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumNames: EnumNames::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>() -> IContextSecurityPerimeter_Vtbl {
        unsafe extern "system" fn GetPerimeterFlag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextSecurityPerimeter_Impl::GetPerimeterFlag(this, core::mem::transmute_copy(&pflag)).into()
        }
        unsafe extern "system" fn SetPerimeterFlag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextSecurityPerimeter_Impl::SetPerimeterFlag(this, core::mem::transmute_copy(&fflag)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPerimeterFlag: GetPerimeterFlag::<Identity, Impl, OFFSET>,
            SetPerimeterFlag: SetPerimeterFlag::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>() -> IContextState_Vtbl {
        unsafe extern "system" fn SetDeactivateOnReturn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextState_Impl::SetDeactivateOnReturn(this, core::mem::transmute_copy(&bdeactivate)).into()
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextState_Impl::GetDeactivateOnReturn(this, core::mem::transmute_copy(&pbdeactivate)).into()
        }
        unsafe extern "system" fn SetMyTransactionVote<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, txvote: TransactionVote) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextState_Impl::SetMyTransactionVote(this, core::mem::transmute_copy(&txvote)).into()
        }
        unsafe extern "system" fn GetMyTransactionVote<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxvote: *mut TransactionVote) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IContextState_Impl::GetMyTransactionVote(this, core::mem::transmute_copy(&ptxvote)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDeactivateOnReturn: SetDeactivateOnReturn::<Identity, Impl, OFFSET>,
            GetDeactivateOnReturn: GetDeactivateOnReturn::<Identity, Impl, OFFSET>,
            SetMyTransactionVote: SetMyTransactionVote::<Identity, Impl, OFFSET>,
            GetMyTransactionVote: GetMyTransactionVote::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithLocalTransaction_Impl, const OFFSET: isize>() -> ICreateWithLocalTransaction_Vtbl {
        unsafe extern "system" fn CreateInstanceWithSysTx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithLocalTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICreateWithLocalTransaction_Impl::CreateInstanceWithSysTx(this, windows_core::from_raw_borrowed(&ptransaction), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstanceWithSysTx: CreateInstanceWithSysTx::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>() -> ICreateWithTipTransactionEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtipurl: core::mem::MaybeUninit<windows_core::BSTR>, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICreateWithTipTransactionEx_Impl::CreateInstance(this, core::mem::transmute(&bstrtipurl), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTransactionEx_Impl, const OFFSET: isize>() -> ICreateWithTransactionEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTransactionEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICreateWithTransactionEx_Impl::CreateInstance(this, windows_core::from_raw_borrowed(&ptransaction), core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>() -> ICrmCompensator_Vtbl {
        unsafe extern "system" fn SetLogControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensator_Impl::SetLogControl(this, windows_core::from_raw_borrowed(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepare<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensator_Impl::BeginPrepare(this).into()
        }
        unsafe extern "system" fn PrepareRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmCompensator_Impl::PrepareRecord(this, core::mem::transmute(&crmlogrec)) {
                Ok(ok__) => {
                    core::ptr::write(pfforget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepare<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmCompensator_Impl::EndPrepare(this) {
                Ok(ok__) => {
                    core::ptr::write(pfoktoprepare, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensator_Impl::BeginCommit(this, core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn CommitRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmCompensator_Impl::CommitRecord(this, core::mem::transmute(&crmlogrec)) {
                Ok(ok__) => {
                    core::ptr::write(pfforget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensator_Impl::EndCommit(this).into()
        }
        unsafe extern "system" fn BeginAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensator_Impl::BeginAbort(this, core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn AbortRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmCompensator_Impl::AbortRecord(this, core::mem::transmute(&crmlogrec)) {
                Ok(ok__) => {
                    core::ptr::write(pfforget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensator_Impl::EndAbort(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLogControl: SetLogControl::<Identity, Impl, OFFSET>,
            BeginPrepare: BeginPrepare::<Identity, Impl, OFFSET>,
            PrepareRecord: PrepareRecord::<Identity, Impl, OFFSET>,
            EndPrepare: EndPrepare::<Identity, Impl, OFFSET>,
            BeginCommit: BeginCommit::<Identity, Impl, OFFSET>,
            CommitRecord: CommitRecord::<Identity, Impl, OFFSET>,
            EndCommit: EndCommit::<Identity, Impl, OFFSET>,
            BeginAbort: BeginAbort::<Identity, Impl, OFFSET>,
            AbortRecord: AbortRecord::<Identity, Impl, OFFSET>,
            EndAbort: EndAbort::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>() -> ICrmCompensatorVariants_Vtbl {
        unsafe extern "system" fn SetLogControlVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensatorVariants_Impl::SetLogControlVariants(this, windows_core::from_raw_borrowed(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepareVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensatorVariants_Impl::BeginPrepareVariants(this).into()
        }
        unsafe extern "system" fn PrepareRecordVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const core::mem::MaybeUninit<windows_core::VARIANT>, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmCompensatorVariants_Impl::PrepareRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                Ok(ok__) => {
                    core::ptr::write(pbforget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepareVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pboktoprepare: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmCompensatorVariants_Impl::EndPrepareVariants(this) {
                Ok(ok__) => {
                    core::ptr::write(pboktoprepare, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommitVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensatorVariants_Impl::BeginCommitVariants(this, core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn CommitRecordVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const core::mem::MaybeUninit<windows_core::VARIANT>, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmCompensatorVariants_Impl::CommitRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                Ok(ok__) => {
                    core::ptr::write(pbforget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommitVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensatorVariants_Impl::EndCommitVariants(this).into()
        }
        unsafe extern "system" fn BeginAbortVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensatorVariants_Impl::BeginAbortVariants(this, core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn AbortRecordVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const core::mem::MaybeUninit<windows_core::VARIANT>, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmCompensatorVariants_Impl::AbortRecordVariants(this, core::mem::transmute_copy(&plogrecord)) {
                Ok(ok__) => {
                    core::ptr::write(pbforget, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbortVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmCompensatorVariants_Impl::EndAbortVariants(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLogControlVariants: SetLogControlVariants::<Identity, Impl, OFFSET>,
            BeginPrepareVariants: BeginPrepareVariants::<Identity, Impl, OFFSET>,
            PrepareRecordVariants: PrepareRecordVariants::<Identity, Impl, OFFSET>,
            EndPrepareVariants: EndPrepareVariants::<Identity, Impl, OFFSET>,
            BeginCommitVariants: BeginCommitVariants::<Identity, Impl, OFFSET>,
            CommitRecordVariants: CommitRecordVariants::<Identity, Impl, OFFSET>,
            EndCommitVariants: EndCommitVariants::<Identity, Impl, OFFSET>,
            BeginAbortVariants: BeginAbortVariants::<Identity, Impl, OFFSET>,
            AbortRecordVariants: AbortRecordVariants::<Identity, Impl, OFFSET>,
            EndAbortVariants: EndAbortVariants::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>() -> ICrmFormatLogRecords_Vtbl {
        unsafe extern "system" fn GetColumnCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcolumncount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmFormatLogRecords_Impl::GetColumnCount(this) {
                Ok(ok__) => {
                    core::ptr::write(plcolumncount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheaders: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmFormatLogRecords_Impl::GetColumnHeaders(this) {
                Ok(ok__) => {
                    core::ptr::write(pheaders, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmFormatLogRecords_Impl::GetColumn(this, core::mem::transmute(&crmlogrec)) {
                Ok(ok__) => {
                    core::ptr::write(pformattedlogrecord, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecord: core::mem::MaybeUninit<windows_core::VARIANT>, pformattedlogrecord: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmFormatLogRecords_Impl::GetColumnVariants(this, core::mem::transmute(&logrecord)) {
                Ok(ok__) => {
                    core::ptr::write(pformattedlogrecord, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetColumnVariants: GetColumnVariants::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>() -> ICrmLogControl_Vtbl {
        unsafe extern "system" fn TransactionUOW<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmLogControl_Impl::TransactionUOW(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCompensator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpcwstrprogidcompensator: windows_core::PCWSTR, lpcwstrdescription: windows_core::PCWSTR, lcrmregflags: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmLogControl_Impl::RegisterCompensator(this, core::mem::transmute(&lpcwstrprogidcompensator), core::mem::transmute(&lpcwstrdescription), core::mem::transmute_copy(&lcrmregflags)).into()
        }
        unsafe extern "system" fn WriteLogRecordVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogrecord: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmLogControl_Impl::WriteLogRecordVariants(this, core::mem::transmute_copy(&plogrecord)).into()
        }
        unsafe extern "system" fn ForceLog<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmLogControl_Impl::ForceLog(this).into()
        }
        unsafe extern "system" fn ForgetLogRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmLogControl_Impl::ForgetLogRecord(this).into()
        }
        unsafe extern "system" fn ForceTransactionToAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmLogControl_Impl::ForceTransactionToAbort(this).into()
        }
        unsafe extern "system" fn WriteLogRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmLogControl_Impl::WriteLogRecord(this, core::mem::transmute_copy(&rgblob), core::mem::transmute_copy(&cblob)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransactionUOW: TransactionUOW::<Identity, Impl, OFFSET>,
            RegisterCompensator: RegisterCompensator::<Identity, Impl, OFFSET>,
            WriteLogRecordVariants: WriteLogRecordVariants::<Identity, Impl, OFFSET>,
            ForceLog: ForceLog::<Identity, Impl, OFFSET>,
            ForgetLogRecord: ForgetLogRecord::<Identity, Impl, OFFSET>,
            ForceTransactionToAbort: ForceTransactionToAbort::<Identity, Impl, OFFSET>,
            WriteLogRecord: WriteLogRecord::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>() -> ICrmMonitor_Vtbl {
        unsafe extern "system" fn GetClerks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclerks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitor_Impl::GetClerks(this) {
                Ok(ok__) => {
                    core::ptr::write(pclerks, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldClerk<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitor_Impl::HoldClerk(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClerks: GetClerks::<Identity, Impl, OFFSET>,
            HoldClerk: HoldClerk::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>() -> ICrmMonitorClerks_Vtbl {
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorClerks_Impl::Item(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorClerks_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorClerks_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgIdCompensator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorClerks_Impl::ProgIdCompensator(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorClerks_Impl::Description(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionUOW<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorClerks_Impl::TransactionUOW(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorClerks_Impl::ActivityId(this, core::mem::transmute(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ProgIdCompensator: ProgIdCompensator::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            TransactionUOW: TransactionUOW::<Identity, Impl, OFFSET>,
            ActivityId: ActivityId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>() -> ICrmMonitorLogRecords_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorLogRecords_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut CrmTransactionState) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorLogRecords_Impl::TransactionState(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StructuredRecords<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorLogRecords_Impl::StructuredRecords(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogRecord<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICrmMonitorLogRecords_Impl::GetLogRecord(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcrmlogrec)).into()
        }
        unsafe extern "system" fn GetLogRecordVariants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexnumber: core::mem::MaybeUninit<windows_core::VARIANT>, plogrecord: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICrmMonitorLogRecords_Impl::GetLogRecordVariants(this, core::mem::transmute(&indexnumber)) {
                Ok(ok__) => {
                    core::ptr::write(plogrecord, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            TransactionState: TransactionState::<Identity, Impl, OFFSET>,
            StructuredRecords: StructuredRecords::<Identity, Impl, OFFSET>,
            GetLogRecord: GetLogRecord::<Identity, Impl, OFFSET>,
            GetLogRecordVariants: GetLogRecordVariants::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>() -> IDispenserDriver_Vtbl {
        unsafe extern "system" fn CreateResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDispenserDriver_Impl::CreateResource(this, core::mem::transmute_copy(&restypid), core::mem::transmute_copy(&presid), core::mem::transmute_copy(&psecsfreebeforedestroy)).into()
        }
        unsafe extern "system" fn RateResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDispenserDriver_Impl::RateResource(this, core::mem::transmute_copy(&restypid), core::mem::transmute_copy(&resid), core::mem::transmute_copy(&frequirestransactionenlistment), core::mem::transmute_copy(&prating)).into()
        }
        unsafe extern "system" fn EnlistResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize, transid: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDispenserDriver_Impl::EnlistResource(this, core::mem::transmute_copy(&resid), core::mem::transmute_copy(&transid)).into()
        }
        unsafe extern "system" fn ResetResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDispenserDriver_Impl::ResetResource(this, core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDispenserDriver_Impl::DestroyResource(this, core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResourceS<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resid: *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDispenserDriver_Impl::DestroyResourceS(this, core::mem::transmute_copy(&resid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateResource: CreateResource::<Identity, Impl, OFFSET>,
            RateResource: RateResource::<Identity, Impl, OFFSET>,
            EnlistResource: EnlistResource::<Identity, Impl, OFFSET>,
            ResetResource: ResetResource::<Identity, Impl, OFFSET>,
            DestroyResource: DestroyResource::<Identity, Impl, OFFSET>,
            DestroyResourceS: DestroyResourceS::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>() -> IDispenserManager_Vtbl {
        unsafe extern "system" fn RegisterDispenser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__idispensermanager0000: *mut core::ffi::c_void, szdispensername: windows_core::PCWSTR, __midl__idispensermanager0001: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDispenserManager_Impl::RegisterDispenser(this, windows_core::from_raw_borrowed(&__midl__idispensermanager0000), core::mem::transmute(&szdispensername)) {
                Ok(ok__) => {
                    core::ptr::write(__midl__idispensermanager0001, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDispenserManager_Impl::GetContext(this, core::mem::transmute_copy(&__midl__idispensermanager0002), core::mem::transmute_copy(&__midl__idispensermanager0003)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDispenser: RegisterDispenser::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>() -> IEnumNames_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgname: *mut core::mem::MaybeUninit<windows_core::BSTR>, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNames_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgname), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNames_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNames_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNames_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>() -> IEventServerTrace_Vtbl {
        unsafe extern "system" fn StartTraceGuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidevent: core::mem::MaybeUninit<windows_core::BSTR>, bstrguidfilter: core::mem::MaybeUninit<windows_core::BSTR>, lpidfilter: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventServerTrace_Impl::StartTraceGuid(this, core::mem::transmute(&bstrguidevent), core::mem::transmute(&bstrguidfilter), core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn StopTraceGuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidevent: core::mem::MaybeUninit<windows_core::BSTR>, bstrguidfilter: core::mem::MaybeUninit<windows_core::BSTR>, lpidfilter: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventServerTrace_Impl::StopTraceGuid(this, core::mem::transmute(&bstrguidevent), core::mem::transmute(&bstrguidfilter), core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn EnumTraceGuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventServerTrace_Impl::EnumTraceGuid(this, core::mem::transmute_copy(&plcntguids), core::mem::transmute_copy(&pbstrguidlist)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartTraceGuid: StartTraceGuid::<Identity, Impl, OFFSET>,
            StopTraceGuid: StopTraceGuid::<Identity, Impl, OFFSET>,
            EnumTraceGuid: EnumTraceGuid::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>() -> IGetAppTrackerData_Vtbl {
        unsafe extern "system" fn GetApplicationProcesses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGetAppTrackerData_Impl::GetApplicationProcesses(this, core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&applicationid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numapplicationprocesses), core::mem::transmute_copy(&applicationprocesses)).into()
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGetAppTrackerData_Impl::GetApplicationProcessDetails(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&summary), core::mem::transmute_copy(&statistics), core::mem::transmute_copy(&recycleinfo), core::mem::transmute_copy(&anycomponentshangmonitored)).into()
        }
        unsafe extern "system" fn GetApplicationsInProcess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGetAppTrackerData_Impl::GetApplicationsInProcess(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numapplicationsinprocess), core::mem::transmute_copy(&applications)).into()
        }
        unsafe extern "system" fn GetComponentsInProcess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, partitionid: *const windows_core::GUID, applicationid: *const windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGetAppTrackerData_Impl::GetComponentsInProcess(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&partitionid), core::mem::transmute_copy(&applicationid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numcomponentsinprocess), core::mem::transmute_copy(&components)).into()
        }
        unsafe extern "system" fn GetComponentDetails<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, applicationinstanceid: *const windows_core::GUID, processid: u32, clsid: *const windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGetAppTrackerData_Impl::GetComponentDetails(this, core::mem::transmute_copy(&applicationinstanceid), core::mem::transmute_copy(&processid), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&summary), core::mem::transmute_copy(&statistics), core::mem::transmute_copy(&hangmonitorinfo)).into()
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, toplevelcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGetAppTrackerData_Impl::GetTrackerDataAsCollectionObject(this) {
                Ok(ok__) => {
                    core::ptr::write(toplevelcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pollingintervalinseconds: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGetAppTrackerData_Impl::GetSuggestedPollingInterval(this) {
                Ok(ok__) => {
                    core::ptr::write(pollingintervalinseconds, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetApplicationProcesses: GetApplicationProcesses::<Identity, Impl, OFFSET>,
            GetApplicationProcessDetails: GetApplicationProcessDetails::<Identity, Impl, OFFSET>,
            GetApplicationsInProcess: GetApplicationsInProcess::<Identity, Impl, OFFSET>,
            GetComponentsInProcess: GetComponentsInProcess::<Identity, Impl, OFFSET>,
            GetComponentDetails: GetComponentDetails::<Identity, Impl, OFFSET>,
            GetTrackerDataAsCollectionObject: GetTrackerDataAsCollectionObject::<Identity, Impl, OFFSET>,
            GetSuggestedPollingInterval: GetSuggestedPollingInterval::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>() -> IGetContextProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGetContextProperties_Impl::Count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pproperty: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGetContextProperties_Impl::GetProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGetContextProperties_Impl::EnumNames(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumNames: EnumNames::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetSecurityCallContext_Impl, const OFFSET: isize>() -> IGetSecurityCallContext_Vtbl {
        unsafe extern "system" fn GetSecurityCallContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetSecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IGetSecurityCallContext_Impl::GetSecurityCallContext(this) {
                Ok(ok__) => {
                    core::ptr::write(ppobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetSecurityCallContext: GetSecurityCallContext::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>() -> IHolder_Vtbl {
        unsafe extern "system" fn AllocResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHolder_Impl::AllocResource(this, core::mem::transmute_copy(&__midl__iholder0000), core::mem::transmute_copy(&__midl__iholder0001)).into()
        }
        unsafe extern "system" fn FreeResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0002: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHolder_Impl::FreeResource(this, core::mem::transmute_copy(&__midl__iholder0002)).into()
        }
        unsafe extern "system" fn TrackResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0003: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHolder_Impl::TrackResource(this, core::mem::transmute_copy(&__midl__iholder0003)).into()
        }
        unsafe extern "system" fn TrackResourceS<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0004: *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHolder_Impl::TrackResourceS(this, core::mem::transmute_copy(&__midl__iholder0004)).into()
        }
        unsafe extern "system" fn UntrackResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHolder_Impl::UntrackResource(this, core::mem::transmute_copy(&__midl__iholder0005), core::mem::transmute_copy(&__midl__iholder0006)).into()
        }
        unsafe extern "system" fn UntrackResourceS<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHolder_Impl::UntrackResourceS(this, core::mem::transmute_copy(&__midl__iholder0007), core::mem::transmute_copy(&__midl__iholder0008)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHolder_Impl::Close(this).into()
        }
        unsafe extern "system" fn RequestDestroyResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iholder0009: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IHolder_Impl::RequestDestroyResource(this, core::mem::transmute_copy(&__midl__iholder0009)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocResource: AllocResource::<Identity, Impl, OFFSET>,
            FreeResource: FreeResource::<Identity, Impl, OFFSET>,
            TrackResource: TrackResource::<Identity, Impl, OFFSET>,
            TrackResourceS: TrackResourceS::<Identity, Impl, OFFSET>,
            UntrackResource: UntrackResource::<Identity, Impl, OFFSET>,
            UntrackResourceS: UntrackResourceS::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            RequestDestroyResource: RequestDestroyResource::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>() -> ILBEvents_Vtbl {
        unsafe extern "system" fn TargetUp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsideng: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILBEvents_Impl::TargetUp(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrclsideng)).into()
        }
        unsafe extern "system" fn TargetDown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, bstrclsideng: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILBEvents_Impl::TargetDown(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrclsideng)).into()
        }
        unsafe extern "system" fn EngineDefined<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, varpropvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrclsideng: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILBEvents_Impl::EngineDefined(this, core::mem::transmute(&bstrpropname), core::mem::transmute_copy(&varpropvalue), core::mem::transmute(&bstrclsideng)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TargetUp: TargetUp::<Identity, Impl, OFFSET>,
            TargetDown: TargetDown::<Identity, Impl, OFFSET>,
            EngineDefined: EngineDefined::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>() -> IMTSActivity_Vtbl {
        unsafe extern "system" fn SynchronousCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMTSActivity_Impl::SynchronousCall(this, windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn AsyncCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMTSActivity_Impl::AsyncCall(this, windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn Reserved1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMTSActivity_Impl::Reserved1(this)
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMTSActivity_Impl::BindToCurrentThread(this).into()
        }
        unsafe extern "system" fn UnbindFromThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMTSActivity_Impl::UnbindFromThread(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, Impl, OFFSET>,
            AsyncCall: AsyncCall::<Identity, Impl, OFFSET>,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, Impl, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSCall_Impl, const OFFSET: isize>() -> IMTSCall_Vtbl {
        unsafe extern "system" fn OnCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMTSCall_Impl::OnCall(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSLocator_Impl, const OFFSET: isize>() -> IMTSLocator_Vtbl {
        unsafe extern "system" fn GetEventDispatcher<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMTSLocator_Impl::GetEventDispatcher(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetEventDispatcher: GetEventDispatcher::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>() -> IManagedActivationEvents_Vtbl {
        unsafe extern "system" fn CreateManagedStub<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void, fdist: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IManagedActivationEvents_Impl::CreateManagedStub(this, windows_core::from_raw_borrowed(&pinfo), core::mem::transmute_copy(&fdist)).into()
        }
        unsafe extern "system" fn DestroyManagedStub<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IManagedActivationEvents_Impl::DestroyManagedStub(this, windows_core::from_raw_borrowed(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateManagedStub: CreateManagedStub::<Identity, Impl, OFFSET>,
            DestroyManagedStub: DestroyManagedStub::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>() -> IManagedObjectInfo_Vtbl {
        unsafe extern "system" fn GetIUnknown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IManagedObjectInfo_Impl::GetIUnknown(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIObjectControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IManagedObjectInfo_Impl::GetIObjectControl(this) {
                Ok(ok__) => {
                    core::ptr::write(pctrl, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPool<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IManagedObjectInfo_Impl::SetInPool(this, core::mem::transmute_copy(&binpool), windows_core::from_raw_borrowed(&ppooledobj)).into()
        }
        unsafe extern "system" fn SetWrapperStrength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IManagedObjectInfo_Impl::SetWrapperStrength(this, core::mem::transmute_copy(&bstrong)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIUnknown: GetIUnknown::<Identity, Impl, OFFSET>,
            GetIObjectControl: GetIObjectControl::<Identity, Impl, OFFSET>,
            SetInPool: SetInPool::<Identity, Impl, OFFSET>,
            SetWrapperStrength: SetWrapperStrength::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedPoolAction_Impl, const OFFSET: isize>() -> IManagedPoolAction_Vtbl {
        unsafe extern "system" fn LastRelease<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedPoolAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IManagedPoolAction_Impl::LastRelease(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LastRelease: LastRelease::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedPooledObj_Impl, const OFFSET: isize>() -> IManagedPooledObj_Vtbl {
        unsafe extern "system" fn SetHeld<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedPooledObj_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IManagedPooledObj_Impl::SetHeld(this, core::mem::transmute_copy(&m_bheld)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetHeld: SetHeld::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>() -> IMessageMover_Vtbl {
        unsafe extern "system" fn SourcePath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMessageMover_Impl::SourcePath(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourcePath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMessageMover_Impl::SetSourcePath(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn DestPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMessageMover_Impl::DestPath(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestPath<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMessageMover_Impl::SetDestPath(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn CommitBatchSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMessageMover_Impl::CommitBatchSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitBatchSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMessageMover_Impl::SetCommitBatchSize(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MoveMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmessagesmoved: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMessageMover_Impl::MoveMessages(this) {
                Ok(ok__) => {
                    core::ptr::write(plmessagesmoved, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SourcePath: SourcePath::<Identity, Impl, OFFSET>,
            SetSourcePath: SetSourcePath::<Identity, Impl, OFFSET>,
            DestPath: DestPath::<Identity, Impl, OFFSET>,
            SetDestPath: SetDestPath::<Identity, Impl, OFFSET>,
            CommitBatchSize: CommitBatchSize::<Identity, Impl, OFFSET>,
            SetCommitBatchSize: SetCommitBatchSize::<Identity, Impl, OFFSET>,
            MoveMessages: MoveMessages::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>() -> IMtsEventInfo_Vtbl {
        unsafe extern "system" fn Names<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEventInfo_Impl::Names(this) {
                Ok(ok__) => {
                    core::ptr::write(punk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEventInfo_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(sdisplayname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sguideventid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEventInfo_Impl::EventID(this) {
                Ok(ok__) => {
                    core::ptr::write(sguideventid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEventInfo_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(lcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, skey: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEventInfo_Impl::get_Value(this, core::mem::transmute(&skey)) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Names: Names::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            EventID: EventID::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Value: get_Value::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>() -> IMtsEvents_Vtbl {
        unsafe extern "system" fn PackageName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEvents_Impl::PackageName(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageGuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEvents_Impl::PackageGuid(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vevent: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMtsEvents_Impl::PostEvent(this, core::mem::transmute_copy(&vevent)).into()
        }
        unsafe extern "system" fn FireEvents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEvents_Impl::FireEvents(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsEvents_Impl::GetProcessID(this) {
                Ok(ok__) => {
                    core::ptr::write(id, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PackageName: PackageName::<Identity, Impl, OFFSET>,
            PackageGuid: PackageGuid::<Identity, Impl, OFFSET>,
            PostEvent: PostEvent::<Identity, Impl, OFFSET>,
            FireEvents: FireEvents::<Identity, Impl, OFFSET>,
            GetProcessID: GetProcessID::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>() -> IMtsGrp_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsGrp_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMtsGrp_Impl::Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppunkdispatcher, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMtsGrp_Impl::Refresh(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>() -> IObjPool_Vtbl {
        unsafe extern "system" fn Reserved1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjPool_Impl::Reserved1(this)
        }
        unsafe extern "system" fn Reserved2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjPool_Impl::Reserved2(this)
        }
        unsafe extern "system" fn Reserved3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjPool_Impl::Reserved3(this)
        }
        unsafe extern "system" fn Reserved4<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjPool_Impl::Reserved4(this)
        }
        unsafe extern "system" fn PutEndTx<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobj: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjPool_Impl::PutEndTx(this, windows_core::from_raw_borrowed(&pobj))
        }
        unsafe extern "system" fn Reserved5<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjPool_Impl::Reserved5(this)
        }
        unsafe extern "system" fn Reserved6<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjPool_Impl::Reserved6(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            Reserved3: Reserved3::<Identity, Impl, OFFSET>,
            Reserved4: Reserved4::<Identity, Impl, OFFSET>,
            PutEndTx: PutEndTx::<Identity, Impl, OFFSET>,
            Reserved5: Reserved5::<Identity, Impl, OFFSET>,
            Reserved6: Reserved6::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstruct_Impl, const OFFSET: isize>() -> IObjectConstruct_Vtbl {
        unsafe extern "system" fn Construct<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstruct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctorobj: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectConstruct_Impl::Construct(this, windows_core::from_raw_borrowed(&pctorobj)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Construct: Construct::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstructString_Impl, const OFFSET: isize>() -> IObjectConstructString_Vtbl {
        unsafe extern "system" fn ConstructString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstructString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectConstructString_Impl::ConstructString(this, core::mem::transmute_copy(&pval)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ConstructString: ConstructString::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>() -> IObjectContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContext_Impl::CreateInstance(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn SetComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContext_Impl::SetComplete(this).into()
        }
        unsafe extern "system" fn SetAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContext_Impl::SetAbort(this).into()
        }
        unsafe extern "system" fn EnableCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContext_Impl::EnableCommit(this).into()
        }
        unsafe extern "system" fn DisableCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContext_Impl::DisableCommit(this).into()
        }
        unsafe extern "system" fn IsInTransaction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContext_Impl::IsInTransaction(this)
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContext_Impl::IsSecurityEnabled(this)
        }
        unsafe extern "system" fn IsCallerInRole<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole), core::mem::transmute_copy(&pfisinrole)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            SetComplete: SetComplete::<Identity, Impl, OFFSET>,
            SetAbort: SetAbort::<Identity, Impl, OFFSET>,
            EnableCommit: EnableCommit::<Identity, Impl, OFFSET>,
            DisableCommit: DisableCommit::<Identity, Impl, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextActivity_Impl, const OFFSET: isize>() -> IObjectContextActivity_Vtbl {
        unsafe extern "system" fn GetActivityId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextActivity_Impl::GetActivityId(this, core::mem::transmute_copy(&pguid)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetActivityId: GetActivityId::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>() -> IObjectContextInfo_Vtbl {
        unsafe extern "system" fn IsInTransaction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextInfo_Impl::IsInTransaction(this)
        }
        unsafe extern "system" fn GetTransaction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptrans: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IObjectContextInfo_Impl::GetTransaction(this) {
                Ok(ok__) => {
                    core::ptr::write(pptrans, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextInfo_Impl::GetTransactionId(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetActivityId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextInfo_Impl::GetActivityId(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetContextId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextInfo_Impl::GetContextId(this, core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, Impl, OFFSET>,
            GetActivityId: GetActivityId::<Identity, Impl, OFFSET>,
            GetContextId: GetContextId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>() -> IObjectContextInfo2_Vtbl {
        unsafe extern "system" fn GetPartitionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextInfo2_Impl::GetPartitionId(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextInfo2_Impl::GetApplicationId(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextInfo2_Impl::GetApplicationInstanceId(this, core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base__: IObjectContextInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPartitionId: GetPartitionId::<Identity, Impl, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, Impl, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextTip_Impl, const OFFSET: isize>() -> IObjectContextTip_Vtbl {
        unsafe extern "system" fn GetTipUrl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextTip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptipurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectContextTip_Impl::GetTipUrl(this, core::mem::transmute_copy(&ptipurl)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTipUrl: GetTipUrl::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>() -> IObjectControl_Vtbl {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectControl_Impl::Activate(this).into()
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectControl_Impl::Deactivate(this)
        }
        unsafe extern "system" fn CanBePooled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IObjectControl_Impl::CanBePooled(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CanBePooled: CanBePooled::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>() -> IPlaybackControl_Vtbl {
        unsafe extern "system" fn FinalClientRetry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPlaybackControl_Impl::FinalClientRetry(this).into()
        }
        unsafe extern "system" fn FinalServerRetry<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPlaybackControl_Impl::FinalServerRetry(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinalClientRetry: FinalClientRetry::<Identity, Impl, OFFSET>,
            FinalServerRetry: FinalServerRetry::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPoolManager_Impl, const OFFSET: isize>() -> IPoolManager_Vtbl {
        unsafe extern "system" fn ShutdownPool<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPoolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidorprogid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPoolManager_Impl::ShutdownPool(this, core::mem::transmute(&clsidorprogid)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ShutdownPool: ShutdownPool::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>() -> IProcessInitializer_Vtbl {
        unsafe extern "system" fn Startup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkprocesscontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProcessInitializer_Impl::Startup(this, windows_core::from_raw_borrowed(&punkprocesscontrol)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProcessInitializer_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Startup: Startup::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>() -> ISecurityCallContext_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallContext_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallContext_Impl::get_Item(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallContext_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole)) {
                Ok(ok__) => {
                    core::ptr::write(pfinrole, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallContext_Impl::IsSecurityEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pfisenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserInRole<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallContext_Impl::IsUserInRole(this, core::mem::transmute_copy(&puser), core::mem::transmute(&bstrrole)) {
                Ok(ok__) => {
                    core::ptr::write(pfinrole, core::mem::transmute(ok__));
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
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsUserInRole: IsUserInRole::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>() -> ISecurityCallersColl_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallersColl_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallersColl_Impl::get_Item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    core::ptr::write(pobj, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityCallersColl_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>() -> ISecurityIdentityColl_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityIdentityColl_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityIdentityColl_Impl::get_Item(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISecurityIdentityColl_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
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
        iid == &<ISecurityIdentityColl as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait ISecurityProperty_Impl: Sized {
    fn GetDirectCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> windows_core::Result<()>;
    fn GetOriginalCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> windows_core::Result<()>;
    fn GetDirectCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> windows_core::Result<()>;
    fn GetOriginalCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> windows_core::Result<()>;
    fn ReleaseSID(&self, psid: super::super::Foundation::PSID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISecurityProperty {}
impl ISecurityProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>() -> ISecurityProperty_Vtbl {
        unsafe extern "system" fn GetDirectCreatorSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISecurityProperty_Impl::GetDirectCreatorSID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISecurityProperty_Impl::GetOriginalCreatorSID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetDirectCallerSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISecurityProperty_Impl::GetDirectCallerSID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCallerSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISecurityProperty_Impl::GetOriginalCallerSID(this, core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn ReleaseSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: super::super::Foundation::PSID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISecurityProperty_Impl::ReleaseSID(this, core::mem::transmute_copy(&psid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDirectCreatorSID: GetDirectCreatorSID::<Identity, Impl, OFFSET>,
            GetOriginalCreatorSID: GetOriginalCreatorSID::<Identity, Impl, OFFSET>,
            GetDirectCallerSID: GetDirectCallerSID::<Identity, Impl, OFFSET>,
            GetOriginalCallerSID: GetOriginalCallerSID::<Identity, Impl, OFFSET>,
            ReleaseSID: ReleaseSID::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>() -> ISelectCOMLBServer_Vtbl {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISelectCOMLBServer_Impl::Init(this).into()
        }
        unsafe extern "system" fn GetLBServer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISelectCOMLBServer_Impl::GetLBServer(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetLBServer: GetLBServer::<Identity, Impl, OFFSET>,
        }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>() -> ISendMethodEvents_Vtbl {
        unsafe extern "system" fn SendMethodCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISendMethodEvents_Impl::SendMethodCall(this, core::mem::transmute_copy(&pidentity), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&dwmeth)).into()
        }
        unsafe extern "system" fn SendMethodReturn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentity: *const core::ffi::c_void, riid: *const windows_core::GUID, dwmeth: u32, hrcall: windows_core::HRESULT, hrserver: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISendMethodEvents_Impl::SendMethodReturn(this, core::mem::transmute_copy(&pidentity), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&dwmeth), core::mem::transmute_copy(&hrcall), core::mem::transmute_copy(&hrserver)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendMethodCall: SendMethodCall::<Identity, Impl, OFFSET>,
            SendMethodReturn: SendMethodReturn::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>() -> IServiceActivity_Vtbl {
        unsafe extern "system" fn SynchronousCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piservicecall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceActivity_Impl::SynchronousCall(this, windows_core::from_raw_borrowed(&piservicecall)).into()
        }
        unsafe extern "system" fn AsynchronousCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piservicecall: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceActivity_Impl::AsynchronousCall(this, windows_core::from_raw_borrowed(&piservicecall)).into()
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceActivity_Impl::BindToCurrentThread(this).into()
        }
        unsafe extern "system" fn UnbindFromThread<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceActivity_Impl::UnbindFromThread(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, Impl, OFFSET>,
            AsynchronousCall: AsynchronousCall::<Identity, Impl, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, Impl, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceCall_Impl, const OFFSET: isize>() -> IServiceCall_Vtbl {
        unsafe extern "system" fn OnCall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceCall_Impl::OnCall(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>() -> IServiceComTIIntrinsicsConfig_Vtbl {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceComTIIntrinsicsConfig_Impl::ComTIIntrinsicsConfig(this, core::mem::transmute_copy(&comtiintrinsicsconfig)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ComTIIntrinsicsConfig: ComTIIntrinsicsConfig::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>() -> IServiceIISIntrinsicsConfig_Vtbl {
        unsafe extern "system" fn IISIntrinsicsConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceIISIntrinsicsConfig_Impl::IISIntrinsicsConfig(this, core::mem::transmute_copy(&iisintrinsicsconfig)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IISIntrinsicsConfig: IISIntrinsicsConfig::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceInheritanceConfig_Impl, const OFFSET: isize>() -> IServiceInheritanceConfig_Vtbl {
        unsafe extern "system" fn ContainingContextTreatment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceInheritanceConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceInheritanceConfig_Impl::ContainingContextTreatment(this, core::mem::transmute_copy(&inheritanceconfig)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ContainingContextTreatment: ContainingContextTreatment::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>() -> IServicePartitionConfig_Vtbl {
        unsafe extern "system" fn PartitionConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePartitionConfig_Impl::PartitionConfig(this, core::mem::transmute_copy(&partitionconfig)).into()
        }
        unsafe extern "system" fn PartitionID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpartitionid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePartitionConfig_Impl::PartitionID(this, core::mem::transmute_copy(&guidpartitionid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PartitionConfig: PartitionConfig::<Identity, Impl, OFFSET>,
            PartitionID: PartitionID::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>() -> IServicePool_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoolconfig: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePool_Impl::Initialize(this, windows_core::from_raw_borrowed(&ppoolconfig)).into()
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePool_Impl::GetObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePool_Impl::Shutdown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>() -> IServicePoolConfig_Vtbl {
        unsafe extern "system" fn SetMaxPoolSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxpool: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::SetMaxPoolSize(this, core::mem::transmute_copy(&dwmaxpool)).into()
        }
        unsafe extern "system" fn MaxPoolSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxpool: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::MaxPoolSize(this, core::mem::transmute_copy(&pdwmaxpool)).into()
        }
        unsafe extern "system" fn SetMinPoolSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminpool: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::SetMinPoolSize(this, core::mem::transmute_copy(&dwminpool)).into()
        }
        unsafe extern "system" fn MinPoolSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminpool: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::MinPoolSize(this, core::mem::transmute_copy(&pdwminpool)).into()
        }
        unsafe extern "system" fn SetCreationTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcreationtimeout: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::SetCreationTimeout(this, core::mem::transmute_copy(&dwcreationtimeout)).into()
        }
        unsafe extern "system" fn CreationTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcreationtimeout: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::CreationTimeout(this, core::mem::transmute_copy(&pdwcreationtimeout)).into()
        }
        unsafe extern "system" fn SetTransactionAffinity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::SetTransactionAffinity(this, core::mem::transmute_copy(&ftxaffinity)).into()
        }
        unsafe extern "system" fn TransactionAffinity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::TransactionAffinity(this, core::mem::transmute_copy(&pftxaffinity)).into()
        }
        unsafe extern "system" fn SetClassFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfactory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServicePoolConfig_Impl::SetClassFactory(this, windows_core::from_raw_borrowed(&pfactory)).into()
        }
        unsafe extern "system" fn ClassFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IServicePoolConfig_Impl::ClassFactory(this) {
                Ok(ok__) => {
                    core::ptr::write(pfactory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMaxPoolSize: SetMaxPoolSize::<Identity, Impl, OFFSET>,
            MaxPoolSize: MaxPoolSize::<Identity, Impl, OFFSET>,
            SetMinPoolSize: SetMinPoolSize::<Identity, Impl, OFFSET>,
            MinPoolSize: MinPoolSize::<Identity, Impl, OFFSET>,
            SetCreationTimeout: SetCreationTimeout::<Identity, Impl, OFFSET>,
            CreationTimeout: CreationTimeout::<Identity, Impl, OFFSET>,
            SetTransactionAffinity: SetTransactionAffinity::<Identity, Impl, OFFSET>,
            TransactionAffinity: TransactionAffinity::<Identity, Impl, OFFSET>,
            SetClassFactory: SetClassFactory::<Identity, Impl, OFFSET>,
            ClassFactory: ClassFactory::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>() -> IServiceSxsConfig_Vtbl {
        unsafe extern "system" fn SxsConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scsconfig: CSC_SxsConfig) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceSxsConfig_Impl::SxsConfig(this, core::mem::transmute_copy(&scsconfig)).into()
        }
        unsafe extern "system" fn SxsName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsxsname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceSxsConfig_Impl::SxsName(this, core::mem::transmute(&szsxsname)).into()
        }
        unsafe extern "system" fn SxsDirectory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szsxsdirectory: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceSxsConfig_Impl::SxsDirectory(this, core::mem::transmute(&szsxsdirectory)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SxsConfig: SxsConfig::<Identity, Impl, OFFSET>,
            SxsName: SxsName::<Identity, Impl, OFFSET>,
            SxsDirectory: SxsDirectory::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSynchronizationConfig_Impl, const OFFSET: isize>() -> IServiceSynchronizationConfig_Vtbl {
        unsafe extern "system" fn ConfigureSynchronization<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSynchronizationConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceSynchronizationConfig_Impl::ConfigureSynchronization(this, core::mem::transmute_copy(&synchconfig)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConfigureSynchronization: ConfigureSynchronization::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSysTxnConfig_Impl, const OFFSET: isize>() -> IServiceSysTxnConfig_Vtbl {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSysTxnConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxproxy: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceSysTxnConfig_Impl::ConfigureBYOTSysTxn(this, windows_core::from_raw_borrowed(&ptxproxy)).into()
        }
        Self { base__: IServiceTransactionConfig_Vtbl::new::<Identity, Impl, OFFSET>(), ConfigureBYOTSysTxn: ConfigureBYOTSysTxn::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>() -> IServiceThreadPoolConfig_Vtbl {
        unsafe extern "system" fn SelectThreadPool<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadpool: CSC_ThreadPool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceThreadPoolConfig_Impl::SelectThreadPool(this, core::mem::transmute_copy(&threadpool)).into()
        }
        unsafe extern "system" fn SetBindingInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, binding: CSC_Binding) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceThreadPoolConfig_Impl::SetBindingInfo(this, core::mem::transmute_copy(&binding)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SelectThreadPool: SelectThreadPool::<Identity, Impl, OFFSET>,
            SetBindingInfo: SetBindingInfo::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTrackerConfig_Impl, const OFFSET: isize>() -> IServiceTrackerConfig_Vtbl {
        unsafe extern "system" fn TrackerConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTrackerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: windows_core::PCWSTR, sztrackerctxname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceTrackerConfig_Impl::TrackerConfig(this, core::mem::transmute_copy(&trackerconfig), core::mem::transmute(&sztrackerappname), core::mem::transmute(&sztrackerctxname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TrackerConfig: TrackerConfig::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfig_Impl, const OFFSET: isize>() -> IServiceTransactionConfig_Vtbl {
        unsafe extern "system" fn ConfigureBYOT<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitxbyot: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceTransactionConfig_Impl::ConfigureBYOT(this, windows_core::from_raw_borrowed(&pitxbyot)).into()
        }
        Self { base__: IServiceTransactionConfigBase_Vtbl::new::<Identity, Impl, OFFSET>(), ConfigureBYOT: ConfigureBYOT::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>() -> IServiceTransactionConfigBase_Vtbl {
        unsafe extern "system" fn ConfigureTransaction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceTransactionConfigBase_Impl::ConfigureTransaction(this, core::mem::transmute_copy(&transactionconfig)).into()
        }
        unsafe extern "system" fn IsolationLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceTransactionConfigBase_Impl::IsolationLevel(this, core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn TransactionTimeout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultimeoutsec: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceTransactionConfigBase_Impl::TransactionTimeout(this, core::mem::transmute_copy(&ultimeoutsec)).into()
        }
        unsafe extern "system" fn BringYourOwnTransaction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztipurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceTransactionConfigBase_Impl::BringYourOwnTransaction(this, core::mem::transmute(&sztipurl)).into()
        }
        unsafe extern "system" fn NewTransactionDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztxdesc: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IServiceTransactionConfigBase_Impl::NewTransactionDescription(this, core::mem::transmute(&sztxdesc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConfigureTransaction: ConfigureTransaction::<Identity, Impl, OFFSET>,
            IsolationLevel: IsolationLevel::<Identity, Impl, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, Impl, OFFSET>,
            BringYourOwnTransaction: BringYourOwnTransaction::<Identity, Impl, OFFSET>,
            NewTransactionDescription: NewTransactionDescription::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>() -> ISharedProperty_Vtbl {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISharedProperty_Impl::Value(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISharedProperty_Impl::SetValue(this, core::mem::transmute(&val)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>() -> ISharedPropertyGroup_Vtbl {
        unsafe extern "system" fn CreatePropertyByPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISharedPropertyGroup_Impl::CreatePropertyByPosition(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn get_PropertyByPosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISharedPropertyGroup_Impl::get_PropertyByPosition(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(ppproperty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISharedPropertyGroup_Impl::CreateProperty(this, core::mem::transmute(&name), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn get_Property<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, ppproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISharedPropertyGroup_Impl::get_Property(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(ppproperty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreatePropertyByPosition: CreatePropertyByPosition::<Identity, Impl, OFFSET>,
            get_PropertyByPosition: get_PropertyByPosition::<Identity, Impl, OFFSET>,
            CreateProperty: CreateProperty::<Identity, Impl, OFFSET>,
            get_Property: get_Property::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>() -> ISharedPropertyGroupManager_Vtbl {
        unsafe extern "system" fn CreatePropertyGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISharedPropertyGroupManager_Impl::CreatePropertyGroup(this, core::mem::transmute(&name), core::mem::transmute_copy(&dwisomode), core::mem::transmute_copy(&dwrelmode), core::mem::transmute_copy(&fexists), core::mem::transmute_copy(&ppgroup)).into()
        }
        unsafe extern "system" fn get_Group<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISharedPropertyGroupManager_Impl::get_Group(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(ppgroup, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISharedPropertyGroupManager_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(retval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreatePropertyGroup: CreatePropertyGroup::<Identity, Impl, OFFSET>,
            get_Group: get_Group::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>() -> ISystemAppEventData_Vtbl {
        unsafe extern "system" fn Startup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISystemAppEventData_Impl::Startup(this).into()
        }
        unsafe extern "system" fn OnDataChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: core::mem::MaybeUninit<windows_core::BSTR>, dwreason: u32, u64tracehandle: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISystemAppEventData_Impl::OnDataChanged(this, core::mem::transmute_copy(&dwpid), core::mem::transmute_copy(&dwmask), core::mem::transmute_copy(&dwnumbersinks), core::mem::transmute(&bstrdwmethodmask), core::mem::transmute_copy(&dwreason), core::mem::transmute_copy(&u64tracehandle)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Startup: Startup::<Identity, Impl, OFFSET>,
            OnDataChanged: OnDataChanged::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>() -> IThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn GetMaxThreads<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcmaxthreads: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::GetMaxThreads(this, core::mem::transmute_copy(&plcmaxthreads)).into()
        }
        unsafe extern "system" fn GetCurrentThreads<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plccurrentthreads: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::GetCurrentThreads(this, core::mem::transmute_copy(&plccurrentthreads)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcmaxthreads: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::SetMaxThreads(this, core::mem::transmute_copy(&lcmaxthreads)).into()
        }
        unsafe extern "system" fn GetDeleteDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsecdeletedelay: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::GetDeleteDelay(this, core::mem::transmute_copy(&pmsecdeletedelay)).into()
        }
        unsafe extern "system" fn SetDeleteDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msecdeletedelay: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::SetDeleteDelay(this, core::mem::transmute_copy(&msecdeletedelay)).into()
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::GetMaxQueuedRequests(this, core::mem::transmute_copy(&plcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::GetCurrentQueuedRequests(this, core::mem::transmute_copy(&plccurrentqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcmaxqueuedrequests: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::SetMaxQueuedRequests(this, core::mem::transmute_copy(&lcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMinThreads<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcminthreads: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::SetMinThreads(this, core::mem::transmute_copy(&lcminthreads)).into()
        }
        unsafe extern "system" fn SetQueueDepth<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcqueuedepth: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IThreadPoolKnobs_Impl::SetQueueDepth(this, core::mem::transmute_copy(&lcqueuedepth)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxThreads: GetMaxThreads::<Identity, Impl, OFFSET>,
            GetCurrentThreads: GetCurrentThreads::<Identity, Impl, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, Impl, OFFSET>,
            GetDeleteDelay: GetDeleteDelay::<Identity, Impl, OFFSET>,
            SetDeleteDelay: SetDeleteDelay::<Identity, Impl, OFFSET>,
            GetMaxQueuedRequests: GetMaxQueuedRequests::<Identity, Impl, OFFSET>,
            GetCurrentQueuedRequests: GetCurrentQueuedRequests::<Identity, Impl, OFFSET>,
            SetMaxQueuedRequests: SetMaxQueuedRequests::<Identity, Impl, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, Impl, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>() -> ITransactionContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprogid: core::mem::MaybeUninit<windows_core::BSTR>, pobject: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITransactionContext_Impl::CreateInstance(this, core::mem::transmute(&pszprogid)) {
                Ok(ok__) => {
                    core::ptr::write(pobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionContext_Impl::Commit(this).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionContext_Impl::Abort(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>() -> ITransactionContextEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, pobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionContextEx_Impl::CreateInstance(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pobject)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionContextEx_Impl::Commit(this).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionContextEx_Impl::Abort(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>() -> ITransactionProperty_Vtbl {
        unsafe extern "system" fn Reserved1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved1(this)
        }
        unsafe extern "system" fn Reserved2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved2(this)
        }
        unsafe extern "system" fn Reserved3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved3(this)
        }
        unsafe extern "system" fn Reserved4<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved4(this)
        }
        unsafe extern "system" fn Reserved5<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved5(this)
        }
        unsafe extern "system" fn Reserved6<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved6(this)
        }
        unsafe extern "system" fn Reserved7<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved7(this)
        }
        unsafe extern "system" fn Reserved8<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved8(this)
        }
        unsafe extern "system" fn Reserved9<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved9(this)
        }
        unsafe extern "system" fn GetTransactionResourcePool<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptxpool: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITransactionProperty_Impl::GetTransactionResourcePool(this) {
                Ok(ok__) => {
                    core::ptr::write(pptxpool, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved10<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved10(this)
        }
        unsafe extern "system" fn Reserved11<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved11(this)
        }
        unsafe extern "system" fn Reserved12<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved12(this)
        }
        unsafe extern "system" fn Reserved13<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved13(this)
        }
        unsafe extern "system" fn Reserved14<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved14(this)
        }
        unsafe extern "system" fn Reserved15<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved15(this)
        }
        unsafe extern "system" fn Reserved16<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved16(this)
        }
        unsafe extern "system" fn Reserved17<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProperty_Impl::Reserved17(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            Reserved3: Reserved3::<Identity, Impl, OFFSET>,
            Reserved4: Reserved4::<Identity, Impl, OFFSET>,
            Reserved5: Reserved5::<Identity, Impl, OFFSET>,
            Reserved6: Reserved6::<Identity, Impl, OFFSET>,
            Reserved7: Reserved7::<Identity, Impl, OFFSET>,
            Reserved8: Reserved8::<Identity, Impl, OFFSET>,
            Reserved9: Reserved9::<Identity, Impl, OFFSET>,
            GetTransactionResourcePool: GetTransactionResourcePool::<Identity, Impl, OFFSET>,
            Reserved10: Reserved10::<Identity, Impl, OFFSET>,
            Reserved11: Reserved11::<Identity, Impl, OFFSET>,
            Reserved12: Reserved12::<Identity, Impl, OFFSET>,
            Reserved13: Reserved13::<Identity, Impl, OFFSET>,
            Reserved14: Reserved14::<Identity, Impl, OFFSET>,
            Reserved15: Reserved15::<Identity, Impl, OFFSET>,
            Reserved16: Reserved16::<Identity, Impl, OFFSET>,
            Reserved17: Reserved17::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>() -> ITransactionProxy_Vtbl {
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProxy_Impl::Commit(this, core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProxy_Impl::Abort(this).into()
        }
        unsafe extern "system" fn Promote<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITransactionProxy_Impl::Promote(this) {
                Ok(ok__) => {
                    core::ptr::write(ptransaction, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVoter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptxasync: *mut core::ffi::c_void, ppballot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITransactionProxy_Impl::CreateVoter(this, windows_core::from_raw_borrowed(&ptxasync)) {
                Ok(ok__) => {
                    core::ptr::write(ppballot, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsolationLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProxy_Impl::GetIsolationLevel(this, core::mem::transmute_copy(&__midl__itransactionproxy0000)).into()
        }
        unsafe extern "system" fn GetIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstridentifier: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProxy_Impl::GetIdentifier(this, core::mem::transmute_copy(&pbstridentifier)).into()
        }
        unsafe extern "system" fn IsReusable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionProxy_Impl::IsReusable(this, core::mem::transmute_copy(&pfisreusable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            Promote: Promote::<Identity, Impl, OFFSET>,
            CreateVoter: CreateVoter::<Identity, Impl, OFFSET>,
            GetIsolationLevel: GetIsolationLevel::<Identity, Impl, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET>,
            IsReusable: IsReusable::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>() -> ITransactionResourcePool_Vtbl {
        unsafe extern "system" fn PutResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppool: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionResourcePool_Impl::PutResource(this, windows_core::from_raw_borrowed(&ppool), windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppool: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ITransactionResourcePool_Impl::GetResource(this, windows_core::from_raw_borrowed(&ppool)) {
                Ok(ok__) => {
                    core::ptr::write(ppunk, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutResource: PutResource::<Identity, Impl, OFFSET>,
            GetResource: GetResource::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>() -> ITransactionStatus_Vtbl {
        unsafe extern "system" fn SetTransactionStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionStatus_Impl::SetTransactionStatus(this, core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn GetTransactionStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrstatus: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITransactionStatus_Impl::GetTransactionStatus(this, core::mem::transmute_copy(&phrstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTransactionStatus: SetTransactionStatus::<Identity, Impl, OFFSET>,
            GetTransactionStatus: GetTransactionStatus::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITxProxyHolder_Impl, const OFFSET: isize>() -> ITxProxyHolder_Vtbl {
        unsafe extern "system" fn GetIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ITxProxyHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidltx: *mut windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ITxProxyHolder_Impl::GetIdentifier(this, core::mem::transmute_copy(&pguidltx))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>() -> ObjectContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprogid: core::mem::MaybeUninit<windows_core::BSTR>, pobject: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::CreateInstance(this, core::mem::transmute(&bstrprogid)) {
                Ok(ok__) => {
                    core::ptr::write(pobject, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ObjectContext_Impl::SetComplete(this).into()
        }
        unsafe extern "system" fn SetAbort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ObjectContext_Impl::SetAbort(this).into()
        }
        unsafe extern "system" fn EnableCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ObjectContext_Impl::EnableCommit(this).into()
        }
        unsafe extern "system" fn DisableCommit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ObjectContext_Impl::DisableCommit(this).into()
        }
        unsafe extern "system" fn IsInTransaction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::IsInTransaction(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisintx, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::IsSecurityEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pbisenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrrole: core::mem::MaybeUninit<windows_core::BSTR>, pbinrole: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::IsCallerInRole(this, core::mem::transmute(&bstrrole)) {
                Ok(ok__) => {
                    core::ptr::write(pbinrole, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(plcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::get_Item(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsecurityproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::Security(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsecurityproperty, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontextinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ObjectContext_Impl::ContextInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcontextinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            SetComplete: SetComplete::<Identity, Impl, OFFSET>,
            SetAbort: SetAbort::<Identity, Impl, OFFSET>,
            EnableCommit: EnableCommit::<Identity, Impl, OFFSET>,
            DisableCommit: DisableCommit::<Identity, Impl, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            ContextInfo: ContextInfo::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>() -> ObjectControl_Vtbl {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ObjectControl_Impl::Activate(this).into()
        }
        unsafe extern "system" fn Deactivate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ObjectControl_Impl::Deactivate(this).into()
        }
        unsafe extern "system" fn CanBePooled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ObjectControl_Impl::CanBePooled(this, core::mem::transmute_copy(&pbpoolable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CanBePooled: CanBePooled::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>() -> SecurityProperty_Vtbl {
        unsafe extern "system" fn GetDirectCallerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match SecurityProperty_Impl::GetDirectCallerName(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrusername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectCreatorName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match SecurityProperty_Impl::GetDirectCreatorName(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrusername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCallerName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match SecurityProperty_Impl::GetOriginalCallerName(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrusername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCreatorName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match SecurityProperty_Impl::GetOriginalCreatorName(this) {
                Ok(ok__) => {
                    core::ptr::write(bstrusername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDirectCallerName: GetDirectCallerName::<Identity, Impl, OFFSET>,
            GetDirectCreatorName: GetDirectCreatorName::<Identity, Impl, OFFSET>,
            GetOriginalCallerName: GetOriginalCallerName::<Identity, Impl, OFFSET>,
            GetOriginalCreatorName: GetOriginalCreatorName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<SecurityProperty as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
