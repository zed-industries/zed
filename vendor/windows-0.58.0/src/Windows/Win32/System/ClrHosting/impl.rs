pub trait IActionOnCLREvent_Impl: Sized {
    fn OnEvent(&self, event: EClrEvent, data: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IActionOnCLREvent {}
impl IActionOnCLREvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActionOnCLREvent_Vtbl
    where
        Identity: IActionOnCLREvent_Impl,
    {
        unsafe extern "system" fn OnEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: EClrEvent, data: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActionOnCLREvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActionOnCLREvent_Impl::OnEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&data)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActionOnCLREvent as windows_core::Interface>::IID
    }
}
pub trait IApartmentCallback_Impl: Sized {
    fn DoCallback(&self, pfunc: usize, pdata: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IApartmentCallback {}
impl IApartmentCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IApartmentCallback_Vtbl
    where
        Identity: IApartmentCallback_Impl,
    {
        unsafe extern "system" fn DoCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunc: usize, pdata: usize) -> windows_core::HRESULT
        where
            Identity: IApartmentCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IApartmentCallback_Impl::DoCallback(this, core::mem::transmute_copy(&pfunc), core::mem::transmute_copy(&pdata)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoCallback: DoCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApartmentCallback as windows_core::Interface>::IID
    }
}
pub trait IAppDomainBinding_Impl: Sized {
    fn OnAppDomain(&self, pappdomain: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAppDomainBinding {}
impl IAppDomainBinding_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAppDomainBinding_Vtbl
    where
        Identity: IAppDomainBinding_Impl,
    {
        unsafe extern "system" fn OnAppDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomain: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAppDomainBinding_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAppDomainBinding_Impl::OnAppDomain(this, windows_core::from_raw_borrowed(&pappdomain)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnAppDomain: OnAppDomain::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAppDomainBinding as windows_core::Interface>::IID
    }
}
pub trait ICLRAppDomainResourceMonitor_Impl: Sized {
    fn GetCurrentAllocated(&self, dwappdomainid: u32, pbytesallocated: *mut u64) -> windows_core::Result<()>;
    fn GetCurrentSurvived(&self, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> windows_core::Result<()>;
    fn GetCurrentCpuTime(&self, dwappdomainid: u32, pmilliseconds: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRAppDomainResourceMonitor {}
impl ICLRAppDomainResourceMonitor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRAppDomainResourceMonitor_Vtbl
    where
        Identity: ICLRAppDomainResourceMonitor_Impl,
    {
        unsafe extern "system" fn GetCurrentAllocated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, pbytesallocated: *mut u64) -> windows_core::HRESULT
        where
            Identity: ICLRAppDomainResourceMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRAppDomainResourceMonitor_Impl::GetCurrentAllocated(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&pbytesallocated)).into()
        }
        unsafe extern "system" fn GetCurrentSurvived<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, pappdomainbytessurvived: *mut u64, ptotalbytessurvived: *mut u64) -> windows_core::HRESULT
        where
            Identity: ICLRAppDomainResourceMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRAppDomainResourceMonitor_Impl::GetCurrentSurvived(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&pappdomainbytessurvived), core::mem::transmute_copy(&ptotalbytessurvived)).into()
        }
        unsafe extern "system" fn GetCurrentCpuTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, pmilliseconds: *mut u64) -> windows_core::HRESULT
        where
            Identity: ICLRAppDomainResourceMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRAppDomainResourceMonitor_Impl::GetCurrentCpuTime(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&pmilliseconds)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentAllocated: GetCurrentAllocated::<Identity, OFFSET>,
            GetCurrentSurvived: GetCurrentSurvived::<Identity, OFFSET>,
            GetCurrentCpuTime: GetCurrentCpuTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRAppDomainResourceMonitor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRAssemblyIdentityManager_Impl: Sized {
    fn GetCLRAssemblyReferenceList(&self, ppwzassemblyreferences: *const windows_core::PCWSTR, dwnumofreferences: u32) -> windows_core::Result<ICLRAssemblyReferenceList>;
    fn GetBindingIdentityFromFile(&self, pwzfilepath: &windows_core::PCWSTR, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>;
    fn GetBindingIdentityFromStream(&self, pstream: Option<&super::Com::IStream>, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>;
    fn GetReferencedAssembliesFromFile(&self, pwzfilepath: &windows_core::PCWSTR, dwflags: u32, pexcludeassemblieslist: Option<&ICLRAssemblyReferenceList>) -> windows_core::Result<ICLRReferenceAssemblyEnum>;
    fn GetReferencedAssembliesFromStream(&self, pstream: Option<&super::Com::IStream>, dwflags: u32, pexcludeassemblieslist: Option<&ICLRAssemblyReferenceList>) -> windows_core::Result<ICLRReferenceAssemblyEnum>;
    fn GetProbingAssembliesFromReference(&self, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: &windows_core::PCWSTR) -> windows_core::Result<ICLRProbingAssemblyEnum>;
    fn IsStronglyNamed(&self, pwzassemblyidentity: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICLRAssemblyIdentityManager {}
#[cfg(feature = "Win32_System_Com")]
impl ICLRAssemblyIdentityManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRAssemblyIdentityManager_Vtbl
    where
        Identity: ICLRAssemblyIdentityManager_Impl,
    {
        unsafe extern "system" fn GetCLRAssemblyReferenceList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwzassemblyreferences: *const windows_core::PCWSTR, dwnumofreferences: u32, ppreferencelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyIdentityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRAssemblyIdentityManager_Impl::GetCLRAssemblyReferenceList(this, core::mem::transmute_copy(&ppwzassemblyreferences), core::mem::transmute_copy(&dwnumofreferences)) {
                Ok(ok__) => {
                    ppreferencelist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindingIdentityFromFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyIdentityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRAssemblyIdentityManager_Impl::GetBindingIdentityFromFile(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        unsafe extern "system" fn GetBindingIdentityFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwflags: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyIdentityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRAssemblyIdentityManager_Impl::GetBindingIdentityFromStream(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        unsafe extern "system" fn GetReferencedAssembliesFromFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, dwflags: u32, pexcludeassemblieslist: *mut core::ffi::c_void, ppreferenceenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyIdentityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRAssemblyIdentityManager_Impl::GetReferencedAssembliesFromFile(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pexcludeassemblieslist)) {
                Ok(ok__) => {
                    ppreferenceenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReferencedAssembliesFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwflags: u32, pexcludeassemblieslist: *mut core::ffi::c_void, ppreferenceenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyIdentityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRAssemblyIdentityManager_Impl::GetReferencedAssembliesFromStream(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&dwflags), windows_core::from_raw_borrowed(&pexcludeassemblieslist)) {
                Ok(ok__) => {
                    ppreferenceenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProbingAssembliesFromReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmachinetype: u32, dwflags: u32, pwzreferenceidentity: windows_core::PCWSTR, ppprobingassemblyenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyIdentityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRAssemblyIdentityManager_Impl::GetProbingAssembliesFromReference(this, core::mem::transmute_copy(&dwmachinetype), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pwzreferenceidentity)) {
                Ok(ok__) => {
                    ppprobingassemblyenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStronglyNamed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzassemblyidentity: windows_core::PCWSTR, pbisstronglynamed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyIdentityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRAssemblyIdentityManager_Impl::IsStronglyNamed(this, core::mem::transmute(&pwzassemblyidentity)) {
                Ok(ok__) => {
                    pbisstronglynamed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLRAssemblyReferenceList: GetCLRAssemblyReferenceList::<Identity, OFFSET>,
            GetBindingIdentityFromFile: GetBindingIdentityFromFile::<Identity, OFFSET>,
            GetBindingIdentityFromStream: GetBindingIdentityFromStream::<Identity, OFFSET>,
            GetReferencedAssembliesFromFile: GetReferencedAssembliesFromFile::<Identity, OFFSET>,
            GetReferencedAssembliesFromStream: GetReferencedAssembliesFromStream::<Identity, OFFSET>,
            GetProbingAssembliesFromReference: GetProbingAssembliesFromReference::<Identity, OFFSET>,
            IsStronglyNamed: IsStronglyNamed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRAssemblyIdentityManager as windows_core::Interface>::IID
    }
}
pub trait ICLRAssemblyReferenceList_Impl: Sized {
    fn IsStringAssemblyReferenceInList(&self, pwzassemblyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsAssemblyReferenceInList(&self, pname: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRAssemblyReferenceList {}
impl ICLRAssemblyReferenceList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRAssemblyReferenceList_Vtbl
    where
        Identity: ICLRAssemblyReferenceList_Impl,
    {
        unsafe extern "system" fn IsStringAssemblyReferenceInList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzassemblyname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyReferenceList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRAssemblyReferenceList_Impl::IsStringAssemblyReferenceInList(this, core::mem::transmute(&pwzassemblyname)).into()
        }
        unsafe extern "system" fn IsAssemblyReferenceInList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRAssemblyReferenceList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRAssemblyReferenceList_Impl::IsAssemblyReferenceInList(this, windows_core::from_raw_borrowed(&pname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStringAssemblyReferenceInList: IsStringAssemblyReferenceInList::<Identity, OFFSET>,
            IsAssemblyReferenceInList: IsAssemblyReferenceInList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRAssemblyReferenceList as windows_core::Interface>::IID
    }
}
pub trait ICLRControl_Impl: Sized {
    fn GetCLRManager(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetAppDomainManagerType(&self, pwzappdomainmanagerassembly: &windows_core::PCWSTR, pwzappdomainmanagertype: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRControl {}
impl ICLRControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRControl_Vtbl
    where
        Identity: ICLRControl_Impl,
    {
        unsafe extern "system" fn GetCLRManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRControl_Impl::GetCLRManager(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn SetAppDomainManagerType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzappdomainmanagerassembly: windows_core::PCWSTR, pwzappdomainmanagertype: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICLRControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRControl_Impl::SetAppDomainManagerType(this, core::mem::transmute(&pwzappdomainmanagerassembly), core::mem::transmute(&pwzappdomainmanagertype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLRManager: GetCLRManager::<Identity, OFFSET>,
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security")]
pub trait ICLRDebugManager_Impl: Sized {
    fn BeginConnection(&self, dwconnectionid: u32, szconnectionname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetConnectionTasks(&self, id: u32, dwcount: u32, ppclrtask: *const Option<ICLRTask>) -> windows_core::Result<()>;
    fn EndConnection(&self, dwconnectionid: u32) -> windows_core::Result<()>;
    fn SetDacl(&self, pacl: *const super::super::Security::ACL) -> windows_core::Result<()>;
    fn GetDacl(&self) -> windows_core::Result<*mut super::super::Security::ACL>;
    fn IsDebuggerAttached(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSymbolReadingPolicy(&self, policy: ESymbolReadingPolicy) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for ICLRDebugManager {}
#[cfg(feature = "Win32_Security")]
impl ICLRDebugManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRDebugManager_Vtbl
    where
        Identity: ICLRDebugManager_Impl,
    {
        unsafe extern "system" fn BeginConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnectionid: u32, szconnectionname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICLRDebugManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDebugManager_Impl::BeginConnection(this, core::mem::transmute_copy(&dwconnectionid), core::mem::transmute(&szconnectionname)).into()
        }
        unsafe extern "system" fn SetConnectionTasks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u32, dwcount: u32, ppclrtask: *const *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRDebugManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDebugManager_Impl::SetConnectionTasks(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&ppclrtask)).into()
        }
        unsafe extern "system" fn EndConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnectionid: u32) -> windows_core::HRESULT
        where
            Identity: ICLRDebugManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDebugManager_Impl::EndConnection(this, core::mem::transmute_copy(&dwconnectionid)).into()
        }
        unsafe extern "system" fn SetDacl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacl: *const super::super::Security::ACL) -> windows_core::HRESULT
        where
            Identity: ICLRDebugManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDebugManager_Impl::SetDacl(this, core::mem::transmute_copy(&pacl)).into()
        }
        unsafe extern "system" fn GetDacl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pacl: *mut *mut super::super::Security::ACL) -> windows_core::HRESULT
        where
            Identity: ICLRDebugManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRDebugManager_Impl::GetDacl(this) {
                Ok(ok__) => {
                    pacl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDebuggerAttached<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICLRDebugManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRDebugManager_Impl::IsDebuggerAttached(this) {
                Ok(ok__) => {
                    pbattached.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSymbolReadingPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: ESymbolReadingPolicy) -> windows_core::HRESULT
        where
            Identity: ICLRDebugManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDebugManager_Impl::SetSymbolReadingPolicy(this, core::mem::transmute_copy(&policy)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginConnection: BeginConnection::<Identity, OFFSET>,
            SetConnectionTasks: SetConnectionTasks::<Identity, OFFSET>,
            EndConnection: EndConnection::<Identity, OFFSET>,
            SetDacl: SetDacl::<Identity, OFFSET>,
            GetDacl: GetDacl::<Identity, OFFSET>,
            IsDebuggerAttached: IsDebuggerAttached::<Identity, OFFSET>,
            SetSymbolReadingPolicy: SetSymbolReadingPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRDebugManager as windows_core::Interface>::IID
    }
}
pub trait ICLRDebugging_Impl: Sized {
    fn OpenVirtualProcess(&self, modulebaseaddress: u64, pdatatarget: Option<&windows_core::IUnknown>, plibraryprovider: Option<&ICLRDebuggingLibraryProvider>, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const windows_core::GUID, ppprocess: *mut Option<windows_core::IUnknown>, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> windows_core::Result<()>;
    fn CanUnloadNow(&self, hmodule: super::super::Foundation::HMODULE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRDebugging {}
impl ICLRDebugging_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRDebugging_Vtbl
    where
        Identity: ICLRDebugging_Impl,
    {
        unsafe extern "system" fn OpenVirtualProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, modulebaseaddress: u64, pdatatarget: *mut core::ffi::c_void, plibraryprovider: *mut core::ffi::c_void, pmaxdebuggersupportedversion: *const CLR_DEBUGGING_VERSION, riidprocess: *const windows_core::GUID, ppprocess: *mut *mut core::ffi::c_void, pversion: *mut CLR_DEBUGGING_VERSION, pdwflags: *mut CLR_DEBUGGING_PROCESS_FLAGS) -> windows_core::HRESULT
        where
            Identity: ICLRDebugging_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDebugging_Impl::OpenVirtualProcess(this, core::mem::transmute_copy(&modulebaseaddress), windows_core::from_raw_borrowed(&pdatatarget), windows_core::from_raw_borrowed(&plibraryprovider), core::mem::transmute_copy(&pmaxdebuggersupportedversion), core::mem::transmute_copy(&riidprocess), core::mem::transmute_copy(&ppprocess), core::mem::transmute_copy(&pversion), core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn CanUnloadNow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmodule: super::super::Foundation::HMODULE) -> windows_core::HRESULT
        where
            Identity: ICLRDebugging_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDebugging_Impl::CanUnloadNow(this, core::mem::transmute_copy(&hmodule)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenVirtualProcess: OpenVirtualProcess::<Identity, OFFSET>,
            CanUnloadNow: CanUnloadNow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRDebugging as windows_core::Interface>::IID
    }
}
pub trait ICLRDebuggingLibraryProvider_Impl: Sized {
    fn ProvideLibrary(&self, pwszfilename: &windows_core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32) -> windows_core::Result<super::super::Foundation::HMODULE>;
}
impl windows_core::RuntimeName for ICLRDebuggingLibraryProvider {}
impl ICLRDebuggingLibraryProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRDebuggingLibraryProvider_Vtbl
    where
        Identity: ICLRDebuggingLibraryProvider_Impl,
    {
        unsafe extern "system" fn ProvideLibrary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR, dwtimestamp: u32, dwsizeofimage: u32, phmodule: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT
        where
            Identity: ICLRDebuggingLibraryProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRDebuggingLibraryProvider_Impl::ProvideLibrary(this, core::mem::transmute(&pwszfilename), core::mem::transmute_copy(&dwtimestamp), core::mem::transmute_copy(&dwsizeofimage)) {
                Ok(ok__) => {
                    phmodule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideLibrary: ProvideLibrary::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRDebuggingLibraryProvider as windows_core::Interface>::IID
    }
}
pub trait ICLRDomainManager_Impl: Sized {
    fn SetAppDomainManagerType(&self, wszappdomainmanagerassembly: &windows_core::PCWSTR, wszappdomainmanagertype: &windows_core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> windows_core::Result<()>;
    fn SetPropertiesForDefaultAppDomain(&self, nproperties: u32, pwszpropertynames: *const windows_core::PCWSTR, pwszpropertyvalues: *const windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRDomainManager {}
impl ICLRDomainManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRDomainManager_Vtbl
    where
        Identity: ICLRDomainManager_Impl,
    {
        unsafe extern "system" fn SetAppDomainManagerType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszappdomainmanagerassembly: windows_core::PCWSTR, wszappdomainmanagertype: windows_core::PCWSTR, dwinitializedomainflags: EInitializeNewDomainFlags) -> windows_core::HRESULT
        where
            Identity: ICLRDomainManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDomainManager_Impl::SetAppDomainManagerType(this, core::mem::transmute(&wszappdomainmanagerassembly), core::mem::transmute(&wszappdomainmanagertype), core::mem::transmute_copy(&dwinitializedomainflags)).into()
        }
        unsafe extern "system" fn SetPropertiesForDefaultAppDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nproperties: u32, pwszpropertynames: *const windows_core::PCWSTR, pwszpropertyvalues: *const windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICLRDomainManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRDomainManager_Impl::SetPropertiesForDefaultAppDomain(this, core::mem::transmute_copy(&nproperties), core::mem::transmute_copy(&pwszpropertynames), core::mem::transmute_copy(&pwszpropertyvalues)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAppDomainManagerType: SetAppDomainManagerType::<Identity, OFFSET>,
            SetPropertiesForDefaultAppDomain: SetPropertiesForDefaultAppDomain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRDomainManager as windows_core::Interface>::IID
    }
}
pub trait ICLRErrorReportingManager_Impl: Sized {
    fn GetBucketParametersForCurrentException(&self, pparams: *mut BucketParameters) -> windows_core::Result<()>;
    fn BeginCustomDump(&self, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *const CustomDumpItem, dwreserved: u32) -> windows_core::Result<()>;
    fn EndCustomDump(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRErrorReportingManager {}
impl ICLRErrorReportingManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRErrorReportingManager_Vtbl
    where
        Identity: ICLRErrorReportingManager_Impl,
    {
        unsafe extern "system" fn GetBucketParametersForCurrentException<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *mut BucketParameters) -> windows_core::HRESULT
        where
            Identity: ICLRErrorReportingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRErrorReportingManager_Impl::GetBucketParametersForCurrentException(this, core::mem::transmute_copy(&pparams)).into()
        }
        unsafe extern "system" fn BeginCustomDump<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflavor: ECustomDumpFlavor, dwnumitems: u32, items: *const CustomDumpItem, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: ICLRErrorReportingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRErrorReportingManager_Impl::BeginCustomDump(this, core::mem::transmute_copy(&dwflavor), core::mem::transmute_copy(&dwnumitems), core::mem::transmute_copy(&items), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn EndCustomDump<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRErrorReportingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRErrorReportingManager_Impl::EndCustomDump(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBucketParametersForCurrentException: GetBucketParametersForCurrentException::<Identity, OFFSET>,
            BeginCustomDump: BeginCustomDump::<Identity, OFFSET>,
            EndCustomDump: EndCustomDump::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRErrorReportingManager as windows_core::Interface>::IID
    }
}
pub trait ICLRGCManager_Impl: Sized {
    fn Collect(&self, generation: i32) -> windows_core::Result<()>;
    fn GetStats(&self, pstats: *mut COR_GC_STATS) -> windows_core::Result<()>;
    fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRGCManager {}
impl ICLRGCManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRGCManager_Vtbl
    where
        Identity: ICLRGCManager_Impl,
    {
        unsafe extern "system" fn Collect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, generation: i32) -> windows_core::HRESULT
        where
            Identity: ICLRGCManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRGCManager_Impl::Collect(this, core::mem::transmute_copy(&generation)).into()
        }
        unsafe extern "system" fn GetStats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut COR_GC_STATS) -> windows_core::HRESULT
        where
            Identity: ICLRGCManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRGCManager_Impl::GetStats(this, core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetGCStartupLimits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> windows_core::HRESULT
        where
            Identity: ICLRGCManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRGCManager_Impl::SetGCStartupLimits(this, core::mem::transmute_copy(&segmentsize), core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Collect: Collect::<Identity, OFFSET>,
            GetStats: GetStats::<Identity, OFFSET>,
            SetGCStartupLimits: SetGCStartupLimits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRGCManager as windows_core::Interface>::IID
    }
}
pub trait ICLRGCManager2_Impl: Sized + ICLRGCManager_Impl {
    fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRGCManager2 {}
impl ICLRGCManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRGCManager2_Vtbl
    where
        Identity: ICLRGCManager2_Impl,
    {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> windows_core::HRESULT
        where
            Identity: ICLRGCManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRGCManager2_Impl::SetGCStartupLimitsEx(this, core::mem::transmute_copy(&segmentsize), core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self { base__: ICLRGCManager_Vtbl::new::<Identity, OFFSET>(), SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRGCManager2 as windows_core::Interface>::IID || iid == &<ICLRGCManager as windows_core::Interface>::IID
    }
}
pub trait ICLRHostBindingPolicyManager_Impl: Sized {
    fn ModifyApplicationPolicy(&self, pwzsourceassemblyidentity: &windows_core::PCWSTR, pwztargetassemblyidentity: &windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> windows_core::Result<()>;
    fn EvaluatePolicy(&self, pwzreferenceidentity: &windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRHostBindingPolicyManager {}
impl ICLRHostBindingPolicyManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRHostBindingPolicyManager_Vtbl
    where
        Identity: ICLRHostBindingPolicyManager_Impl,
    {
        unsafe extern "system" fn ModifyApplicationPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzsourceassemblyidentity: windows_core::PCWSTR, pwztargetassemblyidentity: windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, dwpolicymodifyflags: u32, pbnewapplicationpolicy: *mut u8, pcbnewapppolicysize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRHostBindingPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRHostBindingPolicyManager_Impl::ModifyApplicationPolicy(this, core::mem::transmute(&pwzsourceassemblyidentity), core::mem::transmute(&pwztargetassemblyidentity), core::mem::transmute_copy(&pbapplicationpolicy), core::mem::transmute_copy(&cbapppolicysize), core::mem::transmute_copy(&dwpolicymodifyflags), core::mem::transmute_copy(&pbnewapplicationpolicy), core::mem::transmute_copy(&pcbnewapppolicysize)).into()
        }
        unsafe extern "system" fn EvaluatePolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzreferenceidentity: windows_core::PCWSTR, pbapplicationpolicy: *const u8, cbapppolicysize: u32, pwzpostpolicyreferenceidentity: windows_core::PWSTR, pcchpostpolicyreferenceidentity: *mut u32, pdwpoliciesapplied: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRHostBindingPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRHostBindingPolicyManager_Impl::EvaluatePolicy(this, core::mem::transmute(&pwzreferenceidentity), core::mem::transmute_copy(&pbapplicationpolicy), core::mem::transmute_copy(&cbapppolicysize), core::mem::transmute_copy(&pwzpostpolicyreferenceidentity), core::mem::transmute_copy(&pcchpostpolicyreferenceidentity), core::mem::transmute_copy(&pdwpoliciesapplied)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ModifyApplicationPolicy: ModifyApplicationPolicy::<Identity, OFFSET>,
            EvaluatePolicy: EvaluatePolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRHostBindingPolicyManager as windows_core::Interface>::IID
    }
}
pub trait ICLRHostProtectionManager_Impl: Sized {
    fn SetProtectedCategories(&self, categories: EApiCategories) -> windows_core::Result<()>;
    fn SetEagerSerializeGrantSets(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRHostProtectionManager {}
impl ICLRHostProtectionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRHostProtectionManager_Vtbl
    where
        Identity: ICLRHostProtectionManager_Impl,
    {
        unsafe extern "system" fn SetProtectedCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, categories: EApiCategories) -> windows_core::HRESULT
        where
            Identity: ICLRHostProtectionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRHostProtectionManager_Impl::SetProtectedCategories(this, core::mem::transmute_copy(&categories)).into()
        }
        unsafe extern "system" fn SetEagerSerializeGrantSets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRHostProtectionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRHostProtectionManager_Impl::SetEagerSerializeGrantSets(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProtectedCategories: SetProtectedCategories::<Identity, OFFSET>,
            SetEagerSerializeGrantSets: SetEagerSerializeGrantSets::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRHostProtectionManager as windows_core::Interface>::IID
    }
}
pub trait ICLRIoCompletionManager_Impl: Sized {
    fn OnComplete(&self, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRIoCompletionManager {}
impl ICLRIoCompletionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRIoCompletionManager_Vtbl
    where
        Identity: ICLRIoCompletionManager_Impl,
    {
        unsafe extern "system" fn OnComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwerrorcode: u32, numberofbytestransferred: u32, pvoverlapped: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRIoCompletionManager_Impl::OnComplete(this, core::mem::transmute_copy(&dwerrorcode), core::mem::transmute_copy(&numberofbytestransferred), core::mem::transmute_copy(&pvoverlapped)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnComplete: OnComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRIoCompletionManager as windows_core::Interface>::IID
    }
}
pub trait ICLRMemoryNotificationCallback_Impl: Sized {
    fn OnMemoryNotification(&self, ememoryavailable: EMemoryAvailable) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRMemoryNotificationCallback {}
impl ICLRMemoryNotificationCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRMemoryNotificationCallback_Vtbl
    where
        Identity: ICLRMemoryNotificationCallback_Impl,
    {
        unsafe extern "system" fn OnMemoryNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ememoryavailable: EMemoryAvailable) -> windows_core::HRESULT
        where
            Identity: ICLRMemoryNotificationCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRMemoryNotificationCallback_Impl::OnMemoryNotification(this, core::mem::transmute_copy(&ememoryavailable)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMemoryNotification: OnMemoryNotification::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRMemoryNotificationCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRMetaHost_Impl: Sized {
    fn GetRuntime(&self, pwzversion: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetVersionFromFile(&self, pwzfilepath: &windows_core::PCWSTR, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()>;
    fn EnumerateInstalledRuntimes(&self) -> windows_core::Result<super::Com::IEnumUnknown>;
    fn EnumerateLoadedRuntimes(&self, hndprocess: super::super::Foundation::HANDLE) -> windows_core::Result<super::Com::IEnumUnknown>;
    fn RequestRuntimeLoadedNotification(&self, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> windows_core::Result<()>;
    fn QueryLegacyV2RuntimeBinding(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ExitProcess(&self, iexitcode: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICLRMetaHost {}
#[cfg(feature = "Win32_System_Com")]
impl ICLRMetaHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRMetaHost_Vtbl
    where
        Identity: ICLRMetaHost_Impl,
    {
        unsafe extern "system" fn GetRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzversion: windows_core::PCWSTR, riid: *const windows_core::GUID, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRMetaHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRMetaHost_Impl::GetRuntime(this, core::mem::transmute(&pwzversion), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppruntime)).into()
        }
        unsafe extern "system" fn GetVersionFromFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRMetaHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRMetaHost_Impl::GetVersionFromFile(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn EnumerateInstalledRuntimes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRMetaHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRMetaHost_Impl::EnumerateInstalledRuntimes(this) {
                Ok(ok__) => {
                    ppenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateLoadedRuntimes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRMetaHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRMetaHost_Impl::EnumerateLoadedRuntimes(this, core::mem::transmute_copy(&hndprocess)) {
                Ok(ok__) => {
                    ppenumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRuntimeLoadedNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbackfunction: RuntimeLoadedCallbackFnPtr) -> windows_core::HRESULT
        where
            Identity: ICLRMetaHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRMetaHost_Impl::RequestRuntimeLoadedNotification(this, core::mem::transmute_copy(&pcallbackfunction)).into()
        }
        unsafe extern "system" fn QueryLegacyV2RuntimeBinding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRMetaHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRMetaHost_Impl::QueryLegacyV2RuntimeBinding(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn ExitProcess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iexitcode: i32) -> windows_core::HRESULT
        where
            Identity: ICLRMetaHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRMetaHost_Impl::ExitProcess(this, core::mem::transmute_copy(&iexitcode)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRuntime: GetRuntime::<Identity, OFFSET>,
            GetVersionFromFile: GetVersionFromFile::<Identity, OFFSET>,
            EnumerateInstalledRuntimes: EnumerateInstalledRuntimes::<Identity, OFFSET>,
            EnumerateLoadedRuntimes: EnumerateLoadedRuntimes::<Identity, OFFSET>,
            RequestRuntimeLoadedNotification: RequestRuntimeLoadedNotification::<Identity, OFFSET>,
            QueryLegacyV2RuntimeBinding: QueryLegacyV2RuntimeBinding::<Identity, OFFSET>,
            ExitProcess: ExitProcess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRMetaHost as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICLRMetaHostPolicy_Impl: Sized {
    fn GetRequestedRuntime(&self, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: &windows_core::PCWSTR, pcfgstream: Option<&super::Com::IStream>, pwzversion: &windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const windows_core::GUID, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICLRMetaHostPolicy {}
#[cfg(feature = "Win32_System_Com")]
impl ICLRMetaHostPolicy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRMetaHostPolicy_Vtbl
    where
        Identity: ICLRMetaHostPolicy_Impl,
    {
        unsafe extern "system" fn GetRequestedRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpolicyflags: METAHOST_POLICY_FLAGS, pwzbinary: windows_core::PCWSTR, pcfgstream: *mut core::ffi::c_void, pwzversion: windows_core::PWSTR, pcchversion: *mut u32, pwzimageversion: windows_core::PWSTR, pcchimageversion: *mut u32, pdwconfigflags: *mut u32, riid: *const windows_core::GUID, ppruntime: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRMetaHostPolicy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRMetaHostPolicy_Impl::GetRequestedRuntime(this, core::mem::transmute_copy(&dwpolicyflags), core::mem::transmute(&pwzbinary), windows_core::from_raw_borrowed(&pcfgstream), core::mem::transmute(&pwzversion), core::mem::transmute_copy(&pcchversion), core::mem::transmute_copy(&pwzimageversion), core::mem::transmute_copy(&pcchimageversion), core::mem::transmute_copy(&pdwconfigflags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppruntime)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRequestedRuntime: GetRequestedRuntime::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRMetaHostPolicy as windows_core::Interface>::IID
    }
}
pub trait ICLROnEventManager_Impl: Sized {
    fn RegisterActionOnEvent(&self, event: EClrEvent, paction: Option<&IActionOnCLREvent>) -> windows_core::Result<()>;
    fn UnregisterActionOnEvent(&self, event: EClrEvent, paction: Option<&IActionOnCLREvent>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLROnEventManager {}
impl ICLROnEventManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLROnEventManager_Vtbl
    where
        Identity: ICLROnEventManager_Impl,
    {
        unsafe extern "system" fn RegisterActionOnEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: EClrEvent, paction: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLROnEventManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLROnEventManager_Impl::RegisterActionOnEvent(this, core::mem::transmute_copy(&event), windows_core::from_raw_borrowed(&paction)).into()
        }
        unsafe extern "system" fn UnregisterActionOnEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: EClrEvent, paction: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLROnEventManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLROnEventManager_Impl::UnregisterActionOnEvent(this, core::mem::transmute_copy(&event), windows_core::from_raw_borrowed(&paction)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterActionOnEvent: RegisterActionOnEvent::<Identity, OFFSET>,
            UnregisterActionOnEvent: UnregisterActionOnEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLROnEventManager as windows_core::Interface>::IID
    }
}
pub trait ICLRPolicyManager_Impl: Sized {
    fn SetDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()>;
    fn SetTimeout(&self, operation: EClrOperation, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn SetActionOnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()>;
    fn SetTimeoutAndAction(&self, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> windows_core::Result<()>;
    fn SetActionOnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> windows_core::Result<()>;
    fn SetUnhandledExceptionPolicy(&self, policy: EClrUnhandledException) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRPolicyManager {}
impl ICLRPolicyManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRPolicyManager_Vtbl
    where
        Identity: ICLRPolicyManager_Impl,
    {
        unsafe extern "system" fn SetDefaultAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> windows_core::HRESULT
        where
            Identity: ICLRPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRPolicyManager_Impl::SetDefaultAction(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32) -> windows_core::HRESULT
        where
            Identity: ICLRPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRPolicyManager_Impl::SetTimeout(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn SetActionOnTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> windows_core::HRESULT
        where
            Identity: ICLRPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRPolicyManager_Impl::SetActionOnTimeout(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetTimeoutAndAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, dwmilliseconds: u32, action: EPolicyAction) -> windows_core::HRESULT
        where
            Identity: ICLRPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRPolicyManager_Impl::SetTimeoutAndAction(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetActionOnFailure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> windows_core::HRESULT
        where
            Identity: ICLRPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRPolicyManager_Impl::SetActionOnFailure(this, core::mem::transmute_copy(&failure), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn SetUnhandledExceptionPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: EClrUnhandledException) -> windows_core::HRESULT
        where
            Identity: ICLRPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRPolicyManager_Impl::SetUnhandledExceptionPolicy(this, core::mem::transmute_copy(&policy)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultAction: SetDefaultAction::<Identity, OFFSET>,
            SetTimeout: SetTimeout::<Identity, OFFSET>,
            SetActionOnTimeout: SetActionOnTimeout::<Identity, OFFSET>,
            SetTimeoutAndAction: SetTimeoutAndAction::<Identity, OFFSET>,
            SetActionOnFailure: SetActionOnFailure::<Identity, OFFSET>,
            SetUnhandledExceptionPolicy: SetUnhandledExceptionPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRPolicyManager as windows_core::Interface>::IID
    }
}
pub trait ICLRProbingAssemblyEnum_Impl: Sized {
    fn Get(&self, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRProbingAssemblyEnum {}
impl ICLRProbingAssemblyEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRProbingAssemblyEnum_Vtbl
    where
        Identity: ICLRProbingAssemblyEnum_Impl,
    {
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRProbingAssemblyEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRProbingAssemblyEnum_Impl::Get(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRProbingAssemblyEnum as windows_core::Interface>::IID
    }
}
pub trait ICLRProfiling_Impl: Sized {
    fn AttachProfiler(&self, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const windows_core::GUID, wszprofilerpath: &windows_core::PCWSTR, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRProfiling {}
impl ICLRProfiling_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRProfiling_Vtbl
    where
        Identity: ICLRProfiling_Impl,
    {
        unsafe extern "system" fn AttachProfiler<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofileeprocessid: u32, dwmillisecondsmax: u32, pclsidprofiler: *const windows_core::GUID, wszprofilerpath: windows_core::PCWSTR, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::HRESULT
        where
            Identity: ICLRProfiling_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRProfiling_Impl::AttachProfiler(this, core::mem::transmute_copy(&dwprofileeprocessid), core::mem::transmute_copy(&dwmillisecondsmax), core::mem::transmute_copy(&pclsidprofiler), core::mem::transmute(&wszprofilerpath), core::mem::transmute_copy(&pvclientdata), core::mem::transmute_copy(&cbclientdata)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AttachProfiler: AttachProfiler::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRProfiling as windows_core::Interface>::IID
    }
}
pub trait ICLRReferenceAssemblyEnum_Impl: Sized {
    fn Get(&self, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRReferenceAssemblyEnum {}
impl ICLRReferenceAssemblyEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRReferenceAssemblyEnum_Vtbl
    where
        Identity: ICLRReferenceAssemblyEnum_Impl,
    {
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pwzbuffer: windows_core::PWSTR, pcchbuffersize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRReferenceAssemblyEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRReferenceAssemblyEnum_Impl::Get(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffersize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Get: Get::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRReferenceAssemblyEnum as windows_core::Interface>::IID
    }
}
pub trait ICLRRuntimeHost_Impl: Sized {
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn SetHostControl(&self, phostcontrol: Option<&IHostControl>) -> windows_core::Result<()>;
    fn GetCLRControl(&self) -> windows_core::Result<ICLRControl>;
    fn UnloadAppDomain(&self, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ExecuteInAppDomain(&self, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCurrentAppDomainId(&self) -> windows_core::Result<u32>;
    fn ExecuteApplication(&self, pwzappfullname: &windows_core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const windows_core::PCWSTR) -> windows_core::Result<i32>;
    fn ExecuteInDefaultAppDomain(&self, pwzassemblypath: &windows_core::PCWSTR, pwztypename: &windows_core::PCWSTR, pwzmethodname: &windows_core::PCWSTR, pwzargument: &windows_core::PCWSTR) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ICLRRuntimeHost {}
impl ICLRRuntimeHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRRuntimeHost_Vtbl
    where
        Identity: ICLRRuntimeHost_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeHost_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeHost_Impl::Stop(this).into()
        }
        unsafe extern "system" fn SetHostControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phostcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeHost_Impl::SetHostControl(this, windows_core::from_raw_borrowed(&phostcontrol)).into()
        }
        unsafe extern "system" fn GetCLRControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclrcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRRuntimeHost_Impl::GetCLRControl(this) {
                Ok(ok__) => {
                    pclrcontrol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnloadAppDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, fwaituntildone: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeHost_Impl::UnloadAppDomain(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&fwaituntildone)).into()
        }
        unsafe extern "system" fn ExecuteInAppDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, pcallback: FExecuteInAppDomainCallback, cookie: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeHost_Impl::ExecuteInAppDomain(this, core::mem::transmute_copy(&dwappdomainid), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn GetCurrentAppDomainId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwappdomainid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRRuntimeHost_Impl::GetCurrentAppDomainId(this) {
                Ok(ok__) => {
                    pdwappdomainid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzappfullname: windows_core::PCWSTR, dwmanifestpaths: u32, ppwzmanifestpaths: *const windows_core::PCWSTR, dwactivationdata: u32, ppwzactivationdata: *const windows_core::PCWSTR, preturnvalue: *mut i32) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRRuntimeHost_Impl::ExecuteApplication(this, core::mem::transmute(&pwzappfullname), core::mem::transmute_copy(&dwmanifestpaths), core::mem::transmute_copy(&ppwzmanifestpaths), core::mem::transmute_copy(&dwactivationdata), core::mem::transmute_copy(&ppwzactivationdata)) {
                Ok(ok__) => {
                    preturnvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecuteInDefaultAppDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzassemblypath: windows_core::PCWSTR, pwztypename: windows_core::PCWSTR, pwzmethodname: windows_core::PCWSTR, pwzargument: windows_core::PCWSTR, preturnvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRRuntimeHost_Impl::ExecuteInDefaultAppDomain(this, core::mem::transmute(&pwzassemblypath), core::mem::transmute(&pwztypename), core::mem::transmute(&pwzmethodname), core::mem::transmute(&pwzargument)) {
                Ok(ok__) => {
                    preturnvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            SetHostControl: SetHostControl::<Identity, OFFSET>,
            GetCLRControl: GetCLRControl::<Identity, OFFSET>,
            UnloadAppDomain: UnloadAppDomain::<Identity, OFFSET>,
            ExecuteInAppDomain: ExecuteInAppDomain::<Identity, OFFSET>,
            GetCurrentAppDomainId: GetCurrentAppDomainId::<Identity, OFFSET>,
            ExecuteApplication: ExecuteApplication::<Identity, OFFSET>,
            ExecuteInDefaultAppDomain: ExecuteInDefaultAppDomain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRRuntimeHost as windows_core::Interface>::IID
    }
}
pub trait ICLRRuntimeInfo_Impl: Sized {
    fn GetVersionString(&self, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()>;
    fn GetRuntimeDirectory(&self, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::Result<()>;
    fn IsLoaded(&self, hndprocess: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn LoadErrorString(&self, iresourceid: u32, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> windows_core::Result<()>;
    fn LoadLibraryA(&self, pwzdllname: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::HMODULE>;
    fn GetProcAddress(&self, pszprocname: &windows_core::PCSTR) -> windows_core::Result<*mut core::ffi::c_void>;
    fn GetInterface(&self, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsLoadable(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDefaultStartupFlags(&self, dwstartupflags: u32, pwzhostconfigfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDefaultStartupFlags(&self, pdwstartupflags: *mut u32, pwzhostconfigfile: windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> windows_core::Result<()>;
    fn BindAsLegacyV2Runtime(&self) -> windows_core::Result<()>;
    fn IsStarted(&self, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRRuntimeInfo {}
impl ICLRRuntimeInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRRuntimeInfo_Vtbl
    where
        Identity: ICLRRuntimeInfo_Impl,
    {
        unsafe extern "system" fn GetVersionString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeInfo_Impl::GetVersionString(this, core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn GetRuntimeDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeInfo_Impl::GetRuntimeDirectory(this, core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffer)).into()
        }
        unsafe extern "system" fn IsLoaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hndprocess: super::super::Foundation::HANDLE, pbloaded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRRuntimeInfo_Impl::IsLoaded(this, core::mem::transmute_copy(&hndprocess)) {
                Ok(ok__) => {
                    pbloaded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadErrorString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iresourceid: u32, pwzbuffer: windows_core::PWSTR, pcchbuffer: *mut u32, ilocaleid: i32) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeInfo_Impl::LoadErrorString(this, core::mem::transmute_copy(&iresourceid), core::mem::transmute_copy(&pwzbuffer), core::mem::transmute_copy(&pcchbuffer), core::mem::transmute_copy(&ilocaleid)).into()
        }
        unsafe extern "system" fn LoadLibraryA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzdllname: windows_core::PCWSTR, phndmodule: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRRuntimeInfo_Impl::LoadLibraryA(this, core::mem::transmute(&pwzdllname)) {
                Ok(ok__) => {
                    phndmodule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprocname: windows_core::PCSTR, ppproc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRRuntimeInfo_Impl::GetProcAddress(this, core::mem::transmute(&pszprocname)) {
                Ok(ok__) => {
                    ppproc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeInfo_Impl::GetInterface(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn IsLoadable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbloadable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRRuntimeInfo_Impl::IsLoadable(this) {
                Ok(ok__) => {
                    pbloadable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultStartupFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstartupflags: u32, pwzhostconfigfile: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeInfo_Impl::SetDefaultStartupFlags(this, core::mem::transmute_copy(&dwstartupflags), core::mem::transmute(&pwzhostconfigfile)).into()
        }
        unsafe extern "system" fn GetDefaultStartupFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstartupflags: *mut u32, pwzhostconfigfile: windows_core::PWSTR, pcchhostconfigfile: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeInfo_Impl::GetDefaultStartupFlags(this, core::mem::transmute_copy(&pdwstartupflags), core::mem::transmute_copy(&pwzhostconfigfile), core::mem::transmute_copy(&pcchhostconfigfile)).into()
        }
        unsafe extern "system" fn BindAsLegacyV2Runtime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeInfo_Impl::BindAsLegacyV2Runtime(this).into()
        }
        unsafe extern "system" fn IsStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstarted: *mut super::super::Foundation::BOOL, pdwstartupflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRRuntimeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRRuntimeInfo_Impl::IsStarted(this, core::mem::transmute_copy(&pbstarted), core::mem::transmute_copy(&pdwstartupflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersionString: GetVersionString::<Identity, OFFSET>,
            GetRuntimeDirectory: GetRuntimeDirectory::<Identity, OFFSET>,
            IsLoaded: IsLoaded::<Identity, OFFSET>,
            LoadErrorString: LoadErrorString::<Identity, OFFSET>,
            LoadLibraryA: LoadLibraryA::<Identity, OFFSET>,
            GetProcAddress: GetProcAddress::<Identity, OFFSET>,
            GetInterface: GetInterface::<Identity, OFFSET>,
            IsLoadable: IsLoadable::<Identity, OFFSET>,
            SetDefaultStartupFlags: SetDefaultStartupFlags::<Identity, OFFSET>,
            GetDefaultStartupFlags: GetDefaultStartupFlags::<Identity, OFFSET>,
            BindAsLegacyV2Runtime: BindAsLegacyV2Runtime::<Identity, OFFSET>,
            IsStarted: IsStarted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRRuntimeInfo as windows_core::Interface>::IID
    }
}
pub trait ICLRStrongName_Impl: Sized {
    fn GetHashFromAssemblyFile(&self, pszfilepath: &windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromAssemblyFileW(&self, pwzfilepath: &windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromBlob(&self, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromFile(&self, pszfilepath: &windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromFileW(&self, pwzfilepath: &windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn GetHashFromHandle(&self, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::Result<()>;
    fn StrongNameCompareAssemblies(&self, pwzassembly1: &windows_core::PCWSTR, pwzassembly2: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn StrongNameFreeBuffer(&self, pbmemory: *const u8) -> windows_core::Result<()>;
    fn StrongNameGetBlob(&self, pwzfilepath: &windows_core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameGetBlobFromImage(&self, pbbase: *const u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameGetPublicKey(&self, pwzkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameHashSize(&self, ulhashalg: u32) -> windows_core::Result<u32>;
    fn StrongNameKeyDelete(&self, pwzkeycontainer: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StrongNameKeyGen(&self, pwzkeycontainer: &windows_core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameKeyGenEx(&self, pwzkeycontainer: &windows_core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameKeyInstall(&self, pwzkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32) -> windows_core::Result<()>;
    fn StrongNameSignatureGeneration(&self, pwzfilepath: &windows_core::PCWSTR, pwzkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameSignatureGenerationEx(&self, wszfilepath: &windows_core::PCWSTR, wszkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn StrongNameSignatureSize(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> windows_core::Result<()>;
    fn StrongNameSignatureVerification(&self, pwzfilepath: &windows_core::PCWSTR, dwinflags: u32) -> windows_core::Result<u32>;
    fn StrongNameSignatureVerificationEx(&self, pwzfilepath: &windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN) -> windows_core::Result<u8>;
    fn StrongNameSignatureVerificationFromImage(&self, pbbase: *const u8, dwlength: u32, dwinflags: u32) -> windows_core::Result<u32>;
    fn StrongNameTokenFromAssembly(&self, pwzfilepath: &windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::Result<()>;
    fn StrongNameTokenFromAssemblyEx(&self, pwzfilepath: &windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::Result<()>;
    fn StrongNameTokenFromPublicKey(&self, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRStrongName {}
impl ICLRStrongName_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRStrongName_Vtbl
    where
        Identity: ICLRStrongName_Impl,
    {
        unsafe extern "system" fn GetHashFromAssemblyFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilepath: windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::GetHashFromAssemblyFile(this, core::mem::transmute(&pszfilepath), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromAssemblyFileW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::GetHashFromAssemblyFileW(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromBlob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbblob: *const u8, cchblob: u32, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::GetHashFromBlob(this, core::mem::transmute_copy(&pbblob), core::mem::transmute_copy(&cchblob), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilepath: windows_core::PCSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::GetHashFromFile(this, core::mem::transmute(&pszfilepath), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromFileW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::GetHashFromFileW(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn GetHashFromHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfile: super::super::Foundation::HANDLE, pihashalg: *mut u32, pbhash: *mut u8, cchhash: u32, pchhash: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::GetHashFromHandle(this, core::mem::transmute_copy(&hfile), core::mem::transmute_copy(&pihashalg), core::mem::transmute_copy(&pbhash), core::mem::transmute_copy(&cchhash), core::mem::transmute_copy(&pchhash)).into()
        }
        unsafe extern "system" fn StrongNameCompareAssemblies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzassembly1: windows_core::PCWSTR, pwzassembly2: windows_core::PCWSTR, pdwresult: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRStrongName_Impl::StrongNameCompareAssemblies(this, core::mem::transmute(&pwzassembly1), core::mem::transmute(&pwzassembly2)) {
                Ok(ok__) => {
                    pdwresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameFreeBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmemory: *const u8) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameFreeBuffer(this, core::mem::transmute_copy(&pbmemory)).into()
        }
        unsafe extern "system" fn StrongNameGetBlob<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameGetBlob(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&pbblob), core::mem::transmute_copy(&pcbblob)).into()
        }
        unsafe extern "system" fn StrongNameGetBlobFromImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbase: *const u8, dwlength: u32, pbblob: *mut u8, pcbblob: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameGetBlobFromImage(this, core::mem::transmute_copy(&pbbase), core::mem::transmute_copy(&dwlength), core::mem::transmute_copy(&pbblob), core::mem::transmute_copy(&pcbblob)).into()
        }
        unsafe extern "system" fn StrongNameGetPublicKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameGetPublicKey(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&ppbpublickeyblob), core::mem::transmute_copy(&pcbpublickeyblob)).into()
        }
        unsafe extern "system" fn StrongNameHashSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulhashalg: u32, pcbsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRStrongName_Impl::StrongNameHashSize(this, core::mem::transmute_copy(&ulhashalg)) {
                Ok(ok__) => {
                    pcbsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameKeyDelete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameKeyDelete(this, core::mem::transmute(&pwzkeycontainer)).into()
        }
        unsafe extern "system" fn StrongNameKeyGen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, dwflags: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameKeyGen(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ppbkeyblob), core::mem::transmute_copy(&pcbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameKeyGenEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, dwflags: u32, dwkeysize: u32, ppbkeyblob: *mut *mut u8, pcbkeyblob: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameKeyGenEx(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwkeysize), core::mem::transmute_copy(&ppbkeyblob), core::mem::transmute_copy(&pcbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameKeyInstall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameKeyInstall(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob)).into()
        }
        unsafe extern "system" fn StrongNameSignatureGeneration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, pwzkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameSignatureGeneration(this, core::mem::transmute(&pwzfilepath), core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&ppbsignatureblob), core::mem::transmute_copy(&pcbsignatureblob)).into()
        }
        unsafe extern "system" fn StrongNameSignatureGenerationEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilepath: windows_core::PCWSTR, wszkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameSignatureGenerationEx(this, core::mem::transmute(&wszfilepath), core::mem::transmute(&wszkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&ppbsignatureblob), core::mem::transmute_copy(&pcbsignatureblob), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameSignatureSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpublickeyblob: *const u8, cbpublickeyblob: u32, pcbsize: *const u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameSignatureSize(this, core::mem::transmute_copy(&pbpublickeyblob), core::mem::transmute_copy(&cbpublickeyblob), core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn StrongNameSignatureVerification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, dwinflags: u32, pdwoutflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRStrongName_Impl::StrongNameSignatureVerification(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&dwinflags)) {
                Ok(ok__) => {
                    pdwoutflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pfwasverified: *mut u8) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRStrongName_Impl::StrongNameSignatureVerificationEx(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&fforceverification)) {
                Ok(ok__) => {
                    pfwasverified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameSignatureVerificationFromImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbase: *const u8, dwlength: u32, dwinflags: u32, pdwoutflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRStrongName_Impl::StrongNameSignatureVerificationFromImage(this, core::mem::transmute_copy(&pbbase), core::mem::transmute_copy(&dwlength), core::mem::transmute_copy(&dwinflags)) {
                Ok(ok__) => {
                    pdwoutflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrongNameTokenFromAssembly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameTokenFromAssembly(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&ppbstrongnametoken), core::mem::transmute_copy(&pcbstrongnametoken)).into()
        }
        unsafe extern "system" fn StrongNameTokenFromAssemblyEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfilepath: windows_core::PCWSTR, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameTokenFromAssemblyEx(this, core::mem::transmute(&pwzfilepath), core::mem::transmute_copy(&ppbstrongnametoken), core::mem::transmute_copy(&pcbstrongnametoken), core::mem::transmute_copy(&ppbpublickeyblob), core::mem::transmute_copy(&pcbpublickeyblob)).into()
        }
        unsafe extern "system" fn StrongNameTokenFromPublicKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpublickeyblob: *const u8, cbpublickeyblob: u32, ppbstrongnametoken: *mut *mut u8, pcbstrongnametoken: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName_Impl::StrongNameTokenFromPublicKey(this, core::mem::transmute_copy(&pbpublickeyblob), core::mem::transmute_copy(&cbpublickeyblob), core::mem::transmute_copy(&ppbstrongnametoken), core::mem::transmute_copy(&pcbstrongnametoken)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHashFromAssemblyFile: GetHashFromAssemblyFile::<Identity, OFFSET>,
            GetHashFromAssemblyFileW: GetHashFromAssemblyFileW::<Identity, OFFSET>,
            GetHashFromBlob: GetHashFromBlob::<Identity, OFFSET>,
            GetHashFromFile: GetHashFromFile::<Identity, OFFSET>,
            GetHashFromFileW: GetHashFromFileW::<Identity, OFFSET>,
            GetHashFromHandle: GetHashFromHandle::<Identity, OFFSET>,
            StrongNameCompareAssemblies: StrongNameCompareAssemblies::<Identity, OFFSET>,
            StrongNameFreeBuffer: StrongNameFreeBuffer::<Identity, OFFSET>,
            StrongNameGetBlob: StrongNameGetBlob::<Identity, OFFSET>,
            StrongNameGetBlobFromImage: StrongNameGetBlobFromImage::<Identity, OFFSET>,
            StrongNameGetPublicKey: StrongNameGetPublicKey::<Identity, OFFSET>,
            StrongNameHashSize: StrongNameHashSize::<Identity, OFFSET>,
            StrongNameKeyDelete: StrongNameKeyDelete::<Identity, OFFSET>,
            StrongNameKeyGen: StrongNameKeyGen::<Identity, OFFSET>,
            StrongNameKeyGenEx: StrongNameKeyGenEx::<Identity, OFFSET>,
            StrongNameKeyInstall: StrongNameKeyInstall::<Identity, OFFSET>,
            StrongNameSignatureGeneration: StrongNameSignatureGeneration::<Identity, OFFSET>,
            StrongNameSignatureGenerationEx: StrongNameSignatureGenerationEx::<Identity, OFFSET>,
            StrongNameSignatureSize: StrongNameSignatureSize::<Identity, OFFSET>,
            StrongNameSignatureVerification: StrongNameSignatureVerification::<Identity, OFFSET>,
            StrongNameSignatureVerificationEx: StrongNameSignatureVerificationEx::<Identity, OFFSET>,
            StrongNameSignatureVerificationFromImage: StrongNameSignatureVerificationFromImage::<Identity, OFFSET>,
            StrongNameTokenFromAssembly: StrongNameTokenFromAssembly::<Identity, OFFSET>,
            StrongNameTokenFromAssemblyEx: StrongNameTokenFromAssemblyEx::<Identity, OFFSET>,
            StrongNameTokenFromPublicKey: StrongNameTokenFromPublicKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRStrongName as windows_core::Interface>::IID
    }
}
pub trait ICLRStrongName2_Impl: Sized {
    fn StrongNameGetPublicKeyEx(&self, pwzkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> windows_core::Result<()>;
    fn StrongNameSignatureVerificationEx2(&self, wszfilepath: &windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *const u8, cbecmapublickey: u32) -> windows_core::Result<u8>;
}
impl windows_core::RuntimeName for ICLRStrongName2 {}
impl ICLRStrongName2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRStrongName2_Vtbl
    where
        Identity: ICLRStrongName2_Impl,
    {
        unsafe extern "system" fn StrongNameGetPublicKeyEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, ppbpublickeyblob: *mut *mut u8, pcbpublickeyblob: *mut u32, uhashalgid: u32, ureserved: u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName2_Impl::StrongNameGetPublicKeyEx(this, core::mem::transmute(&pwzkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&ppbpublickeyblob), core::mem::transmute_copy(&pcbpublickeyblob), core::mem::transmute_copy(&uhashalgid), core::mem::transmute_copy(&ureserved)).into()
        }
        unsafe extern "system" fn StrongNameSignatureVerificationEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilepath: windows_core::PCWSTR, fforceverification: super::super::Foundation::BOOLEAN, pbecmapublickey: *const u8, cbecmapublickey: u32, pfwasverified: *mut u8) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRStrongName2_Impl::StrongNameSignatureVerificationEx2(this, core::mem::transmute(&wszfilepath), core::mem::transmute_copy(&fforceverification), core::mem::transmute_copy(&pbecmapublickey), core::mem::transmute_copy(&cbecmapublickey)) {
                Ok(ok__) => {
                    pfwasverified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StrongNameGetPublicKeyEx: StrongNameGetPublicKeyEx::<Identity, OFFSET>,
            StrongNameSignatureVerificationEx2: StrongNameSignatureVerificationEx2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRStrongName2 as windows_core::Interface>::IID
    }
}
pub trait ICLRStrongName3_Impl: Sized {
    fn StrongNameDigestGenerate(&self, wszfilepath: &windows_core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn StrongNameDigestSign(&self, wszkeycontainer: &windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, pbdigestblob: *const u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn StrongNameDigestEmbed(&self, wszfilepath: &windows_core::PCWSTR, pbsignatureblob: *const u8, cbsignatureblob: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRStrongName3 {}
impl ICLRStrongName3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRStrongName3_Vtbl
    where
        Identity: ICLRStrongName3_Impl,
    {
        unsafe extern "system" fn StrongNameDigestGenerate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilepath: windows_core::PCWSTR, ppbdigestblob: *mut *mut u8, pcbdigestblob: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName3_Impl::StrongNameDigestGenerate(this, core::mem::transmute(&wszfilepath), core::mem::transmute_copy(&ppbdigestblob), core::mem::transmute_copy(&pcbdigestblob), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameDigestSign<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszkeycontainer: windows_core::PCWSTR, pbkeyblob: *const u8, cbkeyblob: u32, pbdigestblob: *const u8, cbdigestblob: u32, hashalgid: u32, ppbsignatureblob: *mut *mut u8, pcbsignatureblob: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName3_Impl::StrongNameDigestSign(this, core::mem::transmute(&wszkeycontainer), core::mem::transmute_copy(&pbkeyblob), core::mem::transmute_copy(&cbkeyblob), core::mem::transmute_copy(&pbdigestblob), core::mem::transmute_copy(&cbdigestblob), core::mem::transmute_copy(&hashalgid), core::mem::transmute_copy(&ppbsignatureblob), core::mem::transmute_copy(&pcbsignatureblob), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn StrongNameDigestEmbed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilepath: windows_core::PCWSTR, pbsignatureblob: *const u8, cbsignatureblob: u32) -> windows_core::HRESULT
        where
            Identity: ICLRStrongName3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRStrongName3_Impl::StrongNameDigestEmbed(this, core::mem::transmute(&wszfilepath), core::mem::transmute_copy(&pbsignatureblob), core::mem::transmute_copy(&cbsignatureblob)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StrongNameDigestGenerate: StrongNameDigestGenerate::<Identity, OFFSET>,
            StrongNameDigestSign: StrongNameDigestSign::<Identity, OFFSET>,
            StrongNameDigestEmbed: StrongNameDigestEmbed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRStrongName3 as windows_core::Interface>::IID
    }
}
pub trait ICLRSyncManager_Impl: Sized {
    fn GetMonitorOwner(&self, cookie: usize) -> windows_core::Result<IHostTask>;
    fn CreateRWLockOwnerIterator(&self, cookie: usize) -> windows_core::Result<usize>;
    fn GetRWLockOwnerNext(&self, iterator: usize) -> windows_core::Result<IHostTask>;
    fn DeleteRWLockOwnerIterator(&self, iterator: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRSyncManager {}
impl ICLRSyncManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRSyncManager_Vtbl
    where
        Identity: ICLRSyncManager_Impl,
    {
        unsafe extern "system" fn GetMonitorOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: usize, ppownerhosttask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRSyncManager_Impl::GetMonitorOwner(this, core::mem::transmute_copy(&cookie)) {
                Ok(ok__) => {
                    ppownerhosttask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockOwnerIterator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: usize, piterator: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICLRSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRSyncManager_Impl::CreateRWLockOwnerIterator(this, core::mem::transmute_copy(&cookie)) {
                Ok(ok__) => {
                    piterator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRWLockOwnerNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iterator: usize, ppownerhosttask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRSyncManager_Impl::GetRWLockOwnerNext(this, core::mem::transmute_copy(&iterator)) {
                Ok(ok__) => {
                    ppownerhosttask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRWLockOwnerIterator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iterator: usize) -> windows_core::HRESULT
        where
            Identity: ICLRSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRSyncManager_Impl::DeleteRWLockOwnerIterator(this, core::mem::transmute_copy(&iterator)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMonitorOwner: GetMonitorOwner::<Identity, OFFSET>,
            CreateRWLockOwnerIterator: CreateRWLockOwnerIterator::<Identity, OFFSET>,
            GetRWLockOwnerNext: GetRWLockOwnerNext::<Identity, OFFSET>,
            DeleteRWLockOwnerIterator: DeleteRWLockOwnerIterator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRSyncManager as windows_core::Interface>::IID
    }
}
pub trait ICLRTask_Impl: Sized {
    fn SwitchIn(&self, threadhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SwitchOut(&self) -> windows_core::Result<()>;
    fn GetMemStats(&self) -> windows_core::Result<COR_GC_THREAD_STATS>;
    fn Reset(&self, ffull: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ExitTask(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn RudeAbort(&self) -> windows_core::Result<()>;
    fn NeedsPriorityScheduling(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn YieldTask(&self) -> windows_core::Result<()>;
    fn LocksHeld(&self) -> windows_core::Result<usize>;
    fn SetTaskIdentifier(&self, asked: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRTask {}
impl ICLRTask_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRTask_Vtbl
    where
        Identity: ICLRTask_Impl,
    {
        unsafe extern "system" fn SwitchIn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask_Impl::SwitchIn(this, core::mem::transmute_copy(&threadhandle)).into()
        }
        unsafe extern "system" fn SwitchOut<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask_Impl::SwitchOut(this).into()
        }
        unsafe extern "system" fn GetMemStats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, memusage: *mut COR_GC_THREAD_STATS) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRTask_Impl::GetMemStats(this) {
                Ok(ok__) => {
                    memusage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffull: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask_Impl::Reset(this, core::mem::transmute_copy(&ffull)).into()
        }
        unsafe extern "system" fn ExitTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask_Impl::ExitTask(this).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask_Impl::Abort(this).into()
        }
        unsafe extern "system" fn RudeAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask_Impl::RudeAbort(this).into()
        }
        unsafe extern "system" fn NeedsPriorityScheduling<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbneedspriorityscheduling: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRTask_Impl::NeedsPriorityScheduling(this) {
                Ok(ok__) => {
                    pbneedspriorityscheduling.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YieldTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask_Impl::YieldTask(this).into()
        }
        unsafe extern "system" fn LocksHeld<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plockcount: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRTask_Impl::LocksHeld(this) {
                Ok(ok__) => {
                    plockcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, asked: u64) -> windows_core::HRESULT
        where
            Identity: ICLRTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask_Impl::SetTaskIdentifier(this, core::mem::transmute_copy(&asked)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SwitchIn: SwitchIn::<Identity, OFFSET>,
            SwitchOut: SwitchOut::<Identity, OFFSET>,
            GetMemStats: GetMemStats::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ExitTask: ExitTask::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            RudeAbort: RudeAbort::<Identity, OFFSET>,
            NeedsPriorityScheduling: NeedsPriorityScheduling::<Identity, OFFSET>,
            YieldTask: YieldTask::<Identity, OFFSET>,
            LocksHeld: LocksHeld::<Identity, OFFSET>,
            SetTaskIdentifier: SetTaskIdentifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRTask as windows_core::Interface>::IID
    }
}
pub trait ICLRTask2_Impl: Sized + ICLRTask_Impl {
    fn BeginPreventAsyncAbort(&self) -> windows_core::Result<()>;
    fn EndPreventAsyncAbort(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICLRTask2 {}
impl ICLRTask2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRTask2_Vtbl
    where
        Identity: ICLRTask2_Impl,
    {
        unsafe extern "system" fn BeginPreventAsyncAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTask2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask2_Impl::BeginPreventAsyncAbort(this).into()
        }
        unsafe extern "system" fn EndPreventAsyncAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTask2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTask2_Impl::EndPreventAsyncAbort(this).into()
        }
        Self {
            base__: ICLRTask_Vtbl::new::<Identity, OFFSET>(),
            BeginPreventAsyncAbort: BeginPreventAsyncAbort::<Identity, OFFSET>,
            EndPreventAsyncAbort: EndPreventAsyncAbort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRTask2 as windows_core::Interface>::IID || iid == &<ICLRTask as windows_core::Interface>::IID
    }
}
pub trait ICLRTaskManager_Impl: Sized {
    fn CreateTask(&self) -> windows_core::Result<ICLRTask>;
    fn GetCurrentTask(&self) -> windows_core::Result<ICLRTask>;
    fn SetUILocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn SetLocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn GetCurrentTaskType(&self) -> windows_core::Result<ETaskType>;
}
impl windows_core::RuntimeName for ICLRTaskManager {}
impl ICLRTaskManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICLRTaskManager_Vtbl
    where
        Identity: ICLRTaskManager_Impl,
    {
        unsafe extern "system" fn CreateTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRTaskManager_Impl::CreateTask(this) {
                Ok(ok__) => {
                    ptask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICLRTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRTaskManager_Impl::GetCurrentTask(this) {
                Ok(ok__) => {
                    ptask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUILocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT
        where
            Identity: ICLRTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTaskManager_Impl::SetUILocale(this, core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT
        where
            Identity: ICLRTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICLRTaskManager_Impl::SetLocale(this, core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn GetCurrentTaskType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptasktype: *mut ETaskType) -> windows_core::HRESULT
        where
            Identity: ICLRTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICLRTaskManager_Impl::GetCurrentTaskType(this) {
                Ok(ok__) => {
                    ptasktype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTask: CreateTask::<Identity, OFFSET>,
            GetCurrentTask: GetCurrentTask::<Identity, OFFSET>,
            SetUILocale: SetUILocale::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            GetCurrentTaskType: GetCurrentTaskType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICLRTaskManager as windows_core::Interface>::IID
    }
}
pub trait ICatalogServices_Impl: Sized {
    fn Autodone(&self) -> windows_core::Result<()>;
    fn NotAutodone(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICatalogServices {}
impl ICatalogServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICatalogServices_Vtbl
    where
        Identity: ICatalogServices_Impl,
    {
        unsafe extern "system" fn Autodone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalogServices_Impl::Autodone(this).into()
        }
        unsafe extern "system" fn NotAutodone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICatalogServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICatalogServices_Impl::NotAutodone(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Autodone: Autodone::<Identity, OFFSET>,
            NotAutodone: NotAutodone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICatalogServices as windows_core::Interface>::IID
    }
}
pub trait ICorConfiguration_Impl: Sized {
    fn SetGCThreadControl(&self, pgcthreadcontrol: Option<&IGCThreadControl>) -> windows_core::Result<()>;
    fn SetGCHostControl(&self, pgchostcontrol: Option<&IGCHostControl>) -> windows_core::Result<()>;
    fn SetDebuggerThreadControl(&self, pdebuggerthreadcontrol: Option<&IDebuggerThreadControl>) -> windows_core::Result<()>;
    fn AddDebuggerSpecialThread(&self, dwspecialthreadid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorConfiguration {}
impl ICorConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorConfiguration_Vtbl
    where
        Identity: ICorConfiguration_Impl,
    {
        unsafe extern "system" fn SetGCThreadControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgcthreadcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorConfiguration_Impl::SetGCThreadControl(this, windows_core::from_raw_borrowed(&pgcthreadcontrol)).into()
        }
        unsafe extern "system" fn SetGCHostControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgchostcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorConfiguration_Impl::SetGCHostControl(this, windows_core::from_raw_borrowed(&pgchostcontrol)).into()
        }
        unsafe extern "system" fn SetDebuggerThreadControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdebuggerthreadcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorConfiguration_Impl::SetDebuggerThreadControl(this, windows_core::from_raw_borrowed(&pdebuggerthreadcontrol)).into()
        }
        unsafe extern "system" fn AddDebuggerSpecialThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspecialthreadid: u32) -> windows_core::HRESULT
        where
            Identity: ICorConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorConfiguration_Impl::AddDebuggerSpecialThread(this, core::mem::transmute_copy(&dwspecialthreadid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGCThreadControl: SetGCThreadControl::<Identity, OFFSET>,
            SetGCHostControl: SetGCHostControl::<Identity, OFFSET>,
            SetDebuggerThreadControl: SetDebuggerThreadControl::<Identity, OFFSET>,
            AddDebuggerSpecialThread: AddDebuggerSpecialThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorConfiguration as windows_core::Interface>::IID
    }
}
pub trait ICorRuntimeHost_Impl: Sized {
    fn CreateLogicalThreadState(&self) -> windows_core::Result<()>;
    fn DeleteLogicalThreadState(&self) -> windows_core::Result<()>;
    fn SwitchInLogicalThreadState(&self, pfibercookie: *const u32) -> windows_core::Result<()>;
    fn SwitchOutLogicalThreadState(&self) -> windows_core::Result<*mut u32>;
    fn LocksHeldByLogicalThread(&self) -> windows_core::Result<u32>;
    fn MapFile(&self, hfile: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::HMODULE>;
    fn GetConfiguration(&self) -> windows_core::Result<ICorConfiguration>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn CreateDomain(&self, pwzfriendlyname: &windows_core::PCWSTR, pidentityarray: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
    fn GetDefaultDomain(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn EnumDomains(&self, henum: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn NextDomain(&self, henum: *const core::ffi::c_void) -> windows_core::Result<windows_core::IUnknown>;
    fn CloseEnum(&self, henum: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateDomainEx(&self, pwzfriendlyname: &windows_core::PCWSTR, psetup: Option<&windows_core::IUnknown>, pevidence: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateDomainSetup(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateEvidence(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn UnloadDomain(&self, pappdomain: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CurrentDomain(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ICorRuntimeHost {}
impl ICorRuntimeHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorRuntimeHost_Vtbl
    where
        Identity: ICorRuntimeHost_Impl,
    {
        unsafe extern "system" fn CreateLogicalThreadState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorRuntimeHost_Impl::CreateLogicalThreadState(this).into()
        }
        unsafe extern "system" fn DeleteLogicalThreadState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorRuntimeHost_Impl::DeleteLogicalThreadState(this).into()
        }
        unsafe extern "system" fn SwitchInLogicalThreadState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfibercookie: *const u32) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorRuntimeHost_Impl::SwitchInLogicalThreadState(this, core::mem::transmute_copy(&pfibercookie)).into()
        }
        unsafe extern "system" fn SwitchOutLogicalThreadState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfibercookie: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::SwitchOutLogicalThreadState(this) {
                Ok(ok__) => {
                    pfibercookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocksHeldByLogicalThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::LocksHeldByLogicalThread(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfile: super::super::Foundation::HANDLE, hmapaddress: *mut super::super::Foundation::HMODULE) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::MapFile(this, core::mem::transmute_copy(&hfile)) {
                Ok(ok__) => {
                    hmapaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfiguration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::GetConfiguration(this) {
                Ok(ok__) => {
                    pconfiguration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorRuntimeHost_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorRuntimeHost_Impl::Stop(this).into()
        }
        unsafe extern "system" fn CreateDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfriendlyname: windows_core::PCWSTR, pidentityarray: *mut core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::CreateDomain(this, core::mem::transmute(&pwzfriendlyname), windows_core::from_raw_borrowed(&pidentityarray)) {
                Ok(ok__) => {
                    pappdomain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::GetDefaultDomain(this) {
                Ok(ok__) => {
                    pappdomain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDomains<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorRuntimeHost_Impl::EnumDomains(this, core::mem::transmute_copy(&henum)).into()
        }
        unsafe extern "system" fn NextDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *const core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::NextDomain(this, core::mem::transmute_copy(&henum)) {
                Ok(ok__) => {
                    pappdomain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, henum: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorRuntimeHost_Impl::CloseEnum(this, core::mem::transmute_copy(&henum)).into()
        }
        unsafe extern "system" fn CreateDomainEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfriendlyname: windows_core::PCWSTR, psetup: *mut core::ffi::c_void, pevidence: *mut core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::CreateDomainEx(this, core::mem::transmute(&pwzfriendlyname), windows_core::from_raw_borrowed(&psetup), windows_core::from_raw_borrowed(&pevidence)) {
                Ok(ok__) => {
                    pappdomain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDomainSetup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomainsetup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::CreateDomainSetup(this) {
                Ok(ok__) => {
                    pappdomainsetup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEvidence<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevidence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::CreateEvidence(this) {
                Ok(ok__) => {
                    pevidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnloadDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomain: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorRuntimeHost_Impl::UnloadDomain(this, windows_core::from_raw_borrowed(&pappdomain)).into()
        }
        unsafe extern "system" fn CurrentDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorRuntimeHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorRuntimeHost_Impl::CurrentDomain(this) {
                Ok(ok__) => {
                    pappdomain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateLogicalThreadState: CreateLogicalThreadState::<Identity, OFFSET>,
            DeleteLogicalThreadState: DeleteLogicalThreadState::<Identity, OFFSET>,
            SwitchInLogicalThreadState: SwitchInLogicalThreadState::<Identity, OFFSET>,
            SwitchOutLogicalThreadState: SwitchOutLogicalThreadState::<Identity, OFFSET>,
            LocksHeldByLogicalThread: LocksHeldByLogicalThread::<Identity, OFFSET>,
            MapFile: MapFile::<Identity, OFFSET>,
            GetConfiguration: GetConfiguration::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            CreateDomain: CreateDomain::<Identity, OFFSET>,
            GetDefaultDomain: GetDefaultDomain::<Identity, OFFSET>,
            EnumDomains: EnumDomains::<Identity, OFFSET>,
            NextDomain: NextDomain::<Identity, OFFSET>,
            CloseEnum: CloseEnum::<Identity, OFFSET>,
            CreateDomainEx: CreateDomainEx::<Identity, OFFSET>,
            CreateDomainSetup: CreateDomainSetup::<Identity, OFFSET>,
            CreateEvidence: CreateEvidence::<Identity, OFFSET>,
            UnloadDomain: UnloadDomain::<Identity, OFFSET>,
            CurrentDomain: CurrentDomain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorRuntimeHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
pub trait ICorThreadpool_Impl: Sized {
    fn CorRegisterWaitForSingleObject(&self, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CorUnregisterWait(&self, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CorQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CorCreateTimer(&self, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const core::ffi::c_void, duetime: u32, period: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CorChangeTimer(&self, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CorDeleteTimer(&self, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CorBindIoCompletionCallback(&self, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::Result<()>;
    fn CorCallOrQueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn CorSetMaxThreads(&self, maxworkerthreads: u32, maxiocompletionthreads: u32) -> windows_core::Result<()>;
    fn CorGetMaxThreads(&self, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> windows_core::Result<()>;
    fn CorGetAvailableThreads(&self, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl windows_core::RuntimeName for ICorThreadpool {}
#[cfg(all(feature = "Win32_System_IO", feature = "Win32_System_Threading"))]
impl ICorThreadpool_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorThreadpool_Vtbl
    where
        Identity: ICorThreadpool_Impl,
    {
        unsafe extern "system" fn CorRegisterWaitForSingleObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnewwaitobject: *const super::super::Foundation::HANDLE, hwaitobject: super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, context: *const core::ffi::c_void, timeout: u32, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorThreadpool_Impl::CorRegisterWaitForSingleObject(this, core::mem::transmute_copy(&phnewwaitobject), core::mem::transmute_copy(&hwaitobject), core::mem::transmute_copy(&callback), core::mem::transmute_copy(&context), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&executeonlyonce)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorUnregisterWait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwaitobject: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorThreadpool_Impl::CorUnregisterWait(this, core::mem::transmute_copy(&hwaitobject), core::mem::transmute_copy(&completionevent)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorQueueUserWorkItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, executeonlyonce: super::super::Foundation::BOOL, result: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorThreadpool_Impl::CorQueueUserWorkItem(this, core::mem::transmute_copy(&function), core::mem::transmute_copy(&context), core::mem::transmute_copy(&executeonlyonce)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorCreateTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnewtimer: *const super::super::Foundation::HANDLE, callback: super::Threading::WAITORTIMERCALLBACK, parameter: *const core::ffi::c_void, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorThreadpool_Impl::CorCreateTimer(this, core::mem::transmute_copy(&phnewtimer), core::mem::transmute_copy(&callback), core::mem::transmute_copy(&parameter), core::mem::transmute_copy(&duetime), core::mem::transmute_copy(&period)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorChangeTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32, result: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorThreadpool_Impl::CorChangeTimer(this, core::mem::transmute_copy(&timer), core::mem::transmute_copy(&duetime), core::mem::transmute_copy(&period)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorDeleteTimer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorThreadpool_Impl::CorDeleteTimer(this, core::mem::transmute_copy(&timer), core::mem::transmute_copy(&completionevent)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorBindIoCompletionCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filehandle: super::super::Foundation::HANDLE, callback: super::IO::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorThreadpool_Impl::CorBindIoCompletionCallback(this, core::mem::transmute_copy(&filehandle), core::mem::transmute_copy(&callback)).into()
        }
        unsafe extern "system" fn CorCallOrQueueUserWorkItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, result: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorThreadpool_Impl::CorCallOrQueueUserWorkItem(this, core::mem::transmute_copy(&function), core::mem::transmute_copy(&context)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorSetMaxThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxworkerthreads: u32, maxiocompletionthreads: u32) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorThreadpool_Impl::CorSetMaxThreads(this, core::mem::transmute_copy(&maxworkerthreads), core::mem::transmute_copy(&maxiocompletionthreads)).into()
        }
        unsafe extern "system" fn CorGetMaxThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxworkerthreads: *mut u32, maxiocompletionthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorThreadpool_Impl::CorGetMaxThreads(this, core::mem::transmute_copy(&maxworkerthreads), core::mem::transmute_copy(&maxiocompletionthreads)).into()
        }
        unsafe extern "system" fn CorGetAvailableThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, availableworkerthreads: *mut u32, availableiocompletionthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorThreadpool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorThreadpool_Impl::CorGetAvailableThreads(this, core::mem::transmute_copy(&availableworkerthreads), core::mem::transmute_copy(&availableiocompletionthreads)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CorRegisterWaitForSingleObject: CorRegisterWaitForSingleObject::<Identity, OFFSET>,
            CorUnregisterWait: CorUnregisterWait::<Identity, OFFSET>,
            CorQueueUserWorkItem: CorQueueUserWorkItem::<Identity, OFFSET>,
            CorCreateTimer: CorCreateTimer::<Identity, OFFSET>,
            CorChangeTimer: CorChangeTimer::<Identity, OFFSET>,
            CorDeleteTimer: CorDeleteTimer::<Identity, OFFSET>,
            CorBindIoCompletionCallback: CorBindIoCompletionCallback::<Identity, OFFSET>,
            CorCallOrQueueUserWorkItem: CorCallOrQueueUserWorkItem::<Identity, OFFSET>,
            CorSetMaxThreads: CorSetMaxThreads::<Identity, OFFSET>,
            CorGetMaxThreads: CorGetMaxThreads::<Identity, OFFSET>,
            CorGetAvailableThreads: CorGetAvailableThreads::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorThreadpool as windows_core::Interface>::IID
    }
}
pub trait IDebuggerInfo_Impl: Sized {
    fn IsDebuggerAttached(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IDebuggerInfo {}
impl IDebuggerInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebuggerInfo_Vtbl
    where
        Identity: IDebuggerInfo_Impl,
    {
        unsafe extern "system" fn IsDebuggerAttached<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbattached: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IDebuggerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDebuggerInfo_Impl::IsDebuggerAttached(this) {
                Ok(ok__) => {
                    pbattached.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsDebuggerAttached: IsDebuggerAttached::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebuggerInfo as windows_core::Interface>::IID
    }
}
pub trait IDebuggerThreadControl_Impl: Sized {
    fn ThreadIsBlockingForDebugger(&self) -> windows_core::Result<()>;
    fn ReleaseAllRuntimeThreads(&self) -> windows_core::Result<()>;
    fn StartBlockingForDebugger(&self, dwunused: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDebuggerThreadControl {}
impl IDebuggerThreadControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDebuggerThreadControl_Vtbl
    where
        Identity: IDebuggerThreadControl_Impl,
    {
        unsafe extern "system" fn ThreadIsBlockingForDebugger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDebuggerThreadControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDebuggerThreadControl_Impl::ThreadIsBlockingForDebugger(this).into()
        }
        unsafe extern "system" fn ReleaseAllRuntimeThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDebuggerThreadControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDebuggerThreadControl_Impl::ReleaseAllRuntimeThreads(this).into()
        }
        unsafe extern "system" fn StartBlockingForDebugger<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwunused: u32) -> windows_core::HRESULT
        where
            Identity: IDebuggerThreadControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDebuggerThreadControl_Impl::StartBlockingForDebugger(this, core::mem::transmute_copy(&dwunused)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForDebugger: ThreadIsBlockingForDebugger::<Identity, OFFSET>,
            ReleaseAllRuntimeThreads: ReleaseAllRuntimeThreads::<Identity, OFFSET>,
            StartBlockingForDebugger: StartBlockingForDebugger::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebuggerThreadControl as windows_core::Interface>::IID
    }
}
pub trait IGCHost_Impl: Sized {
    fn SetGCStartupLimits(&self, segmentsize: u32, maxgen0size: u32) -> windows_core::Result<()>;
    fn Collect(&self, generation: i32) -> windows_core::Result<()>;
    fn GetStats(&self, pstats: *mut COR_GC_STATS) -> windows_core::Result<()>;
    fn GetThreadStats(&self, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> windows_core::Result<()>;
    fn SetVirtualMemLimit(&self, sztmaxvirtualmemmb: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGCHost {}
impl IGCHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGCHost_Vtbl
    where
        Identity: IGCHost_Impl,
    {
        unsafe extern "system" fn SetGCStartupLimits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentsize: u32, maxgen0size: u32) -> windows_core::HRESULT
        where
            Identity: IGCHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCHost_Impl::SetGCStartupLimits(this, core::mem::transmute_copy(&segmentsize), core::mem::transmute_copy(&maxgen0size)).into()
        }
        unsafe extern "system" fn Collect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, generation: i32) -> windows_core::HRESULT
        where
            Identity: IGCHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCHost_Impl::Collect(this, core::mem::transmute_copy(&generation)).into()
        }
        unsafe extern "system" fn GetStats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstats: *mut COR_GC_STATS) -> windows_core::HRESULT
        where
            Identity: IGCHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCHost_Impl::GetStats(this, core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetThreadStats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfibercookie: *const u32, pstats: *mut COR_GC_THREAD_STATS) -> windows_core::HRESULT
        where
            Identity: IGCHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCHost_Impl::GetThreadStats(this, core::mem::transmute_copy(&pfibercookie), core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetVirtualMemLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztmaxvirtualmemmb: usize) -> windows_core::HRESULT
        where
            Identity: IGCHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCHost_Impl::SetVirtualMemLimit(this, core::mem::transmute_copy(&sztmaxvirtualmemmb)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGCStartupLimits: SetGCStartupLimits::<Identity, OFFSET>,
            Collect: Collect::<Identity, OFFSET>,
            GetStats: GetStats::<Identity, OFFSET>,
            GetThreadStats: GetThreadStats::<Identity, OFFSET>,
            SetVirtualMemLimit: SetVirtualMemLimit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGCHost as windows_core::Interface>::IID
    }
}
pub trait IGCHost2_Impl: Sized + IGCHost_Impl {
    fn SetGCStartupLimitsEx(&self, segmentsize: usize, maxgen0size: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGCHost2 {}
impl IGCHost2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGCHost2_Vtbl
    where
        Identity: IGCHost2_Impl,
    {
        unsafe extern "system" fn SetGCStartupLimitsEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentsize: usize, maxgen0size: usize) -> windows_core::HRESULT
        where
            Identity: IGCHost2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCHost2_Impl::SetGCStartupLimitsEx(this, core::mem::transmute_copy(&segmentsize), core::mem::transmute_copy(&maxgen0size)).into()
        }
        Self { base__: IGCHost_Vtbl::new::<Identity, OFFSET>(), SetGCStartupLimitsEx: SetGCStartupLimitsEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGCHost2 as windows_core::Interface>::IID || iid == &<IGCHost as windows_core::Interface>::IID
    }
}
pub trait IGCHostControl_Impl: Sized {
    fn RequestVirtualMemLimit(&self, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGCHostControl {}
impl IGCHostControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGCHostControl_Vtbl
    where
        Identity: IGCHostControl_Impl,
    {
        unsafe extern "system" fn RequestVirtualMemLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztmaxvirtualmemmb: usize, psztnewmaxvirtualmemmb: *mut usize) -> windows_core::HRESULT
        where
            Identity: IGCHostControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCHostControl_Impl::RequestVirtualMemLimit(this, core::mem::transmute_copy(&sztmaxvirtualmemmb), core::mem::transmute_copy(&psztnewmaxvirtualmemmb)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestVirtualMemLimit: RequestVirtualMemLimit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGCHostControl as windows_core::Interface>::IID
    }
}
pub trait IGCThreadControl_Impl: Sized {
    fn ThreadIsBlockingForSuspension(&self) -> windows_core::Result<()>;
    fn SuspensionStarting(&self) -> windows_core::Result<()>;
    fn SuspensionEnding(&self, generation: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGCThreadControl {}
impl IGCThreadControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGCThreadControl_Vtbl
    where
        Identity: IGCThreadControl_Impl,
    {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGCThreadControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCThreadControl_Impl::ThreadIsBlockingForSuspension(this).into()
        }
        unsafe extern "system" fn SuspensionStarting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGCThreadControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCThreadControl_Impl::SuspensionStarting(this).into()
        }
        unsafe extern "system" fn SuspensionEnding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, generation: u32) -> windows_core::HRESULT
        where
            Identity: IGCThreadControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGCThreadControl_Impl::SuspensionEnding(this, core::mem::transmute_copy(&generation)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGCThreadControl as windows_core::Interface>::IID
    }
}
pub trait IHostAssemblyManager_Impl: Sized {
    fn GetNonHostStoreAssemblies(&self) -> windows_core::Result<ICLRAssemblyReferenceList>;
    fn GetAssemblyStore(&self) -> windows_core::Result<IHostAssemblyStore>;
}
impl windows_core::RuntimeName for IHostAssemblyManager {}
impl IHostAssemblyManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostAssemblyManager_Vtbl
    where
        Identity: IHostAssemblyManager_Impl,
    {
        unsafe extern "system" fn GetNonHostStoreAssemblies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreferencelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostAssemblyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostAssemblyManager_Impl::GetNonHostStoreAssemblies(this) {
                Ok(ok__) => {
                    ppreferencelist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAssemblyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppassemblystore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostAssemblyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostAssemblyManager_Impl::GetAssemblyStore(this) {
                Ok(ok__) => {
                    ppassemblystore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNonHostStoreAssemblies: GetNonHostStoreAssemblies::<Identity, OFFSET>,
            GetAssemblyStore: GetAssemblyStore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostAssemblyManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IHostAssemblyStore_Impl: Sized {
    fn ProvideAssembly(&self, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut Option<super::Com::IStream>, ppstmpdb: *mut Option<super::Com::IStream>) -> windows_core::Result<()>;
    fn ProvideModule(&self, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut Option<super::Com::IStream>, ppstmpdb: *mut Option<super::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IHostAssemblyStore {}
#[cfg(feature = "Win32_System_Com")]
impl IHostAssemblyStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostAssemblyStore_Vtbl
    where
        Identity: IHostAssemblyStore_Impl,
    {
        unsafe extern "system" fn ProvideAssembly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindinfo: *const AssemblyBindInfo, passemblyid: *mut u64, pcontext: *mut u64, ppstmassemblyimage: *mut *mut core::ffi::c_void, ppstmpdb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostAssemblyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostAssemblyStore_Impl::ProvideAssembly(this, core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&passemblyid), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&ppstmassemblyimage), core::mem::transmute_copy(&ppstmpdb)).into()
        }
        unsafe extern "system" fn ProvideModule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindinfo: *const ModuleBindInfo, pdwmoduleid: *mut u32, ppstmmoduleimage: *mut *mut core::ffi::c_void, ppstmpdb: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostAssemblyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostAssemblyStore_Impl::ProvideModule(this, core::mem::transmute_copy(&pbindinfo), core::mem::transmute_copy(&pdwmoduleid), core::mem::transmute_copy(&ppstmmoduleimage), core::mem::transmute_copy(&ppstmpdb)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProvideAssembly: ProvideAssembly::<Identity, OFFSET>,
            ProvideModule: ProvideModule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostAssemblyStore as windows_core::Interface>::IID
    }
}
pub trait IHostAutoEvent_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn Set(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostAutoEvent {}
impl IHostAutoEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostAutoEvent_Vtbl
    where
        Identity: IHostAutoEvent_Impl,
    {
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT
        where
            Identity: IHostAutoEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostAutoEvent_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Set<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostAutoEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostAutoEvent_Impl::Set(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Wait: Wait::<Identity, OFFSET>, Set: Set::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostAutoEvent as windows_core::Interface>::IID
    }
}
pub trait IHostControl_Impl: Sized {
    fn GetHostManager(&self, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetAppDomainManager(&self, dwappdomainid: u32, punkappdomainmanager: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostControl {}
impl IHostControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostControl_Vtbl
    where
        Identity: IHostControl_Impl,
    {
        unsafe extern "system" fn GetHostManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostControl_Impl::GetHostManager(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn SetAppDomainManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwappdomainid: u32, punkappdomainmanager: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostControl_Impl::SetAppDomainManager(this, core::mem::transmute_copy(&dwappdomainid), windows_core::from_raw_borrowed(&punkappdomainmanager)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHostManager: GetHostManager::<Identity, OFFSET>,
            SetAppDomainManager: SetAppDomainManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostControl as windows_core::Interface>::IID
    }
}
pub trait IHostCrst_Impl: Sized {
    fn Enter(&self, option: u32) -> windows_core::Result<()>;
    fn Leave(&self) -> windows_core::Result<()>;
    fn TryEnter(&self, option: u32) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSpinCount(&self, dwspincount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostCrst {}
impl IHostCrst_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostCrst_Vtbl
    where
        Identity: IHostCrst_Impl,
    {
        unsafe extern "system" fn Enter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: u32) -> windows_core::HRESULT
        where
            Identity: IHostCrst_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostCrst_Impl::Enter(this, core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Leave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostCrst_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostCrst_Impl::Leave(this).into()
        }
        unsafe extern "system" fn TryEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: u32, pbsucceeded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHostCrst_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostCrst_Impl::TryEnter(this, core::mem::transmute_copy(&option)) {
                Ok(ok__) => {
                    pbsucceeded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpinCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspincount: u32) -> windows_core::HRESULT
        where
            Identity: IHostCrst_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostCrst_Impl::SetSpinCount(this, core::mem::transmute_copy(&dwspincount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, OFFSET>,
            Leave: Leave::<Identity, OFFSET>,
            TryEnter: TryEnter::<Identity, OFFSET>,
            SetSpinCount: SetSpinCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostCrst as windows_core::Interface>::IID
    }
}
pub trait IHostGCManager_Impl: Sized {
    fn ThreadIsBlockingForSuspension(&self) -> windows_core::Result<()>;
    fn SuspensionStarting(&self) -> windows_core::Result<()>;
    fn SuspensionEnding(&self, generation: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostGCManager {}
impl IHostGCManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostGCManager_Vtbl
    where
        Identity: IHostGCManager_Impl,
    {
        unsafe extern "system" fn ThreadIsBlockingForSuspension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostGCManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostGCManager_Impl::ThreadIsBlockingForSuspension(this).into()
        }
        unsafe extern "system" fn SuspensionStarting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostGCManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostGCManager_Impl::SuspensionStarting(this).into()
        }
        unsafe extern "system" fn SuspensionEnding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, generation: u32) -> windows_core::HRESULT
        where
            Identity: IHostGCManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostGCManager_Impl::SuspensionEnding(this, core::mem::transmute_copy(&generation)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadIsBlockingForSuspension: ThreadIsBlockingForSuspension::<Identity, OFFSET>,
            SuspensionStarting: SuspensionStarting::<Identity, OFFSET>,
            SuspensionEnding: SuspensionEnding::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostGCManager as windows_core::Interface>::IID
    }
}
pub trait IHostIoCompletionManager_Impl: Sized {
    fn CreateIoCompletionPort(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn CloseIoCompletionPort(&self, hport: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetMaxThreads(&self, dwmaxiocompletionthreads: u32) -> windows_core::Result<()>;
    fn GetMaxThreads(&self) -> windows_core::Result<u32>;
    fn GetAvailableThreads(&self) -> windows_core::Result<u32>;
    fn GetHostOverlappedSize(&self) -> windows_core::Result<u32>;
    fn SetCLRIoCompletionManager(&self, pmanager: Option<&ICLRIoCompletionManager>) -> windows_core::Result<()>;
    fn InitializeHostOverlapped(&self, pvoverlapped: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Bind(&self, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> windows_core::Result<()>;
    fn GetMinThreads(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IHostIoCompletionManager {}
impl IHostIoCompletionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostIoCompletionManager_Vtbl
    where
        Identity: IHostIoCompletionManager_Impl,
    {
        unsafe extern "system" fn CreateIoCompletionPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phport: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostIoCompletionManager_Impl::CreateIoCompletionPort(this) {
                Ok(ok__) => {
                    phport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseIoCompletionPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hport: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostIoCompletionManager_Impl::CloseIoCompletionPort(this, core::mem::transmute_copy(&hport)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxiocompletionthreads: u32) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostIoCompletionManager_Impl::SetMaxThreads(this, core::mem::transmute_copy(&dwmaxiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxiocompletionthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostIoCompletionManager_Impl::GetMaxThreads(this) {
                Ok(ok__) => {
                    pdwmaxiocompletionthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwavailableiocompletionthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostIoCompletionManager_Impl::GetAvailableThreads(this) {
                Ok(ok__) => {
                    pdwavailableiocompletionthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostOverlappedSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostIoCompletionManager_Impl::GetHostOverlappedSize(this) {
                Ok(ok__) => {
                    pcbsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLRIoCompletionManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmanager: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostIoCompletionManager_Impl::SetCLRIoCompletionManager(this, windows_core::from_raw_borrowed(&pmanager)).into()
        }
        unsafe extern "system" fn InitializeHostOverlapped<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoverlapped: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostIoCompletionManager_Impl::InitializeHostOverlapped(this, core::mem::transmute_copy(&pvoverlapped)).into()
        }
        unsafe extern "system" fn Bind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hport: super::super::Foundation::HANDLE, hhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostIoCompletionManager_Impl::Bind(this, core::mem::transmute_copy(&hport), core::mem::transmute_copy(&hhandle)).into()
        }
        unsafe extern "system" fn SetMinThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminiocompletionthreads: u32) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostIoCompletionManager_Impl::SetMinThreads(this, core::mem::transmute_copy(&dwminiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMinThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostIoCompletionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostIoCompletionManager_Impl::GetMinThreads(this) {
                Ok(ok__) => {
                    pdwminiocompletionthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateIoCompletionPort: CreateIoCompletionPort::<Identity, OFFSET>,
            CloseIoCompletionPort: CloseIoCompletionPort::<Identity, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, OFFSET>,
            GetHostOverlappedSize: GetHostOverlappedSize::<Identity, OFFSET>,
            SetCLRIoCompletionManager: SetCLRIoCompletionManager::<Identity, OFFSET>,
            InitializeHostOverlapped: InitializeHostOverlapped::<Identity, OFFSET>,
            Bind: Bind::<Identity, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostIoCompletionManager as windows_core::Interface>::IID
    }
}
pub trait IHostMalloc_Impl: Sized {
    fn Alloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DebugAlloc(&self, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Free(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostMalloc {}
impl IHostMalloc_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostMalloc_Vtbl
    where
        Identity: IHostMalloc_Impl,
    {
        unsafe extern "system" fn Alloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMalloc_Impl::Alloc(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&ecriticallevel), core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn DebugAlloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: usize, ecriticallevel: EMemoryCriticalLevel, pszfilename: *const u8, ilineno: i32, ppmem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMalloc_Impl::DebugAlloc(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&ecriticallevel), core::mem::transmute_copy(&pszfilename), core::mem::transmute_copy(&ilineno), core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn Free<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMalloc_Impl::Free(this, core::mem::transmute_copy(&pmem)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alloc: Alloc::<Identity, OFFSET>,
            DebugAlloc: DebugAlloc::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostMalloc as windows_core::Interface>::IID
    }
}
pub trait IHostManualEvent_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Set(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostManualEvent {}
impl IHostManualEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostManualEvent_Vtbl
    where
        Identity: IHostManualEvent_Impl,
    {
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT
        where
            Identity: IHostManualEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostManualEvent_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostManualEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostManualEvent_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Set<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostManualEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostManualEvent_Impl::Set(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Set: Set::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostManualEvent as windows_core::Interface>::IID
    }
}
pub trait IHostMemoryManager_Impl: Sized {
    fn CreateMalloc(&self, dwmalloctype: u32) -> windows_core::Result<IHostMalloc>;
    fn VirtualAlloc(&self, paddress: *const core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn VirtualFree(&self, lpaddress: *const core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> windows_core::Result<()>;
    fn VirtualQuery(&self, lpaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwlength: usize, presult: *mut usize) -> windows_core::Result<()>;
    fn VirtualProtect(&self, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: u32) -> windows_core::Result<u32>;
    fn GetMemoryLoad(&self, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> windows_core::Result<()>;
    fn RegisterMemoryNotificationCallback(&self, pcallback: Option<&ICLRMemoryNotificationCallback>) -> windows_core::Result<()>;
    fn NeedsVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<()>;
    fn AcquiredVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::Result<()>;
    fn ReleasedVirtualAddressSpace(&self, startaddress: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostMemoryManager {}
impl IHostMemoryManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostMemoryManager_Vtbl
    where
        Identity: IHostMemoryManager_Impl,
    {
        unsafe extern "system" fn CreateMalloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmalloctype: u32, ppmalloc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostMemoryManager_Impl::CreateMalloc(this, core::mem::transmute_copy(&dwmalloctype)) {
                Ok(ok__) => {
                    ppmalloc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualAlloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *const core::ffi::c_void, dwsize: usize, flallocationtype: u32, flprotect: u32, ecriticallevel: EMemoryCriticalLevel, ppmem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMemoryManager_Impl::VirtualAlloc(this, core::mem::transmute_copy(&paddress), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&flallocationtype), core::mem::transmute_copy(&flprotect), core::mem::transmute_copy(&ecriticallevel), core::mem::transmute_copy(&ppmem)).into()
        }
        unsafe extern "system" fn VirtualFree<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpaddress: *const core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMemoryManager_Impl::VirtualFree(this, core::mem::transmute_copy(&lpaddress), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&dwfreetype)).into()
        }
        unsafe extern "system" fn VirtualQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, dwlength: usize, presult: *mut usize) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMemoryManager_Impl::VirtualQuery(this, core::mem::transmute_copy(&lpaddress), core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&dwlength), core::mem::transmute_copy(&presult)).into()
        }
        unsafe extern "system" fn VirtualProtect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: u32, pfloldprotect: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostMemoryManager_Impl::VirtualProtect(this, core::mem::transmute_copy(&lpaddress), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&flnewprotect)) {
                Ok(ok__) => {
                    pfloldprotect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemoryLoad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmemoryload: *mut u32, pavailablebytes: *mut usize) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMemoryManager_Impl::GetMemoryLoad(this, core::mem::transmute_copy(&pmemoryload), core::mem::transmute_copy(&pavailablebytes)).into()
        }
        unsafe extern "system" fn RegisterMemoryNotificationCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMemoryManager_Impl::RegisterMemoryNotificationCallback(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn NeedsVirtualAddressSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMemoryManager_Impl::NeedsVirtualAddressSpace(this, core::mem::transmute_copy(&startaddress), core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn AcquiredVirtualAddressSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startaddress: *const core::ffi::c_void, size: usize) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMemoryManager_Impl::AcquiredVirtualAddressSpace(this, core::mem::transmute_copy(&startaddress), core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn ReleasedVirtualAddressSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startaddress: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostMemoryManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostMemoryManager_Impl::ReleasedVirtualAddressSpace(this, core::mem::transmute_copy(&startaddress)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMalloc: CreateMalloc::<Identity, OFFSET>,
            VirtualAlloc: VirtualAlloc::<Identity, OFFSET>,
            VirtualFree: VirtualFree::<Identity, OFFSET>,
            VirtualQuery: VirtualQuery::<Identity, OFFSET>,
            VirtualProtect: VirtualProtect::<Identity, OFFSET>,
            GetMemoryLoad: GetMemoryLoad::<Identity, OFFSET>,
            RegisterMemoryNotificationCallback: RegisterMemoryNotificationCallback::<Identity, OFFSET>,
            NeedsVirtualAddressSpace: NeedsVirtualAddressSpace::<Identity, OFFSET>,
            AcquiredVirtualAddressSpace: AcquiredVirtualAddressSpace::<Identity, OFFSET>,
            ReleasedVirtualAddressSpace: ReleasedVirtualAddressSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostMemoryManager as windows_core::Interface>::IID
    }
}
pub trait IHostPolicyManager_Impl: Sized {
    fn OnDefaultAction(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()>;
    fn OnTimeout(&self, operation: EClrOperation, action: EPolicyAction) -> windows_core::Result<()>;
    fn OnFailure(&self, failure: EClrFailure, action: EPolicyAction) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostPolicyManager {}
impl IHostPolicyManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostPolicyManager_Vtbl
    where
        Identity: IHostPolicyManager_Impl,
    {
        unsafe extern "system" fn OnDefaultAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> windows_core::HRESULT
        where
            Identity: IHostPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostPolicyManager_Impl::OnDefaultAction(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn OnTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: EClrOperation, action: EPolicyAction) -> windows_core::HRESULT
        where
            Identity: IHostPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostPolicyManager_Impl::OnTimeout(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn OnFailure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, failure: EClrFailure, action: EPolicyAction) -> windows_core::HRESULT
        where
            Identity: IHostPolicyManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostPolicyManager_Impl::OnFailure(this, core::mem::transmute_copy(&failure), core::mem::transmute_copy(&action)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDefaultAction: OnDefaultAction::<Identity, OFFSET>,
            OnTimeout: OnTimeout::<Identity, OFFSET>,
            OnFailure: OnFailure::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostPolicyManager as windows_core::Interface>::IID
    }
}
pub trait IHostSecurityContext_Impl: Sized {
    fn Capture(&self) -> windows_core::Result<IHostSecurityContext>;
}
impl windows_core::RuntimeName for IHostSecurityContext {}
impl IHostSecurityContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostSecurityContext_Vtbl
    where
        Identity: IHostSecurityContext_Impl,
    {
        unsafe extern "system" fn Capture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclonedcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSecurityContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSecurityContext_Impl::Capture(this) {
                Ok(ok__) => {
                    ppclonedcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Capture: Capture::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostSecurityContext as windows_core::Interface>::IID
    }
}
pub trait IHostSecurityManager_Impl: Sized {
    fn ImpersonateLoggedOnUser(&self, htoken: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn RevertToSelf(&self) -> windows_core::Result<()>;
    fn OpenThreadToken(&self, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn SetThreadToken(&self, htoken: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetSecurityContext(&self, econtexttype: EContextType) -> windows_core::Result<IHostSecurityContext>;
    fn SetSecurityContext(&self, econtexttype: EContextType, psecuritycontext: Option<&IHostSecurityContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostSecurityManager {}
impl IHostSecurityManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostSecurityManager_Vtbl
    where
        Identity: IHostSecurityManager_Impl,
    {
        unsafe extern "system" fn ImpersonateLoggedOnUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostSecurityManager_Impl::ImpersonateLoggedOnUser(this, core::mem::transmute_copy(&htoken)).into()
        }
        unsafe extern "system" fn RevertToSelf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostSecurityManager_Impl::RevertToSelf(this).into()
        }
        unsafe extern "system" fn OpenThreadToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdesiredaccess: u32, bopenasself: super::super::Foundation::BOOL, phthreadtoken: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSecurityManager_Impl::OpenThreadToken(this, core::mem::transmute_copy(&dwdesiredaccess), core::mem::transmute_copy(&bopenasself)) {
                Ok(ok__) => {
                    phthreadtoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThreadToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, htoken: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostSecurityManager_Impl::SetThreadToken(this, core::mem::transmute_copy(&htoken)).into()
        }
        unsafe extern "system" fn GetSecurityContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, econtexttype: EContextType, ppsecuritycontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSecurityManager_Impl::GetSecurityContext(this, core::mem::transmute_copy(&econtexttype)) {
                Ok(ok__) => {
                    ppsecuritycontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, econtexttype: EContextType, psecuritycontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSecurityManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostSecurityManager_Impl::SetSecurityContext(this, core::mem::transmute_copy(&econtexttype), windows_core::from_raw_borrowed(&psecuritycontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ImpersonateLoggedOnUser: ImpersonateLoggedOnUser::<Identity, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, OFFSET>,
            OpenThreadToken: OpenThreadToken::<Identity, OFFSET>,
            SetThreadToken: SetThreadToken::<Identity, OFFSET>,
            GetSecurityContext: GetSecurityContext::<Identity, OFFSET>,
            SetSecurityContext: SetSecurityContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostSecurityManager as windows_core::Interface>::IID
    }
}
pub trait IHostSemaphore_Impl: Sized {
    fn Wait(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn ReleaseSemaphore(&self, lreleasecount: i32) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IHostSemaphore {}
impl IHostSemaphore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostSemaphore_Vtbl
    where
        Identity: IHostSemaphore_Impl,
    {
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT
        where
            Identity: IHostSemaphore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostSemaphore_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn ReleaseSemaphore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lreleasecount: i32, lppreviouscount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IHostSemaphore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSemaphore_Impl::ReleaseSemaphore(this, core::mem::transmute_copy(&lreleasecount)) {
                Ok(ok__) => {
                    lppreviouscount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, OFFSET>,
            ReleaseSemaphore: ReleaseSemaphore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostSemaphore as windows_core::Interface>::IID
    }
}
pub trait IHostSyncManager_Impl: Sized {
    fn SetCLRSyncManager(&self, pmanager: Option<&ICLRSyncManager>) -> windows_core::Result<()>;
    fn CreateCrst(&self) -> windows_core::Result<IHostCrst>;
    fn CreateCrstWithSpinCount(&self, dwspincount: u32) -> windows_core::Result<IHostCrst>;
    fn CreateAutoEvent(&self) -> windows_core::Result<IHostAutoEvent>;
    fn CreateManualEvent(&self, binitialstate: super::super::Foundation::BOOL) -> windows_core::Result<IHostManualEvent>;
    fn CreateMonitorEvent(&self, cookie: usize) -> windows_core::Result<IHostAutoEvent>;
    fn CreateRWLockWriterEvent(&self, cookie: usize) -> windows_core::Result<IHostAutoEvent>;
    fn CreateRWLockReaderEvent(&self, binitialstate: super::super::Foundation::BOOL, cookie: usize) -> windows_core::Result<IHostManualEvent>;
    fn CreateSemaphoreA(&self, dwinitial: u32, dwmax: u32) -> windows_core::Result<IHostSemaphore>;
}
impl windows_core::RuntimeName for IHostSyncManager {}
impl IHostSyncManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostSyncManager_Vtbl
    where
        Identity: IHostSyncManager_Impl,
    {
        unsafe extern "system" fn SetCLRSyncManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmanager: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostSyncManager_Impl::SetCLRSyncManager(this, windows_core::from_raw_borrowed(&pmanager)).into()
        }
        unsafe extern "system" fn CreateCrst<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcrst: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSyncManager_Impl::CreateCrst(this) {
                Ok(ok__) => {
                    ppcrst.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCrstWithSpinCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspincount: u32, ppcrst: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSyncManager_Impl::CreateCrstWithSpinCount(this, core::mem::transmute_copy(&dwspincount)) {
                Ok(ok__) => {
                    ppcrst.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAutoEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSyncManager_Impl::CreateAutoEvent(this) {
                Ok(ok__) => {
                    ppevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateManualEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSyncManager_Impl::CreateManualEvent(this, core::mem::transmute_copy(&binitialstate)) {
                Ok(ok__) => {
                    ppevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMonitorEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: usize, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSyncManager_Impl::CreateMonitorEvent(this, core::mem::transmute_copy(&cookie)) {
                Ok(ok__) => {
                    ppevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockWriterEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: usize, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSyncManager_Impl::CreateRWLockWriterEvent(this, core::mem::transmute_copy(&cookie)) {
                Ok(ok__) => {
                    ppevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRWLockReaderEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, binitialstate: super::super::Foundation::BOOL, cookie: usize, ppevent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSyncManager_Impl::CreateRWLockReaderEvent(this, core::mem::transmute_copy(&binitialstate), core::mem::transmute_copy(&cookie)) {
                Ok(ok__) => {
                    ppevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSemaphoreA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinitial: u32, dwmax: u32, ppsemaphore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostSyncManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostSyncManager_Impl::CreateSemaphoreA(this, core::mem::transmute_copy(&dwinitial), core::mem::transmute_copy(&dwmax)) {
                Ok(ok__) => {
                    ppsemaphore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCLRSyncManager: SetCLRSyncManager::<Identity, OFFSET>,
            CreateCrst: CreateCrst::<Identity, OFFSET>,
            CreateCrstWithSpinCount: CreateCrstWithSpinCount::<Identity, OFFSET>,
            CreateAutoEvent: CreateAutoEvent::<Identity, OFFSET>,
            CreateManualEvent: CreateManualEvent::<Identity, OFFSET>,
            CreateMonitorEvent: CreateMonitorEvent::<Identity, OFFSET>,
            CreateRWLockWriterEvent: CreateRWLockWriterEvent::<Identity, OFFSET>,
            CreateRWLockReaderEvent: CreateRWLockReaderEvent::<Identity, OFFSET>,
            CreateSemaphoreA: CreateSemaphoreA::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostSyncManager as windows_core::Interface>::IID
    }
}
pub trait IHostTask_Impl: Sized {
    fn Start(&self) -> windows_core::Result<()>;
    fn Alert(&self) -> windows_core::Result<()>;
    fn Join(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn SetPriority(&self, newpriority: i32) -> windows_core::Result<()>;
    fn GetPriority(&self) -> windows_core::Result<i32>;
    fn SetCLRTask(&self, pclrtask: Option<&ICLRTask>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IHostTask {}
impl IHostTask_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostTask_Vtbl
    where
        Identity: IHostTask_Impl,
    {
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTask_Impl::Start(this).into()
        }
        unsafe extern "system" fn Alert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTask_Impl::Alert(this).into()
        }
        unsafe extern "system" fn Join<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT
        where
            Identity: IHostTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTask_Impl::Join(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newpriority: i32) -> windows_core::HRESULT
        where
            Identity: IHostTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTask_Impl::SetPriority(this, core::mem::transmute_copy(&newpriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IHostTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostTask_Impl::GetPriority(this) {
                Ok(ok__) => {
                    ppriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLRTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclrtask: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTask_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTask_Impl::SetCLRTask(this, windows_core::from_raw_borrowed(&pclrtask)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Alert: Alert::<Identity, OFFSET>,
            Join: Join::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetCLRTask: SetCLRTask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostTask as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Threading")]
pub trait IHostTaskManager_Impl: Sized {
    fn GetCurrentTask(&self) -> windows_core::Result<IHostTask>;
    fn CreateTask(&self, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const core::ffi::c_void) -> windows_core::Result<IHostTask>;
    fn Sleep(&self, dwmilliseconds: u32, option: u32) -> windows_core::Result<()>;
    fn SwitchToTask(&self, option: u32) -> windows_core::Result<()>;
    fn SetUILocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn SetLocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn CallNeedsHostHook(&self, target: usize) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn LeaveRuntime(&self, target: usize) -> windows_core::Result<()>;
    fn EnterRuntime(&self) -> windows_core::Result<()>;
    fn ReverseLeaveRuntime(&self) -> windows_core::Result<()>;
    fn ReverseEnterRuntime(&self) -> windows_core::Result<()>;
    fn BeginDelayAbort(&self) -> windows_core::Result<()>;
    fn EndDelayAbort(&self) -> windows_core::Result<()>;
    fn BeginThreadAffinity(&self) -> windows_core::Result<()>;
    fn EndThreadAffinity(&self) -> windows_core::Result<()>;
    fn SetStackGuarantee(&self, guarantee: u32) -> windows_core::Result<()>;
    fn GetStackGuarantee(&self) -> windows_core::Result<u32>;
    fn SetCLRTaskManager(&self, ppmanager: Option<&ICLRTaskManager>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Threading")]
impl windows_core::RuntimeName for IHostTaskManager {}
#[cfg(feature = "Win32_System_Threading")]
impl IHostTaskManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostTaskManager_Vtbl
    where
        Identity: IHostTaskManager_Impl,
    {
        unsafe extern "system" fn GetCurrentTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostTaskManager_Impl::GetCurrentTask(this) {
                Ok(ok__) => {
                    ptask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstacksize: u32, pstartaddress: super::Threading::LPTHREAD_START_ROUTINE, pparameter: *const core::ffi::c_void, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostTaskManager_Impl::CreateTask(this, core::mem::transmute_copy(&dwstacksize), core::mem::transmute_copy(&pstartaddress), core::mem::transmute_copy(&pparameter)) {
                Ok(ok__) => {
                    pptask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sleep<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32, option: u32) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::Sleep(this, core::mem::transmute_copy(&dwmilliseconds), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SwitchToTask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: u32) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::SwitchToTask(this, core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn SetUILocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::SetUILocale(this, core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn SetLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::SetLocale(this, core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn CallNeedsHostHook<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: usize, pbcallneedshosthook: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostTaskManager_Impl::CallNeedsHostHook(this, core::mem::transmute_copy(&target)) {
                Ok(ok__) => {
                    pbcallneedshosthook.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaveRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: usize) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::LeaveRuntime(this, core::mem::transmute_copy(&target)).into()
        }
        unsafe extern "system" fn EnterRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::EnterRuntime(this).into()
        }
        unsafe extern "system" fn ReverseLeaveRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::ReverseLeaveRuntime(this).into()
        }
        unsafe extern "system" fn ReverseEnterRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::ReverseEnterRuntime(this).into()
        }
        unsafe extern "system" fn BeginDelayAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::BeginDelayAbort(this).into()
        }
        unsafe extern "system" fn EndDelayAbort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::EndDelayAbort(this).into()
        }
        unsafe extern "system" fn BeginThreadAffinity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::BeginThreadAffinity(this).into()
        }
        unsafe extern "system" fn EndThreadAffinity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::EndThreadAffinity(this).into()
        }
        unsafe extern "system" fn SetStackGuarantee<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guarantee: u32) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::SetStackGuarantee(this, core::mem::transmute_copy(&guarantee)).into()
        }
        unsafe extern "system" fn GetStackGuarantee<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguarantee: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostTaskManager_Impl::GetStackGuarantee(this) {
                Ok(ok__) => {
                    pguarantee.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLRTaskManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanager: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IHostTaskManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostTaskManager_Impl::SetCLRTaskManager(this, windows_core::from_raw_borrowed(&ppmanager)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentTask: GetCurrentTask::<Identity, OFFSET>,
            CreateTask: CreateTask::<Identity, OFFSET>,
            Sleep: Sleep::<Identity, OFFSET>,
            SwitchToTask: SwitchToTask::<Identity, OFFSET>,
            SetUILocale: SetUILocale::<Identity, OFFSET>,
            SetLocale: SetLocale::<Identity, OFFSET>,
            CallNeedsHostHook: CallNeedsHostHook::<Identity, OFFSET>,
            LeaveRuntime: LeaveRuntime::<Identity, OFFSET>,
            EnterRuntime: EnterRuntime::<Identity, OFFSET>,
            ReverseLeaveRuntime: ReverseLeaveRuntime::<Identity, OFFSET>,
            ReverseEnterRuntime: ReverseEnterRuntime::<Identity, OFFSET>,
            BeginDelayAbort: BeginDelayAbort::<Identity, OFFSET>,
            EndDelayAbort: EndDelayAbort::<Identity, OFFSET>,
            BeginThreadAffinity: BeginThreadAffinity::<Identity, OFFSET>,
            EndThreadAffinity: EndThreadAffinity::<Identity, OFFSET>,
            SetStackGuarantee: SetStackGuarantee::<Identity, OFFSET>,
            GetStackGuarantee: GetStackGuarantee::<Identity, OFFSET>,
            SetCLRTaskManager: SetCLRTaskManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostTaskManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Threading")]
pub trait IHostThreadpoolManager_Impl: Sized {
    fn QueueUserWorkItem(&self, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, flags: u32) -> windows_core::Result<()>;
    fn SetMaxThreads(&self, dwmaxworkerthreads: u32) -> windows_core::Result<()>;
    fn GetMaxThreads(&self) -> windows_core::Result<u32>;
    fn GetAvailableThreads(&self) -> windows_core::Result<u32>;
    fn SetMinThreads(&self, dwminiocompletionthreads: u32) -> windows_core::Result<()>;
    fn GetMinThreads(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Threading")]
impl windows_core::RuntimeName for IHostThreadpoolManager {}
#[cfg(feature = "Win32_System_Threading")]
impl IHostThreadpoolManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHostThreadpoolManager_Vtbl
    where
        Identity: IHostThreadpoolManager_Impl,
    {
        unsafe extern "system" fn QueueUserWorkItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, function: super::Threading::LPTHREAD_START_ROUTINE, context: *const core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: IHostThreadpoolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostThreadpoolManager_Impl::QueueUserWorkItem(this, core::mem::transmute_copy(&function), core::mem::transmute_copy(&context), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxworkerthreads: u32) -> windows_core::HRESULT
        where
            Identity: IHostThreadpoolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostThreadpoolManager_Impl::SetMaxThreads(this, core::mem::transmute_copy(&dwmaxworkerthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxworkerthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostThreadpoolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostThreadpoolManager_Impl::GetMaxThreads(this) {
                Ok(ok__) => {
                    pdwmaxworkerthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwavailableworkerthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostThreadpoolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostThreadpoolManager_Impl::GetAvailableThreads(this) {
                Ok(ok__) => {
                    pdwavailableworkerthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwminiocompletionthreads: u32) -> windows_core::HRESULT
        where
            Identity: IHostThreadpoolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHostThreadpoolManager_Impl::SetMinThreads(this, core::mem::transmute_copy(&dwminiocompletionthreads)).into()
        }
        unsafe extern "system" fn GetMinThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminiocompletionthreads: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHostThreadpoolManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHostThreadpoolManager_Impl::GetMinThreads(this) {
                Ok(ok__) => {
                    pdwminiocompletionthreads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueueUserWorkItem: QueueUserWorkItem::<Identity, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, OFFSET>,
            GetMaxThreads: GetMaxThreads::<Identity, OFFSET>,
            GetAvailableThreads: GetAvailableThreads::<Identity, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, OFFSET>,
            GetMinThreads: GetMinThreads::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHostThreadpoolManager as windows_core::Interface>::IID
    }
}
pub trait IManagedObject_Impl: Sized {
    fn GetSerializedBuffer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetObjectIdentity(&self, pbstrguid: *mut windows_core::BSTR, appdomainid: *mut i32, pccw: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IManagedObject {}
impl IManagedObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IManagedObject_Vtbl
    where
        Identity: IManagedObject_Impl,
    {
        unsafe extern "system" fn GetSerializedBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IManagedObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IManagedObject_Impl::GetSerializedBuffer(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectIdentity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguid: *mut core::mem::MaybeUninit<windows_core::BSTR>, appdomainid: *mut i32, pccw: *mut i32) -> windows_core::HRESULT
        where
            Identity: IManagedObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IManagedObject_Impl::GetObjectIdentity(this, core::mem::transmute_copy(&pbstrguid), core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&pccw)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSerializedBuffer: GetSerializedBuffer::<Identity, OFFSET>,
            GetObjectIdentity: GetObjectIdentity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManagedObject as windows_core::Interface>::IID
    }
}
pub trait IObjectHandle_Impl: Sized {
    fn Unwrap(&self) -> windows_core::Result<windows_core::VARIANT>;
}
impl windows_core::RuntimeName for IObjectHandle {}
impl IObjectHandle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectHandle_Vtbl
    where
        Identity: IObjectHandle_Impl,
    {
        unsafe extern "system" fn Unwrap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppv: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IObjectHandle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectHandle_Impl::Unwrap(this) {
                Ok(ok__) => {
                    ppv.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Unwrap: Unwrap::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectHandle as windows_core::Interface>::IID
    }
}
pub trait ITypeName_Impl: Sized {
    fn GetNameCount(&self) -> windows_core::Result<u32>;
    fn GetNames(&self, count: u32, rgbsznames: *mut windows_core::BSTR) -> windows_core::Result<u32>;
    fn GetTypeArgumentCount(&self) -> windows_core::Result<u32>;
    fn GetTypeArguments(&self, count: u32, rgparguments: *mut Option<ITypeName>) -> windows_core::Result<u32>;
    fn GetModifierLength(&self) -> windows_core::Result<u32>;
    fn GetModifiers(&self, count: u32, rgmodifiers: *mut u32) -> windows_core::Result<u32>;
    fn GetAssemblyName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl windows_core::RuntimeName for ITypeName {}
impl ITypeName_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeName_Vtbl
    where
        Identity: ITypeName_Impl,
    {
        unsafe extern "system" fn GetNameCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeName_Impl::GetNameCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, rgbsznames: *mut core::mem::MaybeUninit<windows_core::BSTR>, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeName_Impl::GetNames(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&rgbsznames)) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeArgumentCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeName_Impl::GetTypeArgumentCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, rgparguments: *mut *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeName_Impl::GetTypeArguments(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&rgparguments)) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModifierLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeName_Impl::GetModifierLength(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModifiers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, rgmodifiers: *mut u32, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITypeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeName_Impl::GetModifiers(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&rgmodifiers)) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAssemblyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgbszassemblynames: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeName_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeName_Impl::GetAssemblyName(this) {
                Ok(ok__) => {
                    rgbszassemblynames.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNameCount: GetNameCount::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            GetTypeArgumentCount: GetTypeArgumentCount::<Identity, OFFSET>,
            GetTypeArguments: GetTypeArguments::<Identity, OFFSET>,
            GetModifierLength: GetModifierLength::<Identity, OFFSET>,
            GetModifiers: GetModifiers::<Identity, OFFSET>,
            GetAssemblyName: GetAssemblyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeName as windows_core::Interface>::IID
    }
}
pub trait ITypeNameBuilder_Impl: Sized {
    fn OpenGenericArguments(&self) -> windows_core::Result<()>;
    fn CloseGenericArguments(&self) -> windows_core::Result<()>;
    fn OpenGenericArgument(&self) -> windows_core::Result<()>;
    fn CloseGenericArgument(&self) -> windows_core::Result<()>;
    fn AddName(&self, szname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddPointer(&self) -> windows_core::Result<()>;
    fn AddByRef(&self) -> windows_core::Result<()>;
    fn AddSzArray(&self) -> windows_core::Result<()>;
    fn AddArray(&self, rank: u32) -> windows_core::Result<()>;
    fn AddAssemblySpec(&self, szassemblyspec: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ToString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITypeNameBuilder {}
impl ITypeNameBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeNameBuilder_Vtbl
    where
        Identity: ITypeNameBuilder_Impl,
    {
        unsafe extern "system" fn OpenGenericArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::OpenGenericArguments(this).into()
        }
        unsafe extern "system" fn CloseGenericArguments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::CloseGenericArguments(this).into()
        }
        unsafe extern "system" fn OpenGenericArgument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::OpenGenericArgument(this).into()
        }
        unsafe extern "system" fn CloseGenericArgument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::CloseGenericArgument(this).into()
        }
        unsafe extern "system" fn AddName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::AddName(this, core::mem::transmute(&szname)).into()
        }
        unsafe extern "system" fn AddPointer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::AddPointer(this).into()
        }
        unsafe extern "system" fn AddByRef<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::AddByRef(this).into()
        }
        unsafe extern "system" fn AddSzArray<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::AddSzArray(this).into()
        }
        unsafe extern "system" fn AddArray<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rank: u32) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::AddArray(this, core::mem::transmute_copy(&rank)).into()
        }
        unsafe extern "system" fn AddAssemblySpec<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szassemblyspec: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::AddAssemblySpec(this, core::mem::transmute(&szassemblyspec)).into()
        }
        unsafe extern "system" fn ToString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstringrepresentation: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeNameBuilder_Impl::ToString(this) {
                Ok(ok__) => {
                    pszstringrepresentation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITypeNameBuilder_Impl::Clear(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenGenericArguments: OpenGenericArguments::<Identity, OFFSET>,
            CloseGenericArguments: CloseGenericArguments::<Identity, OFFSET>,
            OpenGenericArgument: OpenGenericArgument::<Identity, OFFSET>,
            CloseGenericArgument: CloseGenericArgument::<Identity, OFFSET>,
            AddName: AddName::<Identity, OFFSET>,
            AddPointer: AddPointer::<Identity, OFFSET>,
            AddByRef: AddByRef::<Identity, OFFSET>,
            AddSzArray: AddSzArray::<Identity, OFFSET>,
            AddArray: AddArray::<Identity, OFFSET>,
            AddAssemblySpec: AddAssemblySpec::<Identity, OFFSET>,
            ToString: ToString::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeNameBuilder as windows_core::Interface>::IID
    }
}
pub trait ITypeNameFactory_Impl: Sized {
    fn ParseTypeName(&self, szname: &windows_core::PCWSTR, perror: *mut u32) -> windows_core::Result<ITypeName>;
    fn GetTypeNameBuilder(&self) -> windows_core::Result<ITypeNameBuilder>;
}
impl windows_core::RuntimeName for ITypeNameFactory {}
impl ITypeNameFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITypeNameFactory_Vtbl
    where
        Identity: ITypeNameFactory_Impl,
    {
        unsafe extern "system" fn ParseTypeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, perror: *mut u32, pptypename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeNameFactory_Impl::ParseTypeName(this, core::mem::transmute(&szname), core::mem::transmute_copy(&perror)) {
                Ok(ok__) => {
                    pptypename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeNameBuilder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITypeNameFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITypeNameFactory_Impl::GetTypeNameBuilder(this) {
                Ok(ok__) => {
                    pptypebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseTypeName: ParseTypeName::<Identity, OFFSET>,
            GetTypeNameBuilder: GetTypeNameBuilder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITypeNameFactory as windows_core::Interface>::IID
    }
}
