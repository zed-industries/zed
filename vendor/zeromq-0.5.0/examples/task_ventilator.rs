mod async_helpers;

use rand::Rng;
use std::error::Error;
use std::io::{BufRead, BufReader};

use zeromq::{Socket, SocketSend};

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Socket to send messages on
    let mut sender = zeromq::PushSocket::new();
    sender.bind("tcp://127.0.0.1:5557").await?;

    // Socket with direct access to the sink: used to syncronize start of batch
    let mut sink = zeromq::PushSocket::new();
    sink.connect("tcp://127.0.0.1:5558").await?;

    println!("Press Enter when the workers are ready: ");
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);
    println!("Sending tasks to workers…");

    // The first message is "0" and signals start of batch
    sink.send("0".into()).await?;

    let mut rnd = rand::rng();
    let mut total_msec = 0;
    for _ in 0..100 {
        let workload = rnd.random_range(1..100);
        sender.send(workload.to_string().into()).await?;
        total_msec += workload;
    }
    println!("Total expected cost: {} msec", total_msec);
    sender.close().await;
    sink.close().await;
    Ok(())
}
