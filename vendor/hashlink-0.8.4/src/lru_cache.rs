use core::{
    borrow::Borrow,
    fmt,
    hash::{BuildHasher, Hash},
    usize,
};

use hashbrown::hash_map;

use crate::linked_hash_map::{self, LinkedHashMap};

pub use crate::linked_hash_map::{
    Drain, Entry, IntoIter, Iter, IterMut, OccupiedEntry, RawEntryBuilder, RawEntryBuilderMut,
    RawOccupiedEntryMut, RawVacantEntryMut, VacantEntry,
};

pub struct LruCache<K, V, S = hash_map::DefaultHashBuilder> {
    map: LinkedHashMap<K, V, S>,
    max_size: usize,
}

impl<K: Eq + Hash, V> LruCache<K, V> {
    #[inline]
    pub fn new(capacity: usize) -> Self {
        LruCache {
            map: LinkedHashMap::new(),
            max_size: capacity,
        }
    }

    /// Create a new unbounded `LruCache` that does not automatically evict entries.
    ///
    /// A simple convenience method that is equivalent to `LruCache::new(usize::MAX)`
    #[inline]
    pub fn new_unbounded() -> Self {
        LruCache::new(usize::MAX)
    }
}

impl<K, V, S> LruCache<K, V, S> {
    #[inline]
    pub fn with_hasher(capacity: usize, hash_builder: S) -> Self {
        LruCache {
            map: LinkedHashMap::with_hasher(hash_builder),
            max_size: capacity,
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.max_size
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        self.map.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.map.iter_mut()
    }

    #[inline]
    pub fn drain(&mut self) -> Drain<K, V> {
        self.map.drain()
    }
}

impl<K: Eq + Hash, V, S> LruCache<K, V, S>
where
    S: BuildHasher,
{
    #[inline]
    pub fn contains_key<Q>(&mut self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get_mut(key).is_some()
    }

    /// Insert a new value into the `LruCache`.
    ///
    /// If necessary, will remove the value at the front of the LRU list to make room.
    #[inline]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let old_val = self.map.insert(k, v);
        if self.len() > self.capacity() {
            self.remove_lru();
        }
        old_val
    }

    /// Get the value for the given key, *without* marking the value as recently used and moving it
    /// to the back of the LRU list.
    #[inline]
    pub fn peek<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get(k)
    }

    /// Get the value for the given key mutably, *without* marking the value as recently used and
    /// moving it to the back of the LRU list.
    #[inline]
    pub fn peek_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_mut(k)
    }

    /// Retrieve the given key, marking it as recently used and moving it to the back of the LRU
    /// list.
    #[inline]
    pub fn get<Q>(&mut self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get_mut(k).map(|v| &*v)
    }

    /// Retrieve the given key, marking it as recently used and moving it to the back of the LRU
    /// list.
    #[inline]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        match self.map.raw_entry_mut().from_key(k) {
            linked_hash_map::RawEntryMut::Occupied(mut occupied) => {
                occupied.to_back();
                Some(occupied.into_mut())
            }
            linked_hash_map::RawEntryMut::Vacant(_) => None,
        }
    }

    /// If the returned entry is vacant, it will always have room to insert a single value.  By
    /// using the entry API, you can exceed the configured capacity by 1.
    ///
    /// The returned entry is not automatically moved to the back of the LRU list.  By calling
    /// `Entry::to_back` / `Entry::to_front` you can manually control the position of this entry in
    /// the LRU list.
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S> {
        if self.len() > self.capacity() {
            self.remove_lru();
        }
        self.map.entry(key)
    }

    /// The constructed raw entry is never automatically moved to the back of the LRU list.  By
    /// calling `Entry::to_back` / `Entry::to_front` you can manually control the position of this
    /// entry in the LRU list.
    #[inline]
    pub fn raw_entry(&self) -> RawEntryBuilder<'_, K, V, S> {
        self.map.raw_entry()
    }

    /// If the constructed raw entry is vacant, it will always have room to insert a single value.
    /// By using the raw entry API, you can exceed the configured capacity by 1.
    ///
    /// The constructed raw entry is never automatically moved to the back of the LRU list.  By
    /// calling `Entry::to_back` / `Entry::to_front` you can manually control the position of this
    /// entry in the LRU list.
    #[inline]
    pub fn raw_entry_mut(&mut self) -> RawEntryBuilderMut<'_, K, V, S> {
        if self.len() > self.capacity() {
            self.remove_lru();
        }
        self.map.raw_entry_mut()
    }

    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove(k)
    }

    #[inline]
    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove_entry(k)
    }

    /// Set the new cache capacity for the `LruCache`.
    ///
    /// If there are more entries in the `LruCache` than the new capacity will allow, they are
    /// removed.
    #[inline]
    pub fn set_capacity(&mut self, capacity: usize) {
        for _ in capacity..self.len() {
            self.remove_lru();
        }
        self.max_size = capacity;
    }

    /// Remove the least recently used entry and return it.
    ///
    /// If the `LruCache` is empty this will return None.
    #[inline]
    pub fn remove_lru(&mut self) -> Option<(K, V)> {
        self.map.pop_front()
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher + Clone> Clone for LruCache<K, V, S> {
    #[inline]
    fn clone(&self) -> Self {
        LruCache {
            map: self.map.clone(),
            max_size: self.max_size,
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Extend<(K, V)> for LruCache<K, V, S> {
    #[inline]
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K, V, S> IntoIterator for LruCache<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> IntoIter<K, V> {
        self.map.into_iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a LruCache<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    #[inline]
    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut LruCache<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    #[inline]
    fn into_iter(self) -> IterMut<'a, K, V> {
        self.iter_mut()
    }
}

impl<K, V, S> fmt::Debug for LruCache<K, V, S>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter().rev()).finish()
    }
}
