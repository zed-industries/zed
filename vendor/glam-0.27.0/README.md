# glam

[![Build Status]][github-ci] [![Coverage Status]][coveralls.io]
[![Latest Version]][crates.io] [![docs]][docs.rs]
[![Minimum Supported Rust Version]][Rust 1.68.2]

A simple and fast 3D math library for games and graphics.

## Development status

`glam` is in beta stage. Base functionality has been implemented and the look
and feel of the API has solidified.

## Features

* `f32` types
  * vectors: `Vec2`, `Vec3`, `Vec3A` and `Vec4`
  * square matrices: `Mat2`, `Mat3`, `Mat3A` and `Mat4`
  * a quaternion type: `Quat`
  * affine transformation types: `Affine2` and `Affine3A`
* `f64` types
  * vectors: `DVec2`, `DVec3` and `DVec4`
  * square matrices: `DMat2`, `DMat3` and `DMat4`
  * a quaternion type: `DQuat`
  * affine transformation types: `DAffine2` and `DAffine3`
* `i16` types
  * vectors: `I16Vec2`, `I16Vec3` and `I16Vec4`
* `u16` types
  * vectors: `U16Vec2`, `U16Vec3` and `U16Vec4`
* `i32` types
  * vectors: `IVec2`, `IVec3` and `IVec4`
* `u32` types
  * vectors: `UVec2`, `UVec3` and `UVec4`
* `i64` types
  * vectors: `I64Vec2`, `I64Vec3` and `I64Vec4`
* `u64` types
  * vectors: `U64Vec2`, `U64Vec3` and `U64Vec4`
* `bool` types
  * vectors: `BVec2`, `BVec3` and `BVec4`

### SIMD

The `Vec3A`, `Vec4`, `Quat`, `Mat2`, `Mat3A`, `Mat4`, `Affine2` and `Affine3A`
types use 128-bit wide SIMD vector types for storage on `x86`, `x86_64` and
`wasm32` architectures.  As a result, these types are all 16 byte aligned and
depending on the size of the type or the type's members, they may contain
internal padding.  This results in some wasted space in the cases of `Vec3A`,
`Mat3A`, `Affine2` and `Affine3A`.  However, the use of SIMD generally results
in better performance than scalar math.

`glam` outperforms similar Rust libraries for common operations as tested by the
[`mathbench`][mathbench] project.

[mathbench]: https://github.com/bitshifter/mathbench-rs

### Enabling SIMD

SIMD is supported on `x86`, `x86_64` and `wasm32` targets.

* `SSE2` is enabled by default on `x86_64` targets.
* To enable `SSE2` on `x86` targets add `-C target-feature=+sse2` to
  `RUSTCFLAGS`.
* To enable `simd128` on `wasm32` targets add `-C target-feature=+simd128` to
  `RUSTFLAGS`.
* Experimental [portable simd] support can be enabled with the `core-simd`
  feature. This requires the nightly compiler as it is still unstable in Rust.

Note that SIMD on `wasm32` passes tests but has not been benchmarked,
performance may or may not be better than scalar math.

[portable simd]: https://doc.rust-lang.org/core/simd/index.html

### `no_std` support

`no_std` support can be enabled by compiling with `--no-default-features` to
disable `std` support and `--features libm` for math functions that are only
defined in `std`. For example:

```toml
[dependencies]
glam = { version = "0.27", default-features = false, features = ["libm"] }
```

To support both `std` and `no_std` builds in project, you can use the following
in your `Cargo.toml`:

```toml
[features]
default = ["std"]

std = ["glam/std"]
libm = ["glam/libm"]

[dependencies]
glam = { version = "0.27", default-features = false }
```

### Optional features

* [`approx`] - traits and macros for approximate float comparisons
* [`bytemuck`] - for casting into slices of bytes
* [`libm`] - uses `libm` math functions instead of `std`, required to compile
  with `no_std`
* [`mint`] - for interoperating with other 3D math libraries
* [`rand`] - implementations of `Distribution` trait for all `glam` types.
* [`serde`] - implementations of `Serialize` and `Deserialize` for all `glam`
  types. Note that serialization should work between builds of `glam` with and
  without SIMD enabled
* [`rkyv`] - implementations of `Archive`, `Serialize` and `Deserialize` for
  all `glam` types. Note that serialization is not interoperable with and
  without the `scalar-math` feature. It should work between all other builds of
  `glam`.  Endian conversion is currently not supported
* [`bytecheck`] - to perform archive validation when using the `rkyv` feature

[`approx`]: https://docs.rs/approx
[`bytemuck`]: https://docs.rs/bytemuck
[`libm`]: https://github.com/rust-lang/libm
[`mint`]: https://github.com/kvark/mint
[`rand`]: https://github.com/rust-random/rand
[`serde`]: https://serde.rs
[`rkyv`]: https://github.com/rkyv/rkyv
[`bytecheck`]: https://github.com/rkyv/bytecheck

### Feature gates

