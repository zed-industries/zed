#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Vec4;
use std::ops::Mul;
use support::{random_srt_mat4, random_vec4};

bench_binop!(
    vec4_mul_mat4,
    "vec4 mul mat4",
    op => mul,
    from1 => random_srt_mat4,
    from2 => random_vec4
);

bench_select!(
    vec4_select,
    "vec4 select",
    ty => Vec4,
    op => cmple,
    from => random_vec4
);

criterion_group!(benches, vec4_mul_mat4, vec4_select,);

criterion_main!(benches);
