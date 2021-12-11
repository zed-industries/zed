use std::{cmp::Ordering, fmt::Debug};

use crate::{Bias, Dimension, Item, KeyedItem, SeekTarget, SumTree, Summary};

pub struct TreeMap<K, V>(SumTree<MapEntry<K, V>>)
where
    K: Clone + Debug + Default,
    V: Clone + Debug + Default;

#[derive(Clone)]
pub struct MapEntry<K, V> {
    key: K,
    value: V,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapKey<K>(K);

#[derive(Clone, Debug, Default)]
pub struct MapKeyRef<'a, K>(Option<&'a K>);

impl<K: Clone + Debug + Default + Ord, V: Clone + Debug + Default> TreeMap<K, V> {
    pub fn get<'a>(&self, key: &'a K) -> Option<&V> {
        let mut cursor = self.0.cursor::<MapKeyRef<'_, K>>();
        let key = MapKeyRef(Some(key));
        cursor.seek(&key, Bias::Left, &());
        if key.cmp(cursor.start(), &()) == Ordering::Equal {
            Some(&cursor.item().unwrap().value)
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
        if key.cmp(cursor.start(), &()) == Ordering::Equal {
            removed = Some(cursor.item().unwrap().value.clone());
            cursor.next(&());
        }
        new_tree.push_tree(cursor.suffix(&()), &());
        drop(cursor);
        self.0 = new_tree;
        removed
    }
}

impl<K, V> Item for MapEntry<K, V>
where
    K: Clone + Debug + Default + Clone,
    V: Clone,
{
    type Summary = MapKey<K>;

    fn summary(&self) -> Self::Summary {
        todo!()
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

    fn add_summary(&mut self, summary: &Self, cx: &()) {
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
    fn cmp(&self, cursor_location: &MapKeyRef<K>, cx: &()) -> Ordering {
        if let Some(key) = cursor_location.0 {
            self.0.cmp(&cursor_location.0)
        } else {
            Ordering::Greater
        }
    }
}
