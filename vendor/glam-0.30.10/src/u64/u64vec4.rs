// Generated from vec.rs.tera template. Edit the template, not the generated file.

#[cfg(not(feature = "scalar-math"))]
use crate::BVec4A;
use crate::{
    BVec4, I16Vec4, I64Vec4, I8Vec4, IVec4, U16Vec4, U64Vec2, U64Vec3, U8Vec4, USizeVec4, UVec4,
};

use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(feature = "zerocopy")]
use zerocopy_derive::*;

/// Creates a 4-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn u64vec4(x: u64, y: u64, z: u64, w: u64) -> U64Vec4 {
    U64Vec4::new(x, y, z, w)
}

/// A 4-dimensional vector.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(
    feature = "zerocopy",
    derive(FromBytes, Immutable, IntoBytes, KnownLayout)
)]
#[cfg_attr(feature = "cuda", repr(align(16)))]
#[repr(C)]
#[cfg_attr(target_arch = "spirv", rust_gpu::vector::v1)]
pub struct U64Vec4 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}

impl U64Vec4 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// All ones.
    pub const ONE: Self = Self::splat(1);

    /// All `u64::MIN`.
    pub const MIN: Self = Self::splat(u64::MIN);

    /// All `u64::MAX`.
    pub const MAX: Self = Self::splat(u64::MAX);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1, 0, 0, 0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0, 1, 0, 0);

    /// A unit vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0, 0, 1, 0);

    /// A unit vector pointing along the positive W axis.
    pub const W: Self = Self::new(0, 0, 0, 1);

    /// The unit axes.
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: u64, y: u64, z: u64, w: u64) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: u64) -> Self {
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
        F: Fn(u64) -> u64,
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
    pub fn select(mask: BVec4, if_true: Self, if_false: Self) -> Self {
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
    pub const fn from_array(a: [u64; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// Converts `self` to `[x, y, z, w]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [u64; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Creates a vector from the first 4 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[u64]) -> Self {
        assert!(slice.len() >= 4);
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }

    /// Writes the elements of `self` to the first 4 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 4 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [u64]) {
        slice[..4].copy_from_slice(&self.to_array());
    }

    /// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
    ///
    /// Truncation to [`U64Vec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].
    #[inline]
    #[must_use]
    pub fn truncate(self) -> U64Vec3 {
        use crate::swizzles::Vec4Swizzles;
        self.xyz()
    }

    /// Creates a 4D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: u64) -> Self {
        self.x = x;
        self
    }

    /// Creates a 4D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: u64) -> Self {
        self.y = y;
        self
    }

    /// Creates a 4D vector from `self` with the given value of `z`.
    #[inline]
    #[must_use]
    pub fn with_z(mut self, z: u64) -> Self {
        self.z = z;
        self
    }

    /// Creates a 4D vector from `self` with the given value of `w`.
    #[inline]
    #[must_use]
    pub fn with_w(mut self, w: u64) -> Self {
        self.w = w;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> u64 {
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
    /// In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: if self.x < rhs.x { self.x } else { rhs.x },
            y: if self.y < rhs.y { self.y } else { rhs.y },
            z: if self.z < rhs.z { self.z } else { rhs.z },
            w: if self.w < rhs.w { self.w } else { rhs.w },
        }
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: if self.x > rhs.x { self.x } else { rhs.x },
            y: if self.y > rhs.y { self.y } else { rhs.y },
            z: if self.z > rhs.z { self.z } else { rhs.z },
            w: if self.w > rhs.w { self.w } else { rhs.w },
        }
    }

    /// Component-wise clamping of values, similar to [`u64::clamp`].
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
    pub fn min_element(self) -> u64 {
        let min = |a, b| if a < b { a } else { b };
        min(self.x, min(self.y, min(self.z, self.w)))
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> u64 {
        let max = |a, b| if a > b { a } else { b };
        max(self.x, max(self.y, max(self.z, self.w)))
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
            min = self.z;
            index = 2;
        }
        if self.w < min {
            index = 3;
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
            max = self.z;
            index = 2;
        }
        if self.w > max {
            index = 3;
        }
        index
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> u64 {
        self.x + self.y + self.z + self.w
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> u64 {
        self.x * self.y * self.z * self.w
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpeq(self, rhs: Self) -> BVec4 {
        BVec4::new(
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
    pub fn cmpne(self, rhs: Self) -> BVec4 {
        BVec4::new(
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
    pub fn cmpge(self, rhs: Self) -> BVec4 {
        BVec4::new(
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
    pub fn cmpgt(self, rhs: Self) -> BVec4 {
        BVec4::new(
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
    pub fn cmple(self, rhs: Self) -> BVec4 {
        BVec4::new(
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
    pub fn cmplt(self, rhs: Self) -> BVec4 {
        BVec4::new(
            self.x.lt(&rhs.x),
            self.y.lt(&rhs.y),
            self.z.lt(&rhs.z),
            self.w.lt(&rhs.w),
        )
    }

    /// Computes the squared length of `self`.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> u64 {
        self.dot(self)
    }

    /// Computes the [manhattan distance] between two points.
    ///
    /// # Overflow
    /// This method may overflow if the result is greater than [`u64::MAX`].
    ///
    /// See also [`checked_manhattan_distance`][U64Vec4::checked_manhattan_distance].
    ///
    /// [manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    #[inline]
    #[must_use]
    pub fn manhattan_distance(self, rhs: Self) -> u64 {
        self.x.abs_diff(rhs.x)
            + self.y.abs_diff(rhs.y)
            + self.z.abs_diff(rhs.z)
            + self.w.abs_diff(rhs.w)
    }

    /// Computes the [manhattan distance] between two points.
    ///
    /// This will returns [`None`] if the result is greater than [`u64::MAX`].
    ///
    /// [manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    #[inline]
    #[must_use]
    pub fn checked_manhattan_distance(self, rhs: Self) -> Option<u64> {
        let d = self.x.abs_diff(rhs.x);
        let d = d.checked_add(self.y.abs_diff(rhs.y))?;
        let d = d.checked_add(self.z.abs_diff(rhs.z))?;
        d.checked_add(self.w.abs_diff(rhs.w))
    }

    /// Computes the [chebyshev distance] between two points.
    ///
    /// [chebyshev distance]: https://en.wikipedia.org/wiki/Chebyshev_distance
    #[inline]
    #[must_use]
    pub fn chebyshev_distance(self, rhs: Self) -> u64 {
        // Note: the compiler will eventually optimize out the loop
        [
            self.x.abs_diff(rhs.x),
            self.y.abs_diff(rhs.y),
            self.z.abs_diff(rhs.z),
            self.w.abs_diff(rhs.w),
        ]
        .into_iter()
        .max()
        .unwrap()
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[must_use]
    pub fn as_vec4(&self) -> crate::Vec4 {
        crate::Vec4::new(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
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

    /// Casts all elements of `self` to `usize`.
    #[inline]
    #[must_use]
    pub fn as_usizevec4(&self) -> crate::USizeVec4 {
        crate::USizeVec4::new(
            self.x as usize,
            self.y as usize,
            self.z as usize,
            self.w as usize,
        )
    }

    /// Returns a vector containing the wrapping addition of `self` and `rhs`.
    ///
    /// In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let x = match self.x.checked_add(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_add(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_add(rhs.z) {
            Some(v) => v,
            None => return None,
        };
        let w = match self.w.checked_add(rhs.w) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z, w })
    }

    /// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
    ///
    /// In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        let x = match self.x.checked_sub(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_sub(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_sub(rhs.z) {
            Some(v) => v,
            None => return None,
        };
        let w = match self.w.checked_sub(rhs.w) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z, w })
    }

    /// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
    ///
    /// In other words this computes `Some([self.x * rhs.x, self.y * rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
        let x = match self.x.checked_mul(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_mul(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_mul(rhs.z) {
            Some(v) => v,
            None => return None,
        };
        let w = match self.w.checked_mul(rhs.w) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z, w })
    }

    /// Returns a vector containing the wrapping division of `self` and `rhs`.
    ///
    /// In other words this computes `Some([self.x / rhs.x, self.y / rhs.y, ..])` but returns `None` on any division by zero.
    #[inline]
    #[must_use]
    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
        let x = match self.x.checked_div(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_div(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_div(rhs.z) {
            Some(v) => v,
            None => return None,
        };
        let w = match self.w.checked_div(rhs.w) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z, w })
    }

    /// Returns a vector containing the wrapping addition of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            z: self.z.wrapping_add(rhs.z),
            w: self.w.wrapping_add(rhs.w),
        }
    }

    /// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            z: self.z.wrapping_sub(rhs.z),
            w: self.w.wrapping_sub(rhs.w),
        }
    }

    /// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_mul(rhs.x),
            y: self.y.wrapping_mul(rhs.y),
            z: self.z.wrapping_mul(rhs.z),
            w: self.w.wrapping_mul(rhs.w),
        }
    }

    /// Returns a vector containing the wrapping division of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_div(self, rhs: Self) -> Self {
        Self {
            x: self.x.wrapping_div(rhs.x),
            y: self.y.wrapping_div(rhs.y),
            z: self.z.wrapping_div(rhs.z),
            w: self.w.wrapping_div(rhs.w),
        }
    }

    /// Returns a vector containing the saturating addition of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
            z: self.z.saturating_add(rhs.z),
            w: self.w.saturating_add(rhs.w),
        }
    }

    /// Returns a vector containing the saturating subtraction of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
            z: self.z.saturating_sub(rhs.z),
            w: self.w.saturating_sub(rhs.w),
        }
    }

    /// Returns a vector containing the saturating multiplication of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_mul(rhs.x),
            y: self.y.saturating_mul(rhs.y),
            z: self.z.saturating_mul(rhs.z),
            w: self.w.saturating_mul(rhs.w),
        }
    }

    /// Returns a vector containing the saturating division of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_div(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_div(rhs.x),
            y: self.y.saturating_div(rhs.y),
            z: self.z.saturating_div(rhs.z),
            w: self.w.saturating_div(rhs.w),
        }
    }

    /// Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_add_signed(self, rhs: I64Vec4) -> Option<Self> {
        let x = match self.x.checked_add_signed(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_add_signed(rhs.y) {
            Some(v) => v,
            None => return None,
        };
        let z = match self.z.checked_add_signed(rhs.z) {
            Some(v) => v,
            None => return None,
        };
        let w = match self.w.checked_add_signed(rhs.w) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y, z, w })
    }

    /// Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_add_signed(rhs.x), self.y.wrapping_add_signed(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_add_signed(self, rhs: I64Vec4) -> Self {
        Self {
            x: self.x.wrapping_add_signed(rhs.x),
            y: self.y.wrapping_add_signed(rhs.y),
            z: self.z.wrapping_add_signed(rhs.z),
            w: self.w.wrapping_add_signed(rhs.w),
        }
    }

    /// Returns a vector containing the saturating addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_add_signed(rhs.x), self.y.saturating_add_signed(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_add_signed(self, rhs: I64Vec4) -> Self {
        Self {
            x: self.x.saturating_add_signed(rhs.x),
            y: self.y.saturating_add_signed(rhs.y),
            z: self.z.saturating_add_signed(rhs.z),
            w: self.w.saturating_add_signed(rhs.w),
        }
    }
}

