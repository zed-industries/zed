// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{BVec2, I16Vec3, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec2, UVec2};

use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

/// Creates a 2-dimensional vector.
#[inline(always)]
#[must_use]
pub const fn i16vec2(x: i16, y: i16) -> I16Vec2 {
    I16Vec2::new(x, y)
}

/// A 2-dimensional vector.
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "cuda", repr(align(4)))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
pub struct I16Vec2 {
    pub x: i16,
    pub y: i16,
}

impl I16Vec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// All ones.
    pub const ONE: Self = Self::splat(1);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1);

    /// All `i16::MIN`.
    pub const MIN: Self = Self::splat(i16::MIN);

    /// All `i16::MAX`.
    pub const MAX: Self = Self::splat(i16::MAX);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(1, 0);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0, 1);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1, 0);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0, -1);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: i16) -> Self {
        Self { x: v, y: v }
    }

    /// Returns a vector containing each element of `self` modified by a mapping function `f`.
    #[inline]
    #[must_use]
    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(i16) -> i16,
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
    pub const fn from_array(a: [i16; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// `[x, y]`
    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [i16; 2] {
        [self.x, self.y]
    }

    /// Creates a vector from the first 2 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    #[must_use]
    pub const fn from_slice(slice: &[i16]) -> Self {
        assert!(slice.len() >= 2);
        Self::new(slice[0], slice[1])
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [i16]) {
        slice[..2].copy_from_slice(&self.to_array());
    }

    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline]
    #[must_use]
    pub const fn extend(self, z: i16) -> I16Vec3 {
        I16Vec3::new(self.x, self.y, z)
    }

    /// Creates a 2D vector from `self` with the given value of `x`.
    #[inline]
    #[must_use]
    pub fn with_x(mut self, x: i16) -> Self {
        self.x = x;
        self
    }

    /// Creates a 2D vector from `self` with the given value of `y`.
    #[inline]
    #[must_use]
    pub fn with_y(mut self, y: i16) -> Self {
        self.y = y;
        self
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> i16 {
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

    /// Component-wise clamping of values, similar to [`i16::clamp`].
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
    pub fn min_element(self) -> i16 {
        self.x.min(self.y)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> i16 {
        self.x.max(self.y)
    }

    /// Returns the sum of all elements of `self`.
    ///
    /// In other words, this computes `self.x + self.y + ..`.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> i16 {
        self.x + self.y
    }

    /// Returns the product of all elements of `self`.
    ///
    /// In other words, this computes `self.x * self.y * ..`.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> i16 {
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

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    ///  - `0` if the number is zero
    ///  - `1` if the number is positive
    ///  - `-1` if the number is negative
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    /// Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
    ///
    /// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    #[must_use]
    pub fn is_negative_bitmask(self) -> u32 {
        (self.x.is_negative() as u32) | ((self.y.is_negative() as u32) << 1)
    }

    /// Computes the squared length of `self`.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> i16 {
        self.dot(self)
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, rhs: Self) -> i16 {
        (self - rhs).length_squared()
    }

    /// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
    ///
    /// # Panics
    /// This function will panic if any `rhs` element is 0 or the division results in overflow.
    #[inline]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        Self::new(self.x.div_euclid(rhs.x), self.y.div_euclid(rhs.y))
    }

    /// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
    ///
    /// # Panics
    /// This function will panic if any `rhs` element is 0 or the division results in overflow.
    ///
    /// [Euclidean division]: i16::rem_euclid
    #[inline]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        Self::new(self.x.rem_euclid(rhs.x), self.y.rem_euclid(rhs.y))
    }

    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline]
    #[must_use]
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// The perpendicular dot product of `self` and `rhs`.
    /// Also known as the wedge product, 2D cross product, and determinant.
    #[doc(alias = "wedge")]
    #[doc(alias = "cross")]
    #[doc(alias = "determinant")]
    #[inline]
    #[must_use]
    pub fn perp_dot(self, rhs: Self) -> i16 {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    /// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    #[inline]
    #[must_use]
    pub fn rotate(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.y * rhs.x + self.x * rhs.y,
        }
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

    /// Returns a vector containing the wrapping addition of `self` and unsigned vector `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_add_unsigned(rhs.x), self.y.wrapping_add_unsigned(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_add_unsigned(self, rhs: U16Vec2) -> Self {
        Self {
            x: self.x.wrapping_add_unsigned(rhs.x),
            y: self.y.wrapping_add_unsigned(rhs.y),
        }
    }

    /// Returns a vector containing the wrapping subtraction of `self` and unsigned vector `rhs`.
    ///
    /// In other words this computes `[self.x.wrapping_sub_unsigned(rhs.x), self.y.wrapping_sub_unsigned(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn wrapping_sub_unsigned(self, rhs: U16Vec2) -> Self {
        Self {
            x: self.x.wrapping_sub_unsigned(rhs.x),
            y: self.y.wrapping_sub_unsigned(rhs.y),
        }
    }

    // Returns a vector containing the saturating addition of `self` and unsigned vector `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_add_unsigned(rhs.x), self.y.saturating_add_unsigned(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_add_unsigned(self, rhs: U16Vec2) -> Self {
        Self {
            x: self.x.saturating_add_unsigned(rhs.x),
            y: self.y.saturating_add_unsigned(rhs.y),
        }
    }

    /// Returns a vector containing the saturating subtraction of `self` and unsigned vector `rhs`.
    ///
    /// In other words this computes `[self.x.saturating_sub_unsigned(rhs.x), self.y.saturating_sub_unsigned(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub const fn saturating_sub_unsigned(self, rhs: U16Vec2) -> Self {
        Self {
            x: self.x.saturating_sub_unsigned(rhs.x),
            y: self.y.saturating_sub_unsigned(rhs.y),
        }
    }
}

