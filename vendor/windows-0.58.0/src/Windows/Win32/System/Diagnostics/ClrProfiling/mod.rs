windows_core::imp::define_interface!(ICorProfilerAssemblyReferenceProvider, ICorProfilerAssemblyReferenceProvider_Vtbl, 0x66a78c24_2eef_4f65_b45f_dd1d8038bf3c);
impl core::ops::Deref for ICorProfilerAssemblyReferenceProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerAssemblyReferenceProvider, windows_core::IUnknown);
impl ICorProfilerAssemblyReferenceProvider {
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn AddAssemblyReference(&self, passemblyrefinfo: *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddAssemblyReference)(windows_core::Interface::as_raw(self), passemblyrefinfo).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerAssemblyReferenceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub AddAssemblyReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const COR_PRF_ASSEMBLY_REFERENCE_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    AddAssemblyReference: usize,
}
windows_core::imp::define_interface!(ICorProfilerCallback, ICorProfilerCallback_Vtbl, 0x176fbed1_a55c_4796_98ca_a9da0ef883e7);
impl core::ops::Deref for ICorProfilerCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback, windows_core::IUnknown);
impl ICorProfilerCallback {
    pub unsafe fn Initialize<P0>(&self, picorprofilerinfounk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), picorprofilerinfounk.param().abi()).ok()
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AppDomainCreationStarted(&self, appdomainid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AppDomainCreationStarted)(windows_core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AppDomainCreationFinished)(windows_core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AppDomainShutdownStarted(&self, appdomainid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AppDomainShutdownStarted)(windows_core::Interface::as_raw(self), appdomainid).ok()
    }
    pub unsafe fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AppDomainShutdownFinished)(windows_core::Interface::as_raw(self), appdomainid, hrstatus).ok()
    }
    pub unsafe fn AssemblyLoadStarted(&self, assemblyid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssemblyLoadStarted)(windows_core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssemblyLoadFinished)(windows_core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn AssemblyUnloadStarted(&self, assemblyid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssemblyUnloadStarted)(windows_core::Interface::as_raw(self), assemblyid).ok()
    }
    pub unsafe fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AssemblyUnloadFinished)(windows_core::Interface::as_raw(self), assemblyid, hrstatus).ok()
    }
    pub unsafe fn ModuleLoadStarted(&self, moduleid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModuleLoadStarted)(windows_core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModuleLoadFinished)(windows_core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleUnloadStarted(&self, moduleid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModuleUnloadStarted)(windows_core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModuleUnloadFinished)(windows_core::Interface::as_raw(self), moduleid, hrstatus).ok()
    }
    pub unsafe fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModuleAttachedToAssembly)(windows_core::Interface::as_raw(self), moduleid, assemblyid).ok()
    }
    pub unsafe fn ClassLoadStarted(&self, classid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClassLoadStarted)(windows_core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassLoadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClassLoadFinished)(windows_core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn ClassUnloadStarted(&self, classid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClassUnloadStarted)(windows_core::Interface::as_raw(self), classid).ok()
    }
    pub unsafe fn ClassUnloadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClassUnloadFinished)(windows_core::Interface::as_raw(self), classid, hrstatus).ok()
    }
    pub unsafe fn FunctionUnloadStarted(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FunctionUnloadStarted)(windows_core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn JITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).JITCompilationStarted)(windows_core::Interface::as_raw(self), functionid, fissafetoblock.param().abi()).ok()
    }
    pub unsafe fn JITCompilationFinished<P0>(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).JITCompilationFinished)(windows_core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.param().abi()).ok()
    }
    pub unsafe fn JITCachedFunctionSearchStarted(&self, functionid: usize) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).JITCachedFunctionSearchStarted)(windows_core::Interface::as_raw(self), functionid, &mut result__).map(|| result__)
    }
    pub unsafe fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: COR_PRF_JIT_CACHE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).JITCachedFunctionSearchFinished)(windows_core::Interface::as_raw(self), functionid, result).ok()
    }
    pub unsafe fn JITFunctionPitched(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).JITFunctionPitched)(windows_core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn JITInlining(&self, callerid: usize, calleeid: usize) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).JITInlining)(windows_core::Interface::as_raw(self), callerid, calleeid, &mut result__).map(|| result__)
    }
    pub unsafe fn ThreadCreated(&self, threadid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadCreated)(windows_core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadDestroyed(&self, threadid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadDestroyed)(windows_core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadAssignedToOSThread)(windows_core::Interface::as_raw(self), managedthreadid, osthreadid).ok()
    }
    pub unsafe fn RemotingClientInvocationStarted(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemotingClientInvocationStarted)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingClientSendingMessage<P0>(&self, pcookie: *const windows_core::GUID, fisasync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RemotingClientSendingMessage)(windows_core::Interface::as_raw(self), pcookie, fisasync.param().abi()).ok()
    }
    pub unsafe fn RemotingClientReceivingReply<P0>(&self, pcookie: *const windows_core::GUID, fisasync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RemotingClientReceivingReply)(windows_core::Interface::as_raw(self), pcookie, fisasync.param().abi()).ok()
    }
    pub unsafe fn RemotingClientInvocationFinished(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemotingClientInvocationFinished)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerReceivingMessage<P0>(&self, pcookie: *const windows_core::GUID, fisasync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RemotingServerReceivingMessage)(windows_core::Interface::as_raw(self), pcookie, fisasync.param().abi()).ok()
    }
    pub unsafe fn RemotingServerInvocationStarted(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemotingServerInvocationStarted)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerInvocationReturned(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemotingServerInvocationReturned)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RemotingServerSendingReply<P0>(&self, pcookie: *const windows_core::GUID, fisasync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RemotingServerSendingReply)(windows_core::Interface::as_raw(self), pcookie, fisasync.param().abi()).ok()
    }
    pub unsafe fn UnmanagedToManagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnmanagedToManagedTransition)(windows_core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ManagedToUnmanagedTransition)(windows_core::Interface::as_raw(self), functionid, reason).ok()
    }
    pub unsafe fn RuntimeSuspendStarted(&self, suspendreason: COR_PRF_SUSPEND_REASON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RuntimeSuspendStarted)(windows_core::Interface::as_raw(self), suspendreason).ok()
    }
    pub unsafe fn RuntimeSuspendFinished(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RuntimeSuspendFinished)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeSuspendAborted(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RuntimeSuspendAborted)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeStarted(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RuntimeResumeStarted)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeResumeFinished(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RuntimeResumeFinished)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RuntimeThreadSuspended(&self, threadid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RuntimeThreadSuspended)(windows_core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn RuntimeThreadResumed(&self, threadid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RuntimeThreadResumed)(windows_core::Interface::as_raw(self), threadid).ok()
    }
    pub unsafe fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MovedReferences)(windows_core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn ObjectAllocated(&self, objectid: usize, classid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ObjectAllocated)(windows_core::Interface::as_raw(self), objectid, classid).ok()
    }
    pub unsafe fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ObjectsAllocatedByClass)(windows_core::Interface::as_raw(self), cclasscount, classids, cobjects).ok()
    }
    pub unsafe fn ObjectReferences(&self, objectid: usize, classid: usize, objectrefids: &[usize]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ObjectReferences)(windows_core::Interface::as_raw(self), objectid, classid, objectrefids.len().try_into().unwrap(), core::mem::transmute(objectrefids.as_ptr())).ok()
    }
    pub unsafe fn RootReferences(&self, rootrefids: &[usize]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RootReferences)(windows_core::Interface::as_raw(self), rootrefids.len().try_into().unwrap(), core::mem::transmute(rootrefids.as_ptr())).ok()
    }
    pub unsafe fn ExceptionThrown(&self, thrownobjectid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionThrown)(windows_core::Interface::as_raw(self), thrownobjectid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionSearchFunctionEnter)(windows_core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFunctionLeave(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionSearchFunctionLeave)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchFilterEnter(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionSearchFilterEnter)(windows_core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionSearchFilterLeave(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionSearchFilterLeave)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionSearchCatcherFound(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionSearchCatcherFound)(windows_core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionOSHandlerEnter(&self, __unused: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionOSHandlerEnter)(windows_core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionOSHandlerLeave(&self, __unused: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionOSHandlerLeave)(windows_core::Interface::as_raw(self), __unused).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionUnwindFunctionEnter)(windows_core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFunctionLeave(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionUnwindFunctionLeave)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionUnwindFinallyEnter)(windows_core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ExceptionUnwindFinallyLeave(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionUnwindFinallyLeave)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionCatcherEnter)(windows_core::Interface::as_raw(self), functionid, objectid).ok()
    }
    pub unsafe fn ExceptionCatcherLeave(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionCatcherLeave)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void, cslots: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).COMClassicVTableCreated)(windows_core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable, cslots).ok()
    }
    pub unsafe fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).COMClassicVTableDestroyed)(windows_core::Interface::as_raw(self), wrappedclassid, implementediid, pvtable).ok()
    }
    pub unsafe fn ExceptionCLRCatcherFound(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionCLRCatcherFound)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExceptionCLRCatcherExecute(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExceptionCLRCatcherExecute)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppDomainCreationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AppDomainCreationFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AppDomainShutdownStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AppDomainShutdownFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AssemblyLoadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AssemblyLoadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub AssemblyUnloadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub AssemblyUnloadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ModuleLoadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ModuleLoadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ModuleUnloadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ModuleUnloadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ModuleAttachedToAssembly: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub ClassLoadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ClassLoadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ClassUnloadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ClassUnloadFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub FunctionUnloadStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub JITCompilationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub JITCompilationFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub JITCachedFunctionSearchStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub JITCachedFunctionSearchFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, COR_PRF_JIT_CACHE) -> windows_core::HRESULT,
    pub JITFunctionPitched: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub JITInlining: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ThreadCreated: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ThreadDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ThreadAssignedToOSThread: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32) -> windows_core::HRESULT,
    pub RemotingClientInvocationStarted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotingClientSendingMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RemotingClientReceivingReply: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RemotingClientInvocationFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotingServerReceivingMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RemotingServerInvocationStarted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotingServerInvocationReturned: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotingServerSendingReply: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub UnmanagedToManagedTransition: unsafe extern "system" fn(*mut core::ffi::c_void, usize, COR_PRF_TRANSITION_REASON) -> windows_core::HRESULT,
    pub ManagedToUnmanagedTransition: unsafe extern "system" fn(*mut core::ffi::c_void, usize, COR_PRF_TRANSITION_REASON) -> windows_core::HRESULT,
    pub RuntimeSuspendStarted: unsafe extern "system" fn(*mut core::ffi::c_void, COR_PRF_SUSPEND_REASON) -> windows_core::HRESULT,
    pub RuntimeSuspendFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuntimeSuspendAborted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuntimeResumeStarted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuntimeResumeFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuntimeThreadSuspended: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub RuntimeThreadResumed: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub MovedReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const usize, *const u32) -> windows_core::HRESULT,
    pub ObjectAllocated: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub ObjectsAllocatedByClass: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const u32) -> windows_core::HRESULT,
    pub ObjectReferences: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *const usize) -> windows_core::HRESULT,
    pub RootReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize) -> windows_core::HRESULT,
    pub ExceptionThrown: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionSearchFunctionEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionSearchFunctionLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionSearchFilterEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionSearchFilterLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionSearchCatcherFound: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionOSHandlerEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionOSHandlerLeave: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionUnwindFunctionEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionUnwindFunctionLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionUnwindFinallyEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ExceptionUnwindFinallyLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionCatcherEnter: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub ExceptionCatcherLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub COMClassicVTableCreated: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub COMClassicVTableDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionCLRCatcherFound: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExceptionCLRCatcherExecute: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback10, ICorProfilerCallback10_Vtbl, 0xcec5b60e_c69c_495f_87f6_84d28ee16ffb);
