use anyhow::Result;

#[cfg(windows)]
pub fn main(_socket: &str) -> Result<()> {
    // It looks like we can't get an async stdio stream on Windows from smol.
    //
    // We decided to merge this with a panic on Windows since this is only used
    // by the experimental Claude Code Agent Server.
    //
    // We're tracking this internally, and we will address it before shipping the integration.
    panic!("--nc isn't yet supported on Windows");
}

/// The main function for when Zed is running in netcat mode
#[cfg(not(windows))]
pub fn main(socket: &str) -> Result<()> {
    use futures::{AsyncReadExt as _, AsyncWriteExt as _, FutureExt as _, io::BufReader, select};
    use net::async_net::UnixStream;
    use smol::{Unblock, io::AsyncBufReadExt};

    smol::block_on(async {
        let socket_stream = UnixStream::connect(socket).await?;
        let (socket_read, mut socket_write) = socket_stream.split();
        let mut socket_reader = BufReader::new(socket_read);

        let mut stdout = Unblock::new(std::io::stdout());
        let stdin = Unblock::new(std::io::stdin());
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
