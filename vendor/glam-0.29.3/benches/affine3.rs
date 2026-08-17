#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Affine3A;
use std::ops::Mul;
use support::*;

pub fn random_srt_affine3a(rng: &mut PCG32) -> Affine3A {
    Affine3A::from_scale_rotation_translation(
        random_nonzero_vec3(rng),
        random_quat(rng),
        random_vec3(rng),
    )
}

bench_unop!(affine3a_inverse, "affine3a inverse", op => inverse, from => random_srt_affine3a);

bench_binop!(
    affine3a_transform_point3,
    "affine3a transform point3",
    op => transform_point3,
    from1 => random_srt_affine3a,
    from2 => random_vec3
);

bench_binop!(
    affine3a_transform_vector3,
    "affine3a transform vector3",
    op => transform_vector3,
    from1 => random_srt_affine3a,
    from2 => random_vec3
);

bench_binop!(
    affine3a_transform_point3a,
    "affine3a transform point3a",
    op => transform_point3a,
    from1 => random_srt_affine3a,
    from2 => random_vec3a
);

bench_binop!(
    affine3a_transform_vector3a,
    "affine3a transform vector3a",
    op => transform_vector3a,
    from1 => random_srt_affine3a,
    from2 => random_vec3a
);

bench_binop!(
    affine3a_mul_affine3a,
    "affine3a mul affine3a",
    op => mul,
    from => random_srt_affine3a
);

bench_binop!(affine3a_mul_mat4,
    "affine3a mul mat4",
    op => mul,
    from1 => random_srt_affine3a,
    from2 => random_srt_mat4
);

bench_binop!(
    mat4_mul_affine3a,
    "mat4 mul affine3a",
    op => mul,
    from1 => random_srt_mat4,
    from2 => random_srt_affine3a
);

pub fn affine3a_from_srt(c: &mut Criterion) {
    use glam::{Quat, Vec3};
    const SIZE: usize = 1 << 13;
    let mut rng = support::PCG32::default();
    let inputs = criterion::black_box(
        (0..SIZE)
            .map(|_| {
                (
                    random_nonzero_vec3(&mut rng),
                    random_quat(&mut rng),
                    random_vec3(&mut rng),
                )
            })
            .collect::<Vec<(Vec3, Quat, Vec3)>>(),
    );
    let mut outputs = vec![Affine3A::IDENTITY; SIZE];
    let mut i = 0;
    c.bench_function("affine3a from srt", |b| {
        b.iter(|| {
            i = (i + 1) & (SIZE - 1);
            unsafe {
                let data = inputs.get_unchecked(i);
                *outputs.get_unchecked_mut(i) =
                    Affine3A::from_scale_rotation_translation(data.0, data.1, data.2);
            }
        });
    });
}

criterion_group!(
    benches,
    affine3a_from_srt,
    affine3a_inverse,
    affine3a_mul_affine3a,
    affine3a_mul_mat4,
    affine3a_transform_point3,
    affine3a_transform_point3a,
    affine3a_transform_vector3,
    affine3a_transform_vector3a,
    mat4_mul_affine3a,
);

criterion_main!(benches);
