// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{BVec2, I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec3, USizeVec2, UVec2};

use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

#[cfg(feature = "zerocopy")]
use zerocopy_derive::*;

/// Creates a 2-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn u8vec2(x: u8, y: u8) -> U8Vec2 {
    U8Vec2::new(x, y)
}

/// A 2-dimensional vector.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[cfg_attr(
    feature = "zerocopy",
    derive(FromBytes, Immutable, IntoBytes, KnownLayout)
)]
#[cfg_attr(feature = "cuda", repr(align(2)))]
#[repr(C)]
#[cfg_attr(target_arch = "spirv", rust_gpu::vector::v1)]
pub struct U8Vec2 {
    pub x: u8,
    pub y: u8,
}

impl U8Vec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// All ones.
    pub const ONE: Self = Self::splat(1);

    /// All `u8::MIN`.
    pub const MIN: Self = Self::splat(u8::MIN);

    /// All `u8::MAX`.
    pub const MAX: Self = Self::splat(u8::MAX);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1, 0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0, 1);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: u8) -> Self {
        Self { x: v, y: v }
    }

    /// Returns a vector containing each element of `self` modified by a mapping function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(u8) -> u8,
    {
        Self::new(f(self.x), f(self.y))
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    #[must_use]
    pub fn select(mask: BVec2, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.test(0) { if_true.x } else { if_false.x },
            y: if mask.test(1) { if_true.y } else { if_false.y },
        }
    }

    /// Creates a new vector from an array.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [u8; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// Converts `self` to `[x, y]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [u8; 2] {
        [self.x, self.y]
    }

    /// Creates a vector from the first 2 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() >= 2);
        Self::new(slice[0], slice[1])
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [u8]) {
        slice[..2].copy_from_slice(&self.to_array());
    }

    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline]
    #[must_use]
    pub const fn extend(self, z: u8) -> U8Vec3 {
        U8Vec3::new(self.x, self.y, z)
    }

    /// Creates a 2D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: u8) -> Self {
        self.x = x;
        self
    }

    /// Creates a 2D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: u8) -> Self {
        self.y = y;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> u8 {
        (self.x * rhs.x) + (self.y * rhs.y)
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
        }
    }

    /// Component-wise clamping of values, similar to [`u8::clamp`].
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
    pub fn min_element(self) -> u8 {
        let min = |a, b| if a < b { a } else { b };
        min(self.x, self.y)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> u8 {
        let max = |a, b| if a > b { a } else { b };
        max(self.x, self.y)
    }

    /// Returns the index of the first minimum element of `self`.
    #[doc(alias = "argmin")]
    #[inline]
    #[must_use]
    pub fn min_position(self) -> usize {
        if self.x <= self.y {
            0
        } else {
            1
        }
    }

    /// Returns the index of the first maximum element of `self`.
    #[doc(alias = "argmax")]
    #[inline]
    #[must_use]
    pub fn max_position(self) -> usize {
        if self.x >= self.y {
            0
        } else {
            1
        }
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> u8 {
        self.x + self.y
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> u8 {
        self.x * self.y
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpeq(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.eq(&rhs.x), self.y.eq(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpne(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.ne(&rhs.x), self.y.ne(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpge(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.ge(&rhs.x), self.y.ge(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmpgt(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.gt(&rhs.x), self.y.gt(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmple(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.le(&rhs.x), self.y.le(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    #[must_use]
    pub fn cmplt(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.lt(&rhs.x), self.y.lt(&rhs.y))
    }

    /// Computes the squared length of `self`.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> u8 {
        self.dot(self)
    }

    /// Computes the [manhattan distance] between two points.
    ///
    /// # Overflow
    /// This method may overflow if the result is greater than [`u8::MAX`].
    ///
    /// See also [`checked_manhattan_distance`][U8Vec2::checked_manhattan_distance].
    ///
    /// [manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    #[inline]
    #[must_use]
    pub fn manhattan_distance(self, rhs: Self) -> u8 {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }

    /// Computes the [manhattan distance] between two points.
    ///
    /// This will returns [`None`] if the result is greater than [`u8::MAX`].
    ///
    /// [manhattan distance]: https://en.wikipedia.org/wiki/Taxicab_geometry
    #[inline]
    #[must_use]
    pub fn checked_manhattan_distance(self, rhs: Self) -> Option<u8> {
        let d = self.x.abs_diff(rhs.x);
        d.checked_add(self.y.abs_diff(rhs.y))
    }

    /// Computes the [chebyshev distance] between two points.
    ///
    /// [chebyshev distance]: https://en.wikipedia.org/wiki/Chebyshev_distance
    #[inline]
    #[must_use]
    pub fn chebyshev_distance(self, rhs: Self) -> u8 {
        // Note: the compiler will eventually optimize out the loop
        [self.x.abs_diff(rhs.x), self.y.abs_diff(rhs.y)]
            .into_iter()
            .max()
            .unwrap()
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[must_use]
    pub fn as_vec2(&self) -> crate::Vec2 {
        crate::Vec2::new(self.x as f32, self.y as f32)
    }

    /// Casts all elements of `self` to `f64`.
    #[inline]
    #[must_use]
    pub fn as_dvec2(&self) -> crate::DVec2 {
        crate::DVec2::new(self.x as f64, self.y as f64)
    }

    /// Casts all elements of `self` to `i8`.
    #[inline]
    #[must_use]
    pub fn as_i8vec2(&self) -> crate::I8Vec2 {
        crate::I8Vec2::new(self.x as i8, self.y as i8)
    }

    /// Casts all elements of `self` to `i16`.
    #[inline]
    #[must_use]
    pub fn as_i16vec2(&self) -> crate::I16Vec2 {
        crate::I16Vec2::new(self.x as i16, self.y as i16)
    }

    /// Casts all elements of `self` to `u16`.
    #[inline]
    #[must_use]
    pub fn as_u16vec2(&self) -> crate::U16Vec2 {
        crate::U16Vec2::new(self.x as u16, self.y as u16)
    }

    /// Casts all elements of `self` to `i32`.
    #[inline]
    #[must_use]
    pub fn as_ivec2(&self) -> crate::IVec2 {
        crate::IVec2::new(self.x as i32, self.y as i32)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline]
    #[must_use]
    pub fn as_uvec2(&self) -> crate::UVec2 {
        crate::UVec2::new(self.x as u32, self.y as u32)
    }

    /// Casts all elements of `self` to `i64`.
    #[inline]
    #[must_use]
    pub fn as_i64vec2(&self) -> crate::I64Vec2 {
        crate::I64Vec2::new(self.x as i64, self.y as i64)
    }

    /// Casts all elements of `self` to `u64`.
    #[inline]
    #[must_use]
    pub fn as_u64vec2(&self) -> crate::U64Vec2 {
        crate::U64Vec2::new(self.x as u64, self.y as u64)
    }

    /// Casts all elements of `self` to `usize`.
    #[inline]
    #[must_use]
    pub fn as_usizevec2(&self) -> crate::USizeVec2 {
        crate::USizeVec2::new(self.x as usize, self.y as usize)
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

        Some(Self { x, y })
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

        Some(Self { x, y })
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

        Some(Self { x, y })
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

        Some(Self { x, y })
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
        }
    }

    /// Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.
    #[inline]
    #[must_use]
    pub const fn checked_add_signed(self, rhs: I8Vec2) -> Option<Self> {
        let x = match self.x.checked_add_signed(rhs.x) {
            Some(v) => v,
            None => return None,
        };
        let y = match self.y.checked_add_signed(rhs.y) {
            Some(v) => v,
            None => return None,
        };

        Some(Self { x, y })
    }

    /// Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_add_signed(rhs.x), self.y.wrapping_add_signed(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_add_signed(self, rhs: I8Vec2) -> Self {
        Self {
            x: self.x.wrapping_add_signed(rhs.x),
            y: self.y.wrapping_add_signed(rhs.y),
        }
    }

    /// Returns a vector containing the saturating addition of `self` and signed vector `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_add_signed(rhs.x), self.y.saturating_add_signed(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_add_signed(self, rhs: I8Vec2) -> Self {
        Self {
            x: self.x.saturating_add_signed(rhs.x),
            y: self.y.saturating_add_signed(rhs.y),
        }
    }
}

impl Default for U8Vec2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div for U8Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl Div<&Self> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &Self) -> Self {
        self.div(*rhs)
    }
}

impl Div<&U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn div(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn div(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).div(rhs)
    }
}

impl DivAssign for U8Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl DivAssign<&Self> for U8Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs);
    }
}

impl Div<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: u8) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl Div<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &u8) -> Self {
        self.div(*rhs)
    }
}

