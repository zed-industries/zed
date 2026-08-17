use criterion::{Criterion, criterion_group, criterion_main};
use merman::render::{LayoutOptions, SvgRenderOptions, headless_layout_options};
use merman_core::{Engine, ParseOptions};
use std::hint::black_box;

const ARCH_MANY_SERVICES_ONE_GROUP: &str =
    include_str!("fixtures/stress_architecture_batch3_many_services_one_group_059.mmd");

fn bench_architecture_stress(c: &mut Criterion) {
    let engine = Engine::new();
    let parse_opts = ParseOptions::strict();
    let layout: LayoutOptions = headless_layout_options();

    let parsed = engine
        .parse_diagram_for_render_model_sync(ARCH_MANY_SERVICES_ONE_GROUP, parse_opts)
        .expect("parse")
        .expect("supported diagram");
    let layouted =
        merman_render::layout_parsed_render_layout_only(&parsed, &layout).expect("layout");

    let svg_opts = SvgRenderOptions {
        diagram_id: Some(merman::render::sanitize_svg_id(
            "stress_architecture_batch3_many_services_one_group_059",
        )),
        ..SvgRenderOptions::default()
    };

    let mut group = c.benchmark_group("render_stress");
    group.sample_size(50);

    // Architecture render is very fast (µs-scale) on medium fixtures, so we batch to get stable
    // signals from per-render fixed-cost changes.
    group.bench_function("architecture_many_services_one_group_x200", move |b| {
        b.iter(|| {
            let mut acc: usize = 0;
            for _ in 0..200usize {
                let svg = merman_render::svg::render_layout_svg_parts_for_render_model_with_config(
                    black_box(&layouted),
                    &parsed.model,
                    &parsed.meta.effective_config,
                    parsed.meta.title.as_deref(),
                    layout.text_measurer.as_ref(),
                    &svg_opts,
                )
                .expect("render");
                acc ^= svg.len();
            }
            black_box(acc);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_architecture_stress);
criterion_main!(benches);
