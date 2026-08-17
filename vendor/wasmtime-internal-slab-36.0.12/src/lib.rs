//! A very simple, uniformly-typed slab arena that supports deallocation and
//! reusing deallocated entries' space.
//!
//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.
//!
//! The free list of vacant entries in the slab are stored inline in the slab's
//! existing storage.
//!
//! # Example
//!
//! ```
//! use wasmtime_internal_slab::{Id, Slab};
//!
//! let mut slab = Slab::new();
//!
//! // Insert some values into the slab.
//! let rza = slab.alloc("Robert Fitzgerald Diggs");
//! let gza = slab.alloc("Gary Grice");
//! let bill = slab.alloc("Bill Gates");
//!
//! // Allocated elements can be accessed infallibly via indexing (and missing and
//! // deallocated entries will panic).
//! assert_eq!(slab[rza], "Robert Fitzgerald Diggs");
//!
//! // Alternatively, the `get` and `get_mut` methods provide fallible lookup.
//! if let Some(genius) = slab.get(gza) {
//!     println!("The gza gza genius: {}", genius);
//! }
//! if let Some(val) = slab.get_mut(bill) {
//!     *val = "Bill Gates doesn't belong in this set...";
//! }
//!
//! // We can remove values from the slab.
//! slab.dealloc(bill);
//!
//! // Allocate a new entry.
//! let bill = slab.alloc("Bill Murray");
//! ```
//!
//! # Using `Id`s with the Wrong `Slab`
//!
//! `Slab` does NOT check that `Id`s used to access previously-allocated values
//! came from the current `Slab` instance (as opposed to a different `Slab`
//! instance). Using `Id`s from a different `Slab` is safe, but will yield an
//! unrelated value, if any at all.
//!
//! If you desire checking that an `Id` came from the correct `Slab` instance,
//! it should be easy to layer that functionality on top of this crate by
//! wrapping `Slab` and `Id` in types that additionally maintain a slab instance
//! identifier.
//!
//! # The ABA Problem
//!
//! This `Slab` type does NOT protect against ABA bugs, such as the following
//! sequence:
//!
//! * Value `A` is allocated into the slab, yielding id `i`.
//!
//! * `A` is deallocated, and so `i`'s associated entry is added to the slab's
//!   free list.
//!
//! * Value `B` is allocated into the slab, reusing `i`'s associated entry,
//!   yielding id `i`.
//!
//! * The "original" id `i` is used to access the arena, expecting the
//!   deallocated value `A`, but getting the new value `B`.
//!
//! That is, it does not detect and prevent against the memory-safe version of
//! use-after-free bugs.
//!
//! If you need to protect against ABA bugs, it should be easy to layer that
//! functionality on top of this crate by wrapping `Slab` with something like
//! the following:
//!
//! ```rust
//! pub struct GenerationalId {
//!     id: wasmtime_internal_slab::Id,
//!     generation: u32,
//! }
//!
//! struct GenerationalEntry<T> {
//!     value: T,
//!     generation: u32,
//! }
//!
//! pub struct GenerationalSlab<T> {
//!     slab: wasmtime_internal_slab::Slab<GenerationalEntry<T>>,
//!     generation: u32,
//! }
//!
//! impl<T> GenerationalSlab<T> {
//!     pub fn alloc(&mut self, value: T) -> GenerationalId {
//!         let generation = self.generation;
//!         let id = self.slab.alloc(GenerationalEntry { value, generation });
//!         GenerationalId { id, generation }
//!     }
//!
//!     pub fn get(&self, id: GenerationalId) -> Option<&T> {
//!         let entry = self.slab.get(id.id)?;
//!
//!         // Check that the entry's generation matches the id's generation,
//!         // else we have an ABA bug. (Alternatively, return `None` instead
//!         // of panicking.)
//!         assert_eq!(id.generation, entry.generation);
//!
//!         Some(&entry.value)
//!     }
//!
//!     pub fn dealloc(&mut self, id: GenerationalId) {
//!         // Check that the entry's generation matches the id's generation,
//!         // else we have an ABA bug. (Alternatively, silently return on
//!         // double-free instead of panicking.)
//!         assert_eq!(id.generation, self.slab[id.id].generation);
//!
//!         self.slab.dealloc(id.id);
//!
//!         // Increment our generation whenever we deallocate so that any new
//!         // value placed in this same entry will have a different generation
//!         // and we can detect ABA bugs.
//!         self.generation += 1;
//!     }
//! }
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs, missing_debug_implementations)]

extern crate alloc;

use alloc::vec::Vec;
use core::fmt;
use core::num::NonZeroU32;

/// An identifier for an allocated value inside a `slab`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id(EntryIndex);

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&self.0.index()).finish()
    }
}

impl Id {
    /// Get the raw underlying representation of this `Id`.
    #[inline]
    pub fn into_raw(self) -> u32 {
        u32::try_from(self.0.index()).unwrap()
    }

