//! Contains the sparse secondary map implementation.

#[cfg(all(nightly, any(doc, feature = "unstable")))]
use alloc::collections::TryReserveError;
#[allow(unused_imports)] // MaybeUninit is only used on nightly at the moment.
use core::mem::MaybeUninit;
use std::collections::hash_map::{self, HashMap};
use std::hash;
use std::iter::{Extend, FromIterator, FusedIterator};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use super::{Key, KeyData};
use crate::util::{is_older_version, UnwrapUnchecked};

#[derive(Debug, Clone)]
struct Slot<T> {
    version: u32,
    value: T,
}

/// Sparse secondary map, associate data with previously stored elements in a
/// slot map.
///
/// A [`SparseSecondaryMap`] allows you to efficiently store additional
/// information for each element in a slot map. You can have multiple secondary
/// maps per slot map, but not multiple slot maps per secondary map. It is safe
/// but unspecified behavior if you use keys from multiple different slot maps
/// in the same [`SparseSecondaryMap`].
///
/// A [`SparseSecondaryMap`] does not leak memory even if you never remove
/// elements. In return, when you remove a key from the primary slot map, after
/// any insert the space associated with the removed element may be reclaimed.
/// Don't expect the values associated with a removed key to stick around after
/// an insertion has happened!
///
/// Unlike [`SecondaryMap`], the [`SparseSecondaryMap`] is backed by a
/// [`HashMap`]. This means its access times are higher, but it uses less memory
/// and iterates faster if there are only a few elements of the slot map in the
/// secondary map. If most or all of the elements in a slot map are also found
/// in the secondary map, use a [`SecondaryMap`] instead.
///
/// The current implementation of [`SparseSecondaryMap`] requires [`std`] and is
/// thus not available in `no_std` environments.
///
/// [`SecondaryMap`]: crate::SecondaryMap
/// [`HashMap`]: std::collections::HashMap
///
/// Example usage:
///
/// ```
/// # use slotmap::*;
/// let mut players = SlotMap::new();
/// let mut health = SparseSecondaryMap::new();
/// let mut ammo = SparseSecondaryMap::new();
///
/// let alice = players.insert("alice");
/// let bob = players.insert("bob");
///
/// for p in players.keys() {
///     health.insert(p, 100);
///     ammo.insert(p, 30);
/// }
///
/// // Alice attacks Bob with all her ammo!
/// health[bob] -= ammo[alice] * 3;
/// ammo[alice] = 0;
/// ```

#[derive(Debug, Clone)]
pub struct SparseSecondaryMap<K: Key, V, S: hash::BuildHasher = hash_map::RandomState> {
    slots: HashMap<u32, Slot<V>, S>,
    _k: PhantomData<fn(K) -> K>,
}

impl<K: Key, V> SparseSecondaryMap<K, V, hash_map::RandomState> {
    /// Constructs a new, empty [`SparseSecondaryMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SparseSecondaryMap<DefaultKey, i32> = SparseSecondaryMap::new();
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates an empty [`SparseSecondaryMap`] with the given capacity of slots.
    ///
    /// The secondary map will not reallocate until it holds at least `capacity`
    /// slots.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm: SlotMap<_, i32> = SlotMap::with_capacity(10);
    /// let mut sec: SparseSecondaryMap<DefaultKey, i32> =
    ///     SparseSecondaryMap::with_capacity(sm.capacity());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: HashMap::with_capacity(capacity),
            _k: PhantomData,
        }
    }
}

