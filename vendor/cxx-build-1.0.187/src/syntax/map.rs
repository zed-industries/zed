use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Index;

pub(crate) use self::ordered::OrderedMap;
pub(crate) use self::unordered::UnorderedMap;
pub(crate) use std::collections::hash_map::Entry;

mod ordered {
    use indexmap::Equivalent;
    use std::hash::Hash;

    pub(crate) struct OrderedMap<K, V>(indexmap::IndexMap<K, V>);

    impl<K, V> OrderedMap<K, V> {
        pub(crate) fn new() -> Self {
            OrderedMap(indexmap::IndexMap::new())
        }

        pub(crate) fn keys(&self) -> indexmap::map::Keys<K, V> {
            self.0.keys()
        }

        pub(crate) fn contains_key<Q>(&self, key: &Q) -> bool
        where
            Q: ?Sized + Hash + Equivalent<K>,
        {
            self.0.contains_key(key)
        }
    }

    impl<K, V> OrderedMap<K, V>
    where
        K: Hash + Eq,
    {
        pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
            self.0.insert(key, value)
        }

        pub(crate) fn entry(&mut self, key: K) -> indexmap::map::Entry<K, V> {
            self.0.entry(key)
        }
    }

    impl<'a, K, V> IntoIterator for &'a OrderedMap<K, V> {
        type Item = (&'a K, &'a V);
        type IntoIter = indexmap::map::Iter<'a, K, V>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.iter()
        }
    }
}

mod unordered {
    use crate::syntax::set::UnorderedSet;
    use std::borrow::Borrow;
    use std::collections::hash_map::{Entry, HashMap};
    use std::hash::Hash;

    // Wrapper prohibits accidentally introducing iteration over the map, which
    // could lead to nondeterministic generated code.
    pub(crate) struct UnorderedMap<K, V>(HashMap<K, V>);

    impl<K, V> UnorderedMap<K, V> {
        pub(crate) fn new() -> Self {
            UnorderedMap(HashMap::new())
        }
    }

    impl<K, V> UnorderedMap<K, V>
    where
        K: Hash + Eq,
    {
        pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
            self.0.insert(key, value)
        }

        pub(crate) fn contains_key<Q>(&self, key: &Q) -> bool
        where
            K: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.0.contains_key(key)
        }

        pub(crate) fn get<Q>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.0.get(key)
        }

        pub(crate) fn entry(&mut self, key: K) -> Entry<K, V> {
            self.0.entry(key)
        }

        #[allow(dead_code)] // only used by cxx-build, not cxxbridge-macro
        pub(crate) fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: ?Sized + Hash + Eq,
        {
            self.0.remove(key)
        }

        pub(crate) fn keys(&self) -> UnorderedSet<K>
        where
            K: Copy,
        {
            let mut set = UnorderedSet::new();
            for key in self.0.keys() {
                set.insert(*key);
            }
            set
        }
    }
}

impl<K, V> Default for UnorderedMap<K, V> {
    fn default() -> Self {
        UnorderedMap::new()
    }
}

impl<Q, K, V> Index<&Q> for UnorderedMap<K, V>
where
    K: Borrow<Q> + Hash + Eq,
    Q: ?Sized + Hash + Eq,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).unwrap()
    }
}
