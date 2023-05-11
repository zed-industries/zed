use std::{cmp::Ordering, fmt::Debug, iter};

use crate::{Bias, Dimension, Edit, Item, KeyedItem, SeekTarget, SumTree, Summary};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeMap<K, V>(SumTree<MapEntry<K, V>>)
where
    K: Clone + Debug + Default + Ord,
    V: Clone + Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapEntry<K, V> {
    key: K,
    value: V,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapKey<K>(K);

#[derive(Clone, Debug, Default)]
pub struct MapKeyRef<'a, K>(Option<&'a K>);

#[derive(Clone)]
pub struct TreeSet<K>(TreeMap<K, ()>)
where
    K: Clone + Debug + Default + Ord;

impl<K: Clone + Debug + Default + Ord, V: Clone + Debug> TreeMap<K, V> {
    pub fn from_ordered_entries(entries: impl IntoIterator<Item = (K, V)>) -> Self {
        let tree = SumTree::from_iter(
            entries
                .into_iter()
                .map(|(key, value)| MapEntry { key, value }),
            &(),
        );
        Self(tree)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get<'a>(&self, key: &'a K) -> Option<&V> {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
        cursor.seek(&MapKeyRef(Some(key)), Bias::Left, &());
        if let Some(item) = cursor.item() {
            if *key == item.key().0 {
                Some(&item.value)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.0.insert_or_replace(MapEntry { key, value }, &());
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut removed = None;
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
        let key = MapKeyRef(Some(key));
        let mut new_tree = cursor.slice(&key, Bias::Left, &());
        if key.cmp(&cursor.end(&()), &()) == Ordering::Equal {
            removed = Some(cursor.item().unwrap().value.clone());
            cursor.next(&());
        }
        new_tree.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        self.0 = new_tree;
        removed
    }

    /// Returns the key-value pair with the greatest key less than or equal to the given key.
    pub fn closest(&self, key: &K) -> Option<(&K, &V)> {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
        let key = MapKeyRef(Some(key));
        cursor.seek(&key, Bias::Right, &());
        cursor.prev(&());
        cursor.item().map(|item| (&item.key, &item.value))
    }

    pub fn remove_between(&mut self, from: &K, until: &K) {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
        let from_key = MapKeyRef(Some(from));
        let mut new_tree = cursor.slice(&from_key, Bias::Left, &());
        let until_key = MapKeyRef(Some(until));
        cursor.seek_forward(&until_key, Bias::Left, &());
        new_tree.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        self.0 = new_tree;
    }

    pub fn remove_from_while<F>(&mut self, from: &K, mut f: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
        let from_key = MapKeyRef(Some(from));
        let mut new_tree = cursor.slice(&from_key, Bias::Left, &());
        while let Some(item) = cursor.item() {
            if !f(&item.key, &item.value) {
                break;
            }
            cursor.next(&());
        }
        new_tree.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        self.0 = new_tree;
    }


    pub fn get_from_while<'tree, F>(&'tree self, from: &'tree K, mut f: F) -> impl Iterator<Item = (&K, &V)> + '_
        where
            F: FnMut(&K, &K, &V) -> bool + 'tree,
        {
            let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
            let from_key = MapKeyRef(Some(from));
            cursor.seek(&from_key, Bias::Left, &());

            iter::from_fn(move || {
                let result = cursor.item().and_then(|item| {
                    (f(from, &item.key, &item.value))
                        .then(|| (&item.key, &item.value))
                });
                cursor.next(&());
                result
            })
        }


    pub fn update<F, T>(&mut self, key: &K, f: F) -> Option<T>
    where
        F: FnOnce(&mut V) -> T,
    {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
        let key = MapKeyRef(Some(key));
        let mut new_tree = cursor.slice(&key, Bias::Left, &());
        let mut result = None;
        if key.cmp(&cursor.end(&()), &()) == Ordering::Equal {
            let mut updated = cursor.item().unwrap().clone();
            result = Some(f(&mut updated.value));
            new_tree.push(updated, &());
            cursor.next(&());
        }
        new_tree.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        self.0 = new_tree;
        result
    }

    pub fn retain<F: FnMut(&K, &V) -> bool>(&mut self, mut predicate: F) {
        let mut new_map = SumTree::<MapEntry<K, V>>::default();

        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
        cursor.next(&());
        while let Some(item) = cursor.item() {
            if predicate(&item.key, &item.value) {
                new_map.push(item.clone(), &());
            }
            cursor.next(&());
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

        self.0.edit(edits, &());
    }
}

impl<K, V> Default for TreeMap<K, V>
where
    K: Clone + Debug + Default + Ord,
    V: Clone + Debug,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K, V> Item for MapEntry<K, V>
where
    K: Clone + Debug + Default + Ord,
    V: Clone,
{
    type Summary = MapKey<K>;

    fn summary(&self) -> Self::Summary {
        self.key()
    }
}

impl<K, V> KeyedItem for MapEntry<K, V>
where
    K: Clone + Debug + Default + Ord,
    V: Clone,
{
    type Key = MapKey<K>;

    fn key(&self) -> Self::Key {
        MapKey(self.key.clone())
    }
}

impl<K> Summary for MapKey<K>
where
    K: Clone + Debug + Default,
{
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        *self = summary.clone()
    }
}

impl<'a, K> Dimension<'a, MapKey<K>> for MapKeyRef<'a, K>
where
    K: Clone + Debug + Default + Ord,
{
    fn add_summary(&mut self, summary: &'a MapKey<K>, _: &()) {
        self.0 = Some(&summary.0)
    }
}

impl<'a, K> SeekTarget<'a, MapKey<K>, MapKeyRef<'a, K>> for MapKeyRef<'_, K>
where
    K: Clone + Debug + Default + Ord,
{
    fn cmp(&self, cursor_location: &MapKeyRef<K>, _: &()) -> Ordering {
        self.0.cmp(&cursor_location.0)
    }
}

impl<K> Default for TreeSet<K>
where
    K: Clone + Debug + Default + Ord,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K> TreeSet<K>
where
    K: Clone + Debug + Default + Ord,
{
    pub fn from_ordered_entries(entries: impl IntoIterator<Item = K>) -> Self {
        Self(TreeMap::from_ordered_entries(
            entries.into_iter().map(|key| (key, ())),
        ))
    }

    pub fn insert(&mut self, key: K) {
        self.0.insert(key, ());
    }

    pub fn contains(&self, key: &K) -> bool {
        self.0.get(key).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> + '_ {
        self.0.iter().map(|(k, _)| k)
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
    fn test_remove_between() {
        let mut map = TreeMap::default();

        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("baa", 3);
        map.insert("baaab", 4);
        map.insert("c", 5);

        map.remove_between(&"ba", &"bb");

        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"baaa"), None);
        assert_eq!(map.get(&"baaaab"), None);
        assert_eq!(map.get(&"c"), Some(&5));
    }

    #[test]
    fn test_remove_from_while() {
        let mut map = TreeMap::default();

        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("baa", 3);
        map.insert("baaab", 4);
        map.insert("c", 5);

        map.remove_from_while(&"ba", |key, _| key.starts_with(&"ba"));

        assert_eq!(map.get(&"a"), Some(&1));
        assert_eq!(map.get(&"b"), Some(&2));
        assert_eq!(map.get(&"baaa"), None);
        assert_eq!(map.get(&"baaaab"), None);
        assert_eq!(map.get(&"c"), Some(&5));
    }

    #[test]
    fn test_get_from_while() {
        let mut map = TreeMap::default();

        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("baa", 3);
        map.insert("baaab", 4);
        map.insert("c", 5);

        let result = map.get_from_while(&"ba", |key, _| key.starts_with(&"ba")).collect::<Vec<_>>();

        assert_eq!(result.len(), 2);
        assert!(result.iter().find(|(k, _)| k == &&"baa").is_some());
        assert!(result.iter().find(|(k, _)| k == &&"baaab").is_some());

        let result = map.get_from_while(&"c", |key, _| key.starts_with(&"c")).collect::<Vec<_>>();

        assert_eq!(result.len(), 1);
        assert!(result.iter().find(|(k, _)| k == &&"c").is_some());
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
}
