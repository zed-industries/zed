//! File Protocol
//!
//! Provides an interface to interact with both files and directories. This protocol is typically
//! obtained via an EFI_SIMPLE_FILE_SYSTEM protocol or via another EFI_FILE_PROTOCOL.

pub const REVISION: u64 = 0x0000_0000_0001_0000u64;
pub const REVISION2: u64 = 0x0000_0000_0002_0000u64;
pub const LATEST_REVISION: u64 = REVISION2;

pub const MODE_READ: u64 = 0x0000000000000001u64;
pub const MODE_WRITE: u64 = 0x0000000000000002u64;
pub const MODE_CREATE: u64 = 0x8000000000000000u64;

pub const READ_ONLY: u64 = 0x0000000000000001u64;
pub const HIDDEN: u64 = 0x0000000000000002u64;
pub const SYSTEM: u64 = 0x0000000000000004u64;
pub const RESERVED: u64 = 0x0000000000000008u64;
pub const DIRECTORY: u64 = 0x0000000000000010u64;
pub const ARCHIVE: u64 = 0x0000000000000020u64;
pub const VALID_ATTR: u64 = 0x0000000000000037u64;

pub const INFO_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0x09576e92,
    0x6d3f,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const SYSTEM_INFO_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0x09576e93,
    0x6d3f,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const SYSTEM_VOLUME_LABEL_ID: crate::base::Guid = crate::base::Guid::from_fields(
    0xdb47d7d3,
    0xfe81,
    0x11d3,
    0x9a,
    0x35,
    &[0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IoToken {
    pub event: crate::base::Event,
    pub status: crate::base::Status,
    pub buffer_size: usize,
    pub buffer: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Info<const N: usize = 0> {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: crate::system::Time,
    pub last_access_time: crate::system::Time,
    pub modification_time: crate::system::Time,
    pub attribute: u64,
    pub file_name: [crate::base::Char16; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemInfo<const N: usize = 0> {
    pub size: u64,
    pub read_only: crate::base::Boolean,
    pub volume_size: u64,
    pub free_space: u64,
    pub block_size: u32,
    pub volume_label: [crate::base::Char16; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemVolumeLabel<const N: usize = 0> {
    pub volume_label: [crate::base::Char16; N],
}

pub type ProtocolOpen = eficall! {fn(
    *mut Protocol,
    *mut *mut Protocol,
    *mut crate::base::Char16,
    u64,
    u64,
) -> crate::base::Status};

pub type ProtocolClose = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

pub type ProtocolDelete = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

pub type ProtocolRead = eficall! {fn(
    *mut Protocol,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolWrite = eficall! {fn(
    *mut Protocol,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolGetPosition = eficall! {fn(
    *mut Protocol,
    *mut u64,
) -> crate::base::Status};

pub type ProtocolSetPosition = eficall! {fn(
    *mut Protocol,
    u64,
) -> crate::base::Status};

pub type ProtocolGetInfo = eficall! {fn(
    *mut Protocol,
    *mut crate::base::Guid,
    *mut usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolSetInfo = eficall! {fn(
    *mut Protocol,
    *mut crate::base::Guid,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolFlush = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

pub type ProtocolOpenEx = eficall! {fn(
    *mut Protocol,
    *mut *mut Protocol,
    *mut crate::base::Char16,
    u64,
    u64,
    *mut IoToken,
) -> crate::base::Status};

pub type ProtocolReadEx = eficall! {fn(
    *mut Protocol,
    *mut IoToken,
) -> crate::base::Status};

pub type ProtocolWriteEx = eficall! {fn(
    *mut Protocol,
    *mut IoToken,
) -> crate::base::Status};

pub type ProtocolFlushEx = eficall! {fn(
    *mut Protocol,
    *mut IoToken,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open: ProtocolOpen,
    pub close: ProtocolClose,
    pub delete: ProtocolDelete,
    pub read: ProtocolRead,
    pub write: ProtocolWrite,
    pub get_position: ProtocolGetPosition,
    pub set_position: ProtocolSetPosition,
    pub get_info: ProtocolGetInfo,
    pub set_info: ProtocolSetInfo,
    pub flush: ProtocolFlush,
    pub open_ex: ProtocolOpenEx,
    pub read_ex: ProtocolReadEx,
    pub write_ex: ProtocolWriteEx,
    pub flush_ex: ProtocolFlushEx,
}
