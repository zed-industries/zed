//! A fixed capacity map/dictionary that performs lookups via linear search.
//!
//! Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).

use core::{borrow::Borrow, fmt, iter::FusedIterator, mem, ops, slice};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

use crate::vec::{OwnedVecStorage, Vec, VecInner, ViewVecStorage};

mod storage {
    use crate::vec::{OwnedVecStorage, VecStorage, ViewVecStorage};

    use super::{LinearMapInner, LinearMapView};

    /// Trait defining how data for a [`LinearMap`](super::LinearMap) is stored.
    ///
    /// There's two implementations available:
    ///
    /// - [`OwnedStorage`]: stores the data in an array whose size is known at compile time.
    /// - [`ViewStorage`]: stores the data in an unsized slice
    ///
    /// This allows [`LinearMap`] to be generic over either sized or unsized storage. The
    /// [`linear_map`](super) module contains a [`LinearMapInner`] struct that's generic on
    /// [`LinearMapStorage`], and two type aliases for convenience:
    ///
    /// - [`LinearMap<N>`](crate::linear_map::LinearMap) = `LinearMapInner<OwnedStorage<u8, N>>`
    /// - [`LinearMapView<T>`](crate::linear_map::LinearMapView) = `LinearMapInner<ViewStorage<u8>>`
    ///
    /// `LinearMap` can be unsized into `StrinsgView`, either by unsizing coercions such as `&mut
    /// LinearMap -> &mut LinearMapView` or `Box<LinearMap> -> Box<LinearMapView>`, or
    /// explicitly with [`.as_view()`](crate::linear_map::LinearMap::as_view) or
    /// [`.as_mut_view()`](crate::linear_map::LinearMap::as_mut_view).
    ///
    /// This trait is sealed, so you cannot implement it for your own types. You can only use
    /// the implementations provided by this crate.
    ///
    /// [`LinearMapInner`]: super::LinearMapInner
    /// [`LinearMap`]: super::LinearMap
    /// [`OwnedStorage`]: super::OwnedStorage
    /// [`ViewStorage`]: super::ViewStorage
    pub trait LinearMapStorage<K, V>: LinearMapStorageSealed<K, V> {}
    pub trait LinearMapStorageSealed<K, V>: VecStorage<(K, V)> {
        fn as_linear_map_view(this: &LinearMapInner<K, V, Self>) -> &LinearMapView<K, V>
        where
            Self: LinearMapStorage<K, V>;
        fn as_linear_map_mut_view(
            this: &mut LinearMapInner<K, V, Self>,
        ) -> &mut LinearMapView<K, V>
        where
            Self: LinearMapStorage<K, V>;
    }

    impl<K, V, const N: usize> LinearMapStorage<K, V> for OwnedVecStorage<(K, V), N> {}
    impl<K, V, const N: usize> LinearMapStorageSealed<K, V> for OwnedVecStorage<(K, V), N> {
        fn as_linear_map_view(this: &LinearMapInner<K, V, Self>) -> &LinearMapView<K, V>
        where
            Self: LinearMapStorage<K, V>,
        {
            this
        }
        fn as_linear_map_mut_view(this: &mut LinearMapInner<K, V, Self>) -> &mut LinearMapView<K, V>
        where
            Self: LinearMapStorage<K, V>,
        {
            this
        }
    }

    impl<K, V> LinearMapStorage<K, V> for ViewVecStorage<(K, V)> {}

    impl<K, V> LinearMapStorageSealed<K, V> for ViewVecStorage<(K, V)> {
        fn as_linear_map_view(this: &LinearMapInner<K, V, Self>) -> &LinearMapView<K, V>
        where
            Self: LinearMapStorage<K, V>,
        {
            this
        }
        fn as_linear_map_mut_view(this: &mut LinearMapInner<K, V, Self>) -> &mut LinearMapView<K, V>
        where
            Self: LinearMapStorage<K, V>,
        {
            this
        }
    }
}