impl<K: Key, V, S: hash::BuildHasher> SparseSecondaryMap<K, V, S> {
    /// Creates an empty [`SparseSecondaryMap`] which will use the given hash
    /// builder to hash keys.
    ///
    /// The secondary map will not reallocate until it holds at least `capacity`
    /// slots.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// # use slotmap::*;
    /// let mut sm: SlotMap<_, i32> = SlotMap::with_capacity(10);
    /// let mut sec: SparseSecondaryMap<DefaultKey, i32, _> =
    ///     SparseSecondaryMap::with_hasher(RandomState::new());
    /// ```
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            slots: HashMap::with_hasher(hash_builder),
            _k: PhantomData,
        }
    }

    /// Creates an empty [`SparseSecondaryMap`] with the given capacity of slots,
    /// using `hash_builder` to hash the keys.
    ///
    /// The secondary map will not reallocate until it holds at least `capacity`
    /// slots.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::hash_map::RandomState;
    /// # use slotmap::*;
    /// let mut sm: SlotMap<_, i32> = SlotMap::with_capacity(10);
    /// let mut sec: SparseSecondaryMap<DefaultKey, i32, _> =
    ///     SparseSecondaryMap::with_capacity_and_hasher(10, RandomState::new());
    /// ```
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            slots: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            _k: PhantomData,
        }
    }

    /// Returns the number of elements in the secondary map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let k = sm.insert(4);
    /// let mut squared = SparseSecondaryMap::new();
    /// assert_eq!(squared.len(), 0);
    /// squared.insert(k, 16);
    /// assert_eq!(squared.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Returns if the secondary map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SparseSecondaryMap<DefaultKey, i32> = SparseSecondaryMap::new();
    /// assert!(sec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Returns the number of elements the [`SparseSecondaryMap`] can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SparseSecondaryMap<DefaultKey, i32> = SparseSecondaryMap::with_capacity(10);
    /// assert!(sec.capacity() >= 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Reserves capacity for at least `additional` more slots in the
    /// [`SparseSecondaryMap`]. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SparseSecondaryMap<DefaultKey, i32> = SparseSecondaryMap::new();
    /// sec.reserve(10);
    /// assert!(sec.capacity() >= 10);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.slots.reserve(additional);
    }

    /// Tries to reserve capacity for at least `additional` more slots in the
    /// [`SparseSecondaryMap`].  The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SparseSecondaryMap<DefaultKey, i32> = SparseSecondaryMap::new();
    /// sec.try_reserve(10).unwrap();
    /// assert!(sec.capacity() >= 10);
    /// ```
    #[cfg(all(nightly, any(doc, feature = "unstable")))]
    #[cfg_attr(all(nightly, doc), doc(cfg(feature = "unstable")))]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.slots.try_reserve(additional)
    }

    /// Returns [`true`] if the secondary map contains `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let k = sm.insert(4);
    /// let mut squared = SparseSecondaryMap::new();
    /// assert!(!squared.contains_key(k));
    /// squared.insert(k, 16);
    /// assert!(squared.contains_key(k));
    /// ```
    pub fn contains_key(&self, key: K) -> bool {
        let kd = key.data();
        self.slots.get(&kd.idx).map_or(false, |slot| slot.version == kd.version.get())
    }

    /// Inserts a value into the secondary map at the given `key`. Can silently
    /// fail if `key` was removed from the originating slot map.
    ///
    /// Returns [`None`] if this key was not present in the map, the old value
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let k = sm.insert(4);
    /// let mut squared = SparseSecondaryMap::new();
    /// assert_eq!(squared.insert(k, 0), None);
    /// assert_eq!(squared.insert(k, 4), Some(0));
    /// // You don't have to use insert if the key is already in the secondary map.
    /// squared[k] *= squared[k];
    /// assert_eq!(squared[k], 16);
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if key.is_null() {
            return None;
        }

        let kd = key.data();

        if let Some(slot) = self.slots.get_mut(&kd.idx) {
            if slot.version == kd.version.get() {
                return Some(std::mem::replace(&mut slot.value, value));
            }

            // Don't replace existing newer values.
            if is_older_version(kd.version.get(), slot.version) {
                return None;
            }

            *slot = Slot {
                version: kd.version.get(),
                value,
            };

            return None;
        }

        self.slots.insert(kd.idx, Slot {
            version: kd.version.get(),
            value,
        });

        None
    }

    /// Removes a key from the secondary map, returning the value at the key if
    /// the key was not previously removed. If `key` was removed from the
    /// originating slot map, its corresponding entry in the secondary map may
    /// or may not already be removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut squared = SparseSecondaryMap::new();
    /// let k = sm.insert(4);
    /// squared.insert(k, 16);
    /// squared.remove(k);
    /// assert!(!squared.contains_key(k));
    ///
    /// // It's not necessary to remove keys deleted from the primary slot map, they
    /// // get deleted automatically when their slots are reused on a subsequent insert.
    /// squared.insert(k, 16);
    /// sm.remove(k); // Remove k from the slot map, making an empty slot.
    /// let new_k = sm.insert(2); // Since sm only has one empty slot, this reuses it.
    /// assert!(!squared.contains_key(new_k)); // Space reuse does not mean equal keys.
    /// assert!(squared.contains_key(k)); // Slot has not been reused in squared yet.
    /// squared.insert(new_k, 4);
    /// assert!(!squared.contains_key(k)); // Old key is no longer available.
    /// ```
    pub fn remove(&mut self, key: K) -> Option<V> {
        let kd = key.data();

        if let hash_map::Entry::Occupied(entry) = self.slots.entry(kd.idx) {
            if entry.get().version == kd.version.get() {
                return Some(entry.remove_entry().1.value);
            }
        }

        None
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all key-value pairs `(k, v)` such that
    /// `f(k, &mut v)` returns false. This method invalidates any removed keys.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k1 = sm.insert(0); sec.insert(k1, 10);
    /// let k2 = sm.insert(1); sec.insert(k2, 11);
    /// let k3 = sm.insert(2); sec.insert(k3, 12);
    ///
    /// sec.retain(|key, val| key == k1 || *val == 11);
    ///
    /// assert!(sec.contains_key(k1));
    /// assert!(sec.contains_key(k2));
    /// assert!(!sec.contains_key(k3));
    ///
    /// assert_eq!(2, sec.len());
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(K, &mut V) -> bool,
    {
        self.slots.retain(|&idx, slot| {
            let key = KeyData::new(idx, slot.version).into();
            f(key, &mut slot.value)
        })
    }

    /// Clears the secondary map. Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// for i in 0..10 {
    ///     sec.insert(sm.insert(i), i);
    /// }
    /// assert_eq!(sec.len(), 10);
    /// sec.clear();
    /// assert_eq!(sec.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.slots.clear();
    }

    /// Clears the slot map, returning all key-value pairs in arbitrary order as
    /// an iterator. Keeps the allocated memory for reuse.
    ///
    /// When the iterator is dropped all elements in the slot map are removed,
    /// even if the iterator was not fully consumed. If the iterator is not
    /// dropped (using e.g. [`std::mem::forget`]), only the elements that were
    /// iterated over are removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::iter::FromIterator;
    /// let mut sm = SlotMap::new();
    /// let k = sm.insert(0);
    /// let mut sec = SparseSecondaryMap::new();
    /// sec.insert(k, 1);
    /// let v: Vec<_> = sec.drain().collect();
    /// assert_eq!(sec.len(), 0);
    /// assert_eq!(v, vec![(k, 1)]);
    /// ```
    pub fn drain(&mut self) -> Drain<K, V> {
        Drain {
            inner: self.slots.drain(),
            _k: PhantomData,
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert("foo");
    /// let mut sec = SparseSecondaryMap::new();
    /// sec.insert(key, "bar");
    /// assert_eq!(sec.get(key), Some(&"bar"));
    /// sec.remove(key);
    /// assert_eq!(sec.get(key), None);
    /// ```
    pub fn get(&self, key: K) -> Option<&V> {
        let kd = key.data();
        self.slots
            .get(&kd.idx)
            .filter(|slot| slot.version == kd.version.get())
            .map(|slot| &slot.value)
    }

    /// Returns a reference to the value corresponding to the key without
    /// version or bounds checking.
    ///
    /// # Safety
    ///
    /// This should only be used if `contains_key(key)` is true. Otherwise it is
    /// potentially unsafe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert("foo");
    /// let mut sec = SparseSecondaryMap::new();
    /// sec.insert(key, "bar");
    /// assert_eq!(unsafe { sec.get_unchecked(key) }, &"bar");
    /// sec.remove(key);
    /// // sec.get_unchecked(key) is now dangerous!
    /// ```
    pub unsafe fn get_unchecked(&self, key: K) -> &V {
        debug_assert!(self.contains_key(key));
        self.get(key).unwrap_unchecked_()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert("test");
    /// let mut sec = SparseSecondaryMap::new();
    /// sec.insert(key, 3.5);
    /// if let Some(x) = sec.get_mut(key) {
    ///     *x += 3.0;
    /// }
    /// assert_eq!(sec[key], 6.5);
    /// ```
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let kd = key.data();
        self.slots
            .get_mut(&kd.idx)
            .filter(|slot| slot.version == kd.version.get())
            .map(|slot| &mut slot.value)
    }

    /// Returns a mutable reference to the value corresponding to the key
    /// without version or bounds checking.
    ///
    /// # Safety
    ///
    /// This should only be used if `contains_key(key)` is true. Otherwise it is
    /// potentially unsafe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert("foo");
    /// let mut sec = SparseSecondaryMap::new();
    /// sec.insert(key, "bar");
    /// unsafe { *sec.get_unchecked_mut(key) = "baz" };
    /// assert_eq!(sec[key], "baz");
    /// sec.remove(key);
    /// // sec.get_unchecked_mut(key) is now dangerous!
    /// ```
    pub unsafe fn get_unchecked_mut(&mut self, key: K) -> &mut V {
        debug_assert!(self.contains_key(key));
        self.get_mut(key).unwrap_unchecked_()
    }

    /// Returns mutable references to the values corresponding to the given
    /// keys. All keys must be valid and disjoint, otherwise None is returned.
    ///
    /// Requires at least stable Rust version 1.51.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// let ka = sm.insert(()); sec.insert(ka, "butter");
    /// let kb = sm.insert(()); sec.insert(kb, "apples");
    /// let kc = sm.insert(()); sec.insert(kc, "charlie");
    /// sec.remove(kc); // Make key c invalid.
    /// assert_eq!(sec.get_disjoint_mut([ka, kb, kc]), None); // Has invalid key.
    /// assert_eq!(sec.get_disjoint_mut([ka, ka]), None); // Not disjoint.
    /// let [a, b] = sec.get_disjoint_mut([ka, kb]).unwrap();
    /// std::mem::swap(a, b);
    /// assert_eq!(sec[ka], "apples");
    /// assert_eq!(sec[kb], "butter");
    /// ```
    #[cfg(has_min_const_generics)]
    pub fn get_disjoint_mut<const N: usize>(&mut self, keys: [K; N]) -> Option<[&mut V; N]> {
        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        let mut ptrs: [MaybeUninit<*mut V>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut i = 0;
        while i < N {
            let kd = keys[i].data();

            match self.slots.get_mut(&kd.idx) {
                Some(Slot { version, value }) if *version == kd.version.get() => {
                    // This key is valid, and the slot is occupied. Temporarily
                    // make the version even so duplicate keys would show up as
                    // invalid, since keys always have an odd version. This
                    // gives us a linear time disjointness check.
                    ptrs[i] = MaybeUninit::new(&mut *value);
                    *version ^= 1;
                },

                _ => break,
            }

            i += 1;
        }

        // Undo temporary even versions.
        for k in &keys[0..i] {
            match self.slots.get_mut(&k.data().idx) {
                Some(Slot { version, .. }) => {
                    *version ^= 1;
                },
                _ => unsafe { core::hint::unreachable_unchecked() },
            }
        }

        if i == N {
            // All were valid and disjoint.
            Some(unsafe { core::mem::transmute_copy::<_, [&mut V; N]>(&ptrs) })
        } else {
            None
        }
    }

    /// Returns mutable references to the values corresponding to the given
    /// keys. All keys must be valid and disjoint.
    ///
    /// Requires at least stable Rust version 1.51.
    ///
    /// # Safety
    ///
    /// This should only be used if `contains_key(key)` is true for every given
    /// key and no two keys are equal. Otherwise it is potentially unsafe.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// let ka = sm.insert(()); sec.insert(ka, "butter");
    /// let kb = sm.insert(()); sec.insert(kb, "apples");
    /// let [a, b] = unsafe { sec.get_disjoint_unchecked_mut([ka, kb]) };
    /// std::mem::swap(a, b);
    /// assert_eq!(sec[ka], "apples");
    /// assert_eq!(sec[kb], "butter");
    /// ```
    #[cfg(has_min_const_generics)]
    pub unsafe fn get_disjoint_unchecked_mut<const N: usize>(
        &mut self,
        keys: [K; N],
    ) -> [&mut V; N] {
        // Safe, see get_disjoint_mut.
        let mut ptrs: [MaybeUninit<*mut V>; N] = MaybeUninit::uninit().assume_init();
        for i in 0..N {
            ptrs[i] = MaybeUninit::new(self.get_unchecked_mut(keys[i]));
        }
        core::mem::transmute_copy::<_, [&mut V; N]>(&ptrs)
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The
    /// iterator element type is `(K, &'a V)`.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// let k0 = sm.insert(0); sec.insert(k0, 10);
    /// let k1 = sm.insert(1); sec.insert(k1, 11);
    /// let k2 = sm.insert(2); sec.insert(k2, 12);
    ///
    /// for (k, v) in sec.iter() {
    ///     println!("key: {:?}, val: {}", k, v);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            inner: self.slots.iter(),
            _k: PhantomData,
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order, with
    /// mutable references to the values. The iterator element type is
    /// `(K, &'a mut V)`.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// let k0 = sm.insert(1); sec.insert(k0, 10);
    /// let k1 = sm.insert(2); sec.insert(k1, 20);
    /// let k2 = sm.insert(3); sec.insert(k2, 30);
    ///
    /// for (k, v) in sec.iter_mut() {
    ///     if k != k1 {
    ///         *v *= -1;
    ///     }
    /// }
    ///
    /// assert_eq!(sec[k0], -10);
    /// assert_eq!(sec[k1], 20);
    /// assert_eq!(sec[k2], -30);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            inner: self.slots.iter_mut(),
            _k: PhantomData,
        }
    }

    /// An iterator visiting all keys in arbitrary order. The iterator element
    /// type is `K`.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::collections::HashSet;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// let k0 = sm.insert(1); sec.insert(k0, 10);
    /// let k1 = sm.insert(2); sec.insert(k1, 20);
    /// let k2 = sm.insert(3); sec.insert(k2, 30);
    /// let keys: HashSet<_> = sec.keys().collect();
    /// let check: HashSet<_> = vec![k0, k1, k2].into_iter().collect();
    /// assert_eq!(keys, check);
    /// ```
    pub fn keys(&self) -> Keys<K, V> {
        Keys { inner: self.iter() }
    }

    /// An iterator visiting all values in arbitrary order. The iterator element
    /// type is `&'a V`.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::collections::HashSet;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// let k0 = sm.insert(1); sec.insert(k0, 10);
    /// let k1 = sm.insert(2); sec.insert(k1, 20);
    /// let k2 = sm.insert(3); sec.insert(k2, 30);
    /// let values: HashSet<_> = sec.values().collect();
    /// let check: HashSet<_> = vec![&10, &20, &30].into_iter().collect();
    /// assert_eq!(values, check);
    /// ```
    pub fn values(&self) -> Values<K, V> {
        Values { inner: self.iter() }
    }

    /// An iterator visiting all values mutably in arbitrary order. The iterator
    /// element type is `&'a mut V`.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::collections::HashSet;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// sec.insert(sm.insert(1), 10);
    /// sec.insert(sm.insert(2), 20);
    /// sec.insert(sm.insert(3), 30);
    /// sec.values_mut().for_each(|n| { *n *= 3 });
    /// let values: HashSet<_> = sec.into_iter().map(|(_k, v)| v).collect();
    /// let check: HashSet<_> = vec![30, 60, 90].into_iter().collect();
    /// assert_eq!(values, check);
    /// ```
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }

    /// Gets the given key's corresponding [`Entry`] in the map for in-place
    /// manipulation. May return [`None`] if the key was removed from the
    /// originating slot map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    /// let k = sm.insert(1);
    /// let v = sec.entry(k).unwrap().or_insert(10);
    /// assert_eq!(*v, 10);
    /// ```
    pub fn entry(&mut self, key: K) -> Option<Entry<K, V>> {
        if key.is_null() {
            return None;
        }

        let kd = key.data();

        // Until we can map an OccupiedEntry to a VacantEntry I don't think
        // there is a way to avoid this extra lookup.
        if let hash_map::Entry::Occupied(o) = self.slots.entry(kd.idx) {
            if o.get().version != kd.version.get() {
                // Which is outdated, our key or the slot?
                if is_older_version(o.get().version, kd.version.get()) {
                    o.remove();
                } else {
                    return None;
                }
            }
        }

        Some(match self.slots.entry(kd.idx) {
            hash_map::Entry::Occupied(inner) => {
                // We know for certain that this entry's key matches ours due
                // to the previous if block.
                Entry::Occupied(OccupiedEntry {
                    inner,
                    kd,
                    _k: PhantomData,
                })
            },
            hash_map::Entry::Vacant(inner) => Entry::Vacant(VacantEntry {
                inner,
                kd,
                _k: PhantomData,
            }),
        })
    }
}

