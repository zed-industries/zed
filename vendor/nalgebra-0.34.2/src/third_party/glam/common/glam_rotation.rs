use super::glam::{DMat2, DQuat, Mat2, Quat};
use crate::{Rotation2, Rotation3, UnitComplex, UnitQuaternion};

impl From<Rotation2<f32>> for Mat2 {
    #[inline]
    fn from(e: Rotation2<f32>) -> Mat2 {
        e.into_inner().into()
    }
}

impl From<Rotation2<f64>> for DMat2 {
    #[inline]
    fn from(e: Rotation2<f64>) -> DMat2 {
        e.into_inner().into()
    }
}

impl From<Rotation3<f32>> for Quat {
    #[inline]
    fn from(e: Rotation3<f32>) -> Quat {
        UnitQuaternion::from(e).into()
    }
}

impl From<Rotation3<f64>> for DQuat {
    #[inline]
    fn from(e: Rotation3<f64>) -> DQuat {
        UnitQuaternion::from(e).into()
    }
}

impl From<Mat2> for Rotation2<f32> {
    #[inline]
    fn from(e: Mat2) -> Rotation2<f32> {
        UnitComplex::from(e).to_rotation_matrix()
    }
}

impl From<DMat2> for Rotation2<f64> {
    #[inline]
    fn from(e: DMat2) -> Rotation2<f64> {
        UnitComplex::from(e).to_rotation_matrix()
    }
}

impl From<Quat> for Rotation3<f32> {
    #[inline]
    fn from(e: Quat) -> Rotation3<f32> {
        Rotation3::from(UnitQuaternion::from(e))
    }
}

impl From<DQuat> for Rotation3<f64> {
    #[inline]
    fn from(e: DQuat) -> Rotation3<f64> {
        Rotation3::from(UnitQuaternion::from(e))
    }
}
