//! Contains the dense slot map implementation.

// There is quite a lot of unsafe code in this implementation. To prevent the
// same explanation over and over again, care must be taken that indices in
// slots and keys from key-value pairs **that are stored inside the slot map**
// are valid. Keys that are received from the user are not trusted (as they
// might have come from a different slot map or malicious serde deseralization).

#[cfg(all(nightly, any(doc, feature = "unstable")))]
use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::iter::FusedIterator;
#[allow(unused_imports)] // MaybeUninit is only used on nightly at the moment.
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut};

use crate::util::{Never, UnwrapUnchecked};
use crate::{DefaultKey, Key, KeyData};

// A slot, which represents storage for an index and a current version.
// Can be occupied or vacant.
#[derive(Debug, Clone)]
struct Slot {
    // Even = vacant, odd = occupied.
    version: u32,

    // An index when occupied, the next free slot otherwise.
    idx_or_free: u32,
}

/// Dense slot map, storage with stable unique keys.
///
/// See [crate documentation](crate) for more details.
#[derive(Debug)]
pub struct DenseSlotMap<K: Key, V> {
    keys: Vec<K>,
    values: Vec<V>,
    slots: Vec<Slot>,
    free_head: u32,
}

impl<V> DenseSlotMap<DefaultKey, V> {
    /// Construct a new, empty [`DenseSlotMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm: DenseSlotMap<_, i32> = DenseSlotMap::new();
    /// ```
    pub fn new() -> Self {
        Self::with_capacity_and_key(0)
    }

    /// Creates an empty [`DenseSlotMap`] with the given capacity.
    ///
    /// The slot map will not reallocate until it holds at least `capacity`
    /// elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm: DenseSlotMap<_, i32> = DenseSlotMap::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> DenseSlotMap<DefaultKey, V> {
        Self::with_capacity_and_key(capacity)
    }
}

impl<K: Key, V> DenseSlotMap<K, V> {
    /// Constructs a new, empty [`DenseSlotMap`] with a custom key type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// new_key_type! {
    ///     struct PositionKey;
    /// }
    /// let mut positions: DenseSlotMap<PositionKey, i32> = DenseSlotMap::with_key();
    /// ```
    pub fn with_key() -> Self {
        Self::with_capacity_and_key(0)
    }

    /// Creates an empty [`DenseSlotMap`] with the given capacity and a custom key
    /// type.
    ///
    /// The slot map will not reallocate until it holds at least `capacity`
    /// elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// new_key_type! {
    ///     struct MessageKey;
    /// }
    /// let mut messages = DenseSlotMap::with_capacity_and_key(3);
    /// let welcome: MessageKey = messages.insert("Welcome");
    /// let good_day = messages.insert("Good day");
    /// let hello = messages.insert("Hello");
    /// ```
    pub fn with_capacity_and_key(capacity: usize) -> Self {
        // Create slots with a sentinel at index 0.
        // We don't actually use the sentinel for anything currently, but
        // HopSlotMap does, and if we want keys to remain valid through
        // conversion we have to have one as well.
        let mut slots = Vec::with_capacity(capacity + 1);
        slots.push(Slot {
            idx_or_free: 0,
            version: 0,
        });

        DenseSlotMap {
            keys: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            slots,
            free_head: 1,
        }
    }

