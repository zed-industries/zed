/// A collection that provides a map interface but is backed by vectors.
///
/// This is suitable for small key-value stores where the item count is not
/// large enough to overcome the overhead of a more complex algorithm.
///
/// If this meets your use cases, then [`VecMap`] should be a drop-in
/// replacement for [`std::collections::HashMap`] or [`crate::HashMap`]. Note
/// that we are adding APIs on an as-needed basis. If the API you need is not
/// present yet, please add it!
///
/// Because it uses vectors as a backing store, the map also iterates over items
/// in insertion order, like [`crate::IndexMap`].
///
/// This struct uses a struct-of-arrays (SoA) representation which tends to be
/// more cache efficient and promotes autovectorization when using simple key or
/// value types.
#[derive(Default)]
pub struct VecMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> VecMap<K, V> {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            iter: self.keys.iter().zip(self.values.iter()),
        }
    }
}

impl<K: Eq, V> VecMap<K, V> {
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.keys.iter().position(|k| k == &key) {
            Some(index) => Entry::Occupied(OccupiedEntry {
                key: &self.keys[index],
                value: &mut self.values[index],
            }),
            None => Entry::Vacant(VacantEntry { map: self, key }),
        }
    }

    /// Like [`Self::entry`] but takes its key by reference instead of by value.
    ///
    /// This can be helpful if you have a key where cloning is expensive, as we
    /// can avoid cloning the key until a value is inserted under that entry.
    pub fn entry_ref<'a, 'k>(&'a mut self, key: &'k K) -> EntryRef<'k, 'a, K, V> {
        match self.keys.iter().position(|k| k == key) {
            Some(index) => EntryRef::Occupied(OccupiedEntry {
                key: &self.keys[index],
                value: &mut self.values[index],
            }),
            None => EntryRef::Vacant(VacantEntryRef { map: self, key }),
        }
    }
}

pub struct Iter<'a, K, V> {
    iter: std::iter::Zip<std::slice::Iter<'a, K>, std::slice::Iter<'a, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V> {
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key,
            Entry::Vacant(entry) => &entry.key,
        }
    }

    pub fn or_insert_with_key<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.value,
            Entry::Vacant(entry) => {
                entry.map.values.push(default(&entry.key));
                entry.map.keys.push(entry.key);
                match entry.map.values.last_mut() {
                    Some(value) => value,
                    None => unreachable!("vec empty after pushing to it"),
                }
            }
        }
    }

    pub fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        self.or_insert_with_key(|_| default())
    }

    pub fn or_insert(self, value: V) -> &'a mut V {
        self.or_insert_with_key(|_| value)
    }

    pub fn or_insert_default(self) -> &'a mut V
    where
        V: Default,
    {
        self.or_insert_with_key(|_| Default::default())
    }
}

pub struct OccupiedEntry<'a, K, V> {
    key: &'a K,
    value: &'a mut V,
}

pub struct VacantEntry<'a, K, V> {
    map: &'a mut VecMap<K, V>,
    key: K,
}

pub enum EntryRef<'key, 'map, K, V> {
    Occupied(OccupiedEntry<'map, K, V>),
    Vacant(VacantEntryRef<'key, 'map, K, V>),
}

impl<'key, 'map, K, V> EntryRef<'key, 'map, K, V> {
    pub fn key(&self) -> &K {
        match self {
            EntryRef::Occupied(entry) => entry.key,
            EntryRef::Vacant(entry) => entry.key,
        }
    }
}

impl<'key, 'map, K, V> EntryRef<'key, 'map, K, V>
where
    K: Clone,
{
    pub fn or_insert_with_key<F>(self, default: F) -> &'map mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            EntryRef::Occupied(entry) => entry.value,
            EntryRef::Vacant(entry) => {
                entry.map.values.push(default(entry.key));
                entry.map.keys.push(entry.key.clone());
                match entry.map.values.last_mut() {
                    Some(value) => value,
                    None => unreachable!("vec empty after pushing to it"),
                }
            }
        }
    }

    pub fn or_insert_with<F>(self, default: F) -> &'map mut V
    where
        F: FnOnce() -> V,
    {
        self.or_insert_with_key(|_| default())
    }

    pub fn or_insert(self, value: V) -> &'map mut V {
        self.or_insert_with_key(|_| value)
    }

    pub fn or_insert_default(self) -> &'map mut V
    where
        V: Default,
    {
        self.or_insert_with_key(|_| Default::default())
    }
}

pub struct VacantEntryRef<'key, 'map, K, V> {
    map: &'map mut VecMap<K, V>,
    key: &'key K,
}
