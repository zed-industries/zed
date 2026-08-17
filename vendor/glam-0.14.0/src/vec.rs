// Adds common vector methods to an impl.
// The methods here should be supported for all types of $t and all sizes of vector.
macro_rules! impl_vecn_common_methods {
    ($t:ty, $vecn:ident, $mask:ident, $inner:ident, $vectrait:ident) => {
        /// Creates a vector with all elements set to `0.0`.
        #[deprecated = "use ZERO constant instead"]
        #[inline(always)]
        pub const fn zero() -> Self {
            Self::ZERO
        }

        /// Creates a vector with all elements set to `1.0`.
        #[deprecated = "use ONE constant instead"]
        #[inline(always)]
        pub const fn one() -> Self {
            Self::ONE
        }

        /// Creates a vector with all elements set to `v`.
        #[inline(always)]
        pub fn splat(v: $t) -> Self {
            Self($inner::splat(v))
        }

        /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
        /// for each element of `self`.
        ///
        /// A true element in the mask uses the corresponding element from `if_true`, and false
        /// uses the element from `if_false`.
        #[inline(always)]
        pub fn select(mask: $mask, if_true: $vecn, if_false: $vecn) -> $vecn {
            Self($inner::select(mask.0, if_true.0, if_false.0))
        }

        /// Computes the dot product of `self` and `other`.
        #[inline(always)]
        pub fn dot(self, other: Self) -> $t {
            $vectrait::dot(self.0, other.0)
        }

        /// Returns a vector containing the mininum values for each element of `self` and `other`.
        ///
        /// In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
        #[inline(always)]
        pub fn min(self, other: Self) -> Self {
            Self(self.0.min(other.0))
        }

        /// Returns a vector containing the maximum values for each element of `self` and `other`.
        ///
        /// In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
        #[inline(always)]
        pub fn max(self, other: Self) -> Self {
            Self(self.0.max(other.0))
        }

        /// Component-wise clamping of values, similar to [`std::f32::clamp`].
        ///
        /// Each element in `min` must be less-or-equal to the corresponing element in `max`.
        ///
        /// If the `glam-assert` feature is enabled, the function will panic if the contract is not
        /// met, otherwise the behavior is undefined.
        #[inline(always)]
        pub fn clamp(self, min: Self, max: Self) -> Self {
            Self($vectrait::clamp(self.0, min.0, max.0))
        }

        /// Returns the horizontal minimum of `self`.
        ///
        /// In other words this computes `min(x, y, ..)`.
        #[inline(always)]
        pub fn min_element(self) -> $t {
            $vectrait::min_element(self.0)
        }

        /// Returns the horizontal maximum of `self`.
        ///
        /// In other words this computes `max(x, y, ..)`.
        #[inline(always)]
        pub fn max_element(self) -> $t {
            $vectrait::max_element(self.0)
        }

        /// Returns a vector mask containing the result of a `==` comparison for each element of
        /// `self` and `other`.
        ///
        /// In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
        /// elements.
        #[inline(always)]
        pub fn cmpeq(self, other: Self) -> $mask {
            $mask(self.0.cmpeq(other.0))
        }

        /// Returns a vector mask containing the result of a `!=` comparison for each element of
        /// `self` and `other`.
        ///
        /// In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
        /// elements.
        #[inline(always)]
        pub fn cmpne(self, other: Self) -> $mask {
            $mask(self.0.cmpne(other.0))
        }

        /// Returns a vector mask containing the result of a `>=` comparison for each element of
        /// `self` and `other`.
        ///
        /// In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
        /// elements.
        #[inline(always)]
        pub fn cmpge(self, other: Self) -> $mask {
            $mask(self.0.cmpge(other.0))
        }

        /// Returns a vector mask containing the result of a `>` comparison for each element of
        /// `self` and `other`.
        ///
        /// In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
        /// elements.
        #[inline(always)]
        pub fn cmpgt(self, other: Self) -> $mask {
            $mask(self.0.cmpgt(other.0))
        }

        /// Returns a vector mask containing the result of a `<=` comparison for each element of
        /// `self` and `other`.
        ///
        /// In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
        /// elements.
        #[inline(always)]
        pub fn cmple(self, other: Self) -> $mask {
            $mask(self.0.cmple(other.0))
        }

        /// Returns a vector mask containing the result of a `<` comparison for each element of
        /// `self` and `other`.
        ///
        /// In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
        /// elements.
        #[inline(always)]
        pub fn cmplt(self, other: Self) -> $mask {
            $mask(self.0.cmplt(other.0))
        }

        /// Creates a vector from the first N values in `slice`.
        ///
        /// # Panics
        ///
        /// Panics if `slice` is less than N elements long.
        #[inline(always)]
        pub fn from_slice_unaligned(slice: &[$t]) -> Self {
            Self($vectrait::from_slice_unaligned(slice))
        }

        /// Writes the elements of `self` to the first N elements in `slice`.
        ///
        /// # Panics
        ///
        /// Panics if `slice` is less than N elements long.
        #[inline(always)]
        pub fn write_to_slice_unaligned(self, slice: &mut [$t]) {
            $vectrait::write_to_slice_unaligned(self.0, slice)
        }

        /// Per element multiplication/addition of the three inputs: b + (self * a)
        #[inline(always)]
        #[allow(dead_code)]
        pub(crate) fn mul_add(self, a: Self, b: Self) -> Self {
            Self(self.0.mul_add(a.0, b.0))
        }
    };
}

