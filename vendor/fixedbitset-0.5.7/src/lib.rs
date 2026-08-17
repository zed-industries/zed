//! `FixedBitSet` is a simple fixed size set of bits.
//!
//! ### Crate features
//!
//! - `std` (default feature)  
//!   Disabling this feature disables using std and instead uses crate alloc.
//!
//! ### SIMD Acceleration
//! `fixedbitset` is written with SIMD in mind. The backing store and set operations will use aligned SIMD data types and instructions when compiling
//! for compatible target platforms. The use of SIMD generally enables better performance in many set and batch operations (i.e. intersection/union/inserting a range).
//!
//!  When SIMD is not available on the target, the crate will gracefully fallback to a default implementation.  It is intended to add support for other SIMD architectures
//! once they appear in stable Rust.
//!
//! Currently only SSE2/AVX/AVX2 on x86/x86_64 and wasm32 SIMD are supported as this is what stable Rust supports.
#![no_std]
#![deny(clippy::undocumented_unsafe_blocks)]

extern crate alloc;
use alloc::{vec, vec::Vec};

mod block;
mod range;

#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde")]
mod serde_impl;

use core::fmt::Write;
use core::fmt::{Binary, Display, Error, Formatter};

use core::cmp::Ordering;
use core::hash::Hash;
use core::iter::{Chain, FusedIterator};
use core::mem::ManuallyDrop;
use core::mem::MaybeUninit;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index};
use core::ptr::NonNull;
pub use range::IndexRange;

pub(crate) const BITS: usize = core::mem::size_of::<Block>() * 8;
#[cfg(feature = "serde")]
pub(crate) const BYTES: usize = core::mem::size_of::<Block>();

use block::Block as SimdBlock;
pub type Block = usize;

#[inline]
fn div_rem(x: usize, denominator: usize) -> (usize, usize) {
    (x / denominator, x % denominator)
}

fn vec_into_parts<T>(vec: Vec<T>) -> (NonNull<T>, usize, usize) {
    let mut vec = ManuallyDrop::new(vec);
    (
        // SAFETY: A Vec's internal pointer is always non-null.
        unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) },
        vec.capacity(),
        vec.len(),
    )
}

/// `FixedBitSet` is a simple fixed size set of bits that each can
/// be enabled (1 / **true**) or disabled (0 / **false**).
///
/// The bit set has a fixed capacity in terms of enabling bits (and the
/// capacity can grow using the `grow` method).
///
/// Derived traits depend on both the zeros and ones, so [0,1] is not equal to
/// [0,1,0].
#[derive(Debug, Eq)]
pub struct FixedBitSet {
    pub(crate) data: NonNull<MaybeUninit<SimdBlock>>,
    capacity: usize,
    /// length in bits
    pub(crate) length: usize,
}

// SAFETY: FixedBitset contains no thread-local state and can be safely sent between threads
unsafe impl Send for FixedBitSet {}
// SAFETY: FixedBitset does not provide simultaneous unsynchronized mutable access to the
// underlying buffer.
unsafe impl Sync for FixedBitSet {}

impl FixedBitSet {
    /// Create a new empty **FixedBitSet**.
    pub const fn new() -> Self {
        FixedBitSet {
            data: NonNull::dangling(),
            capacity: 0,
            length: 0,
        }
    }

    /// Create a new **FixedBitSet** with a specific number of bits,
    /// all initially clear.
    pub fn with_capacity(bits: usize) -> Self {
        let (mut blocks, rem) = div_rem(bits, SimdBlock::BITS);
        blocks += (rem > 0) as usize;
        Self::from_blocks_and_len(vec![SimdBlock::NONE; blocks], bits)
    }

    #[inline]
    fn from_blocks_and_len(data: Vec<SimdBlock>, length: usize) -> Self {
        let (data, capacity, _) = vec_into_parts(data);
        FixedBitSet {
            data: data.cast(),
            capacity,
            length,
        }
    }

    /// Create a new **FixedBitSet** with a specific number of bits,
    /// initialized from provided blocks.
    ///
    /// If the blocks are not the exact size needed for the capacity
    /// they will be padded with zeros (if shorter) or truncated to
    /// the capacity (if longer).
    ///
    /// For example:
    /// ```
    /// let data = vec![4];
    /// let bs = fixedbitset::FixedBitSet::with_capacity_and_blocks(4, data);
    /// assert_eq!(format!("{:b}", bs), "0010");
    /// ```
    pub fn with_capacity_and_blocks<I: IntoIterator<Item = Block>>(bits: usize, blocks: I) -> Self {
        let mut bitset = Self::with_capacity(bits);
        for (subblock, value) in bitset.as_mut_slice().iter_mut().zip(blocks.into_iter()) {
            *subblock = value;
        }
        bitset
    }

    /// Grow capacity to **bits**, all new bits initialized to zero
    #[inline]
    pub fn grow(&mut self, bits: usize) {
        #[cold]
        #[track_caller]
        #[inline(never)]
        fn do_grow(slf: &mut FixedBitSet, bits: usize) {
            // SAFETY: The provided fill is initialized to NONE.
            unsafe { slf.grow_inner(bits, MaybeUninit::new(SimdBlock::NONE)) };
        }

        if bits > self.length {
            do_grow(self, bits);
        }
    }

    /// # Safety
    /// If `fill` is uninitialized, the memory must not be accessed and must be immediately
    /// written over
    #[inline(always)]
    unsafe fn grow_inner(&mut self, bits: usize, fill: MaybeUninit<SimdBlock>) {
        // SAFETY: The data pointer and capacity were created from a Vec initially. The block
        // len is identical to that of the original.
        let mut data = unsafe {
            Vec::from_raw_parts(self.data.as_ptr(), self.simd_block_len(), self.capacity)
        };
        let (mut blocks, rem) = div_rem(bits, SimdBlock::BITS);
        blocks += (rem > 0) as usize;
        data.resize(blocks, fill);
        let (data, capacity, _) = vec_into_parts(data);
        self.data = data;
        self.capacity = capacity;
        self.length = bits;
    }

