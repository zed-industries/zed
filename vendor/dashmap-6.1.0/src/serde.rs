use crate::{mapref, setref, DashMap, DashSet};
use core::fmt;
use core::hash::{BuildHasher, Hash};
use core::marker::PhantomData;
use serde::de::{Deserialize, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde::Deserializer;

pub struct DashMapVisitor<K, V, S> {
    marker: PhantomData<fn() -> DashMap<K, V, S>>,
}

impl<K, V, S> DashMapVisitor<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Clone,
{
    fn new() -> Self {
        DashMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V, S> Visitor<'de> for DashMapVisitor<K, V, S>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    S: BuildHasher + Clone + Default,
{
    type Value = DashMap<K, V, S>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a DashMap")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let map =
            DashMap::with_capacity_and_hasher(access.size_hint().unwrap_or(0), Default::default());

        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'de, K, V, S> Deserialize<'de> for DashMap<K, V, S>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    S: BuildHasher + Clone + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DashMapVisitor::<K, V, S>::new())
    }
}

impl<K, V, H> Serialize for DashMap<K, V, H>
where
    K: Serialize + Eq + Hash,
    V: Serialize,
    H: BuildHasher + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;

        for ref_multi in self.iter() {
            map.serialize_entry(ref_multi.key(), ref_multi.value())?;
        }

        map.end()
    }
}

pub struct DashSetVisitor<K, S> {
    marker: PhantomData<fn() -> DashSet<K, S>>,
}

impl<K, S> DashSetVisitor<K, S>
where
    K: Eq + Hash,
    S: BuildHasher + Clone,
{
    fn new() -> Self {
        DashSetVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K, S> Visitor<'de> for DashSetVisitor<K, S>
where
    K: Deserialize<'de> + Eq + Hash,
    S: BuildHasher + Clone + Default,
{
    type Value = DashSet<K, S>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a DashSet")
    }

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let map =
            DashSet::with_capacity_and_hasher(access.size_hint().unwrap_or(0), Default::default());

        while let Some(key) = access.next_element()? {
            map.insert(key);
        }

        Ok(map)
    }
}

impl<'de, K, S> Deserialize<'de> for DashSet<K, S>
where
    K: Deserialize<'de> + Eq + Hash,
    S: BuildHasher + Clone + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(DashSetVisitor::<K, S>::new())
    }
}

impl<K, H> Serialize for DashSet<K, H>
where
    K: Serialize + Eq + Hash,
    H: BuildHasher + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        for ref_multi in self.iter() {
            seq.serialize_element(ref_multi.key())?;
        }

        seq.end()
    }
}

macro_rules! serialize_impl {
    () => {
        fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
        where
            Ser: serde::Serializer,
        {
            std::ops::Deref::deref(self).serialize(serializer)
        }
    };
}

// Map
impl<'a, K: Eq + Hash, V: Serialize> Serialize for mapref::multiple::RefMulti<'a, K, V> {
    serialize_impl! {}
}

impl<'a, K: Eq + Hash, V: Serialize> Serialize for mapref::multiple::RefMutMulti<'a, K, V> {
    serialize_impl! {}
}

impl<'a, K: Eq + Hash, V: Serialize> Serialize for mapref::one::Ref<'a, K, V> {
    serialize_impl! {}
}

impl<'a, K: Eq + Hash, V: Serialize> Serialize for mapref::one::RefMut<'a, K, V> {
    serialize_impl! {}
}

impl<'a, K: Eq + Hash, V, T: Serialize> Serialize for mapref::one::MappedRef<'a, K, V, T> {
    serialize_impl! {}
}

impl<'a, K: Eq + Hash, V, T: Serialize> Serialize for mapref::one::MappedRefMut<'a, K, V, T> {
    serialize_impl! {}
}

// Set
impl<'a, V: Hash + Eq + Serialize> Serialize for setref::multiple::RefMulti<'a, V> {
    serialize_impl! {}
}

impl<'a, V: Hash + Eq + Serialize> Serialize for setref::one::Ref<'a, V> {
    serialize_impl! {}
}
