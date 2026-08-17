//! Storage area.

use super::{cow_slice::CowSlice, error::HintErrorKind};

/// Backing store for the storage area.
///
/// This is just a wrapper for [`CowSlice`] that converts out of bounds
/// accesses to appropriate errors.
pub struct Storage<'a>(CowSlice<'a>);

impl Storage<'_> {
    pub fn get(&self, index: usize) -> Result<i32, HintErrorKind> {
        self.0
            .get(index)
            .ok_or(HintErrorKind::InvalidStorageIndex(index))
    }

    pub fn set(&mut self, index: usize, value: i32) -> Result<(), HintErrorKind> {
        self.0
            .set(index, value)
            .ok_or(HintErrorKind::InvalidStorageIndex(index))
    }
}

impl<'a> From<CowSlice<'a>> for Storage<'a> {
    fn from(value: CowSlice<'a>) -> Self {
        Self(value)
    }
}
