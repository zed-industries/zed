//! Contains the secondary map implementation.

#[cfg(all(nightly, any(doc, feature = "unstable")))]
use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::hint::unreachable_unchecked;
use core::iter::{Enumerate, Extend, FromIterator, FusedIterator};
use core::marker::PhantomData;
use core::mem::replace;
#[allow(unused_imports)] // MaybeUninit is only used on nightly at the moment.
use core::mem::MaybeUninit;
use core::num::NonZeroU32;
use core::ops::{Index, IndexMut};

use super::{Key, KeyData};
use crate::util::is_older_version;

// This representation works because we don't have to store the versions
// of removed elements.
#[derive(Debug, Clone)]
enum Slot<T> {
    Occupied { value: T, version: NonZeroU32 },

    Vacant,
}

use self::Slot::{Occupied, Vacant};

impl<T> Slot<T> {
    pub fn new_occupied(version: u32, value: T) -> Self {
        Occupied {
            value,
            version: unsafe { NonZeroU32::new_unchecked(version | 1u32) },
        }
    }

    pub fn new_vacant() -> Self {
        Vacant
    }

    // Is this slot occupied?
    #[inline(always)]
    pub fn occupied(&self) -> bool {
        match self {
            Occupied { .. } => true,
            Vacant => false,
        }
    }

    #[inline(always)]
    pub fn version(&self) -> u32 {
        match self {
            Occupied { version, .. } => version.get(),
            Vacant => 0,
        }
    }

    pub unsafe fn get_unchecked(&self) -> &T {
        match self {
            Occupied { value, .. } => value,
            Vacant => unreachable_unchecked(),
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self) -> &mut T {
        match self {
            Occupied { value, .. } => value,
            Vacant => unreachable_unchecked(),
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            Occupied { value, .. } => Some(value),
            Vacant => None,
        }
    }
}

/// Secondary map, associate data with previously stored elements in a slot map.
///
/// A [`SecondaryMap`] allows you to efficiently store additional information
/// for each element in a slot map. You can have multiple secondary maps per
/// slot map, but not multiple slot maps per secondary map. It is safe but
/// unspecified behavior if you use keys from multiple different slot maps in
/// the same [`SecondaryMap`].
///
/// A [`SecondaryMap`] does not leak memory even if you never remove elements.
/// In return, when you remove a key from the primary slot map, after any insert
/// the space associated with the removed element may be reclaimed. Don't expect
/// the values associated with a removed key to stick around after an insertion
/// has happened!
///
/// Finally a note on memory complexity, the [`SecondaryMap`] can use memory for
/// each slot in the primary slot map, and has to iterate over every slot during
/// iteration, regardless of whether you have inserted an associative value at
/// that key or not. If you have some property that you only expect to set for a
/// minority of keys, use a [`SparseSecondaryMap`](crate::SparseSecondaryMap),
/// which is backed by a [`HashMap`](std::collections::HashMap).
///
/// Example usage:
///
/// ```
/// # use slotmap::*;
/// let mut players = SlotMap::new();
/// let mut health = SecondaryMap::new();
/// let mut ammo = SecondaryMap::new();
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
pub struct SecondaryMap<K: Key, V> {
    slots: Vec<Slot<V>>,
    num_elems: usize,
    _k: PhantomData<fn(K) -> K>,
}

impl<K: Key, V> SecondaryMap<K, V> {
    /// Constructs a new, empty [`SecondaryMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SecondaryMap<DefaultKey, i32> = SecondaryMap::new();
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates an empty [`SecondaryMap`] with the given capacity of slots.
    ///
    /// The secondary map will not reallocate until it holds at least `capacity`
    /// slots. Even inserting a single key-value pair might require as many
    /// slots as the slot map the key comes from, so it's recommended to match
    /// the capacity of a secondary map to its corresponding slot map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm: SlotMap<_, i32> = SlotMap::with_capacity(10);
    /// let mut sec: SecondaryMap<DefaultKey, i32> = SecondaryMap::with_capacity(sm.capacity());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut slots = Vec::with_capacity(capacity + 1); // Sentinel.
        slots.push(Slot::new_vacant());
        Self {
            slots,
            num_elems: 0,
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
    /// let mut squared = SecondaryMap::new();
    /// assert_eq!(squared.len(), 0);
    /// squared.insert(k, 16);
    /// assert_eq!(squared.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.num_elems
    }

