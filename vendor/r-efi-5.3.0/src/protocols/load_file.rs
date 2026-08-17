//! Load File Protocol
//!
//! The Load File protocol is used to obtain files, that are primarily boot
//! options, from arbitrary devices.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x56ec3091,
    0x954c,
    0x11d2,
    0x8e,
    0x3f,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub type ProtocolLoadFile = eficall! {fn(
  *mut Protocol,
  *mut crate::protocols::device_path::Protocol,
  crate::base::Boolean,
  *mut usize,
  *mut core::ffi::c_void
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub load_file: ProtocolLoadFile,
}