impl Div<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn div(self, rhs: &u8) -> U8Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn div(self, rhs: u8) -> U8Vec2 {
        (*self).div(rhs)
    }
}

impl DivAssign<u8> for U8Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: u8) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl DivAssign<&u8> for U8Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &u8) {
        self.div_assign(*rhs);
    }
}

impl Div<U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn div(self, rhs: U8Vec2) -> U8Vec2 {
        U8Vec2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Div<&U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn div(self, rhs: &U8Vec2) -> U8Vec2 {
        self.div(*rhs)
    }
}

impl Div<&U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn div(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn div(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).div(rhs)
    }
}

impl Mul for U8Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Mul<&Self> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn mul(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn mul(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).mul(rhs)
    }
}

impl MulAssign for U8Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl MulAssign<&Self> for U8Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Mul<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: u8) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Mul<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &u8) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn mul(self, rhs: &u8) -> U8Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn mul(self, rhs: u8) -> U8Vec2 {
        (*self).mul(rhs)
    }
}

impl MulAssign<u8> for U8Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: u8) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl MulAssign<&u8> for U8Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &u8) {
        self.mul_assign(*rhs);
    }
}

impl Mul<U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn mul(self, rhs: U8Vec2) -> U8Vec2 {
        U8Vec2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Mul<&U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn mul(self, rhs: &U8Vec2) -> U8Vec2 {
        self.mul(*rhs)
    }
}

