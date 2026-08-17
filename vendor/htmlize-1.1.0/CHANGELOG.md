# Change log

All notable changes to this project will be documented in this file.

## Release 1.1.0 (2026-04-13)

* Major performance improvements when unescaping text in many cases (for both
  the `unescape` and `unescape_fast` features).
* Major improvements in build time for the `unescape_fast` features (went from
  8 seconds to 3 seconds on my laptop).
* Add `BARE_ENTITY_MAX_LENGTH` constant that contains the length of the longest
  entity without a semicolon (enabled with feature `entities`).
* Clarify examples in documentation and README.
* Fix a few spelling mistakes in documentation.

### Security

* [RUSTSEC-2026-0097]: the [rand] crate was unsound in certain circumstances.
  Htmlize depends on [rand] via [phf] and couldn’t trigger the unsoundness on
  its own. Thanks to [MarkusPettersson98] for the [PR][#124]!

[RUSTSEC-2026-0097]: https://rustsec.org/advisories/RUSTSEC-2026-0097
[rand]: https://crates.io/crates/rand
[phf]: https://crates.io/crates/phf
[MarkusPettersson98]: https://github.com/MarkusPettersson98
[#124]: https://github.com/danielparks/htmlize/pull/124

## Release 1.0.6 (2025-04-26)

* Switch dependency from [paste], which is no longer maintained, to a new fork,
  [pastey], which is.

[paste]: https://github.com/dtolnay/paste
[pastey]: https://github.com/as1100k/pastey

## Release 1.0.5 (2024-03-14)

* Exclude more files from final package to significantly reduce package size.

## Release 1.0.4 (2024-02-18)

* Hide usage of [assert2] in doc examples to make them slightly clearer for
  users not familiar with it.

### Security fixes

* Upgrade indirect dependency [rustix] to fix a [security
  vulnerability][GHSA-c827-hfw6-qwvm] in directory iterators. This does not
  affect htmlize, since rustix is only used by development dependencies.

[assert2]: https://crates.io/crates/assert2
[rustix]: https://crates.io/crates/rustix
[GHSA-c827-hfw6-qwvm]: https://github.com/advisories/GHSA-c827-hfw6-qwvm

## Release 1.0.3 (2023-07-09)

* Enabled feature marks on [docs.rs] to make it clearer what features are
  required by what functions.
* Clarified ownership and licensing of entities.json data file.

[docs.rs]: https://docs.rs/htmlize

## Release 1.0.2 (2023-03-04)

* Fix building with `unescape` feature but not `unescape_fast`. Added tests for
  a few common feature flags — in addition to `--all-features` — to the CI check
  to avoid this sort of problem in the future.

## Release 1.0.1 (2023-03-04)

* Fix [docs.rs] build to enable the `unescape` and `entities` features.

[docs.rs]: https://docs.rs/htmlize

## Release 1.0.0 (2023-03-03)

### Breaking changes

* Hid `unescape()` behind `unescape` feature. This allows users to avoid the
  dependency on [phf] and the build dependency on [serde_json], which cuts build
  times on my machine by more than 90% (from 6.2 seconds to 0.5 seconds).
* Hid `ENTITIES` behind `entities` features for the same reason I added the
  `unescape` feature. Note that the `unescape` feature automatically enables
  the `entities` feature, but `unescape_faster` does not.
* Switched both escape and unescape functions to use `Cow<'a, str>` for input
  and output. This allows for significant performance improvements when the
  input can be returned unchanged.
* Updated minimum supported Rust version (MSRV) to 1.60.

### Improvements

* Significantly optimized both escape and unescape functions. Many of the
  improvements to the escape functions are similar to the ones outlined in Lise
  Henry’s [excellent post on optimizing HTML entity escaping][optimize-post]
  (see also: [its Reddit discussion][optimize-reddit]), though most notably I’m
  using [memchr] directly rather than [regex].
* Added `unescape_faster` feature for even faster unescaping at the cost of
  longer build times (about 30 seconds longer on my machine).
* Added `unescape_attribute()` to handle the special rules for dealing with
  entities in the value of an HTML attribute. Also adds `unescape_in()`, which
  takes a context parameter that can either be `Context::Attribute` or
  `Context::General` (for everything else).
* Added `unescape_bytes_in()` to work on `[u8]` rather than `str`.
* Added `escape_..._bytes()` functions to work on `[u8]` rather than `str`.
* Switched to the [phf_codegen] crate instead of using the `phf_map!` macro.
  On my machine, this cuts build time by about 25% (~2 seconds).
* Clarified documentation of `ENTITIES` to indicate that it’s a `Map`, not just
  a collection of tuples.

### Bug fixes

* `unescape()` incorrectly outputted the replacement character (U+FFFD “�”) for
  certain numeric entities:

    * [Noncharacters]
    * [Control] characters
    * `0x0D` (carriage return)

  A close reading of the [spec] and some browser testing shows that behavior to
  be incorrect. Those characters are now outputted as themselves.

* `unescape()` incorrectly outputted long numeric entities as the literal text
  of the entity.

  A close reading of the [spec] and some browser testing shows that behavior to
  be incorrect. Those long entities are now outputted as the replacement
  character (U+FFFD “�”).

[phf]: https://crates.io/crates/phf
[phf_codegen]: https://crates.io/crates/phf_codegen
[serde_json]: https://crates.io/crates/serde_json
[optimize-post]: https://lise-henry.github.io/articles/optimising_strings.html
[optimize-reddit]: https://www.reddit.com/r/rust/comments/55wpxh/optimising_string_processing_in_rust/
[memchr]: https://docs.rs/memchr
[regex]: https://docs.rs/regex
[Noncharacters]: https://infra.spec.whatwg.org/#noncharacter
[Control]: https://infra.spec.whatwg.org/#control
[spec]: https://html.spec.whatwg.org/multipage/parsing.html#numeric-character-reference-end-state

## Release 0.5.1 (2022-12-13)

### Bug fixes

* Switched from [assertify] (deprecated) to [assert2] for testing.
* Fixed typo of in docs: “entries.json” should have been “entities.json”.
* Fixed formatting and lints.
* Added this change log.

[assertify]: https://crates.io/crates/assertify
[assert2]: https://crates.io/crates/assert2
