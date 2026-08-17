//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::num`.

use core::num::*;

use crate::strategy::*;

arbitrary!(ParseFloatError; "".parse::<f32>().unwrap_err());
arbitrary!(ParseIntError; "".parse::<u32>().unwrap_err());

#[cfg(feature = "unstable")]
arbitrary!(TryFromIntError; {
    use core::convert::TryFrom;
    u8::try_from(-1).unwrap_err()
});

wrap_ctor!(Wrapping, Wrapping);

wrap_ctor!(Saturating, Saturating);

arbitrary!(FpCategory,
    TupleUnion<(WA<Just<Self>>, WA<Just<Self>>, WA<Just<Self>>,
                WA<Just<Self>>, WA<Just<Self>>)>;
    {
        use core::num::FpCategory::*;
        prop_oneof![
            Just(Nan),
            Just(Infinite),
            Just(Zero),
            Just(Subnormal),
            Just(Normal),
        ]
    }
);

#[cfg(test)]
mod test {
    no_panic_test!(
        parse_float_error => ParseFloatError,
        parse_int_error => ParseIntError,
        wrapping => Wrapping<u8>,
        saturating => Saturating<u8>,
        fp_category => FpCategory
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        try_from_int_error => TryFromIntError
    );
}
