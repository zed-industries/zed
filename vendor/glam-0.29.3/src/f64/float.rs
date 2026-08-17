// Generated from float.rs.tera template. Edit the template, not the generated file.

use crate::float::FloatExt;

impl FloatExt for f64 {
    #[inline]
    fn lerp(self, rhs: f64, t: f64) -> f64 {
        self + (rhs - self) * t
    }

    #[inline]
    fn inverse_lerp(a: f64, b: f64, v: f64) -> f64 {
        (v - a) / (b - a)
    }

    #[inline]
    fn remap(self, in_start: f64, in_end: f64, out_start: f64, out_end: f64) -> f64 {
        let t = f64::inverse_lerp(in_start, in_end, self);
        f64::lerp(out_start, out_end, t)
    }
}