impl Default for I16Vec2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div<I16Vec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl Div<&I16Vec2> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: &I16Vec2) -> I16Vec2 {
        self.div(*rhs)
    }
}

impl Div<&I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).div(rhs)
    }
}

impl DivAssign<I16Vec2> for I16Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl DivAssign<&Self> for I16Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs)
    }
}

impl Div<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i16) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl Div<&i16> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: &i16) -> I16Vec2 {
        self.div(*rhs)
    }
}

impl Div<&i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: &i16) -> I16Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: i16) -> I16Vec2 {
        (*self).div(rhs)
    }
}

impl DivAssign<i16> for I16Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: i16) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl DivAssign<&i16> for I16Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: &i16) {
        self.div_assign(*rhs)
    }
}

impl Div<I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: I16Vec2) -> I16Vec2 {
        I16Vec2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Div<&I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: &I16Vec2) -> I16Vec2 {
        self.div(*rhs)
    }
}

impl Div<&I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).div(*rhs)
    }
}

impl Div<I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn div(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).div(rhs)
    }
}

impl Mul<I16Vec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Mul<&I16Vec2> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: &I16Vec2) -> I16Vec2 {
        self.mul(*rhs)
    }
}

impl Mul<&I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).mul(rhs)
    }
}

impl MulAssign<I16Vec2> for I16Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl MulAssign<&Self> for I16Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs)
    }
}

impl Mul<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i16) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Mul<&i16> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: &i16) -> I16Vec2 {
        self.mul(*rhs)
    }
}

impl Mul<&i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: &i16) -> I16Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: i16) -> I16Vec2 {
        (*self).mul(rhs)
    }
}

impl MulAssign<i16> for I16Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: i16) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl MulAssign<&i16> for I16Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &i16) {
        self.mul_assign(*rhs)
    }
}

impl Mul<I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: I16Vec2) -> I16Vec2 {
        I16Vec2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Mul<&I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: &I16Vec2) -> I16Vec2 {
        self.mul(*rhs)
    }
}

impl Mul<&I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).mul(*rhs)
    }
}

impl Mul<I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn mul(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).mul(rhs)
    }
}

impl Add<I16Vec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Add<&I16Vec2> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: &I16Vec2) -> I16Vec2 {
        self.add(*rhs)
    }
}

impl Add<&I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).add(rhs)
    }
}

impl AddAssign<I16Vec2> for I16Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl AddAssign<&Self> for I16Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs)
    }
}

impl Add<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: i16) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Add<&i16> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: &i16) -> I16Vec2 {
        self.add(*rhs)
    }
}

impl Add<&i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: &i16) -> I16Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: i16) -> I16Vec2 {
        (*self).add(rhs)
    }
}

impl AddAssign<i16> for I16Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: i16) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl AddAssign<&i16> for I16Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: &i16) {
        self.add_assign(*rhs)
    }
}

impl Add<I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: I16Vec2) -> I16Vec2 {
        I16Vec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Add<&I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: &I16Vec2) -> I16Vec2 {
        self.add(*rhs)
    }
}

impl Add<&I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).add(*rhs)
    }
}

impl Add<I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn add(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).add(rhs)
    }
}

impl Sub<I16Vec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Sub<&I16Vec2> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: &I16Vec2) -> I16Vec2 {
        self.sub(*rhs)
    }
}

impl Sub<&I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).sub(rhs)
    }
}

impl SubAssign<I16Vec2> for I16Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: I16Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl SubAssign<&Self> for I16Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs)
    }
}

impl Sub<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: i16) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Sub<&i16> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: &i16) -> I16Vec2 {
        self.sub(*rhs)
    }
}

impl Sub<&i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: &i16) -> I16Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: i16) -> I16Vec2 {
        (*self).sub(rhs)
    }
}

impl SubAssign<i16> for I16Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: i16) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl SubAssign<&i16> for I16Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &i16) {
        self.sub_assign(*rhs)
    }
}

impl Sub<I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: I16Vec2) -> I16Vec2 {
        I16Vec2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Sub<&I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: &I16Vec2) -> I16Vec2 {
        self.sub(*rhs)
    }
}

impl Sub<&I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).sub(*rhs)
    }
}

impl Sub<I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn sub(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).sub(rhs)
    }
}

