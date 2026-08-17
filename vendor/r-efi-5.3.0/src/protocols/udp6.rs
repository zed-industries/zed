//! User Datagram Protocol V6
//!
//! It provides simple packet-oriented services to transmit and receive UDP packets.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x4f948815,
    0xb4b9,
    0x43cb,
    0x8a,
    0x33,
    &[0x90, 0xe0, 0x60, 0xb3, 0x49, 0x55],
);

pub const SERVICE_BINDING_PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x66ed4721,
    0x3c98,
    0x4d3e,
    0x81,
    0xe3,
    &[0xd0, 0x3d, 0xd3, 0x9a, 0x72, 0x54],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConfigData {
    pub accept_promiscuous: crate::base::Boolean,
    pub accept_any_port: crate::base::Boolean,
    pub allow_duplicate_port: crate::base::Boolean,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub receive_timeout: u32,
    pub transmit_timeout: u32,
    pub station_address: crate::base::Ipv6Address,
    pub station_port: u16,
    pub remote_address: crate::base::Ipv6Address,
    pub remote_port: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SessionData {
    pub source_address: crate::base::Ipv6Address,
    pub source_port: u16,
    pub destination_address: crate::base::Ipv6Address,
    pub destination_port: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FragmentData {
    pub fragment_length: u32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ReceiveData<const N: usize = 0> {
    pub time_stamp: crate::system::Time,
    pub recycle_signal: crate::base::Event,
    pub udp_session: SessionData,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: [FragmentData; N],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TransmitData<const N: usize = 0> {
    pub udp_session_data: *mut SessionData,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: [FragmentData; N],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CompletionTokenPacket {
    pub rx_data: *mut ReceiveData,
    pub tx_data: *mut TransmitData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CompletionToken {
    pub event: crate::base::Event,
    pub status: crate::base::Status,
    pub packet: CompletionTokenPacket,
}

pub type ProtocolGetModeData = eficall! {fn(
    *mut Protocol,
    *mut ConfigData,
    *mut crate::protocols::ip6::ModeData,
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
    pub transmit: ProtocolTransmit,
    pub receive: ProtocolReceive,
    pub cancel: ProtocolCancel,
    pub poll: ProtocolPoll,
}
