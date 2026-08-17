// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{f64::math, BVec3, BVec3A, DQuat, DVec2, DVec4, FloatExt, IVec3, UVec3, Vec3};

use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(feature = "zerocopy")]
use zerocopy_derive::*;

/// Creates a 3-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn dvec3(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(x, y, z)
}

/// A 3-dimensional vector.
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(
    feature = "zerocopy",
    derive(FromBytes, Immutable, IntoBytes, KnownLayout)
)]
#[repr(C)]
#[cfg_attr(target_arch = "spirv", rust_gpu::vector::v1)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DVec3 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All `f64::MIN`.
    pub const MIN: Self = Self::splat(f64::MIN);

    /// All `f64::MAX`.
    pub const MAX: Self = Self::splat(f64::MAX);

    /// All `f64::NAN`.
    pub const NAN: Self = Self::splat(f64::NAN);

    /// All `f64::INFINITY`.
    pub const INFINITY: Self = Self::splat(f64::INFINITY);

    /// All `f64::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::splat(f64::NEG_INFINITY);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);

    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);

    /// A unit vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    /// The unit axes.
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    /// DVec3 uses Rust Portable SIMD
    pub const USES_CORE_SIMD: bool = false;
    /// DVec3 uses Arm NEON
    pub const USES_NEON: bool = false;
    /// DVec3 uses scalar math
    pub const USES_SCALAR_MATH: bool = true;
    /// DVec3 uses Intel SSE2
    pub const USES_SSE2: bool = false;
    /// DVec3 uses WebAssembly 128-bit SIMD
    pub const USES_WASM_SIMD: bool = false;
    #[deprecated(since = "0.31.0", note = "Renamed to USES_WASM_SIMD")]
    pub const USES_WASM32_SIMD: bool = false;

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Returns a vector containing each element of `self` modified by a mapping function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self::new(f(self.x), f(self.y), f(self.z))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    #[must_use]
    pub fn select(mask: BVec3, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.test(0) { if_true.x } else { if_false.x },
            y: if mask.test(1) { if_true.y } else { if_false.y },
            z: if mask.test(2) { if_true.z } else { if_false.z },
        }
    }

    /// Creates a new vector from an array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [f64; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// Converts `self` to `[x, y, z]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// Creates a vector from the first 3 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[f64]) -> Self {
        assert!(slice.len() >= 3);
        Self::new(slice[0], slice[1], slice[2])
    }

    /// Writes the elements of `self` to the first 3 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 3 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f64]) {
        slice[..3].copy_from_slice(&self.to_array());
    }

    /// Internal method for creating a 3D vector from a 4D vector, discarding `w`.
    #[allow(dead_code)]
    #[inline]
    #[must_use]
    pub(crate) fn from_vec4(v: DVec4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    /// Creates a 4D vector from `self` and the given `w` value.
    #[inline]
    #[must_use]
    pub fn extend(self, w: f64) -> DVec4 {
        DVec4::new(self.x, self.y, self.z, w)
    }

    /// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
    ///
    /// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].
    #[inline]
    #[must_use]
    pub fn truncate(self) -> DVec2 {
        use crate::swizzles::Vec3Swizzles;
        self.xy()
    }

    /// Projects a homogeneous coordinate to 3D space by performing perspective divide.
    ///
    /// # Panics
    ///
    /// Will panic if `v.w` is `0` when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_homogeneous(v: DVec4) -> Self {
        glam_assert!(v.w != 0.0);
        Self::from_vec4(v) / v.w
    }

    /// Creates a homogeneous coordinate from `self`, equivalent to `self.extend(1.0)`.
    #[inline]
    #[must_use]
    pub fn to_homogeneous(self) -> DVec4 {
        self.extend(1.0)
    }

    /// Creates a 3D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }

    /// Creates a 3D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }

    /// Creates a 3D vector from `self` with the given value of `z`.
    #[inline]
    #[must_use]
    pub fn with_z(mut self, z: f64) -> Self {
        self.z = z;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot_into_vec(self, rhs: Self) -> Self {
        Self::splat(self.dot(rhs))
    }

    /// Computes the cross product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on
    /// different SIMD architectures.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: if self.x < rhs.x { self.x } else { rhs.x },
            y: if self.y < rhs.y { self.y } else { rhs.y },
            z: if self.z < rhs.z { self.z } else { rhs.z },
        }
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on
    /// different SIMD architectures.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: if self.x > rhs.x { self.x } else { rhs.x },
            y: if self.y > rhs.y { self.y } else { rhs.y },
            z: if self.z > rhs.z { self.z } else { rhs.z },
        }
    }

    /// Component-wise clamping of values, similar to [`f64::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics and may differ on
    /// different SIMD architectures.
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
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics and may differ on
    /// different SIMD architectures.
    #[inline]
    #[must_use]
    pub fn min_element(self) -> f64 {
        let min = |a, b| if a < b { a } else { b };
        min(self.x, min(self.y, self.z))
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    ///
    /// NaN propogation does not follow IEEE 754-2008 semantics and may differ on
    /// different SIMD architectures.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> f64 {
        let max = |a, b| if a > b { a } else { b };
        max(self.x, max(self.y, self.z))
    }

    /// Returns the index of the first minimum element of `self`.
    #[doc(alias = "argmin")]
    #[inline]
    #[must_use]
    pub fn min_position(self) -> usize {
        let mut min = self.x;
        let mut index = 0;
        if self.y < min {
            min = self.y;
            index = 1;
        }
        if self.z < min {
            index = 2;
        }
        index
    }

    /// Returns the index of the first maximum element of `self`.
    #[doc(alias = "argmax")]
    #[inline]
    #[must_use]
    pub fn max_position(self) -> usize {
        let mut max = self.x;
        let mut index = 0;
        if self.y > max {
            max = self.y;
            index = 1;
        }
        if self.z > max {
            index = 2;
        }
        index
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> f64 {
        self.x + self.y + self.z
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> f64 {
        self.x * self.y * self.z
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpeq(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.eq(&rhs.x), self.y.eq(&rhs.y), self.z.eq(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpne(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.ne(&rhs.x), self.y.ne(&rhs.y), self.z.ne(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpge(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.ge(&rhs.x), self.y.ge(&rhs.y), self.z.ge(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpgt(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.gt(&rhs.x), self.y.gt(&rhs.y), self.z.gt(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmple(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.le(&rhs.x), self.y.le(&rhs.y), self.z.le(&rhs.z))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmplt(self, rhs: Self) -> BVec3 {
        BVec3::new(self.x.lt(&rhs.x), self.y.lt(&rhs.y), self.z.lt(&rhs.z))
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            x: math::abs(self.x),
            y: math::abs(self.y),
            z: math::abs(self.z),
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
        }
    }

    /// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
    ///
    /// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    ///
    /// An element is negative if it has a negative sign, including -0.0, NaNs with negative sign
    /// bit and negative infinity.
    #[inline]
    #[must_use]
    pub fn is_negative_bitmask(self) -> u32 {
        (self.x.is_sign_negative() as u32)
            | ((self.y.is_sign_negative() as u32) << 1)
            | ((self.z.is_sign_negative() as u32) << 2)
    }

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Performs `is_finite` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.
    #[inline]
    #[must_use]
    pub fn is_finite_mask(self) -> BVec3 {
        BVec3::new(self.x.is_finite(), self.y.is_finite(), self.z.is_finite())
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.
    #[inline]
    #[must_use]
    pub fn is_nan_mask(self) -> BVec3 {
        BVec3::new(self.x.is_nan(), self.y.is_nan(), self.z.is_nan())
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> f64 {
        math::sqrt(self.dot(self))
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    #[must_use]
    pub fn length_recip(self) -> f64 {
        self.length().recip()
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance(self, rhs: Self) -> f64 {
        (self - rhs).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, rhs: Self) -> f64 {
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
        )
    }

    /// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
    ///
    /// [Euclidean division]: f64::rem_euclid
    #[inline]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        Self::new(
            math::rem_euclid(self.x, rhs.x),
            math::rem_euclid(self.y, rhs.y),
            math::rem_euclid(self.z, rhs.z),
        )
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
    ///
    /// # Panics
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

    /// Returns `self` normalized to length 1.0 and the length of `self`.
    ///
    /// If `self` is zero length then `(Self::X, 0.0)` is returned.
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, f64) {
        let length = self.length();
        let rcp = 1.0 / length;
        if rcp.is_finite() && rcp > 0.0 {
            (self * rcp, length)
        } else {
            (Self::X, 0.0)
        }
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
        }
    }

    /// Returns a vector containing `0.0` if `rhs < self` and 1.0 otherwise.
    ///
    /// Similar to glsl's step(edge, x), which translates into edge.step(x)
    #[inline]
    #[must_use]
    pub fn step(self, rhs: Self) -> Self {
        Self::select(rhs.cmplt(self), Self::ZERO, Self::ONE)
    }

    /// Returns a vector containing all elements of `self` clamped to the range of `[0, 1]`.
    #[inline]
    #[must_use]
    pub fn saturate(self) -> Self {
        self.clamp(Self::ZERO, Self::ONE)
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
        Self::new(math::exp(self.x), math::exp(self.y), math::exp(self.z))
    }

    /// Returns a vector containing `2^self` for each element of `self`.
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        Self::new(math::exp2(self.x), math::exp2(self.y), math::exp2(self.z))
    }

    /// Returns a vector containing the natural logarithm for each element of `self`.
    /// This returns NaN when the element is negative and negative infinity when the element is zero.
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        Self::new(math::ln(self.x), math::ln(self.y), math::ln(self.z))
    }

    /// Returns a vector containing the base 2 logarithm for each element of `self`.
    /// This returns NaN when the element is negative and negative infinity when the element is zero.
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        Self::new(math::log2(self.x), math::log2(self.y), math::log2(self.z))
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline]
    #[must_use]
    pub fn powf(self, n: f64) -> Self {
        Self::new(
            math::powf(self.x, n),
            math::powf(self.y, n),
            math::powf(self.z, n),
        )
    }

    /// Returns a vector containing the square root for each element of `self`.
    /// This returns NaN when the element is negative.
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        Self::new(math::sqrt(self.x), math::sqrt(self.y), math::sqrt(self.z))
    }

    /// Returns a vector containing the cosine for each element of `self`.
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        Self::new(math::cos(self.x), math::cos(self.y), math::cos(self.z))
    }

    /// Returns a vector containing the sine for each element of `self`.
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        Self::new(math::sin(self.x), math::sin(self.y), math::sin(self.z))
    }

    /// Returns a tuple of two vectors containing the sine and cosine for each element of `self`.
    #[inline]
    #[must_use]
    pub fn sin_cos(self) -> (Self, Self) {
        let (sin_x, cos_x) = math::sin_cos(self.x);
        let (sin_y, cos_y) = math::sin_cos(self.y);
        let (sin_z, cos_z) = math::sin_cos(self.z);

        (
            Self::new(sin_x, sin_y, sin_z),
            Self::new(cos_x, cos_y, cos_z),
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
    pub fn lerp(self, rhs: Self, s: f64) -> Self {
        self * (1.0 - s) + rhs * s
    }

    /// Moves towards `rhs` based on the value `d`.
    ///
    /// When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to
    /// `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.
    #[inline]
    #[must_use]
    pub fn move_towards(self, rhs: Self, d: f64) -> Self {
        let a = rhs - self;
        let len = a.length();
        if len <= d || len <= 1e-4 {
            return rhs;
        }
        self + a / len * d
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
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f64) -> bool {
        self.sub(rhs).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    /// Returns a vector with a length no less than `min` and no more than `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp_length(self, min: f64, max: f64) -> Self {
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
    pub fn clamp_length_max(self, max: f64) -> Self {
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
    pub fn clamp_length_min(self, min: f64) -> Self {
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
    pub fn refract(self, normal: Self, eta: f64) -> Self {
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

    /// Returns the angle (in radians) between two vectors in the range `[0, +π]`.
    ///
    /// The inputs do not need to be unit vectors however they must be non-zero.
    #[inline]
    #[must_use]
    pub fn angle_between(self, rhs: Self) -> f64 {
        math::acos_approx(
            self.dot(rhs)
                .div(math::sqrt(self.length_squared().mul(rhs.length_squared()))),
        )
    }

    /// Rotates around the x axis by `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn rotate_x(self, angle: f64) -> Self {
        let (sina, cosa) = math::sin_cos(angle);
        Self::new(
            self.x,
            self.y * cosa - self.z * sina,
            self.y * sina + self.z * cosa,
        )
    }

    /// Rotates around the y axis by `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn rotate_y(self, angle: f64) -> Self {
        let (sina, cosa) = math::sin_cos(angle);
        Self::new(
            self.x * cosa + self.z * sina,
            self.y,
            self.x * -sina + self.z * cosa,
        )
    }

    /// Rotates around the z axis by `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn rotate_z(self, angle: f64) -> Self {
        let (sina, cosa) = math::sin_cos(angle);
        Self::new(
            self.x * cosa - self.y * sina,
            self.x * sina + self.y * cosa,
            self.z,
        )
    }

    /// Rotates around `axis` by `angle` (in radians).
    ///
    /// The axis must be a unit vector.
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn rotate_axis(self, axis: Self, angle: f64) -> Self {
        DQuat::from_axis_angle(axis, angle) * self
    }

    /// Rotates towards `rhs` up to `max_angle` (in radians).
    ///
    /// When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to
    /// `self.angle_between(rhs)`, the result will be parallel to `rhs`. If `max_angle` is negative,
    /// rotates towards the exact opposite of `rhs`. Will not go past the target.
    #[inline]
    #[must_use]
    pub fn rotate_towards(self, rhs: Self, max_angle: f64) -> Self {
        let angle_between = self.angle_between(rhs);
        // When `max_angle < 0`, rotate no further than `PI` radians away
        let angle = max_angle.clamp(angle_between - core::f64::consts::PI, angle_between);
        let axis = self
            .cross(rhs)
            .try_normalize()
            .unwrap_or_else(|| self.any_orthogonal_vector().normalize());
        DQuat::from_axis_angle(axis, angle) * self
    }

    /// Returns some vector that is orthogonal to the given one.
    ///
    /// The input vector must be finite and non-zero.
    ///
    /// The output vector is not necessarily unit length. For that use
    /// [`Self::any_orthonormal_vector()`] instead.
    #[inline]
    #[must_use]
    pub fn any_orthogonal_vector(self) -> Self {
        // This can probably be optimized
        if math::abs(self.x) > math::abs(self.y) {
            Self::new(-self.z, 0.0, self.x) // self.cross(Self::Y)
        } else {
            Self::new(0.0, self.z, -self.y) // self.cross(Self::X)
        }
    }

    /// Returns any unit vector that is orthogonal to the given one.
    ///
    /// The input vector must be unit length.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn any_orthonormal_vector(self) -> Self {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        let sign = math::signum(self.z);
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        Self::new(b, sign + self.y * self.y * a, -self.y)
    }

    /// Given a unit vector return two other vectors that together form an orthonormal
    /// basis. That is, all three vectors are orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn any_orthonormal_pair(self) -> (Self, Self) {
        glam_assert!(self.is_normalized());
        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        let sign = math::signum(self.z);
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }

    /// Performs a spherical linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[inline]
    #[must_use]
    pub fn slerp(self, rhs: Self, s: f64) -> Self {
        let self_length = self.length();
        let rhs_length = rhs.length();
        // Cosine of the angle between the vectors [-1, 1], or NaN if either vector has a zero length
        let dot = self.dot(rhs) / (self_length * rhs_length);
        // If dot is close to 1 or -1, or is NaN the calculations for t1 and t2 break down
        if math::abs(dot) < 1.0 - 3e-7 {
            // Angle between the vectors [0, +π]
            let theta = math::acos_approx(dot);
            // Sine of the angle between vectors [0, 1]
            let sin_theta = math::sin(theta);
            let t1 = math::sin(theta * (1. - s));
            let t2 = math::sin(theta * s);

            // Interpolate vector lengths
            let result_length = self_length.lerp(rhs_length, s);
            // Scale the vectors to the target length and interpolate them
            return (self * (result_length / self_length) * t1
                + rhs * (result_length / rhs_length) * t2)
                * sin_theta.recip();
        }
        if dot < 0.0 {
            // Vectors are almost parallel in opposing directions

            // Create a rotation from self to rhs along some axis
            let axis = self.any_orthogonal_vector().normalize();
            let rotation = DQuat::from_axis_angle(axis, core::f64::consts::PI * s);
            // Interpolate vector lengths
            let result_length = self_length.lerp(rhs_length, s);
            rotation * self * (result_length / self_length)
        } else {
            // Vectors are almost parallel in the same direction, or dot was NaN
            self.lerp(rhs, s)
        }
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[must_use]
    pub fn as_vec3(self) -> crate::Vec3 {
        crate::Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[must_use]
    pub fn as_vec3a(self) -> crate::Vec3A {
        crate::Vec3A::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `i8`.
    #[inline]
    #[must_use]
    pub fn as_i8vec3(self) -> crate::I8Vec3 {
        crate::I8Vec3::new(self.x as i8, self.y as i8, self.z as i8)
    }

    /// Casts all elements of `self` to `u8`.
    #[inline]
    #[must_use]
    pub fn as_u8vec3(self) -> crate::U8Vec3 {
        crate::U8Vec3::new(self.x as u8, self.y as u8, self.z as u8)
    }

    /// Casts all elements of `self` to `i16`.
    #[inline]
    #[must_use]
    pub fn as_i16vec3(self) -> crate::I16Vec3 {
        crate::I16Vec3::new(self.x as i16, self.y as i16, self.z as i16)
    }

    /// Casts all elements of `self` to `u16`.
    #[inline]
    #[must_use]
    pub fn as_u16vec3(self) -> crate::U16Vec3 {
        crate::U16Vec3::new(self.x as u16, self.y as u16, self.z as u16)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline]
    #[must_use]
    pub fn as_ivec3(self) -> crate::IVec3 {
        crate::IVec3::new(self.x as i32, self.y as i32, self.z as i32)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline]
    #[must_use]
    pub fn as_uvec3(self) -> crate::UVec3 {
        crate::UVec3::new(self.x as u32, self.y as u32, self.z as u32)
    }

    /// Casts all elements of `self` to `i64`.
    #[inline]
    #[must_use]
    pub fn as_i64vec3(self) -> crate::I64Vec3 {
        crate::I64Vec3::new(self.x as i64, self.y as i64, self.z as i64)
    }

    /// Casts all elements of `self` to `u64`.
    #[inline]
    #[must_use]
    pub fn as_u64vec3(self) -> crate::U64Vec3 {
        crate::U64Vec3::new(self.x as u64, self.y as u64, self.z as u64)
    }

    /// Casts all elements of `self` to `isize`.
    #[inline]
    #[must_use]
    pub fn as_isizevec3(self) -> crate::ISizeVec3 {
        crate::ISizeVec3::new(self.x as isize, self.y as isize, self.z as isize)
    }

    /// Casts all elements of `self` to `usize`.
    #[inline]
    #[must_use]
    pub fn as_usizevec3(self) -> crate::USizeVec3 {
        crate::USizeVec3::new(self.x as usize, self.y as usize, self.z as usize)
    }
}

impl Default for DVec3 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div for DVec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
            z: self.z.div(rhs.z),
        }
    }
}

impl Div<&Self> for DVec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &Self) -> Self {
        self.div(*rhs)
    }
}

impl Div<&DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn div(self, rhs: &DVec3) -> DVec3 {
        (*self).div(*rhs)
    }
}

impl Div<DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn div(self, rhs: DVec3) -> DVec3 {
        (*self).div(rhs)
    }
}

