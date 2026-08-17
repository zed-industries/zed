//! Densely numbered entity references as mapping keys.
use crate::EntityRef;
use crate::boxed_slice::BoxedSlice;
use crate::iter::{IntoIter, Iter, IterMut};
use crate::keys::Keys;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::slice;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// A primary mapping `K -> V` allocating dense entity references.
///
/// The `PrimaryMap` data structure uses the dense index space to implement a map with a vector.
///
/// A primary map contains the main definition of an entity, and it can be used to allocate new
/// entity references with the `push` method.
///
/// There should only be a single `PrimaryMap` instance for a given `EntityRef` type, otherwise
/// conflicting references will be created. Using unknown keys for indexing will cause a panic.
///
/// Note that `PrimaryMap` doesn't implement `Deref` or `DerefMut`, which would allow
/// `&PrimaryMap<K, V>` to convert to `&[V]`. One of the main advantages of `PrimaryMap` is
/// that it only allows indexing with the distinct `EntityRef` key type, so converting to a
/// plain slice would make it easier to use incorrectly. To make a slice of a `PrimaryMap`, use
/// `into_boxed_slice`.
#[derive(Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct PrimaryMap<K, V>
where
    K: EntityRef,
{
    elems: Vec<V>,
    unused: PhantomData<K>,
}

impl<K, V> PrimaryMap<K, V>
where
    K: EntityRef,
{
    /// Create a new empty map.
    pub fn new() -> Self {
        Self {
            elems: Vec::new(),
            unused: PhantomData,
        }
    }

    /// Create a new empty map with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elems: Vec::with_capacity(capacity),
            unused: PhantomData,
        }
    }

    /// Check if `k` is a valid key in the map.
    pub fn is_valid(&self, k: K) -> bool {
        k.index() < self.elems.len()
    }

    /// Get the element at `k` if it exists.
    pub fn get(&self, k: K) -> Option<&V> {
        self.elems.get(k.index())
    }

    /// Get the element at `k` if it exists, mutable version.
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        self.elems.get_mut(k.index())
    }

    /// Is this map completely empty?
    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    /// Get the total number of entity references created.
    pub fn len(&self) -> usize {
        self.elems.len()
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

    /// Iterate over all the keys and values in this map.
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self.elems.iter())
    }

    /// Iterate over all the keys and values in this map, mutable edition.
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self.elems.iter_mut())
    }

    /// Remove all entries from this map.
    pub fn clear(&mut self) {
        self.elems.clear()
    }

    /// Get the key that will be assigned to the next pushed value.
    pub fn next_key(&self) -> K {
        K::new(self.elems.len())
    }

    /// Append `v` to the mapping, assigning a new key which is returned.
    pub fn push(&mut self, v: V) -> K {
        let k = self.next_key();
        self.elems.push(v);
        k
    }

    /// Returns the last element that was inserted in the map.
    pub fn last(&self) -> Option<(K, &V)> {
        let len = self.elems.len();
        let last = self.elems.last()?;
        Some((K::new(len - 1), last))
    }

    /// Returns the last element that was inserted in the map.
    pub fn last_mut(&mut self) -> Option<(K, &mut V)> {
        let len = self.elems.len();
        let last = self.elems.last_mut()?;
        Some((K::new(len - 1), last))
    }

    /// Reserves capacity for at least `additional` more elements to be inserted.
    pub fn reserve(&mut self, additional: usize) {
        self.elems.reserve(additional)
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted.
    pub fn reserve_exact(&mut self, additional: usize) {
        self.elems.reserve_exact(additional)
    }

    /// Shrinks the capacity of the `PrimaryMap` as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.elems.shrink_to_fit()
    }

    /// Consumes this `PrimaryMap` and produces a `BoxedSlice`.
    pub fn into_boxed_slice(self) -> BoxedSlice<K, V> {
        unsafe { BoxedSlice::<K, V>::from_raw(Box::<[V]>::into_raw(self.elems.into_boxed_slice())) }
    }

    /// Returns mutable references to many elements at once.
    ///
    /// Returns an error if an element does not exist, or if the same key was passed more than
    /// once.
    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [K; N],
    ) -> Result<[&mut V; N], slice::GetDisjointMutError> {
        self.elems.get_disjoint_mut(indices.map(|k| k.index()))
    }

    /// Performs a binary search on the values with a key extraction function.
    ///
    /// Assumes that the values are sorted by the key extracted by the function.
    ///
    /// If the value is found then `Ok(K)` is returned, containing the entity key
    /// of the matching value.
    ///
    /// If there are multiple matches, then any one of the matches could be returned.
    ///
    /// If the value is not found then Err(K) is returned, containing the entity key
    /// where a matching element could be inserted while maintaining sorted order.
    pub fn binary_search_values_by_key<'a, B, F>(&'a self, b: &B, f: F) -> Result<K, K>
    where
        F: FnMut(&'a V) -> B,
        B: Ord,
    {
        self.elems
            .binary_search_by_key(b, f)
            .map(|i| K::new(i))
            .map_err(|i| K::new(i))
    }

    /// Analog of `get_raw` except that a raw pointer is returned rather than a
    /// mutable reference.
    ///
    /// The default accessors of items in [`PrimaryMap`] will invalidate all
    /// previous borrows obtained from the map according to miri. This function
    /// can be used to acquire a pointer and then subsequently acquire a second
    /// pointer later on without invalidating the first one. In other words
    /// this is only here to help borrow two elements simultaneously with miri.
    pub fn get_raw_mut(&mut self, k: K) -> Option<*mut V> {
        if k.index() < self.elems.len() {
            // SAFETY: the `add` function requires that the index is in-bounds
            // with respect to the allocation which is satisfied here due to
            // the bounds-check above.
            unsafe { Some(self.elems.as_mut_ptr().add(k.index())) }
        } else {
            None
        }
    }
}

