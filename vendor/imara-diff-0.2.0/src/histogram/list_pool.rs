use crate::histogram::MAX_CHAIN_LEN;

/// A small list of entity references allocated from a pool.
///
/// An `ListHandle` type provides similar functionality to `Vec`, but with some important
/// differences in the implementation:
///
/// 1. Memory is allocated from a `ListPool` instead of the global heap.
/// 2. The footprint of an entity list is 4 bytes, compared with the 24 bytes for `Vec`.
/// 3. An entity list doesn't implement `Drop`, leaving it to the pool to manage memory.
///
/// The list pool is intended to be used as a LIFO allocator. After building up a larger data
/// structure with many list references, the whole thing can be discarded quickly by clearing the
/// pool.
///
/// # Safety
///
/// Entity lists are not as safe to use as `Vec`, but they never jeopardize Rust's memory safety
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
/// The `ListHandle` itself is designed to have the smallest possible footprint. This is important
/// because it is used inside very compact data structures like `InstructionData`. The list
/// contains only a 32-bit index into the pool's memory vector, pointing to the first element of
/// the list.
///
/// The pool is just a single `Vec` containing all of the allocated lists. Each list is
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
/// The index stored in an `ListHandle` points to part 2, the list elements. The value 0 is
/// reserved for the empty list which isn't allocated in the vector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListHandle {
    index: u32,
    generation: u32,
    len: u32,
}

/// Create an empty list.
impl Default for ListHandle {
    fn default() -> Self {
        Self {
            index: 0,
            generation: 0,
            len: 0,
        }
    }
}

const MAX_SIZE_CLASS: SizeClass = sclass_for_length(super::MAX_CHAIN_LEN - 1);
const NUM_SIZE_CLASS: usize = MAX_SIZE_CLASS as usize + 1;

/// A memory pool for storing lists of `T`.
#[derive(Clone, Debug)]
pub struct ListPool {
    // The main array containing the lists.
    data: Vec<u32>,

    // Heads of the free lists, one for each size class.
    free: [u32; NUM_SIZE_CLASS],

    generation: u32,
}

/// Lists are allocated in sizes that are powers of two, starting from 4.
/// Each power of two is assigned a size class number, so the size is `4 << SizeClass`.
type SizeClass = u8;

/// Get the size of a given size class. The size includes the length field, so the maximum list
/// length is one less than the class size.
#[inline]
const fn sclass_size(sclass: SizeClass) -> usize {
    4 << sclass
}

/// Get the size class to use for a given list length.
/// This always leaves room for the length element in addition to the list elements.
#[inline]
const fn sclass_for_length(len: u32) -> SizeClass {
    30 - (len | 3).leading_zeros() as SizeClass
}

/// Is `len` the minimum length in its size class?
#[inline]
fn is_sclass_max_length(len: u32) -> bool {
    len > 3 && len.is_power_of_two()
}

impl ListPool {
    /// Create a new list pool.
    pub fn new(capacity: u32) -> Self {
        Self {
            data: Vec::with_capacity(capacity as usize),
            free: [u32::MAX; NUM_SIZE_CLASS],
            generation: 1,
        }
    }

    /// Clear the pool, forgetting about all lists that use it.
    ///
    /// This invalidates any existing entity lists that used this pool to allocate memory.
    ///
    /// The pool's memory is not released to the operating system, but kept around for faster
    /// allocation in the future.
    pub fn clear(&mut self) {
        self.data.clear();
        self.free.fill(u32::MAX);
        self.generation += 1;
    }

    /// Allocate a storage block with a size given by `sclass`.
    ///
    /// Returns the first index of an available segment of `self.data` containing
    /// `sclass_size(sclass)` elements. The allocated memory is filled with reserved
    /// values.
    fn alloc(&mut self, sclass: SizeClass) -> usize {
        let freelist_head = self.free[sclass as usize];
        // First try the free list for this size class.
        if freelist_head == u32::MAX {
            // Nothing on the free list. Allocate more memory.
            let offset = self.data.len();
            self.data.resize(offset + sclass_size(sclass), u32::MAX);
            offset
        } else {
            // take allocation of the free list (linked list)
            self.free[sclass as usize] = self.data[freelist_head as usize];
            freelist_head as usize
        }
    }

    /// Free a storage block with a size given by `sclass`.
    ///
    /// This must be a block that was previously allocated by `alloc()` with the same size class.
    fn free(&mut self, block: usize, sclass: SizeClass) {
        let sclass = sclass as usize;
        // Insert the block on the free list which is a single linked list.
        self.data[block] = self.free[sclass];
        self.free[sclass] = block as u32
    }

    /// Returns two mutable slices representing the two requested blocks.
    ///
    /// The two returned slices can be longer than the blocks. Each block is located at the front
    /// of the respective slice.
    fn mut_slices(&mut self, block0: usize, block1: usize) -> (&mut [u32], &mut [u32]) {
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

        let (old, new) = self.mut_slices(block, new_block);
        new[0..elems_to_copy].copy_from_slice(&old[0..elems_to_copy]);

        self.free(block, from_sclass);
        new_block
    }
}

impl ListHandle {
    /// Get the number of elements in the list.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self, pool: &ListPool) -> u32 {
        if self.generation == pool.generation {
            self.len
        } else {
            0
        }
    }

    /// Get the list as a slice.
    pub fn as_slice<'a>(&'a self, pool: &'a ListPool) -> &'a [u32] {
        let idx = self.index as usize;
        match self.len(pool) {
            0 => &[],
            1 => std::slice::from_ref(&self.index),
            len => &pool.data[idx..idx + len as usize],
        }
    }

    /// Appends an element to the back of the list.
    /// Returns the index where the element was inserted.
    pub fn push(&mut self, element: u32, pool: &mut ListPool) {
        let len = self.len(pool);
        match len {
            0 => {
                self.generation = pool.generation;
                self.index = element;
                self.len = 1;
            }
            1 => {
                // This is an empty list. Allocate a block and set length=1.
                let block = pool.alloc(0);
                pool.data[block] = self.index;
                pool.data[block + 1] = element;
                self.index = block as u32;
                self.len = 2;
            }
            2..=MAX_CHAIN_LEN => {
                // Do we need to reallocate?
                let block;
                let idx = self.index as usize;
                if is_sclass_max_length(len) {
                    // Reallocate, preserving length + all old elements.
                    let sclass = sclass_for_length(len);
                    block = pool.realloc(idx, sclass - 1, sclass, len as usize);
                    self.index = block as u32;
                } else {
                    block = idx;
                }
                pool.data[block + len as usize] = element;
                self.len += 1;
            }

            // ignore elements longer then MAX_CHAIN_LEN
            // these are rarely relevant and if they are we fall back to myers
            _ => (),
        }
    }
}