pub use storage::LinearMapStorage;
/// Implementation of [`LinearMapStorage`] that stores the data in an array whose size is known at
/// compile time.
pub type OwnedStorage<K, V, const N: usize> = OwnedVecStorage<(K, V), N>;
/// Implementation of [`LinearMapStorage`] that stores the data in an unsized slice.
pub type ViewStorage<K, V> = ViewVecStorage<(K, V)>;

/// Base struct for [`LinearMap`] and [`LinearMapView`]
#[cfg_attr(
    feature = "zeroize",
    derive(Zeroize),
    zeroize(bound = "S: Zeroize, K: Zeroize, V: Zeroize")
)]
pub struct LinearMapInner<K, V, S: LinearMapStorage<K, V> + ?Sized> {
    pub(crate) buffer: VecInner<(K, V), usize, S>,
}

/// A fixed capacity map/dictionary that performs lookups via linear search.
///
/// Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).
pub type LinearMap<K, V, const N: usize> = LinearMapInner<K, V, OwnedStorage<K, V, N>>;

/// A dynamic capacity map/dictionary that performs lookups via linear search.
///
/// Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).
pub type LinearMapView<K, V> = LinearMapInner<K, V, ViewStorage<K, V>>;

impl<K, V, const N: usize> LinearMap<K, V, N> {
    /// Creates an empty `LinearMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// // allocate the map on the stack
    /// let mut map: LinearMap<&str, isize, 8> = LinearMap::new();
    ///
    /// // allocate the map in a static variable
    /// static mut MAP: LinearMap<&str, isize, 8> = LinearMap::new();
    /// ```
    pub const fn new() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl<K, V, S: LinearMapStorage<K, V> + ?Sized> LinearMapInner<K, V, S>
where
    K: Eq,
{
    /// Get a reference to the `LinearMap`, erasing the `N` const-generic.
    pub fn as_view(&self) -> &LinearMapView<K, V> {
        S::as_linear_map_view(self)
    }

    /// Get a mutable reference to the `LinearMap`, erasing the `N` const-generic.
    pub fn as_mut_view(&mut self) -> &mut LinearMapView<K, V> {
        S::as_linear_map_mut_view(self)
    }

    /// Returns the number of elements that the map can hold.
    ///
    /// Computes in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let map: LinearMap<&str, isize, 8> = LinearMap::new();
    /// assert_eq!(map.capacity(), 8);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Clears the map, removing all key-value pairs.
    ///
    /// Computes in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert(1, "a").unwrap();
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// Computes in *O*(n) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert(1, "a").unwrap();
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// Computes in *O*(n) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert(1, "a").unwrap();
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.iter()
            .find(|&(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// Computes in *O*(n) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert(1, "a").unwrap();
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    /// ```
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.iter_mut()
            .find(|&(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    /// Returns the number of elements in this map.
    ///
    /// Computes in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut a: LinearMap<_, _, 8> = LinearMap::new();
    /// assert_eq!(a.len(), 0);
    /// a.insert(1, "a").unwrap();
    /// assert_eq!(a.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old value is returned.
    ///
    /// Computes in *O*(n) time
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// assert_eq!(map.insert(37, "a").unwrap(), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b").unwrap();
    /// assert_eq!(map.insert(37, "c").unwrap(), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    pub fn insert(&mut self, key: K, mut value: V) -> Result<Option<V>, (K, V)> {
        if let Some((_, v)) = self.iter_mut().find(|&(k, _)| *k == key) {
            mem::swap(v, &mut value);
            return Ok(Some(value));
        }

        self.buffer.push((key, value))?;
        Ok(None)
    }

    /// Returns true if the map contains no elements.
    ///
    /// Computes in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut a: LinearMap<_, _, 8> = LinearMap::new();
    /// assert!(a.is_empty());
    /// a.insert(1, "a").unwrap();
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the map is full.
    ///
    /// Computes in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut a: LinearMap<_, _, 4> = LinearMap::new();
    /// assert!(!a.is_full());
    /// a.insert(1, "a").unwrap();
    /// a.insert(2, "b").unwrap();
    /// a.insert(3, "c").unwrap();
    /// a.insert(4, "d").unwrap();
    /// assert!(a.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert("a", 1).unwrap();
    /// map.insert("b", 2).unwrap();
    /// map.insert("c", 3).unwrap();
    ///
    /// for (key, val) in map.iter() {
    ///     println!("key: {} val: {}", key, val);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            iter: self.buffer.as_slice().iter(),
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order,
    /// with mutable references to the values.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert("a", 1).unwrap();
    /// map.insert("b", 2).unwrap();
    /// map.insert("c", 3).unwrap();
    ///
    /// // Update all values
    /// for (_, val) in map.iter_mut() {
    ///     *val = 2;
    /// }
    ///
    /// for (key, val) in &map {
    ///     println!("key: {} val: {}", key, val);
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            iter: self.buffer.as_mut_slice().iter_mut(),
        }
    }

    /// An iterator visiting all keys in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert("a", 1).unwrap();
    /// map.insert("b", 2).unwrap();
    /// map.insert("c", 3).unwrap();
    ///
    /// for key in map.keys() {
    ///     println!("{}", key);
    /// }
    /// ```
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(k, _)| k)
    }

