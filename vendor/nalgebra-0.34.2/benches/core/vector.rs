use na::{DVector, SVector, Vector2, Vector3, Vector4};
use rand::Rng;
use rand_isaac::IsaacRng;

use std::ops::{Add, Div, Mul, Sub};

#[path = "../common/macros.rs"]
mod macros;

bench_binop!(vec2_add_v_f32, Vector2<f32>, Vector2<f32>, add);
bench_binop!(vec3_add_v_f32, Vector3<f32>, Vector3<f32>, add);
bench_binop!(vec4_add_v_f32, Vector4<f32>, Vector4<f32>, add);

bench_binop!(vec2_add_v_f64, Vector2<f64>, Vector2<f64>, add);
bench_binop!(vec3_add_v_f64, Vector3<f64>, Vector3<f64>, add);
bench_binop!(vec4_add_v_f64, Vector4<f64>, Vector4<f64>, add);

bench_binop!(vec2_sub_v, Vector2<f32>, Vector2<f32>, sub);
bench_binop!(vec3_sub_v, Vector3<f32>, Vector3<f32>, sub);
bench_binop!(vec4_sub_v, Vector4<f32>, Vector4<f32>, sub);

bench_binop!(vec2_mul_s, Vector2<f32>, f32, mul);
bench_binop!(vec3_mul_s, Vector3<f32>, f32, mul);
bench_binop!(vec4_mul_s, Vector4<f32>, f32, mul);

bench_binop!(vec2_div_s, Vector2<f32>, f32, div);
bench_binop!(vec3_div_s, Vector3<f32>, f32, div);
bench_binop!(vec4_div_s, Vector4<f32>, f32, div);

bench_binop_ref!(vec2_dot_f32, Vector2<f32>, Vector2<f32>, dot);
bench_binop_ref!(vec3_dot_f32, Vector3<f32>, Vector3<f32>, dot);
bench_binop_ref!(vec4_dot_f32, Vector4<f32>, Vector4<f32>, dot);

bench_binop_ref!(vec2_dot_f64, Vector2<f64>, Vector2<f64>, dot);
bench_binop_ref!(vec3_dot_f64, Vector3<f64>, Vector3<f64>, dot);
bench_binop_ref!(vec4_dot_f64, Vector4<f64>, Vector4<f64>, dot);

bench_binop_ref!(vec3_cross, Vector3<f32>, Vector3<f32>, cross);

bench_unop!(vec2_norm, Vector2<f32>, norm);
bench_unop!(vec3_norm, Vector3<f32>, norm);
bench_unop!(vec4_norm, Vector4<f32>, norm);

bench_unop!(vec2_normalize, Vector2<f32>, normalize);
bench_unop!(vec3_normalize, Vector3<f32>, normalize);
bench_unop!(vec4_normalize, Vector4<f32>, normalize);

bench_binop_ref!(vec10000_dot_f64, SVector<f64, 10000>, SVector<f64, 10000>, dot);
bench_binop_ref!(vec10000_dot_f32, SVector<f32, 10000>, SVector<f32, 10000>, dot);

fn vec10000_axpy_f64(bh: &mut criterion::Criterion) {
    use rand::SeedableRng;

    let mut rng = IsaacRng::seed_from_u64(0);

    bh.bench_function("vec10000_axpy_f64", |bh| {
        bh.iter_batched(
            || {
                (
                    DVector::new_random(10000),
                    DVector::new_random(10000),
                    rng.random::<f64>(),
                )
            },
            |(mut a, b, n)| {
                a.axpy(n, &b, 1.0);
                (a, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn vec10000_axpy_beta_f64(bh: &mut criterion::Criterion) {
    use rand::SeedableRng;

    let mut rng = IsaacRng::seed_from_u64(0);

    bh.bench_function("vec10000_axpy_beta_f64", |bh| {
        bh.iter_batched(
            || {
                (
                    DVector::new_random(10000),
                    DVector::new_random(10000),
                    rng.random::<f64>(),
                    rng.random::<f64>(),
                )
            },
            |(mut a, b, n, beta)| {
                a.axpy(n, &b, beta);
                (a, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn vec10000_axpy_f64_slice(bh: &mut criterion::Criterion) {
    use rand::SeedableRng;

    let mut rng = IsaacRng::seed_from_u64(0);

    bh.bench_function("vec10000_axpy_f64_slice", |bh| {
        bh.iter_batched(
            || {
                (
                    DVector::new_random(10000),
                    DVector::new_random(10000),
                    rng.random::<f64>(),
                )
            },
            |(mut a, b, n)| {
                let mut a_slice = a.fixed_rows_mut::<10000>(0);
                let b_slice = b.fixed_rows::<10000>(0);
                a_slice.axpy(n, &b_slice, 1.0);
                (a, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn vec10000_axpy_f64_static(bh: &mut criterion::Criterion) {
    use rand::SeedableRng;

    let mut rng = IsaacRng::seed_from_u64(0);

    // NOTE: for some reasons, it is much faster if the argument are boxed (Box::new(OVector...)).
    bh.bench_function("vec10000_axpy_f64_static", |bh| {
        bh.iter_batched(
            || {
                (
                    SVector::<f64, 10000>::new_random(),
                    SVector::<f64, 10000>::new_random(),
                    rng.random::<f64>(),
                )
            },
            |(mut a, b, n)| {
                a.axpy(n, &b, 1.0);
                (a, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn vec10000_axpy_f32(bh: &mut criterion::Criterion) {
    use rand::SeedableRng;

    let mut rng = IsaacRng::seed_from_u64(0);

    bh.bench_function("vec10000_axpy_f32", |bh| {
        bh.iter_batched(
            || {
                (
                    DVector::new_random(10000),
                    DVector::new_random(10000),
                    rng.random::<f32>(),
                )
            },
            |(mut a, b, n)| {
                a.axpy(n, &b, 1.0);
                (a, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn vec10000_axpy_beta_f32(bh: &mut criterion::Criterion) {
    use rand::SeedableRng;

    let mut rng = IsaacRng::seed_from_u64(0);

    bh.bench_function("vec10000_axpy_beta_f32", |bh| {
        bh.iter_batched(
            || {
                (
                    DVector::new_random(10000),
                    DVector::new_random(10000),
                    rng.random::<f32>(),
                    rng.random::<f32>(),
                )
            },
            |(mut a, b, n, beta)| {
                a.axpy(n, &b, beta);
                (a, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    vector,
    vec2_add_v_f32,
    vec3_add_v_f32,
    vec4_add_v_f32,
    vec2_add_v_f64,
    vec3_add_v_f64,
    vec4_add_v_f64,
    vec2_sub_v,
    vec3_sub_v,
    vec4_sub_v,
    vec2_mul_s,
    vec3_mul_s,
    vec4_mul_s,
    vec2_div_s,
    vec3_div_s,
    vec4_div_s,
    vec2_dot_f32,
    vec3_dot_f32,
    vec4_dot_f32,
    vec2_dot_f64,
    vec3_dot_f64,
    vec4_dot_f64,
    vec3_cross,
    vec2_norm,
    vec3_norm,
    vec4_norm,
    vec2_normalize,
    vec3_normalize,
    vec4_normalize,
    vec10000_dot_f64,
    vec10000_dot_f32,
    vec10000_axpy_f64,
    vec10000_axpy_beta_f64,
    vec10000_axpy_f64_slice,
    vec10000_axpy_f64_static,
    vec10000_axpy_f32,
    vec10000_axpy_beta_f32
);
