# synchronoise

fun synchronization primitives for your fun synchronization needs [![Build Status](https://travis-ci.org/QuietMisdreavus/synchronoise.svg?branch=master)](https://travis-ci.org/QuietMisdreavus/synchronoise) [![Build status](https://ci.appveyor.com/api/projects/status/222hqcyigcdihfw9/branch/master?svg=true)](https://ci.appveyor.com/project/QuietMisdreavus/synchronoise/branch/master)

[Documentation](https://docs.rs/synchronoise) | ([Manually-generated docs for master][doc-dev])

[doc-dev]: https://tonberry.quietmisdreavus.net/synchronoise-dev/synchronoise/

This is a collection of synchronization facilities that aren't part of the standard library that I
wanted to make sure were available for the Rust community.

This crate contains the following synchronization primitives:

* `CountdownEvent`, a port of `System.Threading.CountdownEvent` from .NET (also called
  `CountDownLatch` in Java).
* `SignalEvent`, a port of `System.Threading.EventWaitHandle` (and its derived classes,
  `AutoResetEvent` and `ManualResetEvent`) from .NET.
* `WriterReaderPhaser`, a port of `WriterReaderPhaser` from HdrHistogram.

To add this crate to your project, add the following line to your Cargo.toml:

```toml
[dependencies]
synchronoise = "0.4.0"
```

...and the following to your crate root:

```rust
extern crate synchronoise;
```

# License

synchronoise is licensed under either the MIT License or the Apache License version 2.0, at your
option. See the files `LICENSE-MIT` and `LICENSE-APACHE` for details.

(synchronoise is named after [a move in Pokemon][synch], by the way)

[synch]: http://bulbapedia.bulbagarden.net/wiki/Synchronoise_(move)

<!-- vim: set tw=100 expandtab: -->
