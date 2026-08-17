use crate::core::{
    storage::{XY, XYZ, XYZW},
    traits::{scalar::*, vector::*},
};

impl<T: NumEx> VectorConst for XY<T> {
    const ZERO: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
    };
    const ONE: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> Vector2Const for XY<T> {
    const X: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ZERO,
    };
    const Y: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> VectorConst for XYZ<T> {
    const ZERO: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
    };
    const ONE: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ONE,
        z: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> Vector3Const for XYZ<T> {
    const X: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
    };
    const Y: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ONE,
        z: <T as NumConstEx>::ZERO,
    };
    const Z: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> VectorConst for XYZW<T> {
    const ZERO: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
        w: <T as NumConstEx>::ZERO,
    };
    const ONE: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ONE,
        z: <T as NumConstEx>::ONE,
        w: <T as NumConstEx>::ONE,
    };
}
impl<T: NumEx> Vector4Const for XYZW<T> {
    const X: Self = Self {
        x: <T as NumConstEx>::ONE,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
        w: <T as NumConstEx>::ZERO,
    };
    const Y: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ONE,
        z: <T as NumConstEx>::ZERO,
        w: <T as NumConstEx>::ZERO,
    };
    const Z: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ONE,
        w: <T as NumConstEx>::ZERO,
    };
    const W: Self = Self {
        x: <T as NumConstEx>::ZERO,
        y: <T as NumConstEx>::ZERO,
        z: <T as NumConstEx>::ZERO,
        w: <T as NumConstEx>::ONE,
    };
}

impl<T: NumEx> Vector<T> for XY<T> {
    type Mask = XY<bool>;

    #[inline]
    fn splat(s: T) -> Self {
        Self { x: s, y: s }
    }

    #[inline]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x { if_true.x } else { if_false.x },
            y: if mask.y { if_true.y } else { if_false.y },
        }
    }

    #[inline]
    fn cmpeq(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.eq(&other.x),
            y: self.y.eq(&other.y),
        }
    }

    #[inline]
    fn cmpne(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.ne(&other.x),
            y: self.y.ne(&other.y),
        }
    }

    #[inline]
    fn cmpge(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.ge(&other.x),
            y: self.y.ge(&other.y),
        }
    }

    #[inline]
    fn cmpgt(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.gt(&other.x),
            y: self.y.gt(&other.y),
        }
    }

    #[inline]
    fn cmple(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.le(&other.x),
            y: self.y.le(&other.y),
        }
    }

    #[inline]
    fn cmplt(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.lt(&other.x),
            y: self.y.lt(&other.y),
        }
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        Self {
            x: self.x * b.x + c.x,
            y: self.y * b.y + c.y,
        }
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[inline]
    fn mul_scalar(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }

    #[inline]
    fn div_scalar(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl<T: NumEx> Vector<T> for XYZ<T> {
    type Mask = XYZ<bool>;

    #[inline]
    fn splat(s: T) -> Self {
        Self { x: s, y: s, z: s }
    }

    #[inline]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x { if_true.x } else { if_false.x },
            y: if mask.y { if_true.y } else { if_false.y },
            z: if mask.z { if_true.z } else { if_false.z },
        }
    }

    #[inline]
    fn cmpeq(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.eq(&other.x),
            y: self.y.eq(&other.y),
            z: self.z.eq(&other.z),
        }
    }

    #[inline]
    fn cmpne(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.ne(&other.x),
            y: self.y.ne(&other.y),
            z: self.z.ne(&other.z),
        }
    }

    #[inline]
    fn cmpge(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.ge(&other.x),
            y: self.y.ge(&other.y),
            z: self.z.ge(&other.z),
        }
    }

    #[inline]
    fn cmpgt(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.gt(&other.x),
            y: self.y.gt(&other.y),
            z: self.z.gt(&other.z),
        }
    }

    #[inline]
    fn cmple(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.le(&other.x),
            y: self.y.le(&other.y),
            z: self.z.le(&other.z),
        }
    }

    #[inline]
    fn cmplt(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.lt(&other.x),
            y: self.y.lt(&other.y),
            z: self.z.lt(&other.z),
        }
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        Self {
            x: self.x * b.x + c.x,
            y: self.y * b.y + c.y,
            z: self.z * b.z + c.z,
        }
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    #[inline]
    fn mul_scalar(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    #[inline]
    fn div_scalar(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl<T: NumEx> Vector<T> for XYZW<T> {
    type Mask = XYZW<bool>;

    #[inline]
    fn splat(s: T) -> Self {
        Self {
            x: s,
            y: s,
            z: s,
            w: s,
        }
    }

    #[inline]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x { if_true.x } else { if_false.x },
            y: if mask.y { if_true.y } else { if_false.y },
            z: if mask.z { if_true.z } else { if_false.z },
            w: if mask.w { if_true.w } else { if_false.w },
        }
    }

    #[inline]
    fn cmpeq(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.eq(&other.x),
            y: self.y.eq(&other.y),
            z: self.z.eq(&other.z),
            w: self.w.eq(&other.w),
        }
    }

    #[inline]
    fn cmpne(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.ne(&other.x),
            y: self.y.ne(&other.y),
            z: self.z.ne(&other.z),
            w: self.w.ne(&other.w),
        }
    }

    #[inline]
    fn cmpge(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.ge(&other.x),
            y: self.y.ge(&other.y),
            z: self.z.ge(&other.z),
            w: self.w.ge(&other.w),
        }
    }

    #[inline]
    fn cmpgt(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.gt(&other.x),
            y: self.y.gt(&other.y),
            z: self.z.gt(&other.z),
            w: self.w.gt(&other.w),
        }
    }

    #[inline]
    fn cmple(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.le(&other.x),
            y: self.y.le(&other.y),
            z: self.z.le(&other.z),
            w: self.w.le(&other.w),
        }
    }

    #[inline]
    fn cmplt(self, other: Self) -> Self::Mask {
        Self::Mask {
            x: self.x.lt(&other.x),
            y: self.y.lt(&other.y),
            z: self.z.lt(&other.z),
            w: self.w.lt(&other.w),
        }
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }

    #[inline]
    fn mul_add(self, b: Self, c: Self) -> Self {
        Self {
            x: self.x * b.x + c.x,
            y: self.y * b.y + c.y,
            z: self.z * b.z + c.z,
            w: self.w * b.w + c.w,
        }
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    #[inline]
    fn mul_scalar(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }

    #[inline]
    fn div_scalar(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
            w: self.w.min(other.w),
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }
}

