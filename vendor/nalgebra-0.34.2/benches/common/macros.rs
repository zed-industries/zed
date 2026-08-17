#![macro_use]

/// This will call `Drop::drop()` within the benchmarking loop for all arguments that were not consumed
/// by the binary operation.
///
/// Do not use this macro for types with non-trivial `Drop` implementation unless you want to include it
/// into the measurement.
macro_rules! bench_binop(
    ($name: ident, $t1: ty, $t2: ty, $binop: ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            use rand::SeedableRng;

            let mut rng = IsaacRng::seed_from_u64(0);

            bh.bench_function(stringify!($name), |bh| bh.iter_batched(
                || (rng.random::<$t1>(), rng.random::<$t2>()),
                |args| {
                    args.0.$binop(args.1)
                },
                criterion::BatchSize::SmallInput),
            );
        }
    }
);

macro_rules! bench_binop_ref(
    ($name: ident, $t1: ty, $t2: ty, $binop: ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            use rand::SeedableRng;

            let mut rng = IsaacRng::seed_from_u64(0);

            bh.bench_function(stringify!($name), |bh| bh.iter_batched_ref(
                || (rng.random::<$t1>(), rng.random::<$t2>()),
                |args| {
                    args.0.$binop(&args.1)
                },
                criterion::BatchSize::SmallInput),
            );
        }
    }
);

/// This will call `Drop::drop()` within the benchmarking loop for all arguments that were not consumed
/// by the binary operation.
///
/// Do not use this macro for types with non-trivial `Drop` implementation unless you want to include it
/// into the measurement.
macro_rules! bench_binop_single_1st(
    ($name: ident, $t1: ty, $t2: ty, $binop: ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            use rand::SeedableRng;
            use std::hint::black_box;

            let mut rng = IsaacRng::seed_from_u64(0);

            let first = black_box(rng.random::<$t1>());

            bh.bench_function(stringify!($name), |bh| bh.iter_batched(
                || rng.random::<$t2>(),
                |second| {
                    first.$binop(second)
                },
                criterion::BatchSize::SmallInput),
            );
        }
    }
);

macro_rules! bench_binop_single_1st_ref(
    ($name: ident, $t1: ty, $t2: ty, $binop: ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            use rand::SeedableRng;
            use std::hint::black_box;

            let mut rng = IsaacRng::seed_from_u64(0);

            let first = black_box(rng.random::<$t1>());

            bh.bench_function(stringify!($name), |bh| bh.iter_batched_ref(
                || rng.random::<$t2>(),
                |second| {
                    first.$binop(second)
                },
                criterion::BatchSize::SmallInput),
            );
        }
    }
);

macro_rules! bench_unop(
    ($name: ident, $t: ty, $unop: ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            use rand::SeedableRng;

            let mut rng = IsaacRng::seed_from_u64(0);

            bh.bench_function(stringify!($name), |bh| bh.iter_batched_ref(
                || rng.random::<$t>(),
                |arg| {
                    arg.$unop()
                },
                criterion::BatchSize::SmallInput),
            );
        }
    }
);
