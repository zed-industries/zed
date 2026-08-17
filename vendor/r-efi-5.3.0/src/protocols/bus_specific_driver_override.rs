//! Bus Specific Driver Override Protocol
//!
//! This protocol matches one or more drivers to a controller. This protocol is
//! produced by a bus driver, and it is installed on the child handles of buses
//! that require a bus specific algorithm for matching drivers to controllers.
//! This protocol is used by the `EFI_BOOT_SERVICES.ConnectController()` boot
//! service to select the best driver for a controller. All of the drivers
//! returned by this protocol have a higher precedence than drivers found in
//! the general EFI Driver Binding search algorithm, but a lower precedence
//! than those drivers returned by the EFI Platform Driver Override Protocol.
//! If more than one driver image handle is returned by this protocol, then the
//! drivers image handles are returned in order from highest precedence to
//! lowest precedence.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x3bc1b285,
    0x8a15,
    0x4a82,
    0xaa,
    0xbf,
    &[0x4d, 0x7d, 0x13, 0xfb, 0x32, 0x65],
);

pub type ProtocolGetDriver = eficall! {fn(
    *mut Protocol,
    *mut crate::base::Handle,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub get_driver: ProtocolGetDriver,
}
