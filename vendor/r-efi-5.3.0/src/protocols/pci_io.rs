//! PCI I/O Protocol
//!
//! Used by code, typically drivers, running in the EFI boot services
//! environment to access memory and I/O on a PCI controller.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x4cf5b200,
    0x68b8,
    0x4ca5,
    0x9e,
    0xec,
    &[0xb2, 0x3e, 0x3f, 0x50, 0x02, 0x9a],
);

pub type Width = u32;

pub const WIDTH_UINT8: Width = 0x00000000;
pub const WIDTH_UINT16: Width = 0x00000001;
pub const WIDTH_UINT32: Width = 0x00000002;
pub const WIDTH_UINT64: Width = 0x00000003;
pub const WIDTH_FIFO_UINT8: Width = 0x00000004;
pub const WIDTH_FIFO_UINT16: Width = 0x00000005;
pub const WIDTH_FIFO_UINT32: Width = 0x00000006;
pub const WIDTH_FIFO_UINT64: Width = 0x00000007;
pub const WIDTH_FILL_UINT8: Width = 0x00000008;
pub const WIDTH_FILL_UINT16: Width = 0x00000009;
pub const WIDTH_FILL_UINT32: Width = 0x0000000a;
pub const WIDTH_FILL_UINT64: Width = 0x0000000b;
pub const WIDTH_MAXIMUM: Width = 0x0000000c;

pub type Operation = u32;

pub const OPERATION_BUS_MASTER_READ: Operation = 0x00000000;
pub const OPERATION_BUS_MASTER_WRITE: Operation = 0x00000001;
pub const OPERATION_BUS_MASTER_COMMON_BUFFER: Operation = 0x00000002;
pub const OPERATION_MAXIMUM: Operation = 0x00000003;

pub type Attribute = u64;

pub const ATTRIBUTE_ISA_MOTHERBOARD_IO: Attribute = 0x00000001;
pub const ATTRIBUTE_ISA_IO: Attribute = 0x00000002;
pub const ATTRIBUTE_VGA_PALETTE_IO: Attribute = 0x00000004;
pub const ATTRIBUTE_VGA_MEMORY: Attribute = 0x00000008;
pub const ATTRIBUTE_VGA_IO: Attribute = 0x00000010;
pub const ATTRIBUTE_IDE_PRIMARY_IO: Attribute = 0x00000020;
pub const ATTRIBUTE_IDE_SECONDARY_IO: Attribute = 0x00000040;
pub const ATTRIBUTE_MEMORY_WRITE_COMBINE: Attribute = 0x00000080;
pub const ATTRIBUTE_IO: Attribute = 0x00000100;
pub const ATTRIBUTE_MEMORY: Attribute = 0x00000200;
pub const ATTRIBUTE_BUS_MASTER: Attribute = 0x00000400;
pub const ATTRIBUTE_MEMORY_CACHED: Attribute = 0x00000800;
pub const ATTRIBUTE_MEMORY_DISABLE: Attribute = 0x00001000;
pub const ATTRIBUTE_EMBEDDED_DEVICE: Attribute = 0x00002000;
pub const ATTRIBUTE_EMBEDDED_ROM: Attribute = 0x00004000;
pub const ATTRIBUTE_DUAL_ADDRESS_CYCLE: Attribute = 0x00008000;
pub const ATTRIBUTE_ISA_IO_16: Attribute = 0x00010000;
pub const ATTRIBUTE_VGA_PALETTE_IO_16: Attribute = 0x00020000;
pub const ATTRIBUTE_VGA_IO_16: Attribute = 0x00040000;

pub type AttributeOperation = u32;

pub const ATTRIBUTE_OPERATION_GET: AttributeOperation = 0x00000000;
pub const ATTRIBUTE_OPERATION_SET: AttributeOperation = 0x00000001;
pub const ATTRIBUTE_OPERATION_ENABLE: AttributeOperation = 0x00000002;
pub const ATTRIBUTE_OPERATION_DISABLE: AttributeOperation = 0x00000003;
pub const ATTRIBUTE_OPERATION_SUPPORTED: AttributeOperation = 0x00000004;
pub const ATTRIBUTE_OPERATION_MAXIMUM: AttributeOperation = 0x00000005;

pub const PASS_THROUGH_BAR: u8 = 0xff;

pub type ProtocolPollIoMem = eficall! {fn(
    *mut Protocol,
    Width,
    u8,
    u64,
    u64,
    u64,
    u64,
    *mut u64,
) -> crate::base::Status};

pub type ProtocolIoMem = eficall! {fn(
    *mut Protocol,
    Width,
    u8,
    u64,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolConfig = eficall! {fn(
    *mut Protocol,
    Width,
    u32,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolCopyMem = eficall! {fn(
    *mut Protocol,
    Width,
    u8,
    u64,
    u8,
    u64,
    usize,
) -> crate::base::Status};

pub type ProtocolMap = eficall! {fn(
    *mut Protocol,
    Operation,
    *mut core::ffi::c_void,
    *mut usize,
    *mut crate::base::PhysicalAddress,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolUnmap = eficall! {fn(
    *mut Protocol,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolAllocateBuffer = eficall! {fn(
    *mut Protocol,
    crate::system::AllocateType,
    crate::system::MemoryType,
    usize,
    *mut *mut core::ffi::c_void,
    Attribute,
) -> crate::base::Status};

pub type ProtocolFreeBuffer = eficall! {fn(
    *mut Protocol,
    usize,
    *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolFlush = eficall! {fn(
    *mut Protocol,
) -> crate::base::Status};

pub type ProtocolGetLocation = eficall! {fn(
    *mut Protocol,
    *mut usize,
    *mut usize,
    *mut usize,
    *mut usize,
) -> crate::base::Status};

pub type ProtocolAttributes = eficall! {fn(
    *mut Protocol,
    AttributeOperation,
    Attribute,
    *mut Attribute,
) -> crate::base::Status};

pub type ProtocolGetBarAttributes = eficall! {fn(
    *mut Protocol,
    u8,
    *mut Attribute,
    *mut *mut core::ffi::c_void,
) -> crate::base::Status};

pub type ProtocolSetBarAttributes = eficall! {fn(
    *mut Protocol,
    Attribute,
    u8,
    *mut u64,
    *mut u64,
) -> crate::base::Status};

#[repr(C)]
pub struct Access {
    pub read: ProtocolIoMem,
    pub write: ProtocolIoMem,
}

#[repr(C)]
pub struct ConfigAccess {
    pub read: ProtocolConfig,
    pub write: ProtocolConfig,
}

#[repr(C)]
pub struct Protocol {
    pub poll_mem: ProtocolPollIoMem,
    pub poll_io: ProtocolPollIoMem,
    pub mem: Access,
    pub io: Access,
    pub pci: ConfigAccess,
    pub copy_mem: ProtocolCopyMem,
    pub map: ProtocolMap,
    pub unmap: ProtocolUnmap,
    pub allocate_buffer: ProtocolAllocateBuffer,
    pub free_buffer: ProtocolFreeBuffer,
    pub flush: ProtocolFlush,
    pub get_location: ProtocolGetLocation,
    pub attributes: ProtocolAttributes,
    pub get_bar_attributes: ProtocolGetBarAttributes,
    pub set_bar_attributes: ProtocolSetBarAttributes,
    pub rom_size: u64,
    pub rom_image: *mut core::ffi::c_void,
}
