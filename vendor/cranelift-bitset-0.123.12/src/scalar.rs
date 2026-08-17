//! Scalar bitsets.

use core::mem::size_of;
use core::ops::{Add, BitAnd, BitOr, Not, Shl, Shr, Sub};

/// A small bitset built on top of a single primitive integer type.
///
/// # Example
///
/// ```
/// use cranelift_bitset::ScalarBitSet;
///
/// // Create a new bitset backed with a `u32`.
/// let mut bitset = ScalarBitSet::<u32>::new();
///
/// // Bitsets are initially empty.
/// assert!(bitset.is_empty());
/// assert_eq!(bitset.len(), 0);
///
/// // Insert into the bitset.
/// bitset.insert(4);
/// bitset.insert(5);
/// bitset.insert(6);
///
/// // Now the bitset is not empty.
/// assert_eq!(bitset.len(), 3);
/// assert!(!bitset.is_empty());
/// assert!(bitset.contains(4));
/// assert!(bitset.contains(5));
/// assert!(bitset.contains(6));
///
/// // Remove an element from the bitset.
/// let was_present = bitset.remove(6);
/// assert!(was_present);
/// assert!(!bitset.contains(6));
/// assert_eq!(bitset.len(), 2);
///
/// // Can iterate over the elements in the set.
/// let elems: Vec<_> = bitset.iter().collect();
/// assert_eq!(elems, [4, 5]);
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "enable-serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct ScalarBitSet<T>(pub T);

impl<T> core::fmt::Debug for ScalarBitSet<T>
where
    T: ScalarBitSetStorage,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut s = f.debug_struct(core::any::type_name::<Self>());
        for i in 0..Self::capacity() {
            use alloc::string::ToString;
            s.field(&i.to_string(), &self.contains(i));
        }
        s.finish()
    }
}

impl<T> Default for ScalarBitSet<T>
where
    T: ScalarBitSetStorage,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ScalarBitSet<T>
