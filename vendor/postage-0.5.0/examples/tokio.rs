use postage::{mpsc, prelude::Stream, sink::Sink};

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(8);

    tokio::task::spawn(async move {
        tx.send("Hello".to_string()).await.ok();
        tx.send("World".to_string()).await.ok();
    });

    while let Some(message) = rx.recv().await {
        println!("Sender says {}", message)
    }
}
