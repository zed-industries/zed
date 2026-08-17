<div align="center">

# Kurbo

**A Rust 2D curves library**

[![Linebender Zulip, #kurbo channel](https://img.shields.io/badge/Linebender-%23kurbo-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/260979-kurbo)
[![dependency status](https://deps.rs/repo/github/linebender/kurbo/status.svg)](https://deps.rs/repo/github/linebender/kurbo)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)
[![Build status](https://github.com/linebender/kurbo/workflows/CI/badge.svg)](https://github.com/linebender/kurbo/actions)
[![Crates.io](https://img.shields.io/crates/v/kurbo.svg)](https://crates.io/crates/kurbo)
[![Docs](https://docs.rs/kurbo/badge.svg)](https://docs.rs/kurbo)

</div>

The Kurbo library contains data structures and algorithms for curves and vector paths.
It is probably most appropriate for creative tools, but is general enough it might be useful for other applications.

The name "kurbo" is Esperanto for "curve".

There is a focus on accuracy and good performance in high-accuracy conditions.
Thus, the library might be useful in engineering and science contexts as well, as opposed to visual arts where rough approximations are often sufficient.
Many approximate functions come with an accuracy parameter, and analytical solutions are used where they are practical.
An example is area calculation, which is done using Green's theorem.

The library is still in fairly early development stages.
There are traits intended to be useful for general curves (not just Béziers), but these will probably be reorganized.

## Minimum supported Rust Version (MSRV)

This version of Kurbo has been verified to compile with **Rust 1.85** and later.

Future versions of Kurbo might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of Kurbo's dependencies could have released versions with a higher Rust requirement.
If you encounter a compilation issue due to a dependency and don't want to upgrade your Rust toolchain, then you could downgrade the dependency.

```sh
# Use the problematic dependency's name and version
cargo update -p package_name --precise 0.1.1
```

</details>

## Similar crates

Here we mention a few other curves libraries and touch on some of the decisions made differently here.

* [lyon_geom] has a lot of very good vector algorithms. It's most focused on rendering.

* [flo_curves] has good Bézier primitives, and seems tuned for animation. It's generic on the coordinate type, while we use `f64` for everything.

* [vek] has both 2D and 3D Béziers among other things, and is tuned for game engines.

Some code has been copied from lyon_geom with adaptation, thus the author of lyon_geom, Nicolas Silva, is credited in the [AUTHORS] file.

## More info

To learn more about Bézier curves, [A Primer on Bézier Curves] by Pomax is indispensable.

## Community

[![Linebender Zulip](https://img.shields.io/badge/Linebender-%23kurbo-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/260979-kurbo)

Discussion of Kurbo development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically the [#kurbo channel](https://xi.zulipchat.com/#narrow/channel/260979-kurbo).
All public content can be read without logging in.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.
Please feel free to add your name to the [AUTHORS] file in any substantive pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
[lyon_geom]: https://crates.io/crates/lyon_geom
[flo_curves]: https://crates.io/crates/flo_curves
[vek]: https://crates.io/crates/vek
[A Primer on Bézier Curves]: https://pomax.github.io/bezierinfo/
[AUTHORS]: ./AUTHORS
[CHANGELOG.md]: ./CHANGELOG.md
