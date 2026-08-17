use crate::core::{
    storage::{XY, XYZ, XYZF32A16, XYZW},
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

impl<T: NanConstEx> NanConstEx for XY<T> {
    const NAN: Self = Self {
        x: <T as NanConstEx>::NAN,
        y: <T as NanConstEx>::NAN,
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

impl<T: NanConstEx> NanConstEx for XYZ<T> {
    const NAN: Self = Self {
        x: <T as NanConstEx>::NAN,
        y: <T as NanConstEx>::NAN,
        z: <T as NanConstEx>::NAN,
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

impl<T: NanConstEx> NanConstEx for XYZW<T> {
    const NAN: Self = Self {
        x: <T as NanConstEx>::NAN,
        y: <T as NanConstEx>::NAN,
        z: <T as NanConstEx>::NAN,
        w: <T as NanConstEx>::NAN,
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
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[inline]
    fn add_scalar(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }

    #[inline]
    fn sub_scalar(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
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
    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }

    #[inline]
    fn rem_scalar(self, other: T) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
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
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add_scalar(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }

    fn sub_scalar(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
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
    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        }
    }

    #[inline]
    fn rem_scalar(self, other: T) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
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
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    fn add_scalar(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }

    fn sub_scalar(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
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
    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
            w: self.w % other.w,
        }
    }

    #[inline]
    fn rem_scalar(self, other: T) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
            w: self.w % other,
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
    fn x(self) -> T {
        self.x
    }

    #[inline(always)]
    fn y(self) -> T {
        self.y
    }

    #[inline(always)]
    fn as_ref_xy(&self) -> &XY<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xy(&mut self) -> &mut XY<T> {
        self
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
    fn x(self) -> T {
        self.x
    }

    #[inline(always)]
    fn y(self) -> T {
        self.y
    }

    #[inline(always)]
    fn z(self) -> T {
        self.z
    }

    #[inline(always)]
    fn as_ref_xyz(&self) -> &XYZ<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xyz(&mut self) -> &mut XYZ<T> {
        self
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
    fn clamp(self, min: Self, max: Self) -> Self {
        glam_assert!(min.x <= max.x);
        glam_assert!(min.y <= max.y);
        glam_assert!(min.z <= max.z);
        // we intentionally do not use `f32::clamp` because we don't
        // want panics unless `glam-assert` feature is on.
        Self::new(
            self.x.max(min.x).min(max.x),
            self.y.max(min.y).min(max.y),
            self.z.max(min.z).min(max.z),
        )
    }
}

impl<T: NumEx> Vector4<T> for XYZW<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    #[inline(always)]
    fn x(self) -> T {
        self.x
    }

    #[inline(always)]
    fn y(self) -> T {
        self.y
    }

    #[inline(always)]
    fn z(self) -> T {
        self.z
    }

    #[inline(always)]
    fn w(self) -> T {
        self.w
    }

    #[inline(always)]
    fn as_ref_xyzw(&self) -> &XYZW<T> {
        self
    }

    #[inline(always)]
    fn as_mut_xyzw(&mut self) -> &mut XYZW<T> {
        self
    }

    #[inline]
    fn min_element(self) -> T {
        self.x.min(self.y.min(self.z.min(self.w)))
    }

    #[inline]
    fn max_element(self) -> T {
        self.x.max(self.y.max(self.z.max(self.w)))
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

impl<T: SignedEx> SignedVector2<T> for XY<T> {}

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

impl<T: SignedEx> SignedVector3<T> for XYZ<T> {}
impl<T: SignedEx> SignedVector4<T> for XYZW<T> {}

impl<T: FloatEx> FloatVector2<T> for XY<T> {}
impl<T: FloatEx> FloatVector3<T> for XYZ<T> {}
impl<T: FloatEx> FloatVector4<T> for XYZW<T> {}

impl<T> From<XYZ<T>> for XY<T> {
    #[inline(always)]
    fn from(v: XYZ<T>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl<T> From<XYZW<T>> for XY<T> {
    #[inline(always)]
    fn from(v: XYZW<T>) -> Self {
        Self { x: v.x, y: v.y }
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

impl VectorConst for XYZF32A16 {
    const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
}

impl NanConstEx for XYZF32A16 {
    const NAN: Self = Self {
        x: f32::NAN,
        y: f32::NAN,
        z: f32::NAN,
    };
}

impl Vector3Const for XYZF32A16 {
    const X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    const Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    const Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
}

impl Vector<f32> for XYZF32A16 {
    type Mask = XYZ<bool>;

    #[inline]
    fn splat(s: f32) -> Self {
        Self { x: s, y: s, z: s }
    }

    #[inline]
    fn select(mask: Self::Mask, if_true: Self, if_false: Self) -> Self {
        XYZ::select(mask, if_true.into(), if_false.into()).into()
    }

    #[inline]
    fn cmpeq(self, other: Self) -> Self::Mask {
        XYZ::cmpeq(self.into(), other.into())
    }

    #[inline]
    fn cmpne(self, other: Self) -> Self::Mask {
        XYZ::cmpne(self.into(), other.into())
    }

    #[inline]
    fn cmpge(self, other: Self) -> Self::Mask {
        XYZ::cmpge(self.into(), other.into())
    }

    #[inline]
    fn cmpgt(self, other: Self) -> Self::Mask {
        XYZ::cmpgt(self.into(), other.into())
    }

    #[inline]
    fn cmple(self, other: Self) -> Self::Mask {
        XYZ::cmple(self.into(), other.into())
    }

    #[inline]
    fn cmplt(self, other: Self) -> Self::Mask {
        XYZ::cmplt(self.into(), other.into())
    }

    #[inline]
    fn add(self, other: Self) -> Self {
        XYZ::add(self.into(), other.into()).into()
    }

    #[inline]
    fn div(self, other: Self) -> Self {
        XYZ::div(self.into(), other.into()).into()
    }

    #[inline]
    fn mul(self, other: Self) -> Self {
        XYZ::mul(self.into(), other.into()).into()
    }

    #[inline]
    fn rem(self, other: Self) -> Self {
        XYZ::rem(self.into(), other.into()).into()
    }

    #[inline]
    fn sub(self, other: Self) -> Self {
        XYZ::sub(self.into(), other.into()).into()
    }

    #[inline]
    fn add_scalar(self, other: f32) -> Self {
        XYZ::add_scalar(self.into(), other).into()
    }

    #[inline]
    fn sub_scalar(self, other: f32) -> Self {
        XYZ::sub_scalar(self.into(), other).into()
    }

    #[inline]
    fn mul_scalar(self, other: f32) -> Self {
        XYZ::mul_scalar(self.into(), other).into()
    }

    #[inline]
    fn div_scalar(self, other: f32) -> Self {
        XYZ::div_scalar(self.into(), other).into()
    }

    #[inline]
    fn rem_scalar(self, other: f32) -> Self {
        XYZ::rem_scalar(self.into(), other).into()
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        XYZ::min(self.into(), other.into()).into()
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        XYZ::max(self.into(), other.into()).into()
    }
}

impl Vector3<f32> for XYZF32A16 {
    #[inline(always)]
    fn new(x: f32, y: f32, z: f32) -> Self {
        XYZF32A16 { x, y, z }
    }

    #[inline(always)]
    fn x(self) -> f32 {
        self.x
    }

    #[inline(always)]
    fn y(self) -> f32 {
        self.y
    }

    #[inline(always)]
    fn z(self) -> f32 {
        self.z
    }

    #[inline(always)]
    fn as_ref_xyz(&self) -> &XYZ<f32> {
        unsafe { &*(self as *const Self).cast() }
    }

    #[inline(always)]
    fn as_mut_xyz(&mut self) -> &mut XYZ<f32> {
        unsafe { &mut *(self as *mut Self).cast() }
    }

    #[inline(always)]
    fn min_element(self) -> f32 {
        XYZ::min_element(self.into())
    }

    #[inline(always)]
    fn max_element(self) -> f32 {
        XYZ::max_element(self.into())
    }

    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {
        XYZ::clamp(self.into(), min.into(), max.into()).into()
    }
}

impl SignedVector<f32> for XYZF32A16 {
    #[inline(always)]
    fn neg(self) -> Self {
        XYZ::neg(self.into()).into()
    }
}

impl SignedVector3<f32> for XYZF32A16 {}
impl FloatVector3<f32> for XYZF32A16 {}

// 2D bitwise and shifting

impl<T, Rhs> ScalarShiftOps<Rhs> for XY<T>
where
    T: IntegerShiftOps<Rhs>,
    Rhs: Copy,
{
    #[inline(always)]
    fn scalar_shl(self, rhs: Rhs) -> Self {
        Self {
            x: self.x << rhs,
            y: self.y << rhs,
        }
    }

    #[inline(always)]
    fn scalar_shr(self, rhs: Rhs) -> Self {
        Self {
            x: self.x >> rhs,
            y: self.y >> rhs,
        }
    }
}

impl<T> ScalarBitOps<T> for XY<T>
where
    T: Copy + IntegerBitOps,
{
    #[inline(always)]
    fn scalar_bitand(self, rhs: T) -> Self {
        Self {
            x: self.x & rhs,
            y: self.y & rhs,
        }
    }

    #[inline(always)]
    fn scalar_bitor(self, rhs: T) -> Self {
        Self {
            x: self.x | rhs,
            y: self.y | rhs,
        }
    }

    #[inline(always)]
    fn scalar_bitxor(self, rhs: T) -> Self {
        Self {
            x: self.x ^ rhs,
            y: self.y ^ rhs,
        }
    }
}

impl<T, Rhs> VectorShiftOps<XY<Rhs>> for XY<T>
where
    T: Copy + IntegerShiftOps<Rhs>,
{
    #[inline(always)]
    fn vector_shl(self, rhs: XY<Rhs>) -> Self {
        Self {
            x: self.x << rhs.x,
            y: self.y << rhs.y,
        }
    }

    #[inline(always)]
    fn vector_shr(self, rhs: XY<Rhs>) -> Self {
        Self {
            x: self.x >> rhs.x,
            y: self.y >> rhs.y,
        }
    }
}

impl<T> VectorBitOps<XY<T>> for XY<T>
where
    T: Copy + IntegerBitOps,
{
    #[inline(always)]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }

    #[inline(always)]
    fn vector_bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
        }
    }

    #[inline(always)]
    fn vector_bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
        }
    }

    #[inline(always)]
    fn vector_bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
        }
    }
}

// 3D bitwise and shifting

impl<T, Rhs> ScalarShiftOps<Rhs> for XYZ<T>
where
    T: IntegerShiftOps<Rhs>,
    Rhs: Copy,
{
    #[inline(always)]
    fn scalar_shl(self, rhs: Rhs) -> Self {
        Self {
            x: self.x << rhs,
            y: self.y << rhs,
            z: self.z << rhs,
        }
    }

    #[inline(always)]
    fn scalar_shr(self, rhs: Rhs) -> Self {
        Self {
            x: self.x >> rhs,
            y: self.y >> rhs,
            z: self.z >> rhs,
        }
    }
}

impl<T> ScalarBitOps<T> for XYZ<T>
where
    T: Copy + IntegerBitOps,
{
    #[inline(always)]
    fn scalar_bitand(self, rhs: T) -> Self {
        Self {
            x: self.x & rhs,
            y: self.y & rhs,
            z: self.z & rhs,
        }
    }

    #[inline(always)]
    fn scalar_bitor(self, rhs: T) -> Self {
        Self {
            x: self.x | rhs,
            y: self.y | rhs,
            z: self.z | rhs,
        }
    }

    #[inline(always)]
    fn scalar_bitxor(self, rhs: T) -> Self {
        Self {
            x: self.x ^ rhs,
            y: self.y ^ rhs,
            z: self.z ^ rhs,
        }
    }
}

impl<T, Rhs> VectorShiftOps<XYZ<Rhs>> for XYZ<T>
where
    T: Copy + IntegerShiftOps<Rhs>,
{
    #[inline(always)]
    fn vector_shl(self, rhs: XYZ<Rhs>) -> Self {
        Self {
            x: self.x << rhs.x,
            y: self.y << rhs.y,
            z: self.z << rhs.z,
        }
    }

    #[inline(always)]
    fn vector_shr(self, rhs: XYZ<Rhs>) -> Self {
        Self {
            x: self.x >> rhs.x,
            y: self.y >> rhs.y,
            z: self.z >> rhs.z,
        }
    }
}

impl<T> VectorBitOps<XYZ<T>> for XYZ<T>
where
    T: Copy + IntegerBitOps,
{
    #[inline(always)]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }

    #[inline(always)]
    fn vector_bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
        }
    }

    #[inline(always)]
    fn vector_bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
        }
    }

    #[inline(always)]
    fn vector_bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
        }
    }
}

