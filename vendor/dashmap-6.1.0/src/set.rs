use crate::iter_set::{Iter, OwningIter};
#[cfg(feature = "raw-api")]
use crate::lock::RwLock;
use crate::setref::one::Ref;
use crate::DashMap;
#[cfg(feature = "raw-api")]
use crate::HashMap;
use cfg_if::cfg_if;
use core::borrow::Borrow;
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::iter::FromIterator;
#[cfg(feature = "raw-api")]
use crossbeam_utils::CachePadded;
use std::collections::hash_map::RandomState;

/// DashSet is a thin wrapper around [`DashMap`] using `()` as the value type. It uses
/// methods and types which are more convenient to work with on a set.
///
/// [`DashMap`]: struct.DashMap.html
pub struct DashSet<K, S = RandomState> {
    pub(crate) inner: DashMap<K, (), S>,
}

impl<K: Eq + Hash + fmt::Debug, S: BuildHasher + Clone> fmt::Debug for DashSet<K, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl<K: Eq + Hash + Clone, S: Clone> Clone for DashSet<K, S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner)
    }
}

impl<K, S> Default for DashSet<K, S>
where
    K: Eq + Hash,
    S: Default + BuildHasher + Clone,
{
    fn default() -> Self {
        Self::with_hasher(Default::default())
    }
}

impl<'a, K: 'a + Eq + Hash> DashSet<K, RandomState> {
    /// Creates a new DashSet with a capacity of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let games = DashSet::new();
    /// games.insert("Veloren");
    /// ```
    pub fn new() -> Self {
        Self::with_hasher(RandomState::default())
    }

    /// Creates a new DashMap with a specified starting capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let numbers = DashSet::with_capacity(2);
    /// numbers.insert(2);
    /// numbers.insert(8);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, RandomState::default())
    }
}

impl<'a, K: 'a + Eq + Hash, S: BuildHasher + Clone> DashSet<K, S> {
    /// Creates a new DashMap with a capacity of 0 and the provided hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let games = DashSet::with_hasher(s);
    /// games.insert("Veloren");
    /// ```
    pub fn with_hasher(hasher: S) -> Self {
        Self::with_capacity_and_hasher(0, hasher)
    }

