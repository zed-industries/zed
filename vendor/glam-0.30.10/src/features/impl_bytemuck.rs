#[cfg(test)]
mod test {
    use crate::{
        Affine2, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4,
        I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, I8Vec2, I8Vec3, I8Vec4, IVec2, IVec3,
        IVec4, Mat2, Mat3, Mat3A, Mat4, Quat, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4,
        U8Vec2, U8Vec3, U8Vec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4,
    };
    use core::mem;

    macro_rules! test_pod_t {
        ($name:ident, $t:ty) => {
            #[test]
            fn $name() {
                let t = <$t>::default();
                let b = bytemuck::bytes_of(&t);
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

    macro_rules! test_any_bit_pattern_t {
        ($name:ident, $t:ident) => {
            #[test]
            fn $name() {
                let b = [0_u8; mem::size_of::<$t>()];
                let t: $t = bytemuck::cast(b);
                // should be the same size
                assert_eq!(b.len(), mem::size_of_val(&t));
                // should be zero
                assert_eq!(t, $t::ZERO);
            }
        };
    }

    test_any_bit_pattern_t!(affine2, Affine2);
    test_any_bit_pattern_t!(affine3a, Affine3A);
    test_pod_t!(mat2, Mat2);
    test_pod_t!(mat3, Mat3);
    test_any_bit_pattern_t!(mat3a, Mat3A);
    test_pod_t!(mat4, Mat4);
    test_pod_t!(quat, Quat);
    test_pod_t!(vec2, Vec2);
    test_pod_t!(vec3, Vec3);
    test_any_bit_pattern_t!(vec3a, Vec3A);
    test_pod_t!(vec4, Vec4);

    test_pod_t!(daffine2, DAffine2);
    test_pod_t!(daffine3, DAffine3);
    test_pod_t!(dmat2, DMat2);
    test_pod_t!(dmat3, DMat3);
    test_pod_t!(dmat4, DMat4);
    test_pod_t!(dquat, DQuat);
    test_pod_t!(dvec2, DVec2);
    test_pod_t!(dvec3, DVec3);
    test_pod_t!(dvec4, DVec4);

    test_pod_t!(i8vec2, I8Vec2);
    test_pod_t!(i8vec3, I8Vec3);
    test_pod_t!(i8vec4, I8Vec4);

    test_pod_t!(u8vec2, U8Vec2);
    test_pod_t!(u8vec3, U8Vec3);
    test_pod_t!(u8vec4, U8Vec4);

    test_pod_t!(i16vec2, I16Vec2);
    test_pod_t!(i16vec3, I16Vec3);
    test_pod_t!(i16vec4, I16Vec4);

    test_pod_t!(u16vec2, U16Vec2);
    test_pod_t!(u16vec3, U16Vec3);
    test_pod_t!(u16vec4, U16Vec4);

    test_pod_t!(ivec2, IVec2);
    test_pod_t!(ivec3, IVec3);
    test_pod_t!(ivec4, IVec4);

    test_pod_t!(uvec2, UVec2);
    test_pod_t!(uvec3, UVec3);
    test_pod_t!(uvec4, UVec4);

    test_pod_t!(i64vec2, I64Vec2);
    test_pod_t!(i64vec3, I64Vec3);
    test_pod_t!(i64vec4, I64Vec4);

    test_pod_t!(u64vec2, U64Vec2);
    test_pod_t!(u64vec3, U64Vec3);
    test_pod_t!(u64vec4, U64Vec4);
}
