use crate::{Quaternion, Scalar, SimdValue, UnitQuaternion};

impl<T: Scalar> From<mint::Quaternion<T>> for Quaternion<T> {
    fn from(q: mint::Quaternion<T>) -> Self {
        Self::new(q.s, q.v.x, q.v.y, q.v.z)
    }
}

impl<T: Scalar> Into<mint::Quaternion<T>> for Quaternion<T> {
    fn into(self) -> mint::Quaternion<T> {
        mint::Quaternion {
            v: mint::Vector3 {
                x: self[0].clone(),
                y: self[1].clone(),
                z: self[2].clone(),
            },
            s: self[3].clone(),
        }
    }
}

impl<T: Scalar + SimdValue> Into<mint::Quaternion<T>> for UnitQuaternion<T> {
    fn into(self) -> mint::Quaternion<T> {
        mint::Quaternion {
            v: mint::Vector3 {
                x: self[0].clone(),
                y: self[1].clone(),
                z: self[2].clone(),
            },
            s: self[3].clone(),
        }
    }
}