    /// Returns if the secondary map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SecondaryMap<DefaultKey, i32> = SecondaryMap::new();
    /// assert!(sec.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.num_elems == 0
    }

    /// Returns the number of elements the [`SecondaryMap`] can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SecondaryMap<DefaultKey, i32> = SecondaryMap::with_capacity(10);
    /// assert!(sec.capacity() >= 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.slots.capacity() - 1 // Sentinel.
    }

    /// Sets the capacity of the [`SecondaryMap`] to `new_capacity`, if it is
    /// bigger than the current capacity.
    ///
    /// It is recommended to set the capacity of a [`SecondaryMap`] to the
    /// capacity of its corresponding slot map before inserting many new
    /// elements to prevent frequent reallocations. The collection may reserve
    /// more space than requested.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SecondaryMap<DefaultKey, i32> = SecondaryMap::with_capacity(10);
    /// assert!(sec.capacity() >= 10);
    /// sec.set_capacity(1000);
    /// assert!(sec.capacity() >= 1000);
    /// ```
    pub fn set_capacity(&mut self, new_capacity: usize) {
        let new_capacity = new_capacity + 1; // Sentinel.
        if new_capacity > self.slots.capacity() {
            let needed = new_capacity - self.slots.len();
            self.slots.reserve(needed);
        }
    }

    /// Tries to set the capacity of the [`SecondaryMap`] to `new_capacity`, if it
    /// is bigger than the current capacity.
    ///
    /// It is recommended to set the capacity of a [`SecondaryMap`] to the
    /// capacity of its corresponding slot map before inserting many new
    /// elements to prevent frequent reallocations. The collection may reserve
    /// more space than requested.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sec: SecondaryMap<DefaultKey, i32> = SecondaryMap::with_capacity(10);
    /// assert!(sec.capacity() >= 10);
    /// sec.try_set_capacity(1000).unwrap();
    /// assert!(sec.capacity() >= 1000);
    /// ```
    #[cfg(all(nightly, any(doc, feature = "unstable")))]
    #[cfg_attr(all(nightly, doc), doc(cfg(feature = "unstable")))]
    pub fn try_set_capacity(&mut self, new_capacity: usize) -> Result<(), TryReserveError> {
        let new_capacity = new_capacity + 1; // Sentinel.
        if new_capacity > self.slots.capacity() {
            let needed = new_capacity - self.slots.len();
            self.slots.try_reserve(needed)
        } else {
            Ok(())
        }
    }

    /// Returns [`true`] if the secondary map contains `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let k = sm.insert(4);
    /// let mut squared = SecondaryMap::new();
    /// assert!(!squared.contains_key(k));
    /// squared.insert(k, 16);
    /// assert!(squared.contains_key(k));
    /// ```
    pub fn contains_key(&self, key: K) -> bool {
        let kd = key.data();
        self.slots
            .get(kd.idx as usize)
            .map_or(false, |slot| slot.version() == kd.version.get())
    }

    /// Inserts a value into the secondary map at the given `key`. Can silently
    /// fail and return `None` if `key` was removed from the originating slot
    /// map.
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
    /// let mut squared = SecondaryMap::new();
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
        self.slots
            .extend((self.slots.len()..=kd.idx as usize).map(|_| Slot::new_vacant()));

        let slot = &mut self.slots[kd.idx as usize];
        if slot.version() == kd.version.get() {
            // Is always occupied.
            return Some(replace(unsafe { slot.get_unchecked_mut() }, value));
        }

        if slot.occupied() {
            // Don't replace existing newer values.
            if is_older_version(kd.version.get(), slot.version()) {
                return None;
            }
        } else {
            self.num_elems += 1;
        }

        *slot = Slot::new_occupied(kd.version.get(), value);
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
    /// let mut squared = SecondaryMap::new();
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
        if let Some(slot) = self.slots.get_mut(kd.idx as usize) {
            if slot.version() == kd.version.get() {
                self.num_elems -= 1;
                return replace(slot, Slot::new_vacant()).into_option();
            }
        }

        None
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all key-value pairs `(k, v)` such that
    /// `f(k, &mut v)` returns false. This method invalidates any removed keys.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
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
    /// assert_eq!(sec.len(), 2);
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(K, &mut V) -> bool,
    {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if let Occupied { value, version } = slot {
                let key = KeyData::new(i as u32, version.get()).into();
                if !f(key, value) {
                    self.num_elems -= 1;
                    *slot = Slot::new_vacant();
                }
            }
        }
    }

    /// Clears the secondary map. Keeps the allocated memory for reuse.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
    /// for i in 0..10 {
    ///     sec.insert(sm.insert(i), i);
    /// }
    /// assert_eq!(sec.len(), 10);
    /// sec.clear();
    /// assert_eq!(sec.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.drain();
    }

    /// Clears the slot map, returning all key-value pairs in arbitrary order as
    /// an iterator. Keeps the allocated memory for reuse.
    ///
    /// When the iterator is dropped all elements in the slot map are removed,
    /// even if the iterator was not fully consumed. If the iterator is not
    /// dropped (using e.g. [`std::mem::forget`]), only the elements that were
    /// iterated over are removed.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::iter::FromIterator;
    /// let mut sm = SlotMap::new();
    /// let k = sm.insert(0);
    /// let mut sec = SecondaryMap::new();
    /// sec.insert(k, 1);
    /// let v: Vec<_> = sec.drain().collect();
    /// assert_eq!(sec.len(), 0);
    /// assert_eq!(v, vec![(k, 1)]);
    /// ```
    pub fn drain(&mut self) -> Drain<K, V> {
        Drain { cur: 1, sm: self }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert("foo");
    /// let mut sec = SecondaryMap::new();
    /// sec.insert(key, "bar");
    /// assert_eq!(sec.get(key), Some(&"bar"));
    /// sec.remove(key);
    /// assert_eq!(sec.get(key), None);
    /// ```
    pub fn get(&self, key: K) -> Option<&V> {
        let kd = key.data();
        self.slots
            .get(kd.idx as usize)
            .filter(|slot| slot.version() == kd.version.get())
            .map(|slot| unsafe { slot.get_unchecked() })
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
    /// let mut sec = SecondaryMap::new();
    /// sec.insert(key, "bar");
    /// assert_eq!(unsafe { sec.get_unchecked(key) }, &"bar");
    /// sec.remove(key);
    /// // sec.get_unchecked(key) is now dangerous!
    /// ```
    pub unsafe fn get_unchecked(&self, key: K) -> &V {
        debug_assert!(self.contains_key(key));
        let slot = self.slots.get_unchecked(key.data().idx as usize);
        slot.get_unchecked()
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert("test");
    /// let mut sec = SecondaryMap::new();
    /// sec.insert(key, 3.5);
    /// if let Some(x) = sec.get_mut(key) {
    ///     *x += 3.0;
    /// }
    /// assert_eq!(sec[key], 6.5);
    /// ```
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let kd = key.data();
        self.slots
            .get_mut(kd.idx as usize)
            .filter(|slot| slot.version() == kd.version.get())
            .map(|slot| unsafe { slot.get_unchecked_mut() })
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
    /// let mut sec = SecondaryMap::new();
    /// sec.insert(key, "bar");
    /// unsafe { *sec.get_unchecked_mut(key) = "baz" };
    /// assert_eq!(sec[key], "baz");
    /// sec.remove(key);
    /// // sec.get_unchecked_mut(key) is now dangerous!
    /// ```
    pub unsafe fn get_unchecked_mut(&mut self, key: K) -> &mut V {
        debug_assert!(self.contains_key(key));
        let slot = self.slots.get_unchecked_mut(key.data().idx as usize);
        slot.get_unchecked_mut()
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
    /// let mut sec = SecondaryMap::new();
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
        let mut slot_versions: [MaybeUninit<u32>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut i = 0;
        while i < N {
            let kd = keys[i].data();

            match self.slots.get_mut(kd.idx as usize) {
                Some(Occupied { version, value }) if *version == kd.version => {
                    // This key is valid, and the slot is occupied. Temporarily
                    // set the version to 2 so duplicate keys would show up as
                    // invalid, since keys always have an odd version. This
                    // gives us a linear time disjointness check.
                    ptrs[i] = MaybeUninit::new(&mut *value);
                    slot_versions[i] = MaybeUninit::new(version.get());
                    *version = NonZeroU32::new(2).unwrap();
                },

                _ => break,
            }

            i += 1;
        }

        // Undo temporary unoccupied markings.
        for j in 0..i {
            let idx = keys[j].data().idx as usize;
            unsafe {
                match self.slots.get_mut(idx) {
                    Some(Occupied { version, .. }) => {
                        *version = NonZeroU32::new_unchecked(slot_versions[j].assume_init());
                    },
                    _ => unreachable_unchecked(),
                }
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
    /// let mut sec = SecondaryMap::new();
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
    /// let mut sec = SecondaryMap::new();
    /// let k0 = sm.insert(0); sec.insert(k0, 10);
    /// let k1 = sm.insert(1); sec.insert(k1, 11);
    /// let k2 = sm.insert(2); sec.insert(k2, 12);
    ///
    /// for (k, v) in sm.iter() {
    ///     println!("key: {:?}, val: {}", k, v);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            num_left: self.num_elems,
            slots: self.slots.iter().enumerate(),
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
    /// let mut sec = SecondaryMap::new();
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
            num_left: self.num_elems,
            slots: self.slots.iter_mut().enumerate(),
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
    /// let mut sec = SecondaryMap::new();
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
    /// let mut sec = SecondaryMap::new();
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
    /// let mut sec = SecondaryMap::new();
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
    /// let mut sec = SecondaryMap::new();
    /// let k = sm.insert(1);
    /// let v = sec.entry(k).unwrap().or_insert(10);
    /// assert_eq!(*v, 10);
    /// ```
    pub fn entry(&mut self, key: K) -> Option<Entry<K, V>> {
        if key.is_null() {
            return None;
        }

        let kd = key.data();

        // Ensure the slot exists so the Entry implementation can safely assume
        // the slot always exists without checking.
        self.slots
            .extend((self.slots.len()..=kd.idx as usize).map(|_| Slot::new_vacant()));

        let slot = unsafe { self.slots.get_unchecked(kd.idx as usize) };
        if kd.version.get() == slot.version() {
            Some(Entry::Occupied(OccupiedEntry {
                map: self,
                kd,
                _k: PhantomData,
            }))
        } else if is_older_version(kd.version.get(), slot.version()) {
            None
        } else {
            Some(Entry::Vacant(VacantEntry {
                map: self,
                kd,
                _k: PhantomData,
            }))
        }
    }
}

impl<K: Key, V> Default for SecondaryMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Key, V> Index<K> for SecondaryMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &V {
        match self.get(key) {
            Some(r) => r,
            None => panic!("invalid SecondaryMap key used"),
        }
    }
}

impl<K: Key, V> IndexMut<K> for SecondaryMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut V {
        match self.get_mut(key) {
            Some(r) => r,
            None => panic!("invalid SecondaryMap key used"),
        }
    }
}

impl<K: Key, V: PartialEq> PartialEq for SecondaryMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |other_value| *value == *other_value))
    }
}

