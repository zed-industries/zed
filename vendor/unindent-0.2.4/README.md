# Unindent

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/indoc-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/indoc)
[<img alt="crates.io" src="https://img.shields.io/crates/v/unindent.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/unindent)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-unindent-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/unindent)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/indoc/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/indoc/actions?query=branch%3Amaster)

This crate provides [`indoc`]'s indentation logic for use with strings that are
not statically known at compile time. For unindenting string literals, use
`indoc` instead.

[`indoc`]: https://github.com/dtolnay/indoc

This crate exposes two functions:

- `unindent(&str) -> String`
- `unindent_bytes(&[u8]) -> Vec<u8>`

```rust
use unindent::unindent;

fn main() {
    let indented = "
            line one
            line two";
    assert_eq!("line one\nline two", unindent(indented));
}
```

## Explanation

The following rules characterize the behavior of unindent:

1. Count the leading spaces of each line, ignoring the first line and any lines
   that are empty or contain spaces only.
2. Take the minimum.
3. If the first line is empty i.e. the string begins with a newline, remove the
   first line.
4. Remove the computed number of spaces from the beginning of each line.

This means there are a few equivalent ways to format the same string, so choose
one you like. All of the following result in the string `"line one\nline
two\n"`:

```
unindent("          /      unindent(           /      unindent("line one
   line one        /         "line one        /                 line two
   line two       /           line two       /                  ")
   ")            /            ")            /
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Indoc by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
