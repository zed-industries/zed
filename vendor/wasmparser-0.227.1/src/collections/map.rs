//! Type definitions for a default map.

use core::fmt::Debug;
use core::{borrow::Borrow, hash::Hash, iter::FusedIterator, ops::Index};

#[cfg(all(
    feature = "hash-collections",
    not(feature = "prefer-btree-collections")
))]
mod detail {
    use crate::collections::hash;
    use hashbrown::hash_map;

    pub type MapImpl<K, V> = hash_map::HashMap<K, V, hash::RandomState>;
    pub type EntryImpl<'a, K, V> = hash_map::Entry<'a, K, V, hash::RandomState>;
    pub type OccupiedEntryImpl<'a, K, V> = hash_map::OccupiedEntry<'a, K, V, hash::RandomState>;
    pub type VacantEntryImpl<'a, K, V> = hash_map::VacantEntry<'a, K, V, hash::RandomState>;
    pub type IterImpl<'a, K, V> = hash_map::Iter<'a, K, V>;
    pub type IterMutImpl<'a, K, V> = hash_map::IterMut<'a, K, V>;
    pub type IntoIterImpl<K, V> = hash_map::IntoIter<K, V>;
    pub type KeysImpl<'a, K, V> = hash_map::Keys<'a, K, V>;
    pub type ValuesImpl<'a, K, V> = hash_map::Values<'a, K, V>;
    pub type ValuesMutImpl<'a, K, V> = hash_map::ValuesMut<'a, K, V>;
    pub type IntoKeysImpl<K, V> = hash_map::IntoKeys<K, V>;
    pub type IntoValuesImpl<K, V> = hash_map::IntoValues<K, V>;
}

#[cfg(any(
    not(feature = "hash-collections"),
    feature = "prefer-btree-collections"
))]
mod detail {
    use alloc::collections::btree_map;

    pub type MapImpl<K, V> = btree_map::BTreeMap<K, V>;
    pub type EntryImpl<'a, K, V> = btree_map::Entry<'a, K, V>;
    pub type OccupiedEntryImpl<'a, K, V> = btree_map::OccupiedEntry<'a, K, V>;
    pub type VacantEntryImpl<'a, K, V> = btree_map::VacantEntry<'a, K, V>;
    pub type IterImpl<'a, K, V> = btree_map::Iter<'a, K, V>;
    pub type IterMutImpl<'a, K, V> = btree_map::IterMut<'a, K, V>;
    pub type IntoIterImpl<K, V> = btree_map::IntoIter<K, V>;
    pub type KeysImpl<'a, K, V> = btree_map::Keys<'a, K, V>;
    pub type ValuesImpl<'a, K, V> = btree_map::Values<'a, K, V>;
    pub type ValuesMutImpl<'a, K, V> = btree_map::ValuesMut<'a, K, V>;
    pub type IntoKeysImpl<K, V> = btree_map::IntoKeys<K, V>;
    pub type IntoValuesImpl<K, V> = btree_map::IntoValues<K, V>;
}

/// A default key-value mapping.
///
/// Provides an API compatible with both [`HashMap`] and [`BTreeMap`].
///
/// [`HashMap`]: hashbrown::HashMap
/// [`BTreeMap`]: alloc::collections::BTreeMap
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    inner: detail::MapImpl<K, V>,
}

impl<K, V> Default for Map<K, V> {
    #[inline]
    fn default() -> Self {
        Self {
            inner: detail::MapImpl::default(),
        }
    }
}

impl<K, V> Map<K, V> {
    /// Creates a new empty [`Map`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Clears the [`Map`], removing all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Returns the number of elements in the [`Map`].
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the [`Map`] contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns an iterator that yields the items in the [`Map`].
    #[inline]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    /// Returns a mutable iterator that yields the items in the [`Map`].
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            inner: self.inner.iter_mut(),
        }
    }

    /// Returns an iterator that yields the keys in the [`Map`].
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            inner: self.inner.keys(),
        }
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order.
    ///
    /// The [`Map`] cannot be used after calling this.
    /// The iterator element type is `K`.
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        IntoKeys {
            inner: self.inner.into_keys(),
        }
    }

    /// Returns an iterator that yields the values in the [`Map`].
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        Values {
            inner: self.inner.values(),
        }
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order.
    ///
    /// The [`Map`] cannot be used after calling this.
    /// The iterator element type is `V`.
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V> {
        IntoValues {
            inner: self.inner.into_values(),
        }
    }

    /// Returns a mutable iterator that yields the values in the [`Map`].
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut {
            inner: self.inner.values_mut(),
        }
    }
}

