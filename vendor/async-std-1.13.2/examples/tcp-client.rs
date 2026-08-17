//! TCP client.
//!
//! First start the echo server:
//!
//! ```sh
//! $ cargo run --example tcp-echo
//! ```
//!
//! Then run the client:
//!
//! ```sh
//! $ cargo run --example tcp-client
//! ```

use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

fn main() -> io::Result<()> {
    task::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        println!("Connected to {}", &stream.peer_addr()?);

        let msg = "hello world";
        println!("<- {}", msg);
        stream.write_all(msg.as_bytes()).await?;

        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await?;
        println!("-> {}\n", String::from_utf8_lossy(&buf[..n]));

        Ok(())
    })
}
