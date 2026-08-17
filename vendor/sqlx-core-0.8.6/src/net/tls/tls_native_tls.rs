use std::io::{self, Read, Write};

use crate::io::ReadBuf;
use crate::net::tls::util::StdSocket;
use crate::net::tls::TlsConfig;
use crate::net::Socket;
use crate::Error;

use native_tls::{HandshakeError, Identity};
use std::task::{Context, Poll};

pub struct NativeTlsSocket<S: Socket> {
    stream: native_tls::TlsStream<StdSocket<S>>,
}

impl<S: Socket> Socket for NativeTlsSocket<S> {
    fn try_read(&mut self, buf: &mut dyn ReadBuf) -> io::Result<usize> {
        self.stream.read(buf.init_mut())
    }

    fn try_write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn poll_read_ready(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.stream.get_mut().poll_ready(cx)
    }

    fn poll_write_ready(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.stream.get_mut().poll_ready(cx)
    }

    fn poll_shutdown(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.stream.shutdown() {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => self.stream.get_mut().poll_ready(cx),
            ready => Poll::Ready(ready),
        }
    }
}

pub async fn handshake<S: Socket>(
    socket: S,
    config: TlsConfig<'_>,
) -> crate::Result<NativeTlsSocket<S>> {
    let mut builder = native_tls::TlsConnector::builder();

    builder
        .danger_accept_invalid_certs(config.accept_invalid_certs)
        .danger_accept_invalid_hostnames(config.accept_invalid_hostnames);

    if let Some(root_cert_path) = config.root_cert_path {
        let data = root_cert_path.data().await?;
        builder.add_root_certificate(native_tls::Certificate::from_pem(&data).map_err(Error::tls)?);
    }

    // authentication using user's key-file and its associated certificate
    if let (Some(cert_path), Some(key_path)) = (config.client_cert_path, config.client_key_path) {
        let cert_path = cert_path.data().await?;
        let key_path = key_path.data().await?;
        let identity = Identity::from_pkcs8(&cert_path, &key_path).map_err(Error::tls)?;
        builder.identity(identity);
    }

    let connector = builder.build().map_err(Error::tls)?;

    let mut mid_handshake = match connector.connect(config.hostname, StdSocket::new(socket)) {
        Ok(tls_stream) => return Ok(NativeTlsSocket { stream: tls_stream }),
        Err(HandshakeError::Failure(e)) => return Err(Error::tls(e)),
        Err(HandshakeError::WouldBlock(mid_handshake)) => mid_handshake,
    };

    loop {
        mid_handshake.get_mut().ready().await?;

        match mid_handshake.handshake() {
            Ok(tls_stream) => return Ok(NativeTlsSocket { stream: tls_stream }),
            Err(HandshakeError::Failure(e)) => return Err(Error::tls(e)),
            Err(HandshakeError::WouldBlock(mid_handshake_)) => {
                mid_handshake = mid_handshake_;
            }
        }
    }
}
