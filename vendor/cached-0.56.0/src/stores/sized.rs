use super::Cached;
use crate::lru_list::LRUList;
use hashbrown::HashTable;
use std::cmp::Eq;
use std::fmt;
use std::hash::{BuildHasher, Hash, Hasher};

#[cfg(feature = "ahash")]
use ahash::RandomState;

#[cfg(not(feature = "ahash"))]
use std::collections::hash_map::RandomState;

#[cfg(feature = "async")]
use {super::CachedAsync, async_trait::async_trait, futures::Future};

/// Least Recently Used / `Sized` Cache
///
/// Stores up to a specified size before beginning
/// to evict the least recently used keys
///
/// Note: This cache is in-memory only
#[derive(Clone)]
pub struct SizedCache<K, V> {
    // `store` contains a hash of K -> index of (K, V) tuple in `order`
    pub(super) store: HashTable<usize>,
    pub(super) hash_builder: RandomState,
    pub(super) order: LRUList<(K, V)>,
    pub(super) capacity: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}

impl<K, V> fmt::Debug for SizedCache<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SizedCache")
            .field("order", &self.order)
            .field("capacity", &self.capacity)
            .field("hits", &self.hits)
            .field("misses", &self.misses)
            .finish()
    }
}

impl<K, V> PartialEq for SizedCache<K, V>
where
    K: Eq + Hash + Clone,
    V: PartialEq,
{
    fn eq(&self, other: &SizedCache<K, V>) -> bool {
        self.store.len() == other.store.len() && {
            self.order
                .iter()
                .all(|(key, value)| match other.get_index(other.hash(key), key) {
                    Some(i) => value == &other.order.get(i).1,
                    None => false,
                })
        }
    }
}

impl<K, V> Eq for SizedCache<K, V>
where
    K: Eq + Hash + Clone,
    V: PartialEq,
{
}

impl<K: Hash + Eq + Clone, V> SizedCache<K, V> {
    #[deprecated(since = "0.5.1", note = "method renamed to `with_size`")]
    #[must_use]
    pub fn with_capacity(size: usize) -> SizedCache<K, V> {
        Self::with_size(size)
    }

    /// Creates a new `SizedCache` with a given size limit and pre-allocated backing data
    ///
    /// # Panics
    ///
    /// Will panic if size is 0
    #[must_use]
    pub fn with_size(size: usize) -> SizedCache<K, V> {
        if size == 0 {
            panic!("`size` of `SizedCache` must be greater than zero.");
        }
        SizedCache {
            store: HashTable::with_capacity(size),
            hash_builder: RandomState::new(),
            order: LRUList::<(K, V)>::with_capacity(size),
            capacity: size,
            hits: 0,
            misses: 0,
        }
    }

    /// Creates a new `SizedCache` with a given size limit and pre-allocated backing data
    ///
    /// # Errors
    ///
    /// Will return a `std::io::Error`, depending on the error
    pub fn try_with_size(size: usize) -> std::io::Result<SizedCache<K, V>> {
        if size == 0 {
            // EINVAL
            return Err(std::io::Error::from_raw_os_error(22));
        }

        let mut store = HashTable::new();
        if let Err(e) = store.try_reserve(size, |&index: &usize| {
            let hasher = &mut RandomState::new().build_hasher();
            index.hash(hasher);
            hasher.finish()
        }) {
            let errcode = match e {
                // ENOMEM
                hashbrown::TryReserveError::AllocError { .. } => 12,
                // EINVAL
                hashbrown::TryReserveError::CapacityOverflow => 22,
            };
            return Err(std::io::Error::from_raw_os_error(errcode));
        }

        Ok(SizedCache {
            store,
            hash_builder: RandomState::new(),
            order: LRUList::<(K, V)>::with_capacity(size),
            capacity: size,
            hits: 0,
            misses: 0,
        })
    }

    pub(super) fn iter_order(&self) -> impl Iterator<Item = &(K, V)> {
        self.order.iter()
    }

    /// Return an iterator of keys in the current order from most
    /// to least recently used.
    pub fn key_order(&self) -> impl Iterator<Item = &K> {
        self.order.iter().map(|(k, _v)| k)
    }

    /// Return an iterator of values in the current order from most
    /// to least recently used.
    pub fn value_order(&self) -> impl Iterator<Item = &V> {
        self.order.iter().map(|(_k, v)| v)
    }

