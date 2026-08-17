// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Definitions to be used with the WinSock service provider.
use ctypes::{c_char, c_int, c_long, c_uint};
use shared::basetsd::{DWORD_PTR, PDWORD_PTR, ULONG_PTR};
use shared::guiddef::{GUID, LPGUID};
use shared::minwindef::{
    BOOL, DWORD, INT, LPARAM, LPDWORD, LPHANDLE, LPINT, LPVOID, PBYTE, UINT, WORD, WPARAM,
};
use shared::windef::HWND;
use shared::ws2def::{LPSOCKADDR, LPWSABUF, SOCKADDR};
use shared::wtypesbase::LPBLOB;
use um::winnt::{HANDLE, LPCWSTR, LPWSTR, PVOID, WCHAR};
use um::winsock2::{
    GROUP, LPCONDITIONPROC, LPQOS, LPWSACOMPLETION, LPWSANETWORKEVENTS, LPWSAOVERLAPPED,
    LPWSAOVERLAPPED_COMPLETION_ROUTINE, LPWSAPROTOCOL_INFOW, LPWSAQUERYSET2W, LPWSAQUERYSETW,
    LPWSASERVICECLASSINFOW, SOCKET, WSAESETSERVICEOP, WSAEVENT, fd_set, timeval,
};
#[cfg(target_pointer_width = "64")]
use um::winsock2::{LPWSANAMESPACE_INFOEXW, LPWSANAMESPACE_INFOW};
use vc::vcruntime::size_t;
pub const WSPDESCRIPTION_LEN: usize = 255;
pub const WSS_OPERATION_IN_PROGRESS: ULONG_PTR = 0x00000103;
STRUCT!{struct WSPDATA {
    wVersion: WORD,
    wHighVersion: WORD,
    szDescription: [WCHAR; WSPDESCRIPTION_LEN + 1],
}}
pub type LPWSPDATA = *mut WSPDATA;
STRUCT!{struct WSATHREADID {
    ThreadHandle: HANDLE,
    Reserved: DWORD_PTR,
}}
pub type LPWSATHREADID = *mut WSATHREADID;
FN!{stdcall LPBLOCKINGCALLBACK(
    dwContext: DWORD_PTR,
) -> BOOL}
FN!{stdcall LPWSAUSERAPC(
    dwContext: DWORD_PTR,
) -> ()}
FN!{stdcall LPWSPACCEPT(
    s: SOCKET,
    addr: *mut SOCKADDR,
    addrlen: LPINT,
    lpfnCondition: LPCONDITIONPROC,
    dwCallbackData: DWORD_PTR,
    lpErrno: LPINT,
) -> SOCKET}
FN!{stdcall LPWSPADDRESSTOSTRING(
    lpsaAddress: LPSOCKADDR,
    dwAddressLength: DWORD,
    lpProtocolInfo: LPWSAPROTOCOL_INFOW,
    lpszAddressString: LPWSTR,
    lpdwAddressStringLength: LPDWORD,
    lpErrno: LPINT,
) -> INT}
FN!{stdcall LPWSPASYNCSELECT(
    s: SOCKET,
    hWnd: HWND,
    wMsg: c_uint,
    lEvent: c_long,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPBIND(
    s: SOCKET,
    name: *mut SOCKADDR,
    namelen: c_int,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPCANCELBLOCKINGCALL(
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPCLEANUP(
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPCLOSESOCKET(
    s: SOCKET,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPCONNECT(
    s: SOCKET,
    name: *mut SOCKADDR,
    namelen: c_int,
    lpCallerData: LPWSABUF,
    lpCalleeData: LPWSABUF,
    lpSQOS: LPQOS,
    lpGQOS: LPQOS,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPDUPLICATESOCKET(
    s: SOCKET,
    dwProcessId: DWORD,
    lpProtocolInfo: LPWSAPROTOCOL_INFOW,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPENUMNETWORKEVENTS(
    s: SOCKET,
    hEventObject: WSAEVENT,
    lpNetworkEvents: LPWSANETWORKEVENTS,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPEVENTSELECT(
    s: SOCKET,
    hEventObject: WSAEVENT,
    lNetworkEvents: c_long,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPGETOVERLAPPEDRESULT(
    s: SOCKET,
    lpOverlapped: LPWSAOVERLAPPED,
    lpcbTransfer: LPDWORD,
    fWait: BOOL,
    lpdwFlags: LPDWORD,
    lpErrno: LPINT,
) -> BOOL}
FN!{stdcall LPWSPGETPEERNAME(
    s: SOCKET,
    name: *mut SOCKADDR,
    namelen: LPINT,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPGETSOCKNAME(
    s: SOCKET,
    name: *mut SOCKADDR,
    namelen: LPINT,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPGETSOCKOPT(
    s: SOCKET,
    level: c_int,
    optname: c_int,
    optval: *mut c_char,
    optlen: LPINT,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPGETQOSBYNAME(
    s: SOCKET,
    lpQOSName: LPWSABUF,
    lpQOS: LPQOS,
    lpErrno: LPINT,
) -> BOOL}
FN!{stdcall LPWSPIOCTL(
    s: SOCKET,
    dwIoControlCode: DWORD,
    lpvInBuffer: LPVOID,
    cbInBuffer: DWORD,
    lpvOutBuffer: LPVOID,
    cbOutBuffer: DWORD,
    lpcbBytesReturned: LPDWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    lpThreadId: LPWSATHREADID,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPJOINLEAF(
    s: SOCKET,
    name: *mut SOCKADDR,
    namelen: c_int,
    lpCallerData: LPWSABUF,
    lpCalleeData: LPWSABUF,
    lpSQOS: LPQOS,
    lpGQOS: LPQOS,
    dwFlags: DWORD,
    lpErrno: LPINT,
) -> SOCKET}
FN!{stdcall LPWSPLISTEN(
    s: SOCKET,
    backlog: c_int,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPRECV(
    s: SOCKET,
    lpBuffers: LPWSABUF,
    dwBufferCount: DWORD,
    lpNumberOfBytesRecvd: LPDWORD,
    lpFlags: LPDWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    lpThreadId: LPWSATHREADID,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPRECVDISCONNECT(
    s: SOCKET,
    lpInboundDisconnectData: LPWSABUF,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPRECVFROM(
    s: SOCKET,
    lpBuffers: LPWSABUF,
    dwBufferCount: DWORD,
    lpNumberOfBytesRecvd: LPDWORD,
    lpFlags: LPDWORD,
    lpFrom: *mut SOCKADDR,
    lpFromlen: LPINT,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    lpThreadId: LPWSATHREADID,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPSELECT(
    nfds: c_int,
    readfds: *mut fd_set,
    writefds: *mut fd_set,
    exceptfds: *mut fd_set,
    timeout: *const timeval,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPSEND(
    s: SOCKET,
    lpBuffers: LPWSABUF,
    dwBufferCount: DWORD,
    lpNumberOfBytesSent: LPDWORD,
    dwFlags: DWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    lpThreadId: LPWSATHREADID,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPSENDDISCONNECT(
    s: SOCKET,
    lpOutboundDisconnectData: LPWSABUF,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPSENDTO(
    s: SOCKET,
    lpBuffers: LPWSABUF,
    dwBufferCount: DWORD,
    lpNumberOfBytesSent: LPDWORD,
    dwFlags: DWORD,
    lpTo: *const SOCKADDR,
    iTolen: c_int,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    lpThreadId: LPWSATHREADID,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPSETSOCKOPT(
    s: SOCKET,
    level: c_int,
    optname: c_int,
    optval: *const c_char,
    optlen: c_int,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPSHUTDOWN(
    s: SOCKET,
    how: c_int,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWSPSOCKET(
    af: c_int,
    _type: c_int,
    protocol: c_int,
    lpProtocolInfo: LPWSAPROTOCOL_INFOW,
    g: GROUP,
    dwFlags: DWORD,
    lpErrno: LPINT,
) -> SOCKET}
FN!{stdcall LPWSPSTRINGTOADDRESS(
    AddressString: LPWSTR,
    AddressFamily: INT,
    lpProtocolInfo: LPWSAPROTOCOL_INFOW,
    lpAddress: LPSOCKADDR,
    lpAddressLength: LPINT,
    lpErrno: LPINT,
) -> c_int}
STRUCT!{struct WSPPROC_TABLE {
    lpWSPAccept: LPWSPACCEPT,
    lpWSPAddressToString: LPWSPADDRESSTOSTRING,
    lpWSPAsyncSelect: LPWSPASYNCSELECT,
    lpWSPBind: LPWSPBIND,
    lpWSPCancelBlockingCall: LPWSPCANCELBLOCKINGCALL,
    lpWSPCleanup: LPWSPCLEANUP,
    lpWSPCloseSocket: LPWSPCLOSESOCKET,
    lpWSPConnect: LPWSPCONNECT,
    lpWSPDuplicateSocket: LPWSPDUPLICATESOCKET,
    lpWSPEnumNetworkEvents: LPWSPENUMNETWORKEVENTS,
    lpWSPEventSelect: LPWSPEVENTSELECT,
    lpWSPGetOverlappedResult: LPWSPGETOVERLAPPEDRESULT,
    lpWSPGetPeerName: LPWSPGETPEERNAME,
    lpWSPGetSockName: LPWSPGETSOCKNAME,
    lpWSPGetSockOpt: LPWSPGETSOCKOPT,
    lpWSPGetQOSByName: LPWSPGETQOSBYNAME,
    lpWSPIoctl: LPWSPIOCTL,
    lpWSPJoinLeaf: LPWSPJOINLEAF,
    lpWSPListen: LPWSPLISTEN,
    lpWSPRecv: LPWSPRECV,
    lpWSPRecvDisconnect: LPWSPRECVDISCONNECT,
    lpWSPRecvFrom: LPWSPRECVFROM,
    lpWSPSelect: LPWSPSELECT,
    lpWSPSend: LPWSPSEND,
    lpWSPSendDisconnect: LPWSPSENDDISCONNECT,
    lpWSPSendTo: LPWSPSENDTO,
    lpWSPSetSockOpt: LPWSPSETSOCKOPT,
    lpWSPShutdown: LPWSPSHUTDOWN,
    lpWSPSocket: LPWSPSOCKET,
    lpWSPStringToAddress: LPWSPSTRINGTOADDRESS,
}}
pub type LPWSPPROC_TABLE = *mut WSPPROC_TABLE;
FN!{stdcall LPWPUCLOSEEVENT(
    hEvent: WSAEVENT,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWPUCLOSESOCKETHANDLE(
    s: SOCKET,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWPUCREATEEVENT(
    lpErrno: LPINT,
) -> WSAEVENT}
FN!{stdcall LPWPUCREATESOCKETHANDLE(
    dwCatalogEntryId: DWORD,
    dwContext: DWORD_PTR,
    lpErrno: LPINT,
) -> SOCKET}
FN!{stdcall LPWPUFDISSET(
    s: SOCKET,
    fdset: *mut fd_set,
) -> c_int}
FN!{stdcall LPWPUGETPROVIDERPATH(
    lpProviderId: LPGUID,
    lpszProviderDllPath: *mut WCHAR,
    lpProviderDllPathLen: LPINT,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWPUMODIFYIFSHANDLE(
    dwCatalogEntryId: DWORD,
    ProposedHandle: SOCKET,
    lpErrno: LPINT,
) -> SOCKET}
FN!{stdcall LPWPUPOSTMESSAGE(
    hWnd: HWND,
    Msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> BOOL}
FN!{stdcall LPWPUQUERYBLOCKINGCALLBACK(
    dwCatalogEntryId: DWORD,
    lplpfnCallback: *mut LPBLOCKINGCALLBACK,
    lpdwContext: PDWORD_PTR,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWPUQUERYSOCKETHANDLECONTEXT(
    s: SOCKET,
    lpContext: PDWORD_PTR,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWPUQUEUEAPC(
    lpThreadId: LPWSATHREADID,
    lpfnUserApc: LPWSAUSERAPC,
    dwContext: DWORD_PTR,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWPURESETEVENT(
    hEvent: WSAEVENT,
    lpErrno: LPINT,
) -> BOOL}
FN!{stdcall LPWPUSETEVENT(
    hEvent: WSAEVENT,
    lpErrno: LPINT,
) -> BOOL}
FN!{stdcall LPWPUOPENCURRENTTHREAD(
    lpThreadId: LPWSATHREADID,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWPUCLOSETHREAD(
    lpThreadId: LPWSATHREADID,
    lpErrno: LPINT,
) -> c_int}
FN!{stdcall LPWPUCOMPLETEOVERLAPPEDREQUEST(
    s: SOCKET,
    lpOverlapped: LPWSAOVERLAPPED,
    dwError: DWORD,
    cbTransferred: DWORD,
    lpErrno: LPINT,
) -> c_int}
STRUCT!{struct WSPUPCALLTABLE {
    lpWPUCloseEvent: LPWPUCLOSEEVENT,
    lpWPUCloseSocketHandle: LPWPUCLOSESOCKETHANDLE,
    lpWPUCreateEvent: LPWPUCREATEEVENT,
    lpWPUCreateSocketHandle: LPWPUCREATESOCKETHANDLE,
    lpWPUFDIsSet: LPWPUFDISSET,
    lpWPUGetProviderPath: LPWPUGETPROVIDERPATH,
    lpWPUModifyIFSHandle: LPWPUMODIFYIFSHANDLE,
    lpWPUPostMessage: LPWPUPOSTMESSAGE,
    lpWPUQueryBlockingCallback: LPWPUQUERYBLOCKINGCALLBACK,
    lpWPUQuerySocketHandleContext: LPWPUQUERYSOCKETHANDLECONTEXT,
    lpWPUQueueApc: LPWPUQUEUEAPC,
    lpWPUResetEvent: LPWPURESETEVENT,
    lpWPUSetEvent: LPWPUSETEVENT,
    lpWPUOpenCurrentThread: LPWPUOPENCURRENTTHREAD,
    lpWPUCloseThread: LPWPUCLOSETHREAD,
}}
pub type LPWSPUPCALLTABLE = *mut WSPUPCALLTABLE;
extern "system" {
    pub fn WSPStartup(
        wVersionRequested: WORD,
        lpWSPData: LPWSPDATA,
        lpProtocolInfo: LPWSAPROTOCOL_INFOW,
        UpcallTable: WSPUPCALLTABLE,
        lpProcTable: LPWSPPROC_TABLE,
    ) -> c_int;
}
FN!{stdcall LPWSPSTARTUP(
    wVersionRequested: WORD,
    lpWSPData: LPWSPDATA,
    lpProtocolInfo: LPWSAPROTOCOL_INFOW,
    UpcallTable: WSPUPCALLTABLE,
    lpProcTable: LPWSPPROC_TABLE,
) -> c_int}
extern "system" {
    pub fn WSCEnumProtocols(
        lpiProtocols: LPINT,
        lpProtocolBuffer: LPWSAPROTOCOL_INFOW,
        lpdwBufferLength: LPDWORD,
        lpErrno: LPINT,
    ) -> c_int;
}
FN!{stdcall LPWSCENUMPROTOCOLS(
    lpiProtocols: LPINT,
    lpProtocolBuffer: LPWSAPROTOCOL_INFOW,
    lpdwBufferLength: LPDWORD,
    lpErrno: LPINT,
) -> c_int}
extern "system" {
    #[cfg(target_pointer_width = "64")]
    pub fn WSCEnumProtocols32(
        lpiProtocols: LPINT,
        lpProtocolBuffer: LPWSAPROTOCOL_INFOW,
        lpdwBufferLength: LPDWORD,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WSCDeinstallProvider(
        lpProviderId: LPGUID,
        lpErrno: LPINT,
    ) -> c_int;
}
FN!{stdcall LPWSCDEINSTALLPROVIDER(
    lpProviderId: LPGUID,
    lpErrno: LPINT,
) -> c_int}
extern "system" {
    #[cfg(target_pointer_width = "64")]
    pub fn WSCDeinstallProvider32(
        lpProviderId: LPGUID,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WSCInstallProvider(
        lpProviderId: LPGUID,
        lpszProviderDllPath: *const WCHAR,
        lpProtocolInfoList: LPWSAPROTOCOL_INFOW,
        dwNumberOfEntries: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
}
FN!{stdcall LPWSCINSTALLPROVIDER(
    lpProviderId: LPGUID,
    lpszProviderDllPath: *const WCHAR,
    lpProtocolInfoList: LPWSAPROTOCOL_INFOW,
    dwNumberOfEntries: DWORD,
    lpErrno: LPINT,
) -> c_int}
extern "system" {
    #[cfg(target_pointer_width = "64")]
    pub fn WSCInstallProvider64_32(
        lpProviderId: LPGUID,
        lpszProviderDllPath: *const WCHAR,
        lpProtocolInfoList: LPWSAPROTOCOL_INFOW,
        dwNumberOfEntries: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WSCGetProviderPath(
        lpProviderId: LPGUID,
        lpszProviderDllPath: *mut WCHAR,
        lpProviderDllPathLen: LPINT,
        lpErrno: LPINT,
    ) -> c_int;
}
FN!{stdcall LPWSCGETPROVIDERPATH(
    lpProviderId: LPGUID,
    lpszProviderDllPath: *mut WCHAR,
    lpProviderDllPathLen: LPINT,
    lpErrno: LPINT,
) -> c_int}
extern "system" {
    #[cfg(target_pointer_width = "64")]
    pub fn WSCGetProviderPath32(
        lpProviderId: LPGUID,
        lpszProviderDllPath: *mut WCHAR,
        lpProviderDllPathLen: LPINT,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WSCUpdateProvider(
        lpProviderId: LPGUID,
        lpszProviderDllPath: *const WCHAR,
        lpProtocolInfoList: LPWSAPROTOCOL_INFOW,
        dwNumberOfEntries: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
}
FN!{stdcall LPWSCUPDATEPROVIDER(
    lpProviderId: LPGUID,
    lpszProviderDllPath: *const WCHAR,
    lpProtocolInfoList: LPWSAPROTOCOL_INFOW,
    dwNumberOfEntries: DWORD,
    lpErrno: LPINT,
) -> c_int}
#[cfg(target_pointer_width = "64")]
extern "system" {
    pub fn WSCUpdateProvider32(
        lpProviderId: LPGUID,
        lpszProviderDllPath: *const WCHAR,
        lpProtocolInfoList: LPWSAPROTOCOL_INFOW,
        dwNumberOfEntries: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
}
pub const LSP_SYSTEM: DWORD = 0x80000000;
pub const LSP_INSPECTOR: DWORD = 0x00000001;
pub const LSP_REDIRECTOR: DWORD = 0x00000002;
pub const LSP_PROXY: DWORD = 0x00000004;
pub const LSP_FIREWALL: DWORD = 0x00000008;
pub const LSP_INBOUND_MODIFY: DWORD = 0x00000010;
pub const LSP_OUTBOUND_MODIFY: DWORD = 0x00000020;
pub const LSP_CRYPTO_COMPRESS: DWORD = 0x00000040;
pub const LSP_LOCAL_CACHE: DWORD = 0x00000080;
ENUM!{enum WSC_PROVIDER_INFO_TYPE {
    ProviderInfoLspCategories,
    ProviderInfoAudit,
}}
STRUCT!{struct WSC_PROVIDER_AUDIT_INFO {
    RecordSize: DWORD,
    Reserved: PVOID,
}}
extern "system" {
    pub fn WSCSetProviderInfo(
        lpProviderId: LPGUID,
        InfoType: WSC_PROVIDER_INFO_TYPE,
        Info: PBYTE,
        InfoSize: size_t,
        Flags: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WSCGetProviderInfo(
        lpProviderId: LPGUID,
        InfoType: WSC_PROVIDER_INFO_TYPE,
        Info: PBYTE,
        InfoSize: *mut size_t,
        Flags: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    #[cfg(target_pointer_width = "64")]
    pub fn WSCSetProviderInfo32(
        lpProviderId: LPGUID,
        InfoType: WSC_PROVIDER_INFO_TYPE,
        Info: PBYTE,
        InfoSize: size_t,
        Flags: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    #[cfg(target_pointer_width = "64")]
    pub fn WSCGetProviderInfo32(
        lpProviderId: LPGUID,
        InfoType: WSC_PROVIDER_INFO_TYPE,
        Info: PBYTE,
        InfoSize: *mut size_t,
        Flags: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WSCSetApplicationCategory(
        Path: LPCWSTR,
        PathLength: DWORD,
        Extra: LPCWSTR,
        ExtraLength: DWORD,
        PermittedLspCategories: DWORD,
        pPrevPermLspCat: *mut DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WSCGetApplicationCategory(
        Path: LPCWSTR,
        PathLength: DWORD,
        Extra: LPCWSTR,
        ExtraLength: DWORD,
        pPermittedLspCategories: *mut DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WPUCloseEvent(
        hEvent: WSAEVENT,
        lpErrno: LPINT,
    ) -> BOOL;
    pub fn WPUCloseSocketHandle(
        s: SOCKET,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WPUCreateEvent(
        lpErrno: LPINT,
    ) -> WSAEVENT;
    pub fn WPUCreateSocketHandle(
        dwCatalogEntryId: DWORD,
        dwContext: DWORD_PTR,
        lpErrno: LPINT,
    ) -> SOCKET;
    pub fn WPUFDIsSet(
        s: SOCKET,
        fdset: *mut fd_set,
    ) -> c_int;
    pub fn WPUGetProviderPath(
        lpProviderId: LPGUID,
        lpszProviderDllPath: *mut WCHAR,
        lpProviderDllPathLen: LPINT,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WPUModifyIFSHandle(
        dwCatalogEntryId: DWORD,
        ProposedHandle: SOCKET,
        lpErrno: LPINT,
    ) -> SOCKET;
    pub fn WPUPostMessage(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn WPUQueryBlockingCallback(
        dwCatalogEntryId: DWORD,
        lplpfnCallback: *mut LPBLOCKINGCALLBACK,
        lpdwContext: PDWORD_PTR,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WPUQuerySocketHandleContext(
        s: SOCKET,
        lpContext: PDWORD_PTR,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WPUQueueApc(
        lpThreadId: LPWSATHREADID,
        lpfnUserApc: LPWSAUSERAPC,
        dwContext: DWORD_PTR,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WPUResetEvent(
        hEvent: WSAEVENT,
        lpErrno: LPINT,
    ) -> BOOL;
    pub fn WPUSetEvent(
        hEvent: WSAEVENT,
        lpErrno: LPINT,
    ) -> BOOL;
    pub fn WPUCompleteOverlappedRequest(
        s: SOCKET,
        lpOverlapped: LPWSAOVERLAPPED,
        dwError: DWORD,
        cbTransferred: DWORD,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WPUOpenCurrentThread(
        lpThreadId: LPWSATHREADID,
        lpErrno: LPINT,
    ) -> c_int;
    pub fn WPUCloseThread(
        lpThreadId: LPWSATHREADID,
        lpErrno: LPINT,
    ) -> c_int;
    #[cfg(target_pointer_width = "64")]
    pub fn WSCEnumNameSpaceProviders32(
        lpdwBufferLength: LPDWORD,
        lpnspBuffer: LPWSANAMESPACE_INFOW,
    ) -> INT;
    #[cfg(target_pointer_width = "64")]
    pub fn WSCEnumNameSpaceProvidersEx32(
        lpdwBufferLength: LPDWORD,
        lpnspBuffer: LPWSANAMESPACE_INFOEXW,
    ) -> INT;
    pub fn WSCInstallNameSpace(
        lpszIdentifier: LPWSTR,
        lpszPathName: LPWSTR,
        dwNameSpace: DWORD,
        dwVersion: DWORD,
        lpProviderId: LPGUID,
    ) -> INT;
}
FN!{stdcall LPWSCINSTALLNAMESPACE(
    lpszIdentifier: LPWSTR,
    lpszPathName: LPWSTR,
    dwNameSpace: DWORD,
    dwVersion: DWORD,
    lpProviderId: LPGUID,
) -> INT}
extern "system" {
    #[cfg(target_pointer_width = "64")]
    pub fn WSCInstallNameSpace32(
        lpszIdentifier: LPWSTR,
        lpszPathName: LPWSTR,
        dwNameSpace: DWORD,
        dwVersion: DWORD,
        lpProviderId: LPGUID,
    ) -> INT;
    pub fn WSCUnInstallNameSpace(
        lpProviderId: LPGUID,
    ) -> INT;
}
FN!{stdcall LPWSCUNINSTALLNAMESPACE(
    lpProviderId: LPGUID,
) -> INT}
extern "system" {
    pub fn WSCInstallNameSpaceEx(
        lpszIdentifier: LPWSTR,
        lpszPathName: LPWSTR,
        dwNameSpace: DWORD,
        dwVersion: DWORD,
        lpProviderId: LPGUID,
        lpProviderSpecific: LPBLOB,
    ) -> INT;
    #[cfg(target_pointer_width = "64")]
    pub fn WSCInstallNameSpaceEx32(
        lpszIdentifier: LPWSTR,
        lpszPathName: LPWSTR,
        dwNameSpace: DWORD,
        dwVersion: DWORD,
        lpProviderId: LPGUID,
        lpProviderSpecific: LPBLOB,
    ) -> INT;
    #[cfg(target_pointer_width = "64")]
    pub fn WSCUnInstallNameSpace32(
        lpProviderId: LPGUID,
    ) -> INT;
    pub fn WSCEnableNSProvider(
        lpProviderId: LPGUID,
        fEnable: BOOL,
    ) -> INT;
}
FN!{stdcall LPWSCENABLENSPROVIDER(
    lpProviderId: LPGUID,
    fEnable: BOOL,
) -> INT}
extern "system" {
    #[cfg(target_pointer_width = "64")]
    pub fn WSCEnableNSProvider32(
        lpProviderId: LPGUID,
        fEnable: BOOL,
    ) -> INT;
    #[cfg(target_pointer_width = "64")]
    pub fn WSCInstallProviderAndChains64_32(
        lpProviderId: LPGUID,
        lpszProviderDllPath: LPWSTR,
        lpszProviderDllPath32: LPWSTR,
        lpszLspName: LPWSTR,
        dwServiceFlags: DWORD,
        lpProtocolInfoList: LPWSAPROTOCOL_INFOW,
        dwNumberOfEntries: DWORD,
        lpdwCatalogEntryId: LPDWORD,
        lpErrno: LPINT,
    ) -> c_int;
    #[cfg(target_pointer_width = "32")]
    pub fn WSCInstallProviderAndChains(
        lpProviderId: LPGUID,
        lpszProviderDllPath: LPWSTR,
        lpszLspName: LPWSTR,
        dwServiceFlags: DWORD,
        lpProtocolInfoList: LPWSAPROTOCOL_INFOW,
        dwNumberOfEntries: DWORD,
        lpdwCatalogEntryId: LPDWORD,
        lpErrno: LPINT,
    ) -> c_int;
}
FN!{stdcall LPNSPCLEANUP(
    lpProviderId: LPGUID,
) -> INT}
FN!{stdcall LPNSPLOOKUPSERVICEBEGIN(
    lpProviderId: LPGUID,
    lpqsRestrictions: LPWSAQUERYSETW,
    lpServiceClassInfo: LPWSASERVICECLASSINFOW,
    dwControlFlags: DWORD,
    lphLookup: LPHANDLE,
) -> INT}
FN!{stdcall LPNSPLOOKUPSERVICENEXT(
    hLookup: HANDLE,
    dwControlFlags: DWORD,
    lpdwBufferLength: LPDWORD,
    lpqsResults: LPWSAQUERYSETW,
) -> INT}
FN!{stdcall LPNSPIOCTL(
    hLookup: HANDLE,
    dwControlCode: DWORD,
    lpvInBuffer: LPVOID,
    cbInBuffer: DWORD,
    lpvOutBuffer: LPVOID,
    cbOutBuffer: DWORD,
    lpcbBytesReturned: LPDWORD,
    lpCompletion: LPWSACOMPLETION,
    lpThreadId: LPWSATHREADID,
) -> INT}
FN!{stdcall LPNSPLOOKUPSERVICEEND(
    hLookup: HANDLE,
) -> INT}
FN!{stdcall LPNSPSETSERVICE(
    lpProviderId: LPGUID,
    lpServiceClassInfo: LPWSASERVICECLASSINFOW,
    lpqsRegInfo: LPWSAQUERYSETW,
    essOperation: WSAESETSERVICEOP,
    dwControlFlags: DWORD,
) -> INT}
FN!{stdcall LPNSPINSTALLSERVICECLASS(
    lpProviderId: LPGUID,
    lpServiceClassInfo: LPWSASERVICECLASSINFOW,
) -> INT}
FN!{stdcall LPNSPREMOVESERVICECLASS(
    lpProviderId: LPGUID,
    lpServiceClassId: LPGUID,
) -> INT}
FN!{stdcall LPNSPGETSERVICECLASSINFO(
    lpProviderId: LPGUID,
    lpdwBufSize: LPDWORD,
    lpServiceClassInfo: LPWSASERVICECLASSINFOW,
) -> INT}
STRUCT!{struct NSP_ROUTINE {
    cbSize: DWORD,
    dwMajorVersion: DWORD,
    dwMinorVersion: DWORD,
    NSPCleanup: LPNSPCLEANUP,
    NSPLookupServiceBegin: LPNSPLOOKUPSERVICEBEGIN,
    NSPLookupServiceNext: LPNSPLOOKUPSERVICENEXT,
    NSPLookupServiceEnd: LPNSPLOOKUPSERVICEEND,
    NSPSetService: LPNSPSETSERVICE,
    NSPInstallServiceClass: LPNSPINSTALLSERVICECLASS,
    NSPRemoveServiceClass: LPNSPREMOVESERVICECLASS,
    NSPGetServiceClassInfo: LPNSPGETSERVICECLASSINFO,
    NSPIoctl: LPNSPIOCTL,
}}
pub type LPNSP_ROUTINE = *mut NSP_ROUTINE;
extern "system" {
    pub fn NSPStartup(
        lpProviderId: LPGUID,
        lpnspRoutines: LPNSP_ROUTINE,
    ) -> INT;
}
FN!{stdcall LPNSPSTARTUP(
    lpProviderId: LPGUID,
    lpnspRoutines: LPNSP_ROUTINE,
) -> INT}
FN!{stdcall LPNSPV2STARTUP(
    lpProviderId: LPGUID,
    ppvClientSessionArg: *mut LPVOID,
) -> INT}
FN!{stdcall LPNSPV2CLEANUP(
    lpProviderId: LPGUID,
    pvClientSessionArg: LPVOID,
) -> INT}
FN!{stdcall LPNSPV2LOOKUPSERVICEBEGIN(
    lpProviderId: LPGUID,
    lpqsRestrictions: LPWSAQUERYSET2W,
    dwControlFlags: DWORD,
    lpvClientSessionArg: LPVOID,
    lphLookup: LPHANDLE,
) -> INT}
FN!{stdcall LPNSPV2LOOKUPSERVICENEXTEX(
    hAsyncCall: HANDLE,
    hLookup: HANDLE,
    dwControlFlags: DWORD,
    lpdwBufferLength: LPDWORD,
    lpqsResults: LPWSAQUERYSET2W,
) -> ()}
FN!{stdcall LPNSPV2LOOKUPSERVICEEND(
    hLookup: HANDLE,
) -> INT}
FN!{stdcall LPNSPV2SETSERVICEEX(
    hAsyncCall: HANDLE,
    lpProviderId: LPGUID,
    lpqsRegInfo: LPWSAQUERYSET2W,
    essOperation: WSAESETSERVICEOP,
    dwControlFlags: DWORD,
    lpvClientSessionArg: LPVOID,
) -> ()}
FN!{stdcall LPNSPV2CLIENTSESSIONRUNDOWN(
    lpProviderId: LPGUID,
    pvClientSessionArg: LPVOID,
) -> ()}
STRUCT!{struct NSPV2_ROUTINE {
    cbSize: DWORD,
    dwMajorVersion: DWORD,
    dwMinorVersion: DWORD,
    NSPv2Startup: LPNSPV2STARTUP,
    NSPv2Cleanup: LPNSPV2CLEANUP,
    NSPv2LookupServiceBegin: LPNSPV2LOOKUPSERVICEBEGIN,
    NSPv2LookupServiceNextEx: LPNSPV2LOOKUPSERVICENEXTEX,
    NSPv2LookupServiceEnd: LPNSPV2LOOKUPSERVICEEND,
    NSPv2SetServiceEx: LPNSPV2SETSERVICEEX,
    NSPv2ClientSessionRundown: LPNSPV2CLIENTSESSIONRUNDOWN,
}}
pub type PNSPV2_ROUTINE = *mut NSPV2_ROUTINE;
pub type LPNSPV2_ROUTINE = *mut NSPV2_ROUTINE;
pub type PCNSPV2_ROUTINE = *const NSPV2_ROUTINE;
pub type LPCNSPV2_ROUTINE = *const NSPV2_ROUTINE;
extern "system" {
    pub fn WSAAdvertiseProvider(
        puuidProviderId: *const GUID,
        pNSPv2Routine: *const LPCNSPV2_ROUTINE,
    ) -> INT;
    pub fn WSAUnadvertiseProvider(
        puuidProviderId: *const GUID,
    ) -> INT;
    pub fn WSAProviderCompleteAsyncCall(
        hAsyncCall: HANDLE,
        iRetCode: INT,
    ) -> INT;
}
