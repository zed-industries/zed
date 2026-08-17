use bytes::buf::Chain;
use bytes::{Buf, Bytes};
use std::cmp;

use crate::error::Error;
use crate::io::{BufExt, ProtocolDecode};
use crate::protocol::auth::AuthPlugin;
use crate::protocol::response::Status;
use crate::protocol::Capabilities;

// https://dev.mysql.com/doc/internals/en/connection-phase-packets.html#packet-Protocol::Handshake
// https://mariadb.com/kb/en/connection/#initial-handshake-packet

#[derive(Debug)]
pub(crate) struct Handshake {
    #[allow(unused)]
    pub(crate) protocol_version: u8,
    pub(crate) server_version: String,
    #[allow(unused)]
    pub(crate) connection_id: u32,
    pub(crate) server_capabilities: Capabilities,
    #[allow(unused)]
    pub(crate) server_default_collation: u8,
    #[allow(unused)]
    pub(crate) status: Status,
    pub(crate) auth_plugin: Option<AuthPlugin>,
    pub(crate) auth_plugin_data: Chain<Bytes, Bytes>,
}

impl ProtocolDecode<'_> for Handshake {
    fn decode_with(mut buf: Bytes, _: ()) -> Result<Self, Error> {
        let protocol_version = buf.get_u8(); // int<1>
        let server_version = buf.get_str_nul()?; // string<NUL>
        let connection_id = buf.get_u32_le(); // int<4>
        let auth_plugin_data_1 = buf.get_bytes(8); // string<8>

        buf.advance(1); // reserved: string<1>

        let capabilities_1 = buf.get_u16_le(); // int<2>
        let mut capabilities = Capabilities::from_bits_truncate(capabilities_1.into());

        let collation = buf.get_u8(); // int<1>
        let status = Status::from_bits_truncate(buf.get_u16_le());

        let capabilities_2 = buf.get_u16_le(); // int<2>
        capabilities |= Capabilities::from_bits_truncate(((capabilities_2 as u32) << 16).into());

        let auth_plugin_data_len = if capabilities.contains(Capabilities::PLUGIN_AUTH) {
            buf.get_u8()
        } else {
            buf.advance(1); // int<1>
            0
        };

        buf.advance(6); // reserved: string<6>

        if capabilities.contains(Capabilities::MYSQL) {
            buf.advance(4); // reserved: string<4>
        } else {
            let capabilities_3 = buf.get_u32_le(); // int<4>
            capabilities |= Capabilities::from_bits_truncate((capabilities_3 as u64) << 32);
        }

        let auth_plugin_data_2 = if capabilities.contains(Capabilities::SECURE_CONNECTION) {
            let len = cmp::max(auth_plugin_data_len.saturating_sub(9), 12);
            let v = buf.get_bytes(len as usize);
            buf.advance(1); // NUL-terminator

            v
        } else {
            Bytes::new()
        };

        let auth_plugin = if capabilities.contains(Capabilities::PLUGIN_AUTH) {
            Some(buf.get_str_nul()?.parse()?)
        } else {
            None
        };

        Ok(Self {
            protocol_version,
            server_version,
            connection_id,
            server_default_collation: collation,
            status,
            server_capabilities: capabilities,
            auth_plugin,
            auth_plugin_data: auth_plugin_data_1.chain(auth_plugin_data_2),
        })
    }
}

