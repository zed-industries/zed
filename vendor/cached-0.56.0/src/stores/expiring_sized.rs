use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};
use std::ops::Bound::{Excluded, Included};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Wrap keys so they don't need to implement Clone
#[derive(Eq)]
// todo: can we switch to an Rc?
struct CacheArc<T>(Arc<T>);

impl<T> CacheArc<T> {
    fn new(key: T) -> Self {
        CacheArc(Arc::new(key))
    }
}

impl<T> Clone for CacheArc<T> {
    fn clone(&self) -> Self {
        CacheArc(self.0.clone())
    }
}

impl<T: PartialEq> PartialEq for CacheArc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: PartialOrd> PartialOrd for CacheArc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T: Ord> Ord for CacheArc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Hash> Hash for CacheArc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Borrow<T> for CacheArc<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl Borrow<str> for CacheArc<String> {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

impl<T> Borrow<[T]> for CacheArc<Vec<T>> {
    fn borrow(&self) -> &[T] {
        self.0.as_slice()
    }
}

#[derive(Debug)]
pub enum Error {
    /// Calculating expiration `Instant`s resulted in a
    /// value outside of `Instant`s internal bounds
    TimeBounds,
}

/// A timestamped key to allow identifying key ranges
#[derive(Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Stamped<K> {
    // note: the field order matters here since the derived ord traits
    //       generate lexicographic ordering based on the top-to-bottom
    //       declaration order
    expiry: Instant,

    // wrapped in an option so it's easy to generate
    // a range bound containing None
    key: Option<CacheArc<K>>,
}

impl<K> Clone for Stamped<K> {
    fn clone(&self) -> Self {
        Self {
            expiry: self.expiry,
            key: self.key.clone(),
        }
    }
}

impl<K> Stamped<K> {
    fn bound(expiry: Instant) -> Stamped<K> {
        Stamped { expiry, key: None }
    }
}

/// A timestamped value to allow re-building a timestamped key
struct Entry<K, V> {
    expiry: Instant,
    key: CacheArc<K>,
    value: V,
}

impl<K, V> Entry<K, V> {
    fn as_stamped(&self) -> Stamped<K> {
        Stamped {
            expiry: self.expiry,
            key: Some(self.key.clone()),
        }
    }

    fn is_expired(&self) -> bool {
        self.expiry < Instant::now()
    }
}

macro_rules! impl_get {
    ($_self:expr, $key:expr) => {{
        $_self.map.get($key).and_then(|entry| {
            if entry.is_expired() {
                None
            } else {
                Some(&entry.value)
            }
        })
    }};
}

/// A cache enforcing time expiration and an optional maximum size.
/// When a maximum size is specified, the values are dropped in the
/// order of expiration date, e.g. the next value to expire is dropped.
/// This cache is intended for high read scenarios to allow for concurrent
/// reads while still enforcing expiration and an optional maximum cache size.
///
/// To accomplish this, there are a few trade-offs:
///  - Maximum cache size logic cannot support "LRU", instead dropping the next value to expire
///  - Cache keys must implement `Ord`
///  - The cache's size, reported by `.len` is only guaranteed to be accurate immediately
///    after a call to either `.evict` or `.retain_latest`
///  - Eviction must be explicitly requested, either on its own or while inserting
pub struct ExpiringSizedCache<K, V> {
    // a minimum instant to compare ranges against since
    // all keys must logically expire after the creation
    // of the cache
    min_instant: Instant,

    // k/v where entry contains corresponds to an ordered value in `keys`
    map: HashMap<CacheArc<K>, Entry<K, V>>,

    // ordered in ascending expiration `Instant`s
    // to support retaining/evicting without full traversal
    keys: BTreeSet<Stamped<K>>,

