use mint::IntoMint;

use crate::{
    DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3,
    I64Vec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat3A, Mat4, Quat, U16Vec2, U16Vec3, U16Vec4,
    U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4,
};

macro_rules! impl_vec_types {
    ($t:ty, $vec2:ty, $vec3:ty, $vec4:ty) => {
        impl From<mint::Point2<$t>> for $vec2 {
            fn from(v: mint::Point2<$t>) -> Self {
                Self::new(v.x, v.y)
            }
        }

        impl From<$vec2> for mint::Point2<$t> {
            fn from(v: $vec2) -> Self {
                Self { x: v.x, y: v.y }
            }
        }

        impl From<mint::Vector2<$t>> for $vec2 {
            fn from(v: mint::Vector2<$t>) -> Self {
                Self::new(v.x, v.y)
            }
        }

        impl From<$vec2> for mint::Vector2<$t> {
            fn from(v: $vec2) -> Self {
                Self { x: v.x, y: v.y }
            }
        }

        impl IntoMint for $vec2 {
            type MintType = mint::Vector2<$t>;
        }

        impl From<mint::Point3<$t>> for $vec3 {
            fn from(v: mint::Point3<$t>) -> Self {
                Self::new(v.x, v.y, v.z)
            }
        }

        impl From<$vec3> for mint::Point3<$t> {
            fn from(v: $vec3) -> Self {
                Self {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                }
            }
        }

        impl From<mint::Vector3<$t>> for $vec3 {
            fn from(v: mint::Vector3<$t>) -> Self {
                Self::new(v.x, v.y, v.z)
            }
        }

        impl From<$vec3> for mint::Vector3<$t> {
            fn from(v: $vec3) -> Self {
                Self {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                }
            }
        }

        impl IntoMint for $vec3 {
            type MintType = mint::Vector3<$t>;
        }

        impl From<mint::Vector4<$t>> for $vec4 {
            fn from(v: mint::Vector4<$t>) -> Self {
                Self::new(v.x, v.y, v.z, v.w)
            }
        }

        impl From<$vec4> for mint::Vector4<$t> {
            fn from(v: $vec4) -> Self {
                Self {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                    w: v.w,
                }
            }
        }

        impl IntoMint for $vec4 {
            type MintType = mint::Vector4<$t>;
        }
    };
}

macro_rules! impl_float_types {
    ($t:ty, $mat2:ty, $mat3:ty, $mat4:ty, $quat:ty, $vec2:ty, $vec3:ty, $vec4:ty) => {
        impl_vec_types!($t, $vec2, $vec3, $vec4);

        impl From<mint::Quaternion<$t>> for $quat {
            fn from(q: mint::Quaternion<$t>) -> Self {
                Self::from_xyzw(q.v.x, q.v.y, q.v.z, q.s)
            }
        }

        impl From<$quat> for mint::Quaternion<$t> {
            fn from(q: $quat) -> Self {
                Self {
                    s: q.w,
                    v: mint::Vector3 {
                        x: q.x,
                        y: q.y,
                        z: q.z,
                    },
                }
            }
        }

        impl IntoMint for $quat {
            type MintType = mint::Quaternion<$t>;
        }

        impl From<mint::RowMatrix2<$t>> for $mat2 {
            fn from(m: mint::RowMatrix2<$t>) -> Self {
                Self::from_cols(m.x.into(), m.y.into()).transpose()
            }
        }

        impl From<$mat2> for mint::RowMatrix2<$t> {
            fn from(m: $mat2) -> Self {
                let mt = m.transpose();
                Self {
                    x: mt.x_axis.into(),
                    y: mt.y_axis.into(),
                }
            }
        }

        impl From<mint::ColumnMatrix2<$t>> for $mat2 {
            fn from(m: mint::ColumnMatrix2<$t>) -> Self {
                Self::from_cols(m.x.into(), m.y.into())
            }
        }

        impl From<$mat2> for mint::ColumnMatrix2<$t> {
            fn from(m: $mat2) -> Self {
                Self {
                    x: m.x_axis.into(),
                    y: m.y_axis.into(),
                }
            }
        }

        impl IntoMint for $mat2 {
            type MintType = mint::ColumnMatrix2<$t>;
        }

        impl From<mint::RowMatrix3<$t>> for $mat3 {
            fn from(m: mint::RowMatrix3<$t>) -> Self {
                Self::from_cols(m.x.into(), m.y.into(), m.z.into()).transpose()
            }
        }

        impl From<$mat3> for mint::RowMatrix3<$t> {
            fn from(m: $mat3) -> Self {
                let mt = m.transpose();
                Self {
                    x: mt.x_axis.into(),
                    y: mt.y_axis.into(),
                    z: mt.z_axis.into(),
                }
            }
        }

        impl From<mint::ColumnMatrix3<$t>> for $mat3 {
            fn from(m: mint::ColumnMatrix3<$t>) -> Self {
                Self::from_cols(m.x.into(), m.y.into(), m.z.into())
            }
        }

        impl From<$mat3> for mint::ColumnMatrix3<$t> {
            fn from(m: $mat3) -> Self {
                Self {
                    x: m.x_axis.into(),
                    y: m.y_axis.into(),
                    z: m.z_axis.into(),
                }
            }
        }

        impl IntoMint for $mat3 {
            type MintType = mint::ColumnMatrix3<$t>;
        }

        impl From<mint::RowMatrix4<$t>> for $mat4 {
            fn from(m: mint::RowMatrix4<$t>) -> Self {
                Self::from_cols(m.x.into(), m.y.into(), m.z.into(), m.w.into()).transpose()
            }
        }

        impl From<$mat4> for mint::RowMatrix4<$t> {
            fn from(m: $mat4) -> Self {
                let mt = m.transpose();
                Self {
                    x: mt.x_axis.into(),
                    y: mt.y_axis.into(),
                    z: mt.z_axis.into(),
                    w: mt.w_axis.into(),
                }
            }
        }

        impl From<mint::ColumnMatrix4<$t>> for $mat4 {
            fn from(m: mint::ColumnMatrix4<$t>) -> Self {
                Self::from_cols(m.x.into(), m.y.into(), m.z.into(), m.w.into())
            }
        }

        impl From<$mat4> for mint::ColumnMatrix4<$t> {
            fn from(m: $mat4) -> Self {
                Self {
                    x: m.x_axis.into(),
                    y: m.y_axis.into(),
                    z: m.z_axis.into(),
                    w: m.w_axis.into(),
                }
            }
        }

        impl IntoMint for $mat4 {
            type MintType = mint::ColumnMatrix4<$t>;
        }
    };
}

