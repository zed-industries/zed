//! Block I/O Protocol
//!
//! Used to abstract mass storage devices to allow code running in the EFI boot services environment
//! to access the storage devices without specific knowledge of the type of device or controller that
//! manages the device.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x964e5b21,
    0x6459,
    0x11d2,
    0x8e,
    0x39,
    &[0x0, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub const REVISION: u64 = 0x0000000000010000u64;
pub const REVISION2: u64 = 0x0000000000020001u64;
pub const REVISION3: u64 = 0x000000000002001fu64;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Media {
    pub media_id: u32,
    pub removable_media: bool,
    pub media_present: bool,
    pub logical_partition: bool,
    pub read_only: bool,
    pub write_caching: bool,
    pub block_size: u32,
    pub io_align: u32,
    pub last_block: crate::base::Lba,
    pub lowest_aligned_lba: crate::base::Lba,
    pub logical_blocks_per_physical_block: u32,
    pub optimal_transfer_length_granularity: u32,
}

pub type ProtocolReset = eficall! {fn(
    *mut Protocol,
    crate::base::Boolean,
) -> crate::base::Status};

pub type ProtocolReadBlocks = eficall! {fn(
    *mut Protocol,
    u32,
    crate::base::Lba,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolWriteBlocks = eficall! {fn(
    *mut Protocol,
    u32,
    crate::base::Lba,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolFlushBlocks = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub media: *const Media,
    pub reset: ProtocolReset,
    pub read_blocks: ProtocolReadBlocks,
    pub write_blocks: ProtocolWriteBlocks,
    pub flush_blocks: ProtocolFlushBlocks,
}