// Adds signed type vector methods to an impl.
// The methods here should be supported for signed types of $t and all sizes of vector.
macro_rules! impl_vecn_signed_methods {
    ($t:ty, $vecn:ident, $mask:ident, $inner:ident, $sgntrait:ident) => {
        // impl_vecn_common_methods!($t, $vecn, $mask, $inner, $vectrait);

        /// Returns a vector containing the absolute value of each element of `self`.
        #[inline(always)]
        pub fn abs(self) -> Self {
            Self($sgntrait::abs(self.0))
        }

        /// Returns a vector with elements representing the sign of `self`.
        ///
        /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
        /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
        /// - `NAN` if the number is `NAN`
        #[inline(always)]
        pub fn signum(self) -> Self {
            Self($sgntrait::signum(self.0))
        }
    };
}

// Adds float type vector methods to an impl.
// The methods here should be supported for float types of $t and all sizes of vector.
macro_rules! impl_vecn_float_methods {
    ($t:ty, $vecn:ident, $mask:ident, $inner:ident, $flttrait:ident) => {
        // impl_vecn_signed_methods!($t, $vecn, $mask, $inner, $sgntrait, $vectrait);

        /// Returns `true` if, and only if, all elements are finite.  If any element is either
        /// `NaN`, positive or negative infinity, this will return `false`.
        #[inline(always)]
        pub fn is_finite(self) -> bool {
            $flttrait::is_finite(self.0)
        }

        /// Returns `true` if any elements are `NaN`.
        #[inline(always)]
        pub fn is_nan(self) -> bool {
            $flttrait::is_nan(self.0)
        }

        /// Performs `is_nan` on each element of self, returning a vector mask of the results.
        ///
        /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
        #[inline(always)]
        pub fn is_nan_mask(self) -> $mask {
            $mask($flttrait::is_nan_mask(self.0))
        }

        /// Computes the length of `self`.
        #[cfg_attr(docsrs, doc(alias = "magnitude"))]
        #[inline(always)]
        pub fn length(self) -> $t {
            $flttrait::length(self.0)
        }

        /// Computes the squared length of `self`.
        ///
        /// This is faster than `length()` as it avoids a square root operation.
        #[cfg_attr(docsrs, doc(alias = "magnitude2"))]
        #[inline(always)]
        pub fn length_squared(self) -> $t {
            $flttrait::length_squared(self.0)
        }

        /// Computes `1.0 / length()`.
        ///
        /// For valid results, `self` must _not_ be of length zero.
        #[inline(always)]
        pub fn length_recip(self) -> $t {
            $flttrait::length_recip(self.0)
        }

        /// Computes the Euclidean distance between two points in space.
        #[inline]
        pub fn distance(self, other: Self) -> $t {
            (self - other).length()
        }

        /// Compute the squared euclidean distance between two points in space.
        #[inline]
        pub fn distance_squared(self, other: Self) -> $t {
            (self - other).length_squared()
        }

        /// Returns `self` normalized to length 1.0.
        ///
        /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
        ///
        /// See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
        #[inline(always)]
        pub fn normalize(self) -> Self {
            Self($flttrait::normalize(self.0))
        }

        /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
        ///
        /// In particular, if the input is zero (or very close to zero), or non-finite,
        /// the result of this operation will be `None`.
        ///
        /// See also [`Self::normalize_or_zero`].
        #[inline]
        pub fn try_normalize(self) -> Option<Self> {
            let rcp = self.length_recip();
            if rcp.is_finite() && rcp > 0.0 {
                Some(self * rcp)
            } else {
                None
            }
        }

        /// Returns `self` normalized to length 1.0 if possible, else returns zero.
        ///
        /// In particular, if the input is zero (or very close to zero), or non-finite,
        /// the result of this operation will be zero.
        ///
        /// See also [`Self::try_normalize`].
        #[inline]
        pub fn normalize_or_zero(self) -> Self {
            let rcp = self.length_recip();
            if rcp.is_finite() && rcp > 0.0 {
                self * rcp
            } else {
                Self::ZERO
            }
        }

        /// Returns whether `self` is length `1.0` or not.
        ///
        /// Uses a precision threshold of `1e-6`.
        #[inline(always)]
        pub fn is_normalized(self) -> bool {
            $flttrait::is_normalized(self.0)
        }

        /// Returns a vector containing the nearest integer to a number for each element of `self`.
        /// Round half-way cases away from 0.0.
        #[inline(always)]
        pub fn round(self) -> Self {
            Self($flttrait::round(self.0))
        }

        /// Returns a vector containing the largest integer less than or equal to a number for each
        /// element of `self`.
        #[inline(always)]
        pub fn floor(self) -> Self {
            Self($flttrait::floor(self.0))
        }

        /// Returns a vector containing the smallest integer greater than or equal to a number for
        /// each element of `self`.
        #[inline(always)]
        pub fn ceil(self) -> Self {
            Self($flttrait::ceil(self.0))
        }

        /// Returns a vector containing `e^self` (the exponential function) for each element of
        /// `self`.
        #[inline(always)]
        pub fn exp(self) -> Self {
            Self($flttrait::exp(self.0))
        }

        /// Returns a vector containing each element of `self` raised to the power of `n`.
        #[inline(always)]
        pub fn powf(self, n: $t) -> Self {
            Self($flttrait::powf(self.0, n))
        }

        /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
        #[inline(always)]
        pub fn recip(self) -> Self {
            Self($flttrait::recip(self.0))
        }

        /// Performs a linear interpolation between `self` and `other` based on the value `s`.
        ///
        /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
        /// will be equal to `other`.
        #[cfg_attr(docsrs, doc(alias = "mix"))]
        #[inline]
        pub fn lerp(self, other: Self, s: $t) -> Self {
            self + ((other - self) * s)
        }

        /// Returns true if the absolute difference of all elements between `self` and `other` is
        /// less than or equal to `max_abs_diff`.
        ///
        /// This can be used to compare if two vectors contain similar elements. It works best when
        /// comparing with a known value. The `max_abs_diff` that should be used used depends on
        /// the values being compared against.
        ///
        /// For more see
        /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
        #[inline(always)]
        pub fn abs_diff_eq(self, other: Self, max_abs_diff: $t) -> bool {
            $flttrait::abs_diff_eq(self.0, other.0, max_abs_diff)
        }

        /// Returns a vector with a length no less than `min` and no more than `max`
        #[inline]
        pub fn clamp_length(self, min: $t, max: $t) -> Self {
            let length_sq = self.length_squared();
            if length_sq < min * min {
                self * (length_sq.sqrt().recip() * min)
            } else if length_sq > max * max {
                self * (length_sq.sqrt().recip() * max)
            } else {
                self
            }
        }

        /// Returns a vector with a length no more than `max`
        pub fn clamp_length_max(self, max: $t) -> Self {
            let length_sq = self.length_squared();
            if length_sq > max * max {
                self * (length_sq.sqrt().recip() * max)
            } else {
                self
            }
        }

        /// Returns a vector with a length no less than `min`
        pub fn clamp_length_min(self, min: $t) -> Self {
            let length_sq = self.length_squared();
            if self.length_squared() < min * min {
                self * (length_sq.sqrt().recip() * min)
            } else {
                self
            }
        }
    };
}

