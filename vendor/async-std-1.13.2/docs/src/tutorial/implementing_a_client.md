## Implementing a client

Since the protocol is line-based, implementing a client for the chat is straightforward:

* Lines read from stdin should be sent over the socket.
* Lines read from the socket should be echoed to stdout.

Although async does not significantly affect client performance (as unlike the server, the client interacts solely with one user and only needs limited concurrency), async is still useful for managing concurrency!

The client has to read from stdin and the socket *simultaneously*.
Programming this with threads is cumbersome, especially when implementing a clean shutdown.
With async, the `select!` macro is all that is needed.


```rust,edition2018
# extern crate async_std;
# extern crate futures;
use async_std::{
    io::{stdin, BufReader},
    net::{TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use futures::{select, FutureExt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// main
fn run() -> Result<()> {
    task::block_on(try_run("127.0.0.1:8080"))
}

async fn try_run(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = (&stream, &stream); // 1
    let mut lines_from_server = BufReader::new(reader).lines().fuse(); // 2
    let mut lines_from_stdin = BufReader::new(stdin()).lines().fuse(); // 2
    loop {
        select! { // 3
            line = lines_from_server.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    println!("{}", line);
                },
                None => break,
            },
            line = lines_from_stdin.next().fuse() => match line {
                Some(line) => {
                    let line = line?;
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
                None => break,
            }
        }
    }
    Ok(())
}
```

1. Here we split `TcpStream` into read and write halves: there's `impl AsyncRead for &'_ TcpStream`, just like the one in std.
2. We create a stream of lines for both the socket and stdin.
3. In the main select loop, we print the lines we receive from the server and send the lines we read from the console.