    fn hash<Q>(&self, key: &Q) -> u64
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        let hasher = &mut self.hash_builder.build_hasher();
        key.hash(hasher);
        hasher.finish()
    }

    fn insert_index(&mut self, hash: u64, index: usize) {
        let Self {
            ref mut store,
            ref order,
            ref hash_builder,
            ..
        } = *self;
        // insert the value `index` at `hash`, the closure provided
        // is used to rehash values if a resize is necessary.
        store.insert_unique(hash, index, move |&i| {
            // rehash the "key" value stored at index `i` - requires looking
            // up the original "key" value in the `order` list.
            let hasher = &mut hash_builder.build_hasher();
            order.get(i).0.hash(hasher);
            hasher.finish()
        });
    }

    fn get_index<Q>(&self, hash: u64, key: &Q) -> Option<usize>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        let Self { store, order, .. } = self;
        // Get the `order` index store under `hash`, the closure provided
        // is used to compare against matching hashes - we lookup the original
        // `key` value from the `order` list.
        // This pattern is repeated in other lookup situations.
        store
            .find(hash, |&i| key == order.get(i).0.borrow())
            .copied()
    }

    fn remove_index<Q>(&mut self, hash: u64, key: &Q) -> Option<usize>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        let Self { store, order, .. } = self;
        match store.find_entry(hash, |&i| key == order.get(i).0.borrow()) {
            Ok(entry) => Some(entry.remove().0),
            Err(_) => None,
        }
    }

    fn check_capacity(&mut self) {
        let Self {
            ref mut store,
            ref mut order,
            ref hash_builder,
            capacity,
            ..
        } = *self;
        let len = store.len();
        if len > capacity {
            // store has reached capacity, evict the oldest item.
            // store capacity cannot be zero, so there must be content in `self.order`.
            let index = order.back();
            let key = &order.get(index).0;
            let hasher = &mut hash_builder.build_hasher();
            key.hash(hasher);
            let hash = hasher.finish();

            let order_ = &order;
            match store.find_entry(hash, |&i| *key == order_.get(i).0) {
                Ok(entry) => {
                    entry.remove();
                }
                Err(_) => {
                    panic!("SizedCache::cache_set failed evicting cache key");
                }
            }
            order.remove(index);
        }
    }

    pub(super) fn get_if<F: FnOnce(&V) -> bool, Q>(&mut self, key: &Q, is_valid: F) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        if let Some(index) = self.get_index(self.hash(key), key) {
            if is_valid(&self.order.get(index).1) {
                self.order.move_to_front(index);
                self.hits += 1;
                return Some(&self.order.get(index).1);
            }
        }
        self.misses += 1;
        None
    }

    pub(super) fn get_mut_if<F: FnOnce(&V) -> bool, Q>(
        &mut self,
        key: &Q,
        is_valid: F,
    ) -> Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        if let Some(index) = self.get_index(self.hash(key), key) {
            if is_valid(&self.order.get(index).1) {
                self.order.move_to_front(index);
                self.hits += 1;
                return Some(&mut self.order.get_mut(index).1);
            }
        }
        self.misses += 1;
        None
    }

    /// Get the cached value, or set it using `f` if the value
    /// is either not-set or if `is_valid` returns `false` for
    /// the set value.
    ///
    /// Returns (`was_present`, `was_valid`, mut ref to set value)
    /// `was_valid` will be false when `was_present` is false
    pub(super) fn get_or_set_with_if<F: FnOnce() -> V, FC: FnOnce(&V) -> bool>(
        &mut self,
        key: K,
        f: F,
        is_valid: FC,
    ) -> (bool, bool, &mut V) {
        let hash = self.hash(&key);
        let index = self.get_index(hash, &key);
        if let Some(index) = index {
            self.hits += 1;
            let replace_existing = {
                let v = &self.order.get(index).1;
                !is_valid(v)
            };
            if replace_existing {
                self.order.set(index, (key, f()));
            }
            self.order.move_to_front(index);
            (true, !replace_existing, &mut self.order.get_mut(index).1)
        } else {
            self.misses += 1;
            let index = self.order.push_front((key, f()));
            self.insert_index(hash, index);
            self.check_capacity();
            (false, false, &mut self.order.get_mut(index).1)
        }
    }

    pub(super) fn try_get_or_set_with_if<E, F: FnOnce() -> Result<V, E>, FC: FnOnce(&V) -> bool>(
        &mut self,
        key: K,
        f: F,
        is_valid: FC,
    ) -> Result<(bool, bool, &mut V), E> {
        let hash = self.hash(&key);
        let index = self.get_index(hash, &key);
        if let Some(index) = index {
            self.hits += 1;
            let replace_existing = {
                let v = &self.order.get(index).1;
                !is_valid(v)
            };
            if replace_existing {
                self.order.set(index, (key, f()?));
            }
            self.order.move_to_front(index);
            Ok((true, !replace_existing, &mut self.order.get_mut(index).1))
        } else {
            self.misses += 1;
            let index = self.order.push_front((key, f()?));
            self.insert_index(hash, index);
            self.check_capacity();
            Ok((false, false, &mut self.order.get_mut(index).1))
        }
    }

    /// Returns a reference to the cache's `order`
    #[must_use]
    pub fn get_order(&self) -> &LRUList<(K, V)> {
        &self.order
    }

    pub fn retain<F: Fn(&K, &V) -> bool>(&mut self, keep: F) {
        let remove_keys = self
            .iter_order()
            .filter_map(|(k, v)| if keep(k, v) { None } else { Some(k.clone()) })
            .collect::<Vec<_>>();
        for k in remove_keys {
            self.cache_remove(&k);
        }
    }
}

