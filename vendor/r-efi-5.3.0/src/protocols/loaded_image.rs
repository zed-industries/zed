//! Loaded Image Protocol
//!
//! The loaded image protocol defines how to obtain information about a loaded image from an
//! image handle.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x5b1b31a1,
    0x9562,
    0x11d2,
    0x8e,
    0x3f,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub const REVISION: u32 = 0x00001000u32;

pub type ProtocolUnload = eficall! {fn(
    crate::base::Handle,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u32,
    pub parent_handle: crate::base::Handle,
    pub system_table: *mut crate::system::SystemTable,

    pub device_handle: crate::base::Handle,
    pub file_path: *mut crate::protocols::device_path::Protocol,
    pub reserved: *mut core::ffi::c_void,

    pub load_options_size: u32,
    pub load_options: *mut core::ffi::c_void,

    pub image_base: *mut core::ffi::c_void,
    pub image_size: u64,
    pub image_code_type: crate::system::MemoryType,
    pub image_data_type: crate::system::MemoryType,
    pub unload: Option<ProtocolUnload>,
}
