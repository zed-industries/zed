#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Vec3;
use std::ops::Mul;
use support::*;

bench_binop!(
    vec3_mul_vec3,
    "vec3 mul vec3",
    op => mul,
    from1 => random_vec3,
    from2 => random_vec3
);

#[inline]
fn vec3_to_rgb_op(v: Vec3) -> u32 {
    let (red, green, blue) = (v.min(Vec3::ONE).max(Vec3::ZERO) * 255.0).into();
    ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
}

#[inline]
fn vec3_fields(v: Vec3) -> [f32; 3] {
    [v.x, v.y, v.z]
}

#[inline]
fn vec3_into_array(v: Vec3) -> [f32; 3] {
    v.into()
}

#[inline]
fn vec3_into_tuple(v: Vec3) -> (f32, f32, f32) {
    v.into()
}

bench_func!(
vec3_to_rgb,
"vec3 to rgb",
op => vec3_to_rgb_op,
from => random_vec3
);

bench_func!(
vec3_to_array_fields,
"vec3 into array fields",
op => vec3_fields,
from => random_vec3
);

bench_func!(
vec3_to_array_into,
"vec3 into array fast",
op => vec3_into_array,
from => random_vec3
);

bench_func!(
vec3_to_tuple_into,
"vec3 into tuple fast",
op => vec3_into_tuple,
from => random_vec3
);

// ---

#[inline]
fn vec3_normalize(v: Vec3) -> Vec3 {
    v.normalize()
}

bench_func!(
    vec3_normalize_bench,
    "vec3 normalize",
    op => vec3_normalize,
    from => random_vec3
);

#[inline]
fn vec3_normalize_or(v: Vec3) -> Vec3 {
    v.normalize_or(Vec3::X)
}

bench_func!(
    vec3_normalize_or_bench,
    "vec3 normalize_or",
    op => vec3_normalize_or,
    from => random_vec3
);

#[inline]
fn vec3_normalize_or_zero(v: Vec3) -> Vec3 {
    v.normalize_or_zero()
}

bench_func!(
    vec3_normalize_or_zero_bench,
    "vec3 normalize_or_zero",
    op => vec3_normalize_or_zero,
    from => random_vec3
);

// ---

#[inline(always)]
fn vec3_any_orthogonal_vector(v: Vec3) -> Vec3 {
    v.any_orthogonal_vector()
}

bench_func!(
    vec3_any_orthogonal_vector_bench,
    "vec3 any_orthogonal_vector",
    op => vec3_any_orthogonal_vector,
    from => random_vec3
);

#[inline(always)]
fn vec3_any_orthonormal_vector(v: Vec3) -> Vec3 {
    v.any_orthonormal_vector()
}

bench_func!(
    vec3_any_orthonormal_vector_bench,
    "vec3 any_orthonormal_vector",
    op => vec3_any_orthonormal_vector,
    from => random_vec3
);

#[inline(always)]
fn vec3_any_orthonormal_pair(v: Vec3) -> (Vec3, Vec3) {
    v.any_orthonormal_pair()
}

bench_func!(
    vec3_any_orthonormal_pair_bench,
    "vec3 any_orthonormal_pair",
    op => vec3_any_orthonormal_pair,
    from => random_vec3
);

// ---

euler!(vec3_euler, "vec3 euler", ty => Vec3, storage => Vec3, zero => Vec3::ZERO, rand => random_vec3);

bench_binop!(
    vec3_angle_between,
    "vec3 angle_between",
    op => angle_between,
    from1 => random_vec3,
    from2 => random_vec3
);

bench_binop!(
    vec3_cross,
    "vec3 cross",
    op => cross,
    from1 => random_vec3,
    from2 => random_vec3
);

bench_binop!(
    vec3_dot,
    "vec3 dot",
    op => dot,
    from1 => random_vec3,
    from2 => random_vec3
);

bench_unop!(
    vec3_length,
    "vec3 length",
    op => length,
    from => random_vec3
);

bench_select!(
    vec3_select,
    "vec3 select",
    ty => Vec3,
    op => cmple,
    from => random_vec3
);

criterion_group!(
    benches,
    vec3_angle_between,
    vec3_any_orthogonal_vector_bench,
    vec3_any_orthonormal_pair_bench,
    vec3_any_orthonormal_vector_bench,
    vec3_cross,
    vec3_dot,
    vec3_euler,
    vec3_length,
    vec3_mul_vec3,
    vec3_normalize_bench,
    vec3_normalize_or_bench,
    vec3_normalize_or_zero_bench,
    vec3_select,
    vec3_to_array_fields,
    vec3_to_array_into,
    vec3_to_rgb,
    vec3_to_tuple_into,
);

criterion_main!(benches);
