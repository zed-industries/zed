#[cfg(test)]
mod test {
    use crate::{
        Affine2, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4,
        I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, I8Vec2, I8Vec3, I8Vec4, IVec2, IVec3,
        IVec4, Mat2, Mat3, Mat3A, Mat4, Quat, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4,
        U8Vec2, U8Vec3, U8Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4,
    };
    use core::mem;

    macro_rules! test_into_bytes_t {
        ($name:ident, $t:ty) => {
            #[test]
            fn $name() {
                let t = <$t>::default();
                let b = zerocopy::IntoBytes::as_bytes(&t);
                // the below loop will fail in miri if we're doing something bad here.
                for bi in b {
                    assert_eq!(bi, bi);
                }
                // should be the same address
                assert_eq!(&t as *const $t as usize, b.as_ptr() as usize);
                // should be the same size
                assert_eq!(b.len(), mem::size_of_val(&t));
            }
        };
    }

    macro_rules! test_from_bytes_t {
        ($name:ident, $t:ident) => {
            #[test]
            fn $name() {
                let b = [0_u8; mem::size_of::<$t>()];
                let t = zerocopy::FromBytes::read_from_bytes(&b);
                // should be the same size
                let t: $t = t.unwrap();
                // should be zero
                assert_eq!(t, <$t as zerocopy::FromZeros>::new_zeroed());
            }
        };
    }

    test_from_bytes_t!(affine2, Affine2);
    test_from_bytes_t!(affine3a_from, Affine3A);
    #[cfg(all(
        any(
            target_arch = "aarch64",
            target_feature = "sse2",
            target_feature = "simd128"
        ),
        not(feature = "scalar-math")
    ))]
    test_into_bytes_t!(affine3a_into, Affine3A);
    test_from_bytes_t!(mat2_from, Mat2);
    test_into_bytes_t!(mat2_into, Mat2);
    test_from_bytes_t!(mat3_from, Mat3);
    test_into_bytes_t!(mat3_into, Mat3);
    test_from_bytes_t!(mat3a_from, Mat3A);
    #[cfg(all(
        any(
            feature = "core-simd",
            target_arch = "aarch64",
            target_feature = "sse2",
            target_feature = "simd128"
        ),
        not(feature = "scalar-math")
    ))]
    test_into_bytes_t!(mat3a_into, Mat3A);
    test_from_bytes_t!(mat4_from, Mat4);
    test_into_bytes_t!(mat4_into, Mat4);
    test_from_bytes_t!(quat_from, Quat);
    test_into_bytes_t!(quat_into, Quat);
    test_from_bytes_t!(vec2_from, Vec2);
    test_into_bytes_t!(vec2_into, Vec2);
    test_from_bytes_t!(vec3_from, Vec3);
    test_into_bytes_t!(vec3_into, Vec3);
    test_from_bytes_t!(vec3a_from, Vec3A);
    #[cfg(all(
        any(
            feature = "core-simd",
            target_arch = "aarch64",
            target_feature = "sse2",
            target_feature = "simd128"
        ),
        not(feature = "scalar-math")
    ))]
    test_into_bytes_t!(vec3a_into, Vec3A);
    test_from_bytes_t!(vec4_from, Vec4);
    test_into_bytes_t!(vec4_into, Vec4);

    test_into_bytes_t!(daffine2, DAffine2);
    test_from_bytes_t!(daffine3_from, DAffine3);
    test_into_bytes_t!(daffine3_into, DAffine3);
    test_from_bytes_t!(dmat2_from, DMat2);
    test_into_bytes_t!(dmat2_into, DMat2);
    test_from_bytes_t!(dmat3_from, DMat3);
    test_into_bytes_t!(dmat3_into, DMat3);
    test_from_bytes_t!(dmat4_from, DMat4);
    test_into_bytes_t!(dmat4_into, DMat4);
    test_from_bytes_t!(dquat_from, DQuat);
    test_into_bytes_t!(dquat_into, DQuat);
    test_from_bytes_t!(dvec2_from, DVec2);
    test_into_bytes_t!(dvec2_into, DVec2);
    test_from_bytes_t!(dvec3_from, DVec3);
    test_into_bytes_t!(dvec3_into, DVec3);
    test_from_bytes_t!(dvec4_from, DVec4);
    test_into_bytes_t!(dvec4_into, DVec4);

    test_from_bytes_t!(i8vec2_from, I8Vec2);
    test_into_bytes_t!(i8vec2_into, I8Vec2);
    test_from_bytes_t!(i8vec3_from, I8Vec3);
    test_into_bytes_t!(i8vec3_into, I8Vec3);
    test_from_bytes_t!(i8vec4_from, I8Vec4);
    test_into_bytes_t!(i8vec4_into, I8Vec4);

    test_from_bytes_t!(u8vec2_from, U8Vec2);
    test_into_bytes_t!(u8vec2_into, U8Vec2);
    test_from_bytes_t!(u8vec3_from, U8Vec3);
    test_into_bytes_t!(u8vec3_into, U8Vec3);
    test_from_bytes_t!(u8vec4_from, U8Vec4);
    test_into_bytes_t!(u8vec4_into, U8Vec4);

    test_from_bytes_t!(i16vec2_from, I16Vec2);
    test_into_bytes_t!(i16vec2_into, I16Vec2);
    test_from_bytes_t!(i16vec3_from, I16Vec3);
    test_into_bytes_t!(i16vec3_into, I16Vec3);
    test_from_bytes_t!(i16vec4_from, I16Vec4);
    test_into_bytes_t!(i16vec4_into, I16Vec4);

    test_from_bytes_t!(u16vec2_from, U16Vec2);
    test_into_bytes_t!(u16vec2_into, U16Vec2);
    test_from_bytes_t!(u16vec3_from, U16Vec3);
    test_into_bytes_t!(u16vec3_into, U16Vec3);
    test_from_bytes_t!(u16vec4_from, U16Vec4);
    test_into_bytes_t!(u16vec4_into, U16Vec4);

    test_from_bytes_t!(ivec2_from, IVec2);
    test_into_bytes_t!(ivec2_into, IVec2);
    test_from_bytes_t!(ivec3_from, IVec3);
    test_into_bytes_t!(ivec3_into, IVec3);
    test_from_bytes_t!(ivec4_from, IVec4);
    test_into_bytes_t!(ivec4_into, IVec4);

    test_from_bytes_t!(uvec2_from, UVec2);
    test_into_bytes_t!(uvec2_into, UVec2);
    test_from_bytes_t!(uvec3_from, UVec3);
    test_into_bytes_t!(uvec3_into, UVec3);
    test_from_bytes_t!(uvec4_from, UVec4);
    test_into_bytes_t!(uvec4_into, UVec4);

    test_from_bytes_t!(i64vec2_from, I64Vec2);
    test_into_bytes_t!(i64vec2_into, I64Vec2);
    test_from_bytes_t!(i64vec3_from, I64Vec3);
    test_into_bytes_t!(i64vec3_into, I64Vec3);
    test_from_bytes_t!(i64vec4_from, I64Vec4);
    test_into_bytes_t!(i64vec4_into, I64Vec4);

    test_from_bytes_t!(u64vec2_from, U64Vec2);
    test_into_bytes_t!(u64vec2_into, U64Vec2);
    test_from_bytes_t!(u64vec3_from, U64Vec3);
    test_into_bytes_t!(u64vec3_into, U64Vec3);
    test_from_bytes_t!(u64vec4_from, U64Vec4);
    test_into_bytes_t!(u64vec4_into, U64Vec4);
}
