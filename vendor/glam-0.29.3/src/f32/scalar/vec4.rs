// Generated from vec.rs.tera template. Edit the template, not the generated file.

#[cfg(feature = "scalar-math")]
use crate::BVec4 as BVec4A;

#[cfg(not(feature = "scalar-math"))]
use crate::BVec4A;
use crate::{f32::math, BVec4, Vec2, Vec3, Vec3A};

use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

/// Creates a 4-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}

/// A 4-dimensional vector.
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        not(any(feature = "scalar-math", target_arch = "spirv")),
        feature = "cuda"
    ),
    repr(align(16))
)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All `f32::MIN`.
    pub const MIN: Self = Self::splat(f32::MIN);

    /// All `f32::MAX`.
    pub const MAX: Self = Self::splat(f32::MAX);

    /// All `f32::NAN`.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// All `f32::INFINITY`.
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    /// All `f32::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);

    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);

    /// A unit vector pointing along the positive W axis.
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);

    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);

    /// A unit vector pointing along the negative W axis.
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    /// The unit axes.
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: f32) -> Self {
        Self {
            x: v,

            y: v,

            z: v,

            w: v,
        }
    }

    /// Returns a vector containing each element of `self` modified by a mapping function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self::new(f(self.x), f(self.y), f(self.z), f(self.w))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    #[must_use]
    pub fn select(mask: BVec4A, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.test(0) { if_true.x } else { if_false.x },
            y: if mask.test(1) { if_true.y } else { if_false.y },
            z: if mask.test(2) { if_true.z } else { if_false.z },
            w: if mask.test(3) { if_true.w } else { if_false.w },
        }
    }

    /// Creates a new vector from an array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// `[x, y, z, w]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Creates a vector from the first 4 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[f32]) -> Self {
        assert!(slice.len() >= 4);
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }

    /// Writes the elements of `self` to the first 4 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        slice[..4].copy_from_slice(&self.to_array());
    }

    /// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    /// Truncation to [`Vec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].
    ///
    /// To truncate to [`Vec3A`] use [`Vec3A::from()`].
    #[inline]
    #[must_use]
    pub fn truncate(self) -> Vec3 {
        use crate::swizzles::Vec4Swizzles;
        self.xyz()
    }

    /// Creates a 4D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    /// Creates a 4D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    /// Creates a 4D vector from `self` with the given value of `z`.
    #[inline]
    #[must_use]
    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    /// Creates a 4D vector from `self` with the given value of `w`.
    #[inline]
    #[must_use]
    pub fn with_w(mut self, w: f32) -> Self {
        self.w = w;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z) + (self.w * rhs.w)
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot_into_vec(self, rhs: Self) -> Self {
        Self::splat(self.dot(rhs))
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
            w: self.w.min(rhs.w),
        }
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
            w: self.w.max(rhs.w),
        }
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.cmple(max).all(), "clamp: expected min <= max");
        self.max(min).min(max)
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn min_element(self) -> f32 {
        self.x.min(self.y.min(self.z.min(self.w)))
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> f32 {
        self.x.max(self.y.max(self.z.max(self.w)))
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> f32 {
        self.x + self.y + self.z + self.w
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> f32 {
        self.x * self.y * self.z * self.w
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpeq(self, rhs: Self) -> BVec4A {
        BVec4A::new(
            self.x.eq(&rhs.x),
            self.y.eq(&rhs.y),
            self.z.eq(&rhs.z),
            self.w.eq(&rhs.w),
        )
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpne(self, rhs: Self) -> BVec4A {
        BVec4A::new(
            self.x.ne(&rhs.x),
            self.y.ne(&rhs.y),
            self.z.ne(&rhs.z),
            self.w.ne(&rhs.w),
        )
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpge(self, rhs: Self) -> BVec4A {
        BVec4A::new(
            self.x.ge(&rhs.x),
            self.y.ge(&rhs.y),
            self.z.ge(&rhs.z),
            self.w.ge(&rhs.w),
        )
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpgt(self, rhs: Self) -> BVec4A {
        BVec4A::new(
            self.x.gt(&rhs.x),
            self.y.gt(&rhs.y),
            self.z.gt(&rhs.z),
            self.w.gt(&rhs.w),
        )
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmple(self, rhs: Self) -> BVec4A {
        BVec4A::new(
            self.x.le(&rhs.x),
            self.y.le(&rhs.y),
            self.z.le(&rhs.z),
            self.w.le(&rhs.w),
        )
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmplt(self, rhs: Self) -> BVec4A {
        BVec4A::new(
            self.x.lt(&rhs.x),
            self.y.lt(&rhs.y),
            self.z.lt(&rhs.z),
            self.w.lt(&rhs.w),
        )
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            x: math::abs(self.x),
            y: math::abs(self.y),
            z: math::abs(self.z),
            w: math::abs(self.w),
        }
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        Self {
            x: math::signum(self.x),
            y: math::signum(self.y),
            z: math::signum(self.z),
            w: math::signum(self.w),
        }
    }

    /// Returns a vector with signs of `rhs` and the magnitudes of `self`.
    #[inline]
    #[must_use]
    pub fn copysign(self, rhs: Self) -> Self {
        Self {
            x: math::copysign(self.x, rhs.x),
            y: math::copysign(self.y, rhs.y),
            z: math::copysign(self.z, rhs.z),
            w: math::copysign(self.w, rhs.w),
        }
    }

    /// Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
    ///
    /// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn is_negative_bitmask(self) -> u32 {
        (self.x.is_sign_negative() as u32)
            | ((self.y.is_sign_negative() as u32) << 1)
            | ((self.z.is_sign_negative() as u32) << 2)
            | ((self.w.is_sign_negative() as u32) << 3)
    }

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Performs `is_finite` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.
    pub fn is_finite_mask(self) -> BVec4A {
        BVec4A::new(
            self.x.is_finite(),
            self.y.is_finite(),
            self.z.is_finite(),
            self.w.is_finite(),
        )
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.
    #[inline]
    #[must_use]
    pub fn is_nan_mask(self) -> BVec4A {
        BVec4A::new(
            self.x.is_nan(),
            self.y.is_nan(),
            self.z.is_nan(),
            self.w.is_nan(),
        )
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        math::sqrt(self.dot(self))
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    #[must_use]
    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }

    /// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
    #[inline]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        Self::new(
            math::div_euclid(self.x, rhs.x),
            math::div_euclid(self.y, rhs.y),
            math::div_euclid(self.z, rhs.z),
            math::div_euclid(self.w, rhs.w),
        )
    }

    /// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
    ///
    /// [Euclidean division]: f32::rem_euclid
    #[inline]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        Self::new(
            math::rem_euclid(self.x, rhs.x),
            math::rem_euclid(self.y, rhs.y),
            math::rem_euclid(self.z, rhs.z),
            math::rem_euclid(self.w, rhs.w),
        )
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    /// Panics
    ///
    /// Will panic if the resulting normalized vector is not finite when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        #[allow(clippy::let_and_return)]
        let normalized = self.mul(self.length_recip());
        glam_assert!(normalized.is_finite());
        normalized
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero()`].
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            Some(self * rcp)
        } else {
            None
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns a
    /// fallback value.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be the fallback value.
    ///
    /// See also [`Self::try_normalize()`].
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        let rcp = self.length_recip();
        if rcp.is_finite() && rcp > 0.0 {
            self * rcp
        } else {
            fallback
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be zero.
    ///
    /// See also [`Self::try_normalize()`].
    #[inline]
    #[must_use]
    pub fn normalize_or_zero(self) -> Self {
        self.normalize_or(Self::ZERO)
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of approximately `1e-4`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        math::abs(self.length_squared() - 1.0) <= 2e-4
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[inline]
    #[must_use]
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
    #[doc(alias("plane"))]
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
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
    #[doc(alias("plane"))]
    #[inline]
    #[must_use]
    pub fn reject_from_normalized(self, rhs: Self) -> Self {
        self - self.project_onto_normalized(rhs)
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        Self {
            x: math::round(self.x),
            y: math::round(self.y),
            z: math::round(self.z),
            w: math::round(self.w),
        }
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        Self {
            x: math::floor(self.x),
            y: math::floor(self.y),
            z: math::floor(self.z),
            w: math::floor(self.w),
        }
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        Self {
            x: math::ceil(self.x),
            y: math::ceil(self.y),
            z: math::ceil(self.z),
            w: math::ceil(self.w),
        }
    }

    /// Returns a vector containing the integer part each element of `self`. This means numbers are
    /// always truncated towards zero.
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        Self {
            x: math::trunc(self.x),
            y: math::trunc(self.y),
            z: math::trunc(self.z),
            w: math::trunc(self.w),
        }
    }

    /// Returns a vector containing the fractional part of the vector as `self - self.trunc()`.
    ///
    /// Note that this differs from the GLSL implementation of `fract` which returns
    /// `self - self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    #[must_use]
    pub fn fract(self) -> Self {
        self - self.trunc()
    }

    /// Returns a vector containing the fractional part of the vector as `self - self.floor()`.
    ///
    /// Note that this differs from the Rust implementation of `fract` which returns
    /// `self - self.trunc()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    #[must_use]
    pub fn fract_gl(self) -> Self {
        self - self.floor()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        Self::new(
            math::exp(self.x),
            math::exp(self.y),
            math::exp(self.z),
            math::exp(self.w),
        )
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline]
    #[must_use]
    pub fn powf(self, n: f32) -> Self {
        Self::new(
            math::powf(self.x, n),
            math::powf(self.y, n),
            math::powf(self.z, n),
            math::powf(self.w, n),
        )
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    #[must_use]
    pub fn recip(self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
            w: 1.0 / self.w,
        }
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self * (1.0 - s) + rhs * s
    }

    /// Moves towards `rhs` based on the value `d`.
    ///
    /// When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to
    /// `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.
    #[inline]
    #[must_use]
    pub fn move_towards(&self, rhs: Self, d: f32) -> Self {
        let a = rhs - *self;
        let len = a.length();
        if len <= d || len <= 1e-4 {
            return rhs;
        }
        *self + a / len * d
    }

    /// Calculates the midpoint between `self` and `rhs`.
    ///
    /// The midpoint is the average of, or halfway point between, two vectors.
    /// `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)`
    /// while being slightly cheaper to compute.
    #[inline]
    pub fn midpoint(self, rhs: Self) -> Self {
        (self + rhs) * 0.5
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
    #[must_use]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f32) -> bool {
        self.sub(rhs).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    /// Returns a vector with a length no less than `min` and no more than `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        glam_assert!(0.0 <= min);
        glam_assert!(min <= max);
        let length_sq = self.length_squared();
        if length_sq < min * min {
            min * (self / math::sqrt(length_sq))
        } else if length_sq > max * max {
            max * (self / math::sqrt(length_sq))
        } else {
            self
        }
    }

    /// Returns a vector with a length no more than `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `max` is negative when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length_max(self, max: f32) -> Self {
        glam_assert!(0.0 <= max);
        let length_sq = self.length_squared();
        if length_sq > max * max {
            max * (self / math::sqrt(length_sq))
        } else {
            self
        }
    }

    /// Returns a vector with a length no less than `min`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is negative when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length_min(self, min: f32) -> Self {
        glam_assert!(0.0 <= min);
        let length_sq = self.length_squared();
        if length_sq < min * min {
            min * (self / math::sqrt(length_sq))
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
    #[must_use]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        Self::new(
            math::mul_add(self.x, a.x, b.x),
            math::mul_add(self.y, a.y, b.y),
            math::mul_add(self.z, a.z, b.z),
            math::mul_add(self.w, a.w, b.w),
        )
    }

    /// Returns the reflection vector for a given incident vector `self` and surface normal
    /// `normal`.
    ///
    /// `normal` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `normal` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        glam_assert!(normal.is_normalized());
        self - 2.0 * self.dot(normal) * normal
    }

    /// Returns the refraction direction for a given incident vector `self`, surface normal
    /// `normal` and ratio of indices of refraction, `eta`. When total internal reflection occurs,
    /// a zero vector will be returned.
    ///
    /// `self` and `normal` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `normal` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn refract(self, normal: Self, eta: f32) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(normal.is_normalized());
        let n_dot_i = normal.dot(self);
        let k = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);
        if k >= 0.0 {
            eta * self - (eta * n_dot_i + math::sqrt(k)) * normal
        } else {
            Self::ZERO
        }
    }

    /// Casts all elements of `self` to `f64`.
    #[inline]
    #[must_use]
    pub fn as_dvec4(&self) -> crate::DVec4 {
        crate::DVec4::new(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
    }

    /// Casts all elements of `self` to `i8`.
    #[inline]
    #[must_use]
    pub fn as_i8vec4(&self) -> crate::I8Vec4 {
        crate::I8Vec4::new(self.x as i8, self.y as i8, self.z as i8, self.w as i8)
    }

    /// Casts all elements of `self` to `u8`.
    #[inline]
    #[must_use]
    pub fn as_u8vec4(&self) -> crate::U8Vec4 {
        crate::U8Vec4::new(self.x as u8, self.y as u8, self.z as u8, self.w as u8)
    }

    /// Casts all elements of `self` to `i16`.
    #[inline]
    #[must_use]
    pub fn as_i16vec4(&self) -> crate::I16Vec4 {
        crate::I16Vec4::new(self.x as i16, self.y as i16, self.z as i16, self.w as i16)
    }

    /// Casts all elements of `self` to `u16`.
    #[inline]
    #[must_use]
    pub fn as_u16vec4(&self) -> crate::U16Vec4 {
        crate::U16Vec4::new(self.x as u16, self.y as u16, self.z as u16, self.w as u16)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline]
    #[must_use]
    pub fn as_ivec4(&self) -> crate::IVec4 {
        crate::IVec4::new(self.x as i32, self.y as i32, self.z as i32, self.w as i32)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline]
    #[must_use]
    pub fn as_uvec4(&self) -> crate::UVec4 {
        crate::UVec4::new(self.x as u32, self.y as u32, self.z as u32, self.w as u32)
    }

    /// Casts all elements of `self` to `i64`.
    #[inline]
    #[must_use]
    pub fn as_i64vec4(&self) -> crate::I64Vec4 {
        crate::I64Vec4::new(self.x as i64, self.y as i64, self.z as i64, self.w as i64)
    }

    /// Casts all elements of `self` to `u64`.
    #[inline]
    #[must_use]
    pub fn as_u64vec4(&self) -> crate::U64Vec4 {
        crate::U64Vec4::new(self.x as u64, self.y as u64, self.z as u64, self.w as u64)
    }
}

