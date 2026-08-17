//! Sparse mapping of entity references to larger value types.
//!
//! This module provides a `SparseMap` data structure which implements a sparse mapping from an
//! `EntityRef` key to a value type that may be on the larger side. This implementation is based on
//! the paper:
//!
//! > Briggs, Torczon, *An efficient representation for sparse sets*,
//! > ACM Letters on Programming Languages and Systems, Volume 2, Issue 1-4, March-Dec. 1993.

use crate::EntityRef;
use crate::map::SecondaryMap;
use alloc::vec::Vec;
use core::fmt;
use core::mem;
use core::slice;
use core::u32;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Trait for extracting keys from values stored in a `SparseMap`.
///
/// All values stored in a `SparseMap` must keep track of their own key in the map and implement
/// this trait to provide access to the key.
pub trait SparseMapValue<K> {
    /// Get the key of this sparse map value. This key is not allowed to change while the value
    /// is a member of the map.
    fn key(&self) -> K;
}

/// A sparse mapping of entity references.
///
/// A `SparseMap<K, V>` map provides:
///
/// - Memory usage equivalent to `SecondaryMap<K, u32>` + `Vec<V>`, so much smaller than
///   `SecondaryMap<K, V>` for sparse mappings of larger `V` types.
/// - Constant time lookup, slightly slower than `SecondaryMap`.
/// - A very fast, constant time `clear()` operation.
/// - Fast insert and erase operations.
/// - Stable iteration that is as fast as a `Vec<V>`.
///
/// # Compared to `SecondaryMap`
///
/// When should we use a `SparseMap` instead of a secondary `SecondaryMap`? First of all,
/// `SparseMap` does not provide the functionality of a `PrimaryMap` which can allocate and assign
/// entity references to objects as they are pushed onto the map. It is only the secondary entity
/// maps that can be replaced with a `SparseMap`.
///
/// - A secondary entity map assigns a default mapping to all keys. It doesn't distinguish between
///   an unmapped key and one that maps to the default value. `SparseMap` does not require
///   `Default` values, and it tracks accurately if a key has been mapped or not.
/// - Iterating over the contents of an `SecondaryMap` is linear in the size of the *key space*,
///   while iterating over a `SparseMap` is linear in the number of elements in the mapping. This
///   is an advantage precisely when the mapping is sparse.
/// - `SparseMap::clear()` is constant time and super-fast. `SecondaryMap::clear()` is linear in
///   the size of the key space. (Or, rather the required `resize()` call following the `clear()`
///   is).
/// - `SparseMap` requires the values to implement `SparseMapValue<K>` which means that they must
///   contain their own key.
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct SparseMap<K, V>
where
    K: EntityRef,
    V: SparseMapValue<K>,
{
    sparse: SecondaryMap<K, u32>,
    dense: Vec<V>,
}

