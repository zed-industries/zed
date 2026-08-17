// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{sse2::*, BVec3A, Vec2, Vec3, Vec4};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::Float;

union UnionCast {
    a: [f32; 4],
    v: Vec3A,
}

/// Creates a 3-dimensional vector.
#[inline(always)]
pub const fn vec3a(x: f32, y: f32, z: f32) -> Vec3A {
    Vec3A::new(x, y, z)
}

/// A 3-dimensional vector with SIMD support.
///
/// This type is 16 byte aligned. A SIMD vector type is used for storage on supported platforms for
/// better performance than the `Vec3` type.
///
/// It is possible to convert between `Vec3` and `Vec3A` types using `From` trait implementations.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Vec3A(pub(crate) __m128);

impl Vec3A {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All NAN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);

    /// A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);

    /// A unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    /// A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);

    /// A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);

    /// A unit-length vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// Creates a new vector.
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        unsafe { UnionCast { a: [x, y, z, z] }.v }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: f32) -> Self {
        unsafe { UnionCast { a: [v; 4] }.v }
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    pub fn select(mask: BVec3A, if_true: Self, if_false: Self) -> Self {
        Self(unsafe {
            _mm_or_ps(
                _mm_andnot_ps(mask.0, if_false.0),
                _mm_and_ps(if_true.0, mask.0),
            )
        })
    }

    /// Creates a new vector from an array.
    #[inline]
    pub const fn from_array(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// `[x, y, z]`
    #[inline]
    pub const fn to_array(&self) -> [f32; 3] {
        unsafe { *(self as *const Vec3A as *const [f32; 3]) }
    }

    /// Creates a vector from the first 3 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    pub const fn from_slice(slice: &[f32]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }

    /// Writes the elements of `self` to the first 3 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
    }

    /// Internal method for creating a 3D vector from a 4D vector, discarding `w`.
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn from_vec4(v: Vec4) -> Self {
        Self(v.0)
    }

    /// Creates a 4D vector from `self` and the given `w` value.
    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }

    /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
    #[inline]
    pub fn truncate(self) -> Vec2 {
        use crate::swizzles::Vec3Swizzles;
        self.xy()
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        unsafe { dot3(self.0, rhs.0) }
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot_into_vec(self, rhs: Self) -> Self {
        Self(unsafe { dot3_into_m128(self.0, rhs.0) })
    }

    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        unsafe {
            // x  <-  a.y*b.z - a.z*b.y
            // y  <-  a.z*b.x - a.x*b.z
            // z  <-  a.x*b.y - a.y*b.x
            // We can save a shuffle by grouping it in this wacky order:
            // (self.zxy() * rhs - self * rhs.zxy()).zxy()
            let lhszxy = _mm_shuffle_ps(self.0, self.0, 0b01_01_00_10);
            let rhszxy = _mm_shuffle_ps(rhs.0, rhs.0, 0b01_01_00_10);
            let lhszxy_rhs = _mm_mul_ps(lhszxy, rhs.0);
            let rhszxy_lhs = _mm_mul_ps(rhszxy, self.0);
            let sub = _mm_sub_ps(lhszxy_rhs, rhszxy_lhs);
            Self(_mm_shuffle_ps(sub, sub, 0b01_01_00_10))
        }
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self(unsafe { _mm_min_ps(self.0, rhs.0) })
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self(unsafe { _mm_max_ps(self.0, rhs.0) })
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.cmple(max).all(), "clamp: expected min <= max");
        self.max(min).min(max)
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        unsafe {
            let v = self.0;
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b01_01_10_10));
            let v = _mm_min_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        unsafe {
            let v = self.0;
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_10_10));
            let v = _mm_max_ps(v, _mm_shuffle_ps(v, v, 0b00_00_00_01));
            _mm_cvtss_f32(v)
        }
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpeq(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { _mm_cmpeq_ps(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpne(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { _mm_cmpneq_ps(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpge(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { _mm_cmpge_ps(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpgt(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { _mm_cmpgt_ps(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmple(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { _mm_cmple_ps(self.0, rhs.0) })
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmplt(self, rhs: Self) -> BVec3A {
        BVec3A(unsafe { _mm_cmplt_ps(self.0, rhs.0) })
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self(unsafe { crate::sse2::m128_abs(self.0) })
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        unsafe {
            let result = Self(_mm_or_ps(_mm_and_ps(self.0, Self::NEG_ONE.0), Self::ONE.0));
            let mask = self.is_nan_mask();
            Self::select(mask, self, result)
        }
    }

    /// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    /// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn is_negative_bitmask(self) -> u32 {
        unsafe { (_mm_movemask_ps(self.0) as u32) & 0x7 }
    }

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(self) -> bool {
        self.is_nan_mask().any()
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline]
    pub fn is_nan_mask(self) -> BVec3A {
        BVec3A(unsafe { _mm_cmpunord_ps(self.0, self.0) })
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    pub fn length(self) -> f32 {
        unsafe {
            let dot = dot3_in_x(self.0, self.0);
            _mm_cvtss_f32(_mm_sqrt_ps(dot))
        }
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        unsafe {
            let dot = dot3_in_x(self.0, self.0);
            _mm_cvtss_f32(_mm_div_ps(Self::ONE.0, _mm_sqrt_ps(dot)))
        }
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn normalize(self) -> Self {
        unsafe {
            let length = _mm_sqrt_ps(dot3_into_m128(self.0, self.0));
            #[allow(clippy::let_and_return)]
            let normalized = Self(_mm_div_ps(self.0, length));
            glam_assert!(normalized.is_finite());
            normalized
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero`].
    #[must_use]
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
    #[must_use]
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
    #[inline]
    pub fn is_normalized(self) -> bool {
        // TODO: do something with epsilon
        (self.length_squared() - 1.0).abs() <= 1e-4
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto(self, rhs: Self) -> Self {
        let other_len_sq_rcp = rhs.dot(rhs).recip();
        glam_assert!(other_len_sq_rcp.is_finite());
        rhs * self.dot(rhs) * other_len_sq_rcp
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from(self, rhs: Self) -> Self {
        self - self.project_onto(rhs)
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto_normalized(self, rhs: Self) -> Self {
        glam_assert!(rhs.is_normalized());
        rhs * self.dot(rhs)
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from_normalized(self, rhs: Self) -> Self {
        self - self.project_onto_normalized(rhs)
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        Self(unsafe { m128_round(self.0) })
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self(unsafe { m128_floor(self.0) })
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self(unsafe { m128_ceil(self.0) })
    }

    /// Returns a vector containing the fractional part of the vector, e.g. `self -
    /// self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline]
    pub fn exp(self) -> Self {
        Self::new(self.x.exp(), self.y.exp(), self.z.exp())
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        Self::new(self.x.powf(n), self.y.powf(n), self.z.powf(n))
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    pub fn recip(self) -> Self {
        Self(unsafe { _mm_div_ps(Self::ONE.0, self.0) })
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f32) -> bool {
        self.sub(rhs).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        glam_assert!(min <= max);
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
    pub fn clamp_length_max(self, max: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`
    pub fn clamp_length_min(self, min: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else {
            self
        }
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated fma CPU instruction. However, this is not always true,
    /// and will be heavily dependant on designing algorithms with specific target hardware in
    /// mind.
    #[inline]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        #[cfg(target_feature = "fma")]
        unsafe {
            Self(_mm_fmadd_ps(self.0, a.0, b.0))
        }
        #[cfg(not(target_feature = "fma"))]
        Self::new(
            self.x.mul_add(a.x, b.x),
            self.y.mul_add(a.y, b.y),
            self.z.mul_add(a.z, b.z),
        )
    }

    /// Returns the angle (in radians) between two vectors.
    ///
    /// The input vectors do not need to be unit length however they must be non-zero.
    #[inline]
    pub fn angle_between(self, rhs: Self) -> f32 {
        use crate::FloatEx;
        self.dot(rhs)
            .div(self.length_squared().mul(rhs.length_squared()).sqrt())
            .acos_approx()
    }

    /// Returns some vector that is orthogonal to the given one.
    ///
    /// The input vector must be finite and non-zero.
    ///
    /// The output vector is not necessarily unit-length.
    /// For that use [`Self::any_orthonormal_vector`] instead.
    #[inline]
    pub fn any_orthogonal_vector(&self) -> Self {
        // This can probably be optimized
        if self.x.abs() > self.y.abs() {
            Self::new(-self.z, 0.0, self.x) // self.cross(Self::Y)
        } else {
            Self::new(0.0, self.z, -self.y) // self.cross(Self::X)
        }
    }

    /// Returns any unit-length vector that is orthogonal to the given one.
    /// The input vector must be finite and non-zero.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_vector(&self) -> Self {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        #[cfg(feature = "std")]
        let sign = (1.0_f32).copysign(self.z);
        #[cfg(not(feature = "std"))]
        let sign = self.z.signum();
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        Self::new(b, sign + self.y * self.y * a, -self.y)
    }

    /// Given a unit-length vector return two other vectors that together form an orthonormal
    /// basis.  That is, all three vectors are orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn any_orthonormal_pair(&self) -> (Self, Self) {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        #[cfg(feature = "std")]
        let sign = (1.0_f32).copysign(self.z);
        #[cfg(not(feature = "std"))]
        let sign = self.z.signum();
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }

    /// Casts all elements of `self` to `f64`.
    #[inline]
    pub fn as_dvec3(&self) -> crate::DVec3 {
        crate::DVec3::new(self.x as f64, self.y as f64, self.z as f64)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline]
    pub fn as_ivec3(&self) -> crate::IVec3 {
        crate::IVec3::new(self.x as i32, self.y as i32, self.z as i32)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline]
    pub fn as_uvec3(&self) -> crate::UVec3 {
        crate::UVec3::new(self.x as u32, self.y as u32, self.z as u32)
    }
}

impl Default for Vec3A {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Vec3A {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.cmpeq(*rhs).all()
    }
}

impl Div<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(unsafe { _mm_div_ps(self.0, rhs.0) })
    }
}

impl DivAssign<Vec3A> for Vec3A {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 = unsafe { _mm_div_ps(self.0, rhs.0) };
    }
}

impl Div<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(unsafe { _mm_div_ps(self.0, _mm_set1_ps(rhs)) })
    }
}

impl DivAssign<f32> for Vec3A {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 = unsafe { _mm_div_ps(self.0, _mm_set1_ps(rhs)) };
    }
}

impl Div<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn div(self, rhs: Vec3A) -> Vec3A {
        Vec3A(unsafe { _mm_div_ps(_mm_set1_ps(self), rhs.0) })
    }
}

impl Mul<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(unsafe { _mm_mul_ps(self.0, rhs.0) })
    }
}

impl MulAssign<Vec3A> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = unsafe { _mm_mul_ps(self.0, rhs.0) };
    }
}

impl Mul<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self(unsafe { _mm_mul_ps(self.0, _mm_set1_ps(rhs)) })
    }
}

impl MulAssign<f32> for Vec3A {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 = unsafe { _mm_mul_ps(self.0, _mm_set1_ps(rhs)) };
    }
}

impl Mul<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Vec3A {
        Vec3A(unsafe { _mm_mul_ps(_mm_set1_ps(self), rhs.0) })
    }
}

impl Add<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(unsafe { _mm_add_ps(self.0, rhs.0) })
    }
}

impl AddAssign<Vec3A> for Vec3A {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = unsafe { _mm_add_ps(self.0, rhs.0) };
    }
}

impl Add<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self(unsafe { _mm_add_ps(self.0, _mm_set1_ps(rhs)) })
    }
}

impl AddAssign<f32> for Vec3A {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.0 = unsafe { _mm_add_ps(self.0, _mm_set1_ps(rhs)) };
    }
}

impl Add<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn add(self, rhs: Vec3A) -> Vec3A {
        Vec3A(unsafe { _mm_add_ps(_mm_set1_ps(self), rhs.0) })
    }
}

impl Sub<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(unsafe { _mm_sub_ps(self.0, rhs.0) })
    }
}

impl SubAssign<Vec3A> for Vec3A {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3A) {
        self.0 = unsafe { _mm_sub_ps(self.0, rhs.0) };
    }
}

impl Sub<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self(unsafe { _mm_sub_ps(self.0, _mm_set1_ps(rhs)) })
    }
}

impl SubAssign<f32> for Vec3A {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.0 = unsafe { _mm_sub_ps(self.0, _mm_set1_ps(rhs)) };
    }
}

impl Sub<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn sub(self, rhs: Vec3A) -> Vec3A {
        Vec3A(unsafe { _mm_sub_ps(_mm_set1_ps(self), rhs.0) })
    }
}

impl Rem<Vec3A> for Vec3A {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        unsafe {
            let n = m128_floor(_mm_div_ps(self.0, rhs.0));
            Self(_mm_sub_ps(self.0, _mm_mul_ps(n, rhs.0)))
        }
    }
}

impl RemAssign<Vec3A> for Vec3A {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

impl Rem<f32> for Vec3A {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f32) -> Self {
        self.rem(Self::splat(rhs))
    }
}

impl RemAssign<f32> for Vec3A {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        *self = self.rem(Self::splat(rhs));
    }
}

impl Rem<Vec3A> for f32 {
    type Output = Vec3A;
    #[inline]
    fn rem(self, rhs: Vec3A) -> Vec3A {
        Vec3A::splat(self).rem(rhs)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f32; 3]> for Vec3A {
    #[inline]
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { &*(self as *const Vec3A as *const [f32; 3]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f32; 3]> for Vec3A {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 3] {
        unsafe { &mut *(self as *mut Vec3A as *mut [f32; 3]) }
    }
}

impl Sum for Vec3A {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Vec3A {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Vec3A {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Vec3A {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Neg for Vec3A {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(unsafe { _mm_xor_ps(_mm_set1_ps(-0.0), self.0) })
    }
}

impl Index<usize> for Vec3A {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3A {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for Vec3A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for Vec3A {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Vec3A))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl From<Vec3A> for __m128 {
    #[inline]
    fn from(t: Vec3A) -> Self {
        t.0
    }
}

impl From<__m128> for Vec3A {
    #[inline]
    fn from(t: __m128) -> Self {
        Self(t)
    }
}

impl From<[f32; 3]> for Vec3A {
    #[inline]
    fn from(a: [f32; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<Vec3A> for [f32; 3] {
    #[inline]
    fn from(v: Vec3A) -> Self {
        use crate::Align16;
        use core::mem::MaybeUninit;
        let mut out: MaybeUninit<Align16<Self>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), v.0);
            out.assume_init().0
        }
    }
}

impl From<(f32, f32, f32)> for Vec3A {
    #[inline]
    fn from(t: (f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3A> for (f32, f32, f32) {
    #[inline]
    fn from(v: Vec3A) -> Self {
        use crate::Align16;
        use core::mem::MaybeUninit;
        let mut out: MaybeUninit<Align16<Self>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), v.0);
            out.assume_init().0
        }
    }
}

impl From<Vec3> for Vec3A {
    #[inline]
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vec4> for Vec3A {
    /// Creates a `Vec3A` from the `x`, `y` and `z` elements of `self` discarding `w`.
    ///
    /// On architectures where SIMD is supported such as SSE2 on `x86_64` this conversion is a noop.
    #[inline]
    fn from(v: Vec4) -> Self {
        Self(v.0)
    }
}

impl From<Vec3A> for Vec3 {
    #[inline]
    fn from(v: Vec3A) -> Self {
        use crate::Align16;
        use core::mem::MaybeUninit;
        let mut out: MaybeUninit<Align16<Self>> = MaybeUninit::uninit();
        unsafe {
            _mm_store_ps(out.as_mut_ptr().cast(), v.0);
            out.assume_init().0
        }
    }
}

impl From<(Vec2, f32)> for Vec3A {
    #[inline]
    fn from((v, z): (Vec2, f32)) -> Self {
        Self::new(v.x, v.y, z)
    }
}

impl Deref for Vec3A {
    type Target = crate::deref::Vec3<f32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl DerefMut for Vec3A {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}