impl Default for Vec4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
            w: self.w.div(rhs.w),
        }
    }
}

impl Div<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &Vec4) -> Vec4 {
        self.div(*rhs)
    }
}

impl Div<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &Vec4) -> Vec4 {
        (*self).div(*rhs)
    }
}

impl Div<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: Vec4) -> Vec4 {
        (*self).div(rhs)
    }
}

impl DivAssign<Vec4> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
        self.w.div_assign(rhs.w);
    }
}

impl DivAssign<&Self> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs)
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
            w: self.w.div(rhs),
        }
    }
}

impl Div<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &f32) -> Vec4 {
        self.div(*rhs)
    }
}

impl Div<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &f32) -> Vec4 {
        (*self).div(*rhs)
    }
}

impl Div<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: f32) -> Vec4 {
        (*self).div(rhs)
    }
}

impl DivAssign<f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
        self.w.div_assign(rhs);
    }
}

impl DivAssign<&f32> for Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs)
    }
}

impl Div<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
            w: self.div(rhs.w),
        }
    }
}

impl Div<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &Vec4) -> Vec4 {
        self.div(*rhs)
    }
}

impl Div<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: &Vec4) -> Vec4 {
        (*self).div(*rhs)
    }
}

impl Div<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn div(self, rhs: Vec4) -> Vec4 {
        (*self).div(rhs)
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
            w: self.w.mul(rhs.w),
        }
    }
}

