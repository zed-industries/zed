use crate::{
    DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat,
    UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};
use bytemuck::{Pod, Zeroable};

unsafe impl Pod for Mat2 {}
unsafe impl Zeroable for Mat2 {}
unsafe impl Pod for Mat3 {}
unsafe impl Zeroable for Mat3 {}
unsafe impl Pod for Mat4 {}
unsafe impl Zeroable for Mat4 {}

unsafe impl Pod for Quat {}
unsafe impl Zeroable for Quat {}

unsafe impl Pod for Vec2 {}
unsafe impl Zeroable for Vec2 {}
unsafe impl Pod for Vec3 {}
unsafe impl Zeroable for Vec3 {}
unsafe impl Pod for Vec4 {}
unsafe impl Zeroable for Vec4 {}

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

#[cfg(test)]
mod test {
    use crate::{
        DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4,
        Quat, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
    };
    use core::mem;

    macro_rules! test_t {
        ($name:ident, $t:ty) => {
            #[test]
            fn $name() {
                let t = <$t>::default();
                let b = bytemuck::bytes_of(&t);
                assert_eq!(t.as_ref().as_ptr() as usize, b.as_ptr() as usize);
                assert_eq!(b.len(), mem::size_of_val(&t));
            }
        };
    }

    test_t!(mat2, Mat2);
    test_t!(mat3, Mat3);
    test_t!(mat4, Mat4);
    test_t!(quat, Quat);
    test_t!(vec2, Vec2);
    test_t!(vec3, Vec3);
    test_t!(vec4, Vec4);

    test_t!(dmat2, DMat2);
    test_t!(dmat3, DMat3);
    test_t!(dmat4, DMat4);
    test_t!(dquat, DQuat);
    test_t!(dvec2, DVec2);
    test_t!(dvec3, DVec3);
    test_t!(dvec4, DVec4);

    test_t!(ivec2, IVec2);
    test_t!(ivec3, IVec3);
    test_t!(ivec4, IVec4);

    test_t!(uvec2, UVec2);
    test_t!(uvec3, UVec3);
    test_t!(uvec4, UVec4);
}
