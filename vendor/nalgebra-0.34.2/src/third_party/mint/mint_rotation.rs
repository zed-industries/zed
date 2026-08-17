use crate::{RealField, Rotation3};

impl<T: RealField> From<mint::EulerAngles<T, mint::IntraXYZ>> for Rotation3<T> {
    fn from(euler: mint::EulerAngles<T, mint::IntraXYZ>) -> Self {
        Self::from_euler_angles(euler.a, euler.b, euler.c)
    }
}
