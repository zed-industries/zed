//! UDP client.
//!
//! First start the echo server:
//!
//! ```sh
//! $ cargo run --example udp-echo
//! ```
//!
//! Then run the client:
//!
//! ```sh
//! $ cargo run --example udp-client
//! ```

use async_std::io;
use async_std::net::UdpSocket;
use async_std::task;

fn main() -> io::Result<()> {
    task::block_on(async {
        let socket = UdpSocket::bind("127.0.0.1:8081").await?;
        println!("Listening on {}", socket.local_addr()?);

        let msg = "hello world";
        println!("<- {}", msg);
        socket.send_to(msg.as_bytes(), "127.0.0.1:8080").await?;

        let mut buf = vec![0u8; 1024];
        let (n, _) = socket.recv_from(&mut buf).await?;
        println!("-> {}\n", String::from_utf8_lossy(&buf[..n]));

        Ok(())
    })
}