impl<K, V, S> Default for SparseSecondaryMap<K, V, S>
where
    K: Key,
    S: hash::BuildHasher + Default,
{
    fn default() -> Self {
        Self::with_hasher(Default::default())
    }
}

impl<K, V, S> Index<K> for SparseSecondaryMap<K, V, S>
where
    K: Key,
    S: hash::BuildHasher,
{
    type Output = V;

    fn index(&self, key: K) -> &V {
        match self.get(key) {
            Some(r) => r,
            None => panic!("invalid SparseSecondaryMap key used"),
        }
    }
}

impl<K, V, S> IndexMut<K> for SparseSecondaryMap<K, V, S>
where
    K: Key,
    S: hash::BuildHasher,
{
    fn index_mut(&mut self, key: K) -> &mut V {
        match self.get_mut(key) {
            Some(r) => r,
            None => panic!("invalid SparseSecondaryMap key used"),
        }
    }
}

impl<K, V, S> PartialEq for SparseSecondaryMap<K, V, S>
where
    K: Key,
    V: PartialEq,
    S: hash::BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |other_value| *value == *other_value))
    }
}

impl<K, V, S> Eq for SparseSecondaryMap<K, V, S>
where
    K: Key,
    V: Eq,
    S: hash::BuildHasher,
{
}

