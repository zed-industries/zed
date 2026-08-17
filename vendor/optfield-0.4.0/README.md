# optfield

[![crates.io](https://img.shields.io/crates/v/optfield.svg)][crate]
[![Released API docs](https://docs.rs/optfield/badge.svg)][documentation]
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.61%2B-informational)][rustc]
[![CI](https://img.shields.io/github/actions/workflow/status/roignpar/optfield/ci.yml?branch=main)][ci]

`optfield` is a macro that, given a struct, generates another struct with
the same fields, but wrapped in `Option<T>`.

__Minimum rustc version: `1.61.0`__

### Install
```
cargo add optfield
```

### Use
`optfield` takes the opt struct name as its first argument:
```rust
use optfield::optfield;

#[optfield(Opt)]
struct MyStruct<T> {
    text: String,
    number: i32,
    generic: T,
}
```
This will generate another struct that looks like:
```rust
struct Opt<T> {
    text: Option<String>,
    number: Option<i32>,
    generic: Option<T>,
}
```

`optfield` supports defining visibility, documentation, attributes and merge
methods. For more details and examples check its [documentation].

### License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT](LICENSE-MIT) at your option.

[crate]: https://crates.io/crates/optfield
[documentation]: https://docs.rs/optfield
[rustc]: https://blog.rust-lang.org/2021/10/21/Rust-1.61.0.html
[ci]: https://github.com/roignpar/optfield/actions?query=workflow%3ACI
