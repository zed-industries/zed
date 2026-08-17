#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat3;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat3_transpose,
    "mat3 transpose",
    op => transpose,
    from => random_mat3
);
bench_unop!(
    mat3_determinant,
    "mat3 determinant",
    op => determinant,
    from => random_mat3
);
bench_unop!(mat3_inverse, "mat3 inverse", op => inverse, from => random_mat3);
bench_binop!(mat3_mul_mat3, "mat3 mul mat3", op => mul, from => random_mat3);
bench_from_ypr!(mat3_from_ypr, "mat3 from ypr", ty => Mat3);

bench_binop!(
    mat3_mul_vec3,
    "mat3 mul vec3",
    op => mul,
    from1 => random_mat3,
    from2 => random_vec3
);

bench_binop!(
    mat3_mul_vec3a,
    "mat3 mul vec3a",
    op => mul,
    from1 => random_mat3,
    from2 => random_vec3a
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

criterion_group!(
    benches,
    mat3_transpose,
    mat3_determinant,
    mat3_inverse,
    mat3_mul_vec3,
    mat3_mul_vec3a,
    mat3_mul_mat3,
    mat3_from_ypr,
    mat3_transform_vector2,
    mat3_transform_point2,
);

criterion_main!(benches);
