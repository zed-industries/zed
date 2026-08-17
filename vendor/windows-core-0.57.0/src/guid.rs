#![allow(clippy::many_single_char_names)]

use super::*;

/// A globally unique identifier ([GUID](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid))
/// used to identify COM and WinRT interfaces.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GUID {
    /// Specifies the first 8 hexadecimal digits.
    pub data1: u32,

    /// Specifies the first group of 4 hexadecimal digits.
    pub data2: u16,

    /// Specifies the second group of 4 hexadecimal digits.
    pub data3: u16,

    /// The first 2 bytes contain the third group of 4 hexadecimal digits. The remaining 6 bytes contain the final 12 hexadecimal digits.
    pub data4: [u8; 8],
}

impl GUID {
    /// Creates a unique `GUID` value.
    pub fn new() -> Result<Self> {
        unsafe { imp::CoCreateGuid() }
    }

    /// Creates a `GUID` represented by the all-zero byte-pattern.
    pub const fn zeroed() -> Self {
        Self {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    /// Creates a `GUID` with the given constant values.
    pub const fn from_values(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    /// Creates a `GUID` from a `u128` value.
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }

    /// Converts a `GUID` to a `u128` value.
    pub const fn to_u128(&self) -> u128 {
        ((self.data1 as u128) << 96)
            + ((self.data2 as u128) << 80)
            + ((self.data3 as u128) << 64)
            + u64::from_be_bytes(self.data4) as u128
    }

    /// Creates a `GUID` for a "generic" WinRT type.
    pub const fn from_signature(signature: imp::ConstBuffer) -> Self {
        let data = imp::ConstBuffer::from_slice(&[
            0x11, 0xf4, 0x7a, 0xd5, 0x7b, 0x73, 0x42, 0xc0, 0xab, 0xae, 0x87, 0x8b, 0x1e, 0x16,
            0xad, 0xee,
        ]);

        let data = data.push_other(signature);

        let bytes = imp::sha1(&data).bytes();
        let first = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        let second = u16::from_be_bytes([bytes[4], bytes[5]]);
        let mut third = u16::from_be_bytes([bytes[6], bytes[7]]);
        third = (third & 0x0fff) | (5 << 12);
        let fourth = (bytes[8] & 0x3f) | 0x80;

        Self::from_values(
            first,
            second,
            third,
            [
                fourth, bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
            ],
        )
    }
}

impl RuntimeType for GUID {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"g16");
}

impl TypeKind for GUID {
    type TypeKind = CopyType;
}

impl core::fmt::Debug for GUID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:08X?}-{:04X?}-{:04X?}-{:02X?}{:02X?}-{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7]
        )
    }
}

impl From<&str> for GUID {
    fn from(value: &str) -> Self {
        assert!(value.len() == 36, "Invalid GUID string");
        let mut bytes = value.bytes();

        let a = ((bytes.next_u32() * 16 + bytes.next_u32()) << 24)
            + ((bytes.next_u32() * 16 + bytes.next_u32()) << 16)
            + ((bytes.next_u32() * 16 + bytes.next_u32()) << 8)
            + bytes.next_u32() * 16
            + bytes.next_u32();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let b = ((bytes.next_u16() * 16 + (bytes.next_u16())) << 8)
            + bytes.next_u16() * 16
            + bytes.next_u16();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let c = ((bytes.next_u16() * 16 + bytes.next_u16()) << 8)
            + bytes.next_u16() * 16
            + bytes.next_u16();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");
        let d = bytes.next_u8() * 16 + bytes.next_u8();
        let e = bytes.next_u8() * 16 + bytes.next_u8();
        assert!(bytes.next().unwrap() == b'-', "Invalid GUID string");

        let f = bytes.next_u8() * 16 + bytes.next_u8();
        let g = bytes.next_u8() * 16 + bytes.next_u8();
        let h = bytes.next_u8() * 16 + bytes.next_u8();
        let i = bytes.next_u8() * 16 + bytes.next_u8();
        let j = bytes.next_u8() * 16 + bytes.next_u8();
        let k = bytes.next_u8() * 16 + bytes.next_u8();

        Self::from_values(a, b, c, [d, e, f, g, h, i, j, k])
    }
}

impl From<u128> for GUID {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<GUID> for u128 {
    fn from(value: GUID) -> Self {
        value.to_u128()
    }
}

trait HexReader {
    fn next_u8(&mut self) -> u8;
    fn next_u16(&mut self) -> u16;
    fn next_u32(&mut self) -> u32;
}

impl HexReader for core::str::Bytes<'_> {
    fn next_u8(&mut self) -> u8 {
        let value = self.next().unwrap();
        match value {
            b'0'..=b'9' => value - b'0',
            b'A'..=b'F' => 10 + value - b'A',
            b'a'..=b'f' => 10 + value - b'a',
            _ => panic!(),
        }
    }

    fn next_u16(&mut self) -> u16 {
        self.next_u8().into()
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u8().into()
    }
}