impl Mul<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Vec4 {
        self.mul(*rhs)
    }
}

impl Mul<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Vec4 {
        (*self).mul(*rhs)
    }
}

impl Mul<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        (*self).mul(rhs)
    }
}

impl MulAssign<Vec4> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
        self.w.mul_assign(rhs.w);
    }
}

impl MulAssign<&Self> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
            w: self.w.mul(rhs),
        }
    }
}

impl Mul<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &f32) -> Vec4 {
        self.mul(*rhs)
    }
}

impl Mul<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &f32) -> Vec4 {
        (*self).mul(*rhs)
    }
}

impl Mul<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: f32) -> Vec4 {
        (*self).mul(rhs)
    }
}

impl MulAssign<f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
        self.w.mul_assign(rhs);
    }
}

impl MulAssign<&f32> for Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
            w: self.mul(rhs.w),
        }
    }
}

impl Mul<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Vec4 {
        self.mul(*rhs)
    }
}

impl Mul<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: &Vec4) -> Vec4 {
        (*self).mul(*rhs)
    }
}

impl Mul<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn mul(self, rhs: Vec4) -> Vec4 {
        (*self).mul(rhs)
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
            w: self.w.add(rhs.w),
        }
    }
}

impl Add<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &Vec4) -> Vec4 {
        self.add(*rhs)
    }
}

