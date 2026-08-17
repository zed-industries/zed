// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Core definitions for the Winsock2 specification
use ctypes::{__int64, c_char, c_int, c_long, c_short, c_void};
use shared::basetsd::SIZE_T;
use shared::guiddef::LPGUID;
use shared::inaddr::IN_ADDR;
use shared::minwindef::{DWORD, INT, UCHAR, ULONG, USHORT};
use um::winnt::{CHAR, HANDLE, LONG, PROCESSOR_NUMBER, PWSTR};
use vc::vcruntime::size_t;
pub type ADDRESS_FAMILY = USHORT;
pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_IMPLINK: c_int = 3;
pub const AF_PUP: c_int = 4;
pub const AF_CHAOS: c_int = 5;
pub const AF_NS: c_int = 6;
pub const AF_IPX: c_int = AF_NS;
pub const AF_ISO: c_int = 7;
pub const AF_OSI: c_int = AF_ISO;
pub const AF_ECMA: c_int = 8;
pub const AF_DATAKIT: c_int = 9;
pub const AF_CCITT: c_int = 10;
pub const AF_SNA: c_int = 11;
pub const AF_DECnet: c_int = 12;
pub const AF_DLI: c_int = 13;
pub const AF_LAT: c_int = 14;
pub const AF_HYLINK: c_int = 15;
pub const AF_APPLETALK: c_int = 16;
pub const AF_NETBIOS: c_int = 17;
pub const AF_VOICEVIEW: c_int = 18;
pub const AF_FIREFOX: c_int = 19;
pub const AF_UNKNOWN1: c_int = 20;
pub const AF_BAN: c_int = 21;
pub const AF_ATM: c_int = 22;
pub const AF_INET6: c_int = 23;
pub const AF_CLUSTER: c_int = 24;
pub const AF_12844: c_int = 25;
pub const AF_IRDA: c_int = 26;
pub const AF_NETDES: c_int = 28;
pub const AF_TCNPROCESS: c_int = 29;
pub const AF_TCNMESSAGE: c_int = 30;
pub const AF_ICLFXBM: c_int = 31;
pub const AF_BTH: c_int = 32;
pub const AF_LINK: c_int = 33;
pub const AF_HYPERV: c_int = 34;
pub const AF_MAX: c_int = 35;
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;
pub const SOCK_SEQPACKET: c_int = 5;
pub const SOL_SOCKET: c_int = 0xffff;
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
pub const SO_BSP_STATE: c_int = 0x1009;
pub const SO_GROUP_ID: c_int = 0x2001;
pub const SO_GROUP_PRIORITY: c_int = 0x2002;
pub const SO_MAX_MSG_SIZE: c_int = 0x2003;
pub const SO_CONDITIONAL_ACCEPT: c_int = 0x3002;
pub const SO_PAUSE_ACCEPT: c_int = 0x3003;
pub const SO_COMPARTMENT_ID: c_int = 0x3004;
pub const SO_RANDOMIZE_PORT: c_int = 0x3005;
pub const SO_PORT_SCALABILITY: c_int = 0x3006;
pub const SO_REUSE_UNICASTPORT: c_int = 0x3007;
pub const SO_REUSE_MULTICASTPORT: c_int = 0x3008;
pub const WSK_SO_BASE: c_int = 0x4000;
pub const TCP_NODELAY: c_int = 0x0001;
STRUCT!{struct SOCKADDR {
    sa_family: ADDRESS_FAMILY,
    sa_data: [CHAR; 14],
}}
pub type PSOCKADDR = *mut SOCKADDR;
pub type LPSOCKADDR = *mut SOCKADDR;
STRUCT!{struct SOCKET_ADDRESS {
    lpSockaddr: LPSOCKADDR,
    iSockaddrLength: INT,
}}
pub type PSOCKET_ADDRESS = *mut SOCKET_ADDRESS;
pub type LPSOCKET_ADDRESS = *mut SOCKET_ADDRESS;
STRUCT!{struct SOCKET_ADDRESS_LIST {
    iAddressCount: INT,
    Address: [SOCKET_ADDRESS; 1],
}}
pub type PSOCKET_ADDRESS_LIST = *mut SOCKET_ADDRESS_LIST;
pub type LPSOCKET_ADDRESS_LIST = *mut SOCKET_ADDRESS_LIST;
STRUCT!{struct CSADDR_INFO {
    LocalAddr: SOCKET_ADDRESS,
    RemoteAddr: SOCKET_ADDRESS,
    iSocketType: INT,
    iProtocol: INT,
}}
pub type PCSADDR_INFO = *mut CSADDR_INFO;
pub type LPCSADDR_INFO = *mut CSADDR_INFO;
STRUCT!{struct SOCKADDR_STORAGE_LH {
    ss_family: ADDRESS_FAMILY,
    __ss_pad1: [CHAR; 6],
    __ss_align: __int64,
    __ss_pad2: [CHAR; 112],
}}
pub type PSOCKADDR_STORAGE_LH = *mut SOCKADDR_STORAGE_LH;
pub type LPSOCKADDR_STORAGE_LH = *mut SOCKADDR_STORAGE_LH;
STRUCT!{struct SOCKADDR_STORAGE_XP {
    ss_family: c_short,
    __ss_pad1: [CHAR; 6],
    __ss_align: __int64,
    __ss_pad2: [CHAR; 112],
}}
pub type PSOCKADDR_STORAGE_XP = *mut SOCKADDR_STORAGE_XP;
pub type LPSOCKADDR_STORAGE_XP = *mut SOCKADDR_STORAGE_XP;
pub type SOCKADDR_STORAGE = SOCKADDR_STORAGE_LH;
pub type PSOCKADDR_STORAGE = *mut SOCKADDR_STORAGE;
pub type LPSOCKADDR_STORAGE = *mut SOCKADDR_STORAGE;
STRUCT!{struct SOCKET_PROCESSOR_AFFINITY {
    Processor: PROCESSOR_NUMBER,
    NumaNodeId: USHORT,
    Reserved: USHORT,
}}
pub type PSOCKET_PROCESSOR_AFFINITY = *mut SOCKET_PROCESSOR_AFFINITY;
pub const IOC_UNIX: DWORD = 0x00000000;
pub const IOC_WS2: DWORD = 0x08000000;
pub const IOC_PROTOCOL: DWORD = 0x10000000;
pub const IOC_VENDOR: DWORD = 0x18000000;
pub const IOC_WSK: DWORD = IOC_WS2 | 0x07000000;
pub const SIO_ASSOCIATE_HANDLE: DWORD = _WSAIOW!(IOC_WS2, 1);
pub const SIO_ENABLE_CIRCULAR_QUEUEING: DWORD = _WSAIO!(IOC_WS2, 2);
pub const SIO_FIND_ROUTE: DWORD = _WSAIOR!(IOC_WS2, 3);
pub const SIO_FLUSH: DWORD = _WSAIO!(IOC_WS2, 4);
pub const SIO_GET_BROADCAST_ADDRESS: DWORD = _WSAIOR!(IOC_WS2, 5);
pub const SIO_GET_EXTENSION_FUNCTION_POINTER: DWORD = _WSAIORW!(IOC_WS2, 6);
pub const SIO_GET_QOS: DWORD = _WSAIORW!(IOC_WS2, 7);
pub const SIO_GET_GROUP_QOS: DWORD = _WSAIORW!(IOC_WS2, 8);
pub const SIO_MULTIPOINT_LOOPBACK: DWORD = _WSAIOW!(IOC_WS2, 9);
pub const SIO_MULTICAST_SCOPE: DWORD = _WSAIOW!(IOC_WS2, 10);
pub const SIO_SET_QOS: DWORD = _WSAIOW!(IOC_WS2, 11);
pub const SIO_SET_GROUP_QOS: DWORD = _WSAIOW!(IOC_WS2, 12);
pub const SIO_TRANSLATE_HANDLE: DWORD = _WSAIORW!(IOC_WS2, 13);
pub const SIO_ROUTING_INTERFACE_QUERY: DWORD = _WSAIORW!(IOC_WS2, 20);
pub const SIO_ROUTING_INTERFACE_CHANGE: DWORD = _WSAIOW!(IOC_WS2, 21);
pub const SIO_ADDRESS_LIST_QUERY: DWORD = _WSAIOR!(IOC_WS2, 22);
pub const SIO_ADDRESS_LIST_CHANGE: DWORD = _WSAIO!(IOC_WS2, 23);
pub const SIO_QUERY_TARGET_PNP_HANDLE: DWORD = _WSAIOR!(IOC_WS2, 24);
pub const SIO_QUERY_RSS_PROCESSOR_INFO: DWORD = _WSAIOR!(IOC_WS2, 37);
pub const SIO_ADDRESS_LIST_SORT: DWORD = _WSAIORW!(IOC_WS2, 25);
pub const SIO_RESERVED_1: DWORD = _WSAIOW!(IOC_WS2, 26);
pub const SIO_RESERVED_2: DWORD = _WSAIOW!(IOC_WS2, 33);
pub const SIO_GET_MULTIPLE_EXTENSION_FUNCTION_POINTER: DWORD = _WSAIORW!(IOC_WS2, 36);
pub const IPPROTO_IP: c_int = 0;
ENUM!{enum IPPROTO {
    IPPROTO_HOPOPTS = 0, // IPv6 Hop-by-Hop options
    IPPROTO_ICMP = 1,
    IPPROTO_IGMP = 2,
    IPPROTO_GGP = 3,
    IPPROTO_IPV4 = 4,
    IPPROTO_ST = 5,
    IPPROTO_TCP = 6,
    IPPROTO_CBT = 7,
    IPPROTO_EGP = 8,
    IPPROTO_IGP = 9,
    IPPROTO_PUP = 12,
    IPPROTO_UDP = 17,
    IPPROTO_IDP = 22,
    IPPROTO_RDP = 27,
    IPPROTO_IPV6 = 41, // IPv6 header
    IPPROTO_ROUTING = 43, // IPv6 Routing header
    IPPROTO_FRAGMENT = 44, // IPv6 fragmentation header
    IPPROTO_ESP = 50, // encapsulating security payload
    IPPROTO_AH = 51, // authentication header
    IPPROTO_ICMPV6 = 58, // ICMPv6
    IPPROTO_NONE = 59, // IPv6 no next header
    IPPROTO_DSTOPTS = 60, // IPv6 Destination options
    IPPROTO_ND = 77,
    IPPROTO_ICLFXBM = 78,
    IPPROTO_PIM = 103,
    IPPROTO_PGM = 113,
    IPPROTO_L2TP = 115,
    IPPROTO_SCTP = 132,
    IPPROTO_RAW = 255,
    IPPROTO_MAX = 256,
    IPPROTO_RESERVED_RAW = 257,
    IPPROTO_RESERVED_IPSEC = 258,
    IPPROTO_RESERVED_IPSECOFFLOAD = 259,
    IPPROTO_RESERVED_WNV = 260,
    IPPROTO_RESERVED_MAX = 261,
}}
pub type PIPPROTO = *mut IPPROTO;
pub const IPPORT_TCPMUX: USHORT = 1;
pub const IPPORT_ECHO: USHORT = 7;
pub const IPPORT_DISCARD: USHORT = 9;
pub const IPPORT_SYSTAT: USHORT = 11;
pub const IPPORT_DAYTIME: USHORT = 13;
pub const IPPORT_NETSTAT: USHORT = 15;
pub const IPPORT_QOTD: USHORT = 17;
pub const IPPORT_MSP: USHORT = 18;
pub const IPPORT_CHARGEN: USHORT = 19;
pub const IPPORT_FTP_DATA: USHORT = 20;
pub const IPPORT_FTP: USHORT = 21;
pub const IPPORT_TELNET: USHORT = 23;
pub const IPPORT_SMTP: USHORT = 25;
pub const IPPORT_TIMESERVER: USHORT = 37;
pub const IPPORT_NAMESERVER: USHORT = 42;
pub const IPPORT_WHOIS: USHORT = 43;
pub const IPPORT_MTP: USHORT = 57;
pub const IPPORT_TFTP: USHORT = 69;
pub const IPPORT_RJE: USHORT = 77;
pub const IPPORT_FINGER: USHORT = 79;
pub const IPPORT_TTYLINK: USHORT = 87;
pub const IPPORT_SUPDUP: USHORT = 95;
pub const IPPORT_POP3: USHORT = 110;
pub const IPPORT_NTP: USHORT = 123;
pub const IPPORT_EPMAP: USHORT = 135;
pub const IPPORT_NETBIOS_NS: USHORT = 137;
pub const IPPORT_NETBIOS_DGM: USHORT = 138;
pub const IPPORT_NETBIOS_SSN: USHORT = 139;
pub const IPPORT_IMAP: USHORT = 143;
pub const IPPORT_SNMP: USHORT = 161;
pub const IPPORT_SNMP_TRAP: USHORT = 162;
pub const IPPORT_IMAP3: USHORT = 220;
pub const IPPORT_LDAP: USHORT = 389;
pub const IPPORT_HTTPS: USHORT = 443;
pub const IPPORT_MICROSOFT_DS: USHORT = 445;
pub const IPPORT_EXECSERVER: USHORT = 512;
pub const IPPORT_LOGINSERVER: USHORT = 513;
pub const IPPORT_CMDSERVER: USHORT = 514;
pub const IPPORT_EFSSERVER: USHORT = 520;
pub const IPPORT_BIFFUDP: USHORT = 512;
pub const IPPORT_WHOSERVER: USHORT = 513;
pub const IPPORT_ROUTESERVER: USHORT = 520;
pub const IPPORT_RESERVED: USHORT = 1024;
pub const IPPORT_REGISTERED_MIN: USHORT = IPPORT_RESERVED;
pub const IPPORT_REGISTERED_MAX: USHORT = 0xbfff;
pub const IPPORT_DYNAMIC_MIN: USHORT = 0xc000;
pub const IPPORT_DYNAMIC_MAX: USHORT = 0xffff;
#[inline]
pub fn IN_CLASSA(i: LONG) -> bool {
    (i & 0x80000000) == 0
}
pub const IN_CLASSA_NET: LONG = 0xff000000;
pub const IN_CLASSA_NSHIFT: LONG = 24;
pub const IN_CLASSA_HOST: LONG = 0x00ffffff;
pub const IN_CLASSA_MAX: LONG = 128;
#[inline]
pub fn IN_CLASSB(i: LONG) -> bool {
    (i as u32 & 0xc0000000) == 0x80000000
}
pub const IN_CLASSB_NET: LONG = 0xffff0000;
pub const IN_CLASSB_NSHIFT: LONG = 16;
pub const IN_CLASSB_HOST: LONG = 0x0000ffff;
pub const IN_CLASSB_MAX: LONG = 65536;
#[inline]
pub fn IN_CLASSC(i: LONG) -> bool {
    (i as u32 & 0xe0000000) == 0xc0000000
}
pub const IN_CLASSC_NET: LONG = 0xffffff00;
pub const IN_CLASSC_NSHIFT: LONG = 8;
pub const IN_CLASSC_HOST: LONG = 0x000000ff;
#[inline]
pub fn IN_CLASSD(i: c_long) -> bool {
    (i as u32 & 0xf0000000) == 0xe0000000
}
pub const IN_CLASSD_NET: LONG = 0xf0000000;
pub const IN_CLASSD_NSHIFT: LONG = 28;
pub const IN_CLASSD_HOST: LONG = 0x0fffffff;
#[inline]
pub fn IN_MULTICAST(i: c_long) -> bool {
    IN_CLASSD(i)
}
pub const INADDR_ANY: ULONG = 0x00000000;
pub const INADDR_LOOPBACK: ULONG = 0x7f000001;
pub const INADDR_BROADCAST: ULONG = 0xffffffff;
pub const INADDR_NONE: ULONG = 0xffffffff;
ENUM!{enum SCOPE_LEVEL {
    ScopeLevelInterface = 1,
    ScopeLevelLink = 2,
    ScopeLevelSubnet = 3,
    ScopeLevelAdmin = 4,
    ScopeLevelSite = 5,
    ScopeLevelOrganization = 8,
    ScopeLevelGlobal = 14,
    ScopeLevelCount = 16,
}}
STRUCT!{struct SCOPE_ID_u_s {
    bitfield: ULONG,
}}
BITFIELD!{SCOPE_ID_u_s bitfield: ULONG [
    Zone set_Zone[0..28],
    Level set_Level[28..32],
]}
UNION!{union SCOPE_ID_u {
    [u32; 1],
    s s_mut: SCOPE_ID_u_s,
    Value Value_mut: ULONG,
}}
STRUCT!{struct SCOPE_ID {
    u: SCOPE_ID_u,
}}
pub type PSCOPE_ID = *mut SCOPE_ID;
STRUCT!{struct SOCKADDR_IN {
    sin_family: ADDRESS_FAMILY,
    sin_port: USHORT,
    sin_addr: IN_ADDR,
    sin_zero: [CHAR; 8],
}}
pub type PSOCKADDR_IN = *mut SOCKADDR_IN;
STRUCT!{struct SOCKADDR_DL {
    sdl_family: ADDRESS_FAMILY,
    sdl_data: [UCHAR; 8],
    sdl_zero: [UCHAR; 4],
}}
pub type PSOCKADDR_DL = *mut SOCKADDR_DL;
pub const IOCPARM_MASK: DWORD = 0x7f;
pub const IOC_VOID: DWORD = 0x20000000;
pub const IOC_OUT: DWORD = 0x40000000;
pub const IOC_IN: DWORD = 0x80000000;
pub const IOC_INOUT: DWORD = IOC_IN | IOC_OUT;
STRUCT!{struct WSABUF {
    len: ULONG,
    buf: *mut CHAR,
}}
pub type LPWSABUF = *mut WSABUF;
STRUCT!{struct WSAMSG {
    name: LPSOCKADDR,
    namelen: INT,
    lpBuffers: LPWSABUF,
    dwBufferCount: ULONG,
    Control: WSABUF,
    dwFlags: ULONG,
}}
pub type PWSAMSG = *mut WSAMSG;
pub type LPWSAMSG = *mut WSAMSG;
STRUCT!{struct WSACMSGHDR {
    cmsg_len: SIZE_T,
    cmsg_level: INT,
    cmsg_type: INT,
}}
pub type PWSACMSGHDR = *mut WSACMSGHDR;
pub type LPWSACMSGHDR = *mut WSACMSGHDR;
pub type CMSGHDR = WSACMSGHDR;
pub type PCMSGHDR = *mut WSACMSGHDR;
pub const MSG_TRUNC: ULONG = 0x0100;
pub const MSG_CTRUNC: ULONG = 0x0200;
pub const MSG_BCAST: ULONG = 0x0400;
pub const MSG_MCAST: ULONG = 0x0800;
pub const AI_PASSIVE: c_int = 0x00000001;
pub const AI_CANONNAME: c_int = 0x00000002;
pub const AI_NUMERICHOST: c_int = 0x00000004;
pub const AI_NUMERICSERV: c_int = 0x00000008;
pub const AI_DNS_ONLY: c_int = 0x00000010;
pub const AI_ALL: c_int = 0x00000100;
pub const AI_ADDRCONFIG: c_int = 0x00000400;
pub const AI_V4MAPPED: c_int = 0x00000800;
pub const AI_NON_AUTHORITATIVE: c_int = 0x00004000;
pub const AI_SECURE: c_int = 0x00008000;
pub const AI_RETURN_PREFERRED_NAMES: c_int = 0x00010000;
pub const AI_FQDN: c_int = 0x00020000;
pub const AI_FILESERVER: c_int = 0x00040000;
pub const AI_DISABLE_IDN_ENCODING: c_int = 0x00080000;
pub const AI_EXTENDED: c_int = 0x80000000;
pub const AI_RESOLUTION_HANDLE: c_int = 0x40000000;
STRUCT!{struct ADDRINFOA {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: *mut c_char,
    ai_addr: *mut SOCKADDR,
    ai_next: *mut ADDRINFOA,
}}
pub type PADDRINFOA = *mut ADDRINFOA;
STRUCT!{struct ADDRINFOW {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: PWSTR,
    ai_addr: *mut SOCKADDR,
    ai_next: *mut ADDRINFOW,
}}
pub type PADDRINFOW = *mut ADDRINFOW;
STRUCT!{struct ADDRINFOEXA {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: *mut c_char,
    ai_addr: *mut SOCKADDR,
    ai_blob: *mut c_void,
    ai_bloblen: size_t,
    ai_provider: LPGUID,
    ai_next: *mut ADDRINFOEXA,
}}
pub type PADDRINFOEXA = *mut ADDRINFOEXA;
pub type LPADDRINFOEXA = *mut ADDRINFOEXA;
STRUCT!{struct ADDRINFOEXW {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: PWSTR,
    ai_addr: *mut SOCKADDR,
    ai_blob: *mut c_void,
    ai_bloblen: size_t,
    ai_provider: LPGUID,
    ai_next: *mut ADDRINFOEXW,
}}
pub type PADDRINFOEXW = *mut ADDRINFOEXW;
pub type LPADDRINFOEXW = *mut ADDRINFOEXW;
pub const ADDRINFOEX_VERSION_2: c_int = 2;
pub const ADDRINFOEX_VERSION_3: c_int = 3;
pub const ADDRINFOEX_VERSION_4: c_int = 4;
STRUCT!{struct ADDRINFOEX2A {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: *mut c_char,
    ai_addr: *mut SOCKADDR,
    ai_blob: *mut c_void,
    ai_bloblen: size_t,
    ai_provider: LPGUID,
    ai_next: *mut ADDRINFOEX2W,
    ai_version: c_int,
    ai_fqdn: *mut c_char,
}}
pub type PADDRINFOEX2A = *mut ADDRINFOEX2A;
pub type LPADDRINFOEX2A = *mut ADDRINFOEX2A;
STRUCT!{struct ADDRINFOEX2W {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: PWSTR,
    ai_addr: *mut SOCKADDR,
    ai_blob: *mut c_void,
    ai_bloblen: size_t,
    ai_provider: LPGUID,
    ai_next: *mut ADDRINFOEX2W,
    ai_version: c_int,
    ai_fqdn: PWSTR,
}}
pub type PADDRINFOEX2W = *mut ADDRINFOEX2W;
pub type LPADDRINFOEX2W = *mut ADDRINFOEX2W;
STRUCT!{struct ADDRINFOEX3A {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: *mut c_char,
    ai_addr: *mut SOCKADDR,
    ai_blob: *mut c_void,
    ai_bloblen: size_t,
    ai_provider: LPGUID,
    ai_next: *mut ADDRINFOEX3W,
    ai_version: c_int,
    ai_fqdn: *mut c_char,
    ai_interfaceindex: c_int,
}}
pub type PADDRINFOEX3A = *mut ADDRINFOEX3A;
pub type LPADDRINFOEX3A = *mut ADDRINFOEX3A;
STRUCT!{struct ADDRINFOEX3W {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: PWSTR,
    ai_addr: *mut SOCKADDR,
    ai_blob: *mut c_void,
    ai_bloblen: size_t,
    ai_provider: LPGUID,
    ai_next: *mut ADDRINFOEX3W,
    ai_version: c_int,
    ai_fqdn: PWSTR,
    ai_interfaceindex: c_int,
}}
pub type PADDRINFOEX3W = *mut ADDRINFOEX3W;
pub type LPADDRINFOEX3W = *mut ADDRINFOEX3W;
STRUCT!{struct ADDRINFOEX4 {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: size_t,
    ai_canonname: PWSTR,
    ai_addr: *mut SOCKADDR,
    ai_blob: *mut c_void,
    ai_bloblen: size_t,
    ai_provider: LPGUID,
    ai_next: *mut ADDRINFOEX4,
    ai_version: c_int,
    ai_fqdn: PWSTR,
    ai_interfaceindex: c_int,
    ai_resolutionhandle: HANDLE,
}}
pub type PADDRINFOEX4 = *mut ADDRINFOEX4;
pub type LPADDRINFOEX4 = *mut ADDRINFOEX4;
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
pub const NI_NOFQDN: c_int = 0x01;
pub const NI_NUMERICHOST: c_int = 0x02;
pub const NI_NAMEREQD: c_int = 0x04;
pub const NI_NUMERICSERV: c_int = 0x08;
pub const NI_DGRAM: c_int = 0x10;
pub const NI_MAXHOST: c_int = 1025;
pub const NI_MAXSERV: c_int = 32;
