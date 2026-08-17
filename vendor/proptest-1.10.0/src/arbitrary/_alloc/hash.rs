//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::hash`.

#[cfg(feature = "std")]
use crate::std_facade::hash_map::{DefaultHasher, RandomState};
use core::hash::{BuildHasherDefault, Hasher};

// NOTE: don't impl for std::hash::SipHasher.. since deprecated!

// over-constrain on purpose!
arbitrary!([H: Default + Hasher] BuildHasherDefault<H>; Default::default());

#[cfg(feature = "std")]
lazy_just!(DefaultHasher, Default::default; RandomState, Default::default);

#[cfg(test)]
mod test {
    #[cfg(feature = "std")]
    no_panic_test!(
        default_hasher => DefaultHasher,
        random_state => RandomState,
        build_hasher_default => BuildHasherDefault<DefaultHasher>
    );
}
