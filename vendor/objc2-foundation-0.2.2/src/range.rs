use core::ops::Range;

use objc2::encode::{Encode, Encoding, RefEncode};

use crate::Foundation::NSUInteger;

/// TODO.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsrange?language=objc).
#[repr(C)]
// PartialEq is same as NSEqualRanges
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRange {
    /// The lower bound of the range (inclusive).
    pub location: NSUInteger,
    /// The number of items in the range, starting from `location`.
    pub length: NSUInteger,
}

impl NSRange {
    /// Create a new range with the given values.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::NSRange;
    /// assert_eq!(NSRange::new(3, 2), NSRange::from(3..5));
    /// ```
    #[inline]
    #[doc(alias = "NSMakeRange")]
    pub const fn new(location: usize, length: usize) -> Self {
        // Equivalent to NSMakeRange
        Self { location, length }
    }

    /// Returns `true` if the range contains no items.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::NSRange;
    ///
    /// assert!(!NSRange::from(3..5).is_empty());
    /// assert!( NSRange::from(3..3).is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns `true` if the index is within the range.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::NSRange;
    ///
    /// assert!(!NSRange::from(3..5).contains(2));
    /// assert!( NSRange::from(3..5).contains(3));
    /// assert!( NSRange::from(3..5).contains(4));
    /// assert!(!NSRange::from(3..5).contains(5));
    ///
    /// assert!(!NSRange::from(3..3).contains(3));
    /// ```
    #[inline]
    #[doc(alias = "NSLocationInRange")]
    pub fn contains(&self, index: usize) -> bool {
        // Same as NSLocationInRange
        if let Some(len) = index.checked_sub(self.location) {
            len < self.length
        } else {
            // index < self.location
            false
        }
    }

    /// Returns the upper bound of the range (exclusive).
    #[inline]
    #[doc(alias = "NSMaxRange")]
    pub fn end(&self) -> usize {
        self.location
            .checked_add(self.length)
            .expect("NSRange too large")
    }

    // TODO: https://developer.apple.com/documentation/foundation/1408420-nsrangefromstring
    // TODO: NSUnionRange
    // TODO: NSIntersectionRange
}

// Sadly, we can't do this:
// impl RangeBounds<usize> for NSRange {
//     fn start_bound(&self) -> Bound<&usize> {
//         Bound::Included(&self.location)
//     }
//     fn end_bound(&self) -> Bound<&usize> {
//         Bound::Excluded(&(self.location + self.length))
//     }
// }

impl From<Range<usize>> for NSRange {
    fn from(range: Range<usize>) -> Self {
        let length = range
            .end
            .checked_sub(range.start)
            .expect("Range end < start");
        Self {
            location: range.start,
            length,
        }
    }
}

impl From<NSRange> for Range<usize> {
    #[inline]
    fn from(nsrange: NSRange) -> Self {
        Self {
            start: nsrange.location,
            end: nsrange.end(),
        }
    }
}

unsafe impl Encode for NSRange {
    const ENCODING: Encoding = Encoding::Struct("_NSRange", &[usize::ENCODING, usize::ENCODING]);
}

unsafe impl RefEncode for NSRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_range() {
        let cases: &[(Range<usize>, NSRange)] = &[
            (0..0, NSRange::new(0, 0)),
            (0..10, NSRange::new(0, 10)),
            (10..10, NSRange::new(10, 0)),
            (10..20, NSRange::new(10, 10)),
        ];

        for (range, expected) in cases {
            assert_eq!(NSRange::from(range.clone()), *expected);
        }
    }

    #[test]
    #[should_panic = "Range end < start"]
    #[allow(clippy::reversed_empty_ranges)]
    fn test_from_range_inverted() {
        let _ = NSRange::from(10..0);
    }

    #[test]
    fn test_contains() {
        let range = NSRange::from(10..20);
        assert!(!range.contains(0));
        assert!(!range.contains(9));
        assert!(range.contains(10));
        assert!(range.contains(11));
        assert!(!range.contains(20));
        assert!(!range.contains(21));
    }

    #[test]
    fn test_end() {
        let range = NSRange::from(10..20);
        assert!(!range.contains(0));
        assert!(!range.contains(9));
        assert!(range.contains(10));
        assert!(range.contains(11));
        assert!(!range.contains(20));
        assert!(!range.contains(21));
    }

    #[test]
    #[should_panic = "NSRange too large"]
    fn test_end_large() {
        let _ = NSRange::new(usize::MAX, usize::MAX).end();
    }
}
