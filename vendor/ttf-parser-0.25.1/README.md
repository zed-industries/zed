## ttf-parser
![Build Status](https://github.com/harfbuzz/ttf-parser/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/ttf-parser.svg)](https://crates.io/crates/ttf-parser)
[![Documentation](https://docs.rs/ttf-parser/badge.svg)](https://docs.rs/ttf-parser)
[![Rust 1.59+](https://img.shields.io/badge/rust-1.59+-orange.svg)](https://www.rust-lang.org)
![](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)

A high-level, safe, zero-allocation font parser for
[TrueType](https://docs.microsoft.com/en-us/typography/truetype/),
[OpenType](https://docs.microsoft.com/en-us/typography/opentype/spec/), and
[AAT](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6AATIntro.html).

Can be used as a Rust or C library.

### Features

- A high-level API for most common properties, hiding all parsing and data resolving logic.
- A low-level, but safe API to access TrueType tables data.
- Highly configurable. You can disable most of the features, reducing binary size.
  You can also parse TrueType tables separately, without loading the whole font/face.
- Zero heap allocations.
- Zero unsafe.
- Zero dependencies.
- `no_std`/WASM compatible.
- A basic [C API](./c-api).
- Fast.
- Stateless. All parsing methods are immutable.
- Simple and maintainable code (no magic numbers).

### Safety

- The library must not panic. Any panic considered as a critical bug and should be reported.
- The library forbids unsafe code.
- No heap allocations, so crash due to OOM is not possible.
- All recursive methods have a depth limit.
- Technically, should use less than 64KiB of stack in the worst case scenario.
- Most of arithmetic operations are checked.
- Most of numeric casts are checked.

### Alternatives

It's very hard to compare different libraries, so we are using table-based comparison.
There are roughly three types of TrueType tables:

- A table with a list of properties (like `head`, `OS/2`, etc.).<br/>
  If a library tries to parse it at all then we mark it as supported.
- A table that contains a single type of data (`glyf`, `CFF` (kinda), `hmtx`, etc.).<br/>
  Can only be supported or not.
- A table that contains multiple subtables (`cmap`, `kern`, `GPOS`, etc.).<br/>
  Can be partially supported and we note which subtables are actually supported.

| Feature/Library   | ttf-parser             | FreeType            | stb_truetype                   |
| ----------------- | :--------------------: | :-----------------: | :----------------------------: |
| Memory safe       | ✓                      |                     |                                |
| Thread safe       | ✓                      |                     | ~ (mostly reentrant)           |
| Zero allocation   | ✓                      |                     |                                |
| Variable fonts    | ✓                      | ✓                   |                                |
| Rendering         | -<sup>1</sup>          | ✓                   | ~ (very primitive)             |
| `ankr` table      | ✓                      |                     |                                |
| `avar` table      | ✓                      | ✓                   |                                |
| `bdat` table      | ~ (no 4)               | ✓                   |                                |
| `bloc` table      | ✓                      | ✓                   |                                |
| `CBDT` table      | ~ (no 8, 9)            | ✓                   |                                |
| `CBLC` table      | ✓                      | ✓                   |                                |
| `COLR` table      | ✓                      | ✓                   |                                |
| `CPAL` table      | ✓                      | ✓                   |                                |
| `CFF `&nbsp;table | ✓                      | ✓                   | ~ (no `seac` support)          |
| `CFF2` table      | ✓                      | ✓                   |                                |
| `cmap` table      | ~ (no 8)               | ✓                   | ~ (no 2,8,10,14; Unicode-only) |
| `EBDT` table      | ~ (no 8, 9)            | ✓                   |                                |
| `EBLC` table      | ✓                      | ✓                   |                                |
| `feat` table      | ✓                      |                     |                                |
| `fvar` table      | ✓                      | ✓                   |                                |
| `gasp` table      |                        | ✓                   |                                |
| `GDEF` table      | ~                      |                     |                                |
| `glyf` table      | ~<sup>2</sup>          | ✓                   | ~<sup>2</sup>                  |
| `GPOS` table      | ✓                      |                     | ~ (only 2)                     |
| `GSUB` table      | ✓                      |                     |                                |
| `gvar` table      | ✓                      | ✓                   |                                |
| `head` table      | ✓                      | ✓                   | ✓                              |
| `hhea` table      | ✓                      | ✓                   | ✓                              |
| `hmtx` table      | ✓                      | ✓                   | ✓                              |
| `HVAR` table      | ✓                      | ✓                   |                                |
| `kern` table      | ✓                      | ~ (only 0)          | ~ (only 0)                     |
| `kerx` table      | ✓                      |                     |                                |
| `MATH` table      | ✓                      |                     |                                |
| `maxp` table      | ✓                      | ✓                   | ✓                              |
| `morx` table      | ✓                      |                     |                                |
| `MVAR` table      | ✓                      | ✓                   |                                |
| `name` table      | ✓                      | ✓                   |                                |
| `OS/2` table      | ✓                      | ✓                   |                                |
| `post` table      | ✓                      | ✓                   |                                |
| `sbix` table      | ~ (PNG only)           | ~ (PNG only)        |                                |
| `STAT` table      | ✓                      |                     |                                |
| `SVG `&nbsp;table | ✓                      | ✓                   | ✓                              |
| `trak` table      | ✓                      |                     |                                |
| `vhea` table      | ✓                      | ✓                   |                                |
| `vmtx` table      | ✓                      | ✓                   |                                |
| `VORG` table      | ✓                      | ✓                   |                                |
| `VVAR` table      | ✓                      | ✓                   |                                |
| Language          | Rust + C API           | C                   | C                              |
| Tested version    | 0.17.0                 | 2.12.0              | 1.24                           |
| License           | MIT / Apache-2.0       | FTL / GPLv2         | public domain                  |

Legend:

- ✓ - supported
- ~ - partial
- *nothing* - not supported

Notes:

1. While `ttf-parser` doesn't support rendering by itself,
   there are multiple rendering libraries on top of it:
   [rusttype](https://gitlab.redox-os.org/redox-os/rusttype),
   [ab-glyph](https://github.com/alexheretic/ab-glyph)
   and [fontdue](https://github.com/mooman219/fontdue).
2. Matching points are not supported.

### Performance

TrueType fonts designed for fast querying, so most of the methods are very fast.
The main exception is glyph outlining. Glyphs can be stored using two different methods:
using [Glyph Data Format](https://docs.microsoft.com/en-us/typography/opentype/spec/glyf)
and [Compact Font Format](http://wwwimages.adobe.com/content/dam/Adobe/en/devnet/font/pdfs/5176.CFF.pdf) (pdf).
The first one is fairly simple which makes it faster to process.
The second one is basically a tiny language with a stack-based VM, which makes it way harder to process.

The [benchmark](./benches/outline/) tests how long it takes to outline all glyphs in a font.

x86 (AMD 3700X)

| Table/Library | ttf-parser     | FreeType   | stb_truetype   |
| ------------- | -------------: | ---------: | -------------: |
| `glyf`        |   `0.901 ms`   | `1.171 ms` | **`0.675 ms`** |
| `gvar`        | **`2.972 ms`** | `4.132 ms` |              - |
| `CFF`         | **`1.197 ms`** | `5.647 ms` |   `2.813 ms`   |
| `CFF2`        | **`1.968 ms`** | `6.392 ms` |              - |

ARM (Apple M1)

| Table/Library | ttf-parser     | FreeType   | stb_truetype   |
| ------------- | -------------: | ---------: | -------------: |
| `glyf`        | **`0.550 ms`** | `0.854 ms` |   `0.703 ms`   |
| `gvar`        | **`2.270 ms`** | `4.594 ms` |              - |
| `CFF`         | **`1.054 ms`** | `5.223 ms` |   `3.262 ms`   |
| `CFF2`        | **`1.765 ms`** | `5.995 ms` |              - |

**Note:** FreeType is surprisingly slow, so I'm worried that I've messed something up.

And here are some methods benchmarks:

```text
test outline_glyph_276_from_cff2 ... bench:         867 ns/iter (+/- 15)
test from_data_otf_cff           ... bench:         968 ns/iter (+/- 13)
test from_data_otf_cff2          ... bench:         887 ns/iter (+/- 25)
test outline_glyph_276_from_cff  ... bench:         678 ns/iter (+/- 41)
test outline_glyph_276_from_glyf ... bench:         649 ns/iter (+/- 11)
test outline_glyph_8_from_cff2   ... bench:         534 ns/iter (+/- 14)
test from_data_ttf               ... bench:         467 ns/iter (+/- 11)
test glyph_name_post_276         ... bench:         223 ns/iter (+/- 5)
test outline_glyph_8_from_cff    ... bench:         315 ns/iter (+/- 13)
test outline_glyph_8_from_glyf   ... bench:         291 ns/iter (+/- 5)
test family_name                 ... bench:         183 ns/iter (+/- 102)
test glyph_name_cff_276          ... bench:          62 ns/iter (+/- 1)
test glyph_index_u41             ... bench:          16 ns/iter (+/- 0)
test glyph_name_cff_8            ... bench:           5 ns/iter (+/- 0)
test glyph_name_post_8           ... bench:           2 ns/iter (+/- 0)
test subscript_metrics           ... bench:           2 ns/iter (+/- 0)
test glyph_hor_advance           ... bench:           2 ns/iter (+/- 0)
test glyph_hor_side_bearing      ... bench:           2 ns/iter (+/- 0)
test glyph_name_8                ... bench:           1 ns/iter (+/- 0)
test ascender                    ... bench:           1 ns/iter (+/- 0)
test underline_metrics           ... bench:           1 ns/iter (+/- 0)
test strikeout_metrics           ... bench:           1 ns/iter (+/- 0)
test x_height                    ... bench:           1 ns/iter (+/- 0)
test units_per_em                ... bench:         0.5 ns/iter (+/- 0)
test width                       ... bench:         0.2 ns/iter (+/- 0)
```

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
