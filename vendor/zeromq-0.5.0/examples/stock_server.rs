mod async_helpers;

use rand::Rng;
use std::time::Duration;
use zeromq::*;

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let stocks: Vec<&str> = vec!["AAA", "ABB", "BBB"];
    println!("Starting server");
    let mut socket = zeromq::PubSocket::new();
    socket.bind("tcp://127.0.0.1:5556").await?;

    println!("Start sending loop");
    loop {
        for stock in &stocks {
            let price: u32 = rng.random_range(1..100);
            let mut m: ZmqMessage = ZmqMessage::from(*stock);
            m.push_back(price.to_ne_bytes().to_vec().into());
            println!("Sending: {:?}", m);
            socket.send(m).await?;
        }
        async_helpers::sleep(Duration::from_secs(1)).await;
    }
}