// Adds common vector trait implementations.
// The traits here should be supported for all types of $t and all sizes of vector.
macro_rules! impl_vecn_common_traits {
    ($t:ty, $size:literal, $vecn:ident, $inner:ident, $trait:ident) => {
        impl Default for $vecn {
            #[inline(always)]
            fn default() -> Self {
                Self($inner::ZERO)
            }
        }

        impl PartialEq for $vecn {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                self.cmpeq(*other).all()
            }
        }

        impl PartialOrd for $vecn {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.as_ref().partial_cmp(other.as_ref())
            }
        }

        impl From<$vecn> for $inner {
            #[inline(always)]
            fn from(t: $vecn) -> Self {
                t.0
            }
        }

        impl From<$inner> for $vecn {
            #[inline(always)]
            fn from(t: $inner) -> Self {
                Self(t)
            }
        }

        impl Div<$vecn> for $vecn {
            type Output = Self;
            #[inline(always)]
            fn div(self, other: $vecn) -> Self {
                Self(self.0.div(other.0))
            }
        }

        impl DivAssign<$vecn> for $vecn {
            #[inline(always)]
            fn div_assign(&mut self, other: $vecn) {
                self.0 = self.0.div(other.0)
            }
        }

        impl Div<$t> for $vecn {
            type Output = Self;
            #[inline(always)]
            fn div(self, other: $t) -> Self {
                Self(self.0.div_scalar(other))
            }
        }

        impl DivAssign<$t> for $vecn {
            #[inline(always)]
            fn div_assign(&mut self, other: $t) {
                self.0 = self.0.div_scalar(other)
            }
        }

        impl Div<$vecn> for $t {
            type Output = $vecn;
            #[inline(always)]
            fn div(self, other: $vecn) -> $vecn {
                $vecn($inner::splat(self).div(other.0))
            }
        }

        impl Mul<$vecn> for $vecn {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: $vecn) -> Self {
                Self(self.0.mul(other.0))
            }
        }

        impl MulAssign<$vecn> for $vecn {
            #[inline(always)]
            fn mul_assign(&mut self, other: $vecn) {
                self.0 = self.0.mul(other.0)
            }
        }

        impl Mul<$t> for $vecn {
            type Output = Self;
            #[inline(always)]
            fn mul(self, other: $t) -> Self {
                Self(self.0.mul_scalar(other))
            }
        }

        impl MulAssign<$t> for $vecn {
            #[inline(always)]
            fn mul_assign(&mut self, other: $t) {
                self.0 = self.0.mul_scalar(other)
            }
        }

        impl Mul<$vecn> for $t {
            type Output = $vecn;
            #[inline(always)]
            fn mul(self, other: $vecn) -> $vecn {
                $vecn($inner::splat(self).mul(other.0))
            }
        }

        impl Add for $vecn {
            type Output = Self;
            #[inline(always)]
            fn add(self, other: Self) -> Self {
                Self(self.0.add(other.0))
            }
        }

        impl AddAssign for $vecn {
            #[inline(always)]
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0.add(other.0)
            }
        }

        impl Sub for $vecn {
            type Output = Self;
            #[inline(always)]
            fn sub(self, other: $vecn) -> Self {
                Self(self.0.sub(other.0))
            }
        }

        impl SubAssign for $vecn {
            #[inline(always)]
            fn sub_assign(&mut self, other: $vecn) {
                self.0 = self.0.sub(other.0)
            }
        }

        impl AsRef<[$t; $size]> for $vecn {
            #[inline(always)]
            fn as_ref(&self) -> &[$t; $size] {
                unsafe { &*(self as *const $vecn as *const [$t; $size]) }
            }
        }

        impl From<[$t; $size]> for $vecn {
            #[inline(always)]
            fn from(a: [$t; $size]) -> Self {
                Self($trait::from_array(a))
            }
        }

        impl AsMut<[$t; $size]> for $vecn {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut [$t; $size] {
                unsafe { &mut *(self as *mut $vecn as *mut [$t; $size]) }
            }
        }

        impl From<$vecn> for [$t; $size] {
            #[inline(always)]
            fn from(v: $vecn) -> Self {
                v.into_array()
            }
        }

        impl Index<usize> for $vecn {
            type Output = $t;
            #[inline(always)]
            fn index(&self, index: usize) -> &Self::Output {
                &self.as_ref()[index]
            }
        }

        impl IndexMut<usize> for $vecn {
            #[inline(always)]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.as_mut()[index]
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Sum<&'a Self> for $vecn {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Product<&'a Self> for $vecn {
            #[inline]
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
            }
        }
    };
}