#[test]
fn test_decode_handshake_mysql_8_0_18() {
    const HANDSHAKE_MYSQL_8_0_18: &[u8] = b"\n8.0.18\x00\x19\x00\x00\x00\x114aB0c\x06g\x00\xff\xff\xff\x02\x00\xff\xc7\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00tL\x03s\x0f[4\rl4. \x00caching_sha2_password\x00";

    let p = Handshake::decode(HANDSHAKE_MYSQL_8_0_18.into()).unwrap();

    assert_eq!(p.protocol_version, 10);

    assert_eq!(
        p.server_capabilities,
        Capabilities::MYSQL
            | Capabilities::FOUND_ROWS
            | Capabilities::LONG_FLAG
            | Capabilities::CONNECT_WITH_DB
            | Capabilities::NO_SCHEMA
            | Capabilities::COMPRESS
            | Capabilities::ODBC
            | Capabilities::LOCAL_FILES
            | Capabilities::IGNORE_SPACE
            | Capabilities::PROTOCOL_41
            | Capabilities::INTERACTIVE
            | Capabilities::SSL
            | Capabilities::TRANSACTIONS
            | Capabilities::SECURE_CONNECTION
            | Capabilities::MULTI_STATEMENTS
            | Capabilities::MULTI_RESULTS
            | Capabilities::PS_MULTI_RESULTS
            | Capabilities::PLUGIN_AUTH
            | Capabilities::CONNECT_ATTRS
            | Capabilities::PLUGIN_AUTH_LENENC_DATA
            | Capabilities::CAN_HANDLE_EXPIRED_PASSWORDS
            | Capabilities::SESSION_TRACK
            | Capabilities::DEPRECATE_EOF
            | Capabilities::ZSTD_COMPRESSION_ALGORITHM
            | Capabilities::SSL_VERIFY_SERVER_CERT
            | Capabilities::OPTIONAL_RESULTSET_METADATA
            | Capabilities::REMEMBER_OPTIONS,
    );

    assert_eq!(p.server_default_collation, 255);
    assert!(p.status.contains(Status::SERVER_STATUS_AUTOCOMMIT));

    assert!(matches!(
        p.auth_plugin,
        Some(AuthPlugin::CachingSha2Password)
    ));

    assert_eq!(
        &*p.auth_plugin_data.into_iter().collect::<Vec<_>>(),
        &[17, 52, 97, 66, 48, 99, 6, 103, 116, 76, 3, 115, 15, 91, 52, 13, 108, 52, 46, 32,]
    );
}

#[test]
fn test_decode_handshake_mariadb_10_4_7() {
    const HANDSHAKE_MARIA_DB_10_4_7: &[u8] = b"\n5.5.5-10.4.7-MariaDB-1:10.4.7+maria~bionic\x00\x0b\x00\x00\x00t6L\\j\"dS\x00\xfe\xf7\x08\x02\x00\xff\x81\x15\x00\x00\x00\x00\x00\x00\x07\x00\x00\x00U14Oph9\"<H5n\x00mysql_native_password\x00";

    let p = Handshake::decode(HANDSHAKE_MARIA_DB_10_4_7.into()).unwrap();

    assert_eq!(p.protocol_version, 10);

    assert_eq!(
        &*p.server_version,
        "5.5.5-10.4.7-MariaDB-1:10.4.7+maria~bionic"
    );

    assert_eq!(
        p.server_capabilities,
        Capabilities::FOUND_ROWS
            | Capabilities::LONG_FLAG
            | Capabilities::CONNECT_WITH_DB
            | Capabilities::NO_SCHEMA
            | Capabilities::COMPRESS
            | Capabilities::ODBC
            | Capabilities::LOCAL_FILES
            | Capabilities::IGNORE_SPACE
            | Capabilities::PROTOCOL_41
            | Capabilities::INTERACTIVE
            | Capabilities::TRANSACTIONS
            | Capabilities::SECURE_CONNECTION
            | Capabilities::MULTI_STATEMENTS
            | Capabilities::MULTI_RESULTS
            | Capabilities::PS_MULTI_RESULTS
            | Capabilities::PLUGIN_AUTH
            | Capabilities::CONNECT_ATTRS
            | Capabilities::PLUGIN_AUTH_LENENC_DATA
            | Capabilities::CAN_HANDLE_EXPIRED_PASSWORDS
            | Capabilities::SESSION_TRACK
            | Capabilities::DEPRECATE_EOF
            | Capabilities::REMEMBER_OPTIONS
            | Capabilities::MARIADB_CLIENT_PROGRESS
            | Capabilities::MARIADB_CLIENT_MULTI
            | Capabilities::MARIADB_CLIENT_STMT_BULK_OPERATIONS
    );

    assert_eq!(p.server_default_collation, 8);
    assert!(p.status.contains(Status::SERVER_STATUS_AUTOCOMMIT));
    assert!(matches!(
        p.auth_plugin,
        Some(AuthPlugin::MySqlNativePassword)
    ));

    assert_eq!(
        &*p.auth_plugin_data.into_iter().collect::<Vec<_>>(),
        &[116, 54, 76, 92, 106, 34, 100, 83, 85, 49, 52, 79, 112, 104, 57, 34, 60, 72, 53, 110,]
    );
}
