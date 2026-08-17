use std::cmp::Ordering;
use std::iter::Fuse;
use std::fmt;

use either::Either;

use super::adaptors::{PutBack, put_back};
use crate::either_or_both::EitherOrBoth;
use crate::size_hint::{self, SizeHint};
#[cfg(doc)]
use crate::Itertools;

/// Return an iterator adaptor that merge-joins items from the two base iterators in ascending order.
///
/// [`IntoIterator`] enabled version of [`Itertools::merge_join_by`].
pub fn merge_join_by<I, J, F, T>(left: I, right: J, cmp_fn: F)
    -> MergeJoinBy<I::IntoIter, J::IntoIter, F>
    where I: IntoIterator,
          J: IntoIterator,
          F: FnMut(&I::Item, &J::Item) -> T,
          T: OrderingOrBool<I::Item, J::Item>,
{
    MergeJoinBy {
        left: put_back(left.into_iter().fuse()),
        right: put_back(right.into_iter().fuse()),
        cmp_fn,
    }
}

/// An iterator adaptor that merge-joins items from the two base iterators in ascending order.
///
/// See [`.merge_join_by()`](crate::Itertools::merge_join_by) for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct MergeJoinBy<I: Iterator, J: Iterator, F> {
    left: PutBack<Fuse<I>>,
    right: PutBack<Fuse<J>>,
    cmp_fn: F,
}

pub trait OrderingOrBool<L, R> {
    type MergeResult;
    fn left(left: L) -> Self::MergeResult;
    fn right(right: R) -> Self::MergeResult;
    // "merge" never returns (Some(...), Some(...), ...) so Option<Either<I::Item, J::Item>>
    // is appealing but it is always followed by two put_backs, so we think the compiler is
    // smart enough to optimize it. Or we could move put_backs into "merge".
    fn merge(self, left: L, right: R) -> (Option<L>, Option<R>, Self::MergeResult);
    fn size_hint(left: SizeHint, right: SizeHint) -> SizeHint;
}

impl<L, R> OrderingOrBool<L, R> for Ordering {
    type MergeResult = EitherOrBoth<L, R>;
    fn left(left: L) -> Self::MergeResult {
        EitherOrBoth::Left(left)
    }
    fn right(right: R) -> Self::MergeResult {
        EitherOrBoth::Right(right)
    }
    fn merge(self, left: L, right: R) -> (Option<L>, Option<R>, Self::MergeResult) {
        match self {
            Ordering::Equal => (None, None, EitherOrBoth::Both(left, right)),
            Ordering::Less => (None, Some(right), EitherOrBoth::Left(left)),
            Ordering::Greater => (Some(left), None, EitherOrBoth::Right(right)),
        }
    }
    fn size_hint(left: SizeHint, right: SizeHint) -> SizeHint {
        let (a_lower, a_upper) = left;
        let (b_lower, b_upper) = right;
        let lower = ::std::cmp::max(a_lower, b_lower);
        let upper = match (a_upper, b_upper) {
            (Some(x), Some(y)) => x.checked_add(y),
            _ => None,
        };
        (lower, upper)
    }
}

impl<L, R> OrderingOrBool<L, R> for bool {
    type MergeResult = Either<L, R>;
    fn left(left: L) -> Self::MergeResult {
        Either::Left(left)
    }
    fn right(right: R) -> Self::MergeResult {
        Either::Right(right)
    }
    fn merge(self, left: L, right: R) -> (Option<L>, Option<R>, Self::MergeResult) {
        if self {
            (None, Some(right), Either::Left(left))
        } else {
            (Some(left), None, Either::Right(right))
        }
    }
    fn size_hint(left: SizeHint, right: SizeHint) -> SizeHint {
        // Not ExactSizeIterator because size may be larger than usize
        size_hint::add(left, right)
    }
}

impl<I, J, F> Clone for MergeJoinBy<I, J, F>
    where I: Iterator,
          J: Iterator,
          PutBack<Fuse<I>>: Clone,
          PutBack<Fuse<J>>: Clone,
          F: Clone,
{
    clone_fields!(left, right, cmp_fn);
}

impl<I, J, F> fmt::Debug for MergeJoinBy<I, J, F>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
          J: Iterator + fmt::Debug,
          J::Item: fmt::Debug,
{
    debug_fmt_fields!(MergeJoinBy, left, right);
}

impl<I, J, F, T> Iterator for MergeJoinBy<I, J, F>
    where I: Iterator,
          J: Iterator,
          F: FnMut(&I::Item, &J::Item) -> T,
          T: OrderingOrBool<I::Item, J::Item>,
{
    type Item = T::MergeResult;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.left.next(), self.right.next()) {
            (None, None) => None,
            (Some(left), None) => Some(T::left(left)),
            (None, Some(right)) => Some(T::right(right)),
            (Some(left), Some(right)) => {
                let (left, right, next) = (self.cmp_fn)(&left, &right).merge(left, right);
                if let Some(left) = left {
                    self.left.put_back(left);
                }
                if let Some(right) = right {
                    self.right.put_back(right);
                }
                Some(next)
            }
        }
    }

    fn size_hint(&self) -> SizeHint {
        T::size_hint(self.left.size_hint(), self.right.size_hint())
    }

    fn count(mut self) -> usize {
        let mut count = 0;
        loop {
            match (self.left.next(), self.right.next()) {
                (None, None) => break count,
                (Some(_left), None) => break count + 1 + self.left.into_parts().1.count(),
                (None, Some(_right)) => break count + 1 + self.right.into_parts().1.count(),
                (Some(left), Some(right)) => {
                    count += 1;
                    let (left, right, _) = (self.cmp_fn)(&left, &right).merge(left, right);
                    if let Some(left) = left {
                        self.left.put_back(left);
                    }
                    if let Some(right) = right {
                        self.right.put_back(right);
                    }
                }
            }
        }
    }

    fn last(mut self) -> Option<Self::Item> {
        let mut previous_element = None;
        loop {
            match (self.left.next(), self.right.next()) {
                (None, None) => break previous_element,
                (Some(left), None) => {
                    break Some(T::left(
                        self.left.into_parts().1.last().unwrap_or(left),
                    ))
                }
                (None, Some(right)) => {
                    break Some(T::right(
                        self.right.into_parts().1.last().unwrap_or(right),
                    ))
                }
                (Some(left), Some(right)) => {
                    let (left, right, elem) = (self.cmp_fn)(&left, &right).merge(left, right);
                    if let Some(left) = left {
                        self.left.put_back(left);
                    }
                    if let Some(right) = right {
                        self.right.put_back(right);
                    }
                    previous_element = Some(elem);
                }
            }
        }
    }

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        loop {
            if n == 0 {
                break self.next();
            }
            n -= 1;
            match (self.left.next(), self.right.next()) {
                (None, None) => break None,
                (Some(_left), None) => break self.left.nth(n).map(T::left),
                (None, Some(_right)) => break self.right.nth(n).map(T::right),
                (Some(left), Some(right)) => {
                    let (left, right, _) = (self.cmp_fn)(&left, &right).merge(left, right);
                    if let Some(left) = left {
                        self.left.put_back(left);
                    }
                    if let Some(right) = right {
                        self.right.put_back(right);
                    }
                }
            }
        }
    }
}
