use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use editor::MultiBuffer;
use gpui::TestDispatcher;
use itertools::Itertools;
use multi_buffer::MultiBufferOffset;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::num::NonZeroU32;
use text::Bias;
use util::RandomCharIter;

fn to_tab_point_benchmark(c: &mut Criterion) {
    let dispatcher = TestDispatcher::new(1);
    let cx = gpui::TestAppContext::build(dispatcher, None);

    let create_tab_map = |length: usize| {
        let mut rng = StdRng::seed_from_u64(1);
        let text = RandomCharIter::new(&mut rng)
            .take(length)
            .collect::<String>();
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

        let buffer_snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
        use editor::display_map::*;
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot.clone());
        let fold_point = fold_snapshot.to_fold_point(
            inlay_snapshot.to_point(InlayOffset(
                rng.random_range(MultiBufferOffset(0)..MultiBufferOffset(length)),
            )),
            Bias::Left,
        );
        let (_, snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(4).unwrap());

        (length, snapshot, fold_point)
    };

    let inputs = [1024].into_iter().map(create_tab_map).collect_vec();

    let mut group = c.benchmark_group("To tab point");

    for (batch_size, snapshot, fold_point) in inputs {
        group.bench_with_input(
            BenchmarkId::new("to_tab_point", batch_size),
            &snapshot,
            |bench, snapshot| {
                bench.iter(|| {
                    snapshot.fold_point_to_tab_point(fold_point);
                });
            },
        );
    }

    group.finish();
}

fn to_fold_point_benchmark(c: &mut Criterion) {
    let dispatcher = TestDispatcher::new(1);
    let cx = gpui::TestAppContext::build(dispatcher, None);

    let create_tab_map = |length: usize| {
        let mut rng = StdRng::seed_from_u64(1);
        let text = RandomCharIter::new(&mut rng)
            .take(length)
            .collect::<String>();
        let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));

        let buffer_snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
        use editor::display_map::*;
        let (_, inlay_snapshot) = InlayMap::new(buffer_snapshot);
        let (_, fold_snapshot) = FoldMap::new(inlay_snapshot.clone());

        let fold_point = fold_snapshot.to_fold_point(
            inlay_snapshot.to_point(InlayOffset(
                rng.random_range(MultiBufferOffset(0)..MultiBufferOffset(length)),
            )),
            Bias::Left,
        );

        let (_, snapshot) = TabMap::new(fold_snapshot, NonZeroU32::new(4).unwrap());
        let tab_point = snapshot.fold_point_to_tab_point(fold_point);

        (length, snapshot, tab_point)
    };

    let inputs = [1024].into_iter().map(create_tab_map).collect_vec();

    let mut group = c.benchmark_group("To fold point");

    for (batch_size, snapshot, tab_point) in inputs {
        group.bench_with_input(
            BenchmarkId::new("to_fold_point", batch_size),
            &snapshot,
            |bench, snapshot| {
                bench.iter(|| {
                    snapshot.tab_point_to_fold_point(tab_point, Bias::Left);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, to_tab_point_benchmark, to_fold_point_benchmark);
criterion_main!(benches);
