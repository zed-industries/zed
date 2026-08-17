//! Transmission Control Protocol version 4
//!
//! It provides services to send and receive data streams.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x65530bc7,
    0xa359,
    0x410f,
    0xb0,
    0x10,
    &[0x5a, 0xad, 0xc7, 0xec, 0x2b, 0x62],
);

pub const SERVICE_BINDING_PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x00720665,
    0x67eb,
    0x4a99,
    0xba,
    0xf7,
    &[0xd3, 0xc3, 0x3a, 0x1c, 0x7c, 0xc9],
);

pub type ConnectionState = u32;

pub const STATE_CLOSED: ConnectionState = 0x00000000;
pub const STATE_LISTEN: ConnectionState = 0x00000001;
pub const STATE_SYN_SENT: ConnectionState = 0x00000002;
pub const STATE_SYN_RECEIVED: ConnectionState = 0x00000003;
pub const STATE_ESTABLISHED: ConnectionState = 0x00000004;
pub const STATE_FIN_WAIT1: ConnectionState = 0x00000005;
pub const STATE_FIN_WAIT2: ConnectionState = 0x00000006;
pub const STATE_CLOSING: ConnectionState = 0x00000007;
pub const STATE_TIME_WAIT: ConnectionState = 0x00000008;
pub const STATE_CLOSE_WAIT: ConnectionState = 0x00000009;
pub const STATE_LAST_ACK: ConnectionState = 0x0000000a;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConfigData {
    pub type_of_service: u8,
    pub time_to_live: u8,
    pub access_point: AccessPoint,
    pub control_option: *mut r#Option,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            type_of_service: Default::default(),
            time_to_live: Default::default(),
            access_point: Default::default(),
            control_option: core::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AccessPoint {
    pub use_default_address: crate::base::Boolean,
    pub station_address: crate::base::Ipv4Address,
    pub subnet_mask: crate::base::Ipv4Address,
    pub station_port: u16,
    pub remote_address: crate::base::Ipv4Address,
    pub remote_port: u16,
    pub active_flag: crate::base::Boolean,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct r#Option {
    pub receive_buffer_size: u32,
    pub send_buffer_size: u32,
    pub max_syn_back_log: u32,
    pub connection_timeout: u32,
    pub data_retries: u32,
    pub fin_timeout: u32,
    pub time_wait_timeout: u32,
    pub keep_alive_probes: u32,
    pub keep_alive_time: u32,
    pub keep_alive_interval: u32,
    pub enable_nagle: crate::base::Boolean,
    pub enable_time_stamp: crate::base::Boolean,
    pub enable_window_scaling: crate::base::Boolean,
    pub enable_selective_ack: crate::base::Boolean,
    pub enable_path_mtu_discovery: crate::base::Boolean,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CompletionToken {
    pub event: crate::base::Event,
    pub status: crate::base::Status,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ConnectionToken {
    pub completion_token: CompletionToken,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ListenToken {
    pub completion_token: CompletionToken,
    pub new_child_handle: crate::base::Handle,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IoToken {
    pub completion_token: CompletionToken,
    pub packet: IoTokenPacket,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IoTokenPacket {
    pub rx_data: *mut ReceiveData,
    pub tx_data: *mut TransmitData,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ReceiveData<const N: usize = 0> {
    pub urgent_flag: crate::base::Boolean,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: [FragmentData; N],
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
    pub push: crate::base::Boolean,
    pub urgent: crate::base::Boolean,
    pub data_length: u32,
    pub fragment_count: u32,
    pub fragment_table: [FragmentData; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CloseToken {
    pub completion_token: CompletionToken,
    pub abort_on_close: crate::base::Boolean,
}

pub type ProtocolGetModeData = eficall! {fn(
    *mut Protocol,
    *mut ConnectionState,
    *mut ConfigData,
    *mut crate::protocols::ip4::ModeData,
    *mut crate::protocols::managed_network::ConfigData,
    *mut crate::protocols::simple_network::Mode,
) -> crate::base::Status};

pub type ProtocolConfigure = eficall! {fn(
    *mut Protocol,
    *mut ConfigData,
) -> crate::base::Status};

pub type ProtocolRoutes = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
    *mut crate::base::Ipv4Address,
    *mut crate::base::Ipv4Address,
    *mut crate::base::Ipv4Address,
) -> crate::base::Status};

pub type ProtocolConnect = eficall! {fn(
    *mut Protocol,
    *mut ConnectionToken,
) -> crate::base::Status};

pub type ProtocolAccept = eficall! {fn(
    *mut Protocol,
    *mut ListenToken,
) -> crate::base::Status};

pub type ProtocolTransmit = eficall! {fn(
    *mut Protocol,
    *mut IoToken,
) -> crate::base::Status};

pub type ProtocolReceive = eficall! {fn(
    *mut Protocol,
    *mut IoToken,
) -> crate::base::Status};

pub type ProtocolClose = eficall! {fn(
    *mut Protocol,
    *mut CloseToken,
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
    pub routes: ProtocolRoutes,
    pub connect: ProtocolConnect,
    pub accept: ProtocolAccept,
    pub transmit: ProtocolTransmit,
    pub receive: ProtocolReceive,
    pub close: ProtocolClose,
    pub cancel: ProtocolCancel,
    pub poll: ProtocolPoll,
}