// 4D bitwise and shifting

impl<T, Rhs> ScalarShiftOps<Rhs> for XYZW<T>
where
    T: IntegerShiftOps<Rhs>,
    Rhs: Copy,
{
    #[inline(always)]
    fn scalar_shl(self, rhs: Rhs) -> Self {
        Self {
            x: self.x << rhs,
            y: self.y << rhs,
            z: self.z << rhs,
            w: self.w << rhs,
        }
    }

    #[inline(always)]
    fn scalar_shr(self, rhs: Rhs) -> Self {
        Self {
            x: self.x >> rhs,
            y: self.y >> rhs,
            z: self.z >> rhs,
            w: self.w >> rhs,
        }
    }
}

impl<T> ScalarBitOps<T> for XYZW<T>
where
    T: Copy + IntegerBitOps,
{
    #[inline(always)]
    fn scalar_bitand(self, rhs: T) -> Self {
        Self {
            x: self.x & rhs,
            y: self.y & rhs,
            z: self.z & rhs,
            w: self.w & rhs,
        }
    }

    #[inline(always)]
    fn scalar_bitor(self, rhs: T) -> Self {
        Self {
            x: self.x | rhs,
            y: self.y | rhs,
            z: self.z | rhs,
            w: self.w | rhs,
        }
    }

    #[inline(always)]
    fn scalar_bitxor(self, rhs: T) -> Self {
        Self {
            x: self.x ^ rhs,
            y: self.y ^ rhs,
            z: self.z ^ rhs,
            w: self.w ^ rhs,
        }
    }
}