impl<K, V> SparseMap<K, V>
where
    K: EntityRef,
    V: SparseMapValue<K>,
{
    /// Create a new empty mapping.
    pub fn new() -> Self {
        Self {
            sparse: SecondaryMap::new(),
            dense: Vec::new(),
        }
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.dense.len()
    }

    /// Returns true is the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    /// Remove all elements from the mapping.
    pub fn clear(&mut self) {
        self.dense.clear();
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: K) -> Option<&V> {
        if let Some(idx) = self.sparse.get(key).cloned() {
            if let Some(entry) = self.dense.get(idx as usize) {
                if entry.key() == key {
                    return Some(entry);
                }
            }
        }
        None
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// Note that the returned value must not be mutated in a way that would change its key. This
    /// would invalidate the sparse set data structure.
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        if let Some(idx) = self.sparse.get(key).cloned() {
            if let Some(entry) = self.dense.get_mut(idx as usize) {
                if entry.key() == key {
                    return Some(entry);
                }
            }
        }
        None
    }

    /// Return the index into `dense` of the value corresponding to `key`.
    fn index(&self, key: K) -> Option<usize> {
        if let Some(idx) = self.sparse.get(key).cloned() {
            let idx = idx as usize;
            if let Some(entry) = self.dense.get(idx) {
                if entry.key() == key {
                    return Some(idx);
                }
            }
        }
        None
    }

    /// Return `true` if the map contains a value corresponding to `key`.
    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    /// Insert a value into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old value is returned.
    ///
    /// It is not necessary to provide a key since the value knows its own key already.
    pub fn insert(&mut self, value: V) -> Option<V> {
        let key = value.key();

        // Replace the existing entry for `key` if there is one.
        if let Some(entry) = self.get_mut(key) {
            return Some(mem::replace(entry, value));
        }

        // There was no previous entry for `key`. Add it to the end of `dense`.
        let idx = self.dense.len();
        debug_assert!(idx <= u32::MAX as usize, "SparseMap overflow");
        self.dense.push(value);
        self.sparse[key] = idx as u32;
        None
    }

    /// Remove a value from the map and return it.
    pub fn remove(&mut self, key: K) -> Option<V> {
        if let Some(idx) = self.index(key) {
            let back = self.dense.pop().unwrap();

            // Are we popping the back of `dense`?
            if idx == self.dense.len() {
                return Some(back);
            }

            // We're removing an element from the middle of `dense`.
            // Replace the element at `idx` with the back of `dense`.
            // Repair `sparse` first.
            self.sparse[back.key()] = idx as u32;
            return Some(mem::replace(&mut self.dense[idx], back));
        }

        // Nothing to remove.
        None
    }

    /// Remove the last value from the map.
    pub fn pop(&mut self) -> Option<V> {
        self.dense.pop()
    }

    /// Get an iterator over the values in the map.
    ///
    /// The iteration order is entirely determined by the preceding sequence of `insert` and
    /// `remove` operations. In particular, if no elements were removed, this is the insertion
    /// order.
    pub fn values(&self) -> slice::Iter<'_, V> {
        self.dense.iter()
    }

    /// Get the values as a slice.
    pub fn as_slice(&self) -> &[V] {
        self.dense.as_slice()
    }
}

impl<K, V> Default for SparseMap<K, V>
where
    K: EntityRef,
    V: SparseMapValue<K>,
{
    fn default() -> SparseMap<K, V> {
        SparseMap::new()
    }
}

impl<K, V> fmt::Debug for SparseMap<K, V>
where
    K: EntityRef + fmt::Debug,
    V: SparseMapValue<K> + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.values().map(|v| (v.key(), v)))
            .finish()
    }
}

/// Iterating over the elements of a set.
impl<'a, K, V> IntoIterator for &'a SparseMap<K, V>
where
    K: EntityRef,
    V: SparseMapValue<K>,
{
    type Item = &'a V;
    type IntoIter = slice::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.values()
    }
}

/// Any `EntityRef` can be used as a sparse map value representing itself.
impl<T> SparseMapValue<T> for T
where
    T: EntityRef,
{
    fn key(&self) -> Self {
        *self
    }
}

