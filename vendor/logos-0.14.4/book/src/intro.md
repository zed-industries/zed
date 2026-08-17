# Logos Handbook

[![Crates.io version shield](https://img.shields.io/crates/v/logos.svg)](https://crates.io/crates/logos)
[![Docs](https://docs.rs/logos/badge.svg)](https://docs.rs/logos)
[![Crates.io license shield](https://img.shields.io/crates/l/logos.svg)](https://crates.io/crates/logos)

<img src="https://raw.githubusercontent.com/maciejhirsz/logos/master/logos.svg?sanitize=true" alt="Logos logo" width="250" align="right">

Hi there!

**Logos** is a fast and easy to use [lexer](https://en.wikipedia.org/wiki/Lexical_analysis)
generator written in Rust. While Rust has excellent documentation tools (and you can access
the [API docs for Logos at docs.rs](https://docs.rs/logos/)), it's not the easiest thing to
document custom syntax used by procedural macros, of which Logos has a bit. This Handbook
seeks to remedy this!

## In a nut shell

There are two main types in **Logos**:

+ The `Logos` trait, which comes out with it's own derive macro. The derive
  macro uses custom attributes (the things using these brackets: `#[...]`)
  with plain string or [regular expression](https://en.wikipedia.org/wiki/Regular_expression)
  syntax on `enum` variants as _patterns_ for some input.
+ The `Lexer<T: Logos>`, which is an iterator that takes some input (`&str`,
  sometimes `&[u8]`) and performs lexical analysis on the input on the go,
  producing variants of the enum `T` matching the defined patterns.