#[cfg(feature = "async")]
impl<K, V> SizedCache<K, V>
where
    K: Hash + Eq + Clone + Send,
{
    /// Get the cached value, or set it using `f` if the value
    /// is either not-set or if `is_valid` returns `false` for
    /// the set value.
    ///
    /// Returns (`was_present`, `was_valid`, mut ref to set value)
    /// `was_valid` will be false when `was_present` is false
    pub(super) async fn get_or_set_with_if_async<F, Fut, FC>(
        &mut self,
        key: K,
        f: F,
        is_valid: FC,
    ) -> (bool, bool, &mut V)
    where
        V: Send,
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = V> + Send,
        FC: FnOnce(&V) -> bool,
    {
        let hash = self.hash(&key);
        let index = self.get_index(hash, &key);
        if let Some(index) = index {
            self.hits += 1;
            let replace_existing = {
                let v = &self.order.get(index).1;
                !is_valid(v)
            };
            if replace_existing {
                self.order.set(index, (key, f().await));
            }
            self.order.move_to_front(index);
            (true, !replace_existing, &mut self.order.get_mut(index).1)
        } else {
            self.misses += 1;
            let index = self.order.push_front((key, f().await));
            self.insert_index(hash, index);
            self.check_capacity();
            (false, false, &mut self.order.get_mut(index).1)
        }
    }

    pub(super) async fn try_get_or_set_with_if_async<E, F, Fut, FC>(
        &mut self,
        key: K,
        f: F,
        is_valid: FC,
    ) -> Result<(bool, bool, &mut V), E>
    where
        V: Send,
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<V, E>> + Send,
        FC: FnOnce(&V) -> bool,
    {
        let hash = self.hash(&key);
        let index = self.get_index(hash, &key);
        if let Some(index) = index {
            self.hits += 1;
            let replace_existing = {
                let v = &self.order.get(index).1;
                !is_valid(v)
            };
            if replace_existing {
                self.order.set(index, (key, f().await?));
            }
            self.order.move_to_front(index);
            Ok((true, !replace_existing, &mut self.order.get_mut(index).1))
        } else {
            self.misses += 1;
            let index = self.order.push_front((key, f().await?));
            self.insert_index(hash, index);
            self.check_capacity();
            Ok((false, false, &mut self.order.get_mut(index).1))
        }
    }
}