impl Add<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &Vec4) -> Vec4 {
        (*self).add(*rhs)
    }
}

impl Add<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Vec4 {
        (*self).add(rhs)
    }
}

impl AddAssign<Vec4> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
        self.w.add_assign(rhs.w);
    }
}

impl AddAssign<&Self> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs)
    }
}

impl Add<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
            w: self.w.add(rhs),
        }
    }
}

impl Add<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &f32) -> Vec4 {
        self.add(*rhs)
    }
}

impl Add<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &f32) -> Vec4 {
        (*self).add(*rhs)
    }
}

impl Add<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: f32) -> Vec4 {
        (*self).add(rhs)
    }
}

impl AddAssign<f32> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
        self.w.add_assign(rhs);
    }
}

impl AddAssign<&f32> for Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: &f32) {
        self.add_assign(*rhs)
    }
}

impl Add<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
            w: self.add(rhs.w),
        }
    }
}

impl Add<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &Vec4) -> Vec4 {
        self.add(*rhs)
    }
}

impl Add<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: &Vec4) -> Vec4 {
        (*self).add(*rhs)
    }
}

impl Add<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn add(self, rhs: Vec4) -> Vec4 {
        (*self).add(rhs)
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
            w: self.w.sub(rhs.w),
        }
    }
}

impl Sub<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &Vec4) -> Vec4 {
        self.sub(*rhs)
    }
}

