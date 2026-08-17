use na::{DMatrix, DVector};

fn solve_l_triangular_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("solve_l_triangular_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(100, 100),
                    DVector::<f64>::new_random(100),
                )
            },
            |(m, v)| m.solve_lower_triangular(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn solve_l_triangular_1000x1000(bh: &mut criterion::Criterion) {
    bh.bench_function("solve_l_triangular_1000x1000", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(1000, 1000),
                    DVector::<f64>::new_random(1000),
                )
            },
            |(m, v)| m.solve_lower_triangular(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn tr_solve_l_triangular_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("tr_solve_l_triangular_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(100, 100),
                    DVector::<f64>::new_random(100),
                )
            },
            |(m, v)| m.tr_solve_lower_triangular(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn tr_solve_l_triangular_1000x1000(bh: &mut criterion::Criterion) {
    bh.bench_function("tr_solve_l_triangular_1000x1000", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(1000, 1000),
                    DVector::<f64>::new_random(1000),
                )
            },
            |(m, v)| m.tr_solve_lower_triangular(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn solve_u_triangular_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("solve_u_triangular_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(100, 100),
                    DVector::<f64>::new_random(100),
                )
            },
            |(m, v)| m.solve_upper_triangular(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn solve_u_triangular_1000x1000(bh: &mut criterion::Criterion) {
    bh.bench_function("solve_u_triangular_1000x1000", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(1000, 1000),
                    DVector::<f64>::new_random(1000),
                )
            },
            |(m, v)| m.solve_upper_triangular(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn tr_solve_u_triangular_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("tr_solve_u_triangular_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(100, 100),
                    DVector::<f64>::new_random(100),
                )
            },
            |(m, v)| m.tr_solve_upper_triangular(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn tr_solve_u_triangular_1000x1000(bh: &mut criterion::Criterion) {
    bh.bench_function("tr_solve_u_triangular_1000x1000", |bh| {
        bh.iter_batched_ref(
            || {
                (
                    DMatrix::<f64>::new_random(1000, 1000),
                    DVector::<f64>::new_random(1000),
                )
            },
            |(m, v)| m.tr_solve_upper_triangular(v),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    solve,
    solve_l_triangular_100x100,
    solve_l_triangular_1000x1000,
    tr_solve_l_triangular_100x100,
    tr_solve_l_triangular_1000x1000,
    solve_u_triangular_100x100,
    solve_u_triangular_1000x1000,
    tr_solve_u_triangular_100x100,
    tr_solve_u_triangular_1000x1000
);
