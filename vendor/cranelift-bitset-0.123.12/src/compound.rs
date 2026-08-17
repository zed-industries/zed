//! Compound bit sets.

use crate::scalar::{self, ScalarBitSet, ScalarBitSetStorage};
use alloc::boxed::Box;
use core::{cmp, iter, mem};

/// A large bit set backed by dynamically-sized storage.
///
/// # Example
///
/// ```
/// use cranelift_bitset::CompoundBitSet;
///
/// // Create a new bitset.
/// let mut bitset = CompoundBitSet::new();
///
/// // Bitsets are initially empty.
/// assert!(bitset.is_empty());
/// assert_eq!(bitset.len(), 0);
///
/// // Insert into the bitset.
/// bitset.insert(444);
/// bitset.insert(555);
/// bitset.insert(666);
///
/// // Now the bitset is not empty.
/// assert_eq!(bitset.len(), 3);
/// assert!(!bitset.is_empty());
/// assert!(bitset.contains(444));
/// assert!(bitset.contains(555));
/// assert!(bitset.contains(666));
///
/// // Remove an element from the bitset.
/// let was_present = bitset.remove(666);
/// assert!(was_present);
/// assert!(!bitset.contains(666));
/// assert_eq!(bitset.len(), 2);
///
/// // Can iterate over the elements in the set.
/// let elems: Vec<_> = bitset.iter().collect();
/// assert_eq!(elems, [444, 555]);
/// ```
#[derive(Clone, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct CompoundBitSet<T = usize> {
    elems: Box<[ScalarBitSet<T>]>,
    max: Option<u32>,
}

impl core::fmt::Debug for CompoundBitSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CompoundBitSet ")?;
        f.debug_set().entries(self.iter()).finish()
    }
}

impl CompoundBitSet {
    /// Construct a new, empty bit set.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let bitset = CompoundBitSet::new();
    ///
    /// assert!(bitset.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        CompoundBitSet::default()
    }
}

impl<T: ScalarBitSetStorage> CompoundBitSet<T> {
    const BITS_PER_SCALAR: usize = mem::size_of::<T>() * 8;

