use std::num::NonZeroU32;

use criterion::{Bencher, BenchmarkId, criterion_group, criterion_main};
use editor::{MultiBuffer, display_map::FoldPoint};
use gpui::{AppContext, TestDispatcher};
use itertools::Itertools;
use rand::{RngCore, rngs::StdRng};
use text::Bias;

pub fn benches() {
    let app = gpui::Application::new();

    app.run(move |cx| {
        let mut criterion: criterion::Criterion<_> =
            (criterion::Criterion::default()).configure_from_args();
        // setup app context
        let create_tab_map = |length: usize| {
            dbg!(length);
            let text = std::iter::repeat_n('\t', length).collect::<String>();
            let buffer = MultiBuffer::build_simple(&text, cx);

            let buffer_snapshot = buffer.read(cx).snapshot(cx);
            use editor::display_map::*;
            let (inlay_map, inlay_snapshot) = InlayMap::new(buffer_snapshot.clone());
            let (fold_map, fold_snapshot) = FoldMap::new(inlay_snapshot.clone());
            let fold_point = fold_snapshot
                .to_fold_point(inlay_snapshot.to_point(InlayOffset(length / 2)), Bias::Left);
            let (tab_map, snapshot) =
                TabMap::new(fold_snapshot.clone(), NonZeroU32::new(4).unwrap());

            dbg!("Returnig length snapshot and fold point");
            (length, snapshot, fold_point)
        };

        let inputs = [1024].into_iter().map(create_tab_map).collect_vec();
        for (batch_size, snapshot, fold_point) in inputs {
            dbg!("About to call bench with input");
            criterion.bench_with_input(
                BenchmarkId::new("to_tab_point", batch_size),
                &snapshot,
                |bench, snapshot| {
                    dbg!("About to call bencher");
                    bench.iter(|| {
                        dbg!("calling to tab point");
                        snapshot.to_tab_point(fold_point);
                    });
                },
            );
        }

        dbg!("App finished booting");
        criterion::Criterion::default()
            .configure_from_args()
            .final_summary();
    });
}

// criterion_main!(benches);

fn main() {
    benches();
    dbg!("Getting final summary");
}
