#[cfg(feature = "futures-traits")]
use futures::{SinkExt, StreamExt};
#[cfg(feature = "futures-traits")]
use postage::{mpsc, sink::SendError};

// ignore this.. build issue hack
macro_rules! tokio_main_if {
    ($feature:expr => async fn main() $body:block) => {
        #[tokio::main]
        async fn main() {
            #[cfg(feature = $feature)]
            inner().await;
        }

        #[cfg(feature = $feature)]
        async fn inner() {
            $body
        }
    };
}

tokio_main_if! ("futures-traits" => async fn main() {
    let (mut tx, mut rx) = mpsc::channel(2);

    tx.send(0usize).await.ok();
    tx.send(1usize).await.ok();

    println!("Sender says {:?}", rx.next().await);
    println!("Sender says {:?}", rx.next().await);

    // Let's deal with some errors.
    tx.send(0usize).await.ok();
    tx.send(1usize).await.ok();

    // If the channel is closed, no value is returned.
    // Unfortunately this is due to the design of futures::Sink
    drop(rx);
    assert_eq!(Err(SendError(0usize)), tx.send(0usize).await);
});
