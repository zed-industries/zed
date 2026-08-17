#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Vec2;
use std::ops::Mul;
use support::*;

euler!(
    vec2_euler,
    "vec2 euler",
    ty => Vec2,
    storage => Vec2,
    zero => Vec2::ZERO,
    rand => random_vec2);

bench_binop!(
    vec2_mul_vec2,
    "vec2 mul vec2",
    op => mul,
    from1 => random_vec2,
    from2 => random_vec2
);

bench_binop!(
    vec2_angle_to,
    "vec2 angle_to",
    op => angle_to,
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
    vec2_mul_vec2,
    vec2_euler,
    vec2_select,
    vec2_angle_to
);

criterion_main!(benches);
