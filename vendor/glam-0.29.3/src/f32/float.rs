// Generated from float.rs.tera template. Edit the template, not the generated file.

use crate::float::FloatExt;

impl FloatExt for f32 {
    #[inline]
    fn lerp(self, rhs: f32, t: f32) -> f32 {
        self + (rhs - self) * t
    }

    #[inline]
    fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
        (v - a) / (b - a)
    }

    #[inline]
    fn remap(self, in_start: f32, in_end: f32, out_start: f32, out_end: f32) -> f32 {
        let t = f32::inverse_lerp(in_start, in_end, self);
        f32::lerp(out_start, out_end, t)
    }
}