impl<K, V> Default for PrimaryMap<K, V>
where
    K: EntityRef,
{
    fn default() -> PrimaryMap<K, V> {
        PrimaryMap::new()
    }
}

/// Immutable indexing into an `PrimaryMap`.
/// The indexed value must be in the map.
impl<K, V> Index<K> for PrimaryMap<K, V>
where
    K: EntityRef,
{
    type Output = V;

    fn index(&self, k: K) -> &V {
        &self.elems[k.index()]
    }
}

/// Mutable indexing into an `PrimaryMap`.
impl<K, V> IndexMut<K> for PrimaryMap<K, V>
where
    K: EntityRef,
{
    fn index_mut(&mut self, k: K) -> &mut V {
        &mut self.elems[k.index()]
    }
}

impl<K, V> IntoIterator for PrimaryMap<K, V>
where
    K: EntityRef,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.elems.into_iter())
    }
}

impl<'a, K, V> IntoIterator for &'a PrimaryMap<K, V>
where
    K: EntityRef,
{
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self.elems.iter())
    }
}

impl<'a, K, V> IntoIterator for &'a mut PrimaryMap<K, V>
where
    K: EntityRef,
{
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self.elems.iter_mut())
    }
}

impl<K, V> FromIterator<V> for PrimaryMap<K, V>
where
    K: EntityRef,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = V>,
    {
        Self {
            elems: Vec::from_iter(iter),
            unused: PhantomData,
        }
    }
}

impl<K, V> From<Vec<V>> for PrimaryMap<K, V>
where
    K: EntityRef,
{
    fn from(elems: Vec<V>) -> Self {
        Self {
            elems,
            unused: PhantomData,
        }
    }
}

impl<K, V> From<PrimaryMap<K, V>> for Vec<V>
where
    K: EntityRef,
{
    fn from(map: PrimaryMap<K, V>) -> Self {
        map.elems
    }
}

