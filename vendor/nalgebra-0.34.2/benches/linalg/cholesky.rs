use na::{Cholesky, DMatrix, DVector};

fn random_positive_definite_dmatrix(nrows: usize, ncols: usize) -> DMatrix<f64> {
    // @note(geo-ant) to get positive definite matrices we use M*M^T + alpha*I,
    // where alpha is a constant that is chosen so that the eigenvales stay
    // positive.
    let m = DMatrix::<f64>::new_random(nrows, ncols);
    let alpha = f64::EPSILON.sqrt() * m.norm_squared();
    let nrows = m.nrows();
    &m * m.transpose() + alpha * DMatrix::identity(nrows, nrows)
}

fn cholesky_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_100x100", |bh| {
        bh.iter_batched(
            || random_positive_definite_dmatrix(100, 100),
            |m| Cholesky::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cholesky_500x500(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_500x500", |bh| {
        bh.iter_batched(
            || random_positive_definite_dmatrix(500, 500),
            |m| Cholesky::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

// With unpack.
fn cholesky_decompose_unpack_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_decompose_unpack_100x100", |bh| {
        bh.iter_batched(
            || random_positive_definite_dmatrix(100, 100),
            |m| {
                let chol = Cholesky::new(m).unwrap();
                chol.unpack()
            },
            criterion::BatchSize::SmallInput,
        )
    });
}
fn cholesky_decompose_unpack_500x500(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_decompose_unpack_500x500", |bh| {
        bh.iter_batched(
            || random_positive_definite_dmatrix(500, 500),
            |m| {
                let chol = Cholesky::new(m).unwrap();
                chol.unpack()
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cholesky_solve_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_solve_10x10", |bh| {
        bh.iter_batched_ref(
            || {
                let m = random_positive_definite_dmatrix(10, 10);
                let v = DVector::<f64>::new_random(10);
                let chol = Cholesky::new(m).unwrap();
                (chol, v)
            },
            |(chol, v)| chol.solve(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cholesky_solve_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_solve_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                let m = random_positive_definite_dmatrix(100, 100);
                let v = DVector::<f64>::new_random(100);
                let chol = Cholesky::new(m).unwrap();
                (chol, v)
            },
            |(chol, v)| chol.solve(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cholesky_solve_500x500(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_solve_500x500", |bh| {
        bh.iter_batched_ref(
            || {
                let m = random_positive_definite_dmatrix(500, 500);
                let v = DVector::<f64>::new_random(500);
                let chol = Cholesky::new(m).unwrap();
                (chol, v)
            },
            |(chol, v)| chol.solve(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cholesky_inverse_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_inverse_10x10", |bh| {
        bh.iter_batched_ref(
            || {
                let m = random_positive_definite_dmatrix(10, 10);
                Cholesky::new(m).unwrap()
            },
            |chol| chol.inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cholesky_inverse_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_inverse_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                let m = random_positive_definite_dmatrix(100, 100);
                Cholesky::new(m).unwrap()
            },
            |chol| chol.inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn cholesky_inverse_500x500(bh: &mut criterion::Criterion) {
    bh.bench_function("cholesky_inverse_500x500", |bh| {
        bh.iter_batched_ref(
            || {
                let m = random_positive_definite_dmatrix(500, 500);
                Cholesky::new(m).unwrap()
            },
            |chol| chol.inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    cholesky,
    cholesky_100x100,
    cholesky_500x500,
    cholesky_decompose_unpack_100x100,
    cholesky_decompose_unpack_500x500,
    cholesky_solve_10x10,
    cholesky_solve_100x100,
    cholesky_solve_500x500,
    cholesky_inverse_10x10,
    cholesky_inverse_100x100,
    cholesky_inverse_500x500
);