    #[inline]
    unsafe fn get_unchecked(&self, subblock: usize) -> &Block {
        &*self.data.as_ptr().cast::<Block>().add(subblock)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, subblock: usize) -> &mut Block {
        &mut *self.data.as_ptr().cast::<Block>().add(subblock)
    }

    #[inline]
    fn usize_len(&self) -> usize {
        let (mut blocks, rem) = div_rem(self.length, BITS);
        blocks += (rem > 0) as usize;
        blocks
    }

    #[inline]
    fn simd_block_len(&self) -> usize {
        let (mut blocks, rem) = div_rem(self.length, SimdBlock::BITS);
        blocks += (rem > 0) as usize;
        blocks
    }

    #[inline]
    fn batch_count_ones(blocks: impl IntoIterator<Item = Block>) -> usize {
        blocks.into_iter().map(|x| x.count_ones() as usize).sum()
    }

    #[inline]
    fn as_simd_slice(&self) -> &[SimdBlock] {
        // SAFETY: The slice constructed is within bounds of the underlying allocation. This function
        // is called with a read-only borrow so no other write can happen as long as the returned borrow lives.
        unsafe { core::slice::from_raw_parts(self.data.as_ptr().cast(), self.simd_block_len()) }
    }

    #[inline]
    fn as_mut_simd_slice(&mut self) -> &mut [SimdBlock] {
        // SAFETY: The slice constructed is within bounds of the underlying allocation. This function
        // is called with a mutable borrow so no other read or write can happen as long as the returned borrow lives.
        unsafe { core::slice::from_raw_parts_mut(self.data.as_ptr().cast(), self.simd_block_len()) }
    }

