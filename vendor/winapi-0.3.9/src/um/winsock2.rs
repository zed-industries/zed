// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Definitions to be used with the WinSock 2 DLL and WinSock 2 applications.
use ctypes::{
    __uint32, __uint64, c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint,
    c_ulong, c_ushort,
};
use shared::basetsd::{DWORD_PTR, UINT_PTR, ULONG_PTR};
use shared::guiddef::{GUID, LPGUID};
use shared::inaddr::in_addr;
use shared::minwindef::{
    BOOL, DWORD, FARPROC, HIWORD, INT, LOWORD, LPDWORD, LPHANDLE, LPINT, LPVOID, MAKELONG, UINT,
    ULONG, WORD, WPARAM,
};
use shared::qos::FLOWSPEC;
use shared::windef::HWND;
use shared::winerror::{
    ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER, ERROR_IO_INCOMPLETE, ERROR_IO_PENDING,
    ERROR_NOT_ENOUGH_MEMORY, ERROR_OPERATION_ABORTED, WAIT_TIMEOUT,
};
use shared::ws2def::{
    AF_APPLETALK, AF_ATM, AF_BAN, AF_BTH, AF_CCITT, AF_CHAOS, AF_DATAKIT, AF_DECnet, AF_DLI,
    AF_ECMA, AF_FIREFOX, AF_HYLINK, AF_IMPLINK, AF_INET, AF_INET6, AF_IPX, AF_ISO, AF_LAT,
    AF_MAX, AF_NS, AF_OSI, AF_PUP, AF_SNA, AF_UNIX, AF_UNKNOWN1, AF_UNSPEC, AF_VOICEVIEW,
    INADDR_ANY, LPCSADDR_INFO, LPSOCKADDR, LPWSABUF, LPWSAMSG, PSOCKET_ADDRESS_LIST, SOCKADDR,
    SOCKADDR_IN, WSABUF,
};
use shared::wtypesbase::{BLOB, LPBLOB};
use um::minwinbase::OVERLAPPED;
use um::winbase::{INFINITE, WAIT_FAILED, WAIT_IO_COMPLETION, WAIT_OBJECT_0};
use um::winnt::{
    CHAR, HANDLE, LONG, LPCSTR, LPSTR, LPWSTR, MAXIMUM_WAIT_OBJECTS, PWSTR, SHORT, WCHAR,
};
pub const WINSOCK_VERSION: WORD = 2 | (2 << 8);
pub type u_char = c_uchar;
pub type u_short = c_ushort;
pub type u_int = c_uint;
pub type u_long = c_ulong;
pub type u_int64 = __uint64;
pub type SOCKET = UINT_PTR;
pub const FD_SETSIZE: usize = 64;
STRUCT!{struct fd_set {
    fd_count: u_int,
    fd_array: [SOCKET; FD_SETSIZE],
}}
extern "system" {
    pub fn __WSAFDIsSet(
        fd: SOCKET,
        _: *mut fd_set,
    ) -> c_int;
}
STRUCT!{struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}}
pub const IOCPARM_MASK: c_long = 0x7f;
pub const IOC_VOID: c_long = 0x20000000;
pub const IOC_OUT: c_long = 0x40000000;
pub const IOC_IN: c_long = 0x80000000;
pub const IOC_INOUT: c_long = IOC_IN | IOC_OUT;
pub const FIONREAD: c_long = IOC_OUT | ((4 & IOCPARM_MASK) << 16) | (0x66 << 8) | 127;
pub const FIONBIO: c_long = IOC_IN | ((4 & IOCPARM_MASK) << 16) | (0x66 << 8) | 126;
pub const FIOASYNC: c_long = IOC_IN | ((4 & IOCPARM_MASK) << 16) | (0x66 << 8) | 125;
pub const SIOCSHIWAT: c_long = IOC_IN | ((4 & IOCPARM_MASK) << 16) | (0x73 << 8) | 0;
pub const SIOCGHIWAT: c_long = IOC_OUT | ((4 & IOCPARM_MASK) << 16) | (0x73 << 8) | 1;
pub const SIOCSLOWAT: c_long = IOC_IN | ((4 & IOCPARM_MASK) << 16) | (0x73 << 8) | 2;
pub const SIOCGLOWAT: c_long = IOC_OUT | ((4 & IOCPARM_MASK) << 16) | (0x73 << 8) | 3;
pub const SIOCATMARK: c_long = IOC_OUT | ((4 & IOCPARM_MASK) << 16) | (0x73 << 8) | 7;
STRUCT!{struct hostent {
    h_name: *mut c_char,
    h_aliases: *mut *mut c_char,
    h_addrtype: c_short,
    h_length: c_short,
    h_addr_list: *mut *mut c_char,
}}
STRUCT!{struct netent {
    n_name: *mut c_char,
    n_aliases: *mut *mut c_char,
    n_addrtype: c_short,
    n_net: u_long,
}}
#[cfg(target_pointer_width = "32")]
STRUCT!{struct servent {
    s_name: *mut c_char,
    s_aliases: *mut *mut c_char,
    s_port: c_short,
    s_proto: *mut c_char,
}}
#[cfg(target_pointer_width = "64")]
STRUCT!{struct servent {
    s_name: *mut c_char,
    s_aliases: *mut *mut c_char,
    s_proto: *mut c_char,
    s_port: c_short,
}}
STRUCT!{struct protoent {
    p_name: *mut c_char,
    p_aliases: *mut *mut c_char,
    p_proto: c_short,
}}
pub const IPPORT_ECHO: c_short = 7;
pub const IPPORT_DISCARD: c_short = 9;
pub const IPPORT_SYSTAT: c_short = 11;
pub const IPPORT_DAYTIME: c_short = 13;
pub const IPPORT_NETSTAT: c_short = 15;
pub const IPPORT_FTP: c_short = 21;
pub const IPPORT_TELNET: c_short = 23;
pub const IPPORT_SMTP: c_short = 25;
pub const IPPORT_TIMESERVER: c_short = 37;
pub const IPPORT_NAMESERVER: c_short = 42;
pub const IPPORT_WHOIS: c_short = 43;
pub const IPPORT_MTP: c_short = 57;
pub const IPPORT_TFTP: c_short = 69;
pub const IPPORT_RJE: c_short = 77;
pub const IPPORT_FINGER: c_short = 79;
pub const IPPORT_TTYLINK: c_short = 87;
pub const IPPORT_SUPDUP: c_short = 95;
pub const IPPORT_EXECSERVER: c_short = 512;
pub const IPPORT_LOGINSERVER: c_short = 513;
pub const IPPORT_CMDSERVER: c_short = 514;
pub const IPPORT_EFSSERVER: c_short = 520;
pub const IPPORT_BIFFUDP: c_short = 512;
pub const IPPORT_WHOSERVER: c_short = 513;
pub const IPPORT_ROUTESERVER: c_short = 520;
pub const IPPORT_RESERVED: c_short = 1024;
pub const IMPLINK_IP: c_short = 155;
pub const IMPLINK_LOWEXPER: c_short = 156;
pub const IMPLINK_HIGHEXPER: c_short = 158;
pub const ADDR_ANY: ULONG = INADDR_ANY;
pub const WSADESCRIPTION_LEN: usize = 256;
pub const WSASYS_STATUS_LEN: usize = 128;
#[cfg(target_pointer_width = "32")]
STRUCT!{struct WSADATA {
    wVersion: WORD,
    wHighVersion: WORD,
    szDescription: [c_char; WSADESCRIPTION_LEN + 1],
    szSystemStatus: [c_char; WSASYS_STATUS_LEN + 1],
    iMaxSockets: c_ushort,
    iMaxUdpDg: c_ushort,
    lpVendorInfo: *mut c_char,
}}
#[cfg(target_pointer_width = "64")]
STRUCT!{struct WSADATA {
    wVersion: WORD,
    wHighVersion: WORD,
    iMaxSockets: c_ushort,
    iMaxUdpDg: c_ushort,
    lpVendorInfo: *mut c_char,
    szDescription: [c_char; WSADESCRIPTION_LEN + 1],
    szSystemStatus: [c_char; WSASYS_STATUS_LEN + 1],
}}
pub type LPWSADATA = *mut WSADATA;
pub const INVALID_SOCKET: SOCKET = !0;
pub const SOCKET_ERROR: c_int = -1;
pub const FROM_PROTOCOL_INFO: c_int = -1;
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;
pub const SOCK_SEQPACKET: c_int = 5;
pub const SO_DEBUG: c_int = 0x0001;
pub const SO_ACCEPTCONN: c_int = 0x0002;
pub const SO_REUSEADDR: c_int = 0x0004;
pub const SO_KEEPALIVE: c_int = 0x0008;
pub const SO_DONTROUTE: c_int = 0x0010;
pub const SO_BROADCAST: c_int = 0x0020;
pub const SO_USELOOPBACK: c_int = 0x0040;
pub const SO_LINGER: c_int = 0x0080;
pub const SO_OOBINLINE: c_int = 0x0100;
pub const SO_DONTLINGER: c_int = !SO_LINGER;
pub const SO_EXCLUSIVEADDRUSE: c_int = !SO_REUSEADDR;
pub const SO_SNDBUF: c_int = 0x1001;
pub const SO_RCVBUF: c_int = 0x1002;
pub const SO_SNDLOWAT: c_int = 0x1003;
pub const SO_RCVLOWAT: c_int = 0x1004;
pub const SO_SNDTIMEO: c_int = 0x1005;
pub const SO_RCVTIMEO: c_int = 0x1006;
pub const SO_ERROR: c_int = 0x1007;
pub const SO_TYPE: c_int = 0x1008;
pub const SO_GROUP_ID: c_int = 0x2001;
pub const SO_GROUP_PRIORITY: c_int = 0x2002;
pub const SO_MAX_MSG_SIZE: c_int = 0x2003;
pub const SO_PROTOCOL_INFOA: c_int = 0x2004;
pub const SO_PROTOCOL_INFOW: c_int = 0x2005;
pub const PVD_CONFIG: c_int = 0x3001;
pub const SO_CONDITIONAL_ACCEPT: c_int = 0x3002;
STRUCT!{struct sockproto {
    sp_family: u_short,
    sp_protocol: u_short,
}}
pub const PF_UNSPEC: c_int = AF_UNSPEC;
pub const PF_UNIX: c_int = AF_UNIX;
pub const PF_INET: c_int = AF_INET;
pub const PF_IMPLINK: c_int = AF_IMPLINK;
pub const PF_PUP: c_int = AF_PUP;
pub const PF_CHAOS: c_int = AF_CHAOS;
pub const PF_NS: c_int = AF_NS;
pub const PF_IPX: c_int = AF_IPX;
pub const PF_ISO: c_int = AF_ISO;
pub const PF_OSI: c_int = AF_OSI;
pub const PF_ECMA: c_int = AF_ECMA;
pub const PF_DATAKIT: c_int = AF_DATAKIT;
pub const PF_CCITT: c_int = AF_CCITT;
pub const PF_SNA: c_int = AF_SNA;
pub const PF_DECnet: c_int = AF_DECnet;
pub const PF_DLI: c_int = AF_DLI;
pub const PF_LAT: c_int = AF_LAT;
pub const PF_HYLINK: c_int = AF_HYLINK;
pub const PF_APPLETALK: c_int = AF_APPLETALK;
pub const PF_VOICEVIEW: c_int = AF_VOICEVIEW;
pub const PF_FIREFOX: c_int = AF_FIREFOX;
pub const PF_UNKNOWN1: c_int = AF_UNKNOWN1;
pub const PF_BAN: c_int = AF_BAN;
pub const PF_ATM: c_int = AF_ATM;
pub const PF_INET6: c_int = AF_INET6;
pub const PF_BTH: c_int = AF_BTH;
pub const PF_MAX: c_int = AF_MAX;
STRUCT!{struct linger {
    l_onoff: u_short,
    l_linger: u_short,
}}
pub const SOL_SOCKET: c_int = 0xffff;
pub const SOMAXCONN: c_int = 0x7fffffff;
#[inline]
pub fn SOMAXCONN_HINT(b: c_int) -> c_int {
    -b
}
pub const MSG_OOB: c_int = 0x1;
pub const MSG_PEEK: c_int = 0x2;
pub const MSG_DONTROUTE: c_int = 0x4;
pub const MSG_WAITALL: c_int = 0x8;
pub const MSG_PUSH_IMMEDIATE: c_int = 0x20;
pub const MSG_PARTIAL: c_int = 0x8000;
pub const MSG_INTERRUPT: c_int = 0x10;
pub const MSG_MAXIOVLEN: c_int = 16;
pub const MAXGETHOSTSTRUCT: usize = 1024;
pub const FD_READ_BIT: c_long = 0;
pub const FD_READ: c_long = 1 << FD_READ_BIT;
pub const FD_WRITE_BIT: c_long = 1;
pub const FD_WRITE: c_long = 1 << FD_WRITE_BIT;
pub const FD_OOB_BIT: c_long = 2;
pub const FD_OOB: c_long = 1 << FD_OOB_BIT;
pub const FD_ACCEPT_BIT: c_long = 3;
pub const FD_ACCEPT: c_long = 1 << FD_ACCEPT_BIT;
pub const FD_CONNECT_BIT: c_long = 4;
pub const FD_CONNECT: c_long = 1 << FD_CONNECT_BIT;
pub const FD_CLOSE_BIT: c_long = 5;
pub const FD_CLOSE: c_long = 1 << FD_CLOSE_BIT;
pub const FD_QOS_BIT: c_long = 6;
pub const FD_QOS: c_long = 1 << FD_QOS_BIT;
pub const FD_GROUP_QOS_BIT: c_long = 7;
pub const FD_GROUP_QOS: c_long = 1 << FD_GROUP_QOS_BIT;
pub const FD_ROUTING_INTERFACE_CHANGE_BIT: c_long = 8;
pub const FD_ROUTING_INTERFACE_CHANGE: c_long = 1 << FD_ROUTING_INTERFACE_CHANGE_BIT;
pub const FD_ADDRESS_LIST_CHANGE_BIT: c_long = 9;
pub const FD_ADDRESS_LIST_CHANGE: c_long = 1 << FD_ADDRESS_LIST_CHANGE_BIT;
pub const FD_MAX_EVENTS: usize = 10;
pub const FD_ALL_EVENTS: c_long = (1 << FD_MAX_EVENTS) - 1;
pub const WSABASEERR: c_int = 10000;
pub const WSAEINTR: c_int = WSABASEERR+4;
pub const WSAEBADF: c_int = WSABASEERR+9;
pub const WSAEACCES: c_int = WSABASEERR+13;
pub const WSAEFAULT: c_int = WSABASEERR+14;
pub const WSAEINVAL: c_int = WSABASEERR+22;
pub const WSAEMFILE: c_int = WSABASEERR+24;
pub const WSAEWOULDBLOCK: c_int = WSABASEERR+35;
pub const WSAEINPROGRESS: c_int = WSABASEERR+36;
pub const WSAEALREADY: c_int = WSABASEERR+37;
pub const WSAENOTSOCK: c_int = WSABASEERR+38;
pub const WSAEDESTADDRREQ: c_int = WSABASEERR+39;
pub const WSAEMSGSIZE: c_int = WSABASEERR+40;
pub const WSAEPROTOTYPE: c_int = WSABASEERR+41;
pub const WSAENOPROTOOPT: c_int = WSABASEERR+42;
pub const WSAEPROTONOSUPPORT: c_int = WSABASEERR+43;
pub const WSAESOCKTNOSUPPORT: c_int = WSABASEERR+44;
pub const WSAEOPNOTSUPP: c_int = WSABASEERR+45;
pub const WSAEPFNOSUPPORT: c_int = WSABASEERR+46;
pub const WSAEAFNOSUPPORT: c_int = WSABASEERR+47;
pub const WSAEADDRINUSE: c_int = WSABASEERR+48;
pub const WSAEADDRNOTAVAIL: c_int = WSABASEERR+49;
pub const WSAENETDOWN: c_int = WSABASEERR+50;
pub const WSAENETUNREACH: c_int = WSABASEERR+51;
pub const WSAENETRESET: c_int = WSABASEERR+52;
pub const WSAECONNABORTED: c_int = WSABASEERR+53;
pub const WSAECONNRESET: c_int = WSABASEERR+54;
pub const WSAENOBUFS: c_int = WSABASEERR+55;
pub const WSAEISCONN: c_int = WSABASEERR+56;
pub const WSAENOTCONN: c_int = WSABASEERR+57;
pub const WSAESHUTDOWN: c_int = WSABASEERR+58;
pub const WSAETOOMANYREFS: c_int = WSABASEERR+59;
pub const WSAETIMEDOUT: c_int = WSABASEERR+60;
pub const WSAECONNREFUSED: c_int = WSABASEERR+61;
pub const WSAELOOP: c_int = WSABASEERR+62;
pub const WSAENAMETOOLONG: c_int = WSABASEERR+63;
pub const WSAEHOSTDOWN: c_int = WSABASEERR+64;
pub const WSAEHOSTUNREACH: c_int = WSABASEERR+65;
pub const WSAENOTEMPTY: c_int = WSABASEERR+66;
pub const WSAEPROCLIM: c_int = WSABASEERR+67;
pub const WSAEUSERS: c_int = WSABASEERR+68;
pub const WSAEDQUOT: c_int = WSABASEERR+69;
pub const WSAESTALE: c_int = WSABASEERR+70;
pub const WSAEREMOTE: c_int = WSABASEERR+71;
pub const WSASYSNOTREADY: c_int = WSABASEERR+91;
pub const WSAVERNOTSUPPORTED: c_int = WSABASEERR+92;
pub const WSANOTINITIALISED: c_int = WSABASEERR+93;
pub const WSAEDISCON: c_int = WSABASEERR+101;
pub const WSAENOMORE: c_int = WSABASEERR+102;
pub const WSAECANCELLED: c_int = WSABASEERR+103;
pub const WSAEINVALIDPROCTABLE: c_int = WSABASEERR+104;
pub const WSAEINVALIDPROVIDER: c_int = WSABASEERR+105;
pub const WSAEPROVIDERFAILEDINIT: c_int = WSABASEERR+106;
pub const WSASYSCALLFAILURE: c_int = WSABASEERR+107;
pub const WSASERVICE_NOT_FOUND: c_int = WSABASEERR+108;
pub const WSATYPE_NOT_FOUND: c_int = WSABASEERR+109;
pub const WSA_E_NO_MORE: c_int = WSABASEERR+110;
pub const WSA_E_CANCELLED: c_int = WSABASEERR+111;
pub const WSAEREFUSED: c_int = WSABASEERR+112;
pub const WSAHOST_NOT_FOUND: c_int = WSABASEERR+1001;
pub const WSATRY_AGAIN: c_int = WSABASEERR+1002;
pub const WSANO_RECOVERY: c_int = WSABASEERR+1003;
pub const WSANO_DATA: c_int = WSABASEERR+1004;
pub const WSA_QOS_RECEIVERS: c_int = WSABASEERR + 1005;
pub const WSA_QOS_SENDERS: c_int = WSABASEERR + 1006;
pub const WSA_QOS_NO_SENDERS: c_int = WSABASEERR + 1007;
pub const WSA_QOS_NO_RECEIVERS: c_int = WSABASEERR + 1008;
pub const WSA_QOS_REQUEST_CONFIRMED: c_int = WSABASEERR + 1009;
pub const WSA_QOS_ADMISSION_FAILURE: c_int = WSABASEERR + 1010;
pub const WSA_QOS_POLICY_FAILURE: c_int = WSABASEERR + 1011;
pub const WSA_QOS_BAD_STYLE: c_int = WSABASEERR + 1012;
pub const WSA_QOS_BAD_OBJECT: c_int = WSABASEERR + 1013;
pub const WSA_QOS_TRAFFIC_CTRL_ERROR: c_int = WSABASEERR + 1014;
pub const WSA_QOS_GENERIC_ERROR: c_int = WSABASEERR + 1015;
pub const WSA_QOS_ESERVICETYPE: c_int = WSABASEERR + 1016;
pub const WSA_QOS_EFLOWSPEC: c_int = WSABASEERR + 1017;
pub const WSA_QOS_EPROVSPECBUF: c_int = WSABASEERR + 1018;
pub const WSA_QOS_EFILTERSTYLE: c_int = WSABASEERR + 1019;
pub const WSA_QOS_EFILTERTYPE: c_int = WSABASEERR + 1020;
pub const WSA_QOS_EFILTERCOUNT: c_int = WSABASEERR + 1021;
pub const WSA_QOS_EOBJLENGTH: c_int = WSABASEERR + 1022;
pub const WSA_QOS_EFLOWCOUNT: c_int = WSABASEERR + 1023;
pub const WSA_QOS_EUNKOWNPSOBJ: c_int = WSABASEERR + 1024;
pub const WSA_QOS_EPOLICYOBJ: c_int = WSABASEERR + 1025;
pub const WSA_QOS_EFLOWDESC: c_int = WSABASEERR + 1026;
pub const WSA_QOS_EPSFLOWSPEC: c_int = WSABASEERR + 1027;
pub const WSA_QOS_EPSFILTERSPEC: c_int = WSABASEERR + 1028;
pub const WSA_QOS_ESDMODEOBJ: c_int = WSABASEERR + 1029;
pub const WSA_QOS_ESHAPERATEOBJ: c_int = WSABASEERR + 1030;
pub const WSA_QOS_RESERVED_PETYPE: c_int = WSABASEERR + 1031;
#[inline]
pub unsafe fn h_errno() -> c_int {
    WSAGetLastError()
}
pub const HOST_NOT_FOUND: c_int = WSAHOST_NOT_FOUND;
pub const TRY_AGAIN: c_int = WSATRY_AGAIN;
pub const NO_RECOVERY: c_int = WSANO_RECOVERY;
pub const NO_DATA: c_int = WSANO_DATA;
pub const WSANO_ADDRESS: c_int = WSANO_DATA;
pub const NO_ADDRESS: c_int = WSANO_ADDRESS;
pub type WSAEVENT = HANDLE;
pub type LPWSAEVENT = LPHANDLE;
pub type WSAOVERLAPPED = OVERLAPPED;
pub type LPWSAOVERLAPPED = *mut OVERLAPPED;
pub const WSA_IO_PENDING: c_int = ERROR_IO_PENDING as i32;
pub const WSA_IO_INCOMPLETE: c_int = ERROR_IO_INCOMPLETE as i32;
pub const WSA_INVALID_HANDLE: c_int = ERROR_INVALID_HANDLE as i32;
pub const WSA_INVALID_PARAMETER: c_int = ERROR_INVALID_PARAMETER as i32;
pub const WSA_NOT_ENOUGH_MEMORY: c_int = ERROR_NOT_ENOUGH_MEMORY as i32;
pub const WSA_OPERATION_ABORTED: c_int = ERROR_OPERATION_ABORTED as i32;
pub const WSA_INVALID_EVENT: WSAEVENT = 0 as WSAEVENT;
pub const WSA_MAXIMUM_WAIT_EVENTS: DWORD = MAXIMUM_WAIT_OBJECTS;
pub const WSA_WAIT_FAILED: DWORD = WAIT_FAILED;
pub const WSA_WAIT_EVENT_0: DWORD = WAIT_OBJECT_0;
pub const WSA_WAIT_IO_COMPLETION: DWORD = WAIT_IO_COMPLETION;
pub const WSA_WAIT_TIMEOUT: DWORD = WAIT_TIMEOUT;
pub const WSA_INFINITE: DWORD = INFINITE;
STRUCT!{struct QOS {
    SendingFlowspec: FLOWSPEC,
    FLOWSPEC: FLOWSPEC,
    ProviderSpecific: WSABUF,
}}
pub type LPQOS = *mut QOS;
pub const CF_ACCEPT: c_int = 0x0000;
pub const CF_REJECT: c_int = 0x0001;
pub const CF_DEFER: c_int = 0x0002;
pub const SD_RECEIVE: c_int = 0x00;
pub const SD_SEND: c_int = 0x01;
pub const SD_BOTH: c_int = 0x02;
pub type GROUP = c_uint;
pub const SG_UNCONSTRAINED_GROUP: GROUP = 0x01;
pub const SG_CONSTRAINED_GROUP: GROUP = 0x02;
STRUCT!{struct WSANETWORKEVENTS {
    lNetworkEvents: c_long,
    iErrorCode: [c_int; FD_MAX_EVENTS],
}}
pub type LPWSANETWORKEVENTS = *mut WSANETWORKEVENTS;
pub const MAX_PROTOCOL_CHAIN: usize = 7;
pub const BASE_PROTOCOL: c_int = 1;
pub const LAYERED_PROTOCOL: c_int = 0;
STRUCT!{struct WSAPROTOCOLCHAIN {
    ChainLen: c_int,
    ChainEntries: [DWORD; MAX_PROTOCOL_CHAIN],
}}
pub type LPWSAPROTOCOLCHAIN = *mut WSAPROTOCOLCHAIN;
pub const WSAPROTOCOL_LEN: usize = 255;
STRUCT!{struct WSAPROTOCOL_INFOA {
    dwServiceFlags1: DWORD,
    dwServiceFlags2: DWORD,
    dwServiceFlags3: DWORD,
    dwServiceFlags4: DWORD,
    dwServiceFlags5: DWORD,
    ProviderId: GUID,
    dwCatalogEntryId: DWORD,
    ProtocolChain: WSAPROTOCOLCHAIN,
    iVersion: c_int,
    iAddressFamily: c_int,
    iMaxSockAddr: c_int,
    iMinSockAddr: c_int,
    iSocketType: c_int,
    iProtocol: c_int,
    iProtocolMaxOffset: c_int,
    iNetworkByteOrder: c_int,
    iSecurityScheme: c_int,
    dwMessageSize: DWORD,
    dwProviderReserved: DWORD,
    szProtocol: [CHAR; WSAPROTOCOL_LEN + 1],
}}
pub type LPWSAPROTOCOL_INFOA = *mut WSAPROTOCOL_INFOA;
STRUCT!{struct WSAPROTOCOL_INFOW {
    dwServiceFlags1: DWORD,
    dwServiceFlags2: DWORD,
    dwServiceFlags3: DWORD,
    dwServiceFlags4: DWORD,
    dwServiceFlags5: DWORD,
    ProviderId: GUID,
    dwCatalogEntryId: DWORD,
    ProtocolChain: WSAPROTOCOLCHAIN,
    iVersion: c_int,
    iAddressFamily: c_int,
    iMaxSockAddr: c_int,
    iMinSockAddr: c_int,
    iSocketType: c_int,
    iProtocol: c_int,
    iProtocolMaxOffset: c_int,
    iNetworkByteOrder: c_int,
    iSecurityScheme: c_int,
    dwMessageSize: DWORD,
    dwProviderReserved: DWORD,
    szProtocol: [WCHAR; WSAPROTOCOL_LEN + 1],
}}
pub type LPWSAPROTOCOL_INFOW = *mut WSAPROTOCOL_INFOW;
pub const PFL_MULTIPLE_PROTO_ENTRIES: DWORD = 0x00000001;
pub const PFL_RECOMMENDED_PROTO_ENTRY: DWORD = 0x00000002;
pub const PFL_HIDDEN: DWORD = 0x00000004;
pub const PFL_MATCHES_PROTOCOL_ZERO: DWORD = 0x00000008;
pub const PFL_NETWORKDIRECT_PROVIDER: DWORD = 0x00000010;
pub const XP1_CONNECTIONLESS: DWORD = 0x00000001;
pub const XP1_GUARANTEED_DELIVERY: DWORD = 0x00000002;
pub const XP1_GUARANTEED_ORDER: DWORD = 0x00000004;
pub const XP1_MESSAGE_ORIENTED: DWORD = 0x00000008;
pub const XP1_PSEUDO_STREAM: DWORD = 0x00000010;
pub const XP1_GRACEFUL_CLOSE: DWORD = 0x00000020;
pub const XP1_EXPEDITED_DATA: DWORD = 0x00000040;
pub const XP1_CONNECT_DATA: DWORD = 0x00000080;
pub const XP1_DISCONNECT_DATA: DWORD = 0x00000100;
pub const XP1_SUPPORT_BROADCAST: DWORD = 0x00000200;
pub const XP1_SUPPORT_MULTIPOINT: DWORD = 0x00000400;
pub const XP1_MULTIPOINT_CONTROL_PLANE: DWORD = 0x00000800;
pub const XP1_MULTIPOINT_DATA_PLANE: DWORD = 0x00001000;
pub const XP1_QOS_SUPPORTED: DWORD = 0x00002000;
pub const XP1_INTERRUPT: DWORD = 0x00004000;
pub const XP1_UNI_SEND: DWORD = 0x00008000;
pub const XP1_UNI_RECV: DWORD = 0x00010000;
pub const XP1_IFS_HANDLES: DWORD = 0x00020000;
pub const XP1_PARTIAL_MESSAGE: DWORD = 0x00040000;
pub const XP1_SAN_SUPPORT_SDP: DWORD = 0x00080000;
pub const BIGENDIAN: DWORD = 0x0000;
pub const LITTLEENDIAN: DWORD = 0x0001;
pub const SECURITY_PROTOCOL_NONE: DWORD = 0x0000;
pub const JL_SENDER_ONLY: DWORD = 0x01;
pub const JL_RECEIVER_ONLY: DWORD = 0x02;
pub const JL_BOTH: DWORD = 0x04;
pub const WSA_FLAG_OVERLAPPED: DWORD = 0x01;
pub const WSA_FLAG_MULTIPOINT_C_ROOT: DWORD = 0x02;
pub const WSA_FLAG_MULTIPOINT_C_LEAF: DWORD = 0x04;
pub const WSA_FLAG_MULTIPOINT_D_ROOT: DWORD = 0x08;
pub const WSA_FLAG_MULTIPOINT_D_LEAF: DWORD = 0x10;
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: DWORD = 0x40;
pub const WSA_FLAG_NO_HANDLE_INHERIT: DWORD = 0x80;
pub const WSA_FLAG_REGISTERED_IO: DWORD = 0x100;
FN!{stdcall LPCONDITIONPROC(
    lpCallerId: LPWSABUF,
    lpCallerData: LPWSABUF,
    lpSQOS: LPQOS,
    lpGQOS: LPQOS,
    lpCalleeId: LPWSABUF,
    lpCalleeData: LPWSABUF,
    g: *mut GROUP,
    dwCallbackData: DWORD,
) -> c_int}
FN!{stdcall LPWSAOVERLAPPED_COMPLETION_ROUTINE(
    dwError: DWORD,
    cbTransferred: DWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    dwFlags: DWORD,
) -> ()}
ENUM!{enum WSACOMPLETIONTYPE {
    NSP_NOTIFY_IMMEDIATELY = 0,
    NSP_NOTIFY_HWND,
    NSP_NOTIFY_EVENT,
    NSP_NOTIFY_PORT,
    NSP_NOTIFY_APC,
}}
pub type PWSACOMPLETIONTYPE = *mut WSACOMPLETIONTYPE;
pub type LPWSACOMPLETIONTYPE = *mut WSACOMPLETIONTYPE;
STRUCT!{struct WSACOMPLETION_WindowMessage {
    hWnd: HWND,
    uMsg: UINT,
    context: WPARAM,
}}
STRUCT!{struct WSACOMPLETION_Event {
    lpOverlapped: LPWSAOVERLAPPED,
}}
STRUCT!{struct WSACOMPLETION_Apc {
    lpOverlapped: LPWSAOVERLAPPED,
    lpfnCompletionProc: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}}
