//! Traits implemented by scalar, non-SIMD, types.

pub use self::complex::ComplexField;
pub use self::field::{
    ClosedAdd, ClosedAddAssign, ClosedDiv, ClosedDivAssign, ClosedMul, ClosedMulAssign, ClosedNeg,
    ClosedSub, ClosedSubAssign, Field,
};
#[cfg(feature = "partial_fixed_point_support")]
pub use self::fixed_impl::*;
pub use self::real::RealField;
pub use self::subset::{SubsetOf, SupersetOf};

mod real;
#[macro_use]
mod complex;
mod field;
#[cfg(feature = "partial_fixed_point_support")]
mod fixed_impl;
mod subset;