    /// Construct an `Id` from its raw underlying representation.
    ///
    /// `raw` should be a value that was previously created via
    /// `Id::into_raw`. May panic if given arbitrary values.
    #[inline]
    pub fn from_raw(raw: u32) -> Self {
        let raw = usize::try_from(raw).unwrap();
        Self(EntryIndex::new(raw))
    }
}

/// A simple, uni-typed slab arena.
pub struct Slab<T> {
    /// The slab's entries, each is either occupied and holding a `T` or vacant
    /// and is a link the free list.
    entries: Vec<Entry<T>>,

    /// The index of the first free entry in the free list.
    free: Option<EntryIndex>,

    /// The number of occupied entries is this slab.
    len: u32,
}

impl<T> fmt::Debug for Slab<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

enum Entry<T> {
    /// An occupied entry holding a `T`.
    Occupied(T),

    /// A vacant entry.
    Free {
        /// A link in the slab's free list, pointing to the next free entry, if
        /// any.
        next_free: Option<EntryIndex>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct EntryIndex(NonZeroU32);

impl EntryIndex {
    #[inline]
    fn new(index: usize) -> Self {
        assert!(index <= Slab::<()>::MAX_CAPACITY);
        let x = u32::try_from(index + 1).unwrap();
        Self(NonZeroU32::new(x).unwrap())
    }

    #[inline]
    fn index(&self) -> usize {
        let index = self.0.get() - 1;
        usize::try_from(index).unwrap()
    }
}

impl<T> Default for Slab<T> {
    #[inline]
    fn default() -> Self {
        Self {
            entries: Vec::default(),
            free: None,
            len: 0,
        }
    }
}

impl<T> core::ops::Index<Id> for Slab<T> {
    type Output = T;

    #[inline]
    fn index(&self, id: Id) -> &Self::Output {
        self.get(id)
            .expect("id from different slab or value was deallocated")
    }
}

impl<T> core::ops::IndexMut<Id> for Slab<T> {
    #[inline]
    fn index_mut(&mut self, id: Id) -> &mut Self::Output {
        self.get_mut(id)
            .expect("id from different slab or value was deallocated")
    }
}

impl<T> Slab<T> {
    /// The maximum capacity any `Slab` can have: `u32::MAX - 1`.
    pub const MAX_CAPACITY: usize = (u32::MAX - 1) as usize;

    /// Construct a new, empty slab.
    #[inline]
    pub fn new() -> Self {
        Slab::default()
    }

    /// Construct a new, empty slab, pre-reserving space for at least `capacity`
    /// elements.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut slab = Self::new();
        slab.reserve(capacity);
        slab
    }

    /// Ensure that there is space for at least `additional` elements in this
    /// slab.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `Self::MAX_CAPACITY`.
    pub fn reserve(&mut self, additional: usize) {
        let cap = self.capacity();
        let len = self.len();
        assert!(cap >= len);
        if cap - len >= additional {
            // Already have `additional` capacity available.
            return;
        }

        self.entries.reserve(additional);

        // Maintain the invariant that `i <= MAX_CAPACITY` for all indices `i`
        // in `self.entries`.
        assert!(self.entries.capacity() <= Self::MAX_CAPACITY);
    }

    fn double_capacity(&mut self) {
        // Double our capacity to amortize the cost of resizing. But make sure
        // we add some amount of minimum additional capacity, since doubling
        // zero capacity isn't useful.
        const MIN_CAPACITY: usize = 16;
        let additional = core::cmp::max(self.entries.capacity(), MIN_CAPACITY);
        self.reserve(additional);
    }

    /// What is the capacity of this slab? That is, how many entries can it
    /// contain within its current underlying storage?
    #[inline]
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    /// How many values are currently allocated within this slab?
    #[inline]
    pub fn len(&self) -> usize {
        usize::try_from(self.len).unwrap()
    }

    /// Are there zero allocated values within this slab?
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Try to allocate a `T` value within this slab.
    ///
    /// If there is no available capacity, ownership of the given value is
    /// returned via `Err(value)`.
    #[inline]
    pub fn try_alloc(&mut self, value: T) -> Result<Id, T> {
        if let Some(index) = self.try_alloc_index() {
            let next_free = match self.entries[index.index()] {
                Entry::Free { next_free } => next_free,
                Entry::Occupied { .. } => unreachable!(),
            };
            self.free = next_free;
            self.entries[index.index()] = Entry::Occupied(value);
            self.len += 1;
            Ok(Id(index))
        } else {
            Err(value)
        }
    }

    #[inline]
    fn try_alloc_index(&mut self) -> Option<EntryIndex> {
        self.free.take().or_else(|| {
            if self.entries.len() < self.entries.capacity() {
                let index = EntryIndex::new(self.entries.len());
                self.entries.push(Entry::Free { next_free: None });
                Some(index)
            } else {
                None
            }
        })
    }

    /// Allocate a `T` value within this slab, allocating additional underlying
    /// storage if there is no available capacity.
    ///
    /// # Panics
    ///
    /// Panics if allocating this value requires reallocating the underlying
    /// storage, and the new capacity exceeds `Slab::MAX_CAPACITY`.
    #[inline]
    pub fn alloc(&mut self, value: T) -> Id {
        self.try_alloc(value)
            .unwrap_or_else(|value| self.alloc_slow(value))
    }

    /// Get the `Id` that will be returned for the next allocation in this slab.
    #[inline]
    pub fn next_id(&self) -> Id {
        let index = self.free.unwrap_or_else(|| EntryIndex::new(self.len()));
        Id(index)
    }

    #[inline(never)]
    #[cold]
    fn alloc_slow(&mut self, value: T) -> Id {
        // Reserve additional capacity, since we didn't have space for the
        // allocation.
        self.double_capacity();
        // After which the allocation will succeed.
        self.try_alloc(value).ok().unwrap()
    }

    /// Get a shared borrow of the value associated with `id`.
    ///
    /// Returns `None` if the value has since been deallocated.
    ///
    /// If `id` comes from a different `Slab` instance, this method may panic,
    /// return `None`, or return an arbitrary value.
    #[inline]
    pub fn get(&self, id: Id) -> Option<&T> {
        match self
            .entries
            .get(id.0.index())
            .expect("id from different slab")
        {
            Entry::Occupied(x) => Some(x),
            Entry::Free { .. } => None,
        }
    }

    /// Get an exclusive borrow of the value associated with `id`.
    ///
    /// Returns `None` if the value has since been deallocated.
    ///
    /// If `id` comes from a different `Slab` instance, this method may panic,
    /// return `None`, or return an arbitrary value.
    #[inline]
    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        match self
            .entries
            .get_mut(id.0.index())
            .expect("id from different slab")
        {
            Entry::Occupied(x) => Some(x),
            Entry::Free { .. } => None,
        }
    }

