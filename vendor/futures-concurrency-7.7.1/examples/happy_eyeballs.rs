use async_std::io::prelude::*;
use futures::future::TryFutureExt;
use futures_concurrency::prelude::*;
use futures_time::prelude::*;

use async_std::io;
use async_std::net::TcpStream;
use futures::channel::oneshot;
use futures_concurrency::vec::AggregateError;
use futures_time::time::Duration;
use std::error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    // Connect to a socket
    let mut socket = open_tcp_socket("rust-lang.org", 80, 3).await?;

    // Make an HTTP GET request.
    socket.write_all(b"GET / \r\n").await?;
    io::copy(&mut socket, &mut io::stdout()).await?;

    Ok(())
}

/// Happy eyeballs algorithm!
async fn open_tcp_socket(
    addr: &str,
    port: u16,
    attempts: u64,
) -> Result<TcpStream, AggregateError<io::Error>> {
    let (mut sender, mut receiver) = oneshot::channel();
    let mut futures = Vec::with_capacity(attempts as usize);

    for attempt in 0..attempts {
        // Start a next attempt if the previous one finishes, or timeout expires.
        let tcp = TcpStream::connect((addr, port));
        let start_event = receiver.timeout(Duration::from_secs(attempt));
        futures.push(tcp.delay(start_event).map_err(|err| {
            // If the socket fails, start the next attempt
            let _ = sender.send(());
            err
        }));
        (sender, receiver) = oneshot::channel();
    }

    // Start connecting. If an attempt succeeds, cancel all others attempts.
    futures.race_ok().await
}
