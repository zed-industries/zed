## Receiving messages

Let's implement the receiving part of the protocol.
We need to:

1. split incoming `TcpStream` on `\n` and decode bytes as utf-8
2. interpret the first line as a login
3. parse the rest of the lines as a  `login: message`

```rust,edition2018
# extern crate async_std;
# use async_std::{
#     net::{TcpListener, ToSocketAddrs},
#     prelude::*,
#     task,
# };
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#
use async_std::{
    io::BufReader,
    net::TcpStream,
};

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);
        let _handle = task::spawn(connection_loop(stream)); // 1
    }
    Ok(())
}

async fn connection_loop(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&stream); // 2
    let mut lines = reader.lines();

    let name = match lines.next().await { // 3
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    println!("name = {}", name);

    while let Some(line) = lines.next().await { // 4
        let line = line?;
        let (dest, msg) = match line.find(':') { // 5
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1 ..].trim()),
        };
        let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
        let msg: String = msg.to_string();
    }
    Ok(())
}
```

1. We use `task::spawn` function to spawn an independent task for working with each client.
   That is, after accepting the client the `accept_loop` immediately starts waiting for the next one.
   This is the core benefit of event-driven architecture: we serve many clients concurrently, without spending many hardware threads.

2. Luckily, the "split byte stream into lines" functionality is already implemented.
   `.lines()` call returns a stream of `String`'s.

3. We get the first line -- login

4. And, once again, we implement a manual async for loop.

5. Finally, we parse each line into a list of destination logins and the message itself.

## Managing Errors

One serious problem in the above solution is that, while we correctly propagate errors in the `connection_loop`, we just drop the error on the floor afterwards!
That is, `task::spawn` does not return an error immediately (it can't, it needs to run the future to completion first), only after it is joined.
We can "fix" it by waiting for the task to be joined, like this:

```rust,edition2018
# #![feature(async_closure)]
# extern crate async_std;
# use async_std::{
#     io::BufReader,
#     net::{TcpListener, TcpStream, ToSocketAddrs},
#     prelude::*,
#     task,
# };
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#
# async fn connection_loop(stream: TcpStream) -> Result<()> {
#     let reader = BufReader::new(&stream); // 2
#     let mut lines = reader.lines();
#
#     let name = match lines.next().await { // 3
#         None => Err("peer disconnected immediately")?,
#         Some(line) => line?,
#     };
#     println!("name = {}", name);
#
#     while let Some(line) = lines.next().await { // 4
#         let line = line?;
#         let (dest, msg) = match line.find(':') { // 5
#             None => continue,
#             Some(idx) => (&line[..idx], line[idx + 1 ..].trim()),
#         };
#         let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
#         let msg: String = msg.trim().to_string();
#     }
#     Ok(())
# }
#
# async move |stream| {
let handle = task::spawn(connection_loop(stream));
handle.await?
# };
```

The `.await` waits until the client finishes, and `?` propagates the result.

There are two problems with this solution however!
*First*, because we immediately await the client, we can only handle one client at a time, and that completely defeats the purpose of async!
*Second*, if a client encounters an IO error, the whole server immediately exits.
That is, a flaky internet connection of one peer brings down the whole chat room!

A correct way to handle client errors in this case is log them, and continue serving other clients.
So let's use a helper function for this:

```rust,edition2018
# extern crate async_std;
# use async_std::{
#     io,
#     prelude::*,
#     task,
# };
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
