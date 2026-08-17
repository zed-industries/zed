//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::option`.

use crate::std_facade::string;
use core::ops::RangeInclusive;
use core::option as opt;

use crate::arbitrary::*;
use crate::option::{weighted, OptionStrategy, Probability};
use crate::strategy::statics::static_map;
use crate::strategy::*;

arbitrary!(Probability, MapInto<RangeInclusive<f64>, Self>;
    (0.0..=1.0).prop_map_into()
);

// These are Option<AnUninhabitedType> impls:

arbitrary!(Option<string::ParseError>; None);
#[cfg(feature = "unstable")]
arbitrary!(Option<!>; None);

arbitrary!([A: Arbitrary] opt::Option<A>, OptionStrategy<A::Strategy>,
    product_type![Probability, A::Parameters];
    args => {
        let product_unpack![prob, a] = args;
        weighted(prob, any_with::<A>(a))
    }
);

lift1!([] Option<A>, Probability; base, prob => weighted(prob, base));

arbitrary!([A: Arbitrary] opt::IntoIter<A>, SMapped<Option<A>, Self>,
    <Option<A> as Arbitrary>::Parameters;
    args => static_map(any_with::<Option<A>>(args), Option::into_iter));

lift1!(['static] opt::IntoIter<A>, Probability;
    base, prob => weighted(prob, base).prop_map(Option::into_iter)
);

#[cfg(test)]
mod test {
    no_panic_test!(
        probability => Probability,
        option      => Option<u8>,
        option_iter => opt::IntoIter<u8>,
        option_parse_error => Option<string::ParseError>
    );
}