impl<K, V, S> FromIterator<(K, V)> for SparseSecondaryMap<K, V, S>
where
    K: Key,
    S: hash::BuildHasher + Default,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut sec = Self::default();
        sec.extend(iter);
        sec
    }
}

impl<K, V, S> Extend<(K, V)> for SparseSecondaryMap<K, V, S>
where
    K: Key,
    S: hash::BuildHasher,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<'a, K, V, S> Extend<(K, &'a V)> for SparseSecondaryMap<K, V, S>
where
    K: Key,
    V: 'a + Copy,
    S: hash::BuildHasher,
{
    fn extend<I: IntoIterator<Item = (K, &'a V)>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        for (k, v) in iter {
            self.insert(k, *v);
        }
    }
}

/// A view into a occupied entry in a [`SparseSecondaryMap`]. It is part of the
/// [`Entry`] enum.
#[derive(Debug)]
pub struct OccupiedEntry<'a, K: Key, V> {
    inner: hash_map::OccupiedEntry<'a, u32, Slot<V>>,
    kd: KeyData,
    _k: PhantomData<fn(K) -> K>,
}

/// A view into a vacant entry in a [`SparseSecondaryMap`]. It is part of the
/// [`Entry`] enum.
#[derive(Debug)]
pub struct VacantEntry<'a, K: Key, V> {
    inner: hash_map::VacantEntry<'a, u32, Slot<V>>,
    kd: KeyData,
    _k: PhantomData<fn(K) -> K>,
}

