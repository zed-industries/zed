/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! A number type that implements Javascript / JSON semantics.

use crate::error::{TryFromNumberError, TryFromNumberErrorKind};
#[cfg(all(
    aws_sdk_unstable,
    any(feature = "serde-serialize", feature = "serde-deserialize")
))]
use serde;

/// A number type that implements Javascript / JSON semantics, modeled on serde_json:
/// <https://docs.serde.rs/src/serde_json/number.rs.html#20-22>
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    any(
        all(aws_sdk_unstable, feature = "serde-deserialize"),
        all(aws_sdk_unstable, feature = "serde-serialize")
    ),
    serde(untagged)
)]
pub enum Number {
    /// Unsigned 64-bit integer value.
    PosInt(u64),
    /// Signed 64-bit integer value. The wrapped value is _always_ negative.
    NegInt(i64),
    /// 64-bit floating-point value.
    Float(f64),
}

/* ANCHOR_END: document */

impl Number {
    /// Converts to an `f64` lossily.
    /// Use `Number::try_from` to make the conversion only if it is not lossy.
    pub fn to_f64_lossy(self) -> f64 {
        match self {
            Number::PosInt(v) => v as f64,
            Number::NegInt(v) => v as f64,
            Number::Float(v) => v,
        }
    }

    /// Converts to an `f32` lossily.
    /// Use `Number::try_from` to make the conversion only if it is not lossy.
    pub fn to_f32_lossy(self) -> f32 {
        match self {
            Number::PosInt(v) => v as f32,
            Number::NegInt(v) => v as f32,
            Number::Float(v) => v as f32,
        }
    }
}

macro_rules! to_unsigned_integer_converter {
    ($typ:ident, $styp:expr) => {
        #[doc = "Converts to a `"]
        #[doc = $styp]
        #[doc = "`. This conversion fails if it is lossy."]
        impl TryFrom<Number> for $typ {
            type Error = TryFromNumberError;

            fn try_from(value: Number) -> Result<Self, Self::Error> {
                match value {
                    Number::PosInt(v) => Ok(Self::try_from(v)?),
                    Number::NegInt(v) => {
                        Err(TryFromNumberErrorKind::NegativeToUnsignedLossyConversion(v).into())
                    }
                    Number::Float(v) => attempt_lossless!(v, $typ),
                }
            }
        }
    };

    ($typ:ident) => {
        to_unsigned_integer_converter!($typ, stringify!($typ));
    };
}

macro_rules! to_signed_integer_converter {
    ($typ:ident, $styp:expr) => {
        #[doc = "Converts to a `"]
        #[doc = $styp]
        #[doc = "`. This conversion fails if it is lossy."]
        impl TryFrom<Number> for $typ {
            type Error = TryFromNumberError;

            fn try_from(value: Number) -> Result<Self, Self::Error> {
                match value {
                    Number::PosInt(v) => Ok(Self::try_from(v)?),
                    Number::NegInt(v) => Ok(Self::try_from(v)?),
                    Number::Float(v) => attempt_lossless!(v, $typ),
                }
            }
        }
    };

    ($typ:ident) => {
        to_signed_integer_converter!($typ, stringify!($typ));
    };
}

macro_rules! attempt_lossless {
    ($value: expr, $typ: ty) => {{
        let converted = $value as $typ;
        if (converted as f64 == $value) {
            Ok(converted)
        } else {
            Err(TryFromNumberErrorKind::FloatToIntegerLossyConversion($value).into())
        }
    }};
}

/// Converts to a `u64`. The conversion fails if it is lossy.
impl TryFrom<Number> for u64 {
    type Error = TryFromNumberError;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::PosInt(v) => Ok(v),
            Number::NegInt(v) => {
                Err(TryFromNumberErrorKind::NegativeToUnsignedLossyConversion(v).into())
            }
            Number::Float(v) => attempt_lossless!(v, u64),
        }
    }
}
to_unsigned_integer_converter!(u32);
to_unsigned_integer_converter!(u16);
to_unsigned_integer_converter!(u8);

impl TryFrom<Number> for i64 {
    type Error = TryFromNumberError;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::PosInt(v) => Ok(Self::try_from(v)?),
            Number::NegInt(v) => Ok(v),
            Number::Float(v) => attempt_lossless!(v, i64),
        }
    }
}
to_signed_integer_converter!(i32);
to_signed_integer_converter!(i16);
to_signed_integer_converter!(i8);

