fixedbitset
---

A simple fixed size bitset container for Rust.

Please read the [API documentation here](https://docs.rs/fixedbitset/)

[![build\_status](https://github.com/petgraph/fixedbitset/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/petgraph/fixedbitset/actions)
[![crates](https://img.shields.io/crates/v/fixedbitset.svg)](https://crates.io/crates/fixedbitset)

# Recent Changes

-   0.5.7
    - [#127](https://github.com/petgraph/fixedbitset/pull/127) and [#128](https://github.com/petgraph/fixedbitset/pull/128): Optimize `Clone::clone_from` to avoid
      extra allocations and copies.
-   0.5.6
    - Fixed FixedBitset not implementing Send/Sync due to the stack size shrink.
-   0.5.5 (yanked)
    - [#116](https://github.com/petgraph/fixedbitset/pull/116): Add functions for counting the results of a set operation (`union_count`, 
       `intersection_count`, `difference_count`, `symmetric_difference_count`) by @james7132.
    - [#118](https://github.com/petgraph/fixedbitset/pull/118): Shrink the stack size of FixedBitset. There should be zero stack size overhead
      compared to a Vec.
    - [#119](https://github.com/petgraph/fixedbitset/pull/119): Fix builds for wasm32.
    - [#120](https://github.com/petgraph/fixedbitset/pull/119): Add more utility functions that were previously missing from the public interface:
       `contains_any_in_range`, `contains_all_in_range`, `minimum`, `maximum`, `is_full`, `count_zeroes`, and `remove_range`.
    - [#121](https://github.com/petgraph/fixedbitset/pull/121): Add support for SIMD acceleration for AVX builds.
-   0.5.4
    - [#112](https://github.com/petgraph/fixedbitset/pull/112): Fix undefined behavior in IntoOnes and setup testing with MIRI by @SkiFire13
-   0.5.3 (yanked)
    - [#109](https://github.com/petgraph/fixedbitset/pull/109): Fix non-x86(_64) builds by @james7132
-   0.5.2 (yanked)
    - [#86](https://github.com/petgraph/fixedbitset/pull/86): Explicit SIMD vectorization for set operations by @james7132.
-   0.5.1
    - [#102](https://github.com/petgraph/fixedbitset/pull/102): Added `contains_unchecked`, `insert_unchecked`, `put_unchecked`,
      `set_unchecked`, `toggle_unchecked`, `removed_unchecked`, `copy_bit_unchecked` unsafe variants of the safe functions, by @james7132
    - [#103](https://github.com/petgraph/fixedbitset/pull/103): Added `into_ones` which returns a owned iterator over the one
      values from a bitset, by @james7132.
    - [#104](https://github.com/petgraph/fixedbitset/pull/104): Implemented `DoubleEndedIterator` for `Union`, `Intersection`,
      `Difference`, and `SymmetricDifference` , by @james7132.
-   0.5.0
    - [#74](https://github.com/petgraph/fixedbitset/pull/74): Accelerated set operations (union, intersection, difference, 
      symmetric difference) by using larger blocks internally, by @james7132.
    - [#88](https://github.com/petgraph/fixedbitset/pull/88): Added `FixedBitSet::remove` by @james7132.
    - [#89](https://github.com/petgraph/fixedbitset/pull/89): Added `FixedBitSet::zeros`  and the `Zeros` iterator by @james7132.
    - [#92](https://github.com/petgraph/fixedbitset/pull/92): Added `FixedBitSet::grow_and_insert` function, a 
      non-panicking version of `insert` that grows the underlying storage as need, by @shuoli84.
    - [#98](https://github.com/petgraph/fixedbitset/pull/98): `Ones` now implements `DoubleEndedIterator`, by @tikhu.
    - [#99](https://github.com/petgraph/fixedbitset/pull/99): **Breaking change**: serde now serializes and deserializes from a little-endian encoded
      raw byte buffer. Existing stored instances of the serialized bitsets will need to be
      re-encoded.
    - Bumped MSRV to 1.56.
-   0.4.2
    - [#79](https://github.com/petgraph/fixedbitset/pull/79): Add `is_clear`,
    clarify `is_empty` and `len` documentation by \@nicopap.
-   0.4.1
    - Documentation and formatting fixes.
-   0.4.0
    -   [#61](https://github.com/petgraph/fixedbitset/pull/61): Require
        Rust 1.39.
    -   [#60](https://github.com/petgraph/fixedbitset/pull/60): Add
        `const` `FixedBitSet::new` consructor
        by \@jakobhellermann.
    -   [#59](https://github.com/petgraph/fixedbitset/pull/59): Add
        optional `serde` support by \@keshavsn.
-   0.3.2
    -   [#18](https://github.com/petgraph/fixedbitset/pull/18): Optimize
        `ones` using `trailing_zeroes` by \@vks
-   0.3.1
    -   Add bit assign operators for references by \@flaghacker
    -   Improve assertion error messages by \@lovasoa
    -   Add documentation examples for `with_capacity_and_blocks`
-   0.3.0
    -   Add `with_capacity_and_blocks` by \@luizirber
    -   Add `difference_with` by \@sunshowers
    -   Implement `Binary` and `Display` traits by \@Dolphindalt
    -   Add `toggle_range` by \@wirelyre
-   0.2.0
    -   Add assign operators for the bit operations by \@jrraymond
    -   Add `symmetric_difference`, `union_with`, `intersection_with` by
        \@jrraymond
    -   Add `is_subset`, `is_superset`, `is_disjoint` by \@nwn
    -   Add `.toggle(i)` method by \@ShiroUsagi-san
    -   Add default feature \"std\" which can be disabled to make the
        crate not link the std library. By \@jonimake and \@bluss
    -   Require Rust 1.31.
-   0.1.9
    -   Add intersection, union, difference iterators by \@jrraymond
    -   Add intersection: `&` and union: `|` operator implementations by
        \@jrraymond
    -   Add Extend and FromIterator implementations (from sequences of
        bit indices) by \@jrraymond
-   0.1.8
    -   Add missing `#[inline]` on the ones iterator
    -   Fix docs for `insert_range, set_range`
-   0.1.7
    -   Add fast methods `.insert_range`, `.set_range` by \@kennytm
-   0.1.6
    -   Add iterator `.ones()` by \@mneumann
    -   Fix bug with `.count_ones()` where it would erronously have an
        out-of-bounds panic for even block endpoints
-   0.1.5
    -   Add method `.count_ones(range)`.
-   0.1.4
    -   Remove an assertion in `.copy_bit(from, to)` so that it is in
        line with the documentation. The `from` bit does not need to be
        in bounds.
    -   Improve `.grow()` to use `Vec::resize` internally.
-   0.1.3
    -   Add method `.put()` to enable a bit and return previous value
-   0.1.2
    -   Add method `.copy_bit()` (by fuine)
    -   impl Default
-   0.1.1
    -   Update documentation URL
-   0.1.0
    -   Add method `.grow()`

# License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
 or the [MIT license](https://opensource.org/licenses/MIT),
 at your option. This file may not be copied, modified, or distributed except
according to those terms.
