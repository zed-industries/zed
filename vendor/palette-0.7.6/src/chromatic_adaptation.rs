//! Convert colors from one reference white point to another
//!
//! Chromatic adaptation is the human visual system’s ability to adjust to
//! changes in illumination in order to preserve the appearance of object
//! colors. It is responsible for the stable appearance of object colours
//! despite the wide variation of light which might be reflected from an object
//! and observed by our eyes.
//!
//! This library provides three methods for chromatic adaptation Bradford (which
//! is the default), VonKries and XyzScaling
//!
//! ```
//! use palette::Xyz;
//! use palette::white_point::{A, C};
//! use palette::chromatic_adaptation::AdaptInto;
//!
//!
//! let a = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);
//!
//! //Will convert Xyz<A, f32> to Xyz<C, f32> using Bradford chromatic adaptation
//! let c: Xyz<C, f32> = a.adapt_into();
//!
//! //Should print {x: 0.257963, y: 0.139776,z: 0.058825}
//! println!("{:?}", c)
//! ```

use crate::{
    convert::{FromColorUnclamped, IntoColorUnclamped},
    matrix::{multiply_3x3, multiply_xyz, Mat3},
    num::{Arithmetics, Real, Zero},
    white_point::{Any, WhitePoint},
    Xyz,
};

/// Chromatic adaptation methods implemented in the library
pub enum Method {
    /// Bradford chromatic adaptation method
    Bradford,
    /// VonKries chromatic adaptation method
    VonKries,
    /// XyzScaling chromatic adaptation method
    XyzScaling,
}

/// Holds the matrix coefficients for the chromatic adaptation methods
pub struct ConeResponseMatrices<T> {
    ///3x3 matrix for the cone response domains
    pub ma: Mat3<T>,
    ///3x3 matrix for the inverse of the cone response domains
    pub inv_ma: Mat3<T>,
}

/// Generates a conversion matrix to convert the Xyz tristimulus values from
/// one illuminant to another (`source_wp` to `destination_wp`)
pub trait TransformMatrix<T>
where
    T: Zero + Arithmetics + Clone,
{
    /// Get the cone response functions for the chromatic adaptation method
    #[must_use]
    fn get_cone_response(&self) -> ConeResponseMatrices<T>;

    /// Generates a 3x3 transformation matrix to convert color from one
    /// reference white point to another with the given cone_response
    #[must_use]
    fn generate_transform_matrix(
        &self,
        source_wp: Xyz<Any, T>,
        destination_wp: Xyz<Any, T>,
    ) -> Mat3<T> {
        let adapt = self.get_cone_response();

        let resp_src = multiply_xyz(adapt.ma.clone(), source_wp);
        let resp_dst = multiply_xyz(adapt.ma.clone(), destination_wp);

        #[rustfmt::skip]
        let resp = [
            resp_dst.x / resp_src.x, T::zero(), T::zero(),
            T::zero(), resp_dst.y / resp_src.y, T::zero(),
            T::zero(), T::zero(), resp_dst.z / resp_src.z,
        ];

        let tmp = multiply_3x3(resp, adapt.ma);
        multiply_3x3(adapt.inv_ma, tmp)
    }
}