    /// Removes a key from the map, returning the value at
    /// the key if the key was previously in the map.
    ///
    /// Computes in *O*(n) time
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert(1, "a").unwrap();
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let idx = self
            .keys()
            .enumerate()
            .find(|&(_, k)| k.borrow() == key)
            .map(|(idx, _)| idx);

        idx.map(|idx| self.buffer.swap_remove(idx).1)
    }

    /// An iterator visiting all values in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert("a", 1).unwrap();
    /// map.insert("b", 2).unwrap();
    /// map.insert("c", 3).unwrap();
    ///
    /// for val in map.values() {
    ///     println!("{}", val);
    /// }
    /// ```
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, v)| v)
    }

    /// An iterator visiting all values mutably in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::LinearMap;
    ///
    /// let mut map: LinearMap<_, _, 8> = LinearMap::new();
    /// map.insert("a", 1).unwrap();
    /// map.insert("b", 2).unwrap();
    /// map.insert("c", 3).unwrap();
    ///
    /// for val in map.values_mut() {
    ///     *val += 10;
    /// }
    ///
    /// for val in map.values() {
    ///     println!("{}", val);
    /// }
    /// ```
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.iter_mut().map(|(_, v)| v)
    }

    /// Returns an entry for the corresponding key
    /// ```
    /// use heapless::linear_map;
    /// use heapless::LinearMap;
    /// let mut map = LinearMap::<_, _, 16>::new();
    /// if let linear_map::Entry::Vacant(v) = map.entry("a") {
    ///     v.insert(1).unwrap();
    /// }
    /// if let linear_map::Entry::Occupied(mut o) = map.entry("a") {
    ///     println!("found {}", *o.get()); // Prints 1
    ///     o.insert(2);
    /// }
    /// // Prints 2
    /// println!("val: {}", *map.get("a").unwrap());
    /// ```
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        let idx = self
            .keys()
            .enumerate()
            .find(|&(_, k)| *k.borrow() == key)
            .map(|(idx, _)| idx);

        match idx {
            Some(idx) => Entry::Occupied(OccupiedEntry {
                idx,
                map: self.as_mut_view(),
            }),
            None => Entry::Vacant(VacantEntry {
                key,
                map: self.as_mut_view(),
            }),
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` for which `f(&k, &mut v)` returns `false`.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.buffer.retain_mut(|(k, v)| f(k, v));
    }
}

impl<K, V, Q, S: LinearMapStorage<K, V> + ?Sized> ops::Index<&'_ Q> for LinearMapInner<K, V, S>
where
    K: Borrow<Q> + Eq,
    Q: Eq + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl<K, V, Q, S: LinearMapStorage<K, V> + ?Sized> ops::IndexMut<&'_ Q> for LinearMapInner<K, V, S>
where
    K: Borrow<Q> + Eq,
    Q: Eq + ?Sized,
{
    fn index_mut(&mut self, key: &Q) -> &mut V {
        self.get_mut(key).expect("no entry found for key")
    }
}

impl<K, V, const N: usize> Default for LinearMap<K, V, N>
where
    K: Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, const N: usize> Clone for LinearMap<K, V, N>
where
    K: Eq + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
        }
    }
}

impl<K, V, S: LinearMapStorage<K, V> + ?Sized> fmt::Debug for LinearMapInner<K, V, S>
where
    K: Eq + fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V, const N: usize> FromIterator<(K, V)> for LinearMap<K, V, N>
where
    K: Eq,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut out = Self::new();
        out.buffer.extend(iter);
        out
    }
}

/// An iterator that moves out of a [`LinearMap`].
///
/// This struct is created by calling the [`into_iter`](LinearMap::into_iter) method on
/// [`LinearMap`].
pub struct IntoIter<K, V, const N: usize>
where
    K: Eq,
{
    inner: <Vec<(K, V), N, usize> as IntoIterator>::IntoIter,
}

impl<K, V, const N: usize> Iterator for IntoIter<K, V, N>
where
    K: Eq,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<K, V, const N: usize> FusedIterator for IntoIter<K, V, N> where K: Eq {}

impl<K, V, const N: usize> ExactSizeIterator for IntoIter<K, V, N>
where
    K: Eq,
{
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V, const N: usize> IntoIterator for LinearMap<K, V, N>
where
    K: Eq,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.buffer.into_iter(),
        }
    }
}

impl<'a, K, V, S: LinearMapStorage<K, V> + ?Sized> IntoIterator for &'a LinearMapInner<K, V, S>
where
    K: Eq,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the items of a [`LinearMap`]
///
/// This struct is created by calling the [`iter`](LinearMap::iter) method on [`LinearMap`].
#[derive(Clone, Debug)]
pub struct Iter<'a, K, V> {
    iter: slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (k, v))
    }
}

/// An iterator over the items of a [`LinearMap`] that allows modifying the items
///
/// This struct is created by calling the [`iter_mut`](LinearMap::iter_mut) method on [`LinearMap`].
#[derive(Debug)]
pub struct IterMut<'a, K, V> {
    iter: slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (k as &K, v))
    }
}

impl<K, V1, V2, S1: LinearMapStorage<K, V1> + ?Sized, S2: LinearMapStorage<K, V2> + ?Sized>
    PartialEq<LinearMapInner<K, V2, S2>> for LinearMapInner<K, V1, S1>
where
    K: Eq,
    V1: PartialEq<V2>,
{
    fn eq(&self, other: &LinearMapInner<K, V2, S2>) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .all(|(key, value)| other.get(key).is_some_and(|v| *value == *v))
    }
}

impl<K, V, S: LinearMapStorage<K, V> + ?Sized> Eq for LinearMapInner<K, V, S>
where
    K: Eq,
    V: PartialEq,
{
}

/// A view into an entry in the map
pub enum Entry<'a, K, V> {
    /// The entry corresponding to the key `K` exists in the map
    Occupied(OccupiedEntry<'a, K, V>),
    /// The entry corresponding to the key `K` does not exist in the map
    Vacant(VacantEntry<'a, K, V>),
}

/// An occupied entry which can be manipulated
pub struct OccupiedEntry<'a, K, V> {
    // SAFETY: `idx` must not be modified after construction, and
    // the size of `map` must not be changed.
    idx: usize,
    map: &'a mut LinearMapView<K, V>,
}

impl<'a, K, V> OccupiedEntry<'a, K, V>
where
    K: Eq,
{
    /// Gets a reference to the key that this entity corresponds to
    pub fn key(&self) -> &K {
        // SAFETY: Valid idx from OccupiedEntry construction
        let (k, _v) = unsafe { self.map.buffer.get_unchecked(self.idx) };
        k
    }

    /// Removes this entry from the map and yields its corresponding key and value
    pub fn remove_entry(self) -> (K, V) {
        // SAFETY: Valid idx from OccupiedEntry construction
        unsafe { self.map.buffer.swap_remove_unchecked(self.idx) }
    }

    /// Removes this entry from the map and yields its corresponding key and value
    pub fn remove(self) -> V {
        self.remove_entry().1
    }

    /// Gets a reference to the value associated with this entry
    pub fn get(&self) -> &V {
        // SAFETY: Valid idx from OccupiedEntry construction
        let (_k, v) = unsafe { self.map.buffer.get_unchecked(self.idx) };
        v
    }

    /// Gets a mutable reference to the value associated with this entry
    pub fn get_mut(&mut self) -> &mut V {
        // SAFETY: Valid idx from OccupiedEntry construction
        let (_k, v) = unsafe { self.map.buffer.get_unchecked_mut(self.idx) };
        v
    }

    /// Consumes this entry and yields a reference to the underlying value
    pub fn into_mut(self) -> &'a mut V {
        // SAFETY: Valid idx from OccupiedEntry construction
        let (_k, v) = unsafe { self.map.buffer.get_unchecked_mut(self.idx) };
        v
    }

    /// Overwrites the underlying map's value with this entry's value
    pub fn insert(self, value: V) -> V {
        // SAFETY: Valid idx from OccupiedEntry construction
        let (_k, v) = unsafe { self.map.buffer.get_unchecked_mut(self.idx) };
        mem::replace(v, value)
    }
}

/// A view into an empty slot in the underlying map
pub struct VacantEntry<'a, K, V> {
    key: K,
    map: &'a mut LinearMapView<K, V>,
}

impl<'a, K, V> VacantEntry<'a, K, V>
where
    K: Eq,
{
    /// Get the key associated with this entry
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Consumes this entry to yield to key associated with it
    pub fn into_key(self) -> K {
        self.key
    }

    /// Inserts this entry into to underlying map, yields a mutable reference to the inserted value.
    /// If the map is at capacity the value is returned instead.
    pub fn insert(self, value: V) -> Result<&'a mut V, V> {
        self.map
            .buffer
            .push((self.key, value))
            .map_err(|(_k, v)| v)?;
        let idx = self.map.buffer.len() - 1;
        let r = &mut self.map.buffer[idx];
        Ok(&mut r.1)
    }
}

#[cfg(test)]
mod test {
    use static_assertions::assert_not_impl_any;

    use super::{Entry, LinearMap, LinearMapView};

    // Ensure a `LinearMap` containing `!Send` keys stays `!Send` itself.
    assert_not_impl_any!(LinearMap<*const (), (), 4>: Send);
    // Ensure a `LinearMap` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(LinearMap<(), *const (), 4>: Send);

    #[test]
    fn static_new() {
        static mut _L: LinearMap<i32, i32, 8> = LinearMap::new();
    }

    #[test]
    fn borrow() {
        use crate::String;

        let mut map = LinearMap::<_, _, 8>::new();

        let s = String::<64>::try_from("Hello, world!").unwrap();
        map.insert(s, 42).unwrap();

        assert_eq!(map.get("Hello, world!").unwrap(), &42);
    }

    #[test]
    fn partial_eq() {
        {
            let mut a = LinearMap::<_, _, 1>::new();
            a.insert("k1", "v1").unwrap();

            let mut b = LinearMap::<_, _, 2>::new();
            b.insert("k1", "v1").unwrap();

            assert!(a == b);

            b.insert("k2", "v2").unwrap();

            assert!(a != b);
        }

        {
            let mut a = LinearMap::<_, _, 2>::new();
            a.insert("k1", "v1").unwrap();
            a.insert("k2", "v2").unwrap();

            let mut b = LinearMap::<_, _, 2>::new();
            b.insert("k2", "v2").unwrap();
            b.insert("k1", "v1").unwrap();

            assert!(a == b);
        }
    }

    #[test]
    fn drop() {
        droppable!();

        {
            let mut v: LinearMap<i32, Droppable, 2> = LinearMap::new();
            v.insert(0, Droppable::new()).ok().unwrap();
            v.insert(1, Droppable::new()).ok().unwrap();
            v.remove(&1).unwrap();
        }

        assert_eq!(Droppable::count(), 0);

        {
            let mut v: LinearMap<i32, Droppable, 2> = LinearMap::new();
            v.insert(0, Droppable::new()).ok().unwrap();
            v.insert(1, Droppable::new()).ok().unwrap();
        }

        assert_eq!(Droppable::count(), 0);
    }

    #[test]
    fn into_iter() {
        let mut src: LinearMap<_, _, 4> = LinearMap::new();
        src.insert("k1", "v1").unwrap();
        src.insert("k2", "v2").unwrap();
        src.insert("k3", "v3").unwrap();
        src.insert("k4", "v4").unwrap();
        let clone = src.clone();
        for (k, v) in clone.into_iter() {
            assert_eq!(v, src.remove(k).unwrap());
        }
        assert!(src.is_empty());
    }

    #[test]
    fn into_iter_len() {
        let mut src: LinearMap<_, _, 2> = LinearMap::new();
        src.insert("k1", "v1").unwrap();
        src.insert("k2", "v2").unwrap();
        let mut items = src.into_iter();
        assert_eq!(items.len(), 2);
        let _ = items.next();
        assert_eq!(items.len(), 1);
        let _ = items.next();
        assert_eq!(items.len(), 0);
    }

    fn _test_variance_value<'a: 'b, 'b>(
        x: LinearMap<u32, &'a (), 42>,
    ) -> LinearMap<u32, &'b (), 42> {
        x
    }
    fn _test_variance_value_view<'a: 'b, 'b, 'c>(
        x: &'c LinearMapView<u32, &'a ()>,
    ) -> &'c LinearMapView<u32, &'b ()> {
        x
    }
    fn _test_variance_key<'a: 'b, 'b>(x: LinearMap<&'a (), u32, 42>) -> LinearMap<&'b (), u32, 42> {
        x
    }
    fn _test_variance_key_view<'a: 'b, 'b, 'c>(
        x: &'c LinearMapView<&'a (), u32>,
    ) -> &'c LinearMapView<&'b (), u32> {
        x
    }

    #[test]
    fn partial_eq_floats() {
        // Make sure `PartialEq` is implemented even if `V` doesn't implement `Eq`.
        let map: LinearMap<usize, f32, 4> = Default::default();
        assert_eq!(map, map);
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_linear_map_zeroize() {
        use zeroize::Zeroize;

        let mut map: LinearMap<u8, u8, 8> = LinearMap::new();
        for i in 1..=8 {
            map.insert(i, i * 10).unwrap();
        }

        assert_eq!(map.len(), 8);
        assert_eq!(map.get(&5), Some(&50));

        // zeroized using Vec's implementation
        map.zeroize();

        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    // tests that use this constant take too long to run under miri, specially on CI, with a map of
    // this size so make the map smaller when using miri
    #[cfg(not(miri))]
    const MAP_SLOTS: usize = 4096;
    #[cfg(miri)]
    const MAP_SLOTS: usize = 64;
    fn almost_filled_map() -> LinearMap<usize, usize, MAP_SLOTS> {
        let mut almost_filled = LinearMap::new();
        for i in 1..MAP_SLOTS {
            almost_filled.insert(i, i).unwrap();
        }
        almost_filled
    }

    #[test]
    fn remove() {
        let mut src = almost_filled_map();
        // key doesn't exist
        let k = 0;
        let r = src.remove(&k);
        assert!(r.is_none());

        let k = 5;
        let v = 5;
        let r = src.remove(&k);
        assert_eq!(r, Some(v));
        let r = src.remove(&k);
        assert!(r.is_none());
        assert_eq!(src.len(), MAP_SLOTS - 2);
    }

    #[test]
    fn replace() {
        let mut src = almost_filled_map();
        src.insert(10, 1000).unwrap();
        let v = src.get(&10).unwrap();
        assert_eq!(*v, 1000);

        let mut src = almost_filled_map();
        let v = src.get_mut(&10).unwrap();
        *v = 500;
        let v = src.get(&10).unwrap();
        assert_eq!(*v, 500);
    }

    #[test]
    fn retain() {
        let mut src = almost_filled_map();
        src.retain(|k, _v| k % 2 == 0);
        src.retain(|k, _v| k % 3 == 0);

        for (k, v) in src.iter() {
            assert_eq!(k, v);
            assert_eq!(k % 2, 0);
            assert_eq!(k % 3, 0);
        }

        let mut src = almost_filled_map();
        src.retain(|_k, _v| false);
        assert!(src.is_empty());

        let mut src = almost_filled_map();
        src.retain(|_k, _v| true);
        assert_eq!(src.len(), MAP_SLOTS - 1);
        src.insert(0, 0).unwrap();
        src.retain(|_k, _v| true);
        assert_eq!(src.len(), MAP_SLOTS);
    }

    #[test]
    fn entry_find() {
        let key = 0;
        let value = 0;
        let mut src = almost_filled_map();
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(_) => {
                panic!("Found entry without inserting");
            }
            Entry::Vacant(v) => {
                assert_eq!(&key, v.key());
                assert_eq!(key, v.into_key());
            }
        }
        src.insert(key, value).unwrap();
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(mut o) => {
                assert_eq!(&key, o.key());
                assert_eq!(&value, o.get());
                assert_eq!(&value, o.get_mut());
                assert_eq!(&value, o.into_mut());
            }
            Entry::Vacant(_) => {
                panic!("Entry not found");
            }
        }
    }

    #[test]
    fn entry_vacant_insert() {
        let key = 0;
        let value = 0;
        let mut src = almost_filled_map();
        assert_eq!(MAP_SLOTS - 1, src.len());
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(_) => {
                panic!("Entry found when empty");
            }
            Entry::Vacant(v) => {
                assert_eq!(value, *v.insert(value).unwrap());
            }
        };
        assert_eq!(value, *src.get(&key).unwrap());
    }

    #[test]
    fn entry_vacant_full_insert() {
        let mut src = almost_filled_map();

        // fill the map
        let key = MAP_SLOTS * 2;
        let value = key;
        src.insert(key, value).unwrap();
        assert_eq!(MAP_SLOTS, src.len());

        let key = 0;
        let value = 0;
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(_) => {
                panic!("Entry found when missing");
            }
            Entry::Vacant(v) => {
                // Value is returned since the map is full
                assert_eq!(value, v.insert(value).unwrap_err());
            }
        };
        assert!(src.get(&key).is_none());
    }

    #[test]
    fn entry_occupied_insert() {
        let key = 0;
        let value = 0;
        let value2 = 5;
        let mut src = almost_filled_map();
        assert_eq!(MAP_SLOTS - 1, src.len());
        src.insert(key, value).unwrap();
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(o) => {
                assert_eq!(value, o.insert(value2));
            }
            Entry::Vacant(_) => {
                panic!("Entry not found");
            }
        };
        assert_eq!(value2, *src.get(&key).unwrap());
    }

    #[test]
    fn entry_remove_entry() {
        let key = 0;
        let value = 0;
        let mut src = almost_filled_map();
        src.insert(key, value).unwrap();
        assert_eq!(MAP_SLOTS, src.len());
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(o) => {
                assert_eq!((key, value), o.remove_entry());
            }
            Entry::Vacant(_) => {
                panic!("Entry not found")
            }
        };
        assert_eq!(MAP_SLOTS - 1, src.len());
        assert!(!src.contains_key(&key));
    }

    #[test]
    fn entry_remove() {
        let key = 0;
        let value = 0;
        let mut src = almost_filled_map();
        src.insert(key, value).unwrap();
        assert_eq!(MAP_SLOTS, src.len());
        let entry = src.entry(key);
        match entry {
            Entry::Occupied(o) => {
                assert_eq!(value, o.remove());
            }
            Entry::Vacant(_) => {
                panic!("Entry not found");
            }
        };
        assert_eq!(MAP_SLOTS - 1, src.len());
    }
}
