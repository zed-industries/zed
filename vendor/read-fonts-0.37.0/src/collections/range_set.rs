//! Stores a disjoint collection of ranges over numeric types.
//!
//! Overlapping and adjacent ranges are automatically merged together.

use core::{
    cmp::{max, min},
    fmt::{Debug, Formatter},
    iter::Peekable,
    ops::RangeInclusive,
};
use std::collections::BTreeMap;

use types::Fixed;

#[derive(Default, Clone, PartialEq, Eq)]
/// A set of disjoint ranges over numeric types.
///
/// Overlapping and adjacent ranges are automatically merged together.
pub struct RangeSet<T> {
    // an entry in the map ranges[a] = b implies there is an range [a, b] (inclusive) in this set.
    ranges: BTreeMap<T, T>,
}

/// Allows a two values to be tested for adjacency.
pub trait OrdAdjacency {
    /// Returns true if self is adjacent on either side of rhs.
    fn are_adjacent(self, rhs: Self) -> bool;
}

impl<T> RangeSet<T>
where
    T: Ord + Copy + OrdAdjacency,
{
    // Returns true if there are no members in this set currently.
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Insert a range into this set, automatically merging with existing ranges as needed.
    pub fn insert(&mut self, range: RangeInclusive<T>) {
        if range.end() < range.start() {
            // ignore or malformed ranges.
            return;
        }

        let mut start = *range.start();
        let mut end = *range.end();

        // There may be up to one intersecting range prior to this new range, check for it and merge if needed.
        if let Some((prev_start, prev_end)) = self.prev_range(start) {
            if range_is_subset(start, end, prev_start, prev_end) {
                return;
            }
            if ranges_overlap_or_adjacent(start, end, prev_start, prev_end) {
                start = min(start, prev_start);
                end = max(end, prev_end);
                self.ranges.remove(&prev_start);
            }
        };

        // There may be one or more ranges proceeding this new range that intersect, find and merge them as needed.
        loop {
            let Some((next_start, next_end)) = self.next_range(start) else {
                // No existing ranges which might overlap, can now insert the current range
                self.ranges.insert(start, end);
                return;
            };

            if range_is_subset(start, end, next_start, next_end) {
                return;
            }
            if ranges_overlap_or_adjacent(start, end, next_start, next_end) {
                start = min(start, next_start);
                end = max(end, next_end);
                self.ranges.remove(&next_start);
            } else {
                self.ranges.insert(start, end);
                return;
            }
        }
    }

    /// Returns an iterator over the contained ranges.
    pub fn iter(&'_ self) -> impl Iterator<Item = RangeInclusive<T>> + '_ {
        self.ranges.iter().map(|(a, b)| *a..=*b)
    }

    /// Returns an iterator over the intersection of this and other.
    pub fn intersection<'a>(
        &'a self,
        other: &'a Self,
    ) -> impl Iterator<Item = RangeInclusive<T>> + 'a {
        IntersectionIter {
            it_a: self.iter().peekable(),
            it_b: other.iter().peekable(),
        }
    }

    /// Finds a range in this set with a start greater than or equal to the provided start value.
    fn next_range(&self, start: T) -> Option<(T, T)> {
        let (next_start, next_end) = self.ranges.range(start..).next()?;
        Some((*next_start, *next_end))
    }

    /// Finds a range in this set with a start less than the provided start value.
    fn prev_range(&self, start: T) -> Option<(T, T)> {
        let (next_start, next_end) = self.ranges.range(..start).next_back()?;
        Some((*next_start, *next_end))
    }
}

impl<T> Extend<RangeInclusive<T>> for RangeSet<T>
where
    T: Copy + Ord + OrdAdjacency,
{
    fn extend<I: IntoIterator<Item = RangeInclusive<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(|r| self.insert(r));
    }
}

