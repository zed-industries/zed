//! Connect to an HTTP website, make a GET request, and pipe the response to the standard output.
//!
//! Run with:
//!
//! ```
//! cargo run --example get-request
//! ```

use smol::{io, prelude::*, Async, Unblock};
use std::net::{TcpStream, ToSocketAddrs};

fn main() -> io::Result<()> {
    smol::block_on(async {
        // Connect to http://example.com
        let mut addrs = smol::unblock(move || ("example.com", 80).to_socket_addrs()).await?;
        let addr = addrs.next().unwrap();
        let mut stream = Async::<TcpStream>::connect(addr).await?;

        // Send an HTTP GET request.
        let req = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        stream.write_all(req).await?;

        // Read the response and pipe it to the standard output.
        let mut stdout = Unblock::new(std::io::stdout());
        io::copy(&stream, &mut stdout).await?;
        Ok(())
    })
}
