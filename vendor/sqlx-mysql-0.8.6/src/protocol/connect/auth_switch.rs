use bytes::{Buf, Bytes};

use crate::error::Error;
use crate::io::ProtocolEncode;
use crate::io::{BufExt, ProtocolDecode};
use crate::protocol::auth::AuthPlugin;
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/dev/mysql-server/8.0.12/page_protocol_connection_phase_packets_protocol_auth_switch_request.html

#[derive(Debug)]
pub struct AuthSwitchRequest {
    pub plugin: AuthPlugin,
    pub data: Bytes,
}

impl ProtocolDecode<'_, bool> for AuthSwitchRequest {
    fn decode_with(mut buf: Bytes, enable_cleartext_plugin: bool) -> Result<Self, Error> {
        let header = buf.get_u8();
        if header != 0xfe {
            return Err(err_protocol!(
                "expected 0xfe (AUTH_SWITCH) but found 0x{:x}",
                header
            ));
        }

        let plugin = buf.get_str_nul()?.parse()?;

        if matches!(plugin, AuthPlugin::MySqlClearPassword) && !enable_cleartext_plugin {
            return Err(err_protocol!("mysql_cleartext_plugin disabled"));
        }

        if matches!(plugin, AuthPlugin::MySqlClearPassword) && buf.is_empty() {
            // Contrary to the MySQL protocol, AWS Aurora with IAM sends
            // no data. That is fine because the mysql_clear_password says to
            // ignore any data sent.
            // See: https://dev.mysql.com/doc/dev/mysql-server/latest/page_protocol_connection_phase_authentication_methods_clear_text_password.html
            return Ok(Self {
                plugin,
                data: Bytes::new(),
            });
        }

        // See: https://github.com/mysql/mysql-server/blob/ea7d2e2d16ac03afdd9cb72a972a95981107bf51/sql/auth/sha2_password.cc#L942
        if buf.len() != 21 {
            return Err(err_protocol!(
                "expected 21 bytes but found {} bytes",
                buf.len()
            ));
        }
        let data = buf.get_bytes(20);
        buf.advance(1); // NUL-terminator

        Ok(Self { plugin, data })
    }
}

#[derive(Debug)]
pub struct AuthSwitchResponse(pub Vec<u8>);

impl ProtocolEncode<'_, Capabilities> for AuthSwitchResponse {
    fn encode_with(&self, buf: &mut Vec<u8>, _: Capabilities) -> Result<(), Error> {
        buf.extend_from_slice(&self.0);
        Ok(())
    }
}

#[test]
fn test_decode_auth_switch_packet_data() {
    const AUTH_SWITCH_NO_DATA: &[u8] = b"\xfecaching_sha2_password\x00abcdefghijabcdefghij\x00";

    let p = AuthSwitchRequest::decode_with(AUTH_SWITCH_NO_DATA.into(), true).unwrap();

    assert!(matches!(p.plugin, AuthPlugin::CachingSha2Password));
    assert_eq!(p.data, &b"abcdefghijabcdefghij"[..]);
}

#[test]
fn test_decode_auth_switch_cleartext_disabled() {
    const AUTH_SWITCH_CLEARTEXT: &[u8] = b"\xfemysql_clear_password\x00abcdefghijabcdefghij\x00";

    let e = AuthSwitchRequest::decode_with(AUTH_SWITCH_CLEARTEXT.into(), false).unwrap_err();

    let e_str = e.to_string();

    let expected = "encountered unexpected or invalid data: mysql_cleartext_plugin disabled";

    assert!(
        // Don't want to assert the full string since it contains the module path now.
        e_str.starts_with(expected),
        "expected error string to start with {expected:?}, got {e_str:?}"
    );
}

#[test]
fn test_decode_auth_switch_packet_no_data() {
    const AUTH_SWITCH_NO_DATA: &[u8] = b"\xfemysql_clear_password\x00";

    let p = AuthSwitchRequest::decode_with(AUTH_SWITCH_NO_DATA.into(), true).unwrap();

    assert!(matches!(p.plugin, AuthPlugin::MySqlClearPassword));
    assert_eq!(p.data, Bytes::new());
}
