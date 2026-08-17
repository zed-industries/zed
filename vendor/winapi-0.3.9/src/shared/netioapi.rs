// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{PUINT8, SIZE_T, UINT8, ULONG64};
use shared::guiddef::GUID;
use shared::ifdef::{
    IF_MAX_PHYS_ADDRESS_LENGTH, IF_MAX_STRING_SIZE, IF_OPER_STATUS, NET_IFINDEX,
    NET_IF_ACCESS_TYPE, NET_IF_ADMIN_STATUS, NET_IF_COMPARTMENT_ID, NET_IF_COMPARTMENT_SCOPE,
    NET_IF_CONNECTION_TYPE, NET_IF_DIRECTION_TYPE, NET_IF_MEDIA_CONNECT_STATE, NET_IF_NETWORK_GUID,
    NET_LUID, PNET_IFINDEX, PNET_IF_COMPARTMENT_ID, PNET_IF_COMPARTMENT_SCOPE, PNET_LUID,
    TUNNEL_TYPE,
};
use shared::ipifcons::IFTYPE;
use shared::minwindef::{BYTE, DWORD, PULONG, UCHAR, ULONG, USHORT};
use shared::nldef::{
    NL_BANDWIDTH_INFORMATION, NL_DAD_STATE, NL_INTERFACE_OFFLOAD_ROD,
    NL_LINK_LOCAL_ADDRESS_BEHAVIOR, NL_NEIGHBOR_STATE, NL_PREFIX_ORIGIN,
    NL_ROUTER_DISCOVERY_BEHAVIOR, NL_ROUTE_ORIGIN, NL_ROUTE_PROTOCOL, NL_SUFFIX_ORIGIN,
};
use shared::ntddndis::{NDIS_MEDIUM, NDIS_PHYSICAL_MEDIUM};
use shared::ntdef::{
    BOOLEAN, CHAR, HANDLE, LARGE_INTEGER, PCHAR, PCSTR, PSTR, PVOID, PWCHAR, PWSTR, WCHAR,
};
use shared::ws2def::{ADDRESS_FAMILY, SCOPE_ID, ScopeLevelCount};
use shared::ws2ipdef::{PSOCKADDR_IN6_PAIR, SOCKADDR_IN6, SOCKADDR_INET};
const ANY_SIZE: usize = 1;
pub type NETIO_STATUS = DWORD;
pub type NETIOAPI_API = NETIO_STATUS;
ENUM!{enum MIB_NOTIFICATION_TYPE {
    MibParameterNotification,
    MibAddInstance,
    MibDeleteInstance,
    MibInitialNotification,
}}
pub type PMIB_NOTIFICATION_TYPE = *mut MIB_NOTIFICATION_TYPE;
STRUCT!{struct MIB_IF_ROW2_InterfaceAndOperStatusFlags {
    bitfield: BYTE,
}}
BITFIELD!{MIB_IF_ROW2_InterfaceAndOperStatusFlags bitfield: BOOLEAN [
    HardwareInterface set_HardwareInterface[0..1],
    FilterInterface set_FilterInterface[1..2],
    ConnectorPresent set_ConnectorPresent[2..3],
    NotAuthenticated set_NotAuthenticated[3..4],
    NotMediaConnected set_NotMediaConnected[4..5],
    Paused set_Paused[5..6],
    LowPower set_LowPower[6..7],
    EndPointInterface set_EndPointInterface[7..8],
]}
STRUCT!{struct MIB_IF_ROW2 {
    InterfaceLuid: NET_LUID,
    InterfaceIndex: NET_IFINDEX,
    InterfaceGuid: GUID,
    Alias: [WCHAR; IF_MAX_STRING_SIZE + 1],
    Description: [WCHAR; IF_MAX_STRING_SIZE + 1],
    PhysicalAddressLength: ULONG,
    PhysicalAddress: [UCHAR; IF_MAX_PHYS_ADDRESS_LENGTH],
    PermanentPhysicalAddress: [UCHAR; IF_MAX_PHYS_ADDRESS_LENGTH],
    Mtu: ULONG,
    Type: IFTYPE,
    TunnelType: TUNNEL_TYPE,
    MediaType: NDIS_MEDIUM,
    PhysicalMediumType: NDIS_PHYSICAL_MEDIUM,
    AccessType: NET_IF_ACCESS_TYPE,
    DirectionType: NET_IF_DIRECTION_TYPE,
    InterfaceAndOperStatusFlags: MIB_IF_ROW2_InterfaceAndOperStatusFlags,
    OperStatus: IF_OPER_STATUS,
    AdminStatus: NET_IF_ADMIN_STATUS,
    MediaConnectState: NET_IF_MEDIA_CONNECT_STATE,
    NetworkGuid: NET_IF_NETWORK_GUID,
    ConnectionType: NET_IF_CONNECTION_TYPE,
    TransmitLinkSpeed: ULONG64,
    ReceiveLinkSpeed: ULONG64,
    InOctets: ULONG64,
    InUcastPkts: ULONG64,
    InNUcastPkts: ULONG64,
    InDiscards: ULONG64,
    InErrors: ULONG64,
    InUnknownProtos: ULONG64,
    InUcastOctets: ULONG64,
    InMulticastOctets: ULONG64,
    InBroadcastOctets: ULONG64,
    OutOctets: ULONG64,
    OutUcastPkts: ULONG64,
    OutNUcastPkts: ULONG64,
    OutDiscards: ULONG64,
    OutErrors: ULONG64,
    OutUcastOctets: ULONG64,
    OutMulticastOctets: ULONG64,
    OutBroadcastOctets: ULONG64,
    OutQLen: ULONG64,
}}
pub type PMIB_IF_ROW2 = *mut MIB_IF_ROW2;
STRUCT!{struct MIB_IF_TABLE2 {
    NumEntries: ULONG,
    Table: [MIB_IF_ROW2; ANY_SIZE],
}}
pub type PMIB_IF_TABLE2 = *mut MIB_IF_TABLE2;
extern "system" {
    pub fn GetIfEntry2(
        Row: PMIB_IF_ROW2,
    ) -> NETIOAPI_API;
}
ENUM!{enum MIB_IF_ENTRY_LEVEL {
    MibIfEntryNormal = 0,
    MibIfEntryNormalWithoutStatistics = 2,
}}
pub type PMIB_IF_ENTRY_LEVEL = *mut MIB_IF_ENTRY_LEVEL;
extern "system" {
    pub fn GetIfEntry2Ex(
        Level: MIB_IF_ENTRY_LEVEL,
        Row: PMIB_IF_ROW2,
    ) -> NETIOAPI_API;
    pub fn GetIfTable2(
        Table: *mut PMIB_IF_TABLE2,
    ) -> NETIOAPI_API;
}
ENUM!{enum MIB_IF_TABLE_LEVEL {
    MibIfTableNormal = 0,
    MibIfTableRaw = 1,
    MibIfTableNormalWithoutStatistics = 2,
}}
pub type PMIB_IF_TABLE_LEVEL = *mut MIB_IF_TABLE_LEVEL;
extern "system" {
    pub fn GetIfTable2Ex(
        Level: MIB_IF_TABLE_LEVEL,
        Table: *mut PMIB_IF_TABLE2,
    ) -> NETIOAPI_API;
}
STRUCT!{struct MIB_IPINTERFACE_ROW {
    Family: ADDRESS_FAMILY,
    InterfaceLuid: NET_LUID,
    InterfaceIndex: NET_IFINDEX,
    MaxReassemblySize: ULONG,
    InterfaceIdentifier: ULONG64,
    MinRouterAdvertisementInterval: ULONG,
    MaxRouterAdvertisementInterval: ULONG,
    AdvertisingEnabled: BOOLEAN,
    ForwardingEnabled: BOOLEAN,
    WeakHostSend: BOOLEAN,
    WeakHostReceive: BOOLEAN,
    UseAutomaticMetric: BOOLEAN,
    UseNeighborUnreachabilityDetection: BOOLEAN,
    ManagedAddressConfigurationSupported: BOOLEAN,
    OtherStatefulConfigurationSupported: BOOLEAN,
    AdvertiseDefaultRoute: BOOLEAN,
    RouterDiscoveryBehavior: NL_ROUTER_DISCOVERY_BEHAVIOR,
    DadTransmits: ULONG, // DupAddrDetectTransmits in RFC 2462.
    BaseReachableTime: ULONG,
    RetransmitTime: ULONG,
    PathMtuDiscoveryTimeout: ULONG, // Path MTU discovery timeout (in ms).
    LinkLocalAddressBehavior: NL_LINK_LOCAL_ADDRESS_BEHAVIOR,
    LinkLocalAddressTimeout: ULONG, // In ms.
    ZoneIndices: [ULONG; ScopeLevelCount as usize], // Zone part of a SCOPE_ID.
    SitePrefixLength: ULONG,
    Metric: ULONG,
    NlMtu: ULONG,
    Connected: BOOLEAN,
    SupportsWakeUpPatterns: BOOLEAN,
    SupportsNeighborDiscovery: BOOLEAN,
    SupportsRouterDiscovery: BOOLEAN,
    ReachableTime: ULONG,
    TransmitOffload: NL_INTERFACE_OFFLOAD_ROD,
    ReceiveOffload: NL_INTERFACE_OFFLOAD_ROD,
    DisableDefaultRoutes: BOOLEAN,
}}
pub type PMIB_IPINTERFACE_ROW = *mut MIB_IPINTERFACE_ROW;
STRUCT!{struct MIB_IPINTERFACE_TABLE {
    NumEntries: ULONG,
    Table: [MIB_IPINTERFACE_ROW; ANY_SIZE],
}}
pub type PMIB_IPINTERFACE_TABLE = *mut MIB_IPINTERFACE_TABLE;
STRUCT!{struct MIB_IFSTACK_ROW {
    HigherLayerInterfaceIndex: NET_IFINDEX,
    LowerLayerInterfaceIndex: NET_IFINDEX,
}}
pub type PMIB_IFSTACK_ROW = *mut MIB_IFSTACK_ROW;
STRUCT!{struct MIB_INVERTEDIFSTACK_ROW {
    LowerLayerInterfaceIndex: NET_IFINDEX,
    HigherLayerInterfaceIndex: NET_IFINDEX,
}}
pub type PMIB_INVERTEDIFSTACK_ROW = *mut MIB_INVERTEDIFSTACK_ROW;
STRUCT!{struct MIB_IFSTACK_TABLE {
    NumEntries: ULONG,
    Table: [MIB_IFSTACK_ROW; ANY_SIZE],
}}
pub type PMIB_IFSTACK_TABLE = *mut MIB_IFSTACK_TABLE;
STRUCT!{struct MIB_INVERTEDIFSTACK_TABLE {
    NumEntries: ULONG,
    Table: [MIB_INVERTEDIFSTACK_ROW; ANY_SIZE],
}}
pub type PMIB_INVERTEDIFSTACK_TABLE = *mut MIB_INVERTEDIFSTACK_TABLE;
FN!{stdcall PIPINTERFACE_CHANGE_CALLBACK(
    CallerContext: PVOID,
    Row: PMIB_IPINTERFACE_ROW,
    NotificationType: MIB_NOTIFICATION_TYPE,
) -> ()}
STRUCT!{struct MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES {
    InboundBandwidthInformation: NL_BANDWIDTH_INFORMATION,
    OutboundBandwidthInformation: NL_BANDWIDTH_INFORMATION,
}}
pub type PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES = *mut
    MIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES;