    /// Returns the number of elements in the slot map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::with_capacity(10);
    /// sm.insert("len() counts actual elements, not capacity");
    /// let key = sm.insert("removed elements don't count either");
    /// sm.remove(key);
    /// assert_eq!(sm.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Returns if the slot map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert("dummy");
    /// assert_eq!(sm.is_empty(), false);
    /// sm.remove(key);
    /// assert_eq!(sm.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Returns the number of elements the [`DenseSlotMap`] can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let sm: DenseSlotMap<_, f64> = DenseSlotMap::with_capacity(10);
    /// assert_eq!(sm.capacity(), 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.keys.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the [`DenseSlotMap`]. The collection may reserve more space to
    /// avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// sm.insert("foo");
    /// sm.reserve(32);
    /// assert!(sm.capacity() >= 33);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.keys.reserve(additional);
        self.values.reserve(additional);
        // One slot is reserved for the sentinel.
        let needed = (self.len() + additional).saturating_sub(self.slots.len() - 1);
        self.slots.reserve(needed);
    }

    /// Tries to reserve capacity for at least `additional` more elements to be
    /// inserted in the [`DenseSlotMap`]. The collection may reserve more space to
    /// avoid frequent reallocations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// sm.insert("foo");
    /// sm.try_reserve(32).unwrap();
    /// assert!(sm.capacity() >= 33);
    /// ```
    #[cfg(all(nightly, any(doc, feature = "unstable")))]
    #[cfg_attr(all(nightly, doc), doc(cfg(feature = "unstable")))]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.keys.try_reserve(additional)?;
        self.values.try_reserve(additional)?;
        // One slot is reserved for the sentinel.
        let needed = (self.len() + additional).saturating_sub(self.slots.len() - 1);
        self.slots.try_reserve(needed)
    }

    /// Returns [`true`] if the slot map contains `key`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert(42);
    /// assert_eq!(sm.contains_key(key), true);
    /// sm.remove(key);
    /// assert_eq!(sm.contains_key(key), false);
    /// ```
    pub fn contains_key(&self, key: K) -> bool {
        let kd = key.data();
        self.slots
            .get(kd.idx as usize)
            .map_or(false, |slot| slot.version == kd.version.get())
    }

    /// Inserts a value into the slot map. Returns a unique key that can be used
    /// to access this value.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the slot map equals
    /// 2<sup>32</sup> - 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert(42);
    /// assert_eq!(sm[key], 42);
    /// ```
    #[inline(always)]
    pub fn insert(&mut self, value: V) -> K {
        unsafe { self.try_insert_with_key::<_, Never>(move |_| Ok(value)).unwrap_unchecked_() }
    }

    /// Inserts a value given by `f` into the slot map. The key where the
    /// value will be stored is passed into `f`. This is useful to store values
    /// that contain their own key.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the slot map equals
    /// 2<sup>32</sup> - 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert_with_key(|k| (k, 20));
    /// assert_eq!(sm[key], (key, 20));
    /// ```
    #[inline(always)]
    pub fn insert_with_key<F>(&mut self, f: F) -> K
    where
        F: FnOnce(K) -> V,
    {
        unsafe { self.try_insert_with_key::<_, Never>(move |k| Ok(f(k))).unwrap_unchecked_() }
    }

    /// Inserts a value given by `f` into the slot map. The key where the
    /// value will be stored is passed into `f`. This is useful to store values
    /// that contain their own key.
    ///
    /// If `f` returns `Err`, this method returns the error. The slotmap is untouched.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the slot map equals
    /// 2<sup>32</sup> - 2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.try_insert_with_key::<_, ()>(|k| Ok((k, 20))).unwrap();
    /// assert_eq!(sm[key], (key, 20));
    ///
    /// sm.try_insert_with_key::<_, ()>(|k| Err(())).unwrap_err();
    /// ```
    pub fn try_insert_with_key<F, E>(&mut self, f: F) -> Result<K, E>
    where
        F: FnOnce(K) -> Result<V, E>,
    {
        if self.len() >= (core::u32::MAX - 1) as usize {
            panic!("DenseSlotMap number of elements overflow");
        }

        let idx = self.free_head;

        if let Some(slot) = self.slots.get_mut(idx as usize) {
            let occupied_version = slot.version | 1;
            let key = KeyData::new(idx, occupied_version).into();

            // Push value before adjusting slots/freelist in case f panics or returns an error.
            self.values.push(f(key)?);
            self.keys.push(key);
            self.free_head = slot.idx_or_free;
            slot.idx_or_free = self.keys.len() as u32 - 1;
            slot.version = occupied_version;
            return Ok(key);
        }

        // Push value before adjusting slots/freelist in case f panics or returns an error.
        let key = KeyData::new(idx, 1).into();
        self.values.push(f(key)?);
        self.keys.push(key);
        self.slots.push(Slot {
            version: 1,
            idx_or_free: self.keys.len() as u32 - 1,
        });
        self.free_head = self.slots.len() as u32;
        Ok(key)
    }

    // Helper function to add a slot to the freelist. Returns the index that
    // was stored in the slot.
    #[inline(always)]
    fn free_slot(&mut self, slot_idx: usize) -> u32 {
        let slot = &mut self.slots[slot_idx];
        let value_idx = slot.idx_or_free;
        slot.version = slot.version.wrapping_add(1);
        slot.idx_or_free = self.free_head;
        self.free_head = slot_idx as u32;
        value_idx
    }

    // Helper function to remove a value from a slot and make the slot free.
    // Returns the value removed.
    #[inline(always)]
    fn remove_from_slot(&mut self, slot_idx: usize) -> V {
        let value_idx = self.free_slot(slot_idx);

        // Remove values/slot_indices by swapping to end.
        let _ = self.keys.swap_remove(value_idx as usize);
        let value = self.values.swap_remove(value_idx as usize);

        // Did something take our place? Update its slot to new position.
        if let Some(k) = self.keys.get(value_idx as usize) {
            self.slots[k.data().idx as usize].idx_or_free = value_idx;
        }

        value
    }

    /// Removes a key from the slot map, returning the value at the key if the
    /// key was not previously removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert(42);
    /// assert_eq!(sm.remove(key), Some(42));
    /// assert_eq!(sm.remove(key), None);
    /// ```
    pub fn remove(&mut self, key: K) -> Option<V> {
        let kd = key.data();
        if self.contains_key(kd.into()) {
            Some(self.remove_from_slot(kd.idx as usize))
        } else {
            None
        }
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
    /// let mut sm = DenseSlotMap::new();
    ///
    /// let k3 = sm.insert(2);
    /// let k1 = sm.insert(0);
    /// let k2 = sm.insert(1);
    ///
    /// sm.retain(|key, val| key == k1 || *val == 1);
    ///
    /// assert!(sm.contains_key(k1));
    /// assert!(sm.contains_key(k2));
    /// assert!(!sm.contains_key(k3));
    ///
    /// assert_eq!(2, sm.len());
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(K, &mut V) -> bool,
    {
        let mut i = 0;
        while i < self.keys.len() {
            let (should_keep, slot_idx) = {
                let (kd, mut value) = (self.keys[i].data(), &mut self.values[i]);
                (f(kd.into(), &mut value), kd.idx as usize)
            };

            if should_keep {
                i += 1;
            } else {
                // We do not increment i here intentionally. This index has just
                // been replaced with a new value.
                self.remove_from_slot(slot_idx);
            }
        }
    }

    /// Clears the slot map. Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// for i in 0..10 {
    ///     sm.insert(i);
    /// }
    /// assert_eq!(sm.len(), 10);
    /// sm.clear();
    /// assert_eq!(sm.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.drain();
    }

    /// Clears the slot map, returning all key-value pairs in arbitrary order
    /// as an iterator. Keeps the allocated memory for reuse.
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
    /// let mut sm = DenseSlotMap::new();
    /// let k = sm.insert(0);
    /// let v: Vec<_> = sm.drain().collect();
    /// assert_eq!(sm.len(), 0);
    /// assert_eq!(v, vec![(k, 0)]);
    /// ```
    pub fn drain(&mut self) -> Drain<K, V> {
        Drain { sm: self }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert("bar");
    /// assert_eq!(sm.get(key), Some(&"bar"));
    /// sm.remove(key);
    /// assert_eq!(sm.get(key), None);
    /// ```
    pub fn get(&self, key: K) -> Option<&V> {
        let kd = key.data();
        self.slots
            .get(kd.idx as usize)
            .filter(|slot| slot.version == kd.version.get())
            .map(|slot| unsafe {
                // This is safe because we only store valid indices.
                let idx = slot.idx_or_free as usize;
                self.values.get_unchecked(idx)
            })
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
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert("bar");
    /// assert_eq!(unsafe { sm.get_unchecked(key) }, &"bar");
    /// sm.remove(key);
    /// // sm.get_unchecked(key) is now dangerous!
    /// ```
    pub unsafe fn get_unchecked(&self, key: K) -> &V {
        debug_assert!(self.contains_key(key));
        let idx = self.slots.get_unchecked(key.data().idx as usize).idx_or_free;
        &self.values.get_unchecked(idx as usize)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert(3.5);
    /// if let Some(x) = sm.get_mut(key) {
    ///     *x += 3.0;
    /// }
    /// assert_eq!(sm[key], 6.5);
    /// ```
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let kd = key.data();
        self.slots
            .get(kd.idx as usize)
            .filter(|slot| slot.version == kd.version.get())
            .map(|slot| slot.idx_or_free as usize)
            .map(move |idx| unsafe {
                // This is safe because we only store valid indices.
                self.values.get_unchecked_mut(idx)
            })
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
    /// let mut sm = DenseSlotMap::new();
    /// let key = sm.insert("foo");
    /// unsafe { *sm.get_unchecked_mut(key) = "bar" };
    /// assert_eq!(sm[key], "bar");
    /// sm.remove(key);
    /// // sm.get_unchecked_mut(key) is now dangerous!
    /// ```
    pub unsafe fn get_unchecked_mut(&mut self, key: K) -> &mut V {
        debug_assert!(self.contains_key(key));
        let idx = self.slots.get_unchecked(key.data().idx as usize).idx_or_free;
        self.values.get_unchecked_mut(idx as usize)
    }

    /// Returns mutable references to the values corresponding to the given
    /// keys. All keys must be valid and disjoint, otherwise [`None`] is
    /// returned.
    ///
    /// Requires at least stable Rust version 1.51.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let ka = sm.insert("butter");
    /// let kb = sm.insert("apples");
    /// let kc = sm.insert("charlie");
    /// sm.remove(kc); // Make key c invalid.
    /// assert_eq!(sm.get_disjoint_mut([ka, kb, kc]), None); // Has invalid key.
    /// assert_eq!(sm.get_disjoint_mut([ka, ka]), None); // Not disjoint.
    /// let [a, b] = sm.get_disjoint_mut([ka, kb]).unwrap();
    /// std::mem::swap(a, b);
    /// assert_eq!(sm[ka], "apples");
    /// assert_eq!(sm[kb], "butter");
    /// ```
    #[cfg(has_min_const_generics)]
    pub fn get_disjoint_mut<const N: usize>(&mut self, keys: [K; N]) -> Option<[&mut V; N]> {
        // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
        // safe because the type we are claiming to have initialized here is a
        // bunch of `MaybeUninit`s, which do not require initialization.
        let mut ptrs: [MaybeUninit<*mut V>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut i = 0;
        while i < N {
            // We can avoid this clone after min_const_generics and array_map.
            let kd = keys[i].data();
            if !self.contains_key(kd.into()) {
                break;
            }

            // This key is valid, and thus the slot is occupied. Temporarily
            // mark it as unoccupied so duplicate keys would show up as invalid.
            // This gives us a linear time disjointness check.
            unsafe {
                let slot = self.slots.get_unchecked_mut(kd.idx as usize);
                slot.version ^= 1;
                let ptr = self.values.get_unchecked_mut(slot.idx_or_free as usize);
                ptrs[i] = MaybeUninit::new(ptr);
            }
            i += 1;
        }

        // Undo temporary unoccupied markings.
        for k in &keys[..i] {
            let idx = k.data().idx as usize;
            unsafe {
                self.slots.get_unchecked_mut(idx).version ^= 1;
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
    /// let mut sm = DenseSlotMap::new();
    /// let ka = sm.insert("butter");
    /// let kb = sm.insert("apples");
    /// let [a, b] = unsafe { sm.get_disjoint_unchecked_mut([ka, kb]) };
    /// std::mem::swap(a, b);
    /// assert_eq!(sm[ka], "apples");
    /// assert_eq!(sm[kb], "butter");
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
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let k0 = sm.insert(0);
    /// let k1 = sm.insert(1);
    /// let k2 = sm.insert(2);
    ///
    /// let mut it = sm.iter();
    /// for (k, v) in sm.iter() {
    ///     println!("key: {:?}, val: {}", k, v);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            inner_keys: self.keys.iter(),
            inner_values: self.values.iter(),
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order, with
    /// mutable references to the values. The iterator element type is
    /// `(K, &'a mut V)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = DenseSlotMap::new();
    /// let k0 = sm.insert(10);
    /// let k1 = sm.insert(20);
    /// let k2 = sm.insert(30);
    ///
    /// for (k, v) in sm.iter_mut() {
    ///     if k != k1 {
    ///         *v *= -1;
    ///     }
    /// }
    ///
    /// assert_eq!(sm[k0], -10);
    /// assert_eq!(sm[k1], 20);
    /// assert_eq!(sm[k2], -30);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            inner_keys: self.keys.iter(),
            inner_values: self.values.iter_mut(),
        }
    }

    /// An iterator visiting all keys in arbitrary order. The iterator element
    /// type is K.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::collections::HashSet;
    /// let mut sm = DenseSlotMap::new();
    /// let k0 = sm.insert(10);
    /// let k1 = sm.insert(20);
    /// let k2 = sm.insert(30);
    /// let keys: HashSet<_> = sm.keys().collect();
    /// let check: HashSet<_> = vec![k0, k1, k2].into_iter().collect();
    /// assert_eq!(keys, check);
    /// ```
    pub fn keys(&self) -> Keys<K, V> {
        Keys { inner: self.iter() }
    }

    /// An iterator visiting all values in arbitrary order. The iterator element
    /// type is `&'a V`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::collections::HashSet;
    /// let mut sm = DenseSlotMap::new();
    /// let k0 = sm.insert(10);
    /// let k1 = sm.insert(20);
    /// let k2 = sm.insert(30);
    /// let values: HashSet<_> = sm.values().collect();
    /// let check: HashSet<_> = vec![&10, &20, &30].into_iter().collect();
    /// assert_eq!(values, check);
    /// ```
    pub fn values(&self) -> Values<K, V> {
        Values { inner: self.iter() }
    }

    /// An iterator visiting all values mutably in arbitrary order. The iterator
    /// element type is `&'a mut V`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::collections::HashSet;
    /// let mut sm = DenseSlotMap::new();
    /// sm.insert(1);
    /// sm.insert(2);
    /// sm.insert(3);
    /// sm.values_mut().for_each(|n| { *n *= 3 });
    /// let values: HashSet<_> = sm.into_iter().map(|(_k, v)| v).collect();
    /// let check: HashSet<_> = vec![3, 6, 9].into_iter().collect();
    /// assert_eq!(values, check);
    /// ```
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }
}

impl<K: Key, V> Clone for DenseSlotMap<K, V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
            values: self.values.clone(),
            slots: self.slots.clone(),
            ..*self
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.keys.clone_from(&source.keys);
        self.values.clone_from(&source.values);
        self.slots.clone_from(&source.slots);
        self.free_head = source.free_head;
    }
}

impl<K: Key, V> Default for DenseSlotMap<K, V> {
    fn default() -> Self {
        Self::with_key()
    }
}

impl<K: Key, V> Index<K> for DenseSlotMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &V {
        match self.get(key) {
            Some(r) => r,
            None => panic!("invalid DenseSlotMap key used"),
        }
    }
}

