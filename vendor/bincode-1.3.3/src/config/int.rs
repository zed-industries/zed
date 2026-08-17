use std::io::Write;
use std::mem::size_of;

use super::Options;
use de::read::BincodeRead;
use error::{ErrorKind, Result};

pub trait IntEncoding {
    /// Gets the size (in bytes) that a value would be serialized to.
    fn u16_size(n: u16) -> u64;
    /// Gets the size (in bytes) that a value would be serialized to.
    fn u32_size(n: u32) -> u64;
    /// Gets the size (in bytes) that a value would be serialized to.
    fn u64_size(n: u64) -> u64;

    /// Gets the size (in bytes) that a value would be serialized to.
    fn i16_size(n: i16) -> u64;
    /// Gets the size (in bytes) that a value would be serialized to.
    fn i32_size(n: i32) -> u64;
    /// Gets the size (in bytes) that a value would be serialized to.
    fn i64_size(n: i64) -> u64;

    #[inline(always)]
    fn len_size(len: usize) -> u64 {
        Self::u64_size(len as u64)
    }

    /// Serializes a sequence length.
    #[inline(always)]
    fn serialize_len<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        len: usize,
    ) -> Result<()> {
        Self::serialize_u64(ser, len as u64)
    }

    fn serialize_u16<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        val: u16,
    ) -> Result<()>;

    fn serialize_u32<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        val: u32,
    ) -> Result<()>;

    fn serialize_u64<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        val: u64,
    ) -> Result<()>;

    fn serialize_i16<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        val: i16,
    ) -> Result<()>;

    fn serialize_i32<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        val: i32,
    ) -> Result<()>;

    fn serialize_i64<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        val: i64,
    ) -> Result<()>;

    /// Deserializes a sequence length.
    #[inline(always)]
    fn deserialize_len<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<usize> {
        Self::deserialize_u64(de).and_then(cast_u64_to_usize)
    }

    fn deserialize_u16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<u16>;

    fn deserialize_u32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<u32>;

    fn deserialize_u64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<u64>;

    fn deserialize_i16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<i16>;

    fn deserialize_i32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<i32>;

    fn deserialize_i64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<i64>;

    serde_if_integer128! {
        fn u128_size(v: u128) -> u64;
        fn i128_size(v: i128) -> u64;
        fn serialize_u128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: u128,
        ) -> Result<()>;
        fn deserialize_u128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<u128>;
        fn serialize_i128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: i128,
        ) -> Result<()>;
        fn deserialize_i128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<i128>;
    }
}

/// Fixed-size integer encoding.
///
/// * Fixed size integers are encoded directly
/// * Enum discriminants are encoded as u32
/// * Lengths and usize are encoded as u64
#[derive(Copy, Clone)]
pub struct FixintEncoding;

/// Variable-size integer encoding (excepting [ui]8).
///
/// Encoding an unsigned integer v (of any type excepting u8) works as follows:
///
/// 1. If `u < 251`, encode it as a single byte with that value.
/// 2. If `251 <= u < 2**16`, encode it as a literal byte 251, followed by a u16 with value `u`.
/// 3. If `2**16 <= u < 2**32`, encode it as a literal byte 252, followed by a u32 with value `u`.
/// 4. If `2**32 <= u < 2**64`, encode it as a literal byte 253, followed by a u64 with value `u`.
/// 5. If `2**64 <= u < 2**128`, encode it as a literal byte 254, followed by a
///   u128 with value `u`.
///
/// Then, for signed integers, we first convert to unsigned using the zigzag algorithm,
/// and then encode them as we do for unsigned integers generally. The reason we use this
/// algorithm is that it encodes those values which are close to zero in less bytes; the
/// obvious algorithm, where we encode the cast values, gives a very large encoding for all
/// negative values.
///
/// The zigzag algorithm is defined as follows:
///
/// ```ignore
/// fn zigzag(v: Signed) -> Unsigned {
///     match v {
///         0 => 0,
///         v if v < 0 => |v| * 2 - 1
///         v if v > 0 => v * 2
///     }
/// }
/// ```
///
/// And works such that:
///
/// ```ignore
/// assert_eq!(zigzag(0), 0);
/// assert_eq!(zigzag(-1), 1);
/// assert_eq!(zigzag(1), 2);
/// assert_eq!(zigzag(-2), 3);
/// assert_eq!(zigzag(2), 4);
/// assert_eq!(zigzag(i64::min_value()), u64::max_value());
/// ```
///
/// Note that u256 and the like are unsupported by this format; if and when they are added to the
/// language, they may be supported via the extension point given by the 255 byte.
#[derive(Copy, Clone)]
pub struct VarintEncoding;