    #[inline]
    fn as_simd_slice_uninit(&self) -> &[MaybeUninit<SimdBlock>] {
        // SAFETY: The slice constructed is within bounds of the underlying allocation. This function
        // is called with a read-only borrow so no other write can happen as long as the returned borrow lives.
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.simd_block_len()) }
    }

    #[inline]
    fn as_mut_simd_slice_uninit(&mut self) -> &mut [MaybeUninit<SimdBlock>] {
        // SAFETY: The slice constructed is within bounds of the underlying allocation. This function
        // is called with a mutable borrow so no other read or write can happen as long as the returned borrow lives.
        unsafe { core::slice::from_raw_parts_mut(self.data.as_ptr(), self.simd_block_len()) }
    }

    /// Grows the internal size of the bitset before inserting a bit
    ///
    /// Unlike `insert`, this cannot panic, but may allocate if the bit is outside of the existing buffer's range.
    ///
    /// This is faster than calling `grow` then `insert` in succession.
    #[inline]
    pub fn grow_and_insert(&mut self, bits: usize) {
        self.grow(bits + 1);

        let (blocks, rem) = div_rem(bits, BITS);
        // SAFETY: The above grow ensures that the block is inside the Vec's allocation.
        unsafe {
            *self.get_unchecked_mut(blocks) |= 1 << rem;
        }
    }

    /// The length of the [`FixedBitSet`] in bits.
    ///
    /// Note: `len` includes both set and unset bits.
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let bitset = FixedBitSet::with_capacity(10);
    /// // there are 0 set bits, but 10 unset bits
    /// assert_eq!(bitset.len(), 10);
    /// ```
    /// `len` does not return the count of set bits. For that, use
    /// [`bitset.count_ones(..)`](FixedBitSet::count_ones) instead.
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    /// `true` if the [`FixedBitSet`] is empty.
    ///
    /// Note that an "empty" `FixedBitSet` is a `FixedBitSet` with
    /// no bits (meaning: it's length is zero). If you want to check
    /// if all bits are unset, use [`FixedBitSet::is_clear`].
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let bitset = FixedBitSet::with_capacity(10);
    /// assert!(!bitset.is_empty());
    ///
    /// let bitset = FixedBitSet::with_capacity(0);
    /// assert!(bitset.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// `true` if all bits in the [`FixedBitSet`] are unset.
    ///
    /// As opposed to [`FixedBitSet::is_empty`], which is `true` only for
    /// sets without any bits, set or unset.
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let mut bitset = FixedBitSet::with_capacity(10);
    /// assert!(bitset.is_clear());
    ///
    /// bitset.insert(2);
    /// assert!(!bitset.is_clear());
    /// ```
    ///
    /// This is equivalent to [`bitset.count_ones(..) == 0`](FixedBitSet::count_ones).
    #[inline]
    pub fn is_clear(&self) -> bool {
        self.as_simd_slice().iter().all(|block| block.is_empty())
    }

    /// Finds the lowest set bit in the bitset.
    ///
    /// Returns `None` if there aren't any set bits.
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let mut bitset = FixedBitSet::with_capacity(10);
    /// assert_eq!(bitset.minimum(), None);
    ///
    /// bitset.insert(2);
    /// assert_eq!(bitset.minimum(), Some(2));
    /// bitset.insert(8);
    /// assert_eq!(bitset.minimum(), Some(2));
    /// ```
    #[inline]
    pub fn minimum(&self) -> Option<usize> {
        let (block_idx, block) = self
            .as_simd_slice()
            .iter()
            .enumerate()
            .find(|&(_, block)| !block.is_empty())?;
        let mut inner = 0;
        let mut trailing = 0;
        for subblock in block.into_usize_array() {
            if subblock != 0 {
                trailing = subblock.trailing_zeros() as usize;
                break;
            } else {
                inner += BITS;
            }
        }
        Some(block_idx * SimdBlock::BITS + inner + trailing)
    }

    /// Finds the highest set bit in the bitset.
    ///
    /// Returns `None` if there aren't any set bits.
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let mut bitset = FixedBitSet::with_capacity(10);
    /// assert_eq!(bitset.maximum(), None);
    ///
    /// bitset.insert(8);
    /// assert_eq!(bitset.maximum(), Some(8));
    /// bitset.insert(2);
    /// assert_eq!(bitset.maximum(), Some(8));
    /// ```
    #[inline]
    pub fn maximum(&self) -> Option<usize> {
        let (block_idx, block) = self
            .as_simd_slice()
            .iter()
            .rev()
            .enumerate()
            .find(|&(_, block)| !block.is_empty())?;
        let mut inner = 0;
        let mut leading = 0;
        for subblock in block.into_usize_array().iter().rev() {
            if *subblock != 0 {
                leading = subblock.leading_zeros() as usize;
                break;
            } else {
                inner += BITS;
            }
        }
        let max = self.simd_block_len() * SimdBlock::BITS;
        Some(max - block_idx * SimdBlock::BITS - inner - leading - 1)
    }

    /// `true` if all bits in the [`FixedBitSet`] are set.
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let mut bitset = FixedBitSet::with_capacity(10);
    /// assert!(!bitset.is_full());
    ///
    /// bitset.insert_range(..);
    /// assert!(bitset.is_full());
    /// ```
    ///
    /// This is equivalent to [`bitset.count_ones(..) == bitset.len()`](FixedBitSet::count_ones).
    #[inline]
    pub fn is_full(&self) -> bool {
        self.contains_all_in_range(..)
    }

    /// Return **true** if the bit is enabled in the **FixedBitSet**,
    /// **false** otherwise.
    ///
    /// Note: bits outside the capacity are always disabled.
    ///
    /// Note: Also available with index syntax: `bitset[bit]`.
    #[inline]
    pub fn contains(&self, bit: usize) -> bool {
        (bit < self.length)
            // SAFETY: The above check ensures that the block and bit are within bounds.
            .then(|| unsafe { self.contains_unchecked(bit) })
            .unwrap_or(false)
    }

    /// Return **true** if the bit is enabled in the **FixedBitSet**,
    /// **false** otherwise.
    ///
    /// Note: unlike `contains`, calling this with an invalid `bit`
    /// is undefined behavior.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn contains_unchecked(&self, bit: usize) -> bool {
        let (block, i) = div_rem(bit, BITS);
        (self.get_unchecked(block) & (1 << i)) != 0
    }

    /// Clear all bits.
    #[inline]
    pub fn clear(&mut self) {
        for elt in self.as_mut_simd_slice().iter_mut() {
            *elt = SimdBlock::NONE
        }
    }

    /// Enable `bit`.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn insert(&mut self, bit: usize) {
        assert!(
            bit < self.length,
            "insert at index {} exceeds fixedbitset size {}",
            bit,
            self.length
        );
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe {
            self.insert_unchecked(bit);
        }
    }

    /// Enable `bit` without any length checks.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn insert_unchecked(&mut self, bit: usize) {
        let (block, i) = div_rem(bit, BITS);
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe {
            *self.get_unchecked_mut(block) |= 1 << i;
        }
    }

    /// Disable `bit`.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn remove(&mut self, bit: usize) {
        assert!(
            bit < self.length,
            "remove at index {} exceeds fixedbitset size {}",
            bit,
            self.length
        );
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe {
            self.remove_unchecked(bit);
        }
    }

    /// Disable `bit` without any bounds checking.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn remove_unchecked(&mut self, bit: usize) {
        let (block, i) = div_rem(bit, BITS);
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe {
            *self.get_unchecked_mut(block) &= !(1 << i);
        }
    }

    /// Enable `bit`, and return its previous value.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn put(&mut self, bit: usize) -> bool {
        assert!(
            bit < self.length,
            "put at index {} exceeds fixedbitset size {}",
            bit,
            self.length
        );
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe { self.put_unchecked(bit) }
    }

    /// Enable `bit`, and return its previous value without doing any bounds checking.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn put_unchecked(&mut self, bit: usize) -> bool {
        let (block, i) = div_rem(bit, BITS);
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe {
            let word = self.get_unchecked_mut(block);
            let prev = *word & (1 << i) != 0;
            *word |= 1 << i;
            prev
        }
    }

    /// Toggle `bit` (inverting its state).
    ///
    /// ***Panics*** if **bit** is out of bounds
    #[inline]
    pub fn toggle(&mut self, bit: usize) {
        assert!(
            bit < self.length,
            "toggle at index {} exceeds fixedbitset size {}",
            bit,
            self.length
        );
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe {
            self.toggle_unchecked(bit);
        }
    }

    /// Toggle `bit` (inverting its state) without any bounds checking.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn toggle_unchecked(&mut self, bit: usize) {
        let (block, i) = div_rem(bit, BITS);
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe {
            *self.get_unchecked_mut(block) ^= 1 << i;
        }
    }

    /// Sets a bit to the provided `enabled` value.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn set(&mut self, bit: usize, enabled: bool) {
        assert!(
            bit < self.length,
            "set at index {} exceeds fixedbitset size {}",
            bit,
            self.length
        );
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe {
            self.set_unchecked(bit, enabled);
        }
    }

    /// Sets a bit to the provided `enabled` value without doing any bounds checking.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn set_unchecked(&mut self, bit: usize, enabled: bool) {
        let (block, i) = div_rem(bit, BITS);
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        let elt = unsafe { self.get_unchecked_mut(block) };
        if enabled {
            *elt |= 1 << i;
        } else {
            *elt &= !(1 << i);
        }
    }

    /// Copies boolean value from specified bit to the specified bit.
    ///
    /// If `from` is out-of-bounds, `to` will be unset.
    ///
    /// **Panics** if **to** is out of bounds.
    #[inline]
    pub fn copy_bit(&mut self, from: usize, to: usize) {
        assert!(
            to < self.length,
            "copy to index {} exceeds fixedbitset size {}",
            to,
            self.length
        );
        let enabled = self.contains(from);
        // SAFETY: The above assertion ensures that the block is inside the Vec's allocation.
        unsafe { self.set_unchecked(to, enabled) };
    }

    /// Copies boolean value from specified bit to the specified bit.
    ///
    /// Note: unlike `copy_bit`, calling this with an invalid `from`
    /// is undefined behavior.
    ///
    /// # Safety
    /// `to` must both be less than `self.len()`
    #[inline]
    pub unsafe fn copy_bit_unchecked(&mut self, from: usize, to: usize) {
        // SAFETY: Caller must ensure that `from` is within bounds.
        let enabled = self.contains_unchecked(from);
        // SAFETY: Caller must ensure that `to` is within bounds.
        self.set_unchecked(to, enabled);
    }

    /// Count the number of set bits in the given bit range.
    ///
    /// This function is potentially much faster than using `ones(other).count()`.
    /// Use `..` to count the whole content of the bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn count_ones<T: IndexRange>(&self, range: T) -> usize {
        Self::batch_count_ones(Masks::new(range, self.length).map(|(block, mask)| {
            // SAFETY: Masks cannot return a block index that is out of range.
            unsafe { *self.get_unchecked(block) & mask }
        }))
    }

    /// Count the number of unset bits in the given bit range.
    ///
    /// This function is potentially much faster than using `zeroes(other).count()`.
    /// Use `..` to count the whole content of the bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn count_zeroes<T: IndexRange>(&self, range: T) -> usize {
        Self::batch_count_ones(Masks::new(range, self.length).map(|(block, mask)| {
            // SAFETY: Masks cannot return a block index that is out of range.
            unsafe { !*self.get_unchecked(block) & mask }
        }))
    }

    /// Sets every bit in the given range to the given state (`enabled`)
    ///
    /// Use `..` to set the whole bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn set_range<T: IndexRange>(&mut self, range: T, enabled: bool) {
        if enabled {
            self.insert_range(range);
        } else {
            self.remove_range(range);
        }
    }

    /// Enables every bit in the given range.
    ///
    /// Use `..` to make the whole bitset ones.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn insert_range<T: IndexRange>(&mut self, range: T) {
        for (block, mask) in Masks::new(range, self.length) {
            // SAFETY: Masks cannot return a block index that is out of range.
            let block = unsafe { self.get_unchecked_mut(block) };
            *block |= mask;
        }
    }

    /// Disables every bit in the given range.
    ///
    /// Use `..` to make the whole bitset ones.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn remove_range<T: IndexRange>(&mut self, range: T) {
        for (block, mask) in Masks::new(range, self.length) {
            // SAFETY: Masks cannot return a block index that is out of range.
            let block = unsafe { self.get_unchecked_mut(block) };
            *block &= !mask;
        }
    }

    /// Toggles (inverts) every bit in the given range.
    ///
    /// Use `..` to toggle the whole bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn toggle_range<T: IndexRange>(&mut self, range: T) {
        for (block, mask) in Masks::new(range, self.length) {
            // SAFETY: Masks cannot return a block index that is out of range.
            let block = unsafe { self.get_unchecked_mut(block) };
            *block ^= mask;
        }
    }

    /// Checks if the bitset contains every bit in the given range.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn contains_all_in_range<T: IndexRange>(&self, range: T) -> bool {
        for (block, mask) in Masks::new(range, self.length) {
            // SAFETY: Masks cannot return a block index that is out of range.
            let block = unsafe { self.get_unchecked(block) };
            if block & mask != mask {
                return false;
            }
        }
        true
    }

    /// Checks if the bitset contains at least one set bit in the given range.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn contains_any_in_range<T: IndexRange>(&self, range: T) -> bool {
        for (block, mask) in Masks::new(range, self.length) {
            // SAFETY: Masks cannot return a block index that is out of range.
            let block = unsafe { self.get_unchecked(block) };
            if block & mask != 0 {
                return true;
            }
        }
        false
    }

    /// View the bitset as a slice of `Block` blocks
    #[inline]
    pub fn as_slice(&self) -> &[Block] {
        // SAFETY: The bits from both usize and Block are required to be reinterprettable, and
        // neither have any padding or alignment issues. The slice constructed is within bounds
        // of the underlying allocation. This function is called with a read-only  borrow so
        // no other write can happen as long as the returned borrow lives.
        unsafe {
            let ptr = self.data.as_ptr().cast::<Block>();
            core::slice::from_raw_parts(ptr, self.usize_len())
        }
    }

    /// View the bitset as a mutable slice of `Block` blocks. Writing past the bitlength in the last
    /// will cause `contains` to return potentially incorrect results for bits past the bitlength.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Block] {
        // SAFETY: The bits from both usize and Block are required to be reinterprettable, and
        // neither have any padding or alignment issues. The slice constructed is within bounds
        // of the underlying allocation. This function is called with a mutable borrow so
        // no other read or write can happen as long as the returned borrow lives.
        unsafe {
            let ptr = self.data.as_ptr().cast::<Block>();
            core::slice::from_raw_parts_mut(ptr, self.usize_len())
        }
    }

    /// Iterates over all enabled bits.
    ///
    /// Iterator element is the index of the `1` bit, type `usize`.
    #[inline]
    pub fn ones(&self) -> Ones {
        match self.as_slice().split_first() {
            Some((&first_block, rem)) => {
                let (&last_block, rem) = rem.split_last().unwrap_or((&0, rem));
                Ones {
                    bitset_front: first_block,
                    bitset_back: last_block,
                    block_idx_front: 0,
                    block_idx_back: (1 + rem.len()) * BITS,
                    remaining_blocks: rem.iter(),
                }
            }
            None => Ones {
                bitset_front: 0,
                bitset_back: 0,
                block_idx_front: 0,
                block_idx_back: 0,
                remaining_blocks: [].iter(),
            },
        }
    }

    /// Iterates over all enabled bits.
    ///
    /// Iterator element is the index of the `1` bit, type `usize`.
    /// Unlike `ones`, this function consumes the `FixedBitset`.
    pub fn into_ones(self) -> IntoOnes {
        let ptr = self.data.as_ptr().cast();
        let len = self.simd_block_len() * SimdBlock::USIZE_COUNT;
        // SAFETY:
        // - ptr comes from self.data, so it is valid;
        // - self.data is valid for self.data.len() SimdBlocks,
        //   which is exactly self.data.len() * SimdBlock::USIZE_COUNT usizes;
        // - we will keep this slice around only as long as self.data is,
        //   so it won't become dangling.
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        // SAFETY: The data pointer and capacity were created from a Vec initially. The block
        // len is identical to that of the original.
        let data: Vec<SimdBlock> = unsafe {
            Vec::from_raw_parts(
                self.data.as_ptr().cast(),
                self.simd_block_len(),
                self.capacity,
            )
        };
        let mut iter = slice.iter().copied();

        core::mem::forget(self);

        IntoOnes {
            bitset_front: iter.next().unwrap_or(0),
            bitset_back: iter.next_back().unwrap_or(0),
            block_idx_front: 0,
            block_idx_back: len.saturating_sub(1) * BITS,
            remaining_blocks: iter,
            _buf: data,
        }
    }

    /// Iterates over all disabled bits.
    ///
    /// Iterator element is the index of the `0` bit, type `usize`.
    #[inline]
    pub fn zeroes(&self) -> Zeroes {
        match self.as_slice().split_first() {
            Some((&block, rem)) => Zeroes {
                bitset: !block,
                block_idx: 0,
                len: self.len(),
                remaining_blocks: rem.iter(),
            },
            None => Zeroes {
                bitset: !0,
                block_idx: 0,
                len: self.len(),
                remaining_blocks: [].iter(),
            },
        }
    }

    /// Returns a lazy iterator over the intersection of two `FixedBitSet`s
    pub fn intersection<'a>(&'a self, other: &'a FixedBitSet) -> Intersection<'a> {
        Intersection {
            iter: self.ones(),
            other,
        }
    }

    /// Returns a lazy iterator over the union of two `FixedBitSet`s.
    pub fn union<'a>(&'a self, other: &'a FixedBitSet) -> Union<'a> {
        Union {
            iter: self.ones().chain(other.difference(self)),
        }
    }

    /// Returns a lazy iterator over the difference of two `FixedBitSet`s. The difference of `a`
    /// and `b` is the elements of `a` which are not in `b`.
    pub fn difference<'a>(&'a self, other: &'a FixedBitSet) -> Difference<'a> {
        Difference {
            iter: self.ones(),
            other,
        }
    }

    /// Returns a lazy iterator over the symmetric difference of two `FixedBitSet`s.
    /// The symmetric difference of `a` and `b` is the elements of one, but not both, sets.
    pub fn symmetric_difference<'a>(&'a self, other: &'a FixedBitSet) -> SymmetricDifference<'a> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }

    /// In-place union of two `FixedBitSet`s.
    ///
    /// On calling this method, `self`'s capacity may be increased to match `other`'s.
    pub fn union_with(&mut self, other: &FixedBitSet) {
        if other.len() >= self.len() {
            self.grow(other.len());
        }
        self.as_mut_simd_slice()
            .iter_mut()
            .zip(other.as_simd_slice().iter())
            .for_each(|(x, y)| *x |= *y);
    }

    /// In-place intersection of two `FixedBitSet`s.
    ///
    /// On calling this method, `self`'s capacity will remain the same as before.
    pub fn intersect_with(&mut self, other: &FixedBitSet) {
        let me = self.as_mut_simd_slice();
        let other = other.as_simd_slice();
        me.iter_mut().zip(other.iter()).for_each(|(x, y)| {
            *x &= *y;
        });
        let mn = core::cmp::min(me.len(), other.len());
        for wd in &mut me[mn..] {
            *wd = SimdBlock::NONE;
        }
    }

    /// In-place difference of two `FixedBitSet`s.
    ///
    /// On calling this method, `self`'s capacity will remain the same as before.
    pub fn difference_with(&mut self, other: &FixedBitSet) {
        self.as_mut_simd_slice()
            .iter_mut()
            .zip(other.as_simd_slice().iter())
            .for_each(|(x, y)| {
                *x &= !*y;
            });

        // There's no need to grow self or do any other adjustments.
        //
        // * If self is longer than other, the bits at the end of self won't be affected since other
        //   has them implicitly set to 0.
        // * If other is longer than self, the bits at the end of other are irrelevant since self
        //   has them set to 0 anyway.
    }

    /// In-place symmetric difference of two `FixedBitSet`s.
    ///
    /// On calling this method, `self`'s capacity may be increased to match `other`'s.
    pub fn symmetric_difference_with(&mut self, other: &FixedBitSet) {
        if other.len() >= self.len() {
            self.grow(other.len());
        }
        self.as_mut_simd_slice()
            .iter_mut()
            .zip(other.as_simd_slice().iter())
            .for_each(|(x, y)| {
                *x ^= *y;
            });
    }

    /// Computes how many bits would be set in the union between two bitsets.
    ///
    /// This is potentially much faster than using `union(other).count()`. Unlike
    /// other methods like using [`union_with`] followed by [`count_ones`], this
    /// does not mutate in place or require separate allocations.
    #[inline]
    pub fn union_count(&self, other: &FixedBitSet) -> usize {
        let me = self.as_slice();
        let other = other.as_slice();
        let count = Self::batch_count_ones(me.iter().zip(other.iter()).map(|(x, y)| (*x | *y)));
        match other.len().cmp(&me.len()) {
            Ordering::Greater => count + Self::batch_count_ones(other[me.len()..].iter().copied()),
            Ordering::Less => count + Self::batch_count_ones(me[other.len()..].iter().copied()),
            Ordering::Equal => count,
        }
    }

    /// Computes how many bits would be set in the intersection between two bitsets.
    ///
    /// This is potentially much faster than using `intersection(other).count()`. Unlike
    /// other methods like using [`intersect_with`] followed by [`count_ones`], this
    /// does not mutate in place or require separate allocations.
    #[inline]
    pub fn intersection_count(&self, other: &FixedBitSet) -> usize {
        Self::batch_count_ones(
            self.as_slice()
                .iter()
                .zip(other.as_slice())
                .map(|(x, y)| (*x & *y)),
        )
    }

    /// Computes how many bits would be set in the difference between two bitsets.
    ///
    /// This is potentially much faster than using `difference(other).count()`. Unlike
    /// other methods like using [`difference_with`] followed by [`count_ones`], this
    /// does not mutate in place or require separate allocations.
    #[inline]
    pub fn difference_count(&self, other: &FixedBitSet) -> usize {
        Self::batch_count_ones(
            self.as_slice()
                .iter()
                .zip(other.as_slice().iter())
                .map(|(x, y)| (*x & !*y)),
        )
    }

    /// Computes how many bits would be set in the symmetric difference between two bitsets.
    ///
    /// This is potentially much faster than using `symmetric_difference(other).count()`. Unlike
    /// other methods like using [`symmetric_difference_with`] followed by [`count_ones`], this
    /// does not mutate in place or require separate allocations.
    #[inline]
    pub fn symmetric_difference_count(&self, other: &FixedBitSet) -> usize {
        let me = self.as_slice();
        let other = other.as_slice();
        let count = Self::batch_count_ones(me.iter().zip(other.iter()).map(|(x, y)| (*x ^ *y)));
        match other.len().cmp(&me.len()) {
            Ordering::Greater => count + Self::batch_count_ones(other[me.len()..].iter().copied()),
            Ordering::Less => count + Self::batch_count_ones(me[other.len()..].iter().copied()),
            Ordering::Equal => count,
        }
    }

    /// Returns `true` if `self` has no elements in common with `other`. This
    /// is equivalent to checking for an empty intersection.
    pub fn is_disjoint(&self, other: &FixedBitSet) -> bool {
        self.as_simd_slice()
            .iter()
            .zip(other.as_simd_slice())
            .all(|(x, y)| (*x & *y).is_empty())
    }

    /// Returns `true` if the set is a subset of another, i.e. `other` contains
    /// at least all the values in `self`.
    pub fn is_subset(&self, other: &FixedBitSet) -> bool {
        let me = self.as_simd_slice();
        let other = other.as_simd_slice();
        me.iter()
            .zip(other.iter())
            .all(|(x, y)| x.andnot(*y).is_empty())
            && me.iter().skip(other.len()).all(|x| x.is_empty())
    }

    /// Returns `true` if the set is a superset of another, i.e. `self` contains
    /// at least all the values in `other`.
    pub fn is_superset(&self, other: &FixedBitSet) -> bool {
        other.is_subset(self)
    }
}

