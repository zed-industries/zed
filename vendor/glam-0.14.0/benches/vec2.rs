#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Vec2;
use std::ops::Mul;
use support::{random_mat2, random_srt_mat3, random_vec2};

euler!(
    vec2_euler,
    "vec2 euler",
    ty => Vec2,
    storage => Vec2,
    zero => Vec2::ZERO,
    rand => random_vec2);

bench_binop!(
    mat2_mul_vec2,
    "mat2 mul vec2",
    op => mul,
    from1 => random_mat2,
    from2 => random_vec2
);

bench_binop!(
    mat3_transform_point2,
    "mat3 transform point2",
    op => transform_point2,
    from1 => random_srt_mat3,
    from2 => random_vec2
);

bench_binop!(
    mat3_transform_vector2,
    "mat3 transform vector2",
    op => transform_vector2,
    from1 => random_srt_mat3,
    from2 => random_vec2
);

bench_binop!(
    vec2_angle_between,
    "vec2 angle_between",
    op => angle_between,
    from1 => random_vec2,
    from2 => random_vec2
);

bench_select!(
    vec2_select,
    "vec2 select",
    ty => Vec2,
    op => cmple,
    from => random_vec2
);

criterion_group!(
    benches,
    mat2_mul_vec2,
    mat3_transform_point2,
    mat3_transform_vector2,
    vec2_euler,
    vec2_select,
    vec2_angle_between
);

criterion_main!(benches);
