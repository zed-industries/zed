//! This module implements helper types and traits for `serde` serialization and deserialization.

#![deny(unsafe_code)]

use std::fmt;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;

use serde::Deserializer;
use serde::de::{Deserialize, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use super::{Guard, HashCache, HashIndex, HashMap, HashSet, TreeIndex};

/// Helper type to allow `serde` to access [`HashMap`] entries.
pub struct HashMapVisitor<K: Eq + Hash, V, H: BuildHasher> {
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> HashMap<K, V, H>>,
}

/// The maximum initial capacity for containers.
const MAX_CAPACITY: usize = 1 << 24;

impl<K, V, H> HashMapVisitor<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    fn new() -> Self {
        HashMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'d, K, V, H> Visitor<'d> for HashMapVisitor<K, V, H>
where
    K: Deserialize<'d> + Eq + Hash,
    V: Deserialize<'d>,
    H: BuildHasher + Default,
{
    type Value = HashMap<K, V, H>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("HashMap")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'d>,
    {
        let hashmap = HashMap::with_capacity_and_hasher(
            access.size_hint().unwrap_or(0).min(MAX_CAPACITY),
            H::default(),
        );
        while let Some((key, val)) = access.next_entry()? {
            hashmap.upsert_sync(key, val);
        }
        Ok(hashmap)
    }
}

impl<'d, K, V, H> Deserialize<'d> for HashMap<K, V, H>
where
    K: Deserialize<'d> + Eq + Hash,
    V: Deserialize<'d>,
    H: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_map(HashMapVisitor::<K, V, H>::new())
    }
}

impl<K, V, H> Serialize for HashMap<K, V, H>
where
    K: Eq + Hash + Serialize,
    V: Serialize,
    H: BuildHasher,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        let mut error = None;
        self.iter_sync(|k, v| {
            if error.is_none() {
                if let Err(e) = map.serialize_entry(k, v) {
                    error.replace(e);
                }
            }
            true
        });
        if let Some(e) = error {
            return Err(e);
        }
        map.end()
    }
}

/// Helper type to allow `serde` to access [`HashSet`] entries.
pub struct HashSetVisitor<K: Eq + Hash, H: BuildHasher> {
    marker: PhantomData<fn() -> HashSet<K, H>>,
}

impl<K, H> HashSetVisitor<K, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    fn new() -> Self {
        HashSetVisitor {
            marker: PhantomData,
        }
    }
}

impl<'d, K, H> Visitor<'d> for HashSetVisitor<K, H>
where
    K: Deserialize<'d> + Eq + Hash,
    H: BuildHasher + Default,
{
    type Value = HashSet<K, H>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("HashSet")
    }

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'d>,
    {
        let hashset = HashSet::with_capacity_and_hasher(
            access.size_hint().unwrap_or(0).min(MAX_CAPACITY),
            H::default(),
        );
        while let Some(key) = access.next_element()? {
            let _result = hashset.insert_sync(key);
        }
        Ok(hashset)
    }
}

impl<'d, K, H> Deserialize<'d> for HashSet<K, H>
where
    K: Deserialize<'d> + Eq + Hash,
    H: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_seq(HashSetVisitor::<K, H>::new())
    }
}

impl<K, H> Serialize for HashSet<K, H>
where
    K: Eq + Hash + Serialize,
    H: BuildHasher,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        let mut error = None;
        self.iter_sync(|k| {
            if error.is_none() {
                if let Err(e) = seq.serialize_element(k) {
                    error.replace(e);
                }
            }
            true
        });
        if let Some(e) = error {
            return Err(e);
        }
        seq.end()
    }
}

/// Helper type to allow `serde` to access [`HashIndex`] entries.
pub struct HashIndexVisitor<K: Eq + Hash, V, H: BuildHasher> {
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> HashIndex<K, V, H>>,
}

impl<K, V, H> HashIndexVisitor<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    fn new() -> Self {
        HashIndexVisitor {
            marker: PhantomData,
        }
    }
}

impl<'d, K, V, H> Visitor<'d> for HashIndexVisitor<K, V, H>
where
    K: Deserialize<'d> + Eq + Hash,
    V: Deserialize<'d>,
    H: BuildHasher + Default,
{
    type Value = HashIndex<K, V, H>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("HashIndex")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'d>,
    {
        let hashindex = HashIndex::with_capacity_and_hasher(
            access.size_hint().unwrap_or(0).min(MAX_CAPACITY),
            H::default(),
        );
        while let Some((key, val)) = access.next_entry()? {
            let _result = hashindex.insert_sync(key, val);
        }
        Ok(hashindex)
    }
}

