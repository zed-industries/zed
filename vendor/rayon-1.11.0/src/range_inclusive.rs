//! Parallel iterator types for [inclusive ranges],
//! the type for values created by `a..=b` expressions
//!
//! You will rarely need to interact with this module directly unless you have
//! need to name one of the iterator types.
//!
//! ```
//! use rayon::prelude::*;
//!
//! let r = (0..=100u64).into_par_iter()
//!                     .sum();
//!
//! // compare result with sequential calculation
//! assert_eq!((0..=100).sum::<u64>(), r);
//! ```
//!
//! [inclusive ranges]: std::ops::RangeInclusive

use crate::iter::plumbing::*;
use crate::iter::*;
use std::ops::RangeInclusive;

/// Parallel iterator over an inclusive range, implemented for all integer types and `char`.
///
/// **Note:** The `zip` operation requires `IndexedParallelIterator`
/// which is only implemented for `u8`, `i8`, `u16`, `i16`, and `char`.
///
/// ```
/// use rayon::prelude::*;
///
/// let p = (0..=25u16).into_par_iter()
///                   .zip(0..=25u16)
///                   .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
///                   .map(|(x, y)| x * y)
///                   .sum::<u16>();
///
/// let s = (0..=25u16).zip(0..=25u16)
///                   .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
///                   .map(|(x, y)| x * y)
///                   .sum();
///
/// assert_eq!(p, s);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<T> {
    range: RangeInclusive<T>,
}

impl<T> Iter<T>
where
    RangeInclusive<T>: Eq,
    T: Ord + Copy,
{
    /// Returns `Some((start, end))` for `start..=end`, or `None` if it is exhausted.
    ///
    /// Note that `RangeInclusive` does not specify the bounds of an exhausted iterator,
    /// so this is a way for us to figure out what we've got.  Thankfully, all of the
    /// integer types we care about can be trivially cloned.
    fn bounds(&self) -> Option<(T, T)> {
        let start = *self.range.start();
        let end = *self.range.end();
        if start <= end && self.range == (start..=end) {
            // If the range is still nonempty, this is obviously true
            // If the range is exhausted, either start > end or
            // the range does not equal start..=end.
            Some((start, end))
        } else {
            None
        }
    }
}

/// Implemented for ranges of all primitive integer types and `char`.
impl<T> IntoParallelIterator for RangeInclusive<T>
where
    Iter<T>: ParallelIterator,
{
    type Item = <Iter<T> as ParallelIterator>::Item;
    type Iter = Iter<T>;

    fn into_par_iter(self) -> Self::Iter {
        Iter { range: self }
    }
}

/// These traits help drive integer type inference. Without them, an unknown `{integer}` type only
/// has constraints on `Iter<{integer}>`, which will probably give up and use `i32`. By adding
/// these traits on the item type, the compiler can see a more direct constraint to infer like
/// `{integer}: RangeInteger`, which works better. See `test_issue_833` for an example.
///
/// They have to be `pub` since they're seen in the public `impl ParallelIterator` constraints, but
/// we put them in a private modules so they're not actually reachable in our public API.
mod private {
    use super::*;

    /// Implementation details of `ParallelIterator for Iter<Self>`
    pub trait RangeInteger: Sized + Send {
        private_decl! {}

        fn drive_unindexed<C>(iter: Iter<Self>, consumer: C) -> C::Result
        where
            C: UnindexedConsumer<Self>;

        fn opt_len(iter: &Iter<Self>) -> Option<usize>;
    }

    /// Implementation details of `IndexedParallelIterator for Iter<Self>`
    pub trait IndexedRangeInteger: RangeInteger {
        private_decl! {}

        fn drive<C>(iter: Iter<Self>, consumer: C) -> C::Result
        where
            C: Consumer<Self>;

        fn len(iter: &Iter<Self>) -> usize;

        fn with_producer<CB>(iter: Iter<Self>, callback: CB) -> CB::Output
        where
            CB: ProducerCallback<Self>;
    }
}
use private::{IndexedRangeInteger, RangeInteger};

impl<T: RangeInteger> ParallelIterator for Iter<T> {
    type Item = T;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<T>,
    {
        T::drive_unindexed(self, consumer)
    }

    #[inline]
    fn opt_len(&self) -> Option<usize> {
        T::opt_len(self)
    }
}

impl<T: IndexedRangeInteger> IndexedParallelIterator for Iter<T> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<T>,
    {
        T::drive(self, consumer)
    }

    #[inline]
    fn len(&self) -> usize {
        T::len(self)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<T>,
    {
        T::with_producer(self, callback)
    }
}

macro_rules! convert {
    ( $iter:ident . $method:ident ( $( $arg:expr ),* ) ) => {
        if let Some((start, end)) = $iter.bounds() {
            if let Some(end) = end.checked_add(1) {
                (start..end).into_par_iter().$method($( $arg ),*)
            } else {
                (start..end).into_par_iter().chain(once(end)).$method($( $arg ),*)
            }
        } else {
            empty::<Self>().$method($( $arg ),*)
        }
    };
}

macro_rules! parallel_range_impl {
    ( $t:ty ) => {
        impl RangeInteger for $t {
            private_impl! {}

            fn drive_unindexed<C>(iter: Iter<$t>, consumer: C) -> C::Result
            where
                C: UnindexedConsumer<$t>,
            {
                convert!(iter.drive_unindexed(consumer))
            }

            fn opt_len(iter: &Iter<$t>) -> Option<usize> {
                convert!(iter.opt_len())
            }
        }
    };
}

