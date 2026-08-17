// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::ifdef::IF_INDEX;
use shared::ifmib::MAXLEN_PHYSADDR;
use shared::minwindef::DWORD;
use shared::nldef::NL_ROUTE_PROTOCOL;
use shared::ntdef::{PVOID, UCHAR, ULONG, USHORT};
const ANY_SIZE: usize = 1;
STRUCT!{struct MIB_IPADDRROW_XP {
    dwAddr: DWORD,
    dwIndex: IF_INDEX,
    dwMask: DWORD,
    dwBCastAddr: DWORD,
    dwReasmSize: DWORD,
    unused1: USHORT,
    wType: USHORT,
}}
pub type PMIB_IPADDRROW_XP = *mut MIB_IPADDRROW_XP;
STRUCT!{struct MIB_IPADDRROW_W2K {
    dwAddr: DWORD,
    dwIndex: DWORD,
    dwMask: DWORD,
    dwBCastAddr: DWORD,
    dwReasmSize: DWORD,
    unused1: USHORT,
    unused2: USHORT,
}}
pub type PMIB_IPADDRROW_W2K = *mut MIB_IPADDRROW_W2K;
pub type MIB_IPADDRROW = MIB_IPADDRROW_XP;
pub type PMIB_IPADDRROW = *mut MIB_IPADDRROW;
STRUCT!{struct MIB_IPADDRTABLE {
    dwNumEntries: DWORD,
    table: [MIB_IPADDRROW; ANY_SIZE],
}}
pub type PMIB_IPADDRTABLE = *mut MIB_IPADDRTABLE;
// FIXME: SIZEOF_IPADDRTABLE(x)
STRUCT!{struct MIB_IPFORWARDNUMBER {
    dwValue: DWORD,
}}
pub type PMIB_IPFORWARDNUMBER = *mut MIB_IPFORWARDNUMBER;
pub type MIB_IPFORWARD_PROTO = NL_ROUTE_PROTOCOL;
ENUM!{enum MIB_IPFORWARD_TYPE {
    MIB_IPROUTE_TYPE_OTHER = 1,
    MIB_IPROUTE_TYPE_INVALID = 2,
    MIB_IPROUTE_TYPE_DIRECT = 3,
    MIB_IPROUTE_TYPE_INDIRECT = 4,
}}
STRUCT!{struct MIB_IPFORWARDROW {
    dwForwardDest: DWORD,
    dwForwardMask: DWORD,
    dwForwardPolicy: DWORD,
    dwForwardNextHop: DWORD,
    dwForwardIfIndex: IF_INDEX,
    ForwardType: MIB_IPFORWARD_TYPE,
    ForwardProto: MIB_IPFORWARD_PROTO,
    dwForwardAge: DWORD,
    dwForwardNextHopAS: DWORD,
    dwForwardMetric1: DWORD,
    dwForwardMetric2: DWORD,
    dwForwardMetric3: DWORD,
    dwForwardMetric4: DWORD,
    dwForwardMetric5: DWORD,
}}
pub type PMIB_IPFORWARDROW = *mut MIB_IPFORWARDROW;
STRUCT!{struct MIB_IPFORWARDTABLE {
    dwNumEntries: DWORD,
    table: [MIB_IPFORWARDROW; ANY_SIZE],
}}
pub type PMIB_IPFORWARDTABLE = *mut MIB_IPFORWARDTABLE;
// FIXME: SIZEOF_IPFORWARDTABLE(x)
ENUM!{enum MIB_IPNET_TYPE {
    MIB_IPNET_TYPE_OTHER = 1,
    MIB_IPNET_TYPE_INVALID = 2,
    MIB_IPNET_TYPE_DYNAMIC = 3,
    MIB_IPNET_TYPE_STATIC = 4,
}}
STRUCT!{struct MIB_IPNETROW_LH {
    dwIndex: IF_INDEX,
    dwPhysAddrLen: DWORD,
    bPhysAddr: [UCHAR; MAXLEN_PHYSADDR],
    dwAddr: DWORD,
    Type: MIB_IPNET_TYPE,
}}
pub type PMIB_IPNETROW_LH = *mut MIB_IPNETROW_LH;
STRUCT!{struct MIB_IPNETROW_W2K {
    dwIndex: IF_INDEX,
    dwPhysAddrLen: DWORD,
    bPhysAddr: [UCHAR; MAXLEN_PHYSADDR],
    dwAddr: DWORD,
    dwType: DWORD,
}}
pub type PMIB_IPNETROW_W2K = *mut MIB_IPNETROW_W2K;
pub type MIB_IPNETROW = MIB_IPNETROW_LH;
pub type PMIB_IPNETROW = *mut MIB_IPNETROW;
STRUCT!{struct MIB_IPNETTABLE {
    dwNumEntries: DWORD,
    table: [MIB_IPNETROW; ANY_SIZE],
}}
pub type PMIB_IPNETTABLE = *mut MIB_IPNETTABLE;
// FIXME: SIZEOF_IPNETTABLE(x)
ENUM!{enum MIB_IPSTATS_FORWARDING {
    MIB_IP_FORWARDING = 1,
    MIB_IP_NOT_FORWARDING = 2,
}}
pub type PMIB_IPSTATS_FORWARDING = *mut MIB_IPSTATS_FORWARDING;
STRUCT!{struct MIB_IPSTATS_LH {
    Forwarding: MIB_IPSTATS_FORWARDING,
    dwDefaultTTL: DWORD,
    dwInReceives: DWORD,
    dwInHdrErrors: DWORD,
    dwInAddrErrors: DWORD,
    dwForwDatagrams: DWORD,
    dwInUnknownProtos: DWORD,
    dwInDiscards: DWORD,
    dwInDelivers: DWORD,
    dwOutRequests: DWORD,
    dwRoutingDiscards: DWORD,
    dwOutDiscards: DWORD,
    dwOutNoRoutes: DWORD,
    dwReasmTimeout: DWORD,
    dwReasmReqds: DWORD,
    dwReasmOks: DWORD,
    dwReasmFails: DWORD,
    dwFragOks: DWORD,
    dwFragFails: DWORD,
    dwFragCreates: DWORD,
    dwNumIf: DWORD,
    dwNumAddr: DWORD,
    dwNumRoutes: DWORD,
}}
pub type PMIB_IPSTATS_LH = *mut MIB_IPSTATS_LH;
STRUCT!{struct MIB_IPSTATS_W2K {
    dwForwarding: DWORD,
    dwDefaultTTL: DWORD,
    dwInReceives: DWORD,
    dwInHdrErrors: DWORD,
    dwInAddrErrors: DWORD,
    dwForwDatagrams: DWORD,
    dwInUnknownProtos: DWORD,
    dwInDiscards: DWORD,
    dwInDelivers: DWORD,
    dwOutRequests: DWORD,
    dwRoutingDiscards: DWORD,
    dwOutDiscards: DWORD,
    dwOutNoRoutes: DWORD,
    dwReasmTimeout: DWORD,
    dwReasmReqds: DWORD,
    dwReasmOks: DWORD,
    dwReasmFails: DWORD,
    dwFragOks: DWORD,
    dwFragFails: DWORD,
    dwFragCreates: DWORD,
    dwNumIf: DWORD,
    dwNumAddr: DWORD,
    dwNumRoutes: DWORD,
}}
pub type PMIB_IPSTATS_W2K = *mut MIB_IPSTATS_W2K;
pub type MIB_IPSTATS = MIB_IPSTATS_LH;
pub type PMIB_IPSTATS = *mut MIB_IPSTATS;
STRUCT!{struct MIBICMPSTATS {
    dwMsgs: DWORD,
    dwErrors: DWORD,
    dwDestUnreachs: DWORD,
    dwTimeExcds: DWORD,
    dwParmProbs: DWORD,
    dwSrcQuenchs: DWORD,
    dwRedirects: DWORD,
    dwEchos: DWORD,
    dwEchoReps: DWORD,
    dwTimestamps: DWORD,
    dwTimestampReps: DWORD,
    dwAddrMasks: DWORD,
    dwAddrMaskReps: DWORD,
}}
pub type PMIBICMPSTATS = *mut MIBICMPSTATS;
STRUCT!{struct MIBICMPINFO {
    icmpInStats: MIBICMPSTATS,
    icmpOutStats: MIBICMPSTATS,
}}
STRUCT!{struct MIB_ICMP {
    stats: MIBICMPINFO,
}}
pub type PMIB_ICMP = *mut MIB_ICMP;
STRUCT!{struct MIBICMPSTATS_EX_XPSP1 {
    dwMsgs: DWORD,
    dwErrors: DWORD,
    rgdwTypeCount: [DWORD; 256],
}}
pub type PMIBICMPSTATS_EX_XPSP1 = *mut MIBICMPSTATS_EX_XPSP1;
pub type MIBICMPSTATS_EX = MIBICMPSTATS_EX_XPSP1;
pub type PMIBICMPSTATS_EX = *mut MIBICMPSTATS_EX_XPSP1;
STRUCT!{struct MIB_ICMP_EX_XPSP1 {
    icmpInStats: MIBICMPSTATS_EX,
    icmpOutStats: MIBICMPSTATS_EX,
}}
pub type PMIB_ICMP_EX_XPSP1 = *mut MIB_ICMP_EX_XPSP1;
pub type MIB_ICMP_EX = MIB_ICMP_EX_XPSP1;
pub type PMIB_ICMP_EX = *mut MIB_ICMP_EX_XPSP1;
ENUM!{enum ICMP6_TYPE {
    ICMP6_DST_UNREACH = 1,
    ICMP6_PACKET_TOO_BIG = 2,
    ICMP6_TIME_EXCEEDED = 3,
    ICMP6_PARAM_PROB = 4,
    ICMP6_ECHO_REQUEST = 128,
    ICMP6_ECHO_REPLY = 129,
    ICMP6_MEMBERSHIP_QUERY = 130,
    ICMP6_MEMBERSHIP_REPORT = 131,
    ICMP6_MEMBERSHIP_REDUCTION = 132,
    ND_ROUTER_SOLICIT = 133,
    ND_ROUTER_ADVERT = 134,
    ND_NEIGHBOR_SOLICIT = 135,
    ND_NEIGHBOR_ADVERT = 136,
    ND_REDIRECT = 137,
    ICMP6_V2_MEMBERSHIP_REPORT = 143,
}}
pub type PICMP6_TYPE = *mut ICMP6_TYPE;
ENUM!{enum ICMP4_TYPE {
    ICMP4_ECHO_REPLY = 0,
    ICMP4_DST_UNREACH = 3,
    ICMP4_SOURCE_QUENCH = 4,
    ICMP4_REDIRECT = 5,
    ICMP4_ECHO_REQUEST = 8,
    ICMP4_ROUTER_ADVERT = 9,
    ICMP4_ROUTER_SOLICIT = 10,
    ICMP4_TIME_EXCEEDED = 11,
    ICMP4_PARAM_PROB = 12,
    ICMP4_TIMESTAMP_REQUEST = 13,
    ICMP4_TIMESTAMP_REPLY = 14,
    ICMP4_MASK_REQUEST = 17,
    ICMP4_MASK_REPLY = 18,
}}
pub type PICMP4_TYPE = *mut ICMP4_TYPE;
STRUCT!{struct MIB_IPMCAST_OIF_XP {
    dwOutIfIndex: DWORD,
    dwNextHopAddr: DWORD,
    dwReserved: DWORD,
    dwReserved1: DWORD,
}}
pub type PMIB_IPMCAST_OIF_XP = *mut MIB_IPMCAST_OIF_XP;
STRUCT!{struct MIB_IPMCAST_OIF_W2K {
    dwOutIfIndex: DWORD,
    dwNextHopAddr: DWORD,
    pvReserved: PVOID,
    dwReserved: DWORD,
}}
pub type PMIB_IPMCAST_OIF_W2K = *mut MIB_IPMCAST_OIF_W2K;
pub type MIB_IPMCAST_OIF = MIB_IPMCAST_OIF_XP;
pub type PMIB_IPMCAST_OIF = *mut MIB_IPMCAST_OIF;
STRUCT!{struct MIB_IPMCAST_MFE {
    dwGroup: DWORD,
    dwSource: DWORD,
    dwSrcMask: DWORD,
    dwUpStrmNgbr: DWORD,
    dwInIfIndex: DWORD,
    dwInIfProtocol: DWORD,
    dwRouteProtocol: DWORD,
    dwRouteNetwork: DWORD,
    dwRouteMask: DWORD,
    ulUpTime: ULONG,
    ulExpiryTime: ULONG,
    ulTimeOut: ULONG,
    ulNumOutIf: ULONG,
    fFlags: DWORD,
    dwReserved: DWORD,
    rgmioOutInfo: [MIB_IPMCAST_OIF; ANY_SIZE],
}}
pub type PMIB_IPMCAST_MFE = *mut MIB_IPMCAST_MFE;
STRUCT!{struct MIB_MFE_TABLE {
    dwNumEntries: DWORD,
    table: [MIB_IPMCAST_MFE; ANY_SIZE],
}}
pub type PMIB_MFE_TABLE = *mut MIB_MFE_TABLE;
// FIXME: SIZEOF_BASIC_MIB_MFE
// FIXME: SIZEOF_MIB_MFE(x)
STRUCT!{struct MIB_IPMCAST_OIF_STATS_LH {
    dwOutIfIndex: DWORD,
    dwNextHopAddr: DWORD,
    dwDialContext: DWORD,
    ulTtlTooLow: ULONG,
    ulFragNeeded: ULONG,
    ulOutPackets: ULONG,
    ulOutDiscards: ULONG,
}}
pub type PMIB_IPMCAST_OIF_STATS_LH = *mut MIB_IPMCAST_OIF_STATS_LH;
STRUCT!{struct MIB_IPMCAST_OIF_STATS_W2K {
    dwOutIfIndex: DWORD,
    dwNextHopAddr: DWORD,
    pvDialContext: PVOID,
    ulTtlTooLow: ULONG,
    ulFragNeeded: ULONG,
    ulOutPackets: ULONG,
    ulOutDiscards: ULONG,
}}
pub type PMIB_IPMCAST_OIF_STATS_W2K = *mut MIB_IPMCAST_OIF_STATS_W2K;
pub type MIB_IPMCAST_OIF_STATS = MIB_IPMCAST_OIF_STATS_LH;
pub type PMIB_IPMCAST_OIF_STATS = *mut MIB_IPMCAST_OIF_STATS;
STRUCT!{struct MIB_IPMCAST_MFE_STATS {
    dwGroup: DWORD,
    dwSource: DWORD,
    dwSrcMask: DWORD,
    dwUpStrmNgbr: DWORD,
    dwInIfIndex: DWORD,
    dwInIfProtocol: DWORD,
    dwRouteProtocol: DWORD,
    dwRouteNetwork: DWORD,
    dwRouteMask: DWORD,
    ulUpTime: ULONG,
    ulExpiryTime: ULONG,
    ulNumOutIf: ULONG,
    ulInPkts: ULONG,
    ulInOctets: ULONG,
    ulPktsDifferentIf: ULONG,
    ulQueueOverflow: ULONG,
    rgmiosOutStats: [MIB_IPMCAST_OIF_STATS; ANY_SIZE],
}}
pub type PMIB_IPMCAST_MFE_STATS = *mut MIB_IPMCAST_MFE_STATS;
STRUCT!{struct MIB_MFE_STATS_TABLE {
    dwNumEntries: DWORD,
    table: [MIB_IPMCAST_MFE_STATS; ANY_SIZE],
}}
pub type PMIB_MFE_STATS_TABLE = *mut MIB_MFE_STATS_TABLE;
// FIXME: SIZEOF_BASIC_MIB_MFE_STATS
// FIXME: SIZEOF_MIB_MFE_STATS(x)
STRUCT!{struct MIB_IPMCAST_MFE_STATS_EX_XP {
    dwGroup: DWORD,
    dwSource: DWORD,
    dwSrcMask: DWORD,
    dwUpStrmNgbr: DWORD,
    dwInIfIndex: DWORD,
    dwInIfProtocol: DWORD,
    dwRouteProtocol: DWORD,
    dwRouteNetwork: DWORD,
    dwRouteMask: DWORD,
    ulUpTime: ULONG,
    ulExpiryTime: ULONG,
    ulNumOutIf: ULONG,
    ulInPkts: ULONG,
    ulInOctets: ULONG,
    ulPktsDifferentIf: ULONG,
    ulQueueOverflow: ULONG,
    ulUninitMfe: ULONG,
    ulNegativeMfe: ULONG,
    ulInDiscards: ULONG,
    ulInHdrErrors: ULONG,
    ulTotalOutPackets: ULONG,
    rgmiosOutStats: [MIB_IPMCAST_OIF_STATS; ANY_SIZE],
}}
pub type PMIB_IPMCAST_MFE_STATS_EX_XP = *mut MIB_IPMCAST_MFE_STATS_EX_XP;
pub type MIB_IPMCAST_MFE_STATS_EX = MIB_IPMCAST_MFE_STATS_EX_XP;
pub type PMIB_IPMCAST_MFE_STATS_EX = *mut MIB_IPMCAST_MFE_STATS_EX;
STRUCT!{struct MIB_MFE_STATS_TABLE_EX_XP {
    dwNumEntries: DWORD,
    table: [PMIB_IPMCAST_MFE_STATS_EX_XP; ANY_SIZE],
}}
pub type PMIB_MFE_STATS_TABLE_EX_XP = *mut MIB_MFE_STATS_TABLE_EX_XP;
pub type MIB_MFE_STATS_TABLE_EX = MIB_MFE_STATS_TABLE_EX_XP;
pub type PMIB_MFE_STATS_TABLE_EX = *mut MIB_MFE_STATS_TABLE_EX;
// FIXME: SIZEOF_BASIC_MIB_MFE_STATS_EX
// FIXME: SIZEOF_MIB_MFE_STATS_EX(x)
STRUCT!{struct MIB_IPMCAST_GLOBAL {
    dwEnable: DWORD,
}}
pub type PMIB_IPMCAST_GLOBAL = *mut MIB_IPMCAST_GLOBAL;
STRUCT!{struct MIB_IPMCAST_IF_ENTRY {
    dwIfIndex: DWORD,
    dwTtl: DWORD,
    dwProtocol: DWORD,
    dwRateLimit: DWORD,
    ulInMcastOctets: ULONG,
    ulOutMcastOctets: ULONG,
}}
pub type PMIB_IPMCAST_IF_ENTRY = *mut MIB_IPMCAST_IF_ENTRY;
STRUCT!{struct MIB_IPMCAST_IF_TABLE {
    dwNumEntries: DWORD,
    table: [MIB_IPMCAST_IF_ENTRY; ANY_SIZE],
}}
pub type PMIB_IPMCAST_IF_TABLE = *mut MIB_IPMCAST_IF_TABLE;
// FIXME: SIZEOF_MCAST_IF_TABLE(x)