impl<K: Key, V> IndexMut<K> for DenseSlotMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut V {
        match self.get_mut(key) {
            Some(r) => r,
            None => panic!("invalid DenseSlotMap key used"),
        }
    }
}

// Iterators.
/// A draining iterator for [`DenseSlotMap`].
///
/// This iterator is created by [`DenseSlotMap::drain`].
#[derive(Debug)]
pub struct Drain<'a, K: 'a + Key, V: 'a> {
    sm: &'a mut DenseSlotMap<K, V>,
}

/// An iterator that moves key-value pairs out of a [`DenseSlotMap`].
///
/// This iterator is created by calling the `into_iter` method on [`DenseSlotMap`],
/// provided by the [`IntoIterator`] trait.
#[derive(Debug, Clone)]
pub struct IntoIter<K, V> {
    inner_keys: alloc::vec::IntoIter<K>,
    inner_values: alloc::vec::IntoIter<V>,
}

/// An iterator over the key-value pairs in a [`DenseSlotMap`].
///
/// This iterator is created by [`DenseSlotMap::iter`].
#[derive(Debug)]
pub struct Iter<'a, K: 'a + Key, V: 'a> {
    inner_keys: core::slice::Iter<'a, K>,
    inner_values: core::slice::Iter<'a, V>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Self {
        Iter {
            inner_keys: self.inner_keys.clone(),
            inner_values: self.inner_values.clone(),
        }
    }
}