macro_rules! impl_vecn_eq_hash_traits {
    ($t:ty, $size:literal, $vecn:ident) => {
        impl Eq for $vecn {}

        impl core::hash::Hash for $vecn {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let inner: &[$t; $size] = self.as_ref();
                inner.hash(state);
            }
        }
    };
}

// Adds signed vector trait implementations.
// The traits here should be supported for signed types of $t and all sizes of vector.
macro_rules! impl_vecn_signed_traits {
    ($t:ty, $size:literal, $vecn:ident, $inner:ident, $sgntrait:ident) => {
        impl Neg for $vecn {
            type Output = Self;
            #[inline(always)]
            fn neg(self) -> Self {
                Self(self.0.neg())
            }
        }
    };
}

macro_rules! impl_vecn_as_f32 {
    ($ret:ident, $($e:ident),+) => {
        /// Casts all elements of `self` to `f32`.
        #[inline(always)]
        pub fn as_f32(&self) -> $ret {
            $ret::new($(self.$e as f32),+)
        }
    };
}

macro_rules! impl_vecn_as_f64 {
    ($ret:ident, $($e:ident),+) => {
        /// Casts all elements of `self` to `f64`.
        #[inline(always)]
        pub fn as_f64(&self) -> $ret {
            $ret::new($(self.$e as f64),+)
        }
    };
}

macro_rules! impl_vecn_as_i32 {
    ($ret:ident, $($e:ident),+) => {
        /// Casts all elements of `self` to `i32`.
        #[inline(always)]
        pub fn as_i32(&self) -> $ret {
            $ret::new($(self.$e as i32),+)
        }
    };
}

macro_rules! impl_vecn_as_u32 {
    ($ret:ident, $($e:ident),+) => {
        /// Casts all elements of `self` to `u32`.
        #[inline(always)]
        pub fn as_u32(&self) -> $ret {
            $ret::new($(self.$e as u32),+)
        }
    };
}