impl<T> FromIterator<RangeInclusive<T>> for RangeSet<T>
where
    T: Default + Copy + Ord + OrdAdjacency,
{
    fn from_iter<I: IntoIterator<Item = RangeInclusive<T>>>(iter: I) -> Self {
        let mut result: Self = Default::default();
        result.extend(iter);
        result
    }
}

impl<T> Debug for RangeSet<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "RangeSet {{")?;
        for (start, end) in self.ranges.iter() {
            write!(f, "[{:?}, {:?}], ", start, end)?;
        }
        write!(f, "}}")
    }
}

struct IntersectionIter<A, B, T>
where
    A: Iterator<Item = RangeInclusive<T>>,
    B: Iterator<Item = RangeInclusive<T>>,
{
    it_a: Peekable<A>,
    it_b: Peekable<B>,
}

impl<A, B, T> Iterator for IntersectionIter<A, B, T>
where
    A: Iterator<Item = RangeInclusive<T>>,
    B: Iterator<Item = RangeInclusive<T>>,
    T: Ord + Copy,
{
    type Item = RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (Some(a), Some(b)) = (self.it_a.peek(), self.it_b.peek()) else {
                return None;
            };

            let a = a.clone();
            let b = b.clone();

            match range_intersection(&a, &b) {
                Some(intersection) => {
                    self.step_iterators(&a, &b);
                    return Some(intersection);
                }
                None => self.step_iterators(&a, &b),
            }
        }
    }
}

impl<A, B, T> IntersectionIter<A, B, T>
where
    A: Iterator<Item = RangeInclusive<T>>,
    B: Iterator<Item = RangeInclusive<T>>,
    T: Ord,
{
    fn step_iterators(&mut self, a: &RangeInclusive<T>, b: &RangeInclusive<T>) {
        if a.end() <= b.end() {
            self.it_a.next();
        }

        if a.end() >= b.end() {
            self.it_b.next();
        }
    }
}

impl OrdAdjacency for u32 {
    fn are_adjacent(self, rhs: u32) -> bool {
        matches!(self.checked_add(1).map(|r| r == rhs), Some(true))
            || matches!(rhs.checked_add(1).map(|r| r == self), Some(true))
    }
}

impl OrdAdjacency for u16 {
    fn are_adjacent(self, rhs: u16) -> bool {
        matches!(self.checked_add(1).map(|r| r == rhs), Some(true))
            || matches!(rhs.checked_add(1).map(|r| r == self), Some(true))
    }
}

impl OrdAdjacency for Fixed {
    fn are_adjacent(self, rhs: Fixed) -> bool {
        matches!(
            self.checked_add(Fixed::EPSILON).map(|r| r == rhs),
            Some(true)
        ) || matches!(
            rhs.checked_add(Fixed::EPSILON).map(|r| r == self),
            Some(true)
        )
    }
}

/// If a and b intersect return a range representing the intersection.
fn range_intersection<T: Ord + Copy>(
    a: &RangeInclusive<T>,
    b: &RangeInclusive<T>,
) -> Option<RangeInclusive<T>> {
    if a.start() <= b.end() && b.start() <= a.end() {
        Some(*max(a.start(), b.start())..=*min(a.end(), b.end()))
    } else {
        None
    }
}

/// Returns true if the ranges [a_start, a_end] and [b_start, b_end] overlap or are adjacent to each other.
///
/// All bounds are inclusive.
fn ranges_overlap_or_adjacent<T>(a_start: T, a_end: T, b_start: T, b_end: T) -> bool
where
    T: Ord + OrdAdjacency,
{
    (a_start <= b_end && b_start <= a_end)
        || (a_end.are_adjacent(b_start))
        || (b_end.are_adjacent(a_start))
}

