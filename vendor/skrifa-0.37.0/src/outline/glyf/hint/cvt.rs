//! Control value table.

use super::{cow_slice::CowSlice, error::HintErrorKind, F26Dot6};

/// Backing store for the control value table.
///
/// This is just a wrapper for [`CowSlice`] that converts out of bounds
/// accesses to appropriate errors.
pub struct Cvt<'a>(CowSlice<'a>);

impl Cvt<'_> {
    pub fn get(&self, index: usize) -> Result<F26Dot6, HintErrorKind> {
        self.0
            .get(index)
            .map(F26Dot6::from_bits)
            .ok_or(HintErrorKind::InvalidCvtIndex(index))
    }

    pub fn set(&mut self, index: usize, value: F26Dot6) -> Result<(), HintErrorKind> {
        self.0
            .set(index, value.to_bits())
            .ok_or(HintErrorKind::InvalidCvtIndex(index))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> From<CowSlice<'a>> for Cvt<'a> {
    fn from(value: CowSlice<'a>) -> Self {
        Self(value)
    }
}
