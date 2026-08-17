//! Small lists of entity references.
use crate::EntityRef;
use crate::packed_option::ReservedValue;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::mem;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// A small list of entity references allocated from a pool.
///
/// An `EntityList<T>` type provides similar functionality to `Vec<T>`, but with some important
/// differences in the implementation:
///
/// 1. Memory is allocated from a `ListPool<T>` instead of the global heap.
/// 2. The footprint of an entity list is 4 bytes, compared with the 24 bytes for `Vec<T>`.
/// 3. An entity list doesn't implement `Drop`, leaving it to the pool to manage memory.
///
/// The list pool is intended to be used as a LIFO allocator. After building up a larger data
/// structure with many list references, the whole thing can be discarded quickly by clearing the
/// pool.
///
/// # Safety
///
/// Entity lists are not as safe to use as `Vec<T>`, but they never jeopardize Rust's memory safety
/// guarantees. These are the problems to be aware of:
///
/// - If you lose track of an entity list, its memory won't be recycled until the pool is cleared.
///   This can cause the pool to grow very large with leaked lists.
/// - If entity lists are used after their pool is cleared, they may contain garbage data, and
///   modifying them may corrupt other lists in the pool.
/// - If an entity list is used with two different pool instances, both pools are likely to become
///   corrupted.
///
/// Entity lists can be cloned, but that operation should only be used as part of cloning the whole
/// function they belong to. *Cloning an entity list does not allocate new memory for the clone*.
/// It creates an alias of the same memory.
///
/// Entity lists cannot be hashed and compared for equality because it's not possible to compare the
/// contents of the list without the pool reference.
///
/// # Implementation
///
/// The `EntityList` itself is designed to have the smallest possible footprint. This is important
/// because it is used inside very compact data structures like `InstructionData`. The list
/// contains only a 32-bit index into the pool's memory vector, pointing to the first element of
/// the list. A zero value represents the empty list, which is returned by `EntityList::default`.
///
/// The pool is just a single `Vec<T>` containing all of the allocated lists. Each list is
/// represented as three contiguous parts:
///
/// 1. The number of elements in the list.
/// 2. The list elements.
/// 3. Excess capacity elements.
///
/// The total size of the three parts is always a power of two, and the excess capacity is always
/// as small as possible. This means that shrinking a list may cause the excess capacity to shrink
/// if a smaller power-of-two size becomes available.
///
/// Both growing and shrinking a list may cause it to be reallocated in the pool vector.
///
/// The index stored in an `EntityList` points to part 2, the list elements. The value 0 is
/// reserved for the empty list which isn't allocated in the vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct EntityList<T: EntityRef + ReservedValue> {
    index: u32,
    unused: PhantomData<T>,
}

/// Create an empty list.
impl<T: EntityRef + ReservedValue> Default for EntityList<T> {
    fn default() -> Self {
        Self {
            index: 0,
            unused: PhantomData,
        }
    }
}

/// A memory pool for storing lists of `T`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ListPool<T: EntityRef + ReservedValue> {
    // The main array containing the lists.
    data: Vec<T>,

    // Heads of the free lists, one for each size class.
    free: Vec<usize>,
}

impl<T: EntityRef + ReservedValue> PartialEq for ListPool<T> {
    fn eq(&self, other: &Self) -> bool {
        // ignore the free list
        self.data == other.data
    }
}

impl<T: core::hash::Hash + EntityRef + ReservedValue> core::hash::Hash for ListPool<T> {
    fn hash<H: __core::hash::Hasher>(&self, state: &mut H) {
        // ignore the free list
        self.data.hash(state);
    }
}

impl<T: EntityRef + ReservedValue> Default for ListPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Lists are allocated in sizes that are powers of two, starting from 4.
/// Each power of two is assigned a size class number, so the size is `4 << SizeClass`.
type SizeClass = u8;

/// Get the size of a given size class. The size includes the length field, so the maximum list
/// length is one less than the class size.
#[inline]
fn sclass_size(sclass: SizeClass) -> usize {
    4 << sclass
}

