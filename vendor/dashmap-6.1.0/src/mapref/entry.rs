use super::one::RefMut;
use crate::lock::RwLockWriteGuard;
use crate::util::SharedValue;
use crate::HashMap;
use core::hash::Hash;
use core::mem;

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: Eq + Hash, V> Entry<'a, K, V> {
    /// Apply a function to the stored value if it exists.
    pub fn and_modify(self, f: impl FnOnce(&mut V)) -> Self {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());

                Entry::Occupied(entry)
            }

            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }

    /// Get the key of the entry.
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    /// Into the key of the entry.
    pub fn into_key(self) -> K {
        match self {
            Entry::Occupied(entry) => entry.into_key(),
            Entry::Vacant(entry) => entry.into_key(),
        }
    }

    /// Return a mutable reference to the element if it exists,
    /// otherwise insert the default and return a mutable reference to that.
    pub fn or_default(self) -> RefMut<'a, K, V>
    where
        V: Default,
    {
        match self {
            Entry::Occupied(entry) => entry.into_ref(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }

    /// Return a mutable reference to the element if it exists,
    /// otherwise a provided value and return a mutable reference to that.
    pub fn or_insert(self, value: V) -> RefMut<'a, K, V> {
        match self {
            Entry::Occupied(entry) => entry.into_ref(),
            Entry::Vacant(entry) => entry.insert(value),
        }
    }

    /// Return a mutable reference to the element if it exists,
    /// otherwise insert the result of a provided function and return a mutable reference to that.
    pub fn or_insert_with(self, value: impl FnOnce() -> V) -> RefMut<'a, K, V> {
        match self {
            Entry::Occupied(entry) => entry.into_ref(),
            Entry::Vacant(entry) => entry.insert(value()),
        }
    }

    pub fn or_try_insert_with<E>(
        self,
        value: impl FnOnce() -> Result<V, E>,
    ) -> Result<RefMut<'a, K, V>, E> {
        match self {
            Entry::Occupied(entry) => Ok(entry.into_ref()),
            Entry::Vacant(entry) => Ok(entry.insert(value()?)),
        }
    }

    /// Sets the value of the entry, and returns a reference to the inserted value.
    pub fn insert(self, value: V) -> RefMut<'a, K, V> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry.into_ref()
            }
            Entry::Vacant(entry) => entry.insert(value),
        }
    }

    /// Sets the value of the entry, and returns an OccupiedEntry.
    ///
    /// If you are not interested in the occupied entry,
    /// consider [`insert`] as it doesn't need to clone the key.
    ///
    /// [`insert`]: Entry::insert
    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V>
    where
        K: Clone,
    {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }
}

pub struct VacantEntry<'a, K, V> {
    shard: RwLockWriteGuard<'a, HashMap<K, V>>,
    key: K,
    hash: u64,
    slot: hashbrown::raw::InsertSlot,
}

unsafe impl<'a, K: Eq + Hash + Sync, V: Sync> Send for VacantEntry<'a, K, V> {}
unsafe impl<'a, K: Eq + Hash + Sync, V: Sync> Sync for VacantEntry<'a, K, V> {}

impl<'a, K: Eq + Hash, V> VacantEntry<'a, K, V> {
    pub(crate) unsafe fn new(
        shard: RwLockWriteGuard<'a, HashMap<K, V>>,
        key: K,
        hash: u64,
        slot: hashbrown::raw::InsertSlot,
    ) -> Self {
        Self {
            shard,
            key,
            hash,
            slot,
        }
    }

    pub fn insert(mut self, value: V) -> RefMut<'a, K, V> {
        unsafe {
            let occupied = self.shard.insert_in_slot(
                self.hash,
                self.slot,
                (self.key, SharedValue::new(value)),
            );

            let (k, v) = occupied.as_ref();

            RefMut::new(self.shard, k, v.as_ptr())
        }
    }

    /// Sets the value of the entry with the VacantEntry’s key, and returns an OccupiedEntry.
    pub fn insert_entry(mut self, value: V) -> OccupiedEntry<'a, K, V>
    where
        K: Clone,
    {
        unsafe {
            let bucket = self.shard.insert_in_slot(
                self.hash,
                self.slot,
                (self.key.clone(), SharedValue::new(value)),
            );

            OccupiedEntry::new(self.shard, self.key, bucket)
        }
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn key(&self) -> &K {
        &self.key
    }
}

pub struct OccupiedEntry<'a, K, V> {
    shard: RwLockWriteGuard<'a, HashMap<K, V>>,
    bucket: hashbrown::raw::Bucket<(K, SharedValue<V>)>,
    key: K,
}

unsafe impl<'a, K: Eq + Hash + Sync, V: Sync> Send for OccupiedEntry<'a, K, V> {}
unsafe impl<'a, K: Eq + Hash + Sync, V: Sync> Sync for OccupiedEntry<'a, K, V> {}

impl<'a, K: Eq + Hash, V> OccupiedEntry<'a, K, V> {
    pub(crate) unsafe fn new(
        shard: RwLockWriteGuard<'a, HashMap<K, V>>,
        key: K,
        bucket: hashbrown::raw::Bucket<(K, SharedValue<V>)>,
    ) -> Self {
        Self { shard, bucket, key }
    }

    pub fn get(&self) -> &V {
        unsafe { self.bucket.as_ref().1.get() }
    }

    pub fn get_mut(&mut self) -> &mut V {
        unsafe { self.bucket.as_mut().1.get_mut() }
    }

    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }

    pub fn into_ref(self) -> RefMut<'a, K, V> {
        unsafe {
            let (k, v) = self.bucket.as_ref();
            RefMut::new(self.shard, k, v.as_ptr())
        }
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn key(&self) -> &K {
        unsafe { &self.bucket.as_ref().0 }
    }

    pub fn remove(mut self) -> V {
        let ((_k, v), _) = unsafe { self.shard.remove(self.bucket) };
        v.into_inner()
    }

    pub fn remove_entry(mut self) -> (K, V) {
        let ((k, v), _) = unsafe { self.shard.remove(self.bucket) };
        (k, v.into_inner())
    }

    pub fn replace_entry(self, value: V) -> (K, V) {
        let (k, v) = mem::replace(
            unsafe { self.bucket.as_mut() },
            (self.key, SharedValue::new(value)),
        );
        (k, v.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use crate::DashMap;

    use super::*;

    #[test]
    fn test_insert_entry_into_vacant() {
        let map: DashMap<u32, u32> = DashMap::new();

        let entry = map.entry(1);

        assert!(matches!(entry, Entry::Vacant(_)));

        let entry = entry.insert_entry(2);

        assert_eq!(*entry.get(), 2);

        drop(entry);

        assert_eq!(*map.get(&1).unwrap(), 2);
    }

    #[test]
    fn test_insert_entry_into_occupied() {
        let map: DashMap<u32, u32> = DashMap::new();

        map.insert(1, 1000);

        let entry = map.entry(1);

        assert!(matches!(&entry, Entry::Occupied(entry) if *entry.get() == 1000));

        let entry = entry.insert_entry(2);

        assert_eq!(*entry.get(), 2);

        drop(entry);

        assert_eq!(*map.get(&1).unwrap(), 2);
    }
}
