#[inline]
pub unsafe fn DceErrorInqTextA(rpcstatus: RPC_STATUS, errortext: &mut [u8; 256]) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn DceErrorInqTextA(rpcstatus : RPC_STATUS, errortext : windows_core::PSTR) -> RPC_STATUS);
    unsafe { DceErrorInqTextA(rpcstatus, core::mem::transmute(errortext.as_ptr())) }
}
#[inline]
pub unsafe fn DceErrorInqTextW(rpcstatus: RPC_STATUS, errortext: &mut [u16; 256]) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn DceErrorInqTextW(rpcstatus : RPC_STATUS, errortext : windows_core::PWSTR) -> RPC_STATUS);
    unsafe { DceErrorInqTextW(rpcstatus, core::mem::transmute(errortext.as_ptr())) }
}
#[inline]
pub unsafe fn IUnknown_AddRef_Proxy<P0>(this: P0) -> u32
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rpcrt4.dll" "system" fn IUnknown_AddRef_Proxy(this : * mut core::ffi::c_void) -> u32);
    unsafe { IUnknown_AddRef_Proxy(this.param().abi()) }
}
#[inline]
pub unsafe fn IUnknown_QueryInterface_Proxy<P0>(this: P0, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rpcrt4.dll" "system" fn IUnknown_QueryInterface_Proxy(this : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { IUnknown_QueryInterface_Proxy(this.param().abi(), riid, ppvobject as _).ok() }
}
#[inline]
pub unsafe fn IUnknown_Release_Proxy<P0>(this: P0) -> u32
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("rpcrt4.dll" "system" fn IUnknown_Release_Proxy(this : * mut core::ffi::c_void) -> u32);
    unsafe { IUnknown_Release_Proxy(this.param().abi()) }
}
#[inline]
pub unsafe fn I_RpcAllocate(size: u32) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcAllocate(size : u32) -> *mut core::ffi::c_void);
    unsafe { I_RpcAllocate(size) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn I_RpcAsyncAbortCall(pasync: *const RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcAsyncAbortCall(pasync : *const RPC_ASYNC_STATE, exceptioncode : u32) -> RPC_STATUS);
    unsafe { I_RpcAsyncAbortCall(pasync, exceptioncode) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn I_RpcAsyncSetHandle(message: *const RPC_MESSAGE, pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcAsyncSetHandle(message : *const RPC_MESSAGE, pasync : *const RPC_ASYNC_STATE) -> RPC_STATUS);
    unsafe { I_RpcAsyncSetHandle(message, pasync) }
}
#[inline]
pub unsafe fn I_RpcBindingCopy(sourcebinding: *mut core::ffi::c_void, destinationbinding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingCopy(sourcebinding : *mut core::ffi::c_void, destinationbinding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcBindingCopy(sourcebinding as _, destinationbinding as _) }
}
#[inline]
pub unsafe fn I_RpcBindingCreateNP<P0, P1, P2>(servername: P0, servicename: P1, networkoptions: P2, binding: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingCreateNP(servername : windows_core::PCWSTR, servicename : windows_core::PCWSTR, networkoptions : windows_core::PCWSTR, binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcBindingCreateNP(servername.param().abi(), servicename.param().abi(), networkoptions.param().abi(), binding as _) }
}
#[inline]
pub unsafe fn I_RpcBindingHandleToAsyncHandle(binding: *mut core::ffi::c_void, asynchandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingHandleToAsyncHandle(binding : *mut core::ffi::c_void, asynchandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcBindingHandleToAsyncHandle(binding as _, asynchandle as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqClientTokenAttributes(binding: *const core::ffi::c_void, tokenid: Option<*mut super::super::Foundation::LUID>, authenticationid: Option<*mut super::super::Foundation::LUID>, modifiedid: Option<*mut super::super::Foundation::LUID>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqClientTokenAttributes(binding : *const core::ffi::c_void, tokenid : *mut super::super::Foundation:: LUID, authenticationid : *mut super::super::Foundation:: LUID, modifiedid : *mut super::super::Foundation:: LUID) -> RPC_STATUS);
    unsafe { I_RpcBindingInqClientTokenAttributes(binding, tokenid.unwrap_or(core::mem::zeroed()) as _, authenticationid.unwrap_or(core::mem::zeroed()) as _, modifiedid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointA(binding: *const core::ffi::c_void, dynamicendpoint: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqDynamicEndpointA(binding : *const core::ffi::c_void, dynamicendpoint : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { I_RpcBindingInqDynamicEndpointA(binding, dynamicendpoint as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointW(binding: *const core::ffi::c_void, dynamicendpoint: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqDynamicEndpointW(binding : *const core::ffi::c_void, dynamicendpoint : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { I_RpcBindingInqDynamicEndpointW(binding, dynamicendpoint as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqLocalClientPID(binding: *mut core::ffi::c_void, pid: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqLocalClientPID(binding : *mut core::ffi::c_void, pid : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcBindingInqLocalClientPID(binding as _, pid as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqMarshalledTargetInfo(binding: *const core::ffi::c_void, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqMarshalledTargetInfo(binding : *const core::ffi::c_void, marshalledtargetinfosize : *mut u32, marshalledtargetinfo : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { I_RpcBindingInqMarshalledTargetInfo(binding, marshalledtargetinfosize as _, marshalledtargetinfo as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContext(binding: *mut core::ffi::c_void, securitycontexthandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqSecurityContext(binding : *mut core::ffi::c_void, securitycontexthandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcBindingInqSecurityContext(binding as _, securitycontexthandle as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContextKeyInfo(binding: Option<*const core::ffi::c_void>, keyinfo: *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqSecurityContextKeyInfo(binding : *const core::ffi::c_void, keyinfo : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcBindingInqSecurityContextKeyInfo(binding.unwrap_or(core::mem::zeroed()) as _, keyinfo as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqTransportType(binding: *mut core::ffi::c_void, r#type: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqTransportType(binding : *mut core::ffi::c_void, r#type : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcBindingInqTransportType(binding as _, r#type as _) }
}
#[inline]
pub unsafe fn I_RpcBindingInqWireIdForSnego(binding: *const core::ffi::c_void, wireid: *mut u8) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingInqWireIdForSnego(binding : *const core::ffi::c_void, wireid : *mut u8) -> RPC_STATUS);
    unsafe { I_RpcBindingInqWireIdForSnego(binding, wireid as _) }
}
#[inline]
pub unsafe fn I_RpcBindingIsClientLocal(bindinghandle: *mut core::ffi::c_void, clientlocalflag: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingIsClientLocal(bindinghandle : *mut core::ffi::c_void, clientlocalflag : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcBindingIsClientLocal(bindinghandle as _, clientlocalflag as _) }
}
#[inline]
pub unsafe fn I_RpcBindingIsServerLocal(binding: *const core::ffi::c_void, serverlocalflag: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingIsServerLocal(binding : *const core::ffi::c_void, serverlocalflag : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcBindingIsServerLocal(binding, serverlocalflag as _) }
}
#[inline]
pub unsafe fn I_RpcBindingSetPrivateOption(hbinding: *const core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingSetPrivateOption(hbinding : *const core::ffi::c_void, option : u32, optionvalue : usize) -> RPC_STATUS);
    unsafe { I_RpcBindingSetPrivateOption(hbinding, option, optionvalue) }
}
#[inline]
pub unsafe fn I_RpcBindingToStaticStringBindingW(binding: *mut core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcBindingToStaticStringBindingW(binding : *mut core::ffi::c_void, stringbinding : *mut *mut u16) -> RPC_STATUS);
    unsafe { I_RpcBindingToStaticStringBindingW(binding as _, stringbinding as _) }
}
#[inline]
pub unsafe fn I_RpcClearMutex(mutex: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcClearMutex(mutex : *mut core::ffi::c_void));
    unsafe { I_RpcClearMutex(mutex as _) }
}
#[inline]
pub unsafe fn I_RpcDeleteMutex(mutex: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcDeleteMutex(mutex : *mut core::ffi::c_void));
    unsafe { I_RpcDeleteMutex(mutex as _) }
}
#[inline]
pub unsafe fn I_RpcExceptionFilter(exceptioncode: u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcExceptionFilter(exceptioncode : u32) -> i32);
    unsafe { I_RpcExceptionFilter(exceptioncode) }
}
#[inline]
pub unsafe fn I_RpcFree(object: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFree(object : *mut core::ffi::c_void));
    unsafe { I_RpcFree(object as _) }
}
#[inline]
pub unsafe fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFreeBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcFreeBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcFreePipeBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcFreePipeBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcGetBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetBufferWithObject(message : *mut RPC_MESSAGE, objectuuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { I_RpcGetBufferWithObject(message as _, objectuuid as _) }
}
#[inline]
pub unsafe fn I_RpcGetCurrentCallHandle() -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetCurrentCallHandle() -> *mut core::ffi::c_void);
    unsafe { I_RpcGetCurrentCallHandle() }
}
#[inline]
pub unsafe fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetDefaultSD(ppsecuritydescriptor : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcGetDefaultSD(ppsecuritydescriptor as _) }
}
#[inline]
pub unsafe fn I_RpcGetExtendedError() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcGetExtendedError() -> RPC_STATUS);
    unsafe { I_RpcGetExtendedError() }
}
#[inline]
pub unsafe fn I_RpcIfInqTransferSyntaxes(rpcifhandle: *mut core::ffi::c_void, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcIfInqTransferSyntaxes(rpcifhandle : *mut core::ffi::c_void, transfersyntaxes : *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize : u32, transfersyntaxcount : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcIfInqTransferSyntaxes(rpcifhandle as _, transfersyntaxes as _, transfersyntaxsize, transfersyntaxcount as _) }
}
#[inline]
pub unsafe fn I_RpcMapWin32Status(status: RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcMapWin32Status(status : RPC_STATUS) -> i32);
    unsafe { I_RpcMapWin32Status(status) }
}
#[inline]
pub unsafe fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS);
    unsafe { I_RpcMgmtEnableDedicatedThreadPool() }
}
#[inline]
pub unsafe fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNegotiateTransferSyntax(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcNegotiateTransferSyntax(message as _) }
}
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameA<P2>(binding: *const core::ffi::c_void, entrynamesyntax: u32, entryname: P2) -> RPC_STATUS
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsBindingSetEntryNameA(binding : *const core::ffi::c_void, entrynamesyntax : u32, entryname : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { I_RpcNsBindingSetEntryNameA(binding, entrynamesyntax, entryname.param().abi()) }
}
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameW<P2>(binding: *const core::ffi::c_void, entrynamesyntax: u32, entryname: P2) -> RPC_STATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsBindingSetEntryNameW(binding : *const core::ffi::c_void, entrynamesyntax : u32, entryname : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { I_RpcNsBindingSetEntryNameW(binding, entrynamesyntax, entryname.param().abi()) }
}
#[inline]
pub unsafe fn I_RpcNsGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsGetBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcNsGetBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *const u16, rpcinterfaceinformation: *const RPC_SERVER_INTERFACE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsInterfaceExported(entrynamesyntax : u32, entryname : *const u16, rpcinterfaceinformation : *const RPC_SERVER_INTERFACE) -> RPC_STATUS);
    unsafe { I_RpcNsInterfaceExported(entrynamesyntax, entryname, rpcinterfaceinformation) }
}
#[inline]
pub unsafe fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcNsInterfaceUnexported(entrynamesyntax : u32, entryname : *mut u16, rpcinterfaceinformation : *mut RPC_SERVER_INTERFACE) -> RPC_STATUS);
    unsafe { I_RpcNsInterfaceUnexported(entrynamesyntax, entryname as _, rpcinterfaceinformation as _) }
}
#[inline]
pub unsafe fn I_RpcNsRaiseException(message: *mut RPC_MESSAGE, status: RPC_STATUS) {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsRaiseException(message : *mut RPC_MESSAGE, status : RPC_STATUS));
    unsafe { I_RpcNsRaiseException(message as _, status) }
}
#[inline]
pub unsafe fn I_RpcNsSendReceive(message: *mut RPC_MESSAGE, handle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsSendReceive(message : *mut RPC_MESSAGE, handle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcNsSendReceive(message as _, handle as _) }
}
#[inline]
pub unsafe fn I_RpcOpenClientProcess(binding: Option<*const core::ffi::c_void>, desiredaccess: u32, clientprocess: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcOpenClientProcess(binding : *const core::ffi::c_void, desiredaccess : u32, clientprocess : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcOpenClientProcess(binding.unwrap_or(core::mem::zeroed()) as _, desiredaccess, clientprocess as _) }
}
#[inline]
pub unsafe fn I_RpcPauseExecution(milliseconds: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcPauseExecution(milliseconds : u32));
    unsafe { I_RpcPauseExecution(milliseconds) }
}
#[inline]
pub unsafe fn I_RpcReBindBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcReBindBuffer(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcReBindBuffer(message as _) }
}
#[inline]
pub unsafe fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcReallocPipeBuffer(message : *const RPC_MESSAGE, newsize : u32) -> RPC_STATUS);
    unsafe { I_RpcReallocPipeBuffer(message, newsize) }
}
#[inline]
pub unsafe fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcReceive(message : *mut RPC_MESSAGE, size : u32) -> RPC_STATUS);
    unsafe { I_RpcReceive(message as _, size) }
}
#[inline]
pub unsafe fn I_RpcRecordCalloutFailure(rpcstatus: RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcRecordCalloutFailure(rpcstatus : RPC_STATUS, calloutstate : *mut RDR_CALLOUT_STATE, dllname : *mut u16));
    unsafe { I_RpcRecordCalloutFailure(rpcstatus, calloutstate as _, dllname as _) }
}
#[inline]
pub unsafe fn I_RpcRequestMutex(mutex: *mut *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcRequestMutex(mutex : *mut *mut core::ffi::c_void));
    unsafe { I_RpcRequestMutex(mutex as _) }
}
#[inline]
pub unsafe fn I_RpcSend(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSend(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcSend(message as _) }
}
#[inline]
pub unsafe fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSendReceive(message : *mut RPC_MESSAGE) -> RPC_STATUS);
    unsafe { I_RpcSendReceive(message as _) }
}
#[inline]
pub unsafe fn I_RpcServerCheckClientRestriction(context: *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerCheckClientRestriction(context : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerCheckClientRestriction(context as _) }
}
#[inline]
pub unsafe fn I_RpcServerDisableExceptionFilter() -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerDisableExceptionFilter() -> i32);
    unsafe { I_RpcServerDisableExceptionFilter() }
}
#[inline]
pub unsafe fn I_RpcServerGetAssociationID(binding: Option<*const core::ffi::c_void>, associationid: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerGetAssociationID(binding : *const core::ffi::c_void, associationid : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcServerGetAssociationID(binding.unwrap_or(core::mem::zeroed()) as _, associationid as _) }
}
#[inline]
pub unsafe fn I_RpcServerInqAddressChangeFn() -> *mut RPC_ADDRESS_CHANGE_FN {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqAddressChangeFn() -> *mut RPC_ADDRESS_CHANGE_FN);
    unsafe { I_RpcServerInqAddressChangeFn() }
}
#[inline]
pub unsafe fn I_RpcServerInqLocalConnAddress(binding: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqLocalConnAddress(binding : *mut core::ffi::c_void, buffer : *mut core::ffi::c_void, buffersize : *mut u32, addressformat : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcServerInqLocalConnAddress(binding as _, buffer as _, buffersize as _, addressformat as _) }
}
#[inline]
pub unsafe fn I_RpcServerInqRemoteConnAddress(binding: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqRemoteConnAddress(binding : *mut core::ffi::c_void, buffer : *mut core::ffi::c_void, buffersize : *mut u32, addressformat : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcServerInqRemoteConnAddress(binding as _, buffer as _, buffersize as _, addressformat as _) }
}
#[inline]
pub unsafe fn I_RpcServerInqTransportType(r#type: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerInqTransportType(r#type : *mut u32) -> RPC_STATUS);
    unsafe { I_RpcServerInqTransportType(r#type as _) }
}
#[inline]
pub unsafe fn I_RpcServerRegisterForwardFunction(pforwardfunction: *mut RPC_FORWARD_FUNCTION) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerRegisterForwardFunction(pforwardfunction : *mut RPC_FORWARD_FUNCTION) -> RPC_STATUS);
    unsafe { I_RpcServerRegisterForwardFunction(pforwardfunction as _) }
}
#[inline]
pub unsafe fn I_RpcServerSetAddressChangeFn(paddresschangefn: *mut RPC_ADDRESS_CHANGE_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSetAddressChangeFn(paddresschangefn : *mut RPC_ADDRESS_CHANGE_FN) -> RPC_STATUS);
    unsafe { I_RpcServerSetAddressChangeFn(paddresschangefn as _) }
}
#[inline]
pub unsafe fn I_RpcServerStartService<P0, P1>(protseq: P0, endpoint: P1, ifspec: *const core::ffi::c_void) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerStartService(protseq : windows_core::PCWSTR, endpoint : windows_core::PCWSTR, ifspec : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerStartService(protseq.param().abi(), endpoint.param().abi(), ifspec) }
}
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification(binding: Option<*const core::ffi::c_void>, hevent: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSubscribeForDisconnectNotification(binding : *const core::ffi::c_void, hevent : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerSubscribeForDisconnectNotification(binding.unwrap_or(core::mem::zeroed()) as _, hevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification2(binding: Option<*const core::ffi::c_void>, hevent: *const core::ffi::c_void, subscriptionid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerSubscribeForDisconnectNotification2(binding : *const core::ffi::c_void, hevent : *const core::ffi::c_void, subscriptionid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { I_RpcServerSubscribeForDisconnectNotification2(binding.unwrap_or(core::mem::zeroed()) as _, hevent, subscriptionid as _) }
}
#[inline]
pub unsafe fn I_RpcServerUnsubscribeForDisconnectNotification(binding: Option<*const core::ffi::c_void>, subscriptionid: windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUnsubscribeForDisconnectNotification(binding : *const core::ffi::c_void, subscriptionid : windows_core::GUID) -> RPC_STATUS);
    unsafe { I_RpcServerUnsubscribeForDisconnectNotification(binding.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(subscriptionid)) }
}
#[inline]
pub unsafe fn I_RpcServerUseProtseq2A<P0, P1>(networkaddress: P0, protseq: P1, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseq2A(networkaddress : windows_core::PCSTR, protseq : windows_core::PCSTR, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerUseProtseq2A(networkaddress.param().abi(), protseq.param().abi(), maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn I_RpcServerUseProtseq2W<P0, P1>(networkaddress: P0, protseq: P1, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseq2W(networkaddress : windows_core::PCWSTR, protseq : windows_core::PCWSTR, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerUseProtseq2W(networkaddress.param().abi(), protseq.param().abi(), maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2A<P0, P1, P3>(networkaddress: P0, protseq: P1, maxcalls: u32, endpoint: P3, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseqEp2A(networkaddress : windows_core::PCSTR, protseq : windows_core::PCSTR, maxcalls : u32, endpoint : windows_core::PCSTR, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerUseProtseqEp2A(networkaddress.param().abi(), protseq.param().abi(), maxcalls, endpoint.param().abi(), securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2W<P0, P1, P3>(networkaddress: P0, protseq: P1, maxcalls: u32, endpoint: P3, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const core::ffi::c_void) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcServerUseProtseqEp2W(networkaddress : windows_core::PCWSTR, protseq : windows_core::PCWSTR, maxcalls : u32, endpoint : windows_core::PCWSTR, securitydescriptor : *const core::ffi::c_void, policy : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { I_RpcServerUseProtseqEp2W(networkaddress.param().abi(), protseq.param().abi(), maxcalls, endpoint.param().abi(), securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn I_RpcSessionStrictContextHandle() {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSessionStrictContextHandle());
    unsafe { I_RpcSessionStrictContextHandle() }
}
#[inline]
pub unsafe fn I_RpcSsDontSerializeContext() {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSsDontSerializeContext());
    unsafe { I_RpcSsDontSerializeContext() }
}
#[inline]
pub unsafe fn I_RpcSystemHandleTypeSpecificWork(handle: *mut core::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcSystemHandleTypeSpecificWork(handle : *mut core::ffi::c_void, actualtype : u8, idltype : u8, marshaldirection : LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS);
    unsafe { I_RpcSystemHandleTypeSpecificWork(handle as _, actualtype, idltype, marshaldirection) }
}
#[inline]
pub unsafe fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS);
    unsafe { I_RpcTurnOnEEInfoPropagation() }
}
#[inline]
pub unsafe fn I_UuidCreate(uuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn I_UuidCreate(uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { I_UuidCreate(uuid as _) }
}
#[inline]
pub unsafe fn MesBufferHandleReset(handle: *const core::ffi::c_void, handlestyle: u32, operation: MIDL_ES_CODE, pbuffer: Option<&[u8]>, pencodedsize: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesBufferHandleReset(handle : *const core::ffi::c_void, handlestyle : u32, operation : MIDL_ES_CODE, pbuffer : *const *const i8, buffersize : u32, pencodedsize : *mut u32) -> RPC_STATUS);
    unsafe { MesBufferHandleReset(handle, handlestyle, operation, core::mem::transmute(pbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pencodedsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MesDecodeBufferHandleCreate(buffer: &[u8], phandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesDecodeBufferHandleCreate(buffer : windows_core::PCSTR, buffersize : u32, phandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { MesDecodeBufferHandleCreate(core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), phandle as _) }
}
#[inline]
pub unsafe fn MesDecodeIncrementalHandleCreate(userstate: *mut core::ffi::c_void, readfn: MIDL_ES_READ, phandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesDecodeIncrementalHandleCreate(userstate : *mut core::ffi::c_void, readfn : MIDL_ES_READ, phandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { MesDecodeIncrementalHandleCreate(userstate as _, readfn, phandle as _) }
}
#[inline]
pub unsafe fn MesEncodeDynBufferHandleCreate(pbuffer: *mut *mut i8, pencodedsize: *mut u32, phandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesEncodeDynBufferHandleCreate(pbuffer : *mut *mut i8, pencodedsize : *mut u32, phandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { MesEncodeDynBufferHandleCreate(pbuffer as _, pencodedsize as _, phandle as _) }
}
#[inline]
pub unsafe fn MesEncodeFixedBufferHandleCreate(pbuffer: &mut [u8], pencodedsize: *mut u32, phandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesEncodeFixedBufferHandleCreate(pbuffer : windows_core::PSTR, buffersize : u32, pencodedsize : *mut u32, phandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { MesEncodeFixedBufferHandleCreate(core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), pencodedsize as _, phandle as _) }
}
#[inline]
pub unsafe fn MesEncodeIncrementalHandleCreate(userstate: *mut core::ffi::c_void, allocfn: MIDL_ES_ALLOC, writefn: MIDL_ES_WRITE, phandle: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesEncodeIncrementalHandleCreate(userstate : *mut core::ffi::c_void, allocfn : MIDL_ES_ALLOC, writefn : MIDL_ES_WRITE, phandle : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { MesEncodeIncrementalHandleCreate(userstate as _, allocfn, writefn, phandle as _) }
}
#[inline]
pub unsafe fn MesHandleFree(handle: *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesHandleFree(handle : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { MesHandleFree(handle as _) }
}
#[inline]
pub unsafe fn MesIncrementalHandleReset(handle: *mut core::ffi::c_void, userstate: *mut core::ffi::c_void, allocfn: MIDL_ES_ALLOC, writefn: MIDL_ES_WRITE, readfn: MIDL_ES_READ, operation: MIDL_ES_CODE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesIncrementalHandleReset(handle : *mut core::ffi::c_void, userstate : *mut core::ffi::c_void, allocfn : MIDL_ES_ALLOC, writefn : MIDL_ES_WRITE, readfn : MIDL_ES_READ, operation : MIDL_ES_CODE) -> RPC_STATUS);
    unsafe { MesIncrementalHandleReset(handle as _, userstate as _, allocfn, writefn, readfn, operation) }
}
#[inline]
pub unsafe fn MesInqProcEncodingId(handle: *mut core::ffi::c_void, pinterfaceid: *mut RPC_SYNTAX_IDENTIFIER, pprocnum: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn MesInqProcEncodingId(handle : *mut core::ffi::c_void, pinterfaceid : *mut RPC_SYNTAX_IDENTIFIER, pprocnum : *mut u32) -> RPC_STATUS);
    unsafe { MesInqProcEncodingId(handle as _, pinterfaceid as _, pprocnum as _) }
}
#[inline]
pub unsafe fn NDRCContextBinding(ccontext: isize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextBinding(ccontext : isize) -> *mut core::ffi::c_void);
    unsafe { NDRCContextBinding(ccontext) }
}
#[inline]
pub unsafe fn NDRCContextMarshall(ccontext: Option<isize>, pbuff: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextMarshall(ccontext : isize, pbuff : *mut core::ffi::c_void));
    unsafe { NDRCContextMarshall(ccontext.unwrap_or(core::mem::zeroed()) as _, pbuff as _) }
}
#[inline]
pub unsafe fn NDRCContextUnmarshall(pccontext: Option<*mut isize>, hbinding: *const core::ffi::c_void, pbuff: *const core::ffi::c_void, datarepresentation: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextUnmarshall(pccontext : *mut isize, hbinding : *const core::ffi::c_void, pbuff : *const core::ffi::c_void, datarepresentation : u32));
    unsafe { NDRCContextUnmarshall(pccontext.unwrap_or(core::mem::zeroed()) as _, hbinding, pbuff, datarepresentation) }
}
#[inline]
pub unsafe fn NDRSContextMarshall(ccontext: *const NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshall(ccontext : *const NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN));
    unsafe { NDRSContextMarshall(ccontext, pbuff as _, userrundownin) }
}
#[inline]
pub unsafe fn NDRSContextMarshall2(bindinghandle: *const core::ffi::c_void, ccontext: *const NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN, ctxguard: Option<*const core::ffi::c_void>, flags: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshall2(bindinghandle : *const core::ffi::c_void, ccontext : *const NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN, ctxguard : *const core::ffi::c_void, flags : u32));
    unsafe { NDRSContextMarshall2(bindinghandle, ccontext, pbuff as _, userrundownin, ctxguard.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn NDRSContextMarshallEx(bindinghandle: *const core::ffi::c_void, ccontext: *const NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshallEx(bindinghandle : *const core::ffi::c_void, ccontext : *const NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN));
    unsafe { NDRSContextMarshallEx(bindinghandle, ccontext, pbuff as _, userrundownin) }
}
#[inline]
pub unsafe fn NDRSContextUnmarshall(pbuff: *const core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshall(pbuff : *const core::ffi::c_void, datarepresentation : u32) -> *mut NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshall(pbuff, datarepresentation) }
}
#[inline]
pub unsafe fn NDRSContextUnmarshall2(bindinghandle: *const core::ffi::c_void, pbuff: Option<*const core::ffi::c_void>, datarepresentation: u32, ctxguard: Option<*const core::ffi::c_void>, flags: u32) -> *mut NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshall2(bindinghandle : *const core::ffi::c_void, pbuff : *const core::ffi::c_void, datarepresentation : u32, ctxguard : *const core::ffi::c_void, flags : u32) -> *mut NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshall2(bindinghandle, pbuff.unwrap_or(core::mem::zeroed()) as _, datarepresentation, ctxguard.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn NDRSContextUnmarshallEx(bindinghandle: *const core::ffi::c_void, pbuff: *const core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshallEx(bindinghandle : *const core::ffi::c_void, pbuff : *const core::ffi::c_void, datarepresentation : u32) -> *mut NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshallEx(bindinghandle, pbuff, datarepresentation) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn Ndr64AsyncClientCall(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { Ndr64AsyncClientCall(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[inline]
pub unsafe fn Ndr64AsyncServerCall64(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64AsyncServerCall64(prpcmsg : *mut RPC_MESSAGE));
    unsafe { Ndr64AsyncServerCall64(prpcmsg as _) }
}
#[inline]
pub unsafe fn Ndr64AsyncServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64AsyncServerCallAll(prpcmsg : *mut RPC_MESSAGE));
    unsafe { Ndr64AsyncServerCallAll(prpcmsg as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn Ndr64DcomAsyncClientCall(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { Ndr64DcomAsyncClientCall(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64DcomAsyncStubCall<P0, P1>(pthis: P0, pchannel: P1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32
where
    P0: windows_core::Param<super::Com::IRpcStubBuffer>,
    P1: windows_core::Param<super::Com::IRpcChannelBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64DcomAsyncStubCall(pthis : * mut core::ffi::c_void, pchannel : * mut core::ffi::c_void, prpcmsg : *mut RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { Ndr64DcomAsyncStubCall(pthis.param().abi(), pchannel.param().abi(), prpcmsg as _, pdwstubphase as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrAllocate(pstubmsg: *mut MIDL_STUB_MESSAGE, len: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrAllocate(pstubmsg : *mut MIDL_STUB_MESSAGE, len : usize) -> *mut core::ffi::c_void);
    unsafe { NdrAllocate(core::mem::transmute(pstubmsg), len) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrAsyncClientCall(pstubdescriptor : *mut MIDL_STUB_DESC, pformat : *mut u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrAsyncClientCall(pstubdescriptor as _, pformat as _) }
}
#[inline]
pub unsafe fn NdrAsyncServerCall(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrAsyncServerCall(prpcmsg : *mut RPC_MESSAGE));
    unsafe { NdrAsyncServerCall(prpcmsg as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrByteCountPointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrByteCountPointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrByteCountPointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrByteCountPointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClearOutParameters(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, argaddr: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClearOutParameters(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8, argaddr : *mut core::ffi::c_void));
    unsafe { NdrClearOutParameters(core::mem::transmute(pstubmsg), pformat as _, argaddr as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientCall2(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrClientCall2(pstubdescriptor : *mut MIDL_STUB_DESC, pformat : *mut u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrClientCall2(pstubdescriptor as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrClientCall3(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { NdrClientCall3(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: isize, fcheck: i32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientContextMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : isize, fcheck : i32));
    unsafe { NdrClientContextMarshall(core::mem::transmute(pstubmsg), contexthandle, fcheck) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pcontexthandle: *mut isize, bindhandle: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientContextUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pcontexthandle : *mut isize, bindhandle : *mut core::ffi::c_void));
    unsafe { NdrClientContextUnmarshall(core::mem::transmute(pstubmsg), pcontexthandle as _, bindhandle as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientInitialize(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *mut MIDL_STUB_DESC, procnum : u32));
    unsafe { NdrClientInitialize(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor as _, procnum) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientInitializeNew(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *mut MIDL_STUB_DESC, procnum : u32));
    unsafe { NdrClientInitializeNew(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor as _, procnum) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrComplexArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrComplexArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrComplexArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrComplexArrayMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrComplexArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrComplexStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrComplexStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrComplexStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrComplexStructMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrComplexStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrConformantArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrConformantArrayMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantStringBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrConformantStringMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrConformantStringMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantStringUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrConformantStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrConformantStructMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantVaryingArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantVaryingArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrConformantVaryingArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrConformantVaryingArrayMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantVaryingArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantVaryingStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrConformantVaryingStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrConformantVaryingStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrConformantVaryingStructMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantVaryingStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrContextHandleInitialize(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrContextHandleInitialize(pstubmsg : *const MIDL_STUB_MESSAGE, pformat : *const u8) -> *mut NDR_SCONTEXT);
    unsafe { NdrContextHandleInitialize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrContextHandleSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrContextHandleSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrContextHandleSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConvert(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConvert(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8));
    unsafe { NdrConvert(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConvert2(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, numberparams: i32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConvert2(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8, numberparams : i32));
    unsafe { NdrConvert2(core::mem::transmute(pstubmsg), pformat as _, numberparams) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationFree(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationFree(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrCorrelationFree(core::mem::transmute(pstubmsg)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void, cachesize: u32, flags: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationInitialize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void, cachesize : u32, flags : u32));
    unsafe { NdrCorrelationInitialize(core::mem::transmute(pstubmsg), pmemory as _, cachesize, flags) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationPass(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationPass(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrCorrelationPass(core::mem::transmute(pstubmsg)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCreateServerInterfaceFromStub<P0>(pstub: P0, pserverif: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS
where
    P0: windows_core::Param<super::Com::IRpcStubBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn NdrCreateServerInterfaceFromStub(pstub : * mut core::ffi::c_void, pserverif : *mut RPC_SERVER_INTERFACE) -> RPC_STATUS);
    unsafe { NdrCreateServerInterfaceFromStub(pstub.param().abi(), pserverif as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrDcomAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrDcomAsyncClientCall(pstubdescriptor : *mut MIDL_STUB_DESC, pformat : *mut u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrDcomAsyncClientCall(pstubdescriptor as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrDcomAsyncStubCall<P0, P1>(pthis: P0, pchannel: P1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32
where
    P0: windows_core::Param<super::Com::IRpcStubBuffer>,
    P1: windows_core::Param<super::Com::IRpcChannelBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn NdrDcomAsyncStubCall(pthis : * mut core::ffi::c_void, pchannel : * mut core::ffi::c_void, prpcmsg : *mut RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrDcomAsyncStubCall(pthis.param().abi(), pchannel.param().abi(), prpcmsg as _, pdwstubphase as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrEncapsulatedUnionBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrEncapsulatedUnionFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrEncapsulatedUnionMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrEncapsulatedUnionMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrEncapsulatedUnionUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrFixedArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrFixedArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrFixedArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrFixedArrayMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrFixedArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFreeBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFreeBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrFreeBuffer(core::mem::transmute(pstubmsg)) }
}
#[inline]
pub unsafe fn NdrFullPointerXlatFree(pxlattables: *mut FULL_PTR_XLAT_TABLES) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFullPointerXlatFree(pxlattables : *mut FULL_PTR_XLAT_TABLES));
    unsafe { NdrFullPointerXlatFree(pxlattables as _) }
}
#[inline]
pub unsafe fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> *mut FULL_PTR_XLAT_TABLES {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFullPointerXlatInit(numberofpointers : u32, xlatside : XLAT_SIDE) -> *mut FULL_PTR_XLAT_TABLES);
    unsafe { NdrFullPointerXlatInit(numberofpointers, xlatside) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut core::ffi::c_void) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE, bufferlength : u32, handle : *mut core::ffi::c_void) -> *mut u8);
    unsafe { NdrGetBuffer(core::mem::transmute(pstubmsg), bufferlength, handle as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetDcomProtocolVersion(pstubmsg: *mut MIDL_STUB_MESSAGE, pversion: *mut RPC_VERSION) -> windows_core::Result<()> {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetDcomProtocolVersion(pstubmsg : *mut MIDL_STUB_MESSAGE, pversion : *mut RPC_VERSION) -> windows_core::HRESULT);
    unsafe { NdrGetDcomProtocolVersion(core::mem::transmute(pstubmsg), pversion as _).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetUserMarshalInfo(pflags : *const u32, informationlevel : u32, pmarshalinfo : *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS);
    unsafe { NdrGetUserMarshalInfo(pflags, informationlevel, core::mem::transmute(pmarshalinfo)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrInterfacePointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrInterfacePointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrInterfacePointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrInterfacePointerMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrInterfacePointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMapCommAndFaultStatus(pstubmsg: *mut MIDL_STUB_MESSAGE, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: RPC_STATUS) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMapCommAndFaultStatus(pstubmsg : *mut MIDL_STUB_MESSAGE, pcommstatus : *mut u32, pfaultstatus : *mut u32, status : RPC_STATUS) -> RPC_STATUS);
    unsafe { NdrMapCommAndFaultStatus(core::mem::transmute(pstubmsg), pcommstatus as _, pfaultstatus as _, status) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode(handle: *mut core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) {
    windows_core::link!("rpcrt4.dll" "C" fn NdrMesProcEncodeDecode(handle : *mut core::ffi::c_void, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8));
    unsafe { NdrMesProcEncodeDecode(handle as _, pstubdesc, pformatstring as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode2(handle: *mut core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrMesProcEncodeDecode2(handle : *mut core::ffi::c_void, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrMesProcEncodeDecode2(handle as _, pstubdesc, pformatstring as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode3(handle: *mut core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrMesProcEncodeDecode3(handle : *mut core::ffi::c_void, pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { NdrMesProcEncodeDecode3(handle as _, pproxyinfo, nprocnum, preturnvalue as _) }
}
#[inline]
pub unsafe fn NdrMesSimpleTypeAlignSize(param0: *mut core::ffi::c_void) -> usize {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesSimpleTypeAlignSize(param0 : *mut core::ffi::c_void) -> usize);
    unsafe { NdrMesSimpleTypeAlignSize(param0 as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeAlignSizeAll(handle: *mut core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO) -> usize {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesSimpleTypeAlignSizeAll(handle : *mut core::ffi::c_void, pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO) -> usize);
    unsafe { NdrMesSimpleTypeAlignSizeAll(handle as _, pproxyinfo) }
}
#[inline]
pub unsafe fn NdrMesSimpleTypeDecode(handle: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, size: i16) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesSimpleTypeDecode(handle : *mut core::ffi::c_void, pobject : *mut core::ffi::c_void, size : i16));
    unsafe { NdrMesSimpleTypeDecode(handle as _, pobject as _, size) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeDecodeAll(handle: *mut core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *mut core::ffi::c_void, size: i16) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesSimpleTypeDecodeAll(handle : *mut core::ffi::c_void, pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO, pobject : *mut core::ffi::c_void, size : i16));
    unsafe { NdrMesSimpleTypeDecodeAll(handle as _, pproxyinfo, pobject as _, size) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeEncode(handle: *mut core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pobject: *const core::ffi::c_void, size: i16) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesSimpleTypeEncode(handle : *mut core::ffi::c_void, pstubdesc : *const MIDL_STUB_DESC, pobject : *const core::ffi::c_void, size : i16));
    unsafe { NdrMesSimpleTypeEncode(handle as _, pstubdesc, pobject, size) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeEncodeAll(handle: *mut core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *const core::ffi::c_void, size: i16) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesSimpleTypeEncodeAll(handle : *mut core::ffi::c_void, pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO, pobject : *const core::ffi::c_void, size : i16));
    unsafe { NdrMesSimpleTypeEncodeAll(handle as _, pproxyinfo, pobject, size) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize(handle: *mut core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const core::ffi::c_void) -> usize {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeAlignSize(handle : *mut core::ffi::c_void, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8, pobject : *const core::ffi::c_void) -> usize);
    unsafe { NdrMesTypeAlignSize(handle as _, pstubdesc, pformatstring as _, pobject) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize2(handle: *mut core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const core::ffi::c_void) -> usize {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeAlignSize2(handle : *mut core::ffi::c_void, ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8, pobject : *const core::ffi::c_void) -> usize);
    unsafe { NdrMesTypeAlignSize2(handle as _, ppicklinginfo, pstubdesc, pformatstring as _, pobject) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize3(handle: *mut core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const core::ffi::c_void) -> usize {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeAlignSize3(handle : *mut core::ffi::c_void, ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO, pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset : *const *const u32, ntypeindex : u32, pobject : *const core::ffi::c_void) -> usize);
    unsafe { NdrMesTypeAlignSize3(handle as _, ppicklinginfo, pproxyinfo, arrtypeoffset, ntypeindex, pobject) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode(handle: *mut core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeDecode(handle : *mut core::ffi::c_void, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8, pobject : *mut core::ffi::c_void));
    unsafe { NdrMesTypeDecode(handle as _, pstubdesc, pformatstring as _, pobject as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode2(handle: *mut core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeDecode2(handle : *mut core::ffi::c_void, ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8, pobject : *mut core::ffi::c_void));
    unsafe { NdrMesTypeDecode2(handle as _, ppicklinginfo, pstubdesc, pformatstring as _, pobject as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode3(handle: *mut core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeDecode3(handle : *mut core::ffi::c_void, ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO, pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset : *const *const u32, ntypeindex : u32, pobject : *mut core::ffi::c_void));
    unsafe { NdrMesTypeDecode3(handle as _, ppicklinginfo, pproxyinfo, arrtypeoffset, ntypeindex, pobject as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode(handle: *mut core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeEncode(handle : *mut core::ffi::c_void, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8, pobject : *const core::ffi::c_void));
    unsafe { NdrMesTypeEncode(handle as _, pstubdesc, pformatstring as _, pobject) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode2(handle: *mut core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeEncode2(handle : *mut core::ffi::c_void, ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8, pobject : *const core::ffi::c_void));
    unsafe { NdrMesTypeEncode2(handle as _, ppicklinginfo, pstubdesc, pformatstring as _, pobject) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode3(handle: *mut core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeEncode3(handle : *mut core::ffi::c_void, ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO, pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset : *const *const u32, ntypeindex : u32, pobject : *const core::ffi::c_void));
    unsafe { NdrMesTypeEncode3(handle as _, ppicklinginfo, pproxyinfo, arrtypeoffset, ntypeindex, pobject) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeFree2(handle: *mut core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeFree2(handle : *mut core::ffi::c_void, ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO, pstubdesc : *const MIDL_STUB_DESC, pformatstring : *mut u8, pobject : *mut core::ffi::c_void));
    unsafe { NdrMesTypeFree2(handle as _, ppicklinginfo, pstubdesc, pformatstring as _, pobject as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeFree3(handle: *mut core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMesTypeFree3(handle : *mut core::ffi::c_void, ppicklinginfo : *const MIDL_TYPE_PICKLING_INFO, pproxyinfo : *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset : *const *const u32, ntypeindex : u32, pobject : *mut core::ffi::c_void));
    unsafe { NdrMesTypeFree3(handle as _, ppicklinginfo, pproxyinfo, arrtypeoffset, ntypeindex, pobject as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrNonConformantStringBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrNonConformantStringMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrNonConformantStringMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrNonConformantStringUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrNonEncapsulatedUnionBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrNonEncapsulatedUnionFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrNonEncapsulatedUnionMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrNonEncapsulatedUnionMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrNonEncapsulatedUnionUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNsGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut core::ffi::c_void) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNsGetBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE, bufferlength : u32, handle : *mut core::ffi::c_void) -> *mut u8);
    unsafe { NdrNsGetBuffer(core::mem::transmute(pstubmsg), bufferlength, handle as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNsSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8, pautohandle: *mut *mut core::ffi::c_void) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNsSendReceive(pstubmsg : *mut MIDL_STUB_MESSAGE, pbufferend : *mut u8, pautohandle : *mut *mut core::ffi::c_void) -> *mut u8);
    unsafe { NdrNsSendReceive(core::mem::transmute(pstubmsg), pbufferend as _, pautohandle as _) }
}
#[inline]
pub unsafe fn NdrOleAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrOleAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrOleAllocate(size) }
}
#[inline]
pub unsafe fn NdrOleFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrOleFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrOleFree(nodetofree) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreClientBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreClientBufferSize(core::mem::transmute(pstubmsg), pmemory as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreClientMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreClientMarshall(core::mem::transmute(pstubmsg), pmemory as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut core::ffi::c_void, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreServerInitialize(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut core::ffi::c_void, pformat : *mut u8));
    unsafe { NdrPartialIgnoreServerInitialize(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreServerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreServerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrPointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrPointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrPointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrPointerMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrPointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRangeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRangeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrRangeUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[inline]
pub unsafe fn NdrRpcSmClientAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmClientAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrRpcSmClientAllocate(size) }
}
#[inline]
pub unsafe fn NdrRpcSmClientFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmClientFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrRpcSmClientFree(nodetofree) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSmSetClientToOsf(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmSetClientToOsf(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSmSetClientToOsf(core::mem::transmute(pmessage)) }
}
#[inline]
pub unsafe fn NdrRpcSsDefaultAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDefaultAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrRpcSsDefaultAllocate(size) }
}
#[inline]
pub unsafe fn NdrRpcSsDefaultFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDefaultFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrRpcSsDefaultFree(nodetofree) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSsDisableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDisableAllocate(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSsDisableAllocate(core::mem::transmute(pmessage)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSsEnableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsEnableAllocate(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSsEnableAllocate(core::mem::transmute(pmessage)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSendReceive(pstubmsg : *mut MIDL_STUB_MESSAGE, pbufferend : *mut u8) -> *mut u8);
    unsafe { NdrSendReceive(core::mem::transmute(pstubmsg), pbufferend as _) }
}
#[inline]
pub unsafe fn NdrServerCall2(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCall2(prpcmsg : *mut RPC_MESSAGE));
    unsafe { NdrServerCall2(prpcmsg as _) }
}
#[inline]
pub unsafe fn NdrServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCallAll(prpcmsg : *mut RPC_MESSAGE));
    unsafe { NdrServerCallAll(prpcmsg as _) }
}
#[inline]
pub unsafe fn NdrServerCallNdr64(prpcmsg: *mut RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCallNdr64(prpcmsg : *mut RPC_MESSAGE));
    unsafe { NdrServerCallNdr64(prpcmsg as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT, rundownroutine: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : *mut NDR_SCONTEXT, rundownroutine : NDR_RUNDOWN));
    unsafe { NdrServerContextMarshall(core::mem::transmute(pstubmsg), contexthandle as _, rundownroutine) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextNewMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT, rundownroutine: NDR_RUNDOWN, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextNewMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : *mut NDR_SCONTEXT, rundownroutine : NDR_RUNDOWN, pformat : *mut u8));
    unsafe { NdrServerContextNewMarshall(core::mem::transmute(pstubmsg), contexthandle as _, rundownroutine, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextNewUnmarshall(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextNewUnmarshall(pstubmsg : *const MIDL_STUB_MESSAGE, pformat : *const u8) -> *mut NDR_SCONTEXT);
    unsafe { NdrServerContextNewUnmarshall(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE) -> *mut NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE) -> *mut NDR_SCONTEXT);
    unsafe { NdrServerContextUnmarshall(core::mem::transmute(pstubmsg)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitialize(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *mut MIDL_STUB_DESC) -> *mut u8);
    unsafe { NdrServerInitialize(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeMarshall(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeMarshall(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrServerInitializeMarshall(prpcmsg as _, core::mem::transmute(pstubmsg)) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeNew(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *mut MIDL_STUB_DESC) -> *mut u8);
    unsafe { NdrServerInitializeNew(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializePartial(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, requestedbuffersize: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializePartial(prpcmsg : *mut RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *mut MIDL_STUB_DESC, requestedbuffersize : u32));
    unsafe { NdrServerInitializePartial(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor as _, requestedbuffersize) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, prpcmsg: *mut RPC_MESSAGE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *mut MIDL_STUB_DESC, prpcmsg : *mut RPC_MESSAGE) -> *mut u8);
    unsafe { NdrServerInitializeUnmarshall(core::mem::transmute(pstubmsg), pstubdescriptor as _, prpcmsg as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrSimpleStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrSimpleStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrSimpleStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrSimpleStructMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrSimpleStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleTypeMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleTypeMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, formatchar : u8));
    unsafe { NdrSimpleTypeMarshall(core::mem::transmute(pstubmsg), pmemory as _, formatchar) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleTypeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleTypeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, formatchar : u8));
    unsafe { NdrSimpleTypeUnmarshall(core::mem::transmute(pstubmsg), pmemory as _, formatchar) }
}
#[inline]
pub unsafe fn NdrStubCall2(pthis: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrStubCall2(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrStubCall2(pthis as _, pchannel as _, prpcmsg as _, pdwstubphase as _) }
}
#[inline]
pub unsafe fn NdrStubCall3(pthis: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrStubCall3(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrStubCall3(pthis as _, pchannel as _, prpcmsg as _, pdwstubphase as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrUserMarshalBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrUserMarshalFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrUserMarshalMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrUserMarshalMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[inline]
pub unsafe fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalSimpleTypeConvert(pflags : *mut u32, pbuffer : *mut u8, formatchar : u8) -> *mut u8);
    unsafe { NdrUserMarshalSimpleTypeConvert(pflags as _, pbuffer as _, formatchar) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrUserMarshalUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrVaryingArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrVaryingArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrVaryingArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrVaryingArrayMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrVaryingArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrXmitOrRepAsBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8));
    unsafe { NdrXmitOrRepAsFree(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *mut u8) -> *mut u8);
    unsafe { NdrXmitOrRepAsMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *mut u8) -> u32);
    unsafe { NdrXmitOrRepAsMemorySize(core::mem::transmute(pstubmsg), pformat as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *mut u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrXmitOrRepAsUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat as _, fmustalloc) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RpcAsyncAbortCall(pasync: *mut RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncAbortCall(pasync : *mut RPC_ASYNC_STATE, exceptioncode : u32) -> RPC_STATUS);
    unsafe { RpcAsyncAbortCall(pasync as _, exceptioncode) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RpcAsyncCancelCall(pasync: *mut RPC_ASYNC_STATE, fabort: bool) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncCancelCall(pasync : *mut RPC_ASYNC_STATE, fabort : windows_core::BOOL) -> RPC_STATUS);
    unsafe { RpcAsyncCancelCall(pasync as _, fabort.into()) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RpcAsyncCompleteCall(pasync: *mut RPC_ASYNC_STATE, reply: Option<*mut core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncCompleteCall(pasync : *mut RPC_ASYNC_STATE, reply : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcAsyncCompleteCall(pasync as _, reply.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RpcAsyncGetCallStatus(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncGetCallStatus(pasync : *const RPC_ASYNC_STATE) -> RPC_STATUS);
    unsafe { RpcAsyncGetCallStatus(pasync) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RpcAsyncInitializeHandle(pasync: *mut RPC_ASYNC_STATE, size: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncInitializeHandle(pasync : *mut RPC_ASYNC_STATE, size : u32) -> RPC_STATUS);
    unsafe { RpcAsyncInitializeHandle(pasync as _, size) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RpcAsyncRegisterInfo(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcAsyncRegisterInfo(pasync : *const RPC_ASYNC_STATE) -> RPC_STATUS);
    unsafe { RpcAsyncRegisterInfo(pasync) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RpcBindingBind(pasync: Option<*const RPC_ASYNC_STATE>, binding: *const core::ffi::c_void, ifspec: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingBind(pasync : *const RPC_ASYNC_STATE, binding : *const core::ffi::c_void, ifspec : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingBind(pasync.unwrap_or(core::mem::zeroed()) as _, binding, ifspec) }
}
#[inline]
pub unsafe fn RpcBindingCopy(sourcebinding: *const core::ffi::c_void, destinationbinding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCopy(sourcebinding : *const core::ffi::c_void, destinationbinding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingCopy(sourcebinding, destinationbinding as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: Option<*const RPC_BINDING_HANDLE_SECURITY_V1_A>, options: Option<*const RPC_BINDING_HANDLE_OPTIONS_V1>, binding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCreateA(template : *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security : *const RPC_BINDING_HANDLE_SECURITY_V1_A, options : *const RPC_BINDING_HANDLE_OPTIONS_V1, binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingCreateA(template, security.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, binding as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: Option<*const RPC_BINDING_HANDLE_SECURITY_V1_W>, options: Option<*const RPC_BINDING_HANDLE_OPTIONS_V1>, binding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCreateW(template : *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security : *const RPC_BINDING_HANDLE_SECURITY_V1_W, options : *const RPC_BINDING_HANDLE_OPTIONS_V1, binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingCreateW(template, security.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, binding as _) }
}
#[inline]
pub unsafe fn RpcBindingFree(binding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFree(binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingFree(binding as _) }
}
#[inline]
pub unsafe fn RpcBindingFromStringBindingA<P0>(stringbinding: P0, binding: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFromStringBindingA(stringbinding : windows_core::PCSTR, binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingFromStringBindingA(stringbinding.param().abi(), binding as _) }
}
#[inline]
pub unsafe fn RpcBindingFromStringBindingW<P0>(stringbinding: P0, binding: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFromStringBindingW(stringbinding : windows_core::PCWSTR, binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingFromStringBindingW(stringbinding.param().abi(), binding as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthClientA(clientbinding: Option<*const core::ffi::c_void>, privs: *mut *mut core::ffi::c_void, serverprincname: Option<*mut windows_core::PSTR>, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientA(clientbinding : *const core::ffi::c_void, privs : *mut *mut core::ffi::c_void, serverprincname : *mut windows_core::PSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthClientA(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthClientExA(clientbinding: Option<*const core::ffi::c_void>, privs: *mut *mut core::ffi::c_void, serverprincname: Option<*mut windows_core::PSTR>, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>, flags: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientExA(clientbinding : *const core::ffi::c_void, privs : *mut *mut core::ffi::c_void, serverprincname : *mut windows_core::PSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32, flags : u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthClientExA(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthClientExW(clientbinding: Option<*const core::ffi::c_void>, privs: *mut *mut core::ffi::c_void, serverprincname: Option<*mut windows_core::PWSTR>, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>, flags: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientExW(clientbinding : *const core::ffi::c_void, privs : *mut *mut core::ffi::c_void, serverprincname : *mut windows_core::PWSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32, flags : u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthClientExW(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthClientW(clientbinding: Option<*const core::ffi::c_void>, privs: *mut *mut core::ffi::c_void, serverprincname: Option<*mut windows_core::PWSTR>, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientW(clientbinding : *const core::ffi::c_void, privs : *mut *mut core::ffi::c_void, serverprincname : *mut windows_core::PWSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthClientW(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthInfoA(binding: *const core::ffi::c_void, serverprincname: Option<*mut windows_core::PSTR>, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut *mut core::ffi::c_void>, authzsvc: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoA(binding : *const core::ffi::c_void, serverprincname : *mut windows_core::PSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut *mut core::ffi::c_void, authzsvc : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoA(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExA(binding: *const core::ffi::c_void, serverprincname: Option<*mut windows_core::PSTR>, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut *mut core::ffi::c_void>, authzsvc: Option<*mut u32>, rpcqosversion: u32, securityqos: Option<*mut RPC_SECURITY_QOS>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoExA(binding : *const core::ffi::c_void, serverprincname : *mut windows_core::PSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut *mut core::ffi::c_void, authzsvc : *mut u32, rpcqosversion : u32, securityqos : *mut RPC_SECURITY_QOS) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoExA(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, rpcqosversion, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExW(binding: *const core::ffi::c_void, serverprincname: Option<*mut windows_core::PWSTR>, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut *mut core::ffi::c_void>, authzsvc: Option<*mut u32>, rpcqosversion: u32, securityqos: Option<*mut RPC_SECURITY_QOS>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoExW(binding : *const core::ffi::c_void, serverprincname : *mut windows_core::PWSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut *mut core::ffi::c_void, authzsvc : *mut u32, rpcqosversion : u32, securityqos : *mut RPC_SECURITY_QOS) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoExW(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, rpcqosversion, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqAuthInfoW(binding: *const core::ffi::c_void, serverprincname: Option<*mut windows_core::PWSTR>, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut *mut core::ffi::c_void>, authzsvc: Option<*mut u32>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoW(binding : *const core::ffi::c_void, serverprincname : *mut windows_core::PWSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut *mut core::ffi::c_void, authzsvc : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoW(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingInqMaxCalls(binding: *const core::ffi::c_void, maxcalls: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqMaxCalls(binding : *const core::ffi::c_void, maxcalls : *mut u32) -> RPC_STATUS);
    unsafe { RpcBindingInqMaxCalls(binding, maxcalls as _) }
}
#[inline]
pub unsafe fn RpcBindingInqObject(binding: *const core::ffi::c_void, objectuuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqObject(binding : *const core::ffi::c_void, objectuuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcBindingInqObject(binding, objectuuid as _) }
}
#[inline]
pub unsafe fn RpcBindingInqOption(hbinding: *const core::ffi::c_void, option: u32, poptionvalue: *mut usize) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqOption(hbinding : *const core::ffi::c_void, option : u32, poptionvalue : *mut usize) -> RPC_STATUS);
    unsafe { RpcBindingInqOption(hbinding, option, poptionvalue as _) }
}
#[inline]
pub unsafe fn RpcBindingReset(binding: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingReset(binding : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingReset(binding) }
}
#[inline]
pub unsafe fn RpcBindingServerFromClient(clientbinding: Option<*const core::ffi::c_void>, serverbinding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingServerFromClient(clientbinding : *const core::ffi::c_void, serverbinding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingServerFromClient(clientbinding.unwrap_or(core::mem::zeroed()) as _, serverbinding as _) }
}
#[inline]
pub unsafe fn RpcBindingSetAuthInfoA<P1>(binding: *const core::ffi::c_void, serverprincname: P1, authnlevel: u32, authnsvc: u32, authidentity: Option<*const core::ffi::c_void>, authzsvc: u32) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoA(binding : *const core::ffi::c_void, serverprincname : windows_core::PCSTR, authnlevel : u32, authnsvc : u32, authidentity : *const core::ffi::c_void, authzsvc : u32) -> RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoA(binding, serverprincname.param().abi(), authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExA<P1>(binding: *const core::ffi::c_void, serverprincname: P1, authnlevel: u32, authnsvc: u32, authidentity: Option<*const core::ffi::c_void>, authzsvc: u32, securityqos: Option<*const RPC_SECURITY_QOS>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoExA(binding : *const core::ffi::c_void, serverprincname : windows_core::PCSTR, authnlevel : u32, authnsvc : u32, authidentity : *const core::ffi::c_void, authzsvc : u32, securityqos : *const RPC_SECURITY_QOS) -> RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoExA(binding, serverprincname.param().abi(), authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExW<P1>(binding: *const core::ffi::c_void, serverprincname: P1, authnlevel: u32, authnsvc: u32, authidentity: Option<*const core::ffi::c_void>, authzsvc: u32, securityqos: Option<*const RPC_SECURITY_QOS>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoExW(binding : *const core::ffi::c_void, serverprincname : windows_core::PCWSTR, authnlevel : u32, authnsvc : u32, authidentity : *const core::ffi::c_void, authzsvc : u32, securityqos : *const RPC_SECURITY_QOS) -> RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoExW(binding, serverprincname.param().abi(), authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcBindingSetAuthInfoW<P1>(binding: *const core::ffi::c_void, serverprincname: P1, authnlevel: u32, authnsvc: u32, authidentity: Option<*const core::ffi::c_void>, authzsvc: u32) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoW(binding : *const core::ffi::c_void, serverprincname : windows_core::PCWSTR, authnlevel : u32, authnsvc : u32, authidentity : *const core::ffi::c_void, authzsvc : u32) -> RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoW(binding, serverprincname.param().abi(), authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc) }
}
#[inline]
pub unsafe fn RpcBindingSetObject(binding: *const core::ffi::c_void, objectuuid: *const windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetObject(binding : *const core::ffi::c_void, objectuuid : *const windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcBindingSetObject(binding, objectuuid) }
}
#[inline]
pub unsafe fn RpcBindingSetOption(hbinding: *const core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetOption(hbinding : *const core::ffi::c_void, option : u32, optionvalue : usize) -> RPC_STATUS);
    unsafe { RpcBindingSetOption(hbinding, option, optionvalue) }
}
#[inline]
pub unsafe fn RpcBindingToStringBindingA(binding: *const core::ffi::c_void, stringbinding: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingToStringBindingA(binding : *const core::ffi::c_void, stringbinding : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcBindingToStringBindingA(binding, stringbinding as _) }
}
#[inline]
pub unsafe fn RpcBindingToStringBindingW(binding: *const core::ffi::c_void, stringbinding: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingToStringBindingW(binding : *const core::ffi::c_void, stringbinding : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcBindingToStringBindingW(binding, stringbinding as _) }
}
#[inline]
pub unsafe fn RpcBindingUnbind(binding: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingUnbind(binding : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcBindingUnbind(binding) }
}
#[inline]
pub unsafe fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingVectorFree(bindingvector : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcBindingVectorFree(bindingvector as _) }
}
#[inline]
pub unsafe fn RpcCancelThread(thread: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcCancelThread(thread : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcCancelThread(thread) }
}
#[inline]
pub unsafe fn RpcCancelThreadEx(thread: *const core::ffi::c_void, timeout: i32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcCancelThreadEx(thread : *const core::ffi::c_void, timeout : i32) -> RPC_STATUS);
    unsafe { RpcCancelThreadEx(thread, timeout) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn RpcCertGeneratePrincipalNameA(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcCertGeneratePrincipalNameA(context : *const super::super::Security::Cryptography:: CERT_CONTEXT, flags : u32, pbuffer : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcCertGeneratePrincipalNameA(context, flags, pbuffer as _) }
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn RpcCertGeneratePrincipalNameW(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcCertGeneratePrincipalNameW(context : *const super::super::Security::Cryptography:: CERT_CONTEXT, flags : u32, pbuffer : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcCertGeneratePrincipalNameW(context, flags, pbuffer as _) }
}
#[inline]
pub unsafe fn RpcEpRegisterA<P3>(ifspec: *const core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: P3) -> RPC_STATUS
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterA(ifspec : *const core::ffi::c_void, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcEpRegisterA(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.param().abi()) }
}
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceA<P3>(ifspec: *const core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: P3) -> RPC_STATUS
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterNoReplaceA(ifspec : *const core::ffi::c_void, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcEpRegisterNoReplaceA(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.param().abi()) }
}
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceW<P3>(ifspec: *const core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: P3) -> RPC_STATUS
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterNoReplaceW(ifspec : *const core::ffi::c_void, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcEpRegisterNoReplaceW(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.param().abi()) }
}
#[inline]
pub unsafe fn RpcEpRegisterW<P3>(ifspec: *const core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: P3) -> RPC_STATUS
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterW(ifspec : *const core::ffi::c_void, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcEpRegisterW(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.param().abi()) }
}
#[inline]
pub unsafe fn RpcEpResolveBinding(binding: *const core::ffi::c_void, ifspec: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpResolveBinding(binding : *const core::ffi::c_void, ifspec : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcEpResolveBinding(binding, ifspec) }
}
#[inline]
pub unsafe fn RpcEpUnregister(ifspec: *const core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpUnregister(ifspec : *const core::ffi::c_void, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcEpUnregister(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcErrorAddRecord(errorinfo: *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorAddRecord(errorinfo : *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS);
    unsafe { RpcErrorAddRecord(errorinfo) }
}
#[inline]
pub unsafe fn RpcErrorClearInformation() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorClearInformation());
    unsafe { RpcErrorClearInformation() }
}
#[inline]
pub unsafe fn RpcErrorEndEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorEndEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS);
    unsafe { RpcErrorEndEnumeration(enumhandle as _) }
}
#[inline]
pub unsafe fn RpcErrorGetNextRecord(enumhandle: *const RPC_ERROR_ENUM_HANDLE, copystrings: bool, errorinfo: *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorGetNextRecord(enumhandle : *const RPC_ERROR_ENUM_HANDLE, copystrings : windows_core::BOOL, errorinfo : *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS);
    unsafe { RpcErrorGetNextRecord(enumhandle, copystrings.into(), errorinfo as _) }
}
#[inline]
pub unsafe fn RpcErrorGetNumberOfRecords(enumhandle: *const RPC_ERROR_ENUM_HANDLE, records: *mut i32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorGetNumberOfRecords(enumhandle : *const RPC_ERROR_ENUM_HANDLE, records : *mut i32) -> RPC_STATUS);
    unsafe { RpcErrorGetNumberOfRecords(enumhandle, records as _) }
}
#[inline]
pub unsafe fn RpcErrorLoadErrorInfo(errorblob: *const core::ffi::c_void, blobsize: usize, enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorLoadErrorInfo(errorblob : *const core::ffi::c_void, blobsize : usize, enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS);
    unsafe { RpcErrorLoadErrorInfo(errorblob, blobsize, enumhandle as _) }
}
#[inline]
pub unsafe fn RpcErrorResetEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorResetEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS);
    unsafe { RpcErrorResetEnumeration(enumhandle as _) }
}
#[inline]
pub unsafe fn RpcErrorSaveErrorInfo(enumhandle: *const RPC_ERROR_ENUM_HANDLE, errorblob: *mut *mut core::ffi::c_void, blobsize: *mut usize) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorSaveErrorInfo(enumhandle : *const RPC_ERROR_ENUM_HANDLE, errorblob : *mut *mut core::ffi::c_void, blobsize : *mut usize) -> RPC_STATUS);
    unsafe { RpcErrorSaveErrorInfo(enumhandle, errorblob as _, blobsize as _) }
}
#[inline]
pub unsafe fn RpcErrorStartEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcErrorStartEnumeration(enumhandle : *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS);
    unsafe { RpcErrorStartEnumeration(enumhandle as _) }
}
#[inline]
pub unsafe fn RpcExceptionFilter(exceptioncode: u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn RpcExceptionFilter(exceptioncode : u32) -> i32);
    unsafe { RpcExceptionFilter(exceptioncode) }
}
#[inline]
pub unsafe fn RpcFreeAuthorizationContext(pauthzclientcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcFreeAuthorizationContext(pauthzclientcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcFreeAuthorizationContext(pauthzclientcontext as _) }
}
#[inline]
pub unsafe fn RpcGetAuthorizationContextForClient(clientbinding: Option<*const core::ffi::c_void>, impersonateonreturn: bool, reserved1: Option<*const core::ffi::c_void>, pexpirationtime: Option<*const i64>, reserved2: super::super::Foundation::LUID, reserved3: u32, reserved4: Option<*const core::ffi::c_void>, pauthzclientcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcGetAuthorizationContextForClient(clientbinding : *const core::ffi::c_void, impersonateonreturn : windows_core::BOOL, reserved1 : *const core::ffi::c_void, pexpirationtime : *const i64, reserved2 : super::super::Foundation:: LUID, reserved3 : u32, reserved4 : *const core::ffi::c_void, pauthzclientcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcGetAuthorizationContextForClient(clientbinding.unwrap_or(core::mem::zeroed()) as _, impersonateonreturn.into(), reserved1.unwrap_or(core::mem::zeroed()) as _, pexpirationtime.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(reserved2), reserved3, reserved4.unwrap_or(core::mem::zeroed()) as _, pauthzclientcontext as _) }
}
#[inline]
pub unsafe fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcIfIdVectorFree(ifidvector : *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS);
    unsafe { RpcIfIdVectorFree(ifidvector as _) }
}
#[inline]
pub unsafe fn RpcIfInqId(rpcifhandle: *const core::ffi::c_void, rpcifid: *mut RPC_IF_ID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcIfInqId(rpcifhandle : *const core::ffi::c_void, rpcifid : *mut RPC_IF_ID) -> RPC_STATUS);
    unsafe { RpcIfInqId(rpcifhandle, rpcifid as _) }
}
#[inline]
pub unsafe fn RpcImpersonateClient(bindinghandle: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClient(bindinghandle : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcImpersonateClient(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcImpersonateClient2(bindinghandle: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClient2(bindinghandle : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcImpersonateClient2(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcImpersonateClientContainer(bindinghandle: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClientContainer(bindinghandle : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcImpersonateClientContainer(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS);
    unsafe { RpcMgmtEnableIdleCleanup() }
}
#[inline]
pub unsafe fn RpcMgmtEpEltInqBegin(epbinding: Option<*const core::ffi::c_void>, inquirytype: u32, ifid: Option<*const RPC_IF_ID>, versoption: Option<u32>, objectuuid: Option<*const windows_core::GUID>, inquirycontext: *mut *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqBegin(epbinding : *const core::ffi::c_void, inquirytype : u32, ifid : *const RPC_IF_ID, versoption : u32, objectuuid : *const windows_core::GUID, inquirycontext : *mut *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcMgmtEpEltInqBegin(epbinding.unwrap_or(core::mem::zeroed()) as _, inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcMgmtEpEltInqDone(inquirycontext: *mut *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqDone(inquirycontext : *mut *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcMgmtEpEltInqDone(inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextA(inquirycontext: *const *const core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: Option<*mut *mut core::ffi::c_void>, objectuuid: Option<*mut windows_core::GUID>, annotation: Option<*mut windows_core::PSTR>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqNextA(inquirycontext : *const *const core::ffi::c_void, ifid : *mut RPC_IF_ID, binding : *mut *mut core::ffi::c_void, objectuuid : *mut windows_core::GUID, annotation : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcMgmtEpEltInqNextA(inquirycontext, ifid as _, binding.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextW(inquirycontext: *const *const core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: Option<*mut *mut core::ffi::c_void>, objectuuid: Option<*mut windows_core::GUID>, annotation: Option<*mut windows_core::PWSTR>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqNextW(inquirycontext : *const *const core::ffi::c_void, ifid : *mut RPC_IF_ID, binding : *mut *mut core::ffi::c_void, objectuuid : *mut windows_core::GUID, annotation : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcMgmtEpEltInqNextW(inquirycontext, ifid as _, binding.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtEpUnregister(epbinding: Option<*const core::ffi::c_void>, ifid: *const RPC_IF_ID, binding: *const core::ffi::c_void, objectuuid: Option<*const windows_core::GUID>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpUnregister(epbinding : *const core::ffi::c_void, ifid : *const RPC_IF_ID, binding : *const core::ffi::c_void, objectuuid : *const windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcMgmtEpUnregister(epbinding.unwrap_or(core::mem::zeroed()) as _, ifid, binding, objectuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqComTimeout(binding: *const core::ffi::c_void, timeout: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqComTimeout(binding : *const core::ffi::c_void, timeout : *mut u32) -> RPC_STATUS);
    unsafe { RpcMgmtInqComTimeout(binding, timeout as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqDefaultProtectLevel(authnsvc : u32, authnlevel : *mut u32) -> RPC_STATUS);
    unsafe { RpcMgmtInqDefaultProtectLevel(authnsvc, authnlevel as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqIfIds(binding: Option<*const core::ffi::c_void>, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqIfIds(binding : *const core::ffi::c_void, ifidvector : *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS);
    unsafe { RpcMgmtInqIfIds(binding.unwrap_or(core::mem::zeroed()) as _, ifidvector as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameA(binding: Option<*const core::ffi::c_void>, authnsvc: u32, serverprincname: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqServerPrincNameA(binding : *const core::ffi::c_void, authnsvc : u32, serverprincname : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcMgmtInqServerPrincNameA(binding.unwrap_or(core::mem::zeroed()) as _, authnsvc, serverprincname as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameW(binding: Option<*const core::ffi::c_void>, authnsvc: u32, serverprincname: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqServerPrincNameW(binding : *const core::ffi::c_void, authnsvc : u32, serverprincname : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcMgmtInqServerPrincNameW(binding.unwrap_or(core::mem::zeroed()) as _, authnsvc, serverprincname as _) }
}
#[inline]
pub unsafe fn RpcMgmtInqStats(binding: Option<*const core::ffi::c_void>, statistics: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqStats(binding : *const core::ffi::c_void, statistics : *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS);
    unsafe { RpcMgmtInqStats(binding.unwrap_or(core::mem::zeroed()) as _, statistics as _) }
}
#[inline]
pub unsafe fn RpcMgmtIsServerListening(binding: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtIsServerListening(binding : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcMgmtIsServerListening(binding.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtSetAuthorizationFn(authorizationfn: RPC_MGMT_AUTHORIZATION_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetAuthorizationFn(authorizationfn : RPC_MGMT_AUTHORIZATION_FN) -> RPC_STATUS);
    unsafe { RpcMgmtSetAuthorizationFn(authorizationfn) }
}
#[inline]
pub unsafe fn RpcMgmtSetCancelTimeout(timeout: i32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetCancelTimeout(timeout : i32) -> RPC_STATUS);
    unsafe { RpcMgmtSetCancelTimeout(timeout) }
}
#[inline]
pub unsafe fn RpcMgmtSetComTimeout(binding: *const core::ffi::c_void, timeout: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetComTimeout(binding : *const core::ffi::c_void, timeout : u32) -> RPC_STATUS);
    unsafe { RpcMgmtSetComTimeout(binding, timeout) }
}
#[inline]
pub unsafe fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetServerStackSize(threadstacksize : u32) -> RPC_STATUS);
    unsafe { RpcMgmtSetServerStackSize(threadstacksize) }
}
#[inline]
pub unsafe fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtStatsVectorFree(statsvector : *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS);
    unsafe { RpcMgmtStatsVectorFree(statsvector as _) }
}
#[inline]
pub unsafe fn RpcMgmtStopServerListening(binding: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtStopServerListening(binding : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcMgmtStopServerListening(binding.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcMgmtWaitServerListen() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtWaitServerListen() -> RPC_STATUS);
    unsafe { RpcMgmtWaitServerListen() }
}
#[inline]
pub unsafe fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkInqProtseqsA(protseqvector : *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS);
    unsafe { RpcNetworkInqProtseqsA(protseqvector as _) }
}
#[inline]
pub unsafe fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkInqProtseqsW(protseqvector : *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS);
    unsafe { RpcNetworkInqProtseqsW(protseqvector as _) }
}
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidA<P0>(protseq: P0) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkIsProtseqValidA(protseq : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNetworkIsProtseqValidA(protseq.param().abi()) }
}
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidW<P0>(protseq: P0) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkIsProtseqValidW(protseq : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNetworkIsProtseqValidW(protseq.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsBindingExportA<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, bindingvec: Option<*const RPC_BINDING_VECTOR>, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportA(entrynamesyntax : u32, entryname : windows_core::PCSTR, ifspec : *const core::ffi::c_void, bindingvec : *const RPC_BINDING_VECTOR, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingExportA(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, bindingvec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingExportPnPA<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objectvector: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportPnPA(entrynamesyntax : u32, entryname : windows_core::PCSTR, ifspec : *const core::ffi::c_void, objectvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingExportPnPA(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingExportPnPW<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objectvector: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportPnPW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, ifspec : *const core::ffi::c_void, objectvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingExportPnPW(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingExportW<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, bindingvec: Option<*const RPC_BINDING_VECTOR>, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, ifspec : *const core::ffi::c_void, bindingvec : *const RPC_BINDING_VECTOR, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingExportW(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, bindingvec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingImportBeginA<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objuuid: Option<*const windows_core::GUID>, importcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportBeginA(entrynamesyntax : u32, entryname : windows_core::PCSTR, ifspec : *const core::ffi::c_void, objuuid : *const windows_core::GUID, importcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsBindingImportBeginA(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, importcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingImportBeginW<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objuuid: Option<*const windows_core::GUID>, importcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportBeginW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, ifspec : *const core::ffi::c_void, objuuid : *const windows_core::GUID, importcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsBindingImportBeginW(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, importcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingImportDone(importcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportDone(importcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsBindingImportDone(importcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingImportNext(importcontext: *mut core::ffi::c_void, binding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportNext(importcontext : *mut core::ffi::c_void, binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsBindingImportNext(importcontext as _, binding as _) }
}
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameA(binding: *const core::ffi::c_void, entrynamesyntax: u32, entryname: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNsBindingInqEntryNameA(binding : *const core::ffi::c_void, entrynamesyntax : u32, entryname : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcNsBindingInqEntryNameA(binding, entrynamesyntax, entryname as _) }
}
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameW(binding: *const core::ffi::c_void, entrynamesyntax: u32, entryname: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNsBindingInqEntryNameW(binding : *const core::ffi::c_void, entrynamesyntax : u32, entryname : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcNsBindingInqEntryNameW(binding, entrynamesyntax, entryname as _) }
}
#[inline]
pub unsafe fn RpcNsBindingLookupBeginA<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objuuid: Option<*const windows_core::GUID>, bindingmaxcount: u32, lookupcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupBeginA(entrynamesyntax : u32, entryname : windows_core::PCSTR, ifspec : *const core::ffi::c_void, objuuid : *const windows_core::GUID, bindingmaxcount : u32, lookupcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsBindingLookupBeginA(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, bindingmaxcount, lookupcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingLookupBeginW<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objuuid: Option<*const windows_core::GUID>, bindingmaxcount: u32, lookupcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupBeginW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, ifspec : *const core::ffi::c_void, objuuid : *const windows_core::GUID, bindingmaxcount : u32, lookupcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsBindingLookupBeginW(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, bindingmaxcount, lookupcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingLookupDone(lookupcontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupDone(lookupcontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsBindingLookupDone(lookupcontext as _) }
}
#[inline]
pub unsafe fn RpcNsBindingLookupNext(lookupcontext: *mut core::ffi::c_void, bindingvec: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupNext(lookupcontext : *mut core::ffi::c_void, bindingvec : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingLookupNext(lookupcontext as _, bindingvec as _) }
}
#[inline]
pub unsafe fn RpcNsBindingSelect(bindingvec: *mut RPC_BINDING_VECTOR, binding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingSelect(bindingvec : *mut RPC_BINDING_VECTOR, binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsBindingSelect(bindingvec as _, binding as _) }
}
#[inline]
pub unsafe fn RpcNsBindingUnexportA<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportA(entrynamesyntax : u32, entryname : windows_core::PCSTR, ifspec : *const core::ffi::c_void, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingUnexportA(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPA<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objectvector: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportPnPA(entrynamesyntax : u32, entryname : windows_core::PCSTR, ifspec : *const core::ffi::c_void, objectvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingUnexportPnPA(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPW<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objectvector: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportPnPW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, ifspec : *const core::ffi::c_void, objectvector : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingUnexportPnPW(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsBindingUnexportW<P1>(entrynamesyntax: u32, entryname: P1, ifspec: Option<*const core::ffi::c_void>, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, ifspec : *const core::ffi::c_void, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsBindingUnexportW(entrynamesyntax, entryname.param().abi(), ifspec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsEntryExpandNameA<P1>(entrynamesyntax: u32, entryname: P1, expandedname: *mut windows_core::PSTR) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryExpandNameA(entrynamesyntax : u32, entryname : windows_core::PCSTR, expandedname : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcNsEntryExpandNameA(entrynamesyntax, entryname.param().abi(), expandedname as _) }
}
#[inline]
pub unsafe fn RpcNsEntryExpandNameW<P1>(entrynamesyntax: u32, entryname: P1, expandedname: *mut windows_core::PWSTR) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryExpandNameW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, expandedname : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcNsEntryExpandNameW(entrynamesyntax, entryname.param().abi(), expandedname as _) }
}
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginA<P1>(entrynamesyntax: u32, entryname: P1, inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqBeginA(entrynamesyntax : u32, entryname : windows_core::PCSTR, inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsEntryObjectInqBeginA(entrynamesyntax, entryname.param().abi(), inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginW<P1>(entrynamesyntax: u32, entryname: P1, inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqBeginW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsEntryObjectInqBeginW(entrynamesyntax, entryname.param().abi(), inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsEntryObjectInqDone(inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqDone(inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsEntryObjectInqDone(inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsEntryObjectInqNext(inquirycontext: *mut core::ffi::c_void, objuuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqNext(inquirycontext : *mut core::ffi::c_void, objuuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcNsEntryObjectInqNext(inquirycontext as _, objuuid as _) }
}
#[inline]
pub unsafe fn RpcNsGroupDeleteA<P1>(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: P1) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupDeleteA(groupnamesyntax : GROUP_NAME_SYNTAX, groupname : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupDeleteA(groupnamesyntax, groupname.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsGroupDeleteW<P1>(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: P1) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupDeleteW(groupnamesyntax : GROUP_NAME_SYNTAX, groupname : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupDeleteW(groupnamesyntax, groupname.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrAddA<P1, P3>(groupnamesyntax: u32, groupname: P1, membernamesyntax: u32, membername: P3) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrAddA(groupnamesyntax : u32, groupname : windows_core::PCSTR, membernamesyntax : u32, membername : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrAddA(groupnamesyntax, groupname.param().abi(), membernamesyntax, membername.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrAddW<P1, P3>(groupnamesyntax: u32, groupname: P1, membernamesyntax: u32, membername: P3) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrAddW(groupnamesyntax : u32, groupname : windows_core::PCWSTR, membernamesyntax : u32, membername : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrAddW(groupnamesyntax, groupname.param().abi(), membernamesyntax, membername.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginA<P1>(groupnamesyntax: u32, groupname: P1, membernamesyntax: u32, inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqBeginA(groupnamesyntax : u32, groupname : windows_core::PCSTR, membernamesyntax : u32, inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqBeginA(groupnamesyntax, groupname.param().abi(), membernamesyntax, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginW<P1>(groupnamesyntax: u32, groupname: P1, membernamesyntax: u32, inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqBeginW(groupnamesyntax : u32, groupname : windows_core::PCWSTR, membernamesyntax : u32, inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqBeginW(groupnamesyntax, groupname.param().abi(), membernamesyntax, inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqDone(inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqDone(inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqDone(inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextA(inquirycontext: *mut core::ffi::c_void, membername: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqNextA(inquirycontext : *mut core::ffi::c_void, membername : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqNextA(inquirycontext as _, membername as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextW(inquirycontext: *mut core::ffi::c_void, membername: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqNextW(inquirycontext : *mut core::ffi::c_void, membername : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrInqNextW(inquirycontext as _, membername as _) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveA<P1, P3>(groupnamesyntax: u32, groupname: P1, membernamesyntax: u32, membername: P3) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrRemoveA(groupnamesyntax : u32, groupname : windows_core::PCSTR, membernamesyntax : u32, membername : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrRemoveA(groupnamesyntax, groupname.param().abi(), membernamesyntax, membername.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveW<P1, P3>(groupnamesyntax: u32, groupname: P1, membernamesyntax: u32, membername: P3) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrRemoveW(groupnamesyntax : u32, groupname : windows_core::PCWSTR, membernamesyntax : u32, membername : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNsGroupMbrRemoveW(groupnamesyntax, groupname.param().abi(), membernamesyntax, membername.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportA<P1>(entrynamesyntax: u32, entryname: P1, ifid: Option<*const RPC_IF_ID>, versoption: u32, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtBindingUnexportA(entrynamesyntax : u32, entryname : windows_core::PCSTR, ifid : *const RPC_IF_ID, versoption : u32, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsMgmtBindingUnexportA(entrynamesyntax, entryname.param().abi(), ifid.unwrap_or(core::mem::zeroed()) as _, versoption, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportW<P1>(entrynamesyntax: u32, entryname: P1, ifid: Option<*const RPC_IF_ID>, versoption: u32, objectuuidvec: Option<*const UUID_VECTOR>) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtBindingUnexportW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, ifid : *const RPC_IF_ID, versoption : u32, objectuuidvec : *const UUID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsMgmtBindingUnexportW(entrynamesyntax, entryname.param().abi(), ifid.unwrap_or(core::mem::zeroed()) as _, versoption, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateA<P1>(entrynamesyntax: u32, entryname: P1) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryCreateA(entrynamesyntax : u32, entryname : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryCreateA(entrynamesyntax, entryname.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateW<P1>(entrynamesyntax: u32, entryname: P1) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryCreateW(entrynamesyntax : u32, entryname : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryCreateW(entrynamesyntax, entryname.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteA<P1>(entrynamesyntax: u32, entryname: P1) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryDeleteA(entrynamesyntax : u32, entryname : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryDeleteA(entrynamesyntax, entryname.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteW<P1>(entrynamesyntax: u32, entryname: P1) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryDeleteW(entrynamesyntax : u32, entryname : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryDeleteW(entrynamesyntax, entryname.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsA<P1>(entrynamesyntax: u32, entryname: P1, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax : u32, entryname : windows_core::PCSTR, ifidvec : *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryInqIfIdsA(entrynamesyntax, entryname.param().abi(), ifidvec as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsW<P1>(entrynamesyntax: u32, entryname: P1, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax : u32, entryname : windows_core::PCWSTR, ifidvec : *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS);
    unsafe { RpcNsMgmtEntryInqIfIdsW(entrynamesyntax, entryname.param().abi(), ifidvec as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtHandleSetExpAge(nshandle: *mut core::ffi::c_void, expirationage: u32) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtHandleSetExpAge(nshandle : *mut core::ffi::c_void, expirationage : u32) -> RPC_STATUS);
    unsafe { RpcNsMgmtHandleSetExpAge(nshandle as _, expirationage) }
}
#[inline]
pub unsafe fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtInqExpAge(expirationage : *mut u32) -> RPC_STATUS);
    unsafe { RpcNsMgmtInqExpAge(expirationage as _) }
}
#[inline]
pub unsafe fn RpcNsMgmtSetExpAge(expirationage: u32) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtSetExpAge(expirationage : u32) -> RPC_STATUS);
    unsafe { RpcNsMgmtSetExpAge(expirationage) }
}
#[inline]
pub unsafe fn RpcNsProfileDeleteA<P1>(profilenamesyntax: u32, profilename: P1) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileDeleteA(profilenamesyntax : u32, profilename : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileDeleteA(profilenamesyntax, profilename.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsProfileDeleteW<P1>(profilenamesyntax: u32, profilename: P1) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileDeleteW(profilenamesyntax : u32, profilename : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileDeleteW(profilenamesyntax, profilename.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsProfileEltAddA<P1, P4, P6>(profilenamesyntax: u32, profilename: P1, ifid: Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: P4, priority: u32, annotation: P6) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltAddA(profilenamesyntax : u32, profilename : windows_core::PCSTR, ifid : *const RPC_IF_ID, membernamesyntax : u32, membername : windows_core::PCSTR, priority : u32, annotation : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileEltAddA(profilenamesyntax, profilename.param().abi(), ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername.param().abi(), priority, annotation.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsProfileEltAddW<P1, P4, P6>(profilenamesyntax: u32, profilename: P1, ifid: Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: P4, priority: u32, annotation: P6) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltAddW(profilenamesyntax : u32, profilename : windows_core::PCWSTR, ifid : *const RPC_IF_ID, membernamesyntax : u32, membername : windows_core::PCWSTR, priority : u32, annotation : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileEltAddW(profilenamesyntax, profilename.param().abi(), ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername.param().abi(), priority, annotation.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginA<P1, P6>(profilenamesyntax: u32, profilename: P1, inquirytype: u32, ifid: Option<*const RPC_IF_ID>, versoption: u32, membernamesyntax: u32, membername: P6, inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqBeginA(profilenamesyntax : u32, profilename : windows_core::PCSTR, inquirytype : u32, ifid : *const RPC_IF_ID, versoption : u32, membernamesyntax : u32, membername : windows_core::PCSTR, inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqBeginA(profilenamesyntax, profilename.param().abi(), inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, membernamesyntax, membername.param().abi(), inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginW<P1, P6>(profilenamesyntax: u32, profilename: P1, inquirytype: u32, ifid: Option<*const RPC_IF_ID>, versoption: u32, membernamesyntax: u32, membername: P6, inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqBeginW(profilenamesyntax : u32, profilename : windows_core::PCWSTR, inquirytype : u32, ifid : *const RPC_IF_ID, versoption : u32, membernamesyntax : u32, membername : windows_core::PCWSTR, inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqBeginW(profilenamesyntax, profilename.param().abi(), inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, membernamesyntax, membername.param().abi(), inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqDone(inquirycontext: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqDone(inquirycontext : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqDone(inquirycontext as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqNextA(inquirycontext: *const core::ffi::c_void, ifid: Option<*mut RPC_IF_ID>, membername: *mut windows_core::PSTR, priority: *mut u32, annotation: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqNextA(inquirycontext : *const core::ffi::c_void, ifid : *mut RPC_IF_ID, membername : *mut windows_core::PSTR, priority : *mut u32, annotation : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqNextA(inquirycontext, ifid.unwrap_or(core::mem::zeroed()) as _, membername as _, priority as _, annotation as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltInqNextW(inquirycontext: *const core::ffi::c_void, ifid: Option<*mut RPC_IF_ID>, membername: *mut windows_core::PWSTR, priority: *mut u32, annotation: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqNextW(inquirycontext : *const core::ffi::c_void, ifid : *mut RPC_IF_ID, membername : *mut windows_core::PWSTR, priority : *mut u32, annotation : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileEltInqNextW(inquirycontext, ifid.unwrap_or(core::mem::zeroed()) as _, membername as _, priority as _, annotation as _) }
}
#[inline]
pub unsafe fn RpcNsProfileEltRemoveA<P1, P4>(profilenamesyntax: u32, profilename: P1, ifid: Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: P4) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltRemoveA(profilenamesyntax : u32, profilename : windows_core::PCSTR, ifid : *const RPC_IF_ID, membernamesyntax : u32, membername : windows_core::PCSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileEltRemoveA(profilenamesyntax, profilename.param().abi(), ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername.param().abi()) }
}
#[inline]
pub unsafe fn RpcNsProfileEltRemoveW<P1, P4>(profilenamesyntax: u32, profilename: P1, ifid: Option<*const RPC_IF_ID>, membernamesyntax: u32, membername: P4) -> RPC_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltRemoveW(profilenamesyntax : u32, profilename : windows_core::PCWSTR, ifid : *const RPC_IF_ID, membernamesyntax : u32, membername : windows_core::PCWSTR) -> RPC_STATUS);
    unsafe { RpcNsProfileEltRemoveW(profilenamesyntax, profilename.param().abi(), ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername.param().abi()) }
}
#[inline]
pub unsafe fn RpcObjectInqType(objuuid: *const windows_core::GUID, typeuuid: Option<*mut windows_core::GUID>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectInqType(objuuid : *const windows_core::GUID, typeuuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcObjectInqType(objuuid, typeuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcObjectSetInqFn(inquiryfn: RPC_OBJECT_INQ_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectSetInqFn(inquiryfn : RPC_OBJECT_INQ_FN) -> RPC_STATUS);
    unsafe { RpcObjectSetInqFn(inquiryfn) }
}
#[inline]
pub unsafe fn RpcObjectSetType(objuuid: *const windows_core::GUID, typeuuid: Option<*const windows_core::GUID>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectSetType(objuuid : *const windows_core::GUID, typeuuid : *const windows_core::GUID) -> RPC_STATUS);
    unsafe { RpcObjectSetType(objuuid, typeuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcProtseqVectorFreeA(protseqvector : *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS);
    unsafe { RpcProtseqVectorFreeA(protseqvector as _) }
}
#[inline]
pub unsafe fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcProtseqVectorFreeW(protseqvector : *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS);
    unsafe { RpcProtseqVectorFreeW(protseqvector as _) }
}
#[inline]
pub unsafe fn RpcRaiseException(exception: RPC_STATUS) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRaiseException(exception : RPC_STATUS));
    unsafe { RpcRaiseException(exception) }
}
#[inline]
pub unsafe fn RpcRevertContainerImpersonation() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertContainerImpersonation() -> RPC_STATUS);
    unsafe { RpcRevertContainerImpersonation() }
}
#[inline]
pub unsafe fn RpcRevertToSelf() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertToSelf() -> RPC_STATUS);
    unsafe { RpcRevertToSelf() }
}
#[inline]
pub unsafe fn RpcRevertToSelfEx(bindinghandle: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertToSelfEx(bindinghandle : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcRevertToSelfEx(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerCompleteSecurityCallback(bindinghandle: *const core::ffi::c_void, status: RPC_STATUS) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerCompleteSecurityCallback(bindinghandle : *const core::ffi::c_void, status : RPC_STATUS) -> RPC_STATUS);
    unsafe { RpcServerCompleteSecurityCallback(bindinghandle, status) }
}
#[inline]
pub unsafe fn RpcServerInqBindingHandle(binding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindingHandle(binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInqBindingHandle(binding as _) }
}
#[inline]
pub unsafe fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindings(bindingvector : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcServerInqBindings(bindingvector as _) }
}
#[inline]
pub unsafe fn RpcServerInqBindingsEx(securitydescriptor: Option<*const core::ffi::c_void>, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindingsEx(securitydescriptor : *const core::ffi::c_void, bindingvector : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcServerInqBindingsEx(securitydescriptor.unwrap_or(core::mem::zeroed()) as _, bindingvector as _) }
}
#[inline]
pub unsafe fn RpcServerInqCallAttributesA(clientbinding: Option<*const core::ffi::c_void>, rpccallattributes: *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqCallAttributesA(clientbinding : *const core::ffi::c_void, rpccallattributes : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInqCallAttributesA(clientbinding.unwrap_or(core::mem::zeroed()) as _, rpccallattributes as _) }
}
#[inline]
pub unsafe fn RpcServerInqCallAttributesW(clientbinding: Option<*const core::ffi::c_void>, rpccallattributes: *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqCallAttributesW(clientbinding : *const core::ffi::c_void, rpccallattributes : *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInqCallAttributesW(clientbinding.unwrap_or(core::mem::zeroed()) as _, rpccallattributes as _) }
}
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqDefaultPrincNameA(authnsvc : u32, princname : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcServerInqDefaultPrincNameA(authnsvc, princname as _) }
}
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqDefaultPrincNameW(authnsvc : u32, princname : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcServerInqDefaultPrincNameW(authnsvc, princname as _) }
}
#[inline]
pub unsafe fn RpcServerInqIf(ifspec: *const core::ffi::c_void, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqIf(ifspec : *const core::ffi::c_void, mgrtypeuuid : *const windows_core::GUID, mgrepv : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInqIf(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv as _) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupActivate(ifgroup: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupActivate(ifgroup : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupActivate(ifgroup) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupClose(ifgroup: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupClose(ifgroup : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupClose(ifgroup) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateA(interfaces: &[RPC_INTERFACE_TEMPLATEA], endpoints: &[RPC_ENDPOINT_TEMPLATEA], idleperiod: u32, idlecallbackfn: RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext: *const core::ffi::c_void, ifgroup: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupCreateA(interfaces : *const RPC_INTERFACE_TEMPLATEA, numifs : u32, endpoints : *const RPC_ENDPOINT_TEMPLATEA, numendpoints : u32, idleperiod : u32, idlecallbackfn : RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext : *const core::ffi::c_void, ifgroup : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupCreateA(core::mem::transmute(interfaces.as_ptr()), interfaces.len().try_into().unwrap(), core::mem::transmute(endpoints.as_ptr()), endpoints.len().try_into().unwrap(), idleperiod, idlecallbackfn, idlecallbackcontext, ifgroup as _) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateW(interfaces: &[RPC_INTERFACE_TEMPLATEW], endpoints: &[RPC_ENDPOINT_TEMPLATEW], idleperiod: u32, idlecallbackfn: RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext: *const core::ffi::c_void, ifgroup: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupCreateW(interfaces : *const RPC_INTERFACE_TEMPLATEW, numifs : u32, endpoints : *const RPC_ENDPOINT_TEMPLATEW, numendpoints : u32, idleperiod : u32, idlecallbackfn : RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext : *const core::ffi::c_void, ifgroup : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupCreateW(core::mem::transmute(interfaces.as_ptr()), interfaces.len().try_into().unwrap(), core::mem::transmute(endpoints.as_ptr()), endpoints.len().try_into().unwrap(), idleperiod, idlecallbackfn, idlecallbackcontext, ifgroup as _) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupDeactivate(ifgroup: *const core::ffi::c_void, forcedeactivation: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupDeactivate(ifgroup : *const core::ffi::c_void, forcedeactivation : u32) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupDeactivate(ifgroup, forcedeactivation) }
}
#[inline]
pub unsafe fn RpcServerInterfaceGroupInqBindings(ifgroup: *const core::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupInqBindings(ifgroup : *const core::ffi::c_void, bindingvector : *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS);
    unsafe { RpcServerInterfaceGroupInqBindings(ifgroup, bindingvector as _) }
}
#[inline]
pub unsafe fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerListen(minimumcallthreads : u32, maxcalls : u32, dontwait : u32) -> RPC_STATUS);
    unsafe { RpcServerListen(minimumcallthreads, maxcalls, dontwait) }
}
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoA<P0>(serverprincname: P0, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: Option<*const core::ffi::c_void>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterAuthInfoA(serverprincname : windows_core::PCSTR, authnsvc : u32, getkeyfn : RPC_AUTH_KEY_RETRIEVAL_FN, arg : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerRegisterAuthInfoA(serverprincname.param().abi(), authnsvc, getkeyfn, arg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoW<P0>(serverprincname: P0, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: Option<*const core::ffi::c_void>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterAuthInfoW(serverprincname : windows_core::PCWSTR, authnsvc : u32, getkeyfn : RPC_AUTH_KEY_RETRIEVAL_FN, arg : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerRegisterAuthInfoW(serverprincname.param().abi(), authnsvc, getkeyfn, arg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerRegisterIf(ifspec: *const core::ffi::c_void, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf(ifspec : *const core::ffi::c_void, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerRegisterIf(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerRegisterIf2(ifspec: *const core::ffi::c_void, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: RPC_IF_CALLBACK_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf2(ifspec : *const core::ffi::c_void, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, maxrpcsize : u32, ifcallbackfn : RPC_IF_CALLBACK_FN) -> RPC_STATUS);
    unsafe { RpcServerRegisterIf2(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, maxrpcsize, ifcallbackfn) }
}
#[inline]
pub unsafe fn RpcServerRegisterIf3(ifspec: *const core::ffi::c_void, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: RPC_IF_CALLBACK_FN, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf3(ifspec : *const core::ffi::c_void, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, maxrpcsize : u32, ifcallback : RPC_IF_CALLBACK_FN, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerRegisterIf3(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, maxrpcsize, ifcallback, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerRegisterIfEx(ifspec: *const core::ffi::c_void, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, ifcallback: RPC_IF_CALLBACK_FN) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIfEx(ifspec : *const core::ffi::c_void, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, ifcallback : RPC_IF_CALLBACK_FN) -> RPC_STATUS);
    unsafe { RpcServerRegisterIfEx(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, ifcallback) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn RpcServerSubscribeForNotification(binding: Option<*const core::ffi::c_void>, notification: RPC_NOTIFICATIONS, notificationtype: RPC_NOTIFICATION_TYPES, notificationinfo: *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerSubscribeForNotification(binding : *const core::ffi::c_void, notification : RPC_NOTIFICATIONS, notificationtype : RPC_NOTIFICATION_TYPES, notificationinfo : *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS);
    unsafe { RpcServerSubscribeForNotification(binding.unwrap_or(core::mem::zeroed()) as _, notification, notificationtype, notificationinfo) }
}
#[inline]
pub unsafe fn RpcServerTestCancel(bindinghandle: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerTestCancel(bindinghandle : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerTestCancel(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUnregisterIf(ifspec: Option<*const core::ffi::c_void>, mgrtypeuuid: Option<*const windows_core::GUID>, waitforcallstocomplete: u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUnregisterIf(ifspec : *const core::ffi::c_void, mgrtypeuuid : *const windows_core::GUID, waitforcallstocomplete : u32) -> RPC_STATUS);
    unsafe { RpcServerUnregisterIf(ifspec.unwrap_or(core::mem::zeroed()) as _, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, waitforcallstocomplete) }
}
#[inline]
pub unsafe fn RpcServerUnregisterIfEx(ifspec: Option<*const core::ffi::c_void>, mgrtypeuuid: Option<*const windows_core::GUID>, rundowncontexthandles: i32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUnregisterIfEx(ifspec : *const core::ffi::c_void, mgrtypeuuid : *const windows_core::GUID, rundowncontexthandles : i32) -> RPC_STATUS);
    unsafe { RpcServerUnregisterIfEx(ifspec.unwrap_or(core::mem::zeroed()) as _, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, rundowncontexthandles) }
}
#[inline]
pub unsafe fn RpcServerUnsubscribeForNotification(binding: Option<*const core::ffi::c_void>, notification: RPC_NOTIFICATIONS, notificationsqueued: *mut u32) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUnsubscribeForNotification(binding : *const core::ffi::c_void, notification : RPC_NOTIFICATIONS, notificationsqueued : *mut u32) -> RPC_STATUS);
    unsafe { RpcServerUnsubscribeForNotification(binding.unwrap_or(core::mem::zeroed()) as _, notification, notificationsqueued as _) }
}
#[inline]
pub unsafe fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqs(maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseAllProtseqs(maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsEx(maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsEx(maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: *const core::ffi::c_void, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsIf(maxcalls : u32, ifspec : *const core::ffi::c_void, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsIf(maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: *const core::ffi::c_void, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsIfEx(maxcalls : u32, ifspec : *const core::ffi::c_void, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsIfEx(maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqA<P0>(protseq: P0, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqA(protseq : windows_core::PCSTR, maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqA(protseq.param().abi(), maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqEpA<P0, P2>(protseq: P0, maxcalls: u32, endpoint: P2, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpA(protseq : windows_core::PCSTR, maxcalls : u32, endpoint : windows_core::PCSTR, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqEpA(protseq.param().abi(), maxcalls, endpoint.param().abi(), securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqEpExA<P0, P2>(protseq: P0, maxcalls: u32, endpoint: P2, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpExA(protseq : windows_core::PCSTR, maxcalls : u32, endpoint : windows_core::PCSTR, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqEpExA(protseq.param().abi(), maxcalls, endpoint.param().abi(), securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqEpExW<P0, P2>(protseq: P0, maxcalls: u32, endpoint: P2, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpExW(protseq : windows_core::PCWSTR, maxcalls : u32, endpoint : windows_core::PCWSTR, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqEpExW(protseq.param().abi(), maxcalls, endpoint.param().abi(), securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqEpW<P0, P2>(protseq: P0, maxcalls: u32, endpoint: P2, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpW(protseq : windows_core::PCWSTR, maxcalls : u32, endpoint : windows_core::PCWSTR, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqEpW(protseq.param().abi(), maxcalls, endpoint.param().abi(), securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqExA<P0>(protseq: P0, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqExA(protseq : windows_core::PCSTR, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqExA(protseq.param().abi(), maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqExW<P0>(protseq: P0, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqExW(protseq : windows_core::PCWSTR, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqExW(protseq.param().abi(), maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqIfA<P0>(protseq: P0, maxcalls: u32, ifspec: *const core::ffi::c_void, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfA(protseq : windows_core::PCSTR, maxcalls : u32, ifspec : *const core::ffi::c_void, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqIfA(protseq.param().abi(), maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqIfExA<P0>(protseq: P0, maxcalls: u32, ifspec: *const core::ffi::c_void, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfExA(protseq : windows_core::PCSTR, maxcalls : u32, ifspec : *const core::ffi::c_void, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqIfExA(protseq.param().abi(), maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqIfExW<P0>(protseq: P0, maxcalls: u32, ifspec: *const core::ffi::c_void, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfExW(protseq : windows_core::PCWSTR, maxcalls : u32, ifspec : *const core::ffi::c_void, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqIfExW(protseq.param().abi(), maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqIfW<P0>(protseq: P0, maxcalls: u32, ifspec: *const core::ffi::c_void, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfW(protseq : windows_core::PCWSTR, maxcalls : u32, ifspec : *const core::ffi::c_void, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqIfW(protseq.param().abi(), maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerUseProtseqW<P0>(protseq: P0, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqW(protseq : windows_core::PCWSTR, maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcServerUseProtseqW(protseq.param().abi(), maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerYield() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerYield());
    unsafe { RpcServerYield() }
}
#[inline]
pub unsafe fn RpcSmAllocate(size: usize, pstatus: *mut RPC_STATUS) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmAllocate(size : usize, pstatus : *mut RPC_STATUS) -> *mut core::ffi::c_void);
    unsafe { RpcSmAllocate(size, pstatus as _) }
}
#[inline]
pub unsafe fn RpcSmClientFree(pnodetofree: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmClientFree(pnodetofree : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSmClientFree(pnodetofree) }
}
#[inline]
pub unsafe fn RpcSmDestroyClientContext(contexthandle: *const *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmDestroyClientContext(contexthandle : *const *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSmDestroyClientContext(contexthandle) }
}
#[inline]
pub unsafe fn RpcSmDisableAllocate() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmDisableAllocate() -> RPC_STATUS);
    unsafe { RpcSmDisableAllocate() }
}
#[inline]
pub unsafe fn RpcSmEnableAllocate() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmEnableAllocate() -> RPC_STATUS);
    unsafe { RpcSmEnableAllocate() }
}
#[inline]
pub unsafe fn RpcSmFree(nodetofree: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmFree(nodetofree : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSmFree(nodetofree) }
}
#[inline]
pub unsafe fn RpcSmGetThreadHandle(pstatus: *mut RPC_STATUS) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmGetThreadHandle(pstatus : *mut RPC_STATUS) -> *mut core::ffi::c_void);
    unsafe { RpcSmGetThreadHandle(pstatus as _) }
}
#[inline]
pub unsafe fn RpcSmSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSetClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE) -> RPC_STATUS);
    unsafe { RpcSmSetClientAllocFree(clientalloc, clientfree) }
}
#[inline]
pub unsafe fn RpcSmSetThreadHandle(id: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSetThreadHandle(id : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSmSetThreadHandle(id) }
}
#[inline]
pub unsafe fn RpcSmSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSwapClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE, oldclientalloc : *mut RPC_CLIENT_ALLOC, oldclientfree : *mut RPC_CLIENT_FREE) -> RPC_STATUS);
    unsafe { RpcSmSwapClientAllocFree(clientalloc, clientfree, oldclientalloc as _, oldclientfree as _) }
}
#[inline]
pub unsafe fn RpcSsAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { RpcSsAllocate(size) }
}
#[inline]
pub unsafe fn RpcSsContextLockExclusive(serverbindinghandle: Option<*const core::ffi::c_void>, usercontext: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsContextLockExclusive(serverbindinghandle : *const core::ffi::c_void, usercontext : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSsContextLockExclusive(serverbindinghandle.unwrap_or(core::mem::zeroed()) as _, usercontext) }
}
#[inline]
pub unsafe fn RpcSsContextLockShared(serverbindinghandle: *const core::ffi::c_void, usercontext: *const core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsContextLockShared(serverbindinghandle : *const core::ffi::c_void, usercontext : *const core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSsContextLockShared(serverbindinghandle, usercontext) }
}
#[inline]
pub unsafe fn RpcSsDestroyClientContext(contexthandle: *const *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDestroyClientContext(contexthandle : *const *const core::ffi::c_void));
    unsafe { RpcSsDestroyClientContext(contexthandle) }
}
#[inline]
pub unsafe fn RpcSsDisableAllocate() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDisableAllocate());
    unsafe { RpcSsDisableAllocate() }
}
#[inline]
pub unsafe fn RpcSsDontSerializeContext() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDontSerializeContext());
    unsafe { RpcSsDontSerializeContext() }
}
#[inline]
pub unsafe fn RpcSsEnableAllocate() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsEnableAllocate());
    unsafe { RpcSsEnableAllocate() }
}
#[inline]
pub unsafe fn RpcSsFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsFree(nodetofree : *const core::ffi::c_void));
    unsafe { RpcSsFree(nodetofree) }
}
#[inline]
pub unsafe fn RpcSsGetContextBinding(contexthandle: *const core::ffi::c_void, binding: *mut *mut core::ffi::c_void) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsGetContextBinding(contexthandle : *const core::ffi::c_void, binding : *mut *mut core::ffi::c_void) -> RPC_STATUS);
    unsafe { RpcSsGetContextBinding(contexthandle, binding as _) }
}
#[inline]
pub unsafe fn RpcSsGetThreadHandle() -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsGetThreadHandle() -> *mut core::ffi::c_void);
    unsafe { RpcSsGetThreadHandle() }
}
#[inline]
pub unsafe fn RpcSsSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSetClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE));
    unsafe { RpcSsSetClientAllocFree(clientalloc, clientfree) }
}
#[inline]
pub unsafe fn RpcSsSetThreadHandle(id: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSetThreadHandle(id : *const core::ffi::c_void));
    unsafe { RpcSsSetThreadHandle(id) }
}
#[inline]
pub unsafe fn RpcSsSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSwapClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE, oldclientalloc : *mut RPC_CLIENT_ALLOC, oldclientfree : *mut RPC_CLIENT_FREE));
    unsafe { RpcSsSwapClientAllocFree(clientalloc, clientfree, oldclientalloc as _, oldclientfree as _) }
}
#[inline]
pub unsafe fn RpcStringBindingComposeA<P0, P1, P2, P3, P4>(objuuid: P0, protseq: P1, networkaddr: P2, endpoint: P3, options: P4, stringbinding: Option<*mut windows_core::PSTR>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingComposeA(objuuid : windows_core::PCSTR, protseq : windows_core::PCSTR, networkaddr : windows_core::PCSTR, endpoint : windows_core::PCSTR, options : windows_core::PCSTR, stringbinding : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcStringBindingComposeA(objuuid.param().abi(), protseq.param().abi(), networkaddr.param().abi(), endpoint.param().abi(), options.param().abi(), stringbinding.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcStringBindingComposeW<P0, P1, P2, P3, P4>(objuuid: P0, protseq: P1, networkaddr: P2, endpoint: P3, options: P4, stringbinding: Option<*mut windows_core::PWSTR>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingComposeW(objuuid : windows_core::PCWSTR, protseq : windows_core::PCWSTR, networkaddr : windows_core::PCWSTR, endpoint : windows_core::PCWSTR, options : windows_core::PCWSTR, stringbinding : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcStringBindingComposeW(objuuid.param().abi(), protseq.param().abi(), networkaddr.param().abi(), endpoint.param().abi(), options.param().abi(), stringbinding.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcStringBindingParseA<P0>(stringbinding: P0, objuuid: Option<*mut windows_core::PSTR>, protseq: Option<*mut windows_core::PSTR>, networkaddr: Option<*mut windows_core::PSTR>, endpoint: Option<*mut windows_core::PSTR>, networkoptions: Option<*mut windows_core::PSTR>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingParseA(stringbinding : windows_core::PCSTR, objuuid : *mut windows_core::PSTR, protseq : *mut windows_core::PSTR, networkaddr : *mut windows_core::PSTR, endpoint : *mut windows_core::PSTR, networkoptions : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcStringBindingParseA(stringbinding.param().abi(), objuuid.unwrap_or(core::mem::zeroed()) as _, protseq.unwrap_or(core::mem::zeroed()) as _, networkaddr.unwrap_or(core::mem::zeroed()) as _, endpoint.unwrap_or(core::mem::zeroed()) as _, networkoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcStringBindingParseW<P0>(stringbinding: P0, objuuid: Option<*mut windows_core::PWSTR>, protseq: Option<*mut windows_core::PWSTR>, networkaddr: Option<*mut windows_core::PWSTR>, endpoint: Option<*mut windows_core::PWSTR>, networkoptions: Option<*mut windows_core::PWSTR>) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingParseW(stringbinding : windows_core::PCWSTR, objuuid : *mut windows_core::PWSTR, protseq : *mut windows_core::PWSTR, networkaddr : *mut windows_core::PWSTR, endpoint : *mut windows_core::PWSTR, networkoptions : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcStringBindingParseW(stringbinding.param().abi(), objuuid.unwrap_or(core::mem::zeroed()) as _, protseq.unwrap_or(core::mem::zeroed()) as _, networkaddr.unwrap_or(core::mem::zeroed()) as _, endpoint.unwrap_or(core::mem::zeroed()) as _, networkoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcStringFreeA(string: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringFreeA(string : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { RpcStringFreeA(string as _) }
}
#[inline]
pub unsafe fn RpcStringFreeW(string: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringFreeW(string : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { RpcStringFreeW(string as _) }
}
#[inline]
pub unsafe fn RpcTestCancel() -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcTestCancel() -> RPC_STATUS);
    unsafe { RpcTestCancel() }
}
#[inline]
pub unsafe fn RpcUserFree(asynchandle: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcUserFree(asynchandle : *mut core::ffi::c_void, pbuffer : *mut core::ffi::c_void));
    unsafe { RpcUserFree(asynchandle as _, pbuffer as _) }
}
#[inline]
pub unsafe fn UuidCompare(uuid1: *const windows_core::GUID, uuid2: *const windows_core::GUID, status: *mut RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCompare(uuid1 : *const windows_core::GUID, uuid2 : *const windows_core::GUID, status : *mut RPC_STATUS) -> i32);
    unsafe { UuidCompare(uuid1, uuid2, status as _) }
}
#[inline]
pub unsafe fn UuidCreate(uuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreate(uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidCreate(uuid as _) }
}
#[inline]
pub unsafe fn UuidCreateNil(niluuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreateNil(niluuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidCreateNil(niluuid as _) }
}
#[inline]
pub unsafe fn UuidCreateSequential(uuid: *mut windows_core::GUID) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreateSequential(uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidCreateSequential(uuid as _) }
}
#[inline]
pub unsafe fn UuidEqual(uuid1: *const windows_core::GUID, uuid2: *const windows_core::GUID, status: *mut RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidEqual(uuid1 : *const windows_core::GUID, uuid2 : *const windows_core::GUID, status : *mut RPC_STATUS) -> i32);
    unsafe { UuidEqual(uuid1, uuid2, status as _) }
}
#[inline]
pub unsafe fn UuidFromStringA<P0>(stringuuid: P0, uuid: *mut windows_core::GUID) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn UuidFromStringA(stringuuid : windows_core::PCSTR, uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidFromStringA(stringuuid.param().abi(), uuid as _) }
}
#[inline]
pub unsafe fn UuidFromStringW<P0>(stringuuid: P0, uuid: *mut windows_core::GUID) -> RPC_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rpcrt4.dll" "system" fn UuidFromStringW(stringuuid : windows_core::PCWSTR, uuid : *mut windows_core::GUID) -> RPC_STATUS);
    unsafe { UuidFromStringW(stringuuid.param().abi(), uuid as _) }
}
#[inline]
pub unsafe fn UuidHash(uuid: *const windows_core::GUID, status: *mut RPC_STATUS) -> u16 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidHash(uuid : *const windows_core::GUID, status : *mut RPC_STATUS) -> u16);
    unsafe { UuidHash(uuid, status as _) }
}
#[inline]
pub unsafe fn UuidIsNil(uuid: *const windows_core::GUID, status: *mut RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidIsNil(uuid : *const windows_core::GUID, status : *mut RPC_STATUS) -> i32);
    unsafe { UuidIsNil(uuid, status as _) }
}
#[inline]
pub unsafe fn UuidToStringA(uuid: *const windows_core::GUID, stringuuid: *mut windows_core::PSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidToStringA(uuid : *const windows_core::GUID, stringuuid : *mut windows_core::PSTR) -> RPC_STATUS);
    unsafe { UuidToStringA(uuid, stringuuid as _) }
}
#[inline]
pub unsafe fn UuidToStringW(uuid: *const windows_core::GUID, stringuuid: *mut windows_core::PWSTR) -> RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidToStringW(uuid : *const windows_core::GUID, stringuuid : *mut windows_core::PWSTR) -> RPC_STATUS);
    unsafe { UuidToStringW(uuid, stringuuid as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ARRAY_INFO {
    pub Dimension: i32,
    pub BufferConformanceMark: *mut u32,
    pub BufferVarianceMark: *mut u32,
    pub MaxCountArray: *mut u32,
    pub OffsetArray: *mut u32,
    pub ActualCountArray: *mut u32,
}
impl Default for ARRAY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BinaryParam {
    pub Buffer: *mut core::ffi::c_void,
    pub Size: i16,
}
impl Default for BinaryParam {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLIENT_CALL_RETURN {
    pub Pointer: *mut core::ffi::c_void,
    pub Simple: isize,
}
impl Default for CLIENT_CALL_RETURN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMM_FAULT_OFFSETS {
    pub CommOffset: i16,
    pub FaultOffset: i16,
}
pub type CS_TAG_GETTING_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, fserverside: i32, pulsendingtag: *mut u32, puldesiredreceivingtag: *mut u32, pulreceivingtag: *mut u32, pstatus: *mut u32)>;
pub type CS_TYPE_FROM_NETCS_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, ulnetworkcodeset: u32, pnetworkdata: *mut u8, ulnetworkdatalength: u32, ullocalbuffersize: u32, plocaldata: *mut core::ffi::c_void, pullocaldatalength: *mut u32, pstatus: *mut u32)>;
pub type CS_TYPE_LOCAL_SIZE_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, ulnetworkcodeset: u32, ulnetworkbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pullocalbuffersize: *mut u32, pstatus: *mut u32)>;
pub type CS_TYPE_NET_SIZE_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, ulnetworkcodeset: u32, ullocalbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pulnetworkbuffersize: *mut u32, pstatus: *mut u32)>;
pub type CS_TYPE_TO_NETCS_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, ulnetworkcodeset: u32, plocaldata: *mut core::ffi::c_void, ullocaldatalength: u32, pnetworkdata: *mut u8, pulnetworkdatalength: *mut u32, pstatus: *mut u32)>;
pub const DCE_C_ERROR_STRING_LEN: u32 = 256u32;
pub const EEInfoGCCOM: u32 = 11u32;
pub const EEInfoGCFRS: u32 = 12u32;
pub const EEInfoNextRecordsMissing: u32 = 2u32;
pub const EEInfoPreviousRecordsMissing: u32 = 1u32;
pub const EEInfoUseFileTime: u32 = 4u32;
pub const EPT_S_CANT_CREATE: RPC_STATUS = RPC_STATUS(1899i32);
pub const EPT_S_CANT_PERFORM_OP: RPC_STATUS = RPC_STATUS(1752i32);
pub const EPT_S_INVALID_ENTRY: RPC_STATUS = RPC_STATUS(1751i32);
pub const EPT_S_NOT_REGISTERED: RPC_STATUS = RPC_STATUS(1753i32);
#[cfg(feature = "Win32_System_Com")]
pub type EXPR_EVAL = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPR_TOKEN(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExtendedErrorParamTypes(pub i32);
pub const FC_EXPR_CONST32: EXPR_TOKEN = EXPR_TOKEN(1i32);
pub const FC_EXPR_CONST64: EXPR_TOKEN = EXPR_TOKEN(2i32);
pub const FC_EXPR_END: EXPR_TOKEN = EXPR_TOKEN(6i32);
pub const FC_EXPR_ILLEGAL: EXPR_TOKEN = EXPR_TOKEN(0i32);
pub const FC_EXPR_NOOP: EXPR_TOKEN = EXPR_TOKEN(5i32);
pub const FC_EXPR_OPER: EXPR_TOKEN = EXPR_TOKEN(4i32);
pub const FC_EXPR_START: EXPR_TOKEN = EXPR_TOKEN(0i32);
pub const FC_EXPR_VAR: EXPR_TOKEN = EXPR_TOKEN(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FULL_PTR_XLAT_TABLES {
    pub RefIdToPointer: *mut core::ffi::c_void,
    pub PointerToRefId: *mut core::ffi::c_void,
    pub NextRefId: u32,
    pub XlatSide: XLAT_SIDE,
}
impl Default for FULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GENERIC_BINDING_INFO {
    pub pObj: *mut core::ffi::c_void,
    pub Size: u32,
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl Default for GENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type GENERIC_BINDING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GENERIC_BINDING_ROUTINE_PAIR {
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
pub type GENERIC_UNBIND_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut u8)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_NAME_SYNTAX(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDL_CS_CONVERT(pub i32);
pub const IDL_CS_IN_PLACE_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(1i32);
pub const IDL_CS_NEW_BUFFER_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(2i32);
pub const IDL_CS_NO_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(0i32);
pub const INVALID_FRAGMENT_ID: u32 = 0u32;
pub type I_RpcFreeCalloutStateFn = Option<unsafe extern "system" fn(calloutstate: *mut RDR_CALLOUT_STATE)>;
pub type I_RpcPerformCalloutFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, calloutstate: *mut RDR_CALLOUT_STATE, stage: RPC_HTTP_REDIRECTOR_STAGE) -> RPC_STATUS>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct I_RpcProxyCallbackInterface {
    pub IsValidMachineFn: I_RpcProxyIsValidMachineFn,
    pub GetClientAddressFn: I_RpcProxyGetClientAddressFn,
    pub GetConnectionTimeoutFn: I_RpcProxyGetConnectionTimeoutFn,
    pub PerformCalloutFn: I_RpcPerformCalloutFn,
    pub FreeCalloutStateFn: I_RpcFreeCalloutStateFn,
    pub GetClientSessionAndResourceUUIDFn: I_RpcProxyGetClientSessionAndResourceUUID,
    pub ProxyFilterIfFn: I_RpcProxyFilterIfFn,
    pub RpcProxyUpdatePerfCounterFn: I_RpcProxyUpdatePerfCounterFn,
    pub RpcProxyUpdatePerfCounterBackendServerFn: I_RpcProxyUpdatePerfCounterBackendServerFn,
}
pub type I_RpcProxyFilterIfFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifuuid: *const windows_core::GUID, ifmajorversion: u16, fallow: *mut i32) -> RPC_STATUS>;
pub type I_RpcProxyGetClientAddressFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, buffer: windows_core::PCSTR, bufferlength: *mut u32) -> RPC_STATUS>;
pub type I_RpcProxyGetClientSessionAndResourceUUID = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionidpresent: *mut i32, sessionid: *mut windows_core::GUID, resourceidpresent: *mut i32, resourceid: *mut windows_core::GUID) -> RPC_STATUS>;
pub type I_RpcProxyGetConnectionTimeoutFn = Option<unsafe extern "system" fn(connectiontimeout: *mut u32) -> RPC_STATUS>;
pub type I_RpcProxyIsValidMachineFn = Option<unsafe extern "system" fn(machine: windows_core::PCWSTR, dotmachine: windows_core::PCWSTR, portnumber: u32) -> RPC_STATUS>;
pub type I_RpcProxyUpdatePerfCounterBackendServerFn = Option<unsafe extern "system" fn(machinename: *const u16, isconnectevent: i32)>;
pub type I_RpcProxyUpdatePerfCounterFn = Option<unsafe extern "system" fn(counter: RpcPerfCounters, modifytrend: i32, size: u32)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MALLOC_FREE_STRUCT {
    pub pfnAllocate: isize,
    pub pfnFree: isize,
}
pub const MES_DECODE: MIDL_ES_CODE = MIDL_ES_CODE(1i32);
pub const MES_DYNAMIC_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(2i32);
pub const MES_ENCODE: MIDL_ES_CODE = MIDL_ES_CODE(0i32);
pub const MES_ENCODE_NDR64: MIDL_ES_CODE = MIDL_ES_CODE(2i32);
pub const MES_FIXED_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(1i32);
pub const MES_INCREMENTAL_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(0i32);
pub type MIDL_ES_ALLOC = Option<unsafe extern "system" fn(state: *mut core::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIDL_ES_CODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIDL_ES_HANDLE_STYLE(pub i32);
pub type MIDL_ES_READ = Option<unsafe extern "system" fn(state: *mut core::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32)>;
pub type MIDL_ES_WRITE = Option<unsafe extern "system" fn(state: *mut core::ffi::c_void, buffer: windows_core::PCSTR, size: u32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_FORMAT_STRING {
    pub Pad: i16,
    pub Format: [u8; 1],
}
impl Default for MIDL_FORMAT_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_INTERCEPTION_INFO {
    pub Version: u32,
    pub ProcString: *mut u8,
    pub ProcFormatOffsetTable: *const u16,
    pub ProcCount: u32,
    pub TypeString: *mut u8,
}
impl Default for MIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_INTERFACE_METHOD_PROPERTIES {
    pub MethodCount: u16,
    pub MethodProperties: *const *const MIDL_METHOD_PROPERTY_MAP,
}
impl Default for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIDL_METHOD_PROPERTY {
    pub Id: u32,
    pub Value: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_METHOD_PROPERTY_MAP {
    pub Count: u32,
    pub Properties: *const MIDL_METHOD_PROPERTY,
}
impl Default for MIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_SERVER_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub DispatchTable: *const SERVER_ROUTINE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *const u16,
    pub ThunkTable: *const STUB_THUNK,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_STUBLESS_PROXY_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub ProcFormatString: *mut u8,
    pub FormatStringOffset: *const u16,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct MIDL_STUB_DESC {
    pub RpcInterfaceInformation: *mut core::ffi::c_void,
    pub pfnAllocate: PFN_RPC_ALLOCATE,
    pub pfnFree: PFN_RPC_FREE,
    pub IMPLICIT_HANDLE_INFO: MIDL_STUB_DESC_0,
    pub apfnNdrRundownRoutines: *const NDR_RUNDOWN,
    pub aGenericBindingRoutinePairs: *const GENERIC_BINDING_ROUTINE_PAIR,
    pub apfnExprEval: *const EXPR_EVAL,
    pub aXmitQuintuple: *const XMIT_ROUTINE_QUINTUPLE,
    pub pFormatTypes: *const u8,
    pub fCheckBounds: i32,
    pub Version: u32,
    pub pMallocFreeStruct: *mut MALLOC_FREE_STRUCT,
    pub MIDLVersion: i32,
    pub CommFaultOffsets: *const COMM_FAULT_OFFSETS,
    pub aUserMarshalQuadruple: *const USER_MARSHAL_ROUTINE_QUADRUPLE,
    pub NotifyRoutineTable: *const NDR_NOTIFY_ROUTINE,
    pub mFlags: usize,
    pub CsRoutineTables: *const NDR_CS_ROUTINES,
    pub ProxyServerInfo: *mut core::ffi::c_void,
    pub pExprInfo: *const NDR_EXPR_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union MIDL_STUB_DESC_0 {
    pub pAutoHandle: *mut *mut core::ffi::c_void,
    pub pPrimitiveHandle: *mut *mut core::ffi::c_void,
    pub pGenericBindingInfo: *mut GENERIC_BINDING_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_STUB_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug)]
pub struct MIDL_STUB_MESSAGE {
    pub RpcMsg: *mut RPC_MESSAGE,
    pub Buffer: *mut u8,
    pub BufferStart: *mut u8,
    pub BufferEnd: *mut u8,
    pub BufferMark: *mut u8,
    pub BufferLength: u32,
    pub MemorySize: u32,
    pub Memory: *mut u8,
    pub IsClient: u8,
    pub Pad: u8,
    pub uFlags2: u16,
    pub ReuseBuffer: i32,
    pub pAllocAllNodesContext: *mut NDR_ALLOC_ALL_NODES_CONTEXT,
    pub pPointerQueueState: *mut NDR_POINTER_QUEUE_STATE,
    pub IgnoreEmbeddedPointers: i32,
    pub PointerBufferMark: *mut u8,
    pub CorrDespIncrement: u8,
    pub uFlags: u8,
    pub UniquePtrCount: u16,
    pub MaxCount: usize,
    pub Offset: u32,
    pub ActualCount: u32,
    pub pfnAllocate: PFN_RPC_ALLOCATE,
    pub pfnFree: PFN_RPC_FREE,
    pub StackTop: *mut u8,
    pub pPresentedType: *mut u8,
    pub pTransmitType: *mut u8,
    pub SavedHandle: *mut core::ffi::c_void,
    pub StubDesc: *const MIDL_STUB_DESC,
    pub FullPtrXlatTables: *mut FULL_PTR_XLAT_TABLES,
    pub FullPtrRefId: u32,
    pub PointerLength: u32,
    pub _bitfield: i32,
    pub dwDestContext: u32,
    pub pvDestContext: *mut core::ffi::c_void,
    pub SavedContextHandles: *mut *mut NDR_SCONTEXT,
    pub ParamNumber: i32,
    pub pRpcChannelBuffer: core::mem::ManuallyDrop<Option<super::Com::IRpcChannelBuffer>>,
    pub pArrayInfo: *mut ARRAY_INFO,
    pub SizePtrCountArray: *mut u32,
    pub SizePtrOffsetArray: *mut u32,
    pub SizePtrLengthArray: *mut u32,
    pub pArgQueue: *mut core::ffi::c_void,
    pub dwStubPhase: u32,
    pub LowStackMark: *mut core::ffi::c_void,
    pub pAsyncMsg: PNDR_ASYNC_MESSAGE,
    pub pCorrInfo: PNDR_CORRELATION_INFO,
    pub pCorrMemory: *mut u8,
    pub pMemoryList: *mut core::ffi::c_void,
    pub pCSInfo: isize,
    pub ConformanceMark: *mut u8,
    pub VarianceMark: *mut u8,
    pub Unused: isize,
    pub pContext: *mut _NDR_PROC_CONTEXT,
    pub ContextHandleHash: *mut core::ffi::c_void,
    pub pUserMarshalList: *mut core::ffi::c_void,
    pub Reserved51_3: isize,
    pub Reserved51_4: isize,
    pub Reserved51_5: isize,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_SYNTAX_INFO {
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *const u16,
    pub TypeString: *mut u8,
    pub aUserMarshalQuadruple: *const core::ffi::c_void,
    pub pMethodProperties: *const MIDL_INTERFACE_METHOD_PROPERTIES,
    pub pReserved2: usize,
}
impl Default for MIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_TYPE_PICKLING_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: [usize; 3],
}
impl Default for MIDL_TYPE_PICKLING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    pub Version: u32,
    pub TypeFormatString: *mut u8,
    pub FormatStringSize: u16,
    pub TypeOffset: u16,
    pub StubDesc: *mut MIDL_STUB_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIDL_WINRT_TYPE_SERIALIZATION_INFO_CURRENT_VERSION: i32 = 1i32;
pub const MarshalDirectionMarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(0i32);
pub const MarshalDirectionUnmarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(1i32);
pub const MaxNumberOfEEInfoParams: u32 = 4u32;
pub const MidlInterceptionInfoVersionOne: i32 = 1i32;
pub const MidlWinrtTypeSerializationInfoVersionOne: i32 = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_ARRAY_ELEMENT_INFO {
    pub ElementMemSize: u32,
    pub Element: *mut core::ffi::c_void,
}
impl Default for NDR64_ARRAY_ELEMENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_ARRAY_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NDR64_BINDINGS {
    pub Primitive: NDR64_BIND_PRIMITIVE,
    pub Generic: NDR64_BIND_GENERIC,
    pub Context: NDR64_BIND_CONTEXT,
}
impl Default for NDR64_BINDINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_BIND_AND_NOTIFY_EXTENSION {
    pub Binding: NDR64_BIND_CONTEXT,
    pub NotifyIndex: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_BIND_CONTEXT {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Ordinal: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_BIND_GENERIC {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Size: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_BIND_PRIMITIVE {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub NumberDims: u8,
    pub NumberElements: u32,
    pub Element: *mut core::ffi::c_void,
}
impl Default for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut core::ffi::c_void,
    pub OriginalPointerLayout: *mut core::ffi::c_void,
    pub PointerLayout: *mut core::ffi::c_void,
}
impl Default for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_BUFFER_ALIGN_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Reserved: u16,
    pub Reserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Dimensions: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut core::ffi::c_void,
    pub OriginalPointerLayout: *mut core::ffi::c_void,
    pub PointerLayout: *mut core::ffi::c_void,
    pub ConfArrayDescription: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub ArrayDescription: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut core::ffi::c_void,
    pub VarDescriptor: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    pub FixedArrayFormat: NDR64_BOGUS_ARRAY_HEADER_FORMAT,
    pub ConfDescription: *mut core::ffi::c_void,
    pub VarDescription: *mut core::ffi::c_void,
    pub OffsetDescription: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_CONSTANT_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Guid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_CONTEXT_HANDLE_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_CONTEXT_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub ContextFlags: u8,
    pub RundownRoutineIndex: u8,
    pub Ordinal: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_EMBEDDED_COMPLEX_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub Reserve2: u16,
    pub Type: *mut core::ffi::c_void,
}
impl Default for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemoryOffset: u32,
    pub MemorySize: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_EXPR_CONST32 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_EXPR_CONST64 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_EXPR_NOOP {
    pub ExprType: u8,
    pub Size: u8,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_EXPR_OPERATOR {
    pub ExprType: u8,
    pub Operator: u8,
    pub CastType: u8,
    pub Reserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_EXPR_VAR {
    pub ExprType: u8,
    pub VarType: u8,
    pub Reserved: u16,
    pub Offset: u32,
}
pub const NDR64_FC_AUTO_HANDLE: u32 = 3u32;
pub const NDR64_FC_BIND_GENERIC: u32 = 1u32;
pub const NDR64_FC_BIND_PRIMITIVE: u32 = 2u32;
pub const NDR64_FC_CALLBACK_HANDLE: u32 = 4u32;
pub const NDR64_FC_EXPLICIT_HANDLE: u32 = 0u32;
pub const NDR64_FC_NO_HANDLE: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_FIXED_REPEAT_FORMAT {
    pub RepeatFormat: NDR64_REPEAT_FORMAT,
    pub Iterations: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_FIX_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_IID_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub IIDDescriptor: *mut core::ffi::c_void,
}
impl Default for NDR64_IID_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_MEMPAD_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub MemPad: u16,
    pub Reserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_NON_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub TotalSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_NON_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemorySize: u32,
    pub Switch: *mut core::ffi::c_void,
    pub Reserved: u32,
}
impl Default for NDR64_NON_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_NO_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_PARAM_FLAGS {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_PARAM_FORMAT {
    pub Type: *mut core::ffi::c_void,
    pub Attributes: NDR64_PARAM_FLAGS,
    pub Reserved: u16,
    pub StackOffset: u32,
}
impl Default for NDR64_PARAM_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_PIPE_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut core::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
}
impl Default for NDR64_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_POINTER_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Pointee: *mut core::ffi::c_void,
}
impl Default for NDR64_POINTER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    pub Offset: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_POINTER_REPEAT_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_PROC_FLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_PROC_FORMAT {
    pub Flags: u32,
    pub StackSize: u32,
    pub ConstantClientBufferSize: u32,
    pub ConstantServerBufferSize: u32,
    pub RpcFlags: u16,
    pub FloatDoubleMask: u16,
    pub NumberOfParams: u16,
    pub ExtensionSize: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_RANGED_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub Reserved: u32,
    pub Min: u64,
    pub Max: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_RANGE_FORMAT {
    pub FormatCode: u8,
    pub RangeType: u8,
    pub Reserved: u16,
    pub MinValue: i64,
    pub MaxValue: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_RANGE_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut core::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
    pub MinValue: u32,
    pub MaxValue: u32,
}
impl Default for NDR64_RANGE_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_POINTER_REPEAT_FLAGS,
    pub Reserved: u16,
    pub Increment: u32,
    pub OffsetToArray: u32,
    pub NumberOfPointers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_RPC_FLAGS {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_SIMPLE_MEMBER_FORMAT {
    pub FormatCode: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_SIMPLE_REGION_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub RegionSize: u16,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub SizeDescription: *mut core::ffi::c_void,
}
impl Default for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_STRING_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_STRING_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_STRING_FLAGS,
    pub ElementSize: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_STRUCTURE_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_SYSTEM_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub HandleType: u8,
    pub DesiredAccess: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_TRANSMIT_AS_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_TRANSMIT_AS_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub PresentedTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut core::ffi::c_void,
}
impl Default for NDR64_TRANSMIT_AS_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    pub FormatCode: u8,
    pub RealFormatCode: u8,
    pub Reserved: u16,
    pub Type: *mut core::ffi::c_void,
    pub CtxtFlags: u32,
    pub CtxtID: u32,
}
impl Default for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_UNION_ARM {
    pub CaseValue: i64,
    pub Type: *mut core::ffi::c_void,
    pub Reserved: u32,
}
impl Default for NDR64_UNION_ARM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_UNION_ARM_SELECTOR {
    pub Reserved1: u8,
    pub Alignment: u8,
    pub Reserved2: u16,
    pub Arms: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR64_USER_MARSHAL_FLAGS {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_USER_MARSHAL_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub UserTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut core::ffi::c_void,
}
impl Default for NDR64_USER_MARSHAL_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
    pub ElementSize: u32,
    pub VarDescriptor: *mut core::ffi::c_void,
}
impl Default for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NDR_ALLOC_ALL_NODES_CONTEXT(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_CS_ROUTINES {
    pub pSizeConvertRoutines: *mut NDR_CS_SIZE_CONVERT_ROUTINES,
    pub pTagGettingRoutines: *mut CS_TAG_GETTING_ROUTINE,
}
impl Default for NDR_CS_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NDR_CS_SIZE_CONVERT_ROUTINES {
    pub pfnNetSize: CS_TYPE_NET_SIZE_ROUTINE,
    pub pfnToNetCs: CS_TYPE_TO_NETCS_ROUTINE,
    pub pfnLocalSize: CS_TYPE_LOCAL_SIZE_ROUTINE,
    pub pfnFromNetCs: CS_TYPE_FROM_NETCS_ROUTINE,
}
pub const NDR_CUSTOM_OR_DEFAULT_ALLOCATOR: u32 = 268435456u32;
pub const NDR_DEFAULT_ALLOCATOR: u32 = 536870912u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_EXPR_DESC {
    pub pOffset: *const u16,
    pub pFormatExpr: *mut u8,
}
impl Default for NDR_EXPR_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NDR_NOTIFY2_ROUTINE = Option<unsafe extern "system" fn(flag: u8)>;
pub type NDR_NOTIFY_ROUTINE = Option<unsafe extern "system" fn()>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NDR_POINTER_QUEUE_STATE(pub isize);
pub type NDR_RUNDOWN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_SCONTEXT {
    pub pad: [*mut core::ffi::c_void; 2],
    pub userContext: *mut core::ffi::c_void,
}
impl Default for NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct NDR_USER_MARSHAL_INFO {
    pub InformationLevel: u32,
    pub Anonymous: NDR_USER_MARSHAL_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for NDR_USER_MARSHAL_INFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl Default for NDR_USER_MARSHAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union NDR_USER_MARSHAL_INFO_0 {
    pub Level1: core::mem::ManuallyDrop<NDR_USER_MARSHAL_INFO_LEVEL1>,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for NDR_USER_MARSHAL_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl Default for NDR_USER_MARSHAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug, PartialEq)]
pub struct NDR_USER_MARSHAL_INFO_LEVEL1 {
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub pfnAllocate: isize,
    pub pfnFree: isize,
    pub pRpcChannelBuffer: core::mem::ManuallyDrop<Option<super::Com::IRpcChannelBuffer>>,
    pub Reserved: [usize; 5],
}
#[cfg(feature = "Win32_System_Com")]
impl Default for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NT351_INTERFACE_SIZE: u32 = 64u32;
#[cfg(feature = "Win32_System_IO")]
pub type PFN_RPCNOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(pasync: *mut RPC_ASYNC_STATE, context: *mut core::ffi::c_void, event: RPC_ASYNC_EVENT)>;
pub type PFN_RPC_ALLOCATE = Option<unsafe extern "system" fn(param0: usize) -> *mut core::ffi::c_void>;
pub type PFN_RPC_FREE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PNDR_ASYNC_MESSAGE(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PNDR_CORRELATION_INFO(pub isize);
pub const PROTOCOL_ADDRESS_CHANGE: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(3i32);
pub const PROTOCOL_LOADED: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(2i32);
pub const PROTOCOL_NOT_LOADED: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(1i32);
pub const PROXY_CALCSIZE: PROXY_PHASE = PROXY_PHASE(0i32);
pub const PROXY_GETBUFFER: PROXY_PHASE = PROXY_PHASE(1i32);
pub const PROXY_MARSHAL: PROXY_PHASE = PROXY_PHASE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROXY_PHASE(pub i32);
pub const PROXY_SENDRECEIVE: PROXY_PHASE = PROXY_PHASE(3i32);
pub const PROXY_UNMARSHAL: PROXY_PHASE = PROXY_PHASE(4i32);
pub type PRPC_RUNDOWN = Option<unsafe extern "system" fn(associationcontext: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RDR_CALLOUT_STATE {
    pub LastError: RPC_STATUS,
    pub LastEEInfo: *mut core::ffi::c_void,
    pub LastCalledStage: RPC_HTTP_REDIRECTOR_STAGE,
    pub ServerName: *mut u16,
    pub ServerPort: *mut u16,
    pub RemoteUser: *mut u16,
    pub AuthType: *mut u16,
    pub ResourceTypePresent: u8,
    pub SessionIdPresent: u8,
    pub InterfacePresent: u8,
    pub ResourceType: windows_core::GUID,
    pub SessionId: windows_core::GUID,
    pub Interface: RPC_SYNTAX_IDENTIFIER,
    pub CertContext: *mut core::ffi::c_void,
}
impl Default for RDR_CALLOUT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPCFLG_ACCESSIBILITY_BIT1: u32 = 1048576u32;
pub const RPCFLG_ACCESSIBILITY_BIT2: u32 = 2097152u32;
pub const RPCFLG_ACCESS_LOCAL: u32 = 4194304u32;
pub const RPCFLG_ASYNCHRONOUS: u32 = 1073741824u32;
pub const RPCFLG_AUTO_COMPLETE: u32 = 134217728u32;
pub const RPCFLG_HAS_CALLBACK: u32 = 67108864u32;
pub const RPCFLG_HAS_GUARANTEE: u32 = 16u32;
pub const RPCFLG_HAS_MULTI_SYNTAXES: u32 = 33554432u32;
pub const RPCFLG_INPUT_SYNCHRONOUS: u32 = 536870912u32;
pub const RPCFLG_LOCAL_CALL: u32 = 268435456u32;
pub const RPCFLG_MESSAGE: u32 = 16777216u32;
pub const RPCFLG_NDR64_CONTAINS_ARM_LAYOUT: u32 = 67108864u32;
pub const RPCFLG_NON_NDR: u32 = 2147483648u32;
pub const RPCFLG_SENDER_WAITING_FOR_REPLY: u32 = 8388608u32;
pub const RPCFLG_WINRT_REMOTE_ASYNC: u32 = 32u32;
pub const RPCHTTP_RS_ACCESS_1: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(2i32);
pub const RPCHTTP_RS_ACCESS_2: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(4i32);
pub const RPCHTTP_RS_INTERFACE: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(5i32);
pub const RPCHTTP_RS_REDIRECT: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(1i32);
pub const RPCHTTP_RS_SESSION: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(3i32);
pub type RPCLT_PDU_FILTER_FUNC = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, bufferlength: u32, fdatagram: i32)>;
pub type RPC_ADDRESS_CHANGE_FN = Option<unsafe extern "system" fn(arg: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_ADDRESS_CHANGE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_ASYNC_EVENT(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy)]
pub union RPC_ASYNC_NOTIFICATION_INFO {
    pub APC: RPC_ASYNC_NOTIFICATION_INFO_0,
    pub IOC: RPC_ASYNC_NOTIFICATION_INFO_1,
    pub IntPtr: RPC_ASYNC_NOTIFICATION_INFO_2,
    pub hEvent: super::super::Foundation::HANDLE,
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
}
#[cfg(feature = "Win32_System_IO")]
impl Default for RPC_ASYNC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, Default)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_0 {
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
    pub hThread: super::super::Foundation::HANDLE,
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_1 {
    pub hIOPort: super::super::Foundation::HANDLE,
    pub dwNumberOfBytesTransferred: u32,
    pub dwCompletionKey: usize,
    pub lpOverlapped: *mut super::IO::OVERLAPPED,
}
#[cfg(feature = "Win32_System_IO")]
impl Default for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_2 {
    pub hWnd: super::super::Foundation::HWND,
    pub Msg: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy)]
pub struct RPC_ASYNC_STATE {
    pub Size: u32,
    pub Signature: u32,
    pub Lock: i32,
    pub Flags: u32,
    pub StubInfo: *mut core::ffi::c_void,
    pub UserInfo: *mut core::ffi::c_void,
    pub RuntimeInfo: *mut core::ffi::c_void,
    pub Event: RPC_ASYNC_EVENT,
    pub NotificationType: RPC_NOTIFICATION_TYPES,
    pub u: RPC_ASYNC_NOTIFICATION_INFO,
    pub Reserved: [isize; 4],
}
#[cfg(feature = "Win32_System_IO")]
impl Default for RPC_ASYNC_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_AUTH_KEY_RETRIEVAL_FN = Option<unsafe extern "system" fn(arg: *const core::ffi::c_void, serverprincname: windows_core::PCWSTR, keyver: u32, key: *mut *mut core::ffi::c_void, status: *mut RPC_STATUS)>;
pub const RPC_BHO_DONTLINGER: RPC_BINDING_HANDLE_OPTIONS_FLAGS = RPC_BINDING_HANDLE_OPTIONS_FLAGS(2u32);
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: u32 = 4u32;
pub const RPC_BHO_NONCAUSAL: RPC_BINDING_HANDLE_OPTIONS_FLAGS = RPC_BINDING_HANDLE_OPTIONS_FLAGS(1u32);
pub const RPC_BHT_OBJECT_UUID_VALID: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_BINDING_HANDLE_OPTIONS_FLAGS(pub u32);
impl RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    pub Version: u32,
    pub Flags: RPC_BINDING_HANDLE_OPTIONS_FLAGS,
    pub ComTimeout: u32,
    pub CallTimeout: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    pub Version: u32,
    pub ServerPrincName: *mut u8,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    pub Version: u32,
    pub ServerPrincName: *mut u16,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u8,
    pub StringEndpoint: *mut u8,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_0,
    pub ObjectUuid: windows_core::GUID,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    pub Reserved: *mut u8,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u16,
    pub StringEndpoint: *mut u16,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_0,
    pub ObjectUuid: windows_core::GUID,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    pub Reserved: *mut u16,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_VECTOR {
    pub Count: u32,
    pub BindingH: [*mut core::ffi::c_void; 1],
}
impl Default for RPC_BINDING_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_BLOCKING_FN = Option<unsafe extern "system" fn(hwnd: *mut core::ffi::c_void, context: *mut core::ffi::c_void, hsyncevent: *mut core::ffi::c_void) -> RPC_STATUS>;
pub const RPC_BUFFER_ASYNC: u32 = 32768u32;
pub const RPC_BUFFER_COMPLETE: u32 = 4096u32;
pub const RPC_BUFFER_EXTRA: u32 = 16384u32;
pub const RPC_BUFFER_NONOTIFY: u32 = 65536u32;
pub const RPC_BUFFER_PARTIAL: u32 = 8192u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
}
impl Default for RPC_CALL_ATTRIBUTES_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
}
impl Default for RPC_CALL_ATTRIBUTES_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V2_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
    pub KernelModeCaller: windows_core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
}
impl Default for RPC_CALL_ATTRIBUTES_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V2_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
    pub KernelModeCaller: windows_core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
}
impl Default for RPC_CALL_ATTRIBUTES_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V3_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
    pub KernelModeCaller: windows_core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
impl Default for RPC_CALL_ATTRIBUTES_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V3_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: windows_core::BOOL,
    pub KernelModeCaller: windows_core::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
impl Default for RPC_CALL_ATTRIBUTES_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_CALL_ATTRIBUTES_VERSION: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_LOCAL_ADDRESS_V1 {
    pub Version: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub AddressFormat: RpcLocalAddressFormat,
}
impl Default for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_CALL_STATUS_CANCELLED: u32 = 1u32;
pub const RPC_CALL_STATUS_DISCONNECTED: u32 = 2u32;
pub type RPC_CLIENT_ALLOC = Option<unsafe extern "system" fn(size: usize) -> *mut core::ffi::c_void>;
pub type RPC_CLIENT_FREE = Option<unsafe extern "system" fn(ptr: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CLIENT_INFORMATION1 {
    pub UserName: *mut u8,
    pub ComputerName: *mut u8,
    pub Privilege: u16,
    pub AuthFlags: u32,
}
impl Default for RPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CLIENT_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub Reserved: usize,
    pub InterpreterInfo: *const core::ffi::c_void,
    pub Flags: u32,
}
impl Default for RPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_CONTEXT_HANDLE_DEFAULT_FLAGS: u32 = 0u32;
pub const RPC_CONTEXT_HANDLE_DONT_SERIALIZE: u32 = 536870912u32;
pub const RPC_CONTEXT_HANDLE_FLAGS: u32 = 805306368u32;
pub const RPC_CONTEXT_HANDLE_SERIALIZE: u32 = 268435456u32;
pub const RPC_C_AUTHN_CLOUD_AP: u32 = 36u32;
pub const RPC_C_AUTHN_DCE_PRIVATE: u32 = 1u32;
pub const RPC_C_AUTHN_DCE_PUBLIC: u32 = 2u32;
pub const RPC_C_AUTHN_DEC_PUBLIC: u32 = 4u32;
pub const RPC_C_AUTHN_DEFAULT: i32 = -1i32;
pub const RPC_C_AUTHN_DIGEST: u32 = 21u32;
pub const RPC_C_AUTHN_DPA: u32 = 17u32;
pub const RPC_C_AUTHN_GSS_KERBEROS: u32 = 16u32;
pub const RPC_C_AUTHN_GSS_NEGOTIATE: u32 = 9u32;
pub const RPC_C_AUTHN_GSS_SCHANNEL: u32 = 14u32;
pub const RPC_C_AUTHN_INFO_NONE: RPC_C_AUTHN_INFO_TYPE = RPC_C_AUTHN_INFO_TYPE(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_C_AUTHN_INFO_TYPE(pub u32);
pub const RPC_C_AUTHN_INFO_TYPE_HTTP: RPC_C_AUTHN_INFO_TYPE = RPC_C_AUTHN_INFO_TYPE(1u32);
pub const RPC_C_AUTHN_KERNEL: u32 = 20u32;
pub const RPC_C_AUTHN_LIVEXP_SSP: u32 = 35u32;
pub const RPC_C_AUTHN_LIVE_SSP: u32 = 32u32;
pub const RPC_C_AUTHN_MQ: u32 = 100u32;
pub const RPC_C_AUTHN_MSN: u32 = 18u32;
pub const RPC_C_AUTHN_MSONLINE: u32 = 82u32;
pub const RPC_C_AUTHN_NEGO_EXTENDER: u32 = 30u32;
pub const RPC_C_AUTHN_NONE: u32 = 0u32;
pub const RPC_C_AUTHN_PKU2U: u32 = 31u32;
pub const RPC_C_AUTHN_WINNT: u32 = 10u32;
pub const RPC_C_AUTHZ_DCE: u32 = 2u32;
pub const RPC_C_AUTHZ_DEFAULT: u32 = 4294967295u32;
pub const RPC_C_AUTHZ_NAME: u32 = 1u32;
pub const RPC_C_AUTHZ_NONE: u32 = 0u32;
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: u32 = 5u32;
pub const RPC_C_BINDING_INFINITE_TIMEOUT: u32 = 10u32;
pub const RPC_C_BINDING_MAX_TIMEOUT: u32 = 9u32;
pub const RPC_C_BINDING_MIN_TIMEOUT: u32 = 0u32;
pub const RPC_C_BIND_TO_ALL_NICS: u32 = 1u32;
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: i32 = -1i32;
pub const RPC_C_DONT_FAIL: u32 = 4u32;
pub const RPC_C_EP_ALL_ELTS: u32 = 0u32;
pub const RPC_C_EP_MATCH_BY_BOTH: u32 = 3u32;
pub const RPC_C_EP_MATCH_BY_IF: u32 = 1u32;
pub const RPC_C_EP_MATCH_BY_OBJ: u32 = 2u32;
pub const RPC_C_FULL_CERT_CHAIN: u32 = 1u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: u32 = 1u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: u32 = 65536u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: u32 = 8u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: u32 = 2u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_C_HTTP_AUTHN_TARGET(pub u32);
impl RPC_C_HTTP_AUTHN_TARGET {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: RPC_C_HTTP_AUTHN_TARGET = RPC_C_HTTP_AUTHN_TARGET(2u32);
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: RPC_C_HTTP_AUTHN_TARGET = RPC_C_HTTP_AUTHN_TARGET(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_C_HTTP_FLAGS(pub u32);
impl RPC_C_HTTP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RPC_C_HTTP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RPC_C_HTTP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(16u32);
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(8u32);
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(2u32);
pub const RPC_C_HTTP_FLAG_USE_SSL: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(1u32);
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: u32 = 1234u32;
pub const RPC_C_MGMT_INQ_IF_IDS: u32 = 0u32;
pub const RPC_C_MGMT_INQ_PRINC_NAME: u32 = 1u32;
pub const RPC_C_MGMT_INQ_STATS: u32 = 2u32;
pub const RPC_C_MGMT_IS_SERVER_LISTEN: u32 = 3u32;
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: u32 = 4u32;
pub const RPC_C_MQ_AUTHN_LEVEL_NONE: u32 = 0u32;
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 8u32;
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY: u32 = 16u32;
pub const RPC_C_MQ_CLEAR_ON_OPEN: u32 = 2u32;
pub const RPC_C_MQ_EXPRESS: u32 = 0u32;
pub const RPC_C_MQ_JOURNAL_ALWAYS: u32 = 2u32;
pub const RPC_C_MQ_JOURNAL_DEADLETTER: u32 = 1u32;
pub const RPC_C_MQ_JOURNAL_NONE: u32 = 0u32;
pub const RPC_C_MQ_PERMANENT: u32 = 1u32;
pub const RPC_C_MQ_RECOVERABLE: u32 = 1u32;
pub const RPC_C_MQ_TEMPORARY: u32 = 0u32;
pub const RPC_C_MQ_USE_EXISTING_SECURITY: u32 = 4u32;
pub const RPC_C_NOTIFY_ON_SEND_COMPLETE: u32 = 1u32;
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1i32;
pub const RPC_C_NS_SYNTAX_DCE: GROUP_NAME_SYNTAX = GROUP_NAME_SYNTAX(3u32);
pub const RPC_C_NS_SYNTAX_DEFAULT: GROUP_NAME_SYNTAX = GROUP_NAME_SYNTAX(0u32);
pub const RPC_C_OPT_ASYNC_BLOCK: u32 = 15u32;
pub const RPC_C_OPT_BINDING_NONCAUSAL: u32 = 9u32;
pub const RPC_C_OPT_CALL_TIMEOUT: u32 = 12u32;
pub const RPC_C_OPT_COOKIE_AUTH: u32 = 7u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    pub BufferSize: u32,
    pub Buffer: windows_core::PSTR,
}
pub const RPC_C_OPT_DONT_LINGER: u32 = 13u32;
pub const RPC_C_OPT_MAX_OPTIONS: u32 = 12u32;
pub const RPC_C_OPT_MQ_ACKNOWLEDGE: u32 = 4u32;
pub const RPC_C_OPT_MQ_AUTHN_LEVEL: u32 = 6u32;
pub const RPC_C_OPT_MQ_AUTHN_SERVICE: u32 = 5u32;
pub const RPC_C_OPT_MQ_DELIVERY: u32 = 1u32;
pub const RPC_C_OPT_MQ_JOURNAL: u32 = 3u32;
pub const RPC_C_OPT_MQ_PRIORITY: u32 = 2u32;
pub const RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED: u32 = 8u32;
pub const RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE: u32 = 7u32;
pub const RPC_C_OPT_OPTIMIZE_TIME: u32 = 16u32;
pub const RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND: u32 = 3u32;
pub const RPC_C_OPT_PRIVATE_DO_NOT_DISTURB: u32 = 2u32;
pub const RPC_C_OPT_PRIVATE_SUPPRESS_WAKE: u32 = 1u32;
pub const RPC_C_OPT_RESOURCE_TYPE_UUID: u32 = 8u32;
pub const RPC_C_OPT_SECURITY_CALLBACK: u32 = 10u32;
pub const RPC_C_OPT_SESSION_ID: u32 = 6u32;
pub const RPC_C_OPT_TRANS_SEND_BUFFER_SIZE: u32 = 5u32;
pub const RPC_C_OPT_TRUST_PEER: u32 = 14u32;
pub const RPC_C_OPT_UNIQUE_BINDING: u32 = 11u32;
pub const RPC_C_PARM_BUFFER_LENGTH: u32 = 2u32;
pub const RPC_C_PARM_MAX_PACKET_LENGTH: u32 = 1u32;
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1u32;
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1u32;
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0u32;
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4u32;
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2u32;
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3u32;
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: u32 = 10u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_C_QOS_CAPABILITIES(pub u32);
impl RPC_C_QOS_CAPABILITIES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RPC_C_QOS_CAPABILITIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RPC_C_QOS_CAPABILITIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(4u32);
pub const RPC_C_QOS_CAPABILITIES_DEFAULT: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(0u32);
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(8u32);
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(16u32);
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(2u32);
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(1u32);
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(32u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_C_QOS_IDENTITY(pub u32);
pub const RPC_C_QOS_IDENTITY_DYNAMIC: RPC_C_QOS_IDENTITY = RPC_C_QOS_IDENTITY(1u32);
pub const RPC_C_QOS_IDENTITY_STATIC: RPC_C_QOS_IDENTITY = RPC_C_QOS_IDENTITY(0u32);
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: u32 = 8u32;
pub const RPC_C_SECURITY_QOS_VERSION: i32 = 1i32;
pub const RPC_C_SECURITY_QOS_VERSION_1: i32 = 1i32;
pub const RPC_C_SECURITY_QOS_VERSION_2: i32 = 2i32;
pub const RPC_C_SECURITY_QOS_VERSION_3: i32 = 3i32;
pub const RPC_C_SECURITY_QOS_VERSION_4: i32 = 4i32;
pub const RPC_C_SECURITY_QOS_VERSION_5: i32 = 5i32;
pub const RPC_C_STATS_CALLS_IN: u32 = 0u32;
pub const RPC_C_STATS_CALLS_OUT: u32 = 1u32;
pub const RPC_C_STATS_PKTS_IN: u32 = 2u32;
pub const RPC_C_STATS_PKTS_OUT: u32 = 3u32;
pub const RPC_C_TRY_ENFORCE_MAX_CALLS: u32 = 16u32;
pub const RPC_C_USE_INTERNET_PORT: u32 = 1u32;
pub const RPC_C_USE_INTRANET_PORT: u32 = 2u32;
pub const RPC_C_VERS_ALL: u32 = 1u32;
pub const RPC_C_VERS_COMPATIBLE: u32 = 2u32;
pub const RPC_C_VERS_EXACT: u32 = 3u32;
pub const RPC_C_VERS_MAJOR_ONLY: u32 = 4u32;
pub const RPC_C_VERS_UPTO: u32 = 5u32;
pub type RPC_DISPATCH_FUNCTION = Option<unsafe extern "system" fn(message: *mut RPC_MESSAGE)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RPC_DISPATCH_TABLE {
    pub DispatchTableCount: u32,
    pub DispatchTable: RPC_DISPATCH_FUNCTION,
    pub Reserved: isize,
}
pub const RPC_EEINFO_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_EE_INFO_PARAM {
    pub ParameterType: ExtendedErrorParamTypes,
    pub u: RPC_EE_INFO_PARAM_0,
}
impl Default for RPC_EE_INFO_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_EE_INFO_PARAM_0 {
    pub AnsiString: windows_core::PSTR,
    pub UnicodeString: windows_core::PWSTR,
    pub LVal: i32,
    pub SVal: i16,
    pub PVal: u64,
    pub BVal: BinaryParam,
}
impl Default for RPC_EE_INFO_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ENDPOINT_TEMPLATEA {
    pub Version: u32,
    pub ProtSeq: windows_core::PSTR,
    pub Endpoint: windows_core::PSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub Backlog: u32,
}
impl Default for RPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ENDPOINT_TEMPLATEW {
    pub Version: u32,
    pub ProtSeq: windows_core::PWSTR,
    pub Endpoint: windows_core::PWSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub Backlog: u32,
}
impl Default for RPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ERROR_ENUM_HANDLE {
    pub Signature: u32,
    pub CurrentPos: *mut core::ffi::c_void,
    pub Head: *mut core::ffi::c_void,
}
impl Default for RPC_ERROR_ENUM_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_EXTENDED_ERROR_INFO {
    pub Version: u32,
    pub ComputerName: windows_core::PWSTR,
    pub ProcessID: u32,
    pub u: RPC_EXTENDED_ERROR_INFO_0,
    pub GeneratingComponent: u32,
    pub Status: u32,
    pub DetectionLocation: u16,
    pub Flags: u16,
    pub NumberOfParameters: i32,
    pub Parameters: [RPC_EE_INFO_PARAM; 4],
}
impl Default for RPC_EXTENDED_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_EXTENDED_ERROR_INFO_0 {
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub FileTime: super::super::Foundation::FILETIME,
}
impl Default for RPC_EXTENDED_ERROR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_FLAGS_VALID_BIT: u32 = 32768u32;
pub type RPC_FORWARD_FUNCTION = Option<unsafe extern "system" fn(interfaceid: *mut windows_core::GUID, interfaceversion: *mut RPC_VERSION, objectid: *mut windows_core::GUID, rpcpro: *mut u8, ppdestendpoint: *mut *mut core::ffi::c_void) -> RPC_STATUS>;
pub const RPC_FW_IF_FLAG_DCOM: u32 = 1u32;
pub type RPC_HTTP_PROXY_FREE_STRING = Option<unsafe extern "system" fn(string: windows_core::PCWSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_HTTP_REDIRECTOR_STAGE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub TransportCredentials: *mut core::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut core::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub TransportCredentials: *mut core::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut core::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: u32 = 16u32;
pub const RPC_IF_ALLOW_LOCAL_ONLY: u32 = 32u32;
pub const RPC_IF_ALLOW_SECURE_ONLY: u32 = 8u32;
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: u32 = 4u32;
pub const RPC_IF_ASYNC_CALLBACK: u32 = 256u32;
pub const RPC_IF_AUTOLISTEN: u32 = 1u32;
pub type RPC_IF_CALLBACK_FN = Option<unsafe extern "system" fn(interfaceuuid: *const core::ffi::c_void, context: *const core::ffi::c_void) -> RPC_STATUS>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_IF_ID {
    pub Uuid: windows_core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IF_ID_VECTOR {
    pub Count: u32,
    pub IfId: [*mut RPC_IF_ID; 1],
}
impl Default for RPC_IF_ID_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_IF_OLE: u32 = 2u32;
pub const RPC_IF_SEC_CACHE_PER_PROC: u32 = 128u32;
pub const RPC_IF_SEC_NO_CACHE: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IMPORT_CONTEXT_P {
    pub LookupContext: *mut core::ffi::c_void,
    pub ProposedHandle: *mut core::ffi::c_void,
    pub Bindings: *mut RPC_BINDING_VECTOR,
}
impl Default for RPC_IMPORT_CONTEXT_P {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN = Option<unsafe extern "system" fn(ifgroup: *const core::ffi::c_void, idlecallbackcontext: *const core::ffi::c_void, isgroupidle: u32)>;
pub const RPC_INTERFACE_HAS_PIPES: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RPC_INTERFACE_TEMPLATEA {
    pub Version: u32,
    pub IfSpec: *mut core::ffi::c_void,
    pub MgrTypeUuid: *mut windows_core::GUID,
    pub MgrEpv: *mut core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: windows_core::PSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RPC_INTERFACE_TEMPLATEW {
    pub Version: u32,
    pub IfSpec: *mut core::ffi::c_void,
    pub MgrTypeUuid: *mut windows_core::GUID,
    pub MgrEpv: *mut core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: windows_core::PWSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_MESSAGE {
    pub Handle: *mut core::ffi::c_void,
    pub DataRepresentation: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferLength: u32,
    pub ProcNum: u32,
    pub TransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub RpcInterfaceInformation: *mut core::ffi::c_void,
    pub ReservedForRuntime: *mut core::ffi::c_void,
    pub ManagerEpv: *mut core::ffi::c_void,
    pub ImportContext: *mut core::ffi::c_void,
    pub RpcFlags: u32,
}
impl Default for RPC_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_MGMT_AUTHORIZATION_FN = Option<unsafe extern "system" fn(clientbinding: *const core::ffi::c_void, requestedmgmtoperation: u32, status: *mut RPC_STATUS) -> i32>;
pub const RPC_NCA_FLAGS_BROADCAST: u32 = 2u32;
pub const RPC_NCA_FLAGS_DEFAULT: u32 = 0u32;
pub const RPC_NCA_FLAGS_IDEMPOTENT: u32 = 1u32;
pub const RPC_NCA_FLAGS_MAYBE: u32 = 4u32;
pub type RPC_NEW_HTTP_PROXY_CHANNEL = Option<unsafe extern "system" fn(redirectorstage: RPC_HTTP_REDIRECTOR_STAGE, servername: windows_core::PCWSTR, serverport: windows_core::PCWSTR, remoteuser: windows_core::PCWSTR, authtype: windows_core::PCWSTR, resourceuuid: *mut core::ffi::c_void, sessionid: *mut core::ffi::c_void, interface: *const core::ffi::c_void, reserved: *const core::ffi::c_void, flags: u32, newservername: *mut windows_core::PWSTR, newserverport: *mut windows_core::PWSTR) -> RPC_STATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_NOTIFICATIONS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_NOTIFICATION_TYPES(pub i32);
pub type RPC_OBJECT_INQ_FN = Option<unsafe extern "system" fn(objectuuid: *const windows_core::GUID, typeuuid: *mut windows_core::GUID, status: *mut RPC_STATUS)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_POLICY {
    pub Length: u32,
    pub EndpointFlags: u32,
    pub NICFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_ENDPOINT {
    pub RpcProtocolSequence: *mut u8,
    pub Endpoint: *mut u8,
}
impl Default for RPC_PROTSEQ_ENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_PROTSEQ_HTTP: u32 = 4u32;
pub const RPC_PROTSEQ_LRPC: u32 = 3u32;
pub const RPC_PROTSEQ_NMP: u32 = 2u32;
pub const RPC_PROTSEQ_TCP: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_VECTORA {
    pub Count: u32,
    pub Protseq: [*mut u8; 1],
}
impl Default for RPC_PROTSEQ_VECTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_VECTORW {
    pub Count: u32,
    pub Protseq: [*mut u16; 1],
}
impl Default for RPC_PROTSEQ_VECTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_PROXY_CONNECTION_TYPE_IN_PROXY: u32 = 0u32;
pub const RPC_PROXY_CONNECTION_TYPE_OUT_PROXY: u32 = 1u32;
pub const RPC_P_ADDR_FORMAT_TCP_IPV4: u32 = 1u32;
pub const RPC_P_ADDR_FORMAT_TCP_IPV6: u32 = 2u32;
pub const RPC_QUERY_CALL_LOCAL_ADDRESS: u32 = 8u32;
pub const RPC_QUERY_CLIENT_ID: u32 = 128u32;
pub const RPC_QUERY_CLIENT_PID: u32 = 16u32;
pub const RPC_QUERY_CLIENT_PRINCIPAL_NAME: u32 = 4u32;
pub const RPC_QUERY_IS_CLIENT_LOCAL: u32 = 32u32;
pub const RPC_QUERY_NO_AUTH_REQUIRED: u32 = 64u32;
pub const RPC_QUERY_SERVER_PRINCIPAL_NAME: u32 = 2u32;
pub type RPC_SECURITY_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_SECURITY_QOS {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V2_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_A_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V2_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V2_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V2_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_W_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V2_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V2_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V3_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_A_0,
    pub Sid: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V3_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V3_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V3_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_W_0,
    pub Sid: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V3_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V3_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V4_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_A_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V4_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V4_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V4_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_W_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V4_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V4_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V5_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_A_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V5_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V5_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V5_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_W_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V5_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V5_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_SEC_CONTEXT_KEY_INFO {
    pub EncryptAlgorithm: u32,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SERVER_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub DefaultManagerEpv: *mut core::ffi::c_void,
    pub InterpreterInfo: *const core::ffi::c_void,
    pub Flags: u32,
}
impl Default for RPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_SETFILTER_FUNC = Option<unsafe extern "system" fn(pfnfilter: RPCLT_PDU_FILTER_FUNC)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_STATS_VECTOR {
    pub Count: u32,
    pub Stats: [u32; 1],
}
impl Default for RPC_STATS_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RPC_STATUS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_SYNTAX_IDENTIFIER {
    pub SyntaxGUID: windows_core::GUID,
    pub SyntaxVersion: RPC_VERSION,
}
pub const RPC_SYSTEM_HANDLE_FREE_ALL: u32 = 3u32;
pub const RPC_SYSTEM_HANDLE_FREE_ERROR_ON_CLOSE: u32 = 4u32;
pub const RPC_SYSTEM_HANDLE_FREE_RETRIEVED: u32 = 2u32;
pub const RPC_SYSTEM_HANDLE_FREE_UNRETRIEVED: u32 = 1u32;
pub const RPC_S_ACCESS_DENIED: RPC_STATUS = RPC_STATUS(5i32);
pub const RPC_S_ADDRESS_ERROR: RPC_STATUS = RPC_STATUS(1768i32);
pub const RPC_S_ALREADY_LISTENING: RPC_STATUS = RPC_STATUS(1713i32);
pub const RPC_S_ALREADY_REGISTERED: RPC_STATUS = RPC_STATUS(1711i32);
pub const RPC_S_ASYNC_CALL_PENDING: RPC_STATUS = RPC_STATUS(997i32);
pub const RPC_S_BINDING_HAS_NO_AUTH: RPC_STATUS = RPC_STATUS(1746i32);
pub const RPC_S_BINDING_INCOMPLETE: RPC_STATUS = RPC_STATUS(1819i32);
pub const RPC_S_BUFFER_TOO_SMALL: RPC_STATUS = RPC_STATUS(122i32);
pub const RPC_S_CALL_CANCELLED: RPC_STATUS = RPC_STATUS(1818i32);
pub const RPC_S_CALL_FAILED: RPC_STATUS = RPC_STATUS(1726i32);
pub const RPC_S_CALL_FAILED_DNE: RPC_STATUS = RPC_STATUS(1727i32);
pub const RPC_S_CALL_IN_PROGRESS: RPC_STATUS = RPC_STATUS(1791i32);
pub const RPC_S_CANNOT_SUPPORT: RPC_STATUS = RPC_STATUS(1764i32);
pub const RPC_S_CANT_CREATE_ENDPOINT: RPC_STATUS = RPC_STATUS(1720i32);
pub const RPC_S_COMM_FAILURE: RPC_STATUS = RPC_STATUS(1820i32);
pub const RPC_S_COOKIE_AUTH_FAILED: RPC_STATUS = RPC_STATUS(1833i32);
pub const RPC_S_DO_NOT_DISTURB: RPC_STATUS = RPC_STATUS(1834i32);
pub const RPC_S_DUPLICATE_ENDPOINT: RPC_STATUS = RPC_STATUS(1740i32);
pub const RPC_S_ENTRY_ALREADY_EXISTS: RPC_STATUS = RPC_STATUS(1760i32);
pub const RPC_S_ENTRY_NOT_FOUND: RPC_STATUS = RPC_STATUS(1761i32);
pub const RPC_S_ENTRY_TYPE_MISMATCH: RPC_STATUS = RPC_STATUS(1922i32);
pub const RPC_S_FP_DIV_ZERO: RPC_STATUS = RPC_STATUS(1769i32);
pub const RPC_S_FP_OVERFLOW: RPC_STATUS = RPC_STATUS(1771i32);
pub const RPC_S_FP_UNDERFLOW: RPC_STATUS = RPC_STATUS(1770i32);
pub const RPC_S_GROUP_MEMBER_NOT_FOUND: RPC_STATUS = RPC_STATUS(1898i32);
pub const RPC_S_GRP_ELT_NOT_ADDED: RPC_STATUS = RPC_STATUS(1928i32);
pub const RPC_S_GRP_ELT_NOT_REMOVED: RPC_STATUS = RPC_STATUS(1929i32);
pub const RPC_S_INCOMPLETE_NAME: RPC_STATUS = RPC_STATUS(1755i32);
pub const RPC_S_INTERFACE_NOT_EXPORTED: RPC_STATUS = RPC_STATUS(1924i32);
pub const RPC_S_INTERFACE_NOT_FOUND: RPC_STATUS = RPC_STATUS(1759i32);
pub const RPC_S_INTERNAL_ERROR: RPC_STATUS = RPC_STATUS(1766i32);
pub const RPC_S_INVALID_ARG: RPC_STATUS = RPC_STATUS(87i32);
pub const RPC_S_INVALID_ASYNC_CALL: RPC_STATUS = RPC_STATUS(1915i32);
pub const RPC_S_INVALID_ASYNC_HANDLE: RPC_STATUS = RPC_STATUS(1914i32);
pub const RPC_S_INVALID_AUTH_IDENTITY: RPC_STATUS = RPC_STATUS(1749i32);
pub const RPC_S_INVALID_BINDING: RPC_STATUS = RPC_STATUS(1702i32);
pub const RPC_S_INVALID_BOUND: RPC_STATUS = RPC_STATUS(1734i32);
pub const RPC_S_INVALID_ENDPOINT_FORMAT: RPC_STATUS = RPC_STATUS(1706i32);
pub const RPC_S_INVALID_LEVEL: RPC_STATUS = RPC_STATUS(87i32);
pub const RPC_S_INVALID_NAF_ID: RPC_STATUS = RPC_STATUS(1763i32);
pub const RPC_S_INVALID_NAME_SYNTAX: RPC_STATUS = RPC_STATUS(1736i32);
pub const RPC_S_INVALID_NETWORK_OPTIONS: RPC_STATUS = RPC_STATUS(1724i32);
pub const RPC_S_INVALID_NET_ADDR: RPC_STATUS = RPC_STATUS(1707i32);
pub const RPC_S_INVALID_OBJECT: RPC_STATUS = RPC_STATUS(1900i32);
pub const RPC_S_INVALID_RPC_PROTSEQ: RPC_STATUS = RPC_STATUS(1704i32);
pub const RPC_S_INVALID_SECURITY_DESC: RPC_STATUS = RPC_STATUS(1338i32);
pub const RPC_S_INVALID_STRING_BINDING: RPC_STATUS = RPC_STATUS(1700i32);
pub const RPC_S_INVALID_STRING_UUID: RPC_STATUS = RPC_STATUS(1705i32);
pub const RPC_S_INVALID_TAG: RPC_STATUS = RPC_STATUS(1733i32);
pub const RPC_S_INVALID_TIMEOUT: RPC_STATUS = RPC_STATUS(1709i32);
pub const RPC_S_INVALID_VERS_OPTION: RPC_STATUS = RPC_STATUS(1756i32);
pub const RPC_S_MAX_CALLS_TOO_SMALL: RPC_STATUS = RPC_STATUS(1742i32);
pub const RPC_S_NAME_SERVICE_UNAVAILABLE: RPC_STATUS = RPC_STATUS(1762i32);
pub const RPC_S_NOTHING_TO_EXPORT: RPC_STATUS = RPC_STATUS(1754i32);
pub const RPC_S_NOT_ALL_OBJS_EXPORTED: RPC_STATUS = RPC_STATUS(1923i32);
pub const RPC_S_NOT_ALL_OBJS_UNEXPORTED: RPC_STATUS = RPC_STATUS(1758i32);
pub const RPC_S_NOT_CANCELLED: RPC_STATUS = RPC_STATUS(1826i32);
pub const RPC_S_NOT_ENOUGH_QUOTA: RPC_STATUS = RPC_STATUS(1816i32);
pub const RPC_S_NOT_LISTENING: RPC_STATUS = RPC_STATUS(1715i32);
pub const RPC_S_NOT_RPC_ERROR: RPC_STATUS = RPC_STATUS(1823i32);
pub const RPC_S_NO_BINDINGS: RPC_STATUS = RPC_STATUS(1718i32);
pub const RPC_S_NO_CALL_ACTIVE: RPC_STATUS = RPC_STATUS(1725i32);
pub const RPC_S_NO_CONTEXT_AVAILABLE: RPC_STATUS = RPC_STATUS(1765i32);
pub const RPC_S_NO_ENDPOINT_FOUND: RPC_STATUS = RPC_STATUS(1708i32);
pub const RPC_S_NO_ENTRY_NAME: RPC_STATUS = RPC_STATUS(1735i32);
pub const RPC_S_NO_INTERFACES: RPC_STATUS = RPC_STATUS(1817i32);
pub const RPC_S_NO_MORE_BINDINGS: RPC_STATUS = RPC_STATUS(1806i32);
pub const RPC_S_NO_MORE_MEMBERS: RPC_STATUS = RPC_STATUS(1757i32);
pub const RPC_S_NO_PRINC_NAME: RPC_STATUS = RPC_STATUS(1822i32);
pub const RPC_S_NO_PROTSEQS: RPC_STATUS = RPC_STATUS(1719i32);
pub const RPC_S_NO_PROTSEQS_REGISTERED: RPC_STATUS = RPC_STATUS(1714i32);
pub const RPC_S_OBJECT_NOT_FOUND: RPC_STATUS = RPC_STATUS(1710i32);
pub const RPC_S_OK: RPC_STATUS = RPC_STATUS(0i32);
pub const RPC_S_OUT_OF_MEMORY: RPC_STATUS = RPC_STATUS(14i32);
pub const RPC_S_OUT_OF_RESOURCES: RPC_STATUS = RPC_STATUS(1721i32);
pub const RPC_S_OUT_OF_THREADS: RPC_STATUS = RPC_STATUS(164i32);
pub const RPC_S_PRF_ELT_NOT_ADDED: RPC_STATUS = RPC_STATUS(1926i32);
pub const RPC_S_PRF_ELT_NOT_REMOVED: RPC_STATUS = RPC_STATUS(1927i32);
pub const RPC_S_PROCNUM_OUT_OF_RANGE: RPC_STATUS = RPC_STATUS(1745i32);
pub const RPC_S_PROFILE_NOT_ADDED: RPC_STATUS = RPC_STATUS(1925i32);
pub const RPC_S_PROTOCOL_ERROR: RPC_STATUS = RPC_STATUS(1728i32);
pub const RPC_S_PROTSEQ_NOT_FOUND: RPC_STATUS = RPC_STATUS(1744i32);
pub const RPC_S_PROTSEQ_NOT_SUPPORTED: RPC_STATUS = RPC_STATUS(1703i32);
pub const RPC_S_PROXY_ACCESS_DENIED: RPC_STATUS = RPC_STATUS(1729i32);
pub const RPC_S_RUNTIME_UNINITIALIZED: RPC_STATUS = RPC_STATUS(1i32);
pub const RPC_S_SEC_PKG_ERROR: RPC_STATUS = RPC_STATUS(1825i32);
pub const RPC_S_SEND_INCOMPLETE: RPC_STATUS = RPC_STATUS(1913i32);
pub const RPC_S_SERVER_OUT_OF_MEMORY: RPC_STATUS = RPC_STATUS(1130i32);
pub const RPC_S_SERVER_TOO_BUSY: RPC_STATUS = RPC_STATUS(1723i32);
pub const RPC_S_SERVER_UNAVAILABLE: RPC_STATUS = RPC_STATUS(1722i32);
pub const RPC_S_STRING_TOO_LONG: RPC_STATUS = RPC_STATUS(1743i32);
pub const RPC_S_SYSTEM_HANDLE_COUNT_EXCEEDED: RPC_STATUS = RPC_STATUS(1835i32);
pub const RPC_S_SYSTEM_HANDLE_TYPE_MISMATCH: RPC_STATUS = RPC_STATUS(1836i32);
pub const RPC_S_TIMEOUT: RPC_STATUS = RPC_STATUS(1460i32);
pub const RPC_S_TYPE_ALREADY_REGISTERED: RPC_STATUS = RPC_STATUS(1712i32);
pub const RPC_S_UNKNOWN_AUTHN_LEVEL: RPC_STATUS = RPC_STATUS(1748i32);
pub const RPC_S_UNKNOWN_AUTHN_SERVICE: RPC_STATUS = RPC_STATUS(1747i32);
pub const RPC_S_UNKNOWN_AUTHN_TYPE: RPC_STATUS = RPC_STATUS(1741i32);
pub const RPC_S_UNKNOWN_AUTHZ_SERVICE: RPC_STATUS = RPC_STATUS(1750i32);
pub const RPC_S_UNKNOWN_IF: RPC_STATUS = RPC_STATUS(1717i32);
pub const RPC_S_UNKNOWN_MGR_TYPE: RPC_STATUS = RPC_STATUS(1716i32);
pub const RPC_S_UNKNOWN_PRINCIPAL: RPC_STATUS = RPC_STATUS(1332i32);
pub const RPC_S_UNSUPPORTED_AUTHN_LEVEL: RPC_STATUS = RPC_STATUS(1821i32);
pub const RPC_S_UNSUPPORTED_NAME_SYNTAX: RPC_STATUS = RPC_STATUS(1737i32);
pub const RPC_S_UNSUPPORTED_TRANS_SYN: RPC_STATUS = RPC_STATUS(1730i32);
pub const RPC_S_UNSUPPORTED_TYPE: RPC_STATUS = RPC_STATUS(1732i32);
pub const RPC_S_UUID_LOCAL_ONLY: RPC_STATUS = RPC_STATUS(1824i32);
pub const RPC_S_UUID_NO_ADDRESS: RPC_STATUS = RPC_STATUS(1739i32);
pub const RPC_S_WRONG_KIND_OF_BINDING: RPC_STATUS = RPC_STATUS(1701i32);
pub const RPC_S_ZERO_DIVIDE: RPC_STATUS = RPC_STATUS(1767i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_TRANSFER_SYNTAX {
    pub Uuid: windows_core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
pub const RPC_TYPE_DISCONNECT_EVENT_CONTEXT_HANDLE: u32 = 2147483648u32;
pub const RPC_TYPE_STRICT_CONTEXT_HANDLE: u32 = 1073741824u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const RpcAttemptedLbsDecisions: RpcPerfCounters = RpcPerfCounters(8i32);
pub const RpcAttemptedLbsMessages: RpcPerfCounters = RpcPerfCounters(10i32);
pub const RpcBackEndConnectionAttempts: RpcPerfCounters = RpcPerfCounters(2i32);
pub const RpcBackEndConnectionFailed: RpcPerfCounters = RpcPerfCounters(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RpcCallClientLocality(pub i32);
pub const RpcCallComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RpcCallType(pub i32);
pub const RpcClientCancel: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(4i32);
pub const RpcClientDisconnect: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(3i32);
pub const RpcCurrentUniqueUser: RpcPerfCounters = RpcPerfCounters(1i32);
pub const RpcFailedLbsDecisions: RpcPerfCounters = RpcPerfCounters(9i32);
pub const RpcFailedLbsMessages: RpcPerfCounters = RpcPerfCounters(11i32);
pub const RpcIncomingBandwidth: RpcPerfCounters = RpcPerfCounters(6i32);
pub const RpcIncomingConnections: RpcPerfCounters = RpcPerfCounters(5i32);
pub const RpcLastCounter: RpcPerfCounters = RpcPerfCounters(12i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RpcLocalAddressFormat(pub i32);
pub const RpcNotificationCallCancel: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(2i32);
pub const RpcNotificationCallNone: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(0i32);
pub const RpcNotificationClientDisconnect: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(1i32);
pub const RpcNotificationTypeApc: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(2i32);
pub const RpcNotificationTypeCallback: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(5i32);
pub const RpcNotificationTypeEvent: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(1i32);
pub const RpcNotificationTypeHwnd: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(4i32);
pub const RpcNotificationTypeIoc: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(3i32);
pub const RpcNotificationTypeNone: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(0i32);
pub const RpcOutgoingBandwidth: RpcPerfCounters = RpcPerfCounters(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RpcPerfCounters(pub i32);
pub const RpcReceiveComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(2i32);
pub const RpcRequestsPerSecond: RpcPerfCounters = RpcPerfCounters(4i32);
pub const RpcSendComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCONTEXT_QUEUE {
    pub NumberOfObjects: u32,
    pub ArrayOfObjects: *mut *mut NDR_SCONTEXT,
}
impl Default for SCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_A {
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEC_WINNT_AUTH_IDENTITY_ANSI: SEC_WINNT_AUTH_IDENTITY = SEC_WINNT_AUTH_IDENTITY(1u32);
pub const SEC_WINNT_AUTH_IDENTITY_UNICODE: SEC_WINNT_AUTH_IDENTITY = SEC_WINNT_AUTH_IDENTITY(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVER_ROUTINE = Option<unsafe extern "system" fn() -> i32>;
pub const STUB_CALL_SERVER: STUB_PHASE = STUB_PHASE(1i32);
pub const STUB_CALL_SERVER_NO_HRESULT: STUB_PHASE = STUB_PHASE(3i32);
pub const STUB_MARSHAL: STUB_PHASE = STUB_PHASE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STUB_PHASE(pub i32);
#[cfg(feature = "Win32_System_Com")]
pub type STUB_THUNK = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
pub const STUB_UNMARSHAL: STUB_PHASE = STUB_PHASE(0i32);
pub const SYSTEM_HANDLE_COMPOSITION_OBJECT: system_handle_t = system_handle_t(9i32);
pub const SYSTEM_HANDLE_EVENT: system_handle_t = system_handle_t(2i32);
pub const SYSTEM_HANDLE_FILE: system_handle_t = system_handle_t(0i32);
pub const SYSTEM_HANDLE_INVALID: system_handle_t = system_handle_t(255i32);
pub const SYSTEM_HANDLE_JOB: system_handle_t = system_handle_t(11i32);
pub const SYSTEM_HANDLE_MAX: system_handle_t = system_handle_t(12i32);
pub const SYSTEM_HANDLE_MUTEX: system_handle_t = system_handle_t(3i32);
pub const SYSTEM_HANDLE_PIPE: system_handle_t = system_handle_t(12i32);
pub const SYSTEM_HANDLE_PROCESS: system_handle_t = system_handle_t(4i32);
pub const SYSTEM_HANDLE_REG_KEY: system_handle_t = system_handle_t(7i32);
pub const SYSTEM_HANDLE_SECTION: system_handle_t = system_handle_t(6i32);
pub const SYSTEM_HANDLE_SEMAPHORE: system_handle_t = system_handle_t(1i32);
pub const SYSTEM_HANDLE_SOCKET: system_handle_t = system_handle_t(10i32);
pub const SYSTEM_HANDLE_THREAD: system_handle_t = system_handle_t(8i32);
pub const SYSTEM_HANDLE_TOKEN: system_handle_t = system_handle_t(5i32);
pub const TARGET_IS_NT100_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT1012_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT102_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT351_OR_WIN95_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT40_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT50_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT51_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT60_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT61_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT62_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT63_OR_LATER: u32 = 1u32;
pub const TRANSPORT_TYPE_CN: u32 = 1u32;
pub const TRANSPORT_TYPE_DG: u32 = 2u32;
pub const TRANSPORT_TYPE_LPC: u32 = 4u32;
pub const TRANSPORT_TYPE_WMSG: u32 = 8u32;
pub const USER_CALL_IS_ASYNC: u32 = 256u32;
pub const USER_CALL_NEW_CORRELATION_DESC: u32 = 512u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USER_MARSHAL_CB {
    pub Flags: u32,
    pub pStubMsg: *mut MIDL_STUB_MESSAGE,
    pub pReserve: *mut u8,
    pub Signature: u32,
    pub CBType: USER_MARSHAL_CB_TYPE,
    pub pFormat: *mut u8,
    pub pTypeFormat: *mut u8,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for USER_MARSHAL_CB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USER_MARSHAL_CB_BUFFER_SIZE: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(0i32);
pub const USER_MARSHAL_CB_FREE: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(3i32);
pub const USER_MARSHAL_CB_MARSHALL: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_MARSHAL_CB_TYPE(pub i32);
pub const USER_MARSHAL_CB_UNMARSHALL: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(2i32);
pub const USER_MARSHAL_FC_BYTE: u32 = 1u32;
pub const USER_MARSHAL_FC_CHAR: u32 = 2u32;
pub const USER_MARSHAL_FC_DOUBLE: u32 = 12u32;
pub const USER_MARSHAL_FC_FLOAT: u32 = 10u32;
pub const USER_MARSHAL_FC_HYPER: u32 = 11u32;
pub const USER_MARSHAL_FC_LONG: u32 = 8u32;
pub const USER_MARSHAL_FC_SHORT: u32 = 6u32;
pub const USER_MARSHAL_FC_SMALL: u32 = 3u32;
pub const USER_MARSHAL_FC_ULONG: u32 = 9u32;
pub const USER_MARSHAL_FC_USHORT: u32 = 7u32;
pub const USER_MARSHAL_FC_USMALL: u32 = 4u32;
pub const USER_MARSHAL_FC_WCHAR: u32 = 5u32;
pub type USER_MARSHAL_FREEING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut core::ffi::c_void)>;
pub type USER_MARSHAL_MARSHALLING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut core::ffi::c_void) -> *mut u8>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct USER_MARSHAL_ROUTINE_QUADRUPLE {
    pub pfnBufferSize: USER_MARSHAL_SIZING_ROUTINE,
    pub pfnMarshall: USER_MARSHAL_MARSHALLING_ROUTINE,
    pub pfnUnmarshall: USER_MARSHAL_UNMARSHALLING_ROUTINE,
    pub pfnFree: USER_MARSHAL_FREEING_ROUTINE,
}
pub type USER_MARSHAL_SIZING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: u32, param2: *mut core::ffi::c_void) -> u32>;
pub type USER_MARSHAL_UNMARSHALLING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut core::ffi::c_void) -> *mut u8>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UUID_VECTOR {
    pub Count: u32,
    pub Uuid: [*mut windows_core::GUID; 1],
}
impl Default for UUID_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XLAT_CLIENT: XLAT_SIDE = XLAT_SIDE(2i32);
pub const XLAT_SERVER: XLAT_SIDE = XLAT_SIDE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XLAT_SIDE(pub i32);
#[cfg(feature = "Win32_System_Com")]
pub type XMIT_HELPER_ROUTINE = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Default)]
pub struct XMIT_ROUTINE_QUINTUPLE {
    pub pfnTranslateToXmit: XMIT_HELPER_ROUTINE,
    pub pfnTranslateFromXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeInst: XMIT_HELPER_ROUTINE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct _NDR_PROC_CONTEXT(pub isize);
pub const __RPCPROXY_H_VERSION__: u32 = 477u32;
pub const cbNDRContext: u32 = 20u32;
pub const eeptAnsiString: ExtendedErrorParamTypes = ExtendedErrorParamTypes(1i32);
pub const eeptBinary: ExtendedErrorParamTypes = ExtendedErrorParamTypes(7i32);
pub const eeptLongVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(3i32);
pub const eeptNone: ExtendedErrorParamTypes = ExtendedErrorParamTypes(6i32);
pub const eeptPointerVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(5i32);
pub const eeptShortVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(4i32);
pub const eeptUnicodeString: ExtendedErrorParamTypes = ExtendedErrorParamTypes(2i32);
pub const rcclClientUnknownLocality: RpcCallClientLocality = RpcCallClientLocality(3i32);
pub const rcclInvalid: RpcCallClientLocality = RpcCallClientLocality(0i32);
pub const rcclLocal: RpcCallClientLocality = RpcCallClientLocality(1i32);
pub const rcclRemote: RpcCallClientLocality = RpcCallClientLocality(2i32);
pub const rctGuaranteed: RpcCallType = RpcCallType(3i32);
pub const rctInvalid: RpcCallType = RpcCallType(0i32);
pub const rctNormal: RpcCallType = RpcCallType(1i32);
pub const rctTraining: RpcCallType = RpcCallType(2i32);
pub const rlafIPv4: RpcLocalAddressFormat = RpcLocalAddressFormat(1i32);
pub const rlafIPv6: RpcLocalAddressFormat = RpcLocalAddressFormat(2i32);
pub const rlafInvalid: RpcLocalAddressFormat = RpcLocalAddressFormat(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct system_handle_t(pub i32);