    pub ttl: Duration,
    pub size_limit: Option<usize>,
}
impl<K: Hash + Eq + Ord, V> ExpiringSizedCache<K, V> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            min_instant: Instant::now(),
            map: HashMap::new(),
            keys: BTreeSet::new(),
            ttl,
            size_limit: None,
        }
    }

    pub fn with_capacity(ttl: Duration, size: usize) -> Self {
        let mut new = Self::new(ttl);
        new.map.reserve(size);
        new
    }

    /// Set a size limit. When reached, the next entries to expire are evicted.
    /// Returns the previous value if one was set.
    pub fn size_limit(&mut self, size: usize) -> Option<usize> {
        let prev = self.size_limit;
        self.size_limit = Some(size);
        prev
    }

    /// Increase backing stores with enough capacity to store `more`
    pub fn reserve(&mut self, more: usize) {
        self.map.reserve(more);
    }

    /// Set ttl millis, return previous value
    pub fn ttl_millis(&mut self, ttl: Duration) -> Duration {
        let prev = self.ttl;
        self.ttl = ttl;
        prev
    }

    /// Evict values that have expired.
    /// Returns number of dropped items.
    pub fn evict(&mut self) -> usize {
        let cutoff = Instant::now();
        let min = Stamped::bound(self.min_instant);
        let max = Stamped::bound(cutoff);
        let min = Included(&min);
        let max = Excluded(&max);

        let remove = self.keys.range((min, max)).count();
        let mut count = 0;
        while count < remove {
            match self.keys.pop_first() {
                None => break,
                Some(stamped) => {
                    self.map.remove(
                        &stamped
                            .key
                            .expect("evicting: only artificial bounds are none"),
                    );
                    count += 1;
                }
            }
        }
        count
    }

    /// Retain only the latest `count` values, dropping the next values to expire.
    /// If `evict`, then also evict values that have expired.
    /// Returns number of dropped items.
    pub fn retain_latest(&mut self, count: usize, evict: bool) -> usize {
        let retain_drop_count = self.len().saturating_sub(count);

        let remove = if evict {
            let cutoff = Instant::now();
            let min = Stamped::bound(self.min_instant);
            let max = Stamped::bound(cutoff);
            let min = Included(&min);
            let max = Excluded(&max);
            let to_evict_count = self.keys.range((min, max)).count();
            retain_drop_count.max(to_evict_count)
        } else {
            retain_drop_count
        };

        let mut count = 0;
        while count < remove {
            match self.keys.pop_first() {
                None => break,
                Some(stamped) => {
                    self.map.remove(
                        &stamped
                            .key
                            .expect("retaining: only artificial bounds are none"),
                    );
                    count += 1;
                }
            }
        }
        count
    }

    /// Remove an entry, returning an unexpired value if it was present.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.map.remove(key) {
            None => None,
            Some(removed) => {
                self.keys.remove(&removed.as_stamped());
                if removed.is_expired() {
                    None
                } else {
                    Some(removed.value)
                }
            }
        }
    }

    /// Insert k/v pair without running eviction logic. See `.insert_ttl_evict`
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Error> {
        self.insert_ttl_evict(key, value, None, false)
    }

    /// Insert k/v pair with explicit ttl. See `.insert_ttl_evict`
    pub fn insert_ttl(&mut self, key: K, value: V, ttl: Duration) -> Result<Option<V>, Error> {
        self.insert_ttl_evict(key, value, Some(ttl), false)
    }

    /// Insert k/v pair and run eviction logic. See `.insert_ttl_evict`
    pub fn insert_evict(&mut self, key: K, value: V, evict: bool) -> Result<Option<V>, Error> {
        self.insert_ttl_evict(key, value, None, evict)
    }

    /// Optionally run eviction logic before inserting a k/v pair with an optional explicit TTL.
    /// If a `size_limit` was specified, the next entry to expire will be evicted to make space.
    /// Returns any existing unexpired value.
    pub fn insert_ttl_evict(
        &mut self,
        key: K,
        value: V,
        ttl: Option<Duration>,
        evict: bool,
    ) -> Result<Option<V>, Error> {
        // optionally evict and retain to size
        if let Some(size_limit) = self.size_limit {
            if self.len() > size_limit - 1 {
                self.retain_latest(size_limit - 1, evict);
            }
        } else if evict {
            self.evict();
        }

        let key = CacheArc::new(key);
        let expiry = Instant::now()
            .checked_add(ttl.unwrap_or(self.ttl))
            .ok_or(Error::TimeBounds)?;

        let new_stamped = Stamped {
            expiry,
            key: Some(key.clone()),
        };
        self.keys.insert(new_stamped.clone());
        let old = self.map.insert(key.clone(), Entry { expiry, key, value });
        if let Some(old) = &old {
            let old_stamped = old.as_stamped();
            // new-stamped didn't already replace an existing entry, delete it now
            if old_stamped != new_stamped {
                self.keys.remove(&old_stamped);
            }
        }
        Ok(old.and_then(|entry| {
            if entry.is_expired() {
                None
            } else {
                Some(entry.value)
            }
        }))
    }

    /// Clear all cache entries. Does not release underlying containers
    pub fn clear(&mut self) {
        self.map.clear();
        self.keys.clear();
    }

    /// Return cache size. Note, this does not evict so may return
    /// a size that includes expired entries. Run `evict` or `retain_latest`
    /// first to ensure an accurate length.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retrieve unexpired entry
    pub fn get(&self, key: &K) -> Option<&V> {
        // todo: generically support keys being borrowed types like the underlying map
        impl_get!(self, key)
    }
}

