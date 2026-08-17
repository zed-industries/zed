//! An ordered map based on a B-Tree that keeps insertion order of elements.

#[cfg(all(
    feature = "hash-collections",
    not(feature = "prefer-btree-collections")
))]
mod impls {
    use crate::collections::hash;
    use indexmap::IndexMap;

    pub type IndexMapImpl<K, V> = IndexMap<K, V, hash::RandomState>;
    pub type EntryImpl<'a, K, V> = indexmap::map::Entry<'a, K, V>;
    pub type OccupiedEntryImpl<'a, K, V> = indexmap::map::OccupiedEntry<'a, K, V>;
    pub type VacantEntryImpl<'a, K, V> = indexmap::map::VacantEntry<'a, K, V>;
    pub type IterImpl<'a, K, V> = indexmap::map::Iter<'a, K, V>;
    pub type IterMutImpl<'a, K, V> = indexmap::map::IterMut<'a, K, V>;
    pub type IntoIterImpl<K, V> = indexmap::map::IntoIter<K, V>;
    pub type KeysImpl<'a, K, V> = indexmap::map::Keys<'a, K, V>;
    pub type ValuesImpl<'a, K, V> = indexmap::map::Values<'a, K, V>;
    pub type ValuesMutImpl<'a, K, V> = indexmap::map::ValuesMut<'a, K, V>;
}

#[cfg(any(
    not(feature = "hash-collections"),
    feature = "prefer-btree-collections"
))]
mod impls {
    pub type IndexMapImpl<K, V> = super::IndexMap<K, V>;
    pub type EntryImpl<'a, K, V> = super::Entry<'a, K, V>;
    pub type OccupiedEntryImpl<'a, K, V> = super::OccupiedEntry<'a, K, V>;
    pub type VacantEntryImpl<'a, K, V> = super::VacantEntry<'a, K, V>;
    pub type IterImpl<'a, K, V> = super::Iter<'a, K, V>;
    pub type IterMutImpl<'a, K, V> = super::IterMut<'a, K, V>;
    pub type IntoIterImpl<K, V> = super::IntoIter<K, V>;
    pub type KeysImpl<'a, K, V> = super::Keys<'a, K, V>;
    pub type ValuesImpl<'a, K, V> = super::Values<'a, K, V>;
    pub type ValuesMutImpl<'a, K, V> = super::ValuesMut<'a, K, V>;
}

pub use self::impls::*;

use alloc::collections::{btree_map, BTreeMap};
use alloc::vec::IntoIter as VecIntoIter;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt;
use core::iter::FusedIterator;
use core::mem::replace;
use core::ops::{Index, IndexMut};
use core::slice::Iter as SliceIter;
use core::slice::IterMut as SliceIterMut;

/// A slot index referencing a slot in an [`IndexMap`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SlotIndex(usize);

impl SlotIndex {
    /// Returns the raw `usize` index of the [`SlotIndex`].
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Slot<K, V> {
    /// The key of the [`Slot`].
    key: K,
    /// The value of the [`Slot`].
    value: V,
}

impl<K, V> Slot<K, V> {
    /// Creates a new [`Slot`] from the given `key` and `value`.
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    /// Returns the [`Slot`] as a pair of references to its `key` and `value`.
    pub fn as_pair(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }

    /// Returns the [`Slot`] as a pair of references to its `key` and `value`.
    pub fn as_pair_mut(&mut self) -> (&K, &mut V) {
        (&self.key, &mut self.value)
    }

    /// Converts the [`Slot`] into a pair of its `key` and `value`.
    pub fn into_pair(self) -> (K, V) {
        (self.key, self.value)
    }