impl Hash for FixedBitSet {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.length.hash(state);
        self.as_simd_slice().hash(state);
    }
}

impl PartialEq for FixedBitSet {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.as_simd_slice().eq(other.as_simd_slice())
    }
}

impl PartialOrd for FixedBitSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FixedBitSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length
            .cmp(&other.length)
            .then_with(|| self.as_simd_slice().cmp(other.as_simd_slice()))
    }
}

impl Default for FixedBitSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FixedBitSet {
    fn drop(&mut self) {
        // SAFETY: The data pointer and capacity were created from a Vec initially. The block
        // len is identical to that of the original.
        drop(unsafe {
            Vec::from_raw_parts(self.data.as_ptr(), self.simd_block_len(), self.capacity)
        });
    }
}

impl Binary for FixedBitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if f.alternate() {
            f.write_str("0b")?;
        }

        for i in 0..self.length {
            if self[i] {
                f.write_char('1')?;
            } else {
                f.write_char('0')?;
            }
        }

        Ok(())
    }
}

impl Display for FixedBitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Binary::fmt(&self, f)
    }
}

/// An iterator producing elements in the difference of two sets.
///
/// This struct is created by the [`FixedBitSet::difference`] method.
pub struct Difference<'a> {
    iter: Ones<'a>,
    other: &'a FixedBitSet,
}

