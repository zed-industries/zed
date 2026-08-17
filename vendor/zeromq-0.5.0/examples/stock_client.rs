mod async_helpers;

use std::convert::TryInto;
use std::env;
use std::error::Error;
use zeromq::{Socket, SocketRecv};

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut subscription = "";
    if args.len() > 1 {
        subscription = &args[1];
    }
    let mut socket = zeromq::SubSocket::new();
    socket
        .connect("tcp://127.0.0.1:5556")
        .await
        .expect("Failed to connect");

    socket.subscribe(subscription).await?;

    loop {
        let recv = socket.recv().await?;
        let stock: String = String::from_utf8(recv.get(0).unwrap().to_vec())?;
        let price: u32 = u32::from_ne_bytes(
            recv.get(1)
                .unwrap()
                .to_vec()
                .try_into()
                .expect("Couldn't deserialze u32 from data"),
        );
        println!("{}: {}", stock, price);
    }
}