impl<K: Key, V: Eq> Eq for SecondaryMap<K, V> {}

impl<K: Key, V> FromIterator<(K, V)> for SecondaryMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut sec = Self::new();
        sec.extend(iter);
        sec
    }
}

impl<K: Key, V> Extend<(K, V)> for SecondaryMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<'a, K: Key, V: 'a + Copy> Extend<(K, &'a V)> for SecondaryMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, &'a V)>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        for (k, v) in iter {
            self.insert(k, *v);
        }
    }
}

/// A view into a occupied entry in a [`SecondaryMap`]. It is part of the
/// [`Entry`] enum.
#[derive(Debug)]
pub struct OccupiedEntry<'a, K: Key, V> {
    map: &'a mut SecondaryMap<K, V>,
    kd: KeyData,
    _k: PhantomData<fn(K) -> K>,
}

/// A view into a vacant entry in a [`SecondaryMap`]. It is part of the
/// [`Entry`] enum.
#[derive(Debug)]
pub struct VacantEntry<'a, K: Key, V> {
    map: &'a mut SecondaryMap<K, V>,
    kd: KeyData,
    _k: PhantomData<fn(K) -> K>,
}

/// A view into a single entry in a [`SecondaryMap`], which may either be
/// vacant or occupied.
///
/// This `enum` is constructed using [`SecondaryMap::entry`].
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
    /// let mut sec = SecondaryMap::new();
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
    /// let mut sec = SecondaryMap::new();
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
    /// let mut sec: SecondaryMap<_, ()> = SecondaryMap::new();
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
    /// let mut sec = SecondaryMap::new();
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
    /// let mut sec: SecondaryMap<_, Option<i32>> = SecondaryMap::new();
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
    /// let mut sec = SecondaryMap::new();
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
    /// # use slotmap::secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
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
    /// # use slotmap::secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    /// sec.insert(k, 10);
    ///
    /// if let Entry::Occupied(o) = sec.entry(k).unwrap() {
    ///     assert_eq!(*o.get(), 10);
    /// }
    /// ```
    pub fn get(&self) -> &V {
        unsafe { self.map.get_unchecked(self.kd.into()) }
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
    /// # use slotmap::secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
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
        unsafe { self.map.get_unchecked_mut(self.kd.into()) }
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
    /// # use slotmap::secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
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
        unsafe { self.map.get_unchecked_mut(self.kd.into()) }
    }

    /// Sets the value of the entry, and returns the entry's old value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::secondary::Entry;
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
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
        replace(self.get_mut(), value)
    }

    /// Takes the value out of the entry, and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use slotmap::secondary::Entry;
    ///
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
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
        let slot = unsafe { self.map.slots.get_unchecked_mut(self.kd.idx as usize) };
        self.map.num_elems -= 1;
        match replace(slot, Slot::new_vacant()) {
            Occupied { value, .. } => value,
            Vacant => unsafe { unreachable_unchecked() },
        }
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
    /// # use slotmap::secondary::Entry;
    ///
    /// let mut sm = SlotMap::new();
    /// let mut sec: SecondaryMap<_, ()> = SecondaryMap::new();
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
    /// # use slotmap::secondary::Entry;
    ///
    /// let mut sm = SlotMap::new();
    /// let mut sec = SecondaryMap::new();
    ///
    /// let k = sm.insert(1);
    ///
    /// if let Entry::Vacant(v) = sec.entry(k).unwrap() {
    ///     let new_val = v.insert(3);
    ///     assert_eq!(new_val, &mut 3);
    /// }
    /// ```
    pub fn insert(self, value: V) -> &'a mut V {
        let slot = unsafe { self.map.slots.get_unchecked_mut(self.kd.idx as usize) };
        // Despite the slot being considered Vacant for this entry, it might be occupied
        // with an outdated element.
        match replace(slot, Slot::new_occupied(self.kd.version.get(), value)) {
            Occupied { .. } => {},
            Vacant => self.map.num_elems += 1,
        }
        unsafe { slot.get_unchecked_mut() }
    }
}

