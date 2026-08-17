# CESU-8 encoder/decoder for Rust

[![Build Status](https://travis-ci.org/emk/cesu8-rs.svg)](https://travis-ci.org/emk/cesu8-rs) [![Latest version](https://img.shields.io/crates/v/cesu8.svg)](https://crates.io/crates/cesu8) [![License](https://img.shields.io/crates/l/cesu8.svg)](https://crates.io/crates/cesu8)

[Documentation][apidoc].

[apidoc]: http://emk.github.io/cesu8-rs/cesu8/index.html

Convert between ordinary UTF-8 and [CESU-8][] encodings.

CESU-8 encodes characters outside the Basic Multilingual Plane as two
UTF-16 surrogate chacaters, which are then further re-encoded as invalid,
3-byte UTF-8 characters.  This means that 4-byte UTF-8 sequences become
6-byte CESU-8 sequences.

**Note that CESU-8 is only intended for internal use within tightly-coupled
systems, and not for data interchange.**

This encoding is sometimes needed when working with Java, Oracle or MySQL,
and when trying to store emoji, hieroglyphs, or other characters on the
Supplementary Multilingual Plane or the Supplementary Ideographic Plane.

[CESU-8]: http://www.unicode.org/reports/tr26/tr26-2.html

## License

Some of this code is adapted from Rust's [`src/libcore/str.rs` file][str.rs].
This code is covered by LICENSE-RUST.txt and copyright by The Rust Project
Developers and individual Rust contributors, as described in that file.

The new code in this project is distributed under the same terms.

[str.rs]: https://github.com/rust-lang/rust/blob/master/src/libcore/str.rs
