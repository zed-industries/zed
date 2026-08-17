mod async_helpers;

use std::error::Error;
use std::str::FromStr;
use zeromq::util::PeerIdentity;
use zeromq::{Socket, SocketOptions, SocketRecv, SocketSend};

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut options = SocketOptions::default();
    options.peer_identity(PeerIdentity::from_str("SomeCustomId")?);

    let mut socket = zeromq::ReqSocket::with_options(options);
    socket
        .connect("tcp://127.0.0.1:5555")
        .await
        .expect("Failed to connect");
    println!("Connected to server");

    for _ in 0..10u64 {
        socket.send("Hello".into()).await?;
        let repl = socket.recv().await?;
        println!("Received: {:?}", repl);
    }
    Ok(())
}
