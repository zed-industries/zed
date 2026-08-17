// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! This module contains Microsoft-specific extensions to the core Winsock definitions.
use ctypes::wchar_t;
use shared::basetsd::{UINT32, UINT64, ULONG64};
use shared::guiddef::GUID;
use shared::in6addr::IN6_ADDR;
use shared::inaddr::IN_ADDR;
use shared::minwindef::{DWORD, PULONG, PUSHORT, UCHAR, ULONG, USHORT};
use shared::ws2def::{
    INADDR_ANY, INADDR_BROADCAST, INADDR_NONE, IOC_VENDOR, SOCKADDR_IN,
    SOCKADDR_STORAGE,
};
use um::winnt::{BOOLEAN, LONG, LPCWSTR, PCSTR, PCWSTR, PSTR, PWSTR};
DEFINE_GUID!{SOCKET_DEFAULT2_QM_POLICY,
    0xaec2ef9c, 0x3a4d, 0x4d3e, 0x88, 0x42, 0x23, 0x99, 0x42, 0xe3, 0x9a, 0x47}
DEFINE_GUID!{REAL_TIME_NOTIFICATION_CAPABILITY,
    0x6b59819a, 0x5cae, 0x492d, 0xa9, 0x01, 0x2a, 0x3c, 0x2c, 0x50, 0x16, 0x4f}
DEFINE_GUID!{REAL_TIME_NOTIFICATION_CAPABILITY_EX,
    0x6843da03, 0x154a, 0x4616, 0xa5, 0x08, 0x44, 0x37, 0x12, 0x95, 0xf9, 0x6b}
DEFINE_GUID!{ASSOCIATE_NAMERES_CONTEXT,
    0x59a38b67, 0xd4fe, 0x46e1, 0xba, 0x3c, 0x87, 0xea, 0x74, 0xca, 0x30, 0x49}
