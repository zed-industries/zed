//! A TCP client.
//!
//! First start a server:
//!
//! ```
//! cargo run --example tcp-server
//! ```
//!
//! Then start a client:
//!
//! ```
//! cargo run --example tcp-client
//! ```

use std::net::TcpStream;

use smol::{future, io, Async, Unblock};

fn main() -> io::Result<()> {
    smol::block_on(async {
        // Create async stdin and stdout handles.
        let stdin = Unblock::new(std::io::stdin());
        let mut stdout = Unblock::new(std::io::stdout());

        // Connect to the server.
        let stream = Async::<TcpStream>::connect(([127, 0, 0, 1], 7000)).await?;
        println!("Connected to {}", stream.get_ref().peer_addr()?);
        println!("Type a message and hit enter!\n");

        // Pipe messages from stdin to the server and pipe messages from the server to stdout.
        future::try_zip(
            io::copy(stdin, &mut &stream),
            io::copy(&stream, &mut stdout),
        )
        .await?;

        Ok(())
    })
}
