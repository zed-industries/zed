## svgtypes
![Build Status](https://github.com/RazrFalcon/svgtypes/workflows/svgtypes/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/svgtypes.svg)](https://crates.io/crates/svgtypes)
[![Documentation](https://docs.rs/svgtypes/badge.svg)](https://docs.rs/svgtypes)
[![Rust 1.65+](https://img.shields.io/badge/rust-1.65+-orange.svg)](https://www.rust-lang.org)
![](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)

*svgtypes* is a collection of parsers for [SVG](https://www.w3.org/TR/SVG2/) types.

### Supported SVG types

- [`<color>`](https://www.w3.org/TR/css-color-3/)
- [`<number>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGNumber)
- [`<length>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGLength)
- [`<angle>`](https://www.w3.org/TR/SVG2/types.html#InterfaceSVGAngle)
- [`<viewBox>`](https://www.w3.org/TR/SVG2/coords.html#ViewBoxAttribute)
- [`<path>`](https://www.w3.org/TR/SVG2/paths.html#PathData)
- [`<transform>`](https://www.w3.org/TR/SVG11/types.html#DataTypeTransformList)
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

### Features

- Complete support of paths, so data like `M10-20A5.5.3-4 110-.1` will be parsed correctly.
- Implicit path commands will be automatically converted into explicit one.
- Some SVG2 data types support.
- Pretty fast.

### Limitations

- Accepts only [normalized](https://www.w3.org/TR/REC-xml/#AVNormalize) values,
  e.g. an input text should not contain `&#x20;` or `&data;`.
- All keywords must be lowercase.
  Case-insensitive parsing is supported only for colors (requires allocation for named colors).
- The `<color>` followed by the `<icccolor>` is not supported. As the `<icccolor>` itself.
- [System colors](https://www.w3.org/TR/css3-color/#css2-system), like `fill="AppWorkspace"`,
  are not supported. They were deprecated anyway.

### Safety

- The library should not panic. Any panic considered as a critical bug and should be reported.
- The library forbids unsafe code.

### Alternatives

None.

### License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
