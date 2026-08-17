# `block2`

[![Latest version](https://badgen.net/crates/v/block2)](https://crates.io/crates/block2)
[![License](https://badgen.net/badge/license/MIT/blue)](https://github.com/madsmtm/objc2/blob/master/LICENSE.txt)
[![Documentation](https://docs.rs/block2/badge.svg)](https://docs.rs/block2/)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

Apple's C language extension of blocks in Rust.

This crate provides functionality for interracting with C blocks, which is the
C-equivalent of Rust's closures.

They are _technically_ not limited to only being used in Objective-C, though
in practice it's likely the only place you'll ever encounter them.

See [the docs](https://docs.rs/block2/) for a more thorough overview.

This crate is part of the [`objc2` project](https://github.com/madsmtm/objc2),
see that for related crates.
