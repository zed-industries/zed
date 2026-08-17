# `arraydeque`

[![build status](https://travis-ci.org/andylokandy/arraydeque.svg?branch=master)](https://travis-ci.org/andylokandy/arraydeque)
[![crates.io](https://img.shields.io/crates/v/arraydeque.svg)](https://crates.io/crates/arraydeque)
[![docs.rs](https://docs.rs/arraydeque/badge.svg)](https://docs.rs/arraydeque)

A circular buffer with fixed capacity.  Requires Rust 1.59+.

This crate is inspired by [**bluss/arrayvec**](https://github.com/bluss/arrayvec)

### [**Documentation**](https://docs.rs/arraydeque)

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
arraydeque = "0.5"
```

Next, add this to your crate root:

```rust
extern crate arraydeque;
```

Currently arraydeque by default links to the standard library, but if you would
instead like to use arraydeque in a `#![no_std]` situation or crate you can
request this via:

```toml
[dependencies]
arraydeque = { version = "0.4", default-features = false }
```

## Example

```rust
extern crate arraydeque;

use arraydeque::ArrayDeque;

fn main() {
    let mut deque: ArrayDeque<_, 2> = ArrayDeque::new();
    assert_eq!(deque.capacity(), 2);
    assert_eq!(deque.len(), 0);

    deque.push_back(1);
    deque.push_back(2);
    assert_eq!(deque.len(), 2);

    assert_eq!(deque.pop_front(), Some(1));
    assert_eq!(deque.pop_front(), Some(2));
    assert_eq!(deque.pop_front(), None);
}
```

## Changelog

- 0.5.1 Make `ArrayDeque::new()` a const fn.

- 0.5.0 Support consnt generic capacity. Remove `use_generic_array` feature.

- 0.4.5 Update `generic-array` to `0.12`.

- 0.4.4 Fix UB: `Some(ArrayDeque::new(xs)).is_some() == false`. ([#12](https://github.com/andylokandy/arraydeque/issues/12))

- 0.4.3 Add support for `generic-array` under `use_generic_array` feature.

- 0.4.1 Capacity now equal to backend_array.len().

- 0.3.1 Add behaviors: `Saturating` and `Wrapping`.

## Contribution

All kinds of contribution are welcomed.

- **Issues.** Feel free to open an issue when you find typos, bugs, or have any question.
- **Pull requests**. New collection, better implementation, more tests, more documents and typo fixes are all welcomed.

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
