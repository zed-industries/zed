# async-attributes
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Experimental language-level polyfills for Async Rust.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples

```rust
#[async_attributes::main]
async fn main() {
    println!("Hello, world!");
}
```

## About

Async Rust is a work in progress. The language has enabled us to do some
fantastic things, but not everything is figured out yet. This crate exists
to polyfill language-level support for async idioms before they can be part
of the language.

A great example of this is `async fn main`, which we first introduced as
part of the [`runtime`](https://docs.rs/runtime/0.3.0-alpha.7/runtime/) crate.
Its premise is that if `async fn` is required for every `await` call, it
makes sense to apply that even to `fn main`. Unfortunately this would
require compiler support to enable, so we've provided an experimental
polyfill for it in the mean time.

## Why isn't this crate part of async-std?

We want to make sure `async-std`'s surface area is stable, and only includes
things that would make sense to be part of "an async version of std".
Language level support is really important, but _not_ part of the standard
library.

This has some distinct benefits: in particular it allows us to
version both crates at a different pace. And as features are added to the
language (or we decide they weren't a great idea after all), we can
incrementally shrink the surface area of this crate.

The other big benefit is that it allows libraries to depend on `async-std`
without needing to pull in the rather heavy `syn`, `quote`, and
`proc-macro2` crates. This should help keep compilation times snappy for
everyone.

## Installation

```sh
$ cargo add async-attributes
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- https://docs.rs/runtime-attributes - our original experiments with
  `async fn main`.
- https://docs.rs/async-trait - async trait support by the fantastic
  [David Tolnay](https://github.com/dtolnay/).
- https://docs.rs/futures-async-stream - for iterating and defining streams by
  the skilled [Taiki Endo](https://github.com/taiki-e/).

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/async-attributes.svg?style=flat-square
[2]: https://crates.io/crates/async-attributes
[3]: https://travis-ci.com/async-rs/async-attributes.svg?branch=master
[4]: https://travis-ci.com/async-rs/async-attributes
[5]: https://img.shields.io/crates/d/async-attributes.svg?style=flat-square
[6]: https://crates.io/crates/async-attributes
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/async-attributes

[releases]: https://github.com/async-rs/async-attributes/releases
[contributing]: https://github.com/async-rs/async-attributes/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/async-rs/async-attributes/labels/good%20first%20issue
[help-wanted]: https://github.com/async-rs/async-attributes/labels/help%20wanted