extern "system" {
    pub fn GetIfStackTable(
        Table: *mut PMIB_IFSTACK_TABLE,
    ) -> NETIOAPI_API;
    pub fn GetInvertedIfStackTable(
        Table: *mut PMIB_INVERTEDIFSTACK_TABLE,
    ) -> NETIOAPI_API;
    pub fn GetIpInterfaceEntry(
        Row: PMIB_IPINTERFACE_ROW,
    ) -> NETIOAPI_API;
    pub fn GetIpInterfaceTable(
        Family: ADDRESS_FAMILY,
        Table: *mut PMIB_IPINTERFACE_TABLE,
    ) -> NETIOAPI_API;
    pub fn InitializeIpInterfaceEntry(
        Row: PMIB_IPINTERFACE_ROW,
    );
    pub fn NotifyIpInterfaceChange(
        Family: ADDRESS_FAMILY,
        Callback: PIPINTERFACE_CHANGE_CALLBACK,
        CallerContext: PVOID,
        InitialNotification: BOOLEAN,
        NotificationHandle: *mut HANDLE
    ) -> NETIOAPI_API;
    pub fn SetIpInterfaceEntry(
        Row: PMIB_IPINTERFACE_ROW,
    ) -> NETIOAPI_API;
    pub fn GetIpNetworkConnectionBandwidthEstimates(
        InterfaceIndex: NET_IFINDEX,
        AddressFamily: ADDRESS_FAMILY,
        BandwidthEstimates: PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES,
    ) -> NETIOAPI_API;
}
STRUCT!{struct MIB_UNICASTIPADDRESS_ROW {
    Address: SOCKADDR_INET,
    InterfaceLuid: NET_LUID,
    InterfaceIndex: NET_IFINDEX,
    PrefixOrigin: NL_PREFIX_ORIGIN,
    SuffixOrigin: NL_SUFFIX_ORIGIN,
    ValidLifetime: ULONG,
    PreferredLifetime: ULONG,
    OnLinkPrefixLength: UINT8,
    SkipAsSource: BOOLEAN,
    DadState: NL_DAD_STATE,
    ScopeId: SCOPE_ID,
    CreationTimeStamp: LARGE_INTEGER,
}}
pub type PMIB_UNICASTIPADDRESS_ROW = *mut MIB_UNICASTIPADDRESS_ROW;
STRUCT!{struct MIB_UNICASTIPADDRESS_TABLE {
    NumEntries: ULONG,
    Table: [MIB_UNICASTIPADDRESS_ROW; ANY_SIZE],
}}
pub type PMIB_UNICASTIPADDRESS_TABLE = *mut MIB_UNICASTIPADDRESS_TABLE;
FN!{stdcall PUNICAST_IPADDRESS_CHANGE_CALLBACK(
    CallerContext: PVOID,
    Row: PMIB_UNICASTIPADDRESS_ROW,
    NotificationType: MIB_NOTIFICATION_TYPE,
) -> ()}
extern "system" {
    pub fn CreateUnicastIpAddressEntry(
        Row: *const MIB_UNICASTIPADDRESS_ROW,
    ) -> NETIOAPI_API;
    pub fn DeleteUnicastIpAddressEntry(
        Row: *const MIB_UNICASTIPADDRESS_ROW,
    ) -> NETIOAPI_API;
    pub fn GetUnicastIpAddressEntry(
        Row: PMIB_UNICASTIPADDRESS_ROW
    ) -> NETIOAPI_API;
    pub fn GetUnicastIpAddressTable(
        Family: ADDRESS_FAMILY,
        Table: *mut PMIB_UNICASTIPADDRESS_TABLE,
    ) -> NETIOAPI_API;
    pub fn InitializeUnicastIpAddressEntry(
        Row: PMIB_UNICASTIPADDRESS_ROW,
    );
    pub fn NotifyUnicastIpAddressChange(
        Family: ADDRESS_FAMILY,
        Callback: PUNICAST_IPADDRESS_CHANGE_CALLBACK,
        CallerContext: PVOID,
        InitialNotification: BOOLEAN,
        NotificationHandle: *mut HANDLE,
    ) -> NETIOAPI_API;
}
FN!{stdcall PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK(
    CallerContext: PVOID,
    AddressTable: PMIB_UNICASTIPADDRESS_TABLE,
) -> ()}
extern "system" {
    pub fn NotifyStableUnicastIpAddressTable(
        Family: ADDRESS_FAMILY,
        Table: *mut PMIB_UNICASTIPADDRESS_TABLE,
        CallerCallback: PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK,
        CallerContext: PVOID,
        NotificationHandle: *mut HANDLE,
    ) -> NETIOAPI_API;
    pub fn SetUnicastIpAddressEntry(
        Row: *const MIB_UNICASTIPADDRESS_ROW,
    ) -> NETIOAPI_API;
}
STRUCT!{struct MIB_ANYCASTIPADDRESS_ROW {
    Address: SOCKADDR_INET,
    InterfaceLuid: NET_LUID,
    InterfaceIndex: NET_IFINDEX,
    ScopeId: SCOPE_ID,
}}
pub type PMIB_ANYCASTIPADDRESS_ROW = *mut MIB_ANYCASTIPADDRESS_ROW;
STRUCT!{struct MIB_ANYCASTIPADDRESS_TABLE {
    NumEntries: ULONG,
    Table: [MIB_ANYCASTIPADDRESS_ROW; ANY_SIZE],
}}
pub type PMIB_ANYCASTIPADDRESS_TABLE = *mut MIB_ANYCASTIPADDRESS_TABLE;
extern "system" {
    pub fn CreateAnycastIpAddressEntry(
        Row: *const MIB_ANYCASTIPADDRESS_ROW,
    ) -> NETIOAPI_API;
    pub fn DeleteAnycastIpAddressEntry(
        Row: *const MIB_ANYCASTIPADDRESS_ROW,
    ) -> NETIOAPI_API;
    pub fn GetAnycastIpAddressEntry(
        Row: PMIB_ANYCASTIPADDRESS_ROW,
    ) -> NETIOAPI_API;
    pub fn GetAnycastIpAddressTable(
        Family: ADDRESS_FAMILY,
        Table: *mut PMIB_ANYCASTIPADDRESS_TABLE,
    ) -> NETIOAPI_API;
}
STRUCT!{struct MIB_MULTICASTIPADDRESS_ROW {
    Address: SOCKADDR_INET,
    InterfaceIndex: NET_IFINDEX,
    InterfaceLuid: NET_LUID,
    ScopeId: SCOPE_ID,
}}
pub type PMIB_MULTICASTIPADDRESS_ROW = *mut MIB_MULTICASTIPADDRESS_ROW;
STRUCT!{struct MIB_MULTICASTIPADDRESS_TABLE {
    NumEntries: ULONG,
    Table: [MIB_MULTICASTIPADDRESS_ROW; ANY_SIZE],
}}
pub type PMIB_MULTICASTIPADDRESS_TABLE = *mut MIB_MULTICASTIPADDRESS_TABLE;
extern "system" {
    pub fn GetMulticastIpAddressEntry(
        Row: PMIB_MULTICASTIPADDRESS_ROW,
    ) -> NETIOAPI_API;
    pub fn GetMulticastIpAddressTable(
        Family: ADDRESS_FAMILY,
        Table: *mut PMIB_MULTICASTIPADDRESS_TABLE,
    ) -> NETIOAPI_API;
}
STRUCT!{struct IP_ADDRESS_PREFIX {
    Prefix: SOCKADDR_INET,
    PrefixLength: UINT8,
}}
pub type PIP_ADDRESS_PREFIX = *mut IP_ADDRESS_PREFIX;
STRUCT!{struct MIB_IPFORWARD_ROW2 {
    InterfaceLuid: NET_LUID,
    InterfaceIndex: NET_IFINDEX,
    DestinationPrefix: IP_ADDRESS_PREFIX,
    NextHop: SOCKADDR_INET,
    SitePrefixLength: UCHAR,
    ValidLifetime: ULONG,
    PreferredLifetime: ULONG,
    Metric: ULONG,
    Protocol: NL_ROUTE_PROTOCOL,
    Loopback: BOOLEAN,
    AutoconfigureAddress: BOOLEAN,
    Publish: BOOLEAN,
    Immortal: BOOLEAN,
    Age: ULONG,
    Origin: NL_ROUTE_ORIGIN,
}}
pub type PMIB_IPFORWARD_ROW2 = *mut MIB_IPFORWARD_ROW2;
STRUCT!{struct MIB_IPFORWARD_TABLE2 {
    NumEntries: ULONG,
    Table: [MIB_IPFORWARD_ROW2; ANY_SIZE],
}}
pub type PMIB_IPFORWARD_TABLE2 = *mut MIB_IPFORWARD_TABLE2;
FN!{stdcall PIPFORWARD_CHANGE_CALLBACK(
    CallerContext: PVOID,
    Row: PMIB_IPFORWARD_ROW2,
    NotificationType: MIB_NOTIFICATION_TYPE,
) -> ()}
extern "system" {
    pub fn CreateIpForwardEntry2(
        Row: *const MIB_IPFORWARD_ROW2,
    ) -> NETIOAPI_API;
    pub fn DeleteIpForwardEntry2(
        Row: *const MIB_IPFORWARD_ROW2,
    ) -> NETIOAPI_API;
    pub fn GetBestRoute2(
        InterfaceLuid: *mut NET_LUID,
        InterfaceIndex: NET_IFINDEX,
        SourceAddress: *const SOCKADDR_INET,
        DestinationAddress: *const SOCKADDR_INET,
        AddressSortOptions: ULONG,
        BestRoute: PMIB_IPFORWARD_ROW2,
        BestSourceAddress: *mut SOCKADDR_INET,
    ) -> NETIOAPI_API;
    pub fn GetIpForwardEntry2(
        Row: PMIB_IPFORWARD_ROW2,
    ) -> NETIOAPI_API;
    pub fn GetIpForwardTable2(
        Family: ADDRESS_FAMILY,
        Table: *mut PMIB_IPFORWARD_TABLE2,
    ) -> NETIOAPI_API;
    pub fn InitializeIpForwardEntry(
        Row: PMIB_IPFORWARD_ROW2,
    );
    pub fn NotifyRouteChange2(
        AddressFamily: ADDRESS_FAMILY,
        Callback: PIPFORWARD_CHANGE_CALLBACK,
        CallerContext: PVOID,
        InitialNotification: BOOLEAN,
        NotificationHandle: *mut HANDLE,
    ) -> NETIOAPI_API;
    pub fn SetIpForwardEntry2(
        Route: *const MIB_IPFORWARD_ROW2,
    ) -> NETIOAPI_API;
}
UNION!{union MIB_IPPATH_ROW_u {
    [u32; 1],
    LastReachable LastReachable_mut: ULONG, // Milliseconds.
    LastUnreachable LastUnreachable_mut: ULONG, // Milliseconds.
}}
STRUCT!{struct MIB_IPPATH_ROW {
    Source: SOCKADDR_INET,
    Destination: SOCKADDR_INET,
    InterfaceLuid: NET_LUID,
    InterfaceIndex: NET_IFINDEX,
    CurrentNextHop: SOCKADDR_INET,
    PathMtu: ULONG,
    RttMean: ULONG,
    RttDeviation: ULONG,
    u: MIB_IPPATH_ROW_u,
    IsReachable: BOOLEAN,
    LinkTransmitSpeed: ULONG64,
    LinkReceiveSpeed: ULONG64,
}}
pub type PMIB_IPPATH_ROW = *mut MIB_IPPATH_ROW;
STRUCT!{struct MIB_IPPATH_TABLE {
    NumEntries: ULONG,
    Table: [MIB_IPPATH_ROW; ANY_SIZE],
}}
pub type PMIB_IPPATH_TABLE = *mut MIB_IPPATH_TABLE;
extern "system" {
    pub fn FlushIpPathTable(
        Family: ADDRESS_FAMILY,
    ) -> NETIOAPI_API;
    pub fn GetIpPathEntry(
        Row: PMIB_IPPATH_ROW,
    ) -> NETIOAPI_API;
    pub fn GetIpPathTable(
        Family: ADDRESS_FAMILY,
        Table: *mut PMIB_IPPATH_TABLE,
    ) -> NETIOAPI_API;
}
STRUCT!{struct MIB_IPNET_ROW2_s {
    Flags: UCHAR,
}}
BITFIELD!{MIB_IPNET_ROW2_s Flags: UCHAR [
    IsRouter set_IsRouter[0..1],
    IsUnreachable set_IsUnreachable[1..2],
    Reserved  set_Reserved[2..8],
]}
UNION!{union MIB_IPNET_ROW2_ReachabilityTime {
    [u32; 1],
    LastReachable LastReachable_mut: ULONG,
    LastUnreachable LastUnreachable_mut: ULONG,
}}
STRUCT!{struct MIB_IPNET_ROW2 {
    Address: SOCKADDR_INET,
    InterfaceIndex: NET_IFINDEX,
    InterfaceLuid: NET_LUID,
    PhysicalAddress: [UCHAR; IF_MAX_PHYS_ADDRESS_LENGTH],
    PhysicalAddressLength: ULONG,
    State: NL_NEIGHBOR_STATE,
    s: MIB_IPNET_ROW2_s,
    ReachabilityTime: MIB_IPNET_ROW2_ReachabilityTime,
}}
pub type PMIB_IPNET_ROW2 = *mut MIB_IPNET_ROW2;
STRUCT!{struct MIB_IPNET_TABLE2 {
    NumEntries: ULONG,
    Table: [MIB_IPNET_ROW2; ANY_SIZE],
}}
pub type PMIB_IPNET_TABLE2 = *mut MIB_IPNET_TABLE2;
extern "system" {
    pub fn CreateIpNetEntry2(
        Row: *const MIB_IPNET_ROW2,
    ) -> NETIOAPI_API;
    pub fn DeleteIpNetEntry2(
        Row: *const MIB_IPNET_ROW2,
    ) -> NETIOAPI_API;
    pub fn FlushIpNetTable2(
        Family: ADDRESS_FAMILY,
        InterfaceIndex: NET_IFINDEX,
    ) -> NETIOAPI_API;
    pub fn GetIpNetEntry2(
        Row: PMIB_IPNET_ROW2,
    ) -> NETIOAPI_API;
    pub fn GetIpNetTable2(
        Family: ADDRESS_FAMILY,
        Table: *mut PMIB_IPNET_TABLE2,
    ) -> NETIOAPI_API;
    pub fn ResolveIpNetEntry2(
        Row: PMIB_IPNET_ROW2,
        SourceAddress: *const SOCKADDR_INET,
    ) -> NETIOAPI_API;
    pub fn SetIpNetEntry2(
        Row: PMIB_IPNET_ROW2,
    ) -> NETIOAPI_API;
}
pub const MIB_INVALID_TEREDO_PORT_NUMBER: USHORT = 0;
FN!{stdcall PTEREDO_PORT_CHANGE_CALLBACK(
    CallerContext: PVOID,
    Port: USHORT,
    NotificationType: MIB_NOTIFICATION_TYPE,
) -> ()}
extern "system" {
    pub fn NotifyTeredoPortChange(
        Callback: PTEREDO_PORT_CHANGE_CALLBACK,
        CallerContext: PVOID,
        InitialNotification: BOOLEAN,
        NotificationHandle: *mut HANDLE,
    ) -> NETIOAPI_API;
    pub fn GetTeredoPort(
        Port: *mut USHORT,
    ) -> NETIOAPI_API;
    pub fn CancelMibChangeNotify2(
        NotificationHandle: HANDLE,
    ) -> NETIOAPI_API;
    pub fn FreeMibTable(
        Memory: PVOID,
    );
    pub fn CreateSortedAddressPairs(
        SourceAddressList: *const SOCKADDR_IN6,
        SourceAddressCount: ULONG,
        DestinationAddressList: *const SOCKADDR_IN6,
        DestinationAddressCount: ULONG,
        AddressSortOptions: ULONG,
        SortedAddressPairList: *mut PSOCKADDR_IN6_PAIR,
        SortedAddressPairCount: *mut ULONG,
    ) -> NETIOAPI_API;
    pub fn ConvertCompartmentGuidToId(
        CompartmentGuid: *const GUID,
        CompartmentId: PNET_IF_COMPARTMENT_ID,
    ) -> NETIOAPI_API;
    pub fn ConvertCompartmentIdToGuid(
        CompartmentId: NET_IF_COMPARTMENT_ID,
        CompartmentGuid: *mut GUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceNameToLuidA(
        InterfaceName: *const CHAR,
        InterfaceLuid: *mut NET_LUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceNameToLuidW(
        InterfaceName: *const WCHAR,
        InterfaceLuid: *mut NET_LUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToNameA(
        InterfaceLuid: *const NET_LUID,
        InterfaceName: PSTR,
        Length: SIZE_T,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToNameW(
        InterfaceLuid: *const NET_LUID,
        InterfaceName: PWSTR,
        Length: SIZE_T,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToIndex(
        InterfaceLuid: *const NET_LUID,
        InterfaceIndex: PNET_IFINDEX,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceIndexToLuid(
        InterfaceIndex: NET_IFINDEX,
        InterfaceLuid: PNET_LUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToAlias(
        InterfaceLuid: *const NET_LUID,
        InterfaceAlias: PWSTR,
        Length: SIZE_T,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceAliasToLuid(
        InterfaceAlias: *const WCHAR,
        InterfaceLuid: PNET_LUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceLuidToGuid(
        InterfaceLuid: *const NET_LUID,
        InterfaceGuid: *mut GUID,
    ) -> NETIOAPI_API;
    pub fn ConvertInterfaceGuidToLuid(
        InterfaceGuid: *const GUID,
        InterfaceLuid: PNET_LUID,
    ) -> NETIOAPI_API;
    pub fn if_nametoindex(
        InterfaceName: PCSTR,
    ) -> NET_IFINDEX;
    pub fn if_indextoname(
        InterfaceIndex: NET_IFINDEX,
        InterfaceName: PCHAR,
    ) -> PCHAR;
    pub fn GetCurrentThreadCompartmentId() -> NET_IF_COMPARTMENT_ID;
    pub fn SetCurrentThreadCompartmentId(
        CompartmentId: NET_IF_COMPARTMENT_ID
    ) -> NETIOAPI_API;
    pub fn GetCurrentThreadCompartmentScope(
        CompartmentScope: PNET_IF_COMPARTMENT_SCOPE,
        CompartmentId: PNET_IF_COMPARTMENT_ID,
    );
    pub fn SetCurrentThreadCompartmentScope(
        CompartmentScope: NET_IF_COMPARTMENT_SCOPE,
    ) -> NETIOAPI_API;
    pub fn GetJobCompartmentId(
        JobHandle: HANDLE,
    ) -> NET_IF_COMPARTMENT_ID;
    pub fn SetJobCompartmentId(
        JobHandle: HANDLE,
        CompartmentId: NET_IF_COMPARTMENT_ID,
    ) -> NETIOAPI_API;
    pub fn GetSessionCompartmentId(
        SessionId: ULONG,
    ) -> NET_IF_COMPARTMENT_ID;
    pub fn SetSessionCompartmentId(
        SessionId: ULONG,
        CompartmentId: NET_IF_COMPARTMENT_ID,
    ) -> NETIOAPI_API;
    pub fn GetDefaultCompartmentId() -> NET_IF_COMPARTMENT_ID;
    pub fn GetNetworkInformation(
        NetworkGuid: *const NET_IF_NETWORK_GUID,
        CompartmentId: PNET_IF_COMPARTMENT_ID,
        SiteId: PULONG,
        NetworkName: PWCHAR,
        Length: ULONG,
    ) -> NETIOAPI_API;
    pub fn SetNetworkInformation(
        NetworkGuid: *const NET_IF_NETWORK_GUID,
        CompartmentId: NET_IF_COMPARTMENT_ID,
        NetworkName: *const WCHAR,
    ) -> NETIOAPI_API;
    pub fn ConvertLengthToIpv4Mask(
        MaskLength: ULONG,
        Mask: PULONG,
    ) -> NETIOAPI_API;
    pub fn ConvertIpv4MaskToLength(
        Mask: ULONG,
        MaskLength: PUINT8,
    ) -> NETIOAPI_API;
}
pub const DNS_SETTINGS_VERSION1: ULONG = 0x0001;
pub const DNS_INTERFACE_SETTINGS_VERSION1: ULONG = 0x0001;
pub const DNS_SETTING_IPV6: ULONG64 = 0x0001;
pub const DNS_SETTING_NAMESERVER: ULONG64 = 0x0002;
pub const DNS_SETTING_SEARCHLIST: ULONG64 = 0x0004;
pub const DNS_SETTING_REGISTRATION_ENABLED: ULONG64 = 0x0008;
pub const DNS_SETTING_REGISTER_ADAPTER_NAME: ULONG64 = 0x0010;
pub const DNS_SETTING_DOMAIN: ULONG64 = 0x0020;
pub const DNS_SETTING_HOSTNAME: ULONG64 = 0x0040;
pub const DNS_SETTINGS_ENABLE_LLMNR: ULONG64 = 0x0080;
pub const DNS_SETTINGS_QUERY_ADAPTER_NAME: ULONG64 = 0x0100;
pub const DNS_SETTING_PROFILE_NAMESERVER: ULONG64 = 0x0200;
STRUCT!{struct DNS_SETTINGS {
    Version: ULONG,
    Flags: ULONG64,
    Hostname: PWSTR,
    Domain: PWSTR,
    SearchList: PWSTR,
}}
STRUCT!{struct DNS_INTERFACE_SETTINGS {
    Version: ULONG,
    Flags: ULONG64,
    Domain: PWSTR,
    NameServer: PWSTR,
    SearchList: PWSTR,
    RegistrationEnabled: ULONG,
    RegisterAdapterName: ULONG,
    EnableLLMNR: ULONG,
    QueryAdapterName: ULONG,
    ProfileNameServer: PWSTR,
}}
extern "system" {
    pub fn GetDnsSettings(
        Settings: *mut DNS_SETTINGS,
    ) -> NETIOAPI_API;
    pub fn FreeDnsSettings(
        Settings: *mut DNS_SETTINGS,
    );
    pub fn SetDnsSettings(
        Settings: *const DNS_SETTINGS,
    ) -> NETIOAPI_API;
    pub fn GetInterfaceDnsSettings(
        Interface: GUID,
        Settings: *mut DNS_INTERFACE_SETTINGS,
    ) -> NETIOAPI_API;
    pub fn FreeInterfaceDnsSettings(
        Settings: *mut DNS_INTERFACE_SETTINGS,
    );
    pub fn SetInterfaceDnsSettings(
        Interface: GUID,
        Settings: *const DNS_INTERFACE_SETTINGS,
    ) -> NETIOAPI_API;
}
