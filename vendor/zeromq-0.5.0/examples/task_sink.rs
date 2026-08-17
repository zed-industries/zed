mod async_helpers;

use std::error::Error;
use std::io::Write;
use std::time::Instant;

use zeromq::{Socket, SocketRecv, SocketSend};

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Socket to receive messages on
    let mut receiver = zeromq::PullSocket::new();
    receiver.bind("tcp://127.0.0.1:5558").await?;

    // Socket for worker control
    let mut controller = zeromq::PubSocket::new();
    controller.bind("tcp://127.0.0.1:5559").await?;

    receiver.recv().await?;

    let tstart = Instant::now();

    for task_nbr in 0..100u8 {
        receiver.recv().await?;
        if task_nbr % 10 == 0 {
            print!(":");
        } else {
            print!(".");
        }
        std::io::stdout().flush()?;
    }
    println!(
        "\nTotal elapsed time: {} msec",
        tstart.elapsed().as_millis()
    );

    // Send kill signal to workers
    controller.send("KILL".into()).await?;

    receiver.close().await;
    controller.close().await;
    Ok(())
}
