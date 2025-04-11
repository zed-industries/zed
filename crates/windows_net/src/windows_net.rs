use std::{
    io::Result,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

use listener::WindowsListener;
use smol::{
    Async,
    io::{AsyncRead, AsyncWrite},
};

mod listener;
mod socket;
mod stream;
mod util;

pub struct UnixListener(Async<listener::WindowsListener>);

impl UnixListener {
    pub fn bind(path: &Path) -> Result<Self> {
        Ok(UnixListener(Async::new(WindowsListener::bind(path)?)?))
    }

    pub async fn accept(&self) -> Result<UnixStream> {
        let sock = self.0.read_with(|listener| listener.accept()).await?;
        Ok(UnixStream(Async::new(sock)?))
    }
}

pub struct UnixStream(Async<stream::WindowsStream>);

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

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use crate::{WindowsListener, stream::WindowsStream};

    #[test]
    fn test_windows_listener() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let socket = temp.path().join("socket.sock");
        let listener = WindowsListener::bind(&socket)?;
        println!("Listener bound to {:?}", socket);
        // Server
        let server = std::thread::spawn(move || {
            let mut stream = listener.accept().unwrap();
            let mut buffer = [0; 32];
            let bytes_read = stream.read(&mut buffer).unwrap();
            let string = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Server received: {}<-", string);

            stream.write_all(b"Connection closed").unwrap();
            println!("Server sent: Connection closed.");
        });

        let mut client = WindowsStream::connect(&socket)?;
        client.write_all(b"Hello, server!")?;
        println!("Client sent: Hello, server!");
        let mut buffer = [0; 32];
        let bytes_read = client.read(&mut buffer)?;
        let string = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Client received: {}<-", string);
        client.flush()?;

        server.join().unwrap();
        Ok(())
    }
}
