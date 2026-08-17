// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! WinSock2 Extension for TCP/IP protocols
use ctypes::c_int;
use shared::guiddef::LPGUID;
use shared::minwindef::{DWORD, INT, LPHANDLE, ULONG};
use shared::mstcpip::{
    SOCKET_PEER_TARGET_NAME, SOCKET_SECURITY_QUERY_INFO, SOCKET_SECURITY_QUERY_TEMPLATE,
    SOCKET_SECURITY_SETTINGS,
};
use shared::winerror::{
    WSAEAFNOSUPPORT, WSAEINVAL, WSAESOCKTNOSUPPORT, WSAHOST_NOT_FOUND, WSANO_RECOVERY,
    WSATRY_AGAIN, WSATYPE_NOT_FOUND, WSA_IPSEC_NAME_POLICY_ERROR, WSA_SECURE_HOST_NOT_FOUND,
};
use shared::ws2def::{
    ADDRINFOA, ADDRINFOEXA, ADDRINFOEXW, ADDRINFOW, PADDRINFOA, PADDRINFOEXA, PADDRINFOEXW,
    PADDRINFOW, SOCKADDR, SOCKET_ADDRESS,
};
use shared::wtypesbase::LPBLOB;
use um::minwinbase::LPOVERLAPPED;
use um::winnt::{PCHAR, PCSTR, PCWSTR, PSTR, PVOID, PWCHAR, PWSTR, VOID};
use um::winsock2::{
    LPWSAOVERLAPPED, LPWSAOVERLAPPED_COMPLETION_ROUTINE, SOCKET, WSA_NOT_ENOUGH_MEMORY, timeval,
};
use vc::vcruntime::size_t;
pub const UDP_NOCHECKSUM: c_int = 1;
pub const UDP_CHECKSUM_COVERAGE: c_int = 20;
pub const EAI_AGAIN: DWORD = WSATRY_AGAIN;
pub const EAI_BADFLAGS: DWORD = WSAEINVAL;
pub const EAI_FAIL: DWORD = WSANO_RECOVERY;
pub const EAI_FAMILY: DWORD = WSAEAFNOSUPPORT;
pub const EAI_MEMORY: DWORD = WSA_NOT_ENOUGH_MEMORY as u32;
pub const EAI_NOSECURENAME: DWORD = WSA_SECURE_HOST_NOT_FOUND;
pub const EAI_NONAME: DWORD = WSAHOST_NOT_FOUND;
pub const EAI_SERVICE: DWORD = WSATYPE_NOT_FOUND;
pub const EAI_SOCKTYPE: DWORD = WSAESOCKTNOSUPPORT;
pub const EAI_IPSECPOLICY: DWORD = WSA_IPSEC_NAME_POLICY_ERROR;
pub const EAI_NODATA: DWORD = EAI_NONAME;
pub type ADDRINFO = ADDRINFOA;
pub type LPADDRINFO = *mut ADDRINFOA;
extern "system" {
    pub fn getaddrinfo(
        pNodeName: PCSTR,
        pServiceName: PCSTR,
        pHints: *const ADDRINFOA,
        ppResult: *mut PADDRINFOA,
    ) -> INT;
    pub fn GetAddrInfoW(
        pNodeName: PCWSTR,
        pServiceName: PCWSTR,
        pHints: *const ADDRINFOW,
        ppResult: *mut PADDRINFOW,
    ) -> INT;
}
FN!{stdcall LPFN_GETADDRINFO(
    pNodeName: PCSTR,
    pServiceName: PCSTR,
    pHints: *const ADDRINFOA,
    ppResult: *mut PADDRINFOA,
) -> INT}
FN!{stdcall LPFN_GETADDRINFOW(
    pNodeName: PCWSTR,
    pServiceName: PCWSTR,
    pHints: *const ADDRINFOW,
    ppResult: *mut PADDRINFOW,
) -> INT}
FN!{stdcall LPLOOKUPSERVICE_COMPLETION_ROUTINE(
    dwError: DWORD,
    dwBytes: DWORD,
    lpOverlapped: LPWSAOVERLAPPED,
) -> ()}
extern "system" {
    pub fn GetAddrInfoExA(
        pName: PCSTR,
        pServiceName: PCSTR,
        dwNameSpace: DWORD,
        lpNspId: LPGUID,
        hints: *const ADDRINFOEXA,
        ppResult: *mut PADDRINFOEXA,
        timeout: *mut timeval,
        lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lpNameHandle: LPHANDLE,
    ) -> INT;
    pub fn GetAddrInfoExW(
        pName: PCWSTR,
        pServiceName: PCWSTR,
        dwNameSpace: DWORD,
        lpNspId: LPGUID,
        hints: *const ADDRINFOEXW,
        ppResult: *mut PADDRINFOEXW,
        timeout: *mut timeval,
        lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lpNameHandle: LPHANDLE,
    ) -> INT;
    pub fn GetAddrInfoExCancel(
        lpHandle: LPHANDLE,
    ) -> INT;
    pub fn GetAddrInfoExOverlappedResult(
        lpOverlapped: LPOVERLAPPED,
    ) -> INT;
}
FN!{stdcall LPFN_GETADDRINFOEXA(
    pName: PCSTR,
    pServiceName: PCSTR,
    dwNameSpace: DWORD,
    lpNspId: LPGUID,
    hints: *const ADDRINFOEXA,
    ppResult: *mut PADDRINFOEXA,
    timeout: *mut timeval,
    lpOverlapped: LPOVERLAPPED,
    lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
    lpNameHandle: LPHANDLE,
) -> INT}
FN!{stdcall LPFN_GETADDRINFOEXW(
    pName: PCWSTR,
    pServiceName: PCWSTR,
    dwNameSpace: DWORD,
    lpNspId: LPGUID,
    hints: *const ADDRINFOEXW,
    ppResult: *mut PADDRINFOEXW,
    timeout: *mut timeval,
    lpOverlapped: LPOVERLAPPED,
    lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
    lpNameHandle: LPHANDLE,
) -> INT}
FN!{stdcall LPFN_GETADDRINFOEXCANCEL(
    lpHandle: LPHANDLE,
) -> INT}
FN!{stdcall LPFN_GETADDRINFOEXOVERLAPPEDRESULT(
    lpOverlapped: LPOVERLAPPED,
) -> INT}
extern "system" {
    pub fn SetAddrInfoExA(
        pName: PCSTR,
        pServiceName: PCSTR,
        pAddresses: *mut SOCKET_ADDRESS,
        dwAddressCount: DWORD,
        lpBlob: LPBLOB,
        dwFlags: DWORD,
        dwNameSpace: DWORD,
        lpNspId: LPGUID,
        timeout: *mut timeval,
        lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lpNameHandle: LPHANDLE,
    ) -> INT;
    pub fn SetAddrInfoExW(
        pName: PCWSTR,
        pServiceName: PCWSTR,
        pAddresses: *mut SOCKET_ADDRESS,
        dwAddressCount: DWORD,
        lpBlob: LPBLOB,
        dwFlags: DWORD,
        dwNameSpace: DWORD,
        lpNspId: LPGUID,
        timeout: *mut timeval,
        lpOverlapped: LPOVERLAPPED,
        lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
        lpNameHandle: LPHANDLE,
    ) -> INT;
}
FN!{stdcall LPFN_SETADDRINFOEXA(
    pName: PCSTR,
    pServiceName: PCSTR,
    pAddresses: *mut SOCKET_ADDRESS,
    dwAddressCount: DWORD,
    lpBlob: LPBLOB,
    dwFlags: DWORD,
    dwNameSpace: DWORD,
    lpNspId: LPGUID,
    timeout: *mut timeval,
    lpOverlapped: LPOVERLAPPED,
    lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
    lpNameHandle: LPHANDLE,
) -> INT}
FN!{stdcall LPFN_SETADDRINFOEXW(
    pName: PCWSTR,
    pServiceName: PCWSTR,
    pAddresses: *mut SOCKET_ADDRESS,
    dwAddressCount: DWORD,
    lpBlob: LPBLOB,
    dwFlags: DWORD,
    dwNameSpace: DWORD,
    lpNspId: LPGUID,
    timeout: *mut timeval,
    lpOverlapped: LPOVERLAPPED,
    lpCompletionRoutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE,
    lpNameHandle: LPHANDLE,
) -> INT}
extern "system" {
    pub fn freeaddrinfo(
        pAddrInfo: PADDRINFOA,
    );
    pub fn FreeAddrInfoW(
        pAddrInfo: PADDRINFOW,
    );
}
FN!{stdcall LPFN_FREEADDRINFO(
    pAddrInfo: PADDRINFOA,
) -> ()}
FN!{stdcall LPFN_FREEADDRINFOW(
    pAddrInfo: PADDRINFOW,
) -> ()}
extern "system" {
    pub fn FreeAddrInfoEx(
        pAddrInfoEx: PADDRINFOEXA,
    );
    pub fn FreeAddrInfoExW(
        pAddrInfoEx: PADDRINFOEXW,
    );
}
FN!{stdcall LPFN_FREEADDRINFOEXA(
    pAddrInfoEx: PADDRINFOEXA,
) -> ()}
FN!{stdcall LPFN_FREEADDRINFOEXW(
    pAddrInfoEx: PADDRINFOEXW,
) -> ()}
pub type socklen_t = c_int;
extern "system" {
    pub fn getnameinfo(
        pSockaddr: *const SOCKADDR,
        SockaddrLength: socklen_t,
        pNodeBuffer: PCHAR,
        NodeBufferSize: DWORD,
        pServiceBuffer: PCHAR,
        ServiceBufferSize: DWORD,
        Flags: INT,
    ) -> INT;
    pub fn GetNameInfoW(
        pSockaddr: *const SOCKADDR,
        SockaddrLength: socklen_t,
        pNodeBuffer: PWCHAR,
        NodeBufferSize: DWORD,
        pServiceBuffer: PWCHAR,
        ServiceBufferSize: DWORD,
        Flags: INT,
    ) -> INT;
}
FN!{stdcall LPFN_GETNAMEINFO(
    pSockaddr: *const SOCKADDR,
    SockaddrLength: socklen_t,
    pNodeBuffer: PCHAR,
    NodeBufferSize: DWORD,
    pServiceBuffer: PCHAR,
    ServiceBufferSize: DWORD,
    Flags: INT,
) -> c_int}
FN!{stdcall LPFN_GETNAMEINFOW(
    pSockaddr: *const SOCKADDR,
    SockaddrLength: socklen_t,
    pNodeBuffer: PWCHAR,
    NodeBufferSize: DWORD,
    pServiceBuffer: PWCHAR,
    ServiceBufferSize: DWORD,
    Flags: INT,
) -> INT}
extern "system" {
    pub fn inet_pton(
        Family: INT,
        pszAddrString: PCSTR,
        pAddrBuf: PVOID,
    ) -> INT;
    pub fn InetPtonW(
        Family: INT,
        pszAddrString: PCWSTR,
        pAddrBuf: PVOID,
    ) -> INT;
    pub fn inet_ntop(
        Family: INT,
        pAddr: *const VOID,
        pStringBuf: PSTR,
        StringBufSize: size_t,
    ) -> PCSTR;
    pub fn InetNtopW(
        Family: INT,
        pAddr: *const VOID,
        pStringBuf: PWSTR,
        StringBufSize: size_t,
    ) -> PCWSTR;
}
FN!{stdcall LPFN_INET_PTONA(
    Family: INT,
    pszAddrString: PCSTR,
    pAddrBuf: PVOID,
) -> INT}
FN!{stdcall LPFN_INET_PTONW(
    Family: INT,
    pszAddrString: PCWSTR,
    pAddrBuf: PVOID,
) -> INT}
FN!{stdcall LPFN_INET_NTOPA(
    Family: INT,
    pAddr: *const VOID,
    pStringBuf: PSTR,
    StringBufSize: size_t,
) -> PCSTR}
FN!{stdcall LPFN_INET_NTOPW(
    Family: INT,
    pAddr: *const VOID,
    pStringBuf: PWSTR,
    StringBufSize: size_t,
) -> PCWSTR}
pub const GAI_STRERROR_BUFFER_SIZE: usize = 1024;
extern "system" {
    pub fn WSASetSocketSecurity(
        Socket: SOCKET,
        SecuritySettings: *const SOCKET_SECURITY_SETTINGS,
        SecuritySettingsLen: ULONG,
        Overlapped: LPWSAOVERLAPPED,
        CompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> INT;
    pub fn WSAQuerySocketSecurity(
        Socket: SOCKET,
        SecurityQueryTemplate: *const SOCKET_SECURITY_QUERY_TEMPLATE,
        SecurityQueryTemplateLen: ULONG,
        SecurityQueryInfo: *mut SOCKET_SECURITY_QUERY_INFO,
        SecurityQueryInfoLen: *mut ULONG,
        Overlapped: LPWSAOVERLAPPED,
        CompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> INT;
    pub fn WSASetSocketPeerTargetName(
        Socket: SOCKET,
        PeerTargetName: *const SOCKET_PEER_TARGET_NAME,
        PeerTargetNameLen: ULONG,
        Overlapped: LPWSAOVERLAPPED,
        CompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> INT;
    pub fn WSADeleteSocketPeerTargetName(
        Socket: SOCKET,
        PeerAddr: *const SOCKADDR,
        PeerAddrLen: ULONG,
        Overlapped: LPWSAOVERLAPPED,
        CompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> INT;
    pub fn WSAImpersonateSocketPeer(
        Socket: SOCKET,
        PeerAddr: *const SOCKADDR,
        PeerAddrLen: ULONG,
    ) -> INT;
    pub fn WSARevertImpersonation();
}
