#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat3A;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat3a_transpose,
    "mat3a transpose",
    op => transpose,
    from => random_mat3a
);
bench_unop!(
    mat3a_determinant,
    "mat3a determinant",
    op => determinant,
    from => random_mat3a
);
bench_unop!(mat3a_inverse, "mat3a inverse", op => inverse, from => random_mat3a);
bench_binop!(mat3a_mul_mat3a, "mat3a mul mat3a", op => mul, from => random_mat3a);
bench_from_ypr!(mat3a_from_ypr, "mat3a from ypr", ty => Mat3A);

bench_binop!(
    mat3a_mul_vec3,
    "mat3a mul vec3",
    op => mul,
    from1 => random_mat3a,
    from2 => random_vec3
);

bench_binop!(
    mat3a_mul_vec3a,
    "mat3a mul vec3a",
    op => mul,
    from1 => random_mat3a,
    from2 => random_vec3a
);

bench_binop!(
    mat3a_transform_point2,
    "mat3a transform point2",
    op => transform_point2,
    from1 => random_srt_mat3a,
    from2 => random_vec2
);

bench_binop!(
    mat3a_transform_vector2,
    "mat3a transform vector2",
    op => transform_vector2,
    from1 => random_srt_mat3a,
    from2 => random_vec2
);

criterion_group!(
    benches,
    mat3a_transpose,
    mat3a_determinant,
    mat3a_inverse,
    mat3a_mul_vec3,
    mat3a_mul_vec3a,
    mat3a_mul_mat3a,
    mat3a_from_ypr,
    mat3a_transform_vector2,
    mat3a_transform_point2,
);

criterion_main!(benches);