impl<T> TransformMatrix<T> for Method
where
    T: Real + Zero + Arithmetics + Clone,
{
    #[rustfmt::skip]
    #[inline]
    fn get_cone_response(&self) -> ConeResponseMatrices<T> {
        match *self {
             Method::Bradford => {
                ConeResponseMatrices::<T> {
                    ma: [
                        T::from_f64(0.8951000), T::from_f64(0.2664000), T::from_f64(-0.1614000),
                        T::from_f64(-0.7502000), T::from_f64(1.7135000), T::from_f64(0.0367000),
                        T::from_f64(0.0389000), T::from_f64(-0.0685000), T::from_f64(1.0296000)
                    ],
                    inv_ma: [
                        T::from_f64(0.9869929), T::from_f64(-0.1470543), T::from_f64(0.1599627),
                        T::from_f64(0.4323053), T::from_f64(0.5183603), T::from_f64(0.0492912),
                        T::from_f64(-0.0085287), T::from_f64(0.0400428), T::from_f64(0.9684867)
                    ],
                }
            }
             Method::VonKries => {
                ConeResponseMatrices::<T> {
                    ma: [
                        T::from_f64(0.4002400), T::from_f64(0.7076000), T::from_f64(-0.0808100),
                        T::from_f64(-0.2263000), T::from_f64(1.1653200), T::from_f64(0.0457000),
                        T::from_f64(0.0000000), T::from_f64(0.0000000), T::from_f64(0.9182200)
                    ],
                    inv_ma: [
                        T::from_f64(1.8599364), T::from_f64(-1.1293816), T::from_f64(0.2198974),
                        T::from_f64(0.3611914), T::from_f64(0.6388125), T::from_f64(-0.0000064),
                        T::from_f64(0.0000000), T::from_f64(0.0000000), T::from_f64(1.0890636)
                    ],
                }
            }
             Method::XyzScaling => {
                ConeResponseMatrices::<T> {
                    ma: [
                        T::from_f64(1.0000000), T::from_f64(0.0000000), T::from_f64(0.0000000),
                        T::from_f64(0.0000000), T::from_f64(1.0000000), T::from_f64(0.0000000),
                        T::from_f64(0.0000000), T::from_f64(0.0000000), T::from_f64(1.0000000)
                    ],
                    inv_ma: [
                        T::from_f64(1.0000000), T::from_f64(0.0000000), T::from_f64(0.0000000),
                        T::from_f64(0.0000000), T::from_f64(1.0000000), T::from_f64(0.0000000),
                        T::from_f64(0.0000000), T::from_f64(0.0000000), T::from_f64(1.0000000)
                    ],
                }
            }
        }
    }
}

/// Trait to convert color from one reference white point to another
///
/// Converts a color from the source white point (Swp) to the destination white
/// point (Dwp). Uses the bradford method for conversion by default.
pub trait AdaptFrom<S, Swp, Dwp, T>: Sized
where
    T: Real + Zero + Arithmetics + Clone,
    Swp: WhitePoint<T>,
    Dwp: WhitePoint<T>,
{
    /// Convert the source color to the destination color using the bradford
    /// method by default.
    #[must_use]
    #[inline]
    fn adapt_from(color: S) -> Self {
        Self::adapt_from_using(color, Method::Bradford)
    }
    /// Convert the source color to the destination color using the specified
    /// method.
    #[must_use]
    fn adapt_from_using<M: TransformMatrix<T>>(color: S, method: M) -> Self;
}

impl<S, D, Swp, Dwp, T> AdaptFrom<S, Swp, Dwp, T> for D
where
    T: Real + Zero + Arithmetics + Clone,
    Swp: WhitePoint<T>,
    Dwp: WhitePoint<T>,
    S: IntoColorUnclamped<Xyz<Swp, T>>,
    D: FromColorUnclamped<Xyz<Dwp, T>>,
{
    #[inline]
    fn adapt_from_using<M: TransformMatrix<T>>(color: S, method: M) -> D {
        let src_xyz = color.into_color_unclamped().with_white_point();
        let transform_matrix = method.generate_transform_matrix(Swp::get_xyz(), Dwp::get_xyz());
        let dst_xyz = multiply_xyz(transform_matrix, src_xyz);
        D::from_color_unclamped(dst_xyz.with_white_point())
    }
}

/// Trait to convert color with one reference white point into another
///
/// Converts a color with the source white point (Swp) into the destination
/// white point (Dwp). Uses the bradford method for conversion by default.
pub trait AdaptInto<D, Swp, Dwp, T>: Sized
where
    T: Real + Zero + Arithmetics + Clone,
    Swp: WhitePoint<T>,
    Dwp: WhitePoint<T>,
{
    /// Convert the source color to the destination color using the bradford
    /// method by default.
    #[must_use]
    #[inline]
    fn adapt_into(self) -> D {
        self.adapt_into_using(Method::Bradford)
    }
    /// Convert the source color to the destination color using the specified
    /// method.
    #[must_use]
    fn adapt_into_using<M: TransformMatrix<T>>(self, method: M) -> D;
}

