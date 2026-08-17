//! Loaded Image Device Path Protocol
//!
//! The loaded image device path protocol provides the device path of a loaded image, using the
//! protocol structures of the device-path and loaded-image protocols.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xbc62157e,
    0x3e33,
    0x4fec,
    0x99,
    0x20,
    &[0x2d, 0x3b, 0x36, 0xd7, 0x50, 0xdf],
);