impl Default for U64Vec4 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div for U64Vec4 {
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

impl Div<&Self> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &Self) -> Self {
        self.div(*rhs)
    }
}

impl Div<&U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn div(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).div(*rhs)
    }
}

impl Div<U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn div(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).div(rhs)
    }
}

impl DivAssign for U64Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
        self.z.div_assign(rhs.z);
        self.w.div_assign(rhs.w);
    }
}

impl DivAssign<&Self> for U64Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs);
    }
}

impl Div<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: u64) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
            w: self.w.div(rhs),
        }
    }
}

impl Div<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &u64) -> Self {
        self.div(*rhs)
    }
}

impl Div<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn div(self, rhs: &u64) -> U64Vec4 {
        (*self).div(*rhs)
    }
}

impl Div<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn div(self, rhs: u64) -> U64Vec4 {
        (*self).div(rhs)
    }
}

impl DivAssign<u64> for U64Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: u64) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
        self.z.div_assign(rhs);
        self.w.div_assign(rhs);
    }
}

impl DivAssign<&u64> for U64Vec4 {
    #[inline]
    fn div_assign(&mut self, rhs: &u64) {
        self.div_assign(*rhs);
    }
}

impl Div<U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn div(self, rhs: U64Vec4) -> U64Vec4 {
        U64Vec4 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
            z: self.div(rhs.z),
            w: self.div(rhs.w),
        }
    }
}

