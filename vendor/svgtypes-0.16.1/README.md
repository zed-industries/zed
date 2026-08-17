<div align="center">

# SVG Types

**A collection of parsers for [SVG](https://www.w3.org/TR/SVG2/) types.**

[![Linebender Zulip, #resvg channel](https://img.shields.io/badge/Linebender-%23resvg-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/465085-resvg)
[![dependency status](https://deps.rs/repo/github/linebender/svgtypes/status.svg)](https://deps.rs/repo/github/linebender/svgtypes)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)
[![Build status](https://github.com/linebender/svgtypes/workflows/CI/badge.svg)](https://github.com/linebender/svgtypes/actions)
[![Crates.io](https://img.shields.io/crates/v/svgtypes.svg)](https://crates.io/crates/svgtypes)
[![Docs](https://docs.rs/svgtypes/badge.svg)](https://docs.rs/svgtypes)
![](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)

</div>

## Supported SVG types

- [`<color>`](https://www.w3.org/TR/css-color-3/)
- [`<number>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGNumber)
- [`<length>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLength)
- [`<angle>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGAngle)
- [`<viewBox>`](https://www.w3.org/TR/SVG2/coords.html#ViewBoxAttribute)
- [`<path>`](https://www.w3.org/TR/SVG2/paths.html#PathData)
- [`<transform>`](https://www.w3.org/TR/SVG11/types.html#DataTypeTransformList)
- [`transform-origin`](https://drafts.csswg.org/css-transforms/#transform-origin-property)
- [`<list-of-numbers>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGNumberList)
- [`<list-of-lengths>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLengthList)
- [`<list-of-points>`](https://www.w3.org/TR/SVG11/shapes.html#PointsBNF)
- [`<filter-value-list>`](https://www.w3.org/TR/filter-effects-1/#typedef-filter-value-list)
- [`<paint>`](https://www.w3.org/TR/SVG2/painting.html#SpecifyingPaint)
- [`<preserveAspectRatio>`](https://www.w3.org/TR/SVG11/coords.html#PreserveAspectRatioAttribute)
- [`<enable-background>`](https://www.w3.org/TR/SVG11/filters.html#EnableBackgroundProperty)
- [`<IRI>`](https://www.w3.org/TR/SVG11/types.html#DataTypeIRI)
- [`<FuncIRI>`](https://www.w3.org/TR/SVG11/types.html#DataTypeFuncIRI)
- [`paint-order`](https://www.w3.org/TR/SVG2/painting.html#PaintOrder)
- [`<font-family>`](https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/#font-family-prop)
- [`font`](https://www.w3.org/TR/css-fonts-3/#font-prop)

## Features

- Complete support of paths, so data like `M10-20A5.5.3-4 110-.1` will be parsed correctly.
- Implicit path commands will be automatically converted into explicit one.
- Some SVG2 data types support.
- Pretty fast.

## Limitations

- Accepts only [normalized](https://www.w3.org/TR/REC-xml/#AVNormalize) values, e.g. an input text should not contain `&#x20;` or `&data;`.
- All keywords must be lowercase.
  Case-insensitive parsing is supported only for colors (requires allocation for named colors).
- The `<color>` followed by the `<icccolor>` is not supported.
  As the `<icccolor>` itself.
- [System colors](https://www.w3.org/TR/css3-color/#css2-system), like `fill="AppWorkspace"`, are not supported.
  They were deprecated anyway.

## Safety

- The library should not panic. Any panic considered as a critical bug and should be reported.
- The library forbids unsafe code.

## Alternatives

None.

## Minimum supported Rust Version (MSRV)

This version of SVG Types has been verified to compile with **Rust 1.85** and later.

Future versions of SVG Types might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of SVG Types's dependencies could have released versions with a higher Rust requirement.
If you encounter a compilation issue due to a dependency and don't want to upgrade your Rust toolchain, then you could downgrade the dependency.

```sh
# Use the problematic dependency's name and version
cargo update -p package_name --precise 0.1.1
```
</details>

## Community

[![Linebender Zulip, #resvg channel](https://img.shields.io/badge/Linebender-%23resvg-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/465085-resvg)

Discussion of SVG Types development happens in the Linebender Zulip at <https://xi.zulipchat.com/>, specifically the [#resvg channel](https://xi.zulipchat.com/#narrow/channel/465085-resvg).
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
[AUTHORS]: ./AUTHORS