/// A view into a single entry in a [`SparseSecondaryMap`], which may either be
/// vacant or occupied.
///
/// This `enum` is constructed using [`SparseSecondaryMap::entry`].
#[derive(Debug)]
pub enum Entry<'a, K: Key, V> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),

    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K: Key, V> Entry<'a, K, V> {
    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert("poneyland");
    /// let v = sec.entry(k).unwrap().or_insert(10);
    /// assert_eq!(*v, 10);
    /// *sec.entry(k).unwrap().or_insert(1) *= 2;
    /// assert_eq!(sec[k], 20);
    /// ```
    pub fn or_insert(self, default: V) -> &'a mut V {
        self.or_insert_with(|| default)
    }

    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the
    /// entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// let v = sec.entry(k).unwrap().or_insert_with(|| "foobar".to_string());
    /// assert_eq!(v, &"foobar");
    /// ```
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(x) => x.into_mut(),
            Entry::Vacant(x) => x.insert(default()),
        }
    }

    /// Returns this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec: SparseSecondaryMap<_, ()> = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// let entry = sec.entry(k).unwrap();
    /// assert_eq!(entry.key(), k);
    /// ```
    pub fn key(&self) -> K {
        match self {
            Entry::Occupied(entry) => entry.kd.into(),
            Entry::Vacant(entry) => entry.kd.into(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// sec.insert(k, 0);
    /// sec.entry(k).unwrap().and_modify(|x| *x = 1);
    ///
    /// assert_eq!(sec[k], 1)
    /// ```
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            },
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, K: Key, V: Default> Entry<'a, K, V> {
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec: SparseSecondaryMap<_, Option<i32>> = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// sec.entry(k).unwrap().or_default();
    /// assert_eq!(sec[k], None)
    /// ```
    pub fn or_default(self) -> &'a mut V {
        self.or_insert_with(Default::default)
    }
}

