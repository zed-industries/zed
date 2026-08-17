//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::ops`.

use crate::std_facade::Arc;
use core::ops::*;

use crate::arbitrary::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

arbitrary!(RangeFull; ..);
wrap_ctor!(RangeFrom, |a| a..);
wrap_ctor!(RangeTo, |a| ..a);

wrap_ctor!(RangeToInclusive, |a| ..=a);

arbitrary!(
    [A: PartialOrd + Arbitrary] RangeInclusive<A>,
    SMapped<(A, A), Self>, product_type![A::Parameters, A::Parameters];
    args => static_map(any_with::<(A, A)>(args),
        |(a, b)| if b < a { b..=a } else { a..=b })
);

lift1!([PartialOrd] RangeInclusive<A>; base => {
    let base = Arc::new(base);
    (base.clone(), base).prop_map(|(a, b)| if b < a { b..=a } else { a..=b })
});

arbitrary!(
    [A: PartialOrd + Arbitrary] Range<A>,
    SMapped<(A, A), Self>, product_type![A::Parameters, A::Parameters];
    args => static_map(any_with::<(A, A)>(args),
        |(a, b)| if b < a { b..a } else { a..b })
);

lift1!([PartialOrd] Range<A>; base => {
    let base = Arc::new(base);
    (base.clone(), base).prop_map(|(a, b)| if b < a { b..a } else { a..b })
});

#[cfg(feature = "unstable")]
arbitrary!(
    [Y: Arbitrary, R: Arbitrary] CoroutineState<Y, R>,
    TupleUnion<(WA<SMapped<Y, Self>>, WA<SMapped<R, Self>>)>,
    product_type![Y::Parameters, R::Parameters];
    args => {
        let product_unpack![y, r] = args;
        prop_oneof![
            static_map(any_with::<Y>(y), CoroutineState::Yielded),
            static_map(any_with::<R>(r), CoroutineState::Complete)
        ]
    }
);

#[cfg(feature = "unstable")]
use core::fmt;

#[cfg(feature = "unstable")]
impl<A: fmt::Debug + 'static, B: fmt::Debug + 'static>
    functor::ArbitraryF2<A, B> for CoroutineState<A, B>
{
    type Parameters = ();

    fn lift2_with<AS, BS>(
        fst: AS,
        snd: BS,
        _args: Self::Parameters,
    ) -> BoxedStrategy<Self>
    where
        AS: Strategy<Value = A> + 'static,
        BS: Strategy<Value = B> + 'static,
    {
        prop_oneof![
            fst.prop_map(CoroutineState::Yielded),
            snd.prop_map(CoroutineState::Complete)
        ]
        .boxed()
    }
}

#[cfg(test)]
mod test {
    no_panic_test!(
        range_full => RangeFull,
        range_from => RangeFrom<usize>,
        range_to   => RangeTo<usize>,
        range      => Range<usize>,
        range_inclusive => RangeInclusive<usize>,
        range_to_inclusive => RangeToInclusive<usize>
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        generator_state => CoroutineState<u32, u64>
    );
}
