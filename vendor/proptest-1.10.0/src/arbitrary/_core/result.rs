//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::result`.

use crate::std_facade::string;
use core::fmt;
use core::result::IntoIter;

use crate::arbitrary::*;
use crate::result::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

// These are Result with uninhabited type in some variant:
arbitrary!([A: Arbitrary] Result<A, string::ParseError>,
    SMapped<A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), Result::Ok)
);
arbitrary!([A: Arbitrary] Result<string::ParseError, A>,
    SMapped<A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), Result::Err)
);
#[cfg(feature = "unstable")]
arbitrary!([A: Arbitrary] Result<A, !>,
    SMapped<A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), Result::Ok)
);
#[cfg(feature = "unstable")]
arbitrary!([A: Arbitrary] Result<!, A>,
    SMapped<A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), Result::Err)
);

lift1!([] Result<A, string::ParseError>; Result::Ok);
#[cfg(feature = "unstable")]
lift1!([] Result<A, !>; Result::Ok);

// We assume that `MaybeOk` is canonical as it's the most likely Strategy
// a user wants.

arbitrary!([A: Arbitrary, B: Arbitrary] Result<A, B>,
    MaybeOk<A::Strategy, B::Strategy>,
    product_type![Probability, A::Parameters, B::Parameters];
    args => {
        let product_unpack![prob, a, b] = args;
        let (p, a, b) = (prob, any_with::<A>(a), any_with::<B>(b));
        maybe_ok_weighted(p, a, b)
    }
);

impl<A: fmt::Debug, E: Arbitrary> functor::ArbitraryF1<A> for Result<A, E>
where
    E::Strategy: 'static,
{
    type Parameters = product_type![Probability, E::Parameters];

    fn lift1_with<AS>(base: AS, args: Self::Parameters) -> BoxedStrategy<Self>
    where
        AS: Strategy<Value = A> + 'static,
    {
        let product_unpack![prob, e] = args;
        let (p, a, e) = (prob, base, any_with::<E>(e));
        maybe_ok_weighted(p, a, e).boxed()
    }
}

impl<A: fmt::Debug, B: fmt::Debug> functor::ArbitraryF2<A, B> for Result<A, B> {
    type Parameters = Probability;

    fn lift2_with<AS, BS>(
        fst: AS,
        snd: BS,
        args: Self::Parameters,
    ) -> BoxedStrategy<Self>
    where
        AS: Strategy<Value = A> + 'static,
        BS: Strategy<Value = B> + 'static,
    {
        maybe_ok_weighted(args, fst, snd).boxed()
    }
}

arbitrary!([A: Arbitrary] IntoIter<A>,
    SMapped<Result<A, ()>, Self>,
    <Result<A, ()> as Arbitrary>::Parameters;
    args => static_map(any_with::<Result<A, ()>>(args), Result::into_iter)
);

lift1!(['static] IntoIter<A>, Probability; base, args => {
    maybe_ok_weighted(args, base, Just(())).prop_map(Result::into_iter)
});

#[cfg(test)]
mod test {
    no_panic_test!(
        result    => Result<u8, u16>,
        into_iter => IntoIter<u8>,
        result_a_parse_error => Result<u8, ::std::string::ParseError>,
        result_parse_error_a => Result<::std::string::ParseError, u8>
    );
}
