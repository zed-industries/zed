mod async_helpers;

use std::error::Error;
use zeromq::Socket;
use zeromq::{SocketRecv, SocketSend};

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = zeromq::ReqSocket::new();
    socket
        .connect("tcp://127.0.0.1:5559")
        .await
        .expect("Failed to connect");

    socket.send("Hello".into()).await?;
    let repl = socket.recv().await?;
    println!("{:?}", repl);

    socket.send("Hello".into()).await?;
    let repl = socket.recv().await?;
    println!("{:?}", repl);
    Ok(())
}