    /// Does this slab contain an allocated value for `id`?
    #[inline]
    pub fn contains(&self, id: Id) -> bool {
        match self.entries.get(id.0.index()) {
            Some(Entry::Occupied(_)) => true,
            None | Some(Entry::Free { .. }) => false,
        }
    }

    /// Deallocate the value associated with the given `id`.
    ///
    /// If `id` comes from a different `Slab` instance, this method may panic or
    /// deallocate an arbitrary value.
    #[inline]
    pub fn dealloc(&mut self, id: Id) -> T {
        let entry = core::mem::replace(
            self.entries
                .get_mut(id.0.index())
                .expect("id from a different slab"),
            Entry::Free { next_free: None },
        );
        match entry {
            Entry::Free { .. } => panic!("attempt to deallocate an entry that is already vacant"),
            Entry::Occupied(value) => {
                let next_free = core::mem::replace(&mut self.free, Some(id.0));
                self.entries[id.0.index()] = Entry::Free { next_free };
                self.len -= 1;
                value
            }
        }
    }

    /// Iterate over all values currently allocated within this `Slab`.
    ///
    /// Yields pairs of an `Id` and the `Id`'s associated value.
    ///
    /// Iteration order is undefined.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (Id, &T)> + '_ {
        assert!(self.entries.len() <= Self::MAX_CAPACITY);
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match e {
                Entry::Occupied(x) => Some((Id(EntryIndex::new(i)), x)),
                Entry::Free { .. } => None,
            })
    }

    /// Mutably iterate over all values currently allocated within this `Slab`.
    ///
    /// Yields pairs of an `Id` and the `Id`'s associated value.
    ///
    /// Iteration order is undefined.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut T)> + '_ {
        assert!(self.entries.len() <= Self::MAX_CAPACITY);
        self.entries
            .iter_mut()
            .enumerate()
            .filter_map(|(i, e)| match e {
                Entry::Occupied(x) => Some((Id(EntryIndex::new(i)), x)),
                Entry::Free { .. } => None,
            })
    }

    /// Iterate over and remove all entries in this slab.
    ///
    /// The slab will be empty after calling this method.
    ///
    /// Yields pairs of an `Id` and the `Id`'s associated value.
    ///
    /// Iteration order is undefined.
    #[inline]
    pub fn drain(&mut self) -> impl Iterator<Item = (Id, T)> + '_ {
        assert!(self.entries.len() <= Self::MAX_CAPACITY);
        self.len = 0;
        self.free = None;
        self.entries
            .drain(..)
            .enumerate()
            .filter_map(|(i, e)| match e {
                Entry::Occupied(x) => Some((Id(EntryIndex::new(i)), x)),
                Entry::Free { .. } => None,
            })
    }
}
