use bytes::buf::Buf;
use bytes::Bytes;

use crate::collation::{CharSet, Collation};
use crate::common::StatementCache;
use crate::connection::{tls, MySqlConnectionInner, MySqlStream, MAX_PACKET_SIZE};
use crate::error::Error;
use crate::net::{Socket, WithSocket};
use crate::protocol::connect::{
    AuthSwitchRequest, AuthSwitchResponse, Handshake, HandshakeResponse,
};
use crate::protocol::Capabilities;
use crate::{MySqlConnectOptions, MySqlConnection, MySqlSslMode};

impl MySqlConnection {
    pub(crate) async fn establish(options: &MySqlConnectOptions) -> Result<Self, Error> {
        let do_handshake = DoHandshake::new(options)?;

        let handshake = match &options.socket {
            Some(path) => crate::net::connect_uds(path, do_handshake).await?,
            None => crate::net::connect_tcp(&options.host, options.port, do_handshake).await?,
        };

        let stream = handshake?;

        Ok(Self {
            inner: Box::new(MySqlConnectionInner {
                stream,
                transaction_depth: 0,
                status_flags: Default::default(),
                cache_statement: StatementCache::new(options.statement_cache_capacity),
                log_settings: options.log_settings.clone(),
            }),
        })
    }
}

struct DoHandshake<'a> {
    options: &'a MySqlConnectOptions,
    charset: CharSet,
    collation: Collation,
}

impl<'a> DoHandshake<'a> {
    fn new(options: &'a MySqlConnectOptions) -> Result<Self, Error> {
        let charset: CharSet = options.charset.parse()?;
        let collation: Collation = options
            .collation
            .as_deref()
            .map(|collation| collation.parse())
            .transpose()?
            .unwrap_or_else(|| charset.default_collation());

        if options.enable_cleartext_plugin
            && matches!(
                options.ssl_mode,
                MySqlSslMode::Disabled | MySqlSslMode::Preferred
            )
        {
            log::warn!("Security warning: sending cleartext passwords without requiring SSL");
        }

        Ok(Self {
            options,
            charset,
            collation,
        })
    }

    async fn do_handshake<S: Socket>(self, socket: S) -> Result<MySqlStream, Error> {
        let DoHandshake {
            options,
            charset,
            collation,
        } = self;

        let mut stream = MySqlStream::with_socket(charset, collation, options, socket);

        // https://dev.mysql.com/doc/internals/en/connection-phase.html
        // https://mariadb.com/kb/en/connection/

        let handshake: Handshake = stream.recv_packet().await?.decode()?;

        let mut plugin = handshake.auth_plugin;
        let nonce = handshake.auth_plugin_data;

        // FIXME: server version parse is a bit ugly
        // expecting MAJOR.MINOR.PATCH

        let mut server_version = handshake.server_version.split('.');

        let server_version_major: u16 = server_version
            .next()
            .unwrap_or_default()
            .parse()
            .unwrap_or(0);

        let server_version_minor: u16 = server_version
            .next()
            .unwrap_or_default()
            .parse()
            .unwrap_or(0);

        let server_version_patch: u16 = server_version
            .next()
            .unwrap_or_default()
            .parse()
            .unwrap_or(0);

        stream.server_version = (
            server_version_major,
            server_version_minor,
            server_version_patch,
        );

        stream.capabilities &= handshake.server_capabilities;
        stream.capabilities |= Capabilities::PROTOCOL_41;

        let mut stream = tls::maybe_upgrade(stream, self.options).await?;

        let auth_response = if let (Some(plugin), Some(password)) = (plugin, &options.password) {
            Some(plugin.scramble(&mut stream, password, &nonce).await?)
        } else {
            None
        };

        stream.write_packet(HandshakeResponse {
            collation: stream.collation as u8,
            max_packet_size: MAX_PACKET_SIZE,
            username: &options.username,
            database: options.database.as_deref(),
            auth_plugin: plugin,
            auth_response: auth_response.as_deref(),
        })?;

        stream.flush().await?;

        loop {
            let packet = stream.recv_packet().await?;
            match packet[0] {
                0x00 => {
                    let _ok = packet.ok()?;

                    break;
                }

                0xfe => {
                    let switch: AuthSwitchRequest =
                        packet.decode_with(self.options.enable_cleartext_plugin)?;

                    plugin = Some(switch.plugin);
                    let nonce = switch.data.chain(Bytes::new());

                    let response = switch
                        .plugin
                        .scramble(
                            &mut stream,
                            options.password.as_deref().unwrap_or_default(),
                            &nonce,
                        )
                        .await?;

                    stream.write_packet(AuthSwitchResponse(response))?;
                    stream.flush().await?;
                }

                id => {
                    if let (Some(plugin), Some(password)) = (plugin, &options.password) {
                        if plugin.handle(&mut stream, packet, password, &nonce).await? {
                            // plugin signaled authentication is ok
                            break;
                        }

                        // plugin signaled to continue authentication
                    } else {
                        return Err(err_protocol!(
                            "unexpected packet 0x{:02x} during authentication",
                            id
                        ));
                    }
                }
            }
        }

        Ok(stream)
    }
}

impl<'a> WithSocket for DoHandshake<'a> {
    type Output = Result<MySqlStream, Error>;

    async fn with_socket<S: Socket>(self, socket: S) -> Self::Output {
        self.do_handshake(socket).await
    }
}