const SINGLE_BYTE_MAX: u8 = 250;
const U16_BYTE: u8 = 251;
const U32_BYTE: u8 = 252;
const U64_BYTE: u8 = 253;
const U128_BYTE: u8 = 254;
const DESERIALIZE_EXTENSION_POINT_ERR: &str = r#"
Byte 255 is treated as an extension point; it should not be encoding anything.
Do you have a mismatched bincode version or configuration?
"#;

impl VarintEncoding {
    fn varint_size(n: u64) -> u64 {
        if n <= SINGLE_BYTE_MAX as u64 {
            1
        } else if n <= u16::max_value() as u64 {
            (1 + size_of::<u16>()) as u64
        } else if n <= u32::max_value() as u64 {
            (1 + size_of::<u32>()) as u64
        } else {
            (1 + size_of::<u64>()) as u64
        }
    }

    #[inline(always)]
    fn zigzag_encode(n: i64) -> u64 {
        if n < 0 {
            // let's avoid the edge case of i64::min_value()
            // !n is equal to `-n - 1`, so this is:
            // !n * 2 + 1 = 2(-n - 1) + 1 = -2n - 2 + 1 = -2n - 1
            !(n as u64) * 2 + 1
        } else {
            (n as u64) * 2
        }
    }

    #[inline(always)]
    fn zigzag_decode(n: u64) -> i64 {
        if n % 2 == 0 {
            // positive number
            (n / 2) as i64
        } else {
            // negative number
            // !m * 2 + 1 = n
            // !m * 2 = n - 1
            // !m = (n - 1) / 2
            // m = !((n - 1) / 2)
            // since we have n is odd, we have floor(n / 2) = floor((n - 1) / 2)
            !(n / 2) as i64
        }
    }

    fn serialize_varint<W: Write, O: Options>(
        ser: &mut ::ser::Serializer<W, O>,
        n: u64,
    ) -> Result<()> {
        if n <= SINGLE_BYTE_MAX as u64 {
            ser.serialize_byte(n as u8)
        } else if n <= u16::max_value() as u64 {
            ser.serialize_byte(U16_BYTE)?;
            ser.serialize_literal_u16(n as u16)
        } else if n <= u32::max_value() as u64 {
            ser.serialize_byte(U32_BYTE)?;
            ser.serialize_literal_u32(n as u32)
        } else {
            ser.serialize_byte(U64_BYTE)?;
            ser.serialize_literal_u64(n as u64)
        }
    }

