rust-selectors
==============

* [![Build Status](https://travis-ci.com/servo/rust-selectors.svg?branch=master)](
  https://travis-ci.com/servo/rust-selectors)
* [Documentation](https://docs.rs/selectors/)
* [crates.io](https://crates.io/crates/selectors)

CSS Selectors library for Rust.
Includes parsing and serilization of selectors,
as well as matching against a generic tree of elements.
Pseudo-elements and most pseudo-classes are generic as well.

**Warning:** breaking changes are made to this library fairly frequently
(13 times in 2016, for example).
However you can use this crate without updating it that often,
old versions stay available on crates.io and Cargo will only automatically update
to versions that are numbered as compatible.

To see how to use this library with your own tree representation,
see [Kuchiki’s `src/select.rs`](https://github.com/kuchiki-rs/kuchiki/blob/master/src/select.rs).
(Note however that Kuchiki is not always up to date with the latest rust-selectors version,
so that code may need to be tweaked.)
If you don’t already have a tree data structure,
consider using [Kuchiki](https://github.com/kuchiki-rs/kuchiki) itself.
