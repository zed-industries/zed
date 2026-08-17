#![cfg(all(feature = "proptest-support"))]
use na::{
    self, Affine3, Isometry3, Matrix2, Matrix2x3, Matrix2x4, Matrix2x5, Matrix2x6, Matrix3,
    Matrix3x2, Matrix3x4, Matrix3x5, Matrix3x6, Matrix4, Matrix4x2, Matrix4x3, Matrix4x5,
    Matrix4x6, Matrix5, Matrix5x2, Matrix5x3, Matrix5x4, Matrix5x6, Matrix6, Matrix6x2, Matrix6x3,
    Matrix6x4, Matrix6x5, Projective3, Rotation3, RowVector1, RowVector2, RowVector3, RowVector4,
    RowVector5, RowVector6, Similarity3, Transform3, UnitQuaternion, Vector1, Vector2, Vector3,
    Vector4, Vector5, Vector6,
};
use na::{DMatrix, DMatrixView, DMatrixViewMut, MatrixView, MatrixViewMut};
use na::{U1, U3, U4};

use crate::proptest::*;
use proptest::{prop_assert, prop_assert_eq, proptest};

proptest! {
    #[test]
    fn translation_conversion(t in translation3(), p in point3()) {
        let iso: Isometry3<f64>   = na::convert(t);
        let sim: Similarity3<f64> = na::convert(t);
        let aff: Affine3<f64>     = na::convert(t);
        let prj: Projective3<f64> = na::convert(t);
        let tr:  Transform3<f64>  = na::convert(t);

        prop_assert_eq!(t, na::try_convert(iso).unwrap());
        prop_assert_eq!(t, na::try_convert(sim).unwrap());
        prop_assert_eq!(t, na::try_convert(aff).unwrap());
        prop_assert_eq!(t, na::try_convert(prj).unwrap());
        prop_assert_eq!(t, na::try_convert(tr).unwrap() );

        prop_assert_eq!(t * p, iso * p);
        prop_assert_eq!(t * p, sim * p);
        prop_assert_eq!(t * p, aff * p);
        prop_assert_eq!(t * p, prj * p);
        prop_assert_eq!(t * p, tr  * p);
    }

    #[test]
    fn rotation_conversion(r in rotation3(), v in vector3(), p in point3()) {
        let uq:  UnitQuaternion<f64> = na::convert(r);
        let iso: Isometry3<f64>      = na::convert(r);
        let sim: Similarity3<f64>    = na::convert(r);
        let aff: Affine3<f64>        = na::convert(r);
        let prj: Projective3<f64>    = na::convert(r);
        let tr:  Transform3<f64>     = na::convert(r);

        prop_assert!(relative_eq!(r, na::try_convert(uq).unwrap(),  epsilon = 1.0e-7));
        prop_assert!(relative_eq!(r, na::try_convert(iso).unwrap(), epsilon = 1.0e-7));
        prop_assert!(relative_eq!(r, na::try_convert(sim).unwrap(), epsilon = 1.0e-7));
        prop_assert_eq!(r, na::try_convert(aff).unwrap());
        prop_assert_eq!(r, na::try_convert(prj).unwrap());
        prop_assert_eq!(r, na::try_convert(tr).unwrap() );

        // NOTE: we need relative_eq because Isometry and Similarity use quaternions.
        prop_assert!(relative_eq!(r * v, uq  * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(r * v, iso * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(r * v, sim * v, epsilon = 1.0e-7));
        prop_assert_eq!(r * v, aff * v);
        prop_assert_eq!(r * v, prj * v);
        prop_assert_eq!(r * v, tr  * v);

        prop_assert!(relative_eq!(r * p, uq  * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(r * p, iso * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(r * p, sim * p, epsilon = 1.0e-7));
        prop_assert_eq!(r * p, aff * p);
        prop_assert_eq!(r * p, prj * p);
        prop_assert_eq!(r * p, tr  * p);
    }

    #[test]
    fn unit_quaternion_conversion(uq in unit_quaternion(), v in vector3(), p in point3()) {
        let rot: Rotation3<f64>   = na::convert(uq);
        let iso: Isometry3<f64>   = na::convert(uq);
        let sim: Similarity3<f64> = na::convert(uq);
        let aff: Affine3<f64>     = na::convert(uq);
        let prj: Projective3<f64> = na::convert(uq);
        let tr:  Transform3<f64>  = na::convert(uq);

        prop_assert_eq!(uq, na::try_convert(iso).unwrap());
        prop_assert_eq!(uq, na::try_convert(sim).unwrap());
        prop_assert!(relative_eq!(uq, na::try_convert(rot).unwrap(), epsilon = 1.0e-7));
        prop_assert!(relative_eq!(uq, na::try_convert(aff).unwrap(), epsilon = 1.0e-7));
        prop_assert!(relative_eq!(uq, na::try_convert(prj).unwrap(), epsilon = 1.0e-7));
        prop_assert!(relative_eq!(uq, na::try_convert(tr).unwrap(), epsilon = 1.0e-7) );

        // NOTE: iso and sim use unit quaternions for the rotation so conversions to them are exact.
        prop_assert!(relative_eq!(uq * v, rot * v, epsilon = 1.0e-7));
        prop_assert_eq!(uq * v, iso * v);
        prop_assert_eq!(uq * v, sim * v);
        prop_assert!(relative_eq!(uq * v, aff * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(uq * v, prj * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(uq * v, tr  * v, epsilon = 1.0e-7));

        prop_assert!(relative_eq!(uq * p, rot * p, epsilon = 1.0e-7));
        prop_assert_eq!(uq * p, iso * p);
        prop_assert_eq!(uq * p, sim * p);
        prop_assert!(relative_eq!(uq * p, aff * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(uq * p, prj * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(uq * p, tr  * p, epsilon = 1.0e-7));
    }

    #[test]
    fn isometry_conversion(iso in isometry3(), v in vector3(), p in point3()) {
        let sim: Similarity3<f64> = na::convert(iso);
        let aff: Affine3<f64>     = na::convert(iso);
        let prj: Projective3<f64> = na::convert(iso);
        let tr:  Transform3<f64>  = na::convert(iso);


        prop_assert_eq!(iso, na::try_convert(sim).unwrap());
        prop_assert!(relative_eq!(iso, na::try_convert(aff).unwrap(), epsilon = 1.0e-7));
        prop_assert!(relative_eq!(iso, na::try_convert(prj).unwrap(), epsilon = 1.0e-7));
        prop_assert!(relative_eq!(iso, na::try_convert(tr).unwrap(), epsilon = 1.0e-7) );

        prop_assert_eq!(iso * v, sim * v);
        prop_assert!(relative_eq!(iso * v, aff * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(iso * v, prj * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(iso * v, tr  * v, epsilon = 1.0e-7));

        prop_assert_eq!(iso * p, sim * p);
        prop_assert!(relative_eq!(iso * p, aff * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(iso * p, prj * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(iso * p, tr  * p, epsilon = 1.0e-7));
    }

    #[test]
    fn similarity_conversion(sim in similarity3(), v in vector3(), p in point3()) {
        let aff: Affine3<f64>     = na::convert(sim);
        let prj: Projective3<f64> = na::convert(sim);
        let tr:  Transform3<f64>  = na::convert(sim);

        prop_assert!(relative_eq!(sim, na::try_convert(aff).unwrap(), epsilon = 1.0e-7));
        prop_assert!(relative_eq!(sim, na::try_convert(prj).unwrap(), epsilon = 1.0e-7));
        prop_assert!(relative_eq!(sim, na::try_convert(tr).unwrap(),  epsilon = 1.0e-7));

        prop_assert!(relative_eq!(sim * v, aff * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(sim * v, prj * v, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(sim * v, tr  * v, epsilon = 1.0e-7));

        prop_assert!(relative_eq!(sim * p, aff * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(sim * p, prj * p, epsilon = 1.0e-7));
        prop_assert!(relative_eq!(sim * p, tr  * p, epsilon = 1.0e-7));
    }

    // XXX test Transform
}

macro_rules! array_vector_conversion(
    ($($array_vector_conversion_i: ident, $Vector: ident, $SZ: expr);* $(;)*) => {$(
        #[test]
        fn $array_vector_conversion_i() {
            let v       = $Vector::from_fn(|i, _| i);
            let arr: [usize; $SZ] = v.into();
            let arr_ref: &[usize; $SZ] = v.as_ref();
            let v2      = $Vector::from(arr);

            for i in 0 .. $SZ {
                assert_eq!(arr[i], i);
                assert_eq!(arr_ref[i], i);
            }

            assert_eq!(v, v2);
        }
    )*}
);

array_vector_conversion!(
    array_vector_conversion_1,  Vector1,  1;
    array_vector_conversion_2,  Vector2,  2;
    array_vector_conversion_3,  Vector3,  3;
    array_vector_conversion_4,  Vector4,  4;
    array_vector_conversion_5,  Vector5,  5;
    array_vector_conversion_6,  Vector6,  6;
);

macro_rules! array_row_vector_conversion(
    ($($array_vector_conversion_i: ident, $Vector: ident, $SZ: expr);* $(;)*) => {$(
        #[test]
        fn $array_vector_conversion_i() {
            let v: $Vector<_>       = $Vector::from_fn(|_, i| i);
            let arr: [usize; $SZ] = v.into();
            let arr_ref: &[_] = v.as_ref();
            let v2      = $Vector::from(arr);

            for i in 0 .. $SZ {
                assert_eq!(arr[i], i);
                assert_eq!(arr_ref[i], i);
            }

            assert_eq!(v, v2);
        }
    )*}
);

array_row_vector_conversion!(
    array_row_vector_conversion_1,  RowVector1,  1;
    array_row_vector_conversion_2,  RowVector2,  2;
    array_row_vector_conversion_3,  RowVector3,  3;
    array_row_vector_conversion_4,  RowVector4,  4;
    array_row_vector_conversion_5,  RowVector5,  5;
    array_row_vector_conversion_6,  RowVector6,  6;
);

macro_rules! array_matrix_conversion(
    ($($array_matrix_conversion_i_j: ident, $Matrix: ident, ($NRows: expr, $NCols: expr));* $(;)*) => {$(
        #[test]
        fn $array_matrix_conversion_i_j() {
            let m       = $Matrix::from_fn(|i, j| i * 10 + j);
            let arr: [[usize; $NRows]; $NCols] = m.into();
            let arr_ref: &[_] = m.as_ref();
            let m2      = $Matrix::from(arr);

            for i in 0 .. $NRows {
                for j in 0 .. $NCols {
                    assert_eq!(arr[j][i], i * 10 + j);
                    assert_eq!(arr_ref[j * $NRows + i], i * 10 + j);
                }
            }

            assert_eq!(m, m2);
        }
    )*}
);

array_matrix_conversion!(
    array_matrix_conversion_2_2, Matrix2,   (2, 2);
    array_matrix_conversion_2_3, Matrix2x3, (2, 3);
    array_matrix_conversion_2_4, Matrix2x4, (2, 4);
    array_matrix_conversion_2_5, Matrix2x5, (2, 5);
    array_matrix_conversion_2_6, Matrix2x6, (2, 6);

    array_matrix_conversion_3_2, Matrix3x2, (3, 2);
    array_matrix_conversion_3_3, Matrix3,   (3, 3);
    array_matrix_conversion_3_4, Matrix3x4, (3, 4);
    array_matrix_conversion_3_5, Matrix3x5, (3, 5);
    array_matrix_conversion_3_6, Matrix3x6, (3, 6);

    array_matrix_conversion_4_2, Matrix4x2, (4, 2);
    array_matrix_conversion_4_3, Matrix4x3, (4, 3);
    array_matrix_conversion_4_4, Matrix4,   (4, 4);
    array_matrix_conversion_4_5, Matrix4x5, (4, 5);
    array_matrix_conversion_4_6, Matrix4x6, (4, 6);

    array_matrix_conversion_5_2, Matrix5x2, (5, 2);
    array_matrix_conversion_5_3, Matrix5x3, (5, 3);
    array_matrix_conversion_5_4, Matrix5x4, (5, 4);
    array_matrix_conversion_5_5, Matrix5,   (5, 5);
    array_matrix_conversion_5_6, Matrix5x6, (5, 6);

    array_matrix_conversion_6_2, Matrix6x2, (6, 2);
    array_matrix_conversion_6_3, Matrix6x3, (6, 3);
    array_matrix_conversion_6_4, Matrix6x4, (6, 4);
    array_matrix_conversion_6_5, Matrix6x5, (6, 5);
    array_matrix_conversion_6_6, Matrix6,   (6, 6);
);

#[test]
fn matrix_slice_from_matrix_ref() {
    let a = Matrix3x4::new(
        11.0, 12.0, 13.0, 14.0, 21.0, 22.0, 23.0, 24.0, 31.0, 32.0, 33.0, 34.0,
    );

    // TODO: What's a more idiomatic/better way to convert a static matrix to a dynamic one?
    let d = DMatrix::from(a.get((0..a.nrows(), 0..a.ncols())).unwrap());

    // Note: these have to be macros, and not functions, because the input type is different
    // across the different tests. Moreover, the output type depends on the stride of the input,
    // which is different for static and dynamic matrices.
    macro_rules! dynamic_view {
        ($mref:expr) => {
            DMatrixView::<_>::from($mref)
        };
    }
    macro_rules! dynamic_view_mut {
        ($mref:expr) => {
            DMatrixViewMut::<_>::from($mref)
        };
    }
    macro_rules! fixed_view {
        ($mref:expr) => {
            MatrixView::<_, U3, U4, U1, U3>::from($mref)
        };
    }
    macro_rules! fixed_view_mut {
        ($mref:expr) => {
            MatrixViewMut::<_, U3, U4, U1, U3>::from($mref)
        };
    }

    // TODO: The `into_owned()` is a result of `PartialEq` not being implemented for different
    // Self and RHS. See issue #674. Once this is implemented, we can remove `into_owned`
    // from the below tests.

    // Construct views from reference to a
    {
        assert_eq!(a, fixed_view!(&a).into_owned());
        assert_eq!(d, dynamic_view!(&a).into_owned());
    }

    // Construct views from mutable reference to a
    {
        let mut a_clone = a.clone();
        assert_eq!(a, fixed_view!(&mut a_clone).into_owned());
        assert_eq!(d, dynamic_view!(&mut a_clone).into_owned());
    }

    // Construct mutable slices from mutable reference to a
    {
        let mut a_clone = a.clone();
        assert_eq!(a, fixed_view_mut!(&mut a_clone).into_owned());
        assert_eq!(d, dynamic_view_mut!(&mut a_clone).into_owned());
    }

    // Construct slices from reference to d
    {
        assert_eq!(a, fixed_view!(&d).into_owned());
        assert_eq!(d, dynamic_view!(&d).into_owned());
    }

    // Construct slices from mutable reference to d
    {
        let mut d_clone = a.clone();
        assert_eq!(a, fixed_view!(&mut d_clone).into_owned());
        assert_eq!(d, dynamic_view!(&mut d_clone).into_owned());
    }

    // Construct mutable slices from mutable reference to d
    {
        let mut d_clone = d.clone();
        assert_eq!(a, fixed_view_mut!(&mut d_clone).into_owned());
        assert_eq!(d, dynamic_view_mut!(&mut d_clone).into_owned());
    }

    // Construct slices from a slice of a
    {
        let mut a_slice = fixed_view!(&a);
        assert_eq!(a, fixed_view!(&a_slice).into_owned());
        assert_eq!(a, fixed_view!(&mut a_slice).into_owned());
        assert_eq!(d, dynamic_view!(&a_slice).into_owned());
        assert_eq!(d, dynamic_view!(&mut a_slice).into_owned());
    }

    // Construct slices from a slice mut of a
    {
        // Need a clone of a here, so that we can both have a mutable borrow and compare equality
        let mut a_clone = a.clone();
        let mut a_slice = fixed_view_mut!(&mut a_clone);

        assert_eq!(a, fixed_view!(&a_slice).into_owned());
        assert_eq!(a, fixed_view!(&mut a_slice).into_owned());
        assert_eq!(d, dynamic_view!(&a_slice).into_owned());
        assert_eq!(d, dynamic_view!(&mut a_slice).into_owned());
        assert_eq!(a, fixed_view_mut!(&mut a_slice).into_owned());
        assert_eq!(d, dynamic_view_mut!(&mut a_slice).into_owned());
    }
}
