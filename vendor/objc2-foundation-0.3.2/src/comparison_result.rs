use core::cmp::Ordering;

use objc2::encode::{Encode, Encoding, RefEncode};

/// Constants that indicate sort order.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nscomparisonresult?language=objc).
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NSComparisonResult {
    /// The left operand is smaller than the right operand.
    Ascending = -1,
    /// The two operands are equal.
    Same = 0,
    /// The left operand is greater than the right operand.
    Descending = 1,
}

impl Default for NSComparisonResult {
    #[inline]
    fn default() -> Self {
        Self::Same
    }
}

unsafe impl Encode for NSComparisonResult {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for NSComparisonResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl From<Ordering> for NSComparisonResult {
    #[inline]
    fn from(order: Ordering) -> Self {
        match order {
            Ordering::Less => Self::Ascending,
            Ordering::Equal => Self::Same,
            Ordering::Greater => Self::Descending,
        }
    }
}

impl From<NSComparisonResult> for Ordering {
    #[inline]
    fn from(comparison_result: NSComparisonResult) -> Self {
        match comparison_result {
            NSComparisonResult::Ascending => Self::Less,
            NSComparisonResult::Same => Self::Equal,
            NSComparisonResult::Descending => Self::Greater,
        }
    }
}