impl Sub<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &Vec4) -> Vec4 {
        (*self).sub(*rhs)
    }
}

impl Sub<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Vec4 {
        (*self).sub(rhs)
    }
}

impl SubAssign<Vec4> for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec4) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
        self.w.sub_assign(rhs.w);
    }
}

impl SubAssign<&Self> for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs)
    }
}

impl Sub<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
            w: self.w.sub(rhs),
        }
    }
}

impl Sub<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &f32) -> Vec4 {
        self.sub(*rhs)
    }
}

impl Sub<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &f32) -> Vec4 {
        (*self).sub(*rhs)
    }
}

impl Sub<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: f32) -> Vec4 {
        (*self).sub(rhs)
    }
}

impl SubAssign<f32> for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
        self.w.sub_assign(rhs);
    }
}

impl SubAssign<&f32> for Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: &f32) {
        self.sub_assign(*rhs)
    }
}

impl Sub<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
            w: self.sub(rhs.w),
        }
    }
}

impl Sub<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &Vec4) -> Vec4 {
        self.sub(*rhs)
    }
}

impl Sub<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: &Vec4) -> Vec4 {
        (*self).sub(*rhs)
    }
}

impl Sub<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn sub(self, rhs: Vec4) -> Vec4 {
        (*self).sub(rhs)
    }
}

impl Rem<Vec4> for Vec4 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
            z: self.z.rem(rhs.z),
            w: self.w.rem(rhs.w),
        }
    }
}

