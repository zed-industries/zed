use super::scalar::{FloatEx, SignedEx};
use crate::core::storage::{XY, XYZ, XYZW};
use core::ops::{Add, Mul, Sub};

/// Mask vector constants that are independent of vector length
pub trait MaskVectorConst: Sized {
    const FALSE: Self;
}

/// Mask vector methods that are independent of vector dimension.
pub trait MaskVector: MaskVectorConst {
    fn bitand(self, other: Self) -> Self;
    fn bitor(self, other: Self) -> Self;
    fn not(self) -> Self;
}

/// Mask vector methods specific to 2D vectors.
pub trait MaskVector2: MaskVector {
    fn new(x: bool, y: bool) -> Self;
    fn bitmask(self) -> u32;
    fn any(self) -> bool;
    fn all(self) -> bool;
    fn into_bool_array(self) -> [bool; 2];
    fn into_u32_array(self) -> [u32; 2];
}

/// Mask vector methods specific to 3D vectors.
pub trait MaskVector3: MaskVector {
    fn new(x: bool, y: bool, z: bool) -> Self;
    fn bitmask(self) -> u32;
    fn any(self) -> bool;
    fn all(self) -> bool;
    fn into_bool_array(self) -> [bool; 3];
    fn into_u32_array(self) -> [u32; 3];
}

/// Mask vector methods specific to 4D vectors.
pub trait MaskVector4: MaskVector {
    fn new(x: bool, y: bool, z: bool, w: bool) -> Self;
    fn bitmask(self) -> u32;
    fn any(self) -> bool;
    fn all(self) -> bool;
    fn into_bool_array(self) -> [bool; 4];
    fn into_u32_array(self) -> [u32; 4];
}

/// Vector constants that are independent of vector dimension.
pub trait VectorConst {
    const ZERO: Self;
    const ONE: Self;
}

/// Vector constants specific to 2D vectors.
pub trait Vector2Const: VectorConst {
    const X: Self;
    const Y: Self;
}

/// Vector constants specific to 3D vectors.
pub trait Vector3Const: VectorConst {
    const X: Self;
    const Y: Self;
    const Z: Self;
}

/// Vector constants specific to 4D vectors.
pub trait Vector4Const: VectorConst {
    const X: Self;
    const Y: Self;
    const Z: Self;
    const W: Self;
}

/// Vector methods that are independent of vector dimension.
///
/// These methods typically need to be implemented for each type as while the method signature does
/// not imply any dimensionality, the implementation does.
pub trait Vector<T>: Sized + Copy + Clone {
    type Mask;

    fn splat(s: T) -> Self;

    fn select(mask: Self::Mask, a: Self, b: Self) -> Self;

    fn cmpeq(self, other: Self) -> Self::Mask;
    fn cmpne(self, other: Self) -> Self::Mask;
    fn cmpge(self, other: Self) -> Self::Mask;
    fn cmpgt(self, other: Self) -> Self::Mask;
    fn cmple(self, other: Self) -> Self::Mask;
    fn cmplt(self, other: Self) -> Self::Mask;

    fn add(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn rem(self, rhs: Self) -> Self;
    fn sub(self, other: Self) -> Self;

    fn scale(self, other: T) -> Self {
        self.mul_scalar(other)
    }

    fn add_scalar(self, other: T) -> Self;
    fn sub_scalar(self, other: T) -> Self;
    fn mul_scalar(self, other: T) -> Self;
    fn div_scalar(self, other: T) -> Self;
    fn rem_scalar(self, rhs: T) -> Self;

    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

/// Vector methods specific to 2D vectors.
pub trait Vector2<T>: Vector<T> + Vector2Const
where
    T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T>,
{
    fn new(x: T, y: T) -> Self;
    fn x(self) -> T;
    fn y(self) -> T;

    fn as_ref_xy(&self) -> &XY<T>;
    fn as_mut_xy(&mut self) -> &mut XY<T>;

    // min and max behave differently for float and integer types in Rust, so we can't have a
    // default implementation here.
    fn min_element(self) -> T;
    fn max_element(self) -> T;
    fn clamp(self, min: Self, max: Self) -> Self;

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x())
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y())
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self::new(slice[0], slice[1])
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x();
        slice[1] = self.y();
    }

    #[inline(always)]
    fn into_xyz(self, z: T) -> XYZ<T> {
        XYZ {
            x: self.x(),
            y: self.y(),
            z,
        }
    }

    #[inline(always)]
    fn into_xyzw(self, z: T, w: T) -> XYZW<T> {
        XYZW {
            x: self.x(),
            y: self.y(),
            z,
            w,
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    #[inline(always)]
    fn into_array(self) -> [T; 2] {
        [self.x(), self.y()]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T) {
        (self.x(), self.y())
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x() * other.x()) + (self.y() * other.y())
    }

    #[inline(always)]
    fn dot_into_vec(self, other: Self) -> Self {
        Self::splat(self.dot(other))
    }
}