    /// Returns a shared reference to the key of the [`Slot`].
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Returns a shared reference to the value of the [`Slot`].
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Returns an exclusive reference to the value of the [`Slot`].
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

/// A b-tree map where the iteration order of the key-value
/// pairs is independent of the ordering of the keys.
///
/// The interface is closely compatible with the [`indexmap` crate]
/// and a subset of the features that is relevant for the
/// [`wasmparser-nostd` crate].
///
/// # Differences to original `IndexMap`
///
/// Since the goal of this crate was to maintain a simple
/// `no_std` compatible fork of the [`indexmap` crate] there are some
/// downsides and differences.
///
/// - Some operations such as `IndexMap::insert` now require `K: Clone`.
/// - It is to be expected that this fork performs worse than the original
/// [`indexmap` crate] implementation.
/// - The implementation is based on `BTreeMap` internally instead of
/// `HashMap` which has the effect that methods no longer require `K: Hash`
/// but `K: Ord` instead.
///
/// [`indexmap` crate]: https://crates.io/crates/indexmap
/// [`wasmparser-nostd` crate]: https://crates.io/crates/wasmparser-nostd
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexMap<K, V> {
    /// A mapping from keys to slot indices.
    key2slot: BTreeMap<K, SlotIndex>,
    /// A vector holding all slots of key value pairs.
    slots: Vec<Slot<K, V>>,
}

impl<K, V> Default for IndexMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> IndexMap<K, V> {
    /// Makes a new, empty [`IndexMap`].
    ///
    /// Does not allocate anything on its own.
    pub fn new() -> Self {
        Self {
            key2slot: BTreeMap::new(),
            slots: Vec::new(),
        }
    }

    /// Constructs a new, empty [`IndexMap`] with at least the specified capacity.
    ///
    /// Does not allocate if `capacity` is zero.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            key2slot: BTreeMap::new(),
            slots: Vec::with_capacity(capacity),
        }
    }

    /// Reserve capacity for at least `additional` more key-value pairs.
    pub fn reserve(&mut self, additional: usize) {
        self.slots.reserve(additional);
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map’s key type,
    /// but the ordering on the borrowed form must match the ordering on the key type.
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.key2slot.contains_key(key)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord + Clone,
    {
        self.insert_full(key, value).1
    }

    /// Inserts a key-value pair into the map.
    ///
    /// Returns the unique index to the key-value pair alongside the previous value.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Ord + Clone,
    {
        match self.key2slot.entry(key.clone()) {
            btree_map::Entry::Vacant(entry) => {
                let index = self.slots.len();
                entry.insert(SlotIndex(index));
                self.slots.push(Slot::new(key, value));
                (index, None)
            }
            btree_map::Entry::Occupied(entry) => {
                let index = entry.get().index();
                let new_slot = Slot::new(key, value);
                let old_slot = replace(&mut self.slots[index], new_slot);
                (index, Some(old_slot.value))
            }
        }
    }

    /// Remove the key-value pair equivalent to `key` and return it and
    /// the index it had.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with the
    /// last element of the map and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `key` is not in map.
    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
    {
        self.swap_remove_full(key)
            .map(|(_index, _key, value)| value)
    }

    /// Remove and return the key-value pair equivalent to `key`.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with the
    /// last element of the map and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// [`Vec::swap_remove`]: alloc::vec::Vec::swap_remove
    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
    {
        self.swap_remove_full(key)
            .map(|(_index, key, value)| (key, value))
    }

    /// Remove the key-value pair equivalent to `key` and return it and
    /// the index it had.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with the
    /// last element of the map and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `key` is not in map.
    pub fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
    {
        let index = self.key2slot.remove(key)?.0;
        let removed = self.slots.swap_remove(index);
        if index != self.len() {
            // If the index was referring the last element
            // `swap_remove` would not swap any other element
            // thus adjustments are only needed if this was not the case.
            let swapped = self.slots[index].key.borrow();
            let swapped_index = self
                .key2slot
                .get_mut(swapped)
                .expect("the swapped entry's key must be present");
            *swapped_index = SlotIndex(index);
        }
        Some((index, removed.key, removed.value))
    }

    /// Gets the given key’s corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: K) -> Entry<K, V>
    where
        K: Ord + Clone,
    {
        match self.key2slot.entry(key) {
            btree_map::Entry::Vacant(entry) => Entry::Vacant(VacantEntry {
                vacant: entry,
                slots: &mut self.slots,
            }),
            btree_map::Entry::Occupied(entry) => Entry::Occupied(OccupiedEntry {
                occupied: entry,
                slots: &mut self.slots,
            }),
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type,
    /// but the ordering on the borrowed form must match the ordering on the key type.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
    {
        self.key2slot
            .get(key)
            .map(|slot| &self.slots[slot.index()].value)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type,
    /// but the ordering on the borrowed form must match the ordering on the key type.
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.key2slot
            .get(key)
            .map(|slot| &mut self.slots[slot.index()].value)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type,
    /// but the ordering on the borrowed form *must* match the ordering
    /// on the key type.
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.key2slot
            .get_key_value(key)
            .map(|(key, slot)| (key, &self.slots[slot.index()].value))
    }

    /// Returns the key-value pair corresponding to the supplied key
    /// as well as the unique index of the returned key-value pair.
    ///
    /// The supplied key may be any borrowed form of the map's key type,
    /// but the ordering on the borrowed form *must* match the ordering
    /// on the key type.
    pub fn get_full<Q: ?Sized>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.key2slot.get_key_value(key).map(|(key, slot)| {
            let index = slot.index();
            let value = &self.slots[index].value;
            (index, key, value)
        })
    }

    /// Returns the unique index corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type,
    /// but the ordering on the borrowed form *must* match the ordering
    /// on the key type.
    pub fn get_index_of<Q: ?Sized>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        self.key2slot.get(key).copied().map(SlotIndex::index)
    }

    /// Returns a shared reference to the key-value pair at the given index.
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.slots.get(index).map(Slot::as_pair)
    }

