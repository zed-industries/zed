use crate::{
    Affine2, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4,
    I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat3A,
    Mat4, Quat, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2,
    Vec3, Vec3A, Vec4,
};
use bytemuck::{AnyBitPattern, Pod, Zeroable};

// Affine2 contains internal padding due to Mat2 using SIMD
unsafe impl AnyBitPattern for Affine2 {}
unsafe impl Zeroable for Affine2 {}
unsafe impl AnyBitPattern for Affine3A {}
unsafe impl Zeroable for Affine3A {}

unsafe impl Pod for Mat2 {}
unsafe impl Zeroable for Mat2 {}
unsafe impl Pod for Mat3 {}
unsafe impl Zeroable for Mat3 {}
unsafe impl AnyBitPattern for Mat3A {}
unsafe impl Zeroable for Mat3A {}
unsafe impl Pod for Mat4 {}
unsafe impl Zeroable for Mat4 {}

unsafe impl Pod for Quat {}
unsafe impl Zeroable for Quat {}

unsafe impl Pod for Vec2 {}
unsafe impl Zeroable for Vec2 {}
unsafe impl Pod for Vec3 {}
unsafe impl Zeroable for Vec3 {}
unsafe impl AnyBitPattern for Vec3A {}
unsafe impl Zeroable for Vec3A {}
unsafe impl Pod for Vec4 {}
unsafe impl Zeroable for Vec4 {}

unsafe impl Pod for DAffine2 {}
unsafe impl Zeroable for DAffine2 {}
unsafe impl Pod for DAffine3 {}
unsafe impl Zeroable for DAffine3 {}

unsafe impl Pod for DMat2 {}
unsafe impl Zeroable for DMat2 {}
unsafe impl Pod for DMat3 {}
unsafe impl Zeroable for DMat3 {}
unsafe impl Pod for DMat4 {}
unsafe impl Zeroable for DMat4 {}

unsafe impl Pod for DQuat {}
unsafe impl Zeroable for DQuat {}

unsafe impl Pod for DVec2 {}
unsafe impl Zeroable for DVec2 {}
unsafe impl Pod for DVec3 {}
unsafe impl Zeroable for DVec3 {}
unsafe impl Pod for DVec4 {}
unsafe impl Zeroable for DVec4 {}

unsafe impl Pod for I16Vec2 {}
unsafe impl Zeroable for I16Vec2 {}
unsafe impl Pod for I16Vec3 {}
unsafe impl Zeroable for I16Vec3 {}
unsafe impl Pod for I16Vec4 {}
unsafe impl Zeroable for I16Vec4 {}

unsafe impl Pod for U16Vec2 {}
unsafe impl Zeroable for U16Vec2 {}
unsafe impl Pod for U16Vec3 {}
unsafe impl Zeroable for U16Vec3 {}
unsafe impl Pod for U16Vec4 {}
unsafe impl Zeroable for U16Vec4 {}

unsafe impl Pod for IVec2 {}
unsafe impl Zeroable for IVec2 {}
unsafe impl Pod for IVec3 {}
unsafe impl Zeroable for IVec3 {}
unsafe impl Pod for IVec4 {}
unsafe impl Zeroable for IVec4 {}

unsafe impl Pod for UVec2 {}
unsafe impl Zeroable for UVec2 {}
unsafe impl Pod for UVec3 {}
unsafe impl Zeroable for UVec3 {}
unsafe impl Pod for UVec4 {}
unsafe impl Zeroable for UVec4 {}

unsafe impl Pod for I64Vec2 {}
unsafe impl Zeroable for I64Vec2 {}
unsafe impl Pod for I64Vec3 {}
unsafe impl Zeroable for I64Vec3 {}
unsafe impl Pod for I64Vec4 {}
unsafe impl Zeroable for I64Vec4 {}

unsafe impl Pod for U64Vec2 {}
unsafe impl Zeroable for U64Vec2 {}
unsafe impl Pod for U64Vec3 {}
unsafe impl Zeroable for U64Vec3 {}
unsafe impl Pod for U64Vec4 {}
unsafe impl Zeroable for U64Vec4 {}

#[cfg(test)]
mod test {
    use crate::{
        Affine2, Affine3A, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4,
        I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4, Mat2, Mat3,
        Mat3A, Mat4, Quat, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3,
        UVec4, Vec2, Vec3, Vec3A, Vec4,
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
