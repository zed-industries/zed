use byteorder::{ByteOrder, LittleEndian};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::protocol::text::ColumnType;
use crate::types::Type;
use crate::{MySql, MySqlTypeInfo, MySqlValueFormat, MySqlValueRef};

fn real_compatible(ty: &MySqlTypeInfo) -> bool {
    // NOTE: `DECIMAL` is explicitly excluded because floating-point numbers have different semantics.
    matches!(ty.r#type, ColumnType::Float | ColumnType::Double)
}

impl Type<MySql> for f32 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Float)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        real_compatible(ty)
    }
}

impl Type<MySql> for f64 {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::binary(ColumnType::Double)
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        real_compatible(ty)
    }
}

impl Encode<'_, MySql> for f32 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

impl Encode<'_, MySql> for f64 {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> Result<IsNull, BoxDynError> {
        buf.extend(&self.to_le_bytes());

        Ok(IsNull::No)
    }
}

impl Decode<'_, MySql> for f32 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;

                match buf.len() {
                    // These functions panic if `buf` is not exactly the right size.
                    4 => LittleEndian::read_f32(buf),
                    // MySQL can return 8-byte DOUBLE values for a FLOAT
                    // We take and truncate to f32 as that's the same behavior as *in* MySQL,
                    #[allow(clippy::cast_possible_truncation)]
                    8 => LittleEndian::read_f64(buf) as f32,
                    other => {
                        // Users may try to decode a DECIMAL as floating point;
                        // inform them why that's a bad idea.
                        return Err(format!(
                            "expected a FLOAT as 4 or 8 bytes, got {other} bytes; \
                             note that decoding DECIMAL as `f32` is not supported \
                             due to differing semantics"
                        )
                        .into());
                    }
                }
            }

            MySqlValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode<'_, MySql> for f64 {
    fn decode(value: MySqlValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(match value.format() {
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;

                // The `read_*` functions panic if `buf` is not exactly the right size.
                match buf.len() {
                    // Allow implicit widening here
                    4 => LittleEndian::read_f32(buf) as f64,
                    8 => LittleEndian::read_f64(buf),
                    other => {
                        // Users may try to decode a DECIMAL as floating point;
                        // inform them why that's a bad idea.
                        return Err(format!(
                            "expected a DOUBLE as 4 or 8 bytes, got {other} bytes; \
                             note that decoding DECIMAL as `f64` is not supported \
                             due to differing semantics"
                        )
                        .into());
                    }
                }
            }
            MySqlValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}