impl Div<&U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn div(self, rhs: &U64Vec4) -> U64Vec4 {
        self.div(*rhs)
    }
}

impl Div<&U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn div(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).div(*rhs)
    }
}

impl Div<U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn div(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).div(rhs)
    }
}

impl Mul for U64Vec4 {
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

impl Mul<&Self> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn mul(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).mul(*rhs)
    }
}

impl Mul<U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn mul(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).mul(rhs)
    }
}

impl MulAssign for U64Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
        self.z.mul_assign(rhs.z);
        self.w.mul_assign(rhs.w);
    }
}

impl MulAssign<&Self> for U64Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Mul<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: u64) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
            w: self.w.mul(rhs),
        }
    }
}

impl Mul<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &u64) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn mul(self, rhs: &u64) -> U64Vec4 {
        (*self).mul(*rhs)
    }
}

impl Mul<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn mul(self, rhs: u64) -> U64Vec4 {
        (*self).mul(rhs)
    }
}

impl MulAssign<u64> for U64Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: u64) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
        self.z.mul_assign(rhs);
        self.w.mul_assign(rhs);
    }
}

impl MulAssign<&u64> for U64Vec4 {
    #[inline]
    fn mul_assign(&mut self, rhs: &u64) {
        self.mul_assign(*rhs);
    }
}

