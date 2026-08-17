use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use merman::render::{LayoutOptions, SvgRenderOptions, headless_layout_options};
use merman_core::{Engine, ParseMetadata, ParseOptions};
use std::hint::black_box;

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "flowchart_tiny",
            include_str!("fixtures/flowchart_tiny.mmd"),
        ),
        (
            "flowchart_small",
            include_str!("fixtures/flowchart_small.mmd"),
        ),
        (
            "flowchart_medium",
            include_str!("fixtures/flowchart_medium.mmd"),
        ),
        (
            "flowchart_large",
            include_str!("fixtures/flowchart_large.mmd"),
        ),
        (
            "flowchart_ports_heavy",
            include_str!("fixtures/flowchart_ports_heavy.mmd"),
        ),
        (
            "flowchart_weave",
            include_str!("fixtures/flowchart_weave.mmd"),
        ),
        (
            "flowchart_backedges_subgraphs",
            include_str!("fixtures/flowchart_backedges_subgraphs.mmd"),
        ),
        (
            "flowchart_sparse_components",
            include_str!("fixtures/flowchart_sparse_components.mmd"),
        ),
        (
            "flowchart_lanes_crossfeed",
            include_str!("fixtures/flowchart_lanes_crossfeed.mmd"),
        ),
        (
            "flowchart_grid_feedback",
            include_str!("fixtures/flowchart_grid_feedback.mmd"),
        ),
        (
            "flowchart_fanout_returns",
            include_str!("fixtures/flowchart_fanout_returns.mmd"),
        ),
        (
            "flowchart_label_collision",
            include_str!("fixtures/flowchart_label_collision.mmd"),
        ),
        (
            "flowchart_nested_clusters",
            include_str!("fixtures/flowchart_nested_clusters.mmd"),
        ),
        (
            "flowchart_asymmetric_components",
            include_str!("fixtures/flowchart_asymmetric_components.mmd"),
        ),
        (
            "flowchart_parallel_merges",
            include_str!("fixtures/flowchart_parallel_merges.mmd"),
        ),
        (
            "flowchart_long_edge_labels",
            include_str!("fixtures/flowchart_long_edge_labels.mmd"),
        ),
        (
            "flowchart_selfloop_bidi",
            include_str!("fixtures/flowchart_selfloop_bidi.mmd"),
        ),
        (
            "flowchart_component_packing",
            include_str!("fixtures/flowchart_component_packing.mmd"),
        ),
        (
            "flowchart_direction_conflict",
            include_str!("fixtures/flowchart_direction_conflict.mmd"),
        ),
        (
            "flowchart_parallel_label_stack",
            include_str!("fixtures/flowchart_parallel_label_stack.mmd"),
        ),
        ("class_tiny", include_str!("fixtures/class_tiny.mmd")),
        ("class_medium", include_str!("fixtures/class_medium.mmd")),
        (
            "class_namespace_dense",
            include_str!("fixtures/stress_class_dense_namespaces_generics_001.mmd"),
        ),
        ("state_tiny", include_str!("fixtures/state_tiny.mmd")),
        ("state_medium", include_str!("fixtures/state_medium.mmd")),
        ("sequence_tiny", include_str!("fixtures/sequence_tiny.mmd")),
        (
            "sequence_medium",
            include_str!("fixtures/sequence_medium.mmd"),
        ),
        ("er_medium", include_str!("fixtures/er_medium.mmd")),
        ("info_medium", include_str!("fixtures/info_medium.mmd")),
        ("pie_medium", include_str!("fixtures/pie_medium.mmd")),
        (
            "mindmap_medium",
            include_str!("fixtures/mindmap_medium.mmd"),
        ),
        (
            "journey_medium",
            include_str!("fixtures/journey_medium.mmd"),
        ),
        (
            "timeline_medium",
            include_str!("fixtures/timeline_medium.mmd"),
        ),
        ("gantt_medium", include_str!("fixtures/gantt_medium.mmd")),
        (
            "requirement_medium",
            include_str!("fixtures/requirement_medium.mmd"),
        ),
        (
            "gitgraph_medium",
            include_str!("fixtures/gitgraph_medium.mmd"),
        ),
        ("c4_medium", include_str!("fixtures/c4_medium.mmd")),
        ("sankey_medium", include_str!("fixtures/sankey_medium.mmd")),
        (
            "quadrant_medium",
            include_str!("fixtures/quadrant_medium.mmd"),
        ),
        ("zenuml_medium", include_str!("fixtures/zenuml_medium.mmd")),
        ("block_medium", include_str!("fixtures/block_medium.mmd")),
        ("packet_medium", include_str!("fixtures/packet_medium.mmd")),
        ("kanban_medium", include_str!("fixtures/kanban_medium.mmd")),
        (
            "architecture_medium",
            include_str!("fixtures/architecture_medium.mmd"),
        ),
        ("radar_medium", include_str!("fixtures/radar_medium.mmd")),
        (
            "treemap_medium",
            include_str!("fixtures/treemap_medium.mmd"),
        ),
        (
            "xychart_medium",
            include_str!("fixtures/xychart_medium.mmd"),
        ),
    ]
}

