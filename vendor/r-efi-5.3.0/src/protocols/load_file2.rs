//! Load File 2 Protocol
//!
//! The Load File 2 protocol is used to obtain files from arbitrary devices
//! that are not boot options.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x4006c0c1,
    0xfcb3,
    0x403e,
    0x99,
    0x6d,
    &[0x4a, 0x6c, 0x87, 0x24, 0xe0, 0x6d],
);

pub type Protocol = crate::protocols::load_file::Protocol;