impl<K, V> Map<K, V>
where
    K: Hash + Eq + Ord,
{
    /// Reserves capacity for at least `additional` more elements to be inserted in the [`Map`].
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        #[cfg(all(
            feature = "hash-collections",
            not(feature = "prefer-btree-collections")
        ))]
        self.inner.reserve(additional);
        #[cfg(any(
            not(feature = "hash-collections"),
            feature = "prefer-btree-collections"
        ))]
        let _ = additional;
    }

    /// Returns true if `key` is contains in the [`Map`].
    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.contains_key(key)
    }

    /// Returns a reference to the value corresponding to the `key`.
    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.get(key)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.get_key_value(key)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    #[inline]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.get_mut(key)
    }

    /// Inserts a key-value pair into the [`Map`].
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

    /// Removes a key from the [`Map`], returning the value at the key if the key was previously in the map.
    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq + Ord,
    {
        self.inner.remove(key)
    }

    /// Removes a key from the [`Map`], returning the stored key and value if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but the ordering
    /// on the borrowed form *must* match the ordering on the key type.
    #[inline]
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Ord,
    {
        self.inner.remove_entry(key)
    }

    /// Gets the given key's corresponding entry in the [`Map`] for in-place manipulation.
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.inner.entry(key) {
            detail::EntryImpl::Occupied(entry) => Entry::Occupied(OccupiedEntry { inner: entry }),
            detail::EntryImpl::Vacant(entry) => Entry::Vacant(VacantEntry { inner: entry }),
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` for which `f(&k, &mut v)` returns `false`.
    /// The elements are visited in ascending key order.
    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.retain(f)
    }
}

impl<K, V> PartialEq for Map<K, V>
where
    K: Eq + Hash,
    V: Eq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K, V> Eq for Map<K, V>
where
    K: Eq + Hash,
    V: Eq,
{
}

impl<K, Q, V> Index<&Q> for Map<K, V>
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

impl<'a, K, V> Extend<(&'a K, &'a V)> for Map<K, V>
where
    K: Eq + Hash + Ord + Copy,
    V: Copy,
{
    #[inline]
    fn extend<Iter: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: Iter) {
        self.inner.extend(iter)
    }
}

impl<K, V> Extend<(K, V)> for Map<K, V>
where
    K: Eq + Hash + Ord,
{
    #[inline]
    fn extend<Iter: IntoIterator<Item = (K, V)>>(&mut self, iter: Iter) {
        self.inner.extend(iter)
    }
}

/// A view into a single entry in a [`Map`], which may either be vacant or occupied.
///
/// This enum is constructed from the entry method on [`Map`].
#[derive(Debug)]
pub enum Entry<'a, K: Ord, V> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Hash + Ord,
{
    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    #[inline]
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the [`Entry`] by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    #[inline]
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is in the [`Entry`] by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion by providing the default
    /// function a reference to the key that was moved during the `.entry(key)` method call.
    ///
    /// The reference to the moved key is provided so that cloning or copying the key is
    /// unnecessary, unlike with `.or_insert_with(|| ... )`.
    #[inline]
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }

    /// Returns a reference to this [`Entry`]'s key.
    #[inline]
    pub fn key(&self) -> &K {
        match *self {
            Self::Occupied(ref entry) => entry.key(),
            Self::Vacant(ref entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied [`Entry`] before any
    /// potential inserts into the map.
    #[inline]
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
    K: Hash + Ord,
    V: Default,
{
    /// Ensures a value is in the [`Entry`] by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    #[inline]
    pub fn or_default(self) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(Default::default()),
        }
    }
}

/// A view into an occupied entry in a [`Map`].
///
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, K, V> {
    inner: detail::OccupiedEntryImpl<'a, K, V>,
}

impl<'a, K, V> Debug for OccupiedEntry<'a, K, V>
where
    K: Debug + Ord + 'a,
    V: Debug + 'a,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'a, K, V> OccupiedEntry<'a, K, V>
where
    K: Ord + 'a,
    V: 'a,
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

    /// Take ownership of the key and value from the [`Map`].
    #[inline]
    pub fn remove_entry(self) -> (K, V) {
        self.inner.remove_entry()
    }

    /// Takes the value of the entry out of the [`Map`], and returns it.
    #[inline]
    pub fn remove(self) -> V {
        self.inner.remove()
    }
}

/// A view into a vacant entry in a [`Map`].
///
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, K, V> {
    inner: detail::VacantEntryImpl<'a, K, V>,
}

impl<'a, K, V> Debug for VacantEntry<'a, K, V>
where
    K: Debug + Ord + 'a,
    V: Debug + 'a,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'a, K, V> VacantEntry<'a, K, V>
where
    K: Ord + 'a,
    V: 'a,
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

impl<K, V> FromIterator<(K, V)> for Map<K, V>
where
    K: Hash + Eq + Ord,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        Self {
            inner: <detail::MapImpl<K, V>>::from_iter(iter),
        }
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the items of a [`Map`].
#[derive(Debug, Clone)]
pub struct Iter<'a, K, V> {
    inner: detail::IterImpl<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> Iterator for Iter<'a, K, V> {
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

impl<'a, K: 'a, V: 'a> ExactSizeIterator for Iter<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K: 'a, V: 'a> FusedIterator for Iter<'a, K, V> where
    detail::IterImpl<'a, K, V>: FusedIterator
{
}

impl<'a, K: 'a, V: 'a> IntoIterator for &'a mut Map<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// An iterator over the mutable items of a [`Map`].
#[derive(Debug)]
pub struct IterMut<'a, K, V> {
    inner: detail::IterMutImpl<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> Iterator for IterMut<'a, K, V> {
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

impl<'a, K: 'a, V: 'a> ExactSizeIterator for IterMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K: 'a, V: 'a> FusedIterator for IterMut<'a, K, V> where
    detail::IterMutImpl<'a, K, V>: FusedIterator
{
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
        }
    }
}

/// An iterator over the owned items of an [`Map`].
#[derive(Debug)]
pub struct IntoIter<K, V> {
    inner: detail::IntoIterImpl<K, V>,
}

impl<K, V> Iterator for IntoIter<K, V> {
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

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for IntoIter<K, V> where detail::IntoIterImpl<K, V>: FusedIterator {}

/// An iterator over the keys of a [`Map`].
#[derive(Debug, Clone)]
pub struct Keys<'a, K, V> {
    inner: detail::KeysImpl<'a, K, V>,
}

impl<'a, K: 'a, V> Iterator for Keys<'a, K, V> {
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

impl<'a, K: 'a, V> ExactSizeIterator for Keys<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K: 'a, V> FusedIterator for Keys<'a, K, V> where detail::KeysImpl<'a, K, V>: FusedIterator {}

/// An iterator over the values of a [`Map`].
#[derive(Debug, Clone)]
pub struct Values<'a, K, V> {
    inner: detail::ValuesImpl<'a, K, V>,
}

impl<'a, K, V: 'a> Iterator for Values<'a, K, V> {
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

impl<'a, K, V: 'a> ExactSizeIterator for Values<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V: 'a> FusedIterator for Values<'a, K, V> where
    detail::ValuesImpl<'a, K, V>: FusedIterator
{
}

/// An mutable iterator over the values of a [`Map`].
#[derive(Debug)]
pub struct ValuesMut<'a, K, V> {
    inner: detail::ValuesMutImpl<'a, K, V>,
}

impl<'a, K, V: 'a> Iterator for ValuesMut<'a, K, V> {
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

impl<'a, K, V: 'a> ExactSizeIterator for ValuesMut<'a, K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K, V: 'a> FusedIterator for ValuesMut<'a, K, V> where
    detail::ValuesMutImpl<'a, K, V>: FusedIterator
{
}

/// An iterator over the owned keys of a [`Map`].
#[derive(Debug)]
pub struct IntoKeys<K, V> {
    inner: detail::IntoKeysImpl<K, V>,
}

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<K, V> ExactSizeIterator for IntoKeys<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for IntoKeys<K, V> where detail::IntoKeysImpl<K, V>: FusedIterator {}

/// An iterator over the owned values of a [`Map`].
#[derive(Debug)]
pub struct IntoValues<K, V> {
    inner: detail::IntoValuesImpl<K, V>,
}

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<K, V> ExactSizeIterator for IntoValues<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> FusedIterator for IntoValues<K, V> where detail::IntoValuesImpl<K, V>: FusedIterator {}

#[cfg(feature = "serde")]
impl<K, V> serde::Serialize for Map<K, V>
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
impl<'a, K, V> serde::Deserialize<'a> for Map<K, V>
where
    K: serde::Deserialize<'a> + Eq + Hash + Ord,
    V: serde::Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        Ok(Map {
            inner: serde::Deserialize::deserialize(deserializer)?,
        })
    }
}
