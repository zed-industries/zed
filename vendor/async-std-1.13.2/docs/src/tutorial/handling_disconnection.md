## Handling Disconnections

Currently, we only ever _add_ new peers to the map.
This is clearly wrong: if a peer closes connection to the chat, we should not try to send any more messages to it.

One subtlety with handling disconnection is that we can detect it either in the reader's task, or in the writer's task.
The most obvious solution here is to just remove the peer from the `peers` map in both cases, but this would be wrong.
If _both_ read and write fail, we'll remove the peer twice, but it can be the case that the peer reconnected between the two failures!
To fix this, we will only remove the peer when the write side finishes.
If the read side finishes we will notify the write side that it should stop as well.
That is, we need to add an ability to signal shutdown for the writer task.

One way to approach this is a `shutdown: Receiver<()>` channel.
There's a more minimal solution however, which makes clever use of RAII.
Closing a channel is a synchronization event, so we don't need to send a shutdown message, we can just drop the sender.
This way, we statically guarantee that we issue shutdown exactly once, even if we early return via `?` or panic.

First, let's add a shutdown channel to the `connection_loop`:

```rust,edition2018
# extern crate async_std;
# extern crate futures;
# use async_std::net::TcpStream;
# use futures::channel::mpsc;
# use futures::sink::SinkExt;
# use std::sync::Arc;
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# type Sender<T> = mpsc::UnboundedSender<T>;
# type Receiver<T> = mpsc::UnboundedReceiver<T>;
#
#[derive(Debug)]
enum Void {} // 1

#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
        shutdown: Receiver<Void>, // 2
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn connection_loop(mut broker: Sender<Event>, stream: Arc<TcpStream>) -> Result<()> {
    // ...
#   let name: String = unimplemented!();
    let (_shutdown_sender, shutdown_receiver) = mpsc::unbounded::<Void>(); // 3
    broker.send(Event::NewPeer {
        name: name.clone(),
        stream: Arc::clone(&stream),
        shutdown: shutdown_receiver,
    }).await.unwrap();
    // ...
#   unimplemented!()
}
```

1. To enforce that no messages are sent along the shutdown channel, we use an uninhabited type.
2. We pass the shutdown channel to the writer task.
3. In the reader, we create a `_shutdown_sender` whose only purpose is to get dropped.

In the `connection_writer_loop`, we now need to choose between shutdown and message channels.
We use the `select` macro for this purpose:

```rust,edition2018
# extern crate async_std;
# extern crate futures;
# use async_std::{net::TcpStream, prelude::*};
# use futures::channel::mpsc;
use futures::{select, FutureExt};
# use std::sync::Arc;
# type Receiver<T> = mpsc::UnboundedReceiver<T>;
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
# type Sender<T> = mpsc::UnboundedSender<T>;
# #[derive(Debug)]
# enum Void {} // 1

async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: Arc<TcpStream>,
    shutdown: Receiver<Void>, // 1
) -> Result<()> {
    let mut stream = &*stream;
    let mut messages = messages.fuse();
    let mut shutdown = shutdown.fuse();
    loop { // 2
        select! {
            msg = messages.next().fuse() => match msg { // 3
                Some(msg) => stream.write_all(msg.as_bytes()).await?,
                None => break,
            },
            void = shutdown.next().fuse() => match void {
                Some(void) => match void {}, // 4
                None => break,
            }
        }
    }
    Ok(())
}
```

1. We add shutdown channel as an argument.
2. Because of `select`, we can't use a `while let` loop, so we desugar it further into a `loop`.
3. Function fuse() is used to turn any `Stream` into a `FusedStream`. This is used for fusing a stream such that poll_next will never again be called once it has finished.
4. In the shutdown case we use `match void {}` as a statically-checked `unreachable!()`.

Another problem is that between the moment we detect disconnection in `connection_writer_loop` and the moment when we actually remove the peer from the `peers` map, new messages might be pushed into the peer's channel.
To not lose these messages completely, we'll return the messages channel back to the broker.
This also allows us to establish a useful invariant that the message channel strictly outlives the peer in the `peers` map, and makes the broker itself infallible.

## Final Code

The final code looks like this:

```rust,edition2018
# extern crate async_std;
# extern crate futures;
use async_std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::{select, FutureExt};
use std::{
    collections::hash_map::{Entry, HashMap},
    future::Future,
    sync::Arc,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[derive(Debug)]
enum Void {}

// main
fn run() -> Result<()> {
    task::block_on(accept_loop("127.0.0.1:8080"))
}

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
    drop(broker_sender);
    broker_handle.await;
    Ok(())
}

async fn connection_loop(mut broker: Sender<Event>, stream: TcpStream) -> Result<()> {
    let stream = Arc::new(stream);
    let reader = BufReader::new(&*stream);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    let (_shutdown_sender, shutdown_receiver) = mpsc::unbounded::<Void>();
    broker.send(Event::NewPeer {
        name: name.clone(),
        stream: Arc::clone(&stream),
        shutdown: shutdown_receiver,
    }).await.unwrap();

    while let Some(line) = lines.next().await {
        let line = line?;
        let (dest, msg) = match line.find(':') {
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1 ..].trim()),
        };
        let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
        let msg: String = msg.trim().to_string();

        broker.send(Event::Message {
            from: name.clone(),
            to: dest,
            msg,
        }).await.unwrap();
    }

    Ok(())
}

async fn connection_writer_loop(
    messages: &mut Receiver<String>,
    stream: Arc<TcpStream>,
    shutdown: Receiver<Void>,
) -> Result<()> {
    let mut stream = &*stream;
    let mut messages = messages.fuse();
    let mut shutdown = shutdown.fuse();
    loop {
        select! {
            msg = messages.next().fuse() => match msg {
                Some(msg) => stream.write_all(msg.as_bytes()).await?,
                None => break,
            },
            void = shutdown.next().fuse() => match void {
                Some(void) => match void {},
                None => break,
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
        shutdown: Receiver<Void>,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn broker_loop(events: Receiver<Event>) {
    let (disconnect_sender, mut disconnect_receiver) = // 1
        mpsc::unbounded::<(String, Receiver<String>)>();
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();
    let mut events = events.fuse();
    loop {
        let event = select! {
            event = events.next().fuse() => match event {
                None => break, // 2
                Some(event) => event,
            },
            disconnect = disconnect_receiver.next().fuse() => {
                let (name, _pending_messages) = disconnect.unwrap(); // 3
                assert!(peers.remove(&name).is_some());
                continue;
            },
        };
        match event {
            Event::Message { from, to, msg } => {
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {}: {}\n", from, msg);
                        peer.send(msg).await
                            .unwrap() // 6
                    }
                }
            }
            Event::NewPeer { name, stream, shutdown } => {
                match peers.entry(name.clone()) {
                    Entry::Occupied(..) => (),
                    Entry::Vacant(entry) => {
                        let (client_sender, mut client_receiver) = mpsc::unbounded();
                        entry.insert(client_sender);
                        let mut disconnect_sender = disconnect_sender.clone();
                        spawn_and_log_error(async move {
                            let res = connection_writer_loop(&mut client_receiver, stream, shutdown).await;
                            disconnect_sender.send((name, client_receiver)).await // 4
                                .unwrap();
                            res
                        });
                    }
                }
            }
        }
    }
    drop(peers); // 5
    drop(disconnect_sender); // 6
    while let Some((_name, _pending_messages)) = disconnect_receiver.next().await {
    }
}

fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}
```

1. In the broker, we create a channel to reap disconnected peers and their undelivered messages.
2. The broker's main loop exits when the input events channel is exhausted (that is, when all readers exit).
3. Because broker itself holds a `disconnect_sender`, we know that the disconnections channel can't be fully drained in the main loop.
4. We send peer's name and pending messages to the disconnections channel in both the happy and the not-so-happy path.
   Again, we can safely unwrap because the broker outlives writers.
5. We drop `peers` map to close writers' messages channel and shut down the writers for sure.
   It is not strictly necessary in the current setup, where the broker waits for readers' shutdown anyway.
   However, if we add a server-initiated shutdown (for example, kbd:[ctrl+c] handling), this will be a way for the broker to shutdown the writers.
6. Finally, we close and drain the disconnections channel.