impl<'a> Iterator for Difference<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref().find(|&nxt| !self.other.contains(nxt))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Difference<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .by_ref()
            .rev()
            .find(|&nxt| !self.other.contains(nxt))
    }
}

// Difference will continue to return None once it first returns None.
impl<'a> FusedIterator for Difference<'a> {}

/// An iterator producing elements in the symmetric difference of two sets.
///
/// This struct is created by the [`FixedBitSet::symmetric_difference`] method.
pub struct SymmetricDifference<'a> {
    iter: Chain<Difference<'a>, Difference<'a>>,
}

impl<'a> Iterator for SymmetricDifference<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for SymmetricDifference<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

// SymmetricDifference will continue to return None once it first returns None.
impl<'a> FusedIterator for SymmetricDifference<'a> {}

/// An iterator producing elements in the intersection of two sets.
///
/// This struct is created by the [`FixedBitSet::intersection`] method.
pub struct Intersection<'a> {
    iter: Ones<'a>,
    other: &'a FixedBitSet,
}

impl<'a> Iterator for Intersection<'a> {
    type Item = usize; // the bit position of the '1'

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref().find(|&nxt| self.other.contains(nxt))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Intersection<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .by_ref()
            .rev()
            .find(|&nxt| self.other.contains(nxt))
    }
}

