// Generated from vec.rs.tera template. Edit the template, not the generated file.

use crate::{BVec2, IVec3};

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::{f32, ops::*};

/// Creates a 2-dimensional vector.
#[inline(always)]
pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2::new(x, y)
}

/// A 2-dimensional vector.
#[cfg_attr(not(target_arch = "spirv"), derive(Hash))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "cuda", repr(align(8)))]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// All ones.
    pub const ONE: Self = Self::splat(1);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1);

    /// A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self::new(1, 0);

    /// A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0, 1);

    /// A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1, 0);

    /// A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0, -1);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Creates a new vector.
    #[inline(always)]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v }
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    pub fn select(mask: BVec2, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x { if_true.x } else { if_false.x },
            y: if mask.y { if_true.y } else { if_false.y },
        }
    }

    /// Creates a new vector from an array.
    #[inline]
    pub const fn from_array(a: [i32; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// `[x, y]`
    #[inline]
    pub const fn to_array(&self) -> [i32; 2] {
        [self.x, self.y]
    }

    /// Creates a vector from the first 2 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub const fn from_slice(slice: &[i32]) -> Self {
        Self::new(slice[0], slice[1])
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [i32]) {
        slice[0] = self.x;
        slice[1] = self.y;
    }

    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline]
    pub const fn extend(self, z: i32) -> IVec3 {
        IVec3::new(self.x, self.y, z)
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot(self, rhs: Self) -> i32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Returns a vector where every component is the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot_into_vec(self, rhs: Self) -> Self {
        Self::splat(self.dot(rhs))
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
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
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Component-wise clamping of values, similar to [`i32::clamp`].
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
    pub fn min_element(self) -> i32 {
        self.x.min(self.y)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    pub fn max_element(self) -> i32 {
        self.x.max(self.y)
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpeq(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.eq(&rhs.x), self.y.eq(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpne(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.ne(&rhs.x), self.y.ne(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpge(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.ge(&rhs.x), self.y.ge(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpgt(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.gt(&rhs.x), self.y.gt(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmple(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.le(&rhs.x), self.y.le(&rhs.y))
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmplt(self, rhs: Self) -> BVec2 {
        BVec2::new(self.x.lt(&rhs.x), self.y.lt(&rhs.y))
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
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
    pub fn is_negative_bitmask(self) -> u32 {
        (self.x.is_negative() as u32) | (self.y.is_negative() as u32) << 1
    }

    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline]
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
    pub fn perp_dot(self, rhs: Self) -> i32 {
        (self.x * rhs.y) - (self.y * rhs.x)
    }

    /// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    #[must_use]
    #[inline]
    pub fn rotate(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x - self.y * rhs.y,
            y: self.y * rhs.x + self.x * rhs.y,
        }
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub fn as_vec2(&self) -> crate::Vec2 {
        crate::Vec2::new(self.x as f32, self.y as f32)
    }

    /// Casts all elements of `self` to `f64`.
    #[inline]
    pub fn as_dvec2(&self) -> crate::DVec2 {
        crate::DVec2::new(self.x as f64, self.y as f64)
    }

    /// Casts all elements of `self` to `u32`.
    #[inline]
    pub fn as_uvec2(&self) -> crate::UVec2 {
        crate::UVec2::new(self.x as u32, self.y as u32)
    }
}

impl Default for IVec2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Div<IVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl DivAssign<IVec2> for IVec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl Div<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i32) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl DivAssign<i32> for IVec2 {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl Div<IVec2> for i32 {
    type Output = IVec2;
    #[inline]
    fn div(self, rhs: IVec2) -> IVec2 {
        IVec2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Mul<IVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl MulAssign<IVec2> for IVec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl Mul<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl MulAssign<i32> for IVec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl Mul<IVec2> for i32 {
    type Output = IVec2;
    #[inline]
    fn mul(self, rhs: IVec2) -> IVec2 {
        IVec2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Add<IVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl AddAssign<IVec2> for IVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl Add<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: i32) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl AddAssign<i32> for IVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl Add<IVec2> for i32 {
    type Output = IVec2;
    #[inline]
    fn add(self, rhs: IVec2) -> IVec2 {
        IVec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Sub<IVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl SubAssign<IVec2> for IVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl Sub<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: i32) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl SubAssign<i32> for IVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: i32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl Sub<IVec2> for i32 {
    type Output = IVec2;
    #[inline]
    fn sub(self, rhs: IVec2) -> IVec2 {
        IVec2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Rem<IVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl RemAssign<IVec2> for IVec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl Rem<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: i32) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl RemAssign<i32> for IVec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: i32) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
    }
}

impl Rem<IVec2> for i32 {
    type Output = IVec2;
    #[inline]
    fn rem(self, rhs: IVec2) -> IVec2 {
        IVec2 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[i32; 2]> for IVec2 {
    #[inline]
    fn as_ref(&self) -> &[i32; 2] {
        unsafe { &*(self as *const IVec2 as *const [i32; 2]) }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsMut<[i32; 2]> for IVec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [i32; 2] {
        unsafe { &mut *(self as *mut IVec2 as *mut [i32; 2]) }
    }
}

impl Sum for IVec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for IVec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for IVec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for IVec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}

impl Neg for IVec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl Not for IVec2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self {
            x: self.x.not(),
            y: self.y.not(),
        }
    }
}

impl BitAnd for IVec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitand(rhs.x),
            y: self.y.bitand(rhs.y),
        }
    }
}

impl BitOr for IVec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitor(rhs.x),
            y: self.y.bitor(rhs.y),
        }
    }
}

impl BitXor for IVec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs.x),
            y: self.y.bitxor(rhs.y),
        }
    }
}

impl BitAnd<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitand(rhs),
            y: self.y.bitand(rhs),
        }
    }
}

impl BitOr<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitor(rhs),
            y: self.y.bitor(rhs),
        }
    }
}

impl BitXor<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.bitxor(rhs),
            y: self.y.bitxor(rhs),
        }
    }
}

impl Shl<i8> for IVec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i8> for IVec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i16> for IVec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i16> for IVec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<i32> for IVec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u8> for IVec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u8> for IVec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u16> for IVec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u16> for IVec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u16) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<u32> for IVec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shl(rhs),
            y: self.y.shl(rhs),
        }
    }
}

impl Shr<u32> for IVec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x.shr(rhs),
            y: self.y.shr(rhs),
        }
    }
}

impl Shl<crate::IVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: crate::IVec2) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shr<crate::IVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: crate::IVec2) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl Shl<crate::UVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: crate::UVec2) -> Self::Output {
        Self {
            x: self.x.shl(rhs.x),
            y: self.y.shl(rhs.y),
        }
    }
}

impl Shr<crate::UVec2> for IVec2 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: crate::UVec2) -> Self::Output {
        Self {
            x: self.x.shr(rhs.x),
            y: self.y.shr(rhs.y),
        }
    }
}

impl Index<usize> for IVec2 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for IVec2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for IVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for IVec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(IVec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<[i32; 2]> for IVec2 {
    #[inline]
    fn from(a: [i32; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<IVec2> for [i32; 2] {
    #[inline]
    fn from(v: IVec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(i32, i32)> for IVec2 {
    #[inline]
    fn from(t: (i32, i32)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<IVec2> for (i32, i32) {
    #[inline]
    fn from(v: IVec2) -> Self {
        (v.x, v.y)
    }
}