impl DivAssign for DVec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
    }
}

impl DivAssign<&Self> for DVec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs);
    }
}

impl Div<f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}

impl Div<&f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f64) -> Self {
        self.div(*rhs)
    }
}

impl Div<&f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn div(self, rhs: &f64) -> DVec3 {
        (*self).div(*rhs)
    }
}

impl Div<f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn div(self, rhs: f64) -> DVec3 {
        (*self).div(rhs)
    }
}

impl DivAssign<f64> for DVec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
    }
}

impl DivAssign<&f64> for DVec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &f64) {
        self.div_assign(*rhs);
    }
}

impl Div<DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn div(self, rhs: DVec3) -> DVec3 {
        DVec3 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
        }
    }
}

impl Div<&DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn div(self, rhs: &DVec3) -> DVec3 {
        self.div(*rhs)
    }
}

impl Div<&DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn div(self, rhs: &DVec3) -> DVec3 {
        (*self).div(*rhs)
    }
}

impl Div<DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn div(self, rhs: DVec3) -> DVec3 {
        (*self).div(rhs)
    }
}

impl Mul for DVec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}

impl Mul<&Self> for DVec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: &DVec3) -> DVec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: DVec3) -> DVec3 {
        (*self).mul(rhs)
    }
}

impl MulAssign for DVec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
    }
}

