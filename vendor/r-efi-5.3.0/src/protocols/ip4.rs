//! IPv4 Protocol
//!
//! It implements a simple packet-oriented interface that can be used by
//! drivers, daemons, and applications to transmit and receive network packets.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x41d94cd2,
    0x35b6,
    0x455a,
    0x82,
    0x58,
    &[0xd4, 0xe5, 0x13, 0x34, 0xaa, 0xdd],
);

pub const SERVICE_BINDING_PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xc51711e7,
    0xb4bf,
    0x404a,
    0xbf,
    0xb8,
    &[0x0a, 0x04, 0x8e, 0xf1, 0xff, 0xe4],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConfigData {
    pub default_protocol: u8,
    pub accept_any_protocol: crate::base::Boolean,
    pub accept_icmp_errors: crate::base::Boolean,
    pub accept_broadcast: crate::base::Boolean,
    pub accept_promiscuous: crate::base::Boolean,
    pub use_default_address: crate::base::Boolean,
    pub station_address: crate::base::Ipv4Address,
    pub subnet_mask: crate::base::Ipv4Address,
    pub type_of_service: u8,
    pub time_to_live: u8,
    pub do_not_fragment: crate::base::Boolean,
    pub raw_data: crate::base::Boolean,
    pub receive_timeout: u32,
    pub transmit_timeout: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RouteTable {
    pub subnet_address: crate::base::Ipv4Address,
    pub subnet_mask: crate::base::Ipv4Address,
    pub gateway_address: crate::base::Ipv4Address,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IcmpType {
    pub r#type: u8,
    pub code: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ModeData {
    pub is_started: crate::base::Boolean,
    pub max_packet_size: u32,
    pub config_data: ConfigData,
    pub is_configured: crate::base::Boolean,
    pub group_count: u32,
    pub group_table: *mut crate::base::Ipv4Address,
    pub route_count: u32,
    pub route_table: *mut RouteTable,
    pub icmp_type_count: u32,
    pub icmp_type_list: *mut IcmpType,
}

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
    pub options_length: u32,
    pub options: *mut core::ffi::c_void,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: [FragmentData; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TransmitData<const N: usize = 0> {
    pub destination_address: crate::base::Ipv4Address,
    pub override_data: *mut OverrideData,
    pub options_length: u32,
    pub options_buffer: *mut core::ffi::c_void,
    pub total_data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: [FragmentData; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub header_length_and_version: u8,
    pub type_of_service: u8,
    pub total_length: u16,
    pub identification: u16,
    pub fragmentation: u16,
    pub time_to_live: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub source_address: crate::base::Ipv4Address,
    pub destination_address: crate::base::Ipv4Address,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FragmentData {
    pub fragment_length: u32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OverrideData {
    pub source_address: crate::base::Ipv4Address,
    pub gateway_address: crate::base::Ipv4Address,
    pub protocol: u8,
    pub type_of_service: u8,
    pub time_to_live: u8,
    pub do_not_fragment: crate::base::Boolean,
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
    *mut crate::base::Ipv4Address,
) -> crate::base::Status};

pub type ProtocolRoutes = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
    *mut crate::base::Ipv4Address,
    *mut crate::base::Ipv4Address,
    *mut crate::base::Ipv4Address,
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
    pub transmit: ProtocolTransmit,
    pub receive: ProtocolReceive,
    pub cancel: ProtocolCancel,
    pub poll: ProtocolPoll,
}
