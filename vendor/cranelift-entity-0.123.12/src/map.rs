//! Densely numbered entity references as mapping keys.

use crate::EntityRef;
use crate::iter::{Iter, IterMut};
use crate::keys::Keys;
use alloc::vec::Vec;
use core::cmp::min;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::slice;
#[cfg(feature = "enable-serde")]
use serde::{
    Deserialize, Serialize,
    de::{Deserializer, SeqAccess, Visitor},
    ser::{SerializeSeq, Serializer},
};

/// A mapping `K -> V` for densely indexed entity references.
///
/// The `SecondaryMap` data structure uses the dense index space to implement a map with a vector.
/// Unlike `PrimaryMap`, an `SecondaryMap` can't be used to allocate entity references. It is used
/// to associate secondary information with entities.
///
/// The map does not track if an entry for a key has been inserted or not. Instead it behaves as if
/// all keys have a default entry from the beginning.
#[derive(Clone, Hash)]
pub struct SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone,
{
    elems: Vec<V>,
    default: V,
    unused: PhantomData<K>,
}

/// Shared `SecondaryMap` implementation for all value types.
impl<K, V> SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone,
{
    /// Create a new empty map.
    pub fn new() -> Self
    where
        V: Default,
    {
        Self {
            elems: Vec::new(),
            default: Default::default(),
            unused: PhantomData,
        }
    }

    /// Create a new, empty map with the specified capacity.
    ///
    /// The map will be able to hold exactly `capacity` elements without reallocating.
    pub fn with_capacity(capacity: usize) -> Self
    where
        V: Default,
    {
        Self {
            elems: Vec::with_capacity(capacity),
            default: Default::default(),
            unused: PhantomData,
        }
    }

    /// Create a new empty map with a specified default value.
    ///
    /// This constructor does not require V to implement Default.
    pub fn with_default(default: V) -> Self {
        Self {
            elems: Vec::new(),
            default,
            unused: PhantomData,
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.elems.capacity()
    }

    /// Get the element at `k` if it exists.
    #[inline(always)]
    pub fn get(&self, k: K) -> Option<&V> {
        self.elems.get(k.index())
    }

    /// Is this map completely empty?
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    /// Remove all entries from this map.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.elems.clear()
    }

    /// Iterate over all the keys and values in this map.
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self.elems.iter())
    }

    /// Iterate over all the keys and values in this map, mutable edition.
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self.elems.iter_mut())
    }

    /// Iterate over all the keys in this map.
    pub fn keys(&self) -> Keys<K> {
        Keys::with_len(self.elems.len())
    }

    /// Iterate over all the values in this map.
    pub fn values(&self) -> slice::Iter<'_, V> {
        self.elems.iter()
    }

    /// Iterate over all the values in this map, mutable edition.
    pub fn values_mut(&mut self) -> slice::IterMut<'_, V> {
        self.elems.iter_mut()
    }

    /// Resize the map to have `n` entries by adding default entries as needed.
    pub fn resize(&mut self, n: usize) {
        self.elems.resize(n, self.default.clone());
    }

    /// Slow path for `index_mut` which resizes the vector.
    #[cold]
    fn resize_for_index_mut(&mut self, i: usize) -> &mut V {
        self.elems.resize(i + 1, self.default.clone());
        &mut self.elems[i]
    }
}

impl<K, V> Default for SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone + Default,
{
    fn default() -> SecondaryMap<K, V> {
        SecondaryMap::new()
    }
}

impl<K, V> FromIterator<(K, V)> for SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        let cap = max.unwrap_or_else(|| 2 * min);
        let mut map = Self::with_capacity(cap);
        for (k, v) in iter {
            map[k] = v;
        }
        map
    }
}

/// Immutable indexing into an `SecondaryMap`.
///
/// All keys are permitted. Untouched entries have the default value.
impl<K, V> Index<K> for SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone,
{
    type Output = V;

    #[inline(always)]
    fn index(&self, k: K) -> &V {
        self.elems.get(k.index()).unwrap_or(&self.default)
    }
}

