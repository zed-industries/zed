//! Converting standard types into `ValueBag`s.

use super::{Error, ValueBag};

macro_rules! convert_primitive {
    ($($t:ty: $from:ident, $to:ident,)*) => {
        $(
            impl<'v> From<$t> for ValueBag<'v> {
                #[inline]
                fn from(v: $t) -> Self {
                    ValueBag::$from(v)
                }
            }

            impl<'a, 'v> From<&'a $t> for ValueBag<'v> {
                #[inline]
                fn from(v: &'a $t) -> Self {
                    ValueBag::$from(*v)
                }
            }

            impl<'v> From<Option<$t>> for ValueBag<'v> {
                #[inline]
                fn from(v: Option<$t>) -> Self {
                    ValueBag::from_option(v)
                }
            }

            impl<'v> TryFrom<ValueBag<'v>> for $t {
                type Error = Error;

                #[inline]
                fn try_from(v: ValueBag<'v>) -> Result<Self, Error> {
                    v.$to()
                        .ok_or_else(|| Error::msg("conversion failed"))?
                        .try_into()
                        .map_err(|_| Error::msg("conversion failed"))
                }
            }
        )*
    };
}

impl<'v> From<()> for ValueBag<'v> {
    #[inline]
    fn from(_: ()) -> Self {
        ValueBag::empty()
    }
}

impl<'a, 'v> From<&'a ()> for ValueBag<'v> {
    #[inline]
    fn from(_: &'a ()) -> Self {
        ValueBag::empty()
    }
}

convert_primitive!(
    u8: from_u8, to_u64,
    u16: from_u16, to_u64,
    u32: from_u32, to_u64,
    u64: from_u64, to_u64,
    usize: from_usize, to_u64,
    i8: from_i8, to_i64,
    i16: from_i16, to_i64,
    i32: from_i32, to_i64,
    i64: from_i64, to_i64,
    isize: from_isize, to_i64,
    f64: from_f64, to_f64,
    bool: from_bool, to_bool,
    char: from_char, to_char,
);

impl<'v> From<f32> for ValueBag<'v> {
    #[inline]
    fn from(v: f32) -> Self {
        ValueBag::from_f32(v)
    }
}

impl<'v> From<Option<f32>> for ValueBag<'v> {
    #[inline]
    fn from(v: Option<f32>) -> Self {
        ValueBag::from_option(v)
    }
}

impl<'a, 'v> From<&'a f32> for ValueBag<'v> {
    #[inline]
    fn from(v: &'a f32) -> Self {
        ValueBag::from_f32(*v)
    }
}

#[cfg(feature = "inline-i128")]
impl<'a, 'v> From<&'a u128> for ValueBag<'v> {
    #[inline]
    fn from(v: &'a u128) -> Self {
        ValueBag::from_u128(*v)
    }
}

#[cfg(not(feature = "inline-i128"))]
impl<'v> From<&'v u128> for ValueBag<'v> {
    #[inline]
    fn from(v: &'v u128) -> Self {
        ValueBag::from_u128_ref(v)
    }
}

#[cfg(feature = "inline-i128")]
impl<'v> From<u128> for ValueBag<'v> {
    #[inline]
    fn from(v: u128) -> Self {
        ValueBag::from_u128(v)
    }
}

impl<'v> TryFrom<ValueBag<'v>> for u128 {
    type Error = Error;

    #[inline]
    fn try_from(v: ValueBag<'v>) -> Result<Self, Error> {
        v.to_u128().ok_or_else(|| Error::msg("conversion failed"))
    }
}

#[cfg(feature = "inline-i128")]
impl<'a, 'v> From<&'a i128> for ValueBag<'v> {
    #[inline]
    fn from(v: &'a i128) -> Self {
        ValueBag::from_i128(*v)
    }
}

#[cfg(not(feature = "inline-i128"))]
impl<'v> From<&'v i128> for ValueBag<'v> {
    #[inline]
    fn from(v: &'v i128) -> Self {
        ValueBag::from_i128_ref(v)
    }
}

#[cfg(feature = "inline-i128")]
impl<'v> From<i128> for ValueBag<'v> {
    #[inline]
    fn from(v: i128) -> Self {
        ValueBag::from_i128(v)
    }
}

impl<'v> TryFrom<ValueBag<'v>> for i128 {
    type Error = Error;