// Intersection will continue to return None once it first returns None.
impl<'a> FusedIterator for Intersection<'a> {}

/// An iterator producing elements in the union of two sets.
///
/// This struct is created by the [`FixedBitSet::union`] method.
pub struct Union<'a> {
    iter: Chain<Ones<'a>, Difference<'a>>,
}

impl<'a> Iterator for Union<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Union<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

// Union will continue to return None once it first returns None.
impl<'a> FusedIterator for Union<'a> {}

struct Masks {
    first_block: usize,
    first_mask: usize,
    last_block: usize,
    last_mask: usize,
}

impl Masks {
    #[inline]
    fn new<T: IndexRange>(range: T, length: usize) -> Masks {
        let start = range.start().unwrap_or(0);
        let end = range.end().unwrap_or(length);
        assert!(
            start <= end && end <= length,
            "invalid range {}..{} for a fixedbitset of size {}",
            start,
            end,
            length
        );

        let (first_block, first_rem) = div_rem(start, BITS);
        let (last_block, last_rem) = div_rem(end, BITS);

        Masks {
            first_block,
            first_mask: usize::MAX << first_rem,
            last_block,
            last_mask: (usize::MAX >> 1) >> (BITS - last_rem - 1),
            // this is equivalent to `MAX >> (BITS - x)` with correct semantics when x == 0.
        }
    }
}

