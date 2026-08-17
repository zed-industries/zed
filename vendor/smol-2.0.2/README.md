# smol

[![Build](https://github.com/smol-rs/smol/actions/workflows/ci.yml/badge.svg)](
https://github.com/smol-rs/smol/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](
https://github.com/smol-rs/smol)
[![Cargo](https://img.shields.io/crates/v/smol.svg)](
https://crates.io/crates/smol)
[![Documentation](https://docs.rs/smol/badge.svg)](
https://docs.rs/smol)
[![Chat](https://img.shields.io/matrix/smol-rs%3Amatrix.org)](
https://matrix.to/#/#smol-rs:matrix.org)

<img src="https://raw.githubusercontent.com/smol-rs/smol/master/assets/images/logo_fullsize_transparent.png" alt="kitty" width="100px" style="float: left;" />

A small and fast async runtime.

This crate simply re-exports other smaller async crates (see the source).

To use tokio-based libraries with smol, apply the [`async-compat`] adapter to futures and I/O
types.

See the [`smol-macros`] crate if you want a no proc-macro, fast compiling, easy-to-use
async main and/or multi-threaded Executor setup out of the box.

## Examples

Connect to an HTTP website, make a GET request, and pipe the response to the standard output:

```rust,no_run
use smol::{io, net, prelude::*, Unblock};

fn main() -> io::Result<()> {
    smol::block_on(async {
        let mut stream = net::TcpStream::connect("example.com:80").await?;
        let req = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        stream.write_all(req).await?;

        let mut stdout = Unblock::new(std::io::stdout());
        io::copy(stream, &mut stdout).await?;
        Ok(())
    })
}
```

There's a lot more in the [examples] directory.

[`async-compat`]: https://docs.rs/async-compat/latest/async_compat/
[`smol-macros`]: https://docs.rs/smol-macros/latest/smol_macros/
[examples]: https://github.com/smol-rs/smol/tree/master/examples

## Subcrates

- [async-channel] - Multi-producer multi-consumer channels
- [async-executor] - Composable async executors
- [async-fs] - Async filesystem primitives
- [async-io] - Async adapter for I/O types, also timers
- [async-lock] - Async locks (barrier, mutex, reader-writer lock, semaphore)
- [async-net] - Async networking primitives (TCP/UDP/Unix)
- [async-process] - Async interface for working with processes
- [async-task] - Task abstraction for building executors
- [blocking] - A thread pool for blocking I/O
- [futures-lite] - A lighter fork of [futures]
- [polling] - Portable interface to epoll, kqueue, event ports, and wepoll

[async-io]: https://github.com/smol-rs/async-io
[polling]: https://github.com/smol-rs/polling
[nb-connect]: https://github.com/smol-rs/nb-connect
[async-executor]: https://github.com/smol-rs/async-executor
[async-task]: https://github.com/smol-rs/async-task
[blocking]: https://github.com/smol-rs/blocking
[futures-lite]: https://github.com/smol-rs/futures-lite
[smol]: https://github.com/smol-rs/smol
[async-net]: https://github.com/smol-rs/async-net
[async-process]: https://github.com/smol-rs/async-process
[async-fs]: https://github.com/smol-rs/async-fs
[async-channel]: https://github.com/smol-rs/async-channel
[concurrent-queue]: https://github.com/smol-rs/concurrent-queue
[event-listener]: https://github.com/smol-rs/event-listener
[async-lock]: https://github.com/smol-rs/async-lock
[fastrand]: https://github.com/smol-rs/fastrand
[futures]: https://github.com/rust-lang/futures-rs

## TLS certificate

Some code examples are using TLS for authentication. The repository
contains a self-signed certificate usable for testing, but it should **not**
be used for real-world scenarios. Browsers and tools like curl will
show this certificate as insecure.

In browsers, accept the security prompt or use `curl -k` on the
command line to bypass security warnings.

The certificate file was generated using
[minica](https://github.com/jsha/minica) and
[openssl](https://www.openssl.org/):

```text
minica --domains localhost -ip-addresses 127.0.0.1 -ca-cert certificate.pem
openssl pkcs12 -export -out identity.pfx -inkey localhost/key.pem -in localhost/cert.pem
```

Another useful tool for making certificates is [mkcert].

[mkcert]: https://github.com/FiloSottile/mkcert

## MSRV Policy

The Minimum Supported Rust Version (MSRV) of this crate is **1.63**. As a **tentative** policy, the MSRV will not advance past the [current Rust version provided by Debian Stable](https://packages.debian.org/stable/rust/rustc). At the time of writing, this version of Rust is *1.63*. However, the MSRV may be advanced further in the event of a major ecosystem shift or a security vulnerability.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