where
    T: ScalarBitSetStorage,
{
    /// Create a new, empty bitset.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let bitset = ScalarBitSet::<u64>::new();
    ///
    /// assert!(bitset.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self(T::from(0))
    }

    /// Construct a bitset with the half-open range `[lo, hi)` inserted.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let bitset = ScalarBitSet::<u64>::from_range(3, 6);
    ///
    /// assert_eq!(bitset.len(), 3);
    ///
    /// assert!(bitset.contains(3));
    /// assert!(bitset.contains(4));
    /// assert!(bitset.contains(5));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `lo > hi` or if `hi > Self::capacity()`.
    ///
    /// ```should_panic
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// // The lower bound may not be larger than the upper bound.
    /// let bitset = ScalarBitSet::<u64>::from_range(6, 3);
    /// ```
    ///
    /// ```should_panic
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// // The bounds must fit within the backing scalar type.
    /// let bitset = ScalarBitSet::<u64>::from_range(3, 69);
    /// ```
    #[inline]
    pub fn from_range(lo: u8, hi: u8) -> Self {
        assert!(lo <= hi);
        assert!(hi <= Self::capacity());

        let one = T::from(1);

        // We can't just do (one << hi) - one here as the shift may overflow
        let hi_rng = if hi >= 1 {
            (one << (hi - 1)) + ((one << (hi - 1)) - one)
        } else {
            T::from(0)
        };

        let lo_rng = (one << lo) - one;

        Self(hi_rng - lo_rng)
    }

    /// The maximum number of bits that can be stored in this bitset.
    ///
    /// If you need more bits than this, use a
    /// [`CompoundBitSet`][crate::CompoundBitSet] instead of a `ScalarBitSet`.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// assert_eq!(ScalarBitSet::<u8>::capacity(), 8);
    /// assert_eq!(ScalarBitSet::<u64>::capacity(), 64);
    /// ```
    #[inline]
    pub fn capacity() -> u8 {
        u8::try_from(size_of::<T>()).unwrap() * 8
    }

    /// Get the number of elements in this set.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u64>::new();
    ///
    /// assert_eq!(bitset.len(), 0);
    ///
    /// bitset.insert(24);
    /// bitset.insert(13);
    /// bitset.insert(36);
    ///
    /// assert_eq!(bitset.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> u8 {
        self.0.count_ones()
    }

    /// Is this bitset empty?
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u16>::new();
    ///
    /// assert!(bitset.is_empty());
    ///
    /// bitset.insert(10);
    ///
    /// assert!(!bitset.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == T::from(0)
    }

    /// Check whether this bitset contains `i`.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u8>::new();
    ///
    /// assert!(!bitset.contains(7));
    ///
    /// bitset.insert(7);
    ///
    /// assert!(bitset.contains(7));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `i` is greater than or equal to [`Self::capacity()`][Self::capacity].
    ///
    /// ```should_panic
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u8>::new();
    ///
    /// // A `ScalarBitSet<u8>` can only hold the elements `0..=7`, so `8` is
    /// // out of bounds and will trigger a panic.
    /// bitset.contains(8);
    /// ```
    #[inline]
    pub fn contains(&self, i: u8) -> bool {
        assert!(i < Self::capacity());
        self.0 & (T::from(1) << i) != T::from(0)
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
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u8>::new();
    ///
    /// // When an element is inserted that was not already present in the set,
    /// // then `true` is returned.
    /// let is_new = bitset.insert(7);
    /// assert!(is_new);
    ///
    /// // The element is now present in the set.
    /// assert!(bitset.contains(7));
    ///
    /// // And when the element is already in the set, `false` is returned from
    /// // `insert`.
    /// let is_new = bitset.insert(7);
    /// assert!(!is_new);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `i` is greater than or equal to [`Self::capacity()`][Self::capacity].
    ///
    /// ```should_panic
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u32>::new();
    ///
    /// // A `ScalarBitSet<u32>` can only hold the elements `0..=31`, so `42` is
    /// // out of bounds and will trigger a panic.
    /// bitset.insert(42);
    /// ```
    #[inline]
    pub fn insert(&mut self, i: u8) -> bool {
        let is_new = !self.contains(i);
        self.0 = self.0 | (T::from(1) << i);
        is_new
    }

    /// Remove `i` from this bitset.
    ///
    /// Returns whether `i` was previously in this set or not.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u128>::new();
    ///
    /// // Removing an element that was not present in the set returns `false`.
    /// let was_present = bitset.remove(100);
    /// assert!(!was_present);
    ///
    /// // And when the element was in the set, `true` is returned.
    /// bitset.insert(100);
    /// let was_present = bitset.remove(100);
    /// assert!(was_present);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `i` is greater than or equal to [`Self::capacity()`][Self::capacity].
    ///
    /// ```should_panic
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u16>::new();
    ///
    /// // A `ScalarBitSet<u16>` can only hold the elements `0..=15`, so `20` is
    /// // out of bounds and will trigger a panic.
    /// bitset.remove(20);
    /// ```
    #[inline]
    pub fn remove(&mut self, i: u8) -> bool {
        let was_present = self.contains(i);
        self.0 = self.0 & !(T::from(1) << i);
        was_present
    }

    /// Remove all entries from this bitset.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u32>::new();
    ///
    /// bitset.insert(10);
    /// bitset.insert(20);
    /// bitset.insert(30);
    ///
    /// bitset.clear();
    ///
    /// assert!(bitset.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.0 = T::from(0);
    }

    /// Remove and return the smallest value in the bitset.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u64>::new();
    ///
    /// bitset.insert(0);
    /// bitset.insert(24);
    /// bitset.insert(13);
    /// bitset.insert(36);
    ///
    /// assert_eq!(bitset.pop_min(), Some(0));
    /// assert_eq!(bitset.pop_min(), Some(13));
    /// assert_eq!(bitset.pop_min(), Some(24));
    /// assert_eq!(bitset.pop_min(), Some(36));
    /// assert_eq!(bitset.pop_min(), None);
    /// ```
    #[inline]
    pub fn pop_min(&mut self) -> Option<u8> {
        let min = self.min()?;
        self.remove(min);
        Some(min)
    }

    /// Remove and return the largest value in the bitset.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u64>::new();
    ///
    /// bitset.insert(0);
    /// bitset.insert(24);
    /// bitset.insert(13);
    /// bitset.insert(36);
    ///
    /// assert_eq!(bitset.pop_max(), Some(36));
    /// assert_eq!(bitset.pop_max(), Some(24));
    /// assert_eq!(bitset.pop_max(), Some(13));
    /// assert_eq!(bitset.pop_max(), Some(0));
    /// assert_eq!(bitset.pop_max(), None);
    /// ```
    #[inline]
    pub fn pop_max(&mut self) -> Option<u8> {
        let max = self.max()?;
        self.remove(max);
        Some(max)
    }

    /// Return the smallest number contained in this bitset or `None` if this
    /// bitset is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u64>::new();
    ///
    /// // When the bitset is empty, `min` returns `None`.
    /// assert_eq!(bitset.min(), None);
    ///
    /// bitset.insert(28);
    /// bitset.insert(1);
    /// bitset.insert(63);
    ///
    /// // When the bitset is not empty, it returns the smallest element.
    /// assert_eq!(bitset.min(), Some(1));
    /// ```
    #[inline]
    pub fn min(&self) -> Option<u8> {
        if self.0 == T::from(0) {
            None
        } else {
            Some(self.0.trailing_zeros())
        }
    }

    /// Return the largest number contained in the bitset or None if this bitset
    /// is empty
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u64>::new();
    ///
    /// // When the bitset is empty, `max` returns `None`.
    /// assert_eq!(bitset.max(), None);
    ///
    /// bitset.insert(0);
    /// bitset.insert(36);
    /// bitset.insert(49);
    ///
    /// // When the bitset is not empty, it returns the smallest element.
    /// assert_eq!(bitset.max(), Some(49));
    /// ```
    #[inline]
    pub fn max(&self) -> Option<u8> {
        if self.0 == T::from(0) {
            None
        } else {
            let leading_zeroes = self.0.leading_zeros();
            Some(Self::capacity() - leading_zeroes - 1)
        }
    }

    /// Iterate over the items in this set.
    ///
    /// Items are always yielded in sorted order.
    ///
    /// # Example
    ///
    /// ```
    /// use cranelift_bitset::ScalarBitSet;
    ///
    /// let mut bitset = ScalarBitSet::<u64>::new();
    ///
    /// bitset.insert(19);
    /// bitset.insert(3);
    /// bitset.insert(63);
    /// bitset.insert(0);
    ///
    /// assert_eq!(
    ///     bitset.iter().collect::<Vec<_>>(),
    ///     [0, 3, 19, 63],
    /// );
    /// ```
    #[inline]
    pub fn iter(self) -> Iter<T> {
        Iter { bitset: self }
    }
}