/// Returns true if the range [a_start, a_end] is a subset of [b_start, b_end].
///
/// All bounds are inclusive.
fn range_is_subset<T>(a_start: T, a_end: T, b_start: T, b_end: T) -> bool
where
    T: Ord,
{
    a_start >= b_start && a_end <= b_end
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn insert_invalid() {
        let mut map: RangeSet<u32> = Default::default();
        map.insert(12..=11);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![],);
    }

    #[test]
    fn insert_non_overlapping() {
        let mut map: RangeSet<u32> = Default::default();

        map.insert(11..=11);
        map.insert(2..=3);
        map.insert(6..=9);

        assert_eq!(map.iter().collect::<Vec<_>>(), vec![2..=3, 6..=9, 11..=11],);
    }

    #[test]
    fn insert_subset_before() {
        let mut map: RangeSet<u32> = Default::default();

        map.insert(2..=8);
        map.insert(3..=7);

        assert_eq!(map.iter().collect::<Vec<_>>(), vec![2..=8],);
    }

    #[test]
    fn insert_subset_after() {
        let mut map: RangeSet<u32> = Default::default();

        map.insert(2..=8);
        map.insert(2..=7);
        map.insert(2..=8);

        assert_eq!(map.iter().collect::<Vec<_>>(), vec![2..=8],);
    }

    #[test]
    fn insert_overlapping_before() {
        let mut map: RangeSet<u32> = Default::default();

        map.insert(2..=8);
        map.insert(7..=11);

        assert_eq!(map.iter().collect::<Vec<_>>(), vec![2..=11],);
    }

    #[test]
    fn insert_overlapping_after() {
        let mut map: RangeSet<u32> = Default::default();
        map.insert(10..=14);
        map.insert(7..=11);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![7..=14],);

        let mut map: RangeSet<u32> = Default::default();
        map.insert(10..=14);
        map.insert(10..=17);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![10..=17],);
    }

    #[test]
    fn insert_overlapping_multiple_after() {
        let mut map: RangeSet<u32> = Default::default();
        map.insert(10..=14);
        map.insert(16..=17);
        map.insert(7..=16);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![7..=17],);

        let mut map: RangeSet<u32> = Default::default();
        map.insert(10..=14);
        map.insert(16..=17);
        map.insert(10..=16);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![10..=17],);

        let mut map: RangeSet<u32> = Default::default();
        map.insert(10..=14);
        map.insert(16..=17);
        map.insert(10..=17);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![10..=17],);
    }

    #[test]
    fn insert_overlapping_before_and_after() {
        let mut map: RangeSet<u32> = Default::default();

        map.insert(6..=8);
        map.insert(10..=14);
        map.insert(16..=20);

        map.insert(7..=19);

        assert_eq!(map.iter().collect::<Vec<_>>(), vec![6..=20],);
    }

    #[test]
    fn insert_joins_adjacent() {
        let mut map: RangeSet<u32> = Default::default();
        map.insert(6..=8);
        map.insert(9..=10);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![6..=10],);

        let mut map: RangeSet<u32> = Default::default();
        map.insert(9..=10);
        map.insert(6..=8);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![6..=10],);

        let mut map: RangeSet<u32> = Default::default();
        map.insert(6..=8);
        map.insert(10..=10);
        map.insert(9..=9);
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![6..=10],);
    }

    #[test]
    fn from_iter_and_extend() {
        let mut map: RangeSet<u32> = [2..=5, 13..=64, 7..=9].into_iter().collect();
        assert_eq!(map.iter().collect::<Vec<_>>(), vec![2..=5, 7..=9, 13..=64],);

        map.extend([6..=17, 100..=101]);

        assert_eq!(map.iter().collect::<Vec<_>>(), vec![2..=64, 100..=101],);
    }

    #[test]
    fn intersection() {
        let a: RangeSet<u32> = [2..=5, 7..=9, 13..=64].into_iter().collect();
        let b: RangeSet<u32> = [1..=3, 5..=8, 13..=64, 67..=69].into_iter().collect();

        let expected = vec![2..=3, 5..=5, 7..=8, 13..=64];

        assert_eq!(a.intersection(&b).collect::<Vec<_>>(), expected);
        assert_eq!(b.intersection(&a).collect::<Vec<_>>(), expected);
    }
}