impl MulAssign<&Self> for DVec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Mul<f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}

impl Mul<&f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &f64) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: &f64) -> DVec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: f64) -> DVec3 {
        (*self).mul(rhs)
    }
}

impl MulAssign<f64> for DVec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
    }
}

impl MulAssign<&f64> for DVec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f64) {
        self.mul_assign(*rhs);
    }
}

impl Mul<DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: DVec3) -> DVec3 {
        DVec3 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
        }
    }
}

impl Mul<&DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: &DVec3) -> DVec3 {
        self.mul(*rhs)
    }
}

impl Mul<&DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: &DVec3) -> DVec3 {
        (*self).mul(*rhs)
    }
}

impl Mul<DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn mul(self, rhs: DVec3) -> DVec3 {
        (*self).mul(rhs)
    }
}

impl Add for DVec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl Add<&Self> for DVec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl Add<&DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn add(self, rhs: &DVec3) -> DVec3 {
        (*self).add(*rhs)
    }
}

impl Add<DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn add(self, rhs: DVec3) -> DVec3 {
        (*self).add(rhs)
    }
}

impl AddAssign for DVec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}

impl AddAssign<&Self> for DVec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl Add<f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f64) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
        }
    }
}

impl Add<&f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &f64) -> Self {
        self.add(*rhs)
    }
}

