//! Managed Network Protocol
//!
//! It provides raw (unformatted) asynchronous network packet I/O services.
//! These services make it possible for multiple-event-driven drivers and
//! applications to access and use the system network interfaces at the same
//! time.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x7ab33a91,
    0xace5,
    0x4326,
    0xb5,
    0x72,
    &[0xe7, 0xee, 0x33, 0xd3, 0x9f, 0x16],
);

pub const SERVICE_BINDING_PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xf36ff770,
    0xa7e1,
    0x42cf,
    0x9e,
    0xd2,
    &[0x56, 0xf0, 0xf2, 0x71, 0xf4, 0x4c],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConfigData {
    pub received_queue_timeout_value: u32,
    pub transmit_queue_timeout_value: u32,
    pub protocol_type_filter: u16,
    pub enable_unicast_receive: crate::base::Boolean,
    pub enable_multicast_receive: crate::base::Boolean,
    pub enable_broadcast_receive: crate::base::Boolean,
    pub enable_promiscuous_receive: crate::base::Boolean,
    pub flush_queues_on_reset: crate::base::Boolean,
    pub enable_receive_timestamps: crate::base::Boolean,
    pub disable_background_polling: crate::base::Boolean,
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
pub struct ReceiveData {
    pub timestamp: crate::system::Time,
    pub recycle_event: crate::base::Event,
    pub packet_length: u32,
    pub header_length: u32,
    pub address_length: u32,
    pub data_length: u32,
    pub broadcast_flag: crate::base::Boolean,
    pub multicast_flag: crate::base::Boolean,
    pub promiscuous_flag: crate::base::Boolean,
    pub protocol_type: u16,
    pub destination_address: *mut core::ffi::c_void,
    pub source_address: *mut core::ffi::c_void,
    pub media_header: *mut core::ffi::c_void,
    pub packet_data: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TransmitData<const N: usize = 0> {
    pub destination_address: *mut crate::base::MacAddress,
    pub source_address: *mut crate::base::MacAddress,
    pub protocol_type: u16,
    pub data_length: u32,
    pub header_length: u16,
    pub fragment_count: u16,
    pub fragment_table: [FragmentData; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FragmentData {
    pub fragment_length: u32,
    pub fragment_buffer: *mut core::ffi::c_void,
}

pub type ProtocolGetModeData = eficall! {fn(
    *mut Protocol,
    *mut ConfigData,
    *mut crate::protocols::simple_network::Mode,
) -> crate::base::Status};

pub type ProtocolConfigure = eficall! {fn(
    *mut Protocol,
    *mut ConfigData,
) -> crate::base::Status};

pub type ProtocolMcastIpToMac = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
    *mut crate::base::IpAddress,
    *mut crate::base::MacAddress,
) -> crate::base::Status};

pub type ProtocolGroups = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
    *mut crate::base::MacAddress,
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
    pub mcast_ip_to_mac: ProtocolMcastIpToMac,
    pub groups: ProtocolGroups,
    pub transmit: ProtocolTransmit,
    pub receive: ProtocolReceive,
    pub cancel: ProtocolCancel,
    pub poll: ProtocolPoll,
}