/// Get the size class to use for a given list length.
/// This always leaves room for the length element in addition to the list elements.
#[inline]
fn sclass_for_length(len: usize) -> SizeClass {
    30 - (len as u32 | 3).leading_zeros() as SizeClass
}

/// Is `len` the minimum length in its size class?
#[inline]
fn is_sclass_min_length(len: usize) -> bool {
    len > 3 && len.is_power_of_two()
}

impl<T: EntityRef + ReservedValue> ListPool<T> {
    /// Create a new list pool.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            free: Vec::new(),
        }
    }

    /// Create a new list pool with the given capacity for data pre-allocated.
    pub fn with_capacity(len: usize) -> Self {
        Self {
            data: Vec::with_capacity(len),
            free: Vec::new(),
        }
    }

    /// Get the capacity of this pool. This will be somewhat higher
    /// than the total length of lists that can be stored without
    /// reallocating, because of internal metadata overheads. It is
    /// mostly useful to allow another pool to be allocated that is
    /// likely to hold data transferred from this one without the need
    /// to grow.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Clear the pool, forgetting about all lists that use it.
    ///
    /// This invalidates any existing entity lists that used this pool to allocate memory.
    ///
    /// The pool's memory is not released to the operating system, but kept around for faster
    /// allocation in the future.
    pub fn clear(&mut self) {
        self.data.clear();
        self.free.clear();
    }

    /// Read the length of a list field, if it exists.
    fn len_of(&self, list: &EntityList<T>) -> Option<usize> {
        let idx = list.index as usize;
        // `idx` points at the list elements. The list length is encoded in the element immediately
        // before the list elements.
        //
        // The `wrapping_sub` handles the special case 0, which is the empty list. This way, the
        // cost of the bounds check that we have to pay anyway is co-opted to handle the special
        // case of the empty list.
        self.data.get(idx.wrapping_sub(1)).map(|len| len.index())
    }

    /// Allocate a storage block with a size given by `sclass`.
    ///
    /// Returns the first index of an available segment of `self.data` containing
    /// `sclass_size(sclass)` elements. The allocated memory is filled with reserved
    /// values.
    fn alloc(&mut self, sclass: SizeClass) -> usize {
        // First try the free list for this size class.
        match self.free.get(sclass as usize).cloned() {
            Some(head) if head > 0 => {
                // The free list pointers are offset by 1, using 0 to terminate the list.
                // A block on the free list has two entries: `[ 0, next ]`.
                // The 0 is where the length field would be stored for a block in use.
                // The free list heads and the next pointer point at the `next` field.
                self.free[sclass as usize] = self.data[head].index();
                head - 1
            }
            _ => {
                // Nothing on the free list. Allocate more memory.
                let offset = self.data.len();
                self.data
                    .resize(offset + sclass_size(sclass), T::reserved_value());
                offset
            }
        }
    }

    /// Free a storage block with a size given by `sclass`.
    ///
    /// This must be a block that was previously allocated by `alloc()` with the same size class.
    fn free(&mut self, block: usize, sclass: SizeClass) {
        let sclass = sclass as usize;

        // Make sure we have a free-list head for `sclass`.
        if self.free.len() <= sclass {
            self.free.resize(sclass + 1, 0);
        }

        // Make sure the length field is cleared.
        self.data[block] = T::new(0);
        // Insert the block on the free list which is a single linked list.
        self.data[block + 1] = T::new(self.free[sclass]);
        self.free[sclass] = block + 1
    }

    /// Returns two mutable slices representing the two requested blocks.
    ///
    /// The two returned slices can be longer than the blocks. Each block is located at the front
    /// of the respective slice.
    fn mut_slices(&mut self, block0: usize, block1: usize) -> (&mut [T], &mut [T]) {
        if block0 < block1 {
            let (s0, s1) = self.data.split_at_mut(block1);
            (&mut s0[block0..], s1)
        } else {
            let (s1, s0) = self.data.split_at_mut(block0);
            (s0, &mut s1[block1..])
        }
    }

    /// Reallocate a block to a different size class.
    ///
    /// Copy `elems_to_copy` elements from the old to the new block.
    fn realloc(
        &mut self,
        block: usize,
        from_sclass: SizeClass,
        to_sclass: SizeClass,
        elems_to_copy: usize,
    ) -> usize {
        debug_assert!(elems_to_copy <= sclass_size(from_sclass));
        debug_assert!(elems_to_copy <= sclass_size(to_sclass));
        let new_block = self.alloc(to_sclass);

        if elems_to_copy > 0 {
            let (old, new) = self.mut_slices(block, new_block);
            (&mut new[0..elems_to_copy]).copy_from_slice(&old[0..elems_to_copy]);
        }

        self.free(block, from_sclass);
        new_block
    }
}

