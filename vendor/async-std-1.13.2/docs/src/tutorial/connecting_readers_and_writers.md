
## Connecting Readers and Writers

So how do we make sure that messages read in `connection_loop` flow into the relevant `connection_writer_loop`?
We should somehow maintain a `peers: HashMap<String, Sender<String>>` map which allows a client to find destination channels.
However, this map would be a bit of shared mutable state, so we'll have to wrap an `RwLock` over it and answer tough questions of what should happen if the client joins at the same moment as it receives a message.

One trick to make reasoning about state simpler comes from the actor model.
We can create a dedicated broker task which owns the `peers` map and communicates with other tasks using channels.
By hiding `peers` inside such an "actor" task, we remove the need for mutexes and also make the serialization point explicit.
The order of events "Bob sends message to Alice" and "Alice joins" is determined by the order of the corresponding events in the broker's event queue.

```rust,edition2018
# extern crate async_std;
# extern crate futures;
# use async_std::{
#     net::TcpStream,
#     prelude::*,
#     task,
# };
# use futures::channel::mpsc;
# use futures::sink::SinkExt;
# use std::sync::Arc;
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# type Sender<T> = mpsc::UnboundedSender<T>;
# type Receiver<T> = mpsc::UnboundedReceiver<T>;
#
# async fn connection_writer_loop(
#     mut messages: Receiver<String>,
#     stream: Arc<TcpStream>,
# ) -> Result<()> {
#     let mut stream = &*stream;
#     while let Some(msg) = messages.next().await {
#         stream.write_all(msg.as_bytes()).await?;
#     }
#     Ok(())
# }
#
# fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
# where
#     F: Future<Output = Result<()>> + Send + 'static,
# {
#     task::spawn(async move {
#         if let Err(e) = fut.await {
#             eprintln!("{}", e)
#         }
#     })
# }
#
use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug)]
enum Event { // 1
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn broker_loop(mut events: Receiver<Event>) -> Result<()> {
    let mut peers: HashMap<String, Sender<String>> = HashMap::new(); // 2

    while let Some(event) = events.next().await {
        match event {
            Event::Message { from, to, msg } => {  // 3
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);
                        peer.send(msg).await?
                    }
                }
            }
            Event::NewPeer { name, stream } => {
                match peers.entry(name) {
                    Entry::Occupied(..) => (),
                    Entry::Vacant(entry) => {
                        let (client_sender, client_receiver) = mpsc::unbounded();
                        entry.insert(client_sender); // 4
                        spawn_and_log_error(connection_writer_loop(client_receiver, stream)); // 5
                    }
                }
            }
        }
    }
    Ok(())
}
```

1. The broker task should handle two types of events: a message or an arrival of a new peer.
2. The internal state of the broker is a `HashMap`.
   Note how we don't need a `Mutex` here and can confidently say, at each iteration of the broker's loop, what is the current set of peers
3. To handle a message, we send it over a channel to each destination
4. To handle a new peer, we first register it in the peer's map ...
5. ... and then spawn a dedicated task to actually write the messages to the socket.