/// A mutable iterator over the key-value pairs in a [`DenseSlotMap`].
///
/// This iterator is created by [`DenseSlotMap::iter_mut`].
#[derive(Debug)]
pub struct IterMut<'a, K: 'a + Key, V: 'a> {
    inner_keys: core::slice::Iter<'a, K>,
    inner_values: core::slice::IterMut<'a, V>,
}

/// An iterator over the keys in a [`DenseSlotMap`].
///
/// This iterator is created by [`DenseSlotMap::keys`].
#[derive(Debug)]
pub struct Keys<'a, K: 'a + Key, V> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Self {
        Keys {
            inner: self.inner.clone(),
        }
    }
}

/// An iterator over the values in a [`DenseSlotMap`].
///
/// This iterator is created by [`DenseSlotMap::values`].
#[derive(Debug)]
pub struct Values<'a, K: 'a + Key, V> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Values<'a, K, V> {
    fn clone(&self) -> Self {
        Values {
            inner: self.inner.clone(),
        }
    }
}

/// A mutable iterator over the values in a [`DenseSlotMap`].
///
/// This iterator is created by [`DenseSlotMap::values_mut`].
#[derive(Debug)]
pub struct ValuesMut<'a, K: 'a + Key, V: 'a> {
    inner: IterMut<'a, K, V>,
}

