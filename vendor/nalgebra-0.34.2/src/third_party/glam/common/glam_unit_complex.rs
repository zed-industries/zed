use super::glam::{DMat2, Mat2};
use crate::{Complex, UnitComplex};

impl From<UnitComplex<f32>> for Mat2 {
    #[inline]
    fn from(e: UnitComplex<f32>) -> Mat2 {
        e.to_rotation_matrix().into_inner().into()
    }
}

impl From<UnitComplex<f64>> for DMat2 {
    #[inline]
    fn from(e: UnitComplex<f64>) -> DMat2 {
        e.to_rotation_matrix().into_inner().into()
    }
}

impl From<Mat2> for UnitComplex<f32> {
    #[inline]
    fn from(e: Mat2) -> UnitComplex<f32> {
        UnitComplex::new_normalize(Complex::new(e.x_axis.x, e.x_axis.y))
    }
}

impl From<DMat2> for UnitComplex<f64> {
    #[inline]
    fn from(e: DMat2) -> UnitComplex<f64> {
        UnitComplex::new_normalize(Complex::new(e.x_axis.x, e.x_axis.y))
    }
}
