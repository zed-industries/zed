# roughr-merman

[![Crates.io](https://img.shields.io/crates/v/roughr-merman.svg)](https://crates.io/crates/roughr-merman)
[![Documentation](https://docs.rs/roughr-merman/badge.svg)](https://docs.rs/roughr-merman)
[![Crates.io Downloads](https://img.shields.io/crates/d/roughr-merman.svg)](https://crates.io/crates/roughr-merman)
[![Made with Rust](https://img.shields.io/badge/made%20with-Rust-orange.svg)](https://www.rust-lang.org)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is a fork of `roughr` (Rough.js port) vendored for the `merman` workspace to keep Mermaid SVG
parity stable and deterministic across platforms.

## Differences from upstream `roughr`

- PRNG / seeding semantics are aligned with Rough.js (used by Mermaid) to keep generated ops stable.
- Some defaults are adjusted for Mermaid parity; treat this as an implementation dependency rather than a drop-in replacement.

If you want to use it under the crate name `roughr`, depend on it like this:

```toml
[dependencies]
roughr = { package = "roughr-merman", version = "0.12.0" }
```

<!-- cargo-sync-readme start -->


This crate is a rustlang port of [Rough.js](https://github.com/rough-stuff/rough) npm package written by
[@pshihn](https://github.com/pshihn).

This package exposes functions to generate rough drawing primitives which looks like hand drawn sketches.
This is the core create of operations to create rough drawings. It exposes its own primitive drawing types for lines
curves, arcs, polygons, circles, ellipses and even svg paths.
Works on [Point2D](https://docs.rs/euclid/0.22.7/euclid/struct.Point2D.html) type from [euclid](https://github.com/servo/euclid) crate

On its own this crate can not draw on any context. One needs to use existing drawing libraries such as [piet](https://github.com/linebender/piet),
[raqote](https://github.com/jrmuizel/raqote), [tiny-skia](https://github.com/RazrFalcon/tiny-skia) etc in combination with
roughr. In this workspace an example adapter is implemented for [piet](https://github.com/linebender/piet). Below examples are
output of [rough_piet](https://github.com/orhanbalci/rough-rs/tree/main/rough_piet) adapter.

## Cargo.toml

```toml
[dependencies]
roughr = { package = "roughr-merman", version = "0.12.0" }
```

## Example

### Rectangle

```rust
let options = OptionsBuilder::default()
    .stroke(Srgba::from_raw(&[114u8, 87u8, 82u8, 255u8]).into_format())
    .fill(Srgba::from_raw(&[254u8, 246u8, 201u8, 255u8]).into_format())
    .fill_style(FillStyle::Hachure)
    .fill_weight(DPI * 0.01)
    .build()
    .unwrap();
let generator = KurboGenerator::new(options);
let rect_width = 100.0;
let rect_height = 50.0;
let rect = generator.rectangle::<f32>(
    (WIDTH as f32 - rect_width) / 2.0,
    (HEIGHT as f32 - rect_height) / 2.0,
    rect_width,
    rect_height,
);
let background_color = Color::from_hex_str("96C0B7").unwrap();

rc.fill(
    Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
    &background_color,
);
rect.draw(&mut rc);
```

### Output Rectangle
![rectangle](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/roughr/assets/rectangle.png)

### Circle

```rust
let options = OptionsBuilder::default()
    .stroke(Srgba::from_raw(&[114u8, 87u8, 82u8, 255u8]).into_format())
    .fill(Srgba::from_raw(&[254u8, 246u8, 201u8, 255u8]).into_format())
    .fill_style(FillStyle::Hachure)
    .fill_weight(DPI * 0.01)
    .build()
    .unwrap();
let generator = KurboGenerator::new(options);
let circle_paths = generator.circle::<f32>(
    (WIDTH as f32) / 2.0,
    (HEIGHT as f32) / 2.0,
    HEIGHT as f32 - 10.0f32,
);
let background_color = Color::from_hex_str("96C0B7").unwrap();

rc.fill(
    Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
    &background_color,
);
circle_paths.draw(&mut rc);
```

### Output Circle
![circle](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/roughr/assets/circle.png)


### Ellipse

```rust
let options = OptionsBuilder::default()
    .stroke(Srgba::from_raw(&[114u8, 87u8, 82u8, 255u8]).into_format())
    .fill(Srgba::from_raw(&[254u8, 246u8, 201u8, 255u8]).into_format())
    .fill_style(FillStyle::Hachure)
    .fill_weight(DPI * 0.01)
    .build()
    .unwrap();
let generator = KurboGenerator::new(options);
let ellipse_paths = generator.ellipse::<f32>(
    (WIDTH as f32) / 2.0,
    (HEIGHT as f32) / 2.0,
    WIDTH as f32 - 10.0,
    HEIGHT as f32 - 10.0,
);
let background_color = Color::from_hex_str("96C0B7").unwrap();

rc.fill(
    Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
    &background_color,
);
ellipse_paths.draw(&mut rc);
```

### Output Ellipse
![ellipse](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/roughr/assets/ellipse.png)


### SVG Path

```rust
let options = OptionsBuilder::default()
    .stroke(Srgba::from_raw(&[114u8, 87u8, 82u8, 255u8]).into_format())
    .fill(Srgba::from_raw(&[254u8, 246u8, 201u8, 255u8]).into_format())
    .fill_style(FillStyle::Hachure)
    .fill_weight(DPI * 0.01)
    .build()
    .unwrap();
let generator = KurboGenerator::new(options);
let heart_svg_path  = "M140 20C73 20 20 74 20 140c0 135 136 170 228 303 88-132 229-173 229-303 0-66-54-120-120-120-48 0-90 28-109 69-19-41-60-69-108-69z".into();
let heart_svg_path_drawing = generator.path::<f32>(heart_svg_path);
let background_color = Color::from_hex_str("96C0B7").unwrap();

rc.fill(
    Rect::new(0.0, 0.0, WIDTH as f64, HEIGHT as f64),
    &background_color,
);
heart_svg_path_drawing.draw(&mut rc);
```

### Output SVG Path
![svgheart](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/roughr/assets/heart_svg_path.png)

## Filler Implementation Status
- [x] Hachure
- [x] Zigzag
- [x] Cross-Hatch
- [x] Dots
- [x] Dashed
- [x] Zigzag-Line

## Examples

For more examples have a look at the
[examples](https://github.com/orhanbalci/rough-rs/tree/main/rough_piet/examples) folder.

<!-- cargo-sync-readme end -->

## License

Licensed under MIT License ([LICENSE](LICENSE)).

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
