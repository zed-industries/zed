# web-time

[![Crates.io Version](https://img.shields.io/crates/v/web-time.svg)](https://crates.io/crates/web-time)
[![Live Build Status](https://img.shields.io/github/checks-status/daxpedda/web-time/main?label=CI)](https://github.com/daxpedda/web-time/actions?query=branch%3Amain)
[![Docs.rs Documentation](https://img.shields.io/docsrs/web-time?label=docs.rs)](https://docs.rs/crate/web-time)
[![Main Documentation](https://img.shields.io/github/actions/workflow/status/daxpedda/web-time/documentation.yml?branch=main&label=main%20docs
)](https://daxpedda.github.io/web-time/web_time/index.html)

## Description

Complete drop-in replacement for [`std::time`] that works in browsers.

Currently [`Instant::now()`] and [`SystemTime::now()`] will simply panic
when using the `wasm32-unknown-unknown` target. This implementation uses
[`Performance.now()`] for [`Instant`] and [`Date.now()`] for [`SystemTime`]
to offer a drop-in replacement that works in browsers.

At the same time the library will simply re-export [`std::time`] when not
using the `wasm32-unknown-unknown` target and will not pull in any
dependencies.

Additionally, if compiled with `target-feature = "atomics"` it will
synchronize the timestamps to account for different context's, like web
workers. See [`Performance.timeOrigin`] for more information.

Using `-Ctarget-feature=+nontrapping-fptoint` will improve the performance
of [`Instant::now()`] and [`SystemTime::now()`], but the vast majority of
the time is still spent going through JS.

## Target

This library specifically targets browsers, that support
[`Performance.now()`], with the `wasm32-unknown-unknown` target. Emscripten
is not supported. WASI doesn't require support as it has it's own native API
to deal with [`std::time`].

Furthermore it depends on [`wasm-bindgen`], which is required. This library
will continue to depend on it until a viable alternative presents itself, in
which case multiple ecosystems could be supported.

## Note

### Ticking during sleep

Currently a known bug is affecting browsers on operating system other then
Windows. This bug prevents [`Instant`] from continuing to tick when the
context is asleep. This doesn't necessarily conflict with Rusts requirements
of [`Instant`], but might still be unexpected.

See [the MDN documentation on this](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now#ticking_during_sleep) for more information.

### Context support

The implementation of [`Instant::now()`] relies on the availability of the
[`Performance` object], a lack thereof will cause a panic. This can happen
if called from a [worklet].

## Usage

You can simply import the types you need:
```rust
use web_time::{Instant, SystemTime};

let now = Instant::now();
let time = SystemTime::now();
```

## Features

### `serde`

Implements [`serde::Deserialize`] and [`serde::Serialize`] for
[`SystemTime`].

## MSRV

As this library heavily relies on [`wasm-bindgen`] the MSRV depends on it.
At the point of time this was written the MSRV is 1.60.

## Alternatives

[instant](https://crates.io/crates/instant) [![Crates.io](https://img.shields.io/crates/v/instant.svg)](https://crates.io/crates/instant) is a popular alternative! However the API it implements doesn't match [`std::time`] exactly.

## Changelog

See the [CHANGELOG] file for details.

## Contributing

See the [CONTRIBUTING] file for details.

## Attribution

Inspiration was taken from the [instant](https://github.com/sebcrozet/instant/tree/v0.1.12) project.

Additional insight was taken from the [time](https://github.com/time-rs/time/tree/v0.3.20) project.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

### Copyright

A majority of the code and documentation was taken from [`std::time`]. For
license information see [#License](https://github.com/rust-lang/rust/tree/1.68.1#license).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[CHANGELOG]: https://github.com/daxpedda/web-time/blob/v1.1.0/CHANGELOG.md
[CONTRIBUTING]: https://github.com/daxpedda/web-time/blob/v1.1.0/CONTRIBUTING.md
[LICENSE-MIT]: https://github.com/daxpedda/web-time/blob/v1.1.0/LICENSE-MIT
[LICENSE-APACHE]: https://github.com/daxpedda/web-time/blob/v1.1.0/LICENSE-APACHE
[worklet]: https://developer.mozilla.org/en-US/docs/Web/API/Worklet
[`Date.now()`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now
[`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`Instant::now()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[`SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`SystemTime::now()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now
[`std::time`]: https://doc.rust-lang.org/stable/std/time/
[`performance.now()`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
[`Performance.timeOrigin`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin
[`Performance` object]: https://developer.mozilla.org/en-US/docs/Web/API/performance_property
[`serde::Deserialize`]: https://docs.rs/serde/1/serde/trait.Deserialize.html
[`serde::Serialize`]: https://docs.rs/serde/1/serde/trait.Serialize.html
[`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
