// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// #include <winapifamily.h>
// #include <in6addr.h>
// #include <inaddr.h>
use shared::basetsd::ULONG64;
use shared::in6addr::in6_addr;
use shared::ntdef::{INT, PUCHAR, PVOID, UCHAR, ULONG, USHORT, WCHAR};
pub const MAX_ADAPTER_NAME: usize = 128;
pub const MAX_OPT_SIZE: usize = 40;
pub type IPAddr = ULONG;
pub type IPMask = ULONG;
pub type IP_STATUS = ULONG;
pub type IPv6Addr = in6_addr;
STRUCT!{struct IP_OPTION_INFORMATION {
    Ttl: UCHAR,
    Tos: UCHAR,
    Flags: UCHAR,
    OptionsSize: UCHAR,
    OptionsData: PUCHAR,
}}
pub type PIP_OPTION_INFORMATION = *mut IP_OPTION_INFORMATION;
#[cfg(target_arch = "x86_64")]
STRUCT!{struct IP_OPTION_INFORMATION32 {
    Ttl: UCHAR,
    Tos: UCHAR,
    Flags: UCHAR,
    OptionsSize: UCHAR,
    OptionsData: u32, // UCHAR * POINTER_32
}}
#[cfg(target_arch = "x86_64")]
pub type PIP_OPTION_INFORMATION32 = *mut IP_OPTION_INFORMATION32;
STRUCT!{struct ICMP_ECHO_REPLY {
    Address: IPAddr,
    Status: ULONG,
    RoundTripTime: ULONG,
    DataSize: USHORT,
    Reserved: USHORT,
    Data: PVOID,
    Options: IP_OPTION_INFORMATION,
}}
pub type PICMP_ECHO_REPLY = *mut ICMP_ECHO_REPLY;
#[cfg(target_arch = "x86_64")]
STRUCT!{struct ICMP_ECHO_REPLY32 {
    Address: IPAddr,
    Status: ULONG,
    RoundTripTime: ULONG,
    DataSize: USHORT,
    Reserved: USHORT,
    Data: u32, // VOID * POINTER_32
    Options: IP_OPTION_INFORMATION32,
}}
#[cfg(target_arch = "x86_64")]
pub type PICMP_ECHO_REPLY32 = *mut ICMP_ECHO_REPLY32;
STRUCT!{#[repr(packed)] struct IPV6_ADDRESS_EX {
    sin6_port: USHORT,
    sin6_flowinfo: ULONG,
    sin6_addr: [USHORT; 8],
    sin6_scope_id: ULONG,
}}
pub type PIPV6_ADDRESS_EX = *mut IPV6_ADDRESS_EX;
// #include <packoff.h>
STRUCT!{struct ICMPV6_ECHO_REPLY_LH {
    Address: IPV6_ADDRESS_EX,
    Status: ULONG,
    RoundTripTime: INT,
}}
pub type PICMPV6_ECHO_REPLY_LH = *mut ICMPV6_ECHO_REPLY_LH;
pub type ICMPV6_ECHO_REPLY = ICMPV6_ECHO_REPLY_LH;
pub type PICMPV6_ECHO_REPLY = *mut ICMPV6_ECHO_REPLY;
// #endif
STRUCT!{struct ARP_SEND_REPLY {
    DestAddress: IPAddr,
    SrcAddress: IPAddr,
}}
pub type PARP_SEND_REPLY = *mut ARP_SEND_REPLY;
STRUCT!{struct TCP_RESERVE_PORT_RANGE {
    UpperRange: USHORT,
    LowerRange: USHORT,
}}
pub type PTCP_RESERVE_PORT_RANGE = *mut TCP_RESERVE_PORT_RANGE;
STRUCT!{struct IP_ADAPTER_INDEX_MAP {
    Index: ULONG,
    Name: [WCHAR; MAX_ADAPTER_NAME],
}}
pub type PIP_ADAPTER_INDEX_MAP = *mut IP_ADAPTER_INDEX_MAP;
STRUCT!{struct IP_INTERFACE_INFO {
    NumAdapters: ULONG,
    Adapter: [IP_ADAPTER_INDEX_MAP; 1],
}}
pub type PIP_INTERFACE_INFO = *mut IP_INTERFACE_INFO;
STRUCT!{struct IP_UNIDIRECTIONAL_ADAPTER_ADDRESS {
    NumAdapters: ULONG,
    Address: [IPAddr; 1],
}}
pub type PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS = *mut IP_UNIDIRECTIONAL_ADAPTER_ADDRESS;
STRUCT!{struct IP_ADAPTER_ORDER_MAP {
    NumAdapters: ULONG,
    AdapterOrder: [ULONG; 1],
}}
pub type PIP_ADAPTER_ORDER_MAP = *mut IP_ADAPTER_ORDER_MAP;
STRUCT!{struct IP_MCAST_COUNTER_INFO {
    InMcastOctets: ULONG64,
    OutMcastOctets: ULONG64,
    InMcastPkts: ULONG64,
    OutMcastPkts: ULONG64,
}}
pub type PIP_MCAST_COUNTER_INFO = *mut IP_MCAST_COUNTER_INFO;
// IP_STATUS codes returned from IP APIs
pub const IP_STATUS_BASE: IP_STATUS = 11000;
pub const IP_SUCCESS: IP_STATUS = 0;
pub const IP_BUF_TOO_SMALL: IP_STATUS = IP_STATUS_BASE + 1;
pub const IP_DEST_NET_UNREACHABLE: IP_STATUS = IP_STATUS_BASE + 2;
pub const IP_DEST_HOST_UNREACHABLE: IP_STATUS = IP_STATUS_BASE + 3;
pub const IP_DEST_PROT_UNREACHABLE: IP_STATUS = IP_STATUS_BASE + 4;
pub const IP_DEST_PORT_UNREACHABLE: IP_STATUS = IP_STATUS_BASE + 5;
pub const IP_NO_RESOURCES: IP_STATUS = IP_STATUS_BASE + 6;
pub const IP_BAD_OPTION: IP_STATUS = IP_STATUS_BASE + 7;
pub const IP_HW_ERROR: IP_STATUS = IP_STATUS_BASE + 8;
pub const IP_PACKET_TOO_BIG: IP_STATUS = IP_STATUS_BASE + 9;
pub const IP_REQ_TIMED_OUT: IP_STATUS = IP_STATUS_BASE + 10;
pub const IP_BAD_REQ: IP_STATUS = IP_STATUS_BASE + 11;
pub const IP_BAD_ROUTE: IP_STATUS = IP_STATUS_BASE + 12;
pub const IP_TTL_EXPIRED_TRANSIT: IP_STATUS = IP_STATUS_BASE + 13;
pub const IP_TTL_EXPIRED_REASSEM: IP_STATUS = IP_STATUS_BASE + 14;
pub const IP_PARAM_PROBLEM: IP_STATUS = IP_STATUS_BASE + 15;
pub const IP_SOURCE_QUENCH: IP_STATUS = IP_STATUS_BASE + 16;
pub const IP_OPTION_TOO_BIG: IP_STATUS = IP_STATUS_BASE + 17;
pub const IP_BAD_DESTINATION: IP_STATUS = IP_STATUS_BASE + 18;
pub const IP_DEST_NO_ROUTE: IP_STATUS = IP_STATUS_BASE + 2;
pub const IP_DEST_ADDR_UNREACHABLE: IP_STATUS = IP_STATUS_BASE + 3;
pub const IP_DEST_PROHIBITED: IP_STATUS = IP_STATUS_BASE + 4;
pub const IP_HOP_LIMIT_EXCEEDED: IP_STATUS = IP_STATUS_BASE + 13;
pub const IP_REASSEMBLY_TIME_EXCEEDED: IP_STATUS = IP_STATUS_BASE + 14;
pub const IP_PARAMETER_PROBLEM: IP_STATUS = IP_STATUS_BASE + 15;
pub const IP_DEST_UNREACHABLE: IP_STATUS = IP_STATUS_BASE + 40;
pub const IP_TIME_EXCEEDED: IP_STATUS = IP_STATUS_BASE + 41;
pub const IP_BAD_HEADER: IP_STATUS = IP_STATUS_BASE + 42;
pub const IP_UNRECOGNIZED_NEXT_HEADER: IP_STATUS = IP_STATUS_BASE + 43;
pub const IP_ICMP_ERROR: IP_STATUS = IP_STATUS_BASE + 44;
pub const IP_DEST_SCOPE_MISMATCH: IP_STATUS = IP_STATUS_BASE + 45;
pub const IP_ADDR_DELETED: IP_STATUS = IP_STATUS_BASE + 19;
pub const IP_SPEC_MTU_CHANGE: IP_STATUS = IP_STATUS_BASE + 20;
pub const IP_MTU_CHANGE: IP_STATUS = IP_STATUS_BASE + 21;
pub const IP_UNLOAD: IP_STATUS = IP_STATUS_BASE + 22;
pub const IP_ADDR_ADDED: IP_STATUS = IP_STATUS_BASE + 23;
pub const IP_MEDIA_CONNECT: IP_STATUS = IP_STATUS_BASE + 24;
pub const IP_MEDIA_DISCONNECT: IP_STATUS = IP_STATUS_BASE + 25;
pub const IP_BIND_ADAPTER: IP_STATUS = IP_STATUS_BASE + 26;
pub const IP_UNBIND_ADAPTER: IP_STATUS = IP_STATUS_BASE + 27;
pub const IP_DEVICE_DOES_NOT_EXIST: IP_STATUS = IP_STATUS_BASE + 28;
pub const IP_DUPLICATE_ADDRESS: IP_STATUS = IP_STATUS_BASE + 29;
pub const IP_INTERFACE_METRIC_CHANGE: IP_STATUS = IP_STATUS_BASE + 30;
pub const IP_RECONFIG_SECFLTR: IP_STATUS = IP_STATUS_BASE + 31;
pub const IP_NEGOTIATING_IPSEC: IP_STATUS = IP_STATUS_BASE + 32;
pub const IP_INTERFACE_WOL_CAPABILITY_CHANGE: IP_STATUS = IP_STATUS_BASE + 33;
pub const IP_DUPLICATE_IPADD: IP_STATUS = IP_STATUS_BASE + 34;
pub const IP_GENERAL_FAILURE: IP_STATUS = IP_STATUS_BASE + 50;
pub const MAX_IP_STATUS: IP_STATUS = IP_GENERAL_FAILURE;
pub const IP_PENDING: IP_STATUS = IP_STATUS_BASE + 255;
pub const IP_FLAG_REVERSE: UCHAR = 0x1;
pub const IP_FLAG_DF: UCHAR = 0x2;
pub const IP_OPT_EOL: u8 = 0;
pub const IP_OPT_NOP: u8 = 1;
pub const IP_OPT_SECURITY: u8 = 0x82;
pub const IP_OPT_LSRR: u8 = 0x83;
pub const IP_OPT_SSRR: u8 = 0x89;
pub const IP_OPT_RR: u8 = 0x7;
pub const IP_OPT_TS: u8 = 0x44;
pub const IP_OPT_SID: u8 = 0x88;
pub const IP_OPT_ROUTER_ALERT: u8 = 0x94;
