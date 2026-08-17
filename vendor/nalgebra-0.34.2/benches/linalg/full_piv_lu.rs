use na::{DMatrix, DVector, FullPivLU};

// Without unpack.
fn full_piv_lu_decompose_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("full_piv_lu_decompose_10x10", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(10, 10),
            |m| FullPivLU::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn full_piv_lu_decompose_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("full_piv_lu_decompose_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| FullPivLU::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn full_piv_lu_solve_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("full_piv_lu_solve_10x10", |bh| {
        bh.iter_batched(
            || {
                let m = DMatrix::<f64>::new_random(10, 10);
                (FullPivLU::new(m), DVector::<f64>::new_random(10))
            },
            |(lu, mut b)| {
                lu.solve_mut(&mut b);
                (lu, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn full_piv_lu_solve_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("full_piv_lu_solve_100x100", |bh| {
        bh.iter_batched(
            || {
                let m = DMatrix::<f64>::new_random(100, 100);
                (FullPivLU::new(m), DVector::<f64>::new_random(100))
            },
            |(lu, mut b)| {
                lu.solve_mut(&mut b);
                (lu, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn full_piv_lu_inverse_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("full_piv_lu_inverse_10x10", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(10, 10);
                FullPivLU::new(m)
            },
            |lu| lu.try_inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn full_piv_lu_inverse_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("full_piv_lu_inverse_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(100, 100);
                FullPivLU::new(m)
            },
            |lu| lu.try_inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn full_piv_lu_determinant_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("full_piv_lu_determinant_10x10", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(10, 10);
                FullPivLU::new(m)
            },
            |lu| lu.determinant(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn full_piv_lu_determinant_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("full_piv_lu_determinant_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(100, 100);
                FullPivLU::new(m)
            },
            |lu| lu.determinant(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    full_piv_lu,
    full_piv_lu_decompose_10x10,
    full_piv_lu_decompose_100x100,
    full_piv_lu_solve_10x10,
    full_piv_lu_solve_100x100,
    full_piv_lu_inverse_10x10,
    full_piv_lu_inverse_100x100,
    full_piv_lu_determinant_10x10,
    full_piv_lu_determinant_100x100,
);