impl core::ops::Deref for ICorProfilerCallback10 {
    type Target = ICorProfilerCallback9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback10, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7, ICorProfilerCallback8, ICorProfilerCallback9);
impl ICorProfilerCallback10 {
    pub unsafe fn EventPipeEventDelivered(&self, provider: usize, eventid: u32, eventversion: u32, metadatablob: &[u8], eventdata: &[u8], pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID, eventthread: usize, numstackframes: u32, stackframes: *const usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EventPipeEventDelivered)(windows_core::Interface::as_raw(self), provider, eventid, eventversion, metadatablob.len().try_into().unwrap(), core::mem::transmute(metadatablob.as_ptr()), eventdata.len().try_into().unwrap(), core::mem::transmute(eventdata.as_ptr()), pactivityid, prelatedactivityid, eventthread, numstackframes, stackframes).ok()
    }
    pub unsafe fn EventPipeProviderCreated(&self, provider: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EventPipeProviderCreated)(windows_core::Interface::as_raw(self), provider).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback10_Vtbl {
    pub base__: ICorProfilerCallback9_Vtbl,
    pub EventPipeEventDelivered: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, u32, u32, *const u8, u32, *const u8, *const windows_core::GUID, *const windows_core::GUID, usize, u32, *const usize) -> windows_core::HRESULT,
    pub EventPipeProviderCreated: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback11, ICorProfilerCallback11_Vtbl, 0x42350846_aaed_47f7_b128_fd0c98881cde);
impl core::ops::Deref for ICorProfilerCallback11 {
    type Target = ICorProfilerCallback10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback11, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7, ICorProfilerCallback8, ICorProfilerCallback9, ICorProfilerCallback10);
impl ICorProfilerCallback11 {
    pub unsafe fn LoadAsNotificationOnly(&self, pbnotificationonly: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadAsNotificationOnly)(windows_core::Interface::as_raw(self), pbnotificationonly).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback11_Vtbl {
    pub base__: ICorProfilerCallback10_Vtbl,
    pub LoadAsNotificationOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback2, ICorProfilerCallback2_Vtbl, 0x8a8cc829_ccf2_49fe_bbae_0f022228071a);
impl core::ops::Deref for ICorProfilerCallback2 {
    type Target = ICorProfilerCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback2, windows_core::IUnknown, ICorProfilerCallback);
impl ICorProfilerCallback2 {
    pub unsafe fn ThreadNameChanged(&self, threadid: usize, name: Option<&[u16]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ThreadNameChanged)(windows_core::Interface::as_raw(self), threadid, name.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(name.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn GarbageCollectionStarted(&self, generationcollected: &[super::super::super::Foundation::BOOL], reason: COR_PRF_GC_REASON) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GarbageCollectionStarted)(windows_core::Interface::as_raw(self), generationcollected.len().try_into().unwrap(), core::mem::transmute(generationcollected.as_ptr()), reason).ok()
    }
    pub unsafe fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SurvivingReferences)(windows_core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn GarbageCollectionFinished(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GarbageCollectionFinished)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FinalizeableObjectQueued)(windows_core::Interface::as_raw(self), finalizerflags, objectid).ok()
    }
    pub unsafe fn RootReferences2(&self, crootrefs: u32, rootrefids: *const usize, rootkinds: *const COR_PRF_GC_ROOT_KIND, rootflags: *const COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RootReferences2)(windows_core::Interface::as_raw(self), crootrefs, rootrefids, rootkinds, rootflags, rootids).ok()
    }
    pub unsafe fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleCreated)(windows_core::Interface::as_raw(self), handleid, initialobjectid).ok()
    }
    pub unsafe fn HandleDestroyed(&self, handleid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).HandleDestroyed)(windows_core::Interface::as_raw(self), handleid).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback2_Vtbl {
    pub base__: ICorProfilerCallback_Vtbl,
    pub ThreadNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GarbageCollectionStarted: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const super::super::super::Foundation::BOOL, COR_PRF_GC_REASON) -> windows_core::HRESULT,
    pub SurvivingReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const u32) -> windows_core::HRESULT,
    pub GarbageCollectionFinished: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinalizeableObjectQueued: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize) -> windows_core::HRESULT,
    pub RootReferences2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const COR_PRF_GC_ROOT_KIND, *const COR_PRF_GC_ROOT_FLAGS, *const usize) -> windows_core::HRESULT,
    pub HandleCreated: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub HandleDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback3, ICorProfilerCallback3_Vtbl, 0x4fd2ed52_7731_4b8d_9469_03d2cc3086c5);
impl core::ops::Deref for ICorProfilerCallback3 {
    type Target = ICorProfilerCallback2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback3, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2);
impl ICorProfilerCallback3 {
    pub unsafe fn InitializeForAttach<P0>(&self, pcorprofilerinfounk: P0, pvclientdata: *const core::ffi::c_void, cbclientdata: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).InitializeForAttach)(windows_core::Interface::as_raw(self), pcorprofilerinfounk.param().abi(), pvclientdata, cbclientdata).ok()
    }
    pub unsafe fn ProfilerAttachComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProfilerAttachComplete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProfilerDetachSucceeded(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProfilerDetachSucceeded)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback3_Vtbl {
    pub base__: ICorProfilerCallback2_Vtbl,
    pub InitializeForAttach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ProfilerAttachComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProfilerDetachSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback4, ICorProfilerCallback4_Vtbl, 0x7b63b2e3_107d_4d48_b2f6_f61e229470d2);
impl core::ops::Deref for ICorProfilerCallback4 {
    type Target = ICorProfilerCallback3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback4, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3);
impl ICorProfilerCallback4 {
    pub unsafe fn ReJITCompilationStarted<P0>(&self, functionid: usize, rejitid: usize, fissafetoblock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ReJITCompilationStarted)(windows_core::Interface::as_raw(self), functionid, rejitid, fissafetoblock.param().abi()).ok()
    }
    pub unsafe fn GetReJITParameters<P0>(&self, moduleid: usize, methodid: u32, pfunctioncontrol: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICorProfilerFunctionControl>,
    {
        (windows_core::Interface::vtable(self).GetReJITParameters)(windows_core::Interface::as_raw(self), moduleid, methodid, pfunctioncontrol.param().abi()).ok()
    }
    pub unsafe fn ReJITCompilationFinished<P0>(&self, functionid: usize, rejitid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ReJITCompilationFinished)(windows_core::Interface::as_raw(self), functionid, rejitid, hrstatus, fissafetoblock.param().abi()).ok()
    }
    pub unsafe fn ReJITError(&self, moduleid: usize, methodid: u32, functionid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReJITError)(windows_core::Interface::as_raw(self), moduleid, methodid, functionid, hrstatus).ok()
    }
    pub unsafe fn MovedReferences2(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MovedReferences2)(windows_core::Interface::as_raw(self), cmovedobjectidranges, oldobjectidrangestart, newobjectidrangestart, cobjectidrangelength).ok()
    }
    pub unsafe fn SurvivingReferences2(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SurvivingReferences2)(windows_core::Interface::as_raw(self), csurvivingobjectidranges, objectidrangestart, cobjectidrangelength).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback4_Vtbl {
    pub base__: ICorProfilerCallback3_Vtbl,
    pub ReJITCompilationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetReJITParameters: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReJITCompilationFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, windows_core::HRESULT, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ReJITError: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, windows_core::HRESULT) -> windows_core::HRESULT,
    pub MovedReferences2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const usize, *const usize) -> windows_core::HRESULT,
    pub SurvivingReferences2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback5, ICorProfilerCallback5_Vtbl, 0x8dfba405_8c9f_45f8_bffa_83b14cef78b5);
impl core::ops::Deref for ICorProfilerCallback5 {
    type Target = ICorProfilerCallback4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback5, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4);