impl Rem<I16Vec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl Rem<&I16Vec2> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: &I16Vec2) -> I16Vec2 {
        self.rem(*rhs)
    }
}

impl Rem<&I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<I16Vec2> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).rem(rhs)
    }
}

impl RemAssign<I16Vec2> for I16Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl RemAssign<&Self> for I16Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs)
    }
}

impl Rem<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: i16) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl Rem<&i16> for I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: &i16) -> I16Vec2 {
        self.rem(*rhs)
    }
}

impl Rem<&i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: &i16) -> I16Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<i16> for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: i16) -> I16Vec2 {
        (*self).rem(rhs)
    }
}

impl RemAssign<i16> for I16Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: i16) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
    }
}

impl RemAssign<&i16> for I16Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &i16) {
        self.rem_assign(*rhs)
    }
}

impl Rem<I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: I16Vec2) -> I16Vec2 {
        I16Vec2 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
        }
    }
}

impl Rem<&I16Vec2> for i16 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: &I16Vec2) -> I16Vec2 {
        self.rem(*rhs)
    }
}

impl Rem<&I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: &I16Vec2) -> I16Vec2 {
        (*self).rem(*rhs)
    }
}

impl Rem<I16Vec2> for &i16 {
    type Output = I16Vec2;
    #[inline]
    fn rem(self, rhs: I16Vec2) -> I16Vec2 {
        (*self).rem(rhs)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[i16; 2]> for I16Vec2 {
    #[inline]
    fn as_ref(&self) -> &[i16; 2] {
        unsafe { &*(self as *const I16Vec2 as *const [i16; 2]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[i16; 2]> for I16Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [i16; 2] {
        unsafe { &mut *(self as *mut I16Vec2 as *mut [i16; 2]) }
    }
}

impl Sum for I16Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for I16Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for I16Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for I16Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Neg for I16Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl Neg for &I16Vec2 {
    type Output = I16Vec2;
    #[inline]
    fn neg(self) -> I16Vec2 {
        (*self).neg()
    }
}

impl Not for I16Vec2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            x: self.x.not(),
            y: self.y.not(),
        }
    }
}

impl BitAnd for I16Vec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
        }
    }
}

impl BitOr for I16Vec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
        }
    }
}

impl BitXor for I16Vec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
        }
    }
}

impl BitAnd<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.bitand(rhs),
            y: self.y.bitand(rhs),
        }
    }
}

impl BitOr<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.bitor(rhs),
            y: self.y.bitor(rhs),
        }
    }
}

impl BitXor<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs),
            y: self.y.bitxor(rhs),
        }
    }
}

impl Shl<i8> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i8> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i32> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i32> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i64> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i64> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u8> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u8> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u16> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u32> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u32> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u64> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u64> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<crate::IVec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: crate::IVec2) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shr<crate::IVec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: crate::IVec2) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl Shl<crate::UVec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: crate::UVec2) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shr<crate::UVec2> for I16Vec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: crate::UVec2) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl Index<usize> for I16Vec2 {
    type Output = i16;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for I16Vec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for I16Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl fmt::Debug for I16Vec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(I16Vec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<[i16; 2]> for I16Vec2 {
    #[inline]
    fn from(a: [i16; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<I16Vec2> for [i16; 2] {
    #[inline]
    fn from(v: I16Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(i16, i16)> for I16Vec2 {
    #[inline]
    fn from(t: (i16, i16)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<I16Vec2> for (i16, i16) {
    #[inline]
    fn from(v: I16Vec2) -> Self {
        (v.x, v.y)
    }
}

impl From<I8Vec2> for I16Vec2 {
    #[inline]
    fn from(v: I8Vec2) -> Self {
        Self::new(i16::from(v.x), i16::from(v.y))
    }
}

impl From<U8Vec2> for I16Vec2 {
    #[inline]
    fn from(v: U8Vec2) -> Self {
        Self::new(i16::from(v.x), i16::from(v.y))
    }
}

impl TryFrom<U16Vec2> for I16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: U16Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(i16::try_from(v.x)?, i16::try_from(v.y)?))
    }
}

impl TryFrom<IVec2> for I16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: IVec2) -> Result<Self, Self::Error> {
        Ok(Self::new(i16::try_from(v.x)?, i16::try_from(v.y)?))
    }
}

impl TryFrom<UVec2> for I16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: UVec2) -> Result<Self, Self::Error> {
        Ok(Self::new(i16::try_from(v.x)?, i16::try_from(v.y)?))
    }
}

impl TryFrom<I64Vec2> for I16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: I64Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(i16::try_from(v.x)?, i16::try_from(v.y)?))
    }
}

impl TryFrom<U64Vec2> for I16Vec2 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(v: U64Vec2) -> Result<Self, Self::Error> {
        Ok(Self::new(i16::try_from(v.x)?, i16::try_from(v.y)?))
    }
}

impl From<BVec2> for I16Vec2 {
    #[inline]
    fn from(v: BVec2) -> Self {
        Self::new(i16::from(v.x), i16::from(v.y))
    }
}