impl<V> ExpiringSizedCache<String, V> {
    /// Retrieve unexpired entry, accepting `&str` to check against `String` keys
    /// ```rust
    /// # use cached::stores::ExpiringSizedCache;
    /// # use std::time::Duration;
    /// let mut cache = ExpiringSizedCache::<String, &str>::new(Duration::from_millis(2_000));
    /// cache.insert(String::from("a"), "a").unwrap();
    /// assert_eq!(cache.get_borrowed("a").unwrap(), &"a");
    /// ```
    pub fn get_borrowed(&self, key: &str) -> Option<&V> {
        impl_get!(self, key)
    }
}

impl<T: Hash + Eq + PartialEq, V> ExpiringSizedCache<Vec<T>, V> {
    /// Retrieve unexpired entry, accepting `&[T]` to check against `Vec<T>` keys
    /// ```rust
    /// # use cached::stores::ExpiringSizedCache;
    /// # use std::time::Duration;
    /// let mut cache = ExpiringSizedCache::<Vec<usize>, &str>::new(Duration::from_millis(2_000));
    /// cache.insert(vec![0], "a").unwrap();
    /// assert_eq!(cache.get_borrowed(&[0]).unwrap(), &"a");
    /// ```
    pub fn get_borrowed(&self, key: &[T]) -> Option<&V> {
        impl_get!(self, key)
    }
}

#[cfg(test)]
mod test {
    use crate::stores::ExpiringSizedCache;
    use std::time::Duration;

    #[test]
    fn borrow_keys() {
        let mut cache = ExpiringSizedCache::with_capacity(Duration::from_millis(100), 100);
        cache.insert(String::from("a"), "a").unwrap();
        assert_eq!(cache.get_borrowed("a").unwrap(), &"a");

        let mut cache = ExpiringSizedCache::with_capacity(Duration::from_millis(100), 100);
        cache.insert(vec![0], "a").unwrap();
        assert_eq!(cache.get_borrowed(&[0]).unwrap(), &"a");
    }