impl<T: NumEx> Vector2<T> for XY<T> {
    #[inline(always)]
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x)
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y)
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self::new(slice[0], slice[1])
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x;
        slice[1] = self.y;
    }

    #[inline(always)]
    fn as_ref_xy(&self) -> &XY<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xy(&mut self) -> &mut XY<T> {
        self
    }

    #[inline(always)]
    fn into_xyz(self, z: T) -> XYZ<T> {
        XYZ {
            x: self.x,
            y: self.y,
            z,
        }
    }

    #[inline(always)]
    fn into_xyzw(self, z: T, w: T) -> XYZW<T> {
        XYZW {
            x: self.x,
            y: self.y,
            z,
            w,
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 2]) -> Self {
        Self { x: a[0], y: a[1] }
    }

    #[inline(always)]
    fn into_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    #[inline]
    fn min_element(self) -> T {
        self.x.min(self.y)
    }

    #[inline]
    fn max_element(self) -> T {
        self.x.max(self.y)
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.x <= max.x);
        glam_assert!(min.y <= max.y);
        // we intentionally do not use `f32::clamp` because we don't
        // want panics unless `glam-assert` feature is on.
        Self {
            x: self.x.max(min.x).min(max.x),
            y: self.y.max(min.y).min(max.y),
        }
    }
}

impl<T: NumEx> Vector3<T> for XYZ<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x)
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y)
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        Self::splat(self.z)
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
    }

    #[inline(always)]
    fn as_ref_xyz(&self) -> &XYZ<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xyz(&mut self) -> &mut XYZ<T> {
        self
    }

    #[inline(always)]
    fn into_xy(self) -> XY<T> {
        XY {
            x: self.x,
            y: self.y,
        }
    }

    #[inline(always)]
    fn into_xyzw(self, w: T) -> XYZW<T> {
        XYZW {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 3]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }

    #[inline(always)]
    fn into_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    #[inline]
    fn min_element(self) -> T {
        self.x.min(self.y.min(self.z))
    }

    #[inline]
    fn max_element(self) -> T {
        self.x.max(self.y.max(self.z))
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[inline]
    fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.x <= max.x);
        glam_assert!(min.y <= max.y);
        glam_assert!(min.z <= max.z);
        // we intentionally do not use `f32::clamp` because we don't
        // want panics unless `glam-assert` feature is on.
        Self {
            x: self.x.max(min.x).min(max.x),
            y: self.y.max(min.y).min(max.y),
            z: self.z.max(min.z).min(max.z),
        }
    }
}

