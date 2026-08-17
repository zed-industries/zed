<!-- markdownlint-disable no-inline-html first-line-heading -->

<div align="center">

# `⚙️ cfg-expr`

**A parser and evaluator for Rust `cfg()` expressions. Builtin targets as of [1.93.0] are supported.**

[![Build Status](https://github.com/EmbarkStudios/cfg-expr/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/cfg-expr/actions?workflow=CI)
[![Crates.io](https://img.shields.io/crates/v/cfg-expr.svg)](https://crates.io/crates/cfg-expr)
[![Docs](https://docs.rs/cfg-expr/badge.svg)](https://docs.rs/cfg-expr)
[![Minimum Stable Rust Version](https://img.shields.io/badge/Rust%20MSRV-1.70.0-blue?color=fc8d62&logo=rust)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)
[![Rust Targets](https://img.shields.io/badge/Rust%20Targets-1.93.0-blue.svg)](https://forge.rust-lang.org/release/platform-support.html)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v2.0%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
</div>

## Alternatives

- [cargo-platform](https://crates.io/crates/cargo-platform)
- [parse_cfg](https://crates.io/crates/parse_cfg)

## Usage

`cfg-expr` is a crate that can be used to parse and evaluate Rust `cfg()` expressions, both as declarable in Rust code itself, as well in cargo manifests' `[target.'cfg()'.dependencies]` sections.

It contains a list of all builtin targets known to rustc as of [1.93.0] that can be used to determine if a particular cfg expression is satisfiable.

```rust
use cfg_expr::{targets::get_builtin_target_by_triple, Expression, Predicate};

let specific = Expression::parse(
    r#"all(
        target_os = "windows",
        target_arch = "x86",
        windows,
        target_env = "msvc",
        target_feature = "fxsr",
        target_feature = "sse",
        target_feature = "sse2",
        target_pointer_width = "32",
        target_endian = "little",
        not(target_vendor = "uwp"),
        feature = "cool_thing",
    )"#,
).unwrap();

// cfg_expr includes a list of every builtin target in rustc
let x86_win = get_builtin_target_by_triple("i686-pc-windows-msvc").unwrap();
let uwp_win = get_builtin_target_by_triple("i686-uwp-windows-msvc").unwrap();
let mac = get_builtin_target_by_triple("x86_64-apple-darwin").unwrap();

let avail_target_feats = ["fxsr", "sse", "sse2"];

// This will satisfy all requirements
assert!(specific.eval(|pred| {
    match pred {
        Predicate::Target(tp) => tp.matches(x86_win),
        Predicate::TargetFeature(feat) => avail_target_feats.contains(feat),
        Predicate::Feature(feat) => *feat == "cool_thing",
        _ => false,
    }
}));

// This will *not* satisfy the vendor predicate
assert!(!specific.eval(|pred| {
    match pred {
        Predicate::Target(tp) => tp.matches(uwp_win),
        Predicate::TargetFeature(feat) => avail_target_feats.contains(feat),
        _ => false,
    }
}));

// This will *not* satisfy the vendor, os, or env predicates
assert!(!specific.eval(|pred| {
    match pred {
        Predicate::Target(tp) => tp.matches(mac),
        Predicate::TargetFeature(feat) => avail_target_feats.contains(feat),
        _ => false,
    }
}));
```

## Contributing

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[1.93.0]: (https://forge.rust-lang.org/release/platform-support.html)
