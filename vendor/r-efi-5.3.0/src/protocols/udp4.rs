//! User Datagram Protocol V4
//!
//! It provides simple packet-oriented services to transmit and receive UDP packets.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x3ad9df29,
    0x4501,
    0x478d,
    0xb1,
    0xf8,
    &[0x7f, 0x7f, 0xe7, 0x0e, 0x50, 0xf3],
);

pub const SERVICE_BINDING_PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x83f01464,
    0x99bd,
    0x45e5,
    0xb3,
    0x83,
    &[0xaf, 0x63, 0x05, 0xd8, 0xe9, 0xe6],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConfigData {
    pub accept_broadcast: crate::base::Boolean,
    pub accept_promiscuous: crate::base::Boolean,
    pub accept_any_port: crate::base::Boolean,
    pub allow_duplicate_port: crate::base::Boolean,
    pub type_of_service: u8,
    pub time_to_live: u8,
    pub do_not_fragment: crate::base::Boolean,
    pub receive_timeout: u32,
    pub transmit_timeout: u32,
    pub use_default_address: crate::base::Boolean,
    pub station_address: crate::base::Ipv4Address,
    pub subnet_mask: crate::base::Ipv4Address,
    pub station_port: u16,
    pub remote_address: crate::base::Ipv4Address,
    pub remote_port: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SessionData {
    pub source_address: crate::base::Ipv4Address,
    pub source_port: u16,
    pub destination_address: crate::base::Ipv4Address,
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
    pub gateway_address: *mut crate::base::Ipv4Address,
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
    *mut crate::protocols::ip4::ModeData,
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
