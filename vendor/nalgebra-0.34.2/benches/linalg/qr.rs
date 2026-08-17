use na::{DMatrix, DVector, Matrix4, QR};

#[path = "../common/macros.rs"]
mod macros;

// Without unpack.
fn qr_decompose_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_decompose_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| QR::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_decompose_100x500(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_decompose_100x500", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 500),
            |m| QR::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_decompose_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_decompose_4x4", |bh| {
        bh.iter_batched(
            || Matrix4::<f64>::new_random(),
            |m| QR::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_decompose_500x100(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_decompose_500x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(500, 100),
            |m| QR::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

// With unpack.
fn qr_decompose_unpack_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_decompose_unpack_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| {
                let qr = QR::new(m);
                qr.unpack()
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_decompose_unpack_100x500(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_decompose_unpack_100x500", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 500),
            |m| {
                let qr = QR::new(m);
                qr.unpack()
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_decompose_unpack_500x100(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_decompose_unpack_500x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(500, 100),
            |m| {
                let qr = QR::new(m);
                qr.unpack()
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_solve_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_solve_10x10", |bh| {
        bh.iter_batched(
            || {
                let m = DMatrix::<f64>::new_random(10, 10);
                (QR::new(m), DVector::<f64>::new_random(10))
            },
            |(qr, mut b)| {
                qr.solve_mut(&mut b);
                (qr, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_solve_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_solve_100x100", |bh| {
        bh.iter_batched(
            || {
                let m = DMatrix::<f64>::new_random(100, 100);
                (QR::new(m), DVector::<f64>::new_random(100))
            },
            |(qr, mut b)| {
                qr.solve_mut(&mut b);
                (qr, b)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_inverse_10x10(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_inverse_10x10", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(10, 10);
                QR::new(m)
            },
            |qr| qr.try_inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn qr_inverse_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("qr_inverse_100x100", |bh| {
        bh.iter_batched_ref(
            || {
                let m = DMatrix::<f64>::new_random(100, 100);
                QR::new(m)
            },
            |qr| qr.try_inverse(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    qr,
    qr_decompose_100x100,
    qr_decompose_100x500,
    qr_decompose_4x4,
    qr_decompose_500x100,
    qr_decompose_unpack_100x100,
    qr_decompose_unpack_100x500,
    qr_decompose_unpack_500x100,
    qr_solve_10x10,
    qr_solve_100x100,
    qr_inverse_10x10,
    qr_inverse_100x100,
);
