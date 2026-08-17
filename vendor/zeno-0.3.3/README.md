# zeno

Zeno is a pure Rust crate that provides a high performance, low level 2D 
rasterization library with support for rendering paths of various styles 
into alpha or subpixel masks.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Apache 2.0 or MIT license.][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/zeno.svg
[crates-url]: https://crates.io/crates/zeno
[docs-badge]: https://docs.rs/zeno/badge.svg
[docs-url]: https://docs.rs/zeno
[license-badge]: https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg
[license-url]: #license

## Features

- 256x anti-aliased rasterization (8-bit alpha or 32-bit RGBA subpixel alpha)
- Pixel perfect hit testing with customizable coverage threshold
- Non-zero and even-odd fills
- Stroking with the standard set of joins and caps
    (separate start and end caps are possible)
- Numerically stable dashing for smooth dash offset animation
- Vertex traversal for marker placement
- Stepped distance traversal for animation or text-on-path support
- Abstract representation of path data that imposes no policy on storage

## Usage

Rendering a dashed stroke of a triangle:

```rust
use zeno::{Cap, Join, Mask, PathData, Stroke};

// Buffer to store the mask
let mut mask = [0u8; 64 * 64];

/// Create a mask builder with some path data
Mask::new("M 8,56 32,8 56,56 Z")
    .style(
        // Stroke style with a width of 4
        Stroke::new(4.0)
            // Round line joins
            .join(Join::Round)
            // And round line caps
            .cap(Cap::Round)
            // Dash pattern followed by a dash offset
            .dash(&[10.0, 12.0, 0.0], 0.0),
    )
    // Set the target dimensions
    .size(64, 64)
    // Render into the target buffer
    .render_into(&mut mask, None);
```

Resulting in the following mask: 

![Dashed Triangle](https://muddl.com/zeno/tri_dash.png)

For detail on additional features and more advanced usage,
see the full API [documentation](https://docs.rs/zeno).

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
