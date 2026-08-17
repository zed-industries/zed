//! [`Archive`](crate::Archive) implementation for B-tree sets.

use crate::collections::btree_map::{ArchivedBTreeMap, BTreeMapResolver, Keys};
use core::{borrow::Borrow, fmt};

/// An archived `BTreeSet`. This is a wrapper around a B-tree map with the same key and a value of
/// `()`.
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ArchivedBTreeSet<K>(ArchivedBTreeMap<K, ()>);

impl<K> ArchivedBTreeSet<K> {
    /// Returns `true` if the set contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the set's key type, but the ordering on the borrowed
    /// form _must_ match the ordering on the key type.
    #[inline]
    pub fn contains_key<Q: Ord + ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + Ord,
    {
        self.0.contains_key(key)
    }

    /// Returns a reference to the value int he set, if any, that is equal to the given value.
    ///
    /// The value may be any borrowed form of the set's value type, but the ordering on the borrowed
    /// form _must_ match the ordering on the value type.
    #[inline]
    pub fn get<Q: Ord + ?Sized>(&self, value: &Q) -> Option<&K>
    where
        K: Borrow<Q> + Ord,
    {
        self.0.get_key_value(value).map(|(key, _)| key)
    }

    /// Returns `true` if the set contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets an iterator over the keys of the set, in sorted order.
    #[inline]
    pub fn iter(&self) -> Keys<K, ()> {
        self.0.keys()
    }

    /// Returns the number of items in the archived B-tree set.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Resolves a B-tree set from its length.
    ///
    /// # Safety
    ///
    /// - `len` must be the number of elements that were serialized
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing a B-tree set
    #[inline]
    pub unsafe fn resolve_from_len(
        len: usize,
        pos: usize,
        resolver: BTreeSetResolver,
        out: *mut Self,
    ) {
        let (fp, fo) = out_field!(out.0);
        ArchivedBTreeMap::resolve_from_len(len, pos + fp, resolver.0, fo);
    }
}

#[cfg(feature = "alloc")]
const _: () = {
    use crate::{ser::Serializer, Serialize};

    impl<K> ArchivedBTreeSet<K> {
        /// Serializes an ordered iterator of key-value pairs as a B-tree map.
        ///
        /// # Safety
        ///
        /// - Keys returned by the iterator must be unique
        /// - Keys must be in reverse sorted order from last to first
        pub unsafe fn serialize_from_reverse_iter<'a, UK, S, I>(
            iter: I,
            serializer: &mut S,
        ) -> Result<BTreeSetResolver, S::Error>
        where
            UK: 'a + Serialize<S, Archived = K>,
            S: Serializer + ?Sized,
            I: ExactSizeIterator<Item = &'a UK>,
        {
            Ok(BTreeSetResolver(
                ArchivedBTreeMap::serialize_from_reverse_iter(iter.map(|x| (x, &())), serializer)?,
            ))
        }
    }
};

impl<K: fmt::Debug> fmt::Debug for ArchivedBTreeSet<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<'a, K> IntoIterator for &'a ArchivedBTreeSet<K> {
    type Item = &'a K;
    type IntoIter = Keys<'a, K, ()>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// The resolver for archived B-tree sets.
pub struct BTreeSetResolver(BTreeMapResolver);
