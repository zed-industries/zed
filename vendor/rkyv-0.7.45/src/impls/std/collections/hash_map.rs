use crate::{
    collections::hash_map::{ArchivedHashMap, HashMapResolver},
    ser::{ScratchSpace, Serializer},
    Archive, Deserialize, Fallible, Serialize,
};
use core::{
    borrow::Borrow,
    hash::{BuildHasher, Hash},
};
use std::collections::HashMap;

impl<K: Archive + Hash + Eq, V: Archive, S> Archive for HashMap<K, V, S>
where
    K::Archived: Hash + Eq,
{
    type Archived = ArchivedHashMap<K::Archived, V::Archived>;
    type Resolver = HashMapResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedHashMap::resolve_from_len(self.len(), pos, resolver, out);
    }
}

impl<K, V, S, RandomState> Serialize<S> for HashMap<K, V, RandomState>
where
    K: Serialize<S> + Hash + Eq,
    K::Archived: Hash + Eq,
    V: Serialize<S>,
    S: Serializer + ScratchSpace + ?Sized,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        unsafe { ArchivedHashMap::serialize_from_iter(self.iter(), serializer) }
    }
}

impl<K: Archive + Hash + Eq, V: Archive, D: Fallible + ?Sized, S: Default + BuildHasher>
    Deserialize<HashMap<K, V, S>, D> for ArchivedHashMap<K::Archived, V::Archived>
where
    K::Archived: Deserialize<K, D> + Hash + Eq,
    V::Archived: Deserialize<V, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<HashMap<K, V, S>, D::Error> {
        let mut result = HashMap::with_capacity_and_hasher(self.len(), S::default());
        for (k, v) in self.iter() {
            result.insert(k.deserialize(deserializer)?, v.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

impl<K: Hash + Eq + Borrow<AK>, V, AK: Hash + Eq, AV: PartialEq<V>, S: BuildHasher>
    PartialEq<HashMap<K, V, S>> for ArchivedHashMap<AK, AV>
{
    #[inline]
    fn eq(&self, other: &HashMap<K, V, S>) -> bool {
        if self.len() != other.len() {
            false
        } else {
            self.iter()
                .all(|(key, value)| other.get(key).map_or(false, |v| value.eq(v)))
        }
    }
}

impl<K: Hash + Eq + Borrow<AK>, V, AK: Hash + Eq, AV: PartialEq<V>>
    PartialEq<ArchivedHashMap<AK, AV>> for HashMap<K, V>
{
    #[inline]
    fn eq(&self, other: &ArchivedHashMap<AK, AV>) -> bool {
        other.eq(self)
    }
}
