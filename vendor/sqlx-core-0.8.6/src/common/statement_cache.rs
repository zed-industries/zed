use hashlink::lru_cache::LruCache;

/// A cache for prepared statements. When full, the least recently used
/// statement gets removed.
#[derive(Debug)]
pub struct StatementCache<T> {
    inner: LruCache<String, T>,
}

impl<T> StatementCache<T> {
    /// Create a new cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: LruCache::new(capacity),
        }
    }

    /// Returns a mutable reference to the value corresponding to the given key
    /// in the cache, if any.
    pub fn get_mut(&mut self, k: &str) -> Option<&mut T> {
        self.inner.get_mut(k)
    }

    /// Inserts a new statement to the cache, returning the least recently used
    /// statement id if the cache is full, or if inserting with an existing key,
    /// the replaced existing statement.
    pub fn insert(&mut self, k: &str, v: T) -> Option<T> {
        let mut lru_item = None;

        if self.capacity() == self.len() && !self.contains_key(k) {
            lru_item = self.remove_lru();
        } else if self.contains_key(k) {
            lru_item = self.inner.remove(k);
        }

        self.inner.insert(k.into(), v);

        lru_item
    }

    /// The number of statements in the cache.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Removes the least recently used item from the cache.
    pub fn remove_lru(&mut self) -> Option<T> {
        self.inner.remove_lru().map(|(_, v)| v)
    }

    /// Clear all cached statements from the cache.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// True if cache has a value for the given key.
    pub fn contains_key(&mut self, k: &str) -> bool {
        self.inner.contains_key(k)
    }

    /// Returns the maximum number of statements the cache can hold.
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns true if the cache capacity is more than 0.
    #[allow(dead_code)] // Only used for some `cfg`s
    pub fn is_enabled(&self) -> bool {
        self.capacity() > 0
    }
}
