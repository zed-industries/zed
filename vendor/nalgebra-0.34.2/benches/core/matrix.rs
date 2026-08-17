use na::{DMatrix, DVector, Matrix2, Matrix3, Matrix4, OMatrix, U10, Vector2, Vector3, Vector4};
use rand::Rng;
use rand_isaac::IsaacRng;
use std::hint::black_box;
use std::ops::{Add, Div, Mul, Sub};

#[path = "../common/macros.rs"]
mod macros;

bench_binop!(mat2_mul_m, Matrix2<f32>, Matrix2<f32>, mul);
bench_binop!(mat3_mul_m, Matrix3<f32>, Matrix3<f32>, mul);
bench_binop!(mat4_mul_m, Matrix4<f32>, Matrix4<f32>, mul);

bench_binop_ref!(mat2_tr_mul_m, Matrix2<f32>, Matrix2<f32>, tr_mul);
bench_binop_ref!(mat3_tr_mul_m, Matrix3<f32>, Matrix3<f32>, tr_mul);
bench_binop_ref!(mat4_tr_mul_m, Matrix4<f32>, Matrix4<f32>, tr_mul);

bench_binop!(mat2_add_m, Matrix2<f32>, Matrix2<f32>, add);
bench_binop!(mat3_add_m, Matrix3<f32>, Matrix3<f32>, add);
bench_binop!(mat4_add_m, Matrix4<f32>, Matrix4<f32>, add);

bench_binop!(mat2_sub_m, Matrix2<f32>, Matrix2<f32>, sub);
bench_binop!(mat3_sub_m, Matrix3<f32>, Matrix3<f32>, sub);
bench_binop!(mat4_sub_m, Matrix4<f32>, Matrix4<f32>, sub);

bench_binop!(mat2_mul_v, Matrix2<f32>, Vector2<f32>, mul);
bench_binop!(mat3_mul_v, Matrix3<f32>, Vector3<f32>, mul);
bench_binop!(mat4_mul_v, Matrix4<f32>, Vector4<f32>, mul);

bench_binop_single_1st!(single_mat2_mul_v, Matrix2<f32>, Vector2<f32>, mul);
bench_binop_single_1st!(single_mat3_mul_v, Matrix3<f32>, Vector3<f32>, mul);
bench_binop_single_1st!(single_mat4_mul_v, Matrix4<f32>, Vector4<f32>, mul);

bench_binop_ref!(mat2_tr_mul_v, Matrix2<f32>, Vector2<f32>, tr_mul);
bench_binop_ref!(mat3_tr_mul_v, Matrix3<f32>, Vector3<f32>, tr_mul);
bench_binop_ref!(mat4_tr_mul_v, Matrix4<f32>, Vector4<f32>, tr_mul);

bench_binop_single_1st_ref!(single_mat2_tr_mul_v, Matrix2<f32>, Vector2<f32>, tr_mul);
bench_binop_single_1st_ref!(single_mat3_tr_mul_v, Matrix3<f32>, Vector3<f32>, tr_mul);
bench_binop_single_1st_ref!(single_mat4_tr_mul_v, Matrix4<f32>, Vector4<f32>, tr_mul);

bench_binop!(mat2_mul_s, Matrix2<f32>, f32, mul);
bench_binop!(mat3_mul_s, Matrix3<f32>, f32, mul);
bench_binop!(mat4_mul_s, Matrix4<f32>, f32, mul);

bench_binop!(mat2_div_s, Matrix2<f32>, f32, div);
bench_binop!(mat3_div_s, Matrix3<f32>, f32, div);
bench_binop!(mat4_div_s, Matrix4<f32>, f32, div);

bench_unop!(mat2_inv, Matrix2<f32>, try_inverse);
bench_unop!(mat3_inv, Matrix3<f32>, try_inverse);
bench_unop!(mat4_inv, Matrix4<f32>, try_inverse);

bench_unop!(mat2_transpose, Matrix2<f32>, transpose);
bench_unop!(mat3_transpose, Matrix3<f32>, transpose);
bench_unop!(mat4_transpose, Matrix4<f32>, transpose);

