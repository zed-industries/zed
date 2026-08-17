//! UDP echo server.
//!
//! To send messages, do:
//!
//! ```sh
//! $ nc -u localhost 8080
//! ```

use async_std::io;
use async_std::net::UdpSocket;
use async_std::task;

fn main() -> io::Result<()> {
    task::block_on(async {
        let socket = UdpSocket::bind("127.0.0.1:8080").await?;
        let mut buf = vec![0u8; 1024];

        println!("Listening on {}", socket.local_addr()?);

        loop {
            let (n, peer) = socket.recv_from(&mut buf).await?;
            let sent = socket.send_to(&buf[..n], &peer).await?;
            println!("Sent {} out of {} bytes to {}", sent, n, peer);
        }
    })
}
