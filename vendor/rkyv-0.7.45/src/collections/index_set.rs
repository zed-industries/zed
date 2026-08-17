//! Archived index set implementation.
//!
//! During archiving, index sets are built into minimal perfect index sets using
//! [compress, hash and displace](http://cmph.sourceforge.net/papers/esa09.pdf).

use crate::{
    collections::{
        hash_index::HashBuilder,
        index_map::{ArchivedIndexMap, IndexMapResolver, Keys},
    },
    out_field,
};
use core::{borrow::Borrow, fmt, hash::Hash};

/// An archived `IndexSet`.
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[repr(transparent)]
pub struct ArchivedIndexSet<K> {
    inner: ArchivedIndexMap<K, ()>,
}

impl<K> ArchivedIndexSet<K> {
    /// Returns whether a key is present in the hash set.
    #[inline]
    pub fn contains<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.contains_key(k)
    }

    /// Returns the first key.
    #[inline]
    pub fn first(&self) -> Option<&K> {
        self.inner.first().map(|(k, _)| k)
    }

    /// Returns the value stored in the set, if any.
    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get_full(k).map(|(_, k, _)| k)
    }

    /// Returns the item index and value stored in the set, if any.
    #[inline]
    pub fn get_full<Q: ?Sized>(&self, k: &Q) -> Option<(usize, &K)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get_full(k).map(|(i, k, _)| (i, k))
    }

    /// Gets a key by index.
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<&K> {
        self.inner.get_index(index).map(|(k, _)| k)
    }

    /// Returns the index of a key if it exists in the set.
    #[inline]
    pub fn get_index_of<Q: ?Sized>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get_index_of(key)
    }

    /// Gets the hasher for this index set.
    #[inline]
    pub fn hasher(&self) -> HashBuilder {
        self.inner.hasher()
    }

    /// Returns whether the index set contains no values.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator over the keys of the index set in order.
    #[inline]
    pub fn iter(&self) -> Keys<K, ()> {
        self.inner.keys()
    }

    /// Returns the last key.
    #[inline]
    pub fn last(&self) -> Option<&K> {
        self.inner.last().map(|(k, _)| k)
    }

    /// Returns the number of elements in the index set.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Resolves an archived index map from a given length and parameters.
    ///
    /// # Safety
    ///
    /// - `len` must be the number of elements that were serialized
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing a hash map
    #[inline]
    pub unsafe fn resolve_from_len(
        len: usize,
        pos: usize,
        resolver: IndexSetResolver,
        out: *mut Self,
    ) {
        let (fp, fo) = out_field!(out.inner);
        ArchivedIndexMap::resolve_from_len(len, pos + fp, resolver.0, fo);
    }
}

#[cfg(feature = "alloc")]
const _: () = {
    use crate::{
        ser::{ScratchSpace, Serializer},
        Serialize,
    };

    impl<K> ArchivedIndexSet<K> {
        /// Serializes an iterator of keys as an index set.
        ///
        /// # Safety
        ///
        /// - The keys returned by the iterator must be unique
        /// - The index function must return the index of the given key within the iterator
        #[inline]
        pub unsafe fn serialize_from_iter_index<'a, UK, I, F, S>(
            iter: I,
            index: F,
            serializer: &mut S,
        ) -> Result<IndexSetResolver, S::Error>
        where
            UK: 'a + Hash + Eq + Serialize<S, Archived = K>,
            I: Clone + ExactSizeIterator<Item = &'a UK>,
            F: Fn(&UK) -> usize,
            S: ScratchSpace + Serializer + ?Sized,
        {
            Ok(IndexSetResolver(
                ArchivedIndexMap::serialize_from_iter_index(
                    iter.map(|k| (k, &())),
                    index,
                    serializer,
                )?,
            ))
        }
    }
};

impl<K: fmt::Debug> fmt::Debug for ArchivedIndexSet<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<K: PartialEq> PartialEq for ArchivedIndexSet<K> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

/// The resolver for `IndexSet`.
pub struct IndexSetResolver(IndexMapResolver);
