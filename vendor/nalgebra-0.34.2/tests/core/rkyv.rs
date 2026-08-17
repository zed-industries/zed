#![cfg(feature = "rkyv-serialize")]

use na::{
    Isometry3, IsometryMatrix2, IsometryMatrix3, Matrix3x4, Point2, Point3, Quaternion, Rotation2,
    Rotation3, Similarity3, SimilarityMatrix2, SimilarityMatrix3, Translation2, Translation3,
};
use rand;
use rkyv::{Deserialize, Infallible};

macro_rules! test_rkyv_same_type(
    ($($test: ident, $ty: ident);* $(;)*) => {$(
        #[test]
        fn $test() {
            let value: $ty<f32> = rand::random();
			let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

			let archived = rkyv::check_archived_root::<$ty<f32>>(&bytes[..]).unwrap();
            // Compare archived and non-archived
			assert_eq!(archived, &value);

            // Make sure Debug implementations are the same for Archived and non-Archived versions.
			assert_eq!(format!("{:?}", value), format!("{:?}", archived));
        }
    )*}
);
macro_rules! test_rkyv_diff_type(
    ($($test: ident, $ty: ident);* $(;)*) => {$(
        #[test]
        fn $test() {
            let value: $ty<String> = Default::default();
			let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

			let archived = rkyv::check_archived_root::<$ty<String>>(&bytes[..]).unwrap();
            let deserialized: $ty<String> = archived.deserialize(&mut Infallible).unwrap();
			assert_eq!(deserialized, value);
        }
    )*}
);

// Tests to make sure
test_rkyv_same_type!(
    rkyv_same_type_matrix3x4,          Matrix3x4;
    rkyv_same_type_point3,             Point3;
    rkyv_same_type_translation3,       Translation3;
    rkyv_same_type_rotation3,          Rotation3;
    rkyv_same_type_isometry3,          Isometry3;
    rkyv_same_type_isometry_matrix3,   IsometryMatrix3;
    rkyv_same_type_similarity3,        Similarity3;
    rkyv_same_type_similarity_matrix3, SimilarityMatrix3;
    rkyv_same_type_quaternion,         Quaternion;
    rkyv_same_type_point2,             Point2;
    rkyv_same_type_translation2,       Translation2;
    rkyv_same_type_rotation2,          Rotation2;
    // rkyv_same_type_isometry2,          Isometry2;
    rkyv_same_type_isometry_matrix2,   IsometryMatrix2;
    // rkyv_same_type_similarity2,        Similarity2;
    rkyv_same_type_similarity_matrix2, SimilarityMatrix2;
);

test_rkyv_diff_type! {
    rkyv_diff_type_matrix3x4,          Matrix3x4;
}