    fn deserialize_varint<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::de::Deserializer<R, O>,
    ) -> Result<u64> {
        #[allow(ellipsis_inclusive_range_patterns)]
        match de.deserialize_byte()? {
            byte @ 0...SINGLE_BYTE_MAX => Ok(byte as u64),
            U16_BYTE => Ok(de.deserialize_literal_u16()? as u64),
            U32_BYTE => Ok(de.deserialize_literal_u32()? as u64),
            U64_BYTE => de.deserialize_literal_u64(),
            U128_BYTE => Err(Box::new(ErrorKind::Custom(
                "Invalid value (u128 range): you may have a version or configuration disagreement?"
                    .to_string(),
            ))),
            _ => Err(Box::new(ErrorKind::Custom(
                DESERIALIZE_EXTENSION_POINT_ERR.to_string(),
            ))),
        }
    }

    serde_if_integer128! {
        // see zigzag_encode and zigzag_decode for implementation comments
        #[inline(always)]
        fn zigzag128_encode(n: i128) -> u128 {
            if n < 0 {
                !(n as u128) * 2 + 1
            } else {
                (n as u128) * 2
            }
        }
        #[inline(always)]
        fn zigzag128_decode(n: u128) -> i128 {
            if n % 2 == 0 {
                (n / 2) as i128
            } else {
                !(n / 2) as i128
            }
        }

        fn varint128_size(n: u128) -> u64 {
            if n <= SINGLE_BYTE_MAX as u128 {
                1
            } else if n <= u16::max_value() as u128 {
                (1 + size_of::<u16>()) as u64
            } else if n <= u32::max_value() as u128 {
                (1 + size_of::<u32>()) as u64
            } else if n <= u64::max_value() as u128 {
                (1 + size_of::<u64>()) as u64
            } else {
                (1 + size_of::<u128>()) as u64
            }
        }

        fn serialize_varint128<W: Write, O: Options>(
            ser: &mut ::ser::Serializer<W, O>,
            n: u128,
        ) -> Result<()> {
            if n <= SINGLE_BYTE_MAX as u128 {
                ser.serialize_byte(n as u8)
            } else if n <= u16::max_value() as u128 {
                ser.serialize_byte(U16_BYTE)?;
                ser.serialize_literal_u16(n as u16)
            } else if n <= u32::max_value() as u128 {
                ser.serialize_byte(U32_BYTE)?;
                ser.serialize_literal_u32(n as u32)
            } else if n <= u64::max_value() as u128 {
                ser.serialize_byte(U64_BYTE)?;
                ser.serialize_literal_u64(n as u64)
            } else {
                ser.serialize_byte(U128_BYTE)?;
                ser.serialize_literal_u128(n)
            }
        }

        fn deserialize_varint128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::de::Deserializer<R, O>,
        ) -> Result<u128> {
            #[allow(ellipsis_inclusive_range_patterns)]
            match de.deserialize_byte()? {
                byte @ 0...SINGLE_BYTE_MAX => Ok(byte as u128),
                U16_BYTE => Ok(de.deserialize_literal_u16()? as u128),
                U32_BYTE => Ok(de.deserialize_literal_u32()? as u128),
                U64_BYTE => Ok(de.deserialize_literal_u64()? as u128),
                U128_BYTE => de.deserialize_literal_u128(),
                _ => Err(Box::new(ErrorKind::Custom(DESERIALIZE_EXTENSION_POINT_ERR.to_string()))),
            }
        }
    }
}

impl IntEncoding for FixintEncoding {
    #[inline(always)]
    fn u16_size(_: u16) -> u64 {
        size_of::<u16>() as u64
    }
    #[inline(always)]
    fn u32_size(_: u32) -> u64 {
        size_of::<u32>() as u64
    }
    #[inline(always)]
    fn u64_size(_: u64) -> u64 {
        size_of::<u64>() as u64
    }

    #[inline(always)]
    fn i16_size(_: i16) -> u64 {
        size_of::<i16>() as u64
    }
    #[inline(always)]
    fn i32_size(_: i32) -> u64 {
        size_of::<i32>() as u64
    }
    #[inline(always)]
    fn i64_size(_: i64) -> u64 {
        size_of::<i64>() as u64
    }

