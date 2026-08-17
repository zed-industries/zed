// Take a look at the license at the top of the repository in the LICENSE file.

pub trait SMBIOSType {
    fn length(&self) -> u8;
}

// Described in table 11 in the standard
// little endian: time_low, time_mid, time_hi_and_version
// big endian: time_hi_and_version, clock_seq_low, node
#[repr(C, packed)]
pub struct SMBIOSUuid {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq_hi_and_reserved: u8,
    pub clock_seq_low: u8,
    pub node: [u8; 6],
}

impl std::fmt::Display for SMBIOSUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            u32::from_le(self.time_low),
            u16::from_le(self.time_mid),
            u16::from_le(self.time_hi_and_version),
            self.clock_seq_hi_and_reserved,
            self.clock_seq_low,
            self.node[0],
            self.node[1],
            self.node[2],
            self.node[3],
            self.node[4],
            self.node[5],
        )
    }
}

#[repr(C, packed)]
pub(crate) struct SMBIOSSystemInformation {
    pub(crate) _type: u8,
    pub(crate) length: u8,
    pub(crate) _handle: u16,
    pub(crate) manufacturer: u8,
    pub(crate) product_name: u8,
    pub(crate) version: u8,
    pub(crate) serial_number: u8,
    pub(crate) uuid: SMBIOSUuid,
    pub(crate) wake_up_type: u8,
    pub(crate) sku_number: u8,
    pub(crate) family: u8,
}

impl SMBIOSType for SMBIOSSystemInformation {
    fn length(&self) -> u8 {
        self.length
    }
}

#[repr(C, packed)]
pub(crate) struct SMBIOSBaseboardInformation {
    pub(crate) _type: u8,
    pub(crate) length: u8,
    pub(crate) _handle: u16,
    pub(crate) manufacturer: u8,
    pub(crate) product_name: u8,
    pub(crate) version: u8,
    pub(crate) serial_number: u8,
    pub(crate) asset_tag: u8,
}

impl SMBIOSType for SMBIOSBaseboardInformation {
    fn length(&self) -> u8 {
        self.length
    }
}
