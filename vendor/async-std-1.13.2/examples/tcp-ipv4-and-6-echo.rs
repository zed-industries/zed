//! TCP echo server, accepting connections both on both ipv4 and ipv6 sockets.
//!
//! To send messages, do:
//!
//! ```sh
//! $ nc 127.0.0.1 8080
//! $ nc ::1 8080
//! ```

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

async fn process(stream: TcpStream) -> io::Result<()> {
    println!("Accepted from: {}", stream.peer_addr()?);

    let mut reader = stream.clone();
    let mut writer = stream;
    io::copy(&mut reader, &mut writer).await?;

    Ok(())
}

fn main() -> io::Result<()> {
    task::block_on(async {
        let ipv4_listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", ipv4_listener.local_addr()?);
        let ipv6_listener = TcpListener::bind("[::1]:8080").await?;
        println!("Listening on {}", ipv6_listener.local_addr()?);

        let ipv4_incoming = ipv4_listener.incoming();
        let ipv6_incoming = ipv6_listener.incoming();

        let mut incoming = ipv4_incoming.merge(ipv6_incoming);

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                process(stream).await.unwrap();
            });
        }
        Ok(())
    })
}
