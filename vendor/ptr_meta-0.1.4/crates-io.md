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
