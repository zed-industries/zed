//-
// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::convert::TryFrom;
#[cfg(not(target_arch = "wasm32"))]
use core::num::{NonZeroI128, NonZeroU128};
use core::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::arbitrary::{any, Arbitrary, StrategyFor};
use crate::strategy::{FilterMap, Strategy};

macro_rules! non_zero_impl {
    ($nz:ty, $prim:ty) => {
        impl Arbitrary for $nz {
            type Parameters = ();
            type Strategy =
                FilterMap<StrategyFor<$prim>, fn($prim) -> Option<Self>>;

            fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
                any::<$prim>().prop_filter_map("must be non zero", |i| {
                    Self::try_from(i).ok()
                })
            }
        }
    };
}

non_zero_impl!(NonZeroU8, u8);
non_zero_impl!(NonZeroU16, u16);
non_zero_impl!(NonZeroU32, u32);
non_zero_impl!(NonZeroU64, u64);
#[cfg(not(target_arch = "wasm32"))]
non_zero_impl!(NonZeroU128, u128);
non_zero_impl!(NonZeroUsize, usize);

non_zero_impl!(NonZeroI8, i8);
non_zero_impl!(NonZeroI16, i16);
non_zero_impl!(NonZeroI32, i32);
non_zero_impl!(NonZeroI64, i64);
#[cfg(not(target_arch = "wasm32"))]
non_zero_impl!(NonZeroI128, i128);
non_zero_impl!(NonZeroIsize, isize);

#[cfg(test)]
mod test {
    no_panic_test!(
        u8 => core::num::NonZeroU8,
        u16 => core::num::NonZeroU16,
        u32 => core::num::NonZeroU32,
        u64 => core::num::NonZeroU64,
        usize => core::num::NonZeroUsize,
        i8 => core::num::NonZeroI8,
        i16 => core::num::NonZeroI16,
        i32 => core::num::NonZeroI32,
        i64 => core::num::NonZeroI64,
        isize => core::num::NonZeroIsize
    );
    #[cfg(not(target_arch = "wasm32"))]
    no_panic_test!(
        u128 => core::num::NonZeroU128,
        i128 => core::num::NonZeroI128
    );
}
