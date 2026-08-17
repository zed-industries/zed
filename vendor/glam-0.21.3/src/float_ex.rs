#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::Float;

pub(crate) trait FloatEx {
    /// Returns a very close approximation of `self.clamp(-1.0, 1.0).acos()`.
    fn acos_approx(self) -> Self;
}

impl FloatEx for f32 {
    #[inline(always)]
    fn acos_approx(self) -> Self {
        // Based on https://github.com/microsoft/DirectXMath `XMScalarAcos`
        // Clamp input to [-1,1].
        let nonnegative = self >= 0.0;
        let x = self.abs();
        let mut omx = 1.0 - x;
        if omx < 0.0 {
            omx = 0.0;
        }
        let root = omx.sqrt();

        // 7-degree minimax approximation
        #[allow(clippy::approx_constant)]
        let mut result = ((((((-0.001_262_491_1 * x + 0.006_670_09) * x - 0.017_088_126) * x
            + 0.030_891_88)
            * x
            - 0.050_174_303)
            * x
            + 0.088_978_99)
            * x
            - 0.214_598_8)
            * x
            + 1.570_796_3;
        result *= root;

        // acos(x) = pi - acos(-x) when x < 0
        if nonnegative {
            result
        } else {
            core::f32::consts::PI - result
        }
    }
}

impl FloatEx for f64 {
    #[inline(always)]
    fn acos_approx(self) -> Self {
        f64::acos(self.max(-1.0).min(1.0))
    }
}