impl<T: NumEx> Vector4<T> for XYZW<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x)
    }

    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y)
    }

    #[inline(always)]
    fn splat_z(self) -> Self {
        Self::splat(self.z)
    }

    #[inline(always)]
    fn splat_w(self) -> Self {
        Self::splat(self.w)
    }

    #[inline(always)]
    fn from_slice_unaligned(slice: &[T]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }

    #[inline(always)]
    fn write_to_slice_unaligned(self, slice: &mut [T]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
        slice[3] = self.w;
    }

    #[inline(always)]
    fn as_ref_xyzw(&self) -> &XYZW<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xyzw(&mut self) -> &mut XYZW<T> {
        self
    }

    #[inline(always)]
    fn into_xy(self) -> XY<T> {
        XY {
            x: self.x,
            y: self.y,
        }
    }

    #[inline(always)]
    fn into_xyz(self) -> XYZ<T> {
        XYZ {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline(always)]
    fn from_array(a: [T; 4]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
            w: a[3],
        }
    }

    #[inline(always)]
    fn into_array(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline(always)]
    fn from_tuple(t: (T, T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }

    #[inline(always)]
    fn into_tuple(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }

    #[inline]
    fn min_element(self) -> T {
        self.x.min(self.y.min(self.z.min(self.w)))
    }

    #[inline]
    fn max_element(self) -> T {
        self.x.max(self.y.max(self.z.min(self.w)))
    }

    #[inline]
    fn dot(self, other: Self) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.x <= max.x);
        glam_assert!(min.y <= max.y);
        glam_assert!(min.z <= max.z);
        glam_assert!(min.w <= max.w);
        // we intentionally do not use `f32::clamp` because we don't
        // want panics unless `glam-assert` feature is on.
        Self {
            x: self.x.max(min.x).min(max.x),
            y: self.y.max(min.y).min(max.y),
            z: self.z.max(min.z).min(max.z),
            w: self.w.max(min.w).min(max.w),
        }
    }
}

impl<T: SignedEx> SignedVector<T> for XY<T> {
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl<T: SignedEx> SignedVector2<T> for XY<T> {
    #[inline]
    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    #[inline]
    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    #[inline]
    fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    #[inline]
    fn perp_dot(self, other: Self) -> T {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl<T: SignedEx> SignedVector<T> for XYZ<T> {
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl<T: SignedEx> SignedVector3<T> for XYZ<T> {
    #[inline]
    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    #[inline]
    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }
}

impl<T: SignedEx> SignedVector<T> for XYZW<T> {
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

impl<T: SignedEx> SignedVector4<T> for XYZW<T> {
    #[inline]
    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    #[inline]
    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
            w: self.w.signum(),
        }
    }
}

impl<T: FloatEx> FloatVector2<T> for XY<T> {
    #[inline]
    fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    #[inline]
    fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    #[inline]
    fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    #[inline]
    fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
        }
    }

    #[inline]
    fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
        }
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
        }
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    #[inline]
    fn is_nan_mask(self) -> Self::Mask {
        Self::Mask {
            x: self.x.is_nan(),
            y: self.y.is_nan(),
        }
    }
}

impl<T: FloatEx> FloatVector3<T> for XYZ<T> {
    #[inline]
    fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    #[inline]
    fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    #[inline]
    fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    #[inline]
    fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
        }
    }

    #[inline]
    fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
            z: self.z.exp(),
        }
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
            z: self.z.powf(n),
        }
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    #[inline]
    fn is_nan_mask(self) -> Self::Mask {
        Self::Mask {
            x: self.x.is_nan(),
            y: self.y.is_nan(),
            z: self.z.is_nan(),
        }
    }
}

impl<T: FloatEx> FloatVector4<T> for XYZW<T> {
    #[inline]
    fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
            w: self.w.floor(),
        }
    }

    #[inline]
    fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
            w: self.w.ceil(),
        }
    }

    #[inline]
    fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
            w: self.w.round(),
        }
    }

    #[inline]
    fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
            w: self.w.recip(),
        }
    }

    #[inline]
    fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
            z: self.z.exp(),
            w: self.w.exp(),
        }
    }

    #[inline]
    fn powf(self, n: T) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
            z: self.z.powf(n),
            w: self.w.powf(n),
        }
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }

    #[inline]
    fn is_nan_mask(self) -> Self::Mask {
        Self::Mask {
            x: self.x.is_nan(),
            y: self.y.is_nan(),
            z: self.z.is_nan(),
            w: self.w.is_nan(),
        }
    }
}

impl<T> From<XYZW<T>> for XYZ<T> {
    #[inline(always)]
    fn from(v: XYZW<T>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl<T> From<XYZW<T>> for XY<T> {
    #[inline(always)]
    fn from(v: XYZW<T>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl<T> From<XYZ<T>> for XY<T> {
    #[inline(always)]
    fn from(v: XYZ<T>) -> Self {
        Self { x: v.x, y: v.y }
    }
}
