//! Vendors the [`equivalent`](https://crates.io/crates/equivalent) crate to avoid conflicts.

#![deny(unsafe_code)]

use std::borrow::Borrow;
use std::cmp::Ordering;

/// Key equivalence trait.
///
/// [`Hash`](std::hash::Hash) must be implemented to ensure that the same hash value
/// is generated for equivalent keys.
pub trait Equivalent<K: ?Sized> {
    /// Compares `self` with `key` and returns `true` if they are equal.
    fn equivalent(&self, key: &K) -> bool;
}

impl<Q: ?Sized, K: ?Sized> Equivalent<K> for Q
where
    Q: Eq,
    K: Borrow<Q>,
{
    #[inline]
    fn equivalent(&self, key: &K) -> bool {
        PartialEq::eq(self, key.borrow())
    }
}

/// Key ordering trait.
pub trait Comparable<K: ?Sized>: Equivalent<K> {
    /// Compares `self` with `key` and returns their ordering.
    fn compare(&self, key: &K) -> Ordering;
}

impl<Q: ?Sized, K: ?Sized> Comparable<K> for Q
where
    Q: Ord,
    K: Borrow<Q>,
{
    #[inline]
    fn compare(&self, key: &K) -> Ordering {
        Ord::cmp(self, key.borrow())
    }
}
