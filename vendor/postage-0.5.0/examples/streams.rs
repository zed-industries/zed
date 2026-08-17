use postage::{mpsc, prelude::Stream, sink::Sink, stream::TryRecvError};

#[tokio::main]
async fn main() {
    // Channel receivers require mutable references to take messages.
    let mut rx = create_stream().await;

    // There are a few ways to iterate over streams.
    // You can pull a single message out:
    if let Some(message) = rx.recv().await {
        println!("We got a message: {}", message);
    }

    // You can also iterate over all the messages, until the channel runs out:
    while let Some(message) = rx.recv().await {
        println!("We got a message: {}", message);
    }

    // You can attempt to pull a value from the stream without blocking:
    match rx.try_recv() {
        Ok(message) => println!("We got a message: {}", message),
        Err(TryRecvError::Pending) => {
            // there may be a message in the future
        }
        Err(TryRecvError::Closed) => {
            // the channel is closed.
        }
    }
}

async fn create_stream() -> impl Stream<Item = String> {
    let (mut tx, rx) = mpsc::channel(8);
    tx.send("Hello!".to_string()).await.ok();
    tx.send("World!".to_string()).await.ok();

    rx
}