impl<'a, K: Key, V> OccupiedEntry<'a, K, V> {
    /// Returns this entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// sec.insert(k, 10);
    /// assert_eq!(sec.entry(k).unwrap().key(), k);
    /// ```
    pub fn key(&self) -> K {
        self.kd.into()
    }

    /// Removes the entry from the slot map and returns the key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::sparse_secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let foo = sm.insert("foo");
    /// sec.entry(foo).unwrap().or_insert("bar");
    ///
    /// if let Some(Entry::Occupied(o)) = sec.entry(foo) {
    ///     assert_eq!(o.remove_entry(), (foo, "bar"));
    /// }
    /// assert_eq!(sec.contains_key(foo), false);
    /// ```
    pub fn remove_entry(self) -> (K, V) {
        (self.kd.into(), self.remove())
    }

    /// Gets a reference to the value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::sparse_secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// sec.insert(k, 10);
    ///
    /// if let Entry::Occupied(o) = sec.entry(k).unwrap() {
    ///     assert_eq!(*o.get(), 10);
    /// }
    /// ```
    pub fn get(&self) -> &V {
        &self.inner.get().value
    }

    /// Gets a mutable reference to the value in the entry.
    ///
    /// If you need a reference to the [`OccupiedEntry`] which may outlive the
    /// destruction of the [`Entry`] value, see [`into_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::sparse_secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// sec.insert(k, 10);
    /// if let Entry::Occupied(mut o) = sec.entry(k).unwrap() {
    ///     *o.get_mut() = 20;
    /// }
    /// assert_eq!(sec[k], 20);
    /// ```
    ///
    /// [`into_mut`]: Self::into_mut
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.inner.get_mut().value
    }

    /// Converts the [`OccupiedEntry`] into a mutable reference to the value in
    /// the entry with a lifetime bound to the map itself.
    ///
    /// If you need multiple references to the [`OccupiedEntry`], see
    /// [`get_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::sparse_secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(0);
    /// sec.insert(k, 0);
    ///
    /// let r;
    /// if let Entry::Occupied(o) = sec.entry(k).unwrap() {
    ///     r = o.into_mut(); // v outlives the entry.
    /// } else {
    ///     r = sm.get_mut(k).unwrap();
    /// }
    /// *r = 1;
    /// assert_eq!((sm[k], sec[k]), (0, 1));
    /// ```
    ///
    /// [`get_mut`]: Self::get_mut
    pub fn into_mut(self) -> &'a mut V {
        &mut self.inner.into_mut().value
    }

    /// Sets the value of the entry, and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::sparse_secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// sec.insert(k, 10);
    ///
    /// if let Entry::Occupied(mut o) = sec.entry(k).unwrap() {
    ///     let v = o.insert(20);
    ///     assert_eq!(v, 10);
    ///     assert_eq!(*o.get(), 20);
    /// }
    /// ```
    pub fn insert(&mut self, value: V) -> V {
        std::mem::replace(self.get_mut(), value)
    }

    /// Takes the value out of the entry, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::sparse_secondary::Entry;
    ///
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// sec.insert(k, 10);
    ///
    /// if let Entry::Occupied(mut o) = sec.entry(k).unwrap() {
    ///     assert_eq!(o.remove(), 10);
    ///     assert_eq!(sec.contains_key(k), false);
    /// }
    /// ```
    pub fn remove(self) -> V {
        self.inner.remove().value
    }
}

impl<'a, K: Key, V> VacantEntry<'a, K, V> {
    /// Gets the key that would be used when inserting a value through the
    /// [`VacantEntry`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::sparse_secondary::Entry;
    ///
    /// let mut sm = SlotMap::new();
    /// let mut sec: SparseSecondaryMap<_, ()> = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    ///
    /// if let Entry::Vacant(v) = sec.entry(k).unwrap() {
    ///     assert_eq!(v.key(), k);
    /// }
    /// ```
    pub fn key(&self) -> K {
        self.kd.into()
    }

    /// Sets the value of the entry with the [`VacantEntry`]'s key, and returns
    /// a mutable reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::sparse_secondary::Entry;
    ///
    /// let mut sm = SlotMap::new();
    /// let mut sec = SparseSecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    ///
    /// if let Entry::Vacant(v) = sec.entry(k).unwrap() {
    ///     let new_val = v.insert(3);
    ///     assert_eq!(new_val, &mut 3);
    /// }
    /// ```
    pub fn insert(self, value: V) -> &'a mut V {
        &mut self
            .inner
            .insert(Slot {
                version: self.kd.version.get(),
                value,
            })
            .value
    }
}

