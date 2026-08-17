#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Affine2;
use std::ops::Mul;
use support::*;

pub fn random_srt_affine2(rng: &mut PCG32) -> Affine2 {
    Affine2::from_scale_angle_translation(
        random_nonzero_vec2(rng),
        random_radians(rng),
        random_vec2(rng),
    )
}

bench_unop!(affine2_inverse, "affine2 inverse", op => inverse, from => random_srt_affine2);
bench_binop!(
    affine2_transform_point2,
    "affine2 transform point2",
    op => transform_point2,
    from1 => random_srt_affine2,
    from2 => random_vec2
);

bench_binop!(
    affine2_transform_vector2,
    "affine2 transform vector2",
    op => transform_vector2,
    from1 => random_srt_affine2,
    from2 => random_vec2
);
bench_binop!(affine2_mul_affine2, "affine2 mul affine2", op => mul, from => random_srt_affine2);
bench_binop!(affine2_mul_mat3, "affine2 mul mat3", op => mul, from1 => random_srt_affine2, from2 => random_srt_mat3);
bench_binop!(mat3_mul_affine2, "mat3 mul affine2", op => mul, from1 => random_srt_mat3, from2 => random_srt_affine2);

pub fn affine2_from_srt(c: &mut Criterion) {
    use glam::Vec2;
    const SIZE: usize = 1 << 13;
    let mut rng = support::PCG32::default();
    let inputs = criterion::black_box(
        (0..SIZE)
            .map(|_| {
                (
                    random_nonzero_vec2(&mut rng),
                    random_radians(&mut rng),
                    random_vec2(&mut rng),
                )
            })
            .collect::<Vec<(Vec2, f32, Vec2)>>(),
    );
    let mut outputs = vec![Affine2::default(); SIZE];
    let mut i = 0;
    c.bench_function("affine2 from srt", |b| {
        b.iter(|| {
            i = (i + 1) & (SIZE - 1);
            unsafe {
                let data = inputs.get_unchecked(i);
                *outputs.get_unchecked_mut(i) =
                    Affine2::from_scale_angle_translation(data.0, data.1, data.2);
            }
        });
    });
}

criterion_group!(
    benches,
    affine2_inverse,
    affine2_transform_point2,
    affine2_transform_vector2,
    affine2_mul_affine2,
    affine2_mul_mat3,
    mat3_mul_affine2,
    affine2_from_srt,
);

criterion_main!(benches);
