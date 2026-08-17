//! Type definitions for an ordered map.

use core::borrow::Borrow;
use core::hash::Hash;
use core::iter::FusedIterator;
use core::ops::Index;

mod detail;

#[cfg(test)]
mod tests;

/// A hash table where the iteration order of the key-value pairs is independent of the hash values of the keys.
///
/// Provides an API compatible with both [`IndexMap`] and a custom implementation based on [`BTreeMap`].
///
/// [`IndexMap`]: indexmap::IndexMap
/// [`BTreeMap`]: alloc::collections::BTreeMap
#[derive(Debug, Clone)]
pub struct IndexMap<K, V> {
    inner: detail::IndexMapImpl<K, V>,
}

impl<K, V> Default for IndexMap<K, V> {
    #[inline]
    fn default() -> Self {
        Self {
            inner: detail::IndexMapImpl::default(),
        }
    }
}

impl<K, V> IndexMap<K, V> {
    /// Clears the [`IndexMap`], removing all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Returns the number of elements in the [`IndexMap`].
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the [`IndexMap`] contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator that yields the items in the [`IndexMap`].
    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    /// Returns an iterator that yields the mutable items in the [`IndexMap`].
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            inner: self.inner.iter_mut(),
        }
    }

    /// Returns an iterator that yields the keys in the [`IndexMap`].
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            inner: self.inner.keys(),
        }
    }

    /// Returns an iterator that yields the values in the [`IndexMap`].
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values {
            inner: self.inner.values(),
        }
    }

    /// Returns a mutable iterator that yields the values in the [`IndexMap`].
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            inner: self.inner.values_mut(),
        }
    }

    /// Returns the key-value entry at the given `index` if any.
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.inner.get_index(index)
    }

    /// Returns the mutable key-value entry at the given `index` if any.
    #[inline]
    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.inner.get_index_mut(index)
    }
}

impl<K, V> IndexMap<K, V>
where
    K: Hash + Eq + Ord + Clone,
{
    /// Reserves capacity for at least `additional` more elements to be inserted in the [`IndexMap`].
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Returns true if `key` is contains in the [`IndexMap`].
    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        self.inner.contains_key(key)
    }

    /// Returns a reference to the value corresponding to the `key`.
    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        self.inner.get(key)
    }

    /// Return references to the key-value pair stored for `key`,
    /// if it is present, else `None`.
    #[inline]
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        self.inner.get_key_value(key)
    }

    /// Returns the key-value pair corresponding to the supplied key
    /// as well as the unique index of the returned key-value pair.
    ///
    /// The supplied key may be any borrowed form of the map's key type,
    /// but the ordering on the borrowed form *must* match the ordering
    /// on the key type.
    #[inline]
    pub fn get_full<Q: ?Sized>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        K: Borrow<Q> + Ord,
        Q: Hash + Eq + Ord,
    {
        self.inner.get_full(key)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        self.inner.get_mut(key)
    }

    /// Inserts a key-value pair into the [`IndexMap`].
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// Remove the key-value pair equivalent to `key` and return its value.
    ///
    /// Like [`Vec::swap_remove`], the pair is removed by swapping it with the
    /// last element of the map and popping it off. **This perturbs
    /// the position of what used to be the last element!**
    ///
    /// Return `None` if `key` is not in map.
    ///
    /// [`Vec::swap_remove`]: alloc::vec::Vec::swap_remove
    #[inline]
    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.swap_remove(key)
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
    #[inline]
    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.swap_remove_entry(key)
    }

    /// Gets the given key's corresponding entry in the [`IndexMap`] for in-place manipulation.
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.inner.entry(key) {
            detail::EntryImpl::Occupied(entry) => Entry::Occupied(OccupiedEntry { inner: entry }),
            detail::EntryImpl::Vacant(entry) => Entry::Vacant(VacantEntry { inner: entry }),
        }
    }
}

impl<K, Q, V> Index<&Q> for IndexMap<K, V>
where
    K: Borrow<Q> + Hash + Eq + Ord,
    Q: ?Sized + Hash + Eq + Ord,
{
    type Output = V;

    #[inline]
    fn index(&self, key: &Q) -> &V {
        &self.inner[key]
    }
}

impl<K, V> Index<usize> for IndexMap<K, V>
where
    K: Hash + Eq + Ord,
{
    type Output = V;

    #[inline]
    fn index(&self, key: usize) -> &V {
        &self.inner[key]
    }
}

impl<K, V> Extend<(K, V)> for IndexMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
{
    #[inline]
    fn extend<Iter: IntoIterator<Item = (K, V)>>(&mut self, iter: Iter) {
        self.inner.extend(iter)
    }
}