// Iterators.
/// A draining iterator for [`SparseSecondaryMap`].
///
/// This iterator is created by [`SparseSecondaryMap::drain`].
#[derive(Debug)]
pub struct Drain<'a, K: Key + 'a, V: 'a> {
    inner: hash_map::Drain<'a, u32, Slot<V>>,
    _k: PhantomData<fn(K) -> K>,
}

/// An iterator that moves key-value pairs out of a [`SparseSecondaryMap`].
///
/// This iterator is created by calling the `into_iter` method on [`SparseSecondaryMap`],
/// provided by the [`IntoIterator`] trait.
#[derive(Debug)]
pub struct IntoIter<K: Key, V> {
    inner: hash_map::IntoIter<u32, Slot<V>>,
    _k: PhantomData<fn(K) -> K>,
}

/// An iterator over the key-value pairs in a [`SparseSecondaryMap`].
///
/// This iterator is created by [`SparseSecondaryMap::iter`].
#[derive(Debug)]
pub struct Iter<'a, K: Key + 'a, V: 'a> {
    inner: hash_map::Iter<'a, u32, Slot<V>>,
    _k: PhantomData<fn(K) -> K>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
            _k: self._k,
        }
    }
}

/// A mutable iterator over the key-value pairs in a [`SparseSecondaryMap`].
///
/// This iterator is created by [`SparseSecondaryMap::iter_mut`].
#[derive(Debug)]
pub struct IterMut<'a, K: Key + 'a, V: 'a> {
    inner: hash_map::IterMut<'a, u32, Slot<V>>,
    _k: PhantomData<fn(K) -> K>,
}

/// An iterator over the keys in a [`SparseSecondaryMap`].
///
/// This iterator is created by [`SparseSecondaryMap::keys`].
#[derive(Debug)]
pub struct Keys<'a, K: Key + 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Self {
        Keys {
            inner: self.inner.clone(),
        }
    }
}

/// An iterator over the values in a [`SparseSecondaryMap`].
///
/// This iterator is created by [`SparseSecondaryMap::values`].
#[derive(Debug)]
pub struct Values<'a, K: Key + 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Values<'a, K, V> {
    fn clone(&self) -> Self {
        Values {
            inner: self.inner.clone(),
        }
    }
}

/// A mutable iterator over the values in a [`SparseSecondaryMap`].
///
/// This iterator is created by [`SparseSecondaryMap::values_mut`].
#[derive(Debug)]
pub struct ValuesMut<'a, K: Key + 'a, V: 'a> {
    inner: IterMut<'a, K, V>,
}

impl<'a, K: Key, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        self.inner.next().map(|(idx, slot)| {
            let key = KeyData::new(idx, slot.version).into();
            (key, slot.value)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: Key, V> Drop for Drain<'a, K, V> {
    fn drop(&mut self) {
        self.for_each(|_drop| {});
    }
}

