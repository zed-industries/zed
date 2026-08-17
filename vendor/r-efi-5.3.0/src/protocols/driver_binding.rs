//! Driver Binding Protocol
//!
//! Provides the services required to determine if a driver supports a given controller. If
//! a controller is supported, then it also provides routines to start and stop the controller.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x18a031ab,
    0xb443,
    0x4d1a,
    0xa5,
    0xc0,
    &[0x0c, 0x09, 0x26, 0x1e, 0x9f, 0x71],
);

pub type ProtocolSupported = eficall! {fn(
    *mut Protocol,
    crate::base::Handle,
    *mut crate::protocols::device_path::Protocol,
) -> crate::base::Status};

pub type ProtocolStart = eficall! {fn(
    *mut Protocol,
    crate::base::Handle,
    *mut crate::protocols::device_path::Protocol,
) -> crate::base::Status};

pub type ProtocolStop = eficall! {fn(
    *mut Protocol,
    crate::base::Handle,
    usize,
    *mut crate::base::Handle,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub supported: ProtocolSupported,
    pub start: ProtocolStart,
    pub stop: ProtocolStop,
    pub version: u32,
    pub image_handle: crate::base::Handle,
    pub driver_binding_handle: crate::base::Handle,
}
