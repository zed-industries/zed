/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Errors for Smithy codegen

use std::fmt;

pub mod display;
pub mod metadata;
pub mod operation;

pub use metadata::ErrorMetadata;

#[derive(Debug)]
pub(super) enum TryFromNumberErrorKind {
    /// Used when the conversion from an integer type into a smaller integer type would be lossy.
    OutsideIntegerRange(std::num::TryFromIntError),
    /// Used when the conversion from an `u64` into a floating point type would be lossy.
    U64ToFloatLossyConversion(u64),
    /// Used when the conversion from an `i64` into a floating point type would be lossy.
    I64ToFloatLossyConversion(i64),
    /// Used when attempting to convert an `f64` into an `f32`.
    F64ToF32LossyConversion(f64),
    /// Used when attempting to convert a decimal, infinite, or `NaN` floating point type into an
    /// integer type.
    FloatToIntegerLossyConversion(f64),
    /// Used when attempting to convert a negative [`Number`](crate::Number) into an unsigned integer type.
    NegativeToUnsignedLossyConversion(i64),
}

/// The error type returned when conversion into an integer type or floating point type is lossy.
#[derive(Debug)]
pub struct TryFromNumberError {
    pub(super) kind: TryFromNumberErrorKind,
}

impl fmt::Display for TryFromNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TryFromNumberErrorKind::*;
        match self.kind {
            OutsideIntegerRange(_) => write!(f, "integer too large"),
            FloatToIntegerLossyConversion(v) => write!(
                f,
                "cannot convert floating point number {v} into an integer"
            ),
            NegativeToUnsignedLossyConversion(v) => write!(
                f,
                "cannot convert negative integer {v} into an unsigned integer type"
            ),
            U64ToFloatLossyConversion(v) => {
                write!(
                    f,
                    "cannot convert {v}u64 into a floating point type without precision loss"
                )
            }
            I64ToFloatLossyConversion(v) => {
                write!(
                    f,
                    "cannot convert {v}i64 into a floating point type without precision loss"
                )
            }
            F64ToF32LossyConversion(v) => {
                write!(f, "will not attempt to convert {v}f64 into a f32")
            }
        }
    }
}

impl std::error::Error for TryFromNumberError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use TryFromNumberErrorKind::*;
        match &self.kind {
            OutsideIntegerRange(err) => Some(err as _),
            FloatToIntegerLossyConversion(_)
            | NegativeToUnsignedLossyConversion(_)
            | U64ToFloatLossyConversion(_)
            | I64ToFloatLossyConversion(_)
            | F64ToF32LossyConversion(_) => None,
        }
    }
}

impl From<std::num::TryFromIntError> for TryFromNumberError {
    fn from(value: std::num::TryFromIntError) -> Self {
        Self {
            kind: TryFromNumberErrorKind::OutsideIntegerRange(value),
        }
    }
}

impl From<TryFromNumberErrorKind> for TryFromNumberError {
    fn from(kind: TryFromNumberErrorKind) -> Self {
        Self { kind }
    }
}
