use bytes::{Buf, Bytes};

use crate::error::Error;
use crate::io::MySqlBufExt;
use crate::io::{BufExt, ProtocolDecode};
use crate::protocol::text::ColumnType;
use crate::protocol::Row;
use crate::MySqlColumn;

// https://dev.mysql.com/doc/internals/en/binary-protocol-resultset-row.html#packet-ProtocolBinary::ResultsetRow
// https://dev.mysql.com/doc/internals/en/binary-protocol-value.html

#[derive(Debug)]
pub(crate) struct BinaryRow(pub(crate) Row);

impl<'de> ProtocolDecode<'de, &'de [MySqlColumn]> for BinaryRow {
    fn decode_with(mut buf: Bytes, columns: &'de [MySqlColumn]) -> Result<Self, Error> {
        let header = buf.get_u8();
        if header != 0 {
            return Err(err_protocol!(
                "exepcted 0x00 (ROW) but found 0x{:02x}",
                header
            ));
        }

        let storage = buf.clone();
        let offset = buf.len();

        let null_bitmap_len = (columns.len() + 9) / 8;
        let null_bitmap = buf.get_bytes(null_bitmap_len);

        let mut values = Vec::with_capacity(columns.len());

        for (column_idx, column) in columns.iter().enumerate() {
            // NOTE: the column index starts at the 3rd bit
            let column_null_idx = column_idx + 2;

            let byte_idx = column_null_idx / 8;
            let bit_idx = column_null_idx % 8;

            let is_null = null_bitmap[byte_idx] & (1u8 << bit_idx) != 0;

            if is_null {
                values.push(None);
                continue;
            }

            // NOTE: MySQL will never generate NULL types for non-NULL values
            let type_info = &column.type_info;

            // Unlike Postgres, MySQL does not length-prefix every value in a binary row.
            // Values are *either* fixed-length or length-prefixed,
            // so we need to inspect the type code to be sure.
            let size: usize = match type_info.r#type {
                // All fixed-length types.
                ColumnType::LongLong => 8,
                ColumnType::Long | ColumnType::Int24 => 4,
                ColumnType::Short | ColumnType::Year => 2,
                ColumnType::Tiny => 1,
                ColumnType::Float => 4,
                ColumnType::Double => 8,

                // Blobs and strings are prefixed with their length,
                // which is itself a length-encoded integer.
                ColumnType::String
                | ColumnType::VarChar
                | ColumnType::VarString
                | ColumnType::Enum
                | ColumnType::Set
                | ColumnType::LongBlob
                | ColumnType::MediumBlob
                | ColumnType::Blob
                | ColumnType::TinyBlob
                | ColumnType::Geometry
                | ColumnType::Bit
                | ColumnType::Decimal
                | ColumnType::Json
                | ColumnType::NewDecimal => {
                    let size = buf.get_uint_lenenc();
                    usize::try_from(size)
                        .map_err(|_| err_protocol!("BLOB length out of range: {size}"))?
                }

                // Like strings and blobs, these values are variable-length.
                // Unlike strings and blobs, however, they exclusively use one byte for length.
                ColumnType::Time
                | ColumnType::Timestamp
                | ColumnType::Date
                | ColumnType::Datetime => {
                    // Leave the length byte on the front of the value because decoding uses it.
                    buf[0] as usize + 1
                }

                // NOTE: MySQL will never generate NULL types for non-NULL values
                ColumnType::Null => unreachable!(),
            };

            let offset = offset - buf.len();

            values.push(Some(offset..(offset + size)));

            buf.advance(size);
        }

        Ok(BinaryRow(Row { values, storage }))
    }
}
