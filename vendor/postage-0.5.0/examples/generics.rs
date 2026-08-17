use postage::{
    mpsc,
    prelude::Stream,
    sink::{SendError, Sink},
};

#[tokio::main]
async fn main() {
    let (mut tx, rx) = mpsc::channel(8);

    tokio::task::spawn(async move {
        send_message(&mut tx, "Hello!").await.ok();
        send_message(&mut tx, "Helooooo!").await.ok();
        if let Err(e) = send_message(tx, "This is important").await {
            println!("Failed to send a message to the receiver: {}", e);
        }
    });

    // consumes rx, and prints all the message that the channel emits
    print_messages(rx).await;
}

/// impl Trait can be used to pass channel endpoints to functions
async fn send_message(
    mut tx: impl Sink<Item = String> + Unpin,
    message: &str,
) -> Result<(), SendError<String>> {
    tx.send(message.to_string() + " world!").await
}

async fn print_messages(mut rx: impl Stream<Item = String> + Unpin) {
    while let Some(message) = rx.recv().await {
        println!("Sender says {}", message)
    }
}
