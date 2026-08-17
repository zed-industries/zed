# stacker

[![Build Status](https://github.com/rust-lang/stacker/actions/workflows/test.yml/badge.svg)](https://github.com/rust-lang/stacker/actions)

[Documentation](https://docs.rs/stacker)

A stack-growth library for Rust. Enables annotating fixed points in programs
where the stack may want to grow larger. Spills over to the heap if the stack
has hit its limit.

This library is intended on helping implement recursive algorithms.

```toml
# Cargo.toml
[dependencies]
stacker = "0.1"
```

## Platform Support

This library currently uses psm for its cross platform capabilities, with a
notable exception of Windows, which uses an implementation based on Fibers. See
the README for psm for the support table.

On all unsupported platforms this library is a noop. It should compile and run,
but it won't actually grow the stack and code will continue to hit the guard
pages typically in place.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/license/mit)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
