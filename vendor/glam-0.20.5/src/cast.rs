use crate::{DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4};
use crate::{IVec2, IVec3, IVec4};
use crate::{Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};
use crate::{UVec2, UVec3, UVec4};
#[cfg(target_feature = "simd128")]
use core::arch::wasm32::v128;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[repr(C)]
pub union Vec4Cast {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub m128: __m128,
    #[cfg(target_feature = "simd128")]
    pub v128: v128,
    pub fx4: [f32; 4],
    pub fx2x2: [[f32; 2]; 2],
    pub v4: Vec4,
    pub v3a: Vec3A,
    pub q: Quat,
}

#[repr(C)]
pub union Vec3Cast {
    pub fx3: [f32; 3],
    pub v3: Vec3,
}

#[repr(C)]
pub union Vec2Cast {
    pub fx2: [f32; 2],
    pub v2: Vec2,
}

#[repr(C)]
pub union F32x9Cast {
    pub fx3x3: [[f32; 3]; 3],
    pub fx9: [f32; 9],
}

#[repr(C)]
pub union F32x16Cast {
    pub fx4x4: [[f32; 4]; 4],
    pub fx16: [f32; 16],
}

#[repr(C)]
pub union Mat4Cast {
    pub v4x4: [Vec4; 4],
    pub m4: Mat4,
}

#[repr(C)]
pub union Mat3Cast {
    pub v3x3: [Vec3; 3],
    pub m3: Mat3,
}

#[repr(C)]
pub union Mat3ACast {
    pub v3x3: [Vec3A; 3],
    pub m3: Mat3A,
}

#[repr(C)]
pub union Mat2Cast {
    pub v2x2: [Vec2; 2],
    pub m2: Mat2,
}

#[repr(C)]
pub union DVec4Cast {
    pub fx4: [f64; 4],
    pub fx2x2: [[f64; 2]; 2],
    pub v4: DVec4,
    pub q: DQuat,
}

#[repr(C)]
pub union DVec3Cast {
    pub fx3: [f64; 3],
    pub v3: DVec3,
}

#[repr(C)]
pub union DVec2Cast {
    pub fx2: [f64; 2],
    pub v2: DVec2,
}

#[repr(C)]
pub union F64x9Cast {
    pub fx3x3: [[f64; 3]; 3],
    pub fx9: [f64; 9],
}

#[repr(C)]
pub union F64x16Cast {
    pub fx4x4: [[f64; 4]; 4],
    pub fx16: [f64; 16],
}

#[repr(C)]
pub union DMat4Cast {
    pub v4x4: [DVec4; 4],
    pub m4: DMat4,
}

#[repr(C)]
pub union DMat3Cast {
    pub v3x3: [DVec3; 3],
    pub m3: DMat3,
}

#[repr(C)]
pub union DMat2Cast {
    pub v2x2: [DVec2; 2],
    pub m2: DMat2,
}

#[repr(C)]
pub union IVec4Cast {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub m128: __m128i,
    #[cfg(target_feature = "simd128")]
    pub v128: v128,
    pub ix4: [i32; 4],
    pub ix2x2: [[i32; 2]; 2],
    pub v4: IVec4,
}

#[repr(C)]
pub union IVec3Cast {
    pub ix3: [i32; 3],
    pub v3: IVec3,
}

#[repr(C)]
pub union IVec2Cast {
    pub ix2: [i32; 2],
    pub v2: IVec2,
}

#[repr(C)]
pub union UVec4Cast {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub m128: __m128,
    #[cfg(target_feature = "simd128")]
    pub v128: v128,
    pub ux4: [u32; 4],
    pub ux2x2: [[u32; 2]; 2],
    pub v4: UVec4,
}

#[repr(C)]
pub union UVec3Cast {
    pub ux3: [u32; 3],
    pub v3: UVec3,
}

#[repr(C)]
pub union UVec2Cast {
    pub ux2: [u32; 2],
    pub v2: UVec2,
}
