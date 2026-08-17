// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::ULONG64;
use shared::minwindef::ULONG;
use shared::ntdef::BOOLEAN;
ENUM!{enum NL_PREFIX_ORIGIN {
    IpPrefixOriginOther = 0,
    IpPrefixOriginManual,
    IpPrefixOriginWellKnown,
    IpPrefixOriginDhcp,
    IpPrefixOriginRouterAdvertisement,
    IpPrefixOriginUnchanged = 1 << 4,
}}
pub const NlpoOther: NL_PREFIX_ORIGIN = IpPrefixOriginOther;
pub const NlpoManual: NL_PREFIX_ORIGIN = IpPrefixOriginManual;
pub const NlpoWellKnown: NL_PREFIX_ORIGIN = IpPrefixOriginWellKnown;
pub const NlpoDhcp: NL_PREFIX_ORIGIN = IpPrefixOriginDhcp;
pub const NlpoRouterAdvertisement: NL_PREFIX_ORIGIN = IpPrefixOriginRouterAdvertisement;
ENUM!{enum NL_SUFFIX_ORIGIN {
    NlsoOther = 0,
    NlsoManual,
    NlsoWellKnown,
    NlsoDhcp,
    NlsoLinkLayerAddress,
    NlsoRandom,
    IpSuffixOriginOther = 0,
    IpSuffixOriginManual,
    IpSuffixOriginWellKnown,
    IpSuffixOriginDhcp,
    IpSuffixOriginLinkLayerAddress,
    IpSuffixOriginRandom,
    IpSuffixOriginUnchanged = 1 << 4,
}}
ENUM!{enum NL_DAD_STATE {
    NldsInvalid,
    NldsTentative,
    NldsDuplicate,
    NldsDeprecated,
    NldsPreferred,
    IpDadStateInvalid = 0,
    IpDadStateTentative,
    IpDadStateDuplicate,
    IpDadStateDeprecated,
    IpDadStatePreferred,
}}
pub const NL_MAX_METRIC_COMPONENT: ULONG = (1u32 << 31) - 1;
ENUM!{enum NL_ROUTE_PROTOCOL {
    RouteProtocolOther = 1,
    RouteProtocolLocal = 2,
    RouteProtocolNetMgmt = 3,
    RouteProtocolIcmp = 4,
    RouteProtocolEgp = 5,
    RouteProtocolGgp = 6,
    RouteProtocolHello = 7,
    RouteProtocolRip = 8,
    RouteProtocolIsIs = 9,
    RouteProtocolEsIs = 10,
    RouteProtocolCisco = 11,
    RouteProtocolBbn = 12,
    RouteProtocolOspf = 13,
    RouteProtocolBgp = 14,
    RouteProtocolIdpr = 15,
    RouteProtocolEigrp = 16,
    RouteProtocolDvmrp = 17,
    RouteProtocolRpl = 18,
    RouteProtocolDhcp = 19,
    MIB_IPPROTO_OTHER = 1,
    PROTO_IP_OTHER = 1,
    MIB_IPPROTO_LOCAL = 2,
    PROTO_IP_LOCAL = 2,
    MIB_IPPROTO_NETMGMT = 3,
    PROTO_IP_NETMGMT = 3,
    MIB_IPPROTO_ICMP = 4,
    PROTO_IP_ICMP = 4,
    MIB_IPPROTO_EGP = 5,
    PROTO_IP_EGP = 5,
    MIB_IPPROTO_GGP = 6,
    PROTO_IP_GGP = 6,
    MIB_IPPROTO_HELLO = 7,
    PROTO_IP_HELLO = 7,
    MIB_IPPROTO_RIP = 8,
    PROTO_IP_RIP = 8,
    MIB_IPPROTO_IS_IS = 9,
    PROTO_IP_IS_IS = 9,
    MIB_IPPROTO_ES_IS = 10,
    PROTO_IP_ES_IS = 10,
    MIB_IPPROTO_CISCO = 11,
    PROTO_IP_CISCO = 11,
    MIB_IPPROTO_BBN = 12,
    PROTO_IP_BBN = 12,
    MIB_IPPROTO_OSPF = 13,
    PROTO_IP_OSPF = 13,
    MIB_IPPROTO_BGP = 14,
    PROTO_IP_BGP = 14,
    MIB_IPPROTO_IDPR = 15,
    PROTO_IP_IDPR = 15,
    MIB_IPPROTO_EIGRP = 16,
    PROTO_IP_EIGRP = 16,
    MIB_IPPROTO_DVMRP = 17,
    PROTO_IP_DVMRP = 17,
    MIB_IPPROTO_RPL = 18,
    PROTO_IP_RPL = 18,
    MIB_IPPROTO_DHCP = 19,
    PROTO_IP_DHCP = 19,
    MIB_IPPROTO_NT_AUTOSTATIC = 10002,
    PROTO_IP_NT_AUTOSTATIC = 10002,
    MIB_IPPROTO_NT_STATIC = 10006,
    PROTO_IP_NT_STATIC = 10006,
    MIB_IPPROTO_NT_STATIC_NON_DOD = 10007,
    PROTO_IP_NT_STATIC_NON_DOD = 10007,
}}
pub type PNL_ROUTE_PROTOCOL = *mut NL_ROUTE_PROTOCOL;
ENUM!{enum NL_ADDRESS_TYPE {
    NlatUnspecified = 0,
    NlatUnicast = 1,
    NlatAnycast = 2,
    NlatMulticast = 3,
    NlatBroadcast = 4,
    NlatInvalid = 5,
}}
pub type PNL_ADDRESS_TYPE = *mut NL_ADDRESS_TYPE;
ENUM!{enum NL_ROUTE_ORIGIN {
    NlroManual = 0,
    NlroWellKnown = 1,
    NlroDHCP = 2,
    NlroRouterAdvertisement = 3,
    Nlro6to4 = 4,
}}
pub type PNL_ROUTE_ORIGIN = *mut NL_ROUTE_ORIGIN;
ENUM!{enum NL_NEIGHBOR_STATE {
    NlnsUnreachable = 0,
    NlnsIncomplete = 1,
    NlnsProbe = 2,
    NlnsDelay = 3,
    NlnsStale = 4,
    NlnsReachable = 5,
    NlnsPermanent = 6,
    NlnsMaximum = 7,
}}
pub type PNL_NEIGHBOR_STATE = *mut NL_NEIGHBOR_STATE;
ENUM!{enum NL_LINK_LOCAL_ADDRESS_BEHAVIOR {
    LinkLocalAlwaysOff = 0,
    LinkLocalDelayed = 1,
    LinkLocalAlwaysOn = 2,
    LinkLocalUnchanged = -1i32 as u32,
}}
STRUCT!{struct NL_INTERFACE_OFFLOAD_ROD {
    bitfield: BOOLEAN,
}}
BITFIELD!{NL_INTERFACE_OFFLOAD_ROD bitfield: BOOLEAN [
    NlChecksumSupported set_NlChecksumSupported[0..1],
    NlOptionsSupported set_NlOptionsSupported[1..2],
    TlDatagramChecksumSupported set_TlDatagramChecksumSupported[2..3],
    TlStreamChecksumSupported set_TlStreamChecksumSupported[3..4],
    TlStreamOptionsSupported set_TlStreamOptionsSupported[4..5],
    FastPathCompatible set_FastPathCompatible[5..6],
    TlLargeSendOffloadSupported set_TlLargeSendOffloadSupported[6..7],
    TlGiantSendOffloadSupported set_TlGiantSendOffloadSupported[7..8],
]}
pub type PNL_INTERFACE_OFFLOAD_ROD = *mut NL_INTERFACE_OFFLOAD_ROD;
ENUM!{enum NL_ROUTER_DISCOVERY_BEHAVIOR {
    RouterDiscoveryDisabled = 0,
    RouterDiscoveryEnabled = 1,
    RouterDiscoveryDhcp = 2,
    RouterDiscoveryUnchanged = -1i32 as u32,
}}
ENUM!{enum NL_BANDWIDTH_FLAG {
    NlbwDisabled = 0,
    NlbwEnabled = 1,
    NlbwUnchanged = -1i32 as u32,
}}
pub type PNL_BANDWIDTH_FLAG = *mut NL_BANDWIDTH_FLAG;
STRUCT!{struct NL_PATH_BANDWIDTH_ROD {
    Bandwidth: ULONG64,
    Instability: ULONG64,
    BandwidthPeaked: BOOLEAN,
}}
pub type PNL_PATH_BANDWIDTH_ROD = *mut NL_PATH_BANDWIDTH_ROD;
ENUM!{enum NL_NETWORK_CATEGORY {
    NetworkCategoryPublic = 0,
    NetworkCategoryPrivate = 1,
    NetworkCategoryDomainAuthenticated = 2,
    NetworkCategoryUnchanged = -1i32 as u32,
    NetworkCategoryUnknown = -1i32 as u32,
}}
pub type PNL_NETWORK_CATEGORY = *mut NL_NETWORK_CATEGORY;
ENUM!{enum NL_INTERFACE_NETWORK_CATEGORY_STATE {
    NlincCategoryUnknown = 0,
    NlincPublic = 1,
    NlincPrivate = 2,
    NlincDomainAuthenticated = 3,
    NlincCategoryStateMax = 4,
}}
pub type PNL_INTERFACE_NETWORK_CATEGORY_STATE = *mut NL_INTERFACE_NETWORK_CATEGORY_STATE;
pub const NET_IF_CURRENT_SESSION: ULONG = -1i32 as u32;
STRUCT!{struct NL_BANDWIDTH_INFORMATION {
    Bandwidth: ULONG64,
    Instability: ULONG64,
    BandwidthPeaked: BOOLEAN,
}}
pub type PNL_BANDWIDTH_INFORMATION = *mut NL_BANDWIDTH_INFORMATION;
