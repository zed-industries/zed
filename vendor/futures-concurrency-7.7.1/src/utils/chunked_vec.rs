//! A pin-safe, growable vector that never moves elements after insertion.
//!
//! Storage is organized as fixed-size chunks. Growth adds new chunks without
//! moving existing elements, making it safe to hold pinned references.

use alloc::{boxed::Box, vec::Vec};
use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

use fixedbitset::FixedBitSet;

const DEFAULT_CHUNK_SIZE: usize = 16;

/// A growable vector that provides stable indices and never moves elements.
///
/// This is suitable for storing pinned futures/streams because elements
/// maintain their memory location for their entire lifetime.
pub struct ChunkedVec<T> {
    chunks: Vec<Box<[MaybeUninit<T>]>>,
    occupied: FixedBitSet,
    free_list: Vec<usize>,
    len: usize,
    chunk_size: usize,
}

impl<T> Default for ChunkedVec<T> {
    fn default() -> Self {
        Self {
            chunks: Vec::new(),
            occupied: FixedBitSet::new(),
            free_list: Vec::new(),
            len: 0,
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }
}

impl<T> ChunkedVec<T> {
    /// Creates an empty `ChunkedVec` with the default chunk size (16).
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `ChunkedVec` with the specified chunk size.
    pub fn with_chunk_size(chunk_size: usize) -> Self {
        assert!(chunk_size > 0, "chunk size must be greater than 0");
        Self {
            chunks: Vec::new(),
            occupied: FixedBitSet::new(),
            free_list: Vec::new(),
            len: 0,
            chunk_size,
        }
    }

    /// Creates an empty `ChunkedVec` with *at least* the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut vec = Self::new();
        vec.reserve(capacity);
        vec
    }

    /// Returns the number of elements in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the total capacity of all allocated chunks.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.chunks.len() * self.chunk_size
    }

    /// Reserves capacity for *at least* `additional` more elements.
    pub fn reserve(&mut self, additional: usize) {
        let required = self.len.saturating_add(additional);
        while self.capacity() < required {
            self.grow();
        }
    }

    /// Inserts a value and returns its stable index.
    pub fn insert(&mut self, value: T) -> usize {
        let index = match self.free_list.pop() {
            Some(idx) => idx,
            None => {
                let idx = self.len;
                if idx >= self.capacity() {
                    self.grow();
                }
                idx
            }
        };

        let (chunk, offset) = self.index_to_chunk_offset(index);
        self.chunks[chunk][offset] = MaybeUninit::new(value);
        self.occupied.grow(index + 1);
        self.occupied.set(index, true);
        self.len += 1;
        index
    }

    /// Removes and returns the value at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds or the slot is not occupied.
    pub fn remove(&mut self, index: usize) -> T {
        assert!(self.occupied.contains(index), "slot is not occupied");

        let (chunk, offset) = self.index_to_chunk_offset(index);
        // SAFETY: we just verified the slot is occupied
        let value = unsafe {
            core::mem::replace(&mut self.chunks[chunk][offset], MaybeUninit::uninit()).assume_init()
        };
        self.occupied.set(index, false);
        self.free_list.push(index);
        self.len -= 1;
        value
    }

    /// Returns a reference to the value at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        if !self.occupied.contains(index) {
            return None;
        }
        let (chunk, offset) = self.index_to_chunk_offset(index);
        // SAFETY: we just verified the slot is occupied
        Some(unsafe { self.chunks[chunk][offset].assume_init_ref() })
    }

    /// Returns a mutable reference to the value at the given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if !self.occupied.contains(index) {
            return None;
        }
        let (chunk, offset) = self.index_to_chunk_offset(index);
        // SAFETY: we just verified the slot is occupied
        Some(unsafe { self.chunks[chunk][offset].assume_init_mut() })
    }

    /// Returns `true` if the given index contains a value.
    #[inline]
    pub fn contains(&self, index: usize) -> bool {
        self.occupied.contains(index)
    }

    /// Maps an index to its chunk and offset within that chunk.
    #[inline]
    fn index_to_chunk_offset(&self, index: usize) -> (usize, usize) {
        (index / self.chunk_size, index % self.chunk_size)
    }

    /// Allocates a new chunk.
    fn grow(&mut self) {
        let chunk: Box<[MaybeUninit<T>]> = (0..self.chunk_size)
            .map(|_| MaybeUninit::uninit())
            .collect();
        self.chunks.push(chunk);
    }
}

impl<T> Drop for ChunkedVec<T> {
    fn drop(&mut self) {
        for index in self.occupied.ones() {
            let (chunk, offset) = self.index_to_chunk_offset(index);
            // SAFETY: we're iterating over occupied indices
            unsafe {
                self.chunks[chunk][offset].assume_init_drop();
            }
        }
    }
}

impl<T> Index<usize> for ChunkedVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds or slot empty")
    }
}

impl<T> IndexMut<usize> for ChunkedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .expect("index out of bounds or slot empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_mapping() {
        let vec = ChunkedVec::<()>::with_chunk_size(4);

        assert_eq!(vec.index_to_chunk_offset(0), (0, 0));
        assert_eq!(vec.index_to_chunk_offset(1), (0, 1));
        assert_eq!(vec.index_to_chunk_offset(3), (0, 3));
        assert_eq!(vec.index_to_chunk_offset(4), (1, 0));
        assert_eq!(vec.index_to_chunk_offset(7), (1, 3));
        assert_eq!(vec.index_to_chunk_offset(8), (2, 0));
    }

    #[test]
    fn remove_and_reuse() {
        let mut vec = ChunkedVec::new();
        let idx0 = vec.insert(10);
        let idx1 = vec.insert(20);

        let val = vec.remove(idx1);
        assert_eq!(val, 20);
        assert_eq!(vec.len(), 1);
        assert!(!vec.contains(idx1));
        assert!(vec.contains(idx0));

        // new insert should reuse the freed slot
        let idx3 = vec.insert(40);
        assert_eq!(idx3, idx1);
        assert_eq!(vec[idx3], 40);
    }

    #[test]
    fn capacity_growth() {
        let mut vec = ChunkedVec::<i32>::new();
        assert_eq!(vec.capacity(), 0);

        vec.reserve(1);
        assert!(vec.capacity() >= 1);

        vec.reserve(20);
        assert!(vec.capacity() >= 20);
    }

    #[test]
    fn many_inserts() {
        let mut vec = ChunkedVec::new();
        for i in 0..50 {
            let idx = vec.insert(i);
            assert_eq!(vec[idx], i);
        }
        assert_eq!(vec.len(), 50);
    }

    #[test]
    fn get_none_for_empty_slot() {
        let mut vec = ChunkedVec::new();
        vec.insert(10);
        vec.insert(20);
        vec.remove(0);

        assert!(vec.get(0).is_none());
        assert_eq!(vec.get(1), Some(&20));
    }
}
