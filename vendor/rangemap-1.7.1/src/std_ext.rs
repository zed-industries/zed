use core::ops::{Add, Range, RangeInclusive, Sub};

pub trait RangeExt<T> {
    fn overlaps(&self, other: &Self) -> bool;
    fn touches(&self, other: &Self) -> bool;
}

impl<T> RangeExt<T> for Range<T>
where
    T: Ord,
{
    fn overlaps(&self, other: &Self) -> bool {
        use core::cmp::{max, min};
        // Strictly less than, because ends are excluded.
        max(&self.start, &other.start) < min(&self.end, &other.end)
    }

    fn touches(&self, other: &Self) -> bool {
        use core::cmp::{max, min};
        // Less-than-or-equal-to because if one end is excluded, the other is included.
        // I.e. the two could be joined into a single range, because they're overlapping
        // or immediately adjacent.
        max(&self.start, &other.start) <= min(&self.end, &other.end)
    }
}

pub trait RangeInclusiveExt<T> {
    fn overlaps(&self, other: &Self) -> bool;
    fn touches<StepFnsT>(&self, other: &Self) -> bool
    where
        StepFnsT: StepFns<T>;
}

impl<T> RangeInclusiveExt<T> for RangeInclusive<T>
where
    T: Ord + Clone,
{
    fn overlaps(&self, other: &Self) -> bool {
        use core::cmp::{max, min};
        // Less than or equal, because ends are included.
        max(self.start(), other.start()) <= min(self.end(), other.end())
    }

    fn touches<StepFnsT>(&self, other: &Self) -> bool
    where
        StepFnsT: StepFns<T>,
    {
        use core::cmp::{max, min};

        // Touching for end-inclusive ranges is equivalent to touching of
        // slightly longer end-inclusive ranges.
        //
        // We need to do a small dance to avoid arithmetic overflow
        // at the extremes of the key space. And to do this without
        // needing to bound our key type on something like `num::Bounded`
        // (https://docs.rs/num/0.3.0/num/trait.Bounded.html),
        // we'll just extend the end of the _earlier_ range iff
        // its end is already earlier than the latter range's start.
        let max_start = max(self.start(), other.start());
        let min_range_end = min(self.end(), other.end());
        let min_range_end_extended = if min_range_end < max_start {
            StepFnsT::add_one(min_range_end)
        } else {
            min_range_end.clone()
        };
        *max_start <= min_range_end_extended
    }
}

/// Minimal version of unstable [`Step`](core::iter::Step) trait
/// from the Rust standard library.
///
/// This is needed for [`RangeInclusiveMap`](crate::RangeInclusiveMap)
/// because ranges stored as its keys interact with each other
/// when the start of one is _adjacent_ the end of another.
/// I.e. we need a concept of successor values rather than just
/// equality, and that is what `Step` will
/// eventually provide once it is stabilized.
///
/// **NOTE:** This will likely be deprecated and then eventually
/// removed once the standard library's `Step`
/// trait is stabilised, as most crates will then likely implement `Step`
/// for their types where appropriate.
///
/// See [this issue](https://github.com/rust-lang/rust/issues/42168)
/// for details about that stabilization process.
pub trait StepLite {
    /// Returns the _successor_ of `self`.
    ///
    /// If this would overflow the range of values supported by `Self`,
    /// this function is allowed to panic, wrap, or saturate.
    /// The suggested behavior is to panic when debug assertions are enabled,
    /// and to wrap or saturate otherwise.
    fn add_one(&self) -> Self;

    /// Returns the _predecessor_ of `self`.
    ///
    /// If this would overflow the range of values supported by `Self`,
    /// this function is allowed to panic, wrap, or saturate.
    /// The suggested behavior is to panic when debug assertions are enabled,
    /// and to wrap or saturate otherwise.
    fn sub_one(&self) -> Self;
}

// Implement for all common integer types.
macro_rules! impl_step_lite {
    ($($t:ty)*) => ($(
        impl StepLite for $t {
            #[inline]
            fn add_one(&self) -> Self {
                Add::add(*self, 1)
            }

            #[inline]
            fn sub_one(&self) -> Self {
                Sub::sub(*self, 1)
            }
        }
    )*)
}

impl_step_lite!(usize u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);

// TODO: When on nightly, a blanket implementation for
// all types that implement `core::iter::Step` instead
// of the auto-impl above.

/// Successor and predecessor functions defined for `T`,
/// but as free functions rather than methods on `T` itself.
///
/// This is useful as a workaround for Rust's "orphan rules",
/// which prevent you from implementing [`StepLite`](crate::StepLite) for `T` if `T`
/// is a foreign type.
///
/// **NOTE:** This will likely be deprecated and then eventually
/// removed once the standard library's [`Step`](core::iter::Step)
/// trait is stabilised, as most crates will then likely implement `Step`
/// for their types where appropriate.
///
/// See [this issue](https://github.com/rust-lang/rust/issues/42168)
/// for details about that stabilization process.
///
/// There is also a blanket implementation of `StepFns` for all
/// types implementing `StepLite`. Consumers of this crate should
/// prefer to implement `StepLite` for their own types, and only
/// fall back to `StepFns` when dealing with foreign types.
pub trait StepFns<T> {
    /// Returns the _successor_ of value `start`.
    ///
    /// If this would overflow the range of values supported by `Self`,
    /// this function is allowed to panic, wrap, or saturate.
    /// The suggested behavior is to panic when debug assertions are enabled,
    /// and to wrap or saturate otherwise.
    fn add_one(start: &T) -> T;

    /// Returns the _predecessor_ of value `start`.
    ///
    /// If this would overflow the range of values supported by `Self`,
    /// this function is allowed to panic, wrap, or saturate.
    /// The suggested behavior is to panic when debug assertions are enabled,
    /// and to wrap or saturate otherwise.
    fn sub_one(start: &T) -> T;
}

impl<T> StepFns<T> for T
where
    T: StepLite,
{
    fn add_one(start: &T) -> T {
        start.add_one()
    }

    fn sub_one(start: &T) -> T {
        start.sub_one()
    }
}
