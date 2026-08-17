use crate::geometry::{TAffine, TGeneral, TProjective, Transform};

/// A 2D general transformation that may not be invertible. Stored as a homogeneous 3x3 matrix.
pub type Transform2<T> = Transform<T, TGeneral, 2>;
/// An invertible 2D general transformation. Stored as a homogeneous 3x3 matrix.
pub type Projective2<T> = Transform<T, TProjective, 2>;
/// A 2D affine transformation. Stored as a homogeneous 3x3 matrix.
pub type Affine2<T> = Transform<T, TAffine, 2>;

/// A 3D general transformation that may not be inversible. Stored as a homogeneous 4x4 matrix.
pub type Transform3<T> = Transform<T, TGeneral, 3>;
/// An invertible 3D general transformation. Stored as a homogeneous 4x4 matrix.
pub type Projective3<T> = Transform<T, TProjective, 3>;
/// A 3D affine transformation. Stored as a homogeneous 4x4 matrix.
pub type Affine3<T> = Transform<T, TAffine, 3>;
