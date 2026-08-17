use std::{borrow::Borrow, collections::BTreeSet};

/// Very minimal Vec+binary search set.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct FlatSet<T: Ord>(Vec<T>);

impl<T: Ord> FlatSet<T> {
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
    pub fn contains(&self, value: impl Borrow<T>) -> bool {
        self.0.binary_search(value.borrow()).is_ok()
    }
}

impl<T: Ord> From<BTreeSet<T>> for FlatSet<T> {
    fn from(value: BTreeSet<T>) -> Self {
        FlatSet(value.into_iter().collect())
    }
}

impl<T: Ord> From<Vec<T>> for FlatSet<T> {
    fn from(mut value: Vec<T>) -> Self {
        value.sort_unstable();
        value.dedup();
        FlatSet(value)
    }
}