impl Add<&f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn add(self, rhs: &f64) -> DVec3 {
        (*self).add(*rhs)
    }
}

impl Add<f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn add(self, rhs: f64) -> DVec3 {
        (*self).add(rhs)
    }
}

impl AddAssign<f64> for DVec3 {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
    }
}

impl AddAssign<&f64> for DVec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &f64) {
        self.add_assign(*rhs);
    }
}

impl Add<DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn add(self, rhs: DVec3) -> DVec3 {
        DVec3 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
        }
    }
}

impl Add<&DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn add(self, rhs: &DVec3) -> DVec3 {
        self.add(*rhs)
    }
}

impl Add<&DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn add(self, rhs: &DVec3) -> DVec3 {
        (*self).add(*rhs)
    }
}

impl Add<DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn add(self, rhs: DVec3) -> DVec3 {
        (*self).add(rhs)
    }
}

impl Sub for DVec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
            z: self.z.sub(rhs.z),
        }
    }
}

impl Sub<&Self> for DVec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn sub(self, rhs: &DVec3) -> DVec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn sub(self, rhs: DVec3) -> DVec3 {
        (*self).sub(rhs)
    }
}

impl SubAssign for DVec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
    }
}

impl SubAssign<&Self> for DVec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

impl Sub<f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f64) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
        }
    }
}