impl ICorProfilerCallback5 {
    pub unsafe fn ConditionalWeakTableElementReferences(&self, crootrefs: u32, keyrefids: *const usize, valuerefids: *const usize, rootids: *const usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConditionalWeakTableElementReferences)(windows_core::Interface::as_raw(self), crootrefs, keyrefids, valuerefids, rootids).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback5_Vtbl {
    pub base__: ICorProfilerCallback4_Vtbl,
    pub ConditionalWeakTableElementReferences: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const usize, *const usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback6, ICorProfilerCallback6_Vtbl, 0xfc13df4b_4448_4f4f_950c_ba8d19d00c36);
impl core::ops::Deref for ICorProfilerCallback6 {
    type Target = ICorProfilerCallback5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback6, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5);
impl ICorProfilerCallback6 {
    pub unsafe fn GetAssemblyReferences<P0, P1>(&self, wszassemblypath: P0, pasmrefprovider: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ICorProfilerAssemblyReferenceProvider>,
    {
        (windows_core::Interface::vtable(self).GetAssemblyReferences)(windows_core::Interface::as_raw(self), wszassemblypath.param().abi(), pasmrefprovider.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback6_Vtbl {
    pub base__: ICorProfilerCallback5_Vtbl,
    pub GetAssemblyReferences: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback7, ICorProfilerCallback7_Vtbl, 0xf76a2dba_1d52_4539_866c_2aa518f9efc3);
impl core::ops::Deref for ICorProfilerCallback7 {
    type Target = ICorProfilerCallback6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback7, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6);
impl ICorProfilerCallback7 {
    pub unsafe fn ModuleInMemorySymbolsUpdated(&self, moduleid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ModuleInMemorySymbolsUpdated)(windows_core::Interface::as_raw(self), moduleid).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback7_Vtbl {
    pub base__: ICorProfilerCallback6_Vtbl,
    pub ModuleInMemorySymbolsUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback8, ICorProfilerCallback8_Vtbl, 0x5bed9b15_c079_4d47_bfe2_215a140c07e0);
impl core::ops::Deref for ICorProfilerCallback8 {
    type Target = ICorProfilerCallback7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback8, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7);
impl ICorProfilerCallback8 {
    pub unsafe fn DynamicMethodJITCompilationStarted<P0>(&self, functionid: usize, fissafetoblock: P0, pilheader: *const u8, cbilheader: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).DynamicMethodJITCompilationStarted)(windows_core::Interface::as_raw(self), functionid, fissafetoblock.param().abi(), pilheader, cbilheader).ok()
    }
    pub unsafe fn DynamicMethodJITCompilationFinished<P0>(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).DynamicMethodJITCompilationFinished)(windows_core::Interface::as_raw(self), functionid, hrstatus, fissafetoblock.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback8_Vtbl {
    pub base__: ICorProfilerCallback7_Vtbl,
    pub DynamicMethodJITCompilationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, usize, super::super::super::Foundation::BOOL, *const u8, u32) -> windows_core::HRESULT,
    pub DynamicMethodJITCompilationFinished: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerCallback9, ICorProfilerCallback9_Vtbl, 0x27583ec3_c8f5_482f_8052_194b8ce4705a);
impl core::ops::Deref for ICorProfilerCallback9 {
    type Target = ICorProfilerCallback8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerCallback9, windows_core::IUnknown, ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback3, ICorProfilerCallback4, ICorProfilerCallback5, ICorProfilerCallback6, ICorProfilerCallback7, ICorProfilerCallback8);
impl ICorProfilerCallback9 {
    pub unsafe fn DynamicMethodUnloaded(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DynamicMethodUnloaded)(windows_core::Interface::as_raw(self), functionid).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerCallback9_Vtbl {
    pub base__: ICorProfilerCallback8_Vtbl,
    pub DynamicMethodUnloaded: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerFunctionControl, ICorProfilerFunctionControl_Vtbl, 0xf0963021_e1ea_4732_8581_e01b0bd3c0c6);
impl core::ops::Deref for ICorProfilerFunctionControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerFunctionControl, windows_core::IUnknown);
impl ICorProfilerFunctionControl {
    pub unsafe fn SetCodegenFlags(&self, flags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCodegenFlags)(windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetILFunctionBody(&self, pbnewilmethodheader: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetILFunctionBody)(windows_core::Interface::as_raw(self), pbnewilmethodheader.len().try_into().unwrap(), core::mem::transmute(pbnewilmethodheader.as_ptr())).ok()
    }
    pub unsafe fn SetILInstrumentedCodeMap(&self, rgilmapentries: &[COR_IL_MAP]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetILInstrumentedCodeMap)(windows_core::Interface::as_raw(self), rgilmapentries.len().try_into().unwrap(), core::mem::transmute(rgilmapentries.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerFunctionControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCodegenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetILFunctionBody: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub SetILInstrumentedCodeMap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const COR_IL_MAP) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerFunctionEnum, ICorProfilerFunctionEnum_Vtbl, 0xff71301a_b994_429d_a10b_b345a65280ef);
impl core::ops::Deref for ICorProfilerFunctionEnum {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerFunctionEnum, windows_core::IUnknown);
impl ICorProfilerFunctionEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Next(&self, ids: &mut [COR_PRF_FUNCTION], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ids.len().try_into().unwrap(), core::mem::transmute(ids.as_ptr()), pceltfetched).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerFunctionEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut COR_PRF_FUNCTION, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo, ICorProfilerInfo_Vtbl, 0x28b5557d_3f3f_48b4_90b2_5f9eea2f6c48);
impl core::ops::Deref for ICorProfilerInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo, windows_core::IUnknown);
impl ICorProfilerInfo {
    pub unsafe fn GetClassFromObject(&self, objectid: usize) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassFromObject)(windows_core::Interface::as_raw(self), objectid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetClassFromToken(&self, moduleid: usize, typedef: u32) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassFromToken)(windows_core::Interface::as_raw(self), moduleid, typedef, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCodeInfo(&self, functionid: usize, pstart: *mut *mut u8, pcsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodeInfo)(windows_core::Interface::as_raw(self), functionid, pstart, pcsize).ok()
    }
    pub unsafe fn GetEventMask(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEventMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFunctionFromIP(&self, ip: *const u8) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFunctionFromIP)(windows_core::Interface::as_raw(self), ip, &mut result__).map(|| result__)
    }
    pub unsafe fn GetFunctionFromToken(&self, moduleid: usize, token: u32) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFunctionFromToken)(windows_core::Interface::as_raw(self), moduleid, token, &mut result__).map(|| result__)
    }
    pub unsafe fn GetHandleFromThread(&self, threadid: usize) -> windows_core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHandleFromThread)(windows_core::Interface::as_raw(self), threadid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetObjectSize(&self, objectid: usize) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectSize)(windows_core::Interface::as_raw(self), objectid, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn IsArrayClass(&self, classid: usize, pbaseelemtype: *mut super::super::WinRT::Metadata::CorElementType, pbaseclassid: *mut usize, pcrank: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsArrayClass)(windows_core::Interface::as_raw(self), classid, pbaseelemtype, pbaseclassid, pcrank).ok()
    }
    pub unsafe fn GetThreadInfo(&self, threadid: usize) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThreadInfo)(windows_core::Interface::as_raw(self), threadid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentThreadID(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentThreadID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetClassIDInfo(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClassIDInfo)(windows_core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken).ok()
    }
    pub unsafe fn GetFunctionInfo(&self, functionid: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFunctionInfo)(windows_core::Interface::as_raw(self), functionid, pclassid, pmoduleid, ptoken).ok()
    }
    pub unsafe fn SetEventMask(&self, dwevents: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventMask)(windows_core::Interface::as_raw(self), dwevents).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks(&self, pfuncenter: *const FunctionEnter, pfuncleave: *const FunctionLeave, pfunctailcall: *const FunctionTailcall) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEnterLeaveFunctionHooks)(windows_core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn SetFunctionIDMapper(&self, pfunc: *const FunctionIDMapper) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFunctionIDMapper)(windows_core::Interface::as_raw(self), pfunc).ok()
    }
    pub unsafe fn GetTokenAndMetaDataFromFunction(&self, functionid: usize, riid: *const windows_core::GUID, ppimport: *mut Option<windows_core::IUnknown>, ptoken: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetTokenAndMetaDataFromFunction)(windows_core::Interface::as_raw(self), functionid, riid, core::mem::transmute(ppimport), ptoken).ok()
    }
    pub unsafe fn GetModuleInfo(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetModuleInfo)(windows_core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len().try_into().unwrap(), pcchname, core::mem::transmute(szname.as_ptr()), passemblyid).ok()
    }
    pub unsafe fn GetModuleMetaData(&self, moduleid: usize, dwopenflags: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetModuleMetaData)(windows_core::Interface::as_raw(self), moduleid, dwopenflags, riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetILFunctionBody(&self, moduleid: usize, methodid: u32, ppmethodheader: *mut *mut u8, pcbmethodsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetILFunctionBody)(windows_core::Interface::as_raw(self), moduleid, methodid, ppmethodheader, pcbmethodsize).ok()
    }
    pub unsafe fn GetILFunctionBodyAllocator(&self, moduleid: usize) -> windows_core::Result<IMethodMalloc> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetILFunctionBodyAllocator)(windows_core::Interface::as_raw(self), moduleid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetILFunctionBody(&self, moduleid: usize, methodid: u32, pbnewilmethodheader: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetILFunctionBody)(windows_core::Interface::as_raw(self), moduleid, methodid, pbnewilmethodheader).ok()
    }
    pub unsafe fn GetAppDomainInfo(&self, appdomainid: usize, pcchname: *mut u32, szname: &mut [u16], pprocessid: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAppDomainInfo)(windows_core::Interface::as_raw(self), appdomainid, szname.len().try_into().unwrap(), pcchname, core::mem::transmute(szname.as_ptr()), pprocessid).ok()
    }
    pub unsafe fn GetAssemblyInfo(&self, assemblyid: usize, pcchname: *mut u32, szname: &mut [u16], pappdomainid: *mut usize, pmoduleid: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAssemblyInfo)(windows_core::Interface::as_raw(self), assemblyid, szname.len().try_into().unwrap(), pcchname, core::mem::transmute(szname.as_ptr()), pappdomainid, pmoduleid).ok()
    }
    pub unsafe fn SetFunctionReJIT(&self, functionid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFunctionReJIT)(windows_core::Interface::as_raw(self), functionid).ok()
    }
    pub unsafe fn ForceGC(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ForceGC)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetILInstrumentedCodeMap<P0>(&self, functionid: usize, fstartjit: P0, rgilmapentries: &[COR_IL_MAP]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetILInstrumentedCodeMap)(windows_core::Interface::as_raw(self), functionid, fstartjit.param().abi(), rgilmapentries.len().try_into().unwrap(), core::mem::transmute(rgilmapentries.as_ptr())).ok()
    }
    pub unsafe fn GetInprocInspectionInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInprocInspectionInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInprocInspectionIThisThread(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInprocInspectionIThisThread)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetThreadContext(&self, threadid: usize) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThreadContext)(windows_core::Interface::as_raw(self), threadid, &mut result__).map(|| result__)
    }
    pub unsafe fn BeginInprocDebugging<P0>(&self, fthisthreadonly: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginInprocDebugging)(windows_core::Interface::as_raw(self), fthisthreadonly.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EndInprocDebugging(&self, dwprofilercontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndInprocDebugging)(windows_core::Interface::as_raw(self), dwprofilercontext).ok()
    }
    pub unsafe fn GetILToNativeMapping(&self, functionid: usize, pcmap: *mut u32, map: &mut [COR_DEBUG_IL_TO_NATIVE_MAP]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetILToNativeMapping)(windows_core::Interface::as_raw(self), functionid, map.len().try_into().unwrap(), pcmap, core::mem::transmute(map.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassFromObject: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub GetClassFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut usize) -> windows_core::HRESULT,
    pub GetCodeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetEventMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFunctionFromIP: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut usize) -> windows_core::HRESULT,
    pub GetFunctionFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut usize) -> windows_core::HRESULT,
    pub GetHandleFromThread: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetObjectSize: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub IsArrayClass: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::WinRT::Metadata::CorElementType, *mut usize, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    IsArrayClass: usize,
    pub GetThreadInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentThreadID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub GetClassIDInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut u32) -> windows_core::HRESULT,
    pub GetFunctionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut usize, *mut u32) -> windows_core::HRESULT,
    pub SetEventMask: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetEnterLeaveFunctionHooks: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionEnter, *const FunctionLeave, *const FunctionTailcall) -> windows_core::HRESULT,
    pub SetFunctionIDMapper: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionIDMapper) -> windows_core::HRESULT,
    pub GetTokenAndMetaDataFromFunction: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetModuleInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut u8, u32, *mut u32, windows_core::PWSTR, *mut usize) -> windows_core::HRESULT,
    pub GetModuleMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetILFunctionBody: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetILFunctionBodyAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetILFunctionBody: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const u8) -> windows_core::HRESULT,
    pub GetAppDomainInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, windows_core::PWSTR, *mut usize) -> windows_core::HRESULT,
    pub GetAssemblyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, windows_core::PWSTR, *mut usize, *mut usize) -> windows_core::HRESULT,
    pub SetFunctionReJIT: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub ForceGC: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetILInstrumentedCodeMap: unsafe extern "system" fn(*mut core::ffi::c_void, usize, super::super::super::Foundation::BOOL, u32, *const COR_IL_MAP) -> windows_core::HRESULT,
    pub GetInprocInspectionInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInprocInspectionIThisThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetThreadContext: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub BeginInprocDebugging: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL, *mut u32) -> windows_core::HRESULT,
    pub EndInprocDebugging: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetILToNativeMapping: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo10, ICorProfilerInfo10_Vtbl, 0x2f1b5152_c869_40c9_aa5f_3abe026bd720);
impl core::ops::Deref for ICorProfilerInfo10 {
    type Target = ICorProfilerInfo9;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo10, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9);
impl ICorProfilerInfo10 {
    pub unsafe fn EnumerateObjectReferences(&self, objectid: usize, callback: ObjectReferenceCallback, clientdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateObjectReferences)(windows_core::Interface::as_raw(self), objectid, callback, clientdata).ok()
    }
    pub unsafe fn IsFrozenObject(&self, objectid: usize, pbfrozen: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsFrozenObject)(windows_core::Interface::as_raw(self), objectid, pbfrozen).ok()
    }
    pub unsafe fn GetLOHObjectSizeThreshold(&self, pthreshold: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLOHObjectSizeThreshold)(windows_core::Interface::as_raw(self), pthreshold).ok()
    }
    pub unsafe fn RequestReJITWithInliners(&self, dwrejitflags: u32, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestReJITWithInliners)(windows_core::Interface::as_raw(self), dwrejitflags, cfunctions, moduleids, methodids).ok()
    }
    pub unsafe fn SuspendRuntime(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuspendRuntime)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeRuntime(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResumeRuntime)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo10_Vtbl {
    pub base__: ICorProfilerInfo9_Vtbl,
    pub EnumerateObjectReferences: unsafe extern "system" fn(*mut core::ffi::c_void, usize, ObjectReferenceCallback, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFrozenObject: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLOHObjectSizeThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RequestReJITWithInliners: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const usize, *const u32) -> windows_core::HRESULT,
    pub SuspendRuntime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeRuntime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo11, ICorProfilerInfo11_Vtbl, 0x06398876_8987_4154_b621_40a00d6e4d04);
