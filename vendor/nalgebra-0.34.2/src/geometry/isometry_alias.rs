use crate::geometry::{Isometry, Rotation2, Rotation3, UnitComplex, UnitQuaternion};

/// A 2-dimensional direct isometry using a unit complex number for its rotational part.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Isometry`] type too.**
///
/// Also known as a 2D rigid-body motion, or as an element of SE(2).
pub type Isometry2<T> = Isometry<T, UnitComplex<T>, 2>;

/// A 3-dimensional direct isometry using a unit quaternion for its rotational part.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Isometry`] type too.**
///
/// Also known as a rigid-body motion, or as an element of SE(3).
pub type Isometry3<T> = Isometry<T, UnitQuaternion<T>, 3>;

/// A 2-dimensional direct isometry using a rotation matrix for its rotational part.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Isometry`] type too.**
///
/// Also known as a rigid-body motion, or as an element of SE(2).
pub type IsometryMatrix2<T> = Isometry<T, Rotation2<T>, 2>;

/// A 3-dimensional direct isometry using a rotation matrix for its rotational part.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Isometry`] type too.**
///
/// Also known as a rigid-body motion, or as an element of SE(3).
pub type IsometryMatrix3<T> = Isometry<T, Rotation3<T>, 3>;

// This tests that the types correctly implement `Copy`, without having to run tests
// (when targeting no-std for example).
#[allow(dead_code)]
const fn ensure_copy() {
    const fn is_copy<T: Copy>() {}

    is_copy::<IsometryMatrix2<f32>>();
    is_copy::<IsometryMatrix3<f32>>();
    is_copy::<Isometry2<f32>>();
    is_copy::<Isometry3<f32>>();
}