impl Mul<U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn mul(self, rhs: U64Vec4) -> U64Vec4 {
        U64Vec4 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
            z: self.mul(rhs.z),
            w: self.mul(rhs.w),
        }
    }
}

impl Mul<&U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn mul(self, rhs: &U64Vec4) -> U64Vec4 {
        self.mul(*rhs)
    }
}

impl Mul<&U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn mul(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).mul(*rhs)
    }
}

impl Mul<U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn mul(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).mul(rhs)
    }
}

impl Add for U64Vec4 {
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

impl Add<&Self> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl Add<&U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn add(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).add(*rhs)
    }
}

impl Add<U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn add(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).add(rhs)
    }
}

impl AddAssign for U64Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
        self.w.add_assign(rhs.w);
    }
}

impl AddAssign<&Self> for U64Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl Add<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u64) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
            z: self.z.add(rhs),
            w: self.w.add(rhs),
        }
    }
}

impl Add<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &u64) -> Self {
        self.add(*rhs)
    }
}

impl Add<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn add(self, rhs: &u64) -> U64Vec4 {
        (*self).add(*rhs)
    }
}

impl Add<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn add(self, rhs: u64) -> U64Vec4 {
        (*self).add(rhs)
    }
}

impl AddAssign<u64> for U64Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
        self.z.add_assign(rhs);
        self.w.add_assign(rhs);
    }
}

impl AddAssign<&u64> for U64Vec4 {
    #[inline]
    fn add_assign(&mut self, rhs: &u64) {
        self.add_assign(*rhs);
    }
}

impl Add<U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn add(self, rhs: U64Vec4) -> U64Vec4 {
        U64Vec4 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
            z: self.add(rhs.z),
            w: self.add(rhs.w),
        }
    }
}

impl Add<&U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn add(self, rhs: &U64Vec4) -> U64Vec4 {
        self.add(*rhs)
    }
}

impl Add<&U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn add(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).add(*rhs)
    }
}

impl Add<U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn add(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).add(rhs)
    }
}

impl Sub for U64Vec4 {
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

impl Sub<&Self> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn sub(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).sub(*rhs)
    }
}

impl Sub<U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn sub(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).sub(rhs)
    }
}

impl SubAssign for U64Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
        self.z.sub_assign(rhs.z);
        self.w.sub_assign(rhs.w);
    }
}

impl SubAssign<&Self> for U64Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

impl Sub<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u64) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
            z: self.z.sub(rhs),
            w: self.w.sub(rhs),
        }
    }
}

impl Sub<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &u64) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn sub(self, rhs: &u64) -> U64Vec4 {
        (*self).sub(*rhs)
    }
}

impl Sub<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn sub(self, rhs: u64) -> U64Vec4 {
        (*self).sub(rhs)
    }
}

impl SubAssign<u64> for U64Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: u64) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
        self.z.sub_assign(rhs);
        self.w.sub_assign(rhs);
    }
}

impl SubAssign<&u64> for U64Vec4 {
    #[inline]
    fn sub_assign(&mut self, rhs: &u64) {
        self.sub_assign(*rhs);
    }
}

impl Sub<U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn sub(self, rhs: U64Vec4) -> U64Vec4 {
        U64Vec4 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
            z: self.sub(rhs.z),
            w: self.sub(rhs.w),
        }
    }
}

impl Sub<&U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn sub(self, rhs: &U64Vec4) -> U64Vec4 {
        self.sub(*rhs)
    }
}

impl Sub<&U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn sub(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).sub(*rhs)
    }
}

