//! Boxed slices for `PrimaryMap`.

use crate::EntityRef;
use crate::iter::{Iter, IterMut};
use crate::keys::Keys;
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::slice;

/// A slice mapping `K -> V` allocating dense entity references.
///
/// The `BoxedSlice` data structure uses the dense index space to implement a map with a boxed
/// slice.
#[derive(Debug, Clone)]
pub struct BoxedSlice<K, V>
where
    K: EntityRef,
{
    elems: Box<[V]>,
    unused: PhantomData<K>,
}

impl<K, V> BoxedSlice<K, V>
where
    K: EntityRef,
{
    /// Create a new slice from a raw pointer. A safer way to create slices is
    /// to use `PrimaryMap::into_boxed_slice()`.
    ///
    /// # Safety
    ///
    /// This relies on `raw` pointing to a valid slice of `V`s.
    pub unsafe fn from_raw(raw: *mut [V]) -> Self {
        Self {
            elems: unsafe { Box::from_raw(raw) },
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

    /// Returns the last element that was inserted in the map.
    pub fn last(&self) -> Option<&V> {
        self.elems.last()
    }
}

/// Immutable indexing into a `BoxedSlice`.
/// The indexed value must be in the map.
impl<K, V> Index<K> for BoxedSlice<K, V>
where
    K: EntityRef,
{
    type Output = V;

    fn index(&self, k: K) -> &V {
        &self.elems[k.index()]
    }
}

/// Mutable indexing into a `BoxedSlice`.
impl<K, V> IndexMut<K> for BoxedSlice<K, V>
where
    K: EntityRef,
{
    fn index_mut(&mut self, k: K) -> &mut V {
        &mut self.elems[k.index()]
    }
}

impl<'a, K, V> IntoIterator for &'a BoxedSlice<K, V>
where
    K: EntityRef,
{
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self.elems.iter())
    }
}

impl<'a, K, V> IntoIterator for &'a mut BoxedSlice<K, V>
where
    K: EntityRef,
{
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self.elems.iter_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primary::PrimaryMap;
    use alloc::vec::Vec;

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
        let p = PrimaryMap::<E, isize>::new();
        let m = p.into_boxed_slice();

        let v: Vec<E> = m.keys().collect();
        assert_eq!(v, []);

        assert!(!m.is_valid(r0));
        assert!(!m.is_valid(r1));
    }

    #[test]
    fn iter() {
        let mut p: PrimaryMap<E, usize> = PrimaryMap::new();
        p.push(12);
        p.push(33);
        let mut m = p.into_boxed_slice();

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
        let mut p: PrimaryMap<E, usize> = PrimaryMap::new();
        p.push(12);
        p.push(33);
        let mut m = p.into_boxed_slice();

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
        let mut p: PrimaryMap<E, usize> = PrimaryMap::new();
        p.push(12);
        p.push(33);
        let m = p.into_boxed_slice();

        let mut i = 0;
        for key in m.keys() {
            assert_eq!(key.index(), i);
            i += 1;
        }
    }

    #[test]
    fn keys_rev() {
        let mut p: PrimaryMap<E, usize> = PrimaryMap::new();
        p.push(12);
        p.push(33);
        let m = p.into_boxed_slice();

        let mut i = 2;
        for key in m.keys().rev() {
            i -= 1;
            assert_eq!(key.index(), i);
        }
    }

    #[test]
    fn values() {
        let mut p: PrimaryMap<E, usize> = PrimaryMap::new();
        p.push(12);
        p.push(33);
        let mut m = p.into_boxed_slice();

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
        let mut p: PrimaryMap<E, usize> = PrimaryMap::new();
        p.push(12);
        p.push(33);
        let mut m = p.into_boxed_slice();

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
}