    /// Construct a new, empty bit set with space reserved to store any element
    /// `x` such that `x < capacity`.
    ///
    /// The actual capacity reserved may be greater than that requested.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let bitset = CompoundBitSet::<u32>::with_capacity(4096);
    ///
    /// assert!(bitset.is_empty());
    /// assert!(bitset.capacity() >= 4096);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut bitset = Self::default();
        bitset.ensure_capacity(capacity);
        bitset
    }

    /// Get the number of elements in this bitset.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// assert_eq!(bitset.len(), 0);
    ///
    /// bitset.insert(24);
    /// bitset.insert(130);
    /// bitset.insert(3600);
    ///
    /// assert_eq!(bitset.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.elems.iter().map(|sub| usize::from(sub.len())).sum()
    }

    /// Get `n + 1` where `n` is the largest value that can be stored inside
    /// this set without growing the backing storage.
    ///
    /// That is, this set can store any value `x` such that `x <
    /// bitset.capacity()` without growing the backing storage.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// // New bitsets have zero capacity -- they allocate lazily.
    /// assert_eq!(bitset.capacity(), 0);
    ///
    /// // Insert into the bitset, growing its capacity.
    /// bitset.insert(999);
    ///
    /// // The bitset must now have capacity for at least `999` elements,
    /// // perhaps more.
    /// assert!(bitset.capacity() >= 999);
    ///```
    pub fn capacity(&self) -> usize {
        self.elems.len() * Self::BITS_PER_SCALAR
    }

    /// Is this bitset empty?
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// assert!(bitset.is_empty());
    ///
    /// bitset.insert(1234);
    ///
    /// assert!(!bitset.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert an element `i` into the `word` that can be used to index into
    /// `self.elems` and the `bit` that can be tested in the
    /// `ScalarBitSet<usize>` at `self.elems[word]`.
    #[inline]
    fn word_and_bit(i: usize) -> (usize, u8) {
        let word = i / Self::BITS_PER_SCALAR;
        let bit = i % Self::BITS_PER_SCALAR;
        let bit = u8::try_from(bit).unwrap();
        (word, bit)
    }

    /// The opposite of `word_and_bit`: convert the pair of an index into
    /// `self.elems` and associated bit index into a set element.
    #[inline]
    fn elem(word: usize, bit: u8) -> usize {
        let bit = usize::from(bit);
        debug_assert!(bit < Self::BITS_PER_SCALAR);
        word * Self::BITS_PER_SCALAR + bit
    }

    /// Is `i` contained in this bitset?
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// assert!(!bitset.contains(666));
    ///
    /// bitset.insert(666);
    ///
    /// assert!(bitset.contains(666));
    /// ```
    #[inline]
    pub fn contains(&self, i: usize) -> bool {
        let (word, bit) = Self::word_and_bit(i);
        if word < self.elems.len() {
            self.elems[word].contains(bit)
        } else {
            false
        }
    }

    /// Ensure there is space in this bitset for the values `0..n`, growing the
    /// backing storage if necessary.
    ///
    /// After calling `bitset.ensure_capacity(n)`, inserting any element `i`
    /// where `i < n` is guaranteed to succeed without growing the bitset's
    /// backing storage.
    ///
    /// # Example
    ///
    /// ```
    /// # use cranelift_bitset::CompoundBitSet;
    /// # let mut bitset = CompoundBitSet::new();
    /// // We are going to do a series of inserts where `1000` will be the
    /// // maximum value inserted. Make sure that our bitset has capacity for
    /// // these elements once up front, to avoid growing the backing storage
    /// // multiple times incrementally.
    /// bitset.ensure_capacity(1001);
    ///
    /// for i in 0..=1000 {
    ///     if i % 2 == 0 {
    ///         // Inserting this value should not require growing the backing
    ///         // storage.
    ///         assert!(bitset.capacity() > i);
    ///         bitset.insert(i);
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn ensure_capacity(&mut self, n: usize) {
        // Subtract one from the capacity to get the maximum bit that we might
        // set. If `n` is 0 then nothing need be done as no capacity needs to be
        // allocated.
        let (word, _bit) = Self::word_and_bit(match n.checked_sub(1) {
            None => return,
            Some(n) => n,
        });
        if word >= self.elems.len() {
            assert!(word < usize::try_from(isize::MAX).unwrap());

            let delta = word - self.elems.len();
            let to_grow = delta + 1;

            // Amortize the cost of growing.
            let to_grow = cmp::max(to_grow, self.elems.len() * 2);
            // Don't make ridiculously small allocations.
            let to_grow = cmp::max(to_grow, 4);

            let new_elems = self
                .elems
                .iter()
                .copied()
                .chain(iter::repeat(ScalarBitSet::new()).take(to_grow))
                .collect::<Box<[_]>>();
            self.elems = new_elems;
        }
    }

    /// Insert `i` into this bitset.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// * If the set did not previously contain `i` then `true` is returned.
    ///
    /// * If the set already contained `i` then `false` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// // When an element is inserted that was not already present in the set,
    /// // then `true` is returned.
    /// let is_new = bitset.insert(1234);
    /// assert!(is_new);
    ///
    /// // The element is now present in the set.
    /// assert!(bitset.contains(1234));
    ///
    /// // And when the element is already in the set, `false` is returned from
    /// // `insert`.
    /// let is_new = bitset.insert(1234);
    /// assert!(!is_new);
    /// ```
    #[inline]
    pub fn insert(&mut self, i: usize) -> bool {
        self.ensure_capacity(i + 1);

        let (word, bit) = Self::word_and_bit(i);
        let is_new = self.elems[word].insert(bit);

        let i = u32::try_from(i).unwrap();
        self.max = self.max.map(|max| cmp::max(max, i)).or(Some(i));

        is_new
    }

    /// Remove `i` from this bitset.
    ///
    /// Returns whether `i` was previously in this set or not.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// // Removing an element that was not present in the set returns `false`.
    /// let was_present = bitset.remove(456);
    /// assert!(!was_present);
    ///
    /// // And when the element was in the set, `true` is returned.
    /// bitset.insert(456);
    /// let was_present = bitset.remove(456);
    /// assert!(was_present);
    /// ```
    #[inline]
    pub fn remove(&mut self, i: usize) -> bool {
        let (word, bit) = Self::word_and_bit(i);
        if word < self.elems.len() {
            let sub = &mut self.elems[word];
            let was_present = sub.remove(bit);
            if was_present && self.max() == Some(i) {
                self.update_max(word);
            }
            was_present
        } else {
            false
        }
    }

    /// Update the `self.max` field, based on the old word index of `self.max`.
    fn update_max(&mut self, word_of_old_max: usize) {
        self.max = self.elems[0..word_of_old_max + 1]
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(word, sub)| {
                let bit = sub.max()?;
                Some(u32::try_from(Self::elem(word, bit)).unwrap())
            })
            .next();
    }

    /// Get the largest value in this set, or `None` if this set is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// // Returns `None` if the bitset is empty.
    /// assert!(bitset.max().is_none());
    ///
    /// bitset.insert(123);
    /// bitset.insert(987);
    /// bitset.insert(999);
    ///
    /// // Otherwise, it returns the largest value in the set.
    /// assert_eq!(bitset.max(), Some(999));
    /// ```
    #[inline]
    pub fn max(&self) -> Option<usize> {
        self.max.map(|m| usize::try_from(m).unwrap())
    }

    /// Removes and returns the largest value in this set.
    ///
    /// Returns `None` if this set is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// bitset.insert(111);
    /// bitset.insert(222);
    /// bitset.insert(333);
    /// bitset.insert(444);
    /// bitset.insert(555);
    ///
    /// assert_eq!(bitset.pop(), Some(555));
    /// assert_eq!(bitset.pop(), Some(444));
    /// assert_eq!(bitset.pop(), Some(333));
    /// assert_eq!(bitset.pop(), Some(222));
    /// assert_eq!(bitset.pop(), Some(111));
    /// assert_eq!(bitset.pop(), None);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<usize> {
        let max = self.max()?;
        self.remove(max);
        Some(max)
    }

    /// Remove all elements from this bitset.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// bitset.insert(100);
    /// bitset.insert(200);
    /// bitset.insert(300);
    ///
    /// bitset.clear();
    ///
    /// assert!(bitset.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        let max = match self.max() {
            Some(max) => max,
            None => return,
        };
        let (word, _bit) = Self::word_and_bit(max);
        debug_assert!(self.elems[word + 1..].iter().all(|sub| sub.is_empty()));
        for sub in &mut self.elems[..=word] {
            *sub = ScalarBitSet::new();
        }
        self.max = None;
    }

    /// Iterate over the elements in this bitset.
    ///
    /// The elements are always yielded in sorted order.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::CompoundBitSet;
    ///
    /// let mut bitset = CompoundBitSet::new();
    ///
    /// bitset.insert(0);
    /// bitset.insert(4096);
    /// bitset.insert(123);
    /// bitset.insert(456);
    /// bitset.insert(789);
    ///
    /// assert_eq!(
    ///     bitset.iter().collect::<Vec<_>>(),
    ///     [0, 123, 456, 789, 4096],
    /// );
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            bitset: self,
            word: 0,
            sub: None,
        }
    }

    /// Returns an iterator over the words of this bit-set or the in-memory
    /// representation of the bit set.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::{CompoundBitSet, ScalarBitSet};
    ///
    /// let mut bitset = CompoundBitSet::<u32>::default();
    ///
    /// assert_eq!(
    ///     bitset.iter_scalars().collect::<Vec<_>>(),
    ///     [],
    /// );
    ///
    /// bitset.insert(0);
    ///
    /// assert_eq!(
    ///     bitset.iter_scalars().collect::<Vec<_>>(),
    ///     [ScalarBitSet(0x1)],
    /// );
    ///
    /// bitset.insert(1);
    ///
    /// assert_eq!(
    ///     bitset.iter_scalars().collect::<Vec<_>>(),
    ///     [ScalarBitSet(0x3)],
    /// );
    ///
    /// bitset.insert(32);
    ///
    /// assert_eq!(
    ///     bitset.iter_scalars().collect::<Vec<_>>(),
    ///     [ScalarBitSet(0x3), ScalarBitSet(0x1)],
    /// );
    /// ```
    pub fn iter_scalars(&self) -> impl Iterator<Item = ScalarBitSet<T>> + '_ {
        let nwords = match self.max {
            Some(n) => 1 + (n as usize / Self::BITS_PER_SCALAR),
            None => 0,
        };
        self.elems.iter().copied().take(nwords)
    }
}

impl<'a, T: ScalarBitSetStorage> IntoIterator for &'a CompoundBitSet<T> {
    type Item = usize;

    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the elements in a [`CompoundBitSet`].
pub struct Iter<'a, T = usize> {
    bitset: &'a CompoundBitSet<T>,
    word: usize,
    sub: Option<scalar::Iter<T>>,
}

impl<T: ScalarBitSetStorage> Iterator for Iter<'_, T> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        loop {
            if let Some(sub) = &mut self.sub {
                if let Some(bit) = sub.next() {
                    return Some(CompoundBitSet::<T>::elem(self.word, bit));
                } else {
                    self.word += 1;
                }
            }

            self.sub = Some(self.bitset.elems.get(self.word)?.iter());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_capacity_no_allocs() {
        let set = CompoundBitSet::<u32>::with_capacity(0);
        assert_eq!(set.capacity(), 0);
        let set = CompoundBitSet::new();
        assert_eq!(set.capacity(), 0);
    }
}
