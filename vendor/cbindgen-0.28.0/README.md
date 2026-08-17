# `cbindgen` &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Api Rustdoc]][rustdoc] [![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg?maxAge=3600)](https://github.com/mozilla/cbindgen)

[Build Status]: https://github.com/mozilla/cbindgen/workflows/cbindgen/badge.svg
[actions]: https://github.com/mozilla/cbindgen/actions
[Latest Version]: https://img.shields.io/crates/v/cbindgen.svg
[crates.io]: https://crates.io/crates/cbindgen
[Api Rustdoc]: https://img.shields.io/badge/api-rustdoc-blue.svg
[rustdoc]: https://docs.rs/cbindgen

[Read the full user docs here!](https://github.com/mozilla/cbindgen/blob/master/docs.md)

cbindgen creates C/C++11 headers for Rust libraries which expose a public C API.

While you could do this by hand, it's not a particularly good use of your time.
It's also much more likely to be error-prone than machine-generated headers that
are based on your actual Rust code. The cbindgen developers have also worked
closely with the developers of Rust to ensure that the headers we generate
reflect actual guarantees about Rust's type layout and ABI.

C++ headers are nice because we can use operator overloads, constructors, enum
classes, and templates to make the API more ergonomic and Rust-like. C headers
are nice because you can be more confident that whoever you're interoperating
with can handle them. With cbindgen *you don't need to choose*! You can just
tell it to emit both from the same Rust library.

There are two ways to use cbindgen: as a standalone program, or as a library
(presumably in your build.rs). There isn't really much practical difference,
because cbindgen is a simple rust library with no interesting dependencies.

Using it as a program means people building your software will need it
installed. Using it in your library means people may have to build cbindgen more
frequently (e.g. every time they update their rust compiler).

It's worth noting that the development of cbindgen has been largely adhoc, as
features have been added to support the usecases of the maintainers. This means
cbindgen may randomly fail to support some particular situation simply because
no one has put in the effort to handle it yet. [Please file an issue if you run
into such a situation](https://github.com/mozilla/cbindgen/issues/new). Although
since we all have other jobs, you might need to do the implementation work too
:)

# Quick Start

To install cbindgen, you just need to run

```text
cargo install --force cbindgen
```

(--force just makes it update to the latest cbindgen if it's already installed)

Or with Homebrew, run

```text
brew install cbindgen
```

To use cbindgen you need two things:

* A configuration (cbindgen.toml, which can be empty to start)
* A Rust crate with a public C API

Then all you need to do is run it:

```text
cbindgen --config cbindgen.toml --crate my_rust_library --output my_header.h
```

This produces a header file for C++.  For C, add the `--lang c` switch.

See `cbindgen --help` for more options.

[Read the full user docs here!](docs.md)

[Get a template cbindgen.toml here.](template.toml)

# Examples

We don't currently have a nice tailored example application, but [the
tests](tests/rust/) contain plenty of interesting examples of our features.

You may also find it interesting to browse the projects that are using cbindgen
in production:

* [milksnake](https://github.com/getsentry/milksnake)
* [webrender](https://searchfox.org/mozilla-central/source/gfx/webrender_bindings) ([generated header](https://searchfox.org/mozilla-central/source/__GENERATED__/gfx/webrender_bindings/webrender_ffi_generated.h))
* [stylo](https://searchfox.org/mozilla-central/source/layout/style) ([generated header](https://searchfox.org/mozilla-central/source/__GENERATED__/layout/style/ServoStyleConsts.h))
* [maturin](https://github.com/PyO3/maturin)
* [tquic](https://github.com/Tencent/tquic) ([generated header](https://github.com/Tencent/tquic/blob/develop/include/tquic.h))

If you're using `cbindgen` and would like to be added to this list, please open
a pull request!

# Releases

cbindgen doesn't have a fixed release calendar, please file an issue requesting
a release if there's something fixed in trunk that you need released. Ping
`@emilio` for increased effect.
