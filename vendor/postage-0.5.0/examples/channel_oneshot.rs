use postage::{oneshot, prelude::Stream, sink::Sink};

#[tokio::main]
async fn main() {
    // Postage provides a standard oneshot channel.
    let (mut tx, mut rx) = oneshot::channel::<usize>();

    // Alice and Bob will see both messages
    tx.send(0).await.ok();

    println!("alice got a message: {:?}", rx.recv().await);
}