impl<T: EntityRef + ReservedValue> EntityList<T> {
    /// Create a new empty list.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new list with the contents initialized from a slice.
    pub fn from_slice(slice: &[T], pool: &mut ListPool<T>) -> Self {
        let len = slice.len();
        if len == 0 {
            return Self::new();
        }

        let block = pool.alloc(sclass_for_length(len));
        pool.data[block] = T::new(len);
        pool.data[block + 1..=block + len].copy_from_slice(slice);

        Self {
            index: (block + 1) as u32,
            unused: PhantomData,
        }
    }

    /// Returns `true` if the list has a length of 0.
    pub fn is_empty(&self) -> bool {
        // 0 is a magic value for the empty list. Any list in the pool array must have a positive
        // length.
        self.index == 0
    }

    /// Get the number of elements in the list.
    pub fn len(&self, pool: &ListPool<T>) -> usize {
        // Both the empty list and any invalidated old lists will return `None`.
        pool.len_of(self).unwrap_or(0)
    }

    /// Returns `true` if the list is valid
    pub fn is_valid(&self, pool: &ListPool<T>) -> bool {
        // We consider an empty list to be valid
        self.is_empty() || pool.len_of(self) != None
    }

    /// Get the list as a slice.
    pub fn as_slice<'a>(&self, pool: &'a ListPool<T>) -> &'a [T] {
        let idx = self.index as usize;
        match pool.len_of(self) {
            None => &[],
            Some(len) => &pool.data[idx..idx + len],
        }
    }

    /// Get a single element from the list.
    pub fn get(&self, index: usize, pool: &ListPool<T>) -> Option<T> {
        self.as_slice(pool).get(index).cloned()
    }

    /// Get the first element from the list.
    pub fn first(&self, pool: &ListPool<T>) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(pool.data[self.index as usize])
        }
    }

    /// Get the list as a mutable slice.
    pub fn as_mut_slice<'a>(&'a mut self, pool: &'a mut ListPool<T>) -> &'a mut [T] {
        let idx = self.index as usize;
        match pool.len_of(self) {
            None => &mut [],
            Some(len) => &mut pool.data[idx..idx + len],
        }
    }

    /// Get a mutable reference to a single element from the list.
    pub fn get_mut<'a>(&'a mut self, index: usize, pool: &'a mut ListPool<T>) -> Option<&'a mut T> {
        self.as_mut_slice(pool).get_mut(index)
    }

    /// Create a deep clone of the list, which does not alias the original list.
    pub fn deep_clone(&self, pool: &mut ListPool<T>) -> Self {
        match pool.len_of(self) {
            None => return Self::new(),
            Some(len) => {
                let src = self.index as usize;
                let block = pool.alloc(sclass_for_length(len));
                pool.data[block] = T::new(len);
                pool.data.copy_within(src..src + len, block + 1);

                Self {
                    index: (block + 1) as u32,
                    unused: PhantomData,
                }
            }
        }
    }

    /// Removes all elements from the list.
    ///
    /// The memory used by the list is put back in the pool.
    pub fn clear(&mut self, pool: &mut ListPool<T>) {
        let idx = self.index as usize;
        match pool.len_of(self) {
            None => debug_assert_eq!(idx, 0, "Invalid pool"),
            Some(len) => pool.free(idx - 1, sclass_for_length(len)),
        }
        // Switch back to the empty list representation which has no storage.
        self.index = 0;
    }

    /// Take all elements from this list and return them as a new list. Leave this list empty.
    ///
    /// This is the equivalent of `Option::take()`.
    pub fn take(&mut self) -> Self {
        mem::replace(self, Default::default())
    }

    /// Appends an element to the back of the list.
    /// Returns the index where the element was inserted.
    pub fn push(&mut self, element: T, pool: &mut ListPool<T>) -> usize {
        let idx = self.index as usize;
        match pool.len_of(self) {
            None => {
                // This is an empty list. Allocate a block and set length=1.
                debug_assert_eq!(idx, 0, "Invalid pool");
                let block = pool.alloc(sclass_for_length(1));
                pool.data[block] = T::new(1);
                pool.data[block + 1] = element;
                self.index = (block + 1) as u32;
                0
            }
            Some(len) => {
                // Do we need to reallocate?
                let new_len = len + 1;
                let block;
                if is_sclass_min_length(new_len) {
                    // Reallocate, preserving length + all old elements.
                    let sclass = sclass_for_length(len);
                    block = pool.realloc(idx - 1, sclass, sclass + 1, len + 1);
                    self.index = (block + 1) as u32;
                } else {
                    block = idx - 1;
                }
                pool.data[block + new_len] = element;
                pool.data[block] = T::new(new_len);
                len
            }
        }
    }

    /// Grow list by adding `count` reserved-value elements at the end.
    ///
    /// Returns a mutable slice representing the whole list.
    fn grow<'a>(&'a mut self, count: usize, pool: &'a mut ListPool<T>) -> &'a mut [T] {
        let idx = self.index as usize;
        let new_len;
        let block;
        match pool.len_of(self) {
            None => {
                // This is an empty list. Allocate a block.
                debug_assert_eq!(idx, 0, "Invalid pool");
                if count == 0 {
                    return &mut [];
                }
                new_len = count;
                block = pool.alloc(sclass_for_length(new_len));
                self.index = (block + 1) as u32;
            }
            Some(len) => {
                // Do we need to reallocate?
                let sclass = sclass_for_length(len);
                new_len = len + count;
                let new_sclass = sclass_for_length(new_len);
                if new_sclass != sclass {
                    block = pool.realloc(idx - 1, sclass, new_sclass, len + 1);
                    self.index = (block + 1) as u32;
                } else {
                    block = idx - 1;
                }
            }
        }
        pool.data[block] = T::new(new_len);
        &mut pool.data[block + 1..block + 1 + new_len]
    }

    /// Constructs a list from an iterator.
    pub fn from_iter<I>(elements: I, pool: &mut ListPool<T>) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = Self::new();
        list.extend(elements, pool);
        list
    }

    /// Appends multiple elements to the back of the list.
    pub fn extend<I>(&mut self, elements: I, pool: &mut ListPool<T>)
    where
        I: IntoIterator<Item = T>,
    {
        let iterator = elements.into_iter();
        let (len, upper) = iterator.size_hint();
        // On most iterators this check is optimized down to `true`.
        if upper == Some(len) {
            let data = self.grow(len, pool);
            let offset = data.len() - len;
            for (src, dst) in iterator.zip(data[offset..].iter_mut()) {
                *dst = src;
            }
        } else {
            for x in iterator {
                self.push(x, pool);
            }
        }
    }

    /// Copies a slice from an entity list in the same pool to a position in this one.
    ///
    /// Will panic if `self` is the same list as `other`.
    pub fn copy_from(
        &mut self,
        other: &Self,
        slice: impl core::ops::RangeBounds<usize>,
        index: usize,
        pool: &mut ListPool<T>,
    ) {
        assert!(
            index <= self.len(pool),
            "attempted to copy a slice out of bounds of `self`"
        );
        assert_ne!(
            self.index, other.index,
            "cannot copy within one `EntityList`"
        );

        let (other_index, other_len) = (other.index, other.len(pool));

        let start = match slice.start_bound() {
            core::ops::Bound::Included(&x) => x,
            core::ops::Bound::Excluded(&x) => x + 1,
            core::ops::Bound::Unbounded => 0,
        } + other_index as usize;
        let end = match slice.end_bound() {
            core::ops::Bound::Included(&x) => x + 1,
            core::ops::Bound::Excluded(&x) => x,
            core::ops::Bound::Unbounded => other_len,
        } + other_index as usize;
        let count = end - start;
        assert!(
            count <= other_len,
            "attempted to copy a slice from out of bounds of `other`"
        );

        self.grow_at(index, count, pool);
        pool.data
            .copy_within(start..end, index + self.index as usize);
    }

    /// Inserts an element as position `index` in the list, shifting all elements after it to the
    /// right.
    pub fn insert(&mut self, index: usize, element: T, pool: &mut ListPool<T>) {
        // Increase size by 1.
        self.push(element, pool);

        // Move tail elements.
        let seq = self.as_mut_slice(pool);
        if index < seq.len() {
            let tail = &mut seq[index..];
            for i in (1..tail.len()).rev() {
                tail[i] = tail[i - 1];
            }
            tail[0] = element;
        } else {
            debug_assert_eq!(index, seq.len());
        }
    }

    /// Removes the last element from the list.
    fn remove_last(&mut self, len: usize, pool: &mut ListPool<T>) {
        // Check if we deleted the last element.
        if len == 1 {
            self.clear(pool);
            return;
        }

        // Do we need to reallocate to a smaller size class?
        let mut block = self.index as usize - 1;
        if is_sclass_min_length(len) {
            let sclass = sclass_for_length(len);
            block = pool.realloc(block, sclass, sclass - 1, len);
            self.index = (block + 1) as u32;
        }

        // Finally adjust the length.
        pool.data[block] = T::new(len - 1);
    }

    /// Removes the element at position `index` from the list. Potentially linear complexity.
    pub fn remove(&mut self, index: usize, pool: &mut ListPool<T>) {
        let len;
        {
            let seq = self.as_mut_slice(pool);
            len = seq.len();
            debug_assert!(index < len);

            // Copy elements down.
            for i in index..len - 1 {
                seq[i] = seq[i + 1];
            }
        }

        self.remove_last(len, pool);
    }

    /// Removes the element at `index` in constant time by switching it with the last element of
    /// the list.
    pub fn swap_remove(&mut self, index: usize, pool: &mut ListPool<T>) {
        let seq = self.as_mut_slice(pool);
        let len = seq.len();
        debug_assert!(index < len);
        if index != len - 1 {
            seq.swap(index, len - 1);
        }

        self.remove_last(len, pool);
    }

    /// Shortens the list down to `len` elements.
    ///
    /// Does nothing if the list is already shorter than `len`.
    pub fn truncate(&mut self, new_len: usize, pool: &mut ListPool<T>) {
        if new_len == 0 {
            self.clear(pool);
            return;
        }

        match pool.len_of(self) {
            None => return,
            Some(len) => {
                if len <= new_len {
                    return;
                }

                let block;
                let idx = self.index as usize;
                let sclass = sclass_for_length(len);
                let new_sclass = sclass_for_length(new_len);
                if sclass != new_sclass {
                    block = pool.realloc(idx - 1, sclass, new_sclass, new_len + 1);
                    self.index = (block + 1) as u32;
                } else {
                    block = idx - 1;
                }
                pool.data[block] = T::new(new_len);
            }
        }
    }

    /// Grow the list by inserting `count` elements at `index`.
    ///
    /// The new elements are not initialized, they will contain whatever happened to be in memory.
    /// Since the memory comes from the pool, this will be either zero entity references or
    /// whatever where in a previously deallocated list.
    pub fn grow_at(&mut self, index: usize, count: usize, pool: &mut ListPool<T>) {
        let data = self.grow(count, pool);

        // Copy elements after `index` up.
        for i in (index + count..data.len()).rev() {
            data[i] = data[i - count];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An opaque reference to an instruction in a function.
    #[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Inst(u32);
    entity_impl!(Inst, "inst");

    #[test]
    fn size_classes() {
        assert_eq!(sclass_size(0), 4);
        assert_eq!(sclass_for_length(0), 0);
        assert_eq!(sclass_for_length(1), 0);
        assert_eq!(sclass_for_length(2), 0);
        assert_eq!(sclass_for_length(3), 0);
        assert_eq!(sclass_for_length(4), 1);
        assert_eq!(sclass_for_length(7), 1);
        assert_eq!(sclass_for_length(8), 2);
        assert_eq!(sclass_size(1), 8);
        for l in 0..300 {
            assert!(sclass_size(sclass_for_length(l)) >= l + 1);
        }
    }

    #[test]
    fn block_allocator() {
        let mut pool = ListPool::<Inst>::new();
        let b1 = pool.alloc(0);
        let b2 = pool.alloc(1);
        let b3 = pool.alloc(0);
        assert_ne!(b1, b2);
        assert_ne!(b1, b3);
        assert_ne!(b2, b3);
        pool.free(b2, 1);
        let b2a = pool.alloc(1);
        let b2b = pool.alloc(1);
        assert_ne!(b2a, b2b);
        // One of these should reuse the freed block.
        assert!(b2a == b2 || b2b == b2);

        // Check the free lists for a size class smaller than the largest seen so far.
        pool.free(b1, 0);
        pool.free(b3, 0);
        let b1a = pool.alloc(0);
        let b3a = pool.alloc(0);
        assert_ne!(b1a, b3a);
        assert!(b1a == b1 || b1a == b3);
        assert!(b3a == b1 || b3a == b3);
    }

    #[test]
    fn empty_list() {
        let pool = &mut ListPool::<Inst>::new();
        let mut list = EntityList::<Inst>::default();
        {
            let ilist = &list;
            assert!(ilist.is_empty());
            assert_eq!(ilist.len(pool), 0);
            assert_eq!(ilist.as_slice(pool), &[]);
            assert_eq!(ilist.get(0, pool), None);
            assert_eq!(ilist.get(100, pool), None);
        }
        assert_eq!(list.as_mut_slice(pool), &[]);
        assert_eq!(list.get_mut(0, pool), None);
        assert_eq!(list.get_mut(100, pool), None);

        list.clear(pool);
        assert!(list.is_empty());
        assert_eq!(list.len(pool), 0);
        assert_eq!(list.as_slice(pool), &[]);
        assert_eq!(list.first(pool), None);
    }

    #[test]
    fn from_slice() {
        let pool = &mut ListPool::<Inst>::new();

        let list = EntityList::<Inst>::from_slice(&[Inst(0), Inst(1)], pool);
        assert!(!list.is_empty());
        assert_eq!(list.len(pool), 2);
        assert_eq!(list.as_slice(pool), &[Inst(0), Inst(1)]);
        assert_eq!(list.get(0, pool), Some(Inst(0)));
        assert_eq!(list.get(100, pool), None);

        let list = EntityList::<Inst>::from_slice(&[], pool);
        assert!(list.is_empty());
        assert_eq!(list.len(pool), 0);
        assert_eq!(list.as_slice(pool), &[]);
        assert_eq!(list.get(0, pool), None);
        assert_eq!(list.get(100, pool), None);
    }

    #[test]
    fn push() {
        let pool = &mut ListPool::<Inst>::new();
        let mut list = EntityList::<Inst>::default();

        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let i3 = Inst::new(3);
        let i4 = Inst::new(4);

        assert_eq!(list.push(i1, pool), 0);
        assert_eq!(list.len(pool), 1);
        assert!(!list.is_empty());
        assert_eq!(list.as_slice(pool), &[i1]);
        assert_eq!(list.first(pool), Some(i1));
        assert_eq!(list.get(0, pool), Some(i1));
        assert_eq!(list.get(1, pool), None);

        assert_eq!(list.push(i2, pool), 1);
        assert_eq!(list.len(pool), 2);
        assert!(!list.is_empty());
        assert_eq!(list.as_slice(pool), &[i1, i2]);
        assert_eq!(list.first(pool), Some(i1));
        assert_eq!(list.get(0, pool), Some(i1));
        assert_eq!(list.get(1, pool), Some(i2));
        assert_eq!(list.get(2, pool), None);

        assert_eq!(list.push(i3, pool), 2);
        assert_eq!(list.len(pool), 3);
        assert!(!list.is_empty());
        assert_eq!(list.as_slice(pool), &[i1, i2, i3]);
        assert_eq!(list.first(pool), Some(i1));
        assert_eq!(list.get(0, pool), Some(i1));
        assert_eq!(list.get(1, pool), Some(i2));
        assert_eq!(list.get(2, pool), Some(i3));
        assert_eq!(list.get(3, pool), None);

        // This triggers a reallocation.
        assert_eq!(list.push(i4, pool), 3);
        assert_eq!(list.len(pool), 4);
        assert!(!list.is_empty());
        assert_eq!(list.as_slice(pool), &[i1, i2, i3, i4]);
        assert_eq!(list.first(pool), Some(i1));
        assert_eq!(list.get(0, pool), Some(i1));
        assert_eq!(list.get(1, pool), Some(i2));
        assert_eq!(list.get(2, pool), Some(i3));
        assert_eq!(list.get(3, pool), Some(i4));
        assert_eq!(list.get(4, pool), None);

        list.extend([i1, i1, i2, i2, i3, i3, i4, i4].iter().cloned(), pool);
        assert_eq!(list.len(pool), 12);
        assert_eq!(
            list.as_slice(pool),
            &[i1, i2, i3, i4, i1, i1, i2, i2, i3, i3, i4, i4]
        );

        let list2 = EntityList::from_iter([i1, i1, i2, i2, i3, i3, i4, i4].iter().cloned(), pool);
        assert_eq!(list2.len(pool), 8);
        assert_eq!(list2.as_slice(pool), &[i1, i1, i2, i2, i3, i3, i4, i4]);
    }

    #[test]
    fn insert_remove() {
        let pool = &mut ListPool::<Inst>::new();
        let mut list = EntityList::<Inst>::default();

        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let i3 = Inst::new(3);
        let i4 = Inst::new(4);

        list.insert(0, i4, pool);
        assert_eq!(list.as_slice(pool), &[i4]);

        list.insert(0, i3, pool);
        assert_eq!(list.as_slice(pool), &[i3, i4]);

        list.insert(2, i2, pool);
        assert_eq!(list.as_slice(pool), &[i3, i4, i2]);

        list.insert(2, i1, pool);
        assert_eq!(list.as_slice(pool), &[i3, i4, i1, i2]);

        list.remove(3, pool);
        assert_eq!(list.as_slice(pool), &[i3, i4, i1]);

        list.remove(2, pool);
        assert_eq!(list.as_slice(pool), &[i3, i4]);

        list.remove(0, pool);
        assert_eq!(list.as_slice(pool), &[i4]);

        list.remove(0, pool);
        assert_eq!(list.as_slice(pool), &[]);
        assert!(list.is_empty());
    }

    #[test]
    fn growing() {
        let pool = &mut ListPool::<Inst>::new();
        let mut list = EntityList::<Inst>::default();

        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let i3 = Inst::new(3);
        let i4 = Inst::new(4);

        // This is not supposed to change the list.
        list.grow_at(0, 0, pool);
        assert_eq!(list.len(pool), 0);
        assert!(list.is_empty());

        list.grow_at(0, 2, pool);
        assert_eq!(list.len(pool), 2);

        list.as_mut_slice(pool).copy_from_slice(&[i2, i3]);

        list.grow_at(1, 0, pool);
        assert_eq!(list.as_slice(pool), &[i2, i3]);

        list.grow_at(1, 1, pool);
        list.as_mut_slice(pool)[1] = i1;
        assert_eq!(list.as_slice(pool), &[i2, i1, i3]);

        // Append nothing at the end.
        list.grow_at(3, 0, pool);
        assert_eq!(list.as_slice(pool), &[i2, i1, i3]);

        // Append something at the end.
        list.grow_at(3, 1, pool);
        list.as_mut_slice(pool)[3] = i4;
        assert_eq!(list.as_slice(pool), &[i2, i1, i3, i4]);
    }

    #[test]
    fn deep_clone() {
        let pool = &mut ListPool::<Inst>::new();

        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let i3 = Inst::new(3);
        let i4 = Inst::new(4);

        let mut list1 = EntityList::from_slice(&[i1, i2, i3], pool);
        let list2 = list1.deep_clone(pool);
        assert_eq!(list1.as_slice(pool), &[i1, i2, i3]);
        assert_eq!(list2.as_slice(pool), &[i1, i2, i3]);

        list1.as_mut_slice(pool)[0] = i4;
        assert_eq!(list1.as_slice(pool), &[i4, i2, i3]);
        assert_eq!(list2.as_slice(pool), &[i1, i2, i3]);
    }

    #[test]
    fn truncate() {
        let pool = &mut ListPool::<Inst>::new();

        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let i3 = Inst::new(3);
        let i4 = Inst::new(4);

        let mut list = EntityList::from_slice(&[i1, i2, i3, i4, i1, i2, i3, i4], pool);
        assert_eq!(list.as_slice(pool), &[i1, i2, i3, i4, i1, i2, i3, i4]);
        list.truncate(6, pool);
        assert_eq!(list.as_slice(pool), &[i1, i2, i3, i4, i1, i2]);
        list.truncate(9, pool);
        assert_eq!(list.as_slice(pool), &[i1, i2, i3, i4, i1, i2]);
        list.truncate(2, pool);
        assert_eq!(list.as_slice(pool), &[i1, i2]);
        list.truncate(0, pool);
        assert!(list.is_empty());
    }

    #[test]
    fn copy_from() {
        let pool = &mut ListPool::<Inst>::new();

        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let i3 = Inst::new(3);
        let i4 = Inst::new(4);

        let mut list = EntityList::from_slice(&[i1, i2, i3, i4], pool);
        assert_eq!(list.as_slice(pool), &[i1, i2, i3, i4]);
        let list2 = EntityList::from_slice(&[i4, i3, i2, i1], pool);
        assert_eq!(list2.as_slice(pool), &[i4, i3, i2, i1]);
        list.copy_from(&list2, 0..0, 0, pool);
        assert_eq!(list.as_slice(pool), &[i1, i2, i3, i4]);
        list.copy_from(&list2, 0..4, 0, pool);
        assert_eq!(list.as_slice(pool), &[i4, i3, i2, i1, i1, i2, i3, i4]);
    }

    #[test]
    #[should_panic]
    fn copy_from_self() {
        let pool = &mut ListPool::<Inst>::new();

        let i1 = Inst::new(1);
        let i2 = Inst::new(2);
        let i3 = Inst::new(3);
        let i4 = Inst::new(4);

        let mut list = EntityList::from_slice(&[i4, i3, i2, i1, i1, i2, i3, i4], pool);
        let list_again = list;
        assert_eq!(list.as_slice(pool), &[i4, i3, i2, i1, i1, i2, i3, i4]);
        // Panic should occur on the line below because `list.index == other.index`
        list.copy_from(&list_again, 0..=1, 8, pool);
        assert_eq!(
            list.as_slice(pool),
            &[i4, i3, i2, i1, i1, i2, i3, i4, i4, i3]
        );
        list.copy_from(&list_again, .., 7, pool);
        assert_eq!(
            list.as_slice(pool),
            &[
                i4, i3, i2, i1, i1, i2, i4, i3, i2, i1, i1, i2, i3, i4, i4, i3, i3, i4, i4, i3
            ]
        )
    }
}