impl Sub<&f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &f64) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn sub(self, rhs: &f64) -> DVec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn sub(self, rhs: f64) -> DVec3 {
        (*self).sub(rhs)
    }
}

impl SubAssign<f64> for DVec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f64) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
    }
}

impl SubAssign<&f64> for DVec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &f64) {
        self.sub_assign(*rhs);
    }
}

impl Sub<DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn sub(self, rhs: DVec3) -> DVec3 {
        DVec3 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
        }
    }
}

impl Sub<&DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn sub(self, rhs: &DVec3) -> DVec3 {
        self.sub(*rhs)
    }
}

impl Sub<&DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn sub(self, rhs: &DVec3) -> DVec3 {
        (*self).sub(*rhs)
    }
}

impl Sub<DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn sub(self, rhs: DVec3) -> DVec3 {
        (*self).sub(rhs)
    }
}

impl Rem for DVec3 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
            z: self.z.rem(rhs.z),
        }
    }
}

impl Rem<&Self> for DVec3 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &Self) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn rem(self, rhs: &DVec3) -> DVec3 {
        (*self).rem(*rhs)
    }
}

impl Rem<DVec3> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn rem(self, rhs: DVec3) -> DVec3 {
        (*self).rem(rhs)
    }
}

impl RemAssign for DVec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
        self.z.rem_assign(rhs.z);
    }
}