impl core::ops::Deref for ICorProfilerInfo11 {
    type Target = ICorProfilerInfo10;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo11, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, ICorProfilerInfo10);
impl ICorProfilerInfo11 {
    pub unsafe fn GetEnvironmentVariableA<P0>(&self, szname: P0, pcchvalue: *mut u32, szvalue: &mut [u16]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetEnvironmentVariableA)(windows_core::Interface::as_raw(self), szname.param().abi(), szvalue.len().try_into().unwrap(), pcchvalue, core::mem::transmute(szvalue.as_ptr())).ok()
    }
    pub unsafe fn SetEnvironmentVariable<P0, P1>(&self, szname: P0, szvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetEnvironmentVariable)(windows_core::Interface::as_raw(self), szname.param().abi(), szvalue.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo11_Vtbl {
    pub base__: ICorProfilerInfo10_Vtbl,
    pub GetEnvironmentVariableA: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetEnvironmentVariable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo12, ICorProfilerInfo12_Vtbl, 0x27b24ccd_1cb1_47c5_96ee_98190dc30959);
impl core::ops::Deref for ICorProfilerInfo12 {
    type Target = ICorProfilerInfo11;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo12, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, ICorProfilerInfo10, ICorProfilerInfo11);
impl ICorProfilerInfo12 {
    pub unsafe fn EventPipeStartSession<P0>(&self, pproviderconfigs: &[COR_PRF_EVENTPIPE_PROVIDER_CONFIG], requestrundown: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventPipeStartSession)(windows_core::Interface::as_raw(self), pproviderconfigs.len().try_into().unwrap(), core::mem::transmute(pproviderconfigs.as_ptr()), requestrundown.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EventPipeAddProviderToSession(&self, session: u64, providerconfig: COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EventPipeAddProviderToSession)(windows_core::Interface::as_raw(self), session, core::mem::transmute(providerconfig)).ok()
    }
    pub unsafe fn EventPipeStopSession(&self, session: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EventPipeStopSession)(windows_core::Interface::as_raw(self), session).ok()
    }
    pub unsafe fn EventPipeCreateProvider<P0>(&self, providername: P0) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventPipeCreateProvider)(windows_core::Interface::as_raw(self), providername.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EventPipeGetProviderInfo(&self, provider: usize, pcchname: *mut u32, providername: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EventPipeGetProviderInfo)(windows_core::Interface::as_raw(self), provider, providername.len().try_into().unwrap(), pcchname, core::mem::transmute(providername.as_ptr())).ok()
    }
    pub unsafe fn EventPipeDefineEvent<P0, P1>(&self, provider: usize, eventname: P0, eventid: u32, keywords: u64, eventversion: u32, level: u32, opcode: u8, needstack: P1, pparamdescs: &[COR_PRF_EVENTPIPE_PARAM_DESC]) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventPipeDefineEvent)(windows_core::Interface::as_raw(self), provider, eventname.param().abi(), eventid, keywords, eventversion, level, opcode, needstack.param().abi(), pparamdescs.len().try_into().unwrap(), core::mem::transmute(pparamdescs.as_ptr()), &mut result__).map(|| result__)
    }
    pub unsafe fn EventPipeWriteEvent(&self, event: usize, data: &[COR_PRF_EVENT_DATA], pactivityid: *const windows_core::GUID, prelatedactivityid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EventPipeWriteEvent)(windows_core::Interface::as_raw(self), event, data.len().try_into().unwrap(), core::mem::transmute(data.as_ptr()), pactivityid, prelatedactivityid).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo12_Vtbl {
    pub base__: ICorProfilerInfo11_Vtbl,
    pub EventPipeStartSession: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const COR_PRF_EVENTPIPE_PROVIDER_CONFIG, super::super::super::Foundation::BOOL, *mut u64) -> windows_core::HRESULT,
    pub EventPipeAddProviderToSession: unsafe extern "system" fn(*mut core::ffi::c_void, u64, COR_PRF_EVENTPIPE_PROVIDER_CONFIG) -> windows_core::HRESULT,
    pub EventPipeStopSession: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub EventPipeCreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut usize) -> windows_core::HRESULT,
    pub EventPipeGetProviderInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub EventPipeDefineEvent: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::PCWSTR, u32, u64, u32, u32, u8, super::super::super::Foundation::BOOL, u32, *const COR_PRF_EVENTPIPE_PARAM_DESC, *mut usize) -> windows_core::HRESULT,
    pub EventPipeWriteEvent: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *const COR_PRF_EVENT_DATA, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo13, ICorProfilerInfo13_Vtbl, 0x6e6c7ee2_0701_4ec2_9d29_2e8733b66934);
impl core::ops::Deref for ICorProfilerInfo13 {
    type Target = ICorProfilerInfo12;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo13, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, ICorProfilerInfo10, ICorProfilerInfo11, ICorProfilerInfo12);
impl ICorProfilerInfo13 {
    pub unsafe fn CreateHandle(&self, object: usize, r#type: COR_PRF_HANDLE_TYPE, phandle: *mut *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateHandle)(windows_core::Interface::as_raw(self), object, r#type, phandle).ok()
    }
    pub unsafe fn DestroyHandle(&self, handle: *const *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DestroyHandle)(windows_core::Interface::as_raw(self), handle).ok()
    }
    pub unsafe fn GetObjectIDFromHandle(&self, handle: *const *const core::ffi::c_void) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectIDFromHandle)(windows_core::Interface::as_raw(self), handle, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICorProfilerInfo13_Vtbl {
    pub base__: ICorProfilerInfo12_Vtbl,
    pub CreateHandle: unsafe extern "system" fn(*mut core::ffi::c_void, usize, COR_PRF_HANDLE_TYPE, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DestroyHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectIDFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo14, ICorProfilerInfo14_Vtbl, 0xf460e352_d76d_4fe9_835f_f6af9d6e862d);
impl core::ops::Deref for ICorProfilerInfo14 {
    type Target = ICorProfilerInfo13;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo14, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8, ICorProfilerInfo9, ICorProfilerInfo10, ICorProfilerInfo11, ICorProfilerInfo12, ICorProfilerInfo13);
impl ICorProfilerInfo14 {
    pub unsafe fn EnumerateNonGCObjects(&self) -> windows_core::Result<ICorProfilerObjectEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateNonGCObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNonGCHeapBounds(&self, pcobjectranges: *mut u32, ranges: &mut [COR_PRF_NONGC_HEAP_RANGE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNonGCHeapBounds)(windows_core::Interface::as_raw(self), ranges.len().try_into().unwrap(), pcobjectranges, core::mem::transmute(ranges.as_ptr())).ok()
    }
    pub unsafe fn EventPipeCreateProvider2<P0>(&self, providername: P0, pcallback: *const EventPipeProviderCallback) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventPipeCreateProvider2)(windows_core::Interface::as_raw(self), providername.param().abi(), pcallback, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICorProfilerInfo14_Vtbl {
    pub base__: ICorProfilerInfo13_Vtbl,
    pub EnumerateNonGCObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNonGCHeapBounds: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut COR_PRF_NONGC_HEAP_RANGE) -> windows_core::HRESULT,
    pub EventPipeCreateProvider2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const EventPipeProviderCallback, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo2, ICorProfilerInfo2_Vtbl, 0xcc0935cd_a518_487d_b0bb_a93214e65478);
impl core::ops::Deref for ICorProfilerInfo2 {
    type Target = ICorProfilerInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo2, windows_core::IUnknown, ICorProfilerInfo);
impl ICorProfilerInfo2 {
    pub unsafe fn DoStackSnapshot(&self, thread: usize, callback: *const StackSnapshotCallback, infoflags: u32, clientdata: *const core::ffi::c_void, context: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoStackSnapshot)(windows_core::Interface::as_raw(self), thread, callback, infoflags, clientdata, core::mem::transmute(context.as_ptr()), context.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks2(&self, pfuncenter: *const FunctionEnter2, pfuncleave: *const FunctionLeave2, pfunctailcall: *const FunctionTailcall2) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEnterLeaveFunctionHooks2)(windows_core::Interface::as_raw(self), pfuncenter, pfuncleave, pfunctailcall).ok()
    }
    pub unsafe fn GetFunctionInfo2(&self, funcid: usize, frameinfo: usize, pclassid: *mut usize, pmoduleid: *mut usize, ptoken: *mut u32, ctypeargs: u32, pctypeargs: *mut u32, typeargs: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFunctionInfo2)(windows_core::Interface::as_raw(self), funcid, frameinfo, pclassid, pmoduleid, ptoken, ctypeargs, pctypeargs, typeargs).ok()
    }
    pub unsafe fn GetStringLayout(&self, pbufferlengthoffset: *mut u32, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStringLayout)(windows_core::Interface::as_raw(self), pbufferlengthoffset, pstringlengthoffset, pbufferoffset).ok()
    }
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub unsafe fn GetClassLayout(&self, classid: usize, rfieldoffset: *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, cfieldoffset: u32, pcfieldoffset: *mut u32, pulclasssize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClassLayout)(windows_core::Interface::as_raw(self), classid, rfieldoffset, cfieldoffset, pcfieldoffset, pulclasssize).ok()
    }
    pub unsafe fn GetClassIDInfo2(&self, classid: usize, pmoduleid: *mut usize, ptypedeftoken: *mut u32, pparentclassid: *mut usize, cnumtypeargs: u32, pcnumtypeargs: *mut u32, typeargs: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClassIDInfo2)(windows_core::Interface::as_raw(self), classid, pmoduleid, ptypedeftoken, pparentclassid, cnumtypeargs, pcnumtypeargs, typeargs).ok()
    }
    pub unsafe fn GetCodeInfo2(&self, functionid: usize, pccodeinfos: *mut u32, codeinfos: &mut [COR_PRF_CODE_INFO]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodeInfo2)(windows_core::Interface::as_raw(self), functionid, codeinfos.len().try_into().unwrap(), pccodeinfos, core::mem::transmute(codeinfos.as_ptr())).ok()
    }
    pub unsafe fn GetClassFromTokenAndTypeArgs(&self, moduleid: usize, typedef: u32, typeargs: &[usize]) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassFromTokenAndTypeArgs)(windows_core::Interface::as_raw(self), moduleid, typedef, typeargs.len().try_into().unwrap(), core::mem::transmute(typeargs.as_ptr()), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFunctionFromTokenAndTypeArgs(&self, moduleid: usize, funcdef: u32, classid: usize, typeargs: &[usize]) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFunctionFromTokenAndTypeArgs)(windows_core::Interface::as_raw(self), moduleid, funcdef, classid, typeargs.len().try_into().unwrap(), core::mem::transmute(typeargs.as_ptr()), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumModuleFrozenObjects(&self, moduleid: usize) -> windows_core::Result<ICorProfilerObjectEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumModuleFrozenObjects)(windows_core::Interface::as_raw(self), moduleid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetArrayObjectInfo(&self, objectid: usize, cdimensions: u32, pdimensionsizes: *mut u32, pdimensionlowerbounds: *mut i32, ppdata: *mut *mut u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetArrayObjectInfo)(windows_core::Interface::as_raw(self), objectid, cdimensions, pdimensionsizes, pdimensionlowerbounds, ppdata).ok()
    }
    pub unsafe fn GetBoxClassLayout(&self, classid: usize) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBoxClassLayout)(windows_core::Interface::as_raw(self), classid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetThreadAppDomain(&self, threadid: usize) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetThreadAppDomain)(windows_core::Interface::as_raw(self), threadid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetRVAStaticAddress(&self, classid: usize, fieldtoken: u32, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRVAStaticAddress)(windows_core::Interface::as_raw(self), classid, fieldtoken, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainStaticAddress(&self, classid: usize, fieldtoken: u32, appdomainid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAppDomainStaticAddress)(windows_core::Interface::as_raw(self), classid, fieldtoken, appdomainid, ppaddress).ok()
    }
    pub unsafe fn GetThreadStaticAddress(&self, classid: usize, fieldtoken: u32, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetThreadStaticAddress)(windows_core::Interface::as_raw(self), classid, fieldtoken, threadid, ppaddress).ok()
    }
    pub unsafe fn GetContextStaticAddress(&self, classid: usize, fieldtoken: u32, contextid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetContextStaticAddress)(windows_core::Interface::as_raw(self), classid, fieldtoken, contextid, ppaddress).ok()
    }
    pub unsafe fn GetStaticFieldInfo(&self, classid: usize, fieldtoken: u32) -> windows_core::Result<COR_PRF_STATIC_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStaticFieldInfo)(windows_core::Interface::as_raw(self), classid, fieldtoken, &mut result__).map(|| result__)
    }
    pub unsafe fn GetGenerationBounds(&self, pcobjectranges: *mut u32, ranges: &mut [COR_PRF_GC_GENERATION_RANGE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGenerationBounds)(windows_core::Interface::as_raw(self), ranges.len().try_into().unwrap(), pcobjectranges, core::mem::transmute(ranges.as_ptr())).ok()
    }
    pub unsafe fn GetObjectGeneration(&self, objectid: usize) -> windows_core::Result<COR_PRF_GC_GENERATION_RANGE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectGeneration)(windows_core::Interface::as_raw(self), objectid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetNotifiedExceptionClauseInfo(&self) -> windows_core::Result<COR_PRF_EX_CLAUSE_INFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNotifiedExceptionClauseInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICorProfilerInfo2_Vtbl {
    pub base__: ICorProfilerInfo_Vtbl,
    pub DoStackSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *const StackSnapshotCallback, u32, *const core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub SetEnterLeaveFunctionHooks2: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionEnter2, *const FunctionLeave2, *const FunctionTailcall2) -> windows_core::HRESULT,
    pub GetFunctionInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize, *mut usize, *mut u32, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetStringLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_WinRT_Metadata")]
    pub GetClassLayout: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::WinRT::Metadata::COR_FIELD_OFFSET, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_WinRT_Metadata"))]
    GetClassLayout: usize,
    pub GetClassIDInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut u32, *mut usize, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetCodeInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT,
    pub GetClassFromTokenAndTypeArgs: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, u32, *const usize, *mut usize) -> windows_core::HRESULT,
    pub GetFunctionFromTokenAndTypeArgs: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, u32, *const usize, *mut usize) -> windows_core::HRESULT,
    pub EnumModuleFrozenObjects: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetArrayObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut i32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetBoxClassLayout: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    pub GetThreadAppDomain: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
    pub GetRVAStaticAddress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppDomainStaticAddress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetThreadStaticAddress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContextStaticAddress: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStaticFieldInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut COR_PRF_STATIC_TYPE) -> windows_core::HRESULT,
    pub GetGenerationBounds: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::HRESULT,
    pub GetObjectGeneration: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut COR_PRF_GC_GENERATION_RANGE) -> windows_core::HRESULT,
    pub GetNotifiedExceptionClauseInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut COR_PRF_EX_CLAUSE_INFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo3, ICorProfilerInfo3_Vtbl, 0xb555ed4f_452a_4e54_8b39_b5360bad32a0);
