//! When we are combining two NFAs, we will grab all the outgoing
//! edges from a set of nodes and wind up with a bunch of potentially
//! overlapping character ranges like:
//!
//!     a-z
//!     c-l
//!     0-9
//!
//! This module contains code to turn those into non-overlapping ranges like:
//!
//!     a-b
//!     c-l
//!     m-z
//!     0-9
//!
//! Specifically, we want to ensure that the same set of characters is
//! covered when we started, and that each of the input ranges is
//! covered precisely by some set of ranges in the output.

use crate::collections::Set;
use crate::lexer::nfa::Test;
use std::cmp;
use std::ops::RangeInclusive;

pub fn remove_overlap(ranges: &Set<Test>) -> Vec<Test> {
    // We will do this in the dumbest possible way to start. :)
    // Maintain a result vector that contains disjoint ranges.  To
    // insert a new range, we walk over this vector and split things
    // up as we go. This algorithm is so naive as to be exponential, I
    // think. Sue me.

    let mut disjoint_ranges = vec![];

    for range in ranges {
        add_range(range.to_owned(), 0, &mut disjoint_ranges);
    }

    // the algorithm above leaves some empty ranges in for simplicity;
    // prune them out.
    disjoint_ranges.retain(|r| !r.is_empty());

    disjoint_ranges.sort();

    disjoint_ranges
}

fn add_range(range: Test, start_index: usize, disjoint_ranges: &mut Vec<Test>) {
    if range.is_empty() {
        return;
    }

    // Find first overlapping range in `disjoint_ranges`, if any.
    match disjoint_ranges[start_index..]
        .iter()
        .position(|r| r.intersects(&range))
    {
        Some(index) => {
            let index = index + start_index;
            let overlapping_range = &disjoint_ranges[index];

            // If the range we are trying to add already exists, we're all done.
            if overlapping_range == &range {
                return;
            }

            // Otherwise, we want to create three ranges (some of which may
            // be empty). e.g. imagine one range is `a-z` and the other
            // is `c-l`, we want `a-b`, `c-l`, and `m-z`.
            let min_min = cmp::min(range.start(), overlapping_range.start());
            let mid_min = cmp::max(range.start(), overlapping_range.start());
            let mid_max = cmp::min(range.end(), overlapping_range.end());
            let max_max = cmp::max(range.end(), overlapping_range.end());
            // When working with inclusive ranges, we need to be sure to not double count
            // the meeting points of low-mid_range and mid-max_range.
            // So we adjust the end of the low_range and start of max_range as these elements are already included in the start of their corresponding next ranges.

            let low_range = if mid_min == 0 {
                // This is an edgecase where both ranges start at the null character
                // In this case we don't want to create a range from 0 to -1
                // Thus we create an empty range
                Test::new(RangeInclusive::new(1, 0))
            } else {
                Test::new(min_min..=mid_min - 1)
            };
            let mid_range = Test::new(mid_min..=mid_max);
            let max_range = Test::new(mid_max + 1..=max_max);

            assert!(low_range.is_disjoint(&mid_range));
            assert!(low_range.is_disjoint(&max_range));
            assert!(mid_range.is_disjoint(&max_range));

            // Replace the existing range with the low range, and then
            // add the mid and max ranges in. (The low range may be
            // empty, but we'll prune that out later.)
            disjoint_ranges[index] = low_range;
            add_range(mid_range, index + 1, disjoint_ranges);
            add_range(max_range, index + 1, disjoint_ranges);
        }

        None => {
            // no overlap -- easy case.
            disjoint_ranges.push(range);
        }
    }
}

#[cfg(test)]
macro_rules! test {
    ($($range:expr,)*) => {
        {
            use crate::collections::set;
            use crate::lexer::nfa::Test;
            use std::char;
            let mut s = set();
            $({ let r = $range; s.insert(Test::inclusive_range(*r.start(), *r.end())); })*
            remove_overlap(&s).into_iter()
                              .map(|r|
                                   char::from_u32(r.start()).unwrap() ..=
                                   char::from_u32(r.end()).unwrap())
                              .collect::<Vec<_>>()
        }
    }
}

#[test]
fn alphabet() {
    let result = test! {
        'a' ..= 'z',
        'c' ..= 'l',
        '0' ..= '9',
    };
    assert_eq!(result, vec!['0'..='9', 'a'..='b', 'c'..='l', 'm'..='z']);
}

#[test]
fn repeat() {
    let result = test! {
        'a' ..= 'z',
        'c' ..= 'l',
        'l' ..= 'z',
        '0' ..= '9',
    };
    assert_eq!(
        result,
        vec!['0'..='9', 'a'..='b', 'c'..='k', 'l'..='l', 'm'..='z']
    );
}

#[test]
fn stagger() {
    let result = test! {
        '0' ..= '3',
        '2' ..= '4',
        '3' ..= '5',
    };
    assert_eq!(
        result,
        vec!['0'..='1', '2'..='2', '3'..='3', '4'..='4', '5'..='5']
    );
}

#[test]
fn empty_range() {
    let result = test! {
        'b' ..= 'b',
        'a' ..= 'z',
    };
    assert_eq!(result, vec!['a'..='a', 'b'..='b', 'c'..='z']);
}

#[test]
fn null() {
    let result = test! {
        '\0' ..= '\0',
        '\0' ..= 'a',
    };
    assert_eq!(result, vec!['\0'..='\0', 1 as char..='a']);
}