fn layout_digest(layout: &merman_render::model::LayoutDiagram) -> u64 {
    use merman_render::model::LayoutDiagram;

    fn mix(acc: u64, v: u64) -> u64 {
        acc.rotate_left(7) ^ v.wrapping_mul(0x9E37_79B9_7F4A_7C15)
    }

    fn f64_bits(v: f64) -> u64 {
        v.to_bits()
    }

    fn digest_nodes_edges(
        nodes: &[merman_render::model::LayoutNode],
        edges: &[merman_render::model::LayoutEdge],
    ) -> u64 {
        let mut acc: u64 = 0;
        for n in nodes.iter().take(16) {
            acc = mix(acc, f64_bits(n.x));
            acc = mix(acc, f64_bits(n.y));
            acc = mix(acc, f64_bits(n.width));
            acc = mix(acc, f64_bits(n.height));
        }
        for e in edges.iter().take(16) {
            if let Some(p) = e.points.first() {
                acc = mix(acc, f64_bits(p.x));
                acc = mix(acc, f64_bits(p.y));
            }
            if let Some(p) = e.points.last() {
                acc = mix(acc, f64_bits(p.x));
                acc = mix(acc, f64_bits(p.y));
            }
        }
        acc
    }

    match layout {
        // Cheap, layout-dependent digests to keep the optimizer honest without serializing.
        LayoutDiagram::FlowchartV2(v) => digest_nodes_edges(&v.nodes, &v.edges),
        LayoutDiagram::SequenceDiagram(v) => digest_nodes_edges(&v.nodes, &v.edges),
        LayoutDiagram::StateDiagramV2(v) => digest_nodes_edges(&v.nodes, &v.edges),
        LayoutDiagram::ClassDiagramV2(v) => digest_nodes_edges(&v.nodes, &v.edges),
        LayoutDiagram::ArchitectureDiagram(v) => digest_nodes_edges(&v.nodes, &v.edges),
        LayoutDiagram::MindmapDiagram(v) => digest_nodes_edges(&v.nodes, &v.edges),
        _ => 0,
    }
}

fn bench_parse(c: &mut Criterion) {
    let engine = Engine::new();
    let parse_opts = ParseOptions::strict();

    let mut group = c.benchmark_group("parse");
    for (name, input) in fixtures() {
        // Skip fixtures that are not yet supported by `merman` to keep the bench runnable while
        // we expand coverage. Unsupported fixtures should be tracked separately as parity work.
        if engine
            .parse_diagram_for_render_model_sync(input, parse_opts)
            .is_err()
        {
            eprintln!("[bench][skip][parse] {name}: parse error");
            continue;
        }
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, data| {
            b.iter(|| {
                let parsed =
                    engine.parse_diagram_for_render_model_sync(black_box(data), parse_opts);
                let parsed = match parsed {
                    Ok(v) => v,
                    Err(_) => return,
                };
                black_box(parsed.is_some());
            })
        });
    }
    group.finish();
}