impl Sub<U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn sub(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).sub(rhs)
    }
}

impl Rem for U64Vec4 {
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

impl Rem<&Self> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &Self) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn rem(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).rem(*rhs)
    }
}

impl Rem<U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn rem(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).rem(rhs)
    }
}

impl RemAssign for U64Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
        self.z.rem_assign(rhs.z);
        self.w.rem_assign(rhs.w);
    }
}

impl RemAssign<&Self> for U64Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs);
    }
}

impl Rem<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: u64) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
            z: self.z.rem(rhs),
            w: self.w.rem(rhs),
        }
    }
}

impl Rem<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &u64) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn rem(self, rhs: &u64) -> U64Vec4 {
        (*self).rem(*rhs)
    }
}

impl Rem<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn rem(self, rhs: u64) -> U64Vec4 {
        (*self).rem(rhs)
    }
}

impl RemAssign<u64> for U64Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: u64) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
        self.z.rem_assign(rhs);
        self.w.rem_assign(rhs);
    }
}

impl RemAssign<&u64> for U64Vec4 {
    #[inline]
    fn rem_assign(&mut self, rhs: &u64) {
        self.rem_assign(*rhs);
    }
}

impl Rem<U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn rem(self, rhs: U64Vec4) -> U64Vec4 {
        U64Vec4 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
            z: self.rem(rhs.z),
            w: self.rem(rhs.w),
        }
    }
}

impl Rem<&U64Vec4> for u64 {
    type Output = U64Vec4;
    #[inline]
    fn rem(self, rhs: &U64Vec4) -> U64Vec4 {
        self.rem(*rhs)
    }
}

impl Rem<&U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn rem(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).rem(*rhs)
    }
}

impl Rem<U64Vec4> for &u64 {
    type Output = U64Vec4;
    #[inline]
    fn rem(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).rem(rhs)
    }
}

impl AsRef<[u64; 4]> for U64Vec4 {
    #[inline]
    fn as_ref(&self) -> &[u64; 4] {
        unsafe { &*(self as *const Self as *const [u64; 4]) }
    }
}

impl AsMut<[u64; 4]> for U64Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u64; 4] {
        unsafe { &mut *(self as *mut Self as *mut [u64; 4]) }
    }
}

impl Sum for U64Vec4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for U64Vec4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for U64Vec4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for U64Vec4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Not for U64Vec4 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self {
            x: self.x.not(),
            y: self.y.not(),
            z: self.z.not(),
            w: self.w.not(),
        }
    }
}

impl Not for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn not(self) -> U64Vec4 {
        (*self).not()
    }
}

impl BitAnd for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
            z: self.z.bitand(rhs.z),
            w: self.w.bitand(rhs.w),
        }
    }
}

impl BitAnd<&Self> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: &Self) -> Self {
        self.bitand(*rhs)
    }
}

impl BitAnd<&U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitand(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).bitand(*rhs)
    }
}

impl BitAnd<U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitand(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).bitand(rhs)
    }
}

impl BitAndAssign for U64Vec4 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitAndAssign<&Self> for U64Vec4 {
    #[inline]
    fn bitand_assign(&mut self, rhs: &Self) {
        self.bitand_assign(*rhs);
    }
}

impl BitOr for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
            z: self.z.bitor(rhs.z),
            w: self.w.bitor(rhs.w),
        }
    }
}

impl BitOr<&Self> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: &Self) -> Self {
        self.bitor(*rhs)
    }
}

impl BitOr<&U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitor(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).bitor(*rhs)
    }
}

impl BitOr<U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitor(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).bitor(rhs)
    }
}

impl BitOrAssign for U64Vec4 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitOrAssign<&Self> for U64Vec4 {
    #[inline]
    fn bitor_assign(&mut self, rhs: &Self) {
        self.bitor_assign(*rhs);
    }
}

impl BitXor for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
            z: self.z.bitxor(rhs.z),
            w: self.w.bitxor(rhs.w),
        }
    }
}

impl BitXor<&Self> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: &Self) -> Self {
        self.bitxor(*rhs)
    }
}

impl BitXor<&U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitxor(self, rhs: &U64Vec4) -> U64Vec4 {
        (*self).bitxor(*rhs)
    }
}

impl BitXor<U64Vec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitxor(self, rhs: U64Vec4) -> U64Vec4 {
        (*self).bitxor(rhs)
    }
}

impl BitXorAssign for U64Vec4 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl BitXorAssign<&Self> for U64Vec4 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.bitxor_assign(*rhs);
    }
}

