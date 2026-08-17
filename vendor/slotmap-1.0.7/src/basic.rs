// Needed because assigning to non-Copy union is unsafe in stable but not in nightly.
#![allow(unused_unsafe)]

//! Contains the slot map implementation.

#[cfg(all(nightly, any(doc, feature = "unstable")))]
use alloc::collections::TryReserveError;
use alloc::vec::Vec;
use core::fmt;
use core::iter::{Enumerate, FusedIterator};
use core::marker::PhantomData;
#[allow(unused_imports)] // MaybeUninit is only used on nightly at the moment.
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Index, IndexMut};

use crate::util::{Never, UnwrapUnchecked};
use crate::{DefaultKey, Key, KeyData};

// Storage inside a slot or metadata for the freelist when vacant.
union SlotUnion<T> {
    value: ManuallyDrop<T>,
    next_free: u32,
}

// A slot, which represents storage for a value and a current version.
// Can be occupied or vacant.
struct Slot<T> {
    u: SlotUnion<T>,
    version: u32, // Even = vacant, odd = occupied.
}

// Safe API to read a slot.
enum SlotContent<'a, T: 'a> {
    Occupied(&'a T),
    Vacant(&'a u32),
}

enum SlotContentMut<'a, T: 'a> {
    OccupiedMut(&'a mut T),
    VacantMut(&'a mut u32),
}

use self::SlotContent::{Occupied, Vacant};
use self::SlotContentMut::{OccupiedMut, VacantMut};

impl<T> Slot<T> {
    // Is this slot occupied?
    #[inline(always)]
    pub fn occupied(&self) -> bool {
        self.version % 2 > 0
    }

    pub fn get(&self) -> SlotContent<T> {
        unsafe {
            if self.occupied() {
                Occupied(&*self.u.value)
            } else {
                Vacant(&self.u.next_free)
            }
        }
    }

    pub fn get_mut(&mut self) -> SlotContentMut<T> {
        unsafe {
            if self.occupied() {
                OccupiedMut(&mut *self.u.value)
            } else {
                VacantMut(&mut self.u.next_free)
            }
        }
    }
}

impl<T> Drop for Slot<T> {
    fn drop(&mut self) {
        if core::mem::needs_drop::<T>() && self.occupied() {
            // This is safe because we checked that we're occupied.
            unsafe {
                ManuallyDrop::drop(&mut self.u.value);
            }
        }
    }
}

impl<T: Clone> Clone for Slot<T> {
    fn clone(&self) -> Self {
        Self {
            u: match self.get() {
                Occupied(value) => SlotUnion {
                    value: ManuallyDrop::new(value.clone()),
                },
                Vacant(&next_free) => SlotUnion { next_free },
            },
            version: self.version,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self.get_mut(), source.get()) {
            (OccupiedMut(self_val), Occupied(source_val)) => self_val.clone_from(source_val),
            (VacantMut(self_next_free), Vacant(&source_next_free)) => {
                *self_next_free = source_next_free
            },
            (_, Occupied(value)) => {
                self.u = SlotUnion {
                    value: ManuallyDrop::new(value.clone()),
                }
            },
            (_, Vacant(&next_free)) => self.u = SlotUnion { next_free },
        }
        self.version = source.version;
    }
}

impl<T: fmt::Debug> fmt::Debug for Slot<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = fmt.debug_struct("Slot");
        builder.field("version", &self.version);
        match self.get() {
            Occupied(value) => builder.field("value", value).finish(),
            Vacant(next_free) => builder.field("next_free", next_free).finish(),
        }
    }
}

/// Slot map, storage with stable unique keys.
///
/// See [crate documentation](crate) for more details.
#[derive(Debug)]
pub struct SlotMap<K: Key, V> {
    slots: Vec<Slot<V>>,
    free_head: u32,
    num_elems: u32,
    _k: PhantomData<fn(K) -> K>,
}

impl<V> SlotMap<DefaultKey, V> {
    /// Constructs a new, empty [`SlotMap`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm: SlotMap<_, i32> = SlotMap::new();
    /// ```
    pub fn new() -> Self {
        Self::with_capacity_and_key(0)
    }

    /// Creates an empty [`SlotMap`] with the given capacity.
    ///
    /// The slot map will not reallocate until it holds at least `capacity`
    /// elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm: SlotMap<_, i32> = SlotMap::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_key(capacity)
    }
}

impl<K: Key, V> SlotMap<K, V> {
    /// Constructs a new, empty [`SlotMap`] with a custom key type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// new_key_type! {
    ///     struct PositionKey;
    /// }
    /// let mut positions: SlotMap<PositionKey, i32> = SlotMap::with_key();
    /// ```
    pub fn with_key() -> Self {
        Self::with_capacity_and_key(0)
    }

