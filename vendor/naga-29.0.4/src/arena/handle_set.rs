//! The [`HandleSet`] type and associated definitions.

use crate::arena::{Arena, Handle, UniqueArena};

/// A set of `Handle<T>` values.
#[derive(Debug)]
pub struct HandleSet<T> {
    /// Bound on indexes of handles stored in this set.
    len: usize,

    /// `members[i]` is true if the handle with index `i` is a member.
    members: bit_set::BitSet,

    /// This type is indexed by values of type `T`.
    as_keys: core::marker::PhantomData<T>,
}

impl<T> HandleSet<T> {
    /// Return a new, empty `HandleSet`.
    pub fn new() -> Self {
        Self {
            len: 0,
            members: bit_set::BitSet::new(),
            as_keys: core::marker::PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Return a new, empty `HandleSet`, sized to hold handles from `arena`.
    pub fn for_arena(arena: &impl ArenaType<T>) -> Self {
        let len = arena.len();
        Self {
            len,
            members: bit_set::BitSet::with_capacity(len),
            as_keys: core::marker::PhantomData,
        }
    }

    /// Remove all members from `self`.
    pub fn clear(&mut self) {
        self.members.make_empty();
    }

    /// Remove all members from `self`, and reserve space to hold handles from `arena`.
    pub fn clear_for_arena(&mut self, arena: &impl ArenaType<T>) {
        self.members.make_empty();
        self.members.reserve_len(arena.len());
    }

    /// Return an iterator over all handles that could be made members
    /// of this set.
    pub fn all_possible(&self) -> impl Iterator<Item = Handle<T>> {
        super::Range::full_range_from_size(self.len)
    }

    /// Add `handle` to the set.
    ///
    /// Return `true` if `handle` was not already present in the set.
    pub fn insert(&mut self, handle: Handle<T>) -> bool {
        self.members.insert(handle.index())
    }

    /// Remove `handle` from the set.
    ///
    /// Returns `true` if `handle` was present in the set.
    pub fn remove(&mut self, handle: Handle<T>) -> bool {
        self.members.remove(handle.index())
    }

    /// Add handles from `iter` to the set.
    pub fn insert_iter(&mut self, iter: impl IntoIterator<Item = Handle<T>>) {
        for handle in iter {
            self.insert(handle);
        }
    }

    /// Add all of the handles that can be included in this set.
    pub fn add_all(&mut self) {
        self.members.get_mut().fill(true);
    }

    pub fn contains(&self, handle: Handle<T>) -> bool {
        self.members.contains(handle.index())
    }

    /// Return an iterator over all handles in `self`.
    pub fn iter(&self) -> impl '_ + Iterator<Item = Handle<T>> {
        self.members.iter().map(Handle::from_usize)
    }

    /// Removes and returns the numerically largest handle in the set, or `None`
    /// if the set is empty.
    pub fn pop(&mut self) -> Option<Handle<T>> {
        let members = core::mem::take(&mut self.members);
        let mut vec = members.into_bit_vec();
        let result = vec.iter_mut().enumerate().rev().find_map(|(i, mut bit)| {
            if *bit {
                *bit = false;
                Some(i)
            } else {
                None
            }
        });
        self.members = bit_set::BitSet::from_bit_vec(vec);
        result.map(Handle::from_usize)
    }
}

impl<T> Default for HandleSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ArenaType<T> {
    fn len(&self) -> usize;
}

impl<T> ArenaType<T> for Arena<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: core::hash::Hash + Eq> ArenaType<T> for UniqueArena<T> {
    fn len(&self) -> usize {
        self.len()
    }
}
