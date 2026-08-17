//! Device Path Protocol
//!
//! The device path protocol defines how to obtain generic path/location information
//! concerning the phisycal or logical device.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x09576e91,
    0x6d3f,
    0x11d2,
    0x8e,
    0x39,
    &[0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

pub const TYPE_HARDWARE: u8 = 0x01;
pub const TYPE_ACPI: u8 = 0x02;
pub const TYPE_MESSAGING: u8 = 0x03;
pub const TYPE_MEDIA: u8 = 0x04;
pub const TYPE_BIOS: u8 = 0x05;
pub const TYPE_END: u8 = 0x7f;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Protocol {
    pub r#type: u8,
    pub sub_type: u8,
    pub length: [u8; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct End {
    pub header: Protocol,
}

impl End {
    pub const SUBTYPE_INSTANCE: u8 = 0x01;
    pub const SUBTYPE_ENTIRE: u8 = 0xff;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Hardware {
    pub header: Protocol,
}

impl Hardware {
    pub const SUBTYPE_PCI: u8 = 0x01;
    pub const SUBTYPE_PCCARD: u8 = 0x02;
    pub const SUBTYPE_MMAP: u8 = 0x03;
    pub const SUBTYPE_VENDOR: u8 = 0x04;
    pub const SUBTYPE_CONTROLLER: u8 = 0x05;
    pub const SUBTYPE_BMC: u8 = 0x06;
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct HardDriveMedia {
    pub header: Protocol,
    pub partition_number: u32,
    pub partition_start: u64,
    pub partition_size: u64,
    pub partition_signature: [u8; 16],
    pub partition_format: u8,
    pub signature_type: u8,
}

pub struct Media {
    pub header: Protocol,
}

impl Media {
    pub const SUBTYPE_HARDDRIVE: u8 = 0x01;
    pub const SUBTYPE_CDROM: u8 = 0x02;
    pub const SUBTYPE_VENDOR: u8 = 0x03;
    pub const SUBTYPE_FILE_PATH: u8 = 0x04;
    pub const SUBTYPE_MEDIA_PROTOCOL: u8 = 0x05;
    pub const SUBTYPE_PIWG_FIRMWARE_FILE: u8 = 0x06;
    pub const SUBTYPE_PIWG_FIRMWARE_VOLUME: u8 = 0x07;
    pub const SUBTYPE_RELATIVE_OFFSET_RANGE: u8 = 0x08;
    pub const SUBTYPE_RAM_DISK: u8 = 0x09;
}
