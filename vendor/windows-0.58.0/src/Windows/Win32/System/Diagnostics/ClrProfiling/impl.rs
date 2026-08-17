#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerAssemblyReferenceProvider_Impl: Sized {
    fn AddAssemblyReference(&self, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerAssemblyReferenceProvider {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerAssemblyReferenceProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerAssemblyReferenceProvider_Vtbl
    where
        Identity: ICorProfilerAssemblyReferenceProvider_Impl,
    {
        unsafe extern "system" fn AddAssemblyReference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> windows_core::HRESULT
        where
            Identity: ICorProfilerAssemblyReferenceProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerAssemblyReferenceProvider_Impl::AddAssemblyReference(this, core::mem::transmute_copy(&passemblyrefinfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddAssemblyReference: AddAssemblyReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerAssemblyReferenceProvider as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback_Impl: Sized {
    fn Initialize(&self, picorprofilerinfounk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn AppDomainCreationStarted(&self, appdomainid: usize) -> windows_core::Result<()>;
    fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AppDomainShutdownStarted(&self, appdomainid: usize) -> windows_core::Result<()>;
    fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AssemblyLoadStarted(&self, assemblyid: usize) -> windows_core::Result<()>;
    fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn AssemblyUnloadStarted(&self, assemblyid: usize) -> windows_core::Result<()>;
    fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ModuleLoadStarted(&self, moduleid: usize) -> windows_core::Result<()>;
    fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ModuleUnloadStarted(&self, moduleid: usize) -> windows_core::Result<()>;
    fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> windows_core::Result<()>;
    fn ClassLoadStarted(&self, classid: usize) -> windows_core::Result<()>;
    fn ClassLoadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ClassUnloadStarted(&self, classid: usize) -> windows_core::Result<()>;
    fn ClassUnloadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn FunctionUnloadStarted(&self, functionid: usize) -> windows_core::Result<()>;
    fn JITCompilationStarted(&self, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn JITCompilationFinished(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn JITCachedFunctionSearchStarted(&self, functionid: usize) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> windows_core::Result<()>;
    fn JITFunctionPitched(&self, functionid: usize) -> windows_core::Result<()>;
    fn JITInlining(&self, callerid: usize, calleeid: usize) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn ThreadCreated(&self, threadid: usize) -> windows_core::Result<()>;
    fn ThreadDestroyed(&self, threadid: usize) -> windows_core::Result<()>;
    fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> windows_core::Result<()>;
    fn RemotingClientInvocationStarted(&self) -> windows_core::Result<()>;
    fn RemotingClientSendingMessage(&self, pcookie: *const windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RemotingClientReceivingReply(&self, pcookie: *const windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RemotingClientInvocationFinished(&self) -> windows_core::Result<()>;
    fn RemotingServerReceivingMessage(&self, pcookie: *const windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RemotingServerInvocationStarted(&self) -> windows_core::Result<()>;
    fn RemotingServerInvocationReturned(&self) -> windows_core::Result<()>;
    fn RemotingServerSendingReply(&self, pcookie: *const windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::Result<()>;
    fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::Result<()>;
    fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> windows_core::Result<()>;
    fn RuntimeSuspendFinished(&self) -> windows_core::Result<()>;
    fn RuntimeSuspendAborted(&self) -> windows_core::Result<()>;
    fn RuntimeResumeStarted(&self) -> windows_core::Result<()>;
    fn RuntimeResumeFinished(&self) -> windows_core::Result<()>;
    fn RuntimeThreadSuspended(&self, threadid: usize) -> windows_core::Result<()>;
    fn RuntimeThreadResumed(&self, threadid: usize) -> windows_core::Result<()>;
    fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()>;
    fn ObjectAllocated(&self, objectid: usize, classid: usize) -> windows_core::Result<()>;
    fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> windows_core::Result<()>;
    fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *const usize) -> windows_core::Result<()>;
    fn RootReferences(&self, crootrefs: u32, rootrefids: *const usize) -> windows_core::Result<()>;
    fn ExceptionThrown(&self, thrownobjectid: usize) -> windows_core::Result<()>;
    fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionSearchFunctionLeave(&self) -> windows_core::Result<()>;
    fn ExceptionSearchFilterEnter(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionSearchFilterLeave(&self) -> windows_core::Result<()>;
    fn ExceptionSearchCatcherFound(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionOSHandlerEnter(&self, __unused: usize) -> windows_core::Result<()>;
    fn ExceptionOSHandlerLeave(&self, __unused: usize) -> windows_core::Result<()>;
    fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionUnwindFunctionLeave(&self) -> windows_core::Result<()>;
    fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> windows_core::Result<()>;
    fn ExceptionUnwindFinallyLeave(&self) -> windows_core::Result<()>;
    fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> windows_core::Result<()>;
    fn ExceptionCatcherLeave(&self) -> windows_core::Result<()>;
    fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void, cslots: u32) -> windows_core::Result<()>;
    fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn ExceptionCLRCatcherFound(&self) -> windows_core::Result<()>;
    fn ExceptionCLRCatcherExecute(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback {}
impl ICorProfilerCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback_Vtbl
    where
        Identity: ICorProfilerCallback_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, picorprofilerinfounk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::Initialize(this, windows_core::from_raw_borrowed(&picorprofilerinfounk)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::Shutdown(this).into()
        }
        unsafe extern "system" fn AppDomainCreationStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::AppDomainCreationStarted(this, core::mem::transmute_copy(&appdomainid)).into()
        }
        unsafe extern "system" fn AppDomainCreationFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::AppDomainCreationFinished(this, core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn AppDomainShutdownStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::AppDomainShutdownStarted(this, core::mem::transmute_copy(&appdomainid)).into()
        }
        unsafe extern "system" fn AppDomainShutdownFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::AppDomainShutdownFinished(this, core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn AssemblyLoadStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::AssemblyLoadStarted(this, core::mem::transmute_copy(&assemblyid)).into()
        }
        unsafe extern "system" fn AssemblyLoadFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::AssemblyLoadFinished(this, core::mem::transmute_copy(&assemblyid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn AssemblyUnloadStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::AssemblyUnloadStarted(this, core::mem::transmute_copy(&assemblyid)).into()
        }
        unsafe extern "system" fn AssemblyUnloadFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::AssemblyUnloadFinished(this, core::mem::transmute_copy(&assemblyid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn ModuleLoadStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ModuleLoadStarted(this, core::mem::transmute_copy(&moduleid)).into()
        }
        unsafe extern "system" fn ModuleLoadFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ModuleLoadFinished(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn ModuleUnloadStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ModuleUnloadStarted(this, core::mem::transmute_copy(&moduleid)).into()
        }
        unsafe extern "system" fn ModuleUnloadFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ModuleUnloadFinished(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn ModuleAttachedToAssembly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, assemblyid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ModuleAttachedToAssembly(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&assemblyid)).into()
        }
        unsafe extern "system" fn ClassLoadStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ClassLoadStarted(this, core::mem::transmute_copy(&classid)).into()
        }
        unsafe extern "system" fn ClassLoadFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ClassLoadFinished(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn ClassUnloadStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ClassUnloadStarted(this, core::mem::transmute_copy(&classid)).into()
        }
        unsafe extern "system" fn ClassUnloadFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ClassUnloadFinished(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn FunctionUnloadStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::FunctionUnloadStarted(this, core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn JITCompilationStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::JITCompilationStarted(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&fissafetoblock)).into()
        }
        unsafe extern "system" fn JITCompilationFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::JITCompilationFinished(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&fissafetoblock)).into()
        }
        unsafe extern "system" fn JITCachedFunctionSearchStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, pbusecachedfunction: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerCallback_Impl::JITCachedFunctionSearchStarted(this, core::mem::transmute_copy(&functionid)) {
                Ok(ok__) => {
                    pbusecachedfunction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JITCachedFunctionSearchFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, result: COR_PRF_JIT_CACHE) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::JITCachedFunctionSearchFinished(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn JITFunctionPitched<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::JITFunctionPitched(this, core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn JITInlining<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callerid: usize, calleeid: usize, pfshouldinline: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerCallback_Impl::JITInlining(this, core::mem::transmute_copy(&callerid), core::mem::transmute_copy(&calleeid)) {
                Ok(ok__) => {
                    pfshouldinline.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThreadCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ThreadCreated(this, core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn ThreadDestroyed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ThreadDestroyed(this, core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn ThreadAssignedToOSThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, managedthreadid: usize, osthreadid: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ThreadAssignedToOSThread(this, core::mem::transmute_copy(&managedthreadid), core::mem::transmute_copy(&osthreadid)).into()
        }
        unsafe extern "system" fn RemotingClientInvocationStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RemotingClientInvocationStarted(this).into()
        }
        unsafe extern "system" fn RemotingClientSendingMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RemotingClientSendingMessage(this, core::mem::transmute_copy(&pcookie), core::mem::transmute_copy(&fisasync)).into()
        }
        unsafe extern "system" fn RemotingClientReceivingReply<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RemotingClientReceivingReply(this, core::mem::transmute_copy(&pcookie), core::mem::transmute_copy(&fisasync)).into()
        }
        unsafe extern "system" fn RemotingClientInvocationFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RemotingClientInvocationFinished(this).into()
        }
        unsafe extern "system" fn RemotingServerReceivingMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RemotingServerReceivingMessage(this, core::mem::transmute_copy(&pcookie), core::mem::transmute_copy(&fisasync)).into()
        }
        unsafe extern "system" fn RemotingServerInvocationStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RemotingServerInvocationStarted(this).into()
        }
        unsafe extern "system" fn RemotingServerInvocationReturned<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RemotingServerInvocationReturned(this).into()
        }
        unsafe extern "system" fn RemotingServerSendingReply<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *const windows_core::GUID, fisasync: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RemotingServerSendingReply(this, core::mem::transmute_copy(&pcookie), core::mem::transmute_copy(&fisasync)).into()
        }
        unsafe extern "system" fn UnmanagedToManagedTransition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::UnmanagedToManagedTransition(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&reason)).into()
        }
        unsafe extern "system" fn ManagedToUnmanagedTransition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ManagedToUnmanagedTransition(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&reason)).into()
        }
        unsafe extern "system" fn RuntimeSuspendStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, suspendreason: COR_PRF_SUSPEND_REASON) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RuntimeSuspendStarted(this, core::mem::transmute_copy(&suspendreason)).into()
        }
        unsafe extern "system" fn RuntimeSuspendFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RuntimeSuspendFinished(this).into()
        }
        unsafe extern "system" fn RuntimeSuspendAborted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RuntimeSuspendAborted(this).into()
        }
        unsafe extern "system" fn RuntimeResumeStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RuntimeResumeStarted(this).into()
        }
        unsafe extern "system" fn RuntimeResumeFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RuntimeResumeFinished(this).into()
        }
        unsafe extern "system" fn RuntimeThreadSuspended<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RuntimeThreadSuspended(this, core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn RuntimeThreadResumed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RuntimeThreadResumed(this, core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn MovedReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::MovedReferences(this, core::mem::transmute_copy(&cmovedobjectidranges), core::mem::transmute_copy(&oldobjectidrangestart), core::mem::transmute_copy(&newobjectidrangestart), core::mem::transmute_copy(&cobjectidrangelength)).into()
        }
        unsafe extern "system" fn ObjectAllocated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, classid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ObjectAllocated(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&classid)).into()
        }
        unsafe extern "system" fn ObjectsAllocatedByClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ObjectsAllocatedByClass(this, core::mem::transmute_copy(&cclasscount), core::mem::transmute_copy(&classids), core::mem::transmute_copy(&cobjects)).into()
        }
        unsafe extern "system" fn ObjectReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *const usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ObjectReferences(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&classid), core::mem::transmute_copy(&cobjectrefs), core::mem::transmute_copy(&objectrefids)).into()
        }
        unsafe extern "system" fn RootReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crootrefs: u32, rootrefids: *const usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::RootReferences(this, core::mem::transmute_copy(&crootrefs), core::mem::transmute_copy(&rootrefids)).into()
        }
        unsafe extern "system" fn ExceptionThrown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, thrownobjectid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionThrown(this, core::mem::transmute_copy(&thrownobjectid)).into()
        }
        unsafe extern "system" fn ExceptionSearchFunctionEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionSearchFunctionEnter(this, core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionSearchFunctionLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionSearchFunctionLeave(this).into()
        }
        unsafe extern "system" fn ExceptionSearchFilterEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionSearchFilterEnter(this, core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionSearchFilterLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionSearchFilterLeave(this).into()
        }
        unsafe extern "system" fn ExceptionSearchCatcherFound<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionSearchCatcherFound(this, core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionOSHandlerEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __unused: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionOSHandlerEnter(this, core::mem::transmute_copy(&__unused)).into()
        }
        unsafe extern "system" fn ExceptionOSHandlerLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, __unused: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionOSHandlerLeave(this, core::mem::transmute_copy(&__unused)).into()
        }
        unsafe extern "system" fn ExceptionUnwindFunctionEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionUnwindFunctionEnter(this, core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionUnwindFunctionLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionUnwindFunctionLeave(this).into()
        }
        unsafe extern "system" fn ExceptionUnwindFinallyEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionUnwindFinallyEnter(this, core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ExceptionUnwindFinallyLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionUnwindFinallyLeave(this).into()
        }
        unsafe extern "system" fn ExceptionCatcherEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, objectid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionCatcherEnter(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn ExceptionCatcherLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionCatcherLeave(this).into()
        }
        unsafe extern "system" fn COMClassicVTableCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void, cslots: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::COMClassicVTableCreated(this, core::mem::transmute_copy(&wrappedclassid), core::mem::transmute_copy(&implementediid), core::mem::transmute_copy(&pvtable), core::mem::transmute_copy(&cslots)).into()
        }
        unsafe extern "system" fn COMClassicVTableDestroyed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::COMClassicVTableDestroyed(this, core::mem::transmute_copy(&wrappedclassid), core::mem::transmute_copy(&implementediid), core::mem::transmute_copy(&pvtable)).into()
        }
        unsafe extern "system" fn ExceptionCLRCatcherFound<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionCLRCatcherFound(this).into()
        }
        unsafe extern "system" fn ExceptionCLRCatcherExecute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback_Impl::ExceptionCLRCatcherExecute(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            AppDomainCreationStarted: AppDomainCreationStarted::<Identity, OFFSET>,
            AppDomainCreationFinished: AppDomainCreationFinished::<Identity, OFFSET>,
            AppDomainShutdownStarted: AppDomainShutdownStarted::<Identity, OFFSET>,
            AppDomainShutdownFinished: AppDomainShutdownFinished::<Identity, OFFSET>,
            AssemblyLoadStarted: AssemblyLoadStarted::<Identity, OFFSET>,
            AssemblyLoadFinished: AssemblyLoadFinished::<Identity, OFFSET>,
            AssemblyUnloadStarted: AssemblyUnloadStarted::<Identity, OFFSET>,
            AssemblyUnloadFinished: AssemblyUnloadFinished::<Identity, OFFSET>,
            ModuleLoadStarted: ModuleLoadStarted::<Identity, OFFSET>,
            ModuleLoadFinished: ModuleLoadFinished::<Identity, OFFSET>,
            ModuleUnloadStarted: ModuleUnloadStarted::<Identity, OFFSET>,
            ModuleUnloadFinished: ModuleUnloadFinished::<Identity, OFFSET>,
            ModuleAttachedToAssembly: ModuleAttachedToAssembly::<Identity, OFFSET>,
            ClassLoadStarted: ClassLoadStarted::<Identity, OFFSET>,
            ClassLoadFinished: ClassLoadFinished::<Identity, OFFSET>,
            ClassUnloadStarted: ClassUnloadStarted::<Identity, OFFSET>,
            ClassUnloadFinished: ClassUnloadFinished::<Identity, OFFSET>,
            FunctionUnloadStarted: FunctionUnloadStarted::<Identity, OFFSET>,
            JITCompilationStarted: JITCompilationStarted::<Identity, OFFSET>,
            JITCompilationFinished: JITCompilationFinished::<Identity, OFFSET>,
            JITCachedFunctionSearchStarted: JITCachedFunctionSearchStarted::<Identity, OFFSET>,
            JITCachedFunctionSearchFinished: JITCachedFunctionSearchFinished::<Identity, OFFSET>,
            JITFunctionPitched: JITFunctionPitched::<Identity, OFFSET>,
            JITInlining: JITInlining::<Identity, OFFSET>,
            ThreadCreated: ThreadCreated::<Identity, OFFSET>,
            ThreadDestroyed: ThreadDestroyed::<Identity, OFFSET>,
            ThreadAssignedToOSThread: ThreadAssignedToOSThread::<Identity, OFFSET>,
            RemotingClientInvocationStarted: RemotingClientInvocationStarted::<Identity, OFFSET>,
            RemotingClientSendingMessage: RemotingClientSendingMessage::<Identity, OFFSET>,
            RemotingClientReceivingReply: RemotingClientReceivingReply::<Identity, OFFSET>,
            RemotingClientInvocationFinished: RemotingClientInvocationFinished::<Identity, OFFSET>,
            RemotingServerReceivingMessage: RemotingServerReceivingMessage::<Identity, OFFSET>,
            RemotingServerInvocationStarted: RemotingServerInvocationStarted::<Identity, OFFSET>,
            RemotingServerInvocationReturned: RemotingServerInvocationReturned::<Identity, OFFSET>,
            RemotingServerSendingReply: RemotingServerSendingReply::<Identity, OFFSET>,
            UnmanagedToManagedTransition: UnmanagedToManagedTransition::<Identity, OFFSET>,
            ManagedToUnmanagedTransition: ManagedToUnmanagedTransition::<Identity, OFFSET>,
            RuntimeSuspendStarted: RuntimeSuspendStarted::<Identity, OFFSET>,
            RuntimeSuspendFinished: RuntimeSuspendFinished::<Identity, OFFSET>,
            RuntimeSuspendAborted: RuntimeSuspendAborted::<Identity, OFFSET>,
            RuntimeResumeStarted: RuntimeResumeStarted::<Identity, OFFSET>,
            RuntimeResumeFinished: RuntimeResumeFinished::<Identity, OFFSET>,
            RuntimeThreadSuspended: RuntimeThreadSuspended::<Identity, OFFSET>,
            RuntimeThreadResumed: RuntimeThreadResumed::<Identity, OFFSET>,
            MovedReferences: MovedReferences::<Identity, OFFSET>,
            ObjectAllocated: ObjectAllocated::<Identity, OFFSET>,
            ObjectsAllocatedByClass: ObjectsAllocatedByClass::<Identity, OFFSET>,
            ObjectReferences: ObjectReferences::<Identity, OFFSET>,
            RootReferences: RootReferences::<Identity, OFFSET>,
            ExceptionThrown: ExceptionThrown::<Identity, OFFSET>,
            ExceptionSearchFunctionEnter: ExceptionSearchFunctionEnter::<Identity, OFFSET>,
            ExceptionSearchFunctionLeave: ExceptionSearchFunctionLeave::<Identity, OFFSET>,
            ExceptionSearchFilterEnter: ExceptionSearchFilterEnter::<Identity, OFFSET>,
            ExceptionSearchFilterLeave: ExceptionSearchFilterLeave::<Identity, OFFSET>,
            ExceptionSearchCatcherFound: ExceptionSearchCatcherFound::<Identity, OFFSET>,
            ExceptionOSHandlerEnter: ExceptionOSHandlerEnter::<Identity, OFFSET>,
            ExceptionOSHandlerLeave: ExceptionOSHandlerLeave::<Identity, OFFSET>,
            ExceptionUnwindFunctionEnter: ExceptionUnwindFunctionEnter::<Identity, OFFSET>,
            ExceptionUnwindFunctionLeave: ExceptionUnwindFunctionLeave::<Identity, OFFSET>,
            ExceptionUnwindFinallyEnter: ExceptionUnwindFinallyEnter::<Identity, OFFSET>,
            ExceptionUnwindFinallyLeave: ExceptionUnwindFinallyLeave::<Identity, OFFSET>,
            ExceptionCatcherEnter: ExceptionCatcherEnter::<Identity, OFFSET>,
            ExceptionCatcherLeave: ExceptionCatcherLeave::<Identity, OFFSET>,
            COMClassicVTableCreated: COMClassicVTableCreated::<Identity, OFFSET>,
            COMClassicVTableDestroyed: COMClassicVTableDestroyed::<Identity, OFFSET>,
            ExceptionCLRCatcherFound: ExceptionCLRCatcherFound::<Identity, OFFSET>,
            ExceptionCLRCatcherExecute: ExceptionCLRCatcherExecute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback10_Impl: Sized + ICorProfilerCallback9_Impl {
    fn EventPipeEventDelivered(&self, provider: usize, eventid: u32, eventversion: u32, cbmetadatablob: u32, metadatablob: *const u8, cbeventdata: u32, eventdata: *const u8, pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID, eventthread: usize, numstackframes: u32, stackframes: *const usize) -> windows_core::Result<()>;
    fn EventPipeProviderCreated(&self, provider: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback10 {}
impl ICorProfilerCallback10_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback10_Vtbl
    where
        Identity: ICorProfilerCallback10_Impl,
    {
        unsafe extern "system" fn EventPipeEventDelivered<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: usize, eventid: u32, eventversion: u32, cbmetadatablob: u32, metadatablob: *const u8, cbeventdata: u32, eventdata: *const u8, pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID, eventthread: usize, numstackframes: u32, stackframes: *const usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback10_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback10_Impl::EventPipeEventDelivered(
                this,
                core::mem::transmute_copy(&provider),
                core::mem::transmute_copy(&eventid),
                core::mem::transmute_copy(&eventversion),
                core::mem::transmute_copy(&cbmetadatablob),
                core::mem::transmute_copy(&metadatablob),
                core::mem::transmute_copy(&cbeventdata),
                core::mem::transmute_copy(&eventdata),
                core::mem::transmute_copy(&pactivityid),
                core::mem::transmute_copy(&prelatedactivityid),
                core::mem::transmute_copy(&eventthread),
                core::mem::transmute_copy(&numstackframes),
                core::mem::transmute_copy(&stackframes),
            )
            .into()
        }
        unsafe extern "system" fn EventPipeProviderCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback10_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback10_Impl::EventPipeProviderCreated(this, core::mem::transmute_copy(&provider)).into()
        }
        Self {
            base__: ICorProfilerCallback9_Vtbl::new::<Identity, OFFSET>(),
            EventPipeEventDelivered: EventPipeEventDelivered::<Identity, OFFSET>,
            EventPipeProviderCreated: EventPipeProviderCreated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback10 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback8 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback9 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback11_Impl: Sized + ICorProfilerCallback10_Impl {
    fn LoadAsNotificationOnly(&self, pbnotificationonly: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback11 {}
impl ICorProfilerCallback11_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback11_Vtbl
    where
        Identity: ICorProfilerCallback11_Impl,
    {
        unsafe extern "system" fn LoadAsNotificationOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbnotificationonly: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback11_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback11_Impl::LoadAsNotificationOnly(this, core::mem::transmute_copy(&pbnotificationonly)).into()
        }
        Self { base__: ICorProfilerCallback10_Vtbl::new::<Identity, OFFSET>(), LoadAsNotificationOnly: LoadAsNotificationOnly::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback11 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback8 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback9 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback10 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback2_Impl: Sized + ICorProfilerCallback_Impl {
    fn ThreadNameChanged(&self, threadid: usize, cchname: u32, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *const super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> windows_core::Result<()>;
    fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()>;
    fn GarbageCollectionFinished(&self) -> windows_core::Result<()>;
    fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> windows_core::Result<()>;
    fn RootReferences2(&self, crootrefs: u32, rootrefids: *const usize, rootkinds: *const COR_PRF_GC_ROOT_KIND, rootflags: *const COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> windows_core::Result<()>;
    fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> windows_core::Result<()>;
    fn HandleDestroyed(&self, handleid: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback2 {}
impl ICorProfilerCallback2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback2_Vtbl
    where
        Identity: ICorProfilerCallback2_Impl,
    {
        unsafe extern "system" fn ThreadNameChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, cchname: u32, name: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback2_Impl::ThreadNameChanged(this, core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&cchname), core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn GarbageCollectionStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cgenerations: i32, generationcollected: *const super::super::super::Foundation::BOOL, reason: COR_PRF_GC_REASON) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback2_Impl::GarbageCollectionStarted(this, core::mem::transmute_copy(&cgenerations), core::mem::transmute_copy(&generationcollected), core::mem::transmute_copy(&reason)).into()
        }
        unsafe extern "system" fn SurvivingReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback2_Impl::SurvivingReferences(this, core::mem::transmute_copy(&csurvivingobjectidranges), core::mem::transmute_copy(&objectidrangestart), core::mem::transmute_copy(&cobjectidrangelength)).into()
        }
        unsafe extern "system" fn GarbageCollectionFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback2_Impl::GarbageCollectionFinished(this).into()
        }
        unsafe extern "system" fn FinalizeableObjectQueued<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalizerflags: u32, objectid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback2_Impl::FinalizeableObjectQueued(this, core::mem::transmute_copy(&finalizerflags), core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn RootReferences2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crootrefs: u32, rootrefids: *const usize, rootkinds: *const COR_PRF_GC_ROOT_KIND, rootflags: *const COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback2_Impl::RootReferences2(this, core::mem::transmute_copy(&crootrefs), core::mem::transmute_copy(&rootrefids), core::mem::transmute_copy(&rootkinds), core::mem::transmute_copy(&rootflags), core::mem::transmute_copy(&rootids)).into()
        }
        unsafe extern "system" fn HandleCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handleid: usize, initialobjectid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback2_Impl::HandleCreated(this, core::mem::transmute_copy(&handleid), core::mem::transmute_copy(&initialobjectid)).into()
        }
        unsafe extern "system" fn HandleDestroyed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handleid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback2_Impl::HandleDestroyed(this, core::mem::transmute_copy(&handleid)).into()
        }
        Self {
            base__: ICorProfilerCallback_Vtbl::new::<Identity, OFFSET>(),
            ThreadNameChanged: ThreadNameChanged::<Identity, OFFSET>,
            GarbageCollectionStarted: GarbageCollectionStarted::<Identity, OFFSET>,
            SurvivingReferences: SurvivingReferences::<Identity, OFFSET>,
            GarbageCollectionFinished: GarbageCollectionFinished::<Identity, OFFSET>,
            FinalizeableObjectQueued: FinalizeableObjectQueued::<Identity, OFFSET>,
            RootReferences2: RootReferences2::<Identity, OFFSET>,
            HandleCreated: HandleCreated::<Identity, OFFSET>,
            HandleDestroyed: HandleDestroyed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback3_Impl: Sized + ICorProfilerCallback2_Impl {
    fn InitializeForAttach(&self, pcorprofilerinfounk: Option<&windows_core::IUnknown>, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::Result<()>;
    fn ProfilerAttachComplete(&self) -> windows_core::Result<()>;
    fn ProfilerDetachSucceeded(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback3 {}
impl ICorProfilerCallback3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback3_Vtbl
    where
        Identity: ICorProfilerCallback3_Impl,
    {
        unsafe extern "system" fn InitializeForAttach<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcorprofilerinfounk: *mut core::ffi::c_void, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback3_Impl::InitializeForAttach(this, windows_core::from_raw_borrowed(&pcorprofilerinfounk), core::mem::transmute_copy(&pvclientdata), core::mem::transmute_copy(&cbclientdata)).into()
        }
        unsafe extern "system" fn ProfilerAttachComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback3_Impl::ProfilerAttachComplete(this).into()
        }
        unsafe extern "system" fn ProfilerDetachSucceeded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback3_Impl::ProfilerDetachSucceeded(this).into()
        }
        Self {
            base__: ICorProfilerCallback2_Vtbl::new::<Identity, OFFSET>(),
            InitializeForAttach: InitializeForAttach::<Identity, OFFSET>,
            ProfilerAttachComplete: ProfilerAttachComplete::<Identity, OFFSET>,
            ProfilerDetachSucceeded: ProfilerDetachSucceeded::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback4_Impl: Sized + ICorProfilerCallback3_Impl {
    fn ReJITCompilationStarted(&self, functionid: usize, rejitid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetReJITParameters(&self, moduleid: usize, methodid: u32, pfunctioncontrol: Option<&ICorProfilerFunctionControl>) -> windows_core::Result<()>;
    fn ReJITCompilationFinished(&self, functionid: usize, rejitid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::Result<()>;
    fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback4 {}
impl ICorProfilerCallback4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback4_Vtbl
    where
        Identity: ICorProfilerCallback4_Impl,
    {
        unsafe extern "system" fn ReJITCompilationStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback4_Impl::ReJITCompilationStarted(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&fissafetoblock)).into()
        }
        unsafe extern "system" fn GetReJITParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, methodid: u32, pfunctioncontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback4_Impl::GetReJITParameters(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&methodid), windows_core::from_raw_borrowed(&pfunctioncontrol)).into()
        }
        unsafe extern "system" fn ReJITCompilationFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback4_Impl::ReJITCompilationFinished(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&fissafetoblock)).into()
        }
        unsafe extern "system" fn ReJITError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, methodid: u32, functionid: usize, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback4_Impl::ReJITError(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn MovedReferences2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback4_Impl::MovedReferences2(this, core::mem::transmute_copy(&cmovedobjectidranges), core::mem::transmute_copy(&oldobjectidrangestart), core::mem::transmute_copy(&newobjectidrangestart), core::mem::transmute_copy(&cobjectidrangelength)).into()
        }
        unsafe extern "system" fn SurvivingReferences2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback4_Impl::SurvivingReferences2(this, core::mem::transmute_copy(&csurvivingobjectidranges), core::mem::transmute_copy(&objectidrangestart), core::mem::transmute_copy(&cobjectidrangelength)).into()
        }
        Self {
            base__: ICorProfilerCallback3_Vtbl::new::<Identity, OFFSET>(),
            ReJITCompilationStarted: ReJITCompilationStarted::<Identity, OFFSET>,
            GetReJITParameters: GetReJITParameters::<Identity, OFFSET>,
            ReJITCompilationFinished: ReJITCompilationFinished::<Identity, OFFSET>,
            ReJITError: ReJITError::<Identity, OFFSET>,
            MovedReferences2: MovedReferences2::<Identity, OFFSET>,
            SurvivingReferences2: SurvivingReferences2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback5_Impl: Sized + ICorProfilerCallback4_Impl {
    fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *const usize, valuerefids: *const usize, rootids: *const usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback5 {}
impl ICorProfilerCallback5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback5_Vtbl
    where
        Identity: ICorProfilerCallback5_Impl,
    {
        unsafe extern "system" fn ConditionalWeakTableElementReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crootrefs: u32, keyrefids: *const usize, valuerefids: *const usize, rootids: *const usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback5_Impl::ConditionalWeakTableElementReferences(this, core::mem::transmute_copy(&crootrefs), core::mem::transmute_copy(&keyrefids), core::mem::transmute_copy(&valuerefids), core::mem::transmute_copy(&rootids)).into()
        }
        Self {
            base__: ICorProfilerCallback4_Vtbl::new::<Identity, OFFSET>(),
            ConditionalWeakTableElementReferences: ConditionalWeakTableElementReferences::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback6_Impl: Sized + ICorProfilerCallback5_Impl {
    fn GetAssemblyReferences(&self, wszassemblypath: &windows_core::PCWSTR, pasmrefprovider: Option<&ICorProfilerAssemblyReferenceProvider>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback6 {}
impl ICorProfilerCallback6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback6_Vtbl
    where
        Identity: ICorProfilerCallback6_Impl,
    {
        unsafe extern "system" fn GetAssemblyReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszassemblypath: windows_core::PCWSTR, pasmrefprovider: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback6_Impl::GetAssemblyReferences(this, core::mem::transmute(&wszassemblypath), windows_core::from_raw_borrowed(&pasmrefprovider)).into()
        }
        Self { base__: ICorProfilerCallback5_Vtbl::new::<Identity, OFFSET>(), GetAssemblyReferences: GetAssemblyReferences::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback7_Impl: Sized + ICorProfilerCallback6_Impl {
    fn ModuleInMemorySymbolsUpdated(&self, moduleid: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback7 {}
impl ICorProfilerCallback7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback7_Vtbl
    where
        Identity: ICorProfilerCallback7_Impl,
    {
        unsafe extern "system" fn ModuleInMemorySymbolsUpdated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback7_Impl::ModuleInMemorySymbolsUpdated(this, core::mem::transmute_copy(&moduleid)).into()
        }
        Self { base__: ICorProfilerCallback6_Vtbl::new::<Identity, OFFSET>(), ModuleInMemorySymbolsUpdated: ModuleInMemorySymbolsUpdated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback8_Impl: Sized + ICorProfilerCallback7_Impl {
    fn DynamicMethodJITCompilationStarted(&self, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL, pilheader: *const u8, cbilheader: u32) -> windows_core::Result<()>;
    fn DynamicMethodJITCompilationFinished(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback8 {}
impl ICorProfilerCallback8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback8_Vtbl
    where
        Identity: ICorProfilerCallback8_Impl,
    {
        unsafe extern "system" fn DynamicMethodJITCompilationStarted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, fissafetoblock: super::super::super::Foundation::BOOL, pilheader: *const u8, cbilheader: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback8_Impl::DynamicMethodJITCompilationStarted(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&fissafetoblock), core::mem::transmute_copy(&pilheader), core::mem::transmute_copy(&cbilheader)).into()
        }
        unsafe extern "system" fn DynamicMethodJITCompilationFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback8_Impl::DynamicMethodJITCompilationFinished(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&fissafetoblock)).into()
        }
        Self {
            base__: ICorProfilerCallback7_Vtbl::new::<Identity, OFFSET>(),
            DynamicMethodJITCompilationStarted: DynamicMethodJITCompilationStarted::<Identity, OFFSET>,
            DynamicMethodJITCompilationFinished: DynamicMethodJITCompilationFinished::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback8 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerCallback9_Impl: Sized + ICorProfilerCallback8_Impl {
    fn DynamicMethodUnloaded(&self, functionid: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerCallback9 {}
impl ICorProfilerCallback9_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerCallback9_Vtbl
    where
        Identity: ICorProfilerCallback9_Impl,
    {
        unsafe extern "system" fn DynamicMethodUnloaded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerCallback9_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerCallback9_Impl::DynamicMethodUnloaded(this, core::mem::transmute_copy(&functionid)).into()
        }
        Self { base__: ICorProfilerCallback8_Vtbl::new::<Identity, OFFSET>(), DynamicMethodUnloaded: DynamicMethodUnloaded::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerCallback9 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback as windows_core::Interface>::IID || iid == &<ICorProfilerCallback2 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback3 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback4 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback5 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback6 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback7 as windows_core::Interface>::IID || iid == &<ICorProfilerCallback8 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerFunctionControl_Impl: Sized {
    fn SetCodegenFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn SetILFunctionBody(&self, cbnewilmethodheader: u32, pbnewilmethodheader: *const u8) -> windows_core::Result<()>;
    fn SetILInstrumentedCodeMap(&self, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerFunctionControl {}
impl ICorProfilerFunctionControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerFunctionControl_Vtbl
    where
        Identity: ICorProfilerFunctionControl_Impl,
    {
        unsafe extern "system" fn SetCodegenFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerFunctionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerFunctionControl_Impl::SetCodegenFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetILFunctionBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbnewilmethodheader: u32, pbnewilmethodheader: *const u8) -> windows_core::HRESULT
        where
            Identity: ICorProfilerFunctionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerFunctionControl_Impl::SetILFunctionBody(this, core::mem::transmute_copy(&cbnewilmethodheader), core::mem::transmute_copy(&pbnewilmethodheader)).into()
        }
        unsafe extern "system" fn SetILInstrumentedCodeMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> windows_core::HRESULT
        where
            Identity: ICorProfilerFunctionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerFunctionControl_Impl::SetILInstrumentedCodeMap(this, core::mem::transmute_copy(&cilmapentries), core::mem::transmute_copy(&rgilmapentries)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCodegenFlags: SetCodegenFlags::<Identity, OFFSET>,
            SetILFunctionBody: SetILFunctionBody::<Identity, OFFSET>,
            SetILInstrumentedCodeMap: SetILInstrumentedCodeMap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerFunctionControl as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerFunctionEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerFunctionEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerFunctionEnum {}
impl ICorProfilerFunctionEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerFunctionEnum_Vtbl
    where
        Identity: ICorProfilerFunctionEnum_Impl,
    {
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerFunctionEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerFunctionEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerFunctionEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerFunctionEnum_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerFunctionEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerFunctionEnum_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerFunctionEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerFunctionEnum_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ids: *mut COR_PRF_FUNCTION, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerFunctionEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerFunctionEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ids), core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerFunctionEnum as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo_Impl: Sized {
    fn GetClassFromObject(&self, objectid: usize) -> windows_core::Result<usize>;
    fn GetClassFromToken(&self, moduleid: usize, typedef: u32) -> windows_core::Result<usize>;
    fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> windows_core::Result<()>;
    fn GetEventMask(&self) -> windows_core::Result<u32>;
    fn GetFunctionFromIP(&self, ip: *const u8) -> windows_core::Result<usize>;
    fn GetFunctionFromToken(&self, moduleid: usize, token: u32) -> windows_core::Result<usize>;
    fn GetHandleFromThread(&self, threadid: usize) -> windows_core::Result<super::super::super::Foundation::HANDLE>;
    fn GetObjectSize(&self, objectid: usize) -> windows_core::Result<u32>;
    fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> windows_core::Result<()>;
    fn GetThreadInfo(&self, threadid: usize) -> windows_core::Result<u32>;
    fn GetCurrentThreadID(&self) -> windows_core::Result<usize>;
    fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> windows_core::Result<()>;
    fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> windows_core::Result<()>;
    fn SetEventMask(&self, dwevents: u32) -> windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *const FunctionEnter, pfuncleave: *const FunctionLeave, pfunctailcall: *const FunctionTailcall) -> windows_core::Result<()>;
    fn SetFunctionIDMapper(&self, pfunc: *const FunctionIDMapper) -> windows_core::Result<()>;
    fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const windows_core::GUID, ppimport: *mut Option<windows_core::IUnknown>, ptoken: *mut u32) -> windows_core::Result<()>;
    fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, passemblyid: *mut usize) -> windows_core::Result<()>;
    fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> windows_core::Result<()>;
    fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> windows_core::Result<IMethodMalloc>;
    fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *const u8) -> windows_core::Result<()>;
    fn GetAppDomainInfo(&self, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, pprocessid: *mut usize) -> windows_core::Result<()>;
    fn GetAssemblyInfo(&self, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> windows_core::Result<()>;
    fn SetFunctionReJIT(&self, functionid: usize) -> windows_core::Result<()>;
    fn ForceGC(&self) -> windows_core::Result<()>;
    fn SetILInstrumentedCodeMap(&self, functionid: usize, fstartjit: super::super::super::Foundation::BOOL, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> windows_core::Result<()>;
    fn GetInprocInspectionInterface(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetInprocInspectionIThisThread(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetThreadContext(&self, threadid: usize) -> windows_core::Result<usize>;
    fn BeginInprocDebugging(&self, fthisthreadonly: super::super::super::Foundation::BOOL) -> windows_core::Result<u32>;
    fn EndInprocDebugging(&self, dwprofilercontext: u32) -> windows_core::Result<()>;
    fn GetILToNativeMapping(&self, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo_Vtbl
    where
        Identity: ICorProfilerInfo_Impl,
    {
        unsafe extern "system" fn GetClassFromObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, pclassid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetClassFromObject(this, core::mem::transmute_copy(&objectid)) {
                Ok(ok__) => {
                    pclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassFromToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, typedef: u32, pclassid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetClassFromToken(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&typedef)) {
                Ok(ok__) => {
                    pclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetCodeInfo(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&pstart), core::mem::transmute_copy(&pcsize)).into()
        }
        unsafe extern "system" fn GetEventMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwevents: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetEventMask(this) {
                Ok(ok__) => {
                    pdwevents.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionFromIP<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ip: *const u8, pfunctionid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetFunctionFromIP(this, core::mem::transmute_copy(&ip)) {
                Ok(ok__) => {
                    pfunctionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionFromToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, token: u32, pfunctionid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetFunctionFromToken(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&token)) {
                Ok(ok__) => {
                    pfunctionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHandleFromThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, phthread: *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetHandleFromThread(this, core::mem::transmute_copy(&threadid)) {
                Ok(ok__) => {
                    phthread.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, pcsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetObjectSize(this, core::mem::transmute_copy(&objectid)) {
                Ok(ok__) => {
                    pcsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArrayClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::IsArrayClass(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&pbaseelemtype), core::mem::transmute_copy(&pbaseclassid), core::mem::transmute_copy(&pcrank)).into()
        }
        unsafe extern "system" fn GetThreadInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, pdwwin32threadid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetThreadInfo(this, core::mem::transmute_copy(&threadid)) {
                Ok(ok__) => {
                    pdwwin32threadid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentThreadID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthreadid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetCurrentThreadID(this) {
                Ok(ok__) => {
                    pthreadid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassIDInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetClassIDInfo(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&pmoduleid), core::mem::transmute_copy(&ptypedeftoken)).into()
        }
        unsafe extern "system" fn GetFunctionInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetFunctionInfo(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&pclassid), core::mem::transmute_copy(&pmoduleid), core::mem::transmute_copy(&ptoken)).into()
        }
        unsafe extern "system" fn SetEventMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwevents: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::SetEventMask(this, core::mem::transmute_copy(&dwevents)).into()
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncenter: *const FunctionEnter, pfuncleave: *const FunctionLeave, pfunctailcall: *const FunctionTailcall) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::SetEnterLeaveFunctionHooks(this, core::mem::transmute_copy(&pfuncenter), core::mem::transmute_copy(&pfuncleave), core::mem::transmute_copy(&pfunctailcall)).into()
        }
        unsafe extern "system" fn SetFunctionIDMapper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunc: *const FunctionIDMapper) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::SetFunctionIDMapper(this, core::mem::transmute_copy(&pfunc)).into()
        }
        unsafe extern "system" fn GetTokenAndMetaDataFromFunction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, riid: *const windows_core::GUID, ppimport: *mut *mut core::ffi::c_void, ptoken: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetTokenAndMetaDataFromFunction(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppimport), core::mem::transmute_copy(&ptoken)).into()
        }
        unsafe extern "system" fn GetModuleInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, passemblyid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetModuleInfo(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&ppbaseloadaddress), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&passemblyid)).into()
        }
        unsafe extern "system" fn GetModuleMetaData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, dwopenflags: u32, riid: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetModuleMetaData(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&dwopenflags), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetILFunctionBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetILFunctionBody(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&ppmethodheader), core::mem::transmute_copy(&pcbmethodsize)).into()
        }
        unsafe extern "system" fn GetILFunctionBodyAllocator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, ppmalloc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetILFunctionBodyAllocator(this, core::mem::transmute_copy(&moduleid)) {
                Ok(ok__) => {
                    ppmalloc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetILFunctionBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, methodid: u32, pbnewilmethodheader: *const u8) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::SetILFunctionBody(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&methodid), core::mem::transmute_copy(&pbnewilmethodheader)).into()
        }
        unsafe extern "system" fn GetAppDomainInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, appdomainid: usize, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, pprocessid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetAppDomainInfo(this, core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pprocessid)).into()
        }
        unsafe extern "system" fn GetAssemblyInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, assemblyid: usize, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, pappdomainid: *mut usize, pmoduleid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetAssemblyInfo(this, core::mem::transmute_copy(&assemblyid), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pappdomainid), core::mem::transmute_copy(&pmoduleid)).into()
        }
        unsafe extern "system" fn SetFunctionReJIT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::SetFunctionReJIT(this, core::mem::transmute_copy(&functionid)).into()
        }
        unsafe extern "system" fn ForceGC<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::ForceGC(this).into()
        }
        unsafe extern "system" fn SetILInstrumentedCodeMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, fstartjit: super::super::super::Foundation::BOOL, cilmapentries: u32, rgilmapentries: *const COR_IL_MAP) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::SetILInstrumentedCodeMap(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&fstartjit), core::mem::transmute_copy(&cilmapentries), core::mem::transmute_copy(&rgilmapentries)).into()
        }
        unsafe extern "system" fn GetInprocInspectionInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetInprocInspectionInterface(this) {
                Ok(ok__) => {
                    ppicd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInprocInspectionIThisThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppicd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetInprocInspectionIThisThread(this) {
                Ok(ok__) => {
                    ppicd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, pcontextid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::GetThreadContext(this, core::mem::transmute_copy(&threadid)) {
                Ok(ok__) => {
                    pcontextid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginInprocDebugging<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fthisthreadonly: super::super::super::Foundation::BOOL, pdwprofilercontext: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo_Impl::BeginInprocDebugging(this, core::mem::transmute_copy(&fthisthreadonly)) {
                Ok(ok__) => {
                    pdwprofilercontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInprocDebugging<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprofilercontext: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::EndInprocDebugging(this, core::mem::transmute_copy(&dwprofilercontext)).into()
        }
        unsafe extern "system" fn GetILToNativeMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo_Impl::GetILToNativeMapping(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&cmap), core::mem::transmute_copy(&pcmap), core::mem::transmute_copy(&map)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClassFromObject: GetClassFromObject::<Identity, OFFSET>,
            GetClassFromToken: GetClassFromToken::<Identity, OFFSET>,
            GetCodeInfo: GetCodeInfo::<Identity, OFFSET>,
            GetEventMask: GetEventMask::<Identity, OFFSET>,
            GetFunctionFromIP: GetFunctionFromIP::<Identity, OFFSET>,
            GetFunctionFromToken: GetFunctionFromToken::<Identity, OFFSET>,
            GetHandleFromThread: GetHandleFromThread::<Identity, OFFSET>,
            GetObjectSize: GetObjectSize::<Identity, OFFSET>,
            IsArrayClass: IsArrayClass::<Identity, OFFSET>,
            GetThreadInfo: GetThreadInfo::<Identity, OFFSET>,
            GetCurrentThreadID: GetCurrentThreadID::<Identity, OFFSET>,
            GetClassIDInfo: GetClassIDInfo::<Identity, OFFSET>,
            GetFunctionInfo: GetFunctionInfo::<Identity, OFFSET>,
            SetEventMask: SetEventMask::<Identity, OFFSET>,
            SetEnterLeaveFunctionHooks: SetEnterLeaveFunctionHooks::<Identity, OFFSET>,
            SetFunctionIDMapper: SetFunctionIDMapper::<Identity, OFFSET>,
            GetTokenAndMetaDataFromFunction: GetTokenAndMetaDataFromFunction::<Identity, OFFSET>,
            GetModuleInfo: GetModuleInfo::<Identity, OFFSET>,
            GetModuleMetaData: GetModuleMetaData::<Identity, OFFSET>,
            GetILFunctionBody: GetILFunctionBody::<Identity, OFFSET>,
            GetILFunctionBodyAllocator: GetILFunctionBodyAllocator::<Identity, OFFSET>,
            SetILFunctionBody: SetILFunctionBody::<Identity, OFFSET>,
            GetAppDomainInfo: GetAppDomainInfo::<Identity, OFFSET>,
            GetAssemblyInfo: GetAssemblyInfo::<Identity, OFFSET>,
            SetFunctionReJIT: SetFunctionReJIT::<Identity, OFFSET>,
            ForceGC: ForceGC::<Identity, OFFSET>,
            SetILInstrumentedCodeMap: SetILInstrumentedCodeMap::<Identity, OFFSET>,
            GetInprocInspectionInterface: GetInprocInspectionInterface::<Identity, OFFSET>,
            GetInprocInspectionIThisThread: GetInprocInspectionIThisThread::<Identity, OFFSET>,
            GetThreadContext: GetThreadContext::<Identity, OFFSET>,
            BeginInprocDebugging: BeginInprocDebugging::<Identity, OFFSET>,
            EndInprocDebugging: EndInprocDebugging::<Identity, OFFSET>,
            GetILToNativeMapping: GetILToNativeMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo10_Impl: Sized + ICorProfilerInfo9_Impl {
    fn EnumerateObjectReferences(&self, objectid: usize, callback: ObjectReferenceCallback, clientdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsFrozenObject(&self, objectid: usize, pbfrozen: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLOHObjectSizeThreshold(&self, pthreshold: *mut u32) -> windows_core::Result<()>;
    fn RequestReJITWithInliners(&self, dwrejitflags: u32, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::Result<()>;
    fn SuspendRuntime(&self) -> windows_core::Result<()>;
    fn ResumeRuntime(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo10 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo10_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo10_Vtbl
    where
        Identity: ICorProfilerInfo10_Impl,
    {
        unsafe extern "system" fn EnumerateObjectReferences<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, callback: ObjectReferenceCallback, clientdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo10_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo10_Impl::EnumerateObjectReferences(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&callback), core::mem::transmute_copy(&clientdata)).into()
        }
        unsafe extern "system" fn IsFrozenObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, pbfrozen: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo10_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo10_Impl::IsFrozenObject(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&pbfrozen)).into()
        }
        unsafe extern "system" fn GetLOHObjectSizeThreshold<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthreshold: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo10_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo10_Impl::GetLOHObjectSizeThreshold(this, core::mem::transmute_copy(&pthreshold)).into()
        }
        unsafe extern "system" fn RequestReJITWithInliners<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrejitflags: u32, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo10_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo10_Impl::RequestReJITWithInliners(this, core::mem::transmute_copy(&dwrejitflags), core::mem::transmute_copy(&cfunctions), core::mem::transmute_copy(&moduleids), core::mem::transmute_copy(&methodids)).into()
        }
        unsafe extern "system" fn SuspendRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo10_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo10_Impl::SuspendRuntime(this).into()
        }
        unsafe extern "system" fn ResumeRuntime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo10_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo10_Impl::ResumeRuntime(this).into()
        }
        Self {
            base__: ICorProfilerInfo9_Vtbl::new::<Identity, OFFSET>(),
            EnumerateObjectReferences: EnumerateObjectReferences::<Identity, OFFSET>,
            IsFrozenObject: IsFrozenObject::<Identity, OFFSET>,
            GetLOHObjectSizeThreshold: GetLOHObjectSizeThreshold::<Identity, OFFSET>,
            RequestReJITWithInliners: RequestReJITWithInliners::<Identity, OFFSET>,
            SuspendRuntime: SuspendRuntime::<Identity, OFFSET>,
            ResumeRuntime: ResumeRuntime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo11_Impl: Sized + ICorProfilerInfo10_Impl {
    fn GetEnvironmentVariableA(&self, szname: &windows_core::PCWSTR, cchvalue: u32, pcchvalue: *mut u32, szvalue: windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetEnvironmentVariable(&self, szname: &windows_core::PCWSTR, szvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo11 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo11_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo11_Vtbl
    where
        Identity: ICorProfilerInfo11_Impl,
    {
        unsafe extern "system" fn GetEnvironmentVariableA<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, cchvalue: u32, pcchvalue: *mut u32, szvalue: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo11_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo11_Impl::GetEnvironmentVariableA(this, core::mem::transmute(&szname), core::mem::transmute_copy(&cchvalue), core::mem::transmute_copy(&pcchvalue), core::mem::transmute_copy(&szvalue)).into()
        }
        unsafe extern "system" fn SetEnvironmentVariable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCWSTR, szvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo11_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo11_Impl::SetEnvironmentVariable(this, core::mem::transmute(&szname), core::mem::transmute(&szvalue)).into()
        }
        Self {
            base__: ICorProfilerInfo10_Vtbl::new::<Identity, OFFSET>(),
            GetEnvironmentVariableA: GetEnvironmentVariableA::<Identity, OFFSET>,
            SetEnvironmentVariable: SetEnvironmentVariable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo11 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo12_Impl: Sized + ICorProfilerInfo11_Impl {
    fn EventPipeStartSession(&self, cproviderconfigs: u32, pproviderconfigs: *const COR_PRF_EVENTPIPE_PROVIDER_CONFIG, requestrundown: super::super::super::Foundation::BOOL) -> windows_core::Result<u64>;
    fn EventPipeAddProviderToSession(&self, session: u64, providerconfig: &COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> windows_core::Result<()>;
    fn EventPipeStopSession(&self, session: u64) -> windows_core::Result<()>;
    fn EventPipeCreateProvider(&self, providername: &windows_core::PCWSTR) -> windows_core::Result<usize>;
    fn EventPipeGetProviderInfo(&self, provider: usize, cchname: u32, pcchname: *mut u32, providername: windows_core::PWSTR) -> windows_core::Result<()>;
    fn EventPipeDefineEvent(&self, provider: usize, eventname: &windows_core::PCWSTR, eventid: u32, keywords: u64, eventversion: u32, level: u32, opcode: u8, needstack: super::super::super::Foundation::BOOL, cparamdescs: u32, pparamdescs: *const COR_PRF_EVENTPIPE_PARAM_DESC) -> windows_core::Result<usize>;
    fn EventPipeWriteEvent(&self, event: usize, cdata: u32, data: *const COR_PRF_EVENT_DATA, pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo12 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo12_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo12_Vtbl
    where
        Identity: ICorProfilerInfo12_Impl,
    {
        unsafe extern "system" fn EventPipeStartSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cproviderconfigs: u32, pproviderconfigs: *const COR_PRF_EVENTPIPE_PROVIDER_CONFIG, requestrundown: super::super::super::Foundation::BOOL, psession: *mut u64) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo12_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo12_Impl::EventPipeStartSession(this, core::mem::transmute_copy(&cproviderconfigs), core::mem::transmute_copy(&pproviderconfigs), core::mem::transmute_copy(&requestrundown)) {
                Ok(ok__) => {
                    psession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventPipeAddProviderToSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, session: u64, providerconfig: COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo12_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo12_Impl::EventPipeAddProviderToSession(this, core::mem::transmute_copy(&session), core::mem::transmute(&providerconfig)).into()
        }
        unsafe extern "system" fn EventPipeStopSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, session: u64) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo12_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo12_Impl::EventPipeStopSession(this, core::mem::transmute_copy(&session)).into()
        }
        unsafe extern "system" fn EventPipeCreateProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: windows_core::PCWSTR, pprovider: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo12_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo12_Impl::EventPipeCreateProvider(this, core::mem::transmute(&providername)) {
                Ok(ok__) => {
                    pprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventPipeGetProviderInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: usize, cchname: u32, pcchname: *mut u32, providername: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo12_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo12_Impl::EventPipeGetProviderInfo(this, core::mem::transmute_copy(&provider), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&providername)).into()
        }
        unsafe extern "system" fn EventPipeDefineEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: usize, eventname: windows_core::PCWSTR, eventid: u32, keywords: u64, eventversion: u32, level: u32, opcode: u8, needstack: super::super::super::Foundation::BOOL, cparamdescs: u32, pparamdescs: *const COR_PRF_EVENTPIPE_PARAM_DESC, pevent: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo12_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo12_Impl::EventPipeDefineEvent(this, core::mem::transmute_copy(&provider), core::mem::transmute(&eventname), core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&keywords), core::mem::transmute_copy(&eventversion), core::mem::transmute_copy(&level), core::mem::transmute_copy(&opcode), core::mem::transmute_copy(&needstack), core::mem::transmute_copy(&cparamdescs), core::mem::transmute_copy(&pparamdescs)) {
                Ok(ok__) => {
                    pevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventPipeWriteEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: usize, cdata: u32, data: *const COR_PRF_EVENT_DATA, pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo12_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo12_Impl::EventPipeWriteEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&cdata), core::mem::transmute_copy(&data), core::mem::transmute_copy(&pactivityid), core::mem::transmute_copy(&prelatedactivityid)).into()
        }
        Self {
            base__: ICorProfilerInfo11_Vtbl::new::<Identity, OFFSET>(),
            EventPipeStartSession: EventPipeStartSession::<Identity, OFFSET>,
            EventPipeAddProviderToSession: EventPipeAddProviderToSession::<Identity, OFFSET>,
            EventPipeStopSession: EventPipeStopSession::<Identity, OFFSET>,
            EventPipeCreateProvider: EventPipeCreateProvider::<Identity, OFFSET>,
            EventPipeGetProviderInfo: EventPipeGetProviderInfo::<Identity, OFFSET>,
            EventPipeDefineEvent: EventPipeDefineEvent::<Identity, OFFSET>,
            EventPipeWriteEvent: EventPipeWriteEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo12 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo11 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo13_Impl: Sized + ICorProfilerInfo12_Impl {
    fn CreateHandle(&self, object: usize, r#type: COR_PRF_HANDLE_TYPE, phandle: *mut *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DestroyHandle(&self, handle: *const *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetObjectIDFromHandle(&self, handle: *const *const core::ffi::c_void) -> windows_core::Result<usize>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo13 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo13_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo13_Vtbl
    where
        Identity: ICorProfilerInfo13_Impl,
    {
        unsafe extern "system" fn CreateHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: usize, r#type: COR_PRF_HANDLE_TYPE, phandle: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo13_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo13_Impl::CreateHandle(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&phandle)).into()
        }
        unsafe extern "system" fn DestroyHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: *const *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo13_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo13_Impl::DestroyHandle(this, core::mem::transmute_copy(&handle)).into()
        }
        unsafe extern "system" fn GetObjectIDFromHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: *const *const core::ffi::c_void, pobject: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo13_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo13_Impl::GetObjectIDFromHandle(this, core::mem::transmute_copy(&handle)) {
                Ok(ok__) => {
                    pobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICorProfilerInfo12_Vtbl::new::<Identity, OFFSET>(),
            CreateHandle: CreateHandle::<Identity, OFFSET>,
            DestroyHandle: DestroyHandle::<Identity, OFFSET>,
            GetObjectIDFromHandle: GetObjectIDFromHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo13 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo11 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo12 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo14_Impl: Sized + ICorProfilerInfo13_Impl {
    fn EnumerateNonGCObjects(&self) -> windows_core::Result<ICorProfilerObjectEnum>;
    fn GetNonGCHeapBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_NONGC_HEAP_RANGE) -> windows_core::Result<()>;
    fn EventPipeCreateProvider2(&self, providername: &windows_core::PCWSTR, pcallback: *const EventPipeProviderCallback) -> windows_core::Result<usize>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo14 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo14_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo14_Vtbl
    where
        Identity: ICorProfilerInfo14_Impl,
    {
        unsafe extern "system" fn EnumerateNonGCObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo14_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo14_Impl::EnumerateNonGCObjects(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNonGCHeapBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_NONGC_HEAP_RANGE) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo14_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo14_Impl::GetNonGCHeapBounds(this, core::mem::transmute_copy(&cobjectranges), core::mem::transmute_copy(&pcobjectranges), core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn EventPipeCreateProvider2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: windows_core::PCWSTR, pcallback: *const EventPipeProviderCallback, pprovider: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo14_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo14_Impl::EventPipeCreateProvider2(this, core::mem::transmute(&providername), core::mem::transmute_copy(&pcallback)) {
                Ok(ok__) => {
                    pprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICorProfilerInfo13_Vtbl::new::<Identity, OFFSET>(),
            EnumerateNonGCObjects: EnumerateNonGCObjects::<Identity, OFFSET>,
            GetNonGCHeapBounds: GetNonGCHeapBounds::<Identity, OFFSET>,
            EventPipeCreateProvider2: EventPipeCreateProvider2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo14 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo10 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo11 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo12 as windows_core::Interface>::IID
            || iid == &<ICorProfilerInfo13 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo2_Impl: Sized + ICorProfilerInfo_Impl {
    fn DoStackSnapshot(&self, thread: usize, callback: *const StackSnapshotCallback, infoflags: u32, clientdata: *const core::ffi::c_void, context: *const u8, contextsize: u32) -> windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *const FunctionEnter2, pfuncleave: *const FunctionLeave2, pfunctailcall: *const FunctionTailcall2) -> windows_core::Result<()>;
    fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> windows_core::Result<()>;
    fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::Result<()>;
    fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::Result<()>;
    fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> windows_core::Result<()>;
    fn GetCodeInfo2(&self, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::Result<()>;
    fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *const usize) -> windows_core::Result<usize>;
    fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *const usize) -> windows_core::Result<usize>;
    fn EnumModuleFrozenObjects(&self, moduleid: usize) -> windows_core::Result<ICorProfilerObjectEnum>;
    fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> windows_core::Result<()>;
    fn GetBoxClassLayout(&self, classid: usize) -> windows_core::Result<u32>;
    fn GetThreadAppDomain(&self, threadid: usize) -> windows_core::Result<usize>;
    fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32) -> windows_core::Result<COR_PRF_STATIC_TYPE>;
    fn GetGenerationBounds(&self, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::Result<()>;
    fn GetObjectGeneration(&self, objectid: usize) -> windows_core::Result<COR_PRF_GC_GENERATION_RANGE>;
    fn GetNotifiedExceptionClauseInfo(&self) -> windows_core::Result<COR_PRF_EX_CLAUSE_INFO>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo2 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo2_Vtbl
    where
        Identity: ICorProfilerInfo2_Impl,
    {
        unsafe extern "system" fn DoStackSnapshot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, thread: usize, callback: *const StackSnapshotCallback, infoflags: u32, clientdata: *const core::ffi::c_void, context: *const u8, contextsize: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::DoStackSnapshot(this, core::mem::transmute_copy(&thread), core::mem::transmute_copy(&callback), core::mem::transmute_copy(&infoflags), core::mem::transmute_copy(&clientdata), core::mem::transmute_copy(&context), core::mem::transmute_copy(&contextsize)).into()
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncenter: *const FunctionEnter2, pfuncleave: *const FunctionLeave2, pfunctailcall: *const FunctionTailcall2) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::SetEnterLeaveFunctionHooks2(this, core::mem::transmute_copy(&pfuncenter), core::mem::transmute_copy(&pfuncleave), core::mem::transmute_copy(&pfunctailcall)).into()
        }
        unsafe extern "system" fn GetFunctionInfo2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetFunctionInfo2(this, core::mem::transmute_copy(&funcid), core::mem::transmute_copy(&frameinfo), core::mem::transmute_copy(&pclassid), core::mem::transmute_copy(&pmoduleid), core::mem::transmute_copy(&ptoken), core::mem::transmute_copy(&ctypeargs), core::mem::transmute_copy(&pctypeargs), core::mem::transmute_copy(&typeargs)).into()
        }
        unsafe extern "system" fn GetStringLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetStringLayout(this, core::mem::transmute_copy(&pbufferlengthoffset), core::mem::transmute_copy(&pstringlengthoffset), core::mem::transmute_copy(&pbufferoffset)).into()
        }
        unsafe extern "system" fn GetClassLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetClassLayout(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&rfieldoffset), core::mem::transmute_copy(&cfieldoffset), core::mem::transmute_copy(&pcfieldoffset), core::mem::transmute_copy(&pulclasssize)).into()
        }
        unsafe extern "system" fn GetClassIDInfo2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetClassIDInfo2(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&pmoduleid), core::mem::transmute_copy(&ptypedeftoken), core::mem::transmute_copy(&pparentclassid), core::mem::transmute_copy(&cnumtypeargs), core::mem::transmute_copy(&pcnumtypeargs), core::mem::transmute_copy(&typeargs)).into()
        }
        unsafe extern "system" fn GetCodeInfo2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetCodeInfo2(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&ccodeinfos), core::mem::transmute_copy(&pccodeinfos), core::mem::transmute_copy(&codeinfos)).into()
        }
        unsafe extern "system" fn GetClassFromTokenAndTypeArgs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, typedef: u32, ctypeargs: u32, typeargs: *const usize, pclassid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo2_Impl::GetClassFromTokenAndTypeArgs(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&typedef), core::mem::transmute_copy(&ctypeargs), core::mem::transmute_copy(&typeargs)) {
                Ok(ok__) => {
                    pclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionFromTokenAndTypeArgs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, funcdef: u32, classid: usize, ctypeargs: u32, typeargs: *const usize, pfunctionid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo2_Impl::GetFunctionFromTokenAndTypeArgs(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&funcdef), core::mem::transmute_copy(&classid), core::mem::transmute_copy(&ctypeargs), core::mem::transmute_copy(&typeargs)) {
                Ok(ok__) => {
                    pfunctionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumModuleFrozenObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo2_Impl::EnumModuleFrozenObjects(this, core::mem::transmute_copy(&moduleid)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArrayObjectInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetArrayObjectInfo(this, core::mem::transmute_copy(&objectid), core::mem::transmute_copy(&cdimensions), core::mem::transmute_copy(&pdimensionsizes), core::mem::transmute_copy(&pdimensionlowerbounds), core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn GetBoxClassLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, pbufferoffset: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo2_Impl::GetBoxClassLayout(this, core::mem::transmute_copy(&classid)) {
                Ok(ok__) => {
                    pbufferoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadAppDomain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadid: usize, pappdomainid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo2_Impl::GetThreadAppDomain(this, core::mem::transmute_copy(&threadid)) {
                Ok(ok__) => {
                    pappdomainid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRVAStaticAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetRVAStaticAddress(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetAppDomainStaticAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetAppDomainStaticAddress(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetThreadStaticAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetThreadStaticAddress(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetContextStaticAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetContextStaticAddress(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&contextid), core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetStaticFieldInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, pfieldinfo: *mut COR_PRF_STATIC_TYPE) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo2_Impl::GetStaticFieldInfo(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken)) {
                Ok(ok__) => {
                    pfieldinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjectranges: u32, pcobjectranges: *mut u32, ranges: *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo2_Impl::GetGenerationBounds(this, core::mem::transmute_copy(&cobjectranges), core::mem::transmute_copy(&pcobjectranges), core::mem::transmute_copy(&ranges)).into()
        }
        unsafe extern "system" fn GetObjectGeneration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, range: *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo2_Impl::GetObjectGeneration(this, core::mem::transmute_copy(&objectid)) {
                Ok(ok__) => {
                    range.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNotifiedExceptionClauseInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut COR_PRF_EX_CLAUSE_INFO) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo2_Impl::GetNotifiedExceptionClauseInfo(this) {
                Ok(ok__) => {
                    pinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICorProfilerInfo_Vtbl::new::<Identity, OFFSET>(),
            DoStackSnapshot: DoStackSnapshot::<Identity, OFFSET>,
            SetEnterLeaveFunctionHooks2: SetEnterLeaveFunctionHooks2::<Identity, OFFSET>,
            GetFunctionInfo2: GetFunctionInfo2::<Identity, OFFSET>,
            GetStringLayout: GetStringLayout::<Identity, OFFSET>,
            GetClassLayout: GetClassLayout::<Identity, OFFSET>,
            GetClassIDInfo2: GetClassIDInfo2::<Identity, OFFSET>,
            GetCodeInfo2: GetCodeInfo2::<Identity, OFFSET>,
            GetClassFromTokenAndTypeArgs: GetClassFromTokenAndTypeArgs::<Identity, OFFSET>,
            GetFunctionFromTokenAndTypeArgs: GetFunctionFromTokenAndTypeArgs::<Identity, OFFSET>,
            EnumModuleFrozenObjects: EnumModuleFrozenObjects::<Identity, OFFSET>,
            GetArrayObjectInfo: GetArrayObjectInfo::<Identity, OFFSET>,
            GetBoxClassLayout: GetBoxClassLayout::<Identity, OFFSET>,
            GetThreadAppDomain: GetThreadAppDomain::<Identity, OFFSET>,
            GetRVAStaticAddress: GetRVAStaticAddress::<Identity, OFFSET>,
            GetAppDomainStaticAddress: GetAppDomainStaticAddress::<Identity, OFFSET>,
            GetThreadStaticAddress: GetThreadStaticAddress::<Identity, OFFSET>,
            GetContextStaticAddress: GetContextStaticAddress::<Identity, OFFSET>,
            GetStaticFieldInfo: GetStaticFieldInfo::<Identity, OFFSET>,
            GetGenerationBounds: GetGenerationBounds::<Identity, OFFSET>,
            GetObjectGeneration: GetObjectGeneration::<Identity, OFFSET>,
            GetNotifiedExceptionClauseInfo: GetNotifiedExceptionClauseInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo3_Impl: Sized + ICorProfilerInfo2_Impl {
    fn EnumJITedFunctions(&self) -> windows_core::Result<ICorProfilerFunctionEnum>;
    fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> windows_core::Result<()>;
    fn SetFunctionIDMapper2(&self, pfunc: *const FunctionIDMapper2, clientdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *const FunctionEnter3, pfuncleave3: *const FunctionLeave3, pfunctailcall3: *const FunctionTailcall3) -> windows_core::Result<()>;
    fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *const FunctionEnter3WithInfo, pfuncleave3withinfo: *const FunctionLeave3WithInfo, pfunctailcall3withinfo: *const FunctionTailcall3WithInfo) -> windows_core::Result<()>;
    fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> windows_core::Result<()>;
    fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> windows_core::Result<()>;
    fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize) -> windows_core::Result<usize>;
    fn EnumModules(&self) -> windows_core::Result<ICorProfilerModuleEnum>;
    fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAppDomainsContainingModule(&self, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> windows_core::Result<()>;
    fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo3 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo3_Vtbl
    where
        Identity: ICorProfilerInfo3_Impl,
    {
        unsafe extern "system" fn EnumJITedFunctions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo3_Impl::EnumJITedFunctions(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestProfilerDetach<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwexpectedcompletionmilliseconds: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::RequestProfilerDetach(this, core::mem::transmute_copy(&dwexpectedcompletionmilliseconds)).into()
        }
        unsafe extern "system" fn SetFunctionIDMapper2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfunc: *const FunctionIDMapper2, clientdata: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::SetFunctionIDMapper2(this, core::mem::transmute_copy(&pfunc), core::mem::transmute_copy(&clientdata)).into()
        }
        unsafe extern "system" fn GetStringLayout2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::GetStringLayout2(this, core::mem::transmute_copy(&pstringlengthoffset), core::mem::transmute_copy(&pbufferoffset)).into()
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncenter3: *const FunctionEnter3, pfuncleave3: *const FunctionLeave3, pfunctailcall3: *const FunctionTailcall3) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::SetEnterLeaveFunctionHooks3(this, core::mem::transmute_copy(&pfuncenter3), core::mem::transmute_copy(&pfuncleave3), core::mem::transmute_copy(&pfunctailcall3)).into()
        }
        unsafe extern "system" fn SetEnterLeaveFunctionHooks3WithInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfuncenter3withinfo: *const FunctionEnter3WithInfo, pfuncleave3withinfo: *const FunctionLeave3WithInfo, pfunctailcall3withinfo: *const FunctionTailcall3WithInfo) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::SetEnterLeaveFunctionHooks3WithInfo(this, core::mem::transmute_copy(&pfuncenter3withinfo), core::mem::transmute_copy(&pfuncleave3withinfo), core::mem::transmute_copy(&pfunctailcall3withinfo)).into()
        }
        unsafe extern "system" fn GetFunctionEnter3Info<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::GetFunctionEnter3Info(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&eltinfo), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&pcbargumentinfo), core::mem::transmute_copy(&pargumentinfo)).into()
        }
        unsafe extern "system" fn GetFunctionLeave3Info<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::GetFunctionLeave3Info(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&eltinfo), core::mem::transmute_copy(&pframeinfo), core::mem::transmute_copy(&pretvalrange)).into()
        }
        unsafe extern "system" fn GetFunctionTailcall3Info<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, eltinfo: usize, pframeinfo: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo3_Impl::GetFunctionTailcall3Info(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&eltinfo)) {
                Ok(ok__) => {
                    pframeinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumModules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo3_Impl::EnumModules(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeInformation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, cchversionstring: u32, pcchversionstring: *mut u32, szversionstring: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::GetRuntimeInformation(this, core::mem::transmute_copy(&pclrinstanceid), core::mem::transmute_copy(&pruntimetype), core::mem::transmute_copy(&pmajorversion), core::mem::transmute_copy(&pminorversion), core::mem::transmute_copy(&pbuildnumber), core::mem::transmute_copy(&pqfeversion), core::mem::transmute_copy(&cchversionstring), core::mem::transmute_copy(&pcchversionstring), core::mem::transmute_copy(&szversionstring)).into()
        }
        unsafe extern "system" fn GetThreadStaticAddress2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::GetThreadStaticAddress2(this, core::mem::transmute_copy(&classid), core::mem::transmute_copy(&fieldtoken), core::mem::transmute_copy(&appdomainid), core::mem::transmute_copy(&threadid), core::mem::transmute_copy(&ppaddress)).into()
        }
        unsafe extern "system" fn GetAppDomainsContainingModule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, cappdomainids: u32, pcappdomainids: *mut u32, appdomainids: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::GetAppDomainsContainingModule(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&cappdomainids), core::mem::transmute_copy(&pcappdomainids), core::mem::transmute_copy(&appdomainids)).into()
        }
        unsafe extern "system" fn GetModuleInfo2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, ppbaseloadaddress: *mut *mut u8, cchname: u32, pcchname: *mut u32, szname: windows_core::PWSTR, passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo3_Impl::GetModuleInfo2(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&ppbaseloadaddress), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&passemblyid), core::mem::transmute_copy(&pdwmoduleflags)).into()
        }
        Self {
            base__: ICorProfilerInfo2_Vtbl::new::<Identity, OFFSET>(),
            EnumJITedFunctions: EnumJITedFunctions::<Identity, OFFSET>,
            RequestProfilerDetach: RequestProfilerDetach::<Identity, OFFSET>,
            SetFunctionIDMapper2: SetFunctionIDMapper2::<Identity, OFFSET>,
            GetStringLayout2: GetStringLayout2::<Identity, OFFSET>,
            SetEnterLeaveFunctionHooks3: SetEnterLeaveFunctionHooks3::<Identity, OFFSET>,
            SetEnterLeaveFunctionHooks3WithInfo: SetEnterLeaveFunctionHooks3WithInfo::<Identity, OFFSET>,
            GetFunctionEnter3Info: GetFunctionEnter3Info::<Identity, OFFSET>,
            GetFunctionLeave3Info: GetFunctionLeave3Info::<Identity, OFFSET>,
            GetFunctionTailcall3Info: GetFunctionTailcall3Info::<Identity, OFFSET>,
            EnumModules: EnumModules::<Identity, OFFSET>,
            GetRuntimeInformation: GetRuntimeInformation::<Identity, OFFSET>,
            GetThreadStaticAddress2: GetThreadStaticAddress2::<Identity, OFFSET>,
            GetAppDomainsContainingModule: GetAppDomainsContainingModule::<Identity, OFFSET>,
            GetModuleInfo2: GetModuleInfo2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo4_Impl: Sized + ICorProfilerInfo3_Impl {
    fn EnumThreads(&self) -> windows_core::Result<ICorProfilerThreadEnum>;
    fn InitializeCurrentThread(&self) -> windows_core::Result<()>;
    fn RequestReJIT(&self, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::Result<()>;
    fn RequestRevert(&self, cfunctions: u32, moduleids: *const usize, methodids: *const u32, status: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::Result<()>;
    fn GetFunctionFromIP2(&self, ip: *const u8, pfunctionid: *mut usize, prejitid: *mut usize) -> windows_core::Result<()>;
    fn GetReJITIDs(&self, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> windows_core::Result<()>;
    fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::Result<()>;
    fn EnumJITedFunctions2(&self) -> windows_core::Result<ICorProfilerFunctionEnum>;
    fn GetObjectSize2(&self, objectid: usize) -> windows_core::Result<usize>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo4 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo4_Vtbl
    where
        Identity: ICorProfilerInfo4_Impl,
    {
        unsafe extern "system" fn EnumThreads<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo4_Impl::EnumThreads(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeCurrentThread<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo4_Impl::InitializeCurrentThread(this).into()
        }
        unsafe extern "system" fn RequestReJIT<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo4_Impl::RequestReJIT(this, core::mem::transmute_copy(&cfunctions), core::mem::transmute_copy(&moduleids), core::mem::transmute_copy(&methodids)).into()
        }
        unsafe extern "system" fn RequestRevert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfunctions: u32, moduleids: *const usize, methodids: *const u32, status: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo4_Impl::RequestRevert(this, core::mem::transmute_copy(&cfunctions), core::mem::transmute_copy(&moduleids), core::mem::transmute_copy(&methodids), core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetCodeInfo3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo4_Impl::GetCodeInfo3(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&ccodeinfos), core::mem::transmute_copy(&pccodeinfos), core::mem::transmute_copy(&codeinfos)).into()
        }
        unsafe extern "system" fn GetFunctionFromIP2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ip: *const u8, pfunctionid: *mut usize, prejitid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo4_Impl::GetFunctionFromIP2(this, core::mem::transmute_copy(&ip), core::mem::transmute_copy(&pfunctionid), core::mem::transmute_copy(&prejitid)).into()
        }
        unsafe extern "system" fn GetReJITIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, crejitids: u32, pcrejitids: *mut u32, rejitids: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo4_Impl::GetReJITIDs(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&crejitids), core::mem::transmute_copy(&pcrejitids), core::mem::transmute_copy(&rejitids)).into()
        }
        unsafe extern "system" fn GetILToNativeMapping2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo4_Impl::GetILToNativeMapping2(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&cmap), core::mem::transmute_copy(&pcmap), core::mem::transmute_copy(&map)).into()
        }
        unsafe extern "system" fn EnumJITedFunctions2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo4_Impl::EnumJITedFunctions2(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectSize2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: usize, pcsize: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo4_Impl::GetObjectSize2(this, core::mem::transmute_copy(&objectid)) {
                Ok(ok__) => {
                    pcsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICorProfilerInfo3_Vtbl::new::<Identity, OFFSET>(),
            EnumThreads: EnumThreads::<Identity, OFFSET>,
            InitializeCurrentThread: InitializeCurrentThread::<Identity, OFFSET>,
            RequestReJIT: RequestReJIT::<Identity, OFFSET>,
            RequestRevert: RequestRevert::<Identity, OFFSET>,
            GetCodeInfo3: GetCodeInfo3::<Identity, OFFSET>,
            GetFunctionFromIP2: GetFunctionFromIP2::<Identity, OFFSET>,
            GetReJITIDs: GetReJITIDs::<Identity, OFFSET>,
            GetILToNativeMapping2: GetILToNativeMapping2::<Identity, OFFSET>,
            EnumJITedFunctions2: EnumJITedFunctions2::<Identity, OFFSET>,
            GetObjectSize2: GetObjectSize2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo5_Impl: Sized + ICorProfilerInfo4_Impl {
    fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> windows_core::Result<()>;
    fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo5 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo5_Vtbl
    where
        Identity: ICorProfilerInfo5_Impl,
    {
        unsafe extern "system" fn GetEventMask2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo5_Impl::GetEventMask2(this, core::mem::transmute_copy(&pdweventslow), core::mem::transmute_copy(&pdweventshigh)).into()
        }
        unsafe extern "system" fn SetEventMask2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweventslow: u32, dweventshigh: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo5_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo5_Impl::SetEventMask2(this, core::mem::transmute_copy(&dweventslow), core::mem::transmute_copy(&dweventshigh)).into()
        }
        Self {
            base__: ICorProfilerInfo4_Vtbl::new::<Identity, OFFSET>(),
            GetEventMask2: GetEventMask2::<Identity, OFFSET>,
            SetEventMask2: SetEventMask2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo6_Impl: Sized + ICorProfilerInfo5_Impl {
    fn EnumNgenModuleMethodsInliningThisMethod(&self, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut Option<ICorProfilerMethodEnum>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo6 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo6_Vtbl
    where
        Identity: ICorProfilerInfo6_Impl,
    {
        unsafe extern "system" fn EnumNgenModuleMethodsInliningThisMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo6_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo6_Impl::EnumNgenModuleMethodsInliningThisMethod(this, core::mem::transmute_copy(&inlinersmoduleid), core::mem::transmute_copy(&inlineemoduleid), core::mem::transmute_copy(&inlineemethodid), core::mem::transmute_copy(&incompletedata), core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: ICorProfilerInfo5_Vtbl::new::<Identity, OFFSET>(),
            EnumNgenModuleMethodsInliningThisMethod: EnumNgenModuleMethodsInliningThisMethod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo7_Impl: Sized + ICorProfilerInfo6_Impl {
    fn ApplyMetaData(&self, moduleid: usize) -> windows_core::Result<()>;
    fn GetInMemorySymbolsLength(&self, moduleid: usize) -> windows_core::Result<u32>;
    fn ReadInMemorySymbols(&self, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo7 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo7_Vtbl
    where
        Identity: ICorProfilerInfo7_Impl,
    {
        unsafe extern "system" fn ApplyMetaData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo7_Impl::ApplyMetaData(this, core::mem::transmute_copy(&moduleid)).into()
        }
        unsafe extern "system" fn GetInMemorySymbolsLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, pcountsymbolbytes: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo7_Impl::GetInMemorySymbolsLength(this, core::mem::transmute_copy(&moduleid)) {
                Ok(ok__) => {
                    pcountsymbolbytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInMemorySymbols<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo7_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo7_Impl::ReadInMemorySymbols(this, core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&symbolsreadoffset), core::mem::transmute_copy(&psymbolbytes), core::mem::transmute_copy(&countsymbolbytes), core::mem::transmute_copy(&pcountsymbolbytesread)).into()
        }
        Self {
            base__: ICorProfilerInfo6_Vtbl::new::<Identity, OFFSET>(),
            ApplyMetaData: ApplyMetaData::<Identity, OFFSET>,
            GetInMemorySymbolsLength: GetInMemorySymbolsLength::<Identity, OFFSET>,
            ReadInMemorySymbols: ReadInMemorySymbols::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo8_Impl: Sized + ICorProfilerInfo7_Impl {
    fn IsFunctionDynamic(&self, functionid: usize) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetFunctionFromIP3(&self, ip: *const u8, functionid: *mut usize, prejitid: *mut usize) -> windows_core::Result<()>;
    fn GetDynamicFunctionInfo(&self, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo8 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo8_Vtbl
    where
        Identity: ICorProfilerInfo8_Impl,
    {
        unsafe extern "system" fn IsFunctionDynamic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, isdynamic: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerInfo8_Impl::IsFunctionDynamic(this, core::mem::transmute_copy(&functionid)) {
                Ok(ok__) => {
                    isdynamic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionFromIP3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ip: *const u8, functionid: *mut usize, prejitid: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo8_Impl::GetFunctionFromIP3(this, core::mem::transmute_copy(&ip), core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&prejitid)).into()
        }
        unsafe extern "system" fn GetDynamicFunctionInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo8_Impl::GetDynamicFunctionInfo(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&moduleid), core::mem::transmute_copy(&ppvsig), core::mem::transmute_copy(&pbsig), core::mem::transmute_copy(&cchname), core::mem::transmute_copy(&pcchname), core::mem::transmute_copy(&wszname)).into()
        }
        Self {
            base__: ICorProfilerInfo7_Vtbl::new::<Identity, OFFSET>(),
            IsFunctionDynamic: IsFunctionDynamic::<Identity, OFFSET>,
            GetFunctionFromIP3: GetFunctionFromIP3::<Identity, OFFSET>,
            GetDynamicFunctionInfo: GetDynamicFunctionInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
pub trait ICorProfilerInfo9_Impl: Sized + ICorProfilerInfo8_Impl {
    fn GetNativeCodeStartAddresses(&self, functionid: usize, rejitid: usize, ccodestartaddresses: u32, pccodestartaddresses: *mut u32, codestartaddresses: *mut usize) -> windows_core::Result<()>;
    fn GetILToNativeMapping3(&self, pnativecodestartaddress: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::Result<()>;
    fn GetCodeInfo4(&self, pnativecodestartaddress: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::RuntimeName for ICorProfilerInfo9 {}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl ICorProfilerInfo9_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerInfo9_Vtbl
    where
        Identity: ICorProfilerInfo9_Impl,
    {
        unsafe extern "system" fn GetNativeCodeStartAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: usize, rejitid: usize, ccodestartaddresses: u32, pccodestartaddresses: *mut u32, codestartaddresses: *mut usize) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo9_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo9_Impl::GetNativeCodeStartAddresses(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&rejitid), core::mem::transmute_copy(&ccodestartaddresses), core::mem::transmute_copy(&pccodestartaddresses), core::mem::transmute_copy(&codestartaddresses)).into()
        }
        unsafe extern "system" fn GetILToNativeMapping3<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnativecodestartaddress: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo9_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo9_Impl::GetILToNativeMapping3(this, core::mem::transmute_copy(&pnativecodestartaddress), core::mem::transmute_copy(&cmap), core::mem::transmute_copy(&pcmap), core::mem::transmute_copy(&map)).into()
        }
        unsafe extern "system" fn GetCodeInfo4<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnativecodestartaddress: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT
        where
            Identity: ICorProfilerInfo9_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerInfo9_Impl::GetCodeInfo4(this, core::mem::transmute_copy(&pnativecodestartaddress), core::mem::transmute_copy(&ccodeinfos), core::mem::transmute_copy(&pccodeinfos), core::mem::transmute_copy(&codeinfos)).into()
        }
        Self {
            base__: ICorProfilerInfo8_Vtbl::new::<Identity, OFFSET>(),
            GetNativeCodeStartAddresses: GetNativeCodeStartAddresses::<Identity, OFFSET>,
            GetILToNativeMapping3: GetILToNativeMapping3::<Identity, OFFSET>,
            GetCodeInfo4: GetCodeInfo4::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerInfo9 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo as windows_core::Interface>::IID || iid == &<ICorProfilerInfo2 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo3 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo4 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo5 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo6 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo7 as windows_core::Interface>::IID || iid == &<ICorProfilerInfo8 as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerMethodEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerMethodEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerMethodEnum {}
impl ICorProfilerMethodEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerMethodEnum_Vtbl
    where
        Identity: ICorProfilerMethodEnum_Impl,
    {
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerMethodEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerMethodEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerMethodEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerMethodEnum_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerMethodEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerMethodEnum_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerMethodEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerMethodEnum_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, elements: *mut COR_PRF_METHOD, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerMethodEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerMethodEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&elements), core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerMethodEnum as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerModuleEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerModuleEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerModuleEnum {}
impl ICorProfilerModuleEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerModuleEnum_Vtbl
    where
        Identity: ICorProfilerModuleEnum_Impl,
    {
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerModuleEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerModuleEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerModuleEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerModuleEnum_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerModuleEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerModuleEnum_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerModuleEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerModuleEnum_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerModuleEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerModuleEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ids), core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerModuleEnum as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerObjectEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerObjectEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerObjectEnum {}
impl ICorProfilerObjectEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerObjectEnum_Vtbl
    where
        Identity: ICorProfilerObjectEnum_Impl,
    {
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerObjectEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerObjectEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerObjectEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerObjectEnum_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerObjectEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerObjectEnum_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerObjectEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerObjectEnum_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, objects: *mut usize, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerObjectEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerObjectEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&objects), core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerObjectEnum as windows_core::Interface>::IID
    }
}
pub trait ICorProfilerThreadEnum_Impl: Sized {
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICorProfilerThreadEnum>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Next(&self, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICorProfilerThreadEnum {}
impl ICorProfilerThreadEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICorProfilerThreadEnum_Vtbl
    where
        Identity: ICorProfilerThreadEnum_Impl,
    {
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerThreadEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerThreadEnum_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerThreadEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerThreadEnum_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICorProfilerThreadEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerThreadEnum_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerThreadEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICorProfilerThreadEnum_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcelt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ids: *mut usize, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICorProfilerThreadEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICorProfilerThreadEnum_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ids), core::mem::transmute_copy(&pceltfetched)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICorProfilerThreadEnum as windows_core::Interface>::IID
    }
}
pub trait IMethodMalloc_Impl: Sized {
    fn Alloc(&self, cb: u32) -> *mut core::ffi::c_void;
}
impl windows_core::RuntimeName for IMethodMalloc {}
impl IMethodMalloc_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMethodMalloc_Vtbl
    where
        Identity: IMethodMalloc_Impl,
    {
        unsafe extern "system" fn Alloc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: u32) -> *mut core::ffi::c_void
        where
            Identity: IMethodMalloc_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMethodMalloc_Impl::Alloc(this, core::mem::transmute_copy(&cb))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Alloc: Alloc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMethodMalloc as windows_core::Interface>::IID
    }
}
