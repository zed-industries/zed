#![cfg(feature = "serde-serialize")]

use na::{
    DMatrix, Isometry2, Isometry3, IsometryMatrix2, IsometryMatrix3, Matrix2x3, Matrix3x4, Point2,
    Point3, Quaternion, Rotation2, Rotation3, Similarity2, Similarity3, SimilarityMatrix2,
    SimilarityMatrix3, Translation2, Translation3, Unit, Vector2,
};
use rand;
use serde::{Deserialize, Serialize};
use serde_json;

macro_rules! test_serde(
    ($($test: ident, $ty: ident);* $(;)*) => {$(
        #[test]
        fn $test() {
            let v: $ty<f32> = rand::random();
            let serialized = serde_json::to_string(&v).unwrap();
            let deserialized: $ty<f32> = serde_json::from_str(&serialized).unwrap();
            assert_eq!(v, deserialized);
        }
    )*}
);

#[test]
fn serde_dmatrix() {
    let v: DMatrix<f32> = DMatrix::new_random(3, 4);
    let serialized = serde_json::to_string(&v).unwrap();
    let deserialized: DMatrix<f32> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(v, deserialized);

    let m = DMatrix::from_column_slice(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let mat_str = "[[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],2,3]";
    let deserialized: DMatrix<f32> = serde_json::from_str(&mat_str).unwrap();
    assert_eq!(m, deserialized);

    let m = Matrix2x3::from_column_slice(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let mat_str = "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]";
    let deserialized: Matrix2x3<f32> = serde_json::from_str(&mat_str).unwrap();
    assert_eq!(m, deserialized);
}

#[test]
#[should_panic]
fn serde_dmatrix_invalid_len() {
    // This must fail: we attempt to deserialize a 2x3 with only 5 elements.
    let mat_str = "[[1.0, 2.0, 3.0, 4.0, 5.0],2,3]";
    let _: DMatrix<f32> = serde_json::from_str(&mat_str).unwrap();
}

#[test]
#[should_panic]
fn serde_smatrix_invalid_len() {
    // This must fail: we attempt to deserialize a 2x3 with only 5 elements.
    let mat_str = "[1.0, 2.0, 3.0, 4.0, 5.0]";
    let _: Matrix2x3<f32> = serde_json::from_str(&mat_str).unwrap();
}

test_serde!(
    serde_matrix3x4,          Matrix3x4;
    serde_point3,             Point3;
    serde_translation3,       Translation3;
    serde_rotation3,          Rotation3;
    serde_isometry3,          Isometry3;
    serde_isometry_matrix3,   IsometryMatrix3;
    serde_similarity3,        Similarity3;
    serde_similarity_matrix3, SimilarityMatrix3;
    serde_quaternion,         Quaternion;
    serde_point2,             Point2;
    serde_translation2,       Translation2;
    serde_rotation2,          Rotation2;
    serde_isometry2,          Isometry2;
    serde_isometry_matrix2,   IsometryMatrix2;
    serde_similarity2,        Similarity2;
    serde_similarity_matrix2, SimilarityMatrix2;
);

#[test]
fn serde_flat() {
    // The actual storage is hidden behind three layers of wrapper types that shouldn't appear in serialized form.
    let v = Unit::new_normalize(Quaternion::new(0., 0., 0., 1.));
    let serialized = serde_json::to_string(&v).unwrap();
    assert_eq!(serialized, "[0.0,0.0,1.0,0.0]");
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
enum Stuff {
    A(f64),
    B(f64),
}

#[test]
fn deserialize_enum() {
    let json = r#"[{"letter":"A", "value":123.4}, {"letter":"B", "value":567.8}]"#;
    let parsed: Result<Vector2<Stuff>, _> = serde_json::from_str(json);
    println!("parsed: {:?}", parsed);
}
