mod async_helpers;

use std::error::Error;

use zeromq::{Socket, SocketRecv};

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = zeromq::SubSocket::new();
    socket
        .connect("tcp://127.0.0.1:5556")
        .await
        .expect("Failed to connect");

    socket.subscribe("").await?;

    for i in 0..10 {
        println!("Message {}", i);
        let repl = socket.recv().await?;

        println!("Received: {:?}", repl);
    }
    Ok(())
}
