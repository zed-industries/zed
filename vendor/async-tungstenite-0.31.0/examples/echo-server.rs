//! A simple echo server.
//!
//! You can test this out by running:
//!
//!     cargo run --features="async-std-runtime" --example echo-server 127.0.0.1:12345
//!
//! And then in another window run:
//!
//!     cargo run --features="async-std-runtime" --example client ws://127.0.0.1:12345/

use std::{env, io::Error};

use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use futures::prelude::*;
use log::info;

async fn run() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        task::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = async_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}

fn main() -> Result<(), Error> {
    task::block_on(run())
}
