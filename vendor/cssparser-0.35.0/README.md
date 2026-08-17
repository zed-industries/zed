rust-cssparser
==============

[![Build Status](https://github.com/servo/rust-cssparser/actions/workflows/main.yml/badge.svg)](https://github.com/servo/rust-cssparser/actions)

[Documentation](https://docs.rs/cssparser)

Rust implementation of
[CSS Syntax Module Level 3](https://drafts.csswg.org/css-syntax/)


Overview
--------

Parsing CSS involves a series of steps:

* When parsing from bytes,
  (e.g. reading a file or fetching an URL from the network,)
  detect the character encoding
  (based on a `Content-Type` HTTP header, an `@charset` rule, a BOM, etc.)
  and decode to Unicode text.

  rust-cssparser does not do this yet and just assumes UTF-8.

  This step is skipped when parsing from Unicode, e.g. in an HTML `<style>` element.

* Tokenization, a.k.a. lexing.
  The input, a stream of Unicode text, is transformed into a stream of *tokens*.
  Tokenization never fails, although the output may contain *error tokens*.

* This flat stream of tokens is then transformed into a tree of *component values*,
  which are either *preserved tokens*,
  or blocks/functions (`{ … }`, `[ … ]`, `( … )`, `foo( … )`)
  that contain more component values.

  rust-cssparser does this at the same time as tokenization:
  raw tokens are never materialized, you only get component values.

* Component values can then be parsed into generic rules or declarations.
  The header and body of rules as well as the value of declarations
  are still just lists of component values at this point.
  See [the `Token` enum](src/tokenizer.rs) for the data structure.

* The last step of a full CSS parser is
  parsing the remaining component values
  into [Selectors](https://drafts.csswg.org/selectors/),
  specific CSS properties, etc.

  By design, rust-cssparser does not do this last step
  which depends a lot on what you want to do:
  which properties you want to support, what you want to do with selectors, etc.

  It does however provide some helper functions to parse [CSS colors](src/color.rs)
  and [An+B](src/nth.rs) (the argument to `:nth-child()` and related selectors.

  See [Servo’s `style` crate](https://github.com/servo/stylo/tree/main/style)
  for an example of a parser based on rust-cssparser.