fn mat_div_scalar(b: &mut criterion::Criterion) {
    b.bench_function("mat_div_scalar", |bh| {
        bh.iter_batched(
            || {
                (
                    DMatrix::from_row_slice(1000, 1000, &vec![2.0; 1000000]),
                    42.0,
                )
            },
            |(mut a, n)| {
                let mut b = a.view_mut((0, 0), (1000, 1000));
                b /= n;
                a
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat100_add_mat100(bench: &mut criterion::Criterion) {
    bench.bench_function("mat100_add_mat100", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(100, 100),
                    DMatrix::<f64>::new_random(100, 100),
                )
            },
            |args| (&args.0) + (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat4_mul_mat4(bench: &mut criterion::Criterion) {
    bench.bench_function("mat4_mul_mat4", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(4, 4),
                    DMatrix::<f64>::new_random(4, 4),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat5_mul_mat5(bench: &mut criterion::Criterion) {
    bench.bench_function("mat5_mul_mat5", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(5, 5),
                    DMatrix::<f64>::new_random(5, 5),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat6_mul_mat6(bench: &mut criterion::Criterion) {
    bench.bench_function("mat6_mul_mat6", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(6, 6),
                    DMatrix::<f64>::new_random(6, 6),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat7_mul_mat7(bench: &mut criterion::Criterion) {
    bench.bench_function("mat7_mul_mat7", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(7, 7),
                    DMatrix::<f64>::new_random(7, 7),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat8_mul_mat8(bench: &mut criterion::Criterion) {
    bench.bench_function("mat8_mul_mat8", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(8, 8),
                    DMatrix::<f64>::new_random(8, 8),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat9_mul_mat9(bench: &mut criterion::Criterion) {
    bench.bench_function("mat9_mul_mat9", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(9, 9),
                    DMatrix::<f64>::new_random(9, 9),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat10_mul_mat10(bench: &mut criterion::Criterion) {
    bench.bench_function("mat10_mul_mat10", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(10, 10),
                    DMatrix::<f64>::new_random(10, 10),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat10_mul_mat10_static(bench: &mut criterion::Criterion) {
    bench.bench_function("mat10_mul_mat10_static", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    OMatrix::<f64, U10, U10>::new_random(),
                    OMatrix::<f64, U10, U10>::new_random(),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat100_mul_mat100(bench: &mut criterion::Criterion) {
    bench.bench_function("mat100_mul_mat100", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(100, 100),
                    DMatrix::<f64>::new_random(100, 100),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat500_mul_mat500(bench: &mut criterion::Criterion) {
    bench.bench_function("mat500_mul_mat500", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(500, 500),
                    DMatrix::<f64>::new_random(500, 500),
                )
            },
            |args| (&args.0) * (&args.1),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn iter(bench: &mut criterion::Criterion) {
    bench.bench_function("iter", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(1000, 1000),
            |a| {
                for value in a.iter() {
                    black_box(value);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn iter_rev(bench: &mut criterion::Criterion) {
    bench.bench_function("iter_rev", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(1000, 1000),
            |a| {
                for value in a.iter().rev() {
                    black_box(value);
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn copy_from(bench: &mut criterion::Criterion) {
    bench.bench_function("copy_from", |bh| {
        bh.iter_batched(
            || {
                (
                    DMatrix::<f64>::new_random(1000, 1000),
                    DMatrix::<f64>::new_random(1000, 1000),
                )
            },
            |(a, mut b)| {
                b.copy_from(&a);
                (a, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn axpy(bench: &mut criterion::Criterion) {
    bench.bench_function("axpy", |bh| {
        bh.iter_batched(
            || {
                (
                    DVector::<f64>::new_random(100000),
                    DVector::<f64>::new_random(100000),
                    rand::random(),
                )
            },
            |(x, mut y, a)| {
                y.axpy(a, &x, 1.0);
                (x, y)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn tr_mul_to(bench: &mut criterion::Criterion) {
    bench.bench_function("tr_mul_to", |bh| {
        bh.iter_batched(
            || {
                (
                    DMatrix::<f64>::new_random(1000, 1000),
                    DVector::<f64>::new_random(1000),
                    DVector::from_element(1000, 0.0),
                )
            },
            |(a, b, mut c)| {
                a.tr_mul_to(&b, &mut c);
                (a, b, c)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat_mul_mat_to(bench: &mut criterion::Criterion) {
    bench.bench_function("mat_mul_mat_to", |bh| {
        bh.iter_batched(
            || {
                (
                    DMatrix::<f64>::new_random(100, 100),
                    DMatrix::<f64>::new_random(100, 100),
                    DMatrix::<f64>::from_element(100, 100, 0.0),
                )
            },
            |(a, b, mut ab)| {
                a.mul_to(&b, &mut ab);
                (a, b, ab)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn mat100_from_fn(bench: &mut criterion::Criterion) {
    bench.bench_function("mat100_from_fn", move |bh| {
        bh.iter(|| {
            DMatrix::from_fn(black_box(100), black_box(100), |a, b| {
                black_box(a) + black_box(b)
            })
        })
    });
}

fn mat500_from_fn(bench: &mut criterion::Criterion) {
    bench.bench_function("mat500_from_fn", move |bh| {
        bh.iter(|| {
            DMatrix::from_fn(black_box(500), black_box(500), |a, b| {
                black_box(a) + black_box(b)
            })
        })
    });
}

criterion_group!(
    matrix,
    mat2_mul_m,
    mat3_mul_m,
    mat4_mul_m,
    mat2_tr_mul_m,
    mat3_tr_mul_m,
    mat4_tr_mul_m,
    mat2_add_m,
    mat3_add_m,
    mat4_add_m,
    mat2_sub_m,
    mat3_sub_m,
    mat4_sub_m,
    mat2_mul_v,
    mat3_mul_v,
    mat4_mul_v,
    single_mat2_mul_v,
    single_mat3_mul_v,
    single_mat4_mul_v,
    mat2_tr_mul_v,
    mat3_tr_mul_v,
    mat4_tr_mul_v,
    single_mat2_tr_mul_v,
    single_mat3_tr_mul_v,
    single_mat4_tr_mul_v,
    mat2_mul_s,
    mat3_mul_s,
    mat4_mul_s,
    mat2_div_s,
    mat3_div_s,
    mat4_div_s,
    mat2_inv,
    mat3_inv,
    mat4_inv,
    mat2_transpose,
    mat3_transpose,
    mat4_transpose,
    mat_div_scalar,
    mat100_add_mat100,
    mat4_mul_mat4,
    mat5_mul_mat5,
    mat6_mul_mat6,
    mat7_mul_mat7,
    mat8_mul_mat8,
    mat9_mul_mat9,
    mat10_mul_mat10,
    mat10_mul_mat10_static,
    mat100_mul_mat100,
    mat500_mul_mat500,
    iter,
    iter_rev,
    copy_from,
    axpy,
    tr_mul_to,
    mat_mul_mat_to,
    mat100_from_fn,
    mat500_from_fn,
);