impl<K: EntityRef + fmt::Debug, V: fmt::Debug> fmt::Debug for PrimaryMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut struct_ = f.debug_struct("PrimaryMap");
        for (k, v) in self {
            struct_.field(&alloc::format!("{k:?}"), v);
        }
        struct_.finish()
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
        let m = PrimaryMap::<E, isize>::new();

        let v: Vec<E> = m.keys().collect();
        assert_eq!(v, []);

        assert!(!m.is_valid(r0));
        assert!(!m.is_valid(r1));
    }

    #[test]
    fn push() {
        let mut m = PrimaryMap::new();
        let k0: E = m.push(12);
        let k1 = m.push(33);

        assert_eq!(m[k0], 12);
        assert_eq!(m[k1], 33);

        let v: Vec<E> = m.keys().collect();
        assert_eq!(v, [k0, k1]);
    }

    #[test]
    fn iter() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        m.push(12);
        m.push(33);

        let mut i = 0;
        for (key, value) in &m {
            assert_eq!(key.index(), i);
            match i {
                0 => assert_eq!(*value, 12),
                1 => assert_eq!(*value, 33),
                _ => panic!(),
            }
            i += 1;
        }
        i = 0;
        for (key_mut, value_mut) in m.iter_mut() {
            assert_eq!(key_mut.index(), i);
            match i {
                0 => assert_eq!(*value_mut, 12),
                1 => assert_eq!(*value_mut, 33),
                _ => panic!(),
            }
            i += 1;
        }
    }

    #[test]
    fn iter_rev() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        m.push(12);
        m.push(33);

        let mut i = 2;
        for (key, value) in m.iter().rev() {
            i -= 1;
            assert_eq!(key.index(), i);
            match i {
                0 => assert_eq!(*value, 12),
                1 => assert_eq!(*value, 33),
                _ => panic!(),
            }
        }

        i = 2;
        for (key, value) in m.iter_mut().rev() {
            i -= 1;
            assert_eq!(key.index(), i);
            match i {
                0 => assert_eq!(*value, 12),
                1 => assert_eq!(*value, 33),
                _ => panic!(),
            }
        }
    }
    #[test]
    fn keys() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        m.push(12);
        m.push(33);

        let mut i = 0;
        for key in m.keys() {
            assert_eq!(key.index(), i);
            i += 1;
        }
    }

    #[test]
    fn keys_rev() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        m.push(12);
        m.push(33);

        let mut i = 2;
        for key in m.keys().rev() {
            i -= 1;
            assert_eq!(key.index(), i);
        }
    }

    #[test]
    fn values() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        m.push(12);
        m.push(33);

        let mut i = 0;
        for value in m.values() {
            match i {
                0 => assert_eq!(*value, 12),
                1 => assert_eq!(*value, 33),
                _ => panic!(),
            }
            i += 1;
        }
        i = 0;
        for value_mut in m.values_mut() {
            match i {
                0 => assert_eq!(*value_mut, 12),
                1 => assert_eq!(*value_mut, 33),
                _ => panic!(),
            }
            i += 1;
        }
    }

    #[test]
    fn values_rev() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        m.push(12);
        m.push(33);

        let mut i = 2;
        for value in m.values().rev() {
            i -= 1;
            match i {
                0 => assert_eq!(*value, 12),
                1 => assert_eq!(*value, 33),
                _ => panic!(),
            }
        }
        i = 2;
        for value_mut in m.values_mut().rev() {
            i -= 1;
            match i {
                0 => assert_eq!(*value_mut, 12),
                1 => assert_eq!(*value_mut, 33),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn from_iter() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        m.push(12);
        m.push(33);

        let n = m.values().collect::<PrimaryMap<E, _>>();
        assert!(m.len() == n.len());
        for (me, ne) in m.values().zip(n.values()) {
            assert!(*me == **ne);
        }
    }

    #[test]
    fn from_vec() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        m.push(12);
        m.push(33);

        let n = PrimaryMap::<E, &usize>::from(m.values().collect::<Vec<_>>());
        assert!(m.len() == n.len());
        for (me, ne) in m.values().zip(n.values()) {
            assert!(*me == **ne);
        }
    }

    #[test]
    fn get_many_mut() {
        let mut m: PrimaryMap<E, usize> = PrimaryMap::new();
        let _0 = m.push(0);
        let _1 = m.push(1);
        let _2 = m.push(2);

        assert_eq!([&mut 0, &mut 2], m.get_disjoint_mut([_0, _2]).unwrap());
        assert_eq!(
            m.get_disjoint_mut([_0, _0]),
            Err(slice::GetDisjointMutError::OverlappingIndices)
        );
        assert_eq!(
            m.get_disjoint_mut([E(4)]),
            Err(slice::GetDisjointMutError::IndexOutOfBounds)
        );
    }
}
