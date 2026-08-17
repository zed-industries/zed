use criterion::{Criterion, criterion_group, criterion_main};
use merman::render::{LayoutOptions, headless_layout_options};
use merman_core::{Engine, ParseOptions};
use std::hint::black_box;

const MINDMAP_BALANCED_TREE: &str = include_str!("fixtures/stress_balanced_tree_009.mmd");

fn bench_mindmap_layout_stress(c: &mut Criterion) {
    let engine = Engine::new();
    let parse_opts = ParseOptions::strict();
    let layout: LayoutOptions = headless_layout_options();

    let parsed = engine
        .parse_diagram_for_render_model_sync(MINDMAP_BALANCED_TREE, parse_opts)
        .expect("parse")
        .expect("supported diagram");

    let mut group = c.benchmark_group("layout_stress");
    group.sample_size(50);

    // Mindmap layout is fast (µs–ms scale depending on fixture), so we batch to get stable signals
    // from fixed-cost + allocation changes inside the manatee COSE pipeline.
    group.bench_function("mindmap_balanced_tree_layout_x50", move |b| {
        b.iter(|| {
            let mut acc: usize = 0;
            for _ in 0..50usize {
                let layouted =
                    merman_render::layout_parsed_render_layout_only(black_box(&parsed), &layout)
                        .expect("layout");
                match layouted {
                    merman_render::model::LayoutDiagram::MindmapDiagram(layouted) => {
                        acc ^= layouted.nodes.len();
                        acc ^= layouted.edges.len();
                    }
                    _ => unreachable!("expected mindmap layout"),
                }
            }
            black_box(acc);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_mindmap_layout_stress);
criterion_main!(benches);