impl<'a, K: Key, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        // We make no iteration order guarantees, so we just repeatedly pop.
        let key = self.sm.keys.pop();
        let value = self.sm.values.pop();

        if let (Some(k), Some(v)) = (key, value) {
            self.sm.free_slot(k.data().idx as usize);
            Some((k, v))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.sm.keys.len();
        (len, Some(len))
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
        let key = self.inner_keys.next();
        let value = self.inner_values.next();

        if let (Some(k), Some(v)) = (key, value) {
            Some((k, v))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner_keys.size_hint()
    }
}

impl<'a, K: 'a + Key, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<(K, &'a V)> {
        let key = self.inner_keys.next();
        let value = self.inner_values.next();

        if let (Some(k), Some(v)) = (key, value) {
            Some((*k, v))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner_keys.size_hint()
    }
}

impl<'a, K: 'a + Key, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<(K, &'a mut V)> {
        let key = self.inner_keys.next();
        let value = self.inner_values.next();

        if let (Some(k), Some(v)) = (key, value) {
            Some((*k, v))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner_keys.size_hint()
    }
}

impl<'a, K: 'a + Key, V> Iterator for Keys<'a, K, V> {
    type Item = K;

    fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(key, _)| key)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: 'a + Key, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, value)| value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: 'a + Key, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        self.inner.next().map(|(_, value)| value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K: 'a + Key, V> IntoIterator for &'a DenseSlotMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: 'a + Key, V> IntoIterator for &'a mut DenseSlotMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: Key, V> IntoIterator for DenseSlotMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner_keys: self.keys.into_iter(),
            inner_values: self.values.into_iter(),
        }
    }
}

