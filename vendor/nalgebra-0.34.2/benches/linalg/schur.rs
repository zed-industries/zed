use na::{DMatrix, SMatrix, Schur};

fn schur_decompose_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("schur_decompose_4x4", |bh| {
        bh.iter_batched(
            || SMatrix::<f64, 4, 4>::new_random(),
            |m| Schur::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn schur_decompose_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("schur_decompose_10x10", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(10, 10),
            |m| Schur::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn schur_decompose_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("schur_decompose_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| Schur::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn schur_decompose_200x200(bh: &mut criterion::Criterion) {
    bh.bench_function("schur_decompose_200x200", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(200, 200),
            |m| Schur::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn eigenvalues_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("eigenvalues_4x4", |bh| {
        bh.iter_batched_ref(
            || SMatrix::<f64, 4, 4>::new_random(),
            |m| m.complex_eigenvalues(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn eigenvalues_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("eigenvalues_10x10", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(10, 10),
            |m| m.complex_eigenvalues(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn eigenvalues_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("eigenvalues_100x100", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(100, 100),
            |m| m.complex_eigenvalues(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn eigenvalues_200x200(bh: &mut criterion::Criterion) {
    bh.bench_function("eigenvalues_200x200", |bh| {
        bh.iter_batched_ref(
            || DMatrix::<f64>::new_random(200, 200),
            |m| m.complex_eigenvalues(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    schur,
    schur_decompose_4x4,
    schur_decompose_10x10,
    schur_decompose_100x100,
    schur_decompose_200x200,
    eigenvalues_4x4,
    eigenvalues_10x10,
    eigenvalues_100x100,
    eigenvalues_200x200
);
