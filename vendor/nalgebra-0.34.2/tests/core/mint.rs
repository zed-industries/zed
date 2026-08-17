use mint;
use na::{Matrix2, Matrix2x3, Matrix3, Matrix3x4, Matrix4, Quaternion, Vector2, Vector3, Vector4};

macro_rules! mint_vector_conversion(
    ($($mint_vector_conversion_i: ident, $Vector: ident, $SZ: expr);* $(;)*) => {$(
        #[test]
        fn $mint_vector_conversion_i() {
            let v       = $Vector::from_fn(|i, _| i);
            let mv: mint::$Vector<usize> = v.into();
            let mv_ref: &mint::$Vector<usize> = v.as_ref();
            let v2      = $Vector::from(mv);
            let arr: [usize; $SZ] = mv.into();

            for i in 0 .. $SZ {
                assert_eq!(arr[i], i);
            }

            assert_eq!(&mv, mv_ref);
            assert_eq!(v, v2);
        }
    )*}
);

mint_vector_conversion!(
    mint_vector_conversion_2,  Vector2,  2;
    mint_vector_conversion_3,  Vector3,  3;
    mint_vector_conversion_4,  Vector4,  4;
);

#[test]
fn mint_quaternion_conversions() {
    let q = Quaternion::new(0.1f64, 0.2, 0.3, 0.4);
    let mq: mint::Quaternion<f64> = q.into();
    let q2 = Quaternion::from(mq);

    assert_eq!(mq.v.x, q[0]);
    assert_eq!(mq.v.y, q[1]);
    assert_eq!(mq.v.z, q[2]);
    assert_eq!(mq.s, q[3]);

    assert_eq!(q, q2);
}

macro_rules! mint_matrix_conversion(
    ($($mint_matrix_conversion_i_j: ident, $Matrix: ident, $Mint: ident, ($NRows: expr, $NCols: expr));* $(;)*) => {$(
        #[test]
        fn $mint_matrix_conversion_i_j() {
            let m       = $Matrix::from_fn(|i, j| i * 10 + j);
            let mm: mint::$Mint<usize> = m.into();
            let m2      = $Matrix::from(mm);
            let arr: [[usize; $NRows]; $NCols] = mm.into();

            for i in 0 .. $NRows {
                for j in 0 .. $NCols {
                    assert_eq!(arr[j][i], i * 10 + j);
                }
            }

            assert_eq!(m, m2);
        }
    )*}
);

mint_matrix_conversion!(
    mint_matrix_conversion_2_2, Matrix2,   ColumnMatrix2,   (2, 2);
    mint_matrix_conversion_2_3, Matrix2x3, ColumnMatrix2x3, (2, 3);
    mint_matrix_conversion_3_3, Matrix3,   ColumnMatrix3,   (3, 3);
    mint_matrix_conversion_3_4, Matrix3x4, ColumnMatrix3x4, (3, 4);
    mint_matrix_conversion_4_4, Matrix4,   ColumnMatrix4,   (4, 4);
);
