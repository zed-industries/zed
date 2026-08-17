#![forbid(unsafe_code)]
// Copyright (c) 2016 multimap developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! A MultiMap implementation which is just a wrapper around std::collections::HashMap.
//! See HashMap's documentation for more details.
//!
//! Some of the methods are just thin wrappers, some methods does change a little semantics
//! and some methods are new (doesn't have an equivalent in HashMap.)
//!
//! The MultiMap is generic for the key (K) and the value (V). Internally the values are
//! stored in a generic Vector.
//!
//! # Examples
//!
//! ```
//! use multimap::MultiMap;
//!
//! // create a new MultiMap. An explicit type signature can be omitted because of the
//! // type inference.
//! let mut queries = MultiMap::new();
//!
//! // insert some queries.
//! queries.insert("urls", "http://rust-lang.org");
//! queries.insert("urls", "http://mozilla.org");
//! queries.insert("urls", "http://wikipedia.org");
//! queries.insert("id", "42");
//! queries.insert("name", "roger");
//!
//! // check if there's any urls.
//! println!("Are there any urls in the multimap? {:?}.",
//!     if queries.contains_key("urls") {"Yes"} else {"No"} );
//!
//! // get the first item in a key's vector.
//! assert_eq!(queries.get("urls"), Some(&"http://rust-lang.org"));
//!
//! // get all the urls.
//! assert_eq!(queries.get_vec("urls"),
//!     Some(&vec!["http://rust-lang.org", "http://mozilla.org", "http://wikipedia.org"]));
//!
//! // iterate over all keys and the first value in the key's vector.
//! for (key, value) in queries.iter() {
//!     println!("key: {:?}, val: {:?}", key, value);
//! }
//!
//! // iterate over all keys and the key's vector.
//! for (key, values) in queries.iter_all() {
//!     println!("key: {:?}, values: {:?}", key, values);
//! }
//!
//! // the different methods for getting value(s) from the multimap.
//! let mut map = MultiMap::new();
//!
//! map.insert("key1", 42);
//! map.insert("key1", 1337);
//!
//! assert_eq!(map["key1"], 42);
//! assert_eq!(map.get("key1"), Some(&42));
//! assert_eq!(map.get_vec("key1"), Some(&vec![42, 1337]));
//! ```

use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::hash_map::{Keys, IntoIter, RandomState};
use std::fmt::{self, Debug};
use std::iter::{Iterator, IntoIterator, FromIterator};
use std::hash::{Hash, BuildHasher};
use std::ops::Index;

pub use std::collections::hash_map::Iter as IterAll;
pub use std::collections::hash_map::IterMut as IterAllMut;

pub use entry::{Entry, OccupiedEntry, VacantEntry};

mod entry;

#[cfg(feature = "serde_impl")]
pub mod serde;

#[derive(Clone)]
pub struct MultiMap<K, V, S = RandomState> {
    inner: HashMap<K, Vec<V>, S>,
}

impl<K, V> MultiMap<K, V>
    where K: Eq + Hash
{
    /// Creates an empty MultiMap
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map: MultiMap<&str, isize> = MultiMap::new();
    /// ```
    pub fn new() -> MultiMap<K, V> {
        MultiMap { inner: HashMap::new() }
    }

    /// Creates an empty multimap with the given initial capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map: MultiMap<&str, isize> = MultiMap::with_capacity(20);
    /// ```
    pub fn with_capacity(capacity: usize) -> MultiMap<K, V> {
        MultiMap { inner: HashMap::with_capacity(capacity) }
    }
}

impl<K, V, S> MultiMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher,
{
    /// Creates an empty MultiMap which will use the given hash builder to hash keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut map: MultiMap<&str, isize> = MultiMap::with_hasher(s);
    /// ```
    pub fn with_hasher(hash_builder: S) -> MultiMap<K, V, S> {
        MultiMap {
            inner: HashMap::with_hasher(hash_builder)
        }
    }

