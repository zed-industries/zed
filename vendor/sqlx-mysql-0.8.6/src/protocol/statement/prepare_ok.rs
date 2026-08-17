use bytes::{Buf, Bytes};

use crate::error::Error;
use crate::io::ProtocolDecode;
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/internals/en/com-stmt-prepare-response.html#packet-COM_STMT_PREPARE_OK

#[derive(Debug)]
pub(crate) struct PrepareOk {
    pub(crate) statement_id: u32,
    pub(crate) columns: u16,
    pub(crate) params: u16,
    #[allow(unused)]
    pub(crate) warnings: u16,
}

impl ProtocolDecode<'_, Capabilities> for PrepareOk {
    fn decode_with(buf: Bytes, _: Capabilities) -> Result<Self, Error> {
        const SIZE: usize = 12;

        let mut slice = buf.get(..SIZE).ok_or_else(|| {
            err_protocol!("PrepareOk expected 12 bytes but got {} bytes", buf.len())
        })?;

        let status = slice.get_u8();
        if status != 0x00 {
            return Err(err_protocol!(
                "expected 0x00 (COM_STMT_PREPARE_OK) but found 0x{:02x}",
                status
            ));
        }

        let statement_id = slice.get_u32_le();
        let columns = slice.get_u16_le();
        let params = slice.get_u16_le();

        slice.advance(1); // reserved: string<1>

        let warnings = slice.get_u16_le();

        Ok(Self {
            statement_id,
            columns,
            params,
            warnings,
        })
    }
}
