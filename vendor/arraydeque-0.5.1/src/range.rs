use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

/// **RangeArgument** is implemented by Rust's built-in range types, produced
/// by range syntax like `..`, `a..`, `..b` or `c..d`.
pub trait RangeArgument<T = usize> {
    #[inline]
    /// Start index (inclusive)
    fn start(&self) -> Option<T> {
        None
    }
    #[inline]
    /// End index (exclusive)
    fn end(&self) -> Option<T> {
        None
    }
}

impl<T> RangeArgument<T> for RangeFull {}

impl<T: Copy> RangeArgument<T> for RangeFrom<T> {
    #[inline]
    fn start(&self) -> Option<T> {
        Some(self.start)
    }
}

impl<T: Copy> RangeArgument<T> for RangeTo<T> {
    #[inline]
    fn end(&self) -> Option<T> {
        Some(self.end)
    }
}

impl<T: Copy> RangeArgument<T> for Range<T> {
    #[inline]
    fn start(&self) -> Option<T> {
        Some(self.start)
    }
    #[inline]
    fn end(&self) -> Option<T> {
        Some(self.end)
    }
}
