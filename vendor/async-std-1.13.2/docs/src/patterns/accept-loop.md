# Production-Ready Accept Loop

A production-ready accept loop needs the following things:
1. Handling errors
2. Limiting the number of simultanteous connections to avoid deny-of-service
   (DoS) attacks


## Handling errors

There are two kinds of errors in an accept loop:
1. Per-connection errors. The system uses them to notify that there was a
   connection in the queue and it's dropped by the peer. Subsequent connections
   can be already queued so next connection must be accepted immediately.
2. Resource shortages. When these are encountered it doesn't make sense to
   accept the next socket immediately. But the listener stays active, so you server
   should try to accept socket later.

Here is the example of a per-connection error (printed in normal and debug mode):
```
Error: Connection reset by peer (os error 104)
Error: Os { code: 104, kind: ConnectionReset, message: "Connection reset by peer" }
```

And the following is the most common example of a resource shortage error:
```
Error: Too many open files (os error 24)
Error: Os { code: 24, kind: Other, message: "Too many open files" }
```

### Testing Application

To test your application for these errors try the following (this works
on unixes only).

Lower limits and start the application:
```
$ ulimit -n 100
$ cargo run --example your_app
   Compiling your_app v0.1.0 (/work)
    Finished dev [unoptimized + debuginfo] target(s) in 5.47s
     Running `target/debug/examples/your_app`
Server is listening on: http://127.0.0.1:1234
```
Then in another console run the [`wrk`] benchmark tool:
```
$ wrk -c 1000 http://127.0.0.1:1234
Running 10s test @ http://localhost:8080/
  2 threads and 1000 connections
$ telnet localhost 1234
Trying ::1...
Connected to localhost.
```

Important is to check the following things:

1. The application doesn't crash on error (but may log errors, see below)
2. It's possible to connect to the application again once load is stopped
   (few seconds after `wrk`). This is what `telnet` does in example above,
   make sure it prints `Connected to <hostname>`.
3. The `Too many open files` error is logged in the appropriate log. This
   requires to set "maximum number of simultaneous connections" parameter (see
   below) of your application to a value greater then `100` for this example.
4. Check CPU usage of the app while doing a test. It should not occupy 100%
   of a single CPU core (it's unlikely that you can exhaust CPU by 1000
   connections in Rust, so this means error handling is not right).

#### Testing non-HTTP applications

If it's possible, use the appropriate benchmark tool and set the appropriate
number of connections. For example `redis-benchmark` has a `-c` parameter for
that, if you implement redis protocol.

Alternatively, can still use `wrk`, just make sure that connection is not
immediately closed. If it is, put a temporary timeout before handing
the connection to the protocol handler, like this:

```rust,edition2018
# extern crate async_std;
# use std::time::Duration;
# use async_std::{
#     net::{TcpListener, ToSocketAddrs},
#     prelude::*,
# };
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#
#async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
#    let listener = TcpListener::bind(addr).await?;
#    let mut incoming = listener.incoming();
while let Some(stream) = incoming.next().await {
    task::spawn(async {
        task::sleep(Duration::from_secs(10)).await; // 1
        connection_loop(stream).await;
    });
}
#     Ok(())
# }
```

1. Make sure the sleep coroutine is inside the spawned task, not in the loop.

[`wrk`]: https://github.com/wg/wrk


### Handling Errors Manually

Here is how basic accept loop could look like:

```rust,edition2018
# extern crate async_std;
# use std::time::Duration;
# use async_std::{
#     net::{TcpListener, ToSocketAddrs},
#     prelude::*,
# };
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(result) = incoming.next().await {
        let stream = match result {
            Err(ref e) if is_connection_error(e) => continue, // 1
            Err(e) => {
                eprintln!("Error: {}. Pausing for 500ms.", e); // 3
                task::sleep(Duration::from_millis(500)).await; // 2
                continue;
            }
            Ok(s) => s,
        };
        // body
    }
    Ok(())
}
```

1. Ignore per-connection errors.
2. Sleep and continue on resource shortage.
3. It's important to log the message, because these errors commonly mean the
   misconfiguration of the system and are helpful for operations people running
   the application.

Be sure to [test your application](#testing-application).


### External Crates

The crate [`async-listen`] has a helper to achieve this task:
```rust,edition2018
# extern crate async_std;
# extern crate async_listen;
# use std::time::Duration;
# use async_std::{
#     net::{TcpListener, ToSocketAddrs},
#     prelude::*,
# };
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#
use async_listen::{ListenExt, error_hint};

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {

    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener
        .incoming()
        .log_warnings(log_accept_error) // 1
        .handle_errors(Duration::from_millis(500));
    while let Some(socket) = incoming.next().await { // 2
        // body
    }
    Ok(())
}

fn log_accept_error(e: &io::Error) {
    eprintln!("Error: {}. Listener paused for 0.5s. {}", e, error_hint(e)) // 3
}
```

1. Logs resource shortages (`async-listen` calls them warnings). If you use
   `log` crate or any other in your app this should go to the log.
2. Stream yields sockets without `Result` wrapper after `handle_errors` because
   all errors are already handled.
3. Together with the error we print a hint, which explains some errors for end
   users. For example, it recommends increasing open file limit and gives
   a link.

[`async-listen`]: https://crates.io/crates/async-listen/

Be sure to [test your application](#testing-application).


## Connections Limit

Even if you've applied everything described in
[Handling Errors](#handling-errors) section, there is still a problem.

Let's imagine you have a server that needs to open a file to process
client request. At some point, you might encounter the following situation:

1. There are as many client connection as max file descriptors allowed for
   the application.
2. Listener gets `Too many open files` error so it sleeps.
3. Some client sends a request via the previously open connection.
4. Opening a file to serve request fails, because of the same
   `Too many open files` error, until some other client drops a connection.

There are many more possible situations, this is just a small illustation that
limiting number of connections is very useful. Generally, it's one of the ways
to control resources used by a server and avoiding some kinds of deny of
service (DoS) attacks.

### `async-listen` crate

Limiting maximum number of simultaneous connections with [`async-listen`]
looks like the following:

```rust,edition2018
# extern crate async_std;
# extern crate async_listen;
# use std::time::Duration;
# use async_std::{
#     net::{TcpListener, TcpStream, ToSocketAddrs},
#     prelude::*,
# };
#
# type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#
use async_listen::{ListenExt, Token, error_hint};

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {

    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener
        .incoming()
        .log_warnings(log_accept_error)
        .handle_errors(Duration::from_millis(500)) // 1
        .backpressure(100);
    while let Some((token, socket)) = incoming.next().await { // 2
         task::spawn(async move {
             connection_loop(&token, stream).await; // 3
         });
    }
    Ok(())
}
async fn connection_loop(_token: &Token, stream: TcpStream) { // 4
    // ...
}
# fn log_accept_error(e: &io::Error) {
#     eprintln!("Error: {}. Listener paused for 0.5s. {}", e, error_hint(e));
# }
```

1. We need to handle errors first, because [`backpressure`] helper expects
   stream of `TcpStream` rather than `Result`.
2. The token yielded by a new stream is what is counted by backpressure helper.
   I.e. if you drop a token, new connection can be established.
3. We give the connection loop a reference to token to bind token's lifetime to
   the lifetime of the connection.
4. The token itsellf in the function can be ignored, hence `_token`

[`backpressure`]: https://docs.rs/async-listen/0.1.2/async_listen/trait.ListenExt.html#method.backpressure

Be sure to [test this behavior](#testing-application).
