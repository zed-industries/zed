//! Definitions used in derived implementations of [`core::ops::Add`]-like traits.

use core::{error::Error, fmt};

use crate::UnitError;

/// Error returned by the derived implementations when an arithmetic or logic
/// operation is invoked on mismatched enum variants.
#[derive(Clone, Copy, Debug)]
pub struct WrongVariantError {
    operation_name: &'static str,
}

impl WrongVariantError {
    #[doc(hidden)]
    #[must_use]
    #[inline]
    pub const fn new(operation_name: &'static str) -> Self {
        Self { operation_name }
    }
}

impl fmt::Display for WrongVariantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trying to {}() mismatched enum variants",
            self.operation_name,
        )
    }
}

impl Error for WrongVariantError {}

/// Possible errors returned by the derived implementations of binary
/// arithmetic or logic operations.
#[derive(Clone, Copy, Debug)]
pub enum BinaryError {
    /// Operation is attempted between mismatched enum variants.
    Mismatch(WrongVariantError),

    /// Operation is attempted on unit-like enum variants.
    Unit(UnitError),
}

impl fmt::Display for BinaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mismatch(e) => write!(f, "{e}"),
            Self::Unit(e) => write!(f, "{e}"),
        }
    }
}

impl Error for BinaryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Mismatch(e) => e.source(),
            Self::Unit(e) => e.source(),
        }
    }
}