    #[test]
    fn kitchen_sink() {
        let mut cache = ExpiringSizedCache::with_capacity(Duration::from_millis(100), 100);
        assert_eq!(0, cache.evict());
        assert_eq!(0, cache.retain_latest(100, true));
        assert!(cache.get(&"a".into()).is_none());

        cache.insert("a".to_string(), "A".to_string()).unwrap();
        assert_eq!(cache.get(&"a".into()), Some("A".to_string()).as_ref());
        assert_eq!(cache.len(), 1);
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(1, cache.evict());
        assert!(cache.get(&"a".into()).is_none());
        assert_eq!(cache.len(), 0);

        cache.insert("a".to_string(), "A".to_string()).unwrap();
        assert_eq!(cache.get(&"a".into()), Some("A".to_string()).as_ref());
        assert_eq!(cache.len(), 1);
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(0, cache.retain_latest(1, false));
        // expired
        assert_eq!(cache.get(&"a".into()), None);
        // in size until eviction
        assert_eq!(cache.len(), 1);
        assert_eq!(1, cache.retain_latest(1, true));
        assert!(cache.get(&"a".into()).is_none());
        assert_eq!(cache.len(), 0);

        cache.insert("a".to_string(), "a".to_string()).unwrap();
        cache.insert("b".to_string(), "b".to_string()).unwrap();
        cache.insert("c".to_string(), "c".to_string()).unwrap();
        cache.insert("d".to_string(), "d".to_string()).unwrap();
        cache.insert("e".to_string(), "e".to_string()).unwrap();
        assert_eq!(3, cache.retain_latest(2, false));
        assert_eq!(2, cache.len());
        assert_eq!(cache.get(&"a".into()), None);
        assert_eq!(cache.get(&"b".into()), None);
        assert_eq!(cache.get(&"c".into()), None);
        assert_eq!(cache.get(&"d".into()), Some("d".to_string()).as_ref());
        assert_eq!(cache.get(&"e".into()), Some("e".to_string()).as_ref());

        cache.insert("a".to_string(), "a".to_string()).unwrap();
        cache.insert("a".to_string(), "a".to_string()).unwrap();
        cache.insert("b".to_string(), "b".to_string()).unwrap();
        cache.insert("b".to_string(), "b".to_string()).unwrap();
        assert_eq!(4, cache.len());

        assert_eq!(2, cache.retain_latest(2, false));
        assert_eq!(cache.get(&"d".into()), None);
        assert_eq!(cache.get(&"e".into()), None);
        assert_eq!(cache.get(&"a".into()), Some("a".to_string()).as_ref());
        assert_eq!(cache.get(&"b".into()), Some("b".to_string()).as_ref());
        assert_eq!(2, cache.len());

        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(cache.remove(&"a".into()), None);
        // trying to get something expired will expire values
        assert_eq!(1, cache.len());

        cache.insert("a".to_string(), "a".to_string()).unwrap();
        assert_eq!(cache.remove(&"a".into()), Some("a".to_string()));
        // we haven't done anything to evict "b" so there's still one entry
        assert_eq!(1, cache.len());

        assert_eq!(1, cache.evict());
        assert_eq!(0, cache.len());

        // default ttl is 100ms
        cache
            .insert_ttl("a".to_string(), "a".to_string(), Duration::from_millis(300))
            .unwrap();
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(cache.get(&"a".into()), Some("a".to_string()).as_ref());
        assert_eq!(1, cache.len());

        std::thread::sleep(Duration::from_millis(200));
        cache
            .insert_ttl_evict(
                "b".to_string(),
                "b".to_string(),
                Some(Duration::from_millis(300)),
                true,
            )
            .unwrap();
        // a should now be evicted
        assert_eq!(1, cache.len());
        assert_eq!(cache.get_borrowed("a"), None);
    }

    #[test]
    fn size_limit() {
        let mut cache = ExpiringSizedCache::with_capacity(Duration::from_millis(100), 100);
        cache.size_limit(2);
        assert_eq!(0, cache.evict());
        assert_eq!(0, cache.retain_latest(100, true));
        assert!(cache.get(&"a".into()).is_none());

        cache.insert("a".to_string(), "A".to_string()).unwrap();
        assert_eq!(cache.get(&"a".into()), Some("A".to_string()).as_ref());
        assert_eq!(cache.len(), 1);
        cache.insert("b".to_string(), "B".to_string()).unwrap();
        assert_eq!(cache.get(&"b".into()), Some("B".to_string()).as_ref());
        assert_eq!(cache.len(), 2);
        cache.insert("c".to_string(), "C".to_string()).unwrap();
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"b".into()), Some("B".to_string()).as_ref());
        assert_eq!(cache.get(&"c".into()), Some("C".to_string()).as_ref());
        assert_eq!(cache.get(&"a".into()), None);
    }
}
