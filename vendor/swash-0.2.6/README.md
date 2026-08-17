# swash

Swash is a pure Rust, cross-platform crate that provides font introspection,
complex text shaping and glyph rendering.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Apache 2.0 or MIT license.][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/swash.svg
[crates-url]: https://crates.io/crates/swash
[docs-badge]: https://docs.rs/swash/badge.svg
[docs-url]: https://docs.rs/swash
[license-badge]: https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg
[license-url]: #license

### Goals

This crate aims to lay the foundation for a cross-platform, high performance set
of components for beautiful typography. In particular, this library focuses on
fonts and operations that are directly applicable to them. For the most part, the
desire is to be unopinionated with respect to resource management, higher level
layout and lower level rendering.

### Non goals

Due to the intention of being generally useful and easy to integrate, the following
areas of related interest are specifically avoided:

- Text layout. This is highly application specific and the requirements for both
    features and performance differ greatly among web browsers, word processors,
    text editors, game engines, etc. There is a sibling crate in development that
    does provide general purpose text layout based on this library.

- Composition. Like layout, this is also application specific in addition to being
    hardware dependent. Glyph caching, geometry batching and rendering all belong
    here and should integrate well with the application and the hardware environment.

### General features

- Simple borrowed font representation that imposes no requirements on resource
    management leading to...
- Thread friendly architecture. Acceleration structures are completely separate from
    font data and can be retained per thread, thrown away and rematerialized at any
    time
- Zero transient heap allocations. All scratch buffers and caches are maintained by
    contexts. Resources belonging to evicted cache entries are immediately reused

### Introspection

- Enumerating font collections (ttc/otc)
- Localized strings including names and other metadata
- Font variation axes and named instances
- Comprehensive font and per-glyph metrics (with synthesized vertical metrics if not provided)
- Primary attributes (stretch, weight, style) with support for synthesis suggestions
    based on available variations and scaler transforms (faux bold and oblique)
- Color palettes
- Embedded color and alpha bitmap strikes
- Character to nominal glyph identifier mapping with support for enumerating all pairs
- Writing systems: provides a list of all supported script/language pairs
    and their associated typographic features
- All introspection is zero allocation and zero copy

### Complex text shaping

- Full support for OpenType advanced typography (GSUB/GPOS)
- Partial support for Apple advanced typography: glyph metamorphosis (morx) is fully
    supported while the current extended kerning support (kerx) covers most common
    cases (kerning and mark positioning)
- Full support for variable fonts including positioning and feature substitutions
- Implementation of the Universal Shaping Engine for complex scripts such as Devanagari, Malayalam, etc.
- Arabic joining including Urdu style climbing runs
- Basic shaping support: ligatures, marks, kerning, etc.
- Enable/disable individual features with argument support for activating alternates
    such as _swashes_...
- Pre-shaping cluster parsing with an iterative mapping technique (including normalization) allowing for
    sophisticated font fallback mechanisms without the expense of heuristically shaping runs
- Shaper output is structured by cluster including original source ranges and provides simple
    identification of ligatures and complex multi-glyph clusters
- Pass-through per character user data (a single u32) for accurate association of style
    properties with glyphs
- Pass-through per cluster information for retaining text analysis results such as word
    and line boundaries, whitespace identification and emoji presentation modes

### Scaling

- Scalable outlines with full variation support (TrueType and Postscript)
- Asymmetric vertical hinting (TrueType and Postscript)
- Horizontal subpixel rendering and fractional positioning
- Full emoji support for Apple (sbix), Google (CBLC/CBDT) and Microsoft (COLR/CPAL)
    formats
- Path effects (stroking and dashing)
- Transforms including synthetic emboldening and affine transformations
- Customizable glyph source prioritization (best fit color bitmap -> exact size alpha bitmap -> outline)

### Text analysis

- Unicode character properties related to layout and shaping
- Character composition and decomposition (canonical and compatible)
- Complex, script aware cluster segmentation
- Single pass, iterator based analysis determines word and line boundaries
    and detects whether bidi resolution is necessary

### License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
