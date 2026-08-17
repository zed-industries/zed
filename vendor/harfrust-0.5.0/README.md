# HarfRust

<div align="center">
<p><img src="HarfRust.png" alt="HarfRust Logo" width="256" align="center"/></p>

[![Build Status](https://github.com/harfbuzz/harfrust/actions/workflows/main.yml/badge.svg)](https://github.com/harfbuzz/harfrust/actions/workflows/main.yml)
[![Crates.io](https://img.shields.io/crates/v/harfrust.svg)](https://crates.io/crates/harfrust)
[![Documentation](https://docs.rs/harfrust/badge.svg)](https://docs.rs/harfrust)

</div>

HarfRust is a Rust port of [HarfBuzz](https://github.com/harfbuzz/harfbuzz) text shaping engine.
See [Major changes](#major-changes) below for major differences between HarfRust and HarfBuzz.

HarfRust started as a fork of [RustyBuzz](https://docs.rs/rustybuzz) to explore porting from `ttf-parser` to
[`read-fonts`](https://docs.rs/read-fonts) to avoid shipping (and maintaining)
multiple implementations of core font parsing for [`skrifa`](https://docs.rs/skrifa) consumers.
Further context in https://github.com/googlefonts/fontations/issues/956.

Matches HarfBuzz [v12.3.0](https://github.com/harfbuzz/harfbuzz/releases/tag/12.3.0).

## Why?

https://github.com/googlefonts/oxidize outlines Google Fonts motivations to try to migrate font
production and consumption to Rust.

## Major changes

- No font size property. Shaping is always using UnitsPerEm. You should scale the result manually.
- Most of the font loading and parsing is done using [`read-fonts`](https://docs.rs/read-fonts).
- HarfRust doesn't provide any integration with external libraries, so no FreeType, CoreText, or Uniscribe/DirectWrite font-loading integration, and no ICU, or GLib Unicode-functions integration, as well as no `graphite2` library support.
- `mort` table is not supported, since it's deprecated by Apple.
- No `graphite` font support.

## Conformance

The following conformance issues need to be fixed:

- HarfRust for the most part passes the HarfBuzz test and fuzzing suites, but there are some known issues. See [HARFBUZZ.md](./HARFBUZZ.md) for details.
- Malformed fonts will cause an error. HarfBuzz uses fallback/dummy shaper in this case.
- No Arabic fallback shaper. This requires the ability to build lookups on the fly. In HarfBuzz (C++) this requires serialization code that is associated with subsetting.
- Experimental HarfBuzz features like most of the boring-expansion-spec are not supported yet.

## Performance

HarfRust is less than 25% slower than HarfBuzz on most common fonts. For a comparison see this [spreadsheet][3].
You can run `cargo bench` to see the performance of HarfRust on your machine.


## Notes about the port

HarfRust is not a full port of HarfBuzz. HarfBuzz (C++ edition) can roughly be split into 6 parts:

1. shaping, ported to HarfRust
2. Unicode routines, ported to HarfRust
3. font parsing, handled by [`read-fonts`](https://docs.rs/read-fonts)
4. subsetting, handled by [`klippa`](https://github.com/googlefonts/fontations/tree/main/klippa)
5. custom containers and utilities (HarfBuzz doesn't use C++ standard library), reimplemented in [`fontations`](https://github.com/googlefonts/fontations) where appropriate (e.g. int set)
6. glue for system/3rd party libraries, not ported

## Safety

The library is completely safe.

There are no `unsafe` in this library and in most of its dependencies (excluding `bytemuck`).

## Developer documents

For notes on the backporting process of HarfBuzz code, see [docs/backporting.md](docs/backporting.md).

For notes on generating state machine using `ragel`, see [docs/ragel.md](docs/ragel.md).

The following HarfBuzz _studies_ are relevant to HarfRust development:

- 2025 - [Introducing HarfRust][2]
- 2025 – [Caching][1]

## License

HarfRust is licensed under the **MIT** license.

HarfBuzz is [licensed](https://github.com/harfbuzz/harfbuzz/blob/main/COPYING) under the **Old MIT**

[1]: https://docs.google.com/document/d/1_VgObf6Je0J8byMLsi7HCQHnKo2emGnx_ib_sHo-bt4/preview
[2]: https://docs.google.com/document/d/1aH_waagdEM5UhslQxCeFEb82ECBhPlZjy5_MwLNLBYo/preview
[3]: https://docs.google.com/spreadsheets/d/1lyPPZHXIF8gE0Tpx7_IscwhwaZa4KOpdt7vnV0jQT9o/preview
