//! A TCP chat server.
//!
//! First start a server:
//!
//! ```
//! cargo run --example chat-server
//! ```
//!
//! Then start clients:
//!
//! ```
//! cargo run --example chat-client
//! ```

use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};

use async_channel::{bounded, Receiver, Sender};
use async_dup::Arc;
use smol::{io, prelude::*, Async};

/// An event on the chat server.
enum Event {
    /// A client has joined.
    Join(SocketAddr, Arc<Async<TcpStream>>),

    /// A client has left.
    Leave(SocketAddr),

    /// A client sent a message.
    Message(SocketAddr, String),
}

/// Dispatches events to clients.
async fn dispatch(receiver: Receiver<Event>) -> io::Result<()> {
    // Currently active clients.
    let mut map = HashMap::<SocketAddr, Arc<Async<TcpStream>>>::new();

    // Receive incoming events.
    while let Ok(event) = receiver.recv().await {
        // Process the event and format a message to send to clients.
        let output = match event {
            Event::Join(addr, stream) => {
                map.insert(addr, stream);
                format!("{} has joined\n", addr)
            }
            Event::Leave(addr) => {
                map.remove(&addr);
                format!("{} has left\n", addr)
            }
            Event::Message(addr, msg) => format!("{} says: {}\n", addr, msg),
        };

        // Display the event in the server process.
        print!("{}", output);

        // Send the event to all active clients.
        for stream in map.values_mut() {
            // Ignore errors because the client might disconnect at any point.
            stream.write_all(output.as_bytes()).await.ok();
        }
    }
    Ok(())
}

/// Reads messages from the client and forwards them to the dispatcher task.
async fn read_messages(sender: Sender<Event>, client: Arc<Async<TcpStream>>) -> io::Result<()> {
    let addr = client.get_ref().peer_addr()?;
    let mut lines = io::BufReader::new(client).lines();

    while let Some(line) = lines.next().await {
        let line = line?;
        sender.send(Event::Message(addr, line)).await.ok();
    }
    Ok(())
}

fn main() -> io::Result<()> {
    smol::block_on(async {
        // Create a listener for incoming client connections.
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 6000))?;

        // Intro messages.
        println!("Listening on {}", listener.get_ref().local_addr()?);
        println!("Start a chat client now!\n");

        // Spawn a background task that dispatches events to clients.
        let (sender, receiver) = bounded(100);
        smol::spawn(dispatch(receiver)).detach();

        loop {
            // Accept the next connection.
            let (stream, addr) = listener.accept().await?;
            let client = Arc::new(stream);
            let sender = sender.clone();

            // Spawn a background task reading messages from the client.
            smol::spawn(async move {
                // Client starts with a `Join` event.
                sender.send(Event::Join(addr, client.clone())).await.ok();

                // Read messages from the client and ignore I/O errors when the client quits.
                read_messages(sender.clone(), client).await.ok();

                // Client ends with a `Leave` event.
                sender.send(Event::Leave(addr)).await.ok();
            })
            .detach();
        }
    })
}