    /// Creates an empty [`SlotMap`] with the given capacity and a custom key
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
    /// let mut messages = SlotMap::with_capacity_and_key(3);
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
            u: SlotUnion { next_free: 0 },
            version: 0,
        });

        Self {
            slots,
            free_head: 1,
            num_elems: 0,
            _k: PhantomData,
        }
    }

    /// Returns the number of elements in the slot map.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::with_capacity(10);
    /// sm.insert("len() counts actual elements, not capacity");
    /// let key = sm.insert("removed elements don't count either");
    /// sm.remove(key);
    /// assert_eq!(sm.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.num_elems as usize
    }

    /// Returns if the slot map is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert("dummy");
    /// assert_eq!(sm.is_empty(), false);
    /// sm.remove(key);
    /// assert_eq!(sm.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.num_elems == 0
    }

    /// Returns the number of elements the [`SlotMap`] can hold without
    /// reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let sm: SlotMap<_, f64> = SlotMap::with_capacity(10);
    /// assert_eq!(sm.capacity(), 10);
    /// ```
    pub fn capacity(&self) -> usize {
        // One slot is reserved for the sentinel.
        self.slots.capacity() - 1
    }

    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the [`SlotMap`]. The collection may reserve more space to avoid
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
    /// let mut sm = SlotMap::new();
    /// sm.insert("foo");
    /// sm.reserve(32);
    /// assert!(sm.capacity() >= 33);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        // One slot is reserved for the sentinel.
        let needed = (self.len() + additional).saturating_sub(self.slots.len() - 1);
        self.slots.reserve(needed);
    }

    /// Tries to reserve capacity for at least `additional` more elements to be
    /// inserted in the [`SlotMap`]. The collection may reserve more space to
    /// avoid frequent reallocations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// sm.insert("foo");
    /// sm.try_reserve(32).unwrap();
    /// assert!(sm.capacity() >= 33);
    /// ```
    #[cfg(all(nightly, any(doc, feature = "unstable")))]
    #[cfg_attr(all(nightly, doc), doc(cfg(feature = "unstable")))]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
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
    /// let mut sm = SlotMap::new();
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
    /// let mut sm = SlotMap::new();
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
    /// let mut sm = SlotMap::new();
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
    /// let mut sm = SlotMap::new();
    /// let key = sm.try_insert_with_key::<_, ()>(|k| Ok((k, 20))).unwrap();
    /// assert_eq!(sm[key], (key, 20));
    ///
    /// sm.try_insert_with_key::<_, ()>(|k| Err(())).unwrap_err();
    /// ```
    pub fn try_insert_with_key<F, E>(&mut self, f: F) -> Result<K, E>
    where
        F: FnOnce(K) -> Result<V, E>,
    {
        // In case f panics, we don't make any changes until we have the value.
        let new_num_elems = self.num_elems + 1;
        if new_num_elems == core::u32::MAX {
            panic!("SlotMap number of elements overflow");
        }

        if let Some(slot) = self.slots.get_mut(self.free_head as usize) {
            let occupied_version = slot.version | 1;
            let kd = KeyData::new(self.free_head, occupied_version);

            // Get value first in case f panics or returns an error.
            let value = f(kd.into())?;

            // Update.
            unsafe {
                self.free_head = slot.u.next_free;
                slot.u.value = ManuallyDrop::new(value);
                slot.version = occupied_version;
            }
            self.num_elems = new_num_elems;
            return Ok(kd.into());
        }

        let version = 1;
        let kd = KeyData::new(self.slots.len() as u32, version);

        // Create new slot before adjusting freelist in case f or the allocation panics or errors.
        self.slots.push(Slot {
            u: SlotUnion {
                value: ManuallyDrop::new(f(kd.into())?),
            },
            version,
        });

        self.free_head = kd.idx + 1;
        self.num_elems = new_num_elems;
        Ok(kd.into())
    }

    // Helper function to remove a value from a slot. Safe iff the slot is
    // occupied. Returns the value removed.
    #[inline(always)]
    unsafe fn remove_from_slot(&mut self, idx: usize) -> V {
        // Remove value from slot before overwriting union.
        let slot = self.slots.get_unchecked_mut(idx);
        let value = ManuallyDrop::take(&mut slot.u.value);

        // Maintain freelist.
        slot.u.next_free = self.free_head;
        self.free_head = idx as u32;
        self.num_elems -= 1;
        slot.version = slot.version.wrapping_add(1);

        value
    }

    /// Removes a key from the slot map, returning the value at the key if the
    /// key was not previously removed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert(42);
    /// assert_eq!(sm.remove(key), Some(42));
    /// assert_eq!(sm.remove(key), None);
    /// ```
    pub fn remove(&mut self, key: K) -> Option<V> {
        let kd = key.data();
        if self.contains_key(key) {
            // This is safe because we know that the slot is occupied.
            Some(unsafe { self.remove_from_slot(kd.idx as usize) })
        } else {
            None
        }
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
    ///
    /// let k1 = sm.insert(0);
    /// let k2 = sm.insert(1);
    /// let k3 = sm.insert(2);
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
        for i in 1..self.slots.len() {
            // This is safe because removing elements does not shrink slots.
            let slot = unsafe { self.slots.get_unchecked_mut(i) };
            let version = slot.version;

            let should_remove = if let OccupiedMut(value) = slot.get_mut() {
                let key = KeyData::new(i as u32, version).into();
                !f(key, value)
            } else {
                false
            };

            if should_remove {
                // This is safe because we know that the slot was occupied.
                unsafe { self.remove_from_slot(i) };
            }
        }
    }

    /// Clears the slot map. Keeps the allocated memory for reuse.
    ///
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
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
    /// let mut sm = SlotMap::new();
    /// let k = sm.insert(0);
    /// let v: Vec<_> = sm.drain().collect();
    /// assert_eq!(sm.len(), 0);
    /// assert_eq!(v, vec![(k, 0)]);
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
            .map(|slot| unsafe { &*slot.u.value })
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
    /// let key = sm.insert("bar");
    /// assert_eq!(unsafe { sm.get_unchecked(key) }, &"bar");
    /// sm.remove(key);
    /// // sm.get_unchecked(key) is now dangerous!
    /// ```
    pub unsafe fn get_unchecked(&self, key: K) -> &V {
        debug_assert!(self.contains_key(key));
        &self.slots.get_unchecked(key.data().idx as usize).u.value
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let key = sm.insert(3.5);
    /// if let Some(x) = sm.get_mut(key) {
    ///     *x += 3.0;
    /// }
    /// assert_eq!(sm[key], 6.5);
    /// ```
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let kd = key.data();
        self.slots
            .get_mut(kd.idx as usize)
            .filter(|slot| slot.version == kd.version.get())
            .map(|slot| unsafe { &mut *slot.u.value })
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
    /// unsafe { *sm.get_unchecked_mut(key) = "bar" };
    /// assert_eq!(sm[key], "bar");
    /// sm.remove(key);
    /// // sm.get_unchecked_mut(key) is now dangerous!
    /// ```
    pub unsafe fn get_unchecked_mut(&mut self, key: K) -> &mut V {
        debug_assert!(self.contains_key(key));
        &mut self.slots.get_unchecked_mut(key.data().idx as usize).u.value
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
                ptrs[i] = MaybeUninit::new(&mut *slot.u.value);
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
    /// let mut sm = SlotMap::new();
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
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// let mut sm = SlotMap::new();
    /// let k0 = sm.insert(0);
    /// let k1 = sm.insert(1);
    /// let k2 = sm.insert(2);
    ///
    /// for (k, v) in sm.iter() {
    ///     println!("key: {:?}, val: {}", k, v);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        let mut it = self.slots.iter().enumerate();
        it.next(); // Skip sentinel.
        Iter {
            slots: it,
            num_left: self.len(),
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
        let len = self.len();
        let mut it = self.slots.iter_mut().enumerate();
        it.next(); // Skip sentinel.
        IterMut {
            num_left: len,
            slots: it,
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
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::collections::HashSet;
    /// let mut sm = SlotMap::new();
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
    /// This function must iterate over all slots, empty or not. In the face of
    /// many deleted elements it can be inefficient.
    ///
    /// # Examples
    ///
    /// ```
    /// # use slotmap::*;
    /// # use std::collections::HashSet;
    /// let mut sm = SlotMap::new();
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

impl<K: Key, V> Clone for SlotMap<K, V>
where
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            slots: self.slots.clone(),
            ..*self
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.slots.clone_from(&source.slots);
        self.free_head = source.free_head;
        self.num_elems = source.num_elems;
    }
}

impl<K: Key, V> Default for SlotMap<K, V> {
    fn default() -> Self {
        Self::with_key()
    }
}

impl<K: Key, V> Index<K> for SlotMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &V {
        match self.get(key) {
            Some(r) => r,
            None => panic!("invalid SlotMap key used"),
        }
    }
}

impl<K: Key, V> IndexMut<K> for SlotMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut V {
        match self.get_mut(key) {
            Some(r) => r,
            None => panic!("invalid SlotMap key used"),
        }
    }
}

