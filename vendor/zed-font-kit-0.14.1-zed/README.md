# font-kit

[![Build Status](https://github.com/servo/font-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/servo/font-kit/actions)
[![crates.io](https://img.shields.io/crates/v/font-kit.svg)](https://crates.io/crates/font-kit)
[![Documentation](https://docs.rs/font-kit/badge.svg)](https://docs.rs/font-kit)

`font-kit` provides a common interface to the various system font libraries and provides
services such as finding fonts on the system, performing nearest-font matching, and rasterizing
glyphs.

## Synopsis

```rust
let font = SystemSource::new()
    .select_by_postscript_name("ArialMT")
    .unwrap()
    .load()
    .unwrap();

let glyph_id = font.glyph_for_char('A').unwrap();
let mut canvas = Canvas::new(&Size2D::new(32, 32), Format::A8);

font.rasterize_glyph(
    &mut canvas,
    glyph_id,
    32.0,
    &Point2D::new(0.0, 32.0),
    HintingOptions::None,
    RasterizationOptions::GrayscaleAa,
)
.unwrap();
```

## Backends

`font-kit` delegates to system libraries to perform tasks. It has two types of backends: a *source*
and a *loader*. Sources are platform font databases; they allow lookup of installed fonts by name
or attributes. Loaders are font loading libraries; they allow font files (TTF, OTF, etc.) to be
loaded from a file on disk or from bytes in memory. Sources and loaders can be freely intermixed at
runtime; fonts can be looked up via DirectWrite and rendered via FreeType, for example.

Available loaders:

* Core Text (macOS): The system font loader on macOS. Does not do hinting except when bilevel
  rendering is in use.

* DirectWrite (Windows): The newer system framework for text rendering on Windows. Does vertical
  hinting but not full hinting.

* FreeType (cross-platform): A full-featured font rendering framework.

Available sources:

* Core Text (macOS): The system font database on macOS.

* DirectWrite (Windows): The newer API to query the system font database on Windows.

* Fontconfig (cross-platform): A technically platform-neutral, but in practice Unix-specific, API
  to query and match fonts.

* Filesystem (cross-platform): A simple source that reads fonts from a path on disk. This is the
  default on Android.

* Memory (cross-platform): A source that reads from a fixed set of fonts in memory.

* Multi (cross-platform): A source that allows multiple sources to be queried at once.

On Windows and macOS, the FreeType loader and the Fontconfig source are not built by default.
To build them, use the `loader-freetype` and `source-fontconfig` Cargo features respectively. If
you want them to be the default, instead use the `loader-freetype-default` and
`source-fontconfig-default` Cargo features respectively. Beware that `source-fontconfig-default` is
rarely what you want on those two platforms!

If you don't need to locate fonts on the system at all—for example, if all your fonts are stored
with your app—then you can omit the default `source` feature and none of that code will be
included.

## Features

`font-kit` is capable of doing the following:

* Loading fonts from files or memory.

* Determining whether files on disk or in memory represent fonts.

* Interoperating with native font APIs.

* Querying various metadata about fonts.

* Doing simple glyph-to-character mapping. (For more complex use cases, a shaper is required;
  proper shaping is beyond the scope of `font-kit`.)

* Reading unhinted or hinted vector outlines from glyphs.

* Calculating glyph and font metrics.

* Looking up glyph advances and origins.

* Rasterizing glyphs using the native rasterizer, optionally using hinting. (Custom rasterizers,
  such as Pathfinder, can be used in conjunction with the outline API.)

* Looking up all fonts on the system.

* Searching for specific fonts by family or PostScript name.

* Performing font matching according to the [CSS Fonts Module Level 3] specification.

## Dependencies

**Ubuntu**

`sudo apt install pkg-config libfreetype6-dev libfontconfig1-dev`

## License

`font-kit` is licensed under the same terms as Rust itself.

[CSS Fonts Module Level 3]: https://drafts.csswg.org/css-fonts-3/#font-matching-algorithm
