# waker-fn

[![Build](https://github.com/smol-rs/waker-fn/workflows/Build%20and%20test/badge.svg)](
https://github.com/smol-rs/waker-fn/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](
https://github.com/smol-rs/waker-fn)
[![Cargo](https://img.shields.io/crates/v/waker-fn.svg)](
https://crates.io/crates/waker-fn)
[![Documentation](https://docs.rs/waker-fn/badge.svg)](
https://docs.rs/waker-fn)

Convert closures into wakers.

A [`Waker`] is just a fancy callback. This crate converts regular closures into wakers.

[`Waker`]: https://doc.rust-lang.org/std/task/struct.Waker.html

## Examples

```rust
use waker_fn::waker_fn;

let waker = waker_fn(|| println!("woken"));

waker.wake_by_ref(); // Prints "woken".
waker.wake();        // Prints "woken".
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