// Iterators.
/// A draining iterator for [`SlotMap`].
///
/// This iterator is created by [`SlotMap::drain`].
#[derive(Debug)]
pub struct Drain<'a, K: 'a + Key, V: 'a> {
    sm: &'a mut SlotMap<K, V>,
    cur: usize,
}

/// An iterator that moves key-value pairs out of a [`SlotMap`].
///
/// This iterator is created by calling the `into_iter` method on [`SlotMap`],
/// provided by the [`IntoIterator`] trait.
#[derive(Debug, Clone)]
pub struct IntoIter<K: Key, V> {
    num_left: usize,
    slots: Enumerate<alloc::vec::IntoIter<Slot<V>>>,
    _k: PhantomData<fn(K) -> K>,
}

/// An iterator over the key-value pairs in a [`SlotMap`].
///
/// This iterator is created by [`SlotMap::iter`].
#[derive(Debug)]
pub struct Iter<'a, K: 'a + Key, V: 'a> {
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

/// A mutable iterator over the key-value pairs in a [`SlotMap`].
///
/// This iterator is created by [`SlotMap::iter_mut`].
#[derive(Debug)]
pub struct IterMut<'a, K: 'a + Key, V: 'a> {
    num_left: usize,
    slots: Enumerate<core::slice::IterMut<'a, Slot<V>>>,
    _k: PhantomData<fn(K) -> K>,
}