impl core::ops::Deref for ICorProfilerInfo3 {
    type Target = ICorProfilerInfo2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo3, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2);
impl ICorProfilerInfo3 {
    pub unsafe fn EnumJITedFunctions(&self) -> windows_core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumJITedFunctions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestProfilerDetach(&self, dwexpectedcompletionmilliseconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestProfilerDetach)(windows_core::Interface::as_raw(self), dwexpectedcompletionmilliseconds).ok()
    }
    pub unsafe fn SetFunctionIDMapper2(&self, pfunc: *const FunctionIDMapper2, clientdata: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFunctionIDMapper2)(windows_core::Interface::as_raw(self), pfunc, clientdata).ok()
    }
    pub unsafe fn GetStringLayout2(&self, pstringlengthoffset: *mut u32, pbufferoffset: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStringLayout2)(windows_core::Interface::as_raw(self), pstringlengthoffset, pbufferoffset).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3(&self, pfuncenter3: *const FunctionEnter3, pfuncleave3: *const FunctionLeave3, pfunctailcall3: *const FunctionTailcall3) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEnterLeaveFunctionHooks3)(windows_core::Interface::as_raw(self), pfuncenter3, pfuncleave3, pfunctailcall3).ok()
    }
    pub unsafe fn SetEnterLeaveFunctionHooks3WithInfo(&self, pfuncenter3withinfo: *const FunctionEnter3WithInfo, pfuncleave3withinfo: *const FunctionLeave3WithInfo, pfunctailcall3withinfo: *const FunctionTailcall3WithInfo) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEnterLeaveFunctionHooks3WithInfo)(windows_core::Interface::as_raw(self), pfuncenter3withinfo, pfuncleave3withinfo, pfunctailcall3withinfo).ok()
    }
    pub unsafe fn GetFunctionEnter3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pcbargumentinfo: *mut u32, pargumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFunctionEnter3Info)(windows_core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pcbargumentinfo, pargumentinfo).ok()
    }
    pub unsafe fn GetFunctionLeave3Info(&self, functionid: usize, eltinfo: usize, pframeinfo: *mut usize, pretvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFunctionLeave3Info)(windows_core::Interface::as_raw(self), functionid, eltinfo, pframeinfo, pretvalrange).ok()
    }
    pub unsafe fn GetFunctionTailcall3Info(&self, functionid: usize, eltinfo: usize) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFunctionTailcall3Info)(windows_core::Interface::as_raw(self), functionid, eltinfo, &mut result__).map(|| result__)
    }
    pub unsafe fn EnumModules(&self) -> windows_core::Result<ICorProfilerModuleEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumModules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRuntimeInformation(&self, pclrinstanceid: *mut u16, pruntimetype: *mut COR_PRF_RUNTIME_TYPE, pmajorversion: *mut u16, pminorversion: *mut u16, pbuildnumber: *mut u16, pqfeversion: *mut u16, pcchversionstring: *mut u32, szversionstring: &mut [u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRuntimeInformation)(windows_core::Interface::as_raw(self), pclrinstanceid, pruntimetype, pmajorversion, pminorversion, pbuildnumber, pqfeversion, szversionstring.len().try_into().unwrap(), pcchversionstring, core::mem::transmute(szversionstring.as_ptr())).ok()
    }
    pub unsafe fn GetThreadStaticAddress2(&self, classid: usize, fieldtoken: u32, appdomainid: usize, threadid: usize, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetThreadStaticAddress2)(windows_core::Interface::as_raw(self), classid, fieldtoken, appdomainid, threadid, ppaddress).ok()
    }
    pub unsafe fn GetAppDomainsContainingModule(&self, moduleid: usize, pcappdomainids: *mut u32, appdomainids: &mut [usize]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAppDomainsContainingModule)(windows_core::Interface::as_raw(self), moduleid, appdomainids.len().try_into().unwrap(), pcappdomainids, core::mem::transmute(appdomainids.as_ptr())).ok()
    }
    pub unsafe fn GetModuleInfo2(&self, moduleid: usize, ppbaseloadaddress: *mut *mut u8, pcchname: *mut u32, szname: &mut [u16], passemblyid: *mut usize, pdwmoduleflags: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetModuleInfo2)(windows_core::Interface::as_raw(self), moduleid, ppbaseloadaddress, szname.len().try_into().unwrap(), pcchname, core::mem::transmute(szname.as_ptr()), passemblyid, pdwmoduleflags).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo3_Vtbl {
    pub base__: ICorProfilerInfo2_Vtbl,
    pub EnumJITedFunctions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestProfilerDetach: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetFunctionIDMapper2: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionIDMapper2, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStringLayout2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetEnterLeaveFunctionHooks3: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionEnter3, *const FunctionLeave3, *const FunctionTailcall3) -> windows_core::HRESULT,
    pub SetEnterLeaveFunctionHooks3WithInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const FunctionEnter3WithInfo, *const FunctionLeave3WithInfo, *const FunctionTailcall3WithInfo) -> windows_core::HRESULT,
    pub GetFunctionEnter3Info: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize, *mut u32, *mut COR_PRF_FUNCTION_ARGUMENT_INFO) -> windows_core::HRESULT,
    pub GetFunctionLeave3Info: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize, *mut COR_PRF_FUNCTION_ARGUMENT_RANGE) -> windows_core::HRESULT,
    pub GetFunctionTailcall3Info: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, *mut usize) -> windows_core::HRESULT,
    pub EnumModules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRuntimeInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, *mut COR_PRF_RUNTIME_TYPE, *mut u16, *mut u16, *mut u16, *mut u16, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetThreadStaticAddress2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, usize, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppDomainsContainingModule: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetModuleInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut *mut u8, u32, *mut u32, windows_core::PWSTR, *mut usize, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo4, ICorProfilerInfo4_Vtbl, 0x0d8fdcaa_6257_47bf_b1bf_94dac88466ee);
impl core::ops::Deref for ICorProfilerInfo4 {
    type Target = ICorProfilerInfo3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo4, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3);
impl ICorProfilerInfo4 {
    pub unsafe fn EnumThreads(&self) -> windows_core::Result<ICorProfilerThreadEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumThreads)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InitializeCurrentThread(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeCurrentThread)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestReJIT(&self, cfunctions: u32, moduleids: *const usize, methodids: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestReJIT)(windows_core::Interface::as_raw(self), cfunctions, moduleids, methodids).ok()
    }
    pub unsafe fn RequestRevert(&self, cfunctions: u32, moduleids: *const usize, methodids: *const u32, status: *mut windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestRevert)(windows_core::Interface::as_raw(self), cfunctions, moduleids, methodids, status).ok()
    }
    pub unsafe fn GetCodeInfo3(&self, functionid: usize, rejitid: usize, pccodeinfos: *mut u32, codeinfos: &mut [COR_PRF_CODE_INFO]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodeInfo3)(windows_core::Interface::as_raw(self), functionid, rejitid, codeinfos.len().try_into().unwrap(), pccodeinfos, core::mem::transmute(codeinfos.as_ptr())).ok()
    }
    pub unsafe fn GetFunctionFromIP2(&self, ip: *const u8, pfunctionid: *mut usize, prejitid: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFunctionFromIP2)(windows_core::Interface::as_raw(self), ip, pfunctionid, prejitid).ok()
    }
    pub unsafe fn GetReJITIDs(&self, functionid: usize, pcrejitids: *mut u32, rejitids: &mut [usize]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetReJITIDs)(windows_core::Interface::as_raw(self), functionid, rejitids.len().try_into().unwrap(), pcrejitids, core::mem::transmute(rejitids.as_ptr())).ok()
    }
    pub unsafe fn GetILToNativeMapping2(&self, functionid: usize, rejitid: usize, pcmap: *mut u32, map: &mut [COR_DEBUG_IL_TO_NATIVE_MAP]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetILToNativeMapping2)(windows_core::Interface::as_raw(self), functionid, rejitid, map.len().try_into().unwrap(), pcmap, core::mem::transmute(map.as_ptr())).ok()
    }
    pub unsafe fn EnumJITedFunctions2(&self) -> windows_core::Result<ICorProfilerFunctionEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumJITedFunctions2)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetObjectSize2(&self, objectid: usize) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectSize2)(windows_core::Interface::as_raw(self), objectid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ICorProfilerInfo4_Vtbl {
    pub base__: ICorProfilerInfo3_Vtbl,
    pub EnumThreads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestReJIT: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const u32) -> windows_core::HRESULT,
    pub RequestRevert: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const usize, *const u32, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetCodeInfo3: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *mut u32, *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT,
    pub GetFunctionFromIP2: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut usize, *mut usize) -> windows_core::HRESULT,
    pub GetReJITIDs: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetILToNativeMapping2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *mut u32, *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT,
    pub EnumJITedFunctions2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectSize2: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo5, ICorProfilerInfo5_Vtbl, 0x07602928_ce38_4b83_81e7_74adaf781214);
