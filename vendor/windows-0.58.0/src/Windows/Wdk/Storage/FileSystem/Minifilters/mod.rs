#[inline]
pub unsafe fn FltAcknowledgeEcp<P0>(filter: P0, ecpcontext: *const core::ffi::c_void)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAcknowledgeEcp(filter : PFLT_FILTER, ecpcontext : *const core::ffi::c_void));
    FltAcknowledgeEcp(filter.param().abi(), ecpcontext)
}
#[inline]
pub unsafe fn FltAcquirePushLockExclusive(pushlock: *mut usize) {
    windows_targets::link!("fltmgr.sys" "system" fn FltAcquirePushLockExclusive(pushlock : *mut usize));
    FltAcquirePushLockExclusive(pushlock)
}
#[inline]
pub unsafe fn FltAcquirePushLockExclusiveEx(pushlock: *mut usize, flags: u32) {
    windows_targets::link!("fltmgr.sys" "system" fn FltAcquirePushLockExclusiveEx(pushlock : *mut usize, flags : u32));
    FltAcquirePushLockExclusiveEx(pushlock, flags)
}
#[inline]
pub unsafe fn FltAcquirePushLockShared(pushlock: *mut usize) {
    windows_targets::link!("fltmgr.sys" "system" fn FltAcquirePushLockShared(pushlock : *mut usize));
    FltAcquirePushLockShared(pushlock)
}
#[inline]
pub unsafe fn FltAcquirePushLockSharedEx(pushlock: *mut usize, flags: u32) {
    windows_targets::link!("fltmgr.sys" "system" fn FltAcquirePushLockSharedEx(pushlock : *mut usize, flags : u32));
    FltAcquirePushLockSharedEx(pushlock, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FltAcquireResourceExclusive(resource: *mut super::super::super::Foundation::ERESOURCE) {
    windows_targets::link!("fltmgr.sys" "system" fn FltAcquireResourceExclusive(resource : *mut super::super::super::Foundation:: ERESOURCE));
    FltAcquireResourceExclusive(resource)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FltAcquireResourceShared(resource: *mut super::super::super::Foundation::ERESOURCE) {
    windows_targets::link!("fltmgr.sys" "system" fn FltAcquireResourceShared(resource : *mut super::super::super::Foundation:: ERESOURCE));
    FltAcquireResourceShared(resource)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltAddOpenReparseEntry<P0>(filter: P0, data: *const FLT_CALLBACK_DATA, openreparseentry: *const super::OPEN_REPARSE_LIST_ENTRY) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAddOpenReparseEntry(filter : PFLT_FILTER, data : *const FLT_CALLBACK_DATA, openreparseentry : *const super:: OPEN_REPARSE_LIST_ENTRY) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAddOpenReparseEntry(filter.param().abi(), data, openreparseentry)
}
#[inline]
pub unsafe fn FltAdjustDeviceStackSizeForIoRedirection<P0, P1>(sourceinstance: P0, targetinstance: P1, sourcedevicestacksizemodified: Option<*mut super::super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAdjustDeviceStackSizeForIoRedirection(sourceinstance : PFLT_INSTANCE, targetinstance : PFLT_INSTANCE, sourcedevicestacksizemodified : *mut super::super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAdjustDeviceStackSizeForIoRedirection(sourceinstance.param().abi(), targetinstance.param().abi(), core::mem::transmute(sourcedevicestacksizemodified.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltAllocateCallbackData<P0>(instance: P0, fileobject: Option<*const super::super::super::Foundation::FILE_OBJECT>, retnewcallbackdata: *mut *mut FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateCallbackData(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, retnewcallbackdata : *mut *mut FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAllocateCallbackData(instance.param().abi(), core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), retnewcallbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltAllocateCallbackDataEx<P0>(instance: P0, fileobject: Option<*const super::super::super::Foundation::FILE_OBJECT>, flags: u32, retnewcallbackdata: *mut *mut FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateCallbackDataEx(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, flags : u32, retnewcallbackdata : *mut *mut FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAllocateCallbackDataEx(instance.param().abi(), core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), flags, retnewcallbackdata)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltAllocateContext<P0>(filter: P0, contexttype: u16, contextsize: usize, pooltype: super::super::super::Foundation::POOL_TYPE, returnedcontext: *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateContext(filter : PFLT_FILTER, contexttype : u16, contextsize : usize, pooltype : super::super::super::Foundation:: POOL_TYPE, returnedcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAllocateContext(filter.param().abi(), contexttype, contextsize, pooltype, returnedcontext)
}
#[inline]
pub unsafe fn FltAllocateDeferredIoWorkItem() -> PFLT_DEFERRED_IO_WORKITEM {
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateDeferredIoWorkItem() -> PFLT_DEFERRED_IO_WORKITEM);
    FltAllocateDeferredIoWorkItem()
}
#[inline]
pub unsafe fn FltAllocateExtraCreateParameter<P0>(filter: P0, ecptype: *const windows_core::GUID, sizeofcontext: u32, flags: u32, cleanupcallback: super::PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, pooltag: u32, ecpcontext: *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateExtraCreateParameter(filter : PFLT_FILTER, ecptype : *const windows_core::GUID, sizeofcontext : u32, flags : u32, cleanupcallback : super:: PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, pooltag : u32, ecpcontext : *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAllocateExtraCreateParameter(filter.param().abi(), ecptype, sizeofcontext, flags, cleanupcallback, pooltag, ecpcontext)
}
#[inline]
pub unsafe fn FltAllocateExtraCreateParameterFromLookasideList<P0>(filter: P0, ecptype: *const windows_core::GUID, sizeofcontext: u32, flags: u32, cleanupcallback: super::PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, lookasidelist: *mut core::ffi::c_void, ecpcontext: *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateExtraCreateParameterFromLookasideList(filter : PFLT_FILTER, ecptype : *const windows_core::GUID, sizeofcontext : u32, flags : u32, cleanupcallback : super:: PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, lookasidelist : *mut core::ffi::c_void, ecpcontext : *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAllocateExtraCreateParameterFromLookasideList(filter.param().abi(), ecptype, sizeofcontext, flags, cleanupcallback, lookasidelist, ecpcontext)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltAllocateExtraCreateParameterList<P0>(filter: P0, flags: u32, ecplist: *mut *mut super::super::super::Foundation::ECP_LIST) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateExtraCreateParameterList(filter : PFLT_FILTER, flags : u32, ecplist : *mut *mut super::super::super::Foundation:: ECP_LIST) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAllocateExtraCreateParameterList(filter.param().abi(), flags, ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltAllocateFileLock(completelockcallbackdataroutine: PFLT_COMPLETE_LOCK_CALLBACK_DATA_ROUTINE, unlockroutine: super::PUNLOCK_ROUTINE) -> *mut super::FILE_LOCK {
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateFileLock(completelockcallbackdataroutine : PFLT_COMPLETE_LOCK_CALLBACK_DATA_ROUTINE, unlockroutine : super:: PUNLOCK_ROUTINE) -> *mut super:: FILE_LOCK);
    FltAllocateFileLock(completelockcallbackdataroutine, unlockroutine)
}
#[inline]
pub unsafe fn FltAllocateGenericWorkItem() -> PFLT_GENERIC_WORKITEM {
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocateGenericWorkItem() -> PFLT_GENERIC_WORKITEM);
    FltAllocateGenericWorkItem()
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltAllocatePoolAlignedWithTag<P0>(instance: P0, pooltype: super::super::super::Foundation::POOL_TYPE, numberofbytes: usize, tag: u32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAllocatePoolAlignedWithTag(instance : PFLT_INSTANCE, pooltype : super::super::super::Foundation:: POOL_TYPE, numberofbytes : usize, tag : u32) -> *mut core::ffi::c_void);
    FltAllocatePoolAlignedWithTag(instance.param().abi(), pooltype, numberofbytes, tag)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltApplyPriorityInfoThread<P0>(inputpriorityinfo: *const super::IO_PRIORITY_INFO, outputpriorityinfo: Option<*mut super::IO_PRIORITY_INFO>, thread: P0) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltApplyPriorityInfoThread(inputpriorityinfo : *const super:: IO_PRIORITY_INFO, outputpriorityinfo : *mut super:: IO_PRIORITY_INFO, thread : super::super::super::Foundation:: PETHREAD) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltApplyPriorityInfoThread(inputpriorityinfo, core::mem::transmute(outputpriorityinfo.unwrap_or(std::ptr::null_mut())), thread.param().abi())
}
#[inline]
pub unsafe fn FltAttachVolume<P0, P1>(filter: P0, volume: P1, instancename: Option<*const super::super::super::super::Win32::Foundation::UNICODE_STRING>, retinstance: Option<*mut PFLT_INSTANCE>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAttachVolume(filter : PFLT_FILTER, volume : PFLT_VOLUME, instancename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, retinstance : *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAttachVolume(filter.param().abi(), volume.param().abi(), core::mem::transmute(instancename.unwrap_or(std::ptr::null())), core::mem::transmute(retinstance.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltAttachVolumeAtAltitude<P0, P1>(filter: P0, volume: P1, altitude: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, instancename: Option<*const super::super::super::super::Win32::Foundation::UNICODE_STRING>, retinstance: Option<*mut PFLT_INSTANCE>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltAttachVolumeAtAltitude(filter : PFLT_FILTER, volume : PFLT_VOLUME, altitude : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, instancename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, retinstance : *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltAttachVolumeAtAltitude(filter.param().abi(), volume.param().abi(), altitude, core::mem::transmute(instancename.unwrap_or(std::ptr::null())), core::mem::transmute(retinstance.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FltBuildDefaultSecurityDescriptor(securitydescriptor: *mut super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, desiredaccess: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltBuildDefaultSecurityDescriptor(securitydescriptor : *mut super::super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, desiredaccess : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltBuildDefaultSecurityDescriptor(securitydescriptor, desiredaccess)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCancelFileOpen<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT)
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCancelFileOpen(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT));
    FltCancelFileOpen(instance.param().abi(), fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCancelIo(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltCancelIo(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltCancelIo(callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCancellableWaitForMultipleObjects(objectarray: &[*const core::ffi::c_void], waittype: super::super::super::super::Win32::System::Kernel::WAIT_TYPE, timeout: Option<*const i64>, waitblockarray: Option<*const super::super::super::Foundation::KWAIT_BLOCK>, callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltCancellableWaitForMultipleObjects(count : u32, objectarray : *const *const core::ffi::c_void, waittype : super::super::super::super::Win32::System::Kernel:: WAIT_TYPE, timeout : *const i64, waitblockarray : *const super::super::super::Foundation:: KWAIT_BLOCK, callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCancellableWaitForMultipleObjects(objectarray.len().try_into().unwrap(), core::mem::transmute(objectarray.as_ptr()), waittype, core::mem::transmute(timeout.unwrap_or(std::ptr::null())), core::mem::transmute(waitblockarray.unwrap_or(std::ptr::null())), callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCancellableWaitForSingleObject(object: *const core::ffi::c_void, timeout: Option<*const i64>, callbackdata: Option<*const FLT_CALLBACK_DATA>) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltCancellableWaitForSingleObject(object : *const core::ffi::c_void, timeout : *const i64, callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCancellableWaitForSingleObject(object, core::mem::transmute(timeout.unwrap_or(std::ptr::null())), core::mem::transmute(callbackdata.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqDisable(cbdq: *mut FLT_CALLBACK_DATA_QUEUE) {
    windows_targets::link!("fltmgr.sys" "system" fn FltCbdqDisable(cbdq : *mut FLT_CALLBACK_DATA_QUEUE));
    FltCbdqDisable(cbdq)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqEnable(cbdq: *mut FLT_CALLBACK_DATA_QUEUE) {
    windows_targets::link!("fltmgr.sys" "system" fn FltCbdqEnable(cbdq : *mut FLT_CALLBACK_DATA_QUEUE));
    FltCbdqEnable(cbdq)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqInitialize<P0>(instance: P0, cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbdqinsertio: PFLT_CALLBACK_DATA_QUEUE_INSERT_IO, cbdqremoveio: PFLT_CALLBACK_DATA_QUEUE_REMOVE_IO, cbdqpeeknextio: PFLT_CALLBACK_DATA_QUEUE_PEEK_NEXT_IO, cbdqacquire: PFLT_CALLBACK_DATA_QUEUE_ACQUIRE, cbdqrelease: PFLT_CALLBACK_DATA_QUEUE_RELEASE, cbdqcompletecanceledio: PFLT_CALLBACK_DATA_QUEUE_COMPLETE_CANCELED_IO) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCbdqInitialize(instance : PFLT_INSTANCE, cbdq : *mut FLT_CALLBACK_DATA_QUEUE, cbdqinsertio : PFLT_CALLBACK_DATA_QUEUE_INSERT_IO, cbdqremoveio : PFLT_CALLBACK_DATA_QUEUE_REMOVE_IO, cbdqpeeknextio : PFLT_CALLBACK_DATA_QUEUE_PEEK_NEXT_IO, cbdqacquire : PFLT_CALLBACK_DATA_QUEUE_ACQUIRE, cbdqrelease : PFLT_CALLBACK_DATA_QUEUE_RELEASE, cbdqcompletecanceledio : PFLT_CALLBACK_DATA_QUEUE_COMPLETE_CANCELED_IO) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCbdqInitialize(instance.param().abi(), cbdq, cbdqinsertio, cbdqremoveio, cbdqpeeknextio, cbdqacquire, cbdqrelease, cbdqcompletecanceledio)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqInsertIo(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *const FLT_CALLBACK_DATA, context: Option<*const super::super::super::System::SystemServices::IO_CSQ_IRP_CONTEXT>, insertcontext: Option<*const core::ffi::c_void>) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltCbdqInsertIo(cbdq : *mut FLT_CALLBACK_DATA_QUEUE, cbd : *const FLT_CALLBACK_DATA, context : *const super::super::super::System::SystemServices:: IO_CSQ_IRP_CONTEXT, insertcontext : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCbdqInsertIo(cbdq, cbd, core::mem::transmute(context.unwrap_or(std::ptr::null())), core::mem::transmute(insertcontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqRemoveIo(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, context: *const super::super::super::System::SystemServices::IO_CSQ_IRP_CONTEXT) -> *mut FLT_CALLBACK_DATA {
    windows_targets::link!("fltmgr.sys" "system" fn FltCbdqRemoveIo(cbdq : *mut FLT_CALLBACK_DATA_QUEUE, context : *const super::super::super::System::SystemServices:: IO_CSQ_IRP_CONTEXT) -> *mut FLT_CALLBACK_DATA);
    FltCbdqRemoveIo(cbdq, context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqRemoveNextIo(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, peekcontext: Option<*const core::ffi::c_void>) -> *mut FLT_CALLBACK_DATA {
    windows_targets::link!("fltmgr.sys" "system" fn FltCbdqRemoveNextIo(cbdq : *mut FLT_CALLBACK_DATA_QUEUE, peekcontext : *const core::ffi::c_void) -> *mut FLT_CALLBACK_DATA);
    FltCbdqRemoveNextIo(cbdq, core::mem::transmute(peekcontext.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn FltCheckAndGrowNameControl(namectrl: *mut FLT_NAME_CONTROL, newsize: u16) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltCheckAndGrowNameControl(namectrl : *mut FLT_NAME_CONTROL, newsize : u16) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCheckAndGrowNameControl(namectrl, newsize)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCheckLockForReadAccess(filelock: *const super::FILE_LOCK, callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltCheckLockForReadAccess(filelock : *const super:: FILE_LOCK, callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltCheckLockForReadAccess(filelock, callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCheckLockForWriteAccess(filelock: *const super::FILE_LOCK, callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltCheckLockForWriteAccess(filelock : *const super:: FILE_LOCK, callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltCheckLockForWriteAccess(filelock, callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCheckOplock(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, context: Option<*const core::ffi::c_void>, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltCheckOplock(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, context : *const core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    FltCheckOplock(oplock, callbackdata, core::mem::transmute(context.unwrap_or(std::ptr::null())), waitcompletionroutine, prepostcallbackdataroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCheckOplockEx(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, flags: u32, context: Option<*const core::ffi::c_void>, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltCheckOplockEx(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, flags : u32, context : *const core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    FltCheckOplockEx(oplock, callbackdata, flags, core::mem::transmute(context.unwrap_or(std::ptr::null())), waitcompletionroutine, prepostcallbackdataroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltClearCallbackDataDirty(data: *mut FLT_CALLBACK_DATA) {
    windows_targets::link!("fltmgr.sys" "system" fn FltClearCallbackDataDirty(data : *mut FLT_CALLBACK_DATA));
    FltClearCallbackDataDirty(data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltClearCancelCompletion(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltClearCancelCompletion(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltClearCancelCompletion(callbackdata)
}
#[inline]
pub unsafe fn FltClose<P0>(filehandle: P0) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::super::Win32::Foundation::HANDLE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltClose(filehandle : super::super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltClose(filehandle.param().abi())
}
#[inline]
pub unsafe fn FltCloseClientPort<P0>(filter: P0, clientport: *mut PFLT_PORT)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCloseClientPort(filter : PFLT_FILTER, clientport : *mut PFLT_PORT));
    FltCloseClientPort(filter.param().abi(), clientport)
}
#[inline]
pub unsafe fn FltCloseCommunicationPort<P0>(serverport: P0)
where
    P0: windows_core::Param<PFLT_PORT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCloseCommunicationPort(serverport : PFLT_PORT));
    FltCloseCommunicationPort(serverport.param().abi())
}
#[inline]
pub unsafe fn FltCloseSectionForDataScan<P0>(sectioncontext: P0) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCloseSectionForDataScan(sectioncontext : PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCloseSectionForDataScan(sectioncontext.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltCommitComplete<P0, P1>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: P1) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCommitComplete(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCommitComplete(instance.param().abi(), transaction, transactioncontext.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltCommitFinalizeComplete<P0, P1>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: P1) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCommitFinalizeComplete(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCommitFinalizeComplete(instance.param().abi(), transaction, transactioncontext.param().abi())
}
#[inline]
pub unsafe fn FltCompareInstanceAltitudes<P0, P1>(instance1: P0, instance2: P1) -> i32
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCompareInstanceAltitudes(instance1 : PFLT_INSTANCE, instance2 : PFLT_INSTANCE) -> i32);
    FltCompareInstanceAltitudes(instance1.param().abi(), instance2.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCompletePendedPostOperation(callbackdata: *const FLT_CALLBACK_DATA) {
    windows_targets::link!("fltmgr.sys" "system" fn FltCompletePendedPostOperation(callbackdata : *const FLT_CALLBACK_DATA));
    FltCompletePendedPostOperation(callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCompletePendedPreOperation(callbackdata: *const FLT_CALLBACK_DATA, callbackstatus: FLT_PREOP_CALLBACK_STATUS, context: Option<*const core::ffi::c_void>) {
    windows_targets::link!("fltmgr.sys" "system" fn FltCompletePendedPreOperation(callbackdata : *const FLT_CALLBACK_DATA, callbackstatus : FLT_PREOP_CALLBACK_STATUS, context : *const core::ffi::c_void));
    FltCompletePendedPreOperation(callbackdata, callbackstatus, core::mem::transmute(context.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCopyOpenReparseList<P0>(filter: P0, data: *const FLT_CALLBACK_DATA, ecplist: *mut super::super::super::Foundation::ECP_LIST) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCopyOpenReparseList(filter : PFLT_FILTER, data : *const FLT_CALLBACK_DATA, ecplist : *mut super::super::super::Foundation:: ECP_LIST) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCopyOpenReparseList(filter.param().abi(), data, ecplist)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltCreateCommunicationPort<P0>(filter: P0, serverport: *mut PFLT_PORT, objectattributes: *const super::super::super::Foundation::OBJECT_ATTRIBUTES, serverportcookie: Option<*const core::ffi::c_void>, connectnotifycallback: PFLT_CONNECT_NOTIFY, disconnectnotifycallback: PFLT_DISCONNECT_NOTIFY, messagenotifycallback: PFLT_MESSAGE_NOTIFY, maxconnections: i32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCreateCommunicationPort(filter : PFLT_FILTER, serverport : *mut PFLT_PORT, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, serverportcookie : *const core::ffi::c_void, connectnotifycallback : PFLT_CONNECT_NOTIFY, disconnectnotifycallback : PFLT_DISCONNECT_NOTIFY, messagenotifycallback : PFLT_MESSAGE_NOTIFY, maxconnections : i32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCreateCommunicationPort(filter.param().abi(), serverport, objectattributes, core::mem::transmute(serverportcookie.unwrap_or(std::ptr::null())), connectnotifycallback, disconnectnotifycallback, messagenotifycallback, maxconnections)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FltCreateFile<P0, P1>(filter: P0, instance: P1, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, allocationsize: Option<*const i64>, fileattributes: u32, shareaccess: u32, createdisposition: u32, createoptions: u32, eabuffer: Option<*const core::ffi::c_void>, ealength: u32, flags: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCreateFile(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32, flags : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCreateFile(filter.param().abi(), instance.param().abi(), filehandle, desiredaccess, objectattributes, iostatusblock, core::mem::transmute(allocationsize.unwrap_or(std::ptr::null())), fileattributes, shareaccess, createdisposition, createoptions, core::mem::transmute(eabuffer.unwrap_or(std::ptr::null())), ealength, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateFileEx<P0, P1>(filter: P0, instance: P1, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, fileobject: Option<*mut *mut super::super::super::Foundation::FILE_OBJECT>, desiredaccess: u32, objectattributes: *const super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, allocationsize: Option<*const i64>, fileattributes: u32, shareaccess: u32, createdisposition: u32, createoptions: u32, eabuffer: Option<*const core::ffi::c_void>, ealength: u32, flags: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCreateFileEx(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, fileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32, flags : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCreateFileEx(filter.param().abi(), instance.param().abi(), filehandle, core::mem::transmute(fileobject.unwrap_or(std::ptr::null_mut())), desiredaccess, objectattributes, iostatusblock, core::mem::transmute(allocationsize.unwrap_or(std::ptr::null())), fileattributes, shareaccess, createdisposition, createoptions, core::mem::transmute(eabuffer.unwrap_or(std::ptr::null())), ealength, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateFileEx2<P0, P1>(filter: P0, instance: P1, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, fileobject: Option<*mut *mut super::super::super::Foundation::FILE_OBJECT>, desiredaccess: u32, objectattributes: *const super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, allocationsize: Option<*const i64>, fileattributes: u32, shareaccess: u32, createdisposition: u32, createoptions: u32, eabuffer: Option<*const core::ffi::c_void>, ealength: u32, flags: u32, drivercontext: Option<*const super::super::super::System::SystemServices::IO_DRIVER_CREATE_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCreateFileEx2(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, fileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32, flags : u32, drivercontext : *const super::super::super::System::SystemServices:: IO_DRIVER_CREATE_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCreateFileEx2(filter.param().abi(), instance.param().abi(), filehandle, core::mem::transmute(fileobject.unwrap_or(std::ptr::null_mut())), desiredaccess, objectattributes, iostatusblock, core::mem::transmute(allocationsize.unwrap_or(std::ptr::null())), fileattributes, shareaccess, createdisposition, createoptions, core::mem::transmute(eabuffer.unwrap_or(std::ptr::null())), ealength, flags, core::mem::transmute(drivercontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateMailslotFile<P0, P1>(filter: P0, instance: P1, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, fileobject: Option<*mut *mut super::super::super::Foundation::FILE_OBJECT>, desiredaccess: u32, objectattributes: *const super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, createoptions: u32, mailslotquota: u32, maximummessagesize: u32, readtimeout: *const i64, drivercontext: Option<*const super::super::super::System::SystemServices::IO_DRIVER_CREATE_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCreateMailslotFile(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, fileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, createoptions : u32, mailslotquota : u32, maximummessagesize : u32, readtimeout : *const i64, drivercontext : *const super::super::super::System::SystemServices:: IO_DRIVER_CREATE_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCreateMailslotFile(filter.param().abi(), instance.param().abi(), filehandle, core::mem::transmute(fileobject.unwrap_or(std::ptr::null_mut())), desiredaccess, objectattributes, iostatusblock, createoptions, mailslotquota, maximummessagesize, readtimeout, core::mem::transmute(drivercontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateNamedPipeFile<P0, P1>(filter: P0, instance: P1, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, fileobject: Option<*mut *mut super::super::super::Foundation::FILE_OBJECT>, desiredaccess: u32, objectattributes: *const super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, shareaccess: u32, createdisposition: u32, createoptions: u32, namedpipetype: u32, readmode: u32, completionmode: u32, maximuminstances: u32, inboundquota: u32, outboundquota: u32, defaulttimeout: Option<*const i64>, drivercontext: Option<*const super::super::super::System::SystemServices::IO_DRIVER_CREATE_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCreateNamedPipeFile(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, fileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, shareaccess : u32, createdisposition : u32, createoptions : u32, namedpipetype : u32, readmode : u32, completionmode : u32, maximuminstances : u32, inboundquota : u32, outboundquota : u32, defaulttimeout : *const i64, drivercontext : *const super::super::super::System::SystemServices:: IO_DRIVER_CREATE_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCreateNamedPipeFile(filter.param().abi(), instance.param().abi(), filehandle, core::mem::transmute(fileobject.unwrap_or(std::ptr::null_mut())), desiredaccess, objectattributes, iostatusblock, shareaccess, createdisposition, createoptions, namedpipetype, readmode, completionmode, maximuminstances, inboundquota, outboundquota, core::mem::transmute(defaulttimeout.unwrap_or(std::ptr::null())), core::mem::transmute(drivercontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateSectionForDataScan<P0, P1>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, sectioncontext: P1, desiredaccess: u32, objectattributes: Option<*const super::super::super::Foundation::OBJECT_ATTRIBUTES>, maximumsize: Option<*const i64>, sectionpageprotection: u32, allocationattributes: u32, flags: u32, sectionhandle: *mut super::super::super::super::Win32::Foundation::HANDLE, sectionobject: *mut *mut core::ffi::c_void, sectionfilesize: Option<*mut i64>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCreateSectionForDataScan(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, sectioncontext : PFLT_CONTEXT, desiredaccess : u32, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, flags : u32, sectionhandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, sectionobject : *mut *mut core::ffi::c_void, sectionfilesize : *mut i64) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCreateSectionForDataScan(instance.param().abi(), fileobject, sectioncontext.param().abi(), desiredaccess, core::mem::transmute(objectattributes.unwrap_or(std::ptr::null())), core::mem::transmute(maximumsize.unwrap_or(std::ptr::null())), sectionpageprotection, allocationattributes, flags, sectionhandle, sectionobject, core::mem::transmute(sectionfilesize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltCreateSystemVolumeInformationFolder<P0>(instance: P0) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltCreateSystemVolumeInformationFolder(instance : PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltCreateSystemVolumeInformationFolder(instance.param().abi())
}
#[inline]
pub unsafe fn FltCurrentBatchOplock(oplock: *const *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltCurrentBatchOplock(oplock : *const *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltCurrentBatchOplock(oplock)
}
#[inline]
pub unsafe fn FltCurrentOplock(oplock: *const *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltCurrentOplock(oplock : *const *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltCurrentOplock(oplock)
}
#[inline]
pub unsafe fn FltCurrentOplockH(oplock: *const *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltCurrentOplockH(oplock : *const *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltCurrentOplockH(oplock)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDecodeParameters(callbackdata: *const FLT_CALLBACK_DATA, mdladdresspointer: Option<*mut *mut *mut super::super::super::Foundation::MDL>, buffer: Option<*mut *mut *mut core::ffi::c_void>, length: Option<*mut *mut u32>, desiredaccess: Option<*mut super::super::super::System::SystemServices::LOCK_OPERATION>) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltDecodeParameters(callbackdata : *const FLT_CALLBACK_DATA, mdladdresspointer : *mut *mut *mut super::super::super::Foundation:: MDL, buffer : *mut *mut *mut core::ffi::c_void, length : *mut *mut u32, desiredaccess : *mut super::super::super::System::SystemServices:: LOCK_OPERATION) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDecodeParameters(callbackdata, core::mem::transmute(mdladdresspointer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(length.unwrap_or(std::ptr::null_mut())), core::mem::transmute(desiredaccess.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltDeleteContext<P0>(context: P0)
where
    P0: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeleteContext(context : PFLT_CONTEXT));
    FltDeleteContext(context.param().abi())
}
#[inline]
pub unsafe fn FltDeleteExtraCreateParameterLookasideList<P0>(filter: P0, lookaside: *mut core::ffi::c_void, flags: u32)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeleteExtraCreateParameterLookasideList(filter : PFLT_FILTER, lookaside : *mut core::ffi::c_void, flags : u32));
    FltDeleteExtraCreateParameterLookasideList(filter.param().abi(), lookaside, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDeleteFileContext<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeleteFileContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDeleteFileContext(instance.param().abi(), fileobject, core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltDeleteInstanceContext<P0>(instance: P0, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeleteInstanceContext(instance : PFLT_INSTANCE, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDeleteInstanceContext(instance.param().abi(), core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltDeletePushLock(pushlock: *const usize) {
    windows_targets::link!("fltmgr.sys" "system" fn FltDeletePushLock(pushlock : *const usize));
    FltDeletePushLock(pushlock)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDeleteStreamContext<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeleteStreamContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDeleteStreamContext(instance.param().abi(), fileobject, core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDeleteStreamHandleContext<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeleteStreamHandleContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDeleteStreamHandleContext(instance.param().abi(), fileobject, core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltDeleteTransactionContext<P0>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeleteTransactionContext(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDeleteTransactionContext(instance.param().abi(), transaction, core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltDeleteVolumeContext<P0, P1>(filter: P0, volume: P1, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeleteVolumeContext(filter : PFLT_FILTER, volume : PFLT_VOLUME, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDeleteVolumeContext(filter.param().abi(), volume.param().abi(), core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltDetachVolume<P0, P1>(filter: P0, volume: P1, instancename: Option<*const super::super::super::super::Win32::Foundation::UNICODE_STRING>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDetachVolume(filter : PFLT_FILTER, volume : PFLT_VOLUME, instancename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDetachVolume(filter.param().abi(), volume.param().abi(), core::mem::transmute(instancename.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDeviceIoControlFile<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, iocontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32, lengthreturned: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltDeviceIoControlFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, iocontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltDeviceIoControlFile(instance.param().abi(), fileobject, iocontrolcode, core::mem::transmute(inputbuffer.unwrap_or(std::ptr::null())), inputbufferlength, core::mem::transmute(outputbuffer.unwrap_or(std::ptr::null_mut())), outputbufferlength, core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDoCompletionProcessingWhenSafe(data: *const FLT_CALLBACK_DATA, fltobjects: *const FLT_RELATED_OBJECTS, completioncontext: Option<*const core::ffi::c_void>, flags: u32, safepostcallback: PFLT_POST_OPERATION_CALLBACK, retpostoperationstatus: *mut FLT_POSTOP_CALLBACK_STATUS) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltDoCompletionProcessingWhenSafe(data : *const FLT_CALLBACK_DATA, fltobjects : *const FLT_RELATED_OBJECTS, completioncontext : *const core::ffi::c_void, flags : u32, safepostcallback : PFLT_POST_OPERATION_CALLBACK, retpostoperationstatus : *mut FLT_POSTOP_CALLBACK_STATUS) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltDoCompletionProcessingWhenSafe(data, fltobjects, core::mem::transmute(completioncontext.unwrap_or(std::ptr::null())), flags, safepostcallback, retpostoperationstatus)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltEnlistInTransaction<P0, P1>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: P1, notificationmask: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltEnlistInTransaction(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT, notificationmask : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnlistInTransaction(instance.param().abi(), transaction, transactioncontext.param().abi(), notificationmask)
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateFilterInformation(index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::FILTER_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateFilterInformation(index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: FILTER_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateFilterInformation(index, informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[inline]
pub unsafe fn FltEnumerateFilters(filterlist: Option<&mut [PFLT_FILTER]>, numberfiltersreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateFilters(filterlist : *mut PFLT_FILTER, filterlistsize : u32, numberfiltersreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateFilters(core::mem::transmute(filterlist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), filterlist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), numberfiltersreturned)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltEnumerateInstanceInformationByDeviceObject(deviceobject: *const super::super::super::Foundation::DEVICE_OBJECT, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateInstanceInformationByDeviceObject(deviceobject : *const super::super::super::Foundation:: DEVICE_OBJECT, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateInstanceInformationByDeviceObject(deviceobject, index, informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateInstanceInformationByFilter<P0>(filter: P0, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateInstanceInformationByFilter(filter : PFLT_FILTER, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateInstanceInformationByFilter(filter.param().abi(), index, informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateInstanceInformationByVolume<P0>(volume: P0, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateInstanceInformationByVolume(volume : PFLT_VOLUME, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateInstanceInformationByVolume(volume.param().abi(), index, informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateInstanceInformationByVolumeName(volumename: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateInstanceInformationByVolumeName(volumename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateInstanceInformationByVolumeName(volumename, index, informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[inline]
pub unsafe fn FltEnumerateInstances<P0, P1>(volume: P0, filter: P1, instancelist: Option<&mut [PFLT_INSTANCE]>, numberinstancesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
    P1: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateInstances(volume : PFLT_VOLUME, filter : PFLT_FILTER, instancelist : *mut PFLT_INSTANCE, instancelistsize : u32, numberinstancesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateInstances(volume.param().abi(), filter.param().abi(), core::mem::transmute(instancelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), instancelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), numberinstancesreturned)
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateVolumeInformation<P0>(filter: P0, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::FILTER_VOLUME_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateVolumeInformation(filter : PFLT_FILTER, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: FILTER_VOLUME_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateVolumeInformation(filter.param().abi(), index, informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[inline]
pub unsafe fn FltEnumerateVolumes<P0>(filter: P0, volumelist: Option<&mut [PFLT_VOLUME]>, numbervolumesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltEnumerateVolumes(filter : PFLT_FILTER, volumelist : *mut PFLT_VOLUME, volumelistsize : u32, numbervolumesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltEnumerateVolumes(filter.param().abi(), core::mem::transmute(volumelist.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), volumelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), numbervolumesreturned)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFastIoMdlRead<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut super::super::super::Foundation::MDL, iostatus: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFastIoMdlRead(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::super::Foundation:: MDL, iostatus : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltFastIoMdlRead(initiatinginstance.param().abi(), fileobject, fileoffset, length, lockkey, mdlchain, iostatus)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFastIoMdlReadComplete<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, mdlchain: *const super::super::super::Foundation::MDL) -> super::super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFastIoMdlReadComplete(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, mdlchain : *const super::super::super::Foundation:: MDL) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltFastIoMdlReadComplete(initiatinginstance.param().abi(), fileobject, mdlchain)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFastIoMdlWriteComplete<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, mdlchain: *const super::super::super::Foundation::MDL) -> super::super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFastIoMdlWriteComplete(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, mdlchain : *const super::super::super::Foundation:: MDL) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltFastIoMdlWriteComplete(initiatinginstance.param().abi(), fileobject, fileoffset, mdlchain)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFastIoPrepareMdlWrite<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut super::super::super::Foundation::MDL, iostatus: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> super::super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFastIoPrepareMdlWrite(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::super::Foundation:: MDL, iostatus : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltFastIoPrepareMdlWrite(initiatinginstance.param().abi(), fileobject, fileoffset, length, lockkey, mdlchain, iostatus)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltFindExtraCreateParameter<P0>(filter: P0, ecplist: *const super::super::super::Foundation::ECP_LIST, ecptype: *const windows_core::GUID, ecpcontext: Option<*mut *mut core::ffi::c_void>, ecpcontextsize: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFindExtraCreateParameter(filter : PFLT_FILTER, ecplist : *const super::super::super::Foundation:: ECP_LIST, ecptype : *const windows_core::GUID, ecpcontext : *mut *mut core::ffi::c_void, ecpcontextsize : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltFindExtraCreateParameter(filter.param().abi(), ecplist, ecptype, core::mem::transmute(ecpcontext.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ecpcontextsize.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFlushBuffers<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFlushBuffers(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltFlushBuffers(instance.param().abi(), fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFlushBuffers2<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, flushtype: u32, callbackdata: Option<*const FLT_CALLBACK_DATA>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFlushBuffers2(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, flushtype : u32, callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltFlushBuffers2(instance.param().abi(), fileobject, flushtype, core::mem::transmute(callbackdata.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFreeCallbackData(callbackdata: *const FLT_CALLBACK_DATA) {
    windows_targets::link!("fltmgr.sys" "system" fn FltFreeCallbackData(callbackdata : *const FLT_CALLBACK_DATA));
    FltFreeCallbackData(callbackdata)
}
#[inline]
pub unsafe fn FltFreeDeferredIoWorkItem<P0>(fltworkitem: P0)
where
    P0: windows_core::Param<PFLT_DEFERRED_IO_WORKITEM>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFreeDeferredIoWorkItem(fltworkitem : PFLT_DEFERRED_IO_WORKITEM));
    FltFreeDeferredIoWorkItem(fltworkitem.param().abi())
}
#[inline]
pub unsafe fn FltFreeExtraCreateParameter<P0>(filter: P0, ecpcontext: *const core::ffi::c_void)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFreeExtraCreateParameter(filter : PFLT_FILTER, ecpcontext : *const core::ffi::c_void));
    FltFreeExtraCreateParameter(filter.param().abi(), ecpcontext)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltFreeExtraCreateParameterList<P0>(filter: P0, ecplist: *const super::super::super::Foundation::ECP_LIST)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFreeExtraCreateParameterList(filter : PFLT_FILTER, ecplist : *const super::super::super::Foundation:: ECP_LIST));
    FltFreeExtraCreateParameterList(filter.param().abi(), ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFreeFileLock(filelock: *const super::FILE_LOCK) {
    windows_targets::link!("fltmgr.sys" "system" fn FltFreeFileLock(filelock : *const super:: FILE_LOCK));
    FltFreeFileLock(filelock)
}
#[inline]
pub unsafe fn FltFreeGenericWorkItem<P0>(fltworkitem: P0)
where
    P0: windows_core::Param<PFLT_GENERIC_WORKITEM>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFreeGenericWorkItem(fltworkitem : PFLT_GENERIC_WORKITEM));
    FltFreeGenericWorkItem(fltworkitem.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltFreeOpenReparseList<P0>(filter: P0, ecplist: *const super::super::super::Foundation::ECP_LIST)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFreeOpenReparseList(filter : PFLT_FILTER, ecplist : *const super::super::super::Foundation:: ECP_LIST));
    FltFreeOpenReparseList(filter.param().abi(), ecplist)
}
#[inline]
pub unsafe fn FltFreePoolAlignedWithTag<P0>(instance: P0, buffer: *const core::ffi::c_void, tag: u32)
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFreePoolAlignedWithTag(instance : PFLT_INSTANCE, buffer : *const core::ffi::c_void, tag : u32));
    FltFreePoolAlignedWithTag(instance.param().abi(), buffer, tag)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FltFreeSecurityDescriptor<P0>(securitydescriptor: P0)
where
    P0: windows_core::Param<super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFreeSecurityDescriptor(securitydescriptor : super::super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR));
    FltFreeSecurityDescriptor(securitydescriptor.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFsControlFile<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fscontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32, lengthreturned: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltFsControlFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltFsControlFile(instance.param().abi(), fileobject, fscontrolcode, core::mem::transmute(inputbuffer.unwrap_or(std::ptr::null())), inputbufferlength, core::mem::transmute(outputbuffer.unwrap_or(std::ptr::null_mut())), outputbufferlength, core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetActivityIdCallbackData(callbackdata: *const FLT_CALLBACK_DATA, guid: *mut windows_core::GUID) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetActivityIdCallbackData(callbackdata : *const FLT_CALLBACK_DATA, guid : *mut windows_core::GUID) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetActivityIdCallbackData(callbackdata, guid)
}
#[inline]
pub unsafe fn FltGetBottomInstance<P0>(volume: P0, instance: *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetBottomInstance(volume : PFLT_VOLUME, instance : *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetBottomInstance(volume.param().abi(), instance)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetContexts(fltobjects: *const FLT_RELATED_OBJECTS, desiredcontexts: u16, contexts: *mut FLT_RELATED_CONTEXTS) {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetContexts(fltobjects : *const FLT_RELATED_OBJECTS, desiredcontexts : u16, contexts : *mut FLT_RELATED_CONTEXTS));
    FltGetContexts(fltobjects, desiredcontexts, contexts)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetContextsEx(fltobjects: *const FLT_RELATED_OBJECTS, desiredcontexts: u16, contextssize: usize, contexts: *mut FLT_RELATED_CONTEXTS_EX) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetContextsEx(fltobjects : *const FLT_RELATED_OBJECTS, desiredcontexts : u16, contextssize : usize, contexts : *mut FLT_RELATED_CONTEXTS_EX) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetContextsEx(fltobjects, desiredcontexts, contextssize, contexts)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetDestinationFileNameInformation<P0, P1, P2>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, rootdirectory: P1, filename: P2, filenamelength: u32, nameoptions: u32, retfilenameinformation: *mut *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<super::super::super::super::Win32::Foundation::HANDLE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetDestinationFileNameInformation(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, rootdirectory : super::super::super::super::Win32::Foundation:: HANDLE, filename : windows_core::PCWSTR, filenamelength : u32, nameoptions : u32, retfilenameinformation : *mut *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetDestinationFileNameInformation(instance.param().abi(), fileobject, rootdirectory.param().abi(), filename.param().abi(), filenamelength, nameoptions, retfilenameinformation)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetDeviceObject<P0>(volume: P0, deviceobject: *mut *mut super::super::super::Foundation::DEVICE_OBJECT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetDeviceObject(volume : PFLT_VOLUME, deviceobject : *mut *mut super::super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetDeviceObject(volume.param().abi(), deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetDiskDeviceObject<P0>(volume: P0, diskdeviceobject: *mut *mut super::super::super::Foundation::DEVICE_OBJECT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetDiskDeviceObject(volume : PFLT_VOLUME, diskdeviceobject : *mut *mut super::super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetDiskDeviceObject(volume.param().abi(), diskdeviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetEcpListFromCallbackData<P0>(filter: P0, callbackdata: *const FLT_CALLBACK_DATA, ecplist: *mut *mut super::super::super::Foundation::ECP_LIST) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetEcpListFromCallbackData(filter : PFLT_FILTER, callbackdata : *const FLT_CALLBACK_DATA, ecplist : *mut *mut super::super::super::Foundation:: ECP_LIST) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetEcpListFromCallbackData(filter.param().abi(), callbackdata, ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetFileContext<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, context: *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetFileContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, context : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetFileContext(instance.param().abi(), fileobject, context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetFileNameInformation(callbackdata: *const FLT_CALLBACK_DATA, nameoptions: u32, filenameinformation: *mut *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetFileNameInformation(callbackdata : *const FLT_CALLBACK_DATA, nameoptions : u32, filenameinformation : *mut *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetFileNameInformation(callbackdata, nameoptions, filenameinformation)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetFileNameInformationUnsafe<P0>(fileobject: *const super::super::super::Foundation::FILE_OBJECT, instance: P0, nameoptions: u32, filenameinformation: *mut *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetFileNameInformationUnsafe(fileobject : *const super::super::super::Foundation:: FILE_OBJECT, instance : PFLT_INSTANCE, nameoptions : u32, filenameinformation : *mut *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetFileNameInformationUnsafe(fileobject, instance.param().abi(), nameoptions, filenameinformation)
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltGetFileSystemType(fltobject: *const core::ffi::c_void, filesystemtype: *mut super::super::super::super::Win32::Storage::InstallableFileSystems::FLT_FILESYSTEM_TYPE) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetFileSystemType(fltobject : *const core::ffi::c_void, filesystemtype : *mut super::super::super::super::Win32::Storage::InstallableFileSystems:: FLT_FILESYSTEM_TYPE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetFileSystemType(fltobject, filesystemtype)
}
#[inline]
pub unsafe fn FltGetFilterFromInstance<P0>(instance: P0, retfilter: *mut PFLT_FILTER) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetFilterFromInstance(instance : PFLT_INSTANCE, retfilter : *mut PFLT_FILTER) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetFilterFromInstance(instance.param().abi(), retfilter)
}
#[inline]
pub unsafe fn FltGetFilterFromName(filtername: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, retfilter: *mut PFLT_FILTER) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetFilterFromName(filtername : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, retfilter : *mut PFLT_FILTER) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetFilterFromName(filtername, retfilter)
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltGetFilterInformation<P0>(filter: P0, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::FILTER_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetFilterInformation(filter : PFLT_FILTER, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: FILTER_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetFilterInformation(filter.param().abi(), informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetFsZeroingOffset(data: *const FLT_CALLBACK_DATA, zeroingoffset: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetFsZeroingOffset(data : *const FLT_CALLBACK_DATA, zeroingoffset : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetFsZeroingOffset(data, zeroingoffset)
}
#[inline]
pub unsafe fn FltGetInstanceContext<P0>(instance: P0, context: *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetInstanceContext(instance : PFLT_INSTANCE, context : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetInstanceContext(instance.param().abi(), context)
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltGetInstanceInformation<P0>(instance: P0, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetInstanceInformation(instance : PFLT_INSTANCE, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetInstanceInformation(instance.param().abi(), informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetIoAttributionHandleFromCallbackData(data: *const FLT_CALLBACK_DATA) -> *mut core::ffi::c_void {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetIoAttributionHandleFromCallbackData(data : *const FLT_CALLBACK_DATA) -> *mut core::ffi::c_void);
    FltGetIoAttributionHandleFromCallbackData(data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetIoPriorityHint(data: *const FLT_CALLBACK_DATA) -> super::super::super::Foundation::IO_PRIORITY_HINT {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetIoPriorityHint(data : *const FLT_CALLBACK_DATA) -> super::super::super::Foundation:: IO_PRIORITY_HINT);
    FltGetIoPriorityHint(data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetIoPriorityHintFromCallbackData(data: *const FLT_CALLBACK_DATA) -> super::super::super::Foundation::IO_PRIORITY_HINT {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetIoPriorityHintFromCallbackData(data : *const FLT_CALLBACK_DATA) -> super::super::super::Foundation:: IO_PRIORITY_HINT);
    FltGetIoPriorityHintFromCallbackData(data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetIoPriorityHintFromFileObject(fileobject: *const super::super::super::Foundation::FILE_OBJECT) -> super::super::super::Foundation::IO_PRIORITY_HINT {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetIoPriorityHintFromFileObject(fileobject : *const super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::Foundation:: IO_PRIORITY_HINT);
    FltGetIoPriorityHintFromFileObject(fileobject)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltGetIoPriorityHintFromThread<P0>(thread: P0) -> super::super::super::Foundation::IO_PRIORITY_HINT
where
    P0: windows_core::Param<super::super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetIoPriorityHintFromThread(thread : super::super::super::Foundation:: PETHREAD) -> super::super::super::Foundation:: IO_PRIORITY_HINT);
    FltGetIoPriorityHintFromThread(thread.param().abi())
}
#[inline]
pub unsafe fn FltGetIrpName(irpmajorcode: u8) -> windows_core::PSTR {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetIrpName(irpmajorcode : u8) -> windows_core::PSTR);
    FltGetIrpName(irpmajorcode)
}
#[inline]
pub unsafe fn FltGetLowerInstance<P0>(currentinstance: P0, lowerinstance: *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetLowerInstance(currentinstance : PFLT_INSTANCE, lowerinstance : *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetLowerInstance(currentinstance.param().abi(), lowerinstance)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetNewSystemBufferAddress(callbackdata: *const FLT_CALLBACK_DATA) -> *mut core::ffi::c_void {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetNewSystemBufferAddress(callbackdata : *const FLT_CALLBACK_DATA) -> *mut core::ffi::c_void);
    FltGetNewSystemBufferAddress(callbackdata)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltGetNextExtraCreateParameter<P0>(filter: P0, ecplist: *const super::super::super::Foundation::ECP_LIST, currentecpcontext: Option<*const core::ffi::c_void>, nextecptype: Option<*mut windows_core::GUID>, nextecpcontext: Option<*mut *mut core::ffi::c_void>, nextecpcontextsize: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetNextExtraCreateParameter(filter : PFLT_FILTER, ecplist : *const super::super::super::Foundation:: ECP_LIST, currentecpcontext : *const core::ffi::c_void, nextecptype : *mut windows_core::GUID, nextecpcontext : *mut *mut core::ffi::c_void, nextecpcontextsize : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetNextExtraCreateParameter(filter.param().abi(), ecplist, core::mem::transmute(currentecpcontext.unwrap_or(std::ptr::null())), core::mem::transmute(nextecptype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(nextecpcontext.unwrap_or(std::ptr::null_mut())), core::mem::transmute(nextecpcontextsize.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetRequestorProcess(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::Foundation::PEPROCESS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetRequestorProcess(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::Foundation:: PEPROCESS);
    FltGetRequestorProcess(callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetRequestorProcessId(callbackdata: *const FLT_CALLBACK_DATA) -> u32 {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetRequestorProcessId(callbackdata : *const FLT_CALLBACK_DATA) -> u32);
    FltGetRequestorProcessId(callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetRequestorProcessIdEx(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::HANDLE {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetRequestorProcessIdEx(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: HANDLE);
    FltGetRequestorProcessIdEx(callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetRequestorSessionId(callbackdata: *const FLT_CALLBACK_DATA, sessionid: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetRequestorSessionId(callbackdata : *const FLT_CALLBACK_DATA, sessionid : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetRequestorSessionId(callbackdata, sessionid)
}
#[inline]
pub unsafe fn FltGetRoutineAddress<P0>(fltmgrroutinename: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetRoutineAddress(fltmgrroutinename : windows_core::PCSTR) -> *mut core::ffi::c_void);
    FltGetRoutineAddress(fltmgrroutinename.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetSectionContext<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, context: *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetSectionContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, context : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetSectionContext(instance.param().abi(), fileobject, context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetStreamContext<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, context: *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetStreamContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, context : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetStreamContext(instance.param().abi(), fileobject, context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetStreamHandleContext<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, context: *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetStreamHandleContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, context : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetStreamHandleContext(instance.param().abi(), fileobject, context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetSwappedBufferMdlAddress(callbackdata: *const FLT_CALLBACK_DATA) -> *mut super::super::super::Foundation::MDL {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetSwappedBufferMdlAddress(callbackdata : *const FLT_CALLBACK_DATA) -> *mut super::super::super::Foundation:: MDL);
    FltGetSwappedBufferMdlAddress(callbackdata)
}
#[inline]
pub unsafe fn FltGetTopInstance<P0>(volume: P0, instance: *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetTopInstance(volume : PFLT_VOLUME, instance : *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetTopInstance(volume.param().abi(), instance)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltGetTransactionContext<P0>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, context: *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetTransactionContext(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, context : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetTransactionContext(instance.param().abi(), transaction, context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetTunneledName(callbackdata: *const FLT_CALLBACK_DATA, filenameinformation: *const FLT_FILE_NAME_INFORMATION, rettunneledfilenameinformation: *mut *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltGetTunneledName(callbackdata : *const FLT_CALLBACK_DATA, filenameinformation : *const FLT_FILE_NAME_INFORMATION, rettunneledfilenameinformation : *mut *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetTunneledName(callbackdata, filenameinformation, rettunneledfilenameinformation)
}
#[inline]
pub unsafe fn FltGetUpperInstance<P0>(currentinstance: P0, upperinstance: *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetUpperInstance(currentinstance : PFLT_INSTANCE, upperinstance : *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetUpperInstance(currentinstance.param().abi(), upperinstance)
}
#[inline]
pub unsafe fn FltGetVolumeContext<P0, P1>(filter: P0, volume: P1, context: *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeContext(filter : PFLT_FILTER, volume : PFLT_VOLUME, context : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeContext(filter.param().abi(), volume.param().abi(), context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetVolumeFromDeviceObject<P0>(filter: P0, deviceobject: *const super::super::super::Foundation::DEVICE_OBJECT, retvolume: *mut PFLT_VOLUME) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeFromDeviceObject(filter : PFLT_FILTER, deviceobject : *const super::super::super::Foundation:: DEVICE_OBJECT, retvolume : *mut PFLT_VOLUME) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeFromDeviceObject(filter.param().abi(), deviceobject, retvolume)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetVolumeFromFileObject<P0>(filter: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, retvolume: *mut PFLT_VOLUME) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeFromFileObject(filter : PFLT_FILTER, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, retvolume : *mut PFLT_VOLUME) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeFromFileObject(filter.param().abi(), fileobject, retvolume)
}
#[inline]
pub unsafe fn FltGetVolumeFromInstance<P0>(instance: P0, retvolume: *mut PFLT_VOLUME) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeFromInstance(instance : PFLT_INSTANCE, retvolume : *mut PFLT_VOLUME) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeFromInstance(instance.param().abi(), retvolume)
}
#[inline]
pub unsafe fn FltGetVolumeFromName<P0>(filter: P0, volumename: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, retvolume: *mut PFLT_VOLUME) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeFromName(filter : PFLT_FILTER, volumename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, retvolume : *mut PFLT_VOLUME) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeFromName(filter.param().abi(), volumename, retvolume)
}
#[inline]
pub unsafe fn FltGetVolumeGuidName<P0>(volume: P0, volumeguidname: Option<*mut super::super::super::super::Win32::Foundation::UNICODE_STRING>, buffersizeneeded: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeGuidName(volume : PFLT_VOLUME, volumeguidname : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, buffersizeneeded : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeGuidName(volume.param().abi(), core::mem::transmute(volumeguidname.unwrap_or(std::ptr::null_mut())), core::mem::transmute(buffersizeneeded.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltGetVolumeInformation<P0>(volume: P0, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::FILTER_VOLUME_INFORMATION_CLASS, buffer: Option<*mut core::ffi::c_void>, buffersize: u32, bytesreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeInformation(volume : PFLT_VOLUME, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: FILTER_VOLUME_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeInformation(volume.param().abi(), informationclass, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), buffersize, bytesreturned)
}
#[inline]
pub unsafe fn FltGetVolumeInstanceFromName<P0, P1>(filter: P0, volume: P1, instancename: Option<*const super::super::super::super::Win32::Foundation::UNICODE_STRING>, retinstance: *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeInstanceFromName(filter : PFLT_FILTER, volume : PFLT_VOLUME, instancename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, retinstance : *mut PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeInstanceFromName(filter.param().abi(), volume.param().abi(), core::mem::transmute(instancename.unwrap_or(std::ptr::null())), retinstance)
}
#[inline]
pub unsafe fn FltGetVolumeName<P0>(volume: P0, volumename: Option<*mut super::super::super::super::Win32::Foundation::UNICODE_STRING>, buffersizeneeded: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeName(volume : PFLT_VOLUME, volumename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, buffersizeneeded : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeName(volume.param().abi(), core::mem::transmute(volumename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(buffersizeneeded.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltGetVolumeProperties<P0>(volume: P0, volumeproperties: Option<*mut FLT_VOLUME_PROPERTIES>, volumepropertieslength: u32, lengthreturned: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltGetVolumeProperties(volume : PFLT_VOLUME, volumeproperties : *mut FLT_VOLUME_PROPERTIES, volumepropertieslength : u32, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltGetVolumeProperties(volume.param().abi(), core::mem::transmute(volumeproperties.unwrap_or(std::ptr::null_mut())), volumepropertieslength, lengthreturned)
}
#[inline]
pub unsafe fn FltInitExtraCreateParameterLookasideList<P0>(filter: P0, lookaside: *mut core::ffi::c_void, flags: u32, size: usize, tag: u32)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltInitExtraCreateParameterLookasideList(filter : PFLT_FILTER, lookaside : *mut core::ffi::c_void, flags : u32, size : usize, tag : u32));
    FltInitExtraCreateParameterLookasideList(filter.param().abi(), lookaside, flags, size, tag)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltInitializeFileLock(filelock: *mut super::FILE_LOCK) {
    windows_targets::link!("fltmgr.sys" "system" fn FltInitializeFileLock(filelock : *mut super:: FILE_LOCK));
    FltInitializeFileLock(filelock)
}
#[inline]
pub unsafe fn FltInitializeOplock(oplock: *mut *mut core::ffi::c_void) {
    windows_targets::link!("fltmgr.sys" "system" fn FltInitializeOplock(oplock : *mut *mut core::ffi::c_void));
    FltInitializeOplock(oplock)
}
#[inline]
pub unsafe fn FltInitializePushLock() -> usize {
    windows_targets::link!("fltmgr.sys" "system" fn FltInitializePushLock(pushlock : *mut usize));
    let mut result__ = core::mem::zeroed();
    FltInitializePushLock(&mut result__);
    result__
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltInsertExtraCreateParameter<P0>(filter: P0, ecplist: *mut super::super::super::Foundation::ECP_LIST, ecpcontext: *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltInsertExtraCreateParameter(filter : PFLT_FILTER, ecplist : *mut super::super::super::Foundation:: ECP_LIST, ecpcontext : *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltInsertExtraCreateParameter(filter.param().abi(), ecplist, ecpcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIs32bitProcess(callbackdata: Option<*const FLT_CALLBACK_DATA>) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltIs32bitProcess(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltIs32bitProcess(core::mem::transmute(callbackdata.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsCallbackDataDirty(data: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltIsCallbackDataDirty(data : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltIsCallbackDataDirty(data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsDirectory<P0>(fileobject: *const super::super::super::Foundation::FILE_OBJECT, instance: P0, isdirectory: *mut super::super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltIsDirectory(fileobject : *const super::super::super::Foundation:: FILE_OBJECT, instance : PFLT_INSTANCE, isdirectory : *mut super::super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltIsDirectory(fileobject, instance.param().abi(), isdirectory)
}
#[inline]
pub unsafe fn FltIsEcpAcknowledged<P0>(filter: P0, ecpcontext: *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltIsEcpAcknowledged(filter : PFLT_FILTER, ecpcontext : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltIsEcpAcknowledged(filter.param().abi(), ecpcontext)
}
#[inline]
pub unsafe fn FltIsEcpFromUserMode<P0>(filter: P0, ecpcontext: *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltIsEcpFromUserMode(filter : PFLT_FILTER, ecpcontext : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltIsEcpFromUserMode(filter.param().abi(), ecpcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsFltMgrVolumeDeviceObject(deviceobject: *const super::super::super::Foundation::DEVICE_OBJECT) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltIsFltMgrVolumeDeviceObject(deviceobject : *const super::super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltIsFltMgrVolumeDeviceObject(deviceobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsIoCanceled(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltIsIoCanceled(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltIsIoCanceled(callbackdata)
}
#[inline]
pub unsafe fn FltIsIoRedirectionAllowed<P0, P1>(sourceinstance: P0, targetinstance: P1, redirectionallowed: *mut super::super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltIsIoRedirectionAllowed(sourceinstance : PFLT_INSTANCE, targetinstance : PFLT_INSTANCE, redirectionallowed : *mut super::super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltIsIoRedirectionAllowed(sourceinstance.param().abi(), targetinstance.param().abi(), redirectionallowed)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsIoRedirectionAllowedForOperation<P0>(data: *const FLT_CALLBACK_DATA, targetinstance: P0, redirectionallowedthisio: *mut super::super::super::super::Win32::Foundation::BOOLEAN, redirectionallowedallio: Option<*mut super::super::super::super::Win32::Foundation::BOOLEAN>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltIsIoRedirectionAllowedForOperation(data : *const FLT_CALLBACK_DATA, targetinstance : PFLT_INSTANCE, redirectionallowedthisio : *mut super::super::super::super::Win32::Foundation:: BOOLEAN, redirectionallowedallio : *mut super::super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltIsIoRedirectionAllowedForOperation(data, targetinstance.param().abi(), redirectionallowedthisio, core::mem::transmute(redirectionallowedallio.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsOperationSynchronous(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltIsOperationSynchronous(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltIsOperationSynchronous(callbackdata)
}
#[inline]
pub unsafe fn FltIsVolumeSnapshot(fltobject: *const core::ffi::c_void, issnapshotvolume: *mut super::super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltIsVolumeSnapshot(fltobject : *const core::ffi::c_void, issnapshotvolume : *mut super::super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltIsVolumeSnapshot(fltobject, issnapshotvolume)
}
#[inline]
pub unsafe fn FltIsVolumeWritable(fltobject: *const core::ffi::c_void, iswritable: *mut super::super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltIsVolumeWritable(fltobject : *const core::ffi::c_void, iswritable : *mut super::super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltIsVolumeWritable(fltobject, iswritable)
}
#[inline]
pub unsafe fn FltLoadFilter(filtername: *const super::super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltLoadFilter(filtername : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltLoadFilter(filtername)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltLockUserBuffer(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltLockUserBuffer(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltLockUserBuffer(callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltNotifyFilterChangeDirectory<P0, P1, P2>(notifysync: P0, notifylist: *mut super::super::super::super::Win32::System::Kernel::LIST_ENTRY, fscontext: *const core::ffi::c_void, fulldirectoryname: *const super::super::super::super::Win32::System::Kernel::STRING, watchtree: P1, ignorebuffer: P2, completionfilter: u32, notifycallbackdata: *const FLT_CALLBACK_DATA, traversecallback: super::PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext: Option<*const super::super::super::Foundation::SECURITY_SUBJECT_CONTEXT>, filtercallback: super::PFILTER_REPORT_CHANGE)
where
    P0: windows_core::Param<super::super::super::Foundation::PNOTIFY_SYNC>,
    P1: windows_core::Param<super::super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltNotifyFilterChangeDirectory(notifysync : super::super::super::Foundation:: PNOTIFY_SYNC, notifylist : *mut super::super::super::super::Win32::System::Kernel:: LIST_ENTRY, fscontext : *const core::ffi::c_void, fulldirectoryname : *const super::super::super::super::Win32::System::Kernel:: STRING, watchtree : super::super::super::super::Win32::Foundation:: BOOLEAN, ignorebuffer : super::super::super::super::Win32::Foundation:: BOOLEAN, completionfilter : u32, notifycallbackdata : *const FLT_CALLBACK_DATA, traversecallback : super:: PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext : *const super::super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, filtercallback : super:: PFILTER_REPORT_CHANGE));
    FltNotifyFilterChangeDirectory(notifysync.param().abi(), notifylist, fscontext, fulldirectoryname, watchtree.param().abi(), ignorebuffer.param().abi(), completionfilter, notifycallbackdata, traversecallback, core::mem::transmute(subjectcontext.unwrap_or(std::ptr::null())), filtercallback)
}
#[inline]
pub unsafe fn FltObjectDereference(fltobject: *mut core::ffi::c_void) {
    windows_targets::link!("fltmgr.sys" "system" fn FltObjectDereference(fltobject : *mut core::ffi::c_void));
    FltObjectDereference(fltobject)
}
#[inline]
pub unsafe fn FltObjectReference(fltobject: *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltObjectReference(fltobject : *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltObjectReference(fltobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOpenVolume<P0>(instance: P0, volumehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, volumefileobject: Option<*mut *mut super::super::super::Foundation::FILE_OBJECT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltOpenVolume(instance : PFLT_INSTANCE, volumehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, volumefileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltOpenVolume(instance.param().abi(), volumehandle, core::mem::transmute(volumefileobject.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockBreakH(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, flags: u32, context: Option<*const core::ffi::c_void>, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltOplockBreakH(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, flags : u32, context : *const core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    FltOplockBreakH(oplock, callbackdata, flags, core::mem::transmute(context.unwrap_or(std::ptr::null())), waitcompletionroutine, prepostcallbackdataroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockBreakToNone(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, context: Option<*const core::ffi::c_void>, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltOplockBreakToNone(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, context : *const core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    FltOplockBreakToNone(oplock, callbackdata, core::mem::transmute(context.unwrap_or(std::ptr::null())), waitcompletionroutine, prepostcallbackdataroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockBreakToNoneEx(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, flags: u32, context: Option<*const core::ffi::c_void>, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltOplockBreakToNoneEx(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, flags : u32, context : *const core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    FltOplockBreakToNoneEx(oplock, callbackdata, flags, core::mem::transmute(context.unwrap_or(std::ptr::null())), waitcompletionroutine, prepostcallbackdataroutine)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockFsctrl(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, opencount: u32) -> FLT_PREOP_CALLBACK_STATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltOplockFsctrl(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, opencount : u32) -> FLT_PREOP_CALLBACK_STATUS);
    FltOplockFsctrl(oplock, callbackdata, opencount)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockFsctrlEx(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, opencount: u32, flags: u32) -> FLT_PREOP_CALLBACK_STATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltOplockFsctrlEx(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, opencount : u32, flags : u32) -> FLT_PREOP_CALLBACK_STATUS);
    FltOplockFsctrlEx(oplock, callbackdata, opencount, flags)
}
#[inline]
pub unsafe fn FltOplockIsFastIoPossible(oplock: *const *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltOplockIsFastIoPossible(oplock : *const *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltOplockIsFastIoPossible(oplock)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockIsSharedRequest(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltOplockIsSharedRequest(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltOplockIsSharedRequest(callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockKeysEqual(fo1: Option<*const super::super::super::Foundation::FILE_OBJECT>, fo2: Option<*const super::super::super::Foundation::FILE_OBJECT>) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltOplockKeysEqual(fo1 : *const super::super::super::Foundation:: FILE_OBJECT, fo2 : *const super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltOplockKeysEqual(core::mem::transmute(fo1.unwrap_or(std::ptr::null())), core::mem::transmute(fo2.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn FltParseFileName(filename: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, extension: Option<*mut super::super::super::super::Win32::Foundation::UNICODE_STRING>, stream: Option<*mut super::super::super::super::Win32::Foundation::UNICODE_STRING>, finalcomponent: Option<*mut super::super::super::super::Win32::Foundation::UNICODE_STRING>) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltParseFileName(filename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, extension : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, stream : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, finalcomponent : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltParseFileName(filename, core::mem::transmute(extension.unwrap_or(std::ptr::null_mut())), core::mem::transmute(stream.unwrap_or(std::ptr::null_mut())), core::mem::transmute(finalcomponent.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltParseFileNameInformation(filenameinformation: *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltParseFileNameInformation(filenameinformation : *mut FLT_FILE_NAME_INFORMATION) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltParseFileNameInformation(filenameinformation)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPerformAsynchronousIo(callbackdata: *mut FLT_CALLBACK_DATA, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltPerformAsynchronousIo(callbackdata : *mut FLT_CALLBACK_DATA, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltPerformAsynchronousIo(callbackdata, callbackroutine, callbackcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPerformSynchronousIo(callbackdata: *mut FLT_CALLBACK_DATA) {
    windows_targets::link!("fltmgr.sys" "system" fn FltPerformSynchronousIo(callbackdata : *mut FLT_CALLBACK_DATA));
    FltPerformSynchronousIo(callbackdata)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltPrePrepareComplete<P0, P1>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: P1) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltPrePrepareComplete(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltPrePrepareComplete(instance.param().abi(), transaction, transactioncontext.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltPrepareComplete<P0, P1>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: P1) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltPrepareComplete(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltPrepareComplete(instance.param().abi(), transaction, transactioncontext.param().abi())
}
#[inline]
pub unsafe fn FltPrepareToReuseEcp<P0>(filter: P0, ecpcontext: *const core::ffi::c_void)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltPrepareToReuseEcp(filter : PFLT_FILTER, ecpcontext : *const core::ffi::c_void));
    FltPrepareToReuseEcp(filter.param().abi(), ecpcontext)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltProcessFileLock(filelock: *const super::FILE_LOCK, callbackdata: *const FLT_CALLBACK_DATA, context: Option<*const core::ffi::c_void>) -> FLT_PREOP_CALLBACK_STATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltProcessFileLock(filelock : *const super:: FILE_LOCK, callbackdata : *const FLT_CALLBACK_DATA, context : *const core::ffi::c_void) -> FLT_PREOP_CALLBACK_STATUS);
    FltProcessFileLock(filelock, callbackdata, core::mem::transmute(context.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPropagateActivityIdToThread(callbackdata: *const FLT_CALLBACK_DATA, propagateid: *mut windows_core::GUID, originalid: *mut *mut windows_core::GUID) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltPropagateActivityIdToThread(callbackdata : *const FLT_CALLBACK_DATA, propagateid : *mut windows_core::GUID, originalid : *mut *mut windows_core::GUID) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltPropagateActivityIdToThread(callbackdata, propagateid, originalid)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPropagateIrpExtension(sourcedata: *const FLT_CALLBACK_DATA, targetdata: *mut FLT_CALLBACK_DATA, flags: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltPropagateIrpExtension(sourcedata : *const FLT_CALLBACK_DATA, targetdata : *mut FLT_CALLBACK_DATA, flags : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltPropagateIrpExtension(sourcedata, targetdata, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPurgeFileNameInformationCache<P0>(instance: P0, fileobject: Option<*const super::super::super::Foundation::FILE_OBJECT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltPurgeFileNameInformationCache(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltPurgeFileNameInformationCache(instance.param().abi(), core::mem::transmute(fileobject.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryDirectoryFile<P0, P1, P2>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, returnsingleentry: P1, filename: Option<*const super::super::super::super::Win32::Foundation::UNICODE_STRING>, restartscan: P2, lengthreturned: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<super::super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueryDirectoryFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS, returnsingleentry : super::super::super::super::Win32::Foundation:: BOOLEAN, filename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, restartscan : super::super::super::super::Win32::Foundation:: BOOLEAN, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueryDirectoryFile(instance.param().abi(), fileobject, fileinformation, length, fileinformationclass, returnsingleentry.param().abi(), core::mem::transmute(filename.unwrap_or(std::ptr::null())), restartscan.param().abi(), core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryDirectoryFileEx<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, queryflags: u32, filename: Option<*const super::super::super::super::Win32::Foundation::UNICODE_STRING>, lengthreturned: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueryDirectoryFileEx(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS, queryflags : u32, filename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueryDirectoryFileEx(instance.param().abi(), fileobject, fileinformation, length, fileinformationclass, queryflags, core::mem::transmute(filename.unwrap_or(std::ptr::null())), core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryEaFile<P0, P1, P2>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, returnedeadata: *mut core::ffi::c_void, length: u32, returnsingleentry: P1, ealist: Option<*const core::ffi::c_void>, ealistlength: u32, eaindex: Option<*const u32>, restartscan: P2, lengthreturned: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<super::super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueryEaFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, returnedeadata : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::super::Win32::Foundation:: BOOLEAN, ealist : *const core::ffi::c_void, ealistlength : u32, eaindex : *const u32, restartscan : super::super::super::super::Win32::Foundation:: BOOLEAN, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueryEaFile(instance.param().abi(), fileobject, returnedeadata, length, returnsingleentry.param().abi(), core::mem::transmute(ealist.unwrap_or(std::ptr::null())), ealistlength, core::mem::transmute(eaindex.unwrap_or(std::ptr::null())), restartscan.param().abi(), core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FltQueryInformationByName<P0, P1>(filter: P0, instance: P1, objectattributes: *const super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, drivercontext: Option<*const super::super::super::System::SystemServices::IO_DRIVER_CREATE_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
    P1: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueryInformationByName(filter : PFLT_FILTER, instance : PFLT_INSTANCE, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS, drivercontext : *const super::super::super::System::SystemServices:: IO_DRIVER_CREATE_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueryInformationByName(filter.param().abi(), instance.param().abi(), objectattributes, iostatusblock, fileinformation, length, fileinformationclass, core::mem::transmute(drivercontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryInformationFile<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, lengthreturned: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueryInformationFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueryInformationFile(instance.param().abi(), fileobject, fileinformation, length, fileinformationclass, core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryQuotaInformationFile<P0, P1, P2>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: P1, sidlist: Option<*const core::ffi::c_void>, sidlistlength: u32, startsid: Option<*const u32>, restartscan: P2, lengthreturned: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<super::super::super::super::Win32::Foundation::BOOLEAN>,
    P2: windows_core::Param<super::super::super::super::Win32::Foundation::BOOLEAN>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueryQuotaInformationFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : super::super::super::super::Win32::Foundation:: BOOLEAN, sidlist : *const core::ffi::c_void, sidlistlength : u32, startsid : *const u32, restartscan : super::super::super::super::Win32::Foundation:: BOOLEAN, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueryQuotaInformationFile(instance.param().abi(), fileobject, iostatusblock, buffer, length, returnsingleentry.param().abi(), core::mem::transmute(sidlist.unwrap_or(std::ptr::null())), sidlistlength, core::mem::transmute(startsid.unwrap_or(std::ptr::null())), restartscan.param().abi(), core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQuerySecurityObject<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, securityinformation: u32, securitydescriptor: super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, length: u32, lengthneeded: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQuerySecurityObject(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, securityinformation : u32, securitydescriptor : super::super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, length : u32, lengthneeded : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQuerySecurityObject(instance.param().abi(), fileobject, securityinformation, securitydescriptor, length, core::mem::transmute(lengthneeded.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn FltQueryVolumeInformation<P0>(instance: P0, iosb: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueryVolumeInformation(instance : PFLT_INSTANCE, iosb : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super:: FS_INFORMATION_CLASS) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueryVolumeInformation(instance.param().abi(), iosb, fsinformation, length, fsinformationclass)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryVolumeInformationFile<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS, lengthreturned: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueryVolumeInformationFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super:: FS_INFORMATION_CLASS, lengthreturned : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueryVolumeInformationFile(instance.param().abi(), fileobject, fsinformation, length, fsinformationclass, core::mem::transmute(lengthreturned.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueueDeferredIoWorkItem<P0>(fltworkitem: P0, data: *const FLT_CALLBACK_DATA, workerroutine: PFLT_DEFERRED_IO_WORKITEM_ROUTINE, queuetype: super::super::super::System::SystemServices::WORK_QUEUE_TYPE, context: *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_DEFERRED_IO_WORKITEM>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueueDeferredIoWorkItem(fltworkitem : PFLT_DEFERRED_IO_WORKITEM, data : *const FLT_CALLBACK_DATA, workerroutine : PFLT_DEFERRED_IO_WORKITEM_ROUTINE, queuetype : super::super::super::System::SystemServices:: WORK_QUEUE_TYPE, context : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueueDeferredIoWorkItem(fltworkitem.param().abi(), data, workerroutine, queuetype, context)
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn FltQueueGenericWorkItem<P0>(fltworkitem: P0, fltobject: *const core::ffi::c_void, workerroutine: PFLT_GENERIC_WORKITEM_ROUTINE, queuetype: super::super::super::System::SystemServices::WORK_QUEUE_TYPE, context: Option<*const core::ffi::c_void>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_GENERIC_WORKITEM>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltQueueGenericWorkItem(fltworkitem : PFLT_GENERIC_WORKITEM, fltobject : *const core::ffi::c_void, workerroutine : PFLT_GENERIC_WORKITEM_ROUTINE, queuetype : super::super::super::System::SystemServices:: WORK_QUEUE_TYPE, context : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltQueueGenericWorkItem(fltworkitem.param().abi(), fltobject, workerroutine, queuetype, core::mem::transmute(context.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltReadFile<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, byteoffset: Option<*const i64>, length: u32, buffer: *mut core::ffi::c_void, flags: u32, bytesread: Option<*mut u32>, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltReadFile(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, byteoffset : *const i64, length : u32, buffer : *mut core::ffi::c_void, flags : u32, bytesread : *mut u32, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltReadFile(initiatinginstance.param().abi(), fileobject, core::mem::transmute(byteoffset.unwrap_or(std::ptr::null())), length, buffer, flags, core::mem::transmute(bytesread.unwrap_or(std::ptr::null_mut())), callbackroutine, core::mem::transmute(callbackcontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltReadFileEx<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, byteoffset: Option<*const i64>, length: u32, buffer: Option<*mut core::ffi::c_void>, flags: u32, bytesread: Option<*mut u32>, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>, key: Option<*const u32>, mdl: Option<*const super::super::super::Foundation::MDL>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltReadFileEx(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, byteoffset : *const i64, length : u32, buffer : *mut core::ffi::c_void, flags : u32, bytesread : *mut u32, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *const core::ffi::c_void, key : *const u32, mdl : *const super::super::super::Foundation:: MDL) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltReadFileEx(initiatinginstance.param().abi(), fileobject, core::mem::transmute(byteoffset.unwrap_or(std::ptr::null())), length, core::mem::transmute(buffer.unwrap_or(std::ptr::null_mut())), flags, core::mem::transmute(bytesread.unwrap_or(std::ptr::null_mut())), callbackroutine, core::mem::transmute(callbackcontext.unwrap_or(std::ptr::null())), core::mem::transmute(key.unwrap_or(std::ptr::null())), core::mem::transmute(mdl.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn FltReferenceContext<P0>(context: P0)
where
    P0: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltReferenceContext(context : PFLT_CONTEXT));
    FltReferenceContext(context.param().abi())
}
#[inline]
pub unsafe fn FltReferenceFileNameInformation(filenameinformation: *const FLT_FILE_NAME_INFORMATION) {
    windows_targets::link!("fltmgr.sys" "system" fn FltReferenceFileNameInformation(filenameinformation : *const FLT_FILE_NAME_INFORMATION));
    FltReferenceFileNameInformation(filenameinformation)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRegisterFilter(driver: *const super::super::super::Foundation::DRIVER_OBJECT, registration: *const FLT_REGISTRATION, retfilter: *mut PFLT_FILTER) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltRegisterFilter(driver : *const super::super::super::Foundation:: DRIVER_OBJECT, registration : *const FLT_REGISTRATION, retfilter : *mut PFLT_FILTER) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRegisterFilter(driver, registration, retfilter)
}
#[inline]
pub unsafe fn FltRegisterForDataScan<P0>(instance: P0) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRegisterForDataScan(instance : PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRegisterForDataScan(instance.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltReissueSynchronousIo<P0>(initiatinginstance: P0, callbackdata: *const FLT_CALLBACK_DATA)
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltReissueSynchronousIo(initiatinginstance : PFLT_INSTANCE, callbackdata : *const FLT_CALLBACK_DATA));
    FltReissueSynchronousIo(initiatinginstance.param().abi(), callbackdata)
}
#[inline]
pub unsafe fn FltReleaseContext<P0>(context: P0)
where
    P0: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltReleaseContext(context : PFLT_CONTEXT));
    FltReleaseContext(context.param().abi())
}
#[inline]
pub unsafe fn FltReleaseContexts(contexts: *const FLT_RELATED_CONTEXTS) {
    windows_targets::link!("fltmgr.sys" "system" fn FltReleaseContexts(contexts : *const FLT_RELATED_CONTEXTS));
    FltReleaseContexts(contexts)
}
#[inline]
pub unsafe fn FltReleaseContextsEx(contextssize: usize, contexts: *const FLT_RELATED_CONTEXTS_EX) {
    windows_targets::link!("fltmgr.sys" "system" fn FltReleaseContextsEx(contextssize : usize, contexts : *const FLT_RELATED_CONTEXTS_EX));
    FltReleaseContextsEx(contextssize, contexts)
}
#[inline]
pub unsafe fn FltReleaseFileNameInformation(filenameinformation: *const FLT_FILE_NAME_INFORMATION) {
    windows_targets::link!("fltmgr.sys" "system" fn FltReleaseFileNameInformation(filenameinformation : *const FLT_FILE_NAME_INFORMATION));
    FltReleaseFileNameInformation(filenameinformation)
}
#[inline]
pub unsafe fn FltReleasePushLock(pushlock: *mut usize) {
    windows_targets::link!("fltmgr.sys" "system" fn FltReleasePushLock(pushlock : *mut usize));
    FltReleasePushLock(pushlock)
}
#[inline]
pub unsafe fn FltReleasePushLockEx(pushlock: *mut usize, flags: u32) {
    windows_targets::link!("fltmgr.sys" "system" fn FltReleasePushLockEx(pushlock : *mut usize, flags : u32));
    FltReleasePushLockEx(pushlock, flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FltReleaseResource(resource: *mut super::super::super::Foundation::ERESOURCE) {
    windows_targets::link!("fltmgr.sys" "system" fn FltReleaseResource(resource : *mut super::super::super::Foundation:: ERESOURCE));
    FltReleaseResource(resource)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltRemoveExtraCreateParameter<P0>(filter: P0, ecplist: *mut super::super::super::Foundation::ECP_LIST, ecptype: *const windows_core::GUID, ecpcontext: *mut *mut core::ffi::c_void, ecpcontextsize: Option<*mut u32>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRemoveExtraCreateParameter(filter : PFLT_FILTER, ecplist : *mut super::super::super::Foundation:: ECP_LIST, ecptype : *const windows_core::GUID, ecpcontext : *mut *mut core::ffi::c_void, ecpcontextsize : *mut u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRemoveExtraCreateParameter(filter.param().abi(), ecplist, ecptype, ecpcontext, core::mem::transmute(ecpcontextsize.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRemoveOpenReparseEntry<P0>(filter: P0, data: *const FLT_CALLBACK_DATA, openreparseentry: *const super::OPEN_REPARSE_LIST_ENTRY)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRemoveOpenReparseEntry(filter : PFLT_FILTER, data : *const FLT_CALLBACK_DATA, openreparseentry : *const super:: OPEN_REPARSE_LIST_ENTRY));
    FltRemoveOpenReparseEntry(filter.param().abi(), data, openreparseentry)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRequestFileInfoOnCreateCompletion<P0>(filter: P0, data: *const FLT_CALLBACK_DATA, infoclassflags: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRequestFileInfoOnCreateCompletion(filter : PFLT_FILTER, data : *const FLT_CALLBACK_DATA, infoclassflags : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRequestFileInfoOnCreateCompletion(filter.param().abi(), data, infoclassflags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRequestOperationStatusCallback(data: *const FLT_CALLBACK_DATA, callbackroutine: PFLT_GET_OPERATION_STATUS_CALLBACK, requestercontext: Option<*const core::ffi::c_void>) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltRequestOperationStatusCallback(data : *const FLT_CALLBACK_DATA, callbackroutine : PFLT_GET_OPERATION_STATUS_CALLBACK, requestercontext : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRequestOperationStatusCallback(data, callbackroutine, core::mem::transmute(requestercontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRetainSwappedBufferMdlAddress(callbackdata: *const FLT_CALLBACK_DATA) {
    windows_targets::link!("fltmgr.sys" "system" fn FltRetainSwappedBufferMdlAddress(callbackdata : *const FLT_CALLBACK_DATA));
    FltRetainSwappedBufferMdlAddress(callbackdata)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRetrieveFileInfoOnCreateCompletion<P0>(filter: P0, data: *const FLT_CALLBACK_DATA, infoclass: u32, size: *mut u32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRetrieveFileInfoOnCreateCompletion(filter : PFLT_FILTER, data : *const FLT_CALLBACK_DATA, infoclass : u32, size : *mut u32) -> *mut core::ffi::c_void);
    FltRetrieveFileInfoOnCreateCompletion(filter.param().abi(), data, infoclass, size)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRetrieveFileInfoOnCreateCompletionEx<P0>(filter: P0, data: *const FLT_CALLBACK_DATA, infoclass: u32, retinfosize: *mut u32, retinfobuffer: *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRetrieveFileInfoOnCreateCompletionEx(filter : PFLT_FILTER, data : *const FLT_CALLBACK_DATA, infoclass : u32, retinfosize : *mut u32, retinfobuffer : *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRetrieveFileInfoOnCreateCompletionEx(filter.param().abi(), data, infoclass, retinfosize, retinfobuffer)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRetrieveIoPriorityInfo<P0>(data: Option<*const FLT_CALLBACK_DATA>, fileobject: Option<*const super::super::super::Foundation::FILE_OBJECT>, thread: P0, priorityinfo: *mut super::IO_PRIORITY_INFO) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRetrieveIoPriorityInfo(data : *const FLT_CALLBACK_DATA, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, thread : super::super::super::Foundation:: PETHREAD, priorityinfo : *mut super:: IO_PRIORITY_INFO) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRetrieveIoPriorityInfo(core::mem::transmute(data.unwrap_or(std::ptr::null())), core::mem::transmute(fileobject.unwrap_or(std::ptr::null())), thread.param().abi(), priorityinfo)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltReuseCallbackData(callbackdata: *mut FLT_CALLBACK_DATA) {
    windows_targets::link!("fltmgr.sys" "system" fn FltReuseCallbackData(callbackdata : *mut FLT_CALLBACK_DATA));
    FltReuseCallbackData(callbackdata)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltRollbackComplete<P0, P1>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: P1) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRollbackComplete(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRollbackComplete(instance.param().abi(), transaction, transactioncontext.param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltRollbackEnlistment<P0, P1>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: P1) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltRollbackEnlistment(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltRollbackEnlistment(instance.param().abi(), transaction, transactioncontext.param().abi())
}
#[inline]
pub unsafe fn FltSendMessage<P0>(filter: P0, clientport: *const PFLT_PORT, senderbuffer: *const core::ffi::c_void, senderbufferlength: u32, replybuffer: Option<*mut core::ffi::c_void>, replylength: Option<*mut u32>, timeout: Option<*const i64>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSendMessage(filter : PFLT_FILTER, clientport : *const PFLT_PORT, senderbuffer : *const core::ffi::c_void, senderbufferlength : u32, replybuffer : *mut core::ffi::c_void, replylength : *mut u32, timeout : *const i64) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSendMessage(filter.param().abi(), clientport, senderbuffer, senderbufferlength, core::mem::transmute(replybuffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(replylength.unwrap_or(std::ptr::null_mut())), core::mem::transmute(timeout.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetActivityIdCallbackData(callbackdata: *mut FLT_CALLBACK_DATA, guid: Option<*const windows_core::GUID>) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltSetActivityIdCallbackData(callbackdata : *mut FLT_CALLBACK_DATA, guid : *const windows_core::GUID) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetActivityIdCallbackData(callbackdata, core::mem::transmute(guid.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetCallbackDataDirty(data: *mut FLT_CALLBACK_DATA) {
    windows_targets::link!("fltmgr.sys" "system" fn FltSetCallbackDataDirty(data : *mut FLT_CALLBACK_DATA));
    FltSetCallbackDataDirty(data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetCancelCompletion(callbackdata: *const FLT_CALLBACK_DATA, canceledcallback: PFLT_COMPLETE_CANCELED_CALLBACK) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltSetCancelCompletion(callbackdata : *const FLT_CALLBACK_DATA, canceledcallback : PFLT_COMPLETE_CANCELED_CALLBACK) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetCancelCompletion(callbackdata, canceledcallback)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetEaFile<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, eabuffer: *const core::ffi::c_void, length: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetEaFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, eabuffer : *const core::ffi::c_void, length : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetEaFile(instance.param().abi(), fileobject, eabuffer, length)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetEcpListIntoCallbackData<P0>(filter: P0, callbackdata: *const FLT_CALLBACK_DATA, ecplist: *const super::super::super::Foundation::ECP_LIST) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetEcpListIntoCallbackData(filter : PFLT_FILTER, callbackdata : *const FLT_CALLBACK_DATA, ecplist : *const super::super::super::Foundation:: ECP_LIST) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetEcpListIntoCallbackData(filter.param().abi(), callbackdata, ecplist)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetFileContext<P0, P1>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, operation: FLT_SET_CONTEXT_OPERATION, newcontext: P1, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetFileContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetFileContext(instance.param().abi(), fileobject, operation, newcontext.param().abi(), core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetFsZeroingOffset(data: *const FLT_CALLBACK_DATA, zeroingoffset: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltSetFsZeroingOffset(data : *const FLT_CALLBACK_DATA, zeroingoffset : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetFsZeroingOffset(data, zeroingoffset)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetFsZeroingOffsetRequired(data: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltSetFsZeroingOffsetRequired(data : *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetFsZeroingOffsetRequired(data)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetInformationFile<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileinformation: *const core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetInformationFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetInformationFile(instance.param().abi(), fileobject, fileinformation, length, fileinformationclass)
}
#[inline]
pub unsafe fn FltSetInstanceContext<P0, P1>(instance: P0, operation: FLT_SET_CONTEXT_OPERATION, newcontext: P1, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetInstanceContext(instance : PFLT_INSTANCE, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetInstanceContext(instance.param().abi(), operation, newcontext.param().abi(), core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetIoPriorityHintIntoCallbackData(data: *const FLT_CALLBACK_DATA, priorityhint: super::super::super::Foundation::IO_PRIORITY_HINT) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltSetIoPriorityHintIntoCallbackData(data : *const FLT_CALLBACK_DATA, priorityhint : super::super::super::Foundation:: IO_PRIORITY_HINT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetIoPriorityHintIntoCallbackData(data, priorityhint)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetIoPriorityHintIntoFileObject(fileobject: *const super::super::super::Foundation::FILE_OBJECT, priorityhint: super::super::super::Foundation::IO_PRIORITY_HINT) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltSetIoPriorityHintIntoFileObject(fileobject : *const super::super::super::Foundation:: FILE_OBJECT, priorityhint : super::super::super::Foundation:: IO_PRIORITY_HINT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetIoPriorityHintIntoFileObject(fileobject, priorityhint)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltSetIoPriorityHintIntoThread<P0>(thread: P0, priorityhint: super::super::super::Foundation::IO_PRIORITY_HINT) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::Foundation::PETHREAD>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetIoPriorityHintIntoThread(thread : super::super::super::Foundation:: PETHREAD, priorityhint : super::super::super::Foundation:: IO_PRIORITY_HINT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetIoPriorityHintIntoThread(thread.param().abi(), priorityhint)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetQuotaInformationFile<P0>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, buffer: *const core::ffi::c_void, length: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetQuotaInformationFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, buffer : *const core::ffi::c_void, length : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetQuotaInformationFile(instance.param().abi(), fileobject, buffer, length)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetSecurityObject<P0, P1>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, securityinformation: u32, securitydescriptor: P1) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetSecurityObject(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, securityinformation : u32, securitydescriptor : super::super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetSecurityObject(instance.param().abi(), fileobject, securityinformation, securitydescriptor.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetStreamContext<P0, P1>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, operation: FLT_SET_CONTEXT_OPERATION, newcontext: P1, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetStreamContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetStreamContext(instance.param().abi(), fileobject, operation, newcontext.param().abi(), core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetStreamHandleContext<P0, P1>(instance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, operation: FLT_SET_CONTEXT_OPERATION, newcontext: P1, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetStreamHandleContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetStreamHandleContext(instance.param().abi(), fileobject, operation, newcontext.param().abi(), core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltSetTransactionContext<P0, P1>(instance: P0, transaction: *const super::super::super::Foundation::KTRANSACTION, operation: FLT_SET_CONTEXT_OPERATION, newcontext: P1, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetTransactionContext(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetTransactionContext(instance.param().abi(), transaction, operation, newcontext.param().abi(), core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn FltSetVolumeContext<P0, P1>(volume: P0, operation: FLT_SET_CONTEXT_OPERATION, newcontext: P1, oldcontext: Option<*mut PFLT_CONTEXT>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_VOLUME>,
    P1: windows_core::Param<PFLT_CONTEXT>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetVolumeContext(volume : PFLT_VOLUME, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetVolumeContext(volume.param().abi(), operation, newcontext.param().abi(), core::mem::transmute(oldcontext.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn FltSetVolumeInformation<P0>(instance: P0, iosb: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSetVolumeInformation(instance : PFLT_INSTANCE, iosb : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super:: FS_INFORMATION_CLASS) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltSetVolumeInformation(instance.param().abi(), iosb, fsinformation, length, fsinformationclass)
}
#[inline]
pub unsafe fn FltStartFiltering<P0>(filter: P0) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltStartFiltering(filter : PFLT_FILTER) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltStartFiltering(filter.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSupportsFileContexts(fileobject: *const super::super::super::Foundation::FILE_OBJECT) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltSupportsFileContexts(fileobject : *const super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltSupportsFileContexts(fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSupportsFileContextsEx<P0>(fileobject: *const super::super::super::Foundation::FILE_OBJECT, instance: P0) -> super::super::super::super::Win32::Foundation::BOOLEAN
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltSupportsFileContextsEx(fileobject : *const super::super::super::Foundation:: FILE_OBJECT, instance : PFLT_INSTANCE) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltSupportsFileContextsEx(fileobject, instance.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSupportsStreamContexts(fileobject: *const super::super::super::Foundation::FILE_OBJECT) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltSupportsStreamContexts(fileobject : *const super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltSupportsStreamContexts(fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSupportsStreamHandleContexts(fileobject: *const super::super::super::Foundation::FILE_OBJECT) -> super::super::super::super::Win32::Foundation::BOOLEAN {
    windows_targets::link!("fltmgr.sys" "system" fn FltSupportsStreamHandleContexts(fileobject : *const super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::super::Win32::Foundation:: BOOLEAN);
    FltSupportsStreamHandleContexts(fileobject)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltTagFile<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, filetag: u32, guid: Option<*const windows_core::GUID>, databuffer: *const core::ffi::c_void, databufferlength: u16) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltTagFile(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, filetag : u32, guid : *const windows_core::GUID, databuffer : *const core::ffi::c_void, databufferlength : u16) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltTagFile(initiatinginstance.param().abi(), fileobject, filetag, core::mem::transmute(guid.unwrap_or(std::ptr::null())), databuffer, databufferlength)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltTagFileEx<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, filetag: u32, guid: Option<*const windows_core::GUID>, databuffer: *const core::ffi::c_void, databufferlength: u16, existingfiletag: u32, existingguid: Option<*const windows_core::GUID>, flags: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltTagFileEx(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, filetag : u32, guid : *const windows_core::GUID, databuffer : *const core::ffi::c_void, databufferlength : u16, existingfiletag : u32, existingguid : *const windows_core::GUID, flags : u32) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltTagFileEx(initiatinginstance.param().abi(), fileobject, filetag, core::mem::transmute(guid.unwrap_or(std::ptr::null())), databuffer, databufferlength, existingfiletag, core::mem::transmute(existingguid.unwrap_or(std::ptr::null())), flags)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltUninitializeFileLock(filelock: *const super::FILE_LOCK) {
    windows_targets::link!("fltmgr.sys" "system" fn FltUninitializeFileLock(filelock : *const super:: FILE_LOCK));
    FltUninitializeFileLock(filelock)
}
#[inline]
pub unsafe fn FltUninitializeOplock(oplock: *const *const core::ffi::c_void) {
    windows_targets::link!("fltmgr.sys" "system" fn FltUninitializeOplock(oplock : *const *const core::ffi::c_void));
    FltUninitializeOplock(oplock)
}
#[inline]
pub unsafe fn FltUnloadFilter(filtername: *const super::super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltUnloadFilter(filtername : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltUnloadFilter(filtername)
}
#[inline]
pub unsafe fn FltUnregisterFilter<P0>(filter: P0)
where
    P0: windows_core::Param<PFLT_FILTER>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltUnregisterFilter(filter : PFLT_FILTER));
    FltUnregisterFilter(filter.param().abi())
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltUntagFile<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, filetag: u32, guid: Option<*const windows_core::GUID>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltUntagFile(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, filetag : u32, guid : *const windows_core::GUID) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltUntagFile(initiatinginstance.param().abi(), fileobject, filetag, core::mem::transmute(guid.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltVetoBypassIo<P0>(callbackdata: *const FLT_CALLBACK_DATA, fltobjects: *const FLT_RELATED_OBJECTS, operationstatus: P0, failurereason: *const super::super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<super::super::super::super::Win32::Foundation::NTSTATUS>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltVetoBypassIo(callbackdata : *const FLT_CALLBACK_DATA, fltobjects : *const FLT_RELATED_OBJECTS, operationstatus : super::super::super::super::Win32::Foundation:: NTSTATUS, failurereason : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltVetoBypassIo(callbackdata, fltobjects, operationstatus.param().abi(), failurereason)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltWriteFile<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, byteoffset: Option<*const i64>, length: u32, buffer: *const core::ffi::c_void, flags: u32, byteswritten: Option<*mut u32>, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltWriteFile(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, byteoffset : *const i64, length : u32, buffer : *const core::ffi::c_void, flags : u32, byteswritten : *mut u32, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltWriteFile(initiatinginstance.param().abi(), fileobject, core::mem::transmute(byteoffset.unwrap_or(std::ptr::null())), length, buffer, flags, core::mem::transmute(byteswritten.unwrap_or(std::ptr::null_mut())), callbackroutine, core::mem::transmute(callbackcontext.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltWriteFileEx<P0>(initiatinginstance: P0, fileobject: *const super::super::super::Foundation::FILE_OBJECT, byteoffset: Option<*const i64>, length: u32, buffer: Option<*const core::ffi::c_void>, flags: u32, byteswritten: Option<*mut u32>, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: Option<*const core::ffi::c_void>, key: Option<*const u32>, mdl: Option<*const super::super::super::Foundation::MDL>) -> super::super::super::super::Win32::Foundation::NTSTATUS
where
    P0: windows_core::Param<PFLT_INSTANCE>,
{
    windows_targets::link!("fltmgr.sys" "system" fn FltWriteFileEx(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, byteoffset : *const i64, length : u32, buffer : *const core::ffi::c_void, flags : u32, byteswritten : *mut u32, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *const core::ffi::c_void, key : *const u32, mdl : *const super::super::super::Foundation:: MDL) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltWriteFileEx(initiatinginstance.param().abi(), fileobject, core::mem::transmute(byteoffset.unwrap_or(std::ptr::null())), length, core::mem::transmute(buffer.unwrap_or(std::ptr::null())), flags, core::mem::transmute(byteswritten.unwrap_or(std::ptr::null_mut())), callbackroutine, core::mem::transmute(callbackcontext.unwrap_or(std::ptr::null())), core::mem::transmute(key.unwrap_or(std::ptr::null())), core::mem::transmute(mdl.unwrap_or(std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltpTraceRedirectedFileIo(originatingfileobject: *const super::super::super::Foundation::FILE_OBJECT, childcallbackdata: *mut FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("fltmgr.sys" "system" fn FltpTraceRedirectedFileIo(originatingfileobject : *const super::super::super::Foundation:: FILE_OBJECT, childcallbackdata : *mut FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: NTSTATUS);
    FltpTraceRedirectedFileIo(originatingfileobject, childcallbackdata)
}
pub const FLTFL_CALLBACK_DATA_DIRTY: u32 = 2147483648u32;
pub const FLTFL_CALLBACK_DATA_DRAINING_IO: u32 = 262144u32;
pub const FLTFL_CALLBACK_DATA_FAST_IO_OPERATION: u32 = 2u32;
pub const FLTFL_CALLBACK_DATA_FS_FILTER_OPERATION: u32 = 4u32;
pub const FLTFL_CALLBACK_DATA_GENERATED_IO: u32 = 65536u32;
pub const FLTFL_CALLBACK_DATA_IRP_OPERATION: u32 = 1u32;
pub const FLTFL_CALLBACK_DATA_NEW_SYSTEM_BUFFER: u32 = 1048576u32;
pub const FLTFL_CALLBACK_DATA_POST_OPERATION: u32 = 524288u32;
pub const FLTFL_CALLBACK_DATA_REISSUED_IO: u32 = 131072u32;
pub const FLTFL_CALLBACK_DATA_REISSUE_MASK: u32 = 65535u32;
pub const FLTFL_CALLBACK_DATA_SYSTEM_BUFFER: u32 = 8u32;
pub const FLTFL_CONTEXT_REGISTRATION_NO_EXACT_SIZE_MATCH: u32 = 1u32;
pub const FLTFL_FILE_NAME_PARSED_EXTENSION: u32 = 2u32;
pub const FLTFL_FILE_NAME_PARSED_FINAL_COMPONENT: u32 = 1u32;
pub const FLTFL_FILE_NAME_PARSED_PARENT_DIR: u32 = 8u32;
pub const FLTFL_FILE_NAME_PARSED_STREAM: u32 = 4u32;
pub const FLTFL_FILTER_UNLOAD_MANDATORY: u32 = 1u32;
pub const FLTFL_INSTANCE_SETUP_AUTOMATIC_ATTACHMENT: u32 = 1u32;
pub const FLTFL_INSTANCE_SETUP_DETACHED_VOLUME: u32 = 8u32;
pub const FLTFL_INSTANCE_SETUP_MANUAL_ATTACHMENT: u32 = 2u32;
pub const FLTFL_INSTANCE_SETUP_NEWLY_MOUNTED_VOLUME: u32 = 4u32;
pub const FLTFL_INSTANCE_TEARDOWN_FILTER_UNLOAD: u32 = 2u32;
pub const FLTFL_INSTANCE_TEARDOWN_INTERNAL_ERROR: u32 = 16u32;
pub const FLTFL_INSTANCE_TEARDOWN_MANDATORY_FILTER_UNLOAD: u32 = 4u32;
pub const FLTFL_INSTANCE_TEARDOWN_MANUAL: u32 = 1u32;
pub const FLTFL_INSTANCE_TEARDOWN_VOLUME_DISMOUNT: u32 = 8u32;
pub const FLTFL_IO_OPERATION_DO_NOT_UPDATE_BYTE_OFFSET: u32 = 4u32;
pub const FLTFL_IO_OPERATION_NON_CACHED: u32 = 1u32;
pub const FLTFL_IO_OPERATION_PAGING: u32 = 2u32;
pub const FLTFL_IO_OPERATION_SYNCHRONOUS_PAGING: u32 = 8u32;
pub const FLTFL_NORMALIZE_NAME_CASE_SENSITIVE: u32 = 1u32;
pub const FLTFL_NORMALIZE_NAME_DESTINATION_FILE_NAME: u32 = 2u32;
pub const FLTFL_OPERATION_REGISTRATION_SKIP_CACHED_IO: u32 = 2u32;
pub const FLTFL_OPERATION_REGISTRATION_SKIP_NON_CACHED_NON_PAGING_IO: u32 = 8u32;
pub const FLTFL_OPERATION_REGISTRATION_SKIP_NON_DASD_IO: u32 = 4u32;
pub const FLTFL_OPERATION_REGISTRATION_SKIP_PAGING_IO: u32 = 1u32;
pub const FLTFL_POST_OPERATION_DRAINING: u32 = 1u32;
pub const FLTFL_REGISTRATION_DO_NOT_SUPPORT_SERVICE_STOP: u32 = 1u32;
pub const FLTFL_REGISTRATION_SUPPORT_DAX_VOLUME: u32 = 4u32;
pub const FLTFL_REGISTRATION_SUPPORT_NPFS_MSFS: u32 = 2u32;
pub const FLTFL_REGISTRATION_SUPPORT_WCOS: u32 = 8u32;
pub const FLTTCFL_AUTO_REPARSE: u32 = 1u32;
pub const FLT_ALLOCATE_CALLBACK_DATA_PREALLOCATE_ALL_MEMORY: u32 = 1u32;
pub const FLT_CONTEXT_END: u32 = 65535u32;
pub const FLT_FILE_CONTEXT: u32 = 4u32;
pub const FLT_FILE_NAME_ALLOW_QUERY_ON_REPARSE: u32 = 67108864u32;
pub const FLT_FILE_NAME_DO_NOT_CACHE: u32 = 33554432u32;
pub const FLT_FILE_NAME_NORMALIZED: u32 = 1u32;
pub const FLT_FILE_NAME_OPENED: u32 = 2u32;
pub const FLT_FILE_NAME_QUERY_ALWAYS_ALLOW_CACHE_LOOKUP: u32 = 1024u32;
pub const FLT_FILE_NAME_QUERY_CACHE_ONLY: u32 = 512u32;
pub const FLT_FILE_NAME_QUERY_DEFAULT: u32 = 256u32;
pub const FLT_FILE_NAME_QUERY_FILESYSTEM_ONLY: u32 = 768u32;
pub const FLT_FILE_NAME_REQUEST_FROM_CURRENT_PROVIDER: u32 = 16777216u32;
pub const FLT_FILE_NAME_SHORT: u32 = 3u32;
pub const FLT_FLUSH_TYPE_DATA_SYNC_ONLY: u32 = 8u32;
pub const FLT_FLUSH_TYPE_FILE_DATA_ONLY: u32 = 2u32;
pub const FLT_FLUSH_TYPE_FLUSH_AND_PURGE: u32 = 1u32;
pub const FLT_FLUSH_TYPE_NO_SYNC: u32 = 4u32;
pub const FLT_INSTANCE_CONTEXT: u32 = 2u32;
pub const FLT_INTERNAL_OPERATION_COUNT: u32 = 22u32;
pub const FLT_MAX_DEVICE_REPARSE_ATTEMPTS: u32 = 64u32;
pub const FLT_PORT_CONNECT: u32 = 1u32;
pub const FLT_POSTOP_DISALLOW_FSFILTER_IO: FLT_POSTOP_CALLBACK_STATUS = FLT_POSTOP_CALLBACK_STATUS(2i32);
pub const FLT_POSTOP_FINISHED_PROCESSING: FLT_POSTOP_CALLBACK_STATUS = FLT_POSTOP_CALLBACK_STATUS(0i32);
pub const FLT_POSTOP_MORE_PROCESSING_REQUIRED: FLT_POSTOP_CALLBACK_STATUS = FLT_POSTOP_CALLBACK_STATUS(1i32);
pub const FLT_PREOP_COMPLETE: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(4i32);
pub const FLT_PREOP_DISALLOW_FASTIO: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(3i32);
pub const FLT_PREOP_DISALLOW_FSFILTER_IO: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(6i32);
pub const FLT_PREOP_PENDING: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(2i32);
pub const FLT_PREOP_SUCCESS_NO_CALLBACK: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(1i32);
pub const FLT_PREOP_SUCCESS_WITH_CALLBACK: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(0i32);
pub const FLT_PREOP_SYNCHRONIZE: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(5i32);
pub const FLT_PUSH_LOCK_DISABLE_AUTO_BOOST: u32 = 2u32;
pub const FLT_PUSH_LOCK_ENABLE_AUTO_BOOST: u32 = 1u32;
pub const FLT_PUSH_LOCK_VALID_FLAGS: u32 = 3u32;
pub const FLT_REGISTRATION_VERSION: u32 = 515u32;
pub const FLT_REGISTRATION_VERSION_0200: u32 = 512u32;
pub const FLT_REGISTRATION_VERSION_0201: u32 = 513u32;
pub const FLT_REGISTRATION_VERSION_0202: u32 = 514u32;
pub const FLT_REGISTRATION_VERSION_0203: u32 = 515u32;
pub const FLT_SECTION_CONTEXT: u32 = 64u32;
pub const FLT_SET_CONTEXT_KEEP_IF_EXISTS: FLT_SET_CONTEXT_OPERATION = FLT_SET_CONTEXT_OPERATION(1i32);
pub const FLT_SET_CONTEXT_REPLACE_IF_EXISTS: FLT_SET_CONTEXT_OPERATION = FLT_SET_CONTEXT_OPERATION(0i32);
pub const FLT_STREAMHANDLE_CONTEXT: u32 = 16u32;
pub const FLT_STREAM_CONTEXT: u32 = 8u32;
pub const FLT_TRANSACTION_CONTEXT: u32 = 32u32;
pub const FLT_VALID_FILE_NAME_FLAGS: u32 = 4278190080u32;
pub const FLT_VALID_FILE_NAME_FORMATS: u32 = 255u32;
pub const FLT_VALID_FILE_NAME_QUERY_METHODS: u32 = 65280u32;
pub const FLT_VOLUME_CONTEXT: u32 = 1u32;
pub const GUID_ECP_FLT_CREATEFILE_TARGET: windows_core::GUID = windows_core::GUID::from_u128(0xce08041d_f411_447f_b70d_ccee45c23fac);
pub const IRP_MJ_ACQUIRE_FOR_CC_FLUSH: u16 = 65531u16;
pub const IRP_MJ_ACQUIRE_FOR_MOD_WRITE: u16 = 65533u16;
pub const IRP_MJ_ACQUIRE_FOR_SECTION_SYNCHRONIZATION: u16 = 65535u16;
pub const IRP_MJ_FAST_IO_CHECK_IF_POSSIBLE: u16 = 65523u16;
pub const IRP_MJ_MDL_READ: u16 = 65521u16;
pub const IRP_MJ_MDL_READ_COMPLETE: u16 = 65520u16;
pub const IRP_MJ_MDL_WRITE_COMPLETE: u16 = 65518u16;
pub const IRP_MJ_NETWORK_QUERY_OPEN: u16 = 65522u16;
pub const IRP_MJ_OPERATION_END: u16 = 128u16;
pub const IRP_MJ_PREPARE_MDL_WRITE: u16 = 65519u16;
pub const IRP_MJ_QUERY_OPEN: u16 = 65529u16;
pub const IRP_MJ_RELEASE_FOR_CC_FLUSH: u16 = 65530u16;
pub const IRP_MJ_RELEASE_FOR_MOD_WRITE: u16 = 65532u16;
pub const IRP_MJ_RELEASE_FOR_SECTION_SYNCHRONIZATION: u16 = 65534u16;
pub const IRP_MJ_VOLUME_DISMOUNT: u16 = 65516u16;
pub const IRP_MJ_VOLUME_MOUNT: u16 = 65517u16;
pub const VOL_PROP_FL_DAX_VOLUME: u32 = 1u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FLT_CALLBACK_DATA_QUEUE_FLAGS(pub i32);
impl windows_core::TypeKind for FLT_CALLBACK_DATA_QUEUE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FLT_CALLBACK_DATA_QUEUE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FLT_CALLBACK_DATA_QUEUE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FLT_POSTOP_CALLBACK_STATUS(pub i32);
impl windows_core::TypeKind for FLT_POSTOP_CALLBACK_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FLT_POSTOP_CALLBACK_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FLT_POSTOP_CALLBACK_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FLT_PREOP_CALLBACK_STATUS(pub i32);
impl windows_core::TypeKind for FLT_PREOP_CALLBACK_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FLT_PREOP_CALLBACK_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FLT_PREOP_CALLBACK_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FLT_SET_CONTEXT_OPERATION(pub i32);
impl windows_core::TypeKind for FLT_SET_CONTEXT_OPERATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FLT_SET_CONTEXT_OPERATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FLT_SET_CONTEXT_OPERATION").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_CALLBACK_DATA {
    pub Flags: u32,
    pub Thread: super::super::super::Foundation::PETHREAD,
    pub Iopb: *const FLT_IO_PARAMETER_BLOCK,
    pub IoStatus: super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK,
    pub TagData: *mut FLT_TAG_DATA_BUFFER,
    pub Anonymous: FLT_CALLBACK_DATA_0,
    pub RequestorMode: i8,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_CALLBACK_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_CALLBACK_DATA_0 {
    pub Anonymous: FLT_CALLBACK_DATA_0_0,
    pub FilterContext: [*mut core::ffi::c_void; 4],
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_CALLBACK_DATA_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_CALLBACK_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_CALLBACK_DATA_0_0 {
    pub QueueLinks: super::super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub QueueContext: [*mut core::ffi::c_void; 2],
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_CALLBACK_DATA_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_CALLBACK_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug)]
pub struct FLT_CALLBACK_DATA_QUEUE {
    pub Csq: super::super::super::System::SystemServices::IO_CSQ,
    pub Flags: FLT_CALLBACK_DATA_QUEUE_FLAGS,
    pub Instance: PFLT_INSTANCE,
    pub InsertIo: PFLT_CALLBACK_DATA_QUEUE_INSERT_IO,
    pub RemoveIo: PFLT_CALLBACK_DATA_QUEUE_REMOVE_IO,
    pub PeekNextIo: PFLT_CALLBACK_DATA_QUEUE_PEEK_NEXT_IO,
    pub Acquire: PFLT_CALLBACK_DATA_QUEUE_ACQUIRE,
    pub Release: PFLT_CALLBACK_DATA_QUEUE_RELEASE,
    pub CompleteCanceledIo: PFLT_CALLBACK_DATA_QUEUE_COMPLETE_CANCELED_IO,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_CALLBACK_DATA_QUEUE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_CALLBACK_DATA_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug)]
pub struct FLT_CONTEXT_REGISTRATION {
    pub ContextType: u16,
    pub Flags: u16,
    pub ContextCleanupCallback: PFLT_CONTEXT_CLEANUP_CALLBACK,
    pub Size: usize,
    pub PoolTag: u32,
    pub ContextAllocateCallback: PFLT_CONTEXT_ALLOCATE_CALLBACK,
    pub ContextFreeCallback: PFLT_CONTEXT_FREE_CALLBACK,
    pub Reserved1: *mut core::ffi::c_void,
}
#[cfg(feature = "Wdk_Foundation")]
impl windows_core::TypeKind for FLT_CONTEXT_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_CONTEXT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_CREATEFILE_TARGET_ECP_CONTEXT {
    pub Instance: PFLT_INSTANCE,
    pub Volume: PFLT_VOLUME,
    pub FileNameInformation: *mut FLT_FILE_NAME_INFORMATION,
    pub Flags: u16,
}
impl windows_core::TypeKind for FLT_CREATEFILE_TARGET_ECP_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_CREATEFILE_TARGET_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_FILE_NAME_INFORMATION {
    pub Size: u16,
    pub NamesParsed: u16,
    pub Format: u32,
    pub Name: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Volume: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Share: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Extension: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Stream: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub FinalComponent: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ParentDir: super::super::super::super::Win32::Foundation::UNICODE_STRING,
}
impl windows_core::TypeKind for FLT_FILE_NAME_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_FILE_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_IO_PARAMETER_BLOCK {
    pub IrpFlags: u32,
    pub MajorFunction: u8,
    pub MinorFunction: u8,
    pub OperationFlags: u8,
    pub Reserved: u8,
    pub TargetFileObject: *mut super::super::super::Foundation::FILE_OBJECT,
    pub TargetInstance: PFLT_INSTANCE,
    pub Parameters: FLT_PARAMETERS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_IO_PARAMETER_BLOCK {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_IO_PARAMETER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_NAME_CONTROL {
    pub Name: super::super::super::super::Win32::Foundation::UNICODE_STRING,
}
impl windows_core::TypeKind for FLT_NAME_CONTROL {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_NAME_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug)]
pub struct FLT_OPERATION_REGISTRATION {
    pub MajorFunction: u8,
    pub Flags: u32,
    pub PreOperation: PFLT_PRE_OPERATION_CALLBACK,
    pub PostOperation: PFLT_POST_OPERATION_CALLBACK,
    pub Reserved1: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_OPERATION_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_OPERATION_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS {
    pub Create: FLT_PARAMETERS_4,
    pub CreatePipe: FLT_PARAMETERS_3,
    pub CreateMailslot: FLT_PARAMETERS_2,
    pub Read: FLT_PARAMETERS_24,
    pub Write: FLT_PARAMETERS_32,
    pub QueryFileInformation: FLT_PARAMETERS_19,
    pub SetFileInformation: FLT_PARAMETERS_27,
    pub QueryEa: FLT_PARAMETERS_18,
    pub SetEa: FLT_PARAMETERS_26,
    pub QueryVolumeInformation: FLT_PARAMETERS_23,
    pub SetVolumeInformation: FLT_PARAMETERS_30,
    pub DirectoryControl: FLT_PARAMETERS_6,
    pub FileSystemControl: FLT_PARAMETERS_8,
    pub DeviceIoControl: FLT_PARAMETERS_5,
    pub LockControl: FLT_PARAMETERS_9,
    pub QuerySecurity: FLT_PARAMETERS_22,
    pub SetSecurity: FLT_PARAMETERS_29,
    pub WMI: FLT_PARAMETERS_31,
    pub QueryQuota: FLT_PARAMETERS_21,
    pub SetQuota: FLT_PARAMETERS_28,
    pub Pnp: FLT_PARAMETERS_16,
    pub AcquireForSectionSynchronization: FLT_PARAMETERS_1,
    pub AcquireForModifiedPageWriter: FLT_PARAMETERS_0,
    pub ReleaseForModifiedPageWriter: FLT_PARAMETERS_25,
    pub QueryOpen: FLT_PARAMETERS_20,
    pub FastIoCheckIfPossible: FLT_PARAMETERS_7,
    pub NetworkQueryOpen: FLT_PARAMETERS_14,
    pub MdlRead: FLT_PARAMETERS_11,
    pub MdlReadComplete: FLT_PARAMETERS_10,
    pub PrepareMdlWrite: FLT_PARAMETERS_17,
    pub MdlWriteComplete: FLT_PARAMETERS_12,
    pub MountVolume: FLT_PARAMETERS_13,
    pub Others: FLT_PARAMETERS_15,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_0 {
    pub EndingOffset: *mut i64,
    pub ResourceToRelease: *mut *mut super::super::super::Foundation::ERESOURCE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_1 {
    pub SyncType: super::FS_FILTER_SECTION_SYNC_TYPE,
    pub PageProtection: u32,
    pub OutputInformation: *mut super::FS_FILTER_SECTION_SYNC_OUTPUT,
    pub Flags: u32,
    pub AllocationAttributes: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_2 {
    pub SecurityContext: *mut super::super::super::Foundation::IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_3 {
    pub SecurityContext: *mut super::super::super::Foundation::IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_4 {
    pub SecurityContext: *mut super::super::super::Foundation::IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub FileAttributes: u16,
    pub ShareAccess: u16,
    pub EaLength: u32,
    pub EaBuffer: *mut core::ffi::c_void,
    pub AllocationSize: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_5 {
    pub Common: FLT_PARAMETERS_5_1,
    pub Neither: FLT_PARAMETERS_5_4,
    pub Buffered: FLT_PARAMETERS_5_0,
    pub Direct: FLT_PARAMETERS_5_2,
    pub FastIo: FLT_PARAMETERS_5_3,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_5 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_5_0 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub SystemBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_5_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_5_1 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_5_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_5_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_5_2 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub InputSystemBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputMdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_5_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_5_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_5_3 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub InputBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_5_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_5_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_5_4 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub InputBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputMdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_5_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_5_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_6 {
    pub QueryDirectory: FLT_PARAMETERS_6_2,
    pub NotifyDirectory: FLT_PARAMETERS_6_1,
    pub NotifyDirectoryEx: FLT_PARAMETERS_6_0,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_6 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_6_0 {
    pub Length: u32,
    pub CompletionFilter: u32,
    pub DirectoryNotifyInformationClass: super::super::super::System::SystemServices::DIRECTORY_NOTIFY_INFORMATION_CLASS,
    pub Spare2: u32,
    pub DirectoryBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_6_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_6_1 {
    pub Length: u32,
    pub CompletionFilter: u32,
    pub Spare1: u32,
    pub Spare2: u32,
    pub DirectoryBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_6_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_6_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_6_2 {
    pub Length: u32,
    pub FileName: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
    pub FileIndex: u32,
    pub DirectoryBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_6_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_6_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_7 {
    pub FileOffset: i64,
    pub Length: u32,
    pub LockKey: u32,
    pub CheckForReadOperation: super::super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_7 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_8 {
    pub VerifyVolume: FLT_PARAMETERS_8_4,
    pub Common: FLT_PARAMETERS_8_1,
    pub Neither: FLT_PARAMETERS_8_3,
    pub Buffered: FLT_PARAMETERS_8_0,
    pub Direct: FLT_PARAMETERS_8_2,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_8 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_8_0 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub SystemBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_8_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_8_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_8_1 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_8_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_8_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_8_2 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub InputSystemBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputMdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_8_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_8_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_8_3 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub InputBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputMdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_8_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_8_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_8_4 {
    pub Vpb: *mut super::super::super::Foundation::VPB,
    pub DeviceObject: *mut super::super::super::Foundation::DEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_8_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_8_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_9 {
    pub Length: *mut i64,
    pub Key: u32,
    pub ByteOffset: i64,
    pub ProcessId: super::super::super::Foundation::PEPROCESS,
    pub FailImmediately: super::super::super::super::Win32::Foundation::BOOLEAN,
    pub ExclusiveLock: super::super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_9 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_10 {
    pub MdlChain: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_10 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_11 {
    pub FileOffset: i64,
    pub Length: u32,
    pub Key: u32,
    pub MdlChain: *mut *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_11 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_12 {
    pub FileOffset: i64,
    pub MdlChain: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_12 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_13 {
    pub DeviceType: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_13 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_14 {
    pub Irp: *mut super::super::super::Foundation::IRP,
    pub NetworkInformation: *mut super::FILE_NETWORK_OPEN_INFORMATION,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_14 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_15 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
    pub Argument5: *mut core::ffi::c_void,
    pub Argument6: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_15 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_16 {
    pub StartDevice: FLT_PARAMETERS_16_8,
    pub QueryDeviceRelations: FLT_PARAMETERS_16_2,
    pub QueryInterface: FLT_PARAMETERS_16_5,
    pub DeviceCapabilities: FLT_PARAMETERS_16_0,
    pub FilterResourceRequirements: FLT_PARAMETERS_16_1,
    pub ReadWriteConfig: FLT_PARAMETERS_16_6,
    pub SetLock: FLT_PARAMETERS_16_7,
    pub QueryId: FLT_PARAMETERS_16_4,
    pub QueryDeviceText: FLT_PARAMETERS_16_3,
    pub UsageNotification: FLT_PARAMETERS_16_9,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_0 {
    pub Capabilities: *mut super::super::super::System::SystemServices::DEVICE_CAPABILITIES,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_1 {
    pub IoResourceRequirementList: *mut super::super::super::System::SystemServices::IO_RESOURCE_REQUIREMENTS_LIST,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_2 {
    pub Type: super::super::super::System::SystemServices::DEVICE_RELATION_TYPE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_3 {
    pub DeviceTextType: super::super::super::System::SystemServices::DEVICE_TEXT_TYPE,
    pub LocaleId: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_4 {
    pub IdType: super::super::super::System::SystemServices::BUS_QUERY_ID_TYPE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_5 {
    pub InterfaceType: *const windows_core::GUID,
    pub Size: u16,
    pub Version: u16,
    pub Interface: *mut super::super::super::System::SystemServices::INTERFACE,
    pub InterfaceSpecificData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_5 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_6 {
    pub WhichSpace: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: u32,
    pub Length: u32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_6 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_7 {
    pub Lock: super::super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_7 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_8 {
    pub AllocatedResources: *mut super::super::super::System::SystemServices::CM_RESOURCE_LIST,
    pub AllocatedResourcesTranslated: *mut super::super::super::System::SystemServices::CM_RESOURCE_LIST,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_8 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_16_9 {
    pub InPath: super::super::super::super::Win32::Foundation::BOOLEAN,
    pub Reserved: [super::super::super::super::Win32::Foundation::BOOLEAN; 3],
    pub Type: super::super::super::System::SystemServices::DEVICE_USAGE_NOTIFICATION_TYPE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_16_9 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_16_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_17 {
    pub FileOffset: i64,
    pub Length: u32,
    pub Key: u32,
    pub MdlChain: *mut *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_17 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_17 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_18 {
    pub Length: u32,
    pub EaList: *mut core::ffi::c_void,
    pub EaListLength: u32,
    pub EaIndex: u32,
    pub EaBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_18 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_19 {
    pub Length: u32,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
    pub InfoBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_19 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_20 {
    pub Irp: *mut super::super::super::Foundation::IRP,
    pub FileInformation: *mut core::ffi::c_void,
    pub Length: *mut u32,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_20 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_21 {
    pub Length: u32,
    pub StartSid: super::super::super::super::Win32::Security::PSID,
    pub SidList: *mut super::FILE_GET_QUOTA_INFORMATION,
    pub SidListLength: u32,
    pub QuotaBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_21 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_22 {
    pub SecurityInformation: u32,
    pub Length: u32,
    pub SecurityBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_22 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_23 {
    pub Length: u32,
    pub FsInformationClass: super::FS_INFORMATION_CLASS,
    pub VolumeBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_23 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_23 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_24 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
    pub ReadBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_24 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_24 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_25 {
    pub ResourceToRelease: *mut super::super::super::Foundation::ERESOURCE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_25 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_25 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_26 {
    pub Length: u32,
    pub EaBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_26 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_26 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_27 {
    pub Length: u32,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
    pub ParentOfTarget: *mut super::super::super::Foundation::FILE_OBJECT,
    pub Anonymous: FLT_PARAMETERS_27_0,
    pub InfoBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_27 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_27 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_27_0 {
    pub Anonymous: FLT_PARAMETERS_27_0_0,
    pub ClusterCount: u32,
    pub DeleteHandle: super::super::super::super::Win32::Foundation::HANDLE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_27_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_27_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_27_0_0 {
    pub ReplaceIfExists: super::super::super::super::Win32::Foundation::BOOLEAN,
    pub AdvanceOnly: super::super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_27_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_27_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_28 {
    pub Length: u32,
    pub QuotaBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_28 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_28 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_29 {
    pub SecurityInformation: u32,
    pub SecurityDescriptor: super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_29 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_29 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_30 {
    pub Length: u32,
    pub FsInformationClass: super::FS_INFORMATION_CLASS,
    pub VolumeBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_30 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_30 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_PARAMETERS_31 {
    pub ProviderId: usize,
    pub DataPath: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_31 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_31 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_32 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
    pub WriteBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_PARAMETERS_32 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug)]
pub struct FLT_REGISTRATION {
    pub Size: u16,
    pub Version: u16,
    pub Flags: u32,
    pub ContextRegistration: *const FLT_CONTEXT_REGISTRATION,
    pub OperationRegistration: *const FLT_OPERATION_REGISTRATION,
    pub FilterUnloadCallback: PFLT_FILTER_UNLOAD_CALLBACK,
    pub InstanceSetupCallback: PFLT_INSTANCE_SETUP_CALLBACK,
    pub InstanceQueryTeardownCallback: PFLT_INSTANCE_QUERY_TEARDOWN_CALLBACK,
    pub InstanceTeardownStartCallback: PFLT_INSTANCE_TEARDOWN_CALLBACK,
    pub InstanceTeardownCompleteCallback: PFLT_INSTANCE_TEARDOWN_CALLBACK,
    pub GenerateFileNameCallback: PFLT_GENERATE_FILE_NAME,
    pub NormalizeNameComponentCallback: PFLT_NORMALIZE_NAME_COMPONENT,
    pub NormalizeContextCleanupCallback: PFLT_NORMALIZE_CONTEXT_CLEANUP,
    pub TransactionNotificationCallback: PFLT_TRANSACTION_NOTIFICATION_CALLBACK,
    pub NormalizeNameComponentExCallback: PFLT_NORMALIZE_NAME_COMPONENT_EX,
    pub SectionNotificationCallback: PFLT_SECTION_CONFLICT_NOTIFICATION_CALLBACK,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_RELATED_CONTEXTS {
    pub VolumeContext: PFLT_CONTEXT,
    pub InstanceContext: PFLT_CONTEXT,
    pub FileContext: PFLT_CONTEXT,
    pub StreamContext: PFLT_CONTEXT,
    pub StreamHandleContext: PFLT_CONTEXT,
    pub TransactionContext: PFLT_CONTEXT,
}
impl windows_core::TypeKind for FLT_RELATED_CONTEXTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_RELATED_CONTEXTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_RELATED_CONTEXTS_EX {
    pub VolumeContext: PFLT_CONTEXT,
    pub InstanceContext: PFLT_CONTEXT,
    pub FileContext: PFLT_CONTEXT,
    pub StreamContext: PFLT_CONTEXT,
    pub StreamHandleContext: PFLT_CONTEXT,
    pub TransactionContext: PFLT_CONTEXT,
    pub SectionContext: PFLT_CONTEXT,
}
impl windows_core::TypeKind for FLT_RELATED_CONTEXTS_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_RELATED_CONTEXTS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_RELATED_OBJECTS {
    pub Size: u16,
    pub TransactionContext: u16,
    pub Filter: PFLT_FILTER,
    pub Volume: PFLT_VOLUME,
    pub Instance: PFLT_INSTANCE,
    pub FileObject: *const super::super::super::Foundation::FILE_OBJECT,
    pub Transaction: *const super::super::super::Foundation::KTRANSACTION,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl windows_core::TypeKind for FLT_RELATED_OBJECTS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_RELATED_OBJECTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLT_TAG_DATA_BUFFER {
    pub FileTag: u32,
    pub TagDataLength: u16,
    pub UnparsedNameLength: u16,
    pub Anonymous: FLT_TAG_DATA_BUFFER_0,
}
impl windows_core::TypeKind for FLT_TAG_DATA_BUFFER {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_TAG_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FLT_TAG_DATA_BUFFER_0 {
    pub SymbolicLinkReparseBuffer: FLT_TAG_DATA_BUFFER_0_3,
    pub MountPointReparseBuffer: FLT_TAG_DATA_BUFFER_0_2,
    pub GenericReparseBuffer: FLT_TAG_DATA_BUFFER_0_1,
    pub GenericGUIDReparseBuffer: FLT_TAG_DATA_BUFFER_0_0,
}
impl windows_core::TypeKind for FLT_TAG_DATA_BUFFER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_TAG_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_TAG_DATA_BUFFER_0_0 {
    pub TagGuid: windows_core::GUID,
    pub DataBuffer: [u8; 1],
}
impl windows_core::TypeKind for FLT_TAG_DATA_BUFFER_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_TAG_DATA_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_TAG_DATA_BUFFER_0_1 {
    pub DataBuffer: [u8; 1],
}
impl windows_core::TypeKind for FLT_TAG_DATA_BUFFER_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_TAG_DATA_BUFFER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_TAG_DATA_BUFFER_0_2 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub PathBuffer: [u16; 1],
}
impl windows_core::TypeKind for FLT_TAG_DATA_BUFFER_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_TAG_DATA_BUFFER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_TAG_DATA_BUFFER_0_3 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub Flags: u32,
    pub PathBuffer: [u16; 1],
}
impl windows_core::TypeKind for FLT_TAG_DATA_BUFFER_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_TAG_DATA_BUFFER_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLT_VOLUME_PROPERTIES {
    pub DeviceType: u32,
    pub DeviceCharacteristics: u32,
    pub DeviceObjectFlags: u32,
    pub AlignmentRequirement: u32,
    pub SectorSize: u16,
    pub Flags: u16,
    pub FileSystemDriverName: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub FileSystemDeviceName: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub RealDeviceName: super::super::super::super::Win32::Foundation::UNICODE_STRING,
}
impl windows_core::TypeKind for FLT_VOLUME_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLT_VOLUME_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLT_CONTEXT(pub *mut core::ffi::c_void);
impl PFLT_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFLT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PFLT_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLT_DEFERRED_IO_WORKITEM(pub isize);
impl Default for PFLT_DEFERRED_IO_WORKITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PFLT_DEFERRED_IO_WORKITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLT_FILTER(pub isize);
impl Default for PFLT_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PFLT_FILTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLT_GENERIC_WORKITEM(pub isize);
impl Default for PFLT_GENERIC_WORKITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PFLT_GENERIC_WORKITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLT_INSTANCE(pub isize);
impl Default for PFLT_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PFLT_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLT_PORT(pub isize);
impl Default for PFLT_PORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PFLT_PORT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLT_VOLUME(pub isize);
impl Default for PFLT_VOLUME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PFLT_VOLUME {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE = Option<unsafe extern "system" fn(callbackdata: *const FLT_CALLBACK_DATA, context: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLTOPLOCK_WAIT_COMPLETE_ROUTINE = Option<unsafe extern "system" fn(callbackdata: *const FLT_CALLBACK_DATA, context: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_ACQUIRE = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, irql: *mut u8)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_COMPLETE_CANCELED_IO = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *mut FLT_CALLBACK_DATA)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_INSERT_IO = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *const FLT_CALLBACK_DATA, insertcontext: *const core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_PEEK_NEXT_IO = Option<unsafe extern "system" fn(cbdq: *const FLT_CALLBACK_DATA_QUEUE, cbd: *const FLT_CALLBACK_DATA, peekcontext: *const core::ffi::c_void) -> *mut FLT_CALLBACK_DATA>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_RELEASE = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, irql: u8)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_REMOVE_IO = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *const FLT_CALLBACK_DATA)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_COMPLETED_ASYNC_IO_CALLBACK = Option<unsafe extern "system" fn(callbackdata: *const FLT_CALLBACK_DATA, context: PFLT_CONTEXT)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_COMPLETE_CANCELED_CALLBACK = Option<unsafe extern "system" fn(callbackdata: *const FLT_CALLBACK_DATA)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_COMPLETE_LOCK_CALLBACK_DATA_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFLT_CONNECT_NOTIFY = Option<unsafe extern "system" fn(clientport: PFLT_PORT, serverportcookie: *const core::ffi::c_void, connectioncontext: *const core::ffi::c_void, sizeofcontext: u32, connectionportcookie: *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type PFLT_CONTEXT_ALLOCATE_CALLBACK = Option<unsafe extern "system" fn(pooltype: super::super::super::Foundation::POOL_TYPE, size: usize, contexttype: u16) -> *mut core::ffi::c_void>;
pub type PFLT_CONTEXT_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(context: PFLT_CONTEXT, contexttype: u16)>;
pub type PFLT_CONTEXT_FREE_CALLBACK = Option<unsafe extern "system" fn(pool: *const core::ffi::c_void, contexttype: u16)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_DEFERRED_IO_WORKITEM_ROUTINE = Option<unsafe extern "system" fn(fltworkitem: PFLT_DEFERRED_IO_WORKITEM, callbackdata: *const FLT_CALLBACK_DATA, context: *const core::ffi::c_void)>;
pub type PFLT_DISCONNECT_NOTIFY = Option<unsafe extern "system" fn(connectioncookie: *const core::ffi::c_void)>;
pub type PFLT_FILTER_UNLOAD_CALLBACK = Option<unsafe extern "system" fn(flags: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_GENERATE_FILE_NAME = Option<unsafe extern "system" fn(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, callbackdata: *const FLT_CALLBACK_DATA, nameoptions: u32, cachefilenameinformation: *mut super::super::super::super::Win32::Foundation::BOOLEAN, filename: *mut FLT_NAME_CONTROL) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFLT_GENERIC_WORKITEM_ROUTINE = Option<unsafe extern "system" fn(fltworkitem: PFLT_GENERIC_WORKITEM, fltobject: *const core::ffi::c_void, context: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_GET_OPERATION_STATUS_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *const FLT_RELATED_OBJECTS, iopbsnapshot: *const FLT_IO_PARAMETER_BLOCK, operationstatus: super::super::super::super::Win32::Foundation::NTSTATUS, requestercontext: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_INSTANCE_QUERY_TEARDOWN_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *const FLT_RELATED_OBJECTS, flags: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_INSTANCE_SETUP_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *const FLT_RELATED_OBJECTS, flags: u32, volumedevicetype: u32, volumefilesystemtype: super::super::super::super::Win32::Storage::InstallableFileSystems::FLT_FILESYSTEM_TYPE) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_INSTANCE_TEARDOWN_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *const FLT_RELATED_OBJECTS, reason: u32)>;
pub type PFLT_MESSAGE_NOTIFY = Option<unsafe extern "system" fn(portcookie: *const core::ffi::c_void, inputbuffer: *const core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut core::ffi::c_void, outputbufferlength: u32, returnoutputbufferlength: *mut u32) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFLT_NORMALIZE_CONTEXT_CLEANUP = Option<unsafe extern "system" fn(normalizationcontext: *const *const core::ffi::c_void)>;
pub type PFLT_NORMALIZE_NAME_COMPONENT = Option<unsafe extern "system" fn(instance: PFLT_INSTANCE, parentdirectory: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, volumenamelength: u16, component: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, expandcomponentname: *mut super::FILE_NAMES_INFORMATION, expandcomponentnamelength: u32, flags: u32, normalizationcontext: *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_NORMALIZE_NAME_COMPONENT_EX = Option<unsafe extern "system" fn(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, parentdirectory: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, volumenamelength: u16, component: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, expandcomponentname: *mut super::FILE_NAMES_INFORMATION, expandcomponentnamelength: u32, flags: u32, normalizationcontext: *mut *mut core::ffi::c_void) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_POST_OPERATION_CALLBACK = Option<unsafe extern "system" fn(data: *mut FLT_CALLBACK_DATA, fltobjects: *const FLT_RELATED_OBJECTS, completioncontext: *const core::ffi::c_void, flags: u32) -> FLT_POSTOP_CALLBACK_STATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_PRE_OPERATION_CALLBACK = Option<unsafe extern "system" fn(data: *mut FLT_CALLBACK_DATA, fltobjects: *const FLT_RELATED_OBJECTS, completioncontext: *mut *mut core::ffi::c_void) -> FLT_PREOP_CALLBACK_STATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_SECTION_CONFLICT_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(instance: PFLT_INSTANCE, sectioncontext: PFLT_CONTEXT, data: *const FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_TRANSACTION_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *const FLT_RELATED_OBJECTS, transactioncontext: PFLT_CONTEXT, notificationmask: u32) -> super::super::super::super::Win32::Foundation::NTSTATUS>;