/// A view into a single entry in a [`IndexMap`], which may either be vacant or occupied.
///
/// This enum is constructed from the entry method on [`IndexMap`].
#[derive(Debug)]
pub enum Entry<'a, K: Ord, V> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + Eq + Ord + Clone,
{
    /// Returns a reference to this entry's key.
    #[inline]
    pub fn key(&self) -> &K {
        match *self {
            Self::Occupied(ref entry) => entry.key(),
            Self::Vacant(ref entry) => entry.key(),
        }
    }
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + Eq + Ord + Clone,
    V: Default,
{
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    #[inline]
    pub fn or_default(self) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}

/// A view into an occupied entry in a [`IndexMap`].
///
/// It is part of the [`Entry`] enum.
#[derive(Debug)]
pub struct OccupiedEntry<'a, K: Ord, V> {
    inner: detail::OccupiedEntryImpl<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a, K, V>
where
    K: Ord + Clone,
{
    /// Gets a reference to the key in the entry.
    #[inline]
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Gets a reference to the value in the entry.
    #[inline]
    pub fn get(&self) -> &V {
        self.inner.get()
    }

    /// Gets a mutable reference to the value in the entry.
    #[inline]
    pub fn get_mut(&mut self) -> &mut V {
        self.inner.get_mut()
    }

    /// Sets the value of the entry with the [`OccupiedEntry`]'s key, and returns the entry's old value.
    #[inline]
    pub fn insert(&mut self, value: V) -> V {
        self.inner.insert(value)
    }

    /// Converts the [`OccupiedEntry`] into a mutable reference to the value in the entry
    /// with a lifetime bound to the map itself.
    #[inline]
    pub fn into_mut(self) -> &'a mut V {
        self.inner.into_mut()
    }
}

/// A view into a vacant entry in a [`IndexMap`].
///
/// It is part of the [`Entry`] enum.
#[derive(Debug)]
pub struct VacantEntry<'a, K: Ord, V> {
    inner: detail::VacantEntryImpl<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V>
where
    K: Ord + Clone,
{
    /// Gets a reference to the key in the entry.
    #[inline]
    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /// Take ownership of the key.
    #[inline]
    pub fn into_key(self) -> K {
        self.inner.into_key()
    }

    /// Sets the value of the entry with the [`VacantEntry`]'s key, and returns a mutable reference to it.
    #[inline]
    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Hash,
    {
        self.inner.insert(value)
    }
}

impl<K, V> FromIterator<(K, V)> for IndexMap<K, V>
where
    K: Hash + Ord + Eq + Clone,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        Self {
            inner: <detail::IndexMapImpl<K, V>>::from_iter(iter),
        }
    }
}

impl<'a, K, V> IntoIterator for &'a IndexMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the items of a [`IndexMap`].
#[derive(Debug, Clone)]
pub struct Iter<'a, K, V> {
    inner: detail::IterImpl<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> FusedIterator for Iter<'a, K, V> {}

impl<'a, K, V> IntoIterator for &'a mut IndexMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// An iterator over the mutable items of a [`IndexMap`].
#[derive(Debug)]
pub struct IterMut<'a, K, V> {
    inner: detail::IterMutImpl<'a, K, V>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> FusedIterator for IterMut<'a, K, V> {}

impl<K, V> IntoIterator for IndexMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}

/// An iterator over the owned items of an [`IndexMap`].
#[derive(Debug)]
pub struct IntoIter<K, V> {
    inner: detail::IntoIterImpl<K, V>,
}

impl<'a, K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> ExactSizeIterator for IntoIter<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> FusedIterator for IntoIter<K, V> {}

/// An iterator over the keys of a [`IndexMap`].
#[derive(Debug, Clone)]
pub struct Keys<'a, K, V> {
    inner: detail::KeysImpl<'a, K, V>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> FusedIterator for Keys<'a, K, V> {}

/// An iterator over the values of a [`IndexMap`].
#[derive(Debug, Clone)]
pub struct Values<'a, K, V> {
    inner: detail::ValuesImpl<'a, K, V>,
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> FusedIterator for Values<'a, K, V> {}

/// An mutable iterator over the values of a [`IndexMap`].
#[derive(Debug)]
pub struct ValuesMut<'a, K, V> {
    inner: detail::ValuesMutImpl<'a, K, V>,
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K, V> ExactSizeIterator for ValuesMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V> FusedIterator for ValuesMut<'a, K, V> {}

#[cfg(feature = "serde")]
impl<K, V> serde::Serialize for IndexMap<K, V>
where
    K: serde::Serialize + Eq + Hash + Ord,
    V: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serde::Serialize::serialize(&self.inner, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'a, K, V> serde::Deserialize<'a> for IndexMap<K, V>
where
    K: serde::Deserialize<'a> + Eq + Hash + Ord + Clone,
    V: serde::Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        Ok(IndexMap {
            inner: serde::Deserialize::deserialize(deserializer)?,
        })
    }
}

impl<K, V> PartialEq for IndexMap<K, V>
where
    K: PartialEq + Hash + Ord,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn ne(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

impl<K, V> Eq for IndexMap<K, V>
where
    K: Eq + Hash + Ord,
    V: Eq,
{
}
