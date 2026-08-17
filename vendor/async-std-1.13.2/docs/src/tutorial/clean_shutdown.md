## Clean Shutdown

One of the problems of the current implementation is that it doesn't handle graceful shutdown.
If we break from the accept loop for some reason, all in-flight tasks are just dropped on the floor.
A more correct shutdown sequence would be:

1. Stop accepting new clients
2. Deliver all pending messages
3. Exit the process

A clean shutdown in a channel based architecture is easy, although it can appear a magic trick at first.
In Rust, receiver side of a channel is closed as soon as all senders are dropped.
That is, as soon as producers exit and drop their senders, the rest of the system shuts down naturally.
In `async_std` this translates to two rules:

1. Make sure that channels form an acyclic graph.
2. Take care to wait, in the correct order, until intermediate layers of the system process pending messages.

In `a-chat`, we already have an unidirectional flow of messages: `reader -> broker -> writer`.
However, we never wait for broker and writers, which might cause some messages to get dropped.
Let's add waiting to the server:

```rust,edition2018
# extern crate async_std;
# extern crate futures;
# use async_std::{
#     io::{self, BufReader},
#     net::{TcpListener, TcpStream, ToSocketAddrs},
#     prelude::*,
#     task,
# };
# use futures::channel::mpsc;
# use futures::sink::SinkExt;
# use std::{
#     collections::hash_map::{HashMap, Entry},
#     sync::Arc,
# };
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# type Sender<T> = mpsc::UnboundedSender<T>;
# type Receiver<T> = mpsc::UnboundedReceiver<T>;
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
#
# async fn connection_loop(mut broker: Sender<Event>, stream: TcpStream) -> Result<()> {
#     let stream = Arc::new(stream); // 2
#     let reader = BufReader::new(&*stream);
#     let mut lines = reader.lines();
#
#     let name = match lines.next().await {
#         None => Err("peer disconnected immediately")?,
#         Some(line) => line?,
#     };
#     broker.send(Event::NewPeer { name: name.clone(), stream: Arc::clone(&stream) }).await // 3
#         .unwrap();
#
#     while let Some(line) = lines.next().await {
#         let line = line?;
#         let (dest, msg) = match line.find(':') {
#             None => continue,
#             Some(idx) => (&line[..idx], line[idx + 1 ..].trim()),
#         };
#         let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
#         let msg: String = msg.trim().to_string();
#
#         broker.send(Event::Message { // 4
#             from: name.clone(),
#             to: dest,
#             msg,
#         }).await.unwrap();
#     }
#     Ok(())
# }
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
# #[derive(Debug)]
# enum Event {
#     NewPeer {
#         name: String,
#         stream: Arc<TcpStream>,
#     },
#     Message {
#         from: String,
#         to: Vec<String>,
#         msg: String,
#     },
# }
#
# async fn broker_loop(mut events: Receiver<Event>) -> Result<()> {
#     let mut peers: HashMap<String, Sender<String>> = HashMap::new();
#
#     while let Some(event) = events.next().await {
#         match event {
#             Event::Message { from, to, msg } => {
#                 for addr in to {
#                     if let Some(peer) = peers.get_mut(&addr) {
#                         let msg = format!("from {}: {}\n", from, msg);
#                         peer.send(msg).await?
#                     }
#                 }
#             }
#             Event::NewPeer { name, stream} => {
#                 match peers.entry(name) {
#                     Entry::Occupied(..) => (),
#                     Entry::Vacant(entry) => {
#                         let (client_sender, client_receiver) = mpsc::unbounded();
#                         entry.insert(client_sender); // 4
#                         spawn_and_log_error(connection_writer_loop(client_receiver, stream)); // 5
#                     }
#                 }
#             }
#         }
#     }
#     Ok(())
# }
#
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let (broker_sender, broker_receiver) = mpsc::unbounded();
    let broker_handle = task::spawn(broker_loop(broker_receiver));
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        spawn_and_log_error(connection_loop(broker_sender.clone(), stream));
    }
    drop(broker_sender); // 1
    broker_handle.await?; // 5
    Ok(())
}
```

And to the broker:

```rust,edition2018
# extern crate async_std;
# extern crate futures;
# use async_std::{
#     io::{self, BufReader},
#     net::{TcpListener, TcpStream, ToSocketAddrs},
#     prelude::*,
#     task,
# };
# use futures::channel::mpsc;
# use futures::sink::SinkExt;
# use std::{
#     collections::hash_map::{HashMap, Entry},
#     sync::Arc,
# };
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
# #[derive(Debug)]
# enum Event {
#     NewPeer {
#         name: String,
#         stream: Arc<TcpStream>,
#     },
#     Message {
#         from: String,
#         to: Vec<String>,
#         msg: String,
#     },
# }
#
async fn broker_loop(mut events: Receiver<Event>) -> Result<()> {
    let mut writers = Vec::new();
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();
    while let Some(event) = events.next().await { // 2
        match event {
            Event::Message { from, to, msg } => {
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);
                        peer.send(msg).await?
                    }
                }
            }
            Event::NewPeer { name, stream} => {
                match peers.entry(name) {
                    Entry::Occupied(..) => (),
                    Entry::Vacant(entry) => {
                        let (client_sender, client_receiver) = mpsc::unbounded();
                        entry.insert(client_sender);
                        let handle = spawn_and_log_error(connection_writer_loop(client_receiver, stream));
                        writers.push(handle); // 4
                    }
                }
            }
        }
    }
    drop(peers); // 3
    for writer in writers { // 4
        writer.await;
    }
    Ok(())
}
```

Notice what happens with all of the channels once we exit the accept loop:

1. First, we drop the main broker's sender.
   That way when the readers are done, there's no sender for the broker's channel, and the channel closes.
2. Next, the broker exits `while let Some(event) = events.next().await` loop.
3. It's crucial that, at this stage, we drop the `peers` map.
   This drops writer's senders.
4. Now we can join all of the writers.
5. Finally, we join the broker, which also guarantees that all the writes have terminated.
