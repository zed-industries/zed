# async-signal

[![Build](https://github.com/smol-rs/async-signal/workflows/Build%20and%20test/badge.svg)](
https://github.com/smol-rs/async-signal/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](
https://github.com/smol-rs/async-signal)
[![Cargo](https://img.shields.io/crates/v/async-signal.svg)](
https://crates.io/crates/async-signal)
[![Documentation](https://docs.rs/async-signal/badge.svg)](
https://docs.rs/async-signal)

Asynchronous signal handling.
 
This crate provides the [`Signals`] type, which can be used to listen for POSIX signals asynchronously. It can be seen as an asynchronous version of [`signal_hook::iterator::Signals`].

[`Signals`]: https://docs.rs/async-signal/latest/async_signal/struct.Signals.html
[`signal_hook::iterator::Signals`]: https://docs.rs/signal-hook/latest/signal_hook/iterator/struct.Signals.html

# Implementation

On `unix`, this crate uses the [`signal_hook_registry`] crate to register a listener for each signal. That listener will then send a message through a Unix socket to the [`Signals`] type, which will receive it and notify the user. Asynchronous notification is done through the [`async-io`] crate.

Note that the internal pipe has a limited capacity. Once it has reached capacity, additional signals will be dropped.

On Windows, a different implementation that only supports `SIGINT` is used. This implementation uses a channel to notify the user.

[`signal_hook_registry`]: https://crates.io/crates/signal-hook-registry
[`async-io`]: https://crates.io/crates/async-io

# Examples

```rust
use async_signal::{Signal, Signals};
use futures_lite::prelude::*;
use signal_hook::low_level;

// Register the signals we want to receive.
let mut signals = Signals::new([
    Signal::Term,
    Signal::Quit,
    Signal::Int,
])?;

// Wait for a signal to be received.
while let Some(signal) = signals.next().await {
    // Print the signal.
    eprintln!("Received signal {:?}", signal);

    // After printing it, do whatever the signal was supposed to do in the first place.
    low_level::emulate_default_handler(signal.unwrap() as i32).unwrap();
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