// Iterators.
/// A draining iterator for [`SecondaryMap`].
///
/// This iterator is created by [`SecondaryMap::drain`].
#[derive(Debug)]
pub struct Drain<'a, K: Key + 'a, V: 'a> {
    sm: &'a mut SecondaryMap<K, V>,
    cur: usize,
}

/// An iterator that moves key-value pairs out of a [`SecondaryMap`].
///
/// This iterator is created by calling the `into_iter` method on [`SecondaryMap`],
/// provided by the [`IntoIterator`] trait.
#[derive(Debug)]
pub struct IntoIter<K: Key, V> {
    num_left: usize,
    slots: Enumerate<alloc::vec::IntoIter<Slot<V>>>,
    _k: PhantomData<fn(K) -> K>,
}

/// An iterator over the key-value pairs in a [`SecondaryMap`].
///
/// This iterator is created by [`SecondaryMap::iter`].
#[derive(Debug)]
pub struct Iter<'a, K: Key + 'a, V: 'a> {
    num_left: usize,
    slots: Enumerate<core::slice::Iter<'a, Slot<V>>>,
    _k: PhantomData<fn(K) -> K>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Self {
        Iter {
            num_left: self.num_left,
            slots: self.slots.clone(),
            _k: self._k,
        }
    }
}

