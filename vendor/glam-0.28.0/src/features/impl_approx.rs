use crate::{
    Affine2, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, Mat2,
    Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4,
};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

macro_rules! impl_approx_as_ref {
    ($prim:ident, $type:ty) => {
        impl AbsDiffEq for $type {
            type Epsilon = <$prim as AbsDiffEq>::Epsilon;
            fn default_epsilon() -> Self::Epsilon {
                $prim::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                self.as_ref().abs_diff_eq(other.as_ref(), epsilon)
            }
        }

        impl RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                $prim::default_max_relative()
            }
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                self.as_ref()
                    .relative_eq(other.as_ref(), epsilon, max_relative)
            }
        }

        impl UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                $prim::default_max_ulps()
            }
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.as_ref().ulps_eq(other.as_ref(), epsilon, max_ulps)
            }
        }
    };
}

macro_rules! impl_approx_xzy_axes {
    ($prim:ident, $type:ty) => {
        impl AbsDiffEq for $type {
            type Epsilon = <$prim as AbsDiffEq>::Epsilon;
            fn default_epsilon() -> Self::Epsilon {
                $prim::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                AbsDiffEq::abs_diff_eq(&self.x_axis, &other.x_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.y_axis, &other.y_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.z_axis, &other.z_axis, epsilon)
            }
        }

        impl RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                $prim::default_max_relative()
            }
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                RelativeEq::relative_eq(&self.x_axis, &other.x_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.y_axis, &other.y_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.z_axis, &other.z_axis, epsilon, max_relative)
            }
        }

        impl UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                $prim::default_max_ulps()
            }
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                UlpsEq::ulps_eq(&self.x_axis, &other.x_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.y_axis, &other.y_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.z_axis, &other.z_axis, epsilon, max_ulps)
            }
        }
    };
}

macro_rules! impl_approx_xzyw_axes {
    ($prim:ident, $type:ty) => {
        impl AbsDiffEq for $type {
            type Epsilon = <$prim as AbsDiffEq>::Epsilon;
            fn default_epsilon() -> Self::Epsilon {
                $prim::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                AbsDiffEq::abs_diff_eq(&self.x_axis, &other.x_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.y_axis, &other.y_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.z_axis, &other.z_axis, epsilon)
                    && AbsDiffEq::abs_diff_eq(&self.w_axis, &other.w_axis, epsilon)
            }
        }

        impl RelativeEq for $type {
            fn default_max_relative() -> Self::Epsilon {
                $prim::default_max_relative()
            }
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                RelativeEq::relative_eq(&self.x_axis, &other.x_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.y_axis, &other.y_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.z_axis, &other.z_axis, epsilon, max_relative)
                    && RelativeEq::relative_eq(&self.w_axis, &other.w_axis, epsilon, max_relative)
            }
        }

        impl UlpsEq for $type {
            fn default_max_ulps() -> u32 {
                $prim::default_max_ulps()
            }
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                UlpsEq::ulps_eq(&self.x_axis, &other.x_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.y_axis, &other.y_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.z_axis, &other.z_axis, epsilon, max_ulps)
                    && UlpsEq::ulps_eq(&self.w_axis, &other.w_axis, epsilon, max_ulps)
            }
        }
    };
}

impl_approx_as_ref!(f32, Mat2);
impl_approx_as_ref!(f32, Mat3);
impl_approx_as_ref!(f32, Mat4);
impl_approx_as_ref!(f32, Quat);
impl_approx_as_ref!(f32, Vec2);
impl_approx_as_ref!(f32, Vec3);
impl_approx_as_ref!(f32, Vec4);
impl_approx_as_ref!(f32, Vec3A);

impl_approx_xzy_axes!(f32, Affine2);
impl_approx_xzyw_axes!(f32, Affine3A);
impl_approx_xzy_axes!(f32, Mat3A);

impl_approx_xzy_axes!(f64, DAffine2);
impl_approx_xzyw_axes!(f64, DAffine3);
impl_approx_as_ref!(f64, DMat2);
impl_approx_as_ref!(f64, DMat3);
impl_approx_as_ref!(f64, DMat4);
impl_approx_as_ref!(f64, DQuat);
impl_approx_as_ref!(f64, DVec2);
impl_approx_as_ref!(f64, DVec3);
impl_approx_as_ref!(f64, DVec4);

#[cfg(test)]
mod test {
    use crate::*;
    use approx::*;

    macro_rules! impl_approx_test {
        ($prim:ident, $type:ident, $ones:expr) => {
            let one_eps = $ones * $type::default_epsilon();
            let two_eps = one_eps + one_eps;

            let one_ulp = $ones * $prim::from_bits($prim::to_bits(1.0) + 1);
            let four_ulp = $ones * $prim::from_bits($prim::to_bits(1.0) + 16);

            approx::assert_abs_diff_eq!($ones, $ones);
            approx::assert_abs_diff_eq!($ones, $ones + one_eps);
            approx::assert_abs_diff_eq!($ones, $ones - one_eps);

            approx::assert_abs_diff_ne!($ones, $ones + two_eps);
            approx::assert_abs_diff_ne!($ones, $ones - two_eps);

            approx::assert_relative_eq!($ones, $ones);
            approx::assert_relative_ne!($ones, $ones - $ones);

            // defaults to 4 ulps and I have no idea how to pass other parameters to this macro :)
            approx::assert_ulps_eq!($ones, one_ulp);
            approx::assert_ulps_ne!($ones, four_ulp);
        };
        ($prim:ident, $type:ident) => {
            impl_approx_test!($prim, $type, $type::ONE)
        };
    }

    #[test]
    fn test_approx() {
        const ONESF32: [f32; 16] = [1.0; 16];

        impl_approx_test!(f32, Vec2);
        impl_approx_test!(f32, Vec3);
        impl_approx_test!(f32, Vec3A);
        impl_approx_test!(f32, Vec4);
        impl_approx_test!(f32, Quat, Quat::from_slice(&ONESF32));
        impl_approx_test!(f32, Mat2, Mat2::from_cols_slice(&ONESF32));
        impl_approx_test!(f32, Mat3, Mat3::from_cols_slice(&ONESF32));
        impl_approx_test!(f32, Mat3A, Mat3A::from_cols_slice(&ONESF32));
        impl_approx_test!(f32, Mat4, Mat4::from_cols_slice(&ONESF32));

        const ONESF64: [f64; 16] = [1.0; 16];
        impl_approx_test!(f64, DVec2);
        impl_approx_test!(f64, DVec3);
        impl_approx_test!(f64, DVec4);
        impl_approx_test!(f64, DQuat, DQuat::from_slice(&ONESF64));
        impl_approx_test!(f64, DMat2, DMat2::from_cols_slice(&ONESF64));
        impl_approx_test!(f64, DMat3, DMat3::from_cols_slice(&ONESF64));
        impl_approx_test!(f64, DMat4, DMat4::from_cols_slice(&ONESF64));
    }
}
