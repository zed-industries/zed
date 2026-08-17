use crate::io::ProtocolEncode;
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/dev/mysql-server/8.0.12/page_protocol_connection_phase_packets_protocol_handshake_response.html
// https://dev.mysql.com/doc/internals/en/connection-phase-packets.html#packet-Protocol::SSLRequest

#[derive(Debug)]
pub struct SslRequest {
    pub max_packet_size: u32,
    pub collation: u8,
}

impl ProtocolEncode<'_, Capabilities> for SslRequest {
    fn encode_with(&self, buf: &mut Vec<u8>, context: Capabilities) -> Result<(), crate::Error> {
        // truncation is intended
        #[allow(clippy::cast_possible_truncation)]
        buf.extend(&(context.bits() as u32).to_le_bytes());
        buf.extend(&self.max_packet_size.to_le_bytes());
        buf.push(self.collation);

        // reserved: string<19>
        buf.extend(&[0_u8; 19]);

        if context.contains(Capabilities::MYSQL) {
            // reserved: string<4>
            buf.extend(&[0_u8; 4]);
        } else {
            // extended client capabilities (MariaDB-specified): int<4>
            buf.extend(&((context.bits() >> 32) as u32).to_le_bytes());
        }

        Ok(())
    }
}
