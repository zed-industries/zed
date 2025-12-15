use std::{cmp::Ordering, fmt::Debug};

use crate::{Bias, ContextLessSummary, Dimension, Edit, Item, KeyedItem, SeekTarget, SumTree};

/// A cheaply-cloneable ordered map based on a [SumTree](crate::SumTree).
#[derive(Clone, PartialEq, Eq)]
pub struct TreeMap<K, V>(SumTree<MapEntry<K, V>>)
where
    K: Clone + Ord,
    V: Clone;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapEntry<K, V> {
    key: K,
    value: V,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapKey<K>(Option<K>);

impl<K> Default for MapKey<K> {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Clone, Debug)]
pub struct MapKeyRef<'a, K>(Option<&'a K>);

impl<K> Default for MapKeyRef<'_, K> {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeSet<K>(TreeMap<K, ()>)
where
    K: Clone + Ord;

impl<K: Clone + Ord, V: Clone> TreeMap<K, V> {
    pub fn from_ordered_entries(entries: impl IntoIterator<Item = (K, V)>) -> Self {
        let tree = SumTree::from_iter(
            entries
                .into_iter()
                .map(|(key, value)| MapEntry { key, value }),
            (),
        );
        Self(tree)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let (.., item) = self
            .0
            .find::<MapKeyRef<'_, K>, _>((), &MapKeyRef(Some(key)), Bias::Left);
        if let Some(item) = item {
            if Some(key) == item.key().0.as_ref() {
                Some(&item.value)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.0.insert_or_replace(MapEntry { key, value }, ());
    }

    pub fn insert_or_replace(&mut self, key: K, value: V) -> Option<V> {
        self.0
            .insert_or_replace(MapEntry { key, value }, ())
            .map(|it| it.value)
    }

    pub fn extend(&mut self, iter: impl IntoIterator<Item = (K, V)>) {
        let edits: Vec<_> = iter
            .into_iter()
            .map(|(key, value)| Edit::Insert(MapEntry { key, value }))
            .collect();
        self.0.edit(edits, ());
    }

    pub fn clear(&mut self) {
        self.0 = SumTree::default();
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut removed = None;
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>(());
        let key = MapKeyRef(Some(key));
        let mut new_tree = cursor.slice(&key, Bias::Left);
        if key.cmp(&cursor.end(), ()) == Ordering::Equal {
            removed = Some(cursor.item().unwrap().value.clone());
            cursor.next();
        }
        new_tree.append(cursor.suffix(), ());
        drop(cursor);
        self.0 = new_tree;
        removed
    }

    pub fn remove_range(&mut self, start: &impl MapSeekTarget<K>, end: &impl MapSeekTarget<K>) {
        let start = MapSeekTargetAdaptor(start);
        let end = MapSeekTargetAdaptor(end);
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>(());
        let mut new_tree = cursor.slice(&start, Bias::Left);
        cursor.seek(&end, Bias::Left);
        new_tree.append(cursor.suffix(), ());
        drop(cursor);
        self.0 = new_tree;
    }

    /// Returns the key-value pair with the greatest key less than or equal to the given key.
    pub fn closest(&self, key: &K) -> Option<(&K, &V)> {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>(());
        let key = MapKeyRef(Some(key));
        cursor.seek(&key, Bias::Right);
        cursor.prev();
        cursor.item().map(|item| (&item.key, &item.value))
    }

    pub fn iter_from<'a>(&'a self, from: &K) -> impl Iterator<Item = (&'a K, &'a V)> + 'a {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>(());
        let from_key = MapKeyRef(Some(from));
        cursor.seek(&from_key, Bias::Left);

        cursor.map(|map_entry| (&map_entry.key, &map_entry.value))
    }

    pub fn update<F, T>(&mut self, key: &K, f: F) -> Option<T>
    where
        F: FnOnce(&mut V) -> T,
    {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>(());
        let key = MapKeyRef(Some(key));
        let mut new_tree = cursor.slice(&key, Bias::Left);
        let mut result = None;
        if key.cmp(&cursor.end(), ()) == Ordering::Equal {
            let mut updated = cursor.item().unwrap().clone();
            result = Some(f(&mut updated.value));
            new_tree.push(updated, ());
            cursor.next();
        }
        new_tree.append(cursor.suffix(), ());
        drop(cursor);
        self.0 = new_tree;
        result
    }

    pub fn retain<F: FnMut(&K, &V) -> bool>(&mut self, mut predicate: F) {
        let mut new_map = SumTree::<MapEntry<K, V>>::default();

        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>(());
        cursor.next();
        while let Some(item) = cursor.item() {
            if predicate(&item.key, &item.value) {
                new_map.push(item.clone(), ());
            }
            cursor.next();
        }
        drop(cursor);

        self.0 = new_map;
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + '_ {
        self.0.iter().map(|entry| (&entry.key, &entry.value))
    }

