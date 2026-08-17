//! Platform Driver Override Protocol
//!
//! This protocol matches one or more drivers to a controller. A platform driver
//! produces this protocol, and it is installed on a separate handle. This
//! protocol is used by the `EFI_BOOT_SERVICES.ConnectController()` boot service
//! to select the best driver for a controller. All of the drivers returned by
//! this protocol have a higher precedence than drivers found from an EFI Bus
//! Specific Driver Override Protocol or drivers found from the general UEFI
//! driver binding search algorithm. If more than one driver is returned by this
//! protocol, then the drivers are returned in order from highest precedence to
//! lowest precedence.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x6b30c738,
    0xa391,
    0x11d4,
    0x9a,
    0x3b,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

pub type ProtocolGetDriver = eficall! {fn(
    *mut Protocol,
    crate::base::Handle,
    *mut crate::base::Handle,
) -> crate::base::Status};

pub type ProtocolGetDriverPath = eficall! {fn(
    *mut Protocol,
    crate::base::Handle,
    *mut *mut crate::protocols::device_path::Protocol
) -> crate::base::Status};

pub type ProtocolDriverLoaded = eficall! {fn(
    *mut Protocol,
    crate::base::Handle,
    *mut crate::protocols::device_path::Protocol,
    crate::base::Handle,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub get_driver: ProtocolGetDriver,
    pub get_driver_path: ProtocolGetDriverPath,
    pub driver_loaded: ProtocolDriverLoaded,
}
