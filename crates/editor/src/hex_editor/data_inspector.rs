use anyhow::{Result, anyhow, ensure};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Endian {
    Little,
    Big,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum NumberType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
}

impl NumberType {
    pub const ALL: [Self; 10] = [
        Self::U8,
        Self::I8,
        Self::U16,
        Self::I16,
        Self::U32,
        Self::I32,
        Self::U64,
        Self::I64,
        Self::F32,
        Self::F64,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::I8 => "i8",
            Self::U16 => "u16",
            Self::I16 => "i16",
            Self::U32 => "u32",
            Self::I32 => "i32",
            Self::U64 => "u64",
            Self::I64 => "i64",
            Self::F32 => "f32",
            Self::F64 => "f64",
        }
    }

    pub fn width(self) -> usize {
        match self {
            Self::U8 | Self::I8 => 1,
            Self::U16 | Self::I16 => 2,
            Self::U32 | Self::I32 | Self::F32 => 4,
            Self::U64 | Self::I64 | Self::F64 => 8,
        }
    }

    pub fn decode(self, bytes: &[u8], endian: Endian) -> Option<String> {
        macro_rules! decode {
            ($type:ty) => {{
                let bytes = bytes.get(..self.width())?.try_into().ok()?;
                let value = match endian {
                    Endian::Little => <$type>::from_le_bytes(bytes),
                    Endian::Big => <$type>::from_be_bytes(bytes),
                };
                Some(value.to_string())
            }};
        }
        match self {
            Self::U8 => decode!(u8),
            Self::I8 => decode!(i8),
            Self::U16 => decode!(u16),
            Self::I16 => decode!(i16),
            Self::U32 => decode!(u32),
            Self::I32 => decode!(i32),
            Self::U64 => decode!(u64),
            Self::I64 => decode!(i64),
            Self::F32 => decode!(f32),
            Self::F64 => decode!(f64),
        }
    }

    pub fn encode(self, text: &str, endian: Endian) -> Result<Vec<u8>> {
        macro_rules! encode {
            ($type:ty) => {{
                let value: $type = text
                    .trim()
                    .parse()
                    .map_err(|_| anyhow!("Enter a valid {} value", self.label()))?;
                match endian {
                    Endian::Little => value.to_le_bytes().to_vec(),
                    Endian::Big => value.to_be_bytes().to_vec(),
                }
            }};
        }
        Ok(match self {
            Self::U8 => encode!(u8),
            Self::I8 => encode!(i8),
            Self::U16 => encode!(u16),
            Self::I16 => encode!(i16),
            Self::U32 => encode!(u32),
            Self::I32 => encode!(i32),
            Self::U64 => encode!(u64),
            Self::I64 => encode!(i64),
            Self::F32 => {
                let bytes = encode!(f32);
                // Parsing an overflowing decimal can otherwise silently yield infinity.
                ensure!(
                    text.trim().parse::<f32>()?.is_finite(),
                    "Enter a finite f32 value"
                );
                bytes
            }
            Self::F64 => {
                let bytes = encode!(f64);
                ensure!(
                    text.trim().parse::<f64>()?.is_finite(),
                    "Enter a finite f64 value"
                );
                bytes
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_endianness_bounds_and_round_trip() -> Result<()> {
        for (kind, value) in [
            (NumberType::U8, "255"),
            (NumberType::I8, "-128"),
            (NumberType::U16, "65535"),
            (NumberType::I16, "-32768"),
            (NumberType::U32, "4294967295"),
            (NumberType::I32, "-2147483648"),
            (NumberType::U64, "18446744073709551615"),
            (NumberType::I64, "-9223372036854775808"),
            (NumberType::F32, "1.5"),
            (NumberType::F64, "-2.25"),
        ] {
            for endian in [Endian::Little, Endian::Big] {
                let bytes = kind.encode(value, endian)?;
                assert_eq!(bytes.len(), kind.width());
                assert_eq!(kind.decode(&bytes, endian).as_deref(), Some(value));
                assert!(kind.decode(&bytes[..bytes.len() - 1], endian).is_none());
            }
        }
        assert_eq!(
            NumberType::U16.encode("4660", Endian::Little)?,
            [0x34, 0x12]
        );
        assert_eq!(NumberType::U16.encode("4660", Endian::Big)?, [0x12, 0x34]);
        for (kind, value) in [
            (NumberType::U8, "256"),
            (NumberType::U8, "-1"),
            (NumberType::I8, "128"),
            (NumberType::I8, "-129"),
            (NumberType::U64, "18446744073709551616"),
            (NumberType::F32, "1e100"),
            (NumberType::F64, "inf"),
            (NumberType::F32, "NaN"),
            (NumberType::U16, "nonsense"),
        ] {
            assert!(kind.encode(value, Endian::Little).is_err(), "{value}");
        }
        Ok(())
    }
}