impl<T, Rhs> VectorShiftOps<XYZW<Rhs>> for XYZW<T>
where
    T: Copy + IntegerShiftOps<Rhs>,
{
    #[inline(always)]
    fn vector_shl(self, rhs: XYZW<Rhs>) -> Self {
        Self {
            x: self.x << rhs.x,
            y: self.y << rhs.y,
            z: self.z << rhs.z,
            w: self.w << rhs.w,
        }
    }

    #[inline(always)]
    fn vector_shr(self, rhs: XYZW<Rhs>) -> Self {
        Self {
            x: self.x >> rhs.x,
            y: self.y >> rhs.y,
            z: self.z >> rhs.z,
            w: self.w >> rhs.w,
        }
    }
}

impl<T> VectorBitOps<XYZW<T>> for XYZW<T>
where
    T: Copy + IntegerBitOps,
{
    #[inline(always)]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
            w: !self.w,
        }
    }

    #[inline(always)]
    fn vector_bitand(self, rhs: Self) -> Self {
        Self {
            x: self.x & rhs.x,
            y: self.y & rhs.y,
            z: self.z & rhs.z,
            w: self.w & rhs.w,
        }
    }

    #[inline(always)]
    fn vector_bitor(self, rhs: Self) -> Self {
        Self {
            x: self.x | rhs.x,
            y: self.y | rhs.y,
            z: self.z | rhs.z,
            w: self.w | rhs.w,
        }
    }

    #[inline(always)]
    fn vector_bitxor(self, rhs: Self) -> Self {
        Self {
            x: self.x ^ rhs.x,
            y: self.y ^ rhs.y,
            z: self.z ^ rhs.z,
            w: self.w ^ rhs.w,
        }
    }
}
