pub(crate) const fn crc8(poly: u8, reflect: bool, mut value: u8) -> u8 {
    let mut i = 0;
    if reflect {
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        while i < 8 {
            value = (value << 1) ^ (((value >> 7) & 1) * poly);
            i += 1;
        }
    }
    value
}

pub(crate) const fn crc16(poly: u16, reflect: bool, mut value: u16) -> u16 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= 8;

        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> 15) & 1) * poly);
            i += 1;
        }
    }
    value
}

pub(crate) const fn crc32(poly: u32, reflect: bool, mut value: u32) -> u32 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= 24;

        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> 31) & 1) * poly);
            i += 1;
        }
    }
    value
}

pub(crate) const fn crc64(poly: u64, reflect: bool, mut value: u64) -> u64 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= 56;

        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> 63) & 1) * poly);
            i += 1;
        }
    }
    value
}

pub(crate) const fn crc128(poly: u128, reflect: bool, mut value: u128) -> u128 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= 120;

        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> 127) & 1) * poly);
            i += 1;
        }
    }
    value
}
