use anyhow::Result;
use futures::{AsyncReadExt as _, AsyncWriteExt as _, FutureExt as _, io::BufReader, select};
use net::async_net::UnixStream;
use smol::{Async, io::AsyncBufReadExt};

/// The main function for when Zed is running in netcat mode
pub fn main(socket: &str) -> Result<()> {
    smol::block_on(async {
        let socket_stream = UnixStream::connect(socket).await?;
        let (socket_read, mut socket_write) = socket_stream.split();
        let mut socket_reader = BufReader::new(socket_read);

        let mut stdout = Async::new(std::io::stdout())?;
        let stdin = Async::new(std::io::stdin())?;
        let mut stdin_reader = BufReader::new(stdin);

        let mut socket_line = Vec::new();
        let mut stdin_line = Vec::new();

        loop {
            select! {
                bytes_read = socket_reader.read_until(b'\n', &mut socket_line).fuse() => {
                    if bytes_read? == 0 {
                        break
                    }

                    stdout.write_all(&socket_line).await?;
                    stdout.flush().await?;

                    socket_line.clear();
                }
                bytes_read = stdin_reader.read_until(b'\n', &mut stdin_line).fuse() => {
                    if bytes_read? == 0 {
                        break
                    }

                    socket_write.write_all(&stdin_line).await?;
                    socket_write.flush().await?;

                    stdin_line.clear();
                }
            }
        }

        anyhow::Ok(())
    })
}
