# async-process

[![Build](https://github.com/smol-rs/async-process/actions/workflows/ci.yml/badge.svg)](
https://github.com/smol-rs/async-process/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](
https://github.com/smol-rs/async-process)
[![Cargo](https://img.shields.io/crates/v/async-process.svg)](
https://crates.io/crates/async-process)
[![Documentation](https://docs.rs/async-process/badge.svg)](
https://docs.rs/async-process)

Async interface for working with processes.

This crate is an async version of `std::process`.

## Implementation

A background thread named "async-process" is lazily created on first use, which waits for
spawned child processes to exit and then calls the `wait()` syscall to clean up the "zombie"
processes. This is unlike the `process` API in the standard library, where dropping a running
`Child` leaks its resources.

This crate uses [`async-io`] for async I/O on Unix-like systems and [`blocking`] for async I/O
on Windows.

[`async-io`]: https://docs.rs/async-io
[`blocking`]: https://docs.rs/blocking

## Examples

Spawn a process and collect its output:

```rust
use async_process::Command;

let out = Command::new("echo").arg("hello").arg("world").output().await?;
assert_eq!(out.stdout, b"hello world\n");
```

Read the output line-by-line as it gets produced:

```rust
use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};

let mut child = Command::new("find")
    .arg(".")
    .stdout(Stdio::piped())
    .spawn()?;

let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

while let Some(line) = lines.next().await {
    println!("{}", line?);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
