# Mio – Metal I/O

Mio is a fast, low-level I/O library for Rust focusing on non-blocking APIs and
event notification for building high performance I/O apps with as little
overhead as possible over the OS abstractions.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Build Status][cirrus-badge]][cirrus-url]

[crates-badge]: https://img.shields.io/crates/v/mio.svg
[crates-url]: https://crates.io/crates/mio
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[actions-badge]: https://github.com/tokio-rs/mio/workflows/CI/badge.svg
[actions-url]: https://github.com/tokio-rs/mio/actions?query=workflow%3ACI+branch%3Amaster
[cirrus-badge]: https://api.cirrus-ci.com/github/tokio-rs/mio.svg
[cirrus-url]: https://cirrus-ci.com/github/tokio-rs/mio

**API documentation**

* [v0.8](https://docs.rs/mio/^0.8)
* [v0.7](https://docs.rs/mio/^0.7)

This is a low level library, if you are looking for something easier to get
started with, see [Tokio](https://tokio.rs).

## Usage

To use `mio`, first add this to your `Cargo.toml`:

```toml
[dependencies]
mio = "0.8"
```

Next we can start using Mio. The following is quick introduction using
`TcpListener` and `TcpStream`. Note that `features = ["os-poll", "net"]` must be
specified for this example.

```rust
use std::error::Error;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

// Some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the server socket.
    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Setup the client socket.
    let mut client = TcpStream::connect(addr)?;
    // Register the socket.
    poll.registry()
        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    // Start an event loop.
    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            match event.token() {
                SERVER => {
                    // If this is an event for the server, it means a connection
                    // is ready to be accepted.
                    //
                    // Accept the connection and drop it immediately. This will
                    // close the socket and notify the client of the EOF.
                    let connection = server.accept();
                    drop(connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        // We can (likely) write to the socket without blocking.
                    }

                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                    }

                    // Since the server just shuts down the connection, let's
                    // just exit from our event loop.
                    return Ok(());
                }
                // We don't expect any events with tokens other than those we provided.
                _ => unreachable!(),
            }
        }
    }
}
```

## Features

* Non-blocking TCP, UDP
* I/O event queue backed by epoll, kqueue, and IOCP
* Zero allocations at runtime
* Platform specific extensions

## Non-goals

The following are specifically omitted from Mio and are left to the user
or higher-level libraries.

* File operations
* Thread pools / multi-threaded event loop
* Timers

## Platforms

Currently supported platforms:

* Android (API level 21)
* DragonFly BSD
* FreeBSD
* Linux
* NetBSD
* OpenBSD
* Windows
* iOS
* macOS

There are potentially others. If you find that Mio works on another platform,
submit a PR to update the list!

Mio can handle interfacing with each of the event systems of the aforementioned
platforms. The details of their implementation are further discussed in the
`Poll` type of the API documentation (see above).

The Windows implementation for polling sockets is using the [wepoll] strategy.
This uses the Windows AFD system to access socket readiness events.

[wepoll]: https://github.com/piscisaureus/wepoll

### Unsupported

* Haiku, see [issue #1472]
* Solaris, see [issue #1152]
* Wine, see [issue #1444]

[issue #1472]: https://github.com/tokio-rs/mio/issues/1472
[issue #1152]: https://github.com/tokio-rs/mio/issues/1152
[issue #1444]: https://github.com/tokio-rs/mio/issues/1444

## Unsupported flags

Mio uses different implementations to support the same functionality depending
on the platform. Mio generally uses the "best" implementation possible, where
"best" usually means most efficient for Mio's use case. However this means that
the implementation is often specific to a limited number of platforms, meaning
we often have multiple implementations for the same functionality. In some cases
it might be required to not use the "best" implementation, but another
implementation Mio supports (on other platforms). **Mio does not officially
support secondary implementations on platforms**, however we do have various cfg
flags to force another implementation for these situations.

Current flags:
 * `mio_unsupported_force_poll_poll`, uses an implementation based on `poll(2)`
   for `mio::Poll`.
 * `mio_unsupported_force_waker_pipe`, uses an implementation based on `pipe(2)`
   for `mio::Waker`.

**Again, Mio does not officially supports this**. Furthermore these flags may
disappear in the future.

## Community

A group of Mio users hang out on [Discord], this can be a good place to go for
questions. It's also possible to open a [new issue on GitHub] to ask questions,
report bugs or suggest new features.

[Discord]: https://discord.gg/tokio
[new issue on GitHub]: https://github.com/tokio-rs/mio/issues/new

## Contributing

Interested in getting involved? We would love to help you! For simple
bug fixes, just submit a PR with the fix and we can discuss the fix
directly in the PR. If the fix is more complex, start with an issue.

If you want to propose an API change, create an issue to start a
discussion with the community. Also, feel free to talk with us in Discord.

Finally, be kind. We support the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