STRUCT!{struct WSACOMPLETION_Port {
    lpOverlapped: LPWSAOVERLAPPED,
    hPort: HANDLE,
    Key: ULONG_PTR,
}}
UNION!{union WSACOMPLETION_Parameter {
    [usize; 3],
    WindowMessage WindowMessage_mut: WSACOMPLETION_WindowMessage,
    Event Event_mut: WSACOMPLETION_Event,
    Apc Apc_mut: WSACOMPLETION_Apc,
    Port Port_mut: WSACOMPLETION_Port,
}}
STRUCT!{struct WSACOMPLETION {
    Type: WSACOMPLETIONTYPE,
    Parameters: WSACOMPLETION_Parameter,
}}
pub type PWSACOMPLETION = *mut WSACOMPLETION;
pub type LPWSACOMPLETION = *mut WSACOMPLETION;
pub const TH_NETDEV: DWORD = 0x00000001;
pub const TH_TAPI: DWORD = 0x00000002;
pub const SERVICE_MULTIPLE: DWORD = 0x00000001;
pub const NS_ALL: DWORD = 0;
pub const NS_SAP: DWORD = 1;
pub const NS_NDS: DWORD = 2;
pub const NS_PEER_BROWSE: DWORD = 3;
pub const NS_SLP: DWORD = 5;
pub const NS_DHCP: DWORD = 6;
pub const NS_TCPIP_LOCAL: DWORD = 10;
pub const NS_TCPIP_HOSTS: DWORD = 11;
pub const NS_DNS: DWORD = 12;
pub const NS_NETBT: DWORD = 13;
pub const NS_WINS: DWORD = 14;
pub const NS_NLA: DWORD = 15;
pub const NS_BTH: DWORD = 16;
pub const NS_LOCALNAME: DWORD = 19;
pub const NS_NBP: DWORD = 20;
pub const NS_MS: DWORD = 30;
pub const NS_STDA: DWORD = 31;
pub const NS_NTDS: DWORD = 32;
pub const NS_EMAIL: DWORD = 37;
pub const NS_PNRPNAME: DWORD = 38;
pub const NS_PNRPCLOUD: DWORD = 39;
pub const NS_X500: DWORD = 40;
pub const NS_NIS: DWORD = 41;
pub const NS_NISPLUS: DWORD = 42;
pub const NS_WRQ: DWORD = 50;
pub const NS_NETDES: DWORD = 60;
pub const RES_UNUSED_1: DWORD = 0x00000001;
pub const RES_FLUSH_CACHE: DWORD = 0x00000002;
pub const RES_SERVICE: DWORD = 0x00000004;
pub const SERVICE_TYPE_VALUE_IPXPORT: &'static str = "IpxSocket";
pub const SERVICE_TYPE_VALUE_SAPID: &'static str = "SapId";
pub const SERVICE_TYPE_VALUE_TCPPORT: &'static str = "TcpPort";
pub const SERVICE_TYPE_VALUE_UDPPORT: &'static str = "UdpPort";
pub const SERVICE_TYPE_VALUE_OBJECTID: &'static str = "ObjectId";
STRUCT!{struct AFPROTOCOLS {
    iAddressFamily: INT,
    iProtocol: INT,
}}
pub type PAFPROTOCOLS = *mut AFPROTOCOLS;
pub type LPAFPROTOCOLS = *mut AFPROTOCOLS;
ENUM!{enum WSAECOMPARATOR {
    COMP_EQUAL = 0,
    COMP_NOTLESS,
}}
pub type PWSAECOMPARATOR = *mut WSAECOMPARATOR;
pub type LPWSAECOMPARATOR = *mut WSAECOMPARATOR;
STRUCT!{struct WSAVERSION {
    dwVersion: DWORD,
    ecHow: WSAECOMPARATOR,
}}
pub type PWSAVERSION = *mut WSAVERSION;
pub type LPWSAVERSION = *mut WSAVERSION;
STRUCT!{struct WSAQUERYSETA {
    dwSize: DWORD,
    lpszServiceInstanceName: LPSTR,
    lpServiceClassId: LPGUID,
    lpVersion: LPWSAVERSION,
    lpszComment: LPSTR,
    dwNameSpace: DWORD,
    lpNSProviderId: LPGUID,
    lpszContext: LPSTR,
    dwNumberOfProtocols: DWORD,
    lpafpProtocols: LPAFPROTOCOLS,
    lpszQueryString: LPSTR,
    dwNumberOfCsAddrs: DWORD,
    lpcsaBuffer: LPCSADDR_INFO,
    dwOutputFlags: DWORD,
    lpBlob: LPBLOB,
}}
pub type PWSAQUERYSETA = *mut WSAQUERYSETA;
pub type LPWSAQUERYSETA = *mut WSAQUERYSETA;
STRUCT!{struct WSAQUERYSETW {
    dwSize: DWORD,
    lpszServiceInstanceName: LPWSTR,
    lpServiceClassId: LPGUID,
    lpVersion: LPWSAVERSION,
    lpszComment: LPWSTR,
    dwNameSpace: DWORD,
    lpNSProviderId: LPGUID,
    lpszContext: LPWSTR,
    dwNumberOfProtocols: DWORD,
    lpafpProtocols: LPAFPROTOCOLS,
    lpszQueryString: LPWSTR,
    dwNumberOfCsAddrs: DWORD,
    lpcsaBuffer: LPCSADDR_INFO,
    dwOutputFlags: DWORD,
    lpBlob: LPBLOB,
}}
pub type PWSAQUERYSETW = *mut WSAQUERYSETW;
pub type LPWSAQUERYSETW = *mut WSAQUERYSETW;
STRUCT!{struct WSAQUERYSET2A {
    dwSize: DWORD,
    lpszServiceInstanceName: LPSTR,
    lpVersion: LPWSAVERSION,
    lpszComment: LPSTR,
    dwNameSpace: DWORD,
    lpNSProviderId: LPGUID,
    lpszContext: LPSTR,
    dwNumberOfProtocols: DWORD,
    lpafpProtocols: LPAFPROTOCOLS,
    lpszQueryString: LPSTR,
    dwNumberOfCsAddrs: DWORD,
    lpcsaBuffer: LPCSADDR_INFO,
    dwOutputFlags: DWORD,
    lpBlob: LPBLOB,
}}
pub type PWSAQUERYSET2A = *mut WSAQUERYSET2A;
pub type LPWSAQUERYSET2A = *mut WSAQUERYSET2A;
STRUCT!{struct WSAQUERYSET2W {
    dwSize: DWORD,
    lpszServiceInstanceName: LPWSTR,
    lpVersion: LPWSAVERSION,
    lpszComment: LPWSTR,
    dwNameSpace: DWORD,
    lpNSProviderId: LPGUID,
    lpszContext: LPWSTR,
    dwNumberOfProtocols: DWORD,
    lpafpProtocols: LPAFPROTOCOLS,
    lpszQueryString: LPWSTR,
    dwNumberOfCsAddrs: DWORD,
    lpcsaBuffer: LPCSADDR_INFO,
    dwOutputFlags: DWORD,
    lpBlob: LPBLOB,
}}
pub type PWSAQUERYSET2W = *mut WSAQUERYSET2W;
pub type LPWSAQUERYSET2W = *mut WSAQUERYSET2W;
pub const LUP_DEEP: DWORD = 0x0001;
pub const LUP_CONTAINERS: DWORD = 0x0002;
pub const LUP_NOCONTAINERS: DWORD = 0x0004;
pub const LUP_NEAREST: DWORD = 0x0008;
pub const LUP_RETURN_NAME: DWORD = 0x0010;
pub const LUP_RETURN_TYPE: DWORD = 0x0020;
pub const LUP_RETURN_VERSION: DWORD = 0x0040;
pub const LUP_RETURN_COMMENT: DWORD = 0x0080;
pub const LUP_RETURN_ADDR: DWORD = 0x0100;
pub const LUP_RETURN_BLOB: DWORD = 0x0200;
pub const LUP_RETURN_ALIASES: DWORD = 0x0400;
pub const LUP_RETURN_QUERY_STRING: DWORD = 0x0800;
pub const LUP_RETURN_ALL: DWORD = 0x0FF0;
pub const LUP_RES_SERVICE: DWORD = 0x8000;
pub const LUP_FLUSHCACHE: DWORD = 0x1000;
pub const LUP_FLUSHPREVIOUS: DWORD = 0x2000;
pub const LUP_NON_AUTHORITATIVE: DWORD = 0x4000;
pub const LUP_SECURE: DWORD = 0x8000;
pub const LUP_RETURN_PREFERRED_NAMES: DWORD = 0x10000;
pub const LUP_DNS_ONLY: DWORD = 0x20000;
pub const LUP_ADDRCONFIG: DWORD = 0x00100000;
pub const LUP_DUAL_ADDR: DWORD = 0x00200000;
pub const LUP_FILESERVER: DWORD = 0x00400000;
pub const LUP_DISABLE_IDN_ENCODING: DWORD = 0x00800000;
pub const LUP_API_ANSI: DWORD = 0x01000000;
pub const LUP_RESOLUTION_HANDLE: DWORD = 0x80000000;
pub const RESULT_IS_ALIAS: DWORD = 0x0001;
pub const RESULT_IS_ADDED: DWORD = 0x0010;
pub const RESULT_IS_CHANGED: DWORD = 0x0020;
pub const RESULT_IS_DELETED: DWORD = 0x0040;
ENUM!{enum WSAESETSERVICEOP {
    RNRSERVICE_REGISTER = 0,
    RNRSERVICE_DEREGISTER,
    RNRSERVICE_DELETE,
}}
pub type PWSAESETSERVICEOP = *mut WSAESETSERVICEOP;
pub type LPWSAESETSERVICEOP = *mut WSAESETSERVICEOP;
STRUCT!{struct WSANSCLASSINFOA {
    lpszName: LPSTR,
    dwNameSpace: DWORD,
    dwValueType: DWORD,
    dwValueSize: DWORD,
    lpValue: LPVOID,
}}
pub type PWSANSCLASSINFOA = *mut WSANSCLASSINFOA;
pub type LPWSANSCLASSINFOA = *mut WSANSCLASSINFOA;
STRUCT!{struct WSANSCLASSINFOW {
    lpszName: LPWSTR,
    dwNameSpace: DWORD,
    dwValueType: DWORD,
    dwValueSize: DWORD,
    lpValue: LPVOID,
}}
pub type PWSANSCLASSINFOW = *mut WSANSCLASSINFOW;
pub type LPWSANSCLASSINFOW = *mut WSANSCLASSINFOW;
STRUCT!{struct WSASERVICECLASSINFOA {
    lpServiceClassId: LPGUID,
    lpszServiceClassName: LPSTR,
    dwCount: DWORD,
    lpClassInfos: LPWSANSCLASSINFOA,
}}
pub type PWSASERVICECLASSINFOA = *mut WSASERVICECLASSINFOA;
pub type LPWSASERVICECLASSINFOA = *mut WSASERVICECLASSINFOA;
STRUCT!{struct WSASERVICECLASSINFOW {
    lpServiceClassId: LPGUID,
    lpszServiceClassName: LPWSTR,
    dwCount: DWORD,
    lpClassInfos: LPWSANSCLASSINFOW,
}}
pub type PWSASERVICECLASSINFOW = *mut WSASERVICECLASSINFOW;
pub type LPWSASERVICECLASSINFOW = *mut WSASERVICECLASSINFOW;
STRUCT!{struct WSANAMESPACE_INFOA {
    NSProviderId: GUID,
    dwNameSpace: DWORD,
    fActive: BOOL,
    dwVersion: DWORD,
    lpszIdentifier: LPSTR,
}}
pub type PWSANAMESPACE_INFOA = *mut WSANAMESPACE_INFOA;
pub type LPWSANAMESPACE_INFOA = *mut WSANAMESPACE_INFOA;
STRUCT!{struct WSANAMESPACE_INFOW {
    NSProviderId: GUID,
    dwNameSpace: DWORD,
    fActive: BOOL,
    dwVersion: DWORD,
    lpszIdentifier: LPWSTR,
}}
pub type PWSANAMESPACE_INFOW = *mut WSANAMESPACE_INFOW;
pub type LPWSANAMESPACE_INFOW = *mut WSANAMESPACE_INFOW;
STRUCT!{struct WSANAMESPACE_INFOEXA {
    NSProviderId: GUID,
    dwNameSpace: DWORD,
    fActive: BOOL,
    dwVersion: DWORD,
    lpszIdentifier: LPSTR,
    ProviderSpecific: BLOB,
}}
pub type PWSANAMESPACE_INFOEXA = *mut WSANAMESPACE_INFOEXA;
pub type LPWSANAMESPACE_INFOEXA = *mut WSANAMESPACE_INFOEXA;
STRUCT!{struct WSANAMESPACE_INFOEXW {
    NSProviderId: GUID,
    dwNameSpace: DWORD,
    fActive: BOOL,
    dwVersion: DWORD,
    lpszIdentifier: LPWSTR,
    ProviderSpecific: BLOB,
}}
pub type PWSANAMESPACE_INFOEXW = *mut WSANAMESPACE_INFOEXW;
pub type LPWSANAMESPACE_INFOEXW = *mut WSANAMESPACE_INFOEXW;
pub const POLLRDNORM: SHORT = 0x0100;
pub const POLLRDBAND: SHORT = 0x0200;
pub const POLLIN: SHORT = POLLRDNORM | POLLRDBAND;
pub const POLLPRI: SHORT = 0x0400;
pub const POLLWRNORM: SHORT = 0x0010;
pub const POLLOUT: SHORT = POLLWRNORM;
pub const POLLWRBAND: SHORT = 0x0020;
pub const POLLERR: SHORT = 0x0001;
pub const POLLHUP: SHORT = 0x0002;
pub const POLLNVAL: SHORT = 0x0004;
STRUCT!{struct WSAPOLLFD {
    fd: SOCKET,
    events: SHORT,
    revents: SHORT,
}}
pub type PWSAPOLLFD = *mut WSAPOLLFD;
pub type LPWSAPOLLFD = *mut WSAPOLLFD;
extern "system" {
    pub fn accept(
        s: SOCKET,
        addr: *mut SOCKADDR,
        addrlen: *mut c_int,
    ) -> SOCKET;
    pub fn bind(s: SOCKET,
        name: *const SOCKADDR,
        namelen: c_int,
    ) -> c_int;
    pub fn closesocket(
        s: SOCKET,
    ) -> c_int;
    pub fn connect(
        s: SOCKET,
        name: *const SOCKADDR,
        namelen: c_int,
    ) -> c_int;
    pub fn ioctlsocket(
        s: SOCKET,
        cmd: c_long,
        argp: *mut u_long,
    ) -> c_int;
    pub fn getpeername(
        s: SOCKET,
        name: *mut SOCKADDR,
        namelen: *mut c_int,
    ) -> c_int;
    pub fn getsockname(
        s: SOCKET,
        name: *mut SOCKADDR,
        namelen: *mut c_int,
    ) -> c_int;
    pub fn getsockopt(
        s: SOCKET,
        level: c_int,
        optname: c_int,
        optval: *mut c_char,
        optlen: *mut c_int,
    ) -> c_int;
    pub fn htonl(
        hostlong: u_long,
    ) -> u_long;
    pub fn htons(
        hostshort: u_short,
    ) -> u_short;
    pub fn inet_addr(
        cp: *const c_char,
    ) -> c_ulong;
    pub fn inet_ntoa(
        _in: in_addr,
    ) -> *mut c_char;
}
#[inline]
pub fn _WS2_32_WINSOCK_SWAP_LONG(l: __uint32) -> __uint32 {
    ((l >> 24) & 0x000000FF) | ((l >> 8) & 0x0000FF00) | ((l << 8) & 0x00FF0000)
    | ((l << 24) & 0xFF000000)
}
#[inline]
pub fn _WS2_32_WINSOCK_SWAP_LONGLONG(l: __uint64) -> __uint64 {
    ((l >> 56) & 0x00000000000000FF) | ((l >> 40) & 0x000000000000FF00)
    | ((l >> 24) & 0x0000000000FF0000) | ((l >> 8) & 0x00000000FF000000)
    | ((l << 8) & 0x000000FF00000000) | ((l << 24) & 0x0000FF0000000000)
    | ((l << 40) & 0x00FF000000000000) | ((l << 56) & 0xFF00000000000000)
}
#[inline]
pub fn htonll(Value: __uint64) -> __uint64 {
    _WS2_32_WINSOCK_SWAP_LONGLONG(Value)
}
#[inline]
pub fn ntohll(Value: __uint64) -> __uint64 {
    _WS2_32_WINSOCK_SWAP_LONGLONG(Value)
}
#[inline]
pub fn htonf(Value: c_float) -> __uint32 {
    let Tempval: __uint32 = unsafe { ::core::mem::transmute(Value) };
    _WS2_32_WINSOCK_SWAP_LONG(Tempval)
}
#[inline]
pub fn ntohf(Value: __uint32) -> c_float {
    let Tempval = _WS2_32_WINSOCK_SWAP_LONG(Value);
    unsafe { ::core::mem::transmute(Tempval) }
}
#[inline]
pub fn htond(Value: c_double) -> __uint64 {
    let Tempval: __uint64 = unsafe { ::core::mem::transmute(Value) };
    _WS2_32_WINSOCK_SWAP_LONGLONG(Tempval)
}
#[inline]
pub fn ntohd(Value: __uint64) -> c_double {
    let Tempval = _WS2_32_WINSOCK_SWAP_LONGLONG(Value);
    unsafe { ::core::mem::transmute(Tempval) }
}
extern "system" {
    pub fn listen(
        s: SOCKET,
        backlog: c_int,
    ) -> c_int;
    pub fn ntohl(
        netlong: u_long,
    ) -> u_long;
    pub fn ntohs(
        netshort: u_short,
    ) -> u_short;
    pub fn recv(
        s: SOCKET,
        buf: *mut c_char,
        len: c_int,
        flags: c_int,
    ) -> c_int;
    pub fn recvfrom(
        s: SOCKET,
        buf: *mut c_char,
        len: c_int,
        flags: c_int,
        from: *mut SOCKADDR,
        fromlen: *mut c_int,
    ) -> c_int;
    pub fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *const timeval,
    ) -> c_int;
    pub fn send(
        s: SOCKET,
        buf: *const c_char,
        len: c_int,
        flags: c_int,
    ) -> c_int;
    pub fn sendto(
        s: SOCKET,
        buf: *const c_char,
        len: c_int,
        flags: c_int,
        to: *const SOCKADDR,
        tolen: c_int,
    ) -> c_int;
    pub fn setsockopt(
        s: SOCKET,
        level: c_int,
        optname: c_int,
        optval: *const c_char,
        optlen: c_int,
    ) -> c_int;
    pub fn shutdown(
        s: SOCKET,
        how: c_int,
    ) -> c_int;
    pub fn socket(
        af: c_int,
        _type: c_int,
        protocol: c_int,
    ) -> SOCKET;
    pub fn gethostbyaddr(
        addr: *const c_char,
        len: c_int,
        _type: c_int,
    ) -> *mut hostent;
    pub fn gethostbyname(
        name: *const c_char,
    ) -> *mut hostent;
    pub fn gethostname(
        name: *mut c_char,
        namelen: c_int,
    ) -> c_int;
    pub fn GetHostNameW(
        name: PWSTR,
        namelen: c_int,
    ) -> c_int;
    pub fn getservbyport(
        port: c_int,
        proto: *const c_char,
    ) -> *mut servent;
    pub fn getservbyname(
        name: *const c_char,
        proto: *const c_char,
    ) -> *mut servent;
    pub fn getprotobynumber(
        number: c_int,
    ) -> *mut protoent;
    pub fn getprotobyname(
        name: *const c_char,
    ) -> *mut protoent;
    pub fn WSAStartup(
        wVersionRequested: WORD,
        lpWSAData: LPWSADATA,
    ) -> c_int;
    pub fn WSACleanup() -> c_int;
    pub fn WSASetLastError(
        iError: c_int,
    );
    pub fn WSAGetLastError() -> c_int;
    pub fn WSAIsBlocking() -> BOOL;
    pub fn WSAUnhookBlockingHook() -> c_int;
    pub fn WSASetBlockingHook(
        lpBlockFunc: FARPROC,
    ) -> FARPROC;
    pub fn WSACancelBlockingCall() -> c_int;
    pub fn WSAAsyncGetServByName(
        hWnd: HWND,
        wMsg: u_int,
        name: *const c_char,
        proto: *const c_char,
        buf: *mut c_char,
        buflen: c_int,
    ) -> HANDLE;
    pub fn WSAAsyncGetServByPort(
        hWnd: HWND,
        wMsg: u_int,
        port: c_int,
        proto: *const c_char,
        buf: *mut c_char,
        buflen: c_int,
    ) -> HANDLE;
    pub fn WSAAsyncGetProtoByName(
        hWnd: HWND,
        wMsg: u_int,
        name: *const c_char,
        buf: *mut c_char,
        buflen: c_int,
    ) -> HANDLE;
    pub fn WSAAsyncGetProtoByNumber(
        hWnd: HWND,
        wMsg: u_int,
        number: c_int,
        buf: *mut c_char,
        buflen: c_int,
    ) -> HANDLE;
    pub fn WSAAsyncGetHostByName(
        hWnd: HWND,
        wMsg: u_int,
        name: *const c_char,
        buf: *mut c_char,
        buflen: c_int,
    ) -> HANDLE;
    pub fn WSAAsyncGetHostByAddr(
        hWnd: HWND,
        wMsg: u_int,
        addr: *const c_char,
        len: c_int,
        _type: c_int,
        buf: *mut c_char,
        buflen: c_int,
    ) -> HANDLE;
    pub fn WSACancelAsyncRequest(
        hAsyncTaskHandle: HANDLE,
    ) -> c_int;
    pub fn WSAAsyncSelect(
        s: SOCKET,
        hWnd: HWND,
        wMsg: u_int,
        lEvent: c_long,
    ) -> c_int;
    pub fn WSAAccept(
        s: SOCKET,
        addr: *mut SOCKADDR,
        addrlen: LPINT,
        lpfnCondition: LPCONDITIONPROC,
        dwCallbackData: DWORD_PTR,
    ) -> SOCKET;
    pub fn WSACloseEvent(
        hEvent: WSAEVENT,
    ) -> BOOL;
    pub fn WSAConnect(
        s: SOCKET,
        name: *const SOCKADDR,
        namelen: c_int,
        lpCallerData: LPWSABUF,
        lpCalleeData: LPWSABUF,
        lpSQOS: LPQOS,
        lpGQOS: LPQOS,
    ) -> c_int;
    pub fn WSAConnectByNameW(
        s: SOCKET,
        nodename: LPWSTR,
        servicename: LPWSTR,
        LocalAddressLength: LPDWORD,
        LocalAddress: LPSOCKADDR,
        RemoteAddressLength: LPDWORD,
        RemoteAddress: LPSOCKADDR,
        timeout: *const timeval,
        Reserved: LPWSAOVERLAPPED,
    ) -> BOOL;
    pub fn WSAConnectByNameA(
        s: SOCKET,
        nodename: LPCSTR,
        servicename: LPCSTR,
        LocalAddressLength: LPDWORD,
        LocalAddress: LPSOCKADDR,
        RemoteAddressLength: LPDWORD,
        RemoteAddress: LPSOCKADDR,
        timeout: *const timeval,
        Reserved: LPWSAOVERLAPPED,
    ) -> BOOL;
    pub fn WSAConnectByList(
        s: SOCKET,
        SocketAddress: PSOCKET_ADDRESS_LIST,
        LocalAddressLength: LPDWORD,
        LocalAddress: LPSOCKADDR,
        RemoteAddressLength: LPDWORD,
        RemoteAddress: LPSOCKADDR,
        timeout: *const timeval,
        Reserved: LPWSAOVERLAPPED,
    ) -> BOOL;
    pub fn WSACreateEvent() -> WSAEVENT;
    pub fn WSADuplicateSocketA(
        s: SOCKET,
        dwProcessId: DWORD,
        lpProtocolInfo: LPWSAPROTOCOL_INFOA,
    ) -> c_int;
    pub fn WSADuplicateSocketW(
        s: SOCKET,
        dwProcessId: DWORD,
        lpProtocolInfo: LPWSAPROTOCOL_INFOW,
    ) -> c_int;
    pub fn WSAEnumNetworkEvents(
        s: SOCKET,
        hEventObject: WSAEVENT,
        lpNetworkEvents: LPWSANETWORKEVENTS,
    ) -> c_int;
    pub fn WSAEnumProtocolsA(
        lpiProtocols: LPINT,
        lpProtocolBuffer: LPWSAPROTOCOL_INFOA,
        lpdwBufferLength: LPDWORD,
    ) -> c_int;
    pub fn WSAEnumProtocolsW(
        lpiProtocols: LPINT,
        lpProtocolBuffer: LPWSAPROTOCOL_INFOW,
        lpdwBufferLength: LPDWORD,
    ) -> c_int;
    pub fn WSAEventSelect(
        s: SOCKET,
        hEventObject: WSAEVENT,
        lNetworkEvents: c_long,
    ) -> c_int;
    pub fn WSAGetOverlappedResult(
        s: SOCKET,
        lpOverlapped: LPWSAOVERLAPPED,
        lpcbTransfer: LPDWORD,
        fWait: BOOL,
        lpdwFlags: LPDWORD,
    ) -> BOOL;
    pub fn WSAGetQOSByName(
        s: SOCKET,
        lpQOSName: LPWSABUF,
        lpQOS: LPQOS,
    ) -> BOOL;
    pub fn WSAHtonl(
        s: SOCKET,
        hostlong: u_long,
        lpnetlong: *mut u_long,
    ) -> c_int;
    pub fn WSAHtons(s: SOCKET,
        hostshort: u_short,
        lpnetshort: *mut u_short,
    ) -> c_int;
    pub fn WSAIoctl(
        s: SOCKET,
        dwIoControlCode: DWORD,
        lpvInBuffer: LPVOID,
        cbInBuffer: DWORD,
        lpvOutBuffer: LPVOID,
        cbOutBuffer: DWORD,
        lpcbBytesReturned: LPDWORD,
        lpOverlapped: LPWSAOVERLAPPED,
        lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> c_int;
    pub fn WSAJoinLeaf(
        s: SOCKET,
        name: *const SOCKADDR,
        namelen: c_int,
        lpCallerData: LPWSABUF,
        lpCalleeData: LPWSABUF,
        lpSQOS: LPQOS,
        lpGQOS: LPQOS,
        dwFlags: DWORD,
    ) -> SOCKET;
    pub fn WSANtohl(
        s: SOCKET,
        netlong: u_long,
        lphostlong: *mut c_long,
    ) -> c_int;
    pub fn WSANtohs(
        s: SOCKET,
        netshort: u_short,
        lphostshort: *mut c_short,
    ) -> c_int;
    pub fn WSARecv(
        s: SOCKET,
        lpBuffers: LPWSABUF,
        dwBufferCount: DWORD,
        lpNumberOfBytesRecvd: LPDWORD,
        lpFlags: LPDWORD,
        lpOverlapped: LPWSAOVERLAPPED,
        lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> c_int;
    pub fn WSARecvDisconnect(
        s: SOCKET,
        lpInboundDisconnectData: LPWSABUF,
    ) -> c_int;
    pub fn WSARecvFrom(
        s: SOCKET,
        lpBuffers: LPWSABUF,
        dwBufferCount: DWORD,
        lpNumberOfBytesRecvd: LPDWORD,
        lpFlags: LPDWORD,
        lpFrom: *mut SOCKADDR,
        lpFromlen: LPINT,
        lpOverlapped: LPWSAOVERLAPPED,
        lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> c_int;
    pub fn WSAResetEvent(
        hEvent: WSAEVENT,
    ) -> BOOL;
    pub fn WSASend(
        s: SOCKET,
        lpBuffers: LPWSABUF,
        dwBufferCount: DWORD,
        lpNumberOfBytesSent: LPDWORD,
        dwFlags: DWORD,
        lpOverlapped: LPWSAOVERLAPPED,
        lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> c_int;
    pub fn WSASendMsg(
        Handle: SOCKET,
        lpMsg: LPWSAMSG,
        dwFlags: DWORD,
        lpNumberOfBytesSent: LPDWORD,
        lpOverlapped: LPWSAOVERLAPPED,
        lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> c_int;
    pub fn WSASendDisconnect(
        s: SOCKET,
        lpOutboundDisconnectData: LPWSABUF,
    ) -> c_int;
    pub fn WSASendTo(
        s: SOCKET,
        lpBuffers: LPWSABUF,
        dwBufferCount: DWORD,
        lpNumberOfBytesSent: LPDWORD,
        dwFlags: DWORD,
        lpTo: *const SOCKADDR,
        iToLen: c_int,
        lpOverlapped: LPWSAOVERLAPPED,
        lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> c_int;
    pub fn WSASetEvent(
        hEvent: WSAEVENT,
    ) -> BOOL;
    pub fn WSASocketA(
        af: c_int,
        _type: c_int,
        protocol: c_int,
        lpProtocolInfo: LPWSAPROTOCOL_INFOA,
        g: GROUP,
        dwFlags: DWORD,
    ) -> SOCKET;
    pub fn WSASocketW(
        af: c_int,
        _type: c_int,
        protocol: c_int,
        lpProtocolInfo: LPWSAPROTOCOL_INFOW,
        g: GROUP,
        dwFlags: DWORD,
    ) -> SOCKET;
    pub fn WSAWaitForMultipleEvents(
        cEvents: DWORD,
        lphEvents: *const WSAEVENT,
        fWaitAll: BOOL,
        dwTimeout: DWORD,
        fAlertable: BOOL,
    ) -> DWORD;
    pub fn WSAAddressToStringA(
        lpsaAddress: LPSOCKADDR,
        dwAddressLength: DWORD,
        lpProtocolInfo: LPWSAPROTOCOL_INFOA,
        lpszAddressString: LPSTR,
        lpdwAddressStringLength: LPDWORD,
    ) -> INT;
    pub fn WSAAddressToStringW(
        lpsaAddress: LPSOCKADDR,
        dwAddressLength: DWORD,
        lpProtocolInfo: LPWSAPROTOCOL_INFOW,
        lpszAddressString: LPWSTR,
        lpdwAddressStringLength: LPDWORD,
    ) -> INT;
    pub fn WSAStringToAddressA(
        AddressString: LPSTR,
        AddressFamily: INT,
        lpProtocolInfo: LPWSAPROTOCOL_INFOA,
        lpAddress: LPSOCKADDR,
        lpAddressLength: LPINT,
    ) -> INT;
    pub fn WSAStringToAddressW(
        AddressString: LPWSTR,
        AddressFamily: INT,
        lpProtocolInfo: LPWSAPROTOCOL_INFOW,
        lpAddress: LPSOCKADDR,
        lpAddressLength: LPINT,
    ) -> INT;
    pub fn WSALookupServiceBeginA(
        lpqsRestrictions: LPWSAQUERYSETA,
        dwControlFlags: DWORD,
        lphLookup: LPHANDLE,
    ) -> INT;
    pub fn WSALookupServiceBeginW(
        lpqsRestrictions: LPWSAQUERYSETW,
        dwControlFlags: DWORD,
        lphLookup: LPHANDLE,
    ) -> INT;
    pub fn WSALookupServiceNextA(
        hLookup: HANDLE,
        dwControlFlags: DWORD,
        lpdwBufferLength: LPDWORD,
        lpqsResults: LPWSAQUERYSETA,
    ) -> INT;
    pub fn WSALookupServiceNextW(
        hLookup: HANDLE,
        dwControlFlags: DWORD,
        lpdwBufferLength: LPDWORD,
        lpqsResults: LPWSAQUERYSETW,
    ) -> INT;
    pub fn WSANSPIoctl(
        hLookup: HANDLE,
        dwControlFlags: DWORD,
        lpvInBuffer: LPVOID,
        cbInBuffer: DWORD,
        lpvOutBuffer: LPVOID,
        cbOutBuffer: DWORD,
        lpcbBytesReturned: LPDWORD,
        lpCompletion: LPWSACOMPLETION,
    ) -> INT;
    pub fn WSALookupServiceEnd(
        hLookup: HANDLE,
    ) -> INT;
    pub fn WSAInstallServiceClassA(
        lpServiceClassInfo: LPWSASERVICECLASSINFOA,
    ) -> INT;
    pub fn WSAInstallServiceClassW(
        lpServiceClassInfo: LPWSASERVICECLASSINFOW,
    ) -> INT;
    pub fn WSARemoveServiceClass(
        lpServiceClassId: LPGUID,
    ) -> INT;
    pub fn WSAGetServiceClassInfoA(
        lpProviderId: LPGUID,
        lpServiceClassId: LPGUID,
        lpdwBufSize: LPDWORD,
        lpServiceClassInfo: LPWSASERVICECLASSINFOA,
    ) -> INT;
    pub fn WSAGetServiceClassInfoW(
        lpProviderId: LPGUID,
        lpServiceClassId: LPGUID,
        lpdwBufSize: LPDWORD,
        lpServiceClassInfo: LPWSASERVICECLASSINFOW,
    ) -> INT;
    pub fn WSAEnumNameSpaceProvidersA(
        lpdwBufferLength: LPDWORD,
        lpnspBuffer: LPWSANAMESPACE_INFOA,
    ) -> INT;
    pub fn WSAEnumNameSpaceProvidersW(
        lpdwBufferLength: LPDWORD,
        lpnspBuffer: LPWSANAMESPACE_INFOW,
    ) -> INT;
    pub fn WSAEnumNameSpaceProvidersExA(
        lpdwBufferLength: LPDWORD,
        lpnspBuffer: LPWSANAMESPACE_INFOEXA,
    ) -> INT;
    pub fn WSAEnumNameSpaceProvidersExW(
        lpdwBufferLength: LPDWORD,
        lpnspBuffer: LPWSANAMESPACE_INFOEXW,
    ) -> INT;
    pub fn WSAGetServiceClassNameByClassIdA(
        lpServiceClassId: LPGUID,
        lpszServiceClassName: LPSTR,
        lpdwBufferLength: LPDWORD,
    ) -> INT;
    pub fn WSAGetServiceClassNameByClassIdW(
        lpServiceClassId: LPGUID,
        lpszServiceClassName: LPWSTR,
        lpdwBufferLength: LPDWORD,
    ) -> INT;
    pub fn WSASetServiceA(
        lpqsRegInfo: LPWSAQUERYSETA,
        essoperation: WSAESETSERVICEOP,
        dwControlFlags: DWORD,
    ) -> INT;
    pub fn WSASetServiceW(
        lpqsRegInfo: LPWSAQUERYSETW,
        essoperation: WSAESETSERVICEOP,
        dwControlFlags: DWORD,
    ) -> INT;
    pub fn WSAProviderConfigChange(
        lpNotificationHandle: LPHANDLE,
        lpOverlapped: LPWSAOVERLAPPED,
        lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> INT;
    pub fn WSAPoll(
        fdArray: LPWSAPOLLFD,
        fds: ULONG,
        timeout: INT,
    ) -> c_int;
}
pub type LPSOCKADDR_IN = *mut SOCKADDR_IN;
pub type LINGER = linger;
pub type PLINGER = *mut linger;
pub type LPLINGER = *mut linger;
pub type FD_SET = fd_set;
pub type PFD_SET = *mut fd_set;
pub type LPFD_SET = *mut fd_set;
pub type HOSTENT = hostent;
pub type PHOSTENT = *mut hostent;
pub type LPHOSTENT = *mut hostent;
pub type SERVENT = servent;
pub type PSERVENT = *mut servent;
pub type LPSERVENT = *mut servent;
pub type PROTOENT = protoent;
pub type PPROTOENT = *mut protoent;
pub type LPPROTOENT = *mut protoent;
pub type TIMEVAL = timeval;
pub type PTIMEVAL = *mut timeval;
pub type LPTIMEVAL = *mut timeval;
#[inline]
pub fn WSAMAKEASYNCREPLY(buflen: WORD, error: WORD) -> LONG {
    MAKELONG(buflen, error)
}
#[inline]
pub fn WSAMAKESELECTREPLY(event: WORD, error: WORD) -> LONG {
    MAKELONG(event, error)
}
#[inline]
pub fn WSAGETASYNCBUFLEN(lParam: DWORD) -> WORD {
    LOWORD(lParam)
}
#[inline]
pub fn WSAGETASYNCERROR(lParam: DWORD) -> WORD {
    HIWORD(lParam)
}
#[inline]
pub fn WSAGETSELECTEVENT(lParam: DWORD) -> WORD {
    LOWORD(lParam)
}
#[inline]
pub fn WSAGETSELECTERROR(lParam: DWORD) -> WORD {
    HIWORD(lParam)
}