/// A sparse set of entity references.
///
/// Any type that implements `EntityRef` can be used as a sparse set value too.
pub type SparseSet<T> = SparseMap<T, T>;

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;

    /// An opaque reference to an instruction in a function.
    #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Inst(u32);
    entity_impl!(Inst, "inst");

    // Mock key-value object for testing.
    #[derive(PartialEq, Eq, Debug)]
    struct Obj(Inst, &'static str);

    impl SparseMapValue<Inst> for Obj {
        fn key(&self) -> Inst {
            self.0
        }
    }

    #[test]
    fn empty_immutable_map() {
        let i1 = Inst::new(1);
        let map: SparseMap<Inst, Obj> = SparseMap::new();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.get(i1), None);
        assert_eq!(map.values().count(), 0);
    }

    #[test]
    fn single_entry() {
        let i0 = Inst::new(0);
        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let mut map = SparseMap::new();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(map.get(i1), None);
        assert_eq!(map.get_mut(i1), None);
        assert_eq!(map.remove(i1), None);

        assert_eq!(map.insert(Obj(i1, "hi")), None);
        assert!(!map.is_empty());
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(i0), None);
        assert_eq!(map.get(i1), Some(&Obj(i1, "hi")));
        assert_eq!(map.get(i2), None);
        assert_eq!(map.get_mut(i0), None);
        assert_eq!(map.get_mut(i1), Some(&mut Obj(i1, "hi")));
        assert_eq!(map.get_mut(i2), None);

        assert_eq!(map.remove(i0), None);
        assert_eq!(map.remove(i2), None);
        assert_eq!(map.remove(i1), Some(Obj(i1, "hi")));
        assert_eq!(map.len(), 0);
        assert_eq!(map.get(i1), None);
        assert_eq!(map.get_mut(i1), None);
        assert_eq!(map.remove(i0), None);
        assert_eq!(map.remove(i1), None);
        assert_eq!(map.remove(i2), None);
    }

    #[test]
    fn multiple_entries() {
        let i0 = Inst::new(0);
        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let i3 = Inst::new(3);
        let mut map = SparseMap::new();

        assert_eq!(map.insert(Obj(i2, "foo")), None);
        assert_eq!(map.insert(Obj(i1, "bar")), None);
        assert_eq!(map.insert(Obj(i0, "baz")), None);

        // Iteration order = insertion order when nothing has been removed yet.
        assert_eq!(
            map.values().map(|obj| obj.1).collect::<Vec<_>>(),
            ["foo", "bar", "baz"]
        );

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(i0), Some(&Obj(i0, "baz")));
        assert_eq!(map.get(i1), Some(&Obj(i1, "bar")));
        assert_eq!(map.get(i2), Some(&Obj(i2, "foo")));
        assert_eq!(map.get(i3), None);

        // Remove front object, causing back to be swapped down.
        assert_eq!(map.remove(i1), Some(Obj(i1, "bar")));
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(i0), Some(&Obj(i0, "baz")));
        assert_eq!(map.get(i1), None);
        assert_eq!(map.get(i2), Some(&Obj(i2, "foo")));
        assert_eq!(map.get(i3), None);

        // Reinsert something at a previously used key.
        assert_eq!(map.insert(Obj(i1, "barbar")), None);
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(i0), Some(&Obj(i0, "baz")));
        assert_eq!(map.get(i1), Some(&Obj(i1, "barbar")));
        assert_eq!(map.get(i2), Some(&Obj(i2, "foo")));
        assert_eq!(map.get(i3), None);

        // Replace an entry.
        assert_eq!(map.insert(Obj(i0, "bazbaz")), Some(Obj(i0, "baz")));
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(i0), Some(&Obj(i0, "bazbaz")));
        assert_eq!(map.get(i1), Some(&Obj(i1, "barbar")));
        assert_eq!(map.get(i2), Some(&Obj(i2, "foo")));
        assert_eq!(map.get(i3), None);

        // Check the reference `IntoIter` impl.
        let mut v = Vec::new();
        for i in &map {
            v.push(i.1);
        }
        assert_eq!(v.len(), map.len());
    }

    #[test]
    fn entity_set() {
        let i0 = Inst::new(0);
        let i1 = Inst::new(1);
        let mut set = SparseSet::new();

        assert_eq!(set.insert(i0), None);
        assert_eq!(set.insert(i0), Some(i0));
        assert_eq!(set.insert(i1), None);
        assert_eq!(set.get(i0), Some(&i0));
        assert_eq!(set.get(i1), Some(&i1));
    }

    #[test]
    fn default_impl() {
        let map: SparseMap<Inst, Obj> = SparseMap::default();

        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn debug_impl() {
        let i1 = Inst::new(1);
        let mut map = SparseMap::new();
        assert_eq!(map.insert(Obj(i1, "hi")), None);

        let debug = format!("{map:?}");
        let expected = "{inst1: Obj(inst1, \"hi\")}";
        assert_eq!(debug, expected);
    }
}
