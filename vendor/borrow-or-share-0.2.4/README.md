# borrow-or-share

Traits for either borrowing or sharing data.

[![crates.io](https://img.shields.io/crates/v/borrow-or-share.svg)](https://crates.io/crates/borrow-or-share)
[![build](https://img.shields.io/github/actions/workflow/status/yescallop/borrow-or-share/ci.yml
)](https://github.com/yescallop/borrow-or-share/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/borrow-or-share)](/LICENSE)

See below for a basic usage of the crate.
See the [documentation](https://docs.rs/borrow-or-share) for a detailed walkthrough.

## Basic usage

Suppose that you have a generic type that either owns some data or holds a reference to them.
You can use this crate to implement on this type a method taking `&self` that either borrows from `*self`
or from behind a reference it holds:

```rust
use borrow_or_share::BorrowOrShare;

struct Text<T>(T);

impl<'i, 'o, T: BorrowOrShare<'i, 'o, str>> Text<T> {
    fn as_str(&'i self) -> &'o str {
        self.0.borrow_or_share()
    }
}

// The returned reference is borrowed from `*text`
// and lives as long as `text`.
fn borrow(text: &Text<String>) -> &str {
    text.as_str()
}

// The returned reference is borrowed from `*text.0`, lives
// longer than `text` and is said to be shared with `*text`.
fn share<'a>(text: &Text<&'a str>) -> &'a str {
    text.as_str()
}
```

## Credit

Credit goes to [@beepster4096](https://github.com/beepster4096) for figuring out a safe version of the code.
