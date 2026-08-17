use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, BatchSize,
    BenchmarkGroup, Criterion,
};
use fastrand::Rng;
use float_next_after::NextAfter;

fn get_rng() -> Rng {
    Rng::with_seed(0)
}

fn run<M: Measurement, T: NextAfter + Copy>(
    mut group: BenchmarkGroup<M>,
    get: impl Fn(&Rng) -> T,
    up: T,
    down: T,
) {
    let mut run_one_way = |name, to| {
        let rng = get_rng();
        group.bench_function(name, |b| {
            b.iter_batched(
                || get(&rng),
                |f| black_box(f).next_after(to),
                BatchSize::SmallInput,
            )
        });
    };

    run_one_way("up", up);
    run_one_way("down", down);
}

fn f32(c: &mut Criterion) {
    let group = c.benchmark_group("f32");
    let get_f32 = |rng: &Rng| f32::from_bits(rng.u32(0..=u32::MAX));

    run(group, get_f32, f32::INFINITY, f32::NEG_INFINITY);
}

fn f64(c: &mut Criterion) {
    let group = c.benchmark_group("f64");
    let get_f64 = |rng: &Rng| f64::from_bits(rng.u64(0..=u64::MAX));

    run(group, get_f64, f64::INFINITY, f64::NEG_INFINITY);
}

criterion_group!(benches, f32, f64);
criterion_main!(benches);