impl<S, D, Swp, Dwp, T> AdaptInto<D, Swp, Dwp, T> for S
where
    T: Real + Zero + Arithmetics + Clone,
    Swp: WhitePoint<T>,
    Dwp: WhitePoint<T>,
    D: AdaptFrom<S, Swp, Dwp, T>,
{
    #[inline]
    fn adapt_into_using<M: TransformMatrix<T>>(self, method: M) -> D {
        D::adapt_from_using(self, method)
    }
}

#[cfg(feature = "approx")]
#[cfg(test)]
mod test {
    use super::{AdaptFrom, AdaptInto, Method, TransformMatrix};
    use crate::white_point::{WhitePoint, A, C, D50, D65};
    use crate::Xyz;

    #[test]
    fn d65_to_d50_matrix_xyz_scaling() {
        let expected = [
            1.0144665, 0.0000000, 0.0000000, 0.0000000, 1.0000000, 0.0000000, 0.0000000, 0.0000000,
            0.7578869,
        ];
        let xyz_scaling = Method::XyzScaling;
        let computed = xyz_scaling.generate_transform_matrix(D65::get_xyz(), D50::get_xyz());
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.0001)
        }
    }
    #[test]
    fn d65_to_d50_matrix_von_kries() {
        let expected = [
            1.0160803, 0.0552297, -0.0521326, 0.0060666, 0.9955661, -0.0012235, 0.0000000,
            0.0000000, 0.7578869,
        ];
        let von_kries = Method::VonKries;
        let computed = von_kries.generate_transform_matrix(D65::get_xyz(), D50::get_xyz());
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.0001)
        }
    }
    #[test]
    fn d65_to_d50_matrix_bradford() {
        let expected = [
            1.0478112, 0.0228866, -0.0501270, 0.0295424, 0.9904844, -0.0170491, -0.0092345,
            0.0150436, 0.7521316,
        ];
        let bradford = Method::Bradford;
        let computed = bradford.generate_transform_matrix(D65::get_xyz(), D50::get_xyz());
        for (e, c) in expected.iter().zip(computed.iter()) {
            assert_relative_eq!(e, c, epsilon = 0.0001)
        }
    }

    #[test]
    fn chromatic_adaptation_from_a_to_c() {
        let input_a = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);

        let expected_bradford = Xyz::<C, f32>::new(0.257963, 0.139776, 0.058825);
        let expected_vonkries = Xyz::<C, f32>::new(0.268446, 0.159139, 0.052843);
        let expected_xyz_scaling = Xyz::<C, f32>::new(0.281868, 0.162732, 0.052844);

        let computed_bradford: Xyz<C, f32> = Xyz::adapt_from(input_a);
        assert_relative_eq!(expected_bradford, computed_bradford, epsilon = 0.0001);

        let computed_vonkries: Xyz<C, f32> = Xyz::adapt_from_using(input_a, Method::VonKries);
        assert_relative_eq!(expected_vonkries, computed_vonkries, epsilon = 0.0001);

        let computed_xyz_scaling: Xyz<C, _> = Xyz::adapt_from_using(input_a, Method::XyzScaling);
        assert_relative_eq!(expected_xyz_scaling, computed_xyz_scaling, epsilon = 0.0001);
    }

    #[test]
    fn chromatic_adaptation_into_a_to_c() {
        let input_a = Xyz::<A, f32>::new(0.315756, 0.162732, 0.015905);

        let expected_bradford = Xyz::<C, f32>::new(0.257963, 0.139776, 0.058825);
        let expected_vonkries = Xyz::<C, f32>::new(0.268446, 0.159139, 0.052843);
        let expected_xyz_scaling = Xyz::<C, f32>::new(0.281868, 0.162732, 0.052844);

        let computed_bradford: Xyz<C, f32> = input_a.adapt_into();
        assert_relative_eq!(expected_bradford, computed_bradford, epsilon = 0.0001);

        let computed_vonkries: Xyz<C, f32> = input_a.adapt_into_using(Method::VonKries);
        assert_relative_eq!(expected_vonkries, computed_vonkries, epsilon = 0.0001);

        let computed_xyz_scaling: Xyz<C, _> = input_a.adapt_into_using(Method::XyzScaling);
        assert_relative_eq!(expected_xyz_scaling, computed_xyz_scaling, epsilon = 0.0001);
    }
}
