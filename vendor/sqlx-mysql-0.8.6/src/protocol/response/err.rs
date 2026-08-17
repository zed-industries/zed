use bytes::{Buf, Bytes};

use crate::error::Error;
use crate::io::{BufExt, ProtocolDecode};
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/dev/mysql-server/8.0.12/page_protocol_basic_err_packet.html
// https://mariadb.com/kb/en/err_packet/

/// Indicates that an error occurred.
#[derive(Debug)]
pub struct ErrPacket {
    pub error_code: u16,
    pub sql_state: Option<String>,
    pub error_message: String,
}

impl ProtocolDecode<'_, Capabilities> for ErrPacket {
    fn decode_with(mut buf: Bytes, capabilities: Capabilities) -> Result<Self, Error> {
        let header = buf.get_u8();
        if header != 0xff {
            return Err(err_protocol!(
                "expected 0xff (ERR_Packet) but found 0x{:x}",
                header
            ));
        }

        let error_code = buf.get_u16_le();
        let mut sql_state = None;

        if capabilities.contains(Capabilities::PROTOCOL_41) {
            // If the next byte is '#' then we have a SQL STATE
            if buf.starts_with(b"#") {
                buf.advance(1);
                sql_state = Some(buf.get_str(5)?);
            }
        }

        let error_message = buf.get_str(buf.len())?;

        Ok(Self {
            error_code,
            sql_state,
            error_message,
        })
    }
}

#[test]
fn test_decode_err_packet_out_of_order() {
    const ERR_PACKETS_OUT_OF_ORDER: &[u8] = b"\xff\x84\x04Got packets out of order";

    let p =
        ErrPacket::decode_with(ERR_PACKETS_OUT_OF_ORDER.into(), Capabilities::PROTOCOL_41).unwrap();

    assert_eq!(&p.error_message, "Got packets out of order");
    assert_eq!(p.error_code, 1156);
    assert_eq!(p.sql_state, None);
}

#[test]
fn test_decode_err_packet_unknown_database() {
    const ERR_HANDSHAKE_UNKNOWN_DB: &[u8] = b"\xff\x19\x04#42000Unknown database \'unknown\'";

    let p =
        ErrPacket::decode_with(ERR_HANDSHAKE_UNKNOWN_DB.into(), Capabilities::PROTOCOL_41).unwrap();

    assert_eq!(p.error_code, 1049);
    assert_eq!(p.sql_state.as_deref(), Some("42000"));
    assert_eq!(&p.error_message, "Unknown database \'unknown\'");
}