/// Vector methods specific to 3D vectors.
pub trait Vector3<T>: Vector<T> + Vector3Const
where
    T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T>,
{
    fn new(x: T, y: T, z: T) -> Self;
    fn x(self) -> T;
    fn y(self) -> T;
    fn z(self) -> T;

    fn as_ref_xyz(&self) -> &XYZ<T>;
    fn as_mut_xyz(&mut self) -> &mut XYZ<T>;

    fn min_element(self) -> T;
    fn max_element(self) -> T;
    fn clamp(self, min: Self, max: Self) -> Self;

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x())
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y())
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        Self::splat(self.z())
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self::new(slice[0], slice[1], slice[2])
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x();
        slice[1] = self.y();
        slice[2] = self.z();
    }

    #[inline(always)]
    fn from_xy(v2: XY<T>, z: T) -> Self {
        Self::new(v2.x, v2.y, z)
    }

    #[inline(always)]
    fn from_xyzw(v4: XYZW<T>) -> Self {
        Self::new(v4.x, v4.y, v4.z)
    }

    #[inline(always)]
    fn into_xy(self) -> XY<T> {
        XY {
            x: self.x(),
            y: self.y(),
        }
    }

    #[inline(always)]
    fn into_xyzw(self, w: T) -> XYZW<T> {
        XYZW {
            x: self.x(),
            y: self.y(),
            z: self.z(),
            w,
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    #[inline(always)]
    fn into_array(self) -> [T; 3] {
        [self.x(), self.y(), self.z()]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T, T) {
        (self.x(), self.y(), self.z())
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
    }

    #[inline(always)]
    fn dot_into_vec(self, other: Self) -> Self {
        Self::splat(self.dot(other))
    }

    #[inline]
    fn cross(self, other: Self) -> Self {
        Self::new(
            self.y() * other.z() - other.y() * self.z(),
            self.z() * other.x() - other.z() * self.x(),
            self.x() * other.y() - other.x() * self.y(),
        )
    }
}

/// Vector methods specific to 3D vectors.
pub trait Vector4<T>: Vector<T> + Vector4Const
where
    T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T>,
{
    fn new(x: T, y: T, z: T, w: T) -> Self;

    fn x(self) -> T;
    fn y(self) -> T;
    fn z(self) -> T;
    fn w(self) -> T;

    fn as_ref_xyzw(&self) -> &XYZW<T>;
    fn as_mut_xyzw(&mut self) -> &mut XYZW<T>;

    fn min_element(self) -> T;
    fn max_element(self) -> T;
    fn clamp(self, min: Self, max: Self) -> Self;

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x())
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y())
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        Self::splat(self.z())
    }

    #[inline(always)]
    fn splat_w(self) -> Self {
        Self::splat(self.w())
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self::new(slice[0], slice[1], slice[2], slice[3])
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x();
        slice[1] = self.y();
        slice[2] = self.z();
        slice[3] = self.w();
    }

    #[inline(always)]
    fn from_xy(v2: XY<T>, z: T, w: T) -> Self {
        Self::new(v2.x, v2.y, z, w)
    }

    #[inline(always)]
    fn from_xyz(v3: XYZ<T>, w: T) -> Self {
        Self::new(v3.x, v3.y, v3.z, w)
    }

    #[inline(always)]
    fn into_xy(self) -> XY<T> {
        XY {
            x: self.x(),
            y: self.y(),
        }
    }

    #[inline(always)]
    fn into_xyz(self) -> XYZ<T> {
        XYZ {
            x: self.x(),
            y: self.y(),
            z: self.z(),
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    #[inline(always)]
    fn into_array(self) -> [T; 4] {
        [self.x(), self.y(), self.z(), self.w()]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T, T, T) {
        (self.x(), self.y(), self.z(), self.w())
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x() * other.x())
            + (self.y() * other.y())
            + (self.z() * other.z())
            + (self.w() * other.w())
    }

    #[inline(always)]
    fn dot_into_vec(self, other: Self) -> Self {
        Self::splat(self.dot(other))
    }
}

/// Vector methods for vectors of signed types that are independent of vector dimension.
///
/// These methods typically need to be implemented for each type as while the method signature does
/// not imply any dimensionality, the implementation does.
pub trait SignedVector<T: SignedEx>: Vector<T> {
    fn neg(self) -> Self;
}

/// Vector methods specific to 2D vectors of signed types.
pub trait SignedVector2<T: SignedEx>: SignedVector<T> + Vector2<T> {
    #[inline]
    fn abs(self) -> Self {
        Self::new(self.x().abs(), self.y().abs())
    }

