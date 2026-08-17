unsafe-libyaml
==============

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/unsafe--libyaml-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/unsafe-libyaml)
[<img alt="crates.io" src="https://img.shields.io/crates/v/unsafe-libyaml.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/unsafe-libyaml)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-unsafe--libyaml-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/unsafe-libyaml)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/unsafe-libyaml/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/unsafe-libyaml/actions?query=branch%3Amaster)

This library is [libyaml] translated from C to unsafe Rust with the assistance
of [c2rust].

[libyaml]: https://github.com/yaml/libyaml/tree/2c891fc7a770e8ba2fec34fc6b545c672beb37e6
[c2rust]: https://github.com/immunant/c2rust

```toml
[dependencies]
unsafe-libyaml = "0.2"
```

*Compiler support: requires rustc 1.56+*

## License

<a href="LICENSE-MIT">MIT license</a>, same as libyaml.
