#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Quat;
use std::ops::Mul;
use support::*;

bench_unop!(
    quat_conjugate,
    "quat conjugate",
    op => conjugate,
    from => random_quat
);

bench_binop!(
    quat_mul_vec3,
    "quat mul vec3",
    op => mul,
    from1 => random_quat,
    from2 => random_vec3
);

bench_binop!(
    quat_mul_vec3a,
    "quat mul vec3a",
    op => mul,
    from1 => random_quat,
    from2 => random_vec3a
);

bench_binop!(
    quat_mul_quat,
    "quat mul quat",
    op => mul,
    from => random_quat
);

bench_binop!(
    quat_dot,
    "quat dot",
    op => dot,
    from => random_quat
);

bench_trinop!(
    quat_lerp,
    "quat lerp",
    op => lerp,
    from1 => random_quat,
    from2 => random_quat,
    from3 => random_f32
);

bench_trinop!(
    quat_slerp,
    "quat slerp",
    op => slerp,
    from1 => random_quat,
    from2 => random_quat,
    from3 => random_f32
);

bench_from_ypr!(quat_from_ypr, "quat from ypr", ty => Quat);

criterion_group!(
    benches,
    quat_conjugate,
    quat_dot,
    quat_lerp,
    quat_slerp,
    quat_mul_quat,
    quat_mul_vec3,
    quat_mul_vec3a,
    quat_from_ypr
);

criterion_main!(benches);
