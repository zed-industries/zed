// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{BVec2, I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec3, U64Vec2, U8Vec2, UVec2};

use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

/// Creates a 2-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn u16vec2(x: u16, y: u16) -> U16Vec2 {
    U16Vec2::new(x, y)
}

/// A 2-dimensional vector.
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "cuda", repr(align(4)))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
pub struct U16Vec2 {
    pub x: u16,
    pub y: u16,
}

impl U16Vec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// All ones.
    pub const ONE: Self = Self::splat(1);

    /// All `u16::MIN`.
    pub const MIN: Self = Self::splat(u16::MIN);

    /// All `u16::MAX`.
    pub const MAX: Self = Self::splat(u16::MAX);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1, 0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0, 1);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: u16) -> Self {
        Self { x: v, y: v }
    }

    /// Returns a vector containing each element of `self` modified by a mapping function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(u16) -> u16,
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
    pub const fn from_array(a: [u16; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// `[x, y]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [u16; 2] {
        [self.x, self.y]
    }

    /// Creates a vector from the first 2 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[u16]) -> Self {
        assert!(slice.len() >= 2);
        Self::new(slice[0], slice[1])
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [u16]) {
        slice[..2].copy_from_slice(&self.to_array());
    }

    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline]
    #[must_use]
    pub const fn extend(self, z: u16) -> U16Vec3 {
        U16Vec3::new(self.x, self.y, z)
    }

    /// Creates a 2D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }

    /// Creates a 2D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: u16) -> Self {
        self.y = y;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> u16 {
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
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
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
        }
    }

    /// Component-wise clamping of values, similar to [`u16::clamp`].
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
    pub fn min_element(self) -> u16 {
        self.x.min(self.y)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> u16 {
        self.x.max(self.y)
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> u16 {
        self.x + self.y
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> u16 {
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
    pub fn length_squared(self) -> u16 {
        self.dot(self)
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

    /// Casts all elements of `self` to `u8`.
    #[inline]
    #[must_use]
    pub fn as_u8vec2(&self) -> crate::U8Vec2 {
        crate::U8Vec2::new(self.x as u8, self.y as u8)
    }

    /// Casts all elements of `self` to `i16`.
    #[inline]
    #[must_use]
    pub fn as_i16vec2(&self) -> crate::I16Vec2 {
        crate::I16Vec2::new(self.x as i16, self.y as i16)
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
    /// In other words this computes `[self.x.wrapping_add_signed(rhs.x), self.y.wrapping_add_signed(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_add_signed(self, rhs: I16Vec2) -> Self {
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
    pub const fn saturating_add_signed(self, rhs: I16Vec2) -> Self {
        Self {
            x: self.x.saturating_add_signed(rhs.x),
            y: self.y.saturating_add_signed(rhs.y),
        }
    }
}

impl Default for U16Vec2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div<U16Vec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl Div<&U16Vec2> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: &U16Vec2) -> U16Vec2 {
        self.div(*rhs)
    }
}

impl Div<&U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).div(rhs)
    }
}

impl DivAssign<U16Vec2> for U16Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl DivAssign<&Self> for U16Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs)
    }
}

impl Div<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: u16) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl Div<&u16> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: &u16) -> U16Vec2 {
        self.div(*rhs)
    }
}

impl Div<&u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: &u16) -> U16Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: u16) -> U16Vec2 {
        (*self).div(rhs)
    }
}

impl DivAssign<u16> for U16Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: u16) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl DivAssign<&u16> for U16Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &u16) {
        self.div_assign(*rhs)
    }
}

impl Div<U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: U16Vec2) -> U16Vec2 {
        U16Vec2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Div<&U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: &U16Vec2) -> U16Vec2 {
        self.div(*rhs)
    }
}

impl Div<&U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn div(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).div(rhs)
    }
}

impl Mul<U16Vec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Mul<&U16Vec2> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: &U16Vec2) -> U16Vec2 {
        self.mul(*rhs)
    }
}

impl Mul<&U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).mul(rhs)
    }
}

impl MulAssign<U16Vec2> for U16Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl MulAssign<&Self> for U16Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs)
    }
}

impl Mul<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: u16) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Mul<&u16> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: &u16) -> U16Vec2 {
        self.mul(*rhs)
    }
}

impl Mul<&u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: &u16) -> U16Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: u16) -> U16Vec2 {
        (*self).mul(rhs)
    }
}

impl MulAssign<u16> for U16Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: u16) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl MulAssign<&u16> for U16Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &u16) {
        self.mul_assign(*rhs)
    }
}

impl Mul<U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: U16Vec2) -> U16Vec2 {
        U16Vec2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Mul<&U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: &U16Vec2) -> U16Vec2 {
        self.mul(*rhs)
    }
}

impl Mul<&U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn mul(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).mul(rhs)
    }
}

impl Add<U16Vec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Add<&U16Vec2> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: &U16Vec2) -> U16Vec2 {
        self.add(*rhs)
    }
}

impl Add<&U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).add(rhs)
    }
}

impl AddAssign<U16Vec2> for U16Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl AddAssign<&Self> for U16Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs)
    }
}

impl Add<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u16) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Add<&u16> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: &u16) -> U16Vec2 {
        self.add(*rhs)
    }
}

impl Add<&u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: &u16) -> U16Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: u16) -> U16Vec2 {
        (*self).add(rhs)
    }
}

impl AddAssign<u16> for U16Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: u16) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl AddAssign<&u16> for U16Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &u16) {
        self.add_assign(*rhs)
    }
}

impl Add<U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: U16Vec2) -> U16Vec2 {
        U16Vec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Add<&U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: &U16Vec2) -> U16Vec2 {
        self.add(*rhs)
    }
}

