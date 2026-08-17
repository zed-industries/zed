use na::{Bidiagonal, DMatrix, Matrix4};

#[path = "../common/macros.rs"]
mod macros;

// Without unpack.
fn bidiagonalize_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("bidiagonalize_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| Bidiagonal::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bidiagonalize_100x500(bh: &mut criterion::Criterion) {
    bh.bench_function("bidiagonalize_100x500", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 500),
            |m| Bidiagonal::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bidiagonalize_4x4(bh: &mut criterion::Criterion) {
    bh.bench_function("bidiagonalize_4x4", |bh| {
        bh.iter_batched(
            || Matrix4::<f64>::new_random(),
            |m| Bidiagonal::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bidiagonalize_500x100(bh: &mut criterion::Criterion) {
    bh.bench_function("bidiagonalize_500x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(500, 100),
            |m| Bidiagonal::new(m),
            criterion::BatchSize::SmallInput,
        )
    });
}

// With unpack.
fn bidiagonalize_unpack_100x100(bh: &mut criterion::Criterion) {
    bh.bench_function("bidiagonalize_unpack_100x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 100),
            |m| {
                let bidiag = Bidiagonal::new(m);
                bidiag.unpack()
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bidiagonalize_unpack_100x500(bh: &mut criterion::Criterion) {
    bh.bench_function("bidiagonalize_unpack_100x500", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(100, 500),
            |m| {
                let bidiag = Bidiagonal::new(m);
                bidiag.unpack()
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bidiagonalize_unpack_500x100(bh: &mut criterion::Criterion) {
    bh.bench_function("bidiagonalize_unpack_500x100", |bh| {
        bh.iter_batched(
            || DMatrix::<f64>::new_random(500, 100),
            |m| {
                let bidiag = Bidiagonal::new(m);
                bidiag.unpack()
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    bidiagonal,
    bidiagonalize_100x100,
    bidiagonalize_100x500,
    bidiagonalize_4x4,
    bidiagonalize_500x100,
    bidiagonalize_unpack_100x100,
    bidiagonalize_unpack_100x500,
    bidiagonalize_unpack_500x100,
);