    #[inline(always)]
    fn serialize_u16<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u16) -> Result<()> {
        ser.serialize_literal_u16(val)
    }
    #[inline(always)]
    fn serialize_u32<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u32) -> Result<()> {
        ser.serialize_literal_u32(val)
    }
    #[inline(always)]
    fn serialize_u64<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u64) -> Result<()> {
        ser.serialize_literal_u64(val)
    }

    #[inline(always)]
    fn serialize_i16<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i16) -> Result<()> {
        ser.serialize_literal_u16(val as u16)
    }
    #[inline(always)]
    fn serialize_i32<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i32) -> Result<()> {
        ser.serialize_literal_u32(val as u32)
    }
    #[inline(always)]
    fn serialize_i64<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i64) -> Result<()> {
        ser.serialize_literal_u64(val as u64)
    }

    #[inline(always)]
    fn deserialize_u16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u16> {
        de.deserialize_literal_u16()
    }
    #[inline(always)]
    fn deserialize_u32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u32> {
        de.deserialize_literal_u32()
    }
    #[inline(always)]
    fn deserialize_u64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u64> {
        de.deserialize_literal_u64()
    }

    #[inline(always)]
    fn deserialize_i16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i16> {
        Ok(de.deserialize_literal_u16()? as i16)
    }
    #[inline(always)]
    fn deserialize_i32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i32> {
        Ok(de.deserialize_literal_u32()? as i32)
    }
    #[inline(always)]
    fn deserialize_i64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i64> {
        Ok(de.deserialize_literal_u64()? as i64)
    }

    serde_if_integer128! {
        #[inline(always)]
        fn u128_size(_: u128) -> u64{
            size_of::<u128>() as u64
        }
        #[inline(always)]
        fn i128_size(_: i128) -> u64{
            size_of::<i128>() as u64
        }

        #[inline(always)]
        fn serialize_u128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: u128,
        ) -> Result<()> {
            ser.serialize_literal_u128(val)
        }
        #[inline(always)]
        fn serialize_i128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: i128,
        ) -> Result<()> {
            ser.serialize_literal_u128(val as u128)
        }
        #[inline(always)]
        fn deserialize_u128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<u128> {
            de.deserialize_literal_u128()
        }
        #[inline(always)]
        fn deserialize_i128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<i128> {
            Ok(de.deserialize_literal_u128()? as i128)
        }
    }
}

impl IntEncoding for VarintEncoding {
    #[inline(always)]
    fn u16_size(n: u16) -> u64 {
        Self::varint_size(n as u64)
    }
    #[inline(always)]
    fn u32_size(n: u32) -> u64 {
        Self::varint_size(n as u64)
    }
    #[inline(always)]
    fn u64_size(n: u64) -> u64 {
        Self::varint_size(n)
    }

    #[inline(always)]
    fn i16_size(n: i16) -> u64 {
        Self::varint_size(Self::zigzag_encode(n as i64))
    }
    #[inline(always)]
    fn i32_size(n: i32) -> u64 {
        Self::varint_size(Self::zigzag_encode(n as i64))
    }
    #[inline(always)]
    fn i64_size(n: i64) -> u64 {
        Self::varint_size(Self::zigzag_encode(n))
    }

    #[inline(always)]
    fn serialize_u16<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u16) -> Result<()> {
        Self::serialize_varint(ser, val as u64)
    }
    #[inline(always)]
    fn serialize_u32<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u32) -> Result<()> {
        Self::serialize_varint(ser, val as u64)
    }
    #[inline(always)]
    fn serialize_u64<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: u64) -> Result<()> {
        Self::serialize_varint(ser, val)
    }

    #[inline(always)]
    fn serialize_i16<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i16) -> Result<()> {
        Self::serialize_varint(ser, Self::zigzag_encode(val as i64))
    }
    #[inline(always)]
    fn serialize_i32<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i32) -> Result<()> {
        Self::serialize_varint(ser, Self::zigzag_encode(val as i64))
    }
    #[inline(always)]
    fn serialize_i64<W: Write, O: Options>(ser: &mut ::Serializer<W, O>, val: i64) -> Result<()> {
        Self::serialize_varint(ser, Self::zigzag_encode(val))
    }

    #[inline(always)]
    fn deserialize_u16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u16> {
        Self::deserialize_varint(de).and_then(cast_u64_to_u16)
    }
    #[inline(always)]
    fn deserialize_u32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u32> {
        Self::deserialize_varint(de).and_then(cast_u64_to_u32)
    }
    #[inline(always)]
    fn deserialize_u64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<u64> {
        Self::deserialize_varint(de)
    }

    #[inline(always)]
    fn deserialize_i16<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i16> {
        Self::deserialize_varint(de)
            .map(Self::zigzag_decode)
            .and_then(cast_i64_to_i16)
    }
    #[inline(always)]
    fn deserialize_i32<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i32> {
        Self::deserialize_varint(de)
            .map(Self::zigzag_decode)
            .and_then(cast_i64_to_i32)
    }
    #[inline(always)]
    fn deserialize_i64<'de, R: BincodeRead<'de>, O: Options>(
        de: &mut ::Deserializer<R, O>,
    ) -> Result<i64> {
        Self::deserialize_varint(de).map(Self::zigzag_decode)
    }

    serde_if_integer128! {
        #[inline(always)]
        fn u128_size(n: u128) -> u64 {
            Self::varint128_size(n)
        }
        #[inline(always)]
        fn i128_size(n: i128) -> u64 {
            Self::varint128_size(Self::zigzag128_encode(n))
        }
        #[inline(always)]
        fn serialize_u128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: u128,
        ) -> Result<()> {
            Self::serialize_varint128(ser, val)
        }
        #[inline(always)]
        fn serialize_i128<W: Write, O: Options>(
            ser: &mut ::Serializer<W, O>,
            val: i128,
        ) -> Result<()> {
            Self::serialize_varint128(ser, Self::zigzag128_encode(val))
        }
        #[inline(always)]
        fn deserialize_u128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<u128> {
            Self::deserialize_varint128(de)
        }
        #[inline(always)]
        fn deserialize_i128<'de, R: BincodeRead<'de>, O: Options>(
            de: &mut ::Deserializer<R, O>,
        ) -> Result<i128> {
            Self::deserialize_varint128(de).map(Self::zigzag128_decode)
        }
    }
}

