//! Driver Family Override Protocol
//!
//! When installed, the Driver Family Override Protocol informs the UEFI Boot
//! Service `ConnectController()` that this driver is higher priority than the
//! list of drivers returned by the Bus Specific Driver Override Protocol.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xb1ee129e,
    0xda36,
    0x4181,
    0x91,
    0xf8,
    &[0x04, 0xa4, 0x92, 0x37, 0x66, 0xa7],
);

pub type ProtocolGetVersion = eficall! {fn(
    *mut Protocol,
) -> u32};

#[repr(C)]
pub struct Protocol {
    pub get_version: ProtocolGetVersion,
}