impl From<mint::Point3<f32>> for Vec3A {
    fn from(v: mint::Point3<f32>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vec3A> for mint::Point3<f32> {
    fn from(v: Vec3A) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<mint::Vector3<f32>> for Vec3A {
    fn from(v: mint::Vector3<f32>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Vec3A> for mint::Vector3<f32> {
    fn from(v: Vec3A) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl IntoMint for Vec3A {
    type MintType = mint::Vector3<f32>;
}

impl From<mint::RowMatrix3<f32>> for Mat3A {
    fn from(m: mint::RowMatrix3<f32>) -> Self {
        Self::from_cols(m.x.into(), m.y.into(), m.z.into()).transpose()
    }
}

impl From<Mat3A> for mint::RowMatrix3<f32> {
    fn from(m: Mat3A) -> Self {
        let mt = m.transpose();
        Self {
            x: mt.x_axis.into(),
            y: mt.y_axis.into(),
            z: mt.z_axis.into(),
        }
    }
}

impl From<mint::ColumnMatrix3<f32>> for Mat3A {
    fn from(m: mint::ColumnMatrix3<f32>) -> Self {
        Self::from_cols(m.x.into(), m.y.into(), m.z.into())
    }
}

impl From<Mat3A> for mint::ColumnMatrix3<f32> {
    fn from(m: Mat3A) -> Self {
        Self {
            x: m.x_axis.into(),
            y: m.y_axis.into(),
            z: m.z_axis.into(),
        }
    }
}

impl IntoMint for Mat3A {
    type MintType = mint::ColumnMatrix3<f32>;
}

impl_float_types!(f32, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);
impl_float_types!(f64, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
impl_vec_types!(i16, I16Vec2, I16Vec3, I16Vec4);
impl_vec_types!(u16, U16Vec2, U16Vec3, U16Vec4);
impl_vec_types!(i32, IVec2, IVec3, IVec4);
impl_vec_types!(u32, UVec2, UVec3, UVec4);
impl_vec_types!(i64, I64Vec2, I64Vec3, I64Vec4);
impl_vec_types!(u64, U64Vec2, U64Vec3, U64Vec4);

#[cfg(test)]
mod test {
    macro_rules! impl_vec_tests {
        ($t:ty, $vec2:ident, $vec3:ident, $vec4:ident) => {
            use crate::{$vec2, $vec3, $vec4};
            use mint;

            #[test]
            fn test_point2() {
                let m = mint::Point2 {
                    x: 1 as $t,
                    y: 2 as $t,
                };
                let g = $vec2::from(m);
                assert_eq!(g, $vec2::new(1 as $t, 2 as $t));
                assert_eq!(m, g.into());
            }

            #[test]
            fn test_point3() {
                let m = mint::Point3 {
                    x: 1 as $t,
                    y: 2 as $t,
                    z: 3 as $t,
                };
                let g = $vec3::from(m);
                assert_eq!(g, $vec3::new(1 as $t, 2 as $t, 3 as $t));
                assert_eq!(m, g.into());
            }

            #[test]
            fn test_vector2() {
                let m = mint::Vector2 {
                    x: 1 as $t,
                    y: 2 as $t,
                };
                let g = $vec2::from(m);
                assert_eq!(g, $vec2::new(1 as $t, 2 as $t));
                assert_eq!(m, g.into());
            }

            #[test]
            fn test_vector3() {
                let m = mint::Vector3 {
                    x: 1 as $t,
                    y: 2 as $t,
                    z: 3 as $t,
                };
                let g = $vec3::from(m);
                assert_eq!(g, $vec3::new(1 as $t, 2 as $t, 3 as $t));
                assert_eq!(m, g.into());
            }

            #[test]
            fn test_vector4() {
                let m = mint::Vector4 {
                    x: 1 as $t,
                    y: 2 as $t,
                    z: 3 as $t,
                    w: 4 as $t,
                };
                let g = $vec4::from(m);
                assert_eq!(g, $vec4::new(1 as $t, 2 as $t, 3 as $t, 4 as $t));
                assert_eq!(m, g.into());
            }
        };
    }

    macro_rules! impl_float_tests {
        ($t:ty, $mat2:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
            impl_vec_tests!($t, $vec2, $vec3, $vec4);

            use crate::{$mat2, $mat3, $mat4, $quat};

            #[test]
            fn test_quaternion() {
                let m = mint::Quaternion {
                    v: mint::Vector3 {
                        x: 1.0,
                        y: 2.0,
                        z: 3.0,
                    },
                    s: 4.0,
                };
                let g = $quat::from(m);
                assert_eq!(g, $quat::from_xyzw(1.0, 2.0, 3.0, 4.0));
                assert_eq!(m, g.into());
            }

            #[test]
            fn test_matrix2() {
                let g = $mat2::from_cols_array_2d(&[[1.0, 2.0], [3.0, 4.0]]);
                let m = mint::ColumnMatrix2::from(g);
                assert_eq!(g, $mat2::from(m));
                let mt = mint::RowMatrix2::from(g);
                assert_eq!(mt, mint::RowMatrix2::from([[1.0, 3.0], [2.0, 4.0]]));
                assert_eq!(g, $mat2::from(mt));
            }

            #[test]
            fn test_matrix3() {
                let g =
                    $mat3::from_cols_array_2d(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
                let m = mint::ColumnMatrix3::from(g);
                assert_eq!(g, $mat3::from(m));
                let mt = mint::RowMatrix3::from(g);
                assert_eq!(
                    mt,
                    mint::RowMatrix3::from([[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0]])
                );
                assert_eq!(g, $mat3::from(mt));
            }

            #[test]
            fn test_matrix4() {
                let g = $mat4::from_cols_array_2d(&[
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]);
                let m = mint::ColumnMatrix4::from(g);
                assert_eq!(g, $mat4::from(m));
                let mt = mint::RowMatrix4::from(g);
                assert_eq!(
                    mt,
                    mint::RowMatrix4::from([
                        [1.0, 5.0, 9.0, 13.0],
                        [2.0, 6.0, 10.0, 14.0],
                        [3.0, 7.0, 11.0, 15.0],
                        [4.0, 8.0, 12.0, 16.0]
                    ])
                );
                assert_eq!(g, $mat4::from(mt));
            }
        };
    }

    mod f32 {
        impl_float_tests!(f32, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);

        #[test]
        fn test_point3a() {
            use crate::Vec3A;
            let m = mint::Point3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let g = Vec3A::from(m);
            assert_eq!(g, Vec3A::new(1.0, 2.0, 3.0));
            assert_eq!(m, g.into());
        }

        #[test]
        fn test_vector3a() {
            use crate::Vec3A;
            let m = mint::Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            let g = Vec3A::from(m);
            assert_eq!(g, Vec3A::new(1.0, 2.0, 3.0));
            assert_eq!(m, g.into());
        }

        #[test]
        fn test_mat3a_col_major() {
            use crate::Mat3A;
            let m = mint::ColumnMatrix3 {
                x: [0.0, 1.0, 2.0].into(),
                y: [3.0, 4.0, 5.0].into(),
                z: [6.0, 7.0, 8.0].into(),
            };
            let expected = Mat3A::from_cols(
                [0.0, 1.0, 2.0].into(),
                [3.0, 4.0, 5.0].into(),
                [6.0, 7.0, 8.0].into(),
            );
            assert_eq!(expected, m.into());
            assert_eq!(m, expected.into());
        }

        #[test]
        fn test_mat3a_row_major() {
            use crate::Mat3A;
            let m = mint::RowMatrix3 {
                x: [0.0, 1.0, 2.0].into(),
                y: [3.0, 4.0, 5.0].into(),
                z: [6.0, 7.0, 8.0].into(),
            };
            let expected = Mat3A::from_cols(
                [0.0, 3.0, 6.0].into(),
                [1.0, 4.0, 7.0].into(),
                [2.0, 5.0, 8.0].into(),
            );
            assert_eq!(expected, m.into());
            assert_eq!(m, expected.into());
        }
    }

    mod f64 {
        impl_float_tests!(f64, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
    }

    mod i32 {
        impl_vec_tests!(i32, IVec2, IVec3, IVec4);
    }

    mod u32 {
        impl_vec_tests!(u32, UVec2, UVec3, UVec4);
    }
}
