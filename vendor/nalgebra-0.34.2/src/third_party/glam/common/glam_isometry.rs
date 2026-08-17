use super::glam::{DMat3, DMat4, DQuat, DVec2, DVec3, Mat3, Mat4, Quat, Vec2, Vec3};
use crate::{Isometry2, Isometry3, Matrix3, Matrix4, Translation3, UnitQuaternion};
use std::convert::TryFrom;

impl From<Isometry2<f32>> for Mat3 {
    fn from(iso: Isometry2<f32>) -> Mat3 {
        iso.to_homogeneous().into()
    }
}
impl From<Isometry3<f32>> for Mat4 {
    fn from(iso: Isometry3<f32>) -> Mat4 {
        iso.to_homogeneous().into()
    }
}

impl From<Isometry2<f64>> for DMat3 {
    fn from(iso: Isometry2<f64>) -> DMat3 {
        iso.to_homogeneous().into()
    }
}
impl From<Isometry3<f64>> for DMat4 {
    fn from(iso: Isometry3<f64>) -> DMat4 {
        iso.to_homogeneous().into()
    }
}

impl From<Isometry3<f32>> for (Vec3, Quat) {
    fn from(iso: Isometry3<f32>) -> (Vec3, Quat) {
        (iso.translation.into(), iso.rotation.into())
    }
}

impl From<Isometry3<f64>> for (DVec3, DQuat) {
    fn from(iso: Isometry3<f64>) -> (DVec3, DQuat) {
        (iso.translation.into(), iso.rotation.into())
    }
}

impl From<Isometry2<f32>> for (Vec2, f32) {
    fn from(iso: Isometry2<f32>) -> (Vec2, f32) {
        let tra = Vec2::new(iso.translation.x, iso.translation.y);
        let rot = iso.rotation.angle();
        (tra, rot)
    }
}

impl From<Isometry2<f64>> for (DVec2, f64) {
    fn from(iso: Isometry2<f64>) -> (DVec2, f64) {
        let tra = DVec2::new(iso.translation.x, iso.translation.y);
        let rot = iso.rotation.angle();
        (tra, rot)
    }
}

impl From<(Vec3, Quat)> for Isometry3<f32> {
    fn from((tra, rot): (Vec3, Quat)) -> Self {
        Isometry3::from_parts(tra.into(), rot.into())
    }
}

impl From<(DVec3, DQuat)> for Isometry3<f64> {
    fn from((tra, rot): (DVec3, DQuat)) -> Self {
        Isometry3::from_parts(tra.into(), rot.into())
    }
}

impl From<(Vec2, f32)> for Isometry2<f32> {
    fn from((tra, rot): (Vec2, f32)) -> Self {
        Isometry2::new(tra.into(), rot)
    }
}

impl From<(DVec2, f64)> for Isometry2<f64> {
    fn from((tra, rot): (DVec2, f64)) -> Self {
        Isometry2::new(tra.into(), rot)
    }
}

impl From<Quat> for Isometry3<f32> {
    fn from(rot: Quat) -> Self {
        Isometry3::from_parts(Translation3::identity(), rot.into())
    }
}

impl From<DQuat> for Isometry3<f64> {
    fn from(rot: DQuat) -> Self {
        Isometry3::from_parts(Translation3::identity(), rot.into())
    }
}

impl From<Vec3> for Isometry3<f32> {
    fn from(tra: Vec3) -> Self {
        Isometry3::from_parts(tra.into(), UnitQuaternion::identity())
    }
}

impl From<DVec3> for Isometry3<f64> {
    fn from(tra: DVec3) -> Self {
        Isometry3::from_parts(tra.into(), UnitQuaternion::identity())
    }
}

impl From<Vec2> for Isometry2<f32> {
    fn from(tra: Vec2) -> Self {
        Isometry2::new(tra.into(), 0.0)
    }
}

impl From<DVec2> for Isometry2<f64> {
    fn from(tra: DVec2) -> Self {
        Isometry2::new(tra.into(), 0.0)
    }
}

impl TryFrom<Mat3> for Isometry2<f32> {
    type Error = ();

    fn try_from(mat3: Mat3) -> Result<Isometry2<f32>, Self::Error> {
        crate::try_convert(Matrix3::from(mat3)).ok_or(())
    }
}

impl TryFrom<Mat4> for Isometry3<f32> {
    type Error = ();

    fn try_from(mat4: Mat4) -> Result<Isometry3<f32>, Self::Error> {
        crate::try_convert(Matrix4::from(mat4)).ok_or(())
    }
}

impl TryFrom<DMat3> for Isometry2<f64> {
    type Error = ();

    fn try_from(mat3: DMat3) -> Result<Isometry2<f64>, Self::Error> {
        crate::try_convert(Matrix3::from(mat3)).ok_or(())
    }
}

impl TryFrom<DMat4> for Isometry3<f64> {
    type Error = ();

    fn try_from(mat4: DMat4) -> Result<Isometry3<f64>, Self::Error> {
        crate::try_convert(Matrix4::from(mat4)).ok_or(())
    }
}
