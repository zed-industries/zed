//! Parallel iterator types for [ranges],
//! the type for values created by `a..b` expressions
//!
//! You will rarely need to interact with this module directly unless you have
//! need to name one of the iterator types.
//!
//! ```
//! use rayon::prelude::*;
//!
//! let r = (0..100u64).into_par_iter()
//!                    .sum();
//!
//! // compare result with sequential calculation
//! assert_eq!((0..100).sum::<u64>(), r);
//! ```
//!
//! [ranges]: std::ops::Range

use crate::iter::plumbing::*;
use crate::iter::*;
use std::ops::Range;

/// Parallel iterator over a range, implemented for all integer types and `char`.
///
/// **Note:** The `zip` operation requires `IndexedParallelIterator`
/// which is not implemented for `u64`, `i64`, `u128`, or `i128`.
///
/// ```
/// use rayon::prelude::*;
///
/// let p = (0..25usize).into_par_iter()
///                   .zip(0..25usize)
///                   .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
///                   .map(|(x, y)| x * y)
///                   .sum::<usize>();
///
/// let s = (0..25usize).zip(0..25)
///                   .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
///                   .map(|(x, y)| x * y)
///                   .sum();
///
/// assert_eq!(p, s);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<T> {
    range: Range<T>,
}

/// Implemented for ranges of all primitive integer types and `char`.
impl<T> IntoParallelIterator for Range<T>
where
    Iter<T>: ParallelIterator,
{
    type Item = <Iter<T> as ParallelIterator>::Item;
    type Iter = Iter<T>;

    fn into_par_iter(self) -> Self::Iter {
        Iter { range: self }
    }
}

struct IterProducer<T> {
    range: Range<T>,
}