impl<K: Hash + Eq + Clone, V> Cached<K, V> for SizedCache<K, V> {
    fn cache_get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.get_if(key, |_| true)
    }

    fn cache_get_mut<Q>(&mut self, key: &Q) -> std::option::Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.get_mut_if(key, |_| true)
    }

    fn cache_set(&mut self, key: K, val: V) -> Option<V> {
        let hash = self.hash(&key);
        let v = if let Some(index) = self.get_index(hash, &key) {
            self.order.set(index, (key, val)).map(|(_, v)| v)
        } else {
            let index = self.order.push_front((key, val));
            self.insert_index(hash, index);
            None
        };
        self.check_capacity();
        v
    }

    fn cache_get_or_set_with<F: FnOnce() -> V>(&mut self, key: K, f: F) -> &mut V {
        let (_, _, v) = self.get_or_set_with_if(key, f, |_| true);
        v
    }

    fn cache_try_get_or_set_with<F: FnOnce() -> Result<V, E>, E>(
        &mut self,
        k: K,
        f: F,
    ) -> Result<&mut V, E> {
        let (_, _, v) = self.try_get_or_set_with_if(k, f, |_| true)?;
        Ok(v)
    }

    fn cache_remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        // try and remove item from mapping, and then from order list if it was in mapping
        let hash = self.hash(k);
        if let Some(index) = self.remove_index(hash, k) {
            // need to remove the key in the order list
            let (_key, value) = self.order.remove(index);
            Some(value)
        } else {
            None
        }
    }
    fn cache_clear(&mut self) {
        // clear both the store and the order list
        self.store.clear();
        self.order.clear();
    }
    fn cache_reset(&mut self) {
        // SizedCache uses cache_clear because capacity is fixed.
        self.cache_clear();
    }
    fn cache_reset_metrics(&mut self) {
        self.misses = 0;
        self.hits = 0;
    }
    fn cache_size(&self) -> usize {
        self.store.len()
    }
    fn cache_hits(&self) -> Option<u64> {
        Some(self.hits)
    }
    fn cache_misses(&self) -> Option<u64> {
        Some(self.misses)
    }
    fn cache_capacity(&self) -> Option<usize> {
        Some(self.capacity)
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<K, V> CachedAsync<K, V> for SizedCache<K, V>
where
    K: Hash + Eq + Clone + Send,
{
    async fn get_or_set_with<F, Fut>(&mut self, k: K, f: F) -> &mut V
    where
        V: Send,
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = V> + Send,
    {
        let (_, _, v) = self.get_or_set_with_if_async(k, f, |_| true).await;
        v
    }

    async fn try_get_or_set_with<F, Fut, E>(&mut self, k: K, f: F) -> Result<&mut V, E>
    where
        V: Send,
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<V, E>> + Send,
    {
        let (_, _, v) = self.try_get_or_set_with_if_async(k, f, |_| true).await?;
        Ok(v)
    }
}

#[cfg(test)]
/// Cache store tests
mod tests {
    use super::*;

    #[test]
    fn sized_cache() {
        let mut c = SizedCache::with_size(5);
        assert!(c.cache_get(&1).is_none());
        let misses = c.cache_misses().unwrap();
        assert_eq!(1, misses);

        assert_eq!(c.cache_set(1, 100), None);
        assert!(c.cache_get(&1).is_some());
        let hits = c.cache_hits().unwrap();
        let misses = c.cache_misses().unwrap();
        assert_eq!(1, hits);
        assert_eq!(1, misses);

        assert_eq!(c.cache_set(2, 100), None);
        assert_eq!(c.cache_set(3, 100), None);
        assert_eq!(c.cache_set(4, 100), None);
        assert_eq!(c.cache_set(5, 100), None);

        assert_eq!(c.key_order().copied().collect::<Vec<_>>(), [5, 4, 3, 2, 1]);

        assert_eq!(c.cache_set(6, 100), None);
        assert_eq!(c.cache_set(7, 100), None);

        assert_eq!(c.key_order().copied().collect::<Vec<_>>(), [7, 6, 5, 4, 3]);

        assert!(c.cache_get(&2).is_none());
        assert!(c.cache_get(&3).is_some());

        assert_eq!(c.key_order().copied().collect::<Vec<_>>(), [3, 7, 6, 5, 4]);

        assert_eq!(2, c.cache_misses().unwrap());
        let size = c.cache_size();
        assert_eq!(5, size);

        c.cache_reset_metrics();
        let hits = c.cache_hits().unwrap();
        let misses = c.cache_misses().unwrap();
        let size = c.cache_size();
        assert_eq!(0, hits);
        assert_eq!(0, misses);
        assert_eq!(5, size);

        assert_eq!(c.cache_set(7, 200), Some(100));

        #[derive(Hash, Clone, Eq, PartialEq)]
        struct MyKey {
            v: String,
        }
        let mut c = SizedCache::with_size(5);
        assert_eq!(
            c.cache_set(
                MyKey {
                    v: String::from("s")
                },
                String::from("a")
            ),
            None
        );
        assert_eq!(
            c.cache_set(
                MyKey {
                    v: String::from("s")
                },
                String::from("a")
            ),
            Some(String::from("a"))
        );
        assert_eq!(
            c.cache_set(
                MyKey {
                    v: String::from("s2")
                },
                String::from("b")
            ),
            None
        );
        assert_eq!(
            c.cache_set(
                MyKey {
                    v: String::from("s2")
                },
                String::from("b")
            ),
            Some(String::from("b"))
        );
    }

    #[test]
    fn try_new() {
        let c: std::io::Result<SizedCache<i32, i32>> = SizedCache::try_with_size(0);
        assert_eq!(c.unwrap_err().raw_os_error(), Some(22));
    }

    #[test]
    /// This is a regression test to confirm that racing cache sets on a `SizedCache`
    /// do not cause duplicates to exist in the internal `order`. See issue #7
    fn size_cache_racing_keys_eviction_regression() {
        let mut c = SizedCache::with_size(2);
        assert_eq!(c.cache_set(1, 100), None);
        assert_eq!(c.cache_set(1, 100), Some(100));
        // size would be 1, but internal ordered would be [1, 1]
        assert_eq!(c.cache_set(2, 100), None);
        assert_eq!(c.cache_set(3, 100), None);
        // this next set would fail because a duplicate key would be evicted
        assert_eq!(c.cache_set(4, 100), None);
    }

    #[test]
    fn clear() {
        let mut c = SizedCache::with_size(3);

        assert_eq!(c.cache_set(1, 100), None);
        assert_eq!(c.cache_set(2, 200), None);
        assert_eq!(c.cache_set(3, 300), None);
        c.cache_clear();

        assert_eq!(0, c.cache_size());
    }

    #[test]
    fn reset() {
        let init_capacity = 1;
        let mut c = SizedCache::with_size(init_capacity);
        assert_eq!(c.cache_set(1, 100), None);
        assert_eq!(c.cache_set(2, 200), None);
        assert_eq!(c.cache_set(3, 300), None);
        assert!(init_capacity <= c.store.capacity());

        c.cache_reset();

        assert!(init_capacity <= c.store.capacity());
    }

    #[test]
    fn remove() {
        let mut c = SizedCache::with_size(3);

        assert_eq!(c.cache_set(1, 100), None);
        assert_eq!(c.cache_set(2, 200), None);
        assert_eq!(c.cache_set(3, 300), None);

        assert_eq!(Some(100), c.cache_remove(&1));
        assert_eq!(2, c.cache_size());

        assert_eq!(Some(200), c.cache_remove(&2));
        assert_eq!(1, c.cache_size());

        assert_eq!(None, c.cache_remove(&2));
        assert_eq!(1, c.cache_size());

        assert_eq!(Some(300), c.cache_remove(&3));
        assert_eq!(0, c.cache_size());
    }

    #[test]
    fn sized_cache_get_mut() {
        let mut c = SizedCache::with_size(5);
        assert!(c.cache_get_mut(&1).is_none());
        let misses = c.cache_misses().unwrap();
        assert_eq!(1, misses);

        assert_eq!(c.cache_set(1, 100), None);
        assert_eq!(*c.cache_get_mut(&1).unwrap(), 100);
        let hits = c.cache_hits().unwrap();
        let misses = c.cache_misses().unwrap();
        assert_eq!(1, hits);
        assert_eq!(1, misses);

        let value = c.cache_get_mut(&1).unwrap();
        *value = 10;

        let hits = c.cache_hits().unwrap();
        let misses = c.cache_misses().unwrap();
        assert_eq!(2, hits);
        assert_eq!(1, misses);
        assert_eq!(*c.cache_get_mut(&1).unwrap(), 10);
    }

    #[test]
    fn sized_cache_eviction_fix() {
        let mut cache = SizedCache::<u32, ()>::with_size(3);

        cache.cache_set(1, ());
        cache.cache_set(2, ());
        cache.cache_set(3, ());

        assert!(cache.cache_get(&1).is_some());
        assert!(cache.cache_get(&2).is_some());
        assert!(cache.cache_get(&3).is_some());
        assert!(cache.cache_get(&4).is_none());

        // previous bug: inserting the same key multiple times would continue
        //               to evict the oldest cache member
        cache.cache_set(4, ());
        assert_eq!(cache.cache_size(), 3);
        cache.cache_set(4, ());
        assert_eq!(cache.cache_size(), 3); // previously failed, returning 2

        assert!(cache.cache_get(&1).is_none()); // 1 is evicted by first "4" insert
        assert!(cache.cache_get(&2).is_some()); // previously failed, 2 would be evicted by second "4" insert
        assert!(cache.cache_get(&3).is_some());
        assert!(cache.cache_get(&4).is_some());
    }

    #[test]
    fn get_or_set_with() {
        let mut c = SizedCache::with_size(5);

        assert_eq!(c.cache_get_or_set_with(0, || 0), &0);
        assert_eq!(c.cache_get_or_set_with(1, || 1), &1);
        assert_eq!(c.cache_get_or_set_with(2, || 2), &2);
        assert_eq!(c.cache_get_or_set_with(3, || 3), &3);
        assert_eq!(c.cache_get_or_set_with(4, || 4), &4);
        assert_eq!(c.cache_get_or_set_with(5, || 5), &5);

        assert_eq!(c.cache_misses(), Some(6));

        assert_eq!(c.cache_get_or_set_with(0, || 0), &0);

        assert_eq!(c.cache_misses(), Some(7));

        assert_eq!(c.cache_get_or_set_with(0, || 42), &0);

        assert_eq!(c.cache_misses(), Some(7));

        assert_eq!(c.cache_get_or_set_with(1, || 1), &1);

        assert_eq!(c.cache_misses(), Some(8));

        c.cache_reset();
        fn _try_get(n: usize) -> Result<usize, String> {
            if n < 10 {
                Ok(n)
            } else {
                Err("dead".to_string())
            }
        }
        let res: Result<&mut usize, String> = c.cache_try_get_or_set_with(0, || _try_get(10));
        assert!(res.is_err());
        assert!(c.key_order().next().is_none());

        let res: Result<&mut usize, String> = c.cache_try_get_or_set_with(0, || _try_get(1));
        assert_eq!(res.unwrap(), &1);
        let res: Result<&mut usize, String> = c.cache_try_get_or_set_with(0, || _try_get(5));
        assert_eq!(res.unwrap(), &1);
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async_trait() {
        use crate::CachedAsync;
        let mut c = SizedCache::with_size(5);

        async fn _get(n: usize) -> usize {
            n
        }

        assert_eq!(c.get_or_set_with(0, || async { _get(0).await }).await, &0);
        assert_eq!(c.get_or_set_with(1, || async { _get(1).await }).await, &1);
        assert_eq!(c.get_or_set_with(2, || async { _get(2).await }).await, &2);
        assert_eq!(c.get_or_set_with(3, || async { _get(3).await }).await, &3);

        assert_eq!(c.get_or_set_with(0, || async { _get(3).await }).await, &0);
        assert_eq!(c.get_or_set_with(1, || async { _get(3).await }).await, &1);
        assert_eq!(c.get_or_set_with(2, || async { _get(3).await }).await, &2);
        assert_eq!(c.get_or_set_with(3, || async { _get(1).await }).await, &3);

        c.cache_reset();
        async fn _try_get(n: usize) -> Result<usize, String> {
            if n < 10 {
                Ok(n)
            } else {
                Err("dead".to_string())
            }
        }

        assert_eq!(
            c.try_get_or_set_with(0, || async {
                match _try_get(0).await {
                    Ok(n) => Ok(n),
                    Err(_) => Err("err".to_string()),
                }
            })
            .await
            .unwrap(),
            &0
        );
        assert_eq!(
            c.try_get_or_set_with(0, || async {
                match _try_get(5).await {
                    Ok(n) => Ok(n),
                    Err(_) => Err("err".to_string()),
                }
            })
            .await
            .unwrap(),
            &0
        );

        c.cache_reset();
        let res: Result<&mut usize, String> = c
            .try_get_or_set_with(0, || async { _try_get(10).await })
            .await;
        assert!(res.is_err());
        assert!(c.key_order().next().is_none());

        let res: Result<&mut usize, String> = c
            .try_get_or_set_with(0, || async { _try_get(1).await })
            .await;
        assert_eq!(res.unwrap(), &1);
        let res: Result<&mut usize, String> = c
            .try_get_or_set_with(0, || async { _try_get(5).await })
            .await;
        assert_eq!(res.unwrap(), &1);
    }
}
