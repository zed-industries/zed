# Unarray

[![Docs badge]][docs.rs]
[![crates badge]][crates.io]
[![github badge]][github]
[![license badge]][github]

Utilities for working with uninitialized arrays

 - No dependencies
 - `#[no_std]`
 - No panics (all APIs return `Result` or `Option`)

This crate provides a few sets of APIs:

### `uninit_buf` and `mark_initialized`

These are a pair of functions which are generally used as follows:
 - stack-allocate an uninitialized array with `uninit_buf`
 - initialize each element
 - unsafely convert it to an initialized array with `mark_initialized`

For example:
```rust
use unarray::*;

fn main() {
  let mut buffer = uninit_buf::<i32; 10>();

  for slot in &mut buffer {
    slot.write(123);
  }

  let array = unsafe { mark_initialized(buffer) };

  assert_eq!(array, [123; 10]);
}
```

This is simple to understand, but still requires `unsafe`, which is hard to justify in many cases

### `build_array_*`

Functions to build arrays from a length and a function that maps from index -> value:
```rust
let even_numbers = build_array(|i| i * 2);  // const generic length parameter inferred
assert_eq!(even_numbers, [0, 2, 4]);

let numbers = build_array_option::<usize, 3>(|i| 3.checked_sub(i));
assert_eq!(numbers, Some([3, 2, 1]));

let numbers = build_array_option::<usize, 5>(|i| 3.checked_sub(i));
assert_eq!(numbers, None);  // since a single element failed, the whole operation failed
```
There is also an equivalent `build_array_result` for `Result`-returning functions

### Collecting iterators to arrays

It's fairly common to want to collect an iterator into an array, but this is currently tricky in 
stable Rust, since iterators don't carry compile-time information about their length. Because of this,
arrays don't implement `FromIterator`, which is required for `.collect()` to work.

Instead, this library provides `ArrayFromIter`, which **does** implement `FromIterator`. This struct can
be destructured to get an `Option<[T, N]>`. If the iterator contained exactly `N` elements, this is `Some(array)`, otherwise, it is `None`:
```rust
let iter = [1, 2, 3].into_iter();
match iter.collect() {
  ArrayFromIter(Some([a, b, c])) => println!("exactly 3 elements: {a}, {b}, {c}"),
  ArrayFromIter(None) => println!("not 3 elements"),
}
```

### `UnarrayArrayExt` extension trait

```rust
// mapping an array via a `Result`
let strings = ["123", "234"];
let numbers = strings.map_result(|s| s.parse());
assert_eq!(numbers, Ok([123, 234]));

let bad_strings = ["123", "uh oh"];
let result = bad_strings.map_result(|s| s.parse::<i32>());
assert!(result.is_err());  // since one of the element fails, the whole operation fails
```
There is also `map_option` for functions which return an `Option`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



[Docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-green
[docs.rs]: https://docs.rs/unarray/
[crates badge]: https://img.shields.io/crates/v/unarray
[crates.io]: https://crates.io/crates/unarray
[license badge]: https://img.shields.io/crates/l/unarray
[github badge]: https://img.shields.io/github/checks-status/cameron1024/unarray/master
[github]: https://github.com/cameron1024/unarray
