// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! TCP/IP specific information for use by WinSock2 compatible applications.
use ctypes::c_int;
use shared::in6addr::IN6_ADDR;
use shared::inaddr::IN_ADDR;
use shared::minwindef::{ULONG, USHORT};
use shared::ws2def::{ADDRESS_FAMILY, SCOPE_ID, SOCKADDR_IN};
pub const IFF_UP: ULONG = 0x00000001;
pub const IFF_BROADCAST: ULONG = 0x00000002;
pub const IFF_LOOPBACK: ULONG = 0x00000004;
pub const IFF_POINTTOPOINT: ULONG = 0x00000008;
pub const IFF_MULTICAST: ULONG = 0x00000010;
pub const IP_OPTIONS: c_int = 1;
pub const IP_HDRINCL: c_int = 2;
pub const IP_TOS: c_int = 3;
pub const IP_TTL: c_int = 4;
pub const IP_MULTICAST_IF: c_int = 9;
pub const IP_MULTICAST_TTL: c_int = 10;
pub const IP_MULTICAST_LOOP: c_int = 11;
pub const IP_ADD_MEMBERSHIP: c_int = 12;
pub const IP_DROP_MEMBERSHIP: c_int = 13;
pub const IP_DONTFRAGMENT: c_int = 14;
pub const IP_ADD_SOURCE_MEMBERSHIP: c_int = 15;
pub const IP_DROP_SOURCE_MEMBERSHIP: c_int = 16;
pub const IP_BLOCK_SOURCE: c_int = 17;
pub const IP_UNBLOCK_SOURCE: c_int = 18;
pub const IP_PKTINFO: c_int = 19;
pub const IP_RECEIVE_BROADCAST: c_int = 22;
pub const IP_RECVDSTADDR: c_int = 25;
UNION!{union SOCKADDR_IN6_LH_u {
    [u32; 1],
    sin6_scope_id sin6_scope_id_mut: ULONG,
    sin6_scope_struct sin6_scope_struct_mut: SCOPE_ID,
}}
STRUCT!{struct SOCKADDR_IN6_LH {
    sin6_family: ADDRESS_FAMILY,
    sin6_port: USHORT,
    sin6_flowinfo: ULONG,
    sin6_addr: IN6_ADDR,
    u: SOCKADDR_IN6_LH_u,
}}
pub type PSOCKADDR_IN6_LH = *mut SOCKADDR_IN6_LH;
pub type SOCKADDR_IN6 = SOCKADDR_IN6_LH;
pub type PSOCKADDR_IN6 = *mut SOCKADDR_IN6;
STRUCT!{struct SOCKADDR_IN6_PAIR {
    SourceAddress: PSOCKADDR_IN6,
    DestinationAddress: PSOCKADDR_IN6,
}}
pub type PSOCKADDR_IN6_PAIR = *mut SOCKADDR_IN6_PAIR;
UNION!{union SOCKADDR_INET {
    [u32; 7],
    Ipv4 Ipv4_mut: SOCKADDR_IN,
    Ipv6 Ipv6_mut: SOCKADDR_IN6,
    si_family si_family_mut: ADDRESS_FAMILY,
}}
pub type PSOCKADDR_INET = *mut SOCKADDR_INET;
STRUCT!{struct IP_MREQ {
    imr_multiaddr: IN_ADDR,
    imr_interface: IN_ADDR,
}}
pub type PIP_MREQ = *mut IP_MREQ;
STRUCT!{struct IP_MREQ_SOURCE {
    imr_multiaddr: IN_ADDR,
    imr_sourceaddr: IN_ADDR,
    imr_interface: IN_ADDR,
}}
pub type PIP_MREQ_SOURCE = *mut IP_MREQ_SOURCE;
pub const IPV6_HOPOPTS: c_int = 1;
pub const IPV6_HDRINCL: c_int = 2;
pub const IPV6_UNICAST_HOPS: c_int = 4;
pub const IPV6_MULTICAST_IF: c_int = 9;
pub const IPV6_MULTICAST_HOPS: c_int = 10;
pub const IPV6_MULTICAST_LOOP: c_int = 11;
pub const IPV6_ADD_MEMBERSHIP: c_int = 12;
pub const IPV6_JOIN_GROUP: c_int = IPV6_ADD_MEMBERSHIP;
pub const IPV6_DROP_MEMBERSHIP: c_int = 13;
pub const IPV6_LEAVE_GROUP: c_int = IPV6_DROP_MEMBERSHIP;
pub const IPV6_DONTFRAG: c_int = 14;
pub const IPV6_PKTINFO: c_int = 19;
pub const IPV6_HOPLIMIT: c_int = 21;
pub const IPV6_PROTECTION_LEVEL: c_int = 23;
pub const IPV6_RECVIF: c_int = 24;
pub const IPV6_RECVDSTADDR: c_int = 25;
pub const IPV6_CHECKSUM: c_int = 26;
pub const IPV6_V6ONLY: c_int = 27;
pub const IPV6_IFLIST: c_int = 28;
pub const IPV6_ADD_IFLIST: c_int = 29;
pub const IPV6_DEL_IFLIST: c_int = 30;
pub const IPV6_UNICAST_IF: c_int = 31;
pub const IPV6_RTHDR: c_int = 32;
pub const IPV6_RECVRTHDR: c_int = 38;
pub const IPV6_TCLASS: c_int = 39;
pub const IPV6_RECVTCLASS: c_int = 40;
STRUCT!{struct IPV6_MREQ {
    ipv6mr_multiaddr: IN6_ADDR,
    ipv6mr_interface: ULONG,
}}
pub type PIPV6_MREQ = *mut IPV6_MREQ;
STRUCT!{struct IN_PKTINFO {
    ipi_addr: IN_ADDR,
    ipi_ifindex: ULONG,
}}
pub type PIN_PKTINFO = *mut IN_PKTINFO;
STRUCT!{struct IN6_PKTINFO {
    ipi6_addr: IN6_ADDR,
    ipi6_ifindex: ULONG,
}}
pub type PIN6_PKTINFO = *mut IN6_PKTINFO;
