//! A driver that runs a [`Handshake`] over [`tokio`](::tokio) I/O traits.

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use ::tokio::io::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _, ReadBuf};

use crate::{EstablishError, Handshake, ProxySpec, Step, Target};

/// Runs the proxy handshake over `stream` and returns the tunneled stream.
///
/// The stream must already be connected to the proxy (and wrapped in TLS if
/// [`ProxySpec::tls`] asks for it). See [`Handshake::new`] for how `target`
/// interacts with DNS resolution.
pub async fn establish<S>(
    mut stream: S,
    spec: &ProxySpec,
    target: &Target,
) -> Result<Tunneled<S>, EstablishError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut handshake = Handshake::new(spec, target)?;
    let mut buffer = [0u8; 4096];
    let mut received_length = 0;
    loop {
        let step = handshake.advance(&buffer[..received_length])?;
        received_length = 0;
        match step {
            Step::Send(bytes) => {
                stream.write_all(&bytes).await?;
                stream.flush().await?;
            }
            Step::NeedMoreInput => {
                received_length = stream.read(&mut buffer).await?;
                if received_length == 0 {
                    return Err(EstablishError::Io(io::ErrorKind::UnexpectedEof.into()));
                }
            }
            Step::Done { leftover } => {
                return Ok(Tunneled {
                    leftover,
                    offset: 0,
                    stream,
                });
            }
        }
    }
}

/// A tunneled stream returned by [`establish`]: replays the handshake's
/// leftover bytes before reading from the underlying transport. Writes pass
/// straight through.
pub struct Tunneled<S> {
    leftover: Vec<u8>,
    offset: usize,
    stream: S,
}

impl<S: AsyncRead + Unpin> AsyncRead for Tunneled<S> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        if this.offset < this.leftover.len() {
            let remaining = &this.leftover[this.offset..];
            let length = remaining.len().min(buf.remaining());
            buf.put_slice(&remaining[..length]);
            this.offset += length;
            return Poll::Ready(Ok(()));
        }
        Pin::new(&mut this.stream).poll_read(cx, buf)
    }
}

impl<S: AsyncWrite + Unpin> AsyncWrite for Tunneled<S> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().stream).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().stream).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().stream).poll_shutdown(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn establishes_socks5_tunnel_and_preserves_leftover() {
        let (client, mut server) = ::tokio::io::duplex(1024);
        let spec = ProxySpec::parse(&"socks5h://proxy:1080".parse().unwrap()).unwrap();
        let target = Target::Domain("cloud.example.com".to_string(), 443);
        ::tokio::join!(
            async {
                let mut stream = establish(client, &spec, &target).await.unwrap();
                let mut buffer = [0u8; 12];
                stream.read_exact(&mut buffer).await.unwrap();
                assert_eq!(&buffer, b"early-tunnel");
                stream.write_all(b"hello").await.unwrap();
                stream.flush().await.unwrap();
            },
            async {
                let mut greeting = [0u8; 3];
                server.read_exact(&mut greeting).await.unwrap();
                assert_eq!(greeting, [0x05, 0x01, 0x00]);
                server.write_all(&[0x05, 0x00]).await.unwrap();

                let mut connect_request = vec![0u8; 4 + 1 + 17 + 2];
                server.read_exact(&mut connect_request).await.unwrap();
                // Send the reply and some tunnel bytes in one write so the
                // driver has to hand them back through `leftover`.
                let mut reply = vec![0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0];
                reply.extend_from_slice(b"early-tunnel");
                server.write_all(&reply).await.unwrap();

                let mut buffer = [0u8; 5];
                server.read_exact(&mut buffer).await.unwrap();
                assert_eq!(&buffer, b"hello");
            },
        );
    }
}