    #[inline]
    fn signum(self) -> Self {
        Self::new(self.x().signum(), self.y().signum())
    }

    #[inline]
    fn perp(self) -> Self {
        Self::new(-self.y(), self.x())
    }

    #[inline]
    fn perp_dot(self, other: Self) -> T {
        (self.x() * other.y()) - (self.y() * other.x())
    }
}

/// Vector methods specific to 3D vectors of signed types.
pub trait SignedVector3<T: SignedEx>: SignedVector<T> + Vector3<T> {
    #[inline]
    fn abs(self) -> Self {
        Self::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    #[inline]
    fn signum(self) -> Self {
        Self::new(self.x().signum(), self.y().signum(), self.z().signum())
    }
}

pub trait SignedVector4<T: SignedEx>: SignedVector<T> + Vector4<T> {
    #[inline]
    fn abs(self) -> Self {
        Self::new(
            self.x().abs(),
            self.y().abs(),
            self.z().abs(),
            self.w().abs(),
        )
    }

    #[inline]
    fn signum(self) -> Self {
        Self::new(
            self.x().signum(),
            self.y().signum(),
            self.z().signum(),
            self.w().signum(),
        )
    }
}

pub trait FloatVector2<T: FloatEx>: SignedVector2<T> {
    #[inline]
    fn floor(self) -> Self {
        Self::new(self.x().floor(), self.y().floor())
    }

    #[inline]
    fn ceil(self) -> Self {
        Self::new(self.x().ceil(), self.y().ceil())
    }

    #[inline]
    fn round(self) -> Self {
        Self::new(self.x().round(), self.y().round())
    }

    #[inline]
    fn recip(self) -> Self {
        Self::new(self.x().recip(), self.y().recip())
    }

    #[inline]
    fn exp(self) -> Self {
        Self::new(self.x().exp(), self.y().exp())
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        Self::new(self.x().powf(n), self.y().powf(n))
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x().is_finite() && self.y().is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x().is_nan() || self.y().is_nan()
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        Self::new(
            self.x().mul_add(b.x(), c.x()),
            self.y().mul_add(b.y(), c.y()),
        )
    }

    #[inline]
    fn is_nan_mask(self) -> Self::Mask
    where
        <Self as Vector<T>>::Mask: MaskVector2,
    {
        Self::Mask::new(self.x().is_nan(), self.y().is_nan())
    }

    #[inline]
    fn length(self) -> T {
        self.dot(self).sqrt()
    }

    #[inline]
    fn length_recip(self) -> T {
        self.length().recip()
    }

    #[inline]
    fn normalize(self) -> Self {
        #[allow(clippy::let_and_return)]
        let normalized = self.mul_scalar(self.length_recip());
        glam_assert!(normalized.is_finite());
        normalized
    }

    #[inline(always)]
    fn length_squared(self) -> T {
        self.dot(self)
    }

    #[inline]
    fn is_normalized(self) -> bool {
        // TODO: do something with epsilon
        (self.length_squared() - T::ONE).abs() <= T::from_f64(1e-4)
    }

    #[inline]
    fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool
    where
        <Self as Vector<T>>::Mask: MaskVector2,
    {
        self.sub(other).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    #[inline]
    fn angle_between(self, other: Self) -> T {
        let angle = (self.dot(other) / (self.length_squared() * other.length_squared()).sqrt())
            .acos_approx();

        if self.perp_dot(other) < T::ZERO {
            -angle
        } else {
            angle
        }
    }
}

pub trait FloatVector3<T: FloatEx>: SignedVector3<T> {
    #[inline]
    fn floor(self) -> Self {
        Self::new(self.x().floor(), self.y().floor(), self.z().floor())
    }

    #[inline]
    fn ceil(self) -> Self {
        Self::new(self.x().ceil(), self.y().ceil(), self.z().ceil())
    }

    #[inline]
    fn round(self) -> Self {
        Self::new(self.x().round(), self.y().round(), self.z().round())
    }

    #[inline]
    fn recip(self) -> Self {
        Self::new(self.x().recip(), self.y().recip(), self.z().recip())
    }

    #[inline]
    fn exp(self) -> Self {
        Self::new(self.x().exp(), self.y().exp(), self.z().exp())
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        Self::new(self.x().powf(n), self.y().powf(n), self.z().powf(n))
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x().is_finite() && self.y().is_finite() && self.z().is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x().is_nan() || self.y().is_nan() || self.z().is_nan()
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        Self::new(
            self.x().mul_add(b.x(), c.x()),
            self.y().mul_add(b.y(), c.y()),
            self.z().mul_add(b.z(), c.z()),
        )
    }

    #[inline]
    fn is_nan_mask(self) -> Self::Mask
    where
        <Self as Vector<T>>::Mask: MaskVector3,
    {
        Self::Mask::new(self.x().is_nan(), self.y().is_nan(), self.z().is_nan())
    }