/// An iterator over the keys in a [`SlotMap`].
///
/// This iterator is created by [`SlotMap::keys`].
#[derive(Debug)]
pub struct Keys<'a, K: 'a + Key, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Self {
        Keys {
            inner: self.inner.clone(),
        }
    }
}

/// An iterator over the values in a [`SlotMap`].
///
/// This iterator is created by [`SlotMap::values`].
#[derive(Debug)]
pub struct Values<'a, K: 'a + Key, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: 'a + Key, V: 'a> Clone for Values<'a, K, V> {
    fn clone(&self) -> Self {
        Values {
            inner: self.inner.clone(),
        }
    }
}

/// A mutable iterator over the values in a [`SlotMap`].
///
/// This iterator is created by [`SlotMap::values_mut`].
#[derive(Debug)]
pub struct ValuesMut<'a, K: 'a + Key, V: 'a> {
    inner: IterMut<'a, K, V>,
}

impl<'a, K: Key, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        let len = self.sm.slots.len();
        while self.cur < len {
            let idx = self.cur;
            self.cur += 1;

            // This is safe because removing doesn't shrink slots.
            unsafe {
                let slot = self.sm.slots.get_unchecked(idx);
                if slot.occupied() {
                    let kd = KeyData::new(idx as u32, slot.version);
                    return Some((kd.into(), self.sm.remove_from_slot(idx)));
                }
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
            if slot.occupied() {
                let kd = KeyData::new(idx as u32, slot.version);

                // Prevent dropping after extracting the value.
                slot.version = 0;

                // This is safe because we know the slot was occupied.
                let value = unsafe { ManuallyDrop::take(&mut slot.u.value) };

                self.num_left -= 1;
                return Some((kd.into(), value));
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
            if let Occupied(value) = slot.get() {
                let kd = KeyData::new(idx as u32, slot.version);
                self.num_left -= 1;
                return Some((kd.into(), value));
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
            let version = slot.version;
            if let OccupiedMut(value) = slot.get_mut() {
                let kd = KeyData::new(idx as u32, version);
                self.num_left -= 1;
                return Some((kd.into(), value));
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

impl<'a, K: Key, V> IntoIterator for &'a SlotMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: Key, V> IntoIterator for &'a mut SlotMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: Key, V> IntoIterator for SlotMap<K, V> {
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
                version: self.version,
                value: match self.get() {
                    Occupied(value) => Some(value),
                    Vacant(_) => None,
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

            Ok(Self {
                u: match serde_slot.value {
                    Some(value) => SlotUnion {
                        value: ManuallyDrop::new(value),
                    },
                    None => SlotUnion { next_free: 0 },
                },
                version: serde_slot.version,
            })
        }
    }

    impl<K: Key, V: Serialize> Serialize for SlotMap<K, V> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.slots.serialize(serializer)
        }
    }

    impl<'de, K, V> Deserialize<'de> for SlotMap<K, V>
    where
        K: Key,
        V: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let mut slots: Vec<Slot<V>> = Deserialize::deserialize(deserializer)?;
            if slots.len() >= u32::max_value() as usize {
                return Err(de::Error::custom(&"too many slots"));
            }

            // Ensure the first slot exists and is empty for the sentinel.
            if slots.get(0).map_or(true, |slot| slot.version % 2 == 1) {
                return Err(de::Error::custom(&"first slot not empty"));
            }

            slots[0].version = 0;
            slots[0].u.next_free = 0;

            // We have our slots, rebuild freelist.
            let mut num_elems = 0;
            let mut next_free = slots.len();
            for (i, slot) in slots[1..].iter_mut().enumerate() {
                if slot.occupied() {
                    num_elems += 1;
                } else {
                    slot.u.next_free = next_free as u32;
                    next_free = i + 1;
                }
            }

            Ok(Self {
                num_elems,
                slots,
                free_head: next_free as u32,
                _k: PhantomData,
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
    struct CountDrop<'a>(&'a std::cell::RefCell<usize>);

    impl<'a> Drop for CountDrop<'a> {
        fn drop(&mut self) {
            *self.0.borrow_mut() += 1;
        }
    }

    #[cfg(all(nightly, feature = "unstable"))]
    #[test]
    fn check_drops() {
        let drops = std::cell::RefCell::new(0usize);

        {
            let mut clone = {
                // Insert 1000 items.
                let mut sm = SlotMap::new();
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

            // Reuse some empty slots.
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
        let mut sm = SlotMap::new();

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
            let mut sm = SlotMap::new();
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
        let mut sm = SlotMap::new();
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
        let de: SlotMap<DefaultKey, (DefaultKey, i32)> = serde_json::from_str(&ser).unwrap();
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
        let mut sm = SlotMap::new();
        let k = sm.insert(5i32);
        sm.remove(k);

        let ser = serde_json::to_string(&sm).unwrap();
        let mut de: SlotMap<DefaultKey, i32> = serde_json::from_str(&ser).unwrap();

        de.insert(0);
        de.insert(1);
        de.insert(2);
        assert_eq!(de.len(), 3);
    }
}
