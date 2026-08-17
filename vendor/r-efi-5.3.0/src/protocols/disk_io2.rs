//! Disk I/O 2 Protocol
//!
//! Extends the Disk I/O protocol interface to enable non-blocking /
//! asynchronous byte-oriented disk operation.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x151c8eae,
    0x7f2c,
    0x472c,
    0x9e,
    0x54,
    &[0x98, 0x28, 0x19, 0x4f, 0x6a, 0x88],
);

pub const REVISION: u64 = 0x0000000000020000u64;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Token {
    event: crate::base::Event,
    transaction_status: crate::base::Status,
}

pub type ProtocolCancel = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

pub type ProtocolReadDiskEx = eficall! {fn(
    *mut Protocol,
    u32,
    u64,
    *mut Token,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolWriteDiskEx = eficall! {fn(
    *mut Protocol,
    u32,
    u64,
    *mut Token,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolFlushDiskEx = eficall! {fn(
    *mut Protocol,
    *mut Token,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub cancel: ProtocolCancel,
    pub read_disk_ex: ProtocolReadDiskEx,
    pub write_disk_ex: ProtocolWriteDiskEx,
    pub flush_disk_ex: ProtocolFlushDiskEx,
}
