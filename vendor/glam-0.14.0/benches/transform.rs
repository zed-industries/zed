#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::{TransformRT, TransformSRT};
use std::ops::Mul;
use support::*;

fn random_transform_srt(rng: &mut PCG32) -> TransformSRT {
    TransformSRT::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
    )
}

fn random_transform_rt(rng: &mut PCG32) -> TransformRT {
    TransformRT::from_rotation_translation(random_quat(rng), random_vec3(rng))
}

bench_unop!(
    transform_srt_inverse,
    "transform_srt inverse",
    op => inverse,
    from => random_transform_srt
);

bench_binop!(
    vec3_mul_transform_srt,
    "transform_srt mul vec3",
    op => mul,
    from1 => random_transform_srt,
    from2 => random_vec3
);

bench_binop!(
    vec3_mul_transform_rt,
    "transform_rt mul vec3",
    op => mul,
    from1 => random_transform_rt,
    from2 => random_vec3
);

// bench_unop!(
//     transform_srt_inverse_ptv_scale,
//     "transform_srt inverse (+ve scale)",
//     op => inverse,
//     ty => TransformSRT,
//     from => TransformRT
// );
// bench_unop!(
//     transform_rt_inverse,
//     "transform_rt inverse",
//     op => inverse,
//     ty => TransformRT
// );

bench_binop!(
    transform_srt_mul_srt,
    "transform_srt * transform_srt",
    op => mul,
    from => random_transform_srt
);

bench_binop!(
    transform_rt_mul_rt,
    "transform_rt * transform_rt",
    op => mul,
    from => random_transform_rt
);

criterion_group!(
    benches,
    // transform_rt_inverse,
    transform_srt_inverse,
    // transform_srt_inverse_ptv_scale,
    transform_rt_mul_rt,
    transform_srt_mul_srt,
    vec3_mul_transform_rt,
    vec3_mul_transform_srt,
);

criterion_main!(benches);
