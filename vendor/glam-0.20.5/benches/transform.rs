#![allow(deprecated)]

#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::{TransformRT, TransformSRT};
use std::ops::Mul;
use support::*;

fn random_transformsrt(rng: &mut PCG32) -> TransformSRT {
    TransformSRT::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
    )
}

fn random_transformrt(rng: &mut PCG32) -> TransformRT {
    TransformRT::from_rotation_translation(random_quat(rng), random_vec3(rng))
}

bench_unop!(
    transformrt_inverse,
    "transform_rt inverse",
    op => inverse,
    from => random_transformrt
);

bench_unop!(
    transformsrt_inverse,
    "transform_srt inverse",
    op => inverse,
    from => random_transformsrt
);

bench_binop!(
    transformrt_transform_point3,
    "transform_rt transform point3",
    op => transform_point3,
    from1 => random_transformrt,
    from2 => random_vec3
);

bench_binop!(
    transformrt_transform_point3a,
    "transform_rt transform point3a",
    op => transform_point3a,
    from1 => random_transformrt,
    from2 => random_vec3a
);

bench_binop!(
    transformrt_transform_vector3,
    "transform_rt transform vector3",
    op => transform_vector3,
    from1 => random_transformrt,
    from2 => random_vec3
);

bench_binop!(
    transformrt_transform_vector3a,
    "transform_rt transform vector3a",
    op => transform_vector3a,
    from1 => random_transformrt,
    from2 => random_vec3a
);

bench_binop!(
    transformsrt_transform_point3,
    "transform_srt transform point3",
    op => transform_point3,
    from1 => random_transformsrt,
    from2 => random_vec3
);

bench_binop!(
    transformsrt_transform_point3a,
    "transform_srt transform point3a",
    op => transform_point3a,
    from1 => random_transformsrt,
    from2 => random_vec3a
);

bench_binop!(
    transformsrt_transform_vector3,
    "transform_srt transform vector3",
    op => transform_vector3,
    from1 => random_transformsrt,
    from2 => random_vec3
);

bench_binop!(
    transformsrt_transform_vector3a,
    "transform_srt transform vector3a",
    op => transform_vector3a,
    from1 => random_transformsrt,
    from2 => random_vec3a
);
bench_binop!(
    transformsrt_mul_transformsrt,
    "transform_srt mul transform_srt",
    op => mul,
    from => random_transformsrt
);

bench_binop!(
    transformrt_mul_transformrt,
    "transform_rt mul transform_rt",
    op => mul,
    from => random_transformrt
);

criterion_group!(
    benches,
    transformrt_inverse,
    transformrt_mul_transformrt,
    transformrt_transform_point3,
    transformrt_transform_point3a,
    transformrt_transform_vector3,
    transformrt_transform_vector3a,
    transformsrt_inverse,
    transformsrt_mul_transformsrt,
    transformsrt_transform_point3,
    transformsrt_transform_point3a,
    transformsrt_transform_vector3,
    transformsrt_transform_vector3a,
);

criterion_main!(benches);
