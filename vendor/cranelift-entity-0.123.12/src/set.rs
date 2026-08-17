//! Densely numbered entity references as set keys.

use crate::EntityRef;
use crate::keys::Keys;
use core::fmt;
use core::marker::PhantomData;
use cranelift_bitset::CompoundBitSet;

/// A set of `K` for densely indexed entity references.
///
/// The `EntitySet` data structure uses the dense index space to implement a set with a bitvector.
/// Like `SecondaryMap`, an `EntitySet` is used to associate secondary information with entities.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct EntitySet<K>
where
    K: EntityRef,
{
    bitset: CompoundBitSet,
    unused: PhantomData<K>,
}

impl<K: fmt::Debug> fmt::Debug for EntitySet<K>
where
    K: EntityRef,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.keys()).finish()
    }
}

impl<K: EntityRef> Default for EntitySet<K> {
    fn default() -> Self {
        Self {
            bitset: CompoundBitSet::default(),
            unused: PhantomData,
        }
    }
}

impl<K: EntityRef> Extend<K> for EntitySet<K> {
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for k in iter {
            self.insert(k);
        }
    }
}

/// Shared `EntitySet` implementation for all value types.
impl<K> EntitySet<K>
where
    K: EntityRef,
{
    /// Create a new empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty set with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bitset: CompoundBitSet::with_capacity(capacity),
            unused: PhantomData,
        }
    }

    /// Ensure that the set has enough capacity to hold `capacity` total
    /// elements.
    pub fn ensure_capacity(&mut self, capacity: usize) {
        self.bitset.ensure_capacity(capacity);
    }

    /// Get the element at `k` if it exists.
    pub fn contains(&self, k: K) -> bool {
        let index = k.index();
        self.bitset.contains(index)
    }

    /// Is this set completely empty?
    pub fn is_empty(&self) -> bool {
        self.bitset.is_empty()
    }

    /// Remove all entries from this set.
    pub fn clear(&mut self) {
        self.bitset.clear()
    }

    /// Iterate over all the keys up to the maximum in this set.
    ///
    /// This will yield intermediate keys on the way up to the max key, even if
    /// they are not contained within the set.
    ///
    /// ```
    /// use cranelift_entity::{entity_impl, EntityRef, EntitySet};
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// struct Entity(u32);
    /// entity_impl!(Entity);
    ///
    /// let mut set = EntitySet::new();
    /// set.insert(Entity::new(2));
    ///
    /// let mut keys = set.keys();
    /// assert_eq!(keys.next(), Some(Entity::new(0)));
    /// assert_eq!(keys.next(), Some(Entity::new(1)));
    /// assert_eq!(keys.next(), Some(Entity::new(2)));
    /// assert!(keys.next().is_none());
    /// ```
    pub fn keys(&self) -> Keys<K> {
        Keys::with_len(self.bitset.max().map_or(0, |x| x + 1))
    }

    /// Iterate over the elements of this set.
    ///
    /// ```
    /// use cranelift_entity::{entity_impl, EntityRef, EntitySet};
    ///
    /// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// struct Entity(u32);
    /// entity_impl!(Entity);
    ///
    /// let mut set = EntitySet::new();
    /// set.insert(Entity::new(2));
    /// set.insert(Entity::new(3));
    ///
    /// let mut iter = set.iter();
    /// assert_eq!(iter.next(), Some(Entity::new(2)));
    /// assert_eq!(iter.next(), Some(Entity::new(3)));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> SetIter<'_, K> {
        SetIter {
            inner: self.bitset.iter(),
            _phantom: PhantomData,
        }
    }

    /// Insert the element at `k`.
    ///
    /// Returns `true` if `k` was not present in the set, i.e. this is a
    /// newly-added element. Returns `false` otherwise.
    pub fn insert(&mut self, k: K) -> bool {
        let index = k.index();
        self.bitset.insert(index)
    }

    /// Remove `k` from this bitset.
    ///
    /// Returns whether `k` was previously in this set or not.
    pub fn remove(&mut self, k: K) -> bool {
        let index = k.index();
        self.bitset.remove(index)
    }

    /// Removes and returns the entity from the set if it exists.
    pub fn pop(&mut self) -> Option<K> {
        let index = self.bitset.pop()?;
        Some(K::new(index))
    }
}

/// An iterator over the elements in an `EntitySet`.
pub struct SetIter<'a, K> {
    inner: cranelift_bitset::compound::Iter<'a>,
    _phantom: PhantomData<K>,
}

impl<K> Iterator for SetIter<'_, K>
where
    K: EntityRef,
{
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let k = self.inner.next()?;
        Some(K::new(k))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use core::u32;

    // `EntityRef` impl for testing.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
        let mut m = EntitySet::new();

        let v: Vec<E> = m.keys().collect();
        assert_eq!(v, []);
        assert!(m.is_empty());

        m.insert(r2);
        m.insert(r1);

        assert!(!m.contains(r0));
        assert!(m.contains(r1));
        assert!(m.contains(r2));
        assert!(!m.contains(E(3)));
        assert!(!m.is_empty());

        let v: Vec<E> = m.keys().collect();
        assert_eq!(v, [r0, r1, r2]);

        assert!(!m.contains(E(3)));
        assert!(!m.contains(E(4)));
        assert!(!m.contains(E(8)));
        assert!(!m.contains(E(15)));
        assert!(!m.contains(E(19)));

        m.insert(E(8));
        m.insert(E(15));
        assert!(!m.contains(E(3)));
        assert!(!m.contains(E(4)));
        assert!(m.contains(E(8)));
        assert!(!m.contains(E(9)));
        assert!(!m.contains(E(14)));
        assert!(m.contains(E(15)));
        assert!(!m.contains(E(16)));
        assert!(!m.contains(E(19)));
        assert!(!m.contains(E(20)));
        assert!(!m.contains(E(u32::MAX)));

        m.clear();
        assert!(m.is_empty());
    }

    #[test]
    fn pop_ordered() {
        let r0 = E(0);
        let r1 = E(1);
        let r2 = E(2);
        let mut m = EntitySet::new();
        m.insert(r0);
        m.insert(r1);
        m.insert(r2);

        assert_eq!(r2, m.pop().unwrap());
        assert_eq!(r1, m.pop().unwrap());
        assert_eq!(r0, m.pop().unwrap());
        assert!(m.pop().is_none());
        assert!(m.pop().is_none());
    }

    #[test]
    fn pop_unordered() {
        let mut blocks = [
            E(0),
            E(1),
            E(6),
            E(7),
            E(5),
            E(9),
            E(10),
            E(2),
            E(3),
            E(11),
            E(12),
        ];

        let mut m = EntitySet::new();
        for &block in &blocks {
            m.insert(block);
        }
        assert_eq!(m.bitset.max(), Some(12));
        blocks.sort();

        for &block in blocks.iter().rev() {
            assert_eq!(block, m.pop().unwrap());
        }

        assert!(m.is_empty());
    }
}