impl BitAnd<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.bitand(rhs),
            y: self.y.bitand(rhs),
            z: self.z.bitand(rhs),
            w: self.w.bitand(rhs),
        }
    }
}

impl BitAnd<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: &u64) -> Self {
        self.bitand(*rhs)
    }
}

impl BitAnd<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitand(self, rhs: &u64) -> U64Vec4 {
        (*self).bitand(*rhs)
    }
}

impl BitAnd<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitand(self, rhs: u64) -> U64Vec4 {
        (*self).bitand(rhs)
    }
}

impl BitAndAssign<u64> for U64Vec4 {
    #[inline]
    fn bitand_assign(&mut self, rhs: u64) {
        *self = self.bitand(rhs);
    }
}

impl BitAndAssign<&u64> for U64Vec4 {
    #[inline]
    fn bitand_assign(&mut self, rhs: &u64) {
        self.bitand_assign(*rhs);
    }
}

impl BitOr<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.bitor(rhs),
            y: self.y.bitor(rhs),
            z: self.z.bitor(rhs),
            w: self.w.bitor(rhs),
        }
    }
}

impl BitOr<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: &u64) -> Self {
        self.bitor(*rhs)
    }
}

impl BitOr<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitor(self, rhs: &u64) -> U64Vec4 {
        (*self).bitor(*rhs)
    }
}

impl BitOr<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitor(self, rhs: u64) -> U64Vec4 {
        (*self).bitor(rhs)
    }
}

impl BitOrAssign<u64> for U64Vec4 {
    #[inline]
    fn bitor_assign(&mut self, rhs: u64) {
        *self = self.bitor(rhs);
    }
}

impl BitOrAssign<&u64> for U64Vec4 {
    #[inline]
    fn bitor_assign(&mut self, rhs: &u64) {
        self.bitor_assign(*rhs);
    }
}

impl BitXor<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs),
            y: self.y.bitxor(rhs),
            z: self.z.bitxor(rhs),
            w: self.w.bitxor(rhs),
        }
    }
}

impl BitXor<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: &u64) -> Self {
        self.bitxor(*rhs)
    }
}

impl BitXor<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitxor(self, rhs: &u64) -> U64Vec4 {
        (*self).bitxor(*rhs)
    }
}

impl BitXor<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn bitxor(self, rhs: u64) -> U64Vec4 {
        (*self).bitxor(rhs)
    }
}

impl BitXorAssign<u64> for U64Vec4 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: u64) {
        *self = self.bitxor(rhs);
    }
}

impl BitXorAssign<&u64> for U64Vec4 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &u64) {
        self.bitxor_assign(*rhs);
    }
}

impl Shl<i8> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
            w: self.w.shl(rhs),
        }
    }
}

impl Shl<&i8> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &i8) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&i8> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &i8) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<i8> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: i8) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<i8> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: i8) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&i8> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: &i8) {
        self.shl_assign(*rhs);
    }
}

impl Shr<i8> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
            w: self.w.shr(rhs),
        }
    }
}

impl Shr<&i8> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &i8) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&i8> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &i8) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<i8> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: i8) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<i8> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: i8) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&i8> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: &i8) {
        self.shr_assign(*rhs);
    }
}

impl Shl<i16> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
            w: self.w.shl(rhs),
        }
    }
}

impl Shl<&i16> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &i16) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&i16> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &i16) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<i16> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: i16) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<i16> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: i16) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&i16> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: &i16) {
        self.shl_assign(*rhs);
    }
}

impl Shr<i16> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
            w: self.w.shr(rhs),
        }
    }
}

impl Shr<&i16> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &i16) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&i16> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &i16) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<i16> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: i16) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<i16> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: i16) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&i16> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: &i16) {
        self.shr_assign(*rhs);
    }
}

impl Shl<i32> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
            w: self.w.shl(rhs),
        }
    }
}

impl Shl<&i32> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &i32) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&i32> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &i32) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<i32> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: i32) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<i32> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: i32) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&i32> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: &i32) {
        self.shl_assign(*rhs);
    }
}

impl Shr<i32> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
            w: self.w.shr(rhs),
        }
    }
}

impl Shr<&i32> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &i32) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&i32> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &i32) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<i32> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: i32) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<i32> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: i32) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&i32> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: &i32) {
        self.shr_assign(*rhs);
    }
}

impl Shl<i64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
            w: self.w.shl(rhs),
        }
    }
}

