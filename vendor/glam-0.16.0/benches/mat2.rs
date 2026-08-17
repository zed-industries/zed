#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use std::ops::Mul;
use support::*;

bench_binop!(
    mat2_mul_vec2,
    "mat2 mul vec2",
    op => mul,
    from1 => random_mat2,
    from2 => random_vec2
);

bench_unop!(
    mat2_transpose,
    "mat2 transpose",
    op => transpose,
    from => random_mat2
);
bench_unop!(
    mat2_determinant,
    "mat2 determinant",
    op => determinant,
    from => random_mat2
);
bench_unop!(mat2_inverse, "mat2 inverse", op => inverse, from => random_mat2);
bench_binop!(mat2_mul_mat2, "mat2 mul mat2", op => mul, from => random_mat2);

criterion_group!(
    benches,
    mat2_transpose,
    mat2_determinant,
    mat2_inverse,
    mat2_mul_vec2,
    mat2_mul_mat2,
);

criterion_main!(benches);
