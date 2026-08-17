use crate::simd::{SimdBool, SimdValue};

//pub trait SimdOption {
//    type SimdValue: SimdValue;
//
//    fn simd_unwrap(self) -> Self::SimdValue;
//    fn simd_unwrap_or(self, other: impl FnOnce() -> Self::SimdValue) -> Self::SimdValue;
//}

//impl<T: SimdValue> SimdOption for Option<T> {
//    type SimdValue = T;
//
//    #[inline(always)]
//    fn simd_unwrap(self) -> T {
//        self.unwrap()
//    }
//
//    #[inline(always)]
//    fn simd_unwrap_or(self, other: impl FnOnce() -> Self::SimdValue) -> Self::SimdValue {
//        self.unwrap_or_else(other)
//    }
//}

/// Generalization of Option for SIMD computation.
pub struct SimdOption<V: SimdValue> {
    val: V,
    mask: V::SimdBool,
}

impl<V: SimdValue> SimdOption<V> {
    /// Creates a new SIMD option by combining a value and a mask indicating what lane of the value is valid.
    pub fn new(val: V, mask: V::SimdBool) -> Self {
        SimdOption { val, mask }
    }

    /// Return the underlying SIMD boolean mask.
    pub fn mask(&self) -> V::SimdBool {
        self.mask
    }

    /// Return the underlying unfiltered value.
    pub fn value(&self) -> &V {
        &self.val
    }

    /// Converts this SIMD option to a strandard Option.
    ///
    /// If all the bits of `self.mask` are 1, then this returns `Some(self.value())`.
    /// If any bit of `self.mask` is 0, then this returns `None`.
    pub fn option(self) -> Option<V> {
        if self.mask.all() {
            Some(self.val)
        } else {
            None
        }
    }

    /// Retrieve the underlying value if all the bits of `self.mask` are 1.
    ///
    /// Panics if any of the bits of `self.mask` is 0.
    #[inline]
    pub fn simd_unwrap(self) -> V {
        assert!(
            self.mask.all(),
            "Attempt to unwrap an SIMD value with at least one false lane."
        );
        self.val
    }

    /// Merges the value of `self` with the value of `other`.
    ///
    /// Each lane of the result with a corresponding bit mask set to 1 will be filled with the corresponding lanes of `self.value()`.
    /// The lanes of the result with a corresponding bit mask set to 0 will be filled with the corresponding lanes of `other()`.
    ///
    /// The function in `other` should not do any side-effect. Indeed, implementors of this trait are free to decide in what
    /// cases `other` is called or not.
    #[inline(always)]
    pub fn simd_unwrap_or(self, other: impl FnOnce() -> V) -> V {
        self.val.select(self.mask, other())
    }
}
