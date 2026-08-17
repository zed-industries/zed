//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), allow(unused_macros))]

//==============================================================================
// Macros for quick implementing:
//==============================================================================

macro_rules! arbitrary {
    ([$($bounds : tt)*] $typ: ty, $strat: ty, $params: ty;
        $args: ident => $logic: expr) => {
        impl<$($bounds)*> $crate::arbitrary::Arbitrary for $typ {
            type Parameters = $params;
            type Strategy = $strat;
            fn arbitrary_with($args: Self::Parameters) -> Self::Strategy {
                $logic
            }
        }
    };
    ([$($bounds : tt)*] $typ: ty, $strat: ty; $logic: expr) => {
        arbitrary!([$($bounds)*] $typ, $strat, (); _args => $logic);
    };
    ([$($bounds : tt)*] $typ: ty; $logic: expr) => {
        arbitrary!([$($bounds)*] $typ,
            $crate::strategy::Just<Self>, ();
            _args => $crate::strategy::Just($logic)
        );
    };
    ($typ: ty, $strat: ty, $params: ty; $args: ident => $logic: expr) => {
        arbitrary!([] $typ, $strat, $params; $args => $logic);
    };
    ($typ: ty, $strat: ty; $logic: expr) => {
        arbitrary!([] $typ, $strat; $logic);
    };
    ($strat: ty; $logic: expr) => {
        arbitrary!([] $strat; $logic);
    };
    ($($typ: ident),*) => {
        $(arbitrary!($typ, $typ::Any; $typ::ANY);)*
    };
}

macro_rules! wrap_ctor {
    ($wrap: ident) => {
        wrap_ctor!([] $wrap);
    };
    ($wrap: ident, $maker: expr) => {
        wrap_ctor!([] $wrap, $maker);
    };
    ([$($bound : tt)*] $wrap: ident) => {
        wrap_ctor!([$($bound)*] $wrap, $wrap::new);
    };
    ([$($bound : tt)*] $wrap: ident, $maker: expr) => {
        arbitrary!([A: $crate::arbitrary::Arbitrary + $($bound)*] $wrap<A>,
            $crate::arbitrary::SMapped<A, Self>, A::Parameters;
            args => $crate::strategy::statics::static_map(
                $crate::arbitrary::any_with::<A>(args), $maker));

        lift1!([$($bound)*] $wrap<A>; $maker);
    };
}

macro_rules! wrap_from {
    ($wrap: ident) => {
        wrap_from!([] $wrap);
    };
    ([$($bound : tt)*] $wrap: ident) => {
        arbitrary!([A: $crate::arbitrary::Arbitrary + $($bound)*] $wrap<A>,
            $crate::strategy::MapInto<A::Strategy, Self>, A::Parameters;
            args => $crate::strategy::Strategy::prop_map_into(
                $crate::arbitrary::any_with::<A>(args)));

        lift1!([$($bound)*] $wrap<A>);
    };
}

macro_rules! lazy_just {
    ($($self: ty, $fun: expr);+) => {
        $(
            arbitrary!($self, $crate::strategy::LazyJust<Self, fn() -> Self>;
                $crate::strategy::LazyJust::new($fun));
        )+
    };
}

//==============================================================================
// Macros for testing:
//==============================================================================

/// We are mostly interested in ensuring that generating input from our
/// strategies is able to construct a value, therefore ensuring that
/// no panic occurs is mostly sufficient. Shrinking for strategies that
/// use special shrinking methods can be handled separately.
#[cfg(test)]
macro_rules! no_panic_test {
    ($($module: ident => $self: ty),+) => {
        $(
            mod $module {
                #[allow(unused_imports)]
                use super::super::*;
                proptest! {
                    #[test]
                    fn no_panic(_ in $crate::arbitrary::any::<$self>()) {}
                }
            }
        )+
    };
}