impl Iterator for Masks {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.first_block.cmp(&self.last_block) {
            Ordering::Less => {
                let res = (self.first_block, self.first_mask);
                self.first_block += 1;
                self.first_mask = !0;
                Some(res)
            }
            Ordering::Equal => {
                let mask = self.first_mask & self.last_mask;
                let res = if mask == 0 {
                    None
                } else {
                    Some((self.first_block, mask))
                };
                self.first_block += 1;
                res
            }
            Ordering::Greater => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.first_block..=self.last_block).size_hint()
    }
}

// Masks will continue to return None once it first returns None.
impl FusedIterator for Masks {}

// Masks's size_hint implementation is exact. It never returns an
// unbounded value and always returns an exact number of values.
impl ExactSizeIterator for Masks {}

/// An  iterator producing the indices of the set bit in a set.
///
/// This struct is created by the [`FixedBitSet::ones`] method.
pub struct Ones<'a> {
    bitset_front: usize,
    bitset_back: usize,
    block_idx_front: usize,
    block_idx_back: usize,
    remaining_blocks: core::slice::Iter<'a, usize>,
}

impl<'a> Ones<'a> {
    #[inline]
    pub fn last_positive_bit_and_unset(n: &mut usize) -> usize {
        // Find the last set bit using x & -x
        let last_bit = *n & n.wrapping_neg();

        // Find the position of the last set bit
        let position = last_bit.trailing_zeros();

        // Unset the last set bit
        *n &= *n - 1;

        position as usize
    }

    #[inline]
    fn first_positive_bit_and_unset(n: &mut usize) -> usize {
        /* Identify the first non zero bit */
        let bit_idx = n.leading_zeros();

        /* set that bit to zero */
        let mask = !((1_usize) << (BITS as u32 - bit_idx - 1));
        n.bitand_assign(mask);

        bit_idx as usize
    }
}

impl<'a> DoubleEndedIterator for Ones<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.bitset_back == 0 {
            match self.remaining_blocks.next_back() {
                None => {
                    if self.bitset_front != 0 {
                        self.bitset_back = 0;
                        self.block_idx_back = self.block_idx_front;
                        return Some(
                            self.block_idx_front + BITS
                                - Self::first_positive_bit_and_unset(&mut self.bitset_front)
                                - 1,
                        );
                    } else {
                        return None;
                    }
                }
                Some(next_block) => {
                    self.bitset_back = *next_block;
                    self.block_idx_back -= BITS;
                }
            };
        }

        Some(
            self.block_idx_back - Self::first_positive_bit_and_unset(&mut self.bitset_back) + BITS
                - 1,
        )
    }
}

impl<'a> Iterator for Ones<'a> {
    type Item = usize; // the bit position of the '1'

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.bitset_front == 0 {
            match self.remaining_blocks.next() {
                Some(next_block) => {
                    self.bitset_front = *next_block;
                    self.block_idx_front += BITS;
                }
                None => {
                    if self.bitset_back != 0 {
                        // not needed for iteration, but for size_hint
                        self.block_idx_front = self.block_idx_back;
                        self.bitset_front = 0;

                        return Some(
                            self.block_idx_back
                                + Self::last_positive_bit_and_unset(&mut self.bitset_back),
                        );
                    } else {
                        return None;
                    }
                }
            };
        }

        Some(self.block_idx_front + Self::last_positive_bit_and_unset(&mut self.bitset_front))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            0,
            (Some(self.block_idx_back - self.block_idx_front + 2 * BITS)),
        )
    }
}

// Ones will continue to return None once it first returns None.
impl<'a> FusedIterator for Ones<'a> {}

/// An  iterator producing the indices of the set bit in a set.
///
/// This struct is created by the [`FixedBitSet::ones`] method.
pub struct Zeroes<'a> {
    bitset: usize,
    block_idx: usize,
    len: usize,
    remaining_blocks: core::slice::Iter<'a, usize>,
}

impl<'a> Iterator for Zeroes<'a> {
    type Item = usize; // the bit position of the '1'

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.bitset == 0 {
            self.bitset = !*self.remaining_blocks.next()?;
            self.block_idx += BITS;
        }
        let t = self.bitset & (0_usize).wrapping_sub(self.bitset);
        let r = self.bitset.trailing_zeros() as usize;
        self.bitset ^= t;
        let bit = self.block_idx + r;
        // The remaining zeroes beyond the length of the bitset must be excluded.
        if bit < self.len {
            Some(bit)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len))
    }
}

// Zeroes will stop returning Some when exhausted.
impl<'a> FusedIterator for Zeroes<'a> {}

impl Clone for FixedBitSet {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_blocks_and_len(Vec::from(self.as_simd_slice()), self.length)
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        if self.length < source.length {
            // SAFETY: `fill` is uninitialized, but is immediately initialized from `source`.
            unsafe { self.grow_inner(source.length, MaybeUninit::uninit()) };
        }
        let me = self.as_mut_simd_slice_uninit();
        let them = source.as_simd_slice_uninit();
        match me.len().cmp(&them.len()) {
            Ordering::Greater => {
                let (head, tail) = me.split_at_mut(them.len());
                head.copy_from_slice(them);
                tail.fill(MaybeUninit::new(SimdBlock::NONE));
            }
            Ordering::Equal => me.copy_from_slice(them),
            // The grow_inner above ensures that self is at least as large as source.
            // so this branch is unreachable.
            Ordering::Less => {}
        }
        self.length = source.length;
    }
}

/// Return **true** if the bit is enabled in the bitset,
/// or **false** otherwise.
///
/// Note: bits outside the capacity are always disabled, and thus
/// indexing a FixedBitSet will not panic.
impl Index<usize> for FixedBitSet {
    type Output = bool;

    #[inline]
    fn index(&self, bit: usize) -> &bool {
        if self.contains(bit) {
            &true
        } else {
            &false
        }
    }
}

/// Sets the bit at index **i** to **true** for each item **i** in the input **src**.
impl Extend<usize> for FixedBitSet {
    fn extend<I: IntoIterator<Item = usize>>(&mut self, src: I) {
        let iter = src.into_iter();
        for i in iter {
            if i >= self.len() {
                self.grow(i + 1);
            }
            self.put(i);
        }
    }
}