impl Shl<&i64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &i64) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&i64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &i64) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<i64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: i64) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<i64> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: i64) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&i64> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: &i64) {
        self.shl_assign(*rhs);
    }
}

impl Shr<i64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
            w: self.w.shr(rhs),
        }
    }
}

impl Shr<&i64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &i64) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&i64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &i64) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<i64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: i64) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<i64> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: i64) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&i64> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: &i64) {
        self.shr_assign(*rhs);
    }
}

impl Shl<u8> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
            w: self.w.shl(rhs),
        }
    }
}

impl Shl<&u8> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &u8) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&u8> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &u8) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<u8> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: u8) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<u8> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: u8) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&u8> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: &u8) {
        self.shl_assign(*rhs);
    }
}

impl Shr<u8> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
            w: self.w.shr(rhs),
        }
    }
}

impl Shr<&u8> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &u8) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&u8> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &u8) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<u8> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: u8) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<u8> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: u8) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&u8> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: &u8) {
        self.shr_assign(*rhs);
    }
}

impl Shl<u16> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
            w: self.w.shl(rhs),
        }
    }
}

impl Shl<&u16> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &u16) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&u16> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &u16) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<u16> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: u16) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<u16> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: u16) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&u16> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: &u16) {
        self.shl_assign(*rhs);
    }
}

impl Shr<u16> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
            w: self.w.shr(rhs),
        }
    }
}

impl Shr<&u16> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &u16) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&u16> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &u16) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<u16> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: u16) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<u16> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: u16) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&u16> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: &u16) {
        self.shr_assign(*rhs);
    }
}

impl Shl<u32> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
            w: self.w.shl(rhs),
        }
    }
}

impl Shl<&u32> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &u32) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&u32> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &u32) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<u32> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: u32) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<u32> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&u32> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: &u32) {
        self.shl_assign(*rhs);
    }
}

impl Shr<u32> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
            w: self.w.shr(rhs),
        }
    }
}

impl Shr<&u32> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &u32) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&u32> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &u32) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<u32> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: u32) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<u32> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: u32) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&u32> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: &u32) {
        self.shr_assign(*rhs);
    }
}

impl Shl<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
            z: self.z.shl(rhs),
            w: self.w.shl(rhs),
        }
    }
}

impl Shl<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &u64) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &u64) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: u64) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<u64> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: u64) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&u64> for U64Vec4 {
    #[inline]
    fn shl_assign(&mut self, rhs: &u64) {
        self.shl_assign(*rhs);
    }
}

impl Shr<u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
            z: self.z.shr(rhs),
            w: self.w.shr(rhs),
        }
    }
}

impl Shr<&u64> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &u64) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &u64) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<u64> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: u64) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<u64> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: u64) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&u64> for U64Vec4 {
    #[inline]
    fn shr_assign(&mut self, rhs: &u64) {
        self.shr_assign(*rhs);
    }
}

impl Shl<IVec4> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: IVec4) -> Self {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
            z: self.z.shl(rhs.z),
            w: self.w.shl(rhs.w),
        }
    }
}

impl Shl<&IVec4> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &IVec4) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&IVec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &IVec4) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<IVec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: IVec4) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl Shr<IVec4> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: IVec4) -> Self {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
            z: self.z.shr(rhs.z),
            w: self.w.shr(rhs.w),
        }
    }
}

impl Shr<&IVec4> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &IVec4) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&IVec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &IVec4) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<IVec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: IVec4) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl Shl<UVec4> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: UVec4) -> Self {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
            z: self.z.shl(rhs.z),
            w: self.w.shl(rhs.w),
        }
    }
}

impl Shl<&UVec4> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &UVec4) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&UVec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: &UVec4) -> U64Vec4 {
        (*self).shl(*rhs)
    }
}

impl Shl<UVec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shl(self, rhs: UVec4) -> U64Vec4 {
        (*self).shl(rhs)
    }
}

impl Shr<UVec4> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: UVec4) -> Self {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
            z: self.z.shr(rhs.z),
            w: self.w.shr(rhs.w),
        }
    }
}

impl Shr<&UVec4> for U64Vec4 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &UVec4) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&UVec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: &UVec4) -> U64Vec4 {
        (*self).shr(*rhs)
    }
}

impl Shr<UVec4> for &U64Vec4 {
    type Output = U64Vec4;
    #[inline]
    fn shr(self, rhs: UVec4) -> U64Vec4 {
        (*self).shr(rhs)
    }
}