fn bench_parse_known_type(c: &mut Criterion) {
    let engine = Engine::new();
    let parse_opts = ParseOptions::strict();

    let mut group = c.benchmark_group("parse_known_type");
    for (name, input) in fixtures() {
        let diagram_type = match engine.parse_metadata_sync(input, parse_opts) {
            Ok(Some(v)) => v.diagram_type,
            Ok(None) => {
                eprintln!("[bench][skip][parse_known_type] {name}: not a diagram");
                continue;
            }
            Err(_) => {
                eprintln!("[bench][skip][parse_known_type] {name}: metadata error");
                continue;
            }
        };

        // Pre-check that the known-type parse succeeds.
        if engine
            .parse_diagram_with_type_sync(&diagram_type, input, parse_opts)
            .is_err()
        {
            eprintln!(
                "[bench][skip][parse_known_type] {name}: parse_with_type({diagram_type}) error"
            );
            continue;
        }

        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, data| {
            b.iter(|| {
                let parsed =
                    engine.parse_diagram_with_type_sync(&diagram_type, black_box(data), parse_opts);
                let parsed = match parsed {
                    Ok(v) => v,
                    Err(_) => return,
                };
                black_box(parsed.is_some());
            })
        });
    }
    group.finish();
}

fn bench_layout(c: &mut Criterion) {
    let engine = Engine::new();
    let parse_opts = ParseOptions::strict();
    let layout: LayoutOptions = headless_layout_options();

    let mut group = c.benchmark_group("layout");
    for (name, input) in fixtures() {
        let parsed = match engine.parse_diagram_for_render_model_sync(input, parse_opts) {
            Ok(Some(v)) => v,
            Ok(None) => continue,
            Err(_) => {
                eprintln!("[bench][skip][layout] {name}: parse error");
                continue;
            }
        };

        // Pre-check that layout works.
        if merman_render::layout_parsed_render_layout_only(&parsed, &layout).is_err() {
            eprintln!("[bench][skip][layout] {name}: layout error");
            continue;
        }

        group.bench_with_input(BenchmarkId::from_parameter(name), &parsed, |b, data| {
            b.iter(|| {
                let diagram =
                    match merman_render::layout_parsed_render_layout_only(black_box(data), &layout)
                    {
                        Ok(v) => v,
                        Err(_) => return,
                    };
                black_box(layout_digest(&diagram));
            })
        });
    }
    group.finish();
}

fn bench_render(c: &mut Criterion) {
    let engine = Engine::new();
    let parse_opts = ParseOptions::strict();
    let layout: LayoutOptions = headless_layout_options();

    let mut group = c.benchmark_group("render");
    for (name, input) in fixtures() {
        let parsed = match engine.parse_diagram_for_render_model_sync(input, parse_opts) {
            Ok(Some(v)) => v,
            Ok(None) => continue,
            Err(_) => {
                eprintln!("[bench][skip][render] {name}: parse error");
                continue;
            }
        };
        let layouted = match merman_render::layout_parsed_render_layout_only(&parsed, &layout) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("[bench][skip][render] {name}: layout error");
                continue;
            }
        };

        let svg_opts = SvgRenderOptions {
            diagram_id: Some(merman::render::sanitize_svg_id(name)),
            ..SvgRenderOptions::default()
        };

        // Pre-check that SVG rendering works.
        if merman_render::svg::render_layout_svg_parts_for_render_model_with_config(
            &layouted,
            &parsed.model,
            &parsed.meta.effective_config,
            parsed.meta.title.as_deref(),
            layout.text_measurer.as_ref(),
            &svg_opts,
        )
        .is_err()
        {
            eprintln!("[bench][skip][render] {name}: svg render error");
            continue;
        }

        group.bench_with_input(BenchmarkId::from_parameter(name), &layouted, |b, data| {
            b.iter(|| {
                let svg =
                    match merman_render::svg::render_layout_svg_parts_for_render_model_with_config(
                        black_box(data),
                        &parsed.model,
                        &parsed.meta.effective_config,
                        parsed.meta.title.as_deref(),
                        layout.text_measurer.as_ref(),
                        &svg_opts,
                    ) {
                        Ok(v) => v,
                        Err(_) => return,
                    };
                black_box(svg.len());
            })
        });
    }
    group.finish();
}

fn bench_end_to_end(c: &mut Criterion) {
    let engine = Engine::new();
    let parse_opts = ParseOptions::strict();
    let layout = headless_layout_options();

    let mut group = c.benchmark_group("end_to_end");
    for (name, input) in fixtures() {
        let svg_opts = SvgRenderOptions {
            diagram_id: Some(merman::render::sanitize_svg_id(name)),
            ..SvgRenderOptions::default()
        };

        // Pre-check end-to-end viability once to keep the bench stable.
        if merman::render::render_svg_sync(&engine, input, parse_opts, &layout, &svg_opts).is_err()
        {
            eprintln!("[bench][skip][end_to_end] {name}: svg render error");
            continue;
        }

        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, data| {
            b.iter(|| {
                let svg = match merman::render::render_svg_sync(
                    &engine,
                    black_box(data),
                    parse_opts,
                    &layout,
                    &svg_opts,
                ) {
                    Ok(Some(v)) => v,
                    Ok(None) => return,
                    Err(_) => return,
                };
                black_box(svg.len());
            });
        });
    }
    group.finish();
}

