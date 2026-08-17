//! Graphics Output Protocol
//!
//! Provides means to configure graphics hardware and get access to
//! framebuffers. Replaces the old UGA interface from EFI with a
//! VGA-independent API.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x9042a9de,
    0x23dc,
    0x4a38,
    0x96,
    0xfb,
    &[0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PixelBitmask {
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub reserved_mask: u32,
}

pub type GraphicsPixelFormat = u32;

pub const PIXEL_RED_GREEN_BLUE_RESERVED_8_BIT_PER_COLOR: GraphicsPixelFormat = 0x00000000;
pub const PIXEL_BLUE_GREEN_RED_RESERVED_8_BIT_PER_COLOR: GraphicsPixelFormat = 0x00000001;
pub const PIXEL_BIT_MASK: GraphicsPixelFormat = 0x00000002;
pub const PIXEL_BLT_ONLY: GraphicsPixelFormat = 0x00000003;
pub const PIXEL_FORMAT_MAX: GraphicsPixelFormat = 0x00000004;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ModeInformation {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: GraphicsPixelFormat,
    pub pixel_information: PixelBitmask,
    pub pixels_per_scan_line: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Mode {
    pub max_mode: u32,
    pub mode: u32,
    pub info: *mut ModeInformation,
    pub size_of_info: usize,
    pub frame_buffer_base: crate::base::PhysicalAddress,
    pub frame_buffer_size: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BltPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

pub type BltOperation = u32;

pub const BLT_VIDEO_FILL: BltOperation = 0x00000000;
pub const BLT_VIDEO_TO_BLT_BUFFER: BltOperation = 0x00000001;
pub const BLT_BUFFER_TO_VIDEO: BltOperation = 0x00000002;
pub const BLT_VIDEO_TO_VIDEO: BltOperation = 0x00000003;
pub const BLT_OPERATION_MAX: BltOperation = 0x00000004;

pub type ProtocolQueryMode = eficall! {fn(
    *mut Protocol,
    u32,
    *mut usize,
    *mut *mut ModeInformation,
) -> crate::base::Status};

pub type ProtocolSetMode = eficall! {fn(
    *mut Protocol,
    u32,
) -> crate::base::Status};

pub type ProtocolBlt = eficall! {fn(
    *mut Protocol,
    *mut BltPixel,
    BltOperation,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub query_mode: ProtocolQueryMode,
    pub set_mode: ProtocolSetMode,
    pub blt: ProtocolBlt,
    pub mode: *mut Mode,
}