    #[inline]
    fn length(self) -> T {
        self.dot(self).sqrt()
    }

    #[inline]
    fn length_recip(self) -> T {
        self.length().recip()
    }

    #[inline]
    fn normalize(self) -> Self {
        #[allow(clippy::let_and_return)]
        let normalized = self.mul_scalar(self.length_recip());
        glam_assert!(normalized.is_finite());
        normalized
    }

    #[inline(always)]
    fn length_squared(self) -> T {
        self.dot(self)
    }

    #[inline]
    fn is_normalized(self) -> bool {
        // TODO: do something with epsilon
        (self.length_squared() - T::ONE).abs() <= T::from_f64(1e-4)
    }

    #[inline]
    fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool
    where
        <Self as Vector<T>>::Mask: MaskVector3,
    {
        self.sub(other).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    fn angle_between(self, other: Self) -> T {
        self.dot(other)
            .div(self.length_squared().mul(other.length_squared()).sqrt())
            .acos_approx()
    }
}

pub trait FloatVector4<T: FloatEx>: SignedVector4<T> {
    #[inline]
    fn floor(self) -> Self {
        Self::new(
            self.x().floor(),
            self.y().floor(),
            self.z().floor(),
            self.w().floor(),
        )
    }

    #[inline]
    fn ceil(self) -> Self {
        Self::new(
            self.x().ceil(),
            self.y().ceil(),
            self.z().ceil(),
            self.w().ceil(),
        )
    }

    #[inline]
    fn round(self) -> Self {
        Self::new(
            self.x().round(),
            self.y().round(),
            self.z().round(),
            self.w().round(),
        )
    }

    #[inline]
    fn recip(self) -> Self {
        Self::new(
            self.x().recip(),
            self.y().recip(),
            self.z().recip(),
            self.w().recip(),
        )
    }

    #[inline]
    fn exp(self) -> Self {
        Self::new(
            self.x().exp(),
            self.y().exp(),
            self.z().exp(),
            self.w().exp(),
        )
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        Self::new(
            self.x().powf(n),
            self.y().powf(n),
            self.z().powf(n),
            self.w().powf(n),
        )
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x().is_finite() && self.y().is_finite() && self.z().is_finite() && self.w().is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x().is_nan() || self.y().is_nan() || self.z().is_nan() || self.w().is_nan()
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        Self::new(
            self.x().mul_add(b.x(), c.x()),
            self.y().mul_add(b.y(), c.y()),
            self.z().mul_add(b.z(), c.z()),
            self.w().mul_add(b.w(), c.w()),
        )
    }

    #[inline]
    fn is_nan_mask(self) -> Self::Mask
    where
        <Self as Vector<T>>::Mask: MaskVector4,
    {
        Self::Mask::new(
            self.x().is_nan(),
            self.y().is_nan(),
            self.z().is_nan(),
            self.w().is_nan(),
        )
    }

    #[inline]
    fn length(self) -> T {
        self.dot(self).sqrt()
    }

    #[inline]
    fn length_recip(self) -> T {
        self.length().recip()
    }

    #[inline]
    fn normalize(self) -> Self {
        #[allow(clippy::let_and_return)]
        let normalized = self.mul_scalar(self.length_recip());
        glam_assert!(normalized.is_finite());
        normalized
    }

    #[inline(always)]
    fn length_squared(self) -> T {
        self.dot(self)
    }

    #[inline]
    fn is_normalized(self) -> bool {
        // TODO: do something with epsilon
        (self.length_squared() - T::ONE).abs() <= T::from_f64(1e-4)
    }

    #[inline]
    fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool
    where
        <Self as Vector<T>>::Mask: MaskVector4,
    {
        self.sub(other).abs().cmple(Self::splat(max_abs_diff)).all()
    }
}

pub trait ScalarShiftOps<Rhs> {
    fn scalar_shl(self, rhs: Rhs) -> Self;
    fn scalar_shr(self, rhs: Rhs) -> Self;
}

pub trait VectorShiftOps<Rhs> {
    fn vector_shl(self, rhs: Rhs) -> Self;
    fn vector_shr(self, rhs: Rhs) -> Self;
}

pub trait ScalarBitOps<Rhs> {
    fn scalar_bitand(self, rhs: Rhs) -> Self;
    fn scalar_bitor(self, rhs: Rhs) -> Self;
    fn scalar_bitxor(self, rhs: Rhs) -> Self;
}

pub trait VectorBitOps<Rhs> {
    fn not(self) -> Self;
    fn vector_bitand(self, rhs: Rhs) -> Self;
    fn vector_bitor(self, rhs: Rhs) -> Self;
    fn vector_bitxor(self, rhs: Rhs) -> Self;
}