    /// Returns an exclusive reference to the key-value pair at the given index.
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.slots.get_mut(index).map(Slot::as_pair_mut)
    }

    /// Gets an iterator over the entries of the map, sorted by key.
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            iter: self.slots.iter(),
        }
    }

    /// Gets a mutable iterator over the entries of the map, sorted by key.
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            iter: self.slots.iter_mut(),
        }
    }

    /// Gets an iterator over the values of the map, in order by key.
    pub fn keys(&self) -> Keys<K, V> {
        Keys {
            iter: self.slots.iter(),
        }
    }

    /// Gets an iterator over the values of the map, in order by key.
    pub fn values(&self) -> Values<K, V> {
        Values {
            iter: self.slots.iter(),
        }
    }

    /// Gets a mutable iterator over the values of the map, in order by key.
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut {
            iter: self.slots.iter_mut(),
        }
    }

    /// Clears the map, removing all elements.
    pub fn clear(&mut self) {
        self.key2slot.clear();
        self.slots.clear();
    }
}

impl<'a, K, Q, V> Index<&'a Q> for IndexMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: ?Sized + Ord,
{
    type Output = V;

    fn index(&self, key: &'a Q) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}

impl<K, V> Index<usize> for IndexMap<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        let (_key, value) = self
            .get_index(index)
            .expect("IndexMap: index out of bounds");
        value
    }
}

impl<K, V> IndexMut<usize> for IndexMap<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let (_key, value) = self
            .get_index_mut(index)
            .expect("IndexMap: index out of bounds");
        value
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for IndexMap<K, V>
where
    K: Ord + Copy,
    V: Copy,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (&'a K, &'a V)>,
    {
        self.extend(iter.into_iter().map(|(key, value)| (*key, *value)))
    }
}

impl<K, V> Extend<(K, V)> for IndexMap<K, V>
where
    K: Ord + Clone,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
    {
        iter.into_iter().for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }
}

impl<K, V> FromIterator<(K, V)> for IndexMap<K, V>
where
    K: Ord + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        let mut map = IndexMap::new();
        map.extend(iter);
        map
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for IndexMap<K, V>
where
    K: Ord + Clone,
{
    fn from(items: [(K, V); N]) -> Self {
        items.into_iter().collect()
    }
}

impl<'a, K, V> IntoIterator for &'a IndexMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut IndexMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for IndexMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.slots.into_iter(),
        }
    }
}

/// An iterator over the entries of an [`IndexMap`].
///
/// This `struct` is created by the [`iter`] method on [`IndexMap`]. See its
/// documentation for more.
///
/// [`iter`]: IndexMap::iter
#[derive(Debug, Clone)]
pub struct Iter<'a, K, V> {
    iter: SliceIter<'a, Slot<K, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Slot::as_pair)
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Slot::as_pair)
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for Iter<'a, K, V> {}