    /// Creates an empty MultiMap with the given intial capacity and hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut map: MultiMap<&str, isize> = MultiMap::with_capacity_and_hasher(20, s);
    /// ```
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> MultiMap<K, V, S> {
        MultiMap {
            inner: HashMap::with_capacity_and_hasher(capacity, hash_builder)
        }
    }

    /// Inserts a key-value pair into the multimap. If the key does exist in
    /// the map then the value is pushed to that key's vector. If the key doesn't
    /// exist in the map a new vector with the given value is inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert("key", 42);
    /// ```
    pub fn insert(&mut self, k: K, v: V) {
        match self.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.get_vec_mut().push(v);
            }
            Entry::Vacant(entry) => {
                entry.insert_vec(vec![v]);
            }
        }
    }

    /// Inserts multiple key-value pairs into the multimap. If the key does exist in
    /// the map then the values are extended into that key's vector. If the key
    /// doesn't exist in the map a new vector collected from the given values is inserted.
    ///
    /// This may be more efficient than inserting values independently.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::<&str, &usize>::new();
    /// map.insert_many("key", &[42, 43]);
    /// ```
    pub fn insert_many<I: IntoIterator<Item = V>>(&mut self, k: K, v: I) {
        match self.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.get_vec_mut().extend(v);
            }
            Entry::Vacant(entry) => {
                entry.insert_vec(v.into_iter().collect::<Vec<_>>());
            }
        }
    }

    /// Inserts multiple key-value pairs into the multimap. If the key does exist in
    /// the map then the values are extended into that key's vector. If the key
    /// doesn't exist in the map a new vector collected from the given values is inserted.
    ///
    /// This may be more efficient than inserting values independently.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::<&str, usize>::new();
    /// map.insert_many_from_slice("key", &[42, 43]);
    /// ```
    pub fn insert_many_from_slice(&mut self, k: K, v: &[V])
    where
        V: Clone,
    {
        match self.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.get_vec_mut().extend_from_slice(v);
            }
            Entry::Vacant(entry) => {
                entry.insert_vec(v.to_vec());
            }
        }
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.contains_key(k)
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(2, 1337);
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Removes a key from the map, returning the vector of values at
    /// the key if the key was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.remove(&1), Some(vec![42, 1337]));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<Vec<V>>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.remove(k)
    }

    /// Returns a reference to the first item in the vector corresponding to
    /// the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.get(&1), Some(&42));
    /// ```
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.get(k).map(|v| &v[0])
    }

    /// Returns a mutable reference to the first item in the vector corresponding to
    /// the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// if let Some(v) = map.get_mut(&1) {
    ///     *v = 99;
    /// }
    /// assert_eq!(map[&1], 99);
    /// ```
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.get_mut(k).map(|v| v.get_mut(0).unwrap())
    }

    /// Returns a reference to the vector corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.get_vec(&1), Some(&vec![42, 1337]));
    /// ```
    pub fn get_vec<Q: ?Sized>(&self, k: &Q) -> Option<&Vec<V>>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.get(k)
    }

    /// Returns a mutable reference to the vector corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// if let Some(v) = map.get_vec_mut(&1) {
    ///     (*v)[0] = 1991;
    ///     (*v)[1] = 2332;
    /// }
    /// assert_eq!(map.get_vec(&1), Some(&vec![1991, 2332]));
    /// ```
    pub fn get_vec_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut Vec<V>>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.get_mut(k)
    }

    /// Returns true if the key is multi-valued.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// map.insert(2, 2332);
    ///
    /// assert_eq!(map.is_vec(&1), true);   // key is multi-valued
    /// assert_eq!(map.is_vec(&2), false);  // key is single-valued
    /// assert_eq!(map.is_vec(&3), false);  // key not in map
    /// ```
    pub fn is_vec<Q: ?Sized>(&self, k: &Q) -> bool
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        match self.get_vec(k) {
            Some(val) => { val.len() > 1 }
            None => false
        }
    }


    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let map: MultiMap<usize, usize> = MultiMap::new();
    /// assert!(map.capacity() >= 0);
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// assert!(map.is_empty());
    /// map.insert(1,42);
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clears the map, removing all key-value pairs.
    /// Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// An iterator visiting all keys in arbitrary order.
    /// Iterator element type is &'a K.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(2,1337);
    /// map.insert(4,1991);
    ///
    /// for key in map.keys() {
    ///     println!("{:?}", key);
    /// }
    /// ```
    pub fn keys<'a>(&'a self) -> Keys<'a, K, Vec<V>> {
        self.inner.keys()
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and the first element in the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (key, value) in map.iter() {
    ///     println!("key: {:?}, val: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter { inner: self.inner.iter() }
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and a mutable reference to the first element in the
    /// corresponding key's vector. Iterator element type is (&'a K, &'a mut V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (_, value) in map.iter_mut() {
    ///     *value *= *value;
    /// }
    ///
    /// for (key, value) in map.iter() {
    ///     println!("key: {:?}, val: {:?}", key, value);
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut { inner: self.inner.iter_mut() }
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (key, values) in map.iter_all() {
    ///     println!("key: {:?}, values: {:?}", key, values);
    /// }
    /// ```
    pub fn iter_all(&self) -> IterAll<K, Vec<V>> {
        self.inner.iter()
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (key, values) in map.iter_all_mut() {
    ///     for value in values.iter_mut() {
    ///         *value = 99;
    ///     }
    /// }
    ///
    /// for (key, values) in map.iter_all() {
    ///     println!("key: {:?}, values: {:?}", key, values);
    /// }
    /// ```
    pub fn iter_all_mut(&mut self) -> IterAllMut<K, Vec<V>> {
        self.inner.iter_mut()
    }

    /// Gets the specified key's corresponding entry in the map for in-place manipulation.
    /// It's possible to both manipulate the vector and the 'value' (the first value in the
    /// vector).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut m = MultiMap::new();
    /// m.insert(1, 42);
    ///
    /// {
    ///     let mut v = m.entry(1).or_insert(43);
    ///     assert_eq!(v, &42);
    ///     *v = 44;
    /// }
    /// assert_eq!(m.entry(2).or_insert(666), &666);
    ///
    /// {
    ///     let mut v = m.entry(1).or_insert_vec(vec![43]);
    ///     assert_eq!(v, &vec![44]);
    ///     v.push(50);
    /// }
    /// assert_eq!(m.entry(2).or_insert_vec(vec![666]), &vec![666]);
    ///
    /// assert_eq!(m.get_vec(&1), Some(&vec![44, 50]));
    /// ```
    pub fn entry(&mut self, k: K) -> Entry<K, V> {
        use std::collections::hash_map::Entry as HashMapEntry;
        match self.inner.entry(k) {
            HashMapEntry::Occupied(entry) => Entry::Occupied(OccupiedEntry { inner: entry }),
            HashMapEntry::Vacant(entry) => Entry::Vacant(VacantEntry { inner: entry }),
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k,&mut v)` returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut m = MultiMap::new();
    /// m.insert(1, 42);
    /// m.insert(1, 99);
    /// m.insert(2, 42);
    /// m.retain(|&k, &v| { k == 1 && v == 42 });
    /// assert_eq!(1, m.len());
    /// assert_eq!(Some(&42), m.get(&1));
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
        where F: FnMut(&K, &V) -> bool
    {
        for (key, vector) in &mut self.inner {
            vector.retain(|ref value| f(key, value));
        }
        self.inner.retain(|&_, ref v| !v.is_empty());
    }
}

impl<'a, K, V, S, Q: ?Sized> Index<&'a Q> for MultiMap<K, V, S>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash,
          S: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &Q) -> &V {
        self.inner
            .get(index)
            .map(|v| &v[0])
            .expect("no entry found for key")
    }
}

impl<K, V, S> Debug for MultiMap<K, V, S>
    where K: Eq + Hash + Debug,
          V: Debug,
          S: BuildHasher
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter_all()).finish()
    }
}

impl<K, V, S> PartialEq for MultiMap<K, V, S>
    where K: Eq + Hash,
          V: PartialEq,
          S: BuildHasher
{
    fn eq(&self, other: &MultiMap<K, V, S>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter_all().all(|(key, value)| other.get_vec(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V, S> Eq for MultiMap<K, V, S>
    where K: Eq + Hash,
          V: Eq,
          S: BuildHasher
{
}

impl<K, V, S> Default for MultiMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher + Default
{
    fn default() -> MultiMap<K, V, S> {
        MultiMap { inner: Default::default() }
    }
}

impl<K, V, S> FromIterator<(K, V)> for MultiMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher + Default
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iterable: T) -> MultiMap<K, V, S> {
        let iter = iterable.into_iter();
        let hint = iter.size_hint().0;

        let mut multimap = MultiMap::with_capacity_and_hasher(hint, S::default());
        for (k, v) in iter {
            multimap.insert(k, v);
        }

        multimap
    }
}

impl<'a, K, V, S> IntoIterator for &'a MultiMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    type Item = (&'a K, &'a Vec<V>);
    type IntoIter = IterAll<'a, K, Vec<V>>;

    fn into_iter(self) -> IterAll<'a, K, Vec<V>> {
        self.iter_all()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut MultiMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    type Item = (&'a K, &'a mut Vec<V>);
    type IntoIter = IterAllMut<'a, K, Vec<V>>;

    fn into_iter(self) -> IterAllMut<'a, K, Vec<V>> {
        self.inner.iter_mut()
    }
}

impl<K, V, S> IntoIterator for MultiMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    type Item = (K, Vec<V>);
    type IntoIter = IntoIter<K, Vec<V>>;

    fn into_iter(self) -> IntoIter<K, Vec<V>> {
        self.inner.into_iter()
    }
}

impl<K, V, S> Extend<(K, V)> for MultiMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<'a, K, V, S> Extend<(&'a K, &'a V)> for MultiMap<K, V, S>
    where K: Eq + Hash + Copy,
          V: Copy,
          S: BuildHasher
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }
}

impl<K, V, S> Extend<(K, Vec<V>)> for MultiMap<K, V, S>
    where K: Eq + Hash,
          S: BuildHasher
{
    fn extend<T: IntoIterator<Item = (K, Vec<V>)>>(&mut self, iter: T) {
        for (k, values) in iter {
            match self.entry(k) {
                Entry::Occupied(mut entry) => {
                    entry.get_vec_mut().extend(values);
                }
                Entry::Vacant(entry) => {
                    entry.insert_vec(values);
                }
            }
        }
    }
}

impl<'a, K, V, S> Extend<(&'a K, &'a Vec<V>)> for MultiMap<K, V, S>
    where K: Eq + Hash + Copy,
          V: Copy,
          S: BuildHasher
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a Vec<V>)>>(&mut self, iter: T) {
        self.extend(iter.into_iter().map(|(&key, values)| (key, values.to_owned())));
    }
}

#[derive(Clone)]
pub struct Iter<'a, K: 'a, V: 'a> {
    inner: IterAll<'a, K, Vec<V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.inner.next().map(|(k, v)| (k, &v[0]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    inner: IterAllMut<'a, K, Vec<V>>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        self.inner.next().map(|(k, v)| (k, &mut v[0]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[macro_export]
/// Create a `MultiMap` from a list of key value pairs
///
/// ## Example
///
/// ```
/// # use multimap::MultiMap;
/// #[macro_use] extern crate multimap;
/// # fn main(){
///
/// let map = multimap!(
///     "dog" => "husky",
///     "dog" => "retreaver",
///     "dog" => "shiba inu",
///     "cat" => "cat"
///     );
/// # }
///
/// ```
macro_rules! multimap{
    (@replace_with_unit $_t:tt) => { () };
    (@count $($key:expr),*) => { <[()]>::len(&[$($crate::multimap! { @replace_with_unit $key }),*]) };
    
    ($($key:expr => $value:expr),* $(,)?)=>{
        {
            let mut map = $crate::MultiMap::with_capacity($crate::multimap! { @count $($key),* });
            $(
                map.insert($key,$value);
             )*
            map
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn create() {
        let _: MultiMap<usize, usize> = MultiMap { inner: HashMap::new() };
    }

    #[test]
    fn new() {
        let _: MultiMap<usize, usize> = MultiMap::new();
    }

    #[test]
    fn with_capacity() {
        let _: MultiMap<usize, usize> = MultiMap::with_capacity(20);
    }

    #[test]
    fn insert() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 3);
    }

    #[test]
    fn insert_many() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert_many(1, vec![3, 4]);
        assert_eq!(Some(&vec![3, 4]), m.get_vec(&1));
    }

    #[test]
    fn insert_many_again() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 2);
        m.insert_many(1, vec![3, 4]);
        assert_eq!(Some(&vec![2, 3, 4]), m.get_vec(&1));
    }

    #[test]
    fn insert_many_from_slice() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert_many_from_slice(1, &[3, 4]);
        assert_eq!(Some(&vec![3, 4]), m.get_vec(&1));
    }

    #[test]
    fn insert_many_from_slice_again() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 2);
        m.insert_many_from_slice(1, &[3, 4]);
        assert_eq!(Some(&vec![2, 3, 4]), m.get_vec(&1));
    }

    #[test]
    fn insert_existing() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 3);
        m.insert(1, 4);
    }

    #[test]
    #[should_panic]
    fn index_no_entry() {
        let m: MultiMap<usize, usize> = MultiMap::new();
        &m[&1];
    }

    #[test]
    fn index() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        let values = m[&1];
        assert_eq!(values, 42);
    }

    #[test]
    fn contains_key_true() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        assert!(m.contains_key(&1));
    }

    #[test]
    fn contains_key_false() {
        let m: MultiMap<usize, usize> = MultiMap::new();
        assert_eq!(m.contains_key(&1), false);
    }

    #[test]
    fn len() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.insert(2, 1337);
        m.insert(3, 99);
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn remove_not_present() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        let v = m.remove(&1);
        assert_eq!(v, None);
    }

    #[test]
    fn remove_present() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        let v = m.remove(&1);
        assert_eq!(v, Some(vec![42]));
    }

    #[test]
    fn get_not_present() {
        let m: MultiMap<usize, usize> = MultiMap::new();
        assert_eq!(m.get(&1), None);
    }

    #[test]
    fn get_present() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        assert_eq!(m.get(&1), Some(&42));
    }

    #[test]
    fn get_vec_not_present() {
        let m: MultiMap<usize, usize> = MultiMap::new();
        assert_eq!(m.get_vec(&1), None);
    }

    #[test]
    fn get_vec_present() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 1337);
        assert_eq!(m.get_vec(&1), Some(&vec![42, 1337]));
    }

    #[test]
    fn capacity() {
        let m: MultiMap<usize, usize> = MultiMap::with_capacity(20);
        assert!(m.capacity() >= 20);
    }

    #[test]
    fn is_empty_true() {
        let m: MultiMap<usize, usize> = MultiMap::new();
        assert_eq!(m.is_empty(), true);
    }

    #[test]
    fn is_empty_false() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        assert_eq!(m.is_empty(), false);
    }

    #[test]
    fn clear() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.clear();
        assert!(m.is_empty());
    }

    #[test]
    fn get_mut() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        if let Some(v) = m.get_mut(&1) {
            *v = 1337;
        }
        assert_eq!(m[&1], 1337)
    }

    #[test]
    fn get_vec_mut() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 1337);
        if let Some(v) = m.get_vec_mut(&1) {
            (*v)[0] = 5;
            (*v)[1] = 10;
        }
        assert_eq!(m.get_vec(&1), Some(&vec![5, 10]))
    }

    #[test]
    fn keys() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.insert(2, 42);
        m.insert(4, 42);
        m.insert(8, 42);

        let keys: Vec<_> = m.keys().cloned().collect();
        assert_eq!(keys.len(), 4);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&4));
        assert!(keys.contains(&8));
    }

    #[test]
    fn iter() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 42);
        m.insert(4, 42);
        m.insert(8, 42);

        let mut iter = m.iter();

        for _ in iter.by_ref().take(2) {}

        assert_eq!(iter.len(), 1);
    }

    #[test]
    fn intoiterator_for_reference_type() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 43);
        m.insert(4, 42);
        m.insert(8, 42);

        let keys = vec![1, 4, 8];

        for (key, value) in &m {
            assert!(keys.contains(key));

            if key == &1 {
                assert_eq!(value, &vec![42, 43]);
            } else {
                assert_eq!(value, &vec![42]);
            }
        }
    }

    #[test]
    fn intoiterator_for_mutable_reference_type() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 43);
        m.insert(4, 42);
        m.insert(8, 42);

        let keys = vec![1, 4, 8];

        for (key, value) in &mut m {
            assert!(keys.contains(key));

            if key == &1 {
                assert_eq!(value, &vec![42, 43]);
                value.push(666);
            } else {
                assert_eq!(value, &vec![42]);
            }
        }

        assert_eq!(m.get_vec(&1), Some(&vec![42, 43, 666]));
    }

    #[test]
    fn intoiterator_consuming() {
        let mut m: MultiMap<usize, usize> = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 43);
        m.insert(4, 42);
        m.insert(8, 42);

        let keys = vec![1, 4, 8];

        for (key, value) in m {
            assert!(keys.contains(&key));

            if key == 1 {
                assert_eq!(value, vec![42, 43]);
            } else {
                assert_eq!(value, vec![42]);
            }
        }
    }

    #[test]
    fn test_fmt_debug() {
        let mut map = MultiMap::new();
        let empty: MultiMap<i32, i32> = MultiMap::new();

        map.insert(1, 2);
        map.insert(1, 5);
        map.insert(1, -1);
        map.insert(3, 4);

        let map_str = format!("{:?}", map);

        assert!(map_str == "{1: [2, 5, -1], 3: [4]}" || map_str == "{3: [4], 1: [2, 5, -1]}");
        assert_eq!(format!("{:?}", empty), "{}");
    }

    #[test]
    fn test_eq() {
        let mut m1 = MultiMap::new();
        m1.insert(1, 2);
        m1.insert(2, 3);
        m1.insert(3, 4);
        let mut m2 = MultiMap::new();
        m2.insert(1, 2);
        m2.insert(2, 3);
        assert!(m1 != m2);
        m2.insert(3, 4);
        assert_eq!(m1, m2);
        m2.insert(3, 4);
        assert!(m1 != m2);
        m1.insert(3, 4);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_default() {
        let _: MultiMap<u8, u8> = Default::default();
    }

    #[test]
    fn test_from_iterator() {
        let vals: Vec<(&str, i64)> = vec![("foo", 123), ("bar", 456), ("foo", 789)];
        let multimap: MultiMap<&str, i64> = MultiMap::from_iter(vals);

        let foo_vals: &Vec<i64> = multimap.get_vec("foo").unwrap();
        assert!(foo_vals.contains(&123));
        assert!(foo_vals.contains(&789));

        let bar_vals: &Vec<i64> = multimap.get_vec("bar").unwrap();
        assert!(bar_vals.contains(&456));
    }

    #[test]
    fn test_extend_consuming_hashmap() {
        let mut a = MultiMap::new();
        a.insert(1, 42);

        let mut b = HashMap::new();
        b.insert(1, 43);
        b.insert(2, 666);

        a.extend(b);

        assert_eq!(a.len(), 2);
        assert_eq!(a.get_vec(&1), Some(&vec![42, 43]));
    }

    #[test]
    fn test_extend_ref_hashmap() {
        let mut a = MultiMap::new();
        a.insert(1, 42);

        let mut b = HashMap::new();
        b.insert(1, 43);
        b.insert(2, 666);

        a.extend(&b);

        assert_eq!(a.len(), 2);
        assert_eq!(a.get_vec(&1), Some(&vec![42, 43]));
        assert_eq!(b.len(), 2);
        assert_eq!(b[&1], 43);
    }

    #[test]
    fn test_extend_consuming_multimap() {
        let mut a = MultiMap::new();
        a.insert(1, 42);

        let mut b = MultiMap::new();
        b.insert(1, 43);
        b.insert(1, 44);
        b.insert(2, 666);

        a.extend(b);

        assert_eq!(a.len(), 2);
        assert_eq!(a.get_vec(&1), Some(&vec![42, 43, 44]));
    }

    #[test]
    fn test_extend_ref_multimap() {
        let mut a = MultiMap::new();
        a.insert(1, 42);

        let mut b = MultiMap::new();
        b.insert(1, 43);
        b.insert(1, 44);
        b.insert(2, 666);

        a.extend(&b);

        assert_eq!(a.len(), 2);
        assert_eq!(a.get_vec(&1), Some(&vec![42, 43, 44]));
        assert_eq!(b.len(), 2);
        assert_eq!(b.get_vec(&1), Some(&vec![43, 44]));
    }

    #[test]
    fn test_entry() {
        let mut m = MultiMap::new();
        m.insert(1, 42);

        {
            let v = m.entry(1).or_insert(43);
            assert_eq!(v, &42);
            *v = 44;
        }
        assert_eq!(m.entry(2).or_insert(666), &666);

        assert_eq!(m[&1], 44);
        assert_eq!(m[&2], 666);
    }

    #[test]
    fn test_entry_vec() {
        let mut m = MultiMap::new();
        m.insert(1, 42);

        {
            let v = m.entry(1).or_insert_vec(vec![43]);
            assert_eq!(v, &vec![42]);
            *v.first_mut().unwrap() = 44;
        }
        assert_eq!(m.entry(2).or_insert_vec(vec![666]), &vec![666]);


        assert_eq!(m[&1], 44);
        assert_eq!(m[&2], 666);
    }

    #[test]
    fn test_is_vec() {
        let mut m = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 1337);
        m.insert(2, 2332);

        assert!(m.is_vec(&1));
        assert!(!m.is_vec(&2));
        assert!(!m.is_vec(&3));
    }

    #[test]
    fn test_macro(){
        let mut manual_map = MultiMap::new();
        manual_map.insert("key1", 42);
        assert_eq!(manual_map, multimap!("key1" => 42));

        manual_map.insert("key1", 1337);
        manual_map.insert("key2", 2332);
        let macro_map = multimap!{
            "key1" =>    42,
            "key1" =>  1337,
            "key2" =>  2332
        };
        assert_eq!(manual_map, macro_map);
    }

    #[test]
    fn retain_removes_element() {
        let mut m = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 99);
        m.retain(|&k, &v| { k == 1 && v == 42 });
        assert_eq!(1, m.len());
        assert_eq!(Some(&42), m.get(&1));
    }

    #[test]
    fn retain_also_removes_empty_vector() {
        let mut m = MultiMap::new();
        m.insert(1, 42);
        m.insert(1, 99);
        m.insert(2, 42);
        m.retain(|&k, &v| { k == 1 && v == 42 });
        assert_eq!(1, m.len());
        assert_eq!(Some(&42), m.get(&1));
    }
}