impl<T> IntoIterator for ScalarBitSet<T>
where
    T: ScalarBitSetStorage,
{
    type Item = u8;

    type IntoIter = Iter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a ScalarBitSet<T>
where
    T: ScalarBitSetStorage,
{
    type Item = u8;

    type IntoIter = Iter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: ScalarBitSetStorage> From<T> for ScalarBitSet<T> {
    fn from(bits: T) -> Self {
        Self(bits)
    }
}

/// A trait implemented by all integers that can be used as the backing storage
/// for a [`ScalarBitSet`].
///
/// You shouldn't have to implement this yourself, it is already implemented for
/// `u{8,16,32,64,128}` and if you need more bits than that, then use
/// [`CompoundBitSet`][crate::CompoundBitSet] instead.
pub trait ScalarBitSetStorage:
    Default
    + From<u8>
    + Shl<u8, Output = Self>
    + Shr<u8, Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Not<Output = Self>
    + Sub<Output = Self>
    + Add<Output = Self>
    + PartialEq
    + Copy
{
    /// Count the number of leading zeros.
    fn leading_zeros(self) -> u8;

    /// Count the number of trailing zeros.
    fn trailing_zeros(self) -> u8;

    /// Count the number of bits set in this integer.
    fn count_ones(self) -> u8;
}

macro_rules! impl_storage {
    ( $int:ty ) => {
        impl ScalarBitSetStorage for $int {
            #[inline]
            fn leading_zeros(self) -> u8 {
                u8::try_from(self.leading_zeros()).unwrap()
            }

            #[inline]
            fn trailing_zeros(self) -> u8 {
                u8::try_from(self.trailing_zeros()).unwrap()
            }

            #[inline]
            fn count_ones(self) -> u8 {
                u8::try_from(self.count_ones()).unwrap()
            }
        }
    };
}

impl_storage!(u8);
impl_storage!(u16);
impl_storage!(u32);
impl_storage!(u64);
impl_storage!(u128);
impl_storage!(usize);

/// An iterator over the elements in a [`ScalarBitSet`].
pub struct Iter<T> {
    bitset: ScalarBitSet<T>,
}

impl<T> Iterator for Iter<T>
where
    T: ScalarBitSetStorage,
{
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        self.bitset.pop_min()
    }
}

impl<T> DoubleEndedIterator for Iter<T>
where
    T: ScalarBitSetStorage,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.bitset.pop_max()
    }
}

impl<T> ExactSizeIterator for Iter<T>
where
    T: ScalarBitSetStorage,
{
    #[inline]
    fn len(&self) -> usize {
        usize::from(self.bitset.len())
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T> arbitrary::Arbitrary<'a> for ScalarBitSet<T>
where
    T: ScalarBitSetStorage + arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        T::arbitrary(u).map(Self)
    }
}