    #[inline]
    fn try_from(v: ValueBag<'v>) -> Result<Self, Error> {
        v.to_i128().ok_or_else(|| Error::msg("conversion failed"))
    }
}

impl<'v> From<&'v str> for ValueBag<'v> {
    #[inline]
    fn from(v: &'v str) -> Self {
        ValueBag::from_str(v)
    }
}

impl<'v> From<Option<&'v str>> for ValueBag<'v> {
    #[inline]
    fn from(v: Option<&'v str>) -> Self {
        ValueBag::from_option(v)
    }
}

impl<'v> TryFrom<ValueBag<'v>> for &'v str {
    type Error = Error;

    #[inline]
    fn try_from(v: ValueBag<'v>) -> Result<Self, Error> {
        v.to_borrowed_str()
            .ok_or_else(|| Error::msg("conversion failed"))
    }
}

impl<'v, 'u> From<&'v &'u str> for ValueBag<'v>
where
    'u: 'v,
{
    #[inline]
    fn from(v: &'v &'u str) -> Self {
        ValueBag::from_str(v)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::{borrow::Cow, string::String};

    impl<'v> From<&'v String> for ValueBag<'v> {
        #[inline]
        fn from(v: &'v String) -> Self {
            ValueBag::from_str(v)
        }
    }

    impl<'v> TryFrom<ValueBag<'v>> for String {
        type Error = Error;

        #[inline]
        fn try_from(v: ValueBag<'v>) -> Result<Self, Error> {
            Ok(v.to_str()
                .ok_or_else(|| Error::msg("conversion failed"))?
                .into_owned())
        }
    }

    impl<'v> From<&'v Cow<'v, str>> for ValueBag<'v> {
        #[inline]
        fn from(v: &'v Cow<'v, str>) -> Self {
            ValueBag::from_str(v)
        }
    }

    impl<'v> TryFrom<ValueBag<'v>> for Cow<'v, str> {
        type Error = Error;

        #[inline]
        fn try_from(v: ValueBag<'v>) -> Result<Self, Error> {
            v.to_str().ok_or_else(|| Error::msg("conversion failed"))
        }
    }
}

#[cfg(feature = "owned")]
mod owned_support {
    use super::*;

    use crate::OwnedValueBag;

    impl<'v> From<&'v OwnedValueBag> for ValueBag<'v> {
        #[inline]
        fn from(v: &'v OwnedValueBag) -> ValueBag<'v> {
            v.by_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    use crate::{
        std::{borrow::ToOwned, string::ToString},
        test::{IntoValueBag, TestToken},
    };

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_into_display() {
        assert_eq!(42u64.into_value_bag().by_ref().to_string(), "42");
        assert_eq!(42i64.into_value_bag().by_ref().to_string(), "42");
        assert_eq!(42.01f64.into_value_bag().by_ref().to_string(), "42.01");
        assert_eq!(true.into_value_bag().by_ref().to_string(), "true");
        assert_eq!('a'.into_value_bag().by_ref().to_string(), "a");
        assert_eq!(
            "a loong string".into_value_bag().by_ref().to_string(),
            "a loong string"
        );
        assert_eq!(().into_value_bag().by_ref().to_string(), "None");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_into_structured() {
        assert_eq!(
            42u64.into_value_bag().by_ref().to_test_token(),
            TestToken::U64(42)
        );
        assert_eq!(
            42i64.into_value_bag().by_ref().to_test_token(),
            TestToken::I64(42)
        );
        assert_eq!(
            42.01f64.into_value_bag().by_ref().to_test_token(),
            TestToken::F64(42.01)
        );
        assert_eq!(
            true.into_value_bag().by_ref().to_test_token(),
            TestToken::Bool(true)
        );
        assert_eq!(
            'a'.into_value_bag().by_ref().to_test_token(),
            TestToken::Char('a')
        );
        assert_eq!(
            "a loong string".into_value_bag().by_ref().to_test_token(),
            TestToken::Str("a loong string".to_owned())
        );
        assert_eq!(
            ().into_value_bag().by_ref().to_test_token(),
            TestToken::None
        );

        #[cfg(feature = "inline-i128")]
        {
            assert_eq!(
                42u128.into_value_bag().by_ref().to_test_token(),
                TestToken::U128(42)
            );
            assert_eq!(
                42i128.into_value_bag().by_ref().to_test_token(),
                TestToken::I128(42)
            );
        }
    }
}
