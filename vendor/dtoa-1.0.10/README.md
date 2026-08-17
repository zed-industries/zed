dtoa
====

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/dtoa-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/dtoa)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dtoa.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dtoa)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dtoa-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/dtoa)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/dtoa/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/dtoa/actions?query=branch%3Amaster)

This crate provides fast conversion of floating point primitives to decimal
strings. The implementation is a straightforward Rust port of [Milo Yip]'s C++
implementation [dtoa.h]. The original C++ code of each function is included in
comments.

See also [`itoa`] for printing integer primitives.

*Version requirement: rustc 1.36+*

[Milo Yip]: https://github.com/miloyip
[dtoa.h]: https://github.com/miloyip/rapidjson/blob/master/include/rapidjson/internal/dtoa.h
[`itoa`]: https://github.com/dtolnay/itoa

```toml
[dependencies]
dtoa = "1.0"
```

<br>

## Example

```rust
fn main() {
    let mut buffer = dtoa::Buffer::new();
    let printed = buffer.format(2.71828f64);
    assert_eq!(printed, "2.71828");
}
```

<br>

## Performance (lower is better)

![performance](https://raw.githubusercontent.com/dtolnay/dtoa/master/performance.png)

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
