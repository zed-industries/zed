use bytes::{Buf, Bytes};

use crate::column::MySqlColumn;
use crate::error::Error;
use crate::io::MySqlBufExt;
use crate::io::ProtocolDecode;
use crate::protocol::Row;

#[derive(Debug)]
pub(crate) struct TextRow(pub(crate) Row);

impl<'de> ProtocolDecode<'de, &'de [MySqlColumn]> for TextRow {
    fn decode_with(mut buf: Bytes, columns: &'de [MySqlColumn]) -> Result<Self, Error> {
        let storage = buf.clone();
        let offset = buf.len();

        let mut values = Vec::with_capacity(columns.len());

        for c in columns {
            if buf[0] == 0xfb {
                // NULL is sent as 0xfb
                values.push(None);
                buf.advance(1);
            } else {
                let size = buf.get_uint_lenenc();
                if (buf.remaining() as u64) < size {
                    return Err(err_protocol!(
                        "buffer exhausted when reading data for column {:?}; decoded length is {}, but only {} bytes remain in buffer. Malformed packet or protocol error?",
                        c,
                        size,
                        buf.remaining()));
                }
                let size = usize::try_from(size)
                    .map_err(|_| err_protocol!("TextRow length out of range: {size}"))?;

                let offset = offset - buf.len();

                values.push(Some(offset..(offset + size)));

                buf.advance(size);
            }
        }

        Ok(TextRow(Row { values, storage }))
    }
}