impl<T> IntoIterator for IterProducer<T>
where
    Range<T>: Iterator,
{
    type Item = <Range<T> as Iterator>::Item;
    type IntoIter = Range<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.range
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

macro_rules! indexed_range_impl {
    ( $t:ty ) => {
        impl RangeInteger for $t {
            private_impl! {}

            fn drive_unindexed<C>(iter: Iter<$t>, consumer: C) -> C::Result
            where
                C: UnindexedConsumer<$t>,
            {
                bridge(iter, consumer)
            }

            fn opt_len(iter: &Iter<$t>) -> Option<usize> {
                Some(iter.range.len())
            }
        }

        impl IndexedRangeInteger for $t {
            private_impl! {}

            fn drive<C>(iter: Iter<$t>, consumer: C) -> C::Result
            where
                C: Consumer<$t>,
            {
                bridge(iter, consumer)
            }

            fn len(iter: &Iter<$t>) -> usize {
                iter.range.len()
            }

            fn with_producer<CB>(iter: Iter<$t>, callback: CB) -> CB::Output
            where
                CB: ProducerCallback<$t>,
            {
                callback.callback(IterProducer { range: iter.range })
            }
        }

        impl Producer for IterProducer<$t> {
            type Item = <Range<$t> as Iterator>::Item;
            type IntoIter = Range<$t>;
            fn into_iter(self) -> Self::IntoIter {
                self.range
            }

            fn split_at(self, index: usize) -> (Self, Self) {
                assert!(index <= self.range.len());
                // For signed $t, the length and requested index could be greater than $t::MAX, and
                // then `index as $t` could wrap to negative, so wrapping_add is necessary.
                let mid = self.range.start.wrapping_add(index as $t);
                let left = self.range.start..mid;
                let right = mid..self.range.end;
                (IterProducer { range: left }, IterProducer { range: right })
            }
        }
    };
}

trait UnindexedRangeLen<L> {
    fn unindexed_len(&self) -> L;
}

macro_rules! unindexed_range_impl {
    ( $t:ty, $len_t:ty ) => {
        impl UnindexedRangeLen<$len_t> for Range<$t> {
            fn unindexed_len(&self) -> $len_t {
                let &Range { start, end } = self;
                if end > start {
                    end.wrapping_sub(start) as $len_t
                } else {
                    0
                }
            }
        }

        impl RangeInteger for $t {
            private_impl! {}

            fn drive_unindexed<C>(iter: Iter<$t>, consumer: C) -> C::Result
            where
                C: UnindexedConsumer<$t>,
            {
                #[inline]
                fn offset(start: $t) -> impl Fn(usize) -> $t {
                    move |i| start.wrapping_add(i as $t)
                }

                if let Some(len) = iter.opt_len() {
                    // Drive this in indexed mode for better `collect`.
                    (0..len)
                        .into_par_iter()
                        .map(offset(iter.range.start))
                        .drive(consumer)
                } else {
                    bridge_unindexed(IterProducer { range: iter.range }, consumer)
                }
            }

            fn opt_len(iter: &Iter<$t>) -> Option<usize> {
                usize::try_from(iter.range.unindexed_len()).ok()
            }
        }

        impl UnindexedProducer for IterProducer<$t> {
            type Item = $t;

            fn split(mut self) -> (Self, Option<Self>) {
                let index = self.range.unindexed_len() / 2;
                if index > 0 {
                    let mid = self.range.start.wrapping_add(index as $t);
                    let right = mid..self.range.end;
                    self.range.end = mid;
                    (self, Some(IterProducer { range: right }))
                } else {
                    (self, None)
                }
            }

            fn fold_with<F>(self, folder: F) -> F
            where
                F: Folder<Self::Item>,
            {
                folder.consume_iter(self)
            }
        }
    };
}

// all Range<T> with ExactSizeIterator
indexed_range_impl! {u8}
indexed_range_impl! {u16}
indexed_range_impl! {u32}
indexed_range_impl! {usize}
indexed_range_impl! {i8}
indexed_range_impl! {i16}
indexed_range_impl! {i32}
indexed_range_impl! {isize}

// other Range<T> with just Iterator
unindexed_range_impl! {u64, u64}
unindexed_range_impl! {i64, u64}
unindexed_range_impl! {u128, u128}
unindexed_range_impl! {i128, u128}

// char is special because of the surrogate range hole
macro_rules! convert_char {
    ( $self:ident . $method:ident ( $( $arg:expr ),* ) ) => {{
        let start = $self.range.start as u32;
        let end = $self.range.end as u32;
        if start < 0xD800 && 0xE000 < end {
            // chain the before and after surrogate range fragments
            (start..0xD800)
                .into_par_iter()
                .chain(0xE000..end)
                .map(|codepoint| unsafe { char::from_u32_unchecked(codepoint) })
                .$method($( $arg ),*)
        } else {
            // no surrogate range to worry about
            (start..end)
                .into_par_iter()
                .map(|codepoint| unsafe { char::from_u32_unchecked(codepoint) })
                .$method($( $arg ),*)
        }
    }};
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

impl IndexedParallelIterator for Iter<char> {
    // Split at the surrogate range first if we're allowed to
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        convert_char!(self.drive(consumer))
    }

    fn len(&self) -> usize {
        // Taken from <char as Step>::steps_between
        let start = self.range.start as u32;
        let end = self.range.end as u32;
        if start < end {
            let mut count = end - start;
            if start < 0xD800 && 0xE000 <= end {
                count -= 0x800
            }
            count as usize
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
fn check_range_split_at_overflow() {
    // Note, this split index overflows i8!
    let producer = IterProducer { range: -100i8..100 };
    let (left, right) = producer.split_at(150);
    let r1: i32 = left.range.map(i32::from).sum();
    let r2: i32 = right.range.map(i32::from).sum();
    assert_eq!(r1 + r2, -100);
}

#[test]
fn test_i128_len_doesnt_overflow() {
    // Using parse because some versions of rust don't allow long literals
    let octillion: i128 = "1000000000000000000000000000".parse().unwrap();
    let producer = IterProducer {
        range: 0..octillion,
    };

    assert_eq!(octillion as u128, producer.range.unindexed_len());
    assert_eq!(octillion as u128, (0..octillion).unindexed_len());
    assert_eq!(
        2 * octillion as u128,
        (-octillion..octillion).unindexed_len()
    );

    assert_eq!(u128::MAX, (i128::MIN..i128::MAX).unindexed_len());
}

#[test]
fn test_u64_opt_len() {
    assert_eq!(Some(100), (0..100u64).into_par_iter().opt_len());
    assert_eq!(
        Some(usize::MAX),
        (0..usize::MAX as u64).into_par_iter().opt_len()
    );
    if (usize::MAX as u64) < u64::MAX {
        assert_eq!(
            None,
            (0..(usize::MAX as u64).wrapping_add(1))
                .into_par_iter()
                .opt_len()
        );
        assert_eq!(None, (0..u64::MAX).into_par_iter().opt_len());
    }
}

#[test]
fn test_u128_opt_len() {
    assert_eq!(Some(100), (0..100u128).into_par_iter().opt_len());
    assert_eq!(
        Some(usize::MAX),
        (0..usize::MAX as u128).into_par_iter().opt_len()
    );
    assert_eq!(None, (0..1 + usize::MAX as u128).into_par_iter().opt_len());
    assert_eq!(None, (0..u128::MAX).into_par_iter().opt_len());
}

// `usize as i64` can overflow, so make sure to wrap it appropriately
// when using the `opt_len` "indexed" mode.
#[test]
#[cfg(target_pointer_width = "64")]
fn test_usize_i64_overflow() {
    use crate::ThreadPoolBuilder;

    let iter = (-2..i64::MAX).into_par_iter();
    assert_eq!(iter.opt_len(), Some(i64::MAX as usize + 2));

    // always run with multiple threads to split into, or this will take forever...
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    pool.install(|| assert_eq!(iter.find_last(|_| true), Some(i64::MAX - 1)));
}

#[test]
fn test_issue_833() {
    fn is_even(n: i64) -> bool {
        n % 2 == 0
    }

    // The integer type should be inferred from `is_even`
    let v: Vec<_> = (1..100).into_par_iter().filter(|&x| is_even(x)).collect();
    assert!(v.into_iter().eq((2..100).step_by(2)));

    // Try examples with indexed iterators too
    let pos = (0..100).into_par_iter().position_any(|x| x == 50i16);
    assert_eq!(pos, Some(50usize));

    assert!((0..100)
        .into_par_iter()
        .zip(0..100)
        .all(|(a, b)| i16::eq(&a, &b)));
}
