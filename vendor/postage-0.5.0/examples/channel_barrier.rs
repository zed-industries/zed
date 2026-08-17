use std::time::Duration;

use postage::{barrier, prelude::Stream};

#[tokio::main]
async fn main() {
    // Barriers are synchronization tools.
    // They don't carry a value, but instead mark an event.
    // When the sender is dropped, the channel fires.
    let (tx, mut rx) = barrier::channel();

    tokio::task::spawn(async move {
        // move tx into this task
        let _tx = tx;
        tokio::time::sleep(Duration::from_millis(50)).await;
    });

    rx.recv().await;
    println!("OK, I'm ready.")
}