impl Mul<&U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn mul(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn mul(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).mul(rhs)
    }
}

impl Add for U8Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Add<&Self> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl Add<&U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn add(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn add(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).add(rhs)
    }
}

impl AddAssign for U8Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl AddAssign<&Self> for U8Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl Add<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u8) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Add<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &u8) -> Self {
        self.add(*rhs)
    }
}

impl Add<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn add(self, rhs: &u8) -> U8Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn add(self, rhs: u8) -> U8Vec2 {
        (*self).add(rhs)
    }
}

impl AddAssign<u8> for U8Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: u8) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl AddAssign<&u8> for U8Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &u8) {
        self.add_assign(*rhs);
    }
}

impl Add<U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn add(self, rhs: U8Vec2) -> U8Vec2 {
        U8Vec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Add<&U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn add(self, rhs: &U8Vec2) -> U8Vec2 {
        self.add(*rhs)
    }
}

impl Add<&U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn add(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn add(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).add(rhs)
    }
}

impl Sub for U8Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Sub<&Self> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn sub(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn sub(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).sub(rhs)
    }
}

impl SubAssign for U8Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl SubAssign<&Self> for U8Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

impl Sub<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u8) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Sub<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &u8) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn sub(self, rhs: &u8) -> U8Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn sub(self, rhs: u8) -> U8Vec2 {
        (*self).sub(rhs)
    }
}

impl SubAssign<u8> for U8Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: u8) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl SubAssign<&u8> for U8Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &u8) {
        self.sub_assign(*rhs);
    }
}

impl Sub<U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn sub(self, rhs: U8Vec2) -> U8Vec2 {
        U8Vec2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Sub<&U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn sub(self, rhs: &U8Vec2) -> U8Vec2 {
        self.sub(*rhs)
    }
}

impl Sub<&U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn sub(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn sub(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).sub(rhs)
    }
}

impl Rem for U8Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl Rem<&Self> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &Self) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn rem(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn rem(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).rem(rhs)
    }
}

impl RemAssign for U8Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl RemAssign<&Self> for U8Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs);
    }
}