ENUM!{enum TCPSTATE {
    TCPSTATE_CLOSED,
    TCPSTATE_LISTEN,
    TCPSTATE_SYN_SENT,
    TCPSTATE_SYN_RCVD,
    TCPSTATE_ESTABLISHED,
    TCPSTATE_FIN_WAIT_1,
    TCPSTATE_FIN_WAIT_2,
    TCPSTATE_CLOSE_WAIT,
    TCPSTATE_CLOSING,
    TCPSTATE_LAST_ACK,
    TCPSTATE_TIME_WAIT,
    TCPSTATE_MAX,
}}
STRUCT!{struct TRANSPORT_SETTING_ID {
    Guid: GUID,
}}
pub type PTRANSPORT_SETTING_ID = *mut TRANSPORT_SETTING_ID;
STRUCT!{struct tcp_keepalive {
    onoff: ULONG,
    keepalivetime: ULONG,
    keepaliveinterval: ULONG,
}}
ENUM!{enum CONTROL_CHANNEL_TRIGGER_STATUS {
    CONTROL_CHANNEL_TRIGGER_STATUS_INVALID = 0,
    CONTROL_CHANNEL_TRIGGER_STATUS_SOFTWARE_SLOT_ALLOCATED = 1,
    CONTROL_CHANNEL_TRIGGER_STATUS_HARDWARE_SLOT_ALLOCATED = 2,
    CONTROL_CHANNEL_TRIGGER_STATUS_POLICY_ERROR = 3,
    CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR = 4,
    CONTROL_CHANNEL_TRIGGER_STATUS_TRANSPORT_DISCONNECTED = 5,
    CONTROL_CHANNEL_TRIGGER_STATUS_SERVICE_UNAVAILABLE = 6,
}}
pub type PCONTROL_CHANNEL_TRIGGER_STATUS = *mut CONTROL_CHANNEL_TRIGGER_STATUS;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_MAX: u32 = CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR;
STRUCT!{struct REAL_TIME_NOTIFICATION_SETTING_INPUT {
    TransportSettingId: TRANSPORT_SETTING_ID,
    BrokerEventGuid: GUID,
}}
pub type PREAL_TIME_NOTIFICATION_SETTING_INPUT = *mut REAL_TIME_NOTIFICATION_SETTING_INPUT;
STRUCT!{struct REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    TransportSettingId: TRANSPORT_SETTING_ID,
    BrokerEventGuid: GUID,
    Unmark: BOOLEAN,
}}
pub type PREAL_TIME_NOTIFICATION_SETTING_INPUT_EX = *mut REAL_TIME_NOTIFICATION_SETTING_INPUT_EX;
STRUCT!{struct REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    ChannelStatus: CONTROL_CHANNEL_TRIGGER_STATUS,
}}
pub type PREAL_TIME_NOTIFICATION_SETTING_OUTPUT = *mut REAL_TIME_NOTIFICATION_SETTING_OUTPUT;
STRUCT!{struct ASSOCIATE_NAMERES_CONTEXT_INPUT {
    TransportSettingId: TRANSPORT_SETTING_ID,
    Handle: UINT64,
}}
pub type PASSOCIATE_NAMERES_CONTEXT_INPUT = *mut ASSOCIATE_NAMERES_CONTEXT_INPUT;
pub const SIO_RCVALL: DWORD = _WSAIOW!(IOC_VENDOR,1);
pub const SIO_RCVALL_MCAST: DWORD = _WSAIOW!(IOC_VENDOR,2);
pub const SIO_RCVALL_IGMPMCAST: DWORD = _WSAIOW!(IOC_VENDOR,3);
pub const SIO_KEEPALIVE_VALS: DWORD = _WSAIOW!(IOC_VENDOR,4);
pub const SIO_ABSORB_RTRALERT: DWORD = _WSAIOW!(IOC_VENDOR,5);
pub const SIO_UCAST_IF: DWORD = _WSAIOW!(IOC_VENDOR,6);
pub const SIO_LIMIT_BROADCASTS: DWORD = _WSAIOW!(IOC_VENDOR,7);
pub const SIO_INDEX_BIND: DWORD = _WSAIOW!(IOC_VENDOR,8);
pub const SIO_INDEX_MCASTIF: DWORD = _WSAIOW!(IOC_VENDOR,9);
pub const SIO_INDEX_ADD_MCAST: DWORD = _WSAIOW!(IOC_VENDOR,10);
pub const SIO_INDEX_DEL_MCAST: DWORD = _WSAIOW!(IOC_VENDOR,11);
pub const SIO_RCVALL_MCAST_IF: DWORD = _WSAIOW!(IOC_VENDOR,13);
pub const SIO_RCVALL_IF: DWORD = _WSAIOW!(IOC_VENDOR,14);
pub const SIO_LOOPBACK_FAST_PATH: DWORD = _WSAIOW!(IOC_VENDOR,16);
pub const SIO_TCP_INITIAL_RTO: DWORD = _WSAIOW!(IOC_VENDOR,17);
pub const SIO_APPLY_TRANSPORT_SETTING: DWORD = _WSAIOW!(IOC_VENDOR,19);
pub const SIO_QUERY_TRANSPORT_SETTING: DWORD = _WSAIOW!(IOC_VENDOR,20);
pub const SIO_TCP_SET_ICW: DWORD = _WSAIOW!(IOC_VENDOR,22);
pub const SIO_TCP_SET_ACK_FREQUENCY: DWORD = _WSAIOW!(IOC_VENDOR,23);
pub const SIO_TCP_INFO: DWORD = _WSAIORW!(IOC_VENDOR,39);
ENUM!{enum RCVALL_VALUE {
    RCVALL_OFF = 0,
    RCVALL_ON = 1,
    RCVALL_SOCKETLEVELONLY = 2,
    RCVALL_IPLEVEL = 3,
}}
pub type PRCVALL_VALUE = *mut RCVALL_VALUE;
STRUCT!{struct RCVALL_IF {
    Mode: RCVALL_VALUE,
    Interface: ULONG,
}}
pub type PRCVALL_IF = *mut RCVALL_IF;
pub const TCP_INITIAL_RTO_UNSPECIFIED_RTT: USHORT = -1i16 as u16;
pub const TCP_INITIAL_RTO_UNSPECIFIED_MAX_SYN_RETRANSMISSIONS: UCHAR = -1i8 as u8;
pub const TCP_INITIAL_RTO_DEFAULT_RTT: USHORT = 0;
pub const TCP_INITIAL_RTO_DEFAULT_MAX_SYN_RETRANSMISSIONS: UCHAR = 0;
STRUCT!{struct TCP_INITIAL_RTO_PARAMETERS {
    Rtt: USHORT,
    MaxSynRetransmissions: UCHAR,
}}
pub type PTCP_INITIAL_RTO_PARAMETERS = *mut TCP_INITIAL_RTO_PARAMETERS;
ENUM!{enum TCP_ICW_LEVEL {
    TCP_ICW_LEVEL_DEFAULT = 0,
    TCP_ICW_LEVEL_HIGH = 1,
    TCP_ICW_LEVEL_VERY_HIGH = 2,
    TCP_ICW_LEVEL_AGGRESSIVE = 3,
    TCP_ICW_LEVEL_EXPERIMENTAL = 4,
    TCP_ICW_LEVEL_COMPAT = 254,
    TCP_ICW_LEVEL_MAX = 255,
}}
pub type PTCP_ICW_LEVEL = *mut TCP_ICW_LEVEL;
STRUCT!{struct TCP_ICW_PARAMETERS {
    Level: TCP_ICW_LEVEL,
}}
pub type PTCP_ICW_PARAMETERS = *mut TCP_ICW_PARAMETERS;
STRUCT!{struct TCP_ACK_FREQUENCY_PARAMETERS {
    TcpDelayedAckFrequency: UCHAR,
}}
pub type PTCP_ACK_FREQUENCY_PARAMETERS = *mut TCP_ACK_FREQUENCY_PARAMETERS;
STRUCT!{struct TCP_INFO_v0 {
    State: TCPSTATE,
    Mss: ULONG,
    ConnectionTimeMs: ULONG64,
    TimestampsEnabled: BOOLEAN,
    RttUs: ULONG,
    MinRttUs: ULONG,
    BytesInFlight: ULONG,
    Cwnd: ULONG,
    SndWnd: ULONG,
    RcvWnd: ULONG,
    RcvBuf: ULONG,
    BytesOut: ULONG64,
    BytesIn: ULONG64,
    BytesReordered: ULONG,
    BytesRetrans: ULONG,
    FastRetrans: ULONG,
    DupAcksIn: ULONG,
    TimeoutEpisodes: ULONG,
    SynRetrans: UCHAR,
}}
pub type PTCP_INFO_v0 = *mut TCP_INFO_v0;
pub const SIO_ACQUIRE_PORT_RESERVATION: DWORD = _WSAIOW!(IOC_VENDOR, 100);
pub const SIO_RELEASE_PORT_RESERVATION: DWORD = _WSAIOW!(IOC_VENDOR, 101);
pub const SIO_ASSOCIATE_PORT_RESERVATION: DWORD = _WSAIOW!(IOC_VENDOR, 102);
STRUCT!{struct INET_PORT_RANGE {
    StartPort: USHORT,
    NumberOfPorts: USHORT,
}}
pub type PINET_PORT_RANGE = *mut INET_PORT_RANGE;
pub type INET_PORT_RESERVATION = INET_PORT_RANGE;
pub type PINET_PORT_RESERVATION = *mut INET_PORT_RANGE;
STRUCT!{struct INET_PORT_RESERVATION_TOKEN {
    Token: ULONG64,
}}
pub type PINET_PORT_RESERVATION_TOKEN = *mut INET_PORT_RESERVATION_TOKEN;
STRUCT!{struct INET_PORT_RESERVATION_INSTANCE {
    Reservation: INET_PORT_RESERVATION,
    Token: INET_PORT_RESERVATION_TOKEN,
}}
pub type PINET_PORT_RESERVATION_INSTANCE = *mut INET_PORT_RESERVATION_INSTANCE;
STRUCT!{struct INET_PORT_RESERVATION_INFORMATION {
    OwningPid: ULONG,
}}
pub type PINET_PORT_RESERVATION_INFORMATION = *mut INET_PORT_RESERVATION_INFORMATION;
pub const SIO_SET_SECURITY: DWORD = _WSAIOW!(IOC_VENDOR, 200);
pub const SIO_QUERY_SECURITY: DWORD = _WSAIORW!(IOC_VENDOR, 201);
pub const SIO_SET_PEER_TARGET_NAME: DWORD = _WSAIOW!(IOC_VENDOR, 202);
pub const SIO_DELETE_PEER_TARGET_NAME: DWORD = _WSAIOW!(IOC_VENDOR, 203);
pub const SIO_QUERY_WFP_CONNECTION_REDIRECT_RECORDS: DWORD = _WSAIOW!(IOC_VENDOR, 220);
pub const SIO_QUERY_WFP_CONNECTION_REDIRECT_CONTEXT: DWORD = _WSAIOW!(IOC_VENDOR, 221);
pub const SIO_SET_WFP_CONNECTION_REDIRECT_RECORDS: DWORD = _WSAIOW!(IOC_VENDOR, 222);
pub const SIO_SOCKET_USAGE_NOTIFICATION: DWORD = _WSAIOW!(IOC_VENDOR, 204);
ENUM!{enum SOCKET_USAGE_TYPE {
    SYSTEM_CRITICAL_SOCKET = 1,
}}
ENUM!{enum SOCKET_SECURITY_PROTOCOL {
    SOCKET_SECURITY_PROTOCOL_DEFAULT,
    SOCKET_SECURITY_PROTOCOL_IPSEC,
    SOCKET_SECURITY_PROTOCOL_IPSEC2,
    SOCKET_SECURITY_PROTOCOL_INVALID,
}}
STRUCT!{struct SOCKET_SECURITY_SETTINGS {
    SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    SecurityFlags: ULONG,
}}
pub const SOCKET_SETTINGS_IPSEC_SKIP_FILTER_INSTANTIATION: ULONG = 0x1;
pub const SOCKET_SETTINGS_IPSEC_OPTIONAL_PEER_NAME_VERIFICATION: ULONG = 0x2;
pub const SOCKET_SETTINGS_IPSEC_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED: ULONG = 0x4;
pub const SOCKET_SETTINGS_IPSEC_PEER_NAME_IS_RAW_FORMAT: ULONG = 0x8;
STRUCT!{struct SOCKET_SECURITY_SETTINGS_IPSEC {
    SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    SecurityFlags: ULONG,
    IpsecFlags: ULONG,
    AuthipMMPolicyKey: GUID,
    AuthipQMPolicyKey: GUID,
    Reserved: GUID,
    Reserved2: UINT64,
    UserNameStringLen: ULONG,
    DomainNameStringLen: ULONG,
    PasswordStringLen: ULONG,
    AllStrings: [wchar_t; 0],
}}
STRUCT!{struct SOCKET_PEER_TARGET_NAME {
    SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    PeerAddress: SOCKADDR_STORAGE,
    PeerTargetNameStringLen: ULONG,
    AllStrings: [wchar_t; 0],
}}
STRUCT!{struct SOCKET_SECURITY_QUERY_TEMPLATE {
    SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    PeerAddress: SOCKADDR_STORAGE,
    PeerTokenAccessMask: ULONG,
}}
pub const SOCKET_QUERY_IPSEC2_ABORT_CONNECTION_ON_FIELD_CHANGE: ULONG = 0x1;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_MM_SA_ID: ULONG = 0x1;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_QM_SA_ID: ULONG = 0x2;
STRUCT!{struct SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    PeerAddress: SOCKADDR_STORAGE,
    PeerTokenAccessMask: ULONG,
    Flags: ULONG,
    FieldMask: ULONG,
}}
pub const SOCKET_INFO_CONNECTION_SECURED: ULONG = 0x1;
pub const SOCKET_INFO_CONNECTION_ENCRYPTED: ULONG = 0x2;
pub const SOCKET_INFO_CONNECTION_IMPERSONATED: ULONG = 0x4;
STRUCT!{struct SOCKET_SECURITY_QUERY_INFO {
    SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    Flags: ULONG,
    PeerApplicationAccessTokenHandle: UINT64,
    PeerMachineAccessTokenHandle: UINT64,
}}
STRUCT!{struct SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    Flags: ULONG,
    PeerApplicationAccessTokenHandle: UINT64,
    PeerMachineAccessTokenHandle: UINT64,
    MmSaId: UINT64,
    QmSaId: UINT64,
    NegotiationWinerr: UINT32,
    SaLookupContext: GUID,
}}
pub const SIO_QUERY_WFP_ALE_ENDPOINT_HANDLE: DWORD = _WSAIOR!(IOC_VENDOR, 205);
pub const SIO_QUERY_RSS_SCALABILITY_INFO: DWORD = _WSAIOR!(IOC_VENDOR, 210);
STRUCT!{struct RSS_SCALABILITY_INFO {
    RssEnabled: BOOLEAN,
}}
pub type PRSS_SCALABILITY_INFO = *mut RSS_SCALABILITY_INFO;
#[inline]
pub fn IN4_CLASSA(i: LONG) -> bool {
    (i & 0x80) == 0
}
#[inline]
pub fn IN4_CLASSB(i: LONG) -> bool {
    (i & 0xc0) == 0x80
}
#[inline]
pub fn IN4_CLASSC(i: LONG) -> bool {
    (i & 0xe0) == 0xc0
}
#[inline]
pub fn IN4_CLASSD(i: LONG) -> bool {
    (i & 0xf0) == 0xe0
}
#[inline]
pub fn IN4_MULTICAST(i: LONG) -> bool {
    IN4_CLASSD(i)
}
pub const IN4ADDR_ANY: ULONG = INADDR_ANY;
pub const IN4ADDR_LOOPBACK: ULONG = 0x0100007f;
pub const IN4ADDR_BROADCAST: ULONG = INADDR_BROADCAST;
pub const IN4ADDR_NONE: ULONG = INADDR_NONE;
pub const IN4ADDR_LOOPBACKPREFIX_LENGTH: usize = 8;
pub const IN4ADDR_LINKLOCALPREFIX_LENGTH: usize = 16;
pub const IN4ADDR_MULTICASTPREFIX_LENGTH: usize = 4;
#[inline]
pub fn IN4_ADDR_EQUAL(a: &IN_ADDR, b: &IN_ADDR) -> bool {
    unsafe { *a.S_un.S_addr() == *b.S_un.S_addr() }
}
#[inline]
pub fn IN4_UNALIGNED_ADDR_EQUAL(a: &IN_ADDR, b: &IN_ADDR) -> bool {
    unsafe { *a.S_un.S_addr() == *b.S_un.S_addr() }
}
#[inline]
pub fn IN4_IS_ADDR_UNSPECIFIED(a: &IN_ADDR) -> bool {
    unsafe { *a.S_un.S_addr() == IN4ADDR_ANY }
}
#[inline]
pub fn IN4_IS_UNALIGNED_ADDR_UNSPECIFIED(a: &IN_ADDR) -> bool {
    unsafe { *a.S_un.S_addr() == IN4ADDR_ANY }
}
#[inline]
pub fn IN4_IS_ADDR_LOOPBACK(a: &IN_ADDR) -> bool {
    unsafe { a.S_un.S_un_b().s_b1 == 0x7f }
}
#[inline]
pub fn IN4_IS_UNALIGNED_ADDR_LOOPBACK(a: &IN_ADDR) -> bool {
    unsafe { a.S_un.S_un_b().s_b1 == 0x7f }
}
#[inline]
pub fn IN4_IS_ADDR_BROADCAST(a: &IN_ADDR) -> bool {
    unsafe { *a.S_un.S_addr() == IN4ADDR_BROADCAST }
}
#[inline]
pub fn IN4_IS_UNALIGNED_ADDR_BROADCAST(a: &IN_ADDR) -> bool {
    unsafe { *a.S_un.S_addr() == IN4ADDR_BROADCAST }
}
#[inline]
pub fn IN4_IS_ADDR_MULTICAST(a: &IN_ADDR) -> bool {
    IN4_MULTICAST(unsafe { *a.S_un.S_addr() as LONG })
}
#[inline]
pub fn IN4_IS_UNALIGNED_ADDR_MULTICAST(a: &IN_ADDR) -> bool {
    IN4_MULTICAST(unsafe { *a.S_un.S_addr() as LONG })
}
#[inline]
pub fn IN4_IS_ADDR_LINKLOCAL(a: &IN_ADDR) -> bool {
    unsafe { (*a.S_un.S_addr() & 0xffff) == 0xfea9 }
}
#[inline]
pub fn IN4_IS_UNALIGNED_ADDR_LINKLOCAL(a: &IN_ADDR) -> bool {
    unsafe { (*a.S_un.S_addr() & 0xffff) == 0xfea9 }
}
#[inline]
pub fn IN4_IS_ADDR_SITELOCAL(_: &IN_ADDR) -> bool {
    false
}
#[inline]
pub fn IN4_IS_UNALIGNED_ADDR_SITELOCAL(_: &IN_ADDR) -> bool {
    false
}
#[inline]
pub fn IN4_IS_ADDR_RFC1918(a: &IN_ADDR) -> bool {
    let s_addr = unsafe { *a.S_un.S_addr() };
    ((s_addr & 0x00ff) == 0x0a) || ((s_addr & 0xf0ff) == 0x10ac) || ((s_addr & 0xffff) == 0xa8c0)
}
#[inline]
pub fn IN4_IS_UNALIGNED_ADDR_RFC1918(a: &IN_ADDR) -> bool {
    IN4_IS_ADDR_RFC1918(a)
}
#[inline]
pub fn IN4_IS_ADDR_MC_LINKLOCAL(a: &IN_ADDR) -> bool {
    unsafe { (*a.S_un.S_addr() & 0xffffff) == 0xe0 }
}
#[inline]
pub fn IN4_IS_ADDR_MC_ADMINLOCAL(a: &IN_ADDR) -> bool {
    unsafe { (*a.S_un.S_addr() & 0xffff) == 0xffef }
}
#[inline]
pub fn IN4_IS_ADDR_MC_SITELOCAL(a: &IN_ADDR) -> bool {
    let first = unsafe { (*a.S_un.S_addr() & 0xff) == 0xef };
    first && !IN4_IS_ADDR_MC_ADMINLOCAL(a)
}
#[inline]
pub fn IN4ADDR_ISANY(a: &SOCKADDR_IN) -> bool {
    IN4_IS_ADDR_UNSPECIFIED(&a.sin_addr)
}
#[inline]
pub fn IN4ADDR_ISLOOPBACK(a: &SOCKADDR_IN) -> bool {
    IN4_IS_ADDR_LOOPBACK(&a.sin_addr)
}
extern "system" {
    pub fn RtlIpv4AddressToStringA(
        Addr: *const IN_ADDR,
        S: PSTR,
    ) -> PSTR;
    pub fn RtlIpv4AddressToStringExA(
        Address: *const IN_ADDR,
        Port: USHORT,
        AddressString: PSTR,
        AddressStringLength: PULONG,
    ) -> LONG;
    pub fn RtlIpv4AddressToStringW(
        Addr: *const IN_ADDR,
        S: PWSTR,
    ) -> PWSTR;
    pub fn RtlIpv4AddressToStringExW(
        Address: *const IN_ADDR,
        Port: USHORT,
        AddressString: PWSTR,
        AddressStringLength: PULONG,
    ) -> LONG;
    pub fn RtlIpv4StringToAddressA(
        S: PCSTR,
        Strict: BOOLEAN,
        Terminator: *mut PCSTR,
        Addr: *mut IN_ADDR,
    ) -> LONG;
    pub fn RtlIpv4StringToAddressExA(
        AddressString: PCSTR,
        Strict: BOOLEAN,
        Address: *mut IN_ADDR,
        Port: PUSHORT,
    ) -> LONG;
    pub fn RtlIpv4StringToAddressW(
        S: PCWSTR,
        Strict: BOOLEAN,
        Terminator: *mut LPCWSTR,
        Addr: *mut IN_ADDR,
    ) -> LONG;
    pub fn RtlIpv4StringToAddressExW(
        AddressString: PCWSTR,
        Strict: BOOLEAN,
        Address: *mut IN_ADDR,
        Port: PUSHORT,
    ) -> LONG;
    pub fn RtlIpv6AddressToStringA(
        Addr: *const IN6_ADDR,
        S: PSTR,
    ) -> PSTR;
    pub fn RtlIpv6AddressToStringExA(
        Address: *const IN6_ADDR,
        ScopeId: ULONG,
        Port: USHORT,
        AddressString: PSTR,
        AddressStringLength: PULONG,
    ) -> LONG;
    pub fn RtlIpv6AddressToStringW(
        Addr: *const IN6_ADDR,
        S: PWSTR,
    ) -> PWSTR;
    pub fn RtlIpv6AddressToStringExW(
        Address: *const IN6_ADDR,
        ScopeId: ULONG,
        Port: USHORT,
        AddressString: PWSTR,
        AddressStringLength: PULONG,
    ) -> LONG;
    pub fn RtlIpv6StringToAddressA(
        S: PCSTR,
        Terminator: *mut PCSTR,
        Addr: *mut IN6_ADDR,
    ) -> LONG;
    pub fn RtlIpv6StringToAddressExA(
        AddressString: PCSTR,
        Address: *mut IN6_ADDR,
        ScopeId: PULONG,
        Port: PUSHORT,
    ) -> LONG;
    pub fn RtlIpv6StringToAddressW(
        S: PCWSTR,
        Terminator: *mut PCWSTR,
        Addr: *mut IN6_ADDR,
    ) -> LONG;
    pub fn RtlIpv6StringToAddressExW(
        AddressString: PCWSTR,
        Address: *mut IN6_ADDR,
        ScopeId: PULONG,
        Port: PUSHORT,
    ) -> LONG;
}
DECLARE_HANDLE!{DL_EUI48, _DL_EUI48}
pub type PDL_EUI48 = *mut DL_EUI48;
extern "system" {
    pub fn RtlEthernetAddressToStringA(
        Addr: *const DL_EUI48,
        S: PSTR,
    ) -> PSTR;
    pub fn RtlEthernetAddressToStringW(
        Addr: *const DL_EUI48,
        S: PWSTR,
    ) -> PWSTR;
    pub fn RtlEthernetStringToAddressA(
        S: PCSTR,
        Terminator: *mut PCSTR,
        Addr: *mut DL_EUI48,
    ) -> LONG;
    pub fn RtlEthernetStringToAddressW(
        S: PCWSTR,
        Terminator: *mut LPCWSTR,
        Addr: *mut DL_EUI48,
    ) -> LONG;
}