impl Index<usize> for U64Vec4 {
    type Output = u64;
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

impl IndexMut<usize> for U64Vec4 {
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

impl fmt::Display for U64Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Debug for U64Vec4 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(U64Vec4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl From<[u64; 4]> for U64Vec4 {
    #[inline]
    fn from(a: [u64; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<U64Vec4> for [u64; 4] {
    #[inline]
    fn from(v: U64Vec4) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<(u64, u64, u64, u64)> for U64Vec4 {
    #[inline]
    fn from(t: (u64, u64, u64, u64)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<U64Vec4> for (u64, u64, u64, u64) {
    #[inline]
    fn from(v: U64Vec4) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}

impl From<(U64Vec3, u64)> for U64Vec4 {
    #[inline]
    fn from((v, w): (U64Vec3, u64)) -> Self {
        Self::new(v.x, v.y, v.z, w)
    }
}

impl From<(u64, U64Vec3)> for U64Vec4 {
    #[inline]
    fn from((x, v): (u64, U64Vec3)) -> Self {
        Self::new(x, v.x, v.y, v.z)
    }
}

impl From<(U64Vec2, u64, u64)> for U64Vec4 {
    #[inline]
    fn from((v, z, w): (U64Vec2, u64, u64)) -> Self {
        Self::new(v.x, v.y, z, w)
    }
}

impl From<(U64Vec2, U64Vec2)> for U64Vec4 {
    #[inline]
    fn from((v, u): (U64Vec2, U64Vec2)) -> Self {
        Self::new(v.x, v.y, u.x, u.y)
    }
}

impl From<U8Vec4> for U64Vec4 {
    #[inline]
    fn from(v: U8Vec4) -> Self {
        Self::new(
            u64::from(v.x),
            u64::from(v.y),
            u64::from(v.z),
            u64::from(v.w),
        )
    }
}

impl From<U16Vec4> for U64Vec4 {
    #[inline]
    fn from(v: U16Vec4) -> Self {
        Self::new(
            u64::from(v.x),
            u64::from(v.y),
            u64::from(v.z),
            u64::from(v.w),
        )
    }
}

impl From<UVec4> for U64Vec4 {
    #[inline]
    fn from(v: UVec4) -> Self {
        Self::new(
            u64::from(v.x),
            u64::from(v.y),
            u64::from(v.z),
            u64::from(v.w),
        )
    }
}

impl TryFrom<I8Vec4> for U64Vec4 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I8Vec4) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u64::try_from(v.x)?,
            u64::try_from(v.y)?,
            u64::try_from(v.z)?,
            u64::try_from(v.w)?,
        ))
    }
}

impl TryFrom<I16Vec4> for U64Vec4 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I16Vec4) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u64::try_from(v.x)?,
            u64::try_from(v.y)?,
            u64::try_from(v.z)?,
            u64::try_from(v.w)?,
        ))
    }
}

impl TryFrom<IVec4> for U64Vec4 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: IVec4) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u64::try_from(v.x)?,
            u64::try_from(v.y)?,
            u64::try_from(v.z)?,
            u64::try_from(v.w)?,
        ))
    }
}

impl TryFrom<I64Vec4> for U64Vec4 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I64Vec4) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u64::try_from(v.x)?,
            u64::try_from(v.y)?,
            u64::try_from(v.z)?,
            u64::try_from(v.w)?,
        ))
    }
}

impl TryFrom<USizeVec4> for U64Vec4 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: USizeVec4) -> Result<Self, Self::Error> {
        Ok(Self::new(
            u64::try_from(v.x)?,
            u64::try_from(v.y)?,
            u64::try_from(v.z)?,
            u64::try_from(v.w)?,
        ))
    }
}

impl From<BVec4> for U64Vec4 {
    #[inline]
    fn from(v: BVec4) -> Self {
        Self::new(
            u64::from(v.x),
            u64::from(v.y),
            u64::from(v.z),
            u64::from(v.w),
        )
    }
}

#[cfg(not(feature = "scalar-math"))]
impl From<BVec4A> for U64Vec4 {
    #[inline]
    fn from(v: BVec4A) -> Self {
        let bool_array: [bool; 4] = v.into();
        Self::new(
            u64::from(bool_array[0]),
            u64::from(bool_array[1]),
            u64::from(bool_array[2]),
            u64::from(bool_array[3]),
        )
    }
}