* `scalar-math` - compiles with SIMD support disabled
* `debug-glam-assert` - adds assertions in debug builds which check the validity
  of parameters passed to `glam` to help catch runtime errors
* `glam-assert` - adds validation assertions to all builds
* `cuda` - forces `glam` types to match expected [cuda alignment]
* `fast-math` - By default, glam attempts to provide bit-for-bit identical
  results on all platforms. Using this feature will enable platform specific
  optimizations that may not be identical to other platforms. **Intermediate
  libraries should not use this feature and defer the decision to the final
  binary build**.
* `core-simd` - enables SIMD support via the [portable simd] module. This is an
  unstable feature which requires a nightly Rust toolchain and `std` support.

[cuda alignment]: https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#built-in-vector-types

### Minimum Supported Rust Version (MSRV)

The minimum supported version of Rust for `glam` is `1.68.2`.

## Conventions

### Column vectors

`glam` interprets vectors as column matrices (also known as "column vectors")
meaning when transforming a vector with a matrix the matrix goes on the left,
e.g. `v' = Mv`.  DirectX uses row vectors, OpenGL uses column vectors. There
are pros and cons to both.

### Column-major order

Matrices are stored in column major format. Each column vector is stored in
contiguous memory.

### Co-ordinate system

`glam` is co-ordinate system agnostic and intends to support both right-handed
and left-handed conventions.

## Design Philosophy

The design of this library is guided by a desire for simplicity and good
performance.

* No generics and minimal traits in the public API for simplicity of usage
* All dependencies are optional (e.g. `mint`, `rand` and `serde`)
* Follows the [Rust API Guidelines] where possible
* Aiming for 100% test [coverage]
* Common functionality is benchmarked using [Criterion.rs]

[Rust API Guidelines]: https://rust-lang-nursery.github.io/api-guidelines/
[coverage]: coveralls.io
[Criterion.rs]: https://bheisler.github.io/criterion.rs/book/index.html

## Architecture

See [ARCHITECTURE.md] for details on `glam`'s internals.

[ARCHITECTURE.md]: ARCHITECTURE.md

## Inspirations

There were many inspirations for the interface and internals of glam from the
Rust and C++ worlds. In particular:

* [How to write a maths library in 2016] inspired the initial `Vec3A`
  implementation
* [Realtime Math] - header only C++11 with SSE and NEON SIMD intrinsic support
* [DirectXMath] - header only SIMD C++ linear algebra library for use in games
  and graphics apps
* `glam` is a play on the name of the popular C++ library [GLM]

[How to write a maths library in 2016]: http://www.codersnotes.com/notes/maths-lib-2016/
[Realtime Math]: https://github.com/nfrechette/rtm
[DirectXMath]: https://docs.microsoft.com/en-us/windows/desktop/dxmath/directxmath-portal
[GLM]: https://glm.g-truc.net

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's [Code of Conduct].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

If you are interested in contributing or have a request or suggestion
[start a discussion] on GitHub. See [CONTRIBUTING.md] for more information for
contributors.

Most code in `glam` is generated, see the [codegen README] for details.

Thank you to all of the `glam` [contributors]!

[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[start a discussion]: https://github.com/bitshifter/glam-rs/discussions
[CONTRIBUTING.md]: CONTRIBUTING.md
[codegen README]: codegen/README.md
[contributors]: https://github.com/bitshifter/glam-rs/graphs/contributors

## Support

The [Game Development in Rust Discord] and [Bevy Engine Discord] servers are
not official support channels but can be good places to ask for help with
`glam`.

[Game Development in Rust Discord]: https://discord.gg/yNtPTb2
[Bevy Engine Discord]: https://discord.gg/gMUk5Ph

## Attribution

`glam` contains code ported from the following C++ libraries:

* [DirectXMath] - MIT License - Copyright (c) 2011-2020 Microsoft Corp
* [Realtime Math] - MIT License - Copyright (c) 2018 Nicholas Frechette
* [GLM] - MIT License - Copyright (c) 2005 - G-Truc Creation

See [ATTRIBUTION.md] for details.

[ATTRIBUTION.md]: ATTRIBUTION.md

[Build Status]: https://github.com/bitshifter/glam-rs/actions/workflows/ci.yml/badge.svg
[github-ci]: https://github.com/bitshifter/glam-rs/actions/workflows/ci.yml
[Coverage Status]: https://coveralls.io/repos/github/bitshifter/glam-rs/badge.svg?branch=main
[coveralls.io]: https://coveralls.io/github/bitshifter/glam-rs?branch=main
[Latest Version]: https://img.shields.io/crates/v/glam.svg
[crates.io]: https://crates.io/crates/glam/
[docs]: https://docs.rs/glam/badge.svg
[docs.rs]: https://docs.rs/glam/
[Minimum Supported Rust Version]: https://img.shields.io/badge/Rust-1.68.2-blue?color=fc8d62&logo=rust
[Rust 1.68.2]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1682-2023-03-28