impl<K: Key, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        self.inner.next().map(|(idx, slot)| {
            let key = KeyData::new(idx, slot.version).into();
            (key, slot.value)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: Key, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<(K, &'a V)> {
        self.inner.next().map(|(&idx, slot)| {
            let key = KeyData::new(idx, slot.version).into();
            (key, &slot.value)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: Key, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<(K, &'a mut V)> {
        self.inner.next().map(|(&idx, slot)| {
            let key = KeyData::new(idx, slot.version).into();
            (key, &mut slot.value)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: Key, V> Iterator for Keys<'a, K, V> {
    type Item = K;

    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(key, _)| key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: Key, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, value)| value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: Key, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, value)| value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V, S> IntoIterator for &'a SparseSecondaryMap<K, V, S>
where
    K: Key,
    S: hash::BuildHasher,
{
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut SparseSecondaryMap<K, V, S>
where
    K: Key,
    S: hash::BuildHasher,
{
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, S> IntoIterator for SparseSecondaryMap<K, V, S>
where
    K: Key,
    S: hash::BuildHasher,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.slots.into_iter(),
            _k: PhantomData,
        }
    }
}

impl<'a, K: Key, V> FusedIterator for Iter<'a, K, V> {}
impl<'a, K: Key, V> FusedIterator for IterMut<'a, K, V> {}
impl<'a, K: Key, V> FusedIterator for Keys<'a, K, V> {}
impl<'a, K: Key, V> FusedIterator for Values<'a, K, V> {}
impl<'a, K: Key, V> FusedIterator for ValuesMut<'a, K, V> {}
impl<'a, K: Key, V> FusedIterator for Drain<'a, K, V> {}
impl<K: Key, V> FusedIterator for IntoIter<K, V> {}

impl<'a, K: Key, V> ExactSizeIterator for Iter<'a, K, V> {}
impl<'a, K: Key, V> ExactSizeIterator for IterMut<'a, K, V> {}
impl<'a, K: Key, V> ExactSizeIterator for Keys<'a, K, V> {}
impl<'a, K: Key, V> ExactSizeIterator for Values<'a, K, V> {}
impl<'a, K: Key, V> ExactSizeIterator for ValuesMut<'a, K, V> {}
impl<'a, K: Key, V> ExactSizeIterator for Drain<'a, K, V> {}
impl<K: Key, V> ExactSizeIterator for IntoIter<K, V> {}

// Serialization with serde.
#[cfg(feature = "serde")]
mod serialize {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::*;
    use crate::SecondaryMap;

    impl<K, V, H> Serialize for SparseSecondaryMap<K, V, H>
    where
        K: Key,
        V: Serialize,
        H: hash::BuildHasher,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut serde_sec = SecondaryMap::new();
            for (k, v) in self {
                serde_sec.insert(k, v);
            }

            serde_sec.serialize(serializer)
        }
    }

    impl<'de, K, V, S> Deserialize<'de> for SparseSecondaryMap<K, V, S>
    where
        K: Key,
        V: Deserialize<'de>,
        S: hash::BuildHasher + Default,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let serde_sec: SecondaryMap<K, V> = Deserialize::deserialize(deserializer)?;
            let mut sec = Self::default();

            for (k, v) in serde_sec {
                sec.insert(k, v);
            }

            Ok(sec)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use quickcheck::quickcheck;

    use crate::*;

    #[test]
    fn custom_hasher() {
        type FastSparseSecondaryMap<K, V> = SparseSecondaryMap<K, V, fxhash::FxBuildHasher>;
        let mut sm = SlotMap::new();
        let mut sec = FastSparseSecondaryMap::default();
        let key1 = sm.insert(42);
        sec.insert(key1, 1234);
        assert_eq!(sec[key1], 1234);
        assert_eq!(sec.len(), 1);
        let sec2 = sec.iter().map(|(k, &v)| (k, v)).collect::<FastSparseSecondaryMap<_, _>>();
        assert_eq!(sec, sec2);
    }

    #[cfg(all(nightly, feature = "unstable"))]
    #[test]
    fn disjoint() {
        // Intended to be run with miri to find any potential UB.
        let mut sm = SlotMap::new();
        let mut sec = SparseSecondaryMap::new();

        // Some churn.
        for i in 0..20usize {
            sm.insert(i);
        }
        sm.retain(|_, i| *i % 2 == 0);

        for (i, k) in sm.keys().enumerate() {
            sec.insert(k, i);
        }

        let keys: Vec<_> = sm.keys().collect();
        for i in 0..keys.len() {
            for j in 0..keys.len() {
                if let Some([r0, r1]) = sec.get_disjoint_mut([keys[i], keys[j]]) {
                    *r0 ^= *r1;
                    *r1 = r1.wrapping_add(*r0);
                } else {
                    assert!(i == j);
                }
            }
        }

        for i in 0..keys.len() {
            for j in 0..keys.len() {
                for k in 0..keys.len() {
                    if let Some([r0, r1, r2]) = sec.get_disjoint_mut([keys[i], keys[j], keys[k]]) {
                        *r0 ^= *r1;
                        *r0 = r0.wrapping_add(*r2);
                        *r1 ^= *r0;
                        *r1 = r1.wrapping_add(*r2);
                        *r2 ^= *r0;
                        *r2 = r2.wrapping_add(*r1);
                    } else {
                        assert!(i == j || j == k || i == k);
                    }
                }
            }
        }
    }

    quickcheck! {
        fn qc_secmap_equiv_hashmap(operations: Vec<(u8, u32)>) -> bool {
            let mut hm = HashMap::new();
            let mut hm_keys = Vec::new();
            let mut unique_key = 0u32;
            let mut sm = SlotMap::new();
            let mut sec = SparseSecondaryMap::new();
            let mut sm_keys = Vec::new();

            #[cfg(not(feature = "serde"))]
            let num_ops = 4;
            #[cfg(feature = "serde")]
            let num_ops = 5;

            for (op, val) in operations {
                match op % num_ops {
                    // Insert.
                    0 => {
                        hm.insert(unique_key, val);
                        hm_keys.push(unique_key);
                        unique_key += 1;

                        let k = sm.insert(val);
                        sec.insert(k, val);
                        sm_keys.push(k);
                    }

                    // Delete.
                    1 => {
                        if hm_keys.is_empty() { continue; }

                        let idx = val as usize % hm_keys.len();
                        sm.remove(sm_keys[idx]);
                        if hm.remove(&hm_keys[idx]) != sec.remove(sm_keys[idx]) {
                            return false;
                        }
                    }

                    // Access.
                    2 => {
                        if hm_keys.is_empty() { continue; }
                        let idx = val as usize % hm_keys.len();
                        let (hm_key, sm_key) = (&hm_keys[idx], sm_keys[idx]);

                        if hm.contains_key(hm_key) != sec.contains_key(sm_key) ||
                           hm.get(hm_key) != sec.get(sm_key) {
                            return false;
                        }
                    }

                    // Clone.
                    3 => {
                        sec = sec.clone();
                    }

                    // Serde round-trip.
                    #[cfg(feature = "serde")]
                    4 => {
                        let ser = serde_json::to_string(&sec).unwrap();
                        sec = serde_json::from_str(&ser).unwrap();
                    }

                    _ => unreachable!(),
                }
            }

            let mut secv: Vec<_> = sec.values().collect();
            let mut hmv: Vec<_> = hm.values().collect();
            secv.sort();
            hmv.sort();
            secv == hmv
        }
    }
}
