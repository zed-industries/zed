// Generated from float.rs.tera template. Edit the template, not the generated file.

use crate::float::FloatExt;

impl FloatExt for f64 {
    #[inline]
    fn lerp(self, rhs: Self, t: Self) -> Self {
        self + (rhs - self) * t
    }

    #[inline]
    fn inverse_lerp(a: Self, b: Self, v: Self) -> Self {
        (v - a) / (b - a)
    }

    #[inline]
    fn remap(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self {
        let t = Self::inverse_lerp(in_start, in_end, self);
        Self::lerp(out_start, out_end, t)
    }

    #[inline]
    fn fract_gl(self) -> Self {
        self - crate::f64::math::floor(self)
    }
}
