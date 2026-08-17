use crate::error::Error;
use crate::net::tls::{self, TlsConfig};
use crate::net::{Socket, SocketIntoBox, WithSocket};

use crate::message::SslRequest;
use crate::{PgConnectOptions, PgSslMode};

pub struct MaybeUpgradeTls<'a>(pub &'a PgConnectOptions);

impl<'a> WithSocket for MaybeUpgradeTls<'a> {
    type Output = crate::Result<Box<dyn Socket>>;

    async fn with_socket<S: Socket>(self, socket: S) -> Self::Output {
        maybe_upgrade(socket, self.0).await
    }
}

async fn maybe_upgrade<S: Socket>(
    mut socket: S,
    options: &PgConnectOptions,
) -> Result<Box<dyn Socket>, Error> {
    // https://www.postgresql.org/docs/12/libpq-ssl.html#LIBPQ-SSL-SSLMODE-STATEMENTS
    match options.ssl_mode {
        // FIXME: Implement ALLOW
        PgSslMode::Allow | PgSslMode::Disable => return Ok(Box::new(socket)),

        PgSslMode::Prefer => {
            if !tls::available() {
                return Ok(Box::new(socket));
            }

            // try upgrade, but its okay if we fail
            if !request_upgrade(&mut socket, options).await? {
                return Ok(Box::new(socket));
            }
        }

        PgSslMode::Require | PgSslMode::VerifyFull | PgSslMode::VerifyCa => {
            tls::error_if_unavailable()?;

            if !request_upgrade(&mut socket, options).await? {
                // upgrade failed, die
                return Err(Error::Tls("server does not support TLS".into()));
            }
        }
    }

    let accept_invalid_certs = !matches!(
        options.ssl_mode,
        PgSslMode::VerifyCa | PgSslMode::VerifyFull
    );
    let accept_invalid_hostnames = !matches!(options.ssl_mode, PgSslMode::VerifyFull);

    let config = TlsConfig {
        accept_invalid_certs,
        accept_invalid_hostnames,
        hostname: &options.host,
        root_cert_path: options.ssl_root_cert.as_ref(),
        client_cert_path: options.ssl_client_cert.as_ref(),
        client_key_path: options.ssl_client_key.as_ref(),
    };

    tls::handshake(socket, config, SocketIntoBox).await
}

async fn request_upgrade(
    socket: &mut impl Socket,
    _options: &PgConnectOptions,
) -> Result<bool, Error> {
    // https://www.postgresql.org/docs/current/protocol-flow.html#id-1.10.5.7.11

    // To initiate an SSL-encrypted connection, the frontend initially sends an
    // SSLRequest message rather than a StartupMessage

    socket.write(SslRequest::BYTES).await?;

    // The server then responds with a single byte containing S or N, indicating that
    // it is willing or unwilling to perform SSL, respectively.

    let mut response = [0u8];

    socket.read(&mut &mut response[..]).await?;

    match response[0] {
        b'S' => {
            // The server is ready and willing to accept an SSL connection
            Ok(true)
        }

        b'N' => {
            // The server is _unwilling_ to perform SSL
            Ok(false)
        }

        other => Err(err_protocol!(
            "unexpected response from SSLRequest: 0x{:02x}",
            other
        )),
    }
}
