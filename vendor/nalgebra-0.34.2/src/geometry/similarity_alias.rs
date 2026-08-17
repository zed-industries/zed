use crate::geometry::{Rotation2, Rotation3, Similarity, UnitComplex, UnitQuaternion};

/// A 2-dimensional similarity.
pub type Similarity2<T> = Similarity<T, UnitComplex<T>, 2>;

/// A 3-dimensional similarity.
pub type Similarity3<T> = Similarity<T, UnitQuaternion<T>, 3>;

/// A 2-dimensional similarity using a rotation matrix for its rotation part.
pub type SimilarityMatrix2<T> = Similarity<T, Rotation2<T>, 2>;

/// A 3-dimensional similarity using a rotation matrix for its rotation part.
pub type SimilarityMatrix3<T> = Similarity<T, Rotation3<T>, 3>;
