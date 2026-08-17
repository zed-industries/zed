mod async_helpers;

use futures::{select, FutureExt};
use std::io::Write;
use std::{error::Error, time::Duration};
use zeromq::{Socket, SocketRecv, SocketSend};

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Socket to receive messages on
    let mut receiver = zeromq::PullSocket::new();
    receiver.connect("tcp://127.0.0.1:5557").await?;

    // Socket to send messages to
    let mut sender = zeromq::PushSocket::new();
    sender.connect("tcp://127.0.0.1:5558").await?;

    // Socket for control input
    let mut controller = zeromq::SubSocket::new();
    controller.connect("tcp://127.0.0.1:5559").await?;
    controller.subscribe("").await?;

    // Process messages from receiver and controller
    loop {
        select! {
            message = receiver.recv().fuse() => {
                // Process task
                let message = message.unwrap();
                let workload = String::from_utf8(message.get(0).unwrap().to_vec())?
                    .parse()
                    .expect("Couldn't parse u64 from data");

                // Do the work
                async_helpers::sleep(Duration::from_millis(workload)).await;

                // Send results to sink
                sender.send(message).await?;

                // Simple progress indicator for the viewer
                print!(".");
                std::io::stdout().flush()?;
            },
            // Any waiting controller command acts as 'KILL'
            _kill = controller.recv().fuse() => {
                break
            }
        };
    }

    println!("Done");
    receiver.close().await;
    sender.close().await;
    controller.close().await;
    Ok(())
}
