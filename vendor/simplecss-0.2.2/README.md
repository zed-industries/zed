<div align="center">

# SimpleCSS

**A simple [CSS 2.1](https://www.w3.org/TR/CSS21/) parser and selector.**

[![Linebender Zulip, #resvg channel](https://img.shields.io/badge/Linebender-%23resvg-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/465085-resvg)
[![dependency status](https://deps.rs/repo/github/linebender/simplecss/status.svg)](https://deps.rs/repo/github/linebender/simplecss)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)
[![Build status](https://github.com/linebender/simplecss/workflows/CI/badge.svg)](https://github.com/linebender/simplecss/actions)
[![Crates.io](https://img.shields.io/crates/v/simplecss.svg)](https://crates.io/crates/simplecss)
[![Docs](https://docs.rs/simplecss/badge.svg)](https://docs.rs/simplecss)
![](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)

</div>

This is not a browser-grade CSS parser.
If you need one, use [cssparser](https://crates.io/crates/cssparser) + [selectors](https://crates.io/crates/selectors).

Since it's very simple we will start with limitations:

## Limitations

- [At-rules](https://www.w3.org/TR/CSS21/syndata.html#at-rules) are not supported.
  They will be skipped during parsing.
- Property values are not parsed.
  In CSS like `* { width: 5px }` you will get a `width` property with a `5px` value as a string.
- CDO/CDC comments are not supported.
- Parser is case sensitive.
  All keywords must be lowercase.
- Unicode escape, like `\26`, is not supported.

## Features

- Selector matching support.
- The rules are sorted by specificity.
- `!important` parsing support.
- Has a high-level parsers and low-level, zero-allocation tokenizers.
- No unsafe.

## Minimum supported Rust Version (MSRV)

This version of SimpleCSS has been verified to compile with **Rust 1.65** and later.

Future versions of SimpleCSS might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

<details>
<summary>Click here if compiling fails.</summary>

As time has passed, some of SimpleCSS's dependencies could have released versions with a higher Rust requirement.
If you encounter a compilation issue due to a dependency and don't want to upgrade your Rust toolchain, then you could downgrade the dependency.

```sh
# Use the problematic dependency's name and version
cargo update -p package_name --precise 0.1.1
```
</details>

## Community

[![Linebender Zulip, #resvg channel](https://img.shields.io/badge/Linebender-%23resvg-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/channel/465085-resvg)

Discussion of SimpleCSS development happens in the Linebender Zulip at <https://xi.zulipchat.com/>, specifically the [#resvg channel](https://xi.zulipchat.com/#narrow/channel/465085-resvg).
All public content can be read without logging in.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.
Please feel free to add your name to the [AUTHORS] file in any substantive pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
[AUTHORS]: ./AUTHORS
