mod async_helpers;

use std::convert::TryInto;
use std::error::Error;
use zeromq::{Socket, SocketRecv, SocketSend};

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = zeromq::RepSocket::new();
    socket
        .connect("tcp://127.0.0.1:5560")
        .await
        .expect("Failed to connect");

    loop {
        let mut repl: String = socket.recv().await?.try_into()?;
        println!("Received: {}", repl);
        repl.push_str(" Reply");
        socket.send(repl.into()).await?;
    }
}