    pub fn values(&self) -> impl Iterator<Item = &V> + '_ {
        self.0.iter().map(|entry| &entry.value)
    }

    pub fn first(&self) -> Option<(&K, &V)> {
        self.0.first().map(|entry| (&entry.key, &entry.value))
    }

    pub fn last(&self) -> Option<(&K, &V)> {
        self.0.last().map(|entry| (&entry.key, &entry.value))
    }

    pub fn insert_tree(&mut self, other: TreeMap<K, V>) {
        let edits = other
            .iter()
            .map(|(key, value)| {
                Edit::Insert(MapEntry {
                    key: key.to_owned(),
                    value: value.to_owned(),
                })
            })
            .collect();

        self.0.edit(edits, ());
    }
}

impl<K, V> Debug for TreeMap<K, V>
where
    K: Clone + Debug + Ord,
    V: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[derive(Debug)]
struct MapSeekTargetAdaptor<'a, T>(&'a T);

impl<'a, K: Clone + Ord, T: MapSeekTarget<K>> SeekTarget<'a, MapKey<K>, MapKeyRef<'a, K>>
    for MapSeekTargetAdaptor<'_, T>
{
    fn cmp(&self, cursor_location: &MapKeyRef<K>, _: ()) -> Ordering {
        if let Some(key) = &cursor_location.0 {
            MapSeekTarget::cmp_cursor(self.0, key)
        } else {
            Ordering::Greater
        }
    }
}

pub trait MapSeekTarget<K> {
    fn cmp_cursor(&self, cursor_location: &K) -> Ordering;
}

impl<K: Ord> MapSeekTarget<K> for K {
    fn cmp_cursor(&self, cursor_location: &K) -> Ordering {
        self.cmp(cursor_location)
    }
}

impl<K, V> Default for TreeMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K, V> Item for MapEntry<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    type Summary = MapKey<K>;

    fn summary(&self, _cx: ()) -> Self::Summary {
        self.key()
    }
}

impl<K, V> KeyedItem for MapEntry<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    type Key = MapKey<K>;

    fn key(&self) -> Self::Key {
        MapKey(Some(self.key.clone()))
    }
}

impl<K> ContextLessSummary for MapKey<K>
where
    K: Clone,
{
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        *self = summary.clone()
    }
}

impl<'a, K> Dimension<'a, MapKey<K>> for MapKeyRef<'a, K>
where
    K: Clone + Ord,
{
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a MapKey<K>, _: ()) {
        self.0 = summary.0.as_ref();
    }
}

impl<'a, K> SeekTarget<'a, MapKey<K>, MapKeyRef<'a, K>> for MapKeyRef<'_, K>
where
    K: Clone + Ord,
{
    fn cmp(&self, cursor_location: &MapKeyRef<K>, _: ()) -> Ordering {
        Ord::cmp(&self.0, &cursor_location.0)
    }
}

impl<K> Default for TreeSet<K>
where
    K: Clone + Ord,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K> TreeSet<K>