impl RemAssign<&Self> for DVec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs);
    }
}

impl Rem<f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f64) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
            z: self.z.rem(rhs),
        }
    }
}

impl Rem<&f64> for DVec3 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &f64) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn rem(self, rhs: &f64) -> DVec3 {
        (*self).rem(*rhs)
    }
}

impl Rem<f64> for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn rem(self, rhs: f64) -> DVec3 {
        (*self).rem(rhs)
    }
}

impl RemAssign<f64> for DVec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: f64) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
        self.z.rem_assign(rhs);
    }
}

impl RemAssign<&f64> for DVec3 {
    #[inline]
    fn rem_assign(&mut self, rhs: &f64) {
        self.rem_assign(*rhs);
    }
}

impl Rem<DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn rem(self, rhs: DVec3) -> DVec3 {
        DVec3 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
            z: self.rem(rhs.z),
        }
    }
}

impl Rem<&DVec3> for f64 {
    type Output = DVec3;
    #[inline]
    fn rem(self, rhs: &DVec3) -> DVec3 {
        self.rem(*rhs)
    }
}

impl Rem<&DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn rem(self, rhs: &DVec3) -> DVec3 {
        (*self).rem(*rhs)
    }
}

impl Rem<DVec3> for &f64 {
    type Output = DVec3;
    #[inline]
    fn rem(self, rhs: DVec3) -> DVec3 {
        (*self).rem(rhs)
    }
}