impl<'a, K: 'a + Key, V> FusedIterator for Iter<'a, K, V> {}
impl<'a, K: 'a + Key, V> FusedIterator for IterMut<'a, K, V> {}
impl<'a, K: 'a + Key, V> FusedIterator for Keys<'a, K, V> {}
impl<'a, K: 'a + Key, V> FusedIterator for Values<'a, K, V> {}
impl<'a, K: 'a + Key, V> FusedIterator for ValuesMut<'a, K, V> {}
impl<'a, K: 'a + Key, V> FusedIterator for Drain<'a, K, V> {}
impl<K: Key, V> FusedIterator for IntoIter<K, V> {}

impl<'a, K: 'a + Key, V> ExactSizeIterator for Iter<'a, K, V> {}
impl<'a, K: 'a + Key, V> ExactSizeIterator for IterMut<'a, K, V> {}
impl<'a, K: 'a + Key, V> ExactSizeIterator for Keys<'a, K, V> {}
impl<'a, K: 'a + Key, V> ExactSizeIterator for Values<'a, K, V> {}
impl<'a, K: 'a + Key, V> ExactSizeIterator for ValuesMut<'a, K, V> {}
impl<'a, K: 'a + Key, V> ExactSizeIterator for Drain<'a, K, V> {}
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

    impl<K: Key, V: Serialize> Serialize for DenseSlotMap<K, V> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let serde_slots: Vec<_> = self
                .slots
                .iter()
                .map(|slot| SerdeSlot {
                    value: if slot.version % 2 == 1 {
                        self.values.get(slot.idx_or_free as usize)
                    } else {
                        None
                    },
                    version: slot.version,
                })
                .collect();
            serde_slots.serialize(serializer)
        }
    }

    impl<'de, K: Key, V: Deserialize<'de>> Deserialize<'de> for DenseSlotMap<K, V> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let serde_slots: Vec<SerdeSlot<V>> = Deserialize::deserialize(deserializer)?;
            if serde_slots.len() >= u32::max_value() as usize {
                return Err(de::Error::custom(&"too many slots"));
            }

            // Ensure the first slot exists and is empty for the sentinel.
            if serde_slots.get(0).map_or(true, |slot| slot.version % 2 == 1) {
                return Err(de::Error::custom(&"first slot not empty"));
            }

            // Rebuild slots, key and values.
            let mut keys = Vec::new();
            let mut values = Vec::new();
            let mut slots = Vec::new();
            slots.push(Slot {
                idx_or_free: 0,
                version: 0,
            });

            let mut next_free = serde_slots.len();
            for (i, serde_slot) in serde_slots.into_iter().enumerate().skip(1) {
                let occupied = serde_slot.version % 2 == 1;
                if occupied ^ serde_slot.value.is_some() {
                    return Err(de::Error::custom(&"inconsistent occupation in Slot"));
                }

                if let Some(value) = serde_slot.value {
                    let kd = KeyData::new(i as u32, serde_slot.version);
                    keys.push(kd.into());
                    values.push(value);
                    slots.push(Slot {
                        version: serde_slot.version,
                        idx_or_free: (keys.len() - 1) as u32,
                    });
                } else {
                    slots.push(Slot {
                        version: serde_slot.version,
                        idx_or_free: next_free as u32,
                    });
                    next_free = i;
                }
            }

            Ok(DenseSlotMap {
                keys,
                values,
                slots,
                free_head: next_free as u32,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use quickcheck::quickcheck;

    use super::*;

    #[derive(Clone)]
    struct CountDrop<'a>(&'a core::cell::RefCell<usize>);

    impl<'a> Drop for CountDrop<'a> {
        fn drop(&mut self) {
            *self.0.borrow_mut() += 1;
        }
    }

    #[test]
    fn check_drops() {
        let drops = core::cell::RefCell::new(0usize);

        {
            let mut clone = {
                // Insert 1000 items.
                let mut sm = DenseSlotMap::new();
                let mut sm_keys = Vec::new();
                for _ in 0..1000 {
                    sm_keys.push(sm.insert(CountDrop(&drops)));
                }

                // Remove even keys.
                for i in (0..1000).filter(|i| i % 2 == 0) {
                    sm.remove(sm_keys[i]);
                }

                // Should only have dropped 500 so far.
                assert_eq!(*drops.borrow(), 500);

                // Let's clone ourselves and then die.
                sm.clone()
            };

            // Now all original items should have been dropped exactly once.
            assert_eq!(*drops.borrow(), 1000);

            // Re-use some empty slots.
            for _ in 0..250 {
                clone.insert(CountDrop(&drops));
            }
        }

        // 1000 + 750 drops in total should have happened.
        assert_eq!(*drops.borrow(), 1750);
    }

    #[cfg(all(nightly, feature = "unstable"))]
    #[test]
    fn disjoint() {
        // Intended to be run with miri to find any potential UB.
        let mut sm = DenseSlotMap::new();

        // Some churn.
        for i in 0..20usize {
            sm.insert(i);
        }
        sm.retain(|_, i| *i % 2 == 0);

        let keys: Vec<_> = sm.keys().collect();
        for i in 0..keys.len() {
            for j in 0..keys.len() {
                if let Some([r0, r1]) = sm.get_disjoint_mut([keys[i], keys[j]]) {
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
                    if let Some([r0, r1, r2]) = sm.get_disjoint_mut([keys[i], keys[j], keys[k]]) {
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
        fn qc_slotmap_equiv_hashmap(operations: Vec<(u8, u32)>) -> bool {
            let mut hm = HashMap::new();
            let mut hm_keys = Vec::new();
            let mut unique_key = 0u32;
            let mut sm = DenseSlotMap::new();
            let mut sm_keys = Vec::new();

            #[cfg(not(feature = "serde"))]
            let num_ops = 3;
            #[cfg(feature = "serde")]
            let num_ops = 4;

            for (op, val) in operations {
                match op % num_ops {
                    // Insert.
                    0 => {
                        hm.insert(unique_key, val);
                        hm_keys.push(unique_key);
                        unique_key += 1;

                        sm_keys.push(sm.insert(val));
                    }

                    // Delete.
                    1 => {
                        // 10% of the time test clear.
                        if val % 10 == 0 {
                            let hmvals: HashSet<_> = hm.drain().map(|(_, v)| v).collect();
                            let smvals: HashSet<_> = sm.drain().map(|(_, v)| v).collect();
                            if hmvals != smvals {
                                return false;
                            }
                        }
                        if hm_keys.is_empty() { continue; }

                        let idx = val as usize % hm_keys.len();
                        if hm.remove(&hm_keys[idx]) != sm.remove(sm_keys[idx]) {
                            return false;
                        }
                    }

                    // Access.
                    2 => {
                        if hm_keys.is_empty() { continue; }
                        let idx = val as usize % hm_keys.len();
                        let (hm_key, sm_key) = (&hm_keys[idx], sm_keys[idx]);

                        if hm.contains_key(hm_key) != sm.contains_key(sm_key) ||
                           hm.get(hm_key) != sm.get(sm_key) {
                            return false;
                        }
                    }

                    // Serde round-trip.
                    #[cfg(feature = "serde")]
                    3 => {
                        let ser = serde_json::to_string(&sm).unwrap();
                        sm = serde_json::from_str(&ser).unwrap();
                    }

                    _ => unreachable!(),
                }
            }

            let mut smv: Vec<_> = sm.values().collect();
            let mut hmv: Vec<_> = hm.values().collect();
            smv.sort();
            hmv.sort();
            smv == hmv
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn slotmap_serde() {
        let mut sm = DenseSlotMap::new();
        // Self-referential structure.
        let first = sm.insert_with_key(|k| (k, 23i32));
        let second = sm.insert((first, 42));

        // Make some empty slots.
        let empties = vec![sm.insert((first, 0)), sm.insert((first, 0))];
        empties.iter().for_each(|k| {
            sm.remove(*k);
        });

        let third = sm.insert((second, 0));
        sm[first].0 = third;

        let ser = serde_json::to_string(&sm).unwrap();
        let de: DenseSlotMap<DefaultKey, (DefaultKey, i32)> = serde_json::from_str(&ser).unwrap();
        assert_eq!(de.len(), sm.len());

        let mut smkv: Vec<_> = sm.iter().collect();
        let mut dekv: Vec<_> = de.iter().collect();
        smkv.sort();
        dekv.sort();
        assert_eq!(smkv, dekv);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn slotmap_serde_freelist() {
        let mut sm = DenseSlotMap::new();
        let k0 = sm.insert(5i32);
        let k1 = sm.insert(5i32);
        sm.remove(k0);
        sm.remove(k1);

        let ser = serde_json::to_string(&sm).unwrap();
        let mut de: DenseSlotMap<DefaultKey, i32> = serde_json::from_str(&ser).unwrap();

        de.insert(0);
        de.insert(1);
        de.insert(2);
        assert_eq!(de.len(), 3);
    }
}