where
    K: Clone + Ord,
{
    pub fn from_ordered_entries(entries: impl IntoIterator<Item = K>) -> Self {
        Self(TreeMap::from_ordered_entries(
            entries.into_iter().map(|key| (key, ())),
        ))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, key: K) {
        self.0.insert(key, ());
    }

    pub fn remove(&mut self, key: &K) -> bool {
        self.0.remove(key).is_some()
    }

    pub fn extend(&mut self, iter: impl IntoIterator<Item = K>) {
        self.0.extend(iter.into_iter().map(|key| (key, ())));
    }

    pub fn contains(&self, key: &K) -> bool {
        self.0.get(key).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> + '_ {
        self.0.iter().map(|(k, _)| k)
    }

    pub fn iter_from<'a>(&'a self, key: &K) -> impl Iterator<Item = &'a K> + 'a {
        self.0.iter_from(key).map(move |(k, _)| k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut map = TreeMap::default();
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![]);

        map.insert(3, "c");
        assert_eq!(map.get(&3), Some(&"c"));
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&3, &"c")]);

        map.insert(1, "a");
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&1, &"a"), (&3, &"c")]);

        map.insert(2, "b");
        assert_eq!(map.get(&2), Some(&"b"));
        assert_eq!(map.get(&1), Some(&"a"));
        assert_eq!(map.get(&3), Some(&"c"));
        assert_eq!(
            map.iter().collect::<Vec<_>>(),
            vec![(&1, &"a"), (&2, &"b"), (&3, &"c")]
        );

        assert_eq!(map.closest(&0), None);
        assert_eq!(map.closest(&1), Some((&1, &"a")));
        assert_eq!(map.closest(&10), Some((&3, &"c")));

        map.remove(&2);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&1, &"a"), (&3, &"c")]);

        assert_eq!(map.closest(&2), Some((&1, &"a")));

        map.remove(&3);
        assert_eq!(map.get(&3), None);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&1, &"a")]);

        map.remove(&1);
        assert_eq!(map.get(&1), None);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![]);

        map.insert(4, "d");
        map.insert(5, "e");
        map.insert(6, "f");
        map.retain(|key, _| *key % 2 == 0);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&4, &"d"), (&6, &"f")]);
    }

    #[test]
    fn test_iter_from() {
        let mut map = TreeMap::default();

        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("baa", 3);
        map.insert("baaab", 4);
        map.insert("c", 5);

        let result = map
            .iter_from(&"ba")
            .take_while(|(key, _)| key.starts_with("ba"))
            .collect::<Vec<_>>();

        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|(k, _)| k == &&"baa"));
        assert!(result.iter().any(|(k, _)| k == &&"baaab"));

        let result = map
            .iter_from(&"c")
            .take_while(|(key, _)| key.starts_with("c"))
            .collect::<Vec<_>>();

        assert_eq!(result.len(), 1);
        assert!(result.iter().any(|(k, _)| k == &&"c"));
    }

    #[test]
    fn test_insert_tree() {
        let mut map = TreeMap::default();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let mut other = TreeMap::default();
        other.insert("a", 2);
        other.insert("b", 2);
        other.insert("d", 4);

        map.insert_tree(other);

        assert_eq!(map.iter().count(), 4);
        assert_eq!(map.get(&"a"), Some(&2));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"c"), Some(&3));
        assert_eq!(map.get(&"d"), Some(&4));
    }

    #[test]
    fn test_extend() {
        let mut map = TreeMap::default();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        map.extend([("a", 2), ("b", 2), ("d", 4)]);
        assert_eq!(map.iter().count(), 4);
        assert_eq!(map.get(&"a"), Some(&2));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"c"), Some(&3));
        assert_eq!(map.get(&"d"), Some(&4));
    }

    #[test]
    fn test_remove_between_and_path_successor() {
        use std::path::{Path, PathBuf};

        #[derive(Debug)]
        pub struct PathDescendants<'a>(&'a Path);

        impl MapSeekTarget<PathBuf> for PathDescendants<'_> {
            fn cmp_cursor(&self, key: &PathBuf) -> Ordering {
                if key.starts_with(self.0) {
                    Ordering::Greater
                } else {
                    self.0.cmp(key)
                }
            }
        }

        let mut map = TreeMap::default();

        map.insert(PathBuf::from("a"), 1);
        map.insert(PathBuf::from("a/a"), 1);
        map.insert(PathBuf::from("b"), 2);
        map.insert(PathBuf::from("b/a/a"), 3);
        map.insert(PathBuf::from("b/a/a/a/b"), 4);
        map.insert(PathBuf::from("c"), 5);
        map.insert(PathBuf::from("c/a"), 6);

        map.remove_range(
            &PathBuf::from("b/a"),
            &PathDescendants(&PathBuf::from("b/a")),
        );

        assert_eq!(map.get(&PathBuf::from("a")), Some(&1));
        assert_eq!(map.get(&PathBuf::from("a/a")), Some(&1));
        assert_eq!(map.get(&PathBuf::from("b")), Some(&2));
        assert_eq!(map.get(&PathBuf::from("b/a/a")), None);
        assert_eq!(map.get(&PathBuf::from("b/a/a/a/b")), None);
        assert_eq!(map.get(&PathBuf::from("c")), Some(&5));
        assert_eq!(map.get(&PathBuf::from("c/a")), Some(&6));

        map.remove_range(&PathBuf::from("c"), &PathDescendants(&PathBuf::from("c")));

        assert_eq!(map.get(&PathBuf::from("a")), Some(&1));
        assert_eq!(map.get(&PathBuf::from("a/a")), Some(&1));
        assert_eq!(map.get(&PathBuf::from("b")), Some(&2));
        assert_eq!(map.get(&PathBuf::from("c")), None);
        assert_eq!(map.get(&PathBuf::from("c/a")), None);

        map.remove_range(&PathBuf::from("a"), &PathDescendants(&PathBuf::from("a")));

        assert_eq!(map.get(&PathBuf::from("a")), None);
        assert_eq!(map.get(&PathBuf::from("a/a")), None);
        assert_eq!(map.get(&PathBuf::from("b")), Some(&2));

        map.remove_range(&PathBuf::from("b"), &PathDescendants(&PathBuf::from("b")));

        assert_eq!(map.get(&PathBuf::from("b")), None);
    }
}