fn bench_parse_typed(c: &mut Criterion) {
    let registry = merman_core::DetectorRegistry::for_pinned_mermaid_baseline();

    let mut group = c.benchmark_group("parse_typed");
    for (name, input) in fixtures() {
        if !name.starts_with("class_") {
            continue;
        }

        // Pre-check typed parse viability once to keep the bench stable.
        let pre = match merman_core::preprocess_diagram_with_known_type(
            input,
            &registry,
            Some("classDiagram"),
        ) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let mut effective_config = merman_core::generated::default_site_config();
        effective_config.deep_merge(pre.config.as_value());
        let title = pre
            .title
            .as_ref()
            .map(|t| merman_core::sanitize::sanitize_text(t, &effective_config))
            .filter(|t| !t.is_empty());

        let meta = ParseMetadata {
            diagram_type: "classDiagram".to_string(),
            config: pre.config.clone(),
            effective_config,
            title,
        };

        if merman_core::diagrams::class::parse_class_typed(&pre.code, &meta).is_err() {
            eprintln!("[bench][skip][parse_typed] {name}: parse_class_typed error");
            continue;
        }

        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, data| {
            b.iter(|| {
                let pre = match merman_core::preprocess_diagram_with_known_type(
                    black_box(data),
                    &registry,
                    Some("classDiagram"),
                ) {
                    Ok(v) => v,
                    Err(_) => return,
                };

                let mut effective_config = merman_core::generated::default_site_config();
                effective_config.deep_merge(pre.config.as_value());
                let title = pre
                    .title
                    .as_ref()
                    .map(|t| merman_core::sanitize::sanitize_text(t, &effective_config))
                    .filter(|t| !t.is_empty());

                let meta = ParseMetadata {
                    diagram_type: "classDiagram".to_string(),
                    config: pre.config,
                    effective_config,
                    title,
                };

                let parsed = merman_core::diagrams::class::parse_class_typed(&pre.code, &meta);
                let parsed = match parsed {
                    Ok(v) => v,
                    Err(_) => return,
                };

                black_box(parsed.classes.len());
            })
        });
    }
    group.finish();
}

fn bench_parse_typed_only(c: &mut Criterion) {
    let registry = merman_core::DetectorRegistry::for_pinned_mermaid_baseline();

    let mut group = c.benchmark_group("parse_typed_only");
    for (name, input) in fixtures() {
        if !name.starts_with("class_") {
            continue;
        }

        let pre = match merman_core::preprocess_diagram_with_known_type(
            input,
            &registry,
            Some("classDiagram"),
        ) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let mut effective_config = merman_core::generated::default_site_config();
        effective_config.deep_merge(pre.config.as_value());
        let title = pre
            .title
            .as_ref()
            .map(|t| merman_core::sanitize::sanitize_text(t, &effective_config))
            .filter(|t| !t.is_empty());

        let meta = ParseMetadata {
            diagram_type: "classDiagram".to_string(),
            config: pre.config,
            effective_config,
            title,
        };

        // Pre-check typed parse viability once to keep the bench stable.
        if merman_core::diagrams::class::parse_class_typed(&pre.code, &meta).is_err() {
            eprintln!("[bench][skip][parse_typed_only] {name}: parse_class_typed error");
            continue;
        }

        let code = pre.code;
        let meta = meta;
        group.bench_function(BenchmarkId::from_parameter(name), move |b| {
            b.iter(|| {
                let parsed =
                    merman_core::diagrams::class::parse_class_typed(black_box(&code), &meta);
                let parsed = match parsed {
                    Ok(v) => v,
                    Err(_) => return,
                };
                black_box(parsed.classes.len());
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_parse,
    bench_parse_known_type,
    bench_parse_typed,
    bench_parse_typed_only,
    bench_layout,
    bench_render,
    bench_end_to_end
);
criterion_main!(benches);