impl Add<&U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn add(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).add(rhs)
    }
}

impl Sub<U16Vec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Sub<&U16Vec2> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: &U16Vec2) -> U16Vec2 {
        self.sub(*rhs)
    }
}

impl Sub<&U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).sub(rhs)
    }
}

impl SubAssign<U16Vec2> for U16Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: U16Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl SubAssign<&Self> for U16Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs)
    }
}

impl Sub<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u16) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Sub<&u16> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: &u16) -> U16Vec2 {
        self.sub(*rhs)
    }
}

impl Sub<&u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: &u16) -> U16Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: u16) -> U16Vec2 {
        (*self).sub(rhs)
    }
}

impl SubAssign<u16> for U16Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: u16) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl SubAssign<&u16> for U16Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &u16) {
        self.sub_assign(*rhs)
    }
}

impl Sub<U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: U16Vec2) -> U16Vec2 {
        U16Vec2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Sub<&U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: &U16Vec2) -> U16Vec2 {
        self.sub(*rhs)
    }
}

impl Sub<&U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn sub(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).sub(rhs)
    }
}

impl Rem<U16Vec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl Rem<&U16Vec2> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: &U16Vec2) -> U16Vec2 {
        self.rem(*rhs)
    }
}

impl Rem<&U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<U16Vec2> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).rem(rhs)
    }
}

impl RemAssign<U16Vec2> for U16Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl RemAssign<&Self> for U16Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs)
    }
}

impl Rem<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: u16) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl Rem<&u16> for U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: &u16) -> U16Vec2 {
        self.rem(*rhs)
    }
}

impl Rem<&u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: &u16) -> U16Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<u16> for &U16Vec2 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: u16) -> U16Vec2 {
        (*self).rem(rhs)
    }
}

impl RemAssign<u16> for U16Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: u16) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
    }
}

impl RemAssign<&u16> for U16Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &u16) {
        self.rem_assign(*rhs)
    }
}

impl Rem<U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: U16Vec2) -> U16Vec2 {
        U16Vec2 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
        }
    }
}

impl Rem<&U16Vec2> for u16 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: &U16Vec2) -> U16Vec2 {
        self.rem(*rhs)
    }
}

impl Rem<&U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: &U16Vec2) -> U16Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<U16Vec2> for &u16 {
    type Output = U16Vec2;
    #[inline]
    fn rem(self, rhs: U16Vec2) -> U16Vec2 {
        (*self).rem(rhs)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[u16; 2]> for U16Vec2 {
    #[inline]
    fn as_ref(&self) -> &[u16; 2] {
        unsafe { &*(self as *const U16Vec2 as *const [u16; 2]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[u16; 2]> for U16Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u16; 2] {
        unsafe { &mut *(self as *mut U16Vec2 as *mut [u16; 2]) }
    }
}

impl Sum for U16Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for U16Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for U16Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for U16Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Not for U16Vec2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            x: self.x.not(),
            y: self.y.not(),
        }
    }
}

impl BitAnd for U16Vec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
        }
    }
}

impl BitOr for U16Vec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
        }
    }
}

impl BitXor for U16Vec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
        }
    }
}

impl BitAnd<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.bitand(rhs),
            y: self.y.bitand(rhs),
        }
    }
}

impl BitOr<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.bitor(rhs),
            y: self.y.bitor(rhs),
        }
    }
}

impl BitXor<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs),
            y: self.y.bitxor(rhs),
        }
    }
}

impl Shl<i8> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i8> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i32> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i32> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i64> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i64> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u8> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u8> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u16> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u32> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u32> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u64> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u64> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<crate::IVec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: crate::IVec2) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shr<crate::IVec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: crate::IVec2) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl Shl<crate::UVec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: crate::UVec2) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shr<crate::UVec2> for U16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: crate::UVec2) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl Index<usize> for U16Vec2 {
    type Output = u16;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for U16Vec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for U16Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl fmt::Debug for U16Vec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(U16Vec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<[u16; 2]> for U16Vec2 {
    #[inline]
    fn from(a: [u16; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<U16Vec2> for [u16; 2] {
    #[inline]
    fn from(v: U16Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(u16, u16)> for U16Vec2 {
    #[inline]
    fn from(t: (u16, u16)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<U16Vec2> for (u16, u16) {
    #[inline]
    fn from(v: U16Vec2) -> Self {
        (v.x, v.y)
    }
}

impl From<U8Vec2> for U16Vec2 {
    #[inline]
    fn from(v: U8Vec2) -> Self {
        Self::new(u16::from(v.x), u16::from(v.y))
    }
}

impl TryFrom<I8Vec2> for U16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I8Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u16::try_from(v.x)?, u16::try_from(v.y)?))
    }
}

impl TryFrom<I16Vec2> for U16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I16Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u16::try_from(v.x)?, u16::try_from(v.y)?))
    }
}

impl TryFrom<IVec2> for U16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: IVec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u16::try_from(v.x)?, u16::try_from(v.y)?))
    }
}

impl TryFrom<UVec2> for U16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: UVec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u16::try_from(v.x)?, u16::try_from(v.y)?))
    }
}

impl TryFrom<I64Vec2> for U16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I64Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u16::try_from(v.x)?, u16::try_from(v.y)?))
    }
}

impl TryFrom<U64Vec2> for U16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: U64Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(u16::try_from(v.x)?, u16::try_from(v.y)?))
    }
}

impl From<BVec2> for U16Vec2 {
    #[inline]
    fn from(v: BVec2) -> Self {
        Self::new(u16::from(v.x), u16::from(v.y))
    }
}
