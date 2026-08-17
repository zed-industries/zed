#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use glam::Mat4;
use std::ops::Mul;
use support::*;

bench_unop!(
    mat4_transpose,
    "mat4 transpose",
    op => transpose,
    from => random_srt_mat4
);

bench_unop!(
    mat4_determinant,
    "mat4 determinant",
    op => determinant,
    from => random_srt_mat4
);

bench_unop!(
    mat4_inverse,
    "mat4 inverse",
    op => inverse,
    from => random_srt_mat4
);

bench_binop!(
    mat4_mul_vec4,
    "mat4 mul vec4",
    op => mul,
    from1 => random_srt_mat4,
    from2 => random_vec4
);

bench_binop!(
    mat4_transform_point3,
    "mat4 transform point3",
    op => transform_point3,
    from1 => random_srt_mat4,
    from2 => random_vec3
);

bench_binop!(
    mat4_transform_vector3,
    "mat4 transform vector3",
    op => transform_vector3,
    from1 => random_srt_mat4,
    from2 => random_vec3
);

bench_binop!(
    mat4_transform_point3a,
    "mat4 transform point3a",
    op => transform_point3a,
    from1 => random_srt_mat4,
    from2 => random_vec3a
);

bench_binop!(
    mat4_transform_vector3a,
    "mat4 transform vector3a",
    op => transform_vector3a,
    from1 => random_srt_mat4,
    from2 => random_vec3a
);

bench_binop!(
    mat4_mul_mat4,
    "mat4 mul mat4",
    op => mul,
    from => random_srt_mat4
);

bench_from_ypr!(
    mat4_from_ypr,
    "mat4 from ypr",
    ty => Mat4
);

pub fn mat4_from_srt(c: &mut Criterion) {
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
    let mut outputs = vec![Mat4::default(); SIZE];
    let mut i = 0;
    c.bench_function("mat4 from srt", |b| {
        b.iter(|| {
            i = (i + 1) & (SIZE - 1);
            unsafe {
                let data = inputs.get_unchecked(i);
                *outputs.get_unchecked_mut(i) =
                    Mat4::from_scale_rotation_translation(data.0, data.1, data.2);
            }
        });
    });
}

criterion_group!(
    benches,
    mat4_determinant,
    mat4_from_srt,
    mat4_from_ypr,
    mat4_inverse,
    mat4_mul_mat4,
    mat4_mul_vec4,
    mat4_transform_point3,
    mat4_transform_point3a,
    mat4_transform_vector3,
    mat4_transform_vector3a,
    mat4_transpose,
);

criterion_main!(benches);
