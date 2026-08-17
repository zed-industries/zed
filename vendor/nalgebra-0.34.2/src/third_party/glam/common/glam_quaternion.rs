use super::glam::{DQuat, Quat};
use crate::{Quaternion, UnitQuaternion};

impl From<Quat> for Quaternion<f32> {
    #[inline]
    fn from(e: Quat) -> Quaternion<f32> {
        Quaternion::new(e.w, e.x, e.y, e.z)
    }
}

impl From<Quaternion<f32>> for Quat {
    #[inline]
    fn from(e: Quaternion<f32>) -> Quat {
        Quat::from_xyzw(e.i, e.j, e.k, e.w)
    }
}

impl From<UnitQuaternion<f32>> for Quat {
    #[inline]
    fn from(e: UnitQuaternion<f32>) -> Quat {
        Quat::from_xyzw(e.i, e.j, e.k, e.w)
    }
}

impl From<DQuat> for Quaternion<f64> {
    #[inline]
    fn from(e: DQuat) -> Quaternion<f64> {
        Quaternion::new(e.w, e.x, e.y, e.z)
    }
}

impl From<Quaternion<f64>> for DQuat {
    #[inline]
    fn from(e: Quaternion<f64>) -> DQuat {
        DQuat::from_xyzw(e.i, e.j, e.k, e.w)
    }
}

impl From<UnitQuaternion<f64>> for DQuat {
    #[inline]
    fn from(e: UnitQuaternion<f64>) -> DQuat {
        DQuat::from_xyzw(e.i, e.j, e.k, e.w)
    }
}

impl From<Quat> for UnitQuaternion<f32> {
    #[inline]
    fn from(e: Quat) -> UnitQuaternion<f32> {
        UnitQuaternion::new_normalize(Quaternion::from(e))
    }
}

impl From<DQuat> for UnitQuaternion<f64> {
    #[inline]
    fn from(e: DQuat) -> UnitQuaternion<f64> {
        UnitQuaternion::new_normalize(Quaternion::from(e))
    }
}
