use core::cmp::Ordering;
use core::iter::{FusedIterator, Peekable};
use core::ops::{Range, RangeInclusive};

/// Trait to determine the ordering of the start and end of a range.
trait RangeOrder {
    /// Ordering of start value
    fn order_start(&self, other: &Self) -> Ordering;

    /// Ordering of end value
    fn order_end(&self, other: &Self) -> Ordering;
}

impl<T: Ord> RangeOrder for Range<T> {
    fn order_start(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }

    fn order_end(&self, other: &Self) -> Ordering {
        self.end.cmp(&other.end)
    }
}

impl<T: Ord> RangeOrder for RangeInclusive<T> {
    fn order_start(&self, other: &Self) -> Ordering {
        self.start().cmp(other.start())
    }

    fn order_end(&self, other: &Self) -> Ordering {
        self.end().cmp(other.end())
    }
}

/// Range which can be merged with a next range if they overlap.
trait RangeMerge: Sized {
    /// Merges this range and the next range, if they overlap.
    fn merge(&mut self, next: &Self) -> bool;
}

impl<T: Ord + Clone> RangeMerge for Range<T> {
    fn merge(&mut self, other: &Self) -> bool {
        if !self.contains(&other.start) {
            return false;
        }

        if other.end > self.end {
            self.end = other.end.clone();
        }

        true
    }
}

impl<T: Ord + Clone> RangeMerge for RangeInclusive<T> {
    fn merge(&mut self, other: &Self) -> bool {
        if !self.contains(other.start()) {
            return false;
        }

        if other.end() > self.end() {
            *self = RangeInclusive::new(self.start().clone(), other.end().clone());
        }

        true
    }
}

/// Range which can be merged with a next range if they overlap.
trait RangeIntersect: Sized {
    /// Attempt to merge the next range into the current range, if they overlap.
    fn intersect(&self, next: &Self) -> Option<Self>;
}

impl<T: Ord + Clone> RangeIntersect for Range<T> {
    fn intersect(&self, other: &Self) -> Option<Self> {
        let start = (&self.start).max(&other.start);
        let end = (&self.end).min(&other.end);

        if start >= end {
            return None;
        }

        Some(start.clone()..end.clone())
    }
}

impl<T: Ord + Clone> RangeIntersect for RangeInclusive<T> {
    fn intersect(&self, other: &Self) -> Option<Self> {
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());

        if start > end {
            return None;
        }

        Some(start.clone()..=end.clone())
    }
}

#[test]
fn test_intersect() {
    assert_eq!((0..5).intersect(&(0..3)), Some(0..3));
    assert_eq!((0..3).intersect(&(0..5)), Some(0..3));
    assert_eq!((0..3).intersect(&(3..3)), None);
}

/// Iterator that produces the union of two iterators of sorted ranges.
pub struct Union<'a, T, L, R = L>
where
    T: 'a,
    L: Iterator<Item = &'a T>,
    R: Iterator<Item = &'a T>,
{
    left: Peekable<L>,
    right: Peekable<R>,
}

impl<'a, T, L, R> Union<'a, T, L, R>
where
    T: 'a,
    L: Iterator<Item = &'a T>,
    R: Iterator<Item = &'a T>,
{
    /// Create new Union iterator.
    ///
    /// Requires that the two iterators produce sorted ranges.
    pub fn new(left: L, right: R) -> Self {
        Self {
            left: left.peekable(),
            right: right.peekable(),
        }
    }
}

impl<'a, R, I> Iterator for Union<'a, R, I>
where
    R: RangeOrder + RangeMerge + Clone,
    I: Iterator<Item = &'a R>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        // get start range
        let mut range = match (self.left.peek(), self.right.peek()) {
            // if there is only one possible range, pick that
            (Some(_), None) => self.left.next().unwrap(),
            (None, Some(_)) => self.right.next().unwrap(),
            // when there are two ranges, pick the one with the earlier start
            (Some(left), Some(right)) => {
                if left.order_start(right).is_lt() {
                    self.left.next().unwrap()
                } else {
                    self.right.next().unwrap()
                }
            }
            // otherwise we are done
            (None, None) => return None,
        }
        .clone();

        // peek into next value of iterator and merge if it is contiguous
        let mut join = |iter: &mut Peekable<I>| {
            if let Some(next) = iter.peek() {
                if range.merge(next) {
                    iter.next().unwrap();
                    return true;
                }
            }
            false
        };

        // keep merging ranges as long as we can
        loop {
            if !(join(&mut self.left) || join(&mut self.right)) {
                break;
            }
        }

        Some(range)
    }
}

impl<'a, R, I> FusedIterator for Union<'a, R, I>
where
    R: RangeOrder + RangeMerge + Clone,
    I: Iterator<Item = &'a R>,
{
}

/// Iterator that produces the union of two iterators of sorted ranges.
pub struct Intersection<'a, T, L, R = L>
where
    T: 'a,
    L: Iterator<Item = &'a T>,
    R: Iterator<Item = &'a T>,
{
    left: Peekable<L>,
    right: Peekable<R>,
}

impl<'a, T, L, R> Intersection<'a, T, L, R>
where
    T: 'a,
    L: Iterator<Item = &'a T>,
    R: Iterator<Item = &'a T>,
{
    /// Create new Intersection iterator.
    ///
    /// Requires that the two iterators produce sorted ranges.
    pub fn new(left: L, right: R) -> Self {
        Self {
            left: left.peekable(),
            right: right.peekable(),
        }
    }
}

impl<'a, R, I> Iterator for Intersection<'a, R, I>
where
    R: RangeOrder + RangeIntersect + Clone,
    I: Iterator<Item = &'a R>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // if we don't have at least two ranges, there cannot be an intersection
            let (Some(left), Some(right)) = (self.left.peek(), self.right.peek()) else {
                return None;
            };

            let intersection = left.intersect(right);

            // pop the range that ends earlier
            if left.order_end(right).is_lt() {
                self.left.next();
            } else {
                self.right.next();
            }

            if let Some(intersection) = intersection {
                return Some(intersection);
            }
        }
    }
}
