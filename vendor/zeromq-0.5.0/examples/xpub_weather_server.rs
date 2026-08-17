mod async_helpers;

use rand::Rng;
use std::sync::Arc;
use std::time::Duration;

use zeromq::*;

#[async_helpers::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::try_init().ok();

    println!("Starting XPUB weather server");
    println!("This example demonstrates the key difference between PUB and XPUB:");
    println!("XPUB exposes subscription/unsubscription messages to the application\n");

    let mut socket = zeromq::XPubSocket::new();
    socket.bind("tcp://127.0.0.1:5557").await?;

    println!("XPUB server bound to tcp://127.0.0.1:5557");
    println!(
        "Run the weather_client example and connect to port 5557 to see subscription messages\n"
    );

    // Spawn a background task to handle subscription messages
    // This is the key difference between PUB and XPUB:
    // XPUB exposes subscription/unsubscription messages to the application
    let subscription_counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let _counter_clone = subscription_counter.clone();

    #[cfg(feature = "tokio-runtime")]
    let _subscription_handle = tokio::spawn(async move {
        loop {
            match socket.recv().await {
                Ok(msg) => {
                    let data = msg.get(0).unwrap();
                    if !data.is_empty() {
                        match data[0] {
                            1 => {
                                // Subscribe message (byte value 1)
                                let topic = String::from_utf8_lossy(&data[1..]);
                                if topic.is_empty() {
                                    println!("📥 Client subscribed to ALL topics");
                                } else {
                                    println!("📥 Client subscribed to: '{}'", topic);
                                }
                                _counter_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                            0 => {
                                // Unsubscribe message (byte value 0)
                                let topic = String::from_utf8_lossy(&data[1..]);
                                if topic.is_empty() {
                                    println!("📤 Client unsubscribed from ALL topics");
                                } else {
                                    println!("📤 Client unsubscribed from: '{}'", topic);
                                }
                            }
                            _ => {
                                println!("⚠️  Unknown subscription message type: {}", data[0]);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving subscription: {:?}", e);
                    break;
                }
            }
        }
    });

    #[cfg(feature = "async-std-runtime")]
    let _subscription_handle = async_std::task::spawn(async move {
        loop {
            match socket.recv().await {
                Ok(msg) => {
                    let data = msg.get(0).unwrap();
                    if !data.is_empty() {
                        match data[0] {
                            1 => {
                                let topic = String::from_utf8_lossy(&data[1..]);
                                if topic.is_empty() {
                                    println!("📥 Client subscribed to ALL topics");
                                } else {
                                    println!("📥 Client subscribed to: '{}'", topic);
                                }
                                _counter_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                            0 => {
                                let topic = String::from_utf8_lossy(&data[1..]);
                                if topic.is_empty() {
                                    println!("📤 Client unsubscribed from ALL topics");
                                } else {
                                    println!("📤 Client unsubscribed from: '{}'", topic);
                                }
                            }
                            _ => {
                                println!("⚠️  Unknown subscription message type: {}", data[0]);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving subscription: {:?}", e);
                    break;
                }
            }
        }
    });

    // Give the subscription handler time to start
    async_helpers::sleep(Duration::from_millis(100)).await;

    // Create a new socket for publishing (since we moved the first one into the task)
    let mut pub_socket = zeromq::XPubSocket::new();
    pub_socket.bind("tcp://127.0.0.1:5558").await?;

    println!("Publishing weather updates on tcp://127.0.0.1:5558");
    println!("Format: <zipcode> <temperature> <humidity>\n");

    let mut rng = rand::rng();
    let mut counter = 0;

    loop {
        let zipcode = rng.random_range(10000..10010);
        let temperature = rng.random_range(-80..135);
        let relhumidity = rng.random_range(10..60);

        let message = format!("{} {} {}", zipcode, temperature, relhumidity);
        pub_socket.send(message.into()).await?;

        counter += 1;
        if counter % 50 == 0 {
            let subs = subscription_counter.load(std::sync::atomic::Ordering::Relaxed);
            println!(
                "📡 Sent {} weather updates (active subscriptions: {})",
                counter, subs
            );
        }

        async_helpers::sleep(Duration::from_millis(100)).await;
    }
}
