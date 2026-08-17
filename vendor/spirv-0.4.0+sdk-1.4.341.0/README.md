spirv-headers of the rspirv project
===================================

[![Crate][img-crate-headers]][crate-headers]
[![Documentation][img-doc-headers]][doc-headers]

The headers crate for the rspirv project which provides Rust definitions of
SPIR-V structs, enums, and constants.

Usage
-----

This project uses associated constants, which became available in the stable channel
since [1.20][rust-1.20]. So to compile with a compiler from the stable channel,
please make sure that the version is >= 1.20.

First add to your `Cargo.toml`:

```toml
[dependencies]
spirv = "0.4.0"
```

Examples
--------

Please see the [documentation][doc-headers] and project's
[README][project-readme] for examples.

[img-crate-headers]: https://img.shields.io/crates/v/spirv.svg
[img-doc-headers]: https://docs.rs/spirv/badge.svg
[crate-headers]: https://crates.io/crates/spirv
[doc-headers]: https://docs.rs/spirv
[project-readme]: https://github.com/gfx-rs/rspirv/blob/master/README.md
[rust-1.20]: https://blog.rust-lang.org/2017/08/31/Rust-1.20.html
