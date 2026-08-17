use super::glam::{DMat3, DMat4, Mat3, Mat4};
use crate::{Matrix3, Matrix4, Similarity2, Similarity3};
use std::convert::TryFrom;

impl From<Similarity2<f32>> for Mat3 {
    fn from(iso: Similarity2<f32>) -> Mat3 {
        iso.to_homogeneous().into()
    }
}
impl From<Similarity3<f32>> for Mat4 {
    fn from(iso: Similarity3<f32>) -> Mat4 {
        iso.to_homogeneous().into()
    }
}

impl From<Similarity2<f64>> for DMat3 {
    fn from(iso: Similarity2<f64>) -> DMat3 {
        iso.to_homogeneous().into()
    }
}
impl From<Similarity3<f64>> for DMat4 {
    fn from(iso: Similarity3<f64>) -> DMat4 {
        iso.to_homogeneous().into()
    }
}

impl TryFrom<Mat3> for Similarity2<f32> {
    type Error = ();
    fn try_from(mat3: Mat3) -> Result<Similarity2<f32>, ()> {
        crate::try_convert(Matrix3::from(mat3)).ok_or(())
    }
}

impl TryFrom<Mat4> for Similarity3<f32> {
    type Error = ();
    fn try_from(mat4: Mat4) -> Result<Similarity3<f32>, ()> {
        crate::try_convert(Matrix4::from(mat4)).ok_or(())
    }
}

impl TryFrom<DMat3> for Similarity2<f64> {
    type Error = ();
    fn try_from(mat3: DMat3) -> Result<Similarity2<f64>, ()> {
        crate::try_convert(Matrix3::from(mat3)).ok_or(())
    }
}

impl TryFrom<DMat4> for Similarity3<f64> {
    type Error = ();
    fn try_from(mat4: DMat4) -> Result<Similarity3<f64>, ()> {
        crate::try_convert(Matrix4::from(mat4)).ok_or(())
    }
}
