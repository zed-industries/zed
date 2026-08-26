#[cfg(not(target_os = "windows"))]
pub use smol::net::unix::{UnixListener, UnixStream};

#[cfg(target_os = "windows")]
pub use windows::{UnixListener, UnixStream};

#[cfg(target_os = "windows")]
pub mod windows {
    use std::{
        io::Result,
        path::Path,
        pin::Pin,
        task::{Context, Poll},
    };

    use smol::{
        Async,
        io::{AsyncRead, AsyncWrite},
    };

    pub struct UnixListener(Async<crate::UnixListener>);

    impl UnixListener {
        pub fn bind<P: AsRef<Path>>(path: P) -> Result<Self> {
            Ok(UnixListener(Async::new(crate::UnixListener::bind(path)?)?))
        }

        pub async fn accept(&self) -> Result<(UnixStream, ())> {
            let (sock, _) = self.0.read_with(|listener| listener.accept()).await?;
            Ok((UnixStream(Async::new(sock)?), ()))
        }
    }

    pub struct UnixStream(Async<crate::UnixStream>);

    impl UnixStream {
        pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self> {
            Ok(UnixStream(Async::new(crate::UnixStream::connect(path)?)?))
        }
    }

    impl AsyncRead for UnixStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<Result<usize>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for UnixStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
            Pin::new(&mut self.0).poll_close(cx)
        }
    }
}
