//! A driver that runs a [`Handshake`] over [`futures`] I/O traits.

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::io::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _};

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
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        if this.offset < this.leftover.len() {
            let remaining = &this.leftover[this.offset..];
            let length = remaining.len().min(buf.len());
            buf[..length].copy_from_slice(&remaining[..length]);
            this.offset += length;
            return Poll::Ready(Ok(length));
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

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().stream).poll_close(cx)
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use futures::join;

    use super::*;

    #[test]
    fn establishes_http_tunnel_and_preserves_leftover() {
        let (client, mut server) = duplex();
        let spec = ProxySpec::parse(&"http://proxy:8080".parse().unwrap()).unwrap();
        let target = Target::Domain("cloud.example.com".to_string(), 443);
        block_on(async {
            join!(
                async {
                    let mut stream = establish(client, &spec, &target).await.unwrap();
                    let mut buffer = [0u8; 12];
                    stream.read_exact(&mut buffer).await.unwrap();
                    assert_eq!(&buffer, b"early-tunnel");
                    stream.write_all(b"hello").await.unwrap();
                    stream.flush().await.unwrap();
                },
                async {
                    let mut head = Vec::new();
                    let mut byte = [0u8; 1];
                    while !head.ends_with(b"\r\n\r\n") {
                        server.read_exact(&mut byte).await.unwrap();
                        head.push(byte[0]);
                    }
                    // Send the response and some tunnel bytes in one write so
                    // the driver has to hand them back through `leftover`.
                    server
                        .write_all(b"HTTP/1.1 200 OK\r\n\r\nearly-tunnel")
                        .await
                        .unwrap();
                    let mut buffer = [0u8; 5];
                    server.read_exact(&mut buffer).await.unwrap();
                    assert_eq!(&buffer, b"hello");
                },
            );
        });
    }

    fn duplex() -> (PipeStream, PipeStream) {
        let (client_reader, server_writer) = piper::pipe(1024);
        let (server_reader, client_writer) = piper::pipe(1024);
        (
            PipeStream {
                reader: client_reader,
                writer: client_writer,
            },
            PipeStream {
                reader: server_reader,
                writer: server_writer,
            },
        )
    }

    struct PipeStream {
        reader: piper::Reader,
        writer: piper::Writer,
    }

    impl AsyncRead for PipeStream {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut self.get_mut().reader).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for PipeStream {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            Pin::new(&mut self.get_mut().writer).poll_write(cx, buf)
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut self.get_mut().writer).poll_flush(cx)
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Pin::new(&mut self.get_mut().writer).poll_close(cx)
        }
    }
}
