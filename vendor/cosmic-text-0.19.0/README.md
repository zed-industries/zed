# COSMIC Text

[![crates.io](https://img.shields.io/crates/v/cosmic-text.svg)](https://crates.io/crates/cosmic-text)
[![docs.rs](https://docs.rs/cosmic-text/badge.svg)](https://docs.rs/cosmic-text)
![license](https://img.shields.io/crates/l/cosmic-text.svg)
[![Rust workflow](https://github.com/pop-os/cosmic-text/workflows/Rust/badge.svg?event=push)](https://github.com/pop-os/cosmic-text/actions)

Pure Rust multi-line text handling.

COSMIC Text provides advanced text shaping, layout, and rendering wrapped up
into a simple abstraction. Shaping is provided by HarfRust, and supports a
wide variety of advanced shaping operations. Rendering is provided by swash,
which supports ligatures and color emoji. Layout is implemented custom, in safe
Rust, and supports bidirectional text. Font fallback is also a custom
implementation, reusing some of the static fallback lists in browsers such as
Chromium and Firefox. Linux, macOS, and Windows are supported with the full
feature set. Other platforms may need to implement font fallback capabilities.

## Screenshots

Arabic translation of Universal Declaration of Human Rights
[![Arabic screenshot](screenshots/arabic.png)](screenshots/arabic.png)

Hindi translation of Universal Declaration of Human Rights
[![Hindi screenshot](screenshots/hindi.png)](screenshots/hindi.png)

Simplified Chinese translation of Universal Declaration of Human Rights
[![Simplified Chinses screenshot](screenshots/chinese-simplified.png)](screenshots/chinese-simplified.png)

[View Universal Declaration of Human Rights on OHCHR](https://www.ohchr.org/en/universal-declaration-of-human-rights)

## Roadmap

The following features must be supported before this is "ready":

- [x] Font loading (using fontdb)
  - [x] Preset fonts
  - [x] System fonts
- [x] Text styles (bold, italic, etc.)
  - [x] Per-buffer
  - [x] Per-span
- [x] Font shaping (using HarfRust)
  - [x] Cache results
  - [x] RTL
  - [x] Bidirectional rendering
- [x] Font fallback
  - [x] Choose font based on locale to work around "unification"
  - [x] Per-line granularity
  - [x] Per-character granularity
- [x] Font layout
  - [x] Click detection
  - [x] Simple wrapping
  - [ ] Wrapping with indentation
  - [ ] No wrapping
  - [ ] Ellipsize
- [x] Font rendering (using swash)
  - [x] Cache results
  - [x] Font hinting
  - [x] Ligatures
  - [x] Color emoji
- [x] Text editing
    - [x] Performance improvements
    - [x] Text selection
    - [x] Can automatically recreate https://unicode.org/udhr/ without errors (see below)
    - [x] Bidirectional selection
    - [ ] Copy/paste
- [x] no_std support (with `default-features = false`)
    - [ ] no_std font loading
    - [x] no_std shaping
    - [x] no_std layout
    - [ ] no_std rendering

The UDHR (Universal Declaration of Human Rights) test involves taking the entire
set of UDHR translations (almost 500 languages), concatenating them as one file
(which ends up being 8 megabytes!), then via the `editor-test` example,
automatically simulating the entry of that file into cosmic-text per-character,
with the use of backspace and delete tested per character and per line. Then,
the final contents of the buffer is compared to the original file. All of the
106746 lines are correct.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
