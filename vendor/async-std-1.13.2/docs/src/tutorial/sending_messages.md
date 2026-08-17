## Sending Messages

Now it's time to implement the other half -- sending messages.
A most obvious way to implement sending is to give each `connection_loop` access to the write half of `TcpStream` of each other clients.
That way, a client can directly `.write_all` a message to recipients.
However, this would be wrong: if Alice sends `bob: foo`, and Charley sends `bob: bar`, Bob might actually receive `fobaor`.
Sending a message over a socket might require several syscalls, so two concurrent `.write_all`'s might interfere with each other!

As a rule of thumb, only a single task should write to each `TcpStream`.
So let's create a `connection_writer_loop` task which receives messages over a channel and writes them to the socket.
This task would be the point of serialization of messages.
if Alice and Charley send two messages to Bob at the same time, Bob will see the messages in the same order as they arrive in the channel.

```rust,edition2018
# extern crate async_std;
# extern crate futures;
# use async_std::{
#     net::TcpStream,
#     prelude::*,
# };
use futures::channel::mpsc; // 1
use futures::sink::SinkExt;
use std::sync::Arc;

# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>; // 2
type Receiver<T> = mpsc::UnboundedReceiver<T>;

async fn connection_writer_loop(
    mut messages: Receiver<String>,
    stream: Arc<TcpStream>, // 3
) -> Result<()> {
    let mut stream = &*stream;
    while let Some(msg) = messages.next().await {
        stream.write_all(msg.as_bytes()).await?;
    }
    Ok(())
}
```

1. We will use channels from the `futures` crate.
2. For simplicity, we will use `unbounded` channels, and won't be discussing backpressure in this tutorial.
3. As `connection_loop` and `connection_writer_loop` share the same `TcpStream`, we need to put it into an `Arc`.
   Note that because `client` only reads from the stream and `connection_writer_loop` only writes to the stream, we don't get a race here.
