mod async_helpers;

use std::convert::TryInto;
use zeromq::*;

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Start server");
    let mut socket = zeromq::RepSocket::new();
    socket.bind("tcp://127.0.0.1:5555").await?;

    loop {
        let mut repl: String = socket.recv().await?.try_into()?;
        println!("Received: {:?}", repl);
        repl.push_str(" Reply");
        socket.send(repl.into()).await?;
    }
}
