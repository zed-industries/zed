#![allow(dead_code)]
use core::f32;
use glam::{Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};

pub struct PCG32 {
    state: u64,
    inc: u64,
}

impl PCG32 {
    pub fn seed(initstate: u64, initseq: u64) -> Self {
        let mut rng = PCG32 {
            state: 0,
            inc: (initseq << 1) | 1,
        };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(initstate);
        rng.next_u32();
        rng
    }

    pub fn default() -> Self {
        PCG32::seed(0x853c49e6748fea9b, 0xda3e39cb94b95bdb)
    }

    pub fn next_u32(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc | 1);
        let xorshifted = ((oldstate >> 18) ^ oldstate) >> 27;
        let rot = oldstate >> 59;
        ((xorshifted >> rot) | (xorshifted << (rot.wrapping_neg() & 31))) as u32
    }

    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() & 0xffffff) as f32 / 16777216.0
    }
}

pub fn random_vec2(rng: &mut PCG32) -> Vec2 {
    Vec2::new(rng.next_f32(), rng.next_f32())
}

pub fn random_vec3(rng: &mut PCG32) -> Vec3 {
    Vec3::new(rng.next_f32(), rng.next_f32(), rng.next_f32())
}

pub fn random_vec3a(rng: &mut PCG32) -> Vec3A {
    Vec3A::new(rng.next_f32(), rng.next_f32(), rng.next_f32())
}

pub fn random_vec4(rng: &mut PCG32) -> Vec4 {
    Vec4::new(
        rng.next_f32(),
        rng.next_f32(),
        rng.next_f32(),
        rng.next_f32(),
    )
}

pub fn random_nonzero_vec2(rng: &mut PCG32) -> Vec2 {
    loop {
        let v = random_vec2(rng);
        if v.length_squared() > 0.01 {
            return v;
        }
    }
}

pub fn random_nonzero_vec3(rng: &mut PCG32) -> Vec3 {
    loop {
        let v = random_vec3(rng);
        if v.length_squared() > 0.01 {
            return v;
        }
    }
}

pub fn random_f32(rng: &mut PCG32) -> f32 {
    rng.next_f32()
}

pub fn random_radians(rng: &mut PCG32) -> f32 {
    -f32::consts::PI + rng.next_f32() * 2.0 * f32::consts::PI
}

pub fn random_quat(rng: &mut PCG32) -> Quat {
    let yaw = random_radians(rng);
    let pitch = random_radians(rng);
    let roll = random_radians(rng);
    Quat::from_euler(glam::EulerRot::YXZ, yaw, pitch, roll)
}

pub fn random_mat2(rng: &mut PCG32) -> Mat2 {
    Mat2::from_cols(random_vec2(rng), random_vec2(rng))
}

pub fn random_mat3(rng: &mut PCG32) -> Mat3 {
    Mat3::from_cols(random_vec3(rng), random_vec3(rng), random_vec3(rng))
}

pub fn random_srt_mat3(rng: &mut PCG32) -> Mat3 {
    Mat3::from_scale_angle_translation(
        random_nonzero_vec2(rng),
        random_radians(rng),
        random_vec2(rng),
    )
}

pub fn random_mat3a(rng: &mut PCG32) -> Mat3A {
    Mat3A::from_cols(random_vec3a(rng), random_vec3a(rng), random_vec3a(rng))
}

pub fn random_srt_mat3a(rng: &mut PCG32) -> Mat3A {
    Mat3A::from_scale_angle_translation(
        random_nonzero_vec2(rng),
        random_radians(rng),
        random_vec2(rng),
    )
}

pub fn random_srt_mat4(rng: &mut PCG32) -> Mat4 {
    Mat4::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
    )
}
