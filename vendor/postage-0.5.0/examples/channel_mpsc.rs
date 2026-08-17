use postage::{mpsc, prelude::Stream, sink::Sink};

#[tokio::main]
async fn main() {
    // Postage provides a standard multi-producer, single-consumer queue.
    let (mut tx, mut rx) = mpsc::channel::<usize>(8);

    let mut tx2 = tx.clone();

    // Alice and Bob will see both messages
    tx.send(0).await.ok();
    tx2.send(1).await.ok();
    tx.send(2).await.ok();

    read_message("alice", &mut rx).await;
    read_message("alice", &mut rx).await;
    read_message("alice", &mut rx).await;
}

async fn read_message(name: &'static str, mut rx: impl Stream<Item = usize> + Unpin) {
    if let Some(message) = rx.recv().await {
        println!("{} got a message: {}", name, message);
    }
}
