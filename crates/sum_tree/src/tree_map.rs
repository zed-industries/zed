use std::{cmp::Ordering, fmt::Debug};

use crate::{Bias, Dimension, Item, KeyedItem, SeekTarget, SumTree, Summary};

#[derive(Clone)]
pub struct TreeMap<K, V>(SumTree<MapEntry<K, V>>)
where
    K: Clone + Debug + Default + Ord,
    V: Clone + Debug;

#[derive(Clone)]
pub struct MapEntry<K, V> {
    key: K,
    value: V,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapKey<K>(K);

#[derive(Clone, Debug, Default)]
pub struct MapKeyRef<'a, K>(Option<&'a K>);

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

    pub fn remove<'a>(&mut self, key: &'a K) -> Option<V> {
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

    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (&'a K, &'a V)> {
        self.0.iter().map(|entry| (&entry.key, &entry.value))
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

        map.remove(&2);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&1, &"a"), (&3, &"c")]);

        map.remove(&3);
        assert_eq!(map.get(&3), None);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![(&1, &"a")]);

        map.remove(&1);
        assert_eq!(map.get(&1), None);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![]);
    }
}
