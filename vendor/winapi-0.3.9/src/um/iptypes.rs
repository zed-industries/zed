// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::{UINT8, ULONG64};
use shared::guiddef::GUID;
use shared::ifdef::{
    IF_INDEX, IF_LUID, IF_OPER_STATUS, NET_IF_COMPARTMENT_ID, NET_IF_CONNECTION_TYPE,
    NET_IF_NETWORK_GUID, TUNNEL_TYPE
};
use shared::ipifcons::IFTYPE;
use shared::minwindef::{BOOL, BYTE, DWORD, UCHAR, UINT};
use shared::nldef::{NL_DAD_STATE, NL_PREFIX_ORIGIN, NL_SUFFIX_ORIGIN};
use shared::ntdef::{CHAR, PCHAR, PWCHAR, ULONG, ULONGLONG, WCHAR};
use shared::ws2def::SOCKET_ADDRESS;
use ucrt::corecrt::time_t;
pub const MAX_ADAPTER_DESCRIPTION_LENGTH: usize = 128;
pub const MAX_ADAPTER_NAME_LENGTH: usize = 256;
pub const MAX_ADAPTER_ADDRESS_LENGTH: usize = 8;
pub const DEFAULT_MINIMUM_ENTITIES: usize = 32;
pub const MAX_HOSTNAME_LEN: usize = 128;
pub const MAX_DOMAIN_NAME_LEN: usize = 128;
pub const MAX_SCOPE_ID_LEN: usize = 256;
pub const MAX_DHCPV6_DUID_LENGTH: usize = 130;
pub const MAX_DNS_SUFFIX_STRING_LENGTH: usize = 256;
pub const BROADCAST_NODETYPE: usize = 1;
pub const PEER_TO_PEER_NODETYPE: usize = 2;
pub const MIXED_NODETYPE: usize = 4;
pub const HYBRID_NODETYPE: usize = 8;
STRUCT!{struct IP_ADDRESS_STRING {
    String: [CHAR; 4*4],
}}
pub type PIP_ADDRESS_STRING = *mut IP_ADDRESS_STRING;
pub type IP_MASK_STRING = IP_ADDRESS_STRING;
pub type PIP_MASK_STRING = *mut IP_MASK_STRING;
STRUCT!{struct IP_ADDR_STRING {
    Next: *mut IP_ADDR_STRING,
    IpAddress: IP_ADDRESS_STRING,
    IpMask: IP_MASK_STRING,
    Context: DWORD,
}}
pub type PIP_ADDR_STRING = *mut IP_ADDR_STRING;
STRUCT!{struct IP_ADAPTER_INFO {
    Next: *mut IP_ADAPTER_INFO,
    ComboIndex: DWORD,
    AdapterName: [CHAR; MAX_ADAPTER_NAME_LENGTH + 4],
    Description: [CHAR; MAX_ADAPTER_DESCRIPTION_LENGTH + 4],
    AddressLength: UINT,
    Address: [BYTE; MAX_ADAPTER_ADDRESS_LENGTH],
    Index: DWORD,
    Type: UINT,
    DhcpEnabled: UINT,
    CurrentIpAddress: PIP_ADDR_STRING,
    IpAddressList: IP_ADDR_STRING,
    GatewayList: IP_ADDR_STRING,
    DhcpServer: IP_ADDR_STRING,
    HaveWins: BOOL,
    PrimaryWinsServer: IP_ADDR_STRING,
    SecondaryWinsServer: IP_ADDR_STRING,
    LeaseObtained: time_t,
    LeaseExpires: time_t,
}}
pub type PIP_ADAPTER_INFO = *mut IP_ADAPTER_INFO;
pub type IP_PREFIX_ORIGIN = NL_PREFIX_ORIGIN;
pub type IP_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN;
pub type IP_DAD_STATE = NL_DAD_STATE;
STRUCT!{struct IP_ADAPTER_UNICAST_ADDRESS_LH_u_s {
    Length: ULONG,
    Flags: DWORD,
}}
UNION!{union IP_ADAPTER_UNICAST_ADDRESS_LH_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_UNICAST_ADDRESS_LH_u_s,
}}
STRUCT!{struct IP_ADAPTER_UNICAST_ADDRESS_LH {
    u: IP_ADAPTER_UNICAST_ADDRESS_LH_u,
    Next: *mut IP_ADAPTER_UNICAST_ADDRESS_LH,
    Address: SOCKET_ADDRESS,
    PrefixOrigin: IP_PREFIX_ORIGIN,
    SuffixOrigin: IP_SUFFIX_ORIGIN,
    DadState: IP_DAD_STATE,
    ValidLifetime: ULONG,
    PreferredLifetime: ULONG,
    LeaseLifetime: ULONG,
    OnLinkPrefixLength: UINT8,
}}
pub type PIP_ADAPTER_UNICAST_ADDRESS_LH = *mut IP_ADAPTER_UNICAST_ADDRESS_LH;
STRUCT!{struct IP_ADAPTER_UNICAST_ADDRESS_XP_u_s {
    Length: ULONG,
    Flags: DWORD,
}}
UNION!{union IP_ADAPTER_UNICAST_ADDRESS_XP_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_UNICAST_ADDRESS_XP_u_s,
}}
STRUCT!{struct IP_ADAPTER_UNICAST_ADDRESS_XP {
    u: IP_ADAPTER_UNICAST_ADDRESS_XP_u,
    Next: *mut IP_ADAPTER_UNICAST_ADDRESS_XP,
    Address: SOCKET_ADDRESS,
    PrefixOrigin: IP_PREFIX_ORIGIN,
    SuffixOrigin: IP_SUFFIX_ORIGIN,
    DadState: IP_DAD_STATE,
    ValidLifetime: ULONG,
    PreferredLifetime: ULONG,
    LeaseLifetime: ULONG,
}}
pub type PIP_ADAPTER_UNICAST_ADDRESS_XP = *mut IP_ADAPTER_UNICAST_ADDRESS_XP;
pub type IP_ADAPTER_UNICAST_ADDRESS = IP_ADAPTER_UNICAST_ADDRESS_LH;
// pub type IP_ADAPTER_UNICAST_ADDRESS = IP_ADAPTER_UNICAST_ADDRESS_XP;
pub type PIP_ADAPTER_UNICAST_ADDRESS = *mut IP_ADAPTER_UNICAST_ADDRESS;
pub const IP_ADAPTER_ADDRESS_DNS_ELIGIBLE: usize = 0x01;
pub const IP_ADAPTER_ADDRESS_TRANSIENT: usize = 0x02;
STRUCT!{struct IP_ADAPTER_ANYCAST_ADDRESS_XP_u_s {
    Length: ULONG,
    Flags: DWORD,
}}
UNION!{union IP_ADAPTER_ANYCAST_ADDRESS_XP_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_ANYCAST_ADDRESS_XP_u_s,
}}
STRUCT!{struct IP_ADAPTER_ANYCAST_ADDRESS_XP {
    u: IP_ADAPTER_ANYCAST_ADDRESS_XP_u,
    Next: *mut IP_ADAPTER_ANYCAST_ADDRESS_XP,
    Address: SOCKET_ADDRESS,
}}
pub type PIP_ADAPTER_ANYCAST_ADDRESS_XP = *mut IP_ADAPTER_ANYCAST_ADDRESS_XP;
pub type IP_ADAPTER_ANYCAST_ADDRESS = IP_ADAPTER_ANYCAST_ADDRESS_XP;
pub type PIP_ADAPTER_ANYCAST_ADDRESS = *mut IP_ADAPTER_ANYCAST_ADDRESS;
STRUCT!{struct IP_ADAPTER_MULTICAST_ADDRESS_XP_u_s {
    Length: ULONG,
    Flags: DWORD,
}}
UNION!{union IP_ADAPTER_MULTICAST_ADDRESS_XP_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_MULTICAST_ADDRESS_XP_u_s,
}}
STRUCT!{struct IP_ADAPTER_MULTICAST_ADDRESS_XP {
    u: IP_ADAPTER_MULTICAST_ADDRESS_XP_u,
    Next: *mut IP_ADAPTER_MULTICAST_ADDRESS_XP,
    Address: SOCKET_ADDRESS,
}}
pub type PIP_ADAPTER_MULTICAST_ADDRESS_XP = *mut IP_ADAPTER_MULTICAST_ADDRESS_XP;
pub type IP_ADAPTER_MULTICAST_ADDRESS = IP_ADAPTER_MULTICAST_ADDRESS_XP;
pub type PIP_ADAPTER_MULTICAST_ADDRESS = *mut IP_ADAPTER_MULTICAST_ADDRESS_XP;
STRUCT!{struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP_u_s {
    Length: ULONG,
    Reserved: DWORD,
}}
UNION!{union IP_ADAPTER_DNS_SERVER_ADDRESS_XP_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_u_s,
}}
STRUCT!{struct IP_ADAPTER_DNS_SERVER_ADDRESS_XP {
    u: IP_ADAPTER_DNS_SERVER_ADDRESS_XP_u,
    Next: *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    Address: SOCKET_ADDRESS,
}}
pub type PIP_ADAPTER_DNS_SERVER_ADDRESS_XP = *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
pub type IP_ADAPTER_DNS_SERVER_ADDRESS = IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
pub type PIP_ADAPTER_DNS_SERVER_ADDRESS = *mut IP_ADAPTER_DNS_SERVER_ADDRESS_XP;
STRUCT!{struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH_u_s {
    Length: ULONG,
    Reserved: DWORD,
}}
UNION!{union IP_ADAPTER_WINS_SERVER_ADDRESS_LH_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_u_s,
}}
STRUCT!{struct IP_ADAPTER_WINS_SERVER_ADDRESS_LH {
    u: IP_ADAPTER_WINS_SERVER_ADDRESS_LH_u,
    Next: *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    Address: SOCKET_ADDRESS,
}}
pub type PIP_ADAPTER_WINS_SERVER_ADDRESS_LH = *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
pub type IP_ADAPTER_WINS_SERVER_ADDRESS = IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
pub type PIP_ADAPTER_WINS_SERVER_ADDRESS = *mut IP_ADAPTER_WINS_SERVER_ADDRESS_LH;
STRUCT!{struct IP_ADAPTER_GATEWAY_ADDRESS_LH_u_s {
    Length: ULONG,
    Reserved: DWORD,
}}
UNION!{union IP_ADAPTER_GATEWAY_ADDRESS_LH_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_GATEWAY_ADDRESS_LH_u_s,
}}
STRUCT!{struct IP_ADAPTER_GATEWAY_ADDRESS_LH {
    u: IP_ADAPTER_GATEWAY_ADDRESS_LH_u,
    Next: *mut IP_ADAPTER_GATEWAY_ADDRESS_LH,
    Address: SOCKET_ADDRESS,
}}
pub type PIP_ADAPTER_GATEWAY_ADDRESS_LH = *mut IP_ADAPTER_GATEWAY_ADDRESS_LH;
pub type IP_ADAPTER_GATEWAY_ADDRESS = IP_ADAPTER_GATEWAY_ADDRESS_LH;
pub type PIP_ADAPTER_GATEWAY_ADDRESS = *mut IP_ADAPTER_GATEWAY_ADDRESS_LH;
STRUCT!{struct IP_ADAPTER_PREFIX_XP_u_s {
    Length: ULONG,
    Flags: DWORD,
}}
UNION!{union IP_ADAPTER_PREFIX_XP_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_PREFIX_XP_u_s,
}}
STRUCT!{struct IP_ADAPTER_PREFIX_XP {
    u: IP_ADAPTER_PREFIX_XP_u,
    Next: *mut IP_ADAPTER_PREFIX_XP,
    Address: SOCKET_ADDRESS,
    PrefixLength: ULONG,
}}
pub type PIP_ADAPTER_PREFIX_XP = *mut IP_ADAPTER_PREFIX_XP;
pub type IP_ADAPTER_PREFIX = IP_ADAPTER_PREFIX_XP;
pub type PIP_ADAPTER_PREFIX = *mut IP_ADAPTER_PREFIX_XP;
STRUCT!{struct IP_ADAPTER_DNS_SUFFIX {
    Next: *mut IP_ADAPTER_DNS_SUFFIX,
    String: [WCHAR; MAX_DNS_SUFFIX_STRING_LENGTH],
}}
pub type PIP_ADAPTER_DNS_SUFFIX = *mut IP_ADAPTER_DNS_SUFFIX;
pub const IP_ADAPTER_DDNS_ENABLED: DWORD = 0x00000001;
pub const IP_ADAPTER_REGISTER_ADAPTER_SUFFIX: DWORD = 0x00000002;
pub const IP_ADAPTER_DHCP_ENABLED: DWORD = 0x00000004;
pub const IP_ADAPTER_RECEIVE_ONLY: DWORD = 0x00000008;
pub const IP_ADAPTER_NO_MULTICAST: DWORD = 0x00000010;
pub const IP_ADAPTER_IPV6_OTHER_STATEFUL_CONFIG: DWORD = 0x00000020;
pub const IP_ADAPTER_NETBIOS_OVER_TCPIP_ENABLED: DWORD = 0x00000040;
pub const IP_ADAPTER_IPV4_ENABLED: DWORD = 0x00000080;
pub const IP_ADAPTER_IPV6_ENABLED: DWORD = 0x00000100;
pub const IP_ADAPTER_IPV6_MANAGE_ADDRESS_CONFIG: DWORD = 0x00000200;
STRUCT!{struct IP_ADAPTER_ADDRESSES_LH_u_s {
    Length: ULONG,
    IfIndex: IF_INDEX,
}}
UNION!{union IP_ADAPTER_ADDRESSES_LH_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_ADDRESSES_LH_u_s,
}}
STRUCT!{struct IP_ADAPTER_ADDRESSES_LH {
    u: IP_ADAPTER_ADDRESSES_LH_u,
    Next: *mut IP_ADAPTER_ADDRESSES_LH,
    AdapterName: PCHAR,
    FirstUnicastAddress: PIP_ADAPTER_UNICAST_ADDRESS_LH,
    FirstAnycastAddress: PIP_ADAPTER_ANYCAST_ADDRESS_XP,
    FirstMulticastAddress: PIP_ADAPTER_MULTICAST_ADDRESS_XP,
    FirstDnsServerAddress: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    DnsSuffix: PWCHAR,
    Description: PWCHAR,
    FriendlyName: PWCHAR,
    PhysicalAddress: [BYTE; MAX_ADAPTER_ADDRESS_LENGTH],
    PhysicalAddressLength: ULONG,
    Flags: ULONG,
    Mtu: ULONG,
    IfType: IFTYPE,
    OperStatus: IF_OPER_STATUS,
    Ipv6IfIndex: IF_INDEX,
    ZoneIndices: [ULONG; 16],
    FirstPrefix: PIP_ADAPTER_PREFIX_XP,
    TransmitLinkSpeed: ULONG64,
    ReceiveLinkSpeed: ULONG64,
    FirstWinsServerAddress: PIP_ADAPTER_WINS_SERVER_ADDRESS_LH,
    FirstGatewayAddress: PIP_ADAPTER_GATEWAY_ADDRESS_LH,
    Ipv4Metric: ULONG,
    Ipv6Metric: ULONG,
    Luid: IF_LUID,
    Dhcpv4Server: SOCKET_ADDRESS,
    CompartmentId: NET_IF_COMPARTMENT_ID,
    NetworkGuid: NET_IF_NETWORK_GUID,
    ConnectionType: NET_IF_CONNECTION_TYPE,
    TunnelType: TUNNEL_TYPE,
    Dhcpv6Server: SOCKET_ADDRESS,
    Dhcpv6ClientDuid: [BYTE; MAX_DHCPV6_DUID_LENGTH],
    Dhcpv6ClientDuidLength: ULONG,
    Dhcpv6Iaid: ULONG,
    FirstDnsSuffix: PIP_ADAPTER_DNS_SUFFIX,
}}
BITFIELD!{IP_ADAPTER_ADDRESSES_LH Flags: ULONG [
    DdnsEnabled set_DdnsEnabled[0..1],
    RegisterAdapterSuffix set_RegisterAdapterSuffix[1..2],
    Dhcpv4Enabled set_Dhcpv4Enabled[2..3],
    ReceiveOnly set_ReceiveOnly[3..4],
    NoMulticast set_NoMulticast[4..5],
    Ipv6OtherStatefulConfig set_Ipv6OtherStatefulConfig[5..6],
    NetbiosOverTcpipEnabled set_NetbiosOverTcpipEnabled[6..7],
    Ipv4Enabled set_Ipv4Enabled[7..8],
    Ipv6Enabled set_Ipv6Enabled[8..9],
    Ipv6ManagedAddressConfigurationSupported set_Ipv6ManagedAddressConfigurationSupported[9..10],
]}
pub type PIP_ADAPTER_ADDRESSES_LH = *mut IP_ADAPTER_ADDRESSES_LH;
STRUCT!{struct IP_ADAPTER_ADDRESSES_XP_u_s {
    Length: ULONG,
    IfIndex: DWORD,
}}
UNION!{union IP_ADAPTER_ADDRESSES_XP_u {
    [u64; 1],
    Alignment Alignment_mut: ULONGLONG,
    s s_mut: IP_ADAPTER_ADDRESSES_XP_u_s,
}}
STRUCT!{struct IP_ADAPTER_ADDRESSES_XP {
    u: IP_ADAPTER_ADDRESSES_XP_u,
    Next: *mut IP_ADAPTER_ADDRESSES_XP,
    AdapterName: PCHAR,
    FirstUnicastAddress: PIP_ADAPTER_UNICAST_ADDRESS_XP,
    FirstAnycastAddress: PIP_ADAPTER_ANYCAST_ADDRESS_XP,
    FirstMulticastAddress: PIP_ADAPTER_MULTICAST_ADDRESS_XP,
    FirstDnsServerAddress: PIP_ADAPTER_DNS_SERVER_ADDRESS_XP,
    DnsSuffix: PWCHAR,
    Description: PWCHAR,
    FriendlyName: PWCHAR,
    PhysicalAddress: [BYTE; MAX_ADAPTER_ADDRESS_LENGTH],
    PhysicalAddressLength: DWORD,
    Flags: DWORD,
    Mtu: DWORD,
    IfType: DWORD,
    OperStatus: IF_OPER_STATUS,
    Ipv6IfIndex: DWORD,
    ZoneIndices: [DWORD; 16],
    FirstPrefix: PIP_ADAPTER_PREFIX_XP,
}}
pub type PIP_ADAPTER_ADDRESSES_XP = *mut IP_ADAPTER_ADDRESSES_XP;
pub type IP_ADAPTER_ADDRESSES = IP_ADAPTER_ADDRESSES_LH;
// pub type IP_ADAPTER_ADDRESSES = IP_ADAPTER_ADDRESSES_XP;
pub type PIP_ADAPTER_ADDRESSES = *mut IP_ADAPTER_ADDRESSES;
pub const GAA_FLAG_SKIP_UNICAST: ULONG = 0x0001;
pub const GAA_FLAG_SKIP_ANYCAST: ULONG = 0x0002;
pub const GAA_FLAG_SKIP_MULTICAST: ULONG = 0x0004;
pub const GAA_FLAG_SKIP_DNS_SERVER: ULONG = 0x0008;
pub const GAA_FLAG_INCLUDE_PREFIX: ULONG = 0x0010;
pub const GAA_FLAG_SKIP_FRIENDLY_NAME: ULONG = 0x0020;
pub const GAA_FLAG_INCLUDE_WINS_INFO: ULONG = 0x0040;
pub const GAA_FLAG_INCLUDE_GATEWAYS: ULONG = 0x0080;
pub const GAA_FLAG_INCLUDE_ALL_INTERFACES: ULONG = 0x0100;
pub const GAA_FLAG_INCLUDE_ALL_COMPARTMENTS: ULONG = 0x0200;
pub const GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER: ULONG = 0x0400;
STRUCT!{struct IP_PER_ADAPTER_INFO_W2KSP1 {
    AutoconfigEnabled: UINT,
    AutoconfigActive: UINT,
    CurrentDnsServer: PIP_ADDR_STRING,
    DnsServerList: IP_ADDR_STRING,
}}
pub type PIP_PER_ADAPTER_INFO_W2KSP1 = *mut IP_PER_ADAPTER_INFO_W2KSP1;
pub type IP_PER_ADAPTER_INFO = IP_PER_ADAPTER_INFO_W2KSP1;
pub type PIP_PER_ADAPTER_INFO = *mut IP_PER_ADAPTER_INFO;
STRUCT!{struct FIXED_INFO_W2KSP1 {
    HostName: [CHAR; MAX_HOSTNAME_LEN + 4],
    DomainName: [CHAR; MAX_DOMAIN_NAME_LEN + 4],
    CurrentDnsServer: PIP_ADDR_STRING,
    DnsServerList: IP_ADDR_STRING,
    NodeType: UINT,
    ScopeId: [CHAR; MAX_SCOPE_ID_LEN + 4],
    EnableRouting: UINT,
    EnableProxy: UINT,
    EnableDns: UINT,
}}
pub type PFIXED_INFO_W2KSP1 = *mut FIXED_INFO_W2KSP1;
pub type FIXED_INFO = FIXED_INFO_W2KSP1;
pub type PFIXED_INFO = *mut FIXED_INFO;
STRUCT!{struct IP_INTERFACE_NAME_INFO_W2KSP1 {
    Index: ULONG,
    MediaType: ULONG,
    ConnectionType: UCHAR,
    AccessType: UCHAR,
    DeviceGuid: GUID,
    InterfaceGuid: GUID,
}}
pub type PIP_INTERFACE_NAME_INFO_W2KSP1 = *mut IP_INTERFACE_NAME_INFO_W2KSP1;
pub type IP_INTERFACE_NAME_INFO = IP_INTERFACE_NAME_INFO_W2KSP1;
pub type PIP_INTERFACE_NAME_INFO = *mut IP_INTERFACE_NAME_INFO;
