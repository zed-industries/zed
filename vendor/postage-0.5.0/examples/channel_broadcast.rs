use std::time::Duration;

use postage::{broadcast, prelude::Stream, sink::Sink};

#[tokio::main]
async fn main() {
    // Broadcast channels are bounded multi-producer, multi-consumer channels.
    // Each reciever will observe every message created after the receiver.
    // Let's send two messages from multiple senders
    let (mut tx, rx) = broadcast::channel::<usize>(8);

    let mut tx2 = tx.clone();
    let rx2 = rx.clone();

    // Alice and Bob will see both messages
    tokio::task::spawn(print_messages("alice", rx));
    tokio::task::spawn(print_messages("bob", rx2));

    tx.send(0).await.ok();

    // Charlie will only see the last message
    let rx3 = tx.subscribe();
    tokio::task::spawn(print_messages("charlie", rx3));
    tx2.send(1).await.ok();

    // Wait for all the receivers to print
    tokio::time::sleep(Duration::from_millis(50)).await;
}

async fn print_messages(name: &'static str, mut rx: impl Stream<Item = usize> + Unpin) {
    while let Some(message) = rx.recv().await {
        println!("{} got a message: {}", name, message);
    }
}