impl core::ops::Deref for ICorProfilerInfo5 {
    type Target = ICorProfilerInfo4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo5, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4);
impl ICorProfilerInfo5 {
    pub unsafe fn GetEventMask2(&self, pdweventslow: *mut u32, pdweventshigh: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetEventMask2)(windows_core::Interface::as_raw(self), pdweventslow, pdweventshigh).ok()
    }
    pub unsafe fn SetEventMask2(&self, dweventslow: u32, dweventshigh: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventMask2)(windows_core::Interface::as_raw(self), dweventslow, dweventshigh).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo5_Vtbl {
    pub base__: ICorProfilerInfo4_Vtbl,
    pub GetEventMask2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetEventMask2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo6, ICorProfilerInfo6_Vtbl, 0xf30a070d_bffb_46a7_b1d8_8781ef7b698a);
impl core::ops::Deref for ICorProfilerInfo6 {
    type Target = ICorProfilerInfo5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo6, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5);
impl ICorProfilerInfo6 {
    pub unsafe fn EnumNgenModuleMethodsInliningThisMethod(&self, inlinersmoduleid: usize, inlineemoduleid: usize, inlineemethodid: u32, incompletedata: *mut super::super::super::Foundation::BOOL, ppenum: *mut Option<ICorProfilerMethodEnum>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumNgenModuleMethodsInliningThisMethod)(windows_core::Interface::as_raw(self), inlinersmoduleid, inlineemoduleid, inlineemethodid, incompletedata, core::mem::transmute(ppenum)).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo6_Vtbl {
    pub base__: ICorProfilerInfo5_Vtbl,
    pub EnumNgenModuleMethodsInliningThisMethod: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *mut super::super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo7, ICorProfilerInfo7_Vtbl, 0x9aeecc0d_63e0_4187_8c00_e312f503f663);
impl core::ops::Deref for ICorProfilerInfo7 {
    type Target = ICorProfilerInfo6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo7, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6);
impl ICorProfilerInfo7 {
    pub unsafe fn ApplyMetaData(&self, moduleid: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyMetaData)(windows_core::Interface::as_raw(self), moduleid).ok()
    }
    pub unsafe fn GetInMemorySymbolsLength(&self, moduleid: usize) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInMemorySymbolsLength)(windows_core::Interface::as_raw(self), moduleid, &mut result__).map(|| result__)
    }
    pub unsafe fn ReadInMemorySymbols(&self, moduleid: usize, symbolsreadoffset: u32, psymbolbytes: *mut u8, countsymbolbytes: u32, pcountsymbolbytesread: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadInMemorySymbols)(windows_core::Interface::as_raw(self), moduleid, symbolsreadoffset, psymbolbytes, countsymbolbytes, pcountsymbolbytesread).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo7_Vtbl {
    pub base__: ICorProfilerInfo6_Vtbl,
    pub ApplyMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub GetInMemorySymbolsLength: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut u32) -> windows_core::HRESULT,
    pub ReadInMemorySymbols: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo8, ICorProfilerInfo8_Vtbl, 0xc5ac80a6_782e_4716_8044_39598c60cfbf);
impl core::ops::Deref for ICorProfilerInfo8 {
    type Target = ICorProfilerInfo7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo8, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7);
impl ICorProfilerInfo8 {
    pub unsafe fn IsFunctionDynamic(&self, functionid: usize) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFunctionDynamic)(windows_core::Interface::as_raw(self), functionid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetFunctionFromIP3(&self, ip: *const u8, functionid: *mut usize, prejitid: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFunctionFromIP3)(windows_core::Interface::as_raw(self), ip, functionid, prejitid).ok()
    }
    pub unsafe fn GetDynamicFunctionInfo(&self, functionid: usize, moduleid: *mut usize, ppvsig: *mut *mut u8, pbsig: *mut u32, cchname: u32, pcchname: *mut u32, wszname: windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDynamicFunctionInfo)(windows_core::Interface::as_raw(self), functionid, moduleid, ppvsig, pbsig, cchname, pcchname, core::mem::transmute(wszname)).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo8_Vtbl {
    pub base__: ICorProfilerInfo7_Vtbl,
    pub IsFunctionDynamic: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFunctionFromIP3: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, *mut usize, *mut usize) -> windows_core::HRESULT,
    pub GetDynamicFunctionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, usize, *mut usize, *mut *mut u8, *mut u32, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerInfo9, ICorProfilerInfo9_Vtbl, 0x008170db_f8cc_4796_9a51_dc8aa0b47012);
impl core::ops::Deref for ICorProfilerInfo9 {
    type Target = ICorProfilerInfo8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerInfo9, windows_core::IUnknown, ICorProfilerInfo, ICorProfilerInfo2, ICorProfilerInfo3, ICorProfilerInfo4, ICorProfilerInfo5, ICorProfilerInfo6, ICorProfilerInfo7, ICorProfilerInfo8);
impl ICorProfilerInfo9 {
    pub unsafe fn GetNativeCodeStartAddresses(&self, functionid: usize, rejitid: usize, ccodestartaddresses: u32, pccodestartaddresses: *mut u32, codestartaddresses: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNativeCodeStartAddresses)(windows_core::Interface::as_raw(self), functionid, rejitid, ccodestartaddresses, pccodestartaddresses, codestartaddresses).ok()
    }
    pub unsafe fn GetILToNativeMapping3(&self, pnativecodestartaddress: usize, cmap: u32, pcmap: *mut u32, map: *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetILToNativeMapping3)(windows_core::Interface::as_raw(self), pnativecodestartaddress, cmap, pcmap, map).ok()
    }
    pub unsafe fn GetCodeInfo4(&self, pnativecodestartaddress: usize, ccodeinfos: u32, pccodeinfos: *mut u32, codeinfos: *mut COR_PRF_CODE_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCodeInfo4)(windows_core::Interface::as_raw(self), pnativecodestartaddress, ccodeinfos, pccodeinfos, codeinfos).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerInfo9_Vtbl {
    pub base__: ICorProfilerInfo8_Vtbl,
    pub GetNativeCodeStartAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
    pub GetILToNativeMapping3: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut COR_DEBUG_IL_TO_NATIVE_MAP) -> windows_core::HRESULT,
    pub GetCodeInfo4: unsafe extern "system" fn(*mut core::ffi::c_void, usize, u32, *mut u32, *mut COR_PRF_CODE_INFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerMethodEnum, ICorProfilerMethodEnum_Vtbl, 0xfccee788_0088_454b_a811_c99f298d1942);
