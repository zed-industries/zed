//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::char`.

use crate::std_facade::Vec;
use core::char::*;
use core::iter::once;
use core::ops::Range;

use crate::collection::vec;

multiplex_alloc! {
    core::char::DecodeUtf16, std::char::DecodeUtf16,
    core::char::DecodeUtf16Error, std::char::DecodeUtf16Error,
    core::char::decode_utf16, std::char::decode_utf16
}

const VEC_MAX: usize = ::core::u16::MAX as usize;

use crate::arbitrary::*;
use crate::strategy::statics::static_map;
use crate::strategy::*;

macro_rules! impl_wrap_char {
    ($type: ty, $mapper: expr) => {
        arbitrary!($type, SMapped<char, Self>;
            static_map(any::<char>(), $mapper));
    };
}

impl_wrap_char!(EscapeDebug, char::escape_debug);
impl_wrap_char!(EscapeDefault, char::escape_default);
impl_wrap_char!(EscapeUnicode, char::escape_unicode);
#[cfg(feature = "unstable")]
impl_wrap_char!(ToLowercase, char::to_lowercase);
#[cfg(feature = "unstable")]
impl_wrap_char!(ToUppercase, char::to_uppercase);

arbitrary!(DecodeUtf16<<Vec<u16> as IntoIterator>::IntoIter>,
    SMapped<Vec<u16>, Self>;
    static_map(vec(any::<u16>(), ..VEC_MAX), decode_utf16)
);

arbitrary!(ParseCharError, IndFlatten<Mapped<bool, Just<Self>>>;
    any::<bool>().prop_ind_flat_map(|is_two|
        Just((if is_two { "__" } else { "" }).parse::<char>().unwrap_err()))
);

#[cfg(feature = "unstable")]
arbitrary!(CharTryFromError; {
    use core::convert::TryFrom;
    char::try_from(0xD800 as u32).unwrap_err()
});

arbitrary!(DecodeUtf16Error, SFnPtrMap<Range<u16>, Self>;
    static_map(0xD800..0xE000, |x|
        decode_utf16(once(x)).next().unwrap().unwrap_err())
);

#[cfg(test)]
mod test {
    no_panic_test!(
        escape_debug => EscapeDebug,
        escape_default => EscapeDefault,
        escape_unicode => EscapeUnicode,
        parse_char_error => ParseCharError,
        decode_utf16_error => DecodeUtf16Error
    );

    no_panic_test!(
        decode_utf16 => DecodeUtf16<<Vec<u16> as IntoIterator>::IntoIter>
    );

    #[cfg(feature = "unstable")]
    no_panic_test!(
        to_lowercase => ToLowercase,
        to_uppercase => ToUppercase,
        char_try_from_error => CharTryFromError
    );
}