impl<'d, K, V, H> Deserialize<'d> for HashIndex<K, V, H>
where
    K: Deserialize<'d> + Eq + Hash,
    V: Deserialize<'d>,
    H: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_map(HashIndexVisitor::<K, V, H>::new())
    }
}

impl<K, V, H> Serialize for HashIndex<K, V, H>
where
    K: Eq + Hash + Serialize,
    V: Serialize,
    H: BuildHasher,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        let mut error = None;
        self.iter(&Guard::new()).any(|(k, v)| {
            if let Err(e) = map.serialize_entry(k, v) {
                error.replace(e);
                true
            } else {
                false
            }
        });
        if let Some(e) = error {
            return Err(e);
        }
        map.end()
    }
}

/// Helper type to allow `serde` to access [`HashCache`] entries.
pub struct HashCacheVisitor<K: Eq + Hash, V, H: BuildHasher> {
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> HashCache<K, V, H>>,
}

impl<K, V, H> HashCacheVisitor<K, V, H>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    fn new() -> Self {
        HashCacheVisitor {
            marker: PhantomData,
        }
    }
}

impl<'d, K, V, H> Visitor<'d> for HashCacheVisitor<K, V, H>
where
    K: Deserialize<'d> + Eq + Hash,
    V: Deserialize<'d>,
    H: BuildHasher + Default,
{
    type Value = HashCache<K, V, H>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("HashCache")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'d>,
    {
        let capacity = access.size_hint().unwrap_or(0).min(MAX_CAPACITY);
        let hashcache = HashCache::with_capacity_and_hasher(0, capacity, H::default());
        while let Some((key, val)) = access.next_entry()? {
            let _result = hashcache.put_sync(key, val);
        }
        Ok(hashcache)
    }
}

impl<'d, K, V, H> Deserialize<'d> for HashCache<K, V, H>
where
    K: Deserialize<'d> + Eq + Hash,
    V: Deserialize<'d>,
    H: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_map(HashCacheVisitor::<K, V, H>::new())
    }
}

impl<K, V, H> Serialize for HashCache<K, V, H>
where
    K: Eq + Hash + Serialize,
    V: Serialize,
    H: BuildHasher,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let capacity_range = self.capacity_range();
        let mut map = serializer.serialize_map(Some(*capacity_range.end()))?;
        let mut error = None;
        self.iter_sync(|k, v| {
            if error.is_none() {
                if let Err(e) = map.serialize_entry(k, v) {
                    error.replace(e);
                }
            }
            true
        });
        if let Some(e) = error {
            return Err(e);
        }
        map.end()
    }
}

/// Helper type to allow `serde` to access [`TreeIndex`] entries.
pub struct TreeIndexVisitor<K: 'static + Clone + Ord, V: 'static> {
    #[allow(clippy::type_complexity)]
    marker: PhantomData<fn() -> TreeIndex<K, V>>,
}

impl<K, V> TreeIndexVisitor<K, V>
where
    K: Clone + Ord,
{
    fn new() -> Self {
        TreeIndexVisitor {
            marker: PhantomData,
        }
    }
}

impl<'d, K, V> Visitor<'d> for TreeIndexVisitor<K, V>
where
    K: 'static + Clone + Deserialize<'d> + Ord,
    V: 'static + Deserialize<'d>,
{
    type Value = TreeIndex<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("TreeIndex")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'d>,
    {
        let treeindex = TreeIndex::default();
        while let Some((key, val)) = access.next_entry()? {
            let _result = treeindex.insert_sync(key, val);
        }
        Ok(treeindex)
    }
}

impl<'d, K, V> Deserialize<'d> for TreeIndex<K, V>
where
    K: 'static + Clone + Deserialize<'d> + Ord,
    V: 'static + Deserialize<'d>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_map(TreeIndexVisitor::<K, V>::new())
    }
}

impl<K, V> Serialize for TreeIndex<K, V>
where
    K: 'static + Clone + Ord + Serialize,
    V: 'static + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        let mut error = None;
        self.iter(&Guard::new()).any(|(k, v)| {
            if let Err(e) = map.serialize_entry(k, v) {
                error.replace(e);
                true
            } else {
                false
            }
        });
        if let Some(e) = error {
            return Err(e);
        }
        map.end()
    }
}
