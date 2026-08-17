use postage::{
    mpsc,
    sink::{Sink, TrySendError},
};

#[tokio::main]
async fn main() {
    // Channel senders require mutable references to take messages.
    let (mut tx, rx) = mpsc::channel(8);

    // Senders are fallible.  If a channel is closed, there is nowhere to send the message.
    // If you want to ignore the error, you can use Result::ok
    tx.send("Hello!").await.ok();

    // If you want to check for an error, you can recover the message with `if let`
    // Let's drop rx to trigger the error.
    drop(rx);

    if let Err(message) = tx.send("This is important").await {
        println!("Failed to send a message to the receiver: {}", message);
    }

    // You can also try to send a message without blocking:
    match tx.try_send("You there?") {
        Ok(()) => {
            // the message was sent!
        }
        Err(TrySendError::Pending(_message)) => {
            // the message was returned, but it might be accepted later
        }
        Err(TrySendError::Rejected(_message)) => {
            // the message was rejected.
            // the channel will never accept this message.
        }
    }
}