/// A mutable iterator over the key-value pairs in a [`SecondaryMap`].
///
/// This iterator is created by [`SecondaryMap::iter_mut`].
#[derive(Debug)]
pub struct IterMut<'a, K: Key + 'a, V: 'a> {
    num_left: usize,
    slots: Enumerate<core::slice::IterMut<'a, Slot<V>>>,
    _k: PhantomData<fn(K) -> K>,
}

/// An iterator over the keys in a [`SecondaryMap`].
///
/// This iterator is created by [`SecondaryMap::keys`].
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

/// An iterator over the values in a [`SecondaryMap`].
///
/// This iterator is created by [`SecondaryMap::values`].
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

/// A mutable iterator over the values in a [`SecondaryMap`].
///
/// This iterator is created by [`SecondaryMap::values_mut`].
#[derive(Debug)]
pub struct ValuesMut<'a, K: Key + 'a, V: 'a> {
    inner: IterMut<'a, K, V>,
}

impl<'a, K: Key, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        while let Some(slot) = self.sm.slots.get_mut(self.cur) {
            let idx = self.cur;
            self.cur += 1;
            if let Occupied { value, version } = replace(slot, Slot::new_vacant()) {
                self.sm.num_elems -= 1;
                let key = KeyData::new(idx as u32, version.get()).into();
                return Some((key, value));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.sm.len(), Some(self.sm.len()))
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
        while let Some((idx, mut slot)) = self.slots.next() {
            if let Occupied { value, version } = replace(&mut slot, Slot::new_vacant()) {
                self.num_left -= 1;
                let key = KeyData::new(idx as u32, version.get()).into();
                return Some((key, value));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num_left, Some(self.num_left))
    }
}