    /// Creates a new DashMap with a specified starting capacity and hasher.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let numbers = DashSet::with_capacity_and_hasher(2, s);
    /// numbers.insert(2);
    /// numbers.insert(8);
    /// ```
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            inner: DashMap::with_capacity_and_hasher(capacity, hasher),
        }
    }

    /// Hash a given item to produce a usize.
    /// Uses the provided or default HashBuilder.
    pub fn hash_usize<T: Hash>(&self, item: &T) -> usize {
        self.inner.hash_usize(item)
    }

    cfg_if! {
        if #[cfg(feature = "raw-api")] {
            /// Allows you to peek at the inner shards that store your data.
            /// You should probably not use this unless you know what you are doing.
            ///
            /// Requires the `raw-api` feature to be enabled.
            ///
            /// # Examples
            ///
            /// ```
            /// use dashmap::DashSet;
            ///
            /// let set = DashSet::<()>::new();
            /// println!("Amount of shards: {}", set.shards().len());
            /// ```
            pub fn shards(&self) -> &[CachePadded<RwLock<HashMap<K, ()>>>] {
                self.inner.shards()
            }
        }
    }

    cfg_if! {
        if #[cfg(feature = "raw-api")] {
            /// Finds which shard a certain key is stored in.
            /// You should probably not use this unless you know what you are doing.
            /// Note that shard selection is dependent on the default or provided HashBuilder.
            ///
            /// Requires the `raw-api` feature to be enabled.
            ///
            /// # Examples
            ///
            /// ```
            /// use dashmap::DashSet;
            ///
            /// let set = DashSet::new();
            /// set.insert("coca-cola");
            /// println!("coca-cola is stored in shard: {}", set.determine_map("coca-cola"));
            /// ```
            pub fn determine_map<Q>(&self, key: &Q) -> usize
            where
                K: Borrow<Q>,
                Q: Hash + Eq + ?Sized,
            {
                self.inner.determine_map(key)
            }
        }
    }

    cfg_if! {
        if #[cfg(feature = "raw-api")] {
            /// Finds which shard a certain hash is stored in.
            ///
            /// Requires the `raw-api` feature to be enabled.
            ///
            /// # Examples
            ///
            /// ```
            /// use dashmap::DashSet;
            ///
            /// let set: DashSet<i32> = DashSet::new();
            /// let key = "key";
            /// let hash = set.hash_usize(&key);
            /// println!("hash is stored in shard: {}", set.determine_shard(hash));
            /// ```
            pub fn determine_shard(&self, hash: usize) -> usize {
                self.inner.determine_shard(hash)
            }
        }
    }

    /// Inserts a key into the set. Returns true if the key was not already in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let set = DashSet::new();
    /// set.insert("I am the key!");
    /// ```
    pub fn insert(&self, key: K) -> bool {
        self.inner.insert(key, ()).is_none()
    }

    /// Removes an entry from the map, returning the key if it existed in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let soccer_team = DashSet::new();
    /// soccer_team.insert("Jack");
    /// assert_eq!(soccer_team.remove("Jack").unwrap(), "Jack");
    /// ```
    pub fn remove<Q>(&self, key: &Q) -> Option<K>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.remove(key).map(|(k, _)| k)
    }

    /// Removes an entry from the set, returning the key
    /// if the entry existed and the provided conditional function returned true.
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let soccer_team = DashSet::new();
    /// soccer_team.insert("Sam");
    /// soccer_team.remove_if("Sam", |player| player.starts_with("Ja"));
    /// assert!(soccer_team.contains("Sam"));
    /// ```
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let soccer_team = DashSet::new();
    /// soccer_team.insert("Sam");
    /// soccer_team.remove_if("Jacob", |player| player.starts_with("Ja"));
    /// assert!(!soccer_team.contains("Jacob"));
    /// ```
    pub fn remove_if<Q>(&self, key: &Q, f: impl FnOnce(&K) -> bool) -> Option<K>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // TODO: Don't create another closure around f
        self.inner.remove_if(key, |k, _| f(k)).map(|(k, _)| k)
    }

    /// Creates an iterator over a DashMap yielding immutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let words = DashSet::new();
    /// words.insert("hello");
    /// assert_eq!(words.iter().count(), 1);
    /// ```
    pub fn iter(&'a self) -> Iter<'a, K, S, DashMap<K, (), S>> {
        let iter = self.inner.iter();

        Iter::new(iter)
    }

    /// Get a reference to an entry in the set
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let youtubers = DashSet::new();
    /// youtubers.insert("Bosnian Bill");
    /// assert_eq!(*youtubers.get("Bosnian Bill").unwrap(), "Bosnian Bill");
    /// ```
    pub fn get<Q>(&'a self, key: &Q) -> Option<Ref<'a, K>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get(key).map(Ref::new)
    }

    /// Remove excess capacity to reduce memory usage.
    pub fn shrink_to_fit(&self) {
        self.inner.shrink_to_fit()
    }

    /// Retain elements that whose predicates return true
    /// and discard elements whose predicates return false.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let people = DashSet::new();
    /// people.insert("Albin");
    /// people.insert("Jones");
    /// people.insert("Charlie");
    /// people.retain(|name| name.contains('i'));
    /// assert_eq!(people.len(), 2);
    /// ```
    pub fn retain(&self, mut f: impl FnMut(&K) -> bool) {
        self.inner.retain(|k, _| f(k))
    }

    /// Fetches the total number of keys stored in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let people = DashSet::new();
    /// people.insert("Albin");
    /// people.insert("Jones");
    /// people.insert("Charlie");
    /// assert_eq!(people.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Checks if the set is empty or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let map = DashSet::<()>::new();
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Removes all keys in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let people = DashSet::new();
    /// people.insert("Albin");
    /// assert!(!people.is_empty());
    /// people.clear();
    /// assert!(people.is_empty());
    /// ```
    pub fn clear(&self) {
        self.inner.clear()
    }

    /// Returns how many keys the set can store without reallocating.
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Checks if the set contains a specific key.
    ///
    /// # Examples
    ///
    /// ```
    /// use dashmap::DashSet;
    ///
    /// let people = DashSet::new();
    /// people.insert("Dakota Cherries");
    /// assert!(people.contains("Dakota Cherries"));
    /// ```
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.contains_key(key)
    }
}

impl<K: Eq + Hash, S: BuildHasher + Clone> IntoIterator for DashSet<K, S> {
    type Item = K;

    type IntoIter = OwningIter<K, S>;

    fn into_iter(self) -> Self::IntoIter {
        OwningIter::new(self.inner.into_iter())
    }
}

impl<K: Eq + Hash, S: BuildHasher + Clone> Extend<K> for DashSet<K, S> {
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        let iter = iter.into_iter().map(|k| (k, ()));

        self.inner.extend(iter)
    }
}

impl<K: Eq + Hash, S: BuildHasher + Clone + Default> FromIterator<K> for DashSet<K, S> {
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let mut set = DashSet::default();

        set.extend(iter);

        set
    }
}

#[cfg(feature = "typesize")]
impl<K, S> typesize::TypeSize for DashSet<K, S>
where
    K: typesize::TypeSize + Eq + Hash,
    S: typesize::TypeSize + Clone + BuildHasher,
{
    fn extra_size(&self) -> usize {
        self.inner.extra_size()
    }

    typesize::if_typesize_details! {
        fn get_collection_item_count(&self) -> Option<usize> {
            Some(self.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DashSet;

    #[test]
    fn test_basic() {
        let set = DashSet::new();

        set.insert(0);

        assert_eq!(set.get(&0).as_deref(), Some(&0));
    }

    #[test]
    fn test_default() {
        let set: DashSet<u32> = DashSet::default();

        set.insert(0);

        assert_eq!(set.get(&0).as_deref(), Some(&0));
    }

    #[test]
    fn test_multiple_hashes() {
        let set = DashSet::<u32>::default();

        for i in 0..100 {
            assert!(set.insert(i));
        }

        for i in 0..100 {
            assert!(!set.insert(i));
        }

        for i in 0..100 {
            assert_eq!(Some(i), set.remove(&i));
        }

        for i in 0..100 {
            assert_eq!(None, set.remove(&i));
        }
    }
}