macro_rules! indexed_range_impl {
    ( $t:ty ) => {
        parallel_range_impl! { $t }

        impl IndexedRangeInteger for $t {
            private_impl! {}

            fn drive<C>(iter: Iter<$t>, consumer: C) -> C::Result
            where
                C: Consumer<$t>,
            {
                convert!(iter.drive(consumer))
            }

            fn len(iter: &Iter<$t>) -> usize {
                iter.range.len()
            }

            fn with_producer<CB>(iter: Iter<$t>, callback: CB) -> CB::Output
            where
                CB: ProducerCallback<$t>,
            {
                convert!(iter.with_producer(callback))
            }
        }
    };
}

// all RangeInclusive<T> with ExactSizeIterator
indexed_range_impl! {u8}
indexed_range_impl! {u16}
indexed_range_impl! {i8}
indexed_range_impl! {i16}

// other RangeInclusive<T> with just Iterator
parallel_range_impl! {usize}
parallel_range_impl! {isize}
parallel_range_impl! {u32}
parallel_range_impl! {i32}
parallel_range_impl! {u64}
parallel_range_impl! {i64}
parallel_range_impl! {u128}
parallel_range_impl! {i128}

// char is special
macro_rules! convert_char {
    ( $self:ident . $method:ident ( $( $arg:expr ),* ) ) => {
        if let Some((start, end)) = $self.bounds() {
            let start = start as u32;
            let end = end as u32;
            if start < 0xD800 && 0xE000 <= end {
                // chain the before and after surrogate range fragments
                (start..0xD800)
                    .into_par_iter()
                    .chain(0xE000..end + 1) // cannot use RangeInclusive, so add one to end
                    .map(|codepoint| unsafe { char::from_u32_unchecked(codepoint) })
                    .$method($( $arg ),*)
            } else {
                // no surrogate range to worry about
                (start..end + 1) // cannot use RangeInclusive, so add one to end
                    .into_par_iter()
                    .map(|codepoint| unsafe { char::from_u32_unchecked(codepoint) })
                    .$method($( $arg ),*)
            }
        } else {
            empty::<char>().$method($( $arg ),*)
        }
    };
}

impl ParallelIterator for Iter<char> {
    type Item = char;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        convert_char!(self.drive(consumer))
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

// Range<u32> is broken on 16 bit platforms, may as well benefit from it
impl IndexedParallelIterator for Iter<char> {
    // Split at the surrogate range first if we're allowed to
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        convert_char!(self.drive(consumer))
    }

    fn len(&self) -> usize {
        if let Some((start, end)) = self.bounds() {
            // Taken from <char as Step>::steps_between
            let start = start as u32;
            let end = end as u32;
            let mut count = end - start;
            if start < 0xD800 && 0xE000 <= end {
                count -= 0x800
            }
            (count + 1) as usize // add one for inclusive
        } else {
            0
        }
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        convert_char!(self.with_producer(callback))
    }
}

#[test]
#[cfg(target_pointer_width = "64")]
fn test_u32_opt_len() {
    assert_eq!(Some(101), (0..=100u32).into_par_iter().opt_len());
    assert_eq!(
        Some(u32::MAX as usize),
        (0..=u32::MAX - 1).into_par_iter().opt_len()
    );
    assert_eq!(
        Some(u32::MAX as usize + 1),
        (0..=u32::MAX).into_par_iter().opt_len()
    );
}

#[test]
fn test_u64_opt_len() {
    assert_eq!(Some(101), (0..=100u64).into_par_iter().opt_len());
    assert_eq!(
        Some(usize::MAX),
        (0..=usize::MAX as u64 - 1).into_par_iter().opt_len()
    );
    assert_eq!(None, (0..=usize::MAX as u64).into_par_iter().opt_len());
    assert_eq!(None, (0..=u64::MAX).into_par_iter().opt_len());
}

#[test]
fn test_u128_opt_len() {
    assert_eq!(Some(101), (0..=100u128).into_par_iter().opt_len());
    assert_eq!(
        Some(usize::MAX),
        (0..=usize::MAX as u128 - 1).into_par_iter().opt_len()
    );
    assert_eq!(None, (0..=usize::MAX as u128).into_par_iter().opt_len());
    assert_eq!(None, (0..=u128::MAX).into_par_iter().opt_len());
}

// `usize as i64` can overflow, so make sure to wrap it appropriately
// when using the `opt_len` "indexed" mode.
#[test]
#[cfg(target_pointer_width = "64")]
fn test_usize_i64_overflow() {
    use crate::ThreadPoolBuilder;

    let iter = (-2..=i64::MAX).into_par_iter();
    assert_eq!(iter.opt_len(), Some(i64::MAX as usize + 3));

    // always run with multiple threads to split into, or this will take forever...
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    pool.install(|| assert_eq!(iter.find_last(|_| true), Some(i64::MAX)));
}

#[test]
fn test_issue_833() {
    fn is_even(n: i64) -> bool {
        n % 2 == 0
    }

    // The integer type should be inferred from `is_even`
    let v: Vec<_> = (1..=100).into_par_iter().filter(|&x| is_even(x)).collect();
    assert!(v.into_iter().eq((2..=100).step_by(2)));

    // Try examples with indexed iterators too
    let pos = (0..=100).into_par_iter().position_any(|x| x == 50i16);
    assert_eq!(pos, Some(50usize));

    assert!((0..=100)
        .into_par_iter()
        .zip(0..=100)
        .all(|(a, b)| i16::eq(&a, &b)));
}
