# Changelog

## circular-buffer 1.2.0

* Updated Rust edition from 2021 to 2024
* Updated MSRV from 0.82 to 0.87
* Removed polyfill code for features that have been stabilized in Rust 0.87

## circular-buffer 1.1.0

### New features

* Added `nth_front()` and `nth_back()`.

## circular-buffer 1.0.0

### New features

* When the buffer is full, `push_front()` and `push_back()` now return the
  replaced element ([contributed by Zacchary
  Dempsey-Plante](https://github.com/andreacorbellini/rust-circular-buffer/pull/3)).

### Other changes

* Increased the minimum supported rustc version to 1.82

## circular-buffer 0.1.9

* This release does not introduce any new features or bug fixes. It only
  changes the dev-dependencies used by unit tests ([contributed by Ben
  Beasley](https://github.com/andreacorbellini/rust-circular-buffer/pull/16)).

## circular-buffer 0.1.8

### New features

* Added optional support for
  [`embedded-io`](https://crates.io/crates/embedded-io) and
  [`embedded-io-async`](https://crates.io/crates/embedded-io-async)
  ([contributed by
  DaneSlattery](https://github.com/andreacorbellini/rust-circular-buffer/pull/15)).

### Other changes

* Renamed the `use_std` cargo feature to `std` (the old `use_std` is now an
  alias for `std`, so this is not a breaking change).

## circular-buffer 0.1.7

### New features

* Implemented the traits [`Index<usize>`](https://doc.rust-lang.org/std/ops/trait.Index.html) and
  [`IndexMut<usize>`](https://doc.rust-lang.org/std/ops/trait.IndexMut.html) for `CircularBuffer`. Now elements of the
  buffer can be accessed and modified with indexing operations (`buf[index]`), like in the following example:

  ```rust
  use circular_buffer::CircularBuffer;
  let mut buf = CircularBuffer::<5, char>::from(['a', 'b', 'c']);
  assert_eq!(buf[0], 'a');
  buf[1] = 'd';
  assert_eq!(buf[1], 'd');
  ```

* Added new methods to fill the whole buffer, or the spare capacity of the buffer:
  [`fill()`](https://docs.rs/circular-buffer/0.1.7/circular_buffer/struct.CircularBuffer.html#method.fill),
  [`fill_with()`](https://docs.rs/circular-buffer/0.1.7/circular_buffer/struct.CircularBuffer.html#method.fill_with),
  [`fill_spare()`](https://docs.rs/circular-buffer/0.1.7/circular_buffer/struct.CircularBuffer.html#method.fill_spare),
  [`fill_spare_with()`](https://docs.rs/circular-buffer/0.1.7/circular_buffer/struct.CircularBuffer.html#method.fill_spare_with).

* Added a new `alloc` feature that brings heap-allocation features to `no_std` environments through the
  [`alloc`](https://doc.rust-lang.org/stable/alloc/) crate ([contributed by
  Haoud](https://github.com/andreacorbellini/rust-circular-buffer/pull/11)).

* Implemented the [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) trait for `CircularBuffer` (not
  available in `no_std` environments).

### Bug fixes

* Fixed an out-of-bounds read in
  [`remove()`](https://docs.rs/circular-buffer/0.1.7/circular_buffer/struct.CircularBuffer.html#method.remove).

* Removed `#[must_use]` from
  [`drain()`](https://docs.rs/circular-buffer/0.1.7/circular_buffer/struct.CircularBuffer.html#method.drain): it is
  perfectly acceptable to ignore the return value from this method.

### Other changes

* Raised the minimum rustc version to 1.65

## circular-buffer 0.1.6

* Fixed a bug in bug in bug in the [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) implementation
  that would lead to a panic in some circumstances.

## circular-buffer 0.1.5

* Added
  [`try_push_back()`](https://docs.rs/circular-buffer/0.1.5/circular_buffer/struct.CircularBuffer.html#method.try_push_back)
  and
  [`try_push_front()`](https://docs.rs/circular-buffer/0.1.5/circular_buffer/struct.CircularBuffer.html#method.try_push_front)
  as non-overwriting alternatives to
  [`push_back()`](https://docs.rs/circular-buffer/0.1.5/circular_buffer/struct.CircularBuffer.html#method.push_back) and
  [`push_front()`](https://docs.rs/circular-buffer/0.1.5/circular_buffer/struct.CircularBuffer.html#method.push_front)
  ([contributed by Rinat Shigapov in GH-5](https://github.com/andreacorbellini/rust-circular-buffer/pull/5)).

* Added [`drain()`](https://docs.rs/circular-buffer/0.1.5/circular_buffer/struct.CircularBuffer.html#method.drain) to
  remove ranges of elements.

* Added
  [`make_contiguous()`](https://docs.rs/circular-buffer/0.1.5/circular_buffer/struct.CircularBuffer.html#method.make_contiguous)
  to return a contiguous mutable slice of elements.

* [`Iter`](https://docs.rs/circular-buffer/0.1.5/circular_buffer/struct.Iter.html) and
  [`IterMut`](https://docs.rs/circular-buffer/0.1.5/circular_buffer/struct.IterMut.html) now implement the
  [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) trait.

## circular-buffer 0.1.4

* Fixed a bug in
  [`range()`](https://docs.rs/circular-buffer/0.1.4/circular_buffer/struct.CircularBuffer.html#method.range) and
  [`range_mut()`](https://docs.rs/circular-buffer/0.1.4/circular_buffer/struct.CircularBuffer.html#method.range_mut)
  that made them return more elements than requested in some circumstances.

## circular-buffer 0.1.3

* Fixed [`range()`](https://docs.rs/circular-buffer/0.1.3/circular_buffer/struct.CircularBuffer.html#method.range) and
  [`range_mut()`](https://docs.rs/circular-buffer/0.1.3/circular_buffer/struct.CircularBuffer.html#method.range_mut)
  when passing an empty range ([contributed by Icxolu in
  GH-4](https://github.com/andreacorbellini/rust-circular-buffer/pull/4)).

## circular-buffer 0.1.2

* Made
  [`extend_from_slice()`](https://docs.rs/circular-buffer/0.1.2/circular_buffer/struct.CircularBuffer.html#method.extend_from_slice)
  safer by ensuring that all cloned elements get dropped in case a panic occurs.

* Optimized all [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) implementations.

* Fixed a [strict-provenance](https://github.com/rust-lang/rust/issues/95228) error in
  [`swap()`](https://docs.rs/circular-buffer/0.1.2/circular_buffer/struct.CircularBuffer.html#method.swap) ([contributed
  by René Kijewski in GH-2](https://github.com/andreacorbellini/rust-circular-buffer/pull/2)).

## circular-buffer 0.1.1

* Made circular-buffer compatible with the stable version of rustc.

## circular-buffer 0.1.0

* Initial release.
