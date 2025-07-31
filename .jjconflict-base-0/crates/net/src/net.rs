pub mod async_net;
#[cfg(target_os = "windows")]
pub mod listener;
#[cfg(target_os = "windows")]
pub mod socket;
#[cfg(target_os = "windows")]
pub mod stream;
#[cfg(target_os = "windows")]
mod util;

#[cfg(target_os = "windows")]
pub use listener::*;
#[cfg(target_os = "windows")]
pub use socket::*;
#[cfg(not(target_os = "windows"))]
pub use std::os::unix::net::{UnixListener, UnixStream};
#[cfg(target_os = "windows")]
pub use stream::*;

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use smol::io::{AsyncReadExt, AsyncWriteExt};

    const SERVER_MESSAGE: &str = "Connection closed";
    const CLIENT_MESSAGE: &str = "Hello, server!";
    const BUFFER_SIZE: usize = 32;

    #[test]
    fn test_windows_listener() -> std::io::Result<()> {
        use crate::{UnixListener, UnixStream};

        let temp = tempfile::tempdir()?;
        let socket = temp.path().join("socket.sock");
        let listener = UnixListener::bind(&socket)?;

        // Server
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();

            // Read data from the client
            let mut buffer = [0; BUFFER_SIZE];
            let bytes_read = stream.read(&mut buffer).unwrap();
            let string = String::from_utf8_lossy(&buffer[..bytes_read]);
            assert_eq!(string, CLIENT_MESSAGE);

            // Send a message back to the client
            stream.write_all(SERVER_MESSAGE.as_bytes()).unwrap();
        });

        // Client
        let mut client = UnixStream::connect(&socket)?;

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
        use crate::async_net::{UnixListener, UnixStream};

        smol::block_on(async {
            let temp = tempfile::tempdir()?;
            let socket = temp.path().join("socket.sock");
            let listener = UnixListener::bind(&socket)?;

            // Server
            let server = smol::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();

                // Read data from the client
                let mut buffer = [0; BUFFER_SIZE];
                let bytes_read = stream.read(&mut buffer).await.unwrap();
                let string = String::from_utf8_lossy(&buffer[..bytes_read]);
                assert_eq!(string, CLIENT_MESSAGE);

                // Send a message back to the client
                stream.write_all(SERVER_MESSAGE.as_bytes()).await.unwrap();
            });

            // Client
            let mut client = UnixStream::connect(&socket).await?;
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
}