impl Rem<&Vec4> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &Vec4) -> Vec4 {
        self.rem(*rhs)
    }
}

impl Rem<&Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &Vec4) -> Vec4 {
        (*self).rem(*rhs)
    }
}

impl Rem<Vec4> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: Vec4) -> Vec4 {
        (*self).rem(rhs)
    }
}

impl RemAssign<Vec4> for Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
        self.z.rem_assign(rhs.z);
        self.w.rem_assign(rhs.w);
    }
}

impl RemAssign<&Self> for Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs)
    }
}

impl Rem<f32> for Vec4 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f32) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
            z: self.z.rem(rhs),
            w: self.w.rem(rhs),
        }
    }
}

impl Rem<&f32> for Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &f32) -> Vec4 {
        self.rem(*rhs)
    }
}

impl Rem<&f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &f32) -> Vec4 {
        (*self).rem(*rhs)
    }
}

impl Rem<f32> for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: f32) -> Vec4 {
        (*self).rem(rhs)
    }
}

impl RemAssign<f32> for Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
        self.z.rem_assign(rhs);
        self.w.rem_assign(rhs);
    }
}

impl RemAssign<&f32> for Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: &f32) {
        self.rem_assign(*rhs)
    }
}

impl Rem<Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
            z: self.rem(rhs.z),
            w: self.rem(rhs.w),
        }
    }
}

impl Rem<&Vec4> for f32 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &Vec4) -> Vec4 {
        self.rem(*rhs)
    }
}

impl Rem<&Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: &Vec4) -> Vec4 {
        (*self).rem(*rhs)
    }
}

impl Rem<Vec4> for &f32 {
    type Output = Vec4;
    #[inline]
    fn rem(self, rhs: Vec4) -> Vec4 {
        (*self).rem(rhs)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Vec4 as *const [f32; 4]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Vec4 as *mut [f32; 4]) }
    }
}

impl Sum for Vec4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Vec4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Vec4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Vec4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Neg for Vec4 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
            w: self.w.neg(),
        }
    }
}

impl Neg for &Vec4 {
    type Output = Vec4;
    #[inline]
    fn neg(self) -> Vec4 {
        (*self).neg()
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(
                f,
                "[{:.*}, {:.*}, {:.*}, {:.*}]",
                p, self.x, p, self.y, p, self.z, p, self.w
            )
        } else {
            write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
        }
    }
}

impl fmt::Debug for Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Vec4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(v: Vec4) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Vec4) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}

impl From<(Vec3A, f32)> for Vec4 {
    #[inline]
    fn from((v, w): (Vec3A, f32)) -> Self {
        v.extend(w)
    }
}

impl From<(f32, Vec3A)> for Vec4 {
    #[inline]
    fn from((x, v): (f32, Vec3A)) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}

impl From<(Vec3, f32)> for Vec4 {
    #[inline]
    fn from((v, w): (Vec3, f32)) -> Self {
        Self::new(v.x, v.y, v.z, w)
    }
}

impl From<(f32, Vec3)> for Vec4 {
    #[inline]
    fn from((x, v): (f32, Vec3)) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}

impl From<(Vec2, f32, f32)> for Vec4 {
    #[inline]
    fn from((v, z, w): (Vec2, f32, f32)) -> Self {
        Self::new(v.x, v.y, z, w)
    }
}

impl From<(Vec2, Vec2)> for Vec4 {
    #[inline]
    fn from((v, u): (Vec2, Vec2)) -> Self {
        Self::new(v.x, v.y, u.x, u.y)
    }
}

impl From<BVec4> for Vec4 {
    #[inline]
    fn from(v: BVec4) -> Self {
        Self::new(
            f32::from(v.x),
            f32::from(v.y),
            f32::from(v.z),
            f32::from(v.w),
        )
    }
}

#[cfg(not(feature = "scalar-math"))]
impl From<BVec4A> for Vec4 {
    #[inline]
    fn from(v: BVec4A) -> Self {
        let bool_array: [bool; 4] = v.into();
        Self::new(
            f32::from(bool_array[0]),
            f32::from(bool_array[1]),
            f32::from(bool_array[2]),
            f32::from(bool_array[3]),
        )
    }
}
