//! IPv6 Protocol
//!
//! It implements a simple packet-oriented interface that can be used by
//! drivers, daemons, and applications to transmit and receive network packets.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x2c8759d5,
    0x5c2d,
    0x66ef,
    0x92,
    0x5f,
    &[0xb6, 0x6c, 0x10, 0x19, 0x57, 0xe2],
);

pub const SERVICE_BINDING_PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xec835dd3,
    0xfe0f,
    0x617b,
    0xa6,
    0x21,
    &[0xb3, 0x50, 0xc3, 0xe1, 0x33, 0x88],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ModeData {
    pub is_started: crate::base::Boolean,
    pub max_packet_size: u32,
    pub config_data: ConfigData,
    pub is_configured: crate::base::Boolean,
    pub address_count: u32,
    pub address_list: *mut AddressInfo,
    pub group_count: u32,
    pub group_table: *mut crate::base::Ipv6Address,
    pub route_count: u32,
    pub route_table: *mut RouteTable,
    pub neighbor_count: u32,
    pub neighbor_cache: *mut NeighborCache,
    pub prefix_count: u32,
    pub prefix_table: *mut AddressInfo,
    pub icmp_type_count: u32,
    pub icmp_type_list: *mut IcmpType,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConfigData {
    pub default_protocol: u8,
    pub accept_any_protocol: crate::base::Boolean,
    pub accept_icmp_errors: crate::base::Boolean,
    pub accept_promiscuous: crate::base::Boolean,
    pub destination_address: crate::base::Ipv6Address,
    pub station_address: crate::base::Ipv6Address,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub flow_lable: u32,
    pub receive_timeout: u32,
    pub transmit_timeout: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AddressInfo {
    pub address: crate::base::Ipv6Address,
    pub prefix_length: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RouteTable {
    pub gateway: crate::base::Ipv6Address,
    pub destination: crate::base::Ipv6Address,
    pub prefix_length: u8,
}

pub type NeighborState = u32;

pub const NEIGHBOR_IN_COMPLETE: NeighborState = 0x00000000;
pub const NEIGHBOR_REACHABLE: NeighborState = 0x00000001;
pub const NEIGHBOR_STATE: NeighborState = 0x00000002;
pub const NEIGHBOR_DELAY: NeighborState = 0x00000003;
pub const NEIGHBOR_PROBE: NeighborState = 0x00000004;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NeighborCache {
    pub neighbor: crate::base::Ipv6Address,
    pub link_address: crate::base::MacAddress,
    pub state: NeighborState,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IcmpType {
    pub r#type: u8,
    pub code: u8,
}

pub const ICMP_V6_DEST_UNREACHABLE: u8 = 0x01;
pub const ICMP_V6_PACKET_TOO_BIG: u8 = 0x02;
pub const ICMP_V6_TIME_EXCEEDED: u8 = 0x03;
pub const ICMP_V6_PARAMETER_PROBLEM: u8 = 0x04;

pub const ICMP_V6_ECHO_REQUEST: u8 = 0x80;
pub const ICMP_V6_ECHO_REPLY: u8 = 0x81;
pub const ICMP_V6_LISTENER_QUERY: u8 = 0x82;
pub const ICMP_V6_LISTENER_REPORT: u8 = 0x83;
pub const ICMP_V6_LISTENER_DONE: u8 = 0x84;
pub const ICMP_V6_ROUTER_SOLICIT: u8 = 0x85;
pub const ICMP_V6_ROUTER_ADVERTISE: u8 = 0x86;
pub const ICMP_V6_NEIGHBOR_SOLICIT: u8 = 0x87;
pub const ICMP_V6_NEIGHBOR_ADVERTISE: u8 = 0x88;
pub const ICMP_V6_REDIRECT: u8 = 0x89;
pub const ICMP_V6_LISTENER_REPORT_2: u8 = 0x8f;

pub const ICMP_V6_NO_ROUTE_TO_DEST: u8 = 0x00;
pub const ICMP_V6_COMM_PROHIBITED: u8 = 0x01;
pub const ICMP_V6_BEYOND_SCOPE: u8 = 0x02;
pub const ICMP_V6_ADDR_UNREACHABLE: u8 = 0x03;
pub const ICMP_V6_PORT_UNREACHABLE: u8 = 0x04;
pub const ICMP_V6_SOURCE_ADDR_FAILED: u8 = 0x05;
pub const ICMP_V6_ROUTE_REJECTED: u8 = 0x06;

pub const ICMP_V6_TIMEOUT_HOP_LIMIT: u8 = 0x00;
pub const ICMP_V6_TIMEOUT_REASSEMBLE: u8 = 0x01;

pub const ICMP_V6_ERRONEOUS_HEADER: u8 = 0x00;
pub const ICMP_V6_UNRECOGNIZE_NEXT_HDR: u8 = 0x01;
pub const ICMP_V6_UNRECOGNIZE_OPTION: u8 = 0x02;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CompletionToken {
    pub event: crate::base::Event,
    pub status: crate::base::Status,
    pub packet: CompletionTokenPacket,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CompletionTokenPacket {
    pub rx_data: *mut ReceiveData,
    pub tx_data: *mut TransmitData,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ReceiveData<const N: usize = 0> {
    pub time_stamp: crate::system::Time,
    pub recycle_signal: crate::base::Event,
    pub header_length: u32,
    pub header: *mut Header,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: [FragmentData; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub traffic_class_h_version: u8,
    pub flow_label_h_traffic_class_l: u8,
    pub flow_label_l: u16,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub source_address: crate::base::Ipv6Address,
    pub destination_address: crate::base::Ipv6Address,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FragmentData {
    pub fragment_length: u32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TransmitData<const N: usize = 0> {
    pub destination_address: *mut crate::base::Ipv6Address,
    pub override_data: *mut OverrideData,
    pub ext_hdrs_length: u32,
    pub ext_hdrs: *mut core::ffi::c_void,
    pub next_header: u8,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: [FragmentData; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OverrideData {
    pub protocol: u8,
    pub hop_limit: u8,
    pub flow_label: u32,
}

pub type ProtocolGetModeData = eficall! {fn(
    *mut Protocol,
    *mut ModeData,
    *mut crate::protocols::managed_network::ConfigData,
    *mut crate::protocols::simple_network::Mode,
) -> crate::base::Status};

pub type ProtocolConfigure = eficall! {fn(
    *mut Protocol,
    *mut ConfigData,
) -> crate::base::Status};

pub type ProtocolGroups = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
    *mut crate::base::Ipv6Address,
) -> crate::base::Status};

pub type ProtocolRoutes = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
    *mut crate::base::Ipv6Address,
    u8,
    *mut crate::base::Ipv6Address,
) -> crate::base::Status};

pub type ProtocolNeighbors = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
    *mut crate::base::Ipv6Address,
    *mut crate::base::MacAddress,
    u32,
    crate::base::Boolean,
) -> crate::base::Status};

pub type ProtocolTransmit = eficall! {fn(
    *mut Protocol,
    *mut CompletionToken,
) -> crate::base::Status};

pub type ProtocolReceive = eficall! {fn(
    *mut Protocol,
    *mut CompletionToken,
) -> crate::base::Status};

pub type ProtocolCancel = eficall! {fn(
    *mut Protocol,
    *mut CompletionToken,
) -> crate::base::Status};

pub type ProtocolPoll = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub get_mode_data: ProtocolGetModeData,
    pub configure: ProtocolConfigure,
    pub groups: ProtocolGroups,
    pub routes: ProtocolRoutes,
    pub neighbors: ProtocolNeighbors,
    pub transmit: ProtocolTransmit,
    pub receive: ProtocolReceive,
    pub cancel: ProtocolCancel,
    pub poll: ProtocolPoll,
}
