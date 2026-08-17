use postage::{dispatch, prelude::Stream, sink::Sink};

#[tokio::main]
async fn main() {
    // Dispatch channels are fixed-capacity multi-producer multi-consumer queues.
    // Messages are taken from the channel, rather than cloned (like the broadcast channel)
    let (mut tx, mut rx) = dispatch::channel::<usize>(8);

    let mut rx2 = rx.clone();

    // Alice and Bob will see both messages
    tx.send(0).await.ok();
    tx.send(1).await.ok();
    tx.send(2).await.ok();

    read_message("alice", &mut rx).await;
    read_message("bob", &mut rx2).await;
    read_message("alice", &mut rx).await;
}

async fn read_message(name: &'static str, mut rx: impl Stream<Item = usize> + Unpin) {
    if let Some(message) = rx.recv().await {
        println!("{} got a message: {}", name, message);
    }
}