fn cast_u64_to_usize(n: u64) -> Result<usize> {
    if n <= usize::max_value() as u64 {
        Ok(n as usize)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid size {}: sizes must fit in a usize (0 to {})",
            n,
            usize::max_value()
        ))))
    }
}
fn cast_u64_to_u32(n: u64) -> Result<u32> {
    if n <= u32::max_value() as u64 {
        Ok(n as u32)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid u32 {}: you may have a version disagreement?",
            n,
        ))))
    }
}
fn cast_u64_to_u16(n: u64) -> Result<u16> {
    if n <= u16::max_value() as u64 {
        Ok(n as u16)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid u16 {}: you may have a version disagreement?",
            n,
        ))))
    }
}

fn cast_i64_to_i32(n: i64) -> Result<i32> {
    if n <= i32::max_value() as i64 && n >= i32::min_value() as i64 {
        Ok(n as i32)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid i32 {}: you may have a version disagreement?",
            n,
        ))))
    }
}

fn cast_i64_to_i16(n: i64) -> Result<i16> {
    if n <= i16::max_value() as i64 && n >= i16::min_value() as i64 {
        Ok(n as i16)
    } else {
        Err(Box::new(ErrorKind::Custom(format!(
            "Invalid i16 {}: you may have a version disagreement?",
            n,
        ))))
    }
}

#[cfg(test)]
mod test {
    use super::VarintEncoding;

    #[test]
    fn test_zigzag_encode() {
        let zigzag = VarintEncoding::zigzag_encode;

        assert_eq!(zigzag(0), 0);
        for x in 1..512 {
            assert_eq!(zigzag(x), (x as u64) * 2);
            assert_eq!(zigzag(-x), (x as u64) * 2 - 1);
        }
    }

    #[test]
    fn test_zigzag_decode() {
        // zigzag'
        let zigzagp = VarintEncoding::zigzag_decode;
        for x in (0..512).map(|x| x * 2) {
            assert_eq!(zigzagp(x), x as i64 / 2);
            assert_eq!(zigzagp(x + 1), -(x as i64) / 2 - 1);
        }
    }

    #[test]
    fn test_zigzag_edge_cases() {
        let (zigzag, zigzagp) = (VarintEncoding::zigzag_encode, VarintEncoding::zigzag_decode);

        assert_eq!(zigzag(i64::max_value()), u64::max_value() - 1);
        assert_eq!(zigzag(i64::min_value()), u64::max_value());

        assert_eq!(zigzagp(u64::max_value() - 1), i64::max_value());
        assert_eq!(zigzagp(u64::max_value()), i64::min_value());
    }
}
