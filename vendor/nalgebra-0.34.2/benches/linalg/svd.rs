use na::{DMatrix, SMatrix, SVD};

fn svd_decompose_2x2_f32(bh: &mut criterion::Criterion) {
    bh.bench_function("svd_decompose_2x2", |bh| {
        bh.iter_batched(
            || SMatrix::<f32, 2, 2>::new_random(),
            |m| SVD::new_unordered(m, true, true),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn svd_decompose_3x3_f32(bh: &mut criterion::Criterion) {
    bh.bench_function("svd_decompose_3x3", |bh| {
        bh.iter_batched(
            || SMatrix::<f32, 3, 3>::new_random(),
            |m| SVD::new_unordered(m, true, true),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn svd_decompose_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("svd_decompose_4x4", |bh| {
        bh.iter_batched(
            || SMatrix::<f64, 4, 4>::new_random(),
            |m| SVD::new_unordered(m, true, true),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn svd_decompose_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("svd_decompose_10x10", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(10, 10),
            |m| SVD::new_unordered(m, true, true),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn svd_decompose_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("svd_decompose_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| SVD::new_unordered(m, true, true),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn svd_decompose_200x200(bh: &mut criterion::Criterion) {
    bh.bench_function("svd_decompose_200x200", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(200, 200),
            |m| SVD::new_unordered(m, true, true),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn rank_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("rank_4x4", |bh| {
        bh.iter_batched_ref(
            || SMatrix::<f64, 4, 4>::new_random(),
            |m| m.rank(1.0e-10),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn rank_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("rank_10x10", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(10, 10),
            |m| m.rank(1.0e-10),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn rank_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("rank_100x100", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(100, 100),
            |m| m.rank(1.0e-10),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn rank_200x200(bh: &mut criterion::Criterion) {
    bh.bench_function("rank_200x200", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(200, 200),
            |m| m.rank(1.0e-10),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn singular_values_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("singular_values_4x4", |bh| {
        bh.iter_batched_ref(
            || SMatrix::<f64, 4, 4>::new_random(),
            |m| m.singular_values(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn singular_values_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("singular_values_10x10", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(10, 10),
            |m| m.singular_values(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn singular_values_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("singular_values_100x100", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(100, 100),
            |m| m.singular_values(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn singular_values_200x200(bh: &mut criterion::Criterion) {
    bh.bench_function("singular_values_200x200", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(200, 200),
            |m| m.singular_values(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn pseudo_inverse_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("pseudo_inverse_4x4", |bh| {
        bh.iter_batched(
            || SMatrix::<f64, 4, 4>::new_random(),
            |m| m.pseudo_inverse(1.0e-10),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn pseudo_inverse_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("pseudo_inverse_10x10", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(10, 10),
            |m| m.pseudo_inverse(1.0e-10),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn pseudo_inverse_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("pseudo_inverse_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| m.pseudo_inverse(1.0e-10),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn pseudo_inverse_200x200(bh: &mut criterion::Criterion) {
    bh.bench_function("pseudo_inverse_200x200", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(200, 200),
            |m| m.pseudo_inverse(1.0e-10),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    svd,
    svd_decompose_2x2_f32,
    svd_decompose_3x3_f32,
    svd_decompose_4x4,
    svd_decompose_10x10,
    svd_decompose_100x100,
    svd_decompose_200x200,
    rank_4x4,
    rank_10x10,
    rank_100x100,
    rank_200x200,
    singular_values_4x4,
    singular_values_10x10,
    singular_values_100x100,
    singular_values_200x200,
    pseudo_inverse_4x4,
    pseudo_inverse_10x10,
    pseudo_inverse_100x100,
    pseudo_inverse_200x200
);
