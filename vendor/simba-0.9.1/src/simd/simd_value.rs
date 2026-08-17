use crate::simd::SimdBool;

/// Base trait for every SIMD types.
pub trait SimdValue: Sized {
    /// The number of lanes of this SIMD value.
    const LANES: usize;
    /// The type of the elements of each lane of this SIMD value.
    type Element: SimdValue<Element = Self::Element, SimdBool = bool>;
    /// Type of the result of comparing two SIMD values like `self`.
    type SimdBool: SimdBool;

    /// Initializes an SIMD value with each lanes set to `val`.
    fn splat(val: Self::Element) -> Self;
    /// Extracts the i-th lane of `self`.
    ///
    /// Panics if `i >= Self::LANES`.
    fn extract(&self, i: usize) -> Self::Element;
    /// Extracts the i-th lane of `self` without bound-checking.
    ///
    /// # Safety
    /// Undefined behavior if `i >= Self::LANES`.
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element;
    /// Replaces the i-th lane of `self` by `val`.
    ///
    /// Panics if `i >= Self::LANES`.
    fn replace(&mut self, i: usize, val: Self::Element);
    /// Replaces the i-th lane of `self` by `val` without bound-checking.
    ///
    /// # Safety
    /// Undefined behavior if `i >= Self::LANES`.
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element);

    /// Merges `self` and `other` depending on the lanes of `cond`.
    ///
    /// For each lane of `cond` with bits set to 1, the result's will contain the value of the lane of `self`.
    /// For each lane of `cond` with bits set to 0, the result's will contain the value of the lane of `other`.
    fn select(self, cond: Self::SimdBool, other: Self) -> Self;

    /// Applies a function to each lane of `self`.
    ///
    /// Note that, while convenient, this method can be extremely slow as this
    /// requires to extract each lane of `self` and then combine them again into
    /// a new SIMD value.
    #[inline(always)]
    fn map_lanes(self, f: impl Fn(Self::Element) -> Self::Element) -> Self
    where
        Self: Clone,
    {
        let mut result = self.clone();

        for i in 0..Self::LANES {
            unsafe { result.replace_unchecked(i, f(self.extract_unchecked(i))) }
        }

        result
    }

    /// Applies a function to each lane of `self` paired with the corresponding lane of `b`.
    ///
    /// Note that, while convenient, this method can be extremely slow as this
    /// requires to extract each lane of `self` and then combine them again into
    /// a new SIMD value.
    #[inline(always)]
    fn zip_map_lanes(
        self,
        b: Self,
        f: impl Fn(Self::Element, Self::Element) -> Self::Element,
    ) -> Self
    where
        Self: Clone,
    {
        let mut result = self.clone();

        for i in 0..Self::LANES {
            unsafe {
                let a = self.extract_unchecked(i);
                let b = b.extract_unchecked(i);
                result.replace_unchecked(i, f(a, b))
            }
        }

        result
    }
}

/// Marker trait implemented by SIMD and non-SIMD primitive numeric values.
///
/// This trait is useful for some disambiguations when writing blanked impls.
/// This is implemented by all unsigned integer, integer, float, and complex types, as
/// with only one lane, i.e., `f32`, `f64`, `u32`, `i64`, etc. as well as SIMD types like
/// `f32x4, i32x8`, etc..
pub trait PrimitiveSimdValue: Copy + SimdValue {}

impl<N: SimdValue> SimdValue for num_complex::Complex<N> {
    const LANES: usize = N::LANES;
    type Element = num_complex::Complex<N::Element>;
    type SimdBool = N::SimdBool;

    #[inline(always)]
    fn splat(val: Self::Element) -> Self {
        num_complex::Complex {
            re: N::splat(val.re),
            im: N::splat(val.im),
        }
    }

    #[inline(always)]
    fn extract(&self, i: usize) -> Self::Element {
        num_complex::Complex {
            re: self.re.extract(i),
            im: self.im.extract(i),
        }
    }

    #[inline(always)]
    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        num_complex::Complex {
            re: self.re.extract_unchecked(i),
            im: self.im.extract_unchecked(i),
        }
    }

    #[inline(always)]
    fn replace(&mut self, i: usize, val: Self::Element) {
        self.re.replace(i, val.re);
        self.im.replace(i, val.im);
    }

    #[inline(always)]
    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        self.re.replace_unchecked(i, val.re);
        self.im.replace_unchecked(i, val.im);
    }

    #[inline(always)]
    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        num_complex::Complex {
            re: self.re.select(cond, other.re),
            im: self.im.select(cond, other.im),
        }
    }
}

impl<N: PrimitiveSimdValue> PrimitiveSimdValue for num_complex::Complex<N> {}

macro_rules! impl_primitive_simd_value_for_scalar (
    ($($t: ty),*) => {$(
        impl PrimitiveSimdValue for $t {}
        impl SimdValue for $t {
            const LANES: usize = 1;
            type Element = $t;
            type SimdBool = bool;

            #[inline(always)]
            fn splat(val: Self::Element) -> Self {
                val
            }

            #[inline(always)]
            fn extract(&self, _: usize) -> Self::Element {
                *self
            }

            #[inline(always)]
            unsafe fn extract_unchecked(&self, _: usize) -> Self::Element {
                *self
            }

            #[inline(always)]
            fn replace(&mut self, _: usize, val: Self::Element) {
                *self = val
            }

            #[inline(always)]
            unsafe fn replace_unchecked(&mut self, _: usize, val: Self::Element) {
                *self = val
            }

            #[inline(always)]
            fn select(self, cond: Self::SimdBool, other: Self) -> Self {
                if cond {
                    self
                } else {
                    other
                }
            }
        }
    )*}
);

impl_primitive_simd_value_for_scalar!(
    bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
#[cfg(feature = "decimal")]
impl_primitive_simd_value_for_scalar!(decimal::d128);
