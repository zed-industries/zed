mod async_helpers;

use rand::Rng;
use std::time::Duration;

use zeromq::*;

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    println!("Start server");
    let mut socket = zeromq::PubSocket::new();
    socket.bind("tcp://127.0.0.1:5556").await?;

    println!("Start sending loop");
    loop {
        let zipcode = rng.random_range(10000..10010);
        let temperature = rng.random_range(-80..135);
        let relhumidity = rng.random_range(10..60);
        socket
            .send(format!("{} {} {}", zipcode, temperature, relhumidity).into())
            .await?;
        async_helpers::sleep(Duration::from_millis(100)).await;
    }
}
