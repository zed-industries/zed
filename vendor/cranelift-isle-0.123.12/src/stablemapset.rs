//! Implementations of hashmap and hashset that asvoid observing non-determinism
//! in iteration order. In a separate module so the compiler can prevent access to the internal
//! implementation details.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;

/// A wrapper around a [HashSet] which prevents accidentally observing the non-deterministic
/// iteration order.
#[derive(Clone, Debug, Default)]
pub struct StableSet<T>(HashSet<T>);

impl<T> StableSet<T> {
    pub(crate) fn new() -> Self {
        StableSet(HashSet::new())
    }
}

impl<T: Hash + Eq> StableSet<T> {
    /// Adds a value to the set. Returns whether the value was newly inserted.
    pub fn insert(&mut self, val: T) -> bool {
        self.0.insert(val)
    }

    /// Returns true if the set contains a value.
    pub fn contains(&self, val: &T) -> bool {
        self.0.contains(val)
    }
}

/// A wrapper around a [HashMap] which prevents accidentally observing the non-deterministic
/// iteration order.
#[derive(Clone, Debug)]
pub struct StableMap<K, V>(HashMap<K, V>);

impl<K, V> StableMap<K, V> {
    pub(crate) fn new() -> Self {
        StableMap(HashMap::new())
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

// NOTE: Can't auto-derive this
impl<K, V> Default for StableMap<K, V> {
    fn default() -> Self {
        StableMap(HashMap::new())
    }
}

impl<K: Hash + Eq, V> StableMap<K, V> {
    pub(crate) fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.0.insert(k, v)
    }

    pub(crate) fn contains_key(&self, k: &K) -> bool {
        self.0.contains_key(k)
    }

    pub(crate) fn get(&self, k: &K) -> Option<&V> {
        self.0.get(k)
    }

    pub(crate) fn entry(&mut self, k: K) -> Entry<'_, K, V> {
        self.0.entry(k)
    }
}

impl<K: Hash + Eq, V> Index<&K> for StableMap<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.0.index(index)
    }
}

impl<K, V> From<HashMap<K, V>> for StableMap<K, V> {
    fn from(map: HashMap<K, V>) -> Self {
        StableMap(map)
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for StableMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        StableMap(HashMap::from_iter(iter))
    }
}
