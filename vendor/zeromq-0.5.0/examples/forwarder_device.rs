mod async_helpers;
use std::error::Error;
use zeromq::prelude::*;

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Start forwarder");
    let mut frontend = zeromq::SubSocket::new();
    frontend.bind("tcp://127.0.0.1:30001").await?;

    let mut backend = zeromq::PubSocket::new();
    backend.bind("tcp://127.0.0.1:30002").await?;

    frontend.subscribe("").await?;

    let forward = async move {
        loop {
            let message = frontend.recv().await.unwrap();
            println!("passing message: {:?}", message);
            backend.send(message).await.unwrap();
        }
    };

    forward.await;

    Ok(())
}