/// Mutable indexing into an `SecondaryMap`.
///
/// The map grows as needed to accommodate new keys.
impl<K, V> IndexMut<K> for SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone,
{
    #[inline(always)]
    fn index_mut(&mut self, k: K) -> &mut V {
        let i = k.index();
        if i >= self.elems.len() {
            return self.resize_for_index_mut(i);
        }
        &mut self.elems[i]
    }
}

impl<K, V> PartialEq for SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let min_size = min(self.elems.len(), other.elems.len());
        self.default == other.default
            && self.elems[..min_size] == other.elems[..min_size]
            && self.elems[min_size..].iter().all(|e| *e == self.default)
            && other.elems[min_size..].iter().all(|e| *e == other.default)
    }
}

impl<K, V> Eq for SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone + PartialEq + Eq,
{
}

#[cfg(feature = "enable-serde")]
impl<K, V> Serialize for SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone + PartialEq + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO: bincode encodes option as "byte for Some/None" and then optionally the content
        // TODO: we can actually optimize it by encoding manually bitmask, then elements
        let mut elems_cnt = self.elems.len();
        while elems_cnt > 0 && self.elems[elems_cnt - 1] == self.default {
            elems_cnt -= 1;
        }
        let mut seq = serializer.serialize_seq(Some(1 + elems_cnt))?;
        seq.serialize_element(&Some(self.default.clone()))?;
        for e in self.elems.iter().take(elems_cnt) {
            let some_e = Some(e);
            seq.serialize_element(if *e == self.default { &None } else { &some_e })?;
        }
        seq.end()
    }
}

#[cfg(feature = "enable-serde")]
impl<'de, K, V> Deserialize<'de> for SecondaryMap<K, V>
where
    K: EntityRef,
    V: Clone + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use alloc::fmt;
        struct SecondaryMapVisitor<K, V> {
            unused: PhantomData<fn(K) -> V>,
        }

        impl<'de, K, V> Visitor<'de> for SecondaryMapVisitor<K, V>
        where
            K: EntityRef,
            V: Clone + Deserialize<'de>,
        {
            type Value = SecondaryMap<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SecondaryMap")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                match seq.next_element()? {
                    Some(Some(default_val)) => {
                        let default_val: V = default_val; // compiler can't infer the type
                        let mut m = SecondaryMap::with_default(default_val.clone());
                        let mut idx = 0;
                        while let Some(val) = seq.next_element()? {
                            let val: Option<_> = val; // compiler can't infer the type
                            m[K::new(idx)] = val.unwrap_or_else(|| default_val.clone());
                            idx += 1;
                        }
                        Ok(m)
                    }
                    _ => Err(serde::de::Error::custom("Default value required")),
                }
            }
        }

        deserializer.deserialize_seq(SecondaryMapVisitor {
            unused: PhantomData {},
        })
    }
}

impl<K: EntityRef + fmt::Debug, V: fmt::Debug + Clone> fmt::Debug for SecondaryMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecondaryMap")
            .field("elems", &self.elems)
            .field("default", &self.default)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `EntityRef` impl for testing.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct E(u32);

    impl EntityRef for E {
        fn new(i: usize) -> Self {
            E(i as u32)
        }
        fn index(self) -> usize {
            self.0 as usize
        }
    }

    #[test]
    fn basic() {
        let r0 = E(0);
        let r1 = E(1);
        let r2 = E(2);
        let mut m = SecondaryMap::new();

        let v: Vec<E> = m.keys().collect();
        assert_eq!(v, []);

        m[r2] = 3;
        m[r1] = 5;

        assert_eq!(m[r1], 5);
        assert_eq!(m[r2], 3);

        let v: Vec<E> = m.keys().collect();
        assert_eq!(v, [r0, r1, r2]);

        let shared = &m;
        assert_eq!(shared[r0], 0);
        assert_eq!(shared[r1], 5);
        assert_eq!(shared[r2], 3);
    }
}
