//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::sync`.

use crate::std_facade::Arc;
use core::sync::atomic::*;

use crate::arbitrary::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

wrap_from!(Arc);

macro_rules! atomic {
    ($($type: ident, $base: ty);+) => {
        $(arbitrary!($type, SMapped<$base, Self>;
            static_map(any::<$base>(), $type::new)
        );)+
    };
}

// impl_wrap_gen!(AtomicPtr); // We don't have impl Arbitrary for *mut T yet.
atomic!(AtomicBool, bool; AtomicIsize, isize; AtomicUsize, usize);

#[cfg(feature = "unstable")]
atomic!(AtomicI8, i8; AtomicI16, i16; AtomicI32, i32;
        AtomicU8, u8; AtomicU16, u16; AtomicU32, u32);

#[cfg(all(feature = "unstable", feature = "atomic64bit"))]
atomic!(AtomicI64, i64; AtomicU64, u64);

arbitrary!(Ordering,
    TupleUnion<(WA<Just<Self>>, WA<Just<Self>>, WA<Just<Self>>,
                WA<Just<Self>>, WA<Just<Self>>)>;
    prop_oneof![
        Just(Ordering::Relaxed),
        Just(Ordering::Release),
        Just(Ordering::Acquire),
        Just(Ordering::AcqRel),
        Just(Ordering::SeqCst)
    ]
);

#[cfg(test)]
mod test {
    no_panic_test!(
        arc => Arc<u8>,
        atomic_bool => AtomicBool,
        atomic_isize => AtomicIsize,
        atomic_usize => AtomicUsize,
        ordering => Ordering
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        atomic_i8  => AtomicI8,
        atomic_i16 => AtomicI16,
        atomic_i32 => AtomicI32,
        atomic_u8  => AtomicU8,
        atomic_u16 => AtomicU16,
        atomic_u32 => AtomicU32
    );

    #[cfg(all(feature = "unstable", feature = "atomic64bit"))]
    no_panic_test!(
        atomic_i64 => AtomicI64,
        atomic_u64 => AtomicU64
    );
}
