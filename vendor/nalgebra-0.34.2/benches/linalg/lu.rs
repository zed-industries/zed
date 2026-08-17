use na::{DMatrix, DVector, LU};

// Without unpack.
fn lu_decompose_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("lu_decompose_10x10", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(10, 10),
            |m| LU::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn lu_decompose_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("lu_decompose_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| LU::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn lu_solve_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("lu_solve_10x10", |bh| {
        bh.iter_batched(
            || {
                let m = DMatrix::<f64>::new_random(10, 10);
                (LU::new(m), DVector::<f64>::new_random(10))
            },
            |(lu, mut b)| {
                lu.solve_mut(&mut b);
                (lu, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn lu_solve_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("lu_solve_100x100", |bh| {
        bh.iter_batched(
            || {
                let m = DMatrix::<f64>::new_random(100, 100);
                (LU::new(m), DVector::<f64>::new_random(100))
            },
            |(lu, mut b)| {
                lu.solve_mut(&mut b);
                (lu, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn lu_inverse_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("lu_inverse_10x10", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(10, 10);
                LU::new(m)
            },
            |lu| lu.try_inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn lu_inverse_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("lu_inverse_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(100, 100);
                LU::new(m)
            },
            |lu| lu.try_inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn lu_determinant_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("lu_determinant_10x10", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(10, 10);
                LU::new(m)
            },
            |lu| lu.determinant(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn lu_determinant_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("lu_determinant_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(100, 100);
                LU::new(m)
            },
            |lu| lu.determinant(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    lu,
    lu_decompose_10x10,
    lu_decompose_100x100,
    lu_solve_10x10,
    lu_solve_100x100,
    lu_inverse_10x10,
    lu_inverse_100x100,
    lu_determinant_10x10,
    lu_determinant_100x100,
);
