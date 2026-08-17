/*!
[`RangeMap`] and [`RangeInclusiveMap`] are map data structures whose keys
are stored as ranges. Contiguous and overlapping ranges that map to the same
value are coalesced into a single range.

Corresponding [`RangeSet`] and [`RangeInclusiveSet`] structures are also provided.


# Different kinds of ranges

`RangeMap` and `RangeInclusiveMap` correspond to the [`Range`]
and [`RangeInclusive`] types from the standard library respectively.
For some applications the choice of range type may be obvious,
or even be dictated by pre-existing design decisions. For other applications
the choice may seem arbitrary, and be guided instead by convenience or
aesthetic preference.

If the choice is not obvious in your case, consider these differences:

- If your key type `K` represents points on a continuum (e.g. `f64`),
  and the choice of which of two adjacent ranges "owns" the value
  where they touch is largely arbitrary, then it may be more natural
  to work with half-open `Range`s like `0.0..1.0` and `1.0..2.0`. If you
  were to use closed `RangeInclusive`s here instead, then to represent two such adjacent
  ranges you would need to subtract some infinitesimal (which may depend,
  as it does in the case of `f64`, on the specific value of `K`)
  from the end of the earlier range. (See the last point below for more
  on this problem.)
- If you need to represent ranges that _include_ the maximum
  value in the key domain (e.g. `255u8`) then you will
  probably want to use `RangeInclusive`s like `128u8..=255u8`. Sometimes
  it may be possible to instead work around this by using a wider key
  type than the values you are actually trying to represent (`K=u16`
  even though you are only trying to represent ranges covering `u8`)
  but in these cases the key domain often represents discrete objects
  rather than points on a continuum, and so `RangeInclusive` may
  be a more natural way to express these ranges anyway.
- If you are using `RangeInclusive`, then it must be possible to define
  _successor_ and _predecessor_ functions for your key type `K`,
  because adjacent ranges can not be detected (and thereby coalesced)
  simply by testing their ends for equality. For key types that represent
  points on a continuum, defining these functions may be awkward and error-prone.
  For key types that represent discrete objects, this is usually much
  more straightforward.


# Example: use with Chrono

```rust
use chrono::offset::TimeZone;
use chrono::{Duration, Utc};
use rangemap::RangeMap;

let people = ["Alice", "Bob", "Carol"];
let mut roster = RangeMap::new();

// Set up initial roster.
let start_of_roster = Utc.ymd(2019, 1, 7);
let mut week_start = start_of_roster;
for _ in 0..3 {
    for person in &people {
        let next_week = week_start + Duration::weeks(1);
        roster.insert(week_start..next_week, person);
        week_start = next_week;
    }
}

// Bob is covering Alice's second shift (the fourth shift overall).
let fourth_shift_start = start_of_roster + Duration::weeks(3);
let fourth_shift_end = fourth_shift_start + Duration::weeks(1);
roster.insert(fourth_shift_start..fourth_shift_end, &"Bob");

for (range, person) in roster.iter() {
    println!("{} ({}): {}", range.start, range.end - range.start, person);
}

// Output:
// 2019-01-07UTC (P7D): Alice
// 2019-01-14UTC (P7D): Bob
// 2019-01-21UTC (P7D): Carol
// 2019-01-28UTC (P14D): Bob
// 2019-02-11UTC (P7D): Carol
// 2019-02-18UTC (P7D): Alice
// 2019-02-25UTC (P7D): Bob
// 2019-03-04UTC (P7D): Carol
```


## Crate features

By default this crate has no dependencies on other crates.

If you enable the **serde1** feature it will introduce a dependency on
the _serde_ crate and provide `Serialize` and `Deserialize`
implementations for all map and set types in this crate.

You can enable the **serde1** feature in your _Cargo.toml_ file like so:

```toml
[dependencies]
rangemap = { version = "1", features = ["serde1"] }
```

You can similarly enable support for _quickcheck_ by enabling
the **quickcheck** feature.


## Building without the Rust standard library

This crate can work without the full standard library available
(e.g. when running on bare metal without an operating system)
but relies on the presence of a global allocator &mdash;
i.e. it links the `core` and `alloc` crates, but not `std`.

Presently there is no functionality in the crate that require
the standard library. Such functionality will likely be
introduced in the future, and will be gated behind a default-on
`std` feature.

See [The Rust Programming Language](https://doc.rust-lang.org/1.7.0/book/no-stdlib.html)
book for general information about operating without the standard library.



[`RangeMap`]: crate::RangeMap
[`RangeInclusiveMap`]: crate::RangeInclusiveMap
[`RangeSet`]: crate::RangeSet
[`RangeInclusiveSet`]: crate::RangeInclusiveSet
[`Range`]: core::ops::Range
[`RangeInclusive`]: core::ops::RangeInclusive

*/

#![no_std]
extern crate alloc;

pub mod inclusive_map;
pub mod inclusive_set;
pub mod map;
pub(crate) mod operations;
pub mod set;

#[cfg(test)]
mod dense;
mod range_wrapper;
mod std_ext;

pub use inclusive_map::RangeInclusiveMap;
pub use inclusive_set::RangeInclusiveSet;
pub use map::RangeMap;
pub use set::RangeSet;
pub use std_ext::{StepFns, StepLite};

// Doc tests for README.
#[cfg(feature = "nightly")]
mod readme;
