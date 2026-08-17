//! Simple File System Protocol
//!
//! Provides the `open_volume` function returning a file protocol representing the root directory
//! of a filesystem.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x964e5b22,
    0x6459,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub const REVISION: u64 = 0x0000000000010000u64;

pub type ProtocolOpenVolume = eficall! {fn(
    *mut Protocol,
    *mut *mut crate::protocols::file::Protocol,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open_volume: ProtocolOpenVolume,
}
