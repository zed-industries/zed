use postage::{mpsc, oneshot, prelude::Stream, sink::Sink};

#[derive(Debug)]
enum Message {
    Str(&'static str),
    Code(usize),
}

#[tokio::main]
async fn main() {
    let (mut tx_a, rx_a) = mpsc::channel(8);
    let (mut tx_b, rx_b) = oneshot::channel();

    tx_a.send("Hello!").await.ok();
    tx_b.send(0usize).await.ok();

    let mut rx = rx_a
        // map the first reciever to a common enum type
        .map(|a| Message::Str(a))
        // map the 2nd receiver to the enum type, and then merge it with the first
        .merge(rx_b.map(|b| Message::Code(b)));

    while let Some(message) = rx.recv().await {
        println!("Sender says {:?}", message)
    }
}
