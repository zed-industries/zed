use crate::io::MySqlBufMutExt;
use crate::io::{BufMutExt, ProtocolEncode};
use crate::protocol::auth::AuthPlugin;
use crate::protocol::connect::ssl_request::SslRequest;
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/internals/en/connection-phase-packets.html#packet-Protocol::HandshakeResponse
// https://mariadb.com/kb/en/connection/#client-handshake-response

#[derive(Debug)]
pub struct HandshakeResponse<'a> {
    pub database: Option<&'a str>,

    /// Max size of a command packet that the client wants to send to the server
    pub max_packet_size: u32,

    /// Default collation for the connection
    pub collation: u8,

    /// Name of the SQL account which client wants to log in
    pub username: &'a str,

    /// Authentication method used by the client
    pub auth_plugin: Option<AuthPlugin>,

    /// Opaque authentication response
    pub auth_response: Option<&'a [u8]>,
}

impl ProtocolEncode<'_, Capabilities> for HandshakeResponse<'_> {
    fn encode_with(
        &self,
        buf: &mut Vec<u8>,
        mut context: Capabilities,
    ) -> Result<(), crate::Error> {
        if self.auth_plugin.is_none() {
            // ensure PLUGIN_AUTH is set *only* if we have a defined plugin
            context.remove(Capabilities::PLUGIN_AUTH);
        }

        // NOTE: Half of this packet is identical to the SSL Request packet
        SslRequest {
            max_packet_size: self.max_packet_size,
            collation: self.collation,
        }
        .encode_with(buf, context)?;

        buf.put_str_nul(self.username);

        if context.contains(Capabilities::PLUGIN_AUTH_LENENC_DATA) {
            buf.put_bytes_lenenc(self.auth_response.unwrap_or_default());
        } else if context.contains(Capabilities::SECURE_CONNECTION) {
            let response = self.auth_response.unwrap_or_default();

            let response_len = u8::try_from(response.len())
                .map_err(|_| err_protocol!("auth_response.len() too long: {}", response.len()))?;

            buf.push(response_len);
            buf.extend(response);
        } else {
            buf.push(0);
        }

        if context.contains(Capabilities::CONNECT_WITH_DB) {
            if let Some(database) = &self.database {
                buf.put_str_nul(database);
            } else {
                buf.push(0);
            }
        }

        if context.contains(Capabilities::PLUGIN_AUTH) {
            if let Some(plugin) = &self.auth_plugin {
                buf.put_str_nul(plugin.name());
            } else {
                buf.push(0);
            }
        }

        Ok(())
    }
}