impl Rem<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: u8) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl Rem<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &u8) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn rem(self, rhs: &u8) -> U8Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn rem(self, rhs: u8) -> U8Vec2 {
        (*self).rem(rhs)
    }
}

impl RemAssign<u8> for U8Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: u8) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
    }
}

impl RemAssign<&u8> for U8Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &u8) {
        self.rem_assign(*rhs);
    }
}

impl Rem<U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn rem(self, rhs: U8Vec2) -> U8Vec2 {
        U8Vec2 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
        }
    }
}

impl Rem<&U8Vec2> for u8 {
    type Output = U8Vec2;
    #[inline]
    fn rem(self, rhs: &U8Vec2) -> U8Vec2 {
        self.rem(*rhs)
    }
}

impl Rem<&U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn rem(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<U8Vec2> for &u8 {
    type Output = U8Vec2;
    #[inline]
    fn rem(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).rem(rhs)
    }
}

impl AsRef<[u8; 2]> for U8Vec2 {
    #[inline]
    fn as_ref(&self) -> &[u8; 2] {
        unsafe { &*(self as *const Self as *const [u8; 2]) }
    }
}

impl AsMut<[u8; 2]> for U8Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; 2] {
        unsafe { &mut *(self as *mut Self as *mut [u8; 2]) }
    }
}

impl Sum for U8Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for U8Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for U8Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for U8Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Not for U8Vec2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self {
            x: self.x.not(),
            y: self.y.not(),
        }
    }
}

impl Not for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn not(self) -> U8Vec2 {
        (*self).not()
    }
}

impl BitAnd for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
        }
    }
}

impl BitAnd<&Self> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: &Self) -> Self {
        self.bitand(*rhs)
    }
}

impl BitAnd<&U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitand(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).bitand(*rhs)
    }
}

impl BitAnd<U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitand(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).bitand(rhs)
    }
}

impl BitAndAssign for U8Vec2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitAndAssign<&Self> for U8Vec2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: &Self) {
        self.bitand_assign(*rhs);
    }
}

impl BitOr for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
        }
    }
}

impl BitOr<&Self> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: &Self) -> Self {
        self.bitor(*rhs)
    }
}

impl BitOr<&U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitor(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).bitor(*rhs)
    }
}

impl BitOr<U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitor(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).bitor(rhs)
    }
}

impl BitOrAssign for U8Vec2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitOrAssign<&Self> for U8Vec2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: &Self) {
        self.bitor_assign(*rhs);
    }
}

impl BitXor for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
        }
    }
}

impl BitXor<&Self> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: &Self) -> Self {
        self.bitxor(*rhs)
    }
}

impl BitXor<&U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitxor(self, rhs: &U8Vec2) -> U8Vec2 {
        (*self).bitxor(*rhs)
    }
}

impl BitXor<U8Vec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitxor(self, rhs: U8Vec2) -> U8Vec2 {
        (*self).bitxor(rhs)
    }
}

impl BitXorAssign for U8Vec2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}

impl BitXorAssign<&Self> for U8Vec2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Self) {
        self.bitxor_assign(*rhs);
    }
}

impl BitAnd<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.bitand(rhs),
            y: self.y.bitand(rhs),
        }
    }
}

impl BitAnd<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: &u8) -> Self {
        self.bitand(*rhs)
    }
}

impl BitAnd<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitand(self, rhs: &u8) -> U8Vec2 {
        (*self).bitand(*rhs)
    }
}

impl BitAnd<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitand(self, rhs: u8) -> U8Vec2 {
        (*self).bitand(rhs)
    }
}

impl BitAndAssign<u8> for U8Vec2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: u8) {
        *self = self.bitand(rhs);
    }
}

impl BitAndAssign<&u8> for U8Vec2 {
    #[inline]
    fn bitand_assign(&mut self, rhs: &u8) {
        self.bitand_assign(*rhs);
    }
}

impl BitOr<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.bitor(rhs),
            y: self.y.bitor(rhs),
        }
    }
}

impl BitOr<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: &u8) -> Self {
        self.bitor(*rhs)
    }
}