/// Return a FixedBitSet containing bits set to **true** for every bit index in
/// the iterator, other bits are set to **false**.
impl FromIterator<usize> for FixedBitSet {
    fn from_iter<I: IntoIterator<Item = usize>>(src: I) -> Self {
        let mut fbs = FixedBitSet::with_capacity(0);
        fbs.extend(src);
        fbs
    }
}

pub struct IntoOnes {
    bitset_front: Block,
    bitset_back: Block,
    block_idx_front: usize,
    block_idx_back: usize,
    remaining_blocks: core::iter::Copied<core::slice::Iter<'static, usize>>,
    // Keep buf along so that `remaining_blocks` remains valid.
    _buf: Vec<SimdBlock>,
}

impl IntoOnes {
    #[inline]
    pub fn last_positive_bit_and_unset(n: &mut Block) -> usize {
        // Find the last set bit using x & -x
        let last_bit = *n & n.wrapping_neg();

        // Find the position of the last set bit
        let position = last_bit.trailing_zeros();

        // Unset the last set bit
        *n &= *n - 1;

        position as usize
    }

    #[inline]
    fn first_positive_bit_and_unset(n: &mut Block) -> usize {
        /* Identify the first non zero bit */
        let bit_idx = n.leading_zeros();

        /* set that bit to zero */
        let mask = !((1_usize) << (BITS as u32 - bit_idx - 1));
        n.bitand_assign(mask);

        bit_idx as usize
    }
}

impl DoubleEndedIterator for IntoOnes {
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.bitset_back == 0 {
            match self.remaining_blocks.next_back() {
                None => {
                    if self.bitset_front != 0 {
                        self.bitset_back = 0;
                        self.block_idx_back = self.block_idx_front;
                        return Some(
                            self.block_idx_front + BITS
                                - Self::first_positive_bit_and_unset(&mut self.bitset_front)
                                - 1,
                        );
                    } else {
                        return None;
                    }
                }
                Some(next_block) => {
                    self.bitset_back = next_block;
                    self.block_idx_back -= BITS;
                }
            };
        }

        Some(
            self.block_idx_back - Self::first_positive_bit_and_unset(&mut self.bitset_back) + BITS
                - 1,
        )
    }
}

impl Iterator for IntoOnes {
    type Item = usize; // the bit position of the '1'

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.bitset_front == 0 {
            match self.remaining_blocks.next() {
                Some(next_block) => {
                    self.bitset_front = next_block;
                    self.block_idx_front += BITS;
                }
                None => {
                    if self.bitset_back != 0 {
                        // not needed for iteration, but for size_hint
                        self.block_idx_front = self.block_idx_back;
                        self.bitset_front = 0;

                        return Some(
                            self.block_idx_back
                                + Self::last_positive_bit_and_unset(&mut self.bitset_back),
                        );
                    } else {
                        return None;
                    }
                }
            };
        }

        Some(self.block_idx_front + Self::last_positive_bit_and_unset(&mut self.bitset_front))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            0,
            (Some(self.block_idx_back - self.block_idx_front + 2 * BITS)),
        )
    }
}

// Ones will continue to return None once it first returns None.
impl FusedIterator for IntoOnes {}

impl<'a> BitAnd for &'a FixedBitSet {
    type Output = FixedBitSet;
    fn bitand(self, other: &FixedBitSet) -> FixedBitSet {
        let (short, long) = {
            if self.len() <= other.len() {
                (self.as_simd_slice(), other.as_simd_slice())
            } else {
                (other.as_simd_slice(), self.as_simd_slice())
            }
        };
        let mut data = Vec::from(short);
        for (data, block) in data.iter_mut().zip(long.iter()) {
            *data &= *block;
        }
        let len = core::cmp::min(self.len(), other.len());
        FixedBitSet::from_blocks_and_len(data, len)
    }
}

impl BitAndAssign for FixedBitSet {
    fn bitand_assign(&mut self, other: Self) {
        self.intersect_with(&other);
    }
}

impl BitAndAssign<&Self> for FixedBitSet {
    fn bitand_assign(&mut self, other: &Self) {
        self.intersect_with(other);
    }
}

impl<'a> BitOr for &'a FixedBitSet {
    type Output = FixedBitSet;
    fn bitor(self, other: &FixedBitSet) -> FixedBitSet {
        let (short, long) = {
            if self.len() <= other.len() {
                (self.as_simd_slice(), other.as_simd_slice())
            } else {
                (other.as_simd_slice(), self.as_simd_slice())
            }
        };
        let mut data = Vec::from(long);
        for (data, block) in data.iter_mut().zip(short.iter()) {
            *data |= *block;
        }
        let len = core::cmp::max(self.len(), other.len());
        FixedBitSet::from_blocks_and_len(data, len)
    }
}

impl BitOrAssign for FixedBitSet {
    fn bitor_assign(&mut self, other: Self) {
        self.union_with(&other);
    }
}

impl BitOrAssign<&Self> for FixedBitSet {
    fn bitor_assign(&mut self, other: &Self) {
        self.union_with(other);
    }
}

impl<'a> BitXor for &'a FixedBitSet {
    type Output = FixedBitSet;
    fn bitxor(self, other: &FixedBitSet) -> FixedBitSet {
        let (short, long) = {
            if self.len() <= other.len() {
                (self.as_simd_slice(), other.as_simd_slice())
            } else {
                (other.as_simd_slice(), self.as_simd_slice())
            }
        };
        let mut data = Vec::from(long);
        for (data, block) in data.iter_mut().zip(short.iter()) {
            *data ^= *block;
        }
        let len = core::cmp::max(self.len(), other.len());
        FixedBitSet::from_blocks_and_len(data, len)
    }
}

impl BitXorAssign for FixedBitSet {
    fn bitxor_assign(&mut self, other: Self) {
        self.symmetric_difference_with(&other);
    }
}

impl BitXorAssign<&Self> for FixedBitSet {
    fn bitxor_assign(&mut self, other: &Self) {
        self.symmetric_difference_with(other);
    }
}
