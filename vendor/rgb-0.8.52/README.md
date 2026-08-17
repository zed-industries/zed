# `struct RGB` for [Rust](https://www.rust-lang.org)  [![crate](https://img.shields.io/crates/v/rgb.svg)](https://lib.rs/crates/rgb)

Operating on pixels as weakly-typed vectors of `u8` is error-prone and inconvenient. It's better to use vectors of pixel structs. However, Rust is so strongly typed that *your* RGB pixel struct is not compatible with *my* RGB pixel struct. So let's all use mine :P

[<img src="https://imgs.xkcd.com/comics/standards_2x.png" alt="xkcd: …there are 15 competing standards" width="500">](https://xkcd.com/927/)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rgb = "0.8.51"
```

## Usage

### `RGB` and `RGBA` structs

The structs implement common Rust traits and a few convenience functions, e.g. `map` that repeats an operation on every subpixel:

```rust
use rgb::*; // Laziest way to use traits which add extra methods to the structs

let px = RGB {
    r:255_u8,
    g:0,
    b:255,
};
let inverted = px.map(|ch| 255 - ch);

println!("{}", inverted); // Display: rgb(0,255,0)
assert_eq!(RGB8::new(0, 255, 0), inverted);
```

### Byte slices to pixel slices

For interoperability with functions operating on generic arrays of bytes there are functions for safe casting to and from pixel slices.

```rust
let raw = vec![0u8; width*height*3];
let pixels: &[RGB8] = raw.as_rgb(); /// Safe casts without copying
let raw_again = pixels.as_bytes();
```

Note: if you get an error about "no method named `as_bytes` found", add `use rgb::ComponentBytes`. If you're using a custom component type (`RGB<CustomType>`), implement `rgb::Pod` (plain old data) and `rgb::Zeroable` trait for the component (these traits are from [`bytemuck`](https://lib.rs/bytemuck) crate).

----

## About colorspaces

*Correct* color management is a complex problem, and this crate aims to be the lowest common denominator, so it's intentionally agnostic about it.

However, this library supports any subpixel type for `RGB<T>`, and `RGBA<RGBType, AlphaType>`, so you can use them with a newtype, e.g.:

```rust
struct LinearLight(u16);
type LinearRGB = RGB<LinearLight>;
```


### `BGRA`, `ARGB`, `Gray`, etc.

There are other color types in `rgb::alt::*`. There's also an optional `serde` feature that makes all types (de)serializable.

## Roadmap to 1.0

The plan is to provide easy migration to v1.0. There will be a transitional v0.9 version released that will be mostly backwards-compatible with 0.8, and forwards-compatible with 1.0.

Planned changes:

 * Types will be renamed to follow Rust's naming convention: `RGBA` → `Rgba`. The old names will continue to work as hidden aliases.
 * The `Gray` and `GrayAlpha` types will change from tuple structs with `.0` to structs with named fields `.v` (value) and `.a` (alpha). Through a `Deref` trick both field names will work, but `.0` is going to be deprecated.
 * `bytemuck::Pod` (conversions from/to raw bytes) will require color and alpha components to be the same type (i.e. it will work with `Rgba<u8>`, but not `Rgba<Newtype, DifferentType>`). Currently it's unsound if the alpha has a different size than color components.
 * Many inherent methods will be moved to a new `Pixel` trait.

## Migration from 0.8 to 0.9

1. Update to the latest version of 0.8, and fix all deprecation warnings.

   - rename `.alpha()` to `.with_alpha()`
   - rename `.map_c()` to `.map_colors()`

2. Change field access on `GrayAlpha` from `.0` and `.1` to `.v` and `.a` where possible.
3. Use the `bytemuck` crate for conversions from/to bytes instead of `ComponentBytes` trait. Disable the `as-bytes` feature if possible.
4. Don't enable `gbr` and `argb` features. All pixel types are enabled by default.
5. In generic code operating on pixels, add `Copy + 'static` bounds to the pixel types and/or their components.
6. Test your code with `rgb = { version = "0.8.46", features = ["unstable-experimental"] }`, which enables some of the future breaking changes on the older version. This feature flag is only for testing, and will be changed/removed in the future.
7. Avoid wildcard imports from `rgb::alt::*`, and avoid using `GRAY8`-`GRAYA16` type aliases.