impl core::ops::Deref for ICorProfilerMethodEnum {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerMethodEnum, windows_core::IUnknown);
impl ICorProfilerMethodEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerMethodEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Next(&self, elements: &mut [COR_PRF_METHOD], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), elements.len().try_into().unwrap(), core::mem::transmute(elements.as_ptr()), pceltfetched).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerMethodEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut COR_PRF_METHOD, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerModuleEnum, ICorProfilerModuleEnum_Vtbl, 0xb0266d75_2081_4493_af7f_028ba34db891);
impl core::ops::Deref for ICorProfilerModuleEnum {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerModuleEnum, windows_core::IUnknown);
impl ICorProfilerModuleEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerModuleEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Next(&self, ids: &mut [usize], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ids.len().try_into().unwrap(), core::mem::transmute(ids.as_ptr()), pceltfetched).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerModuleEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerObjectEnum, ICorProfilerObjectEnum_Vtbl, 0x2c6269bd_2d13_4321_ae12_6686365fd6af);
impl core::ops::Deref for ICorProfilerObjectEnum {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerObjectEnum, windows_core::IUnknown);
impl ICorProfilerObjectEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerObjectEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Next(&self, objects: &mut [usize], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), objects.len().try_into().unwrap(), core::mem::transmute(objects.as_ptr()), pceltfetched).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerObjectEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICorProfilerThreadEnum, ICorProfilerThreadEnum_Vtbl, 0x571194f7_25ed_419f_aa8b_7016b3159701);
impl core::ops::Deref for ICorProfilerThreadEnum {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICorProfilerThreadEnum, windows_core::IUnknown);
impl ICorProfilerThreadEnum {
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<ICorProfilerThreadEnum> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Next(&self, ids: &mut [usize], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ids.len().try_into().unwrap(), core::mem::transmute(ids.as_ptr()), pceltfetched).ok()
    }
}
#[repr(C)]
pub struct ICorProfilerThreadEnum_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMethodMalloc, IMethodMalloc_Vtbl, 0xa0efb28b_6ee2_4d7b_b983_a75ef7beedb8);
impl core::ops::Deref for IMethodMalloc {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMethodMalloc, windows_core::IUnknown);
impl IMethodMalloc {
    pub unsafe fn Alloc(&self, cb: u32) -> *mut core::ffi::c_void {
        (windows_core::Interface::vtable(self).Alloc)(windows_core::Interface::as_raw(self), cb)
    }
}
#[repr(C)]
pub struct IMethodMalloc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> *mut core::ffi::c_void,
}
pub const COR_PRF_ALL: COR_PRF_MONITOR = COR_PRF_MONITOR(-1879048193i32);
pub const COR_PRF_ALLOWABLE_AFTER_ATTACH: COR_PRF_MONITOR = COR_PRF_MONITOR(268763902i32);
pub const COR_PRF_ALLOWABLE_NOTIFICATION_PROFILER: COR_PRF_MONITOR = COR_PRF_MONITOR(-1310512257i32);
pub const COR_PRF_CACHED_FUNCTION_FOUND: COR_PRF_JIT_CACHE = COR_PRF_JIT_CACHE(0i32);
pub const COR_PRF_CACHED_FUNCTION_NOT_FOUND: COR_PRF_JIT_CACHE = COR_PRF_JIT_CACHE(1i32);
pub const COR_PRF_CLAUSE_CATCH: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(2i32);
pub const COR_PRF_CLAUSE_FILTER: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(1i32);
pub const COR_PRF_CLAUSE_FINALLY: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(3i32);
pub const COR_PRF_CLAUSE_NONE: COR_PRF_CLAUSE_TYPE = COR_PRF_CLAUSE_TYPE(0i32);
pub const COR_PRF_CODEGEN_DISABLE_ALL_OPTIMIZATIONS: COR_PRF_CODEGEN_FLAGS = COR_PRF_CODEGEN_FLAGS(2i32);
pub const COR_PRF_CODEGEN_DISABLE_INLINING: COR_PRF_CODEGEN_FLAGS = COR_PRF_CODEGEN_FLAGS(1i32);
pub const COR_PRF_CORE_CLR: COR_PRF_RUNTIME_TYPE = COR_PRF_RUNTIME_TYPE(2i32);
pub const COR_PRF_DESKTOP_CLR: COR_PRF_RUNTIME_TYPE = COR_PRF_RUNTIME_TYPE(1i32);
pub const COR_PRF_DISABLE_ALL_NGEN_IMAGES: COR_PRF_MONITOR = COR_PRF_MONITOR(-2147483648i32);
pub const COR_PRF_DISABLE_INLINING: COR_PRF_MONITOR = COR_PRF_MONITOR(2097152i32);
pub const COR_PRF_DISABLE_OPTIMIZATIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(4194304i32);
pub const COR_PRF_DISABLE_TRANSPARENCY_CHECKS_UNDER_FULL_TRUST: COR_PRF_MONITOR = COR_PRF_MONITOR(1073741824i32);
pub const COR_PRF_ENABLE_FRAME_INFO: COR_PRF_MONITOR = COR_PRF_MONITOR(134217728i32);
pub const COR_PRF_ENABLE_FUNCTION_ARGS: COR_PRF_MONITOR = COR_PRF_MONITOR(33554432i32);
pub const COR_PRF_ENABLE_FUNCTION_RETVAL: COR_PRF_MONITOR = COR_PRF_MONITOR(67108864i32);
pub const COR_PRF_ENABLE_INPROC_DEBUGGING: COR_PRF_MONITOR = COR_PRF_MONITOR(524288i32);
pub const COR_PRF_ENABLE_JIT_MAPS: COR_PRF_MONITOR = COR_PRF_MONITOR(1048576i32);
pub const COR_PRF_ENABLE_OBJECT_ALLOCATED: COR_PRF_MONITOR = COR_PRF_MONITOR(8388608i32);
pub const COR_PRF_ENABLE_REJIT: COR_PRF_MONITOR = COR_PRF_MONITOR(262144i32);
pub const COR_PRF_ENABLE_STACK_SNAPSHOT: COR_PRF_MONITOR = COR_PRF_MONITOR(268435456i32);
pub const COR_PRF_EVENTPIPE_ARRAY: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(19i32);
pub const COR_PRF_EVENTPIPE_BOOLEAN: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(3i32);
pub const COR_PRF_EVENTPIPE_BYTE: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(6i32);
pub const COR_PRF_EVENTPIPE_CHAR: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(4i32);
pub const COR_PRF_EVENTPIPE_CRITICAL: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(1i32);
pub const COR_PRF_EVENTPIPE_DATETIME: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(16i32);
pub const COR_PRF_EVENTPIPE_DECIMAL: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(15i32);
pub const COR_PRF_EVENTPIPE_DOUBLE: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(14i32);
pub const COR_PRF_EVENTPIPE_ERROR: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(2i32);
pub const COR_PRF_EVENTPIPE_GUID: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(17i32);
pub const COR_PRF_EVENTPIPE_INFORMATIONAL: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(4i32);
pub const COR_PRF_EVENTPIPE_INT16: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(7i32);
pub const COR_PRF_EVENTPIPE_INT32: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(9i32);
pub const COR_PRF_EVENTPIPE_INT64: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(11i32);
pub const COR_PRF_EVENTPIPE_LOGALWAYS: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(0i32);
pub const COR_PRF_EVENTPIPE_OBJECT: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(1i32);
pub const COR_PRF_EVENTPIPE_SBYTE: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(5i32);
pub const COR_PRF_EVENTPIPE_SINGLE: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(13i32);
pub const COR_PRF_EVENTPIPE_STRING: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(18i32);
pub const COR_PRF_EVENTPIPE_UINT16: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(8i32);
pub const COR_PRF_EVENTPIPE_UINT32: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(10i32);
pub const COR_PRF_EVENTPIPE_UINT64: COR_PRF_EVENTPIPE_PARAM_TYPE = COR_PRF_EVENTPIPE_PARAM_TYPE(12i32);
pub const COR_PRF_EVENTPIPE_VERBOSE: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(5i32);
pub const COR_PRF_EVENTPIPE_WARNING: COR_PRF_EVENTPIPE_LEVEL = COR_PRF_EVENTPIPE_LEVEL(3i32);
pub const COR_PRF_FIELD_APP_DOMAIN_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(1i32);
pub const COR_PRF_FIELD_CONTEXT_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(4i32);
pub const COR_PRF_FIELD_NOT_A_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(0i32);
pub const COR_PRF_FIELD_RVA_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(8i32);
pub const COR_PRF_FIELD_THREAD_STATIC: COR_PRF_STATIC_TYPE = COR_PRF_STATIC_TYPE(2i32);
pub const COR_PRF_FINALIZER_CRITICAL: COR_PRF_FINALIZER_FLAGS = COR_PRF_FINALIZER_FLAGS(1i32);
pub const COR_PRF_GC_GEN_0: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(0i32);
pub const COR_PRF_GC_GEN_1: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(1i32);
pub const COR_PRF_GC_GEN_2: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(2i32);
pub const COR_PRF_GC_INDUCED: COR_PRF_GC_REASON = COR_PRF_GC_REASON(1i32);
pub const COR_PRF_GC_LARGE_OBJECT_HEAP: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(3i32);
pub const COR_PRF_GC_OTHER: COR_PRF_GC_REASON = COR_PRF_GC_REASON(0i32);
pub const COR_PRF_GC_PINNED_OBJECT_HEAP: COR_PRF_GC_GENERATION = COR_PRF_GC_GENERATION(4i32);
pub const COR_PRF_GC_ROOT_FINALIZER: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(2i32);
pub const COR_PRF_GC_ROOT_HANDLE: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(3i32);
pub const COR_PRF_GC_ROOT_INTERIOR: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(4i32);
pub const COR_PRF_GC_ROOT_OTHER: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(0i32);
pub const COR_PRF_GC_ROOT_PINNING: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(1i32);
pub const COR_PRF_GC_ROOT_REFCOUNTED: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(8i32);
pub const COR_PRF_GC_ROOT_STACK: COR_PRF_GC_ROOT_KIND = COR_PRF_GC_ROOT_KIND(1i32);
pub const COR_PRF_GC_ROOT_WEAKREF: COR_PRF_GC_ROOT_FLAGS = COR_PRF_GC_ROOT_FLAGS(2i32);
pub const COR_PRF_HANDLE_TYPE_PINNED: COR_PRF_HANDLE_TYPE = COR_PRF_HANDLE_TYPE(3i32);
pub const COR_PRF_HANDLE_TYPE_STRONG: COR_PRF_HANDLE_TYPE = COR_PRF_HANDLE_TYPE(2i32);
pub const COR_PRF_HANDLE_TYPE_WEAK: COR_PRF_HANDLE_TYPE = COR_PRF_HANDLE_TYPE(1i32);
pub const COR_PRF_HIGH_ADD_ASSEMBLY_REFERENCES: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(1i32);
pub const COR_PRF_HIGH_ALLOWABLE_AFTER_ATTACH: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(246i32);
pub const COR_PRF_HIGH_ALLOWABLE_NOTIFICATION_PROFILER: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(254i32);
pub const COR_PRF_HIGH_BASIC_GC: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(16i32);
pub const COR_PRF_HIGH_DISABLE_TIERED_COMPILATION: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(8i32);
pub const COR_PRF_HIGH_IN_MEMORY_SYMBOLS_UPDATED: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(2i32);
pub const COR_PRF_HIGH_MONITOR_DYNAMIC_FUNCTION_UNLOADS: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(4i32);
pub const COR_PRF_HIGH_MONITOR_EVENT_PIPE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(128i32);
pub const COR_PRF_HIGH_MONITOR_GC_MOVED_OBJECTS: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(32i32);
pub const COR_PRF_HIGH_MONITOR_IMMUTABLE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(8i32);
pub const COR_PRF_HIGH_MONITOR_LARGEOBJECT_ALLOCATED: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(64i32);
pub const COR_PRF_HIGH_MONITOR_NONE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(0i32);
pub const COR_PRF_HIGH_MONITOR_PINNEDOBJECT_ALLOCATED: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(256i32);
pub const COR_PRF_HIGH_REQUIRE_PROFILE_IMAGE: COR_PRF_HIGH_MONITOR = COR_PRF_HIGH_MONITOR(0i32);
pub const COR_PRF_MODULE_COLLECTIBLE: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(8i32);
pub const COR_PRF_MODULE_DISK: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(1i32);
pub const COR_PRF_MODULE_DYNAMIC: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(4i32);
pub const COR_PRF_MODULE_FLAT_LAYOUT: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(32i32);
pub const COR_PRF_MODULE_NGEN: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(2i32);
pub const COR_PRF_MODULE_RESOURCE: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(16i32);
pub const COR_PRF_MODULE_WINDOWS_RUNTIME: COR_PRF_MODULE_FLAGS = COR_PRF_MODULE_FLAGS(64i32);
pub const COR_PRF_MONITOR_ALL: COR_PRF_MONITOR = COR_PRF_MONITOR(17301503i32);
pub const COR_PRF_MONITOR_APPDOMAIN_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(16i32);
pub const COR_PRF_MONITOR_ASSEMBLY_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(8i32);
pub const COR_PRF_MONITOR_CACHE_SEARCHES: COR_PRF_MONITOR = COR_PRF_MONITOR(131072i32);
pub const COR_PRF_MONITOR_CCW: COR_PRF_MONITOR = COR_PRF_MONITOR(8192i32);
pub const COR_PRF_MONITOR_CLASS_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(2i32);
pub const COR_PRF_MONITOR_CLR_EXCEPTIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(16777216i32);
pub const COR_PRF_MONITOR_CODE_TRANSITIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(2048i32);
pub const COR_PRF_MONITOR_ENTERLEAVE: COR_PRF_MONITOR = COR_PRF_MONITOR(4096i32);
pub const COR_PRF_MONITOR_EXCEPTIONS: COR_PRF_MONITOR = COR_PRF_MONITOR(64i32);
pub const COR_PRF_MONITOR_FUNCTION_UNLOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(1i32);
pub const COR_PRF_MONITOR_GC: COR_PRF_MONITOR = COR_PRF_MONITOR(128i32);
pub const COR_PRF_MONITOR_IMMUTABLE: COR_PRF_MONITOR = COR_PRF_MONITOR(-285684736i32);
pub const COR_PRF_MONITOR_JIT_COMPILATION: COR_PRF_MONITOR = COR_PRF_MONITOR(32i32);
pub const COR_PRF_MONITOR_MODULE_LOADS: COR_PRF_MONITOR = COR_PRF_MONITOR(4i32);
pub const COR_PRF_MONITOR_NONE: COR_PRF_MONITOR = COR_PRF_MONITOR(0i32);
pub const COR_PRF_MONITOR_OBJECT_ALLOCATED: COR_PRF_MONITOR = COR_PRF_MONITOR(256i32);
pub const COR_PRF_MONITOR_REMOTING: COR_PRF_MONITOR = COR_PRF_MONITOR(1024i32);
pub const COR_PRF_MONITOR_REMOTING_ASYNC: COR_PRF_MONITOR = COR_PRF_MONITOR(33792i32);
pub const COR_PRF_MONITOR_REMOTING_COOKIE: COR_PRF_MONITOR = COR_PRF_MONITOR(17408i32);
pub const COR_PRF_MONITOR_SUSPENDS: COR_PRF_MONITOR = COR_PRF_MONITOR(65536i32);
pub const COR_PRF_MONITOR_THREADS: COR_PRF_MONITOR = COR_PRF_MONITOR(512i32);
pub const COR_PRF_REJIT_BLOCK_INLINING: COR_PRF_REJIT_FLAGS = COR_PRF_REJIT_FLAGS(1i32);
pub const COR_PRF_REJIT_INLINING_CALLBACKS: COR_PRF_REJIT_FLAGS = COR_PRF_REJIT_FLAGS(2i32);
pub const COR_PRF_REQUIRE_PROFILE_IMAGE: COR_PRF_MONITOR = COR_PRF_MONITOR(536877056i32);
pub const COR_PRF_SNAPSHOT_DEFAULT: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(0i32);
pub const COR_PRF_SNAPSHOT_REGISTER_CONTEXT: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(1i32);
pub const COR_PRF_SNAPSHOT_X86_OPTIMIZED: COR_PRF_SNAPSHOT_INFO = COR_PRF_SNAPSHOT_INFO(2i32);
pub const COR_PRF_SUSPEND_FOR_APPDOMAIN_SHUTDOWN: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(2i32);
pub const COR_PRF_SUSPEND_FOR_CODE_PITCHING: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(3i32);
pub const COR_PRF_SUSPEND_FOR_GC: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(1i32);
pub const COR_PRF_SUSPEND_FOR_GC_PREP: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(7i32);
pub const COR_PRF_SUSPEND_FOR_INPROC_DEBUGGER: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(6i32);
pub const COR_PRF_SUSPEND_FOR_PROFILER: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(9i32);
pub const COR_PRF_SUSPEND_FOR_REJIT: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(8i32);
pub const COR_PRF_SUSPEND_FOR_SHUTDOWN: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(4i32);
pub const COR_PRF_SUSPEND_OTHER: COR_PRF_SUSPEND_REASON = COR_PRF_SUSPEND_REASON(0i32);
pub const COR_PRF_TRANSITION_CALL: COR_PRF_TRANSITION_REASON = COR_PRF_TRANSITION_REASON(0i32);
pub const COR_PRF_TRANSITION_RETURN: COR_PRF_TRANSITION_REASON = COR_PRF_TRANSITION_REASON(1i32);
pub const COR_PRF_USE_PROFILE_IMAGES: COR_PRF_MONITOR = COR_PRF_MONITOR(536870912i32);
pub const EPILOG: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-3i32);
pub const NO_MAPPING: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-1i32);
pub const PROFILER_GLOBAL_CLASS: COR_PRF_MISC = COR_PRF_MISC(-2i32);
pub const PROFILER_GLOBAL_MODULE: COR_PRF_MISC = COR_PRF_MISC(-1i32);
pub const PROFILER_PARENT_UNKNOWN: COR_PRF_MISC = COR_PRF_MISC(-3i32);
pub const PROLOG: CorDebugIlToNativeMappingTypes = CorDebugIlToNativeMappingTypes(-2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_CLAUSE_TYPE(pub i32);
impl windows_core::TypeKind for COR_PRF_CLAUSE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_CLAUSE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_CLAUSE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_CODEGEN_FLAGS(pub i32);
impl windows_core::TypeKind for COR_PRF_CODEGEN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_CODEGEN_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_CODEGEN_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_EVENTPIPE_LEVEL(pub i32);
impl windows_core::TypeKind for COR_PRF_EVENTPIPE_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_EVENTPIPE_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_EVENTPIPE_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_EVENTPIPE_PARAM_TYPE(pub i32);
impl windows_core::TypeKind for COR_PRF_EVENTPIPE_PARAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_EVENTPIPE_PARAM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_EVENTPIPE_PARAM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_FINALIZER_FLAGS(pub i32);
impl windows_core::TypeKind for COR_PRF_FINALIZER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_FINALIZER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_FINALIZER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_GC_GENERATION(pub i32);
impl windows_core::TypeKind for COR_PRF_GC_GENERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_GC_GENERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_GC_GENERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_GC_REASON(pub i32);
impl windows_core::TypeKind for COR_PRF_GC_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_GC_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_GC_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_GC_ROOT_FLAGS(pub i32);
impl windows_core::TypeKind for COR_PRF_GC_ROOT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_GC_ROOT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_GC_ROOT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_GC_ROOT_KIND(pub i32);
impl windows_core::TypeKind for COR_PRF_GC_ROOT_KIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_GC_ROOT_KIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_GC_ROOT_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_HANDLE_TYPE(pub i32);
impl windows_core::TypeKind for COR_PRF_HANDLE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_HANDLE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_HANDLE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_HIGH_MONITOR(pub i32);
impl windows_core::TypeKind for COR_PRF_HIGH_MONITOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_HIGH_MONITOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_HIGH_MONITOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_JIT_CACHE(pub i32);
impl windows_core::TypeKind for COR_PRF_JIT_CACHE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_JIT_CACHE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_JIT_CACHE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_MISC(pub i32);
impl windows_core::TypeKind for COR_PRF_MISC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_MISC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_MISC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_MODULE_FLAGS(pub i32);
impl windows_core::TypeKind for COR_PRF_MODULE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_MODULE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_MODULE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_MONITOR(pub i32);
impl windows_core::TypeKind for COR_PRF_MONITOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_MONITOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_MONITOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_REJIT_FLAGS(pub i32);
impl windows_core::TypeKind for COR_PRF_REJIT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_REJIT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_REJIT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_RUNTIME_TYPE(pub i32);
impl windows_core::TypeKind for COR_PRF_RUNTIME_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_RUNTIME_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_RUNTIME_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_SNAPSHOT_INFO(pub i32);
impl windows_core::TypeKind for COR_PRF_SNAPSHOT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_SNAPSHOT_INFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_SNAPSHOT_INFO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_STATIC_TYPE(pub i32);
impl windows_core::TypeKind for COR_PRF_STATIC_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_STATIC_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_STATIC_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_SUSPEND_REASON(pub i32);
impl windows_core::TypeKind for COR_PRF_SUSPEND_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_SUSPEND_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_SUSPEND_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COR_PRF_TRANSITION_REASON(pub i32);
impl windows_core::TypeKind for COR_PRF_TRANSITION_REASON {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COR_PRF_TRANSITION_REASON {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COR_PRF_TRANSITION_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CorDebugIlToNativeMappingTypes(pub i32);
impl windows_core::TypeKind for CorDebugIlToNativeMappingTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CorDebugIlToNativeMappingTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CorDebugIlToNativeMappingTypes").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_DEBUG_IL_TO_NATIVE_MAP {
    pub ilOffset: u32,
    pub nativeStartOffset: u32,
    pub nativeEndOffset: u32,
}
impl windows_core::TypeKind for COR_DEBUG_IL_TO_NATIVE_MAP {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_DEBUG_IL_TO_NATIVE_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_IL_MAP {
    pub oldOffset: u32,
    pub newOffset: u32,
    pub fAccurate: super::super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for COR_IL_MAP {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_IL_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_WinRT_Metadata")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_ASSEMBLY_REFERENCE_INFO {
    pub pbPublicKeyOrToken: *mut core::ffi::c_void,
    pub cbPublicKeyOrToken: u32,
    pub szName: windows_core::PCWSTR,
    pub pMetaData: *mut super::super::WinRT::Metadata::ASSEMBLYMETADATA,
    pub pbHashValue: *mut core::ffi::c_void,
    pub cbHashValue: u32,
    pub dwAssemblyRefFlags: u32,
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl windows_core::TypeKind for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_WinRT_Metadata")]
impl Default for COR_PRF_ASSEMBLY_REFERENCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_CODE_INFO {
    pub startAddress: usize,
    pub size: usize,
}
impl windows_core::TypeKind for COR_PRF_CODE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_CODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_EVENTPIPE_PARAM_DESC {
    pub r#type: u32,
    pub elementType: u32,
    pub name: windows_core::PCWSTR,
}
impl windows_core::TypeKind for COR_PRF_EVENTPIPE_PARAM_DESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_EVENTPIPE_PARAM_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_EVENTPIPE_PROVIDER_CONFIG {
    pub providerName: windows_core::PCWSTR,
    pub keywords: u64,
    pub loggingLevel: u32,
    pub filterData: windows_core::PCWSTR,
}
impl windows_core::TypeKind for COR_PRF_EVENTPIPE_PROVIDER_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_EVENTPIPE_PROVIDER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_EVENT_DATA {
    pub ptr: u64,
    pub size: u32,
    pub reserved: u32,
}
impl windows_core::TypeKind for COR_PRF_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_EX_CLAUSE_INFO {
    pub clauseType: COR_PRF_CLAUSE_TYPE,
    pub programCounter: usize,
    pub framePointer: usize,
    pub shadowStackPointer: usize,
}
impl windows_core::TypeKind for COR_PRF_EX_CLAUSE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_EX_CLAUSE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_FILTER_DATA {
    pub Ptr: u64,
    pub Size: u32,
    pub Type: u32,
}
impl windows_core::TypeKind for COR_PRF_FILTER_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_FILTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_FUNCTION {
    pub functionId: usize,
    pub reJitId: usize,
}
impl windows_core::TypeKind for COR_PRF_FUNCTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_FUNCTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_FUNCTION_ARGUMENT_INFO {
    pub numRanges: u32,
    pub totalArgumentSize: u32,
    pub ranges: [COR_PRF_FUNCTION_ARGUMENT_RANGE; 1],
}
impl windows_core::TypeKind for COR_PRF_FUNCTION_ARGUMENT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_FUNCTION_ARGUMENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_FUNCTION_ARGUMENT_RANGE {
    pub startAddress: usize,
    pub length: u32,
}
impl windows_core::TypeKind for COR_PRF_FUNCTION_ARGUMENT_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_FUNCTION_ARGUMENT_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_GC_GENERATION_RANGE {
    pub generation: COR_PRF_GC_GENERATION,
    pub rangeStart: usize,
    pub rangeLength: usize,
    pub rangeLengthReserved: usize,
}
impl windows_core::TypeKind for COR_PRF_GC_GENERATION_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_GC_GENERATION_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_METHOD {
    pub moduleId: usize,
    pub methodId: u32,
}
impl windows_core::TypeKind for COR_PRF_METHOD {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_METHOD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COR_PRF_NONGC_HEAP_RANGE {
    pub rangeStart: usize,
    pub rangeLength: usize,
    pub rangeLengthReserved: usize,
}
impl windows_core::TypeKind for COR_PRF_NONGC_HEAP_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for COR_PRF_NONGC_HEAP_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FunctionIDOrClientID {
    pub functionID: usize,
    pub clientID: usize,
}
impl windows_core::TypeKind for FunctionIDOrClientID {
    type TypeKind = windows_core::CopyType;
}
impl Default for FunctionIDOrClientID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EventPipeProviderCallback = Option<unsafe extern "system" fn(source_id: *const u8, is_enabled: u32, level: u8, match_any_keywords: u64, match_all_keywords: u64, filter_data: *mut COR_PRF_FILTER_DATA, callback_data: *mut core::ffi::c_void)>;
pub type FunctionEnter = Option<unsafe extern "system" fn(funcid: usize)>;
pub type FunctionEnter2 = Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize, argumentinfo: *mut COR_PRF_FUNCTION_ARGUMENT_INFO)>;
pub type FunctionEnter3 = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID)>;
pub type FunctionEnter3WithInfo = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize)>;
pub type FunctionIDMapper = Option<unsafe extern "system" fn(funcid: usize, pbhookfunction: *mut super::super::super::Foundation::BOOL) -> usize>;
pub type FunctionIDMapper2 = Option<unsafe extern "system" fn(funcid: usize, clientdata: *mut core::ffi::c_void, pbhookfunction: *mut super::super::super::Foundation::BOOL) -> usize>;
pub type FunctionLeave = Option<unsafe extern "system" fn(funcid: usize)>;
pub type FunctionLeave2 = Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize, retvalrange: *mut COR_PRF_FUNCTION_ARGUMENT_RANGE)>;
pub type FunctionLeave3 = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID)>;
pub type FunctionLeave3WithInfo = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize)>;
pub type FunctionTailcall = Option<unsafe extern "system" fn(funcid: usize)>;
pub type FunctionTailcall2 = Option<unsafe extern "system" fn(funcid: usize, clientdata: usize, func: usize)>;
pub type FunctionTailcall3 = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID)>;
pub type FunctionTailcall3WithInfo = Option<unsafe extern "system" fn(functionidorclientid: FunctionIDOrClientID, eltinfo: usize)>;
pub type ObjectReferenceCallback = Option<unsafe extern "system" fn(root: usize, reference: *mut usize, clientdata: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
pub type StackSnapshotCallback = Option<unsafe extern "system" fn(funcid: usize, ip: usize, frameinfo: usize, contextsize: u32, context: *mut u8, clientdata: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
