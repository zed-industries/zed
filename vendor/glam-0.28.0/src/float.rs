/// A trait for extending [`prim@f32`] and [`prim@f64`] with extra methods.
pub trait FloatExt {
    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0`, the result will be `self`.  When `s` is `1`, the result
    /// will be `rhs`. When `s` is outside of the range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[must_use]
    fn lerp(self, rhs: Self, s: Self) -> Self;

    /// Returns `v` normalized to the range `[a, b]`.
    ///
    /// When `v` is equal to `a` the result will be `0`.  When `v` is equal to `b` will be `1`.
    ///
    /// When `v` is outside of the range `[a, b]`, the result is linearly extrapolated.
    ///
    /// `a` and `b` must not be equal, otherwise the result will be either infinite or `NAN`.
    fn inverse_lerp(a: Self, b: Self, v: Self) -> Self;

    /// Remap `self` from the input range to the output range.
    ///
    /// When `self` is equal to `in_start` this returns `out_start`.
    /// When `self` is equal to `in_end` this returns `out_end`.
    ///
    /// When `self` is outside of the range `[in_start, in_end]`, the result is linearly extrapolated.
    ///
    /// `in_start` and `in_end` must not be equal, otherwise the result will be either infinite or `NAN`.
    #[must_use]
    fn remap(self, in_start: Self, in_end: Self, out_start: Self, out_end: Self) -> Self;
}
