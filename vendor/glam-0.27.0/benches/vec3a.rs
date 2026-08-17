#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::{Vec3, Vec3A};
use std::ops::Mul;
use support::*;

bench_binop!(
    vec3a_mul_vec3a,
    "vec3a mul vec3a",
    op => mul,
    from1 => random_vec3a,
    from2 => random_vec3a
);

#[inline]
fn vec3a_to_rgb_op(v: Vec3A) -> u32 {
    let (red, green, blue) = (v.min(Vec3A::ONE).max(Vec3A::ZERO) * 255.0).into();
    (red as u32) << 16 | (green as u32) << 8 | (blue as u32)
}

#[inline]
fn vec3a_deref(v: Vec3A) -> [f32; 3] {
    [v.x, v.y, v.z]
}

#[inline]
fn vec3a_into_array(v: Vec3A) -> [f32; 3] {
    v.into()
}

#[inline]
fn vec3a_into_tuple(v: Vec3A) -> (f32, f32, f32) {
    v.into()
}

#[inline]
fn vec3a_into_vec3(v: Vec3A) -> Vec3 {
    v.into()
}

bench_func!(
vec3a_to_vec3,
"vec3a into vec3",
op => vec3a_into_vec3,
from => random_vec3a
);

bench_func!(
vec3a_to_rgb,
"vec3a to rgb",
op => vec3a_to_rgb_op,
from => random_vec3a
);

bench_func!(
vec3a_to_array_deref,
"vec3a into array deref",
op => vec3a_deref,
from => random_vec3a
);

bench_func!(
vec3a_to_array_into,
"vec3a into array fast",
op => vec3a_into_array,
from => random_vec3a
);

bench_func!(
vec3a_to_tuple_into,
"vec3a into tuple fast",
op => vec3a_into_tuple,
from => random_vec3a
);

euler!(vec3a_euler, "vec3a euler", ty => Vec3A, storage => Vec3A, zero => Vec3A::ZERO, rand => random_vec3a);

bench_binop!(
    vec3a_angle_between,
    "vec3a angle_between",
    op => angle_between,
    from1 => random_vec3a,
    from2 => random_vec3a
);

bench_select!(
    vec3a_select,
    "vec3a select",
    ty => Vec3A,
    op => cmple,
    from => random_vec3a
);

criterion_group!(
    benches,
    vec3a_mul_vec3a,
    vec3a_angle_between,
    vec3a_euler,
    vec3a_select,
    vec3a_to_array_deref,
    vec3a_to_array_into,
    vec3a_to_rgb,
    vec3a_to_tuple_into,
    vec3a_to_vec3,
);

criterion_main!(benches);
