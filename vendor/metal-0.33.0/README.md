# metal-rs

> [!WARNING]
> 
> Use of this crate is deprecated as the [`objc`] ecosystem of mac system bindings are unmaintained. 
> For new development, please use [`objc2`] and [`objc2-metal`] instead. We will continue to merge basic
> PRs and keep things maintained, at least as long as it takes to migrate [`wgpu`] to the `objc2` ecosystem [PR 5641].

[`objc`]: https://crates.io/crates/objc
[`objc2`]: https://crates.io/crates/objc2
[`objc2-metal`]: https://crates.io/crates/objc2-metal
[`wgpu`]: https://crates.io/crates/wgpu
[PR 5641]: https://github.com/gfx-rs/wgpu/pull/5641

-----


[![Actions Status](https://github.com/gfx-rs/metal-rs/workflows/ci/badge.svg)](https://github.com/gfx-rs/metal-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/metal.svg?label=metal)](https://crates.io/crates/metal)

<p align="center">
  <img width="150" height="150" src="./assets/metal.svg">
</p>

<p align="center">Unsafe Rust bindings for the Metal 3D Graphics API.</p>

## Examples

The [examples](/examples) directory highlights different ways of using the Metal graphics API for rendering
and computation.

Examples can be run using commands such as:

```
# Replace `window` with the name of the example that you would like to run
cargo run --example window
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
