use criterion::{Criterion, criterion_group, criterion_main};
use merman::render::{LayoutOptions, SvgRenderOptions, headless_layout_options};
use merman_core::{Engine, ParseOptions};
use std::hint::black_box;

const FLOWCHART_MEDIUM: &str = include_str!("fixtures/flowchart_medium.mmd");
const FLOWCHART_PORTS_HEAVY: &str = include_str!("fixtures/flowchart_ports_heavy.mmd");

fn bench_flowchart_stress(c: &mut Criterion) {
    let engine = Engine::new();
    let parse_opts = ParseOptions::strict();
    let layout: LayoutOptions = headless_layout_options();

    let mut group = c.benchmark_group("render_stress");
    group.sample_size(50);

    // Render-only stress: amplify micro-bench timings by batching many renders per iteration so
    // small A/B changes are less likely to be lost in noise.
    for (name, input, repeats) in [
        ("flowchart_medium_x50", FLOWCHART_MEDIUM, 50usize),
        ("flowchart_ports_heavy_x20", FLOWCHART_PORTS_HEAVY, 20usize),
    ] {
        let parsed = engine
            .parse_diagram_for_render_model_sync(input, parse_opts)
            .expect("parse")
            .expect("supported diagram");
        let layouted =
            merman_render::layout_parsed_render_layout_only(&parsed, &layout).expect("layout");
        let svg_opts = SvgRenderOptions {
            diagram_id: Some(merman::render::sanitize_svg_id(name)),
            ..SvgRenderOptions::default()
        };
        let text_measurer = layout.text_measurer.clone();

        // Pre-check that rendering works once, outside measurement.
        let _ = merman_render::svg::render_layout_svg_parts_for_render_model_with_config(
            &layouted,
            &parsed.model,
            &parsed.meta.effective_config,
            parsed.meta.title.as_deref(),
            text_measurer.as_ref(),
            &svg_opts,
        )
        .expect("render");

        group.bench_function(name, move |b| {
            b.iter(|| {
                let mut acc: usize = 0;
                for _ in 0..repeats {
                    let svg =
                        merman_render::svg::render_layout_svg_parts_for_render_model_with_config(
                            black_box(&layouted),
                            &parsed.model,
                            &parsed.meta.effective_config,
                            parsed.meta.title.as_deref(),
                            text_measurer.as_ref(),
                            &svg_opts,
                        )
                        .expect("render");
                    acc ^= svg.len();
                }
                black_box(acc);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_flowchart_stress);
criterion_main!(benches);