impl AsRef<[f64; 3]> for DVec3 {
    #[inline]
    fn as_ref(&self) -> &[f64; 3] {
        unsafe { &*(self as *const Self as *const [f64; 3]) }
    }
}

impl AsMut<[f64; 3]> for DVec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f64; 3] {
        unsafe { &mut *(self as *mut Self as *mut [f64; 3]) }
    }
}

impl Sum for DVec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for DVec3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for DVec3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for DVec3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Neg for DVec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl Neg for &DVec3 {
    type Output = DVec3;
    #[inline]
    fn neg(self) -> DVec3 {
        (*self).neg()
    }
}

impl Index<usize> for DVec3 {
    type Output = f64;
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

impl IndexMut<usize> for DVec3 {
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

impl fmt::Display for DVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "[{:.*}, {:.*}, {:.*}]", p, self.x, p, self.y, p, self.z)
        } else {
            write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
        }
    }
}

impl fmt::Debug for DVec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(DVec3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl From<[f64; 3]> for DVec3 {
    #[inline]
    fn from(a: [f64; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<DVec3> for [f64; 3] {
    #[inline]
    fn from(v: DVec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<(f64, f64, f64)> for DVec3 {
    #[inline]
    fn from(t: (f64, f64, f64)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<DVec3> for (f64, f64, f64) {
    #[inline]
    fn from(v: DVec3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl From<(DVec2, f64)> for DVec3 {
    #[inline]
    fn from((v, z): (DVec2, f64)) -> Self {
        Self::new(v.x, v.y, z)
    }
}

impl From<Vec3> for DVec3 {
    #[inline]
    fn from(v: Vec3) -> Self {
        Self::new(f64::from(v.x), f64::from(v.y), f64::from(v.z))
    }
}

impl From<IVec3> for DVec3 {
    #[inline]
    fn from(v: IVec3) -> Self {
        Self::new(f64::from(v.x), f64::from(v.y), f64::from(v.z))
    }
}

impl From<UVec3> for DVec3 {
    #[inline]
    fn from(v: UVec3) -> Self {
        Self::new(f64::from(v.x), f64::from(v.y), f64::from(v.z))
    }
}

impl From<BVec3> for DVec3 {
    #[inline]
    fn from(v: BVec3) -> Self {
        Self::new(f64::from(v.x), f64::from(v.y), f64::from(v.z))
    }
}

impl From<BVec3A> for DVec3 {
    #[inline]
    fn from(v: BVec3A) -> Self {
        let bool_array: [bool; 3] = v.into();
        Self::new(
            f64::from(bool_array[0]),
            f64::from(bool_array[1]),
            f64::from(bool_array[2]),
        )
    }
}
