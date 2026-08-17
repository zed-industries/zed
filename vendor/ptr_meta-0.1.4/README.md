# ptr_meta &emsp; [![Latest Version]][crates.io] [![License]][license path] [![requires: rustc 1.47+]][Rust 1.47]

[Latest Version]: https://img.shields.io/crates/v/ptr_meta.svg
[crates.io]: https://crates.io/crates/ptr_meta
[License]: https://img.shields.io/badge/license-MIT-blue.svg
[license path]: https://github.com/djkoloski/ptr_meta/blob/master/LICENSE
[requires: rustc 1.47+]: https://img.shields.io/badge/rustc-1.47+-lightgray.svg
[Rust 1.47]: https://blog.rust-lang.org/2020/10/08/Rust-1.47.html

# ptr_meta

A radioactive stabilization of the [`ptr_meta` RFC][rfc].

[rfc]: https://rust-lang.github.io/rfcs/2580-ptr-meta.html

## Usage

### Sized types

Sized types already have `Pointee` implemented for them, so most of the time you won't have to worry
about them. However, trying to derive `Pointee` for a struct that may or may not have a DST as its
last field will cause an implementation conflict with the automatic sized implementation.

### `slice`s and `str`s

These core types have implementations built in.

### Structs with a DST as its last field

You can derive `Pointee` for last-field DSTs:

```rust
use ptr_meta::Pointee;

#[derive(Pointee)]
struct Block<H, T> {
    header: H,
    elements: [T],
}
```

### Trait objects

You can generate a `Pointee` for trait objects:

```rust
use ptr_meta::pointee;

// Generates Pointee for dyn Stringy
#[pointee]
trait Stringy {
    fn as_string(&self) -> String;
}
```