/// Converts to an `f64`. The conversion fails if it is lossy.
impl TryFrom<Number> for f64 {
    type Error = TryFromNumberError;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            // Integers can only be represented with full precision in a float if they fit in the
            // significand, which is 24 bits in `f32` and 53 bits in `f64`.
            // https://github.com/rust-lang/rust/blob/58f11791af4f97572e7afd83f11cffe04bbbd12f/library/core/src/convert/num.rs#L151-L153
            Number::PosInt(v) => {
                if v <= (1 << 53) {
                    Ok(v as Self)
                } else {
                    Err(TryFromNumberErrorKind::U64ToFloatLossyConversion(v).into())
                }
            }
            Number::NegInt(v) => {
                if (-(1 << 53)..=(1 << 53)).contains(&v) {
                    Ok(v as Self)
                } else {
                    Err(TryFromNumberErrorKind::I64ToFloatLossyConversion(v).into())
                }
            }
            Number::Float(v) => Ok(v),
        }
    }
}

/// Converts to an `f64`. The conversion fails if it is lossy.
impl TryFrom<Number> for f32 {
    type Error = TryFromNumberError;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        match value {
            Number::PosInt(v) => {
                if v <= (1 << 24) {
                    Ok(v as Self)
                } else {
                    Err(TryFromNumberErrorKind::U64ToFloatLossyConversion(v).into())
                }
            }
            Number::NegInt(v) => {
                if (-(1 << 24)..=(1 << 24)).contains(&v) {
                    Ok(v as Self)
                } else {
                    Err(TryFromNumberErrorKind::I64ToFloatLossyConversion(v).into())
                }
            }
            Number::Float(v) => Err(TryFromNumberErrorKind::F64ToF32LossyConversion(v).into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Number;
    use crate::error::{TryFromNumberError, TryFromNumberErrorKind};

    macro_rules! to_unsigned_converter_tests {
        ($typ:ident) => {
            assert_eq!($typ::try_from(Number::PosInt(69u64)).unwrap(), 69);

            assert!(matches!(
                $typ::try_from(Number::PosInt(($typ::MAX as u64) + 1u64)).unwrap_err(),
                TryFromNumberError {
                    kind: TryFromNumberErrorKind::OutsideIntegerRange(..)
                }
            ));

            assert!(matches!(
                $typ::try_from(Number::NegInt(-1i64)).unwrap_err(),
                TryFromNumberError {
                    kind: TryFromNumberErrorKind::NegativeToUnsignedLossyConversion(..)
                }
            ));

            for val in [69.69f64, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                assert!(matches!(
                    $typ::try_from(Number::Float(val)).unwrap_err(),
                    TryFromNumberError {
                        kind: TryFromNumberErrorKind::FloatToIntegerLossyConversion(..)
                    }
                ));
            }
            assert_eq!($typ::try_from(Number::Float(25.0)).unwrap(), 25);
        };
    }

    #[test]
    fn to_u64() {
        assert_eq!(u64::try_from(Number::PosInt(69u64)).unwrap(), 69u64);

        assert!(matches!(
            u64::try_from(Number::NegInt(-1i64)).unwrap_err(),
            TryFromNumberError {
                kind: TryFromNumberErrorKind::NegativeToUnsignedLossyConversion(..)
            }
        ));

        for val in [69.69f64, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(matches!(
                u64::try_from(Number::Float(val)).unwrap_err(),
                TryFromNumberError {
                    kind: TryFromNumberErrorKind::FloatToIntegerLossyConversion(..)
                }
            ));
        }
    }

    #[test]
    fn to_u32() {
        to_unsigned_converter_tests!(u32);
    }

    #[test]
    fn to_u16() {
        to_unsigned_converter_tests!(u16);
    }

    #[test]
    fn to_u8() {
        to_unsigned_converter_tests!(u8);
    }

    macro_rules! to_signed_converter_tests {
        ($typ:ident) => {
            assert_eq!($typ::try_from(Number::PosInt(69u64)).unwrap(), 69);
            assert_eq!($typ::try_from(Number::NegInt(-69i64)).unwrap(), -69);

            assert!(matches!(
                $typ::try_from(Number::PosInt(($typ::MAX as u64) + 1u64)).unwrap_err(),
                TryFromNumberError {
                    kind: TryFromNumberErrorKind::OutsideIntegerRange(..)
                }
            ));

            assert!(matches!(
                $typ::try_from(Number::NegInt(($typ::MIN as i64) - 1i64)).unwrap_err(),
                TryFromNumberError {
                    kind: TryFromNumberErrorKind::OutsideIntegerRange(..)
                }
            ));

            for val in [69.69f64, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                assert!(matches!(
                    u64::try_from(Number::Float(val)).unwrap_err(),
                    TryFromNumberError {
                        kind: TryFromNumberErrorKind::FloatToIntegerLossyConversion(..)
                    }
                ));
            }

            let range = || ($typ::MIN..=$typ::MAX);

            for val in range().take(1024).chain(range().rev().take(1024)) {
                assert_eq!(val, $typ::try_from(Number::Float(val as f64)).unwrap());
                $typ::try_from(Number::Float((val as f64) + 0.1)).expect_err("not equivalent");
            }
        };
    }

    #[test]
    fn to_i64() {
        assert_eq!(i64::try_from(Number::PosInt(69u64)).unwrap(), 69);
        assert_eq!(i64::try_from(Number::NegInt(-69i64)).unwrap(), -69);

        for val in [69.69f64, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(matches!(
                u64::try_from(Number::Float(val)).unwrap_err(),
                TryFromNumberError {
                    kind: TryFromNumberErrorKind::FloatToIntegerLossyConversion(..)
                }
            ));
        }
        let range = || i64::MIN..=i64::MAX;

        for val in range().take(1024).chain(range().rev().take(1024)) {
            // if we can actually represent the value
            if ((val as f64) as i64) == val {
                assert_eq!(val, i64::try_from(Number::Float(val as f64)).unwrap());
            }
            let fval = val as f64;
            // at the limits of the range, we don't have this precision
            if (fval + 0.1).fract() != 0.0 {
                i64::try_from(Number::Float((val as f64) + 0.1)).expect_err("not equivalent");
            }
        }
    }

    #[test]
    fn to_i32() {
        to_signed_converter_tests!(i32);
    }

    #[test]
    fn to_i16() {
        to_signed_converter_tests!(i16);
    }

    #[test]
    fn to_i8() {
        to_signed_converter_tests!(i8);
        i8::try_from(Number::Float(-3200000.0)).expect_err("overflow");
        i8::try_from(Number::Float(32.1)).expect_err("imprecise");
        i8::try_from(Number::Float(i8::MAX as f64 + 0.1)).expect_err("imprecise");
        i8::try_from(Number::Float(f64::NAN)).expect_err("nan");
        i8::try_from(Number::Float(f64::INFINITY)).expect_err("nan");
    }

    #[test]
    fn to_f64() {
        assert_eq!(f64::try_from(Number::PosInt(69u64)).unwrap(), 69f64);
        assert_eq!(f64::try_from(Number::NegInt(-69i64)).unwrap(), -69f64);
        assert_eq!(f64::try_from(Number::Float(-69f64)).unwrap(), -69f64);
        assert!(f64::try_from(Number::Float(f64::NAN)).unwrap().is_nan());
        assert_eq!(
            f64::try_from(Number::Float(f64::INFINITY)).unwrap(),
            f64::INFINITY
        );
        assert_eq!(
            f64::try_from(Number::Float(f64::NEG_INFINITY)).unwrap(),
            f64::NEG_INFINITY
        );

        let significand_max_u64: u64 = 1 << 53;
        let significand_max_i64: i64 = 1 << 53;

        assert_eq!(
            f64::try_from(Number::PosInt(significand_max_u64)).unwrap(),
            9007199254740992f64
        );

        assert_eq!(
            f64::try_from(Number::NegInt(significand_max_i64)).unwrap(),
            9007199254740992f64
        );
        assert_eq!(
            f64::try_from(Number::NegInt(-significand_max_i64)).unwrap(),
            -9007199254740992f64
        );

        assert!(matches!(
            f64::try_from(Number::PosInt(significand_max_u64 + 1)).unwrap_err(),
            TryFromNumberError {
                kind: TryFromNumberErrorKind::U64ToFloatLossyConversion(..)
            }
        ));

        assert!(matches!(
            f64::try_from(Number::NegInt(significand_max_i64 + 1)).unwrap_err(),
            TryFromNumberError {
                kind: TryFromNumberErrorKind::I64ToFloatLossyConversion(..)
            }
        ));
        assert!(matches!(
            f64::try_from(Number::NegInt(-significand_max_i64 - 1)).unwrap_err(),
            TryFromNumberError {
                kind: TryFromNumberErrorKind::I64ToFloatLossyConversion(..)
            }
        ));
    }

    #[test]
    fn to_f32() {
        assert_eq!(f32::try_from(Number::PosInt(69u64)).unwrap(), 69f32);
        assert_eq!(f32::try_from(Number::NegInt(-69i64)).unwrap(), -69f32);

        let significand_max_u64: u64 = 1 << 24;
        let significand_max_i64: i64 = 1 << 24;

        assert_eq!(
            f32::try_from(Number::PosInt(significand_max_u64)).unwrap(),
            16777216f32
        );

        assert_eq!(
            f32::try_from(Number::NegInt(significand_max_i64)).unwrap(),
            16777216f32
        );
        assert_eq!(
            f32::try_from(Number::NegInt(-significand_max_i64)).unwrap(),
            -16777216f32
        );

        assert!(matches!(
            f32::try_from(Number::PosInt(significand_max_u64 + 1)).unwrap_err(),
            TryFromNumberError {
                kind: TryFromNumberErrorKind::U64ToFloatLossyConversion(..)
            }
        ));

        assert!(matches!(
            f32::try_from(Number::NegInt(significand_max_i64 + 1)).unwrap_err(),
            TryFromNumberError {
                kind: TryFromNumberErrorKind::I64ToFloatLossyConversion(..)
            }
        ));
        assert!(matches!(
            f32::try_from(Number::NegInt(-significand_max_i64 - 1)).unwrap_err(),
            TryFromNumberError {
                kind: TryFromNumberErrorKind::I64ToFloatLossyConversion(..)
            }
        ));

        for val in [69f64, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(matches!(
                f32::try_from(Number::Float(val)).unwrap_err(),
                TryFromNumberError {
                    kind: TryFromNumberErrorKind::F64ToF32LossyConversion(..)
                }
            ));
        }
    }

    #[test]
    fn to_f64_lossy() {
        assert_eq!(Number::PosInt(69u64).to_f64_lossy(), 69f64);
        assert_eq!(
            Number::PosInt((1 << 53) + 1).to_f64_lossy(),
            9007199254740992f64
        );
        assert_eq!(
            Number::NegInt(-(1 << 53) - 1).to_f64_lossy(),
            -9007199254740992f64
        );
    }

    #[test]
    fn to_f32_lossy() {
        assert_eq!(Number::PosInt(69u64).to_f32_lossy(), 69f32);
        assert_eq!(Number::PosInt((1 << 24) + 1).to_f32_lossy(), 16777216f32);
        assert_eq!(Number::NegInt(-(1 << 24) - 1).to_f32_lossy(), -16777216f32);
        assert_eq!(
            Number::Float(1452089033.7674935).to_f32_lossy(),
            1452089100f32
        );
    }

    #[test]
    #[cfg(all(
        test,
        aws_sdk_unstable,
        feature = "serde-deserialize",
        feature = "serde-serialize"
    ))]
    /// ensures that numbers are deserialized as expected
    /// 0 <= PosInt
    /// 0 > NegInt
    /// non integer values == Float
    fn number_serde() {
        let n: Number = serde_json::from_str("1.1").unwrap();
        assert_eq!(n, Number::Float(1.1));
        let n: Number = serde_json::from_str("1").unwrap();
        assert_eq!(n, Number::PosInt(1));
        let n: Number = serde_json::from_str("0").unwrap();
        assert_eq!(n, Number::PosInt(0));
        let n: Number = serde_json::from_str("-1").unwrap();
        assert_eq!(n, Number::NegInt(-1));

        assert_eq!("1.1", serde_json::to_string(&Number::Float(1.1)).unwrap());
        assert_eq!("1", serde_json::to_string(&Number::PosInt(1)).unwrap());
        assert_eq!("0", serde_json::to_string(&Number::PosInt(0)).unwrap());
        assert_eq!("-1", serde_json::to_string(&Number::NegInt(-1)).unwrap());
    }
}