/// A mutable iterator over the entries of an [`IndexMap`].
///
/// This `struct` is created by the [`iter_mut`] method on [`IndexMap`]. See its
/// documentation for more.
///
/// [`iter_mut`]: IndexMap::iter_mut
#[derive(Debug)]
pub struct IterMut<'a, K, V> {
    iter: SliceIterMut<'a, Slot<K, V>>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Slot::as_pair_mut)
    }
}

impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Slot::as_pair_mut)
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for IterMut<'a, K, V> {}

/// An owning iterator over the entries of a [`IndexMap`].
///
/// This `struct` is created by the [`into_iter`] method on [`IndexMap`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
///
/// [`into_iter`]: IntoIterator::into_iter
/// [`IntoIterator`]: core::iter::IntoIterator
#[derive(Debug)]
pub struct IntoIter<K, V> {
    iter: VecIntoIter<Slot<K, V>>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Slot::into_pair)
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Slot::into_pair)
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> FusedIterator for IntoIter<K, V> {}

/// An iterator over the keys of an [`IndexMap`].
///
/// This `struct` is created by the [`keys`] method on [`IndexMap`]. See its
/// documentation for more.
///
/// [`keys`]: IndexMap::keys
#[derive(Debug, Clone)]
pub struct Keys<'a, K, V> {
    iter: SliceIter<'a, Slot<K, V>>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Slot::key)
    }
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Slot::key)
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for Keys<'a, K, V> {}

/// An iterator over the values of an [`IndexMap`].
///
/// This `struct` is created by the [`values`] method on [`IndexMap`]. See its
/// documentation for more.
///
/// [`values`]: IndexMap::values
#[derive(Debug, Clone)]
pub struct Values<'a, K, V> {
    iter: SliceIter<'a, Slot<K, V>>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Slot::value)
    }
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Slot::value)
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for Values<'a, K, V> {}

/// An iterator over the values of an [`IndexMap`].
///
/// This `struct` is created by the [`values`] method on [`IndexMap`]. See its
/// documentation for more.
///
/// [`values`]: IndexMap::values
#[derive(Debug)]
pub struct ValuesMut<'a, K, V> {
    iter: SliceIterMut<'a, Slot<K, V>>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Slot::value_mut)
    }
}

impl<'a, K, V> DoubleEndedIterator for ValuesMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Slot::value_mut)
    }
}

impl<'a, K, V> ExactSizeIterator for ValuesMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for ValuesMut<'a, K, V> {}

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This `enum` is constructed from the [`entry`] method on [`IndexMap`].
///
/// [`entry`]: IndexMap::entry
pub enum Entry<'a, K, V> {
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V>),
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),
}

impl<'a, K: Ord, V> Entry<'a, K, V> {
    /// Ensures a value is in the entry by inserting the default if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert(self, default: V) -> &'a mut V
    where
        K: Clone,
    {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result
    /// of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V
    where
        K: Clone,
    {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is in the entry by inserting,
    /// if empty, the result of the default function.
    ///
    /// This method allows for generating key-derived values for
    /// insertion by providing the default function a reference
    /// to the key that was moved during the `.entry(key)` method call.
    ///
    /// The reference to the moved key is provided
    /// so that cloning or copying the key is
    /// unnecessary, unlike with `.or_insert_with(|| ... )`.
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V
    where
        K: Clone,
    {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }

    /// Returns a reference to this entry’s key.
    pub fn key(&self) -> &K {
        match *self {
            Self::Occupied(ref entry) => entry.key(),
            Self::Vacant(ref entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry
    /// before any potential inserts into the map.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Self::Occupied(mut entry) => {
                f(entry.get_mut());
                Self::Occupied(entry)
            }
            Self::Vacant(entry) => Self::Vacant(entry),
        }
    }
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Ord + Clone,
    V: Default,
{
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_default(self) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}

impl<'a, K, V> fmt::Debug for Entry<'a, K, V>
where
    K: fmt::Debug + Ord,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Vacant(entry) => entry.fmt(f),
            Entry::Occupied(entry) => entry.fmt(f),
        }
    }
}