impl<'a, K: Key, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<(K, &'a V)> {
        while let Some((idx, slot)) = self.slots.next() {
            if let Occupied { value, version } = slot {
                self.num_left -= 1;
                let key = KeyData::new(idx as u32, version.get()).into();
                return Some((key, value));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num_left, Some(self.num_left))
    }
}

impl<'a, K: Key, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<(K, &'a mut V)> {
        while let Some((idx, slot)) = self.slots.next() {
            if let Occupied { value, version } = slot {
                let key = KeyData::new(idx as u32, version.get()).into();
                self.num_left -= 1;
                return Some((key, value));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num_left, Some(self.num_left))
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

impl<'a, K: Key, V> IntoIterator for &'a SecondaryMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: Key, V> IntoIterator for &'a mut SecondaryMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: Key, V> IntoIterator for SecondaryMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        let mut it = self.slots.into_iter().enumerate();
        it.next(); // Skip sentinel.
        IntoIter {
            num_left: len,
            slots: it,
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
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct SerdeSlot<T> {
        value: Option<T>,
        version: u32,
    }

    impl<T: Serialize> Serialize for Slot<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let serde_slot = SerdeSlot {
                version: self.version(),
                value: match self {
                    Occupied { value, .. } => Some(value),
                    Vacant => None,
                },
            };
            serde_slot.serialize(serializer)
        }
    }

    impl<'de, T> Deserialize<'de> for Slot<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let serde_slot: SerdeSlot<T> = Deserialize::deserialize(deserializer)?;
            let occupied = serde_slot.version % 2 == 1;
            if occupied ^ serde_slot.value.is_some() {
                return Err(de::Error::custom(&"inconsistent occupation in Slot"));
            }

            Ok(match serde_slot.value {
                Some(value) => Self::new_occupied(serde_slot.version, value),
                None => Self::new_vacant(),
            })
        }
    }

    impl<K: Key, V: Serialize> Serialize for SecondaryMap<K, V> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.slots.serialize(serializer)
        }
    }

    impl<'de, K: Key, V: Deserialize<'de>> Deserialize<'de> for SecondaryMap<K, V> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let mut slots: Vec<Slot<V>> = Deserialize::deserialize(deserializer)?;
            if slots.len() >= (u32::max_value() - 1) as usize {
                return Err(de::Error::custom(&"too many slots"));
            }

            // Ensure the first slot exists and is empty for the sentinel.
            if slots.get(0).map_or(true, |slot| slot.occupied()) {
                return Err(de::Error::custom(&"first slot not empty"));
            }

            slots[0] = Slot::new_vacant();
            let num_elems = slots.iter().map(|s| s.occupied() as usize).sum();

            Ok(Self {
                num_elems,
                slots,
                _k: PhantomData,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use quickcheck::quickcheck;

    use crate::*;

    #[cfg(all(nightly, feature = "unstable"))]
    #[test]
    fn disjoint() {
        // Intended to be run with miri to find any potential UB.
        let mut sm = SlotMap::new();
        let mut sec = SecondaryMap::new();

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
            let mut sec = SecondaryMap::new();
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
