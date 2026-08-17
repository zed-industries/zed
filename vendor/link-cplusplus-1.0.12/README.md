`-lstdc++` or `-lc++`
=====================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/link--cplusplus-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/link-cplusplus)
[<img alt="crates.io" src="https://img.shields.io/crates/v/link-cplusplus.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/link-cplusplus)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-link--cplusplus-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/link-cplusplus)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/link-cplusplus/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/link-cplusplus/actions?query=branch%3Amaster)

This crate exists for the purpose of passing `-lstdc++` or `-lc++` to the
linker, while making it possible for an application to make that choice on
behalf of its library dependencies.

Without this crate, a library would need to:

- pick one or the other to link, with no way for downstream applications to
  override the choice;
- or link neither and require an explicit link flag provided by downstream
  applications even if they would be fine with a default choice;

neither of which are good experiences.

<br>

## Options

An application or library that is fine with either of libstdc++ or libc++ being
linked, whichever is the platform's default, should use the following in
Cargo.toml:

```toml
[dependencies]
link-cplusplus = "1"
```

An application that wants a particular one or the other linked should use:

```toml
[dependencies]
link-cplusplus = { version = "1", features = ["libstdc++"] }

# or

link-cplusplus = { version = "1", features = ["libc++"] }
```

An application that wants to handle its own more complicated logic for link
flags from its build script can make this crate do nothing by using:

```toml
[dependencies]
link-cplusplus = { version = "1", features = ["nothing"] }
```

Lastly, make sure to add an explicit `extern crate` dependency to your crate
root, since the link-cplusplus crate will be otherwise unused and its link flags
dropped.

```rust
// src/lib.rs

extern crate link_cplusplus;
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
