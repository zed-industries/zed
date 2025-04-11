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
use stream::WindowsStream;

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

impl UnixStream {
    pub fn connect(path: &Path) -> Result<Self> {
        Ok(UnixStream(Async::new(WindowsStream::connect(path)?)?))
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

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use smol::io::{AsyncReadExt, AsyncWriteExt};

    use crate::{UnixListener, UnixStream, WindowsListener, stream::WindowsStream};

    const SERVER_MESSAGE: &str = "Connection closed";
    const CLIENT_MESSAGE: &str = "Hello, server!";
    const BUFFER_SIZE: usize = 32;

    #[test]
    fn test_windows_listener() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let socket = temp.path().join("socket.sock");
        let listener = WindowsListener::bind(&socket)?;

        // Server
        let server = std::thread::spawn(move || {
            let mut stream = listener.accept().unwrap();

            // Read data from the client
            let mut buffer = [0; BUFFER_SIZE];
            let bytes_read = stream.read(&mut buffer).unwrap();
            let string = String::from_utf8_lossy(&buffer[..bytes_read]);
            assert_eq!(string, CLIENT_MESSAGE);

            // Send a message back to the client
            stream.write_all(SERVER_MESSAGE.as_bytes()).unwrap();
        });

        // Client
        let mut client = WindowsStream::connect(&socket)?;

        // Send data to the server
        client.write_all(CLIENT_MESSAGE.as_bytes())?;
        let mut buffer = [0; BUFFER_SIZE];

        // Read the response from the server
        let bytes_read = client.read(&mut buffer)?;
        let string = String::from_utf8_lossy(&buffer[..bytes_read]);
        assert_eq!(string, SERVER_MESSAGE);
        client.flush()?;

        server.join().unwrap();
        Ok(())
    }

    #[test]
    fn test_unix_listener() -> std::io::Result<()> {
        smol::block_on(async {
            let temp = tempfile::tempdir()?;
            let socket = temp.path().join("socket.sock");
            let listener = UnixListener::bind(&socket)?;

            // Server
            let server = smol::spawn(async move {
                let mut stream = listener.accept().await.unwrap();

                // Read data from the client
                let mut buffer = [0; BUFFER_SIZE];
                let bytes_read = stream.read(&mut buffer).await.unwrap();
                let string = String::from_utf8_lossy(&buffer[..bytes_read]);
                assert_eq!(string, CLIENT_MESSAGE);

                // Send a message back to the client
                stream.write_all(SERVER_MESSAGE.as_bytes()).await.unwrap();
            });

            // Client
            let mut client = UnixStream::connect(&socket)?;
            client.write_all(CLIENT_MESSAGE.as_bytes()).await?;

            // Read the response from the server
            let mut buffer = [0; BUFFER_SIZE];
            let bytes_read = client.read(&mut buffer).await?;
            let string = String::from_utf8_lossy(&buffer[..bytes_read]);
            assert_eq!(string, "Connection closed");
            client.flush().await?;

            server.await;
            Ok(())
        })
    }

    #[test]
    fn test_connection() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let socket = temp.path().join("socket.sock");
        println!("Socket path: {:?}", socket);
        smol::block_on(async move {
            let listener = UnixListener::bind(&socket)?;

            // Server
            let server = smol::spawn(async move {
                let mut stream = listener.accept().await.unwrap();

                // Read data from the client
                let mut buffer = [0; BUFFER_SIZE];
                let bytes_read = stream.read(&mut buffer).await.unwrap();
                let string = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received from client: {}", string);

                // Send a message back to the client
                stream.write_all(SERVER_MESSAGE.as_bytes()).await.unwrap();
            });

            // Client
            // let mut client = UnixStream::connect(&socket)?;
            // client.write_all(CLIENT_MESSAGE.as_bytes()).await?;

            // // Read the response from the server
            // let mut buffer = [0; BUFFER_SIZE];
            // let bytes_read = client.read(&mut buffer).await?;
            // let string = String::from_utf8_lossy(&buffer[..bytes_read]);
            // assert_eq!(string, "Connection closed");
            // client.flush().await?;

            server.await;
            Ok(())
        })
    }
}
