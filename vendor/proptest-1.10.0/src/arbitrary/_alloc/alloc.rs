//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::hash`.

use core::cmp;
use core::ops::Range;
use core::usize;

multiplex_alloc!(::alloc::alloc, ::std::alloc);

use crate::arbitrary::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

arbitrary!(self::alloc::Global; self::alloc::Global);

// Not Debug.
//lazy_just!(System, || System);

arbitrary!(self::alloc::Layout, SFnPtrMap<(Range<u8>, StrategyFor<usize>), Self>;
    // 1. align must be a power of two and <= (1 << 31):
    // 2. "when rounded up to the nearest multiple of align, must not overflow".
    static_map((0u8..32u8, any::<usize>()), |(align_power, size)| {
        let align = 1usize << align_power;
        // TODO: This may only work on 64 bit processors, but previously it was broken
        // even on 64 bit so still an improvement. 63 -> uint size - 1.
        let max_size = (1usize << (usize::BITS - 1)) - (1 << usize::from(align_power));
        // Not quite a uniform distribution due to clamping,
        // but probably good enough
        self::alloc::Layout::from_size_align(cmp::min(max_size, size), align).unwrap()
    })
);

arbitrary!(self::alloc::AllocError, Just<Self>; Just(self::alloc::AllocError));
/* 2018-07-28 CollectionAllocErr is not currently available outside of using
 * the `alloc` crate, which would require a different nightly feature. For now,
 * disable.
arbitrary!(alloc::collections::CollectionAllocErr, TupleUnion<(WA<Just<Self>>, WA<Just<Self>>)>;
           prop_oneof![Just(alloc::collections::CollectionAllocErr::AllocErr),
                       Just(alloc::collections::CollectionAllocErr::CapacityOverflow)]);
 */

#[cfg(test)]
mod test {
    multiplex_alloc!(::alloc::alloc, ::std::alloc);

    no_panic_test!(
        layout => self::alloc::Layout,
        alloc_err => self::alloc::AllocError
        //collection_alloc_err => alloc::collections::CollectionAllocErr
    );
}