impl BitOr<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitor(self, rhs: &u8) -> U8Vec2 {
        (*self).bitor(*rhs)
    }
}

impl BitOr<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitor(self, rhs: u8) -> U8Vec2 {
        (*self).bitor(rhs)
    }
}

impl BitOrAssign<u8> for U8Vec2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: u8) {
        *self = self.bitor(rhs);
    }
}

impl BitOrAssign<&u8> for U8Vec2 {
    #[inline]
    fn bitor_assign(&mut self, rhs: &u8) {
        self.bitor_assign(*rhs);
    }
}

impl BitXor<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs),
            y: self.y.bitxor(rhs),
        }
    }
}

impl BitXor<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: &u8) -> Self {
        self.bitxor(*rhs)
    }
}

impl BitXor<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitxor(self, rhs: &u8) -> U8Vec2 {
        (*self).bitxor(*rhs)
    }
}

impl BitXor<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn bitxor(self, rhs: u8) -> U8Vec2 {
        (*self).bitxor(rhs)
    }
}

impl BitXorAssign<u8> for U8Vec2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: u8) {
        *self = self.bitxor(rhs);
    }
}

impl BitXorAssign<&u8> for U8Vec2 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &u8) {
        self.bitxor_assign(*rhs);
    }
}

impl Shl<i8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shl<&i8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &i8) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&i8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &i8) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<i8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: i8) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<i8> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: i8) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&i8> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: &i8) {
        self.shl_assign(*rhs);
    }
}

impl Shr<i8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shr<&i8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &i8) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&i8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &i8) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<i8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: i8) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<i8> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: i8) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&i8> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: &i8) {
        self.shr_assign(*rhs);
    }
}

impl Shl<i16> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shl<&i16> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &i16) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&i16> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &i16) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<i16> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: i16) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<i16> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: i16) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&i16> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: &i16) {
        self.shl_assign(*rhs);
    }
}

impl Shr<i16> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shr<&i16> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &i16) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&i16> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &i16) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<i16> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: i16) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<i16> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: i16) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&i16> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: &i16) {
        self.shr_assign(*rhs);
    }
}

impl Shl<i32> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shl<&i32> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &i32) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&i32> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &i32) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<i32> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: i32) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<i32> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: i32) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&i32> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: &i32) {
        self.shl_assign(*rhs);
    }
}

impl Shr<i32> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shr<&i32> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &i32) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&i32> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &i32) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<i32> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: i32) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<i32> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: i32) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&i32> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: &i32) {
        self.shr_assign(*rhs);
    }
}

impl Shl<i64> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shl<&i64> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &i64) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&i64> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &i64) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<i64> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: i64) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<i64> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: i64) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&i64> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: &i64) {
        self.shl_assign(*rhs);
    }
}

impl Shr<i64> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shr<&i64> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &i64) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&i64> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &i64) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<i64> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: i64) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<i64> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: i64) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&i64> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: &i64) {
        self.shr_assign(*rhs);
    }
}

impl Shl<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shl<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &u8) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &u8) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: u8) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<u8> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: u8) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&u8> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: &u8) {
        self.shl_assign(*rhs);
    }
}

impl Shr<u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shr<&u8> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &u8) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &u8) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<u8> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: u8) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<u8> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: u8) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&u8> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: &u8) {
        self.shr_assign(*rhs);
    }
}

impl Shl<u16> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shl<&u16> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &u16) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&u16> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &u16) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<u16> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: u16) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<u16> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: u16) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&u16> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: &u16) {
        self.shl_assign(*rhs);
    }
}

impl Shr<u16> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shr<&u16> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &u16) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&u16> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &u16) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<u16> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: u16) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<u16> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: u16) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&u16> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: &u16) {
        self.shr_assign(*rhs);
    }
}

impl Shl<u32> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shl<&u32> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &u32) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&u32> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &u32) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<u32> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: u32) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<u32> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: u32) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&u32> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: &u32) {
        self.shl_assign(*rhs);
    }
}