/// A view into a vacant entry in an [`IndexMap`]. It is part of the [`Entry`] `enum`.
pub struct VacantEntry<'a, K, V> {
    /// The underlying vacant entry.
    vacant: btree_map::VacantEntry<'a, K, SlotIndex>,
    /// The vector that stores all slots.
    slots: &'a mut Vec<Slot<K, V>>,
}

impl<'a, K, V> VacantEntry<'a, K, V>
where
    K: Ord,
{
    /// Gets a reference to the key that would be used when inserting a value through the VacantEntry.
    pub fn key(&self) -> &K {
        self.vacant.key()
    }

    /// Take ownership of the key.
    pub fn into_key(self) -> K {
        self.vacant.into_key()
    }

    /// Sets the value of the entry with the `VacantEntry`’s key,
    /// and returns a mutable reference to it.
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Clone,
    {
        let index = self.slots.len();
        let key = self.vacant.key().clone();
        self.vacant.insert(SlotIndex(index));
        self.slots.push(Slot::new(key, value));
        &mut self.slots[index].value
    }
}

impl<'a, K, V> fmt::Debug for VacantEntry<'a, K, V>
where
    K: fmt::Debug + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VacantEntry")
            .field("key", self.key())
            .finish()
    }
}

/// A view into an occupied entry in a [`IndexMap`]. It is part of the [`Entry`] `enum`.
pub struct OccupiedEntry<'a, K, V> {
    /// The underlying occupied entry.
    occupied: btree_map::OccupiedEntry<'a, K, SlotIndex>,
    /// The vector that stores all slots.
    slots: &'a mut Vec<Slot<K, V>>,
}

impl<'a, K, V> OccupiedEntry<'a, K, V>
where
    K: Ord,
{
    /// Gets a reference to the key in the entry.
    pub fn key(&self) -> &K {
        self.occupied.key()
    }

    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        let index = self.occupied.get().index();
        &self.slots[index].value
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` that may outlive the
    /// destruction of the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: OccupiedEntry::into_mut
    pub fn get_mut(&mut self) -> &mut V {
        let index = self.occupied.get().index();
        &mut self.slots[index].value
    }

    /// Converts the entry into a mutable reference to its value.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: OccupiedEntry::get_mut
    pub fn into_mut(self) -> &'a mut V {
        let index = self.occupied.get().index();
        &mut self.slots[index].value
    }

    /// Sets the value of the entry with the `OccupiedEntry`’s key,
    /// and returns the entry’s old value.
    pub fn insert(&mut self, value: V) -> V
    where
        K: Clone,
    {
        let index = self.occupied.get().index();
        let key = self.key().clone();
        let new_slot = Slot::new(key, value);
        let old_slot = replace(&mut self.slots[index], new_slot);
        old_slot.value
    }
}

impl<'a, K, V> fmt::Debug for OccupiedEntry<'a, K, V>
where
    K: fmt::Debug + Ord,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::IndexMap;
    use core::fmt;
    use core::marker::PhantomData;
    use serde::de::{Deserialize, MapAccess, Visitor};
    use serde::ser::{Serialize, SerializeMap, Serializer};

    impl<K, V> Serialize for IndexMap<K, V>
    where
        K: Serialize + Ord,
        V: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.len()))?;
            for (k, v) in self.iter() {
                map.serialize_entry(k, v)?;
            }
            map.end()
        }
    }

    impl<'a, K, V> Deserialize<'a> for IndexMap<K, V>
    where
        K: Deserialize<'a> + Clone + Ord,
        V: Deserialize<'a>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'a>,
        {
            deserializer.deserialize_map(IndexMapVisitor {
                _marker: PhantomData,
            })
        }
    }

    struct IndexMapVisitor<K, V> {
        _marker: PhantomData<fn() -> IndexMap<K, V>>,
    }

    impl<'de, K, V> Visitor<'de> for IndexMapVisitor<K, V>
    where
        K: Deserialize<'de> + Clone + Ord,
        V: Deserialize<'de>,
    {
        type Value = IndexMap<K, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = IndexMap::with_capacity(access.size_hint().unwrap_or(0));

            while let Some((key, value)) = access.next_entry()? {
                map.insert(key, value);
            }

            Ok(map)
        }
    }
}
