use crate::{IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
use encase::{
    matrix::{impl_matrix, AsMutMatrixParts, AsRefMatrixParts, FromMatrixParts, MatrixScalar},
    vector::impl_vector,
};

impl_vector!(2, Vec2, f32; using AsRef AsMut From);
impl_vector!(2, UVec2, u32; using AsRef AsMut From);
impl_vector!(2, IVec2, i32; using AsRef AsMut From);

impl_vector!(3, Vec3, f32; using AsRef AsMut From);
impl_vector!(3, UVec3, u32; using AsRef AsMut From);
impl_vector!(3, IVec3, i32; using AsRef AsMut From);

impl_vector!(4, Vec4, f32; using AsRef AsMut From);
impl_vector!(4, UVec4, u32; using AsRef AsMut From);
impl_vector!(4, IVec4, i32; using AsRef AsMut From);

macro_rules! impl_matrix_traits {
    ($c:literal, $r:literal, $type:ty, $el_ty:ty) => {
        impl AsRefMatrixParts<$el_ty, $c, $r> for $type
        where
            Self: AsRef<[$el_ty; $r * $c]>,
            $el_ty: MatrixScalar,
        {
            fn as_ref_parts(&self) -> &[[$el_ty; $r]; $c] {
                unsafe {
                    ::core::mem::transmute::<&[$el_ty; $r * $c], &[[$el_ty; $r]; $c]>(self.as_ref())
                }
            }
        }

        impl AsMutMatrixParts<$el_ty, $c, $r> for $type
        where
            Self: AsMut<[$el_ty; $r * $c]>,
            $el_ty: MatrixScalar,
        {
            fn as_mut_parts(&mut self) -> &mut [[$el_ty; $r]; $c] {
                unsafe {
                    ::core::mem::transmute::<&mut [$el_ty; $r * $c], &mut [[$el_ty; $r]; $c]>(
                        self.as_mut(),
                    )
                }
            }
        }

        impl FromMatrixParts<$el_ty, $c, $r> for $type {
            fn from_parts(parts: [[$el_ty; $r]; $c]) -> Self {
                Self::from_cols_array_2d(&parts)
            }
        }
    };
}

impl_matrix_traits!(2, 2, Mat2, f32);
impl_matrix_traits!(3, 3, Mat3, f32);
impl_matrix_traits!(4, 4, Mat4, f32);

impl_matrix!(2, 2, Mat2, f32);
impl_matrix!(3, 3, Mat3, f32);
impl_matrix!(4, 4, Mat4, f32);

#[cfg(test)]
macro_rules! impl_vec_test {
    ($dim:literal, $test:ident, $ty:ty, $value:expr) => {
        #[test]
        fn $test() {
            let v = <$ty>::from_array($value);

            let mut buf = [255u8; $dim * 4];
            let mut s_buf = StorageBuffer::new(&mut buf);

            s_buf.write(&v).unwrap();
            for i in 0..$dim {
                assert_eq!(&s_buf.as_ref()[(i * 4)..][..4], &v[i].to_le_bytes());
            }

            let mut v_out = <$ty>::ZERO;
            s_buf.read(&mut v_out).unwrap();
            assert_eq!(v, v_out);

            assert_eq!(v, s_buf.create().unwrap());
        }
    };
}

#[cfg(test)]
macro_rules! impl_mat_test {
    ($dim:literal, $stride:literal, $test:ident, $ty:ty, $value:expr) => {
        #[test]
        fn $test() {
            let m = <$ty>::from_cols_array_2d(&$value);

            let mut buf = [255u8; $dim * $stride * 4];
            let mut s_buf = StorageBuffer::new(&mut buf);

            s_buf.write(&m).unwrap();
            for i in 0..$dim {
                for j in 0..$dim {
                    let stride = j * $stride * 4;
                    let offset = i * 4;
                    assert_eq!(
                        &s_buf.as_ref()[(stride + offset)..][..4],
                        &m.col(j)[i].to_le_bytes()
                    );
                }
            }

            let mut m_out = <$ty>::ZERO;
            s_buf.read(&mut m_out).unwrap();
            assert_eq!(m, m_out);

            assert_eq!(m, s_buf.create().unwrap());
        }
    };
}

#[cfg(test)]
mod test {
    use crate::{IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
    use encase::StorageBuffer;

    impl_vec_test!(2, vec2, Vec2, [1.12, 3.04]);
    impl_vec_test!(2, ivec2, IVec2, [1, 3]);
    impl_vec_test!(2, uvec2, UVec2, [1, 3]);

    impl_vec_test!(3, vec3, Vec3, [1.12, 3.04, 6.75]);
    impl_vec_test!(3, ivec3, IVec3, [1, 3, 6]);
    impl_vec_test!(3, uvec3, UVec3, [1, 3, 6]);

    impl_vec_test!(4, vec4, Vec4, [1.12, 3.04, 6.75, 9.99]);
    impl_vec_test!(4, ivec4, IVec4, [1, 3, 6, 9]);
    impl_vec_test!(4, uvec4, UVec4, [1, 3, 6, 9]);

    impl_mat_test!(2, 2, mat2, Mat2, [[1.12, 3.04], [8.65, 2.34]]);

    impl_mat_test!(
        3,
        4,
        mat3,
        Mat3,
        [[1.12, 3.04, 6.75], [8.65, 2.34, 94.6], [5.45, 6.34, 89.41]]
    );

    impl_mat_test!(
        4,
        4,
        mat4,
        Mat4,
        [
            [1.12, 3.04, 6.75, 9.99],
            [8.65, 2.34, 94.6, 113.3],
            [5.45, 6.34, 89.41, 185.1],
            [3.56, 5.43, 77.90, 140.67]
        ]
    );
}