impl Shr<u32> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shr<&u32> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &u32) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&u32> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &u32) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<u32> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: u32) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<u32> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: u32) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&u32> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: &u32) {
        self.shr_assign(*rhs);
    }
}

impl Shl<u64> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shl<&u64> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &u64) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&u64> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &u64) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<u64> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: u64) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl ShlAssign<u64> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: u64) {
        *self = self.shl(rhs);
    }
}

impl ShlAssign<&u64> for U8Vec2 {
    #[inline]
    fn shl_assign(&mut self, rhs: &u64) {
        self.shl_assign(*rhs);
    }
}

impl Shr<u64> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shr<&u64> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &u64) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&u64> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &u64) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<u64> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: u64) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl ShrAssign<u64> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: u64) {
        *self = self.shr(rhs);
    }
}

impl ShrAssign<&u64> for U8Vec2 {
    #[inline]
    fn shr_assign(&mut self, rhs: &u64) {
        self.shr_assign(*rhs);
    }
}

impl Shl<IVec2> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: IVec2) -> Self {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shl<&IVec2> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &IVec2) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&IVec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &IVec2) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<IVec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: IVec2) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl Shr<IVec2> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: IVec2) -> Self {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl Shr<&IVec2> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &IVec2) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&IVec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &IVec2) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<IVec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: IVec2) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl Shl<UVec2> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: UVec2) -> Self {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shl<&UVec2> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: &UVec2) -> Self {
        self.shl(*rhs)
    }
}

impl Shl<&UVec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: &UVec2) -> U8Vec2 {
        (*self).shl(*rhs)
    }
}

impl Shl<UVec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shl(self, rhs: UVec2) -> U8Vec2 {
        (*self).shl(rhs)
    }
}

impl Shr<UVec2> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: UVec2) -> Self {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl Shr<&UVec2> for U8Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: &UVec2) -> Self {
        self.shr(*rhs)
    }
}

impl Shr<&UVec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: &UVec2) -> U8Vec2 {
        (*self).shr(*rhs)
    }
}

impl Shr<UVec2> for &U8Vec2 {
    type Output = U8Vec2;
    #[inline]
    fn shr(self, rhs: UVec2) -> U8Vec2 {
        (*self).shr(rhs)
    }
}

impl Index<usize> for U8Vec2 {
    type Output = u8;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for U8Vec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for U8Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl fmt::Debug for U8Vec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(U8Vec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<[u8; 2]> for U8Vec2 {
    #[inline]
    fn from(a: [u8; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<U8Vec2> for [u8; 2] {
    #[inline]
    fn from(v: U8Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(u8, u8)> for U8Vec2 {
    #[inline]
    fn from(t: (u8, u8)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<U8Vec2> for (u8, u8) {
    #[inline]
    fn from(v: U8Vec2) -> Self {
        (v.x, v.y)
    }
}

impl TryFrom<I8Vec2> for U8Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I8Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u8::try_from(v.x)?, u8::try_from(v.y)?))
    }
}

impl TryFrom<I16Vec2> for U8Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I16Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u8::try_from(v.x)?, u8::try_from(v.y)?))
    }
}

impl TryFrom<U16Vec2> for U8Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: U16Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u8::try_from(v.x)?, u8::try_from(v.y)?))
    }
}

impl TryFrom<IVec2> for U8Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: IVec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u8::try_from(v.x)?, u8::try_from(v.y)?))
    }
}

impl TryFrom<UVec2> for U8Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: UVec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u8::try_from(v.x)?, u8::try_from(v.y)?))
    }
}

impl TryFrom<I64Vec2> for U8Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I64Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u8::try_from(v.x)?, u8::try_from(v.y)?))
    }
}

impl TryFrom<U64Vec2> for U8Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: U64Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u8::try_from(v.x)?, u8::try_from(v.y)?))
    }
}

impl TryFrom<USizeVec2> for U8Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: USizeVec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u8::try_from(v.x)?, u8::try_from(v.y)?))
    }
}

impl From<BVec2> for U8Vec2 {
    #[inline]
    fn from(v: BVec2) -> Self {
        Self::new(u8::from(v.x), u8::from(v.y))
    }
}
