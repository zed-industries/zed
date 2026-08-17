# Correctly encode and decode HTML entities

[![docs.rs](https://img.shields.io/docsrs/htmlize)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/htmlize)][crates.io]
![Rust version 1.60+](https://img.shields.io/badge/Rust%20version-1.60%2B-success)

Htmlize handles both encoding raw strings to be safely inserted in HTML, and
decoding HTML text with entities to get back a raw string. It closely follows
the [official WHATWG spec] for encoding and decoding text.

```rust
use htmlize::{escape_attribute, escape_text};
assert!(escape_attribute("abc & < > \" '") == "abc &amp; &lt; &gt; &quot; '");
assert!(escape_text("abc & < > \" '") == "abc &amp; &lt; &gt; \" '");
```

If you enable the `unescape` or `unescape_fast` feature:

```rust
assert!(htmlize::unescape("3 &times 4 &gt; 10") == "3 × 4 > 10");
```

## Quick start

If you only need to escape text for embedding into HTML, then adding htmlize to
your crate is as simple as:

```sh
cargo add htmlize
```

If you want to unescape entities back into raw text, see [Unescaping entities
into text](#unescaping-entities-into-text) below.

This only deals with HTML entities; it does not add or remove HTML tags.

## Escaping text into entities

If the text goes in an attribute, use [`escape_attribute()`], otherwise use
[`escape_text()`].

|                         | `&` | `<` | `>` | `"` | `'` |
|-------------------------|:---:|:---:|:---:|:---:|:---:|
| [`escape_text()`]       |  ✓  |  ✓  |  ✓  |     |     |
| [`escape_attribute()`]  |  ✓  |  ✓  |  ✓  |  ✓  |     |
| [`escape_all_quotes()`] |  ✓  |  ✓  |  ✓  |  ✓  |  ✓  |

You should almost never need [`escape_all_quotes()`], but it is included because
sometimes it’s convenient to wrap attribute values in single quotes.

For other characters, e.g. “★”, I recommend just using the character directly
rather than escaping it with an entity. Please file an [issue][issues] with your
use case if you need to encode other entities.

### `escape_text(string) -> string`

Escape a string so that it can be embedded in the main text. This does not
escape quotes at all.

[Reference][`escape_text()`]. See also [`escape_text_bytes()`].

### `escape_attribute(string) -> string`

Escape a string so that it can be embedded in an attribute. Always use double
quotes around attributes.

[Reference][`escape_attribute()`]. See also [`escape_attribute_bytes()`].

### `escape_all_quotes(string) -> string`

Escape both single and double quotes in a string along with other standard
characters. In general you should not need to use this.

[Reference][`escape_all_quotes()`]. See also [`escape_all_quotes_bytes()`].

## Unescaping entities into text

This requires the `unescape` or `unescape_fast` feature. (`unescape` builds
much faster, so unless you really need the very fastest unescape, use it.) To
configure it:

```sh
cargo add htmlize --features unescape
```

### `unescape(string) -> string`

This follows the [official WHATWG spec] for expanding entities outside of
attributes, i.e. in the text.

Strictly speaking, this does not correctly handle text from the value of
attributes. It’s probably fine for most uses, but if you know that the input
string came from the value of an attribute, use [`unescape_attribute()`]
instead. See the [`unescape_in()` reference documentation][`unescape_in()`] for
more information.

[Reference][`unescape()`].

### `unescape_attribute(string) -> string`

This follows the [official WHATWG spec] for expanding entities found in the
value of an attribute.

The only difference is in how this handles named entities without a trailing
semicolon. See the [`unescape_in()` reference documentation][`unescape_in()`]
for more information.

[Reference][`unescape_attribute()`].

### `unescape_in(string, Htmlize::Context) -> string`

This follows the [official WHATWG spec] for expanding entities based on the
context where they are found. See the [reference documentation][`unescape_in()`]
for more information.

[Reference][`unescape_in()`].

### `unescape_bytes_in([u8], Htmlize::Context) -> [u8]`

This is the same as [`unescape_in()`], except that it works on bytes rather than
strings. (Note that both functions actually take and return [`Cow`]s.)

[Reference][`unescape_bytes_in()`].

## Features

The `escape` functions are all available with no features enabled.

  * `unescape_fast`: provide fast version of [`unescape()`]. This does _not_
    enable the `entities` feature automatically.

    This takes perhaps 2 seconds longer to build than `unescape`, but the
    performance is significantly better in the worst cases. That said, the
    performance of of the `unescape` version is already pretty good, so I don’t
    recommend enabling this unless you really need it.

  * `unescape`: provide normal version of `unescape()`. This will
    automatically enable the `entities` feature.

  * `entities`: build `ENTITIES` map. Enabling this will add a dependency
    on [phf] and may slow builds by a few seconds.

All other features are internal and should not be used when specifying a
dependency. See the [reference documentation][features].

## Benchmarks

This has two suites of benchmarks. One is a typical multi-run benchmark using
[criterion]. These can be run with `cargo bench` or [`cargo criterion`] if you
have it installed.

To run benchmarks on the unescape functions, enable features `bench` and
`unescape` or `unescape_fast` (or both).

**Note:** The internal `bench` feature is required to expose internal functions
like `unescape_fast()` and `unescape_slow()` to the benchmarks. You must not
enable this feature when specifying a dependency, since its behavior is not
guaranteed to stay the same from point release to point release.

### iai benchmarks

The other suite of benchmarks uses [iai] to count instructions, cache accesses,
and to estimate cycles. It requires the internal `iai` feature to be enabled,
and only really works well on Linux.

To run iai benchmarks locally:

```sh
cargo bench --features iai iai
```

You may want to use `--all-features` or `--features iai,bench,unescape` or
`--features iai,bench,unescape_fast` to enable benchmarks of the `unescape()`
functions.

To run in a Docker container, use the `docker.sh` script. It will build an image
if necessary, then use that image for all future runs:

```sh
./docker.sh cargo bench --features iai iai
```

You can also start it in interactive mode and run the benchmark multiple times:

```
❯ ./docker.sh
root@d0a0db46770d:/work# cargo bench --features iai iai
   Compiling htmlize [...]
```

## Development

This is stable. I have no features planned for the future, though I’m open to
[suggestions][issues].

### LLM use

This crate includes no LLM (AI) produced code.

### License

Unless otherwise noted, this project is dual-licensed under the Apache 2 and MIT
licenses. You may choose to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

The entities.json file is copyright WHATWG, and is copied from
<https://html.spec.whatwg.org/entities.json>. It is licensed under the [BSD
3-Clause License](entities.json-LICENSE).

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[docs.rs]: https://docs.rs/htmlize/latest/htmlize/
[crates.io]: https://crates.io/crates/htmlize
[`escape_text()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.escape_text.html
[`escape_text_bytes()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.escape_text_bytes.html
[`escape_attribute()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.escape_attribute.html
[`escape_attribute_bytes()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.escape_attribute_bytes.html
[`escape_all_quotes()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.escape_all_quotes.html
[`escape_all_quotes_bytes()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.escape_all_quotes_bytes.html
[`unescape()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.unescape.html
[`unescape_attribute()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.unescape_attribute.html
[`unescape_in()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.unescape_in.html
[`unescape_bytes_in()`]: https://docs.rs/htmlize/1.1.0/htmlize/fn.unescape_bytes_in.html
[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[official WHATWG spec]: https://html.spec.whatwg.org/multipage/parsing.html#character-reference-state
[phf]: https://crates.io/crates/phf
[features]: https://docs.rs/htmlize/1.1.0/htmlize/index.html#features
[iai]: https://crates.io/crates/iai
[criterion]: https://crates.io/crates/criterion
[`cargo criterion`]: https://crates.io/crates/cargo-criterion
[issues]: https://github.com/danielparks/htmlize/issues
