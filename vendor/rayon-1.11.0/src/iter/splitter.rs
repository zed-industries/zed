use super::plumbing::*;
use super::*;

use std::fmt::{self, Debug};

/// The `split` function takes arbitrary data and a closure that knows how to
/// split it, and turns this into a `ParallelIterator`.
///
/// # Examples
///
/// As a simple example, Rayon can recursively split ranges of indices
///
/// ```
/// use rayon::iter;
/// use rayon::prelude::*;
/// use std::ops::Range;
///
///
/// // We define a range of indices as follows
/// type Range1D = Range<usize>;
///
/// // Splitting it in two can be done like this
/// fn split_range1(r: Range1D) -> (Range1D, Option<Range1D>) {
///     // We are mathematically unable to split the range if there is only
///     // one point inside of it, but we could stop splitting before that.
///     if r.end - r.start <= 1 { return (r, None); }
///
///     // Here, our range is considered large enough to be splittable
///     let midpoint = r.start + (r.end - r.start) / 2;
///     (r.start..midpoint, Some(midpoint..r.end))
/// }
///
/// // By using iter::split, Rayon will split the range until it has enough work
/// // to feed the CPU cores, then give us the resulting sub-ranges
/// iter::split(0..4096, split_range1).for_each(|sub_range| {
///     // As our initial range had a power-of-two size, the final sub-ranges
///     // should have power-of-two sizes too
///     assert!((sub_range.end - sub_range.start).is_power_of_two());
/// });
/// ```
///
/// This recursive splitting can be extended to two or three dimensions,
/// to reproduce a classic "block-wise" parallelization scheme of graphics and
/// numerical simulations:
///
/// ```
/// # use rayon::iter;
/// # use rayon::prelude::*;
/// # use std::ops::Range;
/// # type Range1D = Range<usize>;
/// # fn split_range1(r: Range1D) -> (Range1D, Option<Range1D>) {
/// #     if r.end - r.start <= 1 { return (r, None); }
/// #     let midpoint = r.start + (r.end - r.start) / 2;
/// #     (r.start..midpoint, Some(midpoint..r.end))
/// # }
/// #
/// // A two-dimensional range of indices can be built out of two 1D ones
/// struct Range2D {
///     // Range of horizontal indices
///     pub rx: Range1D,
///
///     // Range of vertical indices
///     pub ry: Range1D,
/// }
///
/// // We want to recursively split them by the largest dimension until we have
/// // enough sub-ranges to feed our mighty multi-core CPU. This function
/// // carries out one such split.
/// fn split_range2(r2: Range2D) -> (Range2D, Option<Range2D>) {
///     // Decide on which axis (horizontal/vertical) the range should be split
///     let width = r2.rx.end - r2.rx.start;
///     let height = r2.ry.end - r2.ry.start;
///     if width >= height {
///         // This is a wide range, split it on the horizontal axis
///         let (split_rx, ry) = (split_range1(r2.rx), r2.ry);
///         let out1 = Range2D {
///             rx: split_rx.0,
///             ry: ry.clone(),
///         };
///         let out2 = split_rx.1.map(|rx| Range2D { rx, ry });
///         (out1, out2)
///     } else {
///         // This is a tall range, split it on the vertical axis
///         let (rx, split_ry) = (r2.rx, split_range1(r2.ry));
///         let out1 = Range2D {
///             rx: rx.clone(),
///             ry: split_ry.0,
///         };
///         let out2 = split_ry.1.map(|ry| Range2D { rx, ry, });
///         (out1, out2)
///     }
/// }
///
/// // Again, rayon can handle the recursive splitting for us
/// let range = Range2D { rx: 0..800, ry: 0..600 };
/// iter::split(range, split_range2).for_each(|sub_range| {
///     // If the sub-ranges were indeed split by the largest dimension, then
///     // if no dimension was twice larger than the other initially, this
///     // property will remain true in the final sub-ranges.
///     let width = sub_range.rx.end - sub_range.rx.start;
///     let height = sub_range.ry.end - sub_range.ry.start;
///     assert!((width / 2 <= height) && (height / 2 <= width));
/// });
/// ```
///
pub fn split<D, S>(data: D, splitter: S) -> Split<D, S>
where
    D: Send,
    S: Fn(D) -> (D, Option<D>) + Sync,
{
    Split { data, splitter }
}

/// `Split` is a parallel iterator using arbitrary data and a splitting function.
/// This struct is created by the [`split()`] function.
#[derive(Clone)]
pub struct Split<D, S> {
    data: D,
    splitter: S,
}

impl<D: Debug, S> Debug for Split<D, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Split").field("data", &self.data).finish()
    }
}

impl<D, S> ParallelIterator for Split<D, S>
where
    D: Send,
    S: Fn(D) -> (D, Option<D>) + Sync + Send,
{
    type Item = D;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = SplitProducer {
            data: self.data,
            splitter: &self.splitter,
        };
        bridge_unindexed(producer, consumer)
    }
}

struct SplitProducer<'a, D, S> {
    data: D,
    splitter: &'a S,
}

impl<'a, D, S> UnindexedProducer for SplitProducer<'a, D, S>
where
    D: Send,
    S: Fn(D) -> (D, Option<D>) + Sync,
{
    type Item = D;

    fn split(mut self) -> (Self, Option<Self>) {
        let splitter = self.splitter;
        let (left, right) = splitter(self.data);
        self.data = left;
        (self, right.map(|data| SplitProducer { data, splitter }))
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        folder.consume(self.data)
    }
}
