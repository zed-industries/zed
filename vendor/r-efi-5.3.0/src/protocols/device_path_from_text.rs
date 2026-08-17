//! Device Path From Text Protocol
//!
//! Convert text to device paths and device nodes.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x5c99a21,
    0xc70f,
    0x4ad2,
    0x8a,
    0x5f,
    &[0x35, 0xdf, 0x33, 0x43, 0xf5, 0x1e],
);

pub type DevicePathFromTextNode = eficall! {fn(
    *const crate::base::Char16,
) -> *mut crate::protocols::device_path::Protocol};

pub type DevicePathFromTextPath = eficall! {fn(
    *const crate::base::Char16,
) -> *mut crate::protocols::device_path::Protocol};

#[repr(C)]
pub struct Protocol {
    pub convert_text_to_device_node: DevicePathFromTextNode,
    pub convert_text_to_device_path: DevicePathFromTextPath,
}
