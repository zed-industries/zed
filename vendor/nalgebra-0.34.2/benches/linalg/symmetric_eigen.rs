use na::{DMatrix, SMatrix, SymmetricEigen};

fn symmetric_eigen_decompose_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("symmetric_eigen_decompose_4x4", |bh| {
        bh.iter_batched(
            || SMatrix::<f64, 4, 4>::new_random(),
            |m| SymmetricEigen::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn symmetric_eigen_decompose_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("symmetric_eigen_decompose_10x10", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(10, 10),
            |m| SymmetricEigen::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn symmetric_eigen_decompose_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("symmetric_eigen_decompose_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| SymmetricEigen::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn symmetric_eigen_decompose_200x200(bh: &mut criterion::Criterion) {
    bh.bench_function("symmetric_eigen_decompose_200x200", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(200, 200),
            |m| SymmetricEigen::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    symmetric_eigen,
    symmetric_eigen_decompose_4x4,
    symmetric_eigen_decompose_10x10,
    symmetric_eigen_decompose_100x100,
    symmetric_eigen_decompose_200x200
);
